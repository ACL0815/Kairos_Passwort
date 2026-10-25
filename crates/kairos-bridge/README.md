# kairos-bridge

Kontextdienst für die bestätigte Violentmonkey-Phase. `Bridge::start` bindet nur
`127.0.0.1:19471` mit begrenztem HTTP-Worker. `begin_pairing` ausschließlich auf
Desktop-Nutzeraktion; `latest_for` liefert einen höchstens fünf Sekunden alten
Vorschlag zur lokal erfassten `BrowserWindow`-Identität, keinen Tab-Beweis.

`SecretStore` ermöglicht isolierte Tests; `ProtectedStore` verwendet den
bestehenden insert-only `kairos-keepassxc::DpapiStore` mit separater Datei/Marker.
Keine generierten Passwörter im Kontextdienst. HMAC-SHA256 mit 256-Bit-Schlüssel, kein Secret im HTTP-Kanal,
keine Verschlüsselung oder Serveridentität. Pairing einmalig, zwei Minuten offen;
keine bestehenden Schlüssel ersetzen. Fehler führen zum Abbruch.

Protokoll, Grenzen, Datenschutz, Wiederherstellung und Anleitung:
[browser/README.md](../../browser/README.md). Tests: Pairing, Speicherfehler,
HMAC-Integrität, Replay/Zeitfenster, Fensterzuordnung, URL, HTTP-Framing/Origin/Host und
isolierte DPAPI-Datei.

## Open-Source-Vorbilder

Modelled after: Syncthing (`lib/api/api.go`, MPL-2.0) für Host-/CSRF-Schutz,
KeePassXC und 2FAS für bewusste Kopplung, Violentmonkey für privilegierte GM-Grenze.
Keine Implementierung übernommen. `httparse` (MIT OR Apache-2.0), `hmac`/`sha2`
(MIT OR Apache-2.0), `serde`/`serde_json`, `base64`, `getrandom` und `zeroize` sind
unveränderte kompatible Abhängigkeiten. Quellen/Lizenzen/Bewertung:
[browser.md](../../docs/sources/browser.md).
