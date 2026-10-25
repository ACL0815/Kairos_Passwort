# Architektur und API-Vertrag

## Abhängigkeitsrichtung

`kairos-wordlists` → `kairos-core` und `kairos-store` → `kairos-core`.
Core hat keine Abhängigkeit zu seinen Wortquellen. `kairos-platform` und
`kairos-keepassxc` bleiben unabhängig von Core und voneinander.
`apps/kairos` verdrahtet derzeit Core, deutsche Wortquelle und Plattform.
Die ungenutzten Scaffold-Abhängigkeiten zu Store und KeePassXC sind entfernt;
beide bleiben eigene Workspace-Mitglieder. Der Store ist ohne UI nutzbar, die
KeePassXC-Crate enthält noch keine Protokollkommunikation. Plattformaufrufe
erhalten ihre Daten von der Anwendung, nicht durch einen eigenen Generator.

## Gemeinsamer Vertrag

Der maßgebliche Rust-Vertrag steht in
[`crates/kairos-core/src/lib.rs`](../crates/kairos-core/src/lib.rs).
Während der parallelen PRs gilt: **Vertrag nur additiv ändern**. Bestehende Namen,
Signaturen, Feldtypen und Semantik bleiben kompatibel. Neue Pflichtfelder in
öffentlich konstruierbaren Strukturen wären ebenfalls eine brechende Änderung.
Erweiterungen müssen bestehende Verbraucher weiter kompilieren lassen.

| Element | Festlegung |
|---|---|
| `WordSource` | Objektsicher; `len() -> usize`, `word(index) -> &str`, `language(index) -> Language`; zusätzlich `is_empty()` mit Standardimplementierung |
| Wortquellen | Während einer Generierung stabil; nichtleere, eindeutige Wörter; gültige Indizes `0..len()` |
| `Language` | `#[non_exhaustive]`, `German`, `English`, `Latin`, `Other(String)`; Sprache je Wort |
| `ProfileId` | Privates `u8`, gültig nur 0–12; `new(u8) -> Result<_, InvalidProfileId>`, `TryFrom<u8>`, `get() -> u8`, Default 0 |
| `GeneratorConfig` | Öffentliche Felder `profile: ProfileId`, `excluded_chars: Vec<char>`; Default Profil 0, keine Ausschlüsse |
| Ausschlüsse | Betreffen Trenner; doppelte Einträge ändern die effektive Auswahl nicht; leerer effektiver Pool ergibt `InvalidConfig` |
| `SegmentKind` | `Word`, `Separator`, `Digit`, `Uppercase` |
| `Segment` | Öffentliche Felder `kind` und `range: Range<usize>`; UTF-8-Bytebereiche, gültige Zeichengrenzen, sortiert und überlappungsfrei |
| Hervorhebung | Großbuchstaben und eingefügte Ziffern teilen Wortsegmente; GUI darf Byteindizes nicht als Zeichenindizes interpretieren |
| `GeneratedPassword` | Öffentliche Felder `secret: zeroize::Zeroizing<String>`, `segments: Vec<Segment>`, `entropy_bits: f64`; kein `Debug`/`Clone` für Geheimnisse |
| `generate` | `(&dyn WordSource, &GeneratorConfig) -> Result<GeneratedPassword, GenerateError>` |
| `GenerateError` | Nicht erschöpfend: `NotImplemented` (reserviert), `EmptySource`, `InvalidConfig`, `InvalidSource`, `RandomSourceUnavailable`; Fehler enthalten kein Passwort |
| `entropy_bits` | `(source_len: usize, &GeneratorConfig) -> f64`; Sechs-Buchstaben-Modell, keine geeignete GUI-Stärkeanzeige |
| `entropy_bits_for` | `(&dyn WordSource, &GeneratorConfig) -> Result<f64, GenerateError>`; konservative Min-Entropie aus der tatsächlichen Quelle; auch Grundlage von `GeneratedPassword.entropy_bits` |
| `profile_rules`, `profile_warnings` | Datengetriebene Profile und separate Listenempfehlungen; eine kleinere Liste bleibt nutzbar und erhält keine fiktiven Auswahlbits |
| `CustomProfile`, `ProfileRotation` | Validierte eigene Regeln, versioniertes Serde-Format und 1–16 gleichgewichtete Rotationseinträge; additive API aus PR-8 |
| `generate_custom`, `generate_rotating` | Feste eigene Regeln oder frische Zufallswahl; `RotatedPassword` enthält Passwort und `rotation_bonus_bits`, keine gezogene Profil-ID |
| `entropy_bits_for_custom`, `entropy_bits_for_rotation` | Quellenabhängige Schranken; Rotation verwendet die schwächste Profilstärke und nur bei bewiesener Disjunktheit/gleicher Stärke/unbekannter Wahl einen Bonus |

Der Generator ist implementiert. Die GUI zeigt ausschließlich die Metadaten
des aktuellen Core-Ergebnisses. Die quellenabhängige Berechnung prüft unter
anderem eindeutige alphabetische Wörter, umkehrbare Großschreibpositionen und
innere Ziffernpositionen. Die genaue konservative Schranke steht im
[Core-README](../crates/kairos-core/README.md). Nach manueller Nacharbeit
entfallen Entropie und Rotationsbonus; es gibt keinen Passwortstärke-Schätzer
für menschliche Änderungen.

## Komponenten im integrierten Stand

| Komponente | Implementierung / Grenze |
|---|---|
| `kairos-wordlists::german()` | Einmal indexierte, eingebettete Quelle mit 7.130 unterschiedlichen ASCII-Wörtern, Sprache je Wort; Daten CC0-1.0. Kein Download zur Laufzeit. `combined()` aus dem offenen GitHub-PR #7 ist hier nicht verfügbar. |
| `kairos-store` | SQLCipher, CurrentUser-DPAPI, validierter Import, Sprach-/Listenfilter, stabile deduplizierte WordSource-Snapshots und Herkunftsmetadaten. Keine App-Importoberfläche in PR-W. |
| `kairos-platform` | Unabhängige Windows-Dienste. `GlobalHotkeys::with_wakeup`, `FocusTarget`, `copy_secret` und `ClipboardReceipt` verbinden OS und App. Andere Betriebssysteme liefern explizite Fehler. |
| `kairos-keepassxc` | Noch Scaffold. F11 zeigt einen sichtbaren PR-6-Hinweis und schreibt weder ins Feld noch in einen Tresor. |
| `apps/kairos` | eframe/egui 0.37, editierbares farbiges Passwort, JetBrains Mono NL, Profile/Ausschlüsse, optionale Rotation, geschützte Kopieraktionen und Hotkey-Editor. Keine Passwortpersistenz. |

## App-Ablauf und Plattformgrenzen

Der Einstieg erfasst das Vordergrundziel vor `run_native` und reicht
`german_source()` als `Box<dyn WordSource>` hinein. Der Adapter leiht die
eingebettete Liste ohne Wortkopien. Derselbe Konstruktor akzeptiert später
eine kombinierte Quelle oder einen eigenen Store-Snapshot. Ein Quellenwechsel
braucht keine Änderung im Core; Store- und Sprachauswahl-UI gehören zu Folge-PRs.
Die App modelliert das Ziel als `Unavailable`, `ReturnOnly` oder `Editor`.
Ein Startziel ist ausschließlich `ReturnOnly`, auch mit `--popup`. Es darf
nur den Fokus zurückgeben, niemals eine Texteingabe freigeben. Erst F9 im
gewünschten Zielfeld erzeugt `Editor`. So kann ein normaler Start aus Terminal
oder Explorer kein Passwort dorthin tippen. Die Schaltflächen spiegeln diese
Berechtigung und das Vorhandensein eines Rücksprungziels wider.

Die App hält genau einen Hotkey-Dienst. Dessen Callback fordert nur ein Repaint
an; `App::logic` verarbeitet Ereignisse auch bei verborgenem Fenster. Das Ziel
wird bereits im Hotkey-Thread erfasst. F9 merkt es, generiert und zeigt den
kompakten Root-Editor. F10 generiert, tippt über das erfasste Ziel und kopiert
erst nach angenommener Eingabe, ohne einen Fokusbefehl an Kairos zu senden.
Fehler erscheinen beim Öffnen im App-Hinweis, Eingabe-/Kopierfehler zusätzlich
als Titel-/Taskleistenhinweis. Registrierungsfehler bleiben sichtbar und sind
mit einem erneuten Registrierungsversuch behebbar. F11 öffnet den sichtbaren
KeePassXC-Hinweis und bewahrt das aktuelle Passwort samt Nacharbeit, gibt aber
kein neues Einfügeziel frei. Eigene Prozessziele werden in der App abgewiesen.
Nach einem verarbeiteten Hotkey fordert die App ein weiteres Repaint an, damit
ein während der Verarbeitung eingetroffenes Ereignis nicht liegen bleibt.

`FocusTarget::restore()` aktiviert vor dem Minimieren das gemerkte Fenster.
Bei Fehler bleibt die App offen. Einfügen aus dem Editor nutzt additiv
`restore_and_type_text()`: Ein ausdrücklicher neuer Klick bestätigt die
ursprüngliche Fenster-/Prozess-/Thread-/Control-Identität und startet nach
geprüfter Rückgabe eine frische Eingabefrist. Es wird kein neues Ziel erfasst.
Der Editor verbraucht den Auftrag auch bei Fehler; kein doppeltes Senden und
kein automatischer Clipboard-Fallback. Das Passwort bleibt zur manuellen Kopie
erhalten; Hinweise erklären, dass ein neues F9 auch neu generiert. Direkte Hotkey-Eingaben behalten die
Fünf-Sekunden-Frist. Integritäts-, Modifier- und Unicode-Prüfungen gelten für
beide Pfade. `is_current_process()` liefert ausschließlich Zielmetadaten.

Windows kann Fokusaktivierung verweigern. `SendInput` bindet die Zustellung
nicht atomar an ein Fenster, native HWNDs unterscheiden keine Browser-DOM-Felder.
Teileingaben werden weder gelöscht noch erneut gesendet. Angenommene Ereignisse
sind keine bestätigte Zustellung. Diese Grenzen sind auch im Editor erklärt.

`ClipboardAccess` ist die mockbare App-Grenze. Schreiben verwendet ausschließlich
`copy_secret(secret, Some(DEFAULT_CLEAR_AFTER))`. Alle drei Schutzformate und
der Text entstehen in derselben OS-Transaktion. Ein Receipt enthält nur
Metadaten; der Plattform-Timer läuft unabhängig von der GUI. Beim normalen
App-Ende wird nur die letzte noch eigene Kopie zu leeren versucht. Strg+V
liest über clipboard-win, nicht über den normalen eframe-Export. Der frühere
arboard-Schreibadapter und seine Übergangstests sind ersetzt; die nativen
Plattformtests bleiben maßgeblich.

Rotation ist standardmäßig aus. Der Haken zieht aus Profil 0–12 neu,
Wiederholungen sind erlaubt. Diese Profile überlappen und sind unterschiedlich
stark, deshalb beträgt `rotation_bonus_bits` hier 0. Die Anzeige nennt die
schwächste Profilstärke, keine durchschnittliche oder gezogene Profilstärke.
Das ist besonders beim Wechsel von Profil 12 auf Rotation zu beachten.

## Ausbauregeln

Profile 0–12 sind datengetrieben nach der Konzepttabelle umgesetzt.
Zahlen aus dem Konzept sind Modellwerte; die Anzeige verwendet die reale
Listengröße und die tatsächlich erlaubten Zufallsentscheidungen. Für eine
Sprachmischung sind Überschneidungen vor der Auswahl zu entfernen.
Ein Profilbonus bis 4 Bit darf nur unter den im Konzept genannten und im
Core geprüften Voraussetzungen angezeigt werden. Die feste App-Rotation
über alle eingebauten Profile erfüllt diese nicht.

CSPRNG und gleichverteilte Auswahl kommen über `rand` mit `OsRng`/`getrandom`.
Windows-Systemzufall und die frühere zweite Zufallsphase fallen damit zusammen.
Fehler führen zum Abbruch; es gibt keinen schwächeren Ersatzgenerator.

Geheimnisse dürfen nicht in Logs, Debug-Ausgaben oder einem ungefragten Verlauf
landen. Die GUI muss zusätzliche Kopien, Bearbeitungspuffer und Undo-Daten
gesondert berücksichtigen. Die Windows-Zwischenablage erhält Verlauf-/Cloud-
Ausschluss und Timeout. Speicherhärtung und Dump-Tests folgen in PR-9a,
obfstr/Wortlistenschutz nach PR-7 in PR-9b. Bildschirmfreigabeschutz folgt
separat in PR-11; er ist hier noch nicht implementiert.

Automatisierte Zustands-/Core-/Plattformtests und die Desktop-Checkliste aus
[MANUAL-TEST.md](MANUAL-TEST.md) ergänzen sich. Ignorierte interaktive Tests
gelten ohne separaten Lauf nicht als Abnahmebeleg.

Bereichsquellen besitzen eindeutige Verantwortliche in [SOURCES.md](SOURCES.md).
Gemeinsame Integrationsdateien wie App-Manifest und `Cargo.lock` werden beim
Rebase abgestimmt; die Arbeitsaufteilung steht in [ROADMAP.md](ROADMAP.md).
