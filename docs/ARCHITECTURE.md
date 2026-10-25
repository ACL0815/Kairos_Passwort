# Architektur und API-Vertrag

## Abhängigkeitsrichtung

`kairos-wordlists` → `kairos-core` und `kairos-store` → `kairos-core`.
Core hat keine Abhängigkeit zu seinen Wortquellen. `kairos-platform` und
`kairos-keepassxc` bleiben unabhängig von Core und voneinander.
`apps/kairos` verdrahtet alle fünf Bibliotheken. Plattform-/Protokollaufrufe
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
| Ausschlüsse | Betreffen Trenner; doppelte Einträge ändern die effektive Auswahl nicht; verbleibende Auswahl und Entropie werden in PR-1 gemeinsam berechnet |
| `SegmentKind` | `Word`, `Separator`, `Digit`, `Uppercase` |
| `Segment` | Öffentliche Felder `kind` und `range: Range<usize>`; UTF-8-Bytebereiche, gültige Zeichengrenzen, sortiert und überlappungsfrei |
| Hervorhebung | Großbuchstaben und eingefügte Ziffern teilen Wortsegmente; GUI darf Byteindizes nicht als Zeichenindizes interpretieren |
| `GeneratedPassword` | Öffentliche Felder `secret: zeroize::Zeroizing<String>`, `segments: Vec<Segment>`, `entropy_bits: f64`; kein `Debug`/`Clone` für Geheimnisse |
| `generate` | `(&dyn WordSource, &GeneratorConfig) -> Result<GeneratedPassword, GenerateError>` |
| `GenerateError` | Nicht erschöpfend: `NotImplemented`, `EmptySource`, `InvalidConfig`, `RandomSourceUnavailable`; Fehler enthalten kein Passwort |
| `entropy_bits` | `(source_len: usize, &GeneratorConfig) -> f64`; Stub, bis PR-1 das Verhalten implementiert |

`generate` gibt im Scaffold ausdrücklich `NotImplemented` zurück.
`entropy_bits` enthält ein dokumentiertes `todo!()` und wird im Scaffold nicht
aufgerufen. Diese Stubs sind keine nutzbare Sicherheitsimplementierung.
PR-1 definiert und testet Randfälle der Entropie (leere Quellen, vollständig
ausgeschlossene Trenner) zusammen mit der Generatorvalidierung. Die einfache
Signatur bildet keine wortlängenabhängige exakte Entropie ab; Annahmen zu
inneren Positionen und Großschreibung müssen dort offengelegt werden.

## Ausbauregeln

Profile 0–12 werden in PR-1 datengetrieben nach der Konzepttabelle umgesetzt.
Zahlen aus dem Konzept sind Modellwerte; die Anzeige verwendet die reale
Listengröße und die tatsächlich erlaubten Zufallsentscheidungen. Für eine
Sprachmischung sind Überschneidungen vor der Auswahl zu entfernen.
Ein späterer Profilbonus von bis zu 4 Bit darf nur unter den im Konzept
genannten Voraussetzungen angezeigt werden.

CSPRNG und gleichverteilte Auswahl kommen über `rand` mit `OsRng`/`getrandom`.
Windows-Systemzufall und die frühere zweite Zufallsphase fallen damit zusammen.
Fehler führen zum Abbruch; es gibt keinen schwächeren Ersatzgenerator.

Geheimnisse dürfen nicht in Logs, Debug-Ausgaben oder einem ungefragten Verlauf
landen. Die GUI muss zusätzliche Kopien, Bearbeitungspuffer und Undo-Daten
gesondert berücksichtigen. Die Windows-Zwischenablage erhält Verlauf-/Cloud-
Ausschluss und Timeout. Speicherhärtung und Dump-Tests folgen in PR-9.

Bereichsquellen besitzen eindeutige Verantwortliche in [SOURCES.md](SOURCES.md).
Gemeinsame Integrationsdateien wie App-Manifest und `Cargo.lock` werden beim
Rebase abgestimmt; die Arbeitsaufteilung steht in [ROADMAP.md](ROADMAP.md).
