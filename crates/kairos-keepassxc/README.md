# kairos-keepassxc

## App-Integration aus PR-6b / PR-10

F11 verwendet jetzt diese API: `test_associate`, bei fehlender Zuordnung eine
getrennte bewusste `associate`-Aktion, anschließend bestätigtes `set_login`.
Die API selbst bleibt unverändert. `kairos-bridge` verwendet den vorhandenen
insert-only `DpapiStore` mit separater `browser-bridge.dpapi` und festem
Record-Marker; keine Wiederverwendung von KeePassXC-Schlüsseln. Modelled after:
KeePassXC/2FAS bewusste Association, Syncthing Host-/CSRF-Schichten. Keine
Codeübernahme. URLs aus VM bleiben manuell bestätigte Vorschläge; Tippen folgt
erst nach Annahme und neuem Klick. Protokoll-/Persistenzgrenzen unten gelten
weiter. [Browser-Anleitung/Bedrohungsmodell](../../browser/README.md),
[Quellenbewertung](../../docs/sources/browser.md).

Eigenständiger synchroner Client für das KeePassXC-Browser-Protokoll, PR-6,
Priorität 3, Basis `main`. Keine Abhängigkeit von Core, App, Hotkeys oder Store.
Die App verbindet `GenerateTypeKeePassXC` mit dieser API. In dieser Crate werden
weder Browser-URLs ermittelt noch Passwörter erzeugt oder ins Zielfeld geschrieben.

## API und Ablauf

`Client<T: Transport, S: AssociationStore>` trennt Transport, Protokoll und Ablage.
Unter Windows liefern `NamedPipeTransport` und `DpapiStore` die Adapter. Alle
Aufrufe gehören in einen blockierenden App-Worker außerhalb einer Tokio-Runtime;
ein verschachtelter Aufruf wird mit `InvalidContext` abgewiesen. Die interne
Tokio-Runtime betreibt ausschließlich abbrechbare Overlapped-Pipe-I/O.

1. `NamedPipeTransport::connect(Duration::from_secs(60))` und
   `DpapiStore::for_current_user()` an `Client::connect` übergeben. `connect`
   führt `change-public-keys` mit frischen Sitzungsschlüsseln/clientID aus.
2. Für die einmalige, ausdrücklich angeforderte Kopplung `associate()` aufrufen.
   KeePassXC fragt selbst nach Name und Freigabe. Vorhandene oder beschädigte
   lokale Daten werden niemals durch eine neue Identität ersetzt.
3. Bei späteren Sitzungen `test_associate()` verwenden; `get_databasehash()`
   liest unabhängig davon den Hash der aktiven, bereits entsperrten Datenbank.
4. `set_login(url, username, password, group)` fordert einen **neuen** Eintrag an.
   Beispiel: `set_login("https://example.invalid/signup", "user", &secret, None)`.
   `Some(Group { name, uuid })` wählt eine bestehende Gruppe; `uuid` ist deren
   32-stellige hexadezimale Gruppen-ID. Eine fehlende/ungültige Gruppe fällt
   serverseitig auf die Standardgruppe zurück. Gruppen werden nicht neu angelegt.

Vor `set-login` prüft der Client die gespeicherte ID, den Identifikationsschlüssel
und den Datenbank-Hash erneut. Ein Feld `uuid` für einen bestehenden Eintrag wird
nie gesendet. Favicon-Downloads bleiben aus. Die URL muss eine absolute ASCII-
HTTP(S)-URI ohne eingebettete Zugangsdaten, Leerraum oder Backslashes sein.
Internationale Hosts/Pfade benötigen Punycode/Percent-Encoding. Es wird weder
ein Schema erraten noch Pfad, Query oder Fragment entfernt. Diese Bestandteile
können selbst Geheimnisse enthalten; der Aufrufer muss die benötigte URL wählen.

**Ergebnisgrenze:** `SetLoginOutcome::Accepted` bestätigt die angenommene Anfrage.
KeePassXC 2.7.12 liefert keine neue Eintrags-ID und kann die Anfrage auch nach
abgebrochener Datenbankauswahl als erfolgreich quittieren. Eine bestimmte
Zieldatenbank, Zielgruppe oder Speicherung auf Platte ist damit nicht bewiesen.
Die Hash-Prüfung schließt Rennen mit einer späteren Datenbankauswahl nicht aus.
Nach Timeout oder Verbindungsfehler kann der Eintrag bereits angelegt sein;
niemals automatisch wiederholen. Die Verbindung wird nach Protokoll-/Kryptofehlern
geschlossen und muss explizit neu aufgebaut werden.

## Transport und Schutz

Windows-Pipe: `\\.\pipe\org.keepassxc.KeePassXC.BrowserServer_<USERNAME>`.
Der Benutzerteil wird wie bei Qt aus C-Runtime-Umgebungsbytes UTF-8-lossy dekodiert;
dadurch können Nicht-ASCII-Kontonamen anders aussehen als in der weiten Windows-
Umgebung. Direkte Pipe: rohe UTF-8-JSON-Objekte, **kein Längenpräfix** und kein
Newline-Trenner. Das vier Byte lange Native-Messaging-Präfix gehört zur separaten
Browser-STDIO-Verbindung von `keepassxc-proxy`.

Eine Anfrage gleichzeitig, maximal 1 MiB äußere Nachricht, insgesamt 1 ms bis
120 Sekunden Deadline je Austausch, einschließlich bereits aufgelaufener Lock-/
Unlock-Broadcasts. Die Deadline wird auch beim Abarbeiten gepufferter Ereignisse
geprüft; eine reine Zählgrenze verwirft keine gültige Antwort. Empfangene Fragmente
und zusammengefasste JSON-Objekte werden anhand
der JSON-Struktur getrennt. KeePassXC 2.7.12 selbst liest mit `readAll` ohne
vollständige Stream-Rahmung; fragmentierte große Anfragen können dort scheitern.
Es gibt deshalb auch bei diesem Fehler keinen automatischen Wiederholungsversuch.

NaCl verwendet X25519–XSalsa20–Poly1305 (`crypto_box::SalsaBox`), 24 zufällige
Nonce-Bytes je Anfrage und den MAC **vor** dem Ciphertext. Beide Antwort-Nonces
(außen und authentifiziert innen) müssen little-endian um genau eins erhöht sein.
Nicht beitragende Host-Schlüssel werden abgewiesen. Der erste öffentliche
Schlüsselwechsel authentifiziert das lokale Serverprogramm nicht: benutzereigene
Pipe, Pairing und DPAPI ersetzen keinen Schutz gegen Schadsoftware desselben
Windows-Benutzers. Tokio öffnet die Pipe mit `SECURITY_IDENTIFICATION`.

`DpapiStore` schützt ID, idKey und Datenbank-Hash mit DPAPI **CurrentUser**, ohne
Machine-Flag, ohne Dialog und ohne Klartext-Ersatzpfad. Standard:
`%APPDATA%\Kairos\keepassxc-association.dpapi`. Ein DPAPI-Blob wird vollständig
geschrieben, synchronisiert und atomar mit `persist_noclobber` veröffentlicht;
vorhandene Dateien werden nicht überschrieben. Ein Store enthält eine Zuordnung.
Für mehrere bewusst gekoppelte Datenbanken sind getrennte explizite Pfade nötig.
Verzeichnisse erben Eltern-ACLs. Verlust/Korruption benötigt bewusste Neukopplung
und gegebenenfalls das manuelle Widerrufen einer verwaisten KeePassXC-Zuordnung.
Auch nach Widerruf einer Verbindung wird eine vorhandene Datei nicht ersetzt.
Derzeit gibt es bewusst keine `forget`-/`replace`-API: Für eine Neukopplung einen
neuen expliziten Store-Pfad verwenden oder die bisherige DPAPI-Datei selbst als
Backup umbenennen und danach einen neuen Client koppeln. Das ist eine manuelle
Wiederherstellungsentscheidung; diese Crate löscht keine Zuordnung. Ein geführter
Widerrufs-/Neukopplungsablauf bleibt beim App-Folge-PR.

Der dauerhaft gespeicherte `idKey` ist laut Protokoll die **öffentliche** Hälfte
eines separaten Identifikationspaars, wirkt aber als Bearer-Geheimnis; dessen
private Hälfte wird nicht gebraucht. Eigene Schlüssel-, ID-, Klartext-JSON- und
DPAPI-Puffer liegen in `Zeroizing`; Secret-Typen haben kein `Debug`/`Clone`.
Serialisierung nutzt feste Puffer, um alte Allokationen durch Vec-Wachstum zu
vermeiden. DPAPI-Ausgaben werden vor `LocalFree` überschrieben. Der Adapter löscht
zusätzlich die vollständige `crypto_box`-SecretKey-Allokation; die Abhängigkeit
selbst löscht in 0.9.1 nur den Skalar. Salsa20s Zeroize-Feature ist aktiviert.
Historische Compiler-/Stack-/OS-Kopien, fremde Bibliotheken, Dumps und Swap sind
damit nicht vollständig beherrscht. Fehler enthalten ausschließlich eigene Texte
und gegebenenfalls einen numerischen Servercode; keine Passwort-Logs.

Außerhalb Windows sind Protokoll, Trait und In-Memory-Tests nutzbar.
`DpapiStore` meldet `UnsupportedPlatform`; der native Pipe-Adapter ist `cfg(windows)`.

## URL-Ermittlung für den Folge-PR

Empfohlen: eine kleine Erweiterung liest nach ihrer eigenen Nutzeraktion oder
ihrem Shortcut `tab.url` über `activeTab` und sendet nur diese URL über einen
Native-Messaging-Host mit Extension-Allowlist. Ein globaler Desktop-Hotkey allein
gewährt `activeTab` nicht. Anfrage, Tab/Fenster und Navigationsstand müssen an
dieselbe kurzlebige Aktion gebunden bleiben.

UI Automation kann vorübergehend eine Quelle für bereits fokussierte Browser
sein; Adressleisten können bearbeiteten Text enthalten, Provider unterscheiden
sich nach Chrome-/Edge-/Firefox-Version und Accessibility kann fehlen. Kein
Fenster aktivieren, kein Ctrl+L/Kopieren, kein Titel-Fallback und kein Schema
ergänzen. Bei fehlendem oder mehrdeutigem Ergebnis abbrechen. Für Violentmonkey
zunächst nur ein explizit bestätigter Prototyp: Userscripts besitzen keine
vertrauenswürdige globale Tabs-API und keinen eigenen Native-Messaging-Kanal.
Ein Loopback-Prototyp braucht Pairing, Origin-/Replay-Prüfungen, strikte lokale
Bindung und kurze Transaktionen; Seiten-Nachrichten sind keine Tab-Identität.
Details und Abwägung: [keepassxc.md](../../docs/sources/keepassxc.md).

## Prüfung

Rust mindestens 1.89; im betreuten Windows-Build zuerst:

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Tests: libsodium-NaCl-Testdaten (keine Implementierung übernommen), separat
formulierte Protokoll-JSON-Fixtures, Nonce-Replay/Manipulation, vollständiger
Mock-Rundlauf mit In-Memory-Store und Sitzungswechsel, DB-Wechsel, redigierte
Fehler, kein Retry, DPAPI-/Korruptions-/Überschreibschutz und echte private
Dummy-Pipes mit Fragmenten/Broadcasts/Timeout. Diese Pipes berühren KeePassXC nicht.
Ein Regressionstest prüft 40 während einer Leerlaufphase angesammelte Lock-/Unlock-
Broadcasts vor einer weiterhin gültigen Antwort.

Ein echter KeePassXC-Test ist absichtlich `#[ignore]`, nur lesend: Handshake,
Hash und optional Prüfung einer **bereits vorhandenen Test-Assoziation**. Er
koppelt nicht und ruft nie `set-login` auf. Nur mit einer wegwerfbaren Testdatenbank
und bereits manuell aktivierter Browserintegration ausführen. Die Implementierung
konfiguriert weder KeePassXC noch eine Datenbank. Ohne entsprechende isolierte
Testumgebung den Test überspringen.

```powershell
# Optional: Pfad zu einer bereits vorhandenen DPAPI-Test-Assoziation, kein Secretwert.
# $env:KAIROS_KEEPASSXC_TEST_ASSOCIATION_FILE = '<isolierter Testpfad>'
cargo test -p kairos-keepassxc --test keepassxc_read_only -- --ignored --test-threads=1
```

Eine erfolgreiche reale Eintragsanlage muss später separat in einer ausdrücklich
freigegebenen Testdatenbank geprüft werden, einschließlich Gruppe, Abbruch der
Datenbankauswahl, gesperrter DB und mehrerer offener Datenbanken.

## Open-Source-Vorbilder (modellieren, nicht kopieren)

KeePassXC 2.7.12 und KeePassXC-Browser liefern Protokoll, Pairing und Schlüsselrollen;
`git-credential-keepassxc` liefert den Vergleich eines unabhängigen Rust-Clients.
Der ältere Rust-Proxy wurde wegen veraltetem Windows-Pfad und unvollständigem
Windows-Code verworfen. 2FAS und Bitwarden begründen bewusste Paarung,
zielgebundene Freigaben und Extension-Allowlisting. UIA-v2 und WebAutoType zeigen
die Grenzen der Browser-Ermittlung über Accessibility.

Unveränderte kompatible Crates liefern Krypto (`crypto_box`, `curve25519-dalek`,
`salsa20`), Systemzufall (`getrandom`), JSON (`serde`/`serde_json`), Base64,
URI-Parsing (`fluent-uri`), Pufferlöschung, Overlapped-I/O (`tokio`), Windows-
Bindings und atomare Dateiveröffentlichung (`tempfile`). Sie sind Abhängigkeiten,
keine kopierten Quellen. GPL-/BUSL-Vorbilder werden ausschließlich in ihren
Prinzipien bewertet. Permalinks, Dateilizenzen und Passungswerte stehen vollständig
in [keepassxc.md](../../docs/sources/keepassxc.md). Es wurde kein fremder
Implementierungscode kopiert.
