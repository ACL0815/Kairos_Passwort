// Independently written protocol/privacy tests for the actual userscript.
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { runInNewContext } = require('node:vm');
const { webcrypto, createHash, createHmac } = require('node:crypto');
const script = readFileSync(__dirname + '/kairos-bridge.user.js', 'utf8');
for (const line of ['@connect      127.0.0.1', '@noframes', '@inject-into  content', '@grant        GM_xmlhttpRequest']) assert(script.includes(line));
assert(!script.includes('@require'));
assert(!script.includes('unsafeWindow'));
const menus = new Map(); const store = new Map(); const sent = []; const events = new Map();
let heartbeat; let status = 204; let answer = Buffer.alloc(32, 42).toString('base64url'); let hold = false; let held;
let now = 1000000; let failNetwork = false;
let pageFocused = true;
const location = { href: 'https://example.invalid/signup?private=TOKEN#secret' };
class Input { constructor() { this.form = null; this.type = 'password'; } }
const focused = new Input();
const document = { visibilityState: 'visible', hasFocus: () => pageFocused, activeElement: focused,
    addEventListener: (type, fn) => events.set(type, fn) };
const hooks = {};
runInNewContext(script.replace('})();', 'testHooks.send = send; testHooks.hmac = hmac; testHooks.busy = () => inFlight; })();'), {
    testHooks: hooks,
    crypto: webcrypto, URL, Uint8Array, TextEncoder, Date: { now: () => now }, Promise, Error, Array,
    btoa: text => Buffer.from(text, 'binary').toString('base64'),
    atob: text => Buffer.from(text, 'base64').toString('binary'),
    HTMLInputElement: Input, HTMLTextAreaElement: class {},
    location, document,
    window: { addEventListener: (type, fn) => events.set(type, fn) },
    setInterval: fn => { heartbeat = fn; },
    prompt: () => answer, confirm: () => { pageFocused = true; return true; }, alert: () => {},
    GM_getValue: (key, fallback) => store.get(key) ?? fallback,
    GM_setValue: (key, value) => store.set(key, value), GM_deleteValue: key => store.delete(key),
    GM_registerMenuCommand: (label, fn) => menus.set(label, fn),
    GM_xmlhttpRequest: options => { sent.push(options); if (failNetwork) options.ontimeout(); else if (hold) held = options; else options.onload({ status }); },
});
const flush = async () => {
    const deadline = Date.now() + 5000;
    while (hooks.busy() && !held) {
        if (Date.now() > deadline) throw new Error('userscript stayed busy');
        await new Promise(resolve => setTimeout(resolve, 1));
    }
};
const trigger = type => { now += 600; events.get(type)({isTrusted:true, target:focused}); };
(async () => {
    assert.equal(sent.length, 0, 'unpaired must not publish');
    const pair = menus.get('Kairos: mit Kopplungscode verbinden');
    status = 403; await pair(); assert.equal(store.size, 0, 'failed pairing must not persist');
    status = 204; await pair(); await flush(); assert.equal(store.size, 1);
    const context = sent.at(-1); const body = JSON.parse(context.data);
    assert.equal(context.url, 'http://127.0.0.1:19471/v2/context');
    assert.equal(body.url, 'https://example.invalid/signup');
    assert(!('username' in body)); assert(!('password' in body));
    assert.equal(context.anonymous, true);
    const clientKey = createHmac('sha256', Buffer.from(answer, 'base64url')).update('kairos-bridge-v2-client-key').digest('base64url');
    assert.equal(store.get('kairos-bridge-v2-key'), clientKey);
    assert.notEqual(clientKey, answer);
    for (const request of sent) {
        assert(!('X-Kairos-Token' in request.headers));
        assert(!JSON.stringify(request.headers).includes(answer));
        assert(!request.data.includes(answer)); assert(!request.data.includes(clientKey));
        const message = JSON.parse(request.data), path = new URL(request.url).pathname;
        const canonical = ['kairos-bridge-v2', 'POST', path, '127.0.0.1:19471', String(message.timestamp), message.nonce, createHash('sha256').update(request.data).digest('hex')].join('\n');
        const secret = path === '/v2/pair' ? answer : clientKey;
        assert.equal(request.headers['X-Kairos-HMAC'], createHmac('sha256', Buffer.from(secret, 'base64url')).update(canonical).digest('hex'));
    }
    assert.match(body.nonce, /^[0-9a-f]{32}$/);
    assert.equal(new Set(sent.map(s => JSON.parse(s.data).nonce)).size, sent.length);
    focused.form = { querySelector: () => Object.assign(new Input(), { type:'email', value:'user@example.invalid' }) };
    pageFocused = false; // extension menu temporarily owns focus
    await menus.get('Kairos: Formular-Benutzername einmal senden')(); await flush();
    assert.equal(JSON.parse(sent.at(-1).data).username, 'user@example.invalid');
    trigger('focus'); await flush(); assert(!('username' in JSON.parse(sent.at(-1).data)));
    hold = true; trigger('focus'); await flush();
    const beforeBusy = sent.length;
    await menus.get('Kairos: Formular-Benutzername einmal senden')();
    assert.equal(sent.length, beforeBusy, 'busy must visibly discard username action');
    location.href = 'https://example.invalid/different';
    held.onload({status:204}); held = undefined; hold = false; await flush();
    trigger('focus'); await flush();
    assert(!('username' in JSON.parse(sent.at(-1).data)), 'no deferred username in a different context');
    const beforeFlood = sent.length;
    for (let i = 0; i < 1100; i++) { events.get('focus')({isTrusted:true}); await flush(); }
    assert.equal(sent.length, beforeFlood, 'forced events obey the minimum interval');
    now += 1000; events.get('focus')({isTrusted:false}); await flush();
    assert.equal(sent.length, beforeFlood, 'synthetic events never invoke privileged requests');
    document.visibilityState = 'hidden'; trigger('visibilitychange'); await flush();
    assert(sent.at(-1).url.endsWith('/v2/clear'));
    const count = sent.length; heartbeat(); await flush(); assert.equal(sent.length, count);
    await menus.get('Kairos: Verbindung im Userscript vergessen')(); assert.equal(store.size, 0);
    failNetwork = true; await pair();
    assert.equal(store.size, 1, 'uncertain pairing keeps only the provisional token');
    assert(!store.has('kairos-bridge-v2-key'));
    assert(store.has('kairos-bridge-v2-pending-key'));
    const beforeRePair = sent.length; await pair();
    assert.equal(sent.length, beforeRePair, 'uncertain pairing cannot overwrite its recovery key');
    failNetwork = false;
    await menus.get('Kairos: Ausstehende Kopplung prüfen')();
    assert(sent.at(-1).url.endsWith('/v2/probe'));
    assert(store.has('kairos-bridge-v2-key')); assert(!store.has('kairos-bridge-v2-pending-key'));
    await menus.get('Kairos: Verbindung im Userscript vergessen')(); assert.equal(store.size, 0);
    const vector = JSON.parse(readFileSync(__dirname + '/hmac-vector.json', 'utf8'));
    assert.equal(Buffer.from(await hooks.hmac(vector.secret, vector.canonical)).toString('hex'), vector.hmac);
    assert.equal(Buffer.from(await hooks.hmac(vector.secret, 'kairos-bridge-v2-client-key')).toString('base64url'), vector.client_key);
    now = 1700000000000;
    const originalRandom = webcrypto.getRandomValues;
    webcrypto.getRandomValues = bytes => { for (let i=0; i<bytes.length; i++) bytes[i]=i; return bytes; };
    await hooks.send('/v2/context', vector.secret, {url:'https://example.invalid/signup',username:'Jörg'});
    webcrypto.getRandomValues = originalRandom;
    assert.equal(sent.at(-1).data, vector.body);
    assert.equal(sent.at(-1).headers['X-Kairos-HMAC'], vector.hmac);
    assert.equal(createHash('sha256').update(vector.body).digest('hex'), vector.body_sha256);
    console.log('HMAC Rust=WebCrypto=Node vector: ' + vector.hmac);
    console.log('Userscript: protocol/privacy assertions passed (HMAC, no transmitted secret, pairing separation, username race).');
})().catch(error => { console.error(error); process.exitCode = 1; });
