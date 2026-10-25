Historischer Verifier-Bericht, inhaltlich unverändert übernommen als Beleg der Endstufenentscheidung; Leerzeichen am Zeilenende entfernt.

Original: `t3-0f4ffdad/target/verify/verify-pr7-final.md`; SHA-256 der Originaldatei: `7b41168887df30fbe3c311ad2db964926839ce304578861aeee73750a6f2289d`.

Die darin bewertete Nulltoleranz-Rubrik ist historisch. Der endgültige Standard steht in [curation-rubric.md](../curation-rubric.md#endgültiger-lead-entscheid-und-abnahme). Die relativen Rohloglinks im Originalbericht beziehen sich auf dessen ursprüngliches Verzeichnis. Es werden keine neuen Rohdaten oder Filterdaten importiert.

---

# PR #7 – unabhängige Abschlussverifikation

**Ergebnis: FAIL.** Der verbindliche Grenzwert von 0 A-Fällen wird in allen drei frischen 300er-Stichproben verfehlt. Alle 900 Zeilen sind bewertet; keine neue Ziehung und keine Ursachenbehebung durch diesen Verifier.

Stand: Sonntag, 25. Oktober 2026 um 21:31:52 MEZ. Repository https://github.com/ACL0815/Kairos_Passwort, PR #7 feat/languages-en-la → main.

Geprüfter Head: `86962555fc22d7b9a2df50a520b7133ade3ddf51`. Aktuelles main: `322c3ca4b9fd1ec45d0f6c47fba45acef6a2736a` (enthält #8). Gemeinsame Basis für den PR-Diff: `fa35aded13b50a67a817db79258b1ba1c95a9d46`.

Patch-SHA-256 (exakte Bytes aus git diff --binary --full-index): `939c683bfffb7e3785b9ed48ad4ff9510be1149ce3d341b5fdc86dbe9bfc75ef`. Stable patch-id: `d7b8223e1ba7be571c01e56aa08693be3fca6c70`. Diffstat: **84 Dateien, +73.265 / −574**.

## Matrix

Kriterium | Befehl | Exitcode | Ergebnis + Belegpfad
--- | --- | --- | ---
| Identität / Fetch / Detached Head | `git fetch origin; git rev-parse HEAD; git symbolic-ref -q HEAD` | 0 / 0 / 1 (detached) | PASS: 86962555fc22d7b9a2df50a520b7133ade3ddf51; [Head](final-head.log), [Fetch](refresh-fetch.log). Erwarteter Head und PR-headRefOid stimmen überein. |
| Aktuelles main und PR-Basis | `git rev-parse origin/main; gh api repos/ACL0815/Kairos_Passwort/git/ref/heads/main; git merge-base origin/main HEAD` | 0 / 0 / 0 | PASS: aktuelles main 322c3ca4b9fd1ec45d0f6c47fba45acef6a2736a enthält #8. Gemeinsame Basis fa35aded13b50a67a817db79258b1ba1c95a9d46; [main](main-tip.log), [GitHub-Ref](remote-main-api.log), [Basis](merge-base.log). gh pr view meldete baseRefOid=fa35aded13b50a67a817db79258b1ba1c95a9d46; das wurde nicht als aktueller main-SHA übernommen. |
| Diffstat / Patchhash | `git diff --stat fa35aded13b50a67a817db79258b1ba1c95a9d46 HEAD; git diff --binary --full-index fa35aded13b50a67a817db79258b1ba1c95a9d46 HEAD; git patch-id --stable` | 0 / 0 / 0 | PASS: 84 Dateien, +73.265 / −574. SHA-256 939c683bfffb7e3785b9ed48ad4ff9510be1149ce3d341b5fdc86dbe9bfc75ef; [Diffstat](diffstat.log), [vollständiger Patch](patch.log), [stable patch-id](patch-id.log). |
| Erlaubte Schreibpfade des PR | `git diff --name-status fa35aded13b50a67a817db79258b1ba1c95a9d46 HEAD; node target/verify/prepare.mjs` | 0 | PASS: alle 84 Pfade innerhalb crates/kairos-wordlists/** bzw. docs/sources/languages.md und docs/sources/wordlists.md; [Pfadliste](changed-paths.log), [Auswertung](identity.json). Cargo.lock bleibt unverändert. |
| Deutsche Nettolöschungen / Register | `git show fa35aded13b50a67a817db79258b1ba1c95a9d46:crates/kairos-wordlists/data/german.txt; node target/verify/prepare.mjs` | 0 | PASS: 7130 → 6909; 224 entfernt, 3 hinzugefügt, netto -221. Entfernungs- und Hinzufügungsregister stimmen als Mengen exakt. [Auswertung](german-delta.json), [tatsächlich entfernt](german-actual-removed.txt), [hinzugefügt](german-actual-added.txt). Das Register behauptet nicht, dass jede konservative Entfernung ein gesicherter A-Fall sei. |
| fmt | `cargo fmt --check` | 0 | PASS: [vollständiges Rohlog](fmt.log). |
| clippy | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | PASS: [vollständiges Rohlog](clippy.log). |
| test | `cargo test --workspace` | 0 | PASS: [vollständiges Rohlog](test.log). |
| rebuild-german | `cargo run -p kairos-wordlists --example rebuild-german -- --check` | 0 | PASS: [vollständiges Rohlog](rebuild-german.log). |
| rebuild-languages | `cargo run -p kairos-wordlists --example rebuild-languages -- --check` | 0 | PASS: [vollständiges Rohlog](rebuild-languages.log). |
| GitHub Checks | `gh pr checks 7` | 0 | [Snapshot](gh-checks.log). Beide windows-Checks bestanden (11m0s und 11m37s). Es wurde kein Watcher und kein Polling eingerichtet. |
| Mergebarkeit gegen aktuelles main | `gh pr view 7 --json mergeable,mergeStateStatus; git merge-tree --write-tree origin/main HEAD` | 0 / 0 | Lokaler Merge PASS (Exit 0, keine Konflikte): [Merge-tree](merge-tree.log). [GitHub-Zustand](gh-mergeable.log). GitHub meldet MERGEABLE/CLEAN; beide Checks sind grün. Kein Checkout/Merge-Commit des Ergebnisses erstellt. |
| Größen / a–z / Länge / Disjunktheit | `node target/verify/strict-data.mjs` | 0 | PASS: DE 6909, EN 6897, LA 6897; gesamt 20703. Jeweils eindeutig, ausschließlich a–z und Länge 3–9; alle drei Schnittmengen leer. [Berechnung](invariants.json), [strenge Byte-/Hashprüfung](strict-invariants.json), [Prüfer](strict-data.mjs). |
| combined() mit tatsächlichen Sprachflags | `pwsh -NoProfile -File target/verify/run-probe.ps1` | 0 / 0 | PASS: eigenständiger Rust-Prozess gegen die frisch gebauten echten Crates; alle 20.703 Einträge samt Flags mit Originaltextdateien verglichen. [Compiler](probe-build.log), [Lauf](probe.log), [Aufruf und rlibs](probe-result.json), [Prüfquelle](probe.rs). |
| WordNet-Auszug reproduzierbar | `node crates/kairos-wordlists/tools/prepare-english-senses.mjs --check` | 0 | PASS: [Rohlog](wordnet-extract.log). Die unabhängige Stichprobe nutzt zusätzlich Original-Tar-Daten und sense-Reihenfolge aus index.*, nicht die Autorenentscheidung. |
| Lizenztexte / NOTICE vollständig | `node target/verify/additional.mjs + manuelle Primärquellenlektüre` | 0 | PASS: EFF CC BY 4.0, Wordnik MIT, Whitaker-Freigabe und WordNet 3.0 vollständig enthalten (nur Whitespace normalisiert). WordNet LICENSE außerdem mit Original-Tararchiv identisch. [Volltextabgleich/Hashes](license-check.json), [Primärquellen](dictionary-evidence.md). Der anfängliche bytegenaue WordNet-Substring-Test in identity.json war wegen Zeilenend-Leerzeichen false; der vollständige normalisierte Vergleich ist true. |
| 14 Re-Verifier-Fälle + servula | `pwsh -NoProfile -File C:/Users/Axel_desktop/.t3/worktrees/kairos-password/t3-d9ce504b/target/verify/check-independent-findings.ps1; node target/verify/prepare.mjs` | 0 | [Gate-Rohlog](independent-gate.log). Zusätzlich alle 15 Wörter selbst mit Endlisten abgeglichen: keines enthalten, siehe regressions in [identity.json](identity.json). Gate-Datei nur gelesen/ausgeführt, nicht geändert. |
| A/B-Rubrik entspricht Lead-Abnahme | `Get-Content crates/kairos-wordlists/data/curation-rubric.md` | 0 | FAIL hinsichtlich Stichprobenumfang: Zeile 74 fordert nur je 300 EN/LA, DE fehlt gegenüber dem Auftrag. A-Hauptklassen und eingeschränkte Nebenbedeutungsregel entsprechen sonst dem vorgegebenen Standard. [Zeilennummerierte Rubrik](rubric-numbered.txt). Diese Verifikation prüft trotzdem alle drei Sprachen. |
| Frische Stichprobe DE | `node target/verify/prepare.mjs; manuelle Einzelbewertung; node target/verify/assess.mjs` | 0 / manuell / 1 (A-Gesamtgate) | FAIL: vollständig 300/300 bewertet, A/B/N=1/51/248, offen=0. Seed 35ac09f0ace0d8c3253f6b2b78f45941. [bewertete TSV](sample-de-assessed.tsv), [direkter Quellenauszug](sample-de-source.tsv), [Seed](seeds.json), [Gate](semantic-assessment.log), [Integritätscheck](sample-integrity.json). |
| Frische Stichprobe EN | `node target/verify/prepare.mjs; manuelle Einzelbewertung; node target/verify/assess.mjs` | 0 / manuell / 1 (A-Gesamtgate) | FAIL: vollständig 300/300 bewertet, A/B/N=2/85/213, offen=0. Seed fe19b40ce02fd13c833f026b7a7d26bc. [bewertete TSV](sample-en-assessed.tsv), [direkter Quellenauszug](sample-en-source.tsv), [Seed](seeds.json), [Gate](semantic-assessment.log), [Integritätscheck](sample-integrity.json). |
| Frische Stichprobe LA | `node target/verify/prepare.mjs; manuelle Einzelbewertung; node target/verify/assess.mjs` | 0 / manuell / 1 (A-Gesamtgate) | FAIL: vollständig 300/300 bewertet, A/B/N=1/67/232, offen=0. Seed 943fe332f5f7bbea14b29683ed8a9772. [bewertete TSV](sample-la-assessed.tsv), [direkter Quellenauszug](sample-la-source.tsv), [Seed](seeds.json), [Gate](semantic-assessment.log), [Integritätscheck](sample-integrity.json). |
| Co-Author-Trailer neuer Commits | `git log fa35aded13b50a67a817db79258b1ba1c95a9d46..HEAD --format="commit %H%nparents %P%n%B"; node target/verify/additional.mjs` | 0 | PASS: 7/7 neue PR-Commits einschließlich beider Merge-Commits besitzen Co-Authored-By. [Originalnachrichten](commits.log), [Auswertung](trailers.json). |
| PR-Beschreibung aktuell | `gh pr view 7 --json number,url,title,body,headRefName,headRefOid,baseRefName,baseRefOid,mergeable,mergeStateStatus,statusCheckRollup` | 0 | PASS für Implementierungsstand: korrekter Head 8696255, Rubrik, Endgrößen, Deltas und offene unabhängige Abnahme angegeben; kein semantischer PASS behauptet. [vollständiger Abschlusssnapshot](final-pr.log). Die hier neu gefundenen Befunde sind naturgemäß noch nicht Bestandteil der Beschreibung. |
| Unveränderte Quellen / Grenzen | `git status --porcelain; git rev-parse HEAD; git diff --check` | 0 / 0 / 0 | PASS: [Git-Status](final-status.log), [Head](final-head.log), [Whitespace-Prüfung](diff-check.log). Nur eigene Dateien unter target/verify und Build-/Temp-Dateien im lokalen target. Ein Cargo-Target, Debug/Test, keine Release-Builds. Kein Commit, Push, Kommentar, Review, Merge oder Issue. |

## Semantische Befunde

1. **DE farbig**, `crates/kairos-wordlists/data/german.txt:2054`: Nebenbedeutung:Slur. Personenbezogener ethnischer Sinn laut Duden diskriminierend; neutraler Farbsinn hebt den Slur-Nebensinn nicht auf. [Bedeutungsquelle](https://www.duden.de/rechtschreibung/farbig).

2. **EN lunchbox**, `crates/kairos-wordlists/data/languages/english.txt:3463`: Nebenbedeutung:Sexualitaet. Britischer Slang für männliche Genitalien innerhalb der Kleidung; WordNet NO_ENTRY; Wörterbuch schließt Evidenzlücke. [Bedeutungsquelle](https://en.wiktionary.org/w/index.php?title=lunchbox&oldid=93705835).

3. **EN pucker**, `crates/kairos-wordlists/data/languages/english.txt:4520`: Nebenbedeutung:Sexualitaet. Anus-Slang; Wörterbuch belegt ausdrücklich anal-sexuellen Gebrauch, nicht nur neutrale Anatomie. WordNet erfasst diesen Nebensinn nicht. [Bedeutungsquelle](https://en.wiktionary.org/wiki/pucker).

4. **LA lascivio**, `crates/kairos-wordlists/data/languages/latin.txt:3382`: Nebenbedeutung:Sexualitaet. Rohglosse nur frisk/sport/run riot; Lewis & Short ergänzt ausdrücklich in Venerem, to be lascivious (Col. 6,24,2). [Bedeutungsquelle](https://thrax.mpiwg-berlin.mpg.de/mpiwg-mpdl-cms-web/lt/GetDictionaryEntries?dictionary=ls&inputType=lemma&language=lat&outputFormat=html&outputType=dictFull&query=lascivio).

Die Kriterien für Nebenbedeutungen wurden eng angewandt. Tier-/Gegenstandsmetaphern bleiben B; nichtsexuelle Nebenbedeutungen zu Gewalt, Alkohol, Krankheit oder Verbrechen wurden nicht pauschal als A gezählt. Beispiele: baggie und trip sind B, medizinische Versorgung/Neutrale Anatomie ebenfalls. Die Bedeutung von pucker wurde wegen des ausdrücklich sexuellen Wörterbuchbelegs als A beurteilt. Weitere Belegdetails und kurze Originalzitate stehen in [dictionary-evidence.md](dictionary-evidence.md).

## Stichprobenmethode und Grenzen

Pro Sprache ein unabhängig erzeugter 128-Bit-Zufallsseed, vor der Bewertung festgeschrieben in seeds.json. Alle Endwörter werden nach SHA-256(seed + NUL + Sprachcode + NUL + Wort) aufsteigend gerankt; die ersten 300 bilden die Probe ohne Zurücklegen. Die Ausgabe ist nur für die Lektüre alphabetisch geordnet. Es gab genau eine Ziehung pro Sprache. Auswahl und Unverändertheit wurden nach der Bewertung erneut rechnerisch kontrolliert.

DE: Einzellektüre mit wortbezogener deutscher Bedeutung pro Zeile und Wörterbuchkontrolle von Grenzfällen. EN: unabhängiger Parser des originalen WordNet-3.0.tar.gz, alle passenden Sinne über Daten-/Indexdateien und morphologische Ausnahmen; Wörterbücher bei Zweifeln. NO_ENTRY bleibt im TSV sichtbar und wird nicht als automatische Freigabe behandelt. Mechanische Fehltreffer (his → hi) wurden semantisch nicht übernommen. LA: unabhängig aus Latin-1-Rohquelle rekonstruierte Lemmata; alle verfügbaren Glossen derselben decodierten Schreibweise, auch Frequenzen außerhalb A–D, vollständig gelesen. Zusätzliche lateinische Wörterbuchkontrolle bei lascivio.

Die A/B/N-Zuordnung ist die dokumentierte Entscheidung dieses Verifiers nach Einzelprüfung, keine Keyword-Klassifikation. Die Hilfsskripte ziehen/prüfen Stichproben und schreiben diese Entscheidungen aus. B zählt akzeptable Grenzfälle; N bezeichnet unauffälligen neutralen Wortgebrauch. Eine Stichprobe ist keine Vollabnahme aller 20.703 Bedeutungen.

Build-Umgebung: vorgeschriebener Helper dot-gesourct, Rust/Cargo 1.99.1 MSVC, CARGO_TARGET_DIR ausschließlich lokales target, incremental=0, Debugdaten=0, zwei Buildjobs; [Umgebungslog](build-environment.log). Aus dem vollständigen frischen Testlog: **148 bestanden, 0 fehlgeschlagen, 7 ignoriert** (Unit/Integration/Doctests zusammen).

## Blocker und nächster Schritt

Semantische Abnahme blockiert durch die vier A-Fälle. Zusätzlich fehlt DE im dokumentierten 300er-Abnahmeumfang der Rubrik.

An Lead/Autor zurückgeben: konkrete A-Befunde und Evidenzlücken behandeln, die Rubrik auf DE/EN/LA vervollständigen und anschließend nach der festgelegten Korrektur-/STOP-Regel eine neue unabhängige Abnahme beauftragen. Dieser Verifier ändert keine Produktdateien und erteilt keine Merge-Freigabe. Kein wiederholter Infrastrukturfehler; STOP-Regel wurde nicht ausgelöst.
