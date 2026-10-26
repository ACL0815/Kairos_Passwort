# Englisch, Latein und die kombinierte Quelle

**Endstufenentscheidung umgesetzt.** Der endgültige Standard verlangt eine
unabhängige 900er-Stichprobe (je 300 DE/EN/LA), A-Quote ≤ 0,5 % und keinen
A-Fall nach Hauptbedeutung. Die historische Verifier-Stichprobe zeigt rund
0,4 % Restrisiko. [Lead-Entscheid und Verifier-Matrix](../curation-stage-two.md#lead-entscheid-endstufe).

Aktueller Standard ist die [verbindliche A/B-Rubrik](../curation-rubric.md).
Der historische STOP aus Stufe 1 ist in [curation-diagnosis.md](../curation-diagnosis.md)
erhalten. Aktuelle Abnahme und Belege stehen in [Stufe 2](../curation-stage-two.md).
PR #2 ist gemergt; #7 basiert jetzt auf main und enthält die deutsche A-Triage.

| Sprache | Wörter | Mittelwert | SHA-256 |
|---|---:|---:|---|
| DE | 6.907 | 6,577 | `d2ce7e12303150ab6c8d17f968777b9e52709fdb712949852460b0dfef8c4480` |
| EN | 6.895 | 6,659 | `5918f9ec9fd68f6796106b447b40696527184ebc4e5935507caecb6008737655` |
| LA | 6.895 | 6,987 | `762f102d623c66d3a898e85375b781ca3947cb0019f16b5448b9e8c3e08bc309` |

Die Vereinigung hat **20.697** verschiedene Wörter mit korrekten Flags.
Die API verwendet `&'static EmbeddedWordList` und `OnceLock`. `build.rs` bettet
maskierte Bytes ein; vor dem Indexieren wird der entschleierte Klartext gegen
seinen eingebetteten SHA-256 geprüft. [Nutzen und Grenzen](../../README.md#verschleierung-und-grenzen-pr-9b-prio-5).
Nur die Wortlisten werden eingebettet; Glossen und Kuration sind Entwicklungsdaten.

## Semantik vor Auswahl

`semantic-decisions.tsv` hat genau 21.190 Kandidatenentscheidungen, 9.558 EN und
11.632 LA. Format ohne Kopfzeile: Wort, Sprache, allow/block, Klasse, Begründung,
Quell-SHA-256, getrennt durch TAB; ordinal nach Sprache und Wort sortiert.
`allow` verlangt `neutral`. Blockklassen sind `Krankheit`, `Verletzung`,
`Tod-Trauer`, `Gewalt-Waffen`, `Sexualitaet`, `Ausscheidung`, `Drogen-Alkohol`,
`Abwertung`, `Vulgaer`, `Slur`, `Sklaverei-Unterdrueckung`, `Verbrechen`,
`Herabsetzung-Extremismus` und historische beziehungsweise konservative `Zweifel`.
Mehrere Klassen werden mit Komma getrennt. Ein Block ist nicht automatisch
ein gesicherter A-Fund. [Stufe-2-Adjudikationen](stage2-decisions.tsv) halten
A/B und die tatsächliche allow/block-Aktion getrennt fest.

Nur A zählt als semantischer Fehler. B umfasst neutrale Anatomie, Tier-/Bewegungsverben,
milde Wertungen und Tier-/Gegenstandsmetaphern bei neutraler Hauptbedeutung.
Nebenbedeutungen zählen nur bei wortbezogener Wörterbuchmarkierung als
vulgär/obszön/diskriminierend oder überwiegendem entsprechendem Gebrauch als A.
Ein Grundverb in einer einzelnen sexuellen Wendung genügt nicht; Hauptsinne
bleiben streng nach A-Klasse ausgeschlossen. `nimmst`, `scio`, `squirrel`,
`klage`, `kappe`, `reptile`, `pouncing`, `bladder`,
`absurde` und `situs` bleiben erlaubt;
`servula` ist als Sklavin gesperrt. Vorherige strengere Sperren bleiben erhalten.

EN-Fingerprints binden jetzt den vollständigen WordNet-Quelltext und die explizite
EFF-Large-Mitgliedschaft. Der ganze Pool besitzt 8.924 WordNet-Treffer und 634
ausdrückliche NO_ENTRY-Zeilen; 7.772 Kandidaten stehen in EFF Large. In der
Endliste haben 6492 Wörter WordNet-Belege und 403
keinen WordNet-Eintrag; 5571 tragen das EFF-Large-Signal.
Lücken sind keine Wörterbuchfreigabe. EFF ist ein externes Kurationssignal,
keine automatische A/B-Entscheidung. [Extraktor, Originalarchiv und Grenzen](upstream/wordnet/README.md).

LA-Fingerprints enthalten alle decodierbaren Originaldatensätze derselben
Schreibweise mit Quellzeile, Frequenz und voller Glosse. Auch seltene Frequenzen
außerhalb A–D gehören dazu. Mindestens ein A–D-Datensatz ist zur Aufnahme nötig.
Fehlende, ungeordnete, doppelte oder ungültige Entscheidungen, neue Kandidaten
und geänderte Quellen brechen den Build ab. Die Keyword-Wächter bleiben ein
konservatives Zusatznetz; sie beweisen keine vollständige semantische Eignung.

## Quellen und Ranking

EN verwendet drei EFF-Listen von 2016 mit 10.368 Rohzeilen, 8.301 passenden
eindeutigen Wörtern und 1.257 ausdrücklichen Wordnik-Ergänzungen. Der übrige
Wordnik-Rohbestand ist kein ungeprüfter Nachrückerpool. 1.229 redaktionelle
Sperren bleiben wirksam. Nach Kuration und deutschem Sprachabzug gilt kürzer
zuerst, dann ordinal.

LA verwendet Whitakers WORDS bei Commit `1f2f0fb0867a896d7b9284a03d615ed635d6f992`
mit 39.335 Datensätzen. Der eigene Decoder unterstützt bestimmte Nominative,
regelmäßige/deponentische Präsensformen und unveränderliche Wörter. Fehlende
zzz-Stämme, reine Namen und nicht unterstützte Muster werden nicht übernommen.
2.354 redaktionelle Sperren gelten zusätzlich. Die Reihenfolge ist Frequenz
A vor B vor C vor D, dann Länge und ordinale Schreibweise. Identische vollständige
erste Glossen werden auf die zuerst gerankte Form reduziert; dies kann auch
Synonyme ausdünnen und ist keine vollständige linguistische Variantenanalyse.

Deutsch hat Vorrang vor Englisch, Englisch vor Latein. Endlisten sind disjunkt;
das Flag bezeichnet den zugeordneten Wortgebrauch, nicht historische Etymologie.
EN/LA werden höchstens auf den DE-Umfang und sonst auf das verfügbare Minimum
gekürzt. Der vorsichtige LA-Abzug gegen eine größere EN-Vorauswahl wird bei
einer eventuellen späteren Kürzung nicht rückgängig gemacht.

Vier unabhängig und gleichverteilt aus der Vereinigung gezogene Wörter ergeben bei festem Trenner **57,349 Bit**. Der Größenfaktor gegenüber DE ist **80,625**; der Sprachanteil ist bereits enthalten und darf nicht nochmals addiert werden.

## Prüfungen und Weitergabe

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p kairos-wordlists --example rebuild-german -- --check
cargo run -p kairos-wordlists --example rebuild-languages -- --check
node crates/kairos-wordlists/tools/prepare-english-senses.mjs --check
```

Die Pipeline prüft Quellen, Lizenzen, alle Entscheidungen, Ausgabehashes,
Filterbericht und LA-Glossen. Hashfreigaben werden bewusst separat gesetzt.
Nach einem Schreiblauf müssen geänderte Klartexte geprüft und die zugehörigen
`.sha256`-Freigaben von Hand aktualisiert werden. Bis dahin blockiert `build.rs`
auch weitere Pipeline-Beispiele, Tests und `--check`-Builds. Bei ungewollter
Beschädigung die freigegebenen Dateien wiederherstellen, statt neue Hashes blind
zu akzeptieren. [Build-Gate und Reproduktion](../../README.md#reproduktion-und-datenlizenzen).
Das Stichprobengate prüft aktuelle Listenhashes, Auswahl und sämtliche
Quelltexte; unbewertete oder veraltete Proben scheitern. B wird protokolliert.
Die historische Nulltoleranzprüfung erlaubte einen Ursachenfix und eine neue
Probe; bei erneutem A galt STOP. Explizite `Nebenbedeutung:`-Kategorien unterscheiden
Sexualitaet/Vulgaer/Slur (A) von Tier-Gegenstandsmetapher (B).
Das Gate beweist nur die ausgewählte Probe. Die historische zweite Probe
ist unvollständig; `pwsh -NoProfile -File crates/kairos-wordlists/tools/check-semantic-sample.ps1`
verweigert deshalb eine Freigabe. Dieses historische Gate implementiert nicht
den endgültigen 900er-Standard. Der Implementierungsworker startet keine
weitere semantische Runde. Eine Nutzer-Sperrliste für eigene Ausschlüsse
ist als Folge-Feature geplant.

Alle Wörter haben 3–9 ASCII-Kleinbuchstaben. Sechs Buchstaben sind ein Komfortziel;
Tests begrenzen EN auf 6,7 und LA auf 7,0 im Mittel. Die frühere EN-Grenze 6,6
wurde nach zusätzlichen kurzen Ausschlüssen offengelegt angepasst.
Allgemeine Präfixfreiheit
und Mindest-Editierabstand werden nicht versprochen. Feste Trenner halten Grenzen
eindeutig. [Filterbericht](report.txt), [Provenienz](PROVENANCE.md),
[deutsche Triage](stage2-german-triage.tsv), [historische Stufe-2-Deltas](stage2-statistics.json),
[Endstufendeltas und Größen](final-curation-statistics.json).

Die aktualisierten Klassifikationseingaben lassen sich mit `tools/prepare-review.ps1`
aus dem Crate-Verzeichnis rekonstruieren. Die historischen Eingabehashes der
Stufe 1 gehören zum damaligen Skript bei `691860b`.
`NOTICE.txt` bündelt die vier vollständigen Datenlizenzen. Die spätere
Release-Integration muss die Angaben mit der EXE weitergeben; die Verdrahtung
liegt außerhalb dieses Auftrags.
