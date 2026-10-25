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
Sprachflag. `include_str!` und `OnceLock` erzeugen einmalig einen Index aus
geliehenen Zeichenketten. Zur Laufzeit sind weder Dateien noch Netzwerk nötig.

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
