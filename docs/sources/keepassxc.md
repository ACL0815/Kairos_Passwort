# KeePassXC-Browser-Protokoll und spätere URL-Brücke

## Ergänzung PR-6b / PR-10: bestätigter F11-Fluss

`feat/keepassxc-browser-bridge` verbindet den vorhandenen Client mit F11.
Die Desktop-Hotkey-Grenze bleibt erhalten: keine verlässliche Tab-URL aus dem
Hotkey. Violentmonkey liefert nur einen kurzlebigen Vorschlag mit lokalem
HWND/PID-Hinweis, den der Nutzer ausdrücklich übernimmt. Fehlt dieser, bleibt
das URL-Feld leer. Query und Fragment werden vor dem Schreiben entfernt.

| Projekt | Permalink | Lizenz | Datei | Prinzip | Passung + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| KeePassXC | [BrowserAction](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserAction.cpp) | GPL-2.0 OR GPL-3.0, Dateiheader | `src/browser/BrowserAction.cpp` | `test-associate`, bewusstes `associate`, neuer `set-login` | **5/5**, bestehende Crate unverändert wiederverwendet. Kein Code kopiert; Annahme ist keine Persistenzbestätigung. | 2026-10-25 |
| 2FAS Auth | [Crypt.js](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/src/background/functions/Crypt.js) | GPL-3.0-or-later | `src/background/functions/Crypt.js` | Nutzerbestätigte Kopplung und getrennte Schlüsselrollen | **4/5**, Ablaufvorbild. Keine Übernahme von RSA-/Relay-Architektur. | 2026-10-25 |
| Syncthing | [api.go](https://github.com/syncthing/syncthing/blob/a4a2da477f32d29eda98cc6eb08554dfe5078875/lib/api/api.go) | MPL-2.0 | `lib/api/api.go` | Host-/CSRF-Prüfung zusätzlich zu Auth | **5/5**, lokale Brücke übernimmt Prinzipien mit strengeren Origin-/CORS-Regeln. | 2026-10-25 |
| Bitwarden | [Native Messaging](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/apps/desktop/src/main/native-messaging.main.ts) | GPL-3.0 | `apps/desktop/src/main/native-messaging.main.ts` | Extension-ID-Bindung | **4/5**, späterer Zielpfad; mit Userscript nicht erreichbar. | 2026-10-25 |

F11-Reihenfolge: Zielfeld erfassen, frisch generieren, URL/Benutzer/Gruppe prüfen,
`test_associate`, bei `MissingAssociation` getrennte geführte Nutzerkopplung,
erneut Eintragen bestätigen, `set_login`. Erst nach Annahme folgt ein frischer
Klick zum Tippen über das ursprüngliche `FocusTarget`. Das vermeidet eine
veraltete Autotype-Freigabe nach langen Dialogen. Ablehnung/Timeout: kein Tippen,
kein Retry. Eingabefehler nach Annahme: sichtbares Teilergebnis, kein automatisches
Löschen eines möglicherweise angelegten Eintrags. Browser-DOM-Feldwechsel und
KeePassXCs DB-/Gruppen-/Persistenzgrenzen bleiben sichtbar dokumentiert.

Die Brücke verwendet einen getrennten DPAPI-Blob mit festem Marker im vorhandenen
insert-only Adapter; KeePassXC-Identifikationsschlüssel werden nicht geteilt.
Beschädigte/widerrufene Zuordnung wird nicht automatisch ersetzt. Bewusster
Neukopplungsablauf per manuellem Backup-Umbenennen und Widerruf in KeePassXC
steht in der [Browser-Anleitung](../../browser/README.md). Vollständige
Quellenbewertung für VM und localhost: [browser.md](browser.md).

---

Zuständig: **PR-6**, `feat/keepassxc-browser`, Priorität 3, Basis `main` bei
`fa35aded13b50a67a817db79258b1ba1c95a9d46`. Der präzisierte PR-Auftrag verschiebt
die App-/Hotkey-Integration in einen Folge-PR. Keine Änderungen an gemeinsamen
Architektur-, Roadmap- oder Quellenindexdateien. Stand/Abrufdatum: **2026-10-25**.

## Bewertete Vorbilder

Es wurden Prinzipien übertragen, **keine fremden Implementierungen kopiert**.
GPL-/BUSL-Quellen sind ausschließlich Modelle, keine Abhängigkeiten. Die
NaCl-Fixture enthält veröffentlichte kryptografische Testdaten, keinen Algorithmus.
Direkte unveränderte Crate-Abhängigkeiten stehen in der zweiten Tabelle.

| Projekt | Permalink | Lizenz | Datei | Prinzip | Passung 1–5 + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| KeePassXC 2.7.12 – Actions | [BrowserAction](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserAction.cpp), [Service](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserService.cpp) | GPL-3.0-or-later laut Dateikopf, [Lizenz](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/LICENSE.GPL-3) | `src/browser/BrowserAction.cpp`, `BrowserService.cpp` | Pairing-Dialog, ID-/Key-Prüfung, Eintragsanlage ohne UUID, vorhandene Gruppe | **5** – maßgebliche Host-Implementierung; Erfolgsantwort beweist keinen gespeicherten neuen Eintrag | 2026-10-25 |
| KeePassXC 2.7.12 – Transport/Krypto | [Shared](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserShared.cpp), [Host](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserHost.cpp), [Proxy](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/proxy/NativeMessagingProxy.cpp), [MessageBuilder](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/src/browser/BrowserMessageBuilder.cpp) | GPL-3.0-or-later, [Lizenz](https://github.com/keepassxreboot/keepassxc/blob/860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda/LICENSE.GPL-3) | `BrowserShared.cpp`, `BrowserHost.cpp`, `NativeMessagingProxy.cpp`, `BrowserMessageBuilder.cpp` | Pipe-Pfad, rohe JSON-Bytes, `crypto_box_easy`, Nonce +1 | **5** – entscheidend für Kompatibilität; die `readAll`-Annahme des alten Hosts wird nicht als Client-Framing übernommen | 2026-10-25 |
| KeePassXC-Browser | [Protokoll](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-protocol.md), [Client](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-browser/background/keepass.js) | GPL-3.0, [Lizenz](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/LICENSE) | `keepassxc-protocol.md`, `background/keepass.js` | Getrennte Sitzungs-/Identifikationsschlüssel, Nonce-Antwortprüfung, JSON-Schema | **5** – direkter Vertrag; Beispiele teils verkürzt/veraltet, deshalb Host-Code zusätzlich geprüft | 2026-10-25 |
| git-credential-keepassxc | [Nachrichten](https://github.com/Frederick888/git-credential-keepassxc/blob/d8442e050128789f54aaf51d160e2b27aa5b927e/src/keepassxc/messages/structs.rs), [Windows-Socket](https://github.com/Frederick888/git-credential-keepassxc/blob/d8442e050128789f54aaf51d160e2b27aa5b927e/src/utils/socket.rs) | GPL-3.0-or-later, [Manifest](https://github.com/Frederick888/git-credential-keepassxc/blob/d8442e050128789f54aaf51d160e2b27aa5b927e/Cargo.toml), [Lizenz](https://github.com/Frederick888/git-credential-keepassxc/blob/d8442e050128789f54aaf51d160e2b27aa5b927e/LICENSE) | `messages/structs.rs`, `utils/socket.rs` | Unabhängiger typisierter Rust-Client, direkter Pipe-Zugriff und Qt-Benutzerencoding | **4** – sehr passend; kurze Reads/Timeouts als Nachrichtenende werden bewusst nicht übernommen | 2026-10-25 |
| keepassxc-proxy-rust | [main](https://github.com/varjolintu/keepassxc-proxy-rust/blob/ece4006f801c1b8bfd9694b1df499e2685fe8231/src/main.rs), [Socket](https://github.com/varjolintu/keepassxc-proxy-rust/blob/ece4006f801c1b8bfd9694b1df499e2685fe8231/src/proxy_socket.rs) | GPL-3.0-or-later, [Lizenz](https://github.com/varjolintu/keepassxc-proxy-rust/blob/ece4006f801c1b8bfd9694b1df499e2685fe8231/LICENSE) | `src/main.rs`, `src/proxy_socket.rs` | STDIO-Rahmung und lokales IPC getrennt halten | **2** – veralteter verschachtelter Windows-Pfad, Windows-`try_clone` unimplementiert; keine Laufzeitvorlage | 2026-10-25 |
| 2FAS Auth Extension | [Pairing](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/src/background/events/handleConfigurationRequest.js), [Zustellung](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/src/background/events/handleLoginRequest.js) | GPL-3.0-or-later, [Lizenz](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/LICENSE) | `handleConfigurationRequest.js`, `handleLoginRequest.js` | Paarung von Geräte-ID/Key, ungekoppelte Geräte abweisen, fokussierten Frame und Origin vor Zustellung prüfen | **3** – gute Sicherheitsprinzipien für die spätere Brücke; Mobiltelefon/WebSocket/OTP nicht erforderlich für PR-6 | 2026-10-25 |
| 2FAS Pass Extension | [QR-Daten](https://github.com/twofas/2fas-pass-browser-extension/blob/8a385e3fcfcd07eeb521d561942638111d907c9c/entrypoints/background/websocket/connect/generateQRData.js) | BUSL-1.1 mit späterem GPL-Wechsel, [Lizenz](https://github.com/twofas/2fas-pass-browser-extension/blob/8a385e3fcfcd07eeb521d561942638111d907c9c/LICENSE) | `entrypoints/background/websocket/connect/generateQRData.js` | Sitzungs-ID, dauerhafter/kurzlebiger Key und Signatur in expliziter Kopplung | **3** – thematisch nah, aber mobile WebSocket-Architektur und Lizenzbeschränkung; nur Prinzipien, keine Abhängigkeit | 2026-10-25 |
| Bitwarden Desktop/Browser | [Desktop](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/apps/desktop/src/main/native-messaging.main.ts), [Browser](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/apps/browser/src/background/nativeMessaging.background.ts) | GPL-3.0 für geprüfte Dateien außerhalb `bitwarden_license`, [Lizenz](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/LICENSE.txt) | `native-messaging.main.ts`, `nativeMessaging.background.ts` | Extension-IDs erlauben, Sitzungsschlüsselaustausch und verschlüsselte Desktop-Kommunikation | **3** – nützliche Vertrauensgrenze; biometrisches Entsperren ist ein anderer Zweck | 2026-10-25 |
| Chromium os_crypt | [Windows-Adapter](https://github.com/chromium/chromium/blob/130.0.6723.58/components/os_crypt/sync/os_crypt_win.cc) | BSD-3-Clause, [Lizenz](https://github.com/chromium/chromium/blob/130.0.6723.58/LICENSE) | `components/os_crypt/sync/os_crypt_win.cc` | Persistente Schlüssel durch CurrentUser-DPAPI schützen | **5** – kleiner passender OS-Adapter; kein Abhängigkeitsbedarf zu kairos-store | 2026-10-25 |
| Microsoft UI Automation | [Überblick](https://github.com/MicrosoftDocs/win32/blob/ca8274decead49403d70ceeed4af8d4ca74eaf11/desktop-src/WinAuto/uiauto-uiautomationoverview.md), [Patterns](https://github.com/MicrosoftDocs/win32/blob/ca8274decead49403d70ceeed4af8d4ca74eaf11/desktop-src/WinAuto/uiauto-controlpatternsoverview.md) | Dokumentation CC-BY-4.0, [Lizenz](https://github.com/MicrosoftDocs/win32/blob/ca8274decead49403d70ceeed4af8d4ca74eaf11/LICENSE) | `uiauto-uiautomationoverview.md`, `uiauto-controlpatternsoverview.md` | Werte über Accessibility-Baum/ValuePattern lesen | **3** – Windows-Grundlage; Browser-Provider und Aktualität werden nicht garantiert | 2026-10-25 |
| Descolada UIA-v2 | [Browser-Helfer](https://github.com/Descolada/UIA-v2/blob/2846a9b10518a95cf26a6c43671a9512b231ccb7/Lib/UIA_Browser.ahk) | MIT, [Lizenz](https://github.com/Descolada/UIA-v2/blob/2846a9b10518a95cf26a6c43671a9512b231ccb7/LICENSE) | `Lib/UIA_Browser.ahk` | Chrome/Edge/Firefox unterscheiden; Adressleiste versus Dokument-URL | **3** – gute Fehlerfall-Referenz; automatisches Aktivieren und `https://`-Ergänzen werden nicht übernommen | 2026-10-25 |
| WebAutoType, geprüfter Aldaviva-Fork | [README](https://github.com/Aldaviva/WebAutoType/blob/03ba75d4639f0a2448552f91fd35eed7f4b2239c/Readme.txt), [Chrome](https://github.com/Aldaviva/WebAutoType/blob/03ba75d4639f0a2448552f91fd35eed7f4b2239c/WebAutoType/ChromeBrowserUrlReader.cs), [Firefox](https://github.com/Aldaviva/WebAutoType/blob/03ba75d4639f0a2448552f91fd35eed7f4b2239c/WebAutoType/FirefoxBrowserUrlReader.cs) | Im geprüften Fork keine eindeutige Lizenzdatei/Dateilizenz gefunden; keine Übernahme/Abhängigkeit | `Readme.txt`, `ChromeBrowserUrlReader.cs`, `FirefoxBrowserUrlReader.cs` | URL statt Fenstertitel; browserspezifisches IAccessible und fehlende Accessibility als Grenze | **2** – älterer Ansatzvergleich, keine Zusage heutiger Browserkompatibilität; [Originalprojekt](https://sourceforge.net/projects/webautotype/) | 2026-10-25 |
| Violentmonkey | [Metadaten](https://github.com/violentmonkey/violentmonkey.github.io/blob/84ebe202ff7f05d16d49fd932196f7b4c6748f62/content/pages/api/metadata-block.mdx), [GM-API](https://github.com/violentmonkey/violentmonkey.github.io/blob/84ebe202ff7f05d16d49fd932196f7b4c6748f62/content/pages/api/gm.mdx) | Engine MIT, [Lizenz](https://github.com/violentmonkey/violentmonkey/blob/8d003838f3f3adad398fc62459929b437ca5bfa1/LICENSE); separate Doku nur verlinkt | `metadata-block.mdx`, `gm.mdx` | Grants begrenzen, Seitenkontext und privilegierte Extension-API unterscheiden | **2** – sinnvoll für ausdrücklich bestätigten Prototyp, ungeeignet als alleinige vertrauenswürdige aktive-Tab-Quelle | 2026-10-25 |

## Verwendete kompatible Bibliotheken und Testdaten

| Projekt | Permalink | Lizenz | Datei | Prinzip | Passung 1–5 + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| RustCrypto crypto_box 0.9.1 | [Box](https://github.com/RustCrypto/nacl-compat/blob/91078b60b1cacf674885c1dc86bd00b97adaf828/crypto_box/src/lib.rs), [SecretKey](https://github.com/RustCrypto/nacl-compat/blob/91078b60b1cacf674885c1dc86bd00b97adaf828/crypto_box/src/secret_key.rs), [Manifest](https://github.com/RustCrypto/nacl-compat/blob/91078b60b1cacf674885c1dc86bd00b97adaf828/crypto_box/Cargo.toml) | MIT OR Apache-2.0 | `crypto_box/src/lib.rs`, `secret_key.rs` | NaCl-kompatible X25519–XSalsa20–Poly1305 ohne eigene Kryptografie | **5** – unveränderte Abhängigkeit; SecretKey-Rohbytes werden zusätzlich vom Kairos-Owner gelöscht | 2026-10-25 |
| curve25519-dalek 4.1.3 | [Montgomery](https://github.com/dalek-cryptography/curve25519-dalek/blob/curve25519-4.1.3/curve25519-dalek/src/montgomery.rs), [Lizenz](https://github.com/dalek-cryptography/curve25519-dalek/blob/curve25519-4.1.3/curve25519-dalek/LICENSE) | BSD-3-Clause | `curve25519-dalek/src/montgomery.rs` | Öffentlichen Key ableiten und nicht beitragende DH-Peer-Schlüssel erkennen | **5** – bereits crypto_box-Abhängigkeit; validierte Primitive statt selbst implementierter Kurvenarithmetik | 2026-10-25 |
| RustCrypto salsa20 0.10.2 | [Quelle/Manifest](https://github.com/RustCrypto/stream-ciphers/tree/salsa20-v0.10.2/salsa20) | MIT OR Apache-2.0 | `salsa20/src/lib.rs`, `Cargo.toml` | Zeroize-Feature für internen Cipher-Zustand aktivieren | **5** – Feature-Unifizierung, keine zweite Kryptoimplementierung | 2026-10-25 |
| libsodium 1.0.20 | [Vektor-Eingabe](https://github.com/jedisct1/libsodium/blob/9511c982fb1d046470a8b42aa36556cdb7da15de/test/default/box_easy.c), [Erwartungswerte](https://github.com/jedisct1/libsodium/blob/9511c982fb1d046470a8b42aa36556cdb7da15de/test/default/box_easy.exp), [Lizenz](https://github.com/jedisct1/libsodium/blob/9511c982fb1d046470a8b42aa36556cdb7da15de/LICENSE) | ISC | `test/default/box_easy.c`, `box_easy.exp` | Unabhängiger veröffentlichter NaCl-Testvektor, MAC-Präfix und Bytefolge | **5** – ausschließlich Testdaten als Fixture; keine Bibliotheksabhängigkeit/Codeübernahme | 2026-10-25 |
| getrandom 0.2.17 | [Quelle/Manifest](https://github.com/rust-random/getrandom/tree/v0.2.17) | MIT OR Apache-2.0 | `src/lib.rs`, Windows-Backend | Fallibler OS-Zufall für Schlüssel, Nonces und clientID | **5** – kein schwacher Ersatzpfad | 2026-10-25 |
| base64 0.22.1 | [Quelle/Manifest](https://github.com/marshallpierce/rust-base64/tree/v0.22.1) | MIT OR Apache-2.0 | `src/engine/general_purpose`, `Cargo.toml` | Standard-Base64 mit Padding und Längenvalidierung | **5** – Protokollkodierung als Abhängigkeit | 2026-10-25 |
| serde / serde_json | [serde 1.0.229](https://github.com/serde-rs/serde/tree/v1.0.229), [serde_json 1.0.154](https://github.com/serde-rs/json/tree/v1.0.154) | MIT OR Apache-2.0 | `serde/src`, `serde_json/src/{ser,de,iter}.rs` | Typisierte JSON-Felder, Borrowing bei Requests, vollständige JSON-Rahmung | **5** – kein eigener JSON-Parser; Fehlertexte werden verworfen | 2026-10-25 |
| fluent-uri 0.4.1 | [Parser](https://github.com/yescallop/fluent-uri-rs/tree/d9a6a20614f34b00476837eb8904fb01ca3e54df/src), [Lizenz](https://github.com/yescallop/fluent-uri-rs/blob/d9a6a20614f34b00476837eb8904fb01ca3e54df/LICENSE) | MIT | `src/parse.rs`, `component.rs` | RFC-3986-Validierung ohne Schema-/Host-Raten | **4** – kleine Borrowing-API, bewusst ASCII-URI statt Browser-Normalisierung | 2026-10-25 |
| zeroize 1.8.2 | [Quelle/Manifest](https://github.com/RustCrypto/utils/tree/zeroize-v1.8.2/zeroize) | MIT OR Apache-2.0 | `zeroize/src/lib.rs` | Eigene Schlüssel-/Klartextpuffer und MaybeUninit-Repräsentation überschreiben | **5** – unveränderte Workspace-Version; kein Schutz gegen alle historischen Kopien | 2026-10-25 |
| Tokio 1.54.1 | [Named Pipes](https://github.com/tokio-rs/tokio/blob/b53b2001ed2ccfd9b87e32a0c6da1a1eebdc80b8/tokio/src/net/windows/named_pipe.rs), [Lizenz](https://github.com/tokio-rs/tokio/blob/b53b2001ed2ccfd9b87e32a0c6da1a1eebdc80b8/LICENSE) | MIT | `tokio/src/net/windows/named_pipe.rs`, `time/timeout.rs` | Overlapped-I/O, Gesamtdeadline, SECURITY_IDENTIFICATION | **5** – vermeidet eigenen unsicheren Overlapped-/CancelIoEx-Adapter; synchroner Worker-Facade | 2026-10-25 |
| windows-sys 0.61.2 | [Quelle/Manifest](https://github.com/microsoft/windows-rs/tree/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/sys) | MIT OR Apache-2.0 | `crates/libs/sys/src/Windows/Win32/Security/Cryptography` | Offizielle DPAPI-/LocalFree-Bindings | **5** – kleiner eigener DPAPI-Adapter ohne Store-Abhängigkeit | 2026-10-25 |
| tempfile 3.27.0 | [Quelle/Manifest](https://github.com/Stebalien/tempfile/tree/v3.27.0) | MIT OR Apache-2.0 | `src/file/mod.rs`, Windows-Implementierung | Vollständig geschriebene verschlüsselte Datei atomar ohne Überschreiben veröffentlichen | **5** – bewährtes `persist_noclobber`, auch für konkurrierende Initialisierung | 2026-10-25 |

## Belegter Transportvertrag

Zielstand ist **KeePassXC 2.7.12**, Commit
`860e51d8e9a5a67e73de79b173ae9bd5bdb5ebda`; installiert wurde lesend Version
2.7.12.0 festgestellt. Es wurden weder Einstellungen noch reale Datenbanken verändert.

`BrowserShared::localServerPath()` bildet unter Windows
`/org.keepassxc.KeePassXC.BrowserServer_` plus `qgetenv("USERNAME")`. Qt macht
daraus `\\.\pipe\org.keepassxc.KeePassXC.BrowserServer_<USERNAME>`.
Der unabhängige Rust-Client bestätigt Vollpfad und Encoding-Fallstrick. Kairos
liest mit dem kopierenden CRT-`getenv_s` den schmalen Wert und dekodiert wie Qt
UTF-8-lossy; die weite Windows-Umgebung kann bei Nicht-ASCII-Benutzern abweichen.
[Microsoft-CRT-Vertrag](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/getenv-s-wgetenv-s),
[Qt-Umgebungsvariablen](https://doc.qt.io/qt-6/qtenvironmentvariables.html).

`NativeMessagingProxy::transferStdinMessage` schreibt nur JSON-Text auf den
Socket. `transferSocketMessage` ergänzt erst für Browser-STDOUT eine native
32-Bit-Byte-Länge. `BrowserHost::sendClientData` schreibt rohe kompakte JSON-Bytes.
Die direkte Pipe hat **kein** Präfix und keinen Newline-Delimiter. Der Client
erkennt JSON-Grenzen und erhält Restbytes. Genau eine Anfrage ist offen; bekannte
`database-locked`-/`database-unlocked`-Events werden unter derselben Deadline
übersprungen. Eine Zählgrenze wird nicht verwendet, da diese nach längerer
Leerlaufphase gültige Antworten verwerfen könnte. Die monotone Gesamtdeadline
wird auch während der Verarbeitung bereits gepufferter Ereignisse geprüft.
Jede Antwort muss die erwartete Action besitzen.
Äußere Nachrichten sind auf 1 MiB und Eingabefelder auf 16 KiB begrenzt; die
maximale serialisierte Anfrage liegt unter KeePassXCs innerem 655360-Byte-Limit.
Der 2.7.12-Host verwendet selbst `readAll` statt vollständiger Stream-Rahmung;
Teilung großer Schreibvorgänge kann serverseitig zu einem Timeout führen. Das
wird als unbekannter Ausgang behandelt, niemals durch automatischen Retry kaschiert.

## Nachrichten und Schlüsselrollen

| Aktion | Klartext vor Verschlüsselung | Antwortprüfung |
|---|---|---|
| `change-public-keys` | Unverschlüsselt: `action`, `publicKey`, `nonce`, `clientID` | `success` als String `"true"`, Host-Key 32 Bytes, Nonce +1; kein kryptografischer Identitätsbeweis des Servers |
| `associate` | `action`, `key` = Sitzungs-Public-Key, `idKey` = separater Identifikations-Public-Key | KeePassXC-Freigabedialog; authentifizierte ID und Hash dauerhaft speichern |
| `test-associate` | `action`, gespeicherte `id`, Identifikations-`key` | Nonce, MAC, ID und Datenbank-Hash prüfen |
| `get-databasehash` | `action` | Authentifizierten aktiven DB-Hash lesen, kein `triggerUnlock` |
| `set-login` | `action`, `id`, `url`, `submitUrl`, `login`, `password`; optional `group` + `groupUuid` | Nonce, MAC, Erfolg und Hash; **kein** bestehender Eintrags-`uuid`, keine Favicon-Anforderung |

Die letzten vier Aktionen verwenden außen `{action, message, nonce, clientID}`.
`message` ist Base64 über `MAC || Ciphertext` von NaCl X25519–XSalsa20–Poly1305.
Jede Anfrage erhält 24 frische OS-Zufallsbytes; die Antwort verwendet die
little-endian um eins inkrementierte Nonce (wie `sodium_increment`). Äußere und
authentifiziert innere Antwort-Nonce werden geprüft. Falsche Action, MAC,
Nonce, manipulierte Nachricht oder Transportfehler sperren die Sitzung. Der
erste öffentliche Schlüsselwechsel ist bewusst kein authentifiziertes Handshake.
Nicht beitragende DH-Schlüssel werden anhand des berechneten gemeinsamen Werts
abgewiesen. Systemzufallsfehler führen zum Abbruch.

Die dauerhafte `idKey`-Public-Key ist im Protokoll eine Bearer-Berechtigung und
deshalb wie ein Geheimnis zu schützen; ihre private Hälfte wird verworfen.
IDs und Keys werden nicht geloggt. DPAPI CurrentUser schützt die gesamte
versionierte Zuordnung, atomar und ohne Überschreiben. Keine automatische
Neukopplung bei beschädigtem/verlorenem Store. Eigene Puffer nutzen `Zeroizing`;
`crypto_box`-SecretKey-Rohbytes erhalten zusätzlich einen stabilen Owner, der
nach dem Bibliotheksdestruktor die gesamte MaybeUninit-Repräsentation löscht.
Salsa20-Zeroize ist aktiviert. Vollständige Dump-/Compiler-/OS-Kopienlöschung
wird nicht behauptet. Andere Prozesse desselben Benutzers können grundsätzlich
DPAPI verwenden oder lokale IPC angreifen.
[Microsoft-DPAPI-Grenze](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata).

Aktuell ist eine bewusste Neukopplung manuell: neuer expliziter Store-Pfad oder
bisherige DPAPI-Datei als Backup umbenennen und einen neuen Client koppeln;
gegebenenfalls die alte Verbindung in KeePassXC widerrufen. Es gibt noch keine
`forget`-/`replace`-API und keine automatische Löschung. Ein geführter Ablauf ist
eine Aufgabe des späteren App-PRs.

Generische Protokollfehler sind unverschlüsselt. `errorCode` wird als String
oder Zahl akzeptiert und nur numerisch weitergegeben; Servertexte werden nie
zu öffentlichen Fehlern. Diese Fehler sind keine authentifizierten Aussagen.

### Grenzen der Eintragsanlage

`BrowserAction::handleSetLogin` ruft bei fehlender Eintrags-UUID `addEntry` auf.
In 2.7.12 ist diese Funktion `void`; abgebrochene Datenbankauswahl kann ohne
Anlage zurückkehren, während der Action-Handler trotzdem Erfolg antwortet.
Die DB-Auswahl ist außerdem nicht strikt an die ID gebunden (entsprechendes
TODO im Host). Eine vorherige Hash-/Key-Prüfung verhindert keine spätere
Auswahl oder ein Rennen. `SetLoginOutcome::Accepted` ist deshalb absichtlich
keine Persistenzbestätigung. Gruppen werden nur bei Name plus vorhandener UUID
gewählt; sonst verwendet der Host seine Standardgruppe. Ein neu erstellter
Eintrags-UUID wird nicht zurückgegeben. Keine automatische Wiederholung nach
Timeout, sonst drohen doppelte Einträge. Ein späterer App-Flow muss diesen
unklaren Ausgang verständlich anzeigen und in einer Testdatenbank verifizieren.

## URL des aktiven Browser-Tabs: Optionen und Empfehlung

| Option | Vorteil | Grenze / Entscheidung |
|---|---|---|
| Fenstertitel | Ohne zusätzliche Browserkomponente zugänglich | Seiten können den Titel bestimmen; keine URL-/Origin-Identität. Nicht verwenden. |
| UI Automation / IAccessible | Lokaler Windows-Adapter ohne Extension möglich | Adressleiste kann bearbeiteten Text enthalten oder das Schema ausblenden. Dokument-URL/Provider unterscheiden sich nach Browser und Version; Fokus-/Navigationsrennen. Nur begrenzter späterer Adapter mit Abbruch bei Unklarheit. |
| Eigene Extension mit `activeTab` + Native Messaging | Browser liefert `tab.url`; Extension-ID-Allowlist; kein offener HTTP-Port | Installation/Host-Manifest nötig. `activeTab` verlangt eine Extension-Nutzeraktion oder deren Shortcut; der Desktop-Hotkey allein reicht nicht. Empfohlener Zielpfad. |
| Violentmonkey + lokale Brücke | Passt zum geplanten frühen Browser-Prototyp | Userscript kennt nur seinen Seitenkontext, besitzt keine globale Tabs-/Native-Messaging-API. Seiten-/Userscript-Daten sind keine vertrauenswürdige aktive-Tab-Identität. Nur ausdrücklich bestätigter Prototyp mit zusätzlicher Paarung/Origin-/Replay-Prüfung. |

**Jetzt:** `set_login(url, username, password, group)` erhält die URL explizit.
`validate_url` akzeptiert nur absolute HTTP(S)-ASCII-URIs mit Host, ohne
Zugangsdaten, Leerraum, Backslash oder mehrdeutiges Host-Encoding. Kein Schema
ergänzen, keine aktive Seite erraten, keine URL still auf die Domain verkürzen.
Punycode/Percent-Encoding sind explizit. Query/Fragment können Tokens enthalten;
der spätere Aufrufer wählt bewusst die tatsächlich benötigte URL.

**Später empfohlen:** Browseraktion/Extension-Shortcut liest die URL und bindet
sie an eine kurzlebige Transaktion mit Tab-/Fensterkennung und Navigationsstand.
Vor Zustellung aktiven Tab/Origin erneut prüfen, keine Seitenbotschaft ungeprüft
als URL übernehmen. Native Messaging erlaubt nur konkrete Extension-IDs. Das
ist eine andere Prüfung als die Web-Origin. `activeTab` kann umfassende
Hostberechtigungen vermeiden, solange die Aktion im Browser beginnt.
[Chrome activeTab](https://developer.chrome.com/docs/extensions/develop/concepts/activeTab),
[Chrome Native Messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging),
[Firefox activeTab](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/activeTab_permission).

UIA-Zwischenlösung: Vordergrundziel vor dem Popup erfassen, nur bereits aktive
Browser lesen, vor Verwendung erneut prüfen; keine Fensteraktivierung, kein
Ctrl+L, keine Zwischenablage. Bei fehlender/mehrdeutiger/alter URL abbrechen.
UIA-v2 ergänzt im Adressleistenpfad selbst `https://` und aktiviert teils Fenster;
beides wird nicht übernommen. Firefox-Accessibility muss verfügbar sein;
Kairos soll keine Browserflags oder versteckte Einstellungen ändern.
[Chromium UIA](https://developer.chrome.com/blog/windows-uia-support-update),
[Mozilla Provider-Unterschiede](https://bugzilla.mozilla.org/show_bug.cgi?id=1949871).

Für eine Violentmonkey-Zwischenphase muss ein späterer Entwurf die Grenzen
sichtbar machen: explizite URL-Bestätigung, lokal gebundener Kanal, kein
pauschales CORS, zufälliges Pairing-Geheimnis, authentifizierte begrenzte
Requests und Replay-Abwehr; keine ungefragte Passwortübermittlung an Webseiten.
2FAS liefert dafür Pairing/Zielbindung, Bitwarden Extension-Allowlisting.
Beide Architekturen werden nicht als zusätzlicher Protokollstack in PR-6 eingebaut.

## Verifikation und offene Integration

Testfixtures formulieren das dokumentierte JSON-Schema mit Dummy-Daten neu;
der libsodium-Vektor prüft separat MAC-Position und NaCl-Kompatibilität.
Mock-Peer/In-Memory-Store prüfen Sitzungswechsel, Wiedererkennung, Nonce/MAC,
URL/Gruppe, DB-Wechsel, Fehlerredaktion und fehlenden Retry. Reale DPAPI-Tests
verwenden ausschließlich Temp-Dateien; Pipe-Tests ausschließlich eigene private
Dummy-Server. Echter KeePassXC-Test bleibt `#[ignore]` und liest nur Handshake,
Hash sowie optional eine vorhandene isolierte Test-Assoziation. Anleitung in
der [Crate-README](../../crates/kairos-keepassxc/README.md).

Offen für Folge-PRs: App-/Hotkey-Verdrahtung, konkrete Browserbrücke und ihre
Berechtigungen, URL-Auswahl im Produktablauf, Gruppen-Auswahloberfläche,
mehrere bewusst gepaarte Datenbanken sowie reale Eintragsanlage/Abbruchfälle
in einer ausdrücklich freigegebenen Testdatenbank. Keine echte Datenbank wurde
für die Entwicklung konfiguriert oder beschrieben.
