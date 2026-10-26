# kairos-wordlists

**Endstufenentscheidung umgesetzt.** Sieben bestätigte Wörter sind zusätzlich
gesperrt. Der endgültige Standard verlangt eine unabhängige 900er-Stichprobe
mit A-Quote ≤ 0,5 % und ohne A-Fall nach Hauptbedeutung. Das dokumentierte
Restrisiko beträgt ungefähr 0,4 % nach der historischen Verifier-Stichprobe.
[Lead-Entscheid, Verifier-Matrix und Prüfgrenzen](data/curation-stage-two.md#lead-entscheid-endstufe).

`german()`, `english()`, `latin()` und `combined()` liefern gemeinsam
`&'static EmbeddedWordList` nach dem Vertrag von `kairos_core::WordSource`.
Die drei Listen sind paarweise disjunkt und enthalten zusammen **20.697**
Schreibweisen. Jedes Wort besteht aus 3–9 ASCII-Kleinbuchstaben und trägt sein
Sprachflag. Buildzeit-Maskierung und `OnceLock` entschleiern und indexieren jede
Sprache einmalig. Der Index leiht aus dem dauerhaft gehaltenen Klartextpuffer;
es entstehen keine einzelnen Wortkopien. Zur Laufzeit sind weder Dateien noch
Netzwerk nötig.

## Verschleierung und Grenzen (PR-9b, Prio 5)

**Verschleierung ist kein kryptografischer Schutz.** Maske/Schlüssel und Prüfsumme
liegen in der EXE. Die Wortliste gilt als öffentlich; die Sicherheit erzeugter
Passwörter hängt ausschließlich von der bekannten Wortmenge, den Profilregeln
und sicherer gleichverteilter Zufallsauswahl ab. Es kommen keine Entropiebits hinzu.
Ziel ist, vollständige Listen und Integritätskennungen nicht trivial per `strings`
zu extrahieren und Signatur-/Grep-basierte Änderungen zu erschweren.

`build.rs` prüft zuerst die freigegebenen Klartext-SHA-256-Dateien. Es schreibt
pro Sprache eine gleich lange OS-zufällige XOR-Maske und maskierte Bytes nach
`OUT_DIR`; nur diese und der binäre Klartext-Hash werden eingebettet. Ausfall des
Build-Zufalls oder ein nicht freigegebener Datenhash bricht den Build ab.
Der erste Zugriff erzeugt einen Heap-Puffer, prüft dessen SHA-256 und UTF-8 und
gibt ihn erst dann zum Indexieren frei. Abweichung löst eine klare Panic mit
Listenkennung aus. `OnceLock<String>` hält den Puffer für die Prozesslebensdauer.
`combined()` behält Reihenfolge, Flags und geliehene Wörter unverändert.

Die Maske verhindert keine Rückgewinnung durch Reverse Engineering. Nach dem
ersten Zugriff stehen die öffentlichen Wörter im Prozessspeicher. SHA-256 erkennt
geänderte Daten-/Maskenbytes bei unverändertem Prüfcode/Hash; ein Angreifer kann
auch Hash und Prüfroutine patchen. Das ist keine Authentisierung der EXE, kein
Ersatz für Codesignaturen und kein Schutz gegen Debugger oder privilegierte
Angreifer. `black_box` erschwert konstante Rückfaltung, garantiert dies aber nicht
für alle Compiler. Deshalb gehört eine Release-Artefaktprobe zur Prüfung.
Der öffentliche Digest bleibt buildübergreifend gleich und kann selbst als
Suchanker dienen, um Daten und Maske zu lokalisieren. Ein gezielter Austausch
von Daten und Digest ist auch ohne Änderung des Prüfcodes möglich.

Die Klartext-Pipeline und ihre `--check`-Ergebnisse bleiben reproduzierbar.
Byteidentische EXEs sind bewusst nicht zugesichert: Bei jedem erneuten Lauf des
Buildskripts entstehen neue Masken. Cargo darf unveränderte Buildausgaben cachen;
ein weiterer `cargo build` erzeugt daher nicht zwingend neue Masken.

`obfstr` **0.4.6 (MIT)** ist eine ausdrücklich gewünschte Crate-Abhängigkeit.
`obfstr!` schützt die drei internen Integritätskennungen, die erst zur Laufzeit
in Fehlermeldungen erscheinen. Die menschenlesbaren Fehler bleiben lesbar.
Pipe-/Bridge-IDs und Store-Manifestlabels liegen in anderen Crates; ihre Umstellung
bleibt eine dokumentierte Empfehlung. Es gibt keine solchen bestehenden Protokoll-
IDs in `kairos-wordlists` oder `kairos-hardening`, die hier zu ändern wären.

Modelled after: CasualX/obfstr für die Trennung zwischen gespeicherten Bytes und
Laufzeit-Klartext. Es wurde kein Implementierungscode übernommen. `obfbytes!`
ist für die zusammen 160.215 Bytes grundsätzlich möglich, legt aber vollständige
temporäre Arrays an und verlagert die Großdatenverarbeitung in die Const-Auswertung.
`obfstmt!` schützt Kontrollfluss, keine Wortdaten. Die gewählte Build-/Heap-Lösung
vermeidet diese zusätzlichen großen Stack-Temporärwerte und ist direkt prüfbar.
Vorbilder, Alternativen und Lizenzen stehen in
[protection.md](../../docs/sources/protection.md#wortlistenverschleierung-pr-9b).

### Release-Probe

`embedded-probe` ruft ausschließlich die produktive API auf und enthält keine
Klartext-Testfixtures. `tools/check-embedded.ps1` sucht außerhalb der EXE nach
20 zufällig ohne Wiederholung ausgewählten Wörtern mit mindestens sieben Zeichen
(DE 7 / EN 7 / LA 6), als ASCII-Teilfolge und UTF-16LE, sowie nach den drei
Integritätskennungen als ASCII. Die Längeneinschränkung vermeidet zufällige
Treffer kurzer Wörter in Standardbibliotheksmeldungen. Diese Stichprobe ist kein
Beweis, dass sämtliche Wörter oder andere Darstellungen fehlen.

```powershell
cargo build --release -p kairos-wordlists --example embedded-probe --target-dir F:\kairos-build\pr9b
# Vor Änderung der Bibliothek erzeugte EXE separat als embedded-probe-before.exe sichern.
pwsh -NoProfile -File crates/kairos-wordlists/tools/check-embedded.ps1 `
  -Before F:\kairos-build\pr9b\embedded-probe-before.exe `
  -After F:\kairos-build\pr9b\release\examples\embedded-probe.exe
```

Der konkrete Vorher-/Nachher-Beleg steht in [obfuscation-probe.json](obfuscation-probe.json).
`-SampleFile crates/kairos-wordlists/obfuscation-probe.json` verwendet dieselben
Wörter erneut, einschließlich Prüfung ihrer Zugehörigkeit zur freigegebenen Liste.
Die Probe ist derzeit manuell. Ein separater CI-Release-Job für diesen Scanner
bleibt Folgearbeit außerhalb der Schreibfreigabe von PR-9b; die normalen Unit-
Tests ersetzen keine Artefaktprüfung bei Compiler-/Linkeränderungen.

| Sprache | Wörter | Mittelwert | SHA-256 |
|---|---:|---:|---|
| DE | 6.907 | 6,577 | `d2ce7e12303150ab6c8d17f968777b9e52709fdb712949852460b0dfef8c4480` |
| EN | 6.895 | 6,659 | `5918f9ec9fd68f6796106b447b40696527184ebc4e5935507caecb6008737655` |
| LA | 6.895 | 6,987 | `762f102d623c66d3a898e85375b781ca3947cb0019f16b5448b9e8c3e08bc309` |

Der aktuelle Standard ist die [A/B-Rubrik](data/curation-rubric.md).
Entscheidungen, Proben, Review und Prüfstand stehen in
[Stufe 2](data/curation-stage-two.md). Der STOP in
[Stufe 1](data/curation-diagnosis.md) beschreibt den historischen Head `691860b`.

```rust
use kairos_core::WordSource;
let words = kairos_wordlists::german();
assert_eq!(words.len(), 6_907);
assert_eq!(words.word(0), "aal");
```

## Deutsche Auswahl

Basis sind die beiden fest versionierten dys2p-Listen unter CC0. Von 9.824
Rohzeilen erfüllen 7.853 die Zeichen-/Längenregeln; daraus entstehen 6.867
eindeutige Kandidaten. Die aktuelle Sperrliste hat 1.568 Einträge, davon greifen
984 in der Basis. Zu 5.883 Basiswörtern kommen 1.906 ausdrückliche enz-Ergänzungen.
Der Endungsfilter entfernt 882 Varianten. Im Ergebnis stammen 5.281 Wörter aus
der Basis und 1.626 aus den Ergänzungen.

Die fünf redaktionellen Kategorien bleiben `vulgär` (2), `beleidigend` (18),
`belastend` (968), `Eigenname` (11) und `Schreibbarkeit` (569).
Der A/B-Standard ist davon getrennt. Die zusätzliche deutsche A-Triage nutzt
die bestehende Pipeline-Sperrliste; konservative Grenzfälle sind im Bericht
kenntlich. In Stufe 2 entfielen 224 alte Endwörter. Die nach der Sperrung sichtbar
gewordenen Formen wurden ebenfalls geprüft und bei Bedarf gesperrt;
`bares`, `burgen` und `coupe` kommen als neutrale B-Wörter hinzu.
[299er-Triage](data/languages/stage2-german-triage.tsv),
[Nachrücker](data/languages/stage2-german-followup.tsv).
Die Endstufenentscheidung entfernt zusätzlich `farbig` und `pritsche`;
weitere deutsche Wörter wurden nicht aufgenommen.

Die Ergänzungen sind höchstens sieben Buchstaben lang und müssen in der
vendorten enz-Quelle enthalten sein. ASCII-Großbuchstaben werden kleingeschrieben;
Umlaute, ß, Akzente, Zahlen und Satzzeichen werden ausgeschlossen. Es gibt keine
Transliteration. Der Endungsfilter behält die zuerst gerankte kürzere Form für
`e/n/s/en/er/es/em/r`. Dies ist eine Schreibhilfe, keine vollständige Lemmatisierung.
Namenhomographen mit gewöhnlicher Wortbedeutung sind zulässig.

## Englisch, Latein und Wortwahl

Englisch nutzt EFF und ausdrücklich ausgewählte Wordnik-Ergänzungen. Princeton
WordNet 3.0 liefert zusätzlich fest gebundene Bedeutungen; Lücken und das
EFF-Large-Positivsignal bleiben sichtbar. Latein nutzt Whitaker mit einem
begrenzten eigenen Decoder und sämtlichen verfügbaren Glossen jeder
Kandidatenschreibweise. Alle 21.190 EN/LA-Kandidaten einschließlich Reserven
benötigen gültige quellgebundene Entscheidungen vor der Auswahl.
[Auswahl, Grenzen und Offline-Pipeline](data/languages/README.md).

Vier unabhängig und gleichverteilt aus der Vereinigung gezogene Wörter ergeben bei festem Trenner **57,349 Bit**. Der Größenfaktor gegenüber DE ist **80,625**; der Sprachanteil ist bereits enthalten und darf nicht nochmals addiert werden.

Sechs Buchstaben bleiben ein Komfortziel. Die aktuellen Mittelwerte stehen
oben. Allgemeine Präfixfreiheit, eindeutige Kurzpräfixe, Mindest-Editierabstand
und automatische Tippfehlerkorrektur werden nicht zugesichert. Feste Trenner
halten Wortgrenzen eindeutig. Zufallsauswahl und Profilregeln gehören zu Core.

## Reproduktion und Datenlizenzen

```powershell
cargo run -p kairos-wordlists --example rebuild-german -- --check
cargo run -p kairos-wordlists --example rebuild-languages -- --check
node crates/kairos-wordlists/tools/prepare-english-senses.mjs --check
```

Ein `rebuild-*`-Schreiblauf ohne `--check` kann die `.txt`-Dateien verändern.
Danach müssen die neuen Klartexte zuerst geprüft und ihre `.sha256`-Freigaben
bewusst von Hand gesetzt werden. Bis dahin verweigert das Build-Gate jeden Build
dieser Crate, einschließlich weiterer Beispiele, Tests und `rebuild-* -- --check`.
Die Hashes nicht nur zur Beseitigung des Fehlers ungeprüft aktualisieren.
Bei beschädigten Daten zunächst die freigegebene Fassung wiederherstellen.

`check-semantic-sample.ps1` gehört zur historischen Nulltoleranzprüfung der
unvollständigen EN/LA-Probe; es implementiert nicht den endgültigen Standard.
Eine Nutzer-Sperrliste für eigene Ausschlüsse ist als Folge-Feature geplant.

Quellen, Original-Lizenzen und Hashes stehen in
[DE-Provenienz](data/PROVENANCE.md) und [EN/LA-Provenienz](data/languages/PROVENANCE.md).
Die Softwarelizenz MIT OR Apache-2.0 ersetzt keine Datenlizenz.
[NOTICE.txt](data/languages/NOTICE.txt) enthält EFF CC BY 4.0, Wordnik MIT,
Whitakers individuelle Freigabe und die vollständige Princeton-WordNet-Lizenz.
Die Release-Integration muss diese Angaben mit der Anwendung ausliefern.
Die aktuelle Prüfung dieser PR ändert ausschließlich den freigegebenen Bereich.
