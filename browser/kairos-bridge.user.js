// ==UserScript==
// @name         Kairos Passwort – lokale Kontextbrücke
// @namespace    https://github.com/ACL0815/Kairos_Passwort
// @version      0.2.0
// @description  Bestätigter Seitenkontext für Kairos; keine Passwortauslese.
// @license      MIT
// @match        https://*/*
// @match        http://*/*
// @noframes
// @run-at       document-idle
// @inject-into  content
// @grant        GM_xmlhttpRequest
// @grant        GM_getValue
// @grant        GM_setValue
// @grant        GM_deleteValue
// @grant        GM_registerMenuCommand
// @connect      127.0.0.1
// ==/UserScript==

// Modelled after: Violentmonkey privileged GM requests/storage and KeePassXC
// association. See docs/sources/browser.md; independently written, no copied code.
// Modelled after: https://github.com/violentmonkey/violentmonkey/blob/8d003838f3f3adad398fc62459929b437ca5bfa1/src/background/utils/requests.js
(() => {
    'use strict';
    const endpoint = 'http://127.0.0.1:19471';
    const key = 'kairos-bridge-v2-key';
    const pendingKey = 'kairos-bridge-v2-pending-key';
    let lastSent = 0;
    let inFlight = false;
    const nonce = () => Array.from(crypto.getRandomValues(new Uint8Array(16)),
        byte => byte.toString(16).padStart(2, '0')).join('');

    const hex = bytes => Array.from(new Uint8Array(bytes), byte => byte.toString(16).padStart(2, '0')).join('');
    const encode = text => new TextEncoder().encode(text);
    async function hmac(secret, data) {
        if (!crypto.subtle) throw new Error('WebCrypto nicht verfügbar. Kopplung auf einer vertrauenswürdigen HTTPS-Seite ausführen.');
        if (!/^[A-Za-z0-9_-]{43}$/.test(secret)) throw new Error('Ungültiger Kairos-Schlüssel.');
        const raw = Uint8Array.from(atob(secret.replaceAll('-', '+').replaceAll('_', '/') + '='), c => c.charCodeAt(0));
        if (btoa(String.fromCharCode(...raw)).replaceAll('+', '-').replaceAll('/', '_').replace(/=+$/, '') !== secret) {
            raw.fill(0); throw new Error('Nichtkanonischer Kairos-Schlüssel.');
        }
        try {
            const imported = await crypto.subtle.importKey('raw', raw, { name: 'HMAC', hash: 'SHA-256' }, false, ['sign']);
            return await crypto.subtle.sign('HMAC', imported, encode(data));
        } finally { raw.fill(0); }
    }
    async function send(path, secret, fields = {}, timeout = 1500) {
        const message = { timestamp: Math.floor(Date.now() / 1000), nonce: nonce(), ...fields };
        const data = JSON.stringify(message);
        const digest = hex(await crypto.subtle.digest('SHA-256', encode(data)));
        const canonical = ['kairos-bridge-v2', 'POST', path, '127.0.0.1:19471', String(message.timestamp), message.nonce, digest].join('\n');
        const signature = hex(await hmac(secret, canonical));
        return new Promise((resolve, reject) => {
            GM_xmlhttpRequest({
                method: 'POST', url: endpoint + path,
                // Never forward page cookies or a page Referer/Origin.
                anonymous: true, timeout,
                headers: { 'Content-Type': 'application/json', 'X-Kairos-HMAC': signature },
                data,
                onload: response => {
                    if (response.status === 204) resolve();
                    else { const error = new Error('Kairos hat die Nachricht abgelehnt.'); error.status = response.status; reject(error); }
                },
                onerror: () => reject(new Error('Kairos-Brücke nicht erreichbar.')),
                ontimeout: () => reject(new Error('Kairos-Brücke: Zeitüberschreitung.')),
            });
        });
    }

    async function publish(force = false) {
        const secret = GM_getValue(key, '');
        if (!secret || inFlight || document.visibilityState !== 'visible' || !document.hasFocus()) return;
        if (Date.now() - lastSent < (force ? 500 : 1000)) return;
        const url = new URL(location.href);
        if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password) return;
        inFlight = true;
        lastSent = Date.now();
        const fields = { url: url.origin + url.pathname };
        try { await send('/v2/context', secret, fields); }
        catch { /* Automatic context is best effort. The F11 dialog stays explicit. */ }
        finally { inFlight = false; }
    }

    GM_registerMenuCommand('Kairos: mit Kopplungscode verbinden', async () => {
        if (GM_getValue(pendingKey, '')) { alert('Zuerst „Ausstehende Kopplung prüfen“ nutzen oder die Verbindung bewusst vergessen. Der vorläufige Schlüssel wird nicht überschrieben.'); return; }
        const code = prompt('Kairos → Browser koppeln. Den dort angezeigten Code nur hier eingeben:');
        if (code === null) return;
        if (!/^[A-Za-z0-9_-]{43}$/.test(code.trim())) { alert('Ungültiger Kairos-Code.'); return; }
        try {
            const clientKey = btoa(String.fromCharCode(...new Uint8Array(await hmac(code.trim(), 'kairos-bridge-v2-client-key'))))
                .replaceAll('+', '-').replaceAll('/', '_').replace(/=+$/, '');
            // Preserve an uncertain pairing outcome without exposing the key
            // to the page or automatically retrying a state-changing request.
            GM_setValue(pendingKey, clientKey);
            await send('/v2/pair', code.trim(), {}, 10000);
            GM_setValue(key, clientKey);
            GM_deleteValue(pendingKey);
            alert('Kairos gekoppelt. Seiten-URL wird bei aktivem Tab lokal gemeldet. Benutzername nur über den separaten Menüpunkt.');
            await publish(true);
        } catch (error) {
            if (error.status === 403) GM_deleteValue(pendingKey);
            alert(error.message + ' Bei unbekanntem Ergebnis zuerst „Ausstehende Kopplung prüfen“ nutzen.');
        }
    });
    GM_registerMenuCommand('Kairos: Ausstehende Kopplung prüfen', async () => {
        const secret = GM_getValue(pendingKey, '');
        if (!secret) { alert('Keine ausstehende Kopplung.'); return; }
        try {
            await send('/v2/probe', secret);
            GM_setValue(key, secret); GM_deleteValue(pendingKey);
            alert('Vorhandene Kopplung bestätigt. Keine erneute Kopplung angefordert.');
        } catch (error) {
            if (error.status === 403) GM_deleteValue(pendingKey);
            alert(error.message + ' Bei fehlender Zuordnung neuen Code in Kairos erzeugen.');
        }
    });
    GM_registerMenuCommand('Kairos: Formular-Benutzername einmal senden', async () => {
        if (inFlight) { alert('Eine Kontextmeldung läuft. Menüpunkt danach erneut bewusst ausführen; nichts vorgemerkt.'); return; }
        const secret = GM_getValue(key, '');
        if (!secret) { alert('Zuerst mit Kairos koppeln.'); return; }
        // The extension menu itself may temporarily own focus. Visibility plus
        // the immutable URL/form snapshot is sufficient before confirmation.
        if (document.visibilityState !== 'visible') { alert('Zuerst die gewünschte Seite aktivieren.'); return; }
        const href = location.href;
        const url = new URL(href);
        if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password) return;
        const focused = document.activeElement;
        const form = focused instanceof HTMLInputElement ? focused.form : null;
        const field = form?.querySelector('input[autocomplete="username"], input[type="email"], input[name="username"], input[name="email"]');
        if (!(field instanceof HTMLInputElement) || field.type === 'password') { alert('Kein eindeutiges Benutzername-/E-Mail-Feld im fokussierten Formular erkannt. In Kairos manuell eintragen.'); return; }
        const value = field.value;
        if (value.length > 320 || /[\u0000-\u001f\u007f]/.test(value)) { alert('Benutzername zu lang oder ungültig.'); return; }
        if (!confirm('Diesen Benutzernamen einmal lokal an Kairos senden?\n' + value)) return;
        if (location.href !== href || !document.hasFocus() || document.visibilityState !== 'visible') { alert('Seite oder Fokus geändert. Nichts gesendet.'); return; }
        inFlight = true;
        lastSent = Date.now();
        try { await send('/v2/context', secret, { url: url.origin + url.pathname, username: value }); }
        catch (error) { alert(error.message + ' Nichts automatisch wiederholen.'); }
        finally { inFlight = false; }
    });
    GM_registerMenuCommand('Kairos: Verbindung im Userscript vergessen', () => {
        GM_deleteValue(key);
        GM_deleteValue(pendingKey);
        alert('Lokaler Script-Schlüssel entfernt. Kairos-DPAPI-Datei bleibt erhalten; siehe README zur bewussten Neukopplung.');
    });
    document.addEventListener('focusin', event => {
        if (event.isTrusted && (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement)) void publish(true);
    });
    window.addEventListener('focus', event => { if (event.isTrusted) void publish(true); });
    document.addEventListener('visibilitychange', event => {
        if (!event.isTrusted) return;
        if (document.visibilityState === 'visible') void publish(true);
        else {
            const secret = GM_getValue(key, '');
            if (secret && !inFlight && Date.now() - lastSent >= 500) {
                inFlight = true; lastSent = Date.now();
                void send('/v2/clear', secret).catch(() => {}).finally(() => { inFlight = false; });
            }
        }
    });
    // Short-lived context, also refreshed while the user stays in one field.
    setInterval(() => void publish(), 2000);
    void publish(true);
})();
