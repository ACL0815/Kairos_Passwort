# Violentmonkey und lokale Brücke

Zuständig: **PR-6b + PR-10**, `feat/keepassxc-browser-bridge`, auf integriertem
PR-6. Prinzipien modelliert, kein fremder Implementierungscode kopiert.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| 2FAS Browser Extension | [Quellstand](https://github.com/twofas/2fas-browser-extension/tree/9ad6e83d12792d0b3d8d66f018ace6765ebc3139) | GPL-3.0; Dateilizenzen beachten | `src/background/`, `src/background/events/` | Browser-Ereignisse, Kopplung und vertrauenswürdige Nachrichtenflüsse voneinander trennen | 4 |
| keepassxc-browser | [Protokollclient](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-browser/background/keepass.js) | GPL-3.0; Dateilizenz beachten | `keepassxc-browser/background/keepass.js` | Authentifizierte Verbindung, Nachrichtenvalidierung und Lebenszyklus der lokalen Brücke | 5 |

Ein Userscript besitzt andere Vertrauensgrenzen als eine Browser-Erweiterung.
Transport, erlaubte Ursprünge, Kopplung, Nachrichtenformate und Fehlerzustände
müssen vor der Umsetzung festgelegt werden. Die Vorbilder liefern Prinzipien;
ihre Erweiterungsarchitektur wird nicht ungeprüft auf Violentmonkey übertragen.

## Recherche und Umsetzung, 2026-10-25

| Projekt | Permalink | Lizenz | Datei | Prinzip | Passung + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| 2FAS Auth | [Crypt.js](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/src/background/functions/Crypt.js), [decryptToken.js](https://github.com/twofas/2fas-browser-extension/blob/9ad6e83d12792d0b3d8d66f018ace6765ebc3139/src/background/functions/decryptToken.js) | GPL-3.0-or-later, Dateiheader und LICENSE | `src/background/functions/{Crypt,decryptToken}.js` | Bewusstes Pairing, Schlüsselbesitz, getrennte Freigabe | **4/5** Ablaufvorbild. RSA-OAEP/Relay nicht übernommen; HMAC schützt Integrität und Schlüsselbesitz, bietet keine entsprechende Verschlüsselung. | 2026-10-25 |
| KeePassXC-Browser | [Protokoll](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-protocol.md) | GPL-3.0 | `keepassxc-protocol.md` | Association und Sitzungs-/Nachrichtenprüfung trennen | **5/5** für vorhandenen Client; kein zweiter NaCl-Protokollstack im Userscript. | 2026-10-25 |
| Syncthing | [api.go](https://github.com/syncthing/syncthing/blob/a4a2da477f32d29eda98cc6eb08554dfe5078875/lib/api/api.go) | MPL-2.0, Dateiheader | `lib/api/api.go` | Host-Prüfung gegen DNS-Rebinding, Auth und CSRF als separate Schranken | **5/5**, Architekturvorbild. Keine permissive OPTIONS-/CORS-Behandlung übernehmen. | 2026-10-25 |
| Violentmonkey | [GM-Doku](https://github.com/violentmonkey/violentmonkey.github.io/blob/84ebe202ff7f05d16d49fd932196f7b4c6748f62/content/pages/api/gm.mdx), [Metadaten](https://github.com/violentmonkey/violentmonkey.github.io/blob/84ebe202ff7f05d16d49fd932196f7b4c6748f62/content/pages/api/metadata-block.mdx), [Engine-Lizenz](https://github.com/violentmonkey/violentmonkey/blob/8d003838f3f3adad398fc62459929b437ca5bfa1/LICENSE) | Engine MIT; separate Doku nur verlinkt | `gm.mdx`, `metadata-block.mdx` | Privilegierte Requests/Storage, `anonymous`, Content-Kontext, keine Frames | **5/5** API-Vorbild; keine globale Tabs-Identität. Origin/Host können durch GM gesetzt werden. `@connect` ersetzt keine Authentisierung. | 2026-10-25 |
| Bitwarden Desktop | [Native Messaging](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/apps/desktop/src/main/native-messaging.main.ts) | GPL-3.0, LICENSE_GPL.txt | `apps/desktop/src/main/native-messaging.main.ts` | Extension-ID-Allowlist und IPC statt allgemeinem HTTP-Port | **4/5**, Zielpfad für echte Extension; Userscript besitzt diese APIs nicht. | 2026-10-25 |
| KeePassRPC / Kee | [KeePassRPCExt.cs](https://github.com/kee-org/keepassrpc/blob/dc0a59b60b3ac21da9944bc5b98fe96e706af80a/KeePassRPC/KeePassRPCExt.cs) | GPL-2.0 | `KeePassRPC/KeePassRPCExt.cs` | Loopback und explizite Verbindungserlaubnis | **3/5**, Vergleich. Origin-Schema allein reicht nicht zur Identität. | 2026-10-25 |
| Zotero Connector | [README](https://github.com/zotero/zotero-connectors/blob/a9bcb4d442fd93ef0a9b7cd83f493d94bd2a76a8/README.md) | AGPL-3.0, COPYING | `README.md` | Seitenskripte von privilegiertem Hintergrund trennen | **3/5**, Architekturvergleich; README allein belegt keinen Pairing-/CSRF-Schutz. | 2026-10-25 |
| 1Password | [Verbindungssicherheit](https://support.1password.com/1password-browser-connection-security/) | Proprietär, Herstellerdokumentation | Sicherheitsbeschreibung, keine Quelldatei | Lokales Gegenüber über Browser-Signatur prüfen | **3/5**, Vergleich zur bloßen Headerprüfung; keine Implementierung einsehbar/übernommen. | 2026-10-25 |
| httparse | [1.10.1](https://github.com/seanmonstar/httparse/blob/9f29e79f9832dbd0ae5220acb17c1866745bdecd/src/lib.rs) | MIT OR Apache-2.0 | `src/lib.rs` | Bestehender HTTP-Headerparser statt eigenem Parser | **5/5**, unveränderte Abhängigkeit; Kairos prüft Framing/Host/Origin separat. | 2026-10-25 |
| subtle | [2.6.1](https://github.com/dalek-cryptography/subtle/blob/5457b5448b021d1da101ababbb854e6657233943/src/lib.rs) | BSD-3-Clause | `src/lib.rs` | Konstantzeitvergleich im RustCrypto-MAC-Verifikationspfad | **5/5**, unveränderte Abhängigkeit. Kein Schutz gegen lokalen Prozesszugriff. | 2026-10-25 |

Drei read-only Luna-Recherchen lieferten Vergleiche. Worker-Stichproben: 2FAS
`Crypt.js` (Dateilizenz, RSA-OAEP, Speicher-Fallback), KeePassXC-Protokoll
(Association/Identifikationskey), Syncthing `api.go` (MPL-Header, Host-/CSRF),
offizielle [GM-API](https://violentmonkey.github.io/api/gm/) (`anonymous`, setzbare
Origin/Host/Referer). Keine direkte Codeübernahme; GPL/AGPL/MPL dienen als Modelle.

Entscheidung nach Abnahme-Nachbesserung: Protokoll v2 mit HMAC-SHA256
(RustCrypto `hmac` 0.12.1 / `sha2` 0.10.9, MIT OR Apache-2.0; WebCrypto).
Der manuell aus Kairos übertragene 256-Bit-Code signiert ausschließlich Pairing;
der Dauerschlüssel wird auf beiden Seiten domänengetrennt per HMAC abgeleitet.
Kein Code, Dauerschlüssel oder client_token im HTTP-Kanal. Konstante MAC-Prüfung,
exakte Body-Bytes, Methode/Pfad/Host/Version/Zeit/Nonce gebunden. Separater
DPAPI-Blob, zwei Minuten Pairing, ±30 Sekunden Nachrichtenfenster, 61 Sekunden
Noncecache. Vollständige Kanonisierung und gemeinsamer Rust-/WebCrypto-/Node-
[Testvektor](../../browser/hmac-vector.json) im
[Bedrohungsmodell/Protokoll](../../browser/README.md).

KeePassXC-Association trennt Besitzprüfung und persistente Zuordnung, verwendet
aber Native Messaging/NaCl. 2FAS Crypt.js bindet RSA-OAEP an seinen Relay-Kanal.
Wir übernehmen die Trennung, keinen zweiten NaCl-/RSA-Stack. Der manuelle
Pairing-Kanal bleibt vertrauensbedürftig. Port-Impersonation kann signierte
Requests weiterleiten und falsche 204 senden, ohne selbst signieren zu können.
HTTP-Kontext ist unverschlüsselt, Serveridentität und aktive Relay-Abwehr fehlen.
Malware im selben Konto und manipulierte Scripts bleiben außerhalb der Grenze.
Native Messaging bleibt der Zielpfad. Keine CORS-/normalen Origins, exakter Host;
Kontextvorschläge bleiben ausdrücklich zu bestätigen.

Speicherhygiene: Rohe Schlüsselbytes werden mit `Zeroizing` gepuffert. Die
internen schlüsselabgeleiteten ipad/opad-Zustände von `hmac` 0.12 werden beim
Drop nicht garantiert gelöscht. JavaScript-/CryptoKey-Kopien besitzen ebenfalls
keine zugesagte Speicherlöschung; Prozessspeicherzugriff bleibt außerhalb der
geschützten Grenze.

Review-Nachbesserung: Winsock-Sockets vor Timeout-I/O explizit blockierend;
verzögerte Header/Body als echter TCP-Regressionstest. Kanonische Extension-
Origins zusätzlich zur HMAC-Prüfung zulässig, normale Origins weiter verboten;
VMs Origin-Entfernung im MV2-Pfad ist in
[requests-core.js](https://github.com/violentmonkey/violentmonkey/blob/8d003838f3f3adad398fc62459929b437ca5bfa1/src/background/utils/requests-core.js)
belegt. Engine-Metadaten sind keine Authentisierung. Mindestabstand 500 ms im
Script/250 ms im Server verhindert Fokusfluten. Bei verlorener Pairing-Antwort
bleibt ein vorläufiger GM-Schlüssel für eine explizite read-only Probe erhalten.
Username-Opt-in bleibt an URL/Fenster und eine nicht verlängerbare Zehn-Sekunden-
Frist gebunden. Echte VM-Kompatibilität bleibt manuell zu prüfen.
