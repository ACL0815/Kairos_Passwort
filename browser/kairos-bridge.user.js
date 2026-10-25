// ==UserScript==
// @name         Kairos Passwort – lokale Kontextbrücke
// @namespace    https://github.com/ACL0815/Kairos_Passwort
// @version      0.1.0
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
    const key = 'kairos-bridge-v1-token';
    let lastSent = 0;
    let inFlight = false;
    const nonce = () => Array.from(crypto.getRandomValues(new Uint8Array(16)),
        byte => byte.toString(16).padStart(2, '0')).join('');

    function send(path, token, fields = {}) {
        return new Promise((resolve, reject) => {
            GM_xmlhttpRequest({
                method: 'POST', url: endpoint + path,
                // Never forward page cookies or a page Referer/Origin.
                anonymous: true, timeout: 1500,
                headers: { 'Content-Type': 'application/json', 'X-Kairos-Token': token },
                data: JSON.stringify({ timestamp: Math.floor(Date.now() / 1000), nonce: nonce(), ...fields }),
                onload: response => response.status === 204 ? resolve() : reject(new Error('Kairos hat die Nachricht abgelehnt.')),
                onerror: () => reject(new Error('Kairos-Brücke nicht erreichbar.')),
                ontimeout: () => reject(new Error('Kairos-Brücke: Zeitüberschreitung.')),
            });
        });
    }

    async function publish(force = false) {
        const token = GM_getValue(key, '');
        if (!token || inFlight || document.visibilityState !== 'visible' || !document.hasFocus()) return;
        if (!force && Date.now() - lastSent < 1000) return;
        const url = new URL(location.href);
        if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password) return;
        inFlight = true;
        lastSent = Date.now();
        const fields = { url: url.origin + url.pathname };
        try { await send('/v1/context', token, fields); }
        catch { /* Automatic context is best effort. The F11 dialog stays explicit. */ }
        finally { inFlight = false; }
    }

    GM_registerMenuCommand('Kairos: mit Kopplungscode verbinden', async () => {
        const code = prompt('Kairos → Browser koppeln. Den dort angezeigten Code nur hier eingeben:');
        if (code === null) return;
        if (!/^[A-Za-z0-9_-]{43}$/.test(code.trim())) { alert('Ungültiger Kairos-Code.'); return; }
        try {
            const clientToken = btoa(String.fromCharCode(...crypto.getRandomValues(new Uint8Array(32))))
                .replaceAll('+', '-').replaceAll('/', '_').replace(/=+$/, '');
            await send('/v1/pair', code.trim(), { client_token: clientToken });
            GM_setValue(key, clientToken);
            alert('Kairos gekoppelt. Seiten-URL wird bei aktivem Tab lokal gemeldet. Benutzername nur über den separaten Menüpunkt.');
            await publish(true);
        } catch (error) { alert(error.message + ' Neuen Code in Kairos erzeugen und erneut versuchen.'); }
    });
    GM_registerMenuCommand('Kairos: Formular-Benutzername einmal senden', async () => {
        if (inFlight) { alert('Eine Kontextmeldung läuft. Menüpunkt danach erneut bewusst ausführen; nichts vorgemerkt.'); return; }
        const token = GM_getValue(key, '');
        if (!token) { alert('Zuerst mit Kairos koppeln.'); return; }
        if (!document.hasFocus() || document.visibilityState !== 'visible') { alert('Zuerst die gewünschte Seite aktivieren.'); return; }
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
        try { await send('/v1/context', token, { url: url.origin + url.pathname, username: value }); }
        catch (error) { alert(error.message + ' Nichts automatisch wiederholen.'); }
        finally { inFlight = false; }
    });
    GM_registerMenuCommand('Kairos: Verbindung im Userscript vergessen', () => {
        GM_deleteValue(key);
        alert('Lokaler Script-Schlüssel entfernt. Kairos-DPAPI-Datei bleibt erhalten; siehe README zur bewussten Neukopplung.');
    });
    document.addEventListener('focusin', event => {
        if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) void publish(true);
    });
    window.addEventListener('focus', () => void publish(true));
    document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible') void publish(true);
        else {
            const token = GM_getValue(key, '');
            if (token) void send('/v1/clear', token).catch(() => {});
        }
    });
    // Short-lived context, also refreshed while the user stays in one field.
    setInterval(() => void publish(), 2000);
    void publish(true);
})();
