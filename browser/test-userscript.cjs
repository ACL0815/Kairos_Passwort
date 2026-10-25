// Independently written protocol/privacy tests for the actual userscript.
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { runInNewContext } = require('node:vm');
const { webcrypto } = require('node:crypto');
const script = readFileSync(__dirname + '/kairos-bridge.user.js', 'utf8');
for (const line of ['@connect      127.0.0.1', '@noframes', '@inject-into  content', '@grant        GM_xmlhttpRequest']) assert(script.includes(line));
assert(!script.includes('@require'));
assert(!script.includes('unsafeWindow'));
const menus = new Map(); const store = new Map(); const sent = []; const events = new Map();
let heartbeat; let status = 204; let answer = 'a'.repeat(43); let hold = false; let held;
const location = { href: 'https://example.invalid/signup?private=TOKEN#secret' };
class Input { constructor() { this.form = null; this.type = 'password'; } }
const focused = new Input();
const document = { visibilityState: 'visible', hasFocus: () => true, activeElement: focused,
    addEventListener: (type, fn) => events.set(type, fn) };
runInNewContext(script, {
    crypto: webcrypto, URL, Uint8Array, Date, Promise, Error, Array,
    btoa: text => Buffer.from(text, 'binary').toString('base64'),
    HTMLInputElement: Input, HTMLTextAreaElement: class {},
    location, document,
    window: { addEventListener: (type, fn) => events.set(type, fn) },
    setInterval: fn => { heartbeat = fn; },
    prompt: () => answer, confirm: () => true, alert: () => {},
    GM_getValue: (key, fallback) => store.get(key) ?? fallback,
    GM_setValue: (key, value) => store.set(key, value), GM_deleteValue: key => store.delete(key),
    GM_registerMenuCommand: (label, fn) => menus.set(label, fn),
    GM_xmlhttpRequest: options => { sent.push(options); if (hold) held = options; else options.onload({ status }); },
});
const flush = () => new Promise(resolve => setImmediate(resolve));
(async () => {
    assert.equal(sent.length, 0, 'unpaired must not publish');
    const pair = menus.get('Kairos: mit Kopplungscode verbinden');
    status = 403; await pair(); assert.equal(store.size, 0, 'failed pairing must not persist');
    status = 204; await pair(); await flush(); assert.equal(store.size, 1);
    const context = sent.at(-1); const body = JSON.parse(context.data);
    assert.equal(context.url, 'http://127.0.0.1:19471/v1/context');
    assert.equal(body.url, 'https://example.invalid/signup');
    assert(!('username' in body)); assert(!('password' in body));
    assert.equal(context.anonymous, true);
    assert.notEqual(context.headers['X-Kairos-Token'], answer, 'pairing code is not the persistent key');
    assert.equal(context.headers['X-Kairos-Token'], JSON.parse(sent.at(-2).data).client_token);
    assert.match(body.nonce, /^[0-9a-f]{32}$/);
    assert.equal(new Set(sent.map(s => JSON.parse(s.data).nonce)).size, sent.length);
    focused.form = { querySelector: () => Object.assign(new Input(), { type:'email', value:'user@example.invalid' }) };
    await menus.get('Kairos: Formular-Benutzername einmal senden')(); await flush();
    assert.equal(JSON.parse(sent.at(-1).data).username, 'user@example.invalid');
    events.get('focus')(); await flush(); assert(!('username' in JSON.parse(sent.at(-1).data)));
    hold = true; events.get('focus')();
    const beforeBusy = sent.length;
    await menus.get('Kairos: Formular-Benutzername einmal senden')();
    assert.equal(sent.length, beforeBusy, 'busy must visibly discard username action');
    location.href = 'https://example.invalid/different';
    held.onload({status:204}); hold = false; await flush();
    events.get('focus')(); await flush();
    assert(!('username' in JSON.parse(sent.at(-1).data)), 'no deferred username in a different context');
    document.visibilityState = 'hidden'; events.get('visibilitychange')(); await flush();
    assert(sent.at(-1).url.endsWith('/v1/clear'));
    const count = sent.length; heartbeat(); await flush(); assert.equal(sent.length, count);
    await menus.get('Kairos: Verbindung im Userscript vergessen')(); assert.equal(store.size, 0);
    console.log('Userscript: protocol/privacy assertions passed (pairing key separation, username race included).');
})().catch(error => { console.error(error); process.exitCode = 1; });
