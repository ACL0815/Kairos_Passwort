# Englisch, Latein und die kombinierte Quelle

**Bereit zur unabhängigen Prüfung (präzisierte Rubrik).** `reptile` ist
nach Lead-Entscheid B. Die eigene zweite Probe bleibt unvollständig.
[Lead-Entscheid und Übergabe](../curation-stage-two.md).

Aktueller Standard ist die [verbindliche A/B-Rubrik](../curation-rubric.md).
Der historische STOP aus Stufe 1 ist in [curation-diagnosis.md](../curation-diagnosis.md)
erhalten. Aktuelle Abnahme und Belege stehen in [Stufe 2](../curation-stage-two.md).
PR #2 ist gemergt; #7 basiert jetzt auf main und enthält die deutsche A-Triage.

| Sprache | Wörter | Mittelwert | SHA-256 |
|---|---:|---:|---|
| DE | 6.909 | 6,577 | `02911732a22dbba0967453121e60d245bc7452e13383d808acfbbef47efb2ed0` |
| EN | 6.897 | 6,659 | `8aae2952cab7bb2406864e7f87b685f8f5c3ced1d7c7b4f04d4a666f9253899a` |
| LA | 6.897 | 6,988 | `7ccd0262310397edb42e1c65e33d5a781a4426fd84ad14d7edf8bedee28fd8c3` |

Die Vereinigung hat **20.703** verschiedene Wörter mit korrekten Flags.
Die API verwendet `&'static EmbeddedWordList`, `include_str!` und `OnceLock`.
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
Nur sexuelle, vulgäre/obszöne und Slur-Nebensinne erzwingen A; Hauptsinne
bleiben streng nach A-Klasse ausgeschlossen. `reptile`, `pouncing`, `bladder`,
`absurde` und `situs` bleiben erlaubt;
`servula` ist als Sklavin gesperrt. Vorherige strengere Sperren bleiben erhalten.

EN-Fingerprints binden jetzt den vollständigen WordNet-Quelltext und die explizite
EFF-Large-Mitgliedschaft. Der ganze Pool besitzt 8.924 WordNet-Treffer und 634
ausdrückliche NO_ENTRY-Zeilen; 7.772 Kandidaten stehen in EFF Large. In der
Endliste haben 6493 Wörter WordNet-Belege und 404
keinen WordNet-Eintrag; 5573 tragen das EFF-Large-Signal.
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
Wordnik-Rohbestand ist kein ungeprüfter Nachrückerpool. 1.226 ältere redaktionelle
Sperren bleiben wirksam. Nach Kuration und deutschem Sprachabzug gilt kürzer
zuerst, dann ordinal.

LA verwendet Whitakers WORDS bei Commit `1f2f0fb0867a896d7b9284a03d615ed635d6f992`
mit 39.335 Datensätzen. Der eigene Decoder unterstützt bestimmte Nominative,
regelmäßige/deponentische Präsensformen und unveränderliche Wörter. Fehlende
zzz-Stämme, reine Namen und nicht unterstützte Muster werden nicht übernommen.
2.352 ältere redaktionelle Sperren gelten zusätzlich. Die Reihenfolge ist Frequenz
A vor B vor C vor D, dann Länge und ordinale Schreibweise. Identische vollständige
erste Glossen werden auf die zuerst gerankte Form reduziert; dies kann auch
Synonyme ausdünnen und ist keine vollständige linguistische Variantenanalyse.

Deutsch hat Vorrang vor Englisch, Englisch vor Latein. Endlisten sind disjunkt;
das Flag bezeichnet den zugeordneten Wortgebrauch, nicht historische Etymologie.
EN/LA werden höchstens auf den DE-Umfang und sonst auf das verfügbare Minimum
gekürzt. Der vorsichtige LA-Abzug gegen eine größere EN-Vorauswahl wird bei
einer eventuellen späteren Kürzung nicht rückgängig gemacht.

Vier unabhängig und gleichverteilt aus der Vereinigung gezogene Wörter ergeben bei festem Trenner **57,350 Bit**. Der Größenfaktor gegenüber DE ist **80,625**; der Sprachanteil ist bereits enthalten und darf nicht nochmals addiert werden.

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
Das Stichprobengate prüft aktuelle Listenhashes, Auswahl und sämtliche
Quelltexte; unbewertete oder veraltete Proben scheitern. B wird protokolliert.
Ein A-Fund erlaubt einen Ursachenfix und eine neue frische Probe; findet auch
diese A, gilt STOP. Explizite `Nebenbedeutung:`-Kategorien unterscheiden
Sexualitaet/Vulgaer/Slur (A) von Tier-Gegenstandsmetapher (B).
Das Gate beweist nur die ausgewählte Probe. Die historische zweite Probe
ist unvollständig; `pwsh -NoProfile -File crates/kairos-wordlists/tools/check-semantic-sample.ps1`
verweigert deshalb eine Freigabe. Eine neue semantische Runde führt der
unabhängige Verifier durch.

Alle Wörter haben 3–9 ASCII-Kleinbuchstaben. Sechs Buchstaben sind ein Komfortziel;
Tests begrenzen EN auf 6,7 und LA auf 7,0 im Mittel. Die frühere EN-Grenze 6,6
wurde nach zusätzlichen kurzen Ausschlüssen offengelegt angepasst.
Allgemeine Präfixfreiheit
und Mindest-Editierabstand werden nicht versprochen. Feste Trenner halten Grenzen
eindeutig. [Filterbericht](report.txt), [Provenienz](PROVENANCE.md),
[deutsche Triage](stage2-german-triage.tsv), [vollständige Wortdeltas](stage2-statistics.json).

Die aktualisierten Klassifikationseingaben lassen sich mit `tools/prepare-review.ps1`
aus dem Crate-Verzeichnis rekonstruieren. Die historischen Eingabehashes der
Stufe 1 gehören zum damaligen Skript bei `691860b`.
`NOTICE.txt` bündelt die vier vollständigen Datenlizenzen. Die spätere
Release-Integration muss die Angaben mit der EXE weitergeben; die Verdrahtung
liegt außerhalb dieses Auftrags.
