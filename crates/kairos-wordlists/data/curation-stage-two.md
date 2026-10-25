# Abnahmestandard A/B und Stufe 2

**25.10.2026 (Europe/Berlin): Bereit zur unabhängigen Prüfung (präzisierte Rubrik).**

Der Lead hat `en/reptile` als B entschieden. Der frühere STOP beruhte auf
der inzwischen ersetzten Nebenbedeutungsregel. Die zweite eigene Probe bleibt
unvollständig; eine vollständige semantische Abnahme steht weiterhin aus.

Ausgangspunkt war PR #7 bei `691860b603c8042b4bca862015628ce4a568b215`.
Die [Diagnose aus Stufe 1](curation-diagnosis.md) und das damalige
[Review](languages/fixdiff-review.md) bleiben historische Belege.
Die lokale Basis ist `5395f2af7b801e2877ed2b98cacb429666a1ec3e`.
Die Stufe-2-Implementierung wird mit der Rubrik-Präzisierung auf dem
PR-Branch `feat/languages-en-la` veröffentlicht.

## Lead-Entscheid Rubrik-Präzisierung

Der ausdrückliche Lead-Entscheid vom 25.10.2026 ersetzt die bisherige
Nebenbedeutungsregel: Nur sexuelle, vulgäre/obszöne oder Slur-Nebensinne
erzwingen A. Slurs betreffen Ethnie, Religion, Behinderung,
Geschlecht/Sexualität oder Herkunft. Die strengen A-Klassen für
Hauptbedeutungen bleiben bestehen.

Tier- oder Gegenstandsmetaphern mit neutraler Hauptbedeutung sind B,
ausdrücklich `reptile`, `snake`, `pig`, `rat`, `cow`, `worm` und `weasel`.
`reptile` bleibt in EN. Die bereits dokumentierte abwertende
Menschenmetapher ([Merriam-Webster, Sinn 3](https://www.merriam-webster.com/dictionary/reptile))
erzwingt damit keinen Ausschluss. Bereits gesperrte Wörter bleiben gesperrt.

Begründung des Leads: Fast jedes Alltagssubstantiv besitzt eine mögliche
abwertende Metapher. Der Maßstab soll peinliche/verletzende Passwörter
verhindern, ohne die Sprache auszudünnen; methodischer Vergleich ist die
EFF-Wordlist-Kuratierung. Die [verbindliche Rubrik](curation-rubric.md),
das Stichprobengate und die positiven/negativen Regressionen bilden diese
Unterscheidung ab. Es wurde keine neue semantische Stichprobe gezogen und
keine offene Zeile nachklassifiziert. Die vorhandene `reptile`-Zeile wurde
ausschließlich nach dem ausdrücklichen Lead-Entscheid von A auf B korrigiert.
Ein frischer unabhängiger Verifier übernimmt die fachliche Prüfung.

## Abhängigkeit und deutsche Triage

`gh pr view 2 --repo ACL0815/Kairos_Passwort --json state` ergab `MERGED`.
`gh pr edit 7 --repo ACL0815/Kairos_Passwort --base main` war erfolgreich.
`origin/main` wurde konfliktfrei übernommen, Commit
`5395f2af7b801e2877ed2b98cacb429666a1ec3e`,
`Merge main after German wordlist integration`, mit dem verlangten Co-Author-Trailer.
Es erfolgte kein GitHub-Merge. Eigene Änderungen liegen in den freigegebenen
Wortlisten-/Quelldokumentationspfaden; der Hauptcheckout wurde nicht geändert.

Die [299er-Triage](languages/stage2-german-triage.tsv) ergibt **224 zur Sperrung
adjudizierte Fälle und 75 beibehaltene B-Fälle**. Dazu gehören ausdrücklich
gekennzeichnete konservative Grenzfälle, keine Behauptung von 224 gleichermaßen
eindeutigen A-Befunden. Die [39 Nachrücker](languages/stage2-german-followup.tsv)
ergeben 35 A-Formen gesperrt, 1 B vorsorglich gesperrt und 3 B aufgenommen.
Die drei neuen Endwörter sind `bares`, `burgen` und `coupe`.

Umgesetzt in der deutschen Pipeline: 260 zusätzliche Sperrlisteneingaben,
52 widersprüchliche Ergänzungsfreigaben entfernt. Von den alten Endwörtern
entfallen 224; 3 neutrale Formen kommen hinzu. DE endet bei **6.909** statt
7.130. `klopapier`, `klosett` und `toilette` bleiben als neutrale
Hygienegegenstände/-räume B; sie bezeichnen nicht selbst eine Ausscheidung.
Quellen und deutscher Decoder bleiben unverändert.

## Englische Bedeutungsquelle

Princeton WordNet 3.0 ließ sich mit vertretbarem Aufwand integrieren.
Das [unveränderte Originalarchiv](https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz),
die [Originallizenz](languages/upstream/wordnet/LICENSE) und der reproduzierbare
Auszug liegen lokal. Die vollständige Lizenz steht zusätzlich in
`languages/NOTICE.txt`. Das [Quellen-README](languages/upstream/wordnet/README.md)
erläutert Verfahren, Containerhash und Grenzen.

Der eigene Extraktor liest alle vier Wortarten, vier Ausnahmedateien und
`lexnames`. Er vereinigt exakte und dokumentierte einfache morphologische
Treffer. Alle zugeordneten Definitionen, Synset-Offsets, Lemmata,
`lex_filenum`/`lexname` und das explizite EFF-Large-Signal sind an den
SHA-256-Fingerprint jeder EN-Entscheidung gebunden.
Die 9.558 Kandidaten haben **8.924 WordNet-Treffer**, **634 NO_ENTRY-Zeilen**
und **7.772 EFF-Large-Mitgliedschaften**. In der jetzigen EN-Endliste haben
6.493 Wörter WordNet-Belege, 404 keinen WordNet-Eintrag und 5.573 das
EFF-Large-Signal. Alle 404 verbleibenden Lücken besitzen das EFF-Signal.

Die direkte, bereits vendorte EFF-Quelle bleibt unter ihrer dokumentierten
CC-BY-4.0-Freigabe; die im Auftrag beispielhaft genannte CC BY 3.0 betrifft
ältere Kopien. EFF ist ein zusätzliches externes Kurationssignal, keine
automatische A/B-Freigabe. Ein vollständiger Wiktionary-Dump wurde nicht
importiert. [16 eigene zusätzliche Befundnotizen mit Primärquellenlinks](languages/english-extra-evidence.tsv)
ergänzen belegte sexuelle, vulgäre oder abwertende Sinne, die WordNet auslässt.
Der Build kontrolliert ihre Bindung an den Auszug.

Die Quellenlücke bleibt bestehen: Ein WordNet-Eintrag kann weitere Sinne
auslassen. Bei `reptile` fehlt die zusätzlich belegte Menschenmetapher;
nach präzisierter Rubrik ist diese B. Weder WordNet noch EFF sind ein
vollständiger Unbedenklichkeitsnachweis nach dieser Rubrik.

## Sichtung, Ursachen-Korrekturdurchgang und Review

Genau zwei lesende Klassifikationskinder liefen auf `codex/gpt-6.1-sol`,
`reasoningEffort=high`, `serviceTier=priority`.
Die [LA-Klassifikation](languages/stage2-la-classification.md) umfasst
8.680 anfangs freigegebene Schreibweisen, alle 9.796 zugeordneten Rohglossen
und 1.714 Reserven. Die [EN/DE-Klassifikation](languages/stage2-en-de-classification.md)
prüfte alle 8.150 anfangs erlaubten EN-Schreibweisen, durchsuchte deren
29.118 WordNet-Zuordnungen klassenweise und triagierte alle 299 DE-Meldungen.
Die EN-Sichtung war keine vollständige menschliche Einzelabnahme aller Sinne.

Genau ein lesendes Fixdiff-Review lief auf
`claudeAgent/claude-opus-5-5`, `effort=medium`, `fastMode=false`.
[Unverändertes Review mit anschließendem Umgang](languages/stage2-fixdiff-review.md).
Urteil vor dem Korrekturdurchgang: **FAIL**. Eigene 200er-Probe des Reviewers,
Seed `20261025-review-worker-ab200-v1`, EN 1 A / 5 B / 94 N,
LA 0 A / 7 B / 93 N. Die Stichprobe überschneidet sich in 6 Wörtern mit
der ersten eigenen Probe. Keine Weiterdelegation, keine Repo-Änderungen der Kinder.

Im einen gemeinsamen Ursachen-Korrekturdurchgang wurden die bestätigten
Review-Befunde `cat creature gull lamb doormat shark pretender pseudo lizard
renegade puppet dirtiness skirt` ausgeschlossen. Alle 669 noch freigegebenen
EN-Kandidaten mit **938 Personenbedeutungen** einschließlich Reserven wurden
vollständig in diesem Teilbereich gelesen und
[quellgebunden protokolliert](languages/stage2-person-senses.tsv).
Der Build verlangt diese explizite Sichtung für jede erlaubte
`noun.person`-Zuordnung. Ohne WordNet und ohne EFF-Large-Signal wurden
[86 Kandidaten](languages/stage2-english-evidence-gaps.txt) zunächst gesperrt;
darunter ist cyber als A bestätigt, 85 bleiben konservative B-Sperren.
Dies sind keine 86 bestätigten A-Funde.

Weitere gezielte Restsuchen betrafen Ausscheidung, sexuelle/ethnische Slangsinne,
Sklaverei, antike Haushaltsrollen, Verbannung und Betrug. Die
[Stufe-2-Adjudikationen](languages/stage2-decisions.tsv) unterscheiden
Urteil und Aktion: EN **220 A-Sperren, 102 B erlaubt, 155 B konservativ gesperrt**;
LA **83 A-Sperren und 60 B konservativ gesperrt** (historische Einordnung
der 620 Entscheidungen vor der Präzisierung). Hinzu kommt die ausdrückliche
B-Freigabe von `reptile` durch den Lead, insgesamt 621 Registereinträge.
Die 21.190 vollständigen
semantischen Entscheidungen werden deterministisch konsumiert. Alte
strengere Sperren wurden nicht zurückgenommen. Die zwischenzeitliche
Freigabe von B-Vorschlägen betraf nur Vorschläge dieser uncommitteten Stufe.

`servula` bleibt durch die verbindliche Lead-Adjudikation A. Die Rohglosse
`young servant girl` allein ist als historische Statusbestimmung begrenzt;
dies ist sichtbar dokumentiert. `pouncing bladder absurde situs` bleiben
als positive B-Regressionen enthalten. Alle 14 Re-Verifier-Fälle plus
`servula` und die neuen Ursachenfälle werden negativ getestet.
Der Quellenwächter behandelt Dienerschaft nicht pauschal als Sklaverei.

## Historische eigene Proben und unvollständige zweite Probe

Auswahl: je Sprache die 300 kleinsten Werte von
`SHA256(UTF8(seed + "|" + language + "|" + word))`, ohne Wiederholung.
Die TSV-Dateien enthalten Auswahlhash, Wort, Sprache, vollständigen Quelltext,
Urteil, Kategorie und Begründung; JSON bindet Seed und Listenhashes.

| Probe | Sprache | A | B | N | Unbewertet |
|---|---|---:|---:|---:|---:|
| `20261025-kairos-stage2-own-ab-v1` | EN | 2 | 147 | 151 | 0 |
| derselbe Seed | LA | 2 | 142 | 156 | 0 |
| `20261025-kairos-stage2-own-ab-v2` | EN | 0 | 96 | 55 | 149 |
| derselbe zweite Seed | LA | nicht geprüft | 0 | 0 | 300 |

[Erste vollständige Probe](languages/stage2-own-sample-v1.tsv):
A = EN `cyber skirt`, LA `hera relegatio`. Gate vor Korrektur **Exit 1**,
`semantic_sample_findings=4`. Die erste Probe ist eine historische Momentaufnahme;
sie ist nach geänderten Quellen und Listen absichtlich nicht als aktuelle Abnahme wiederverwendbar.

[Zweite Probe](languages/stage2-own-sample.tsv): Die ersten 150 EN-Zeilen
sind fertig protokolliert; zusätzlich ist `reptile` nach dem Lead-Entscheid
als B dokumentiert (historisch A nach alter Regel). 149 EN- und alle 300
LA-Zeilen bleiben ausdrücklich unbewertet. Die Null in der EN-A-Spalte
betrifft nur die bewerteten Zeilen; sie ist keine Freigabe der ganzen Probe.
Das Stichprobengate verweigert weiterhin eine unvollständige Probe. Es ist
kein Bestandteil der abgeschlossenen technischen Abnahme und wird nicht
als grün ausgegeben. Kein dritter Seed und keine neue semantische Probenrunde.

## Lokaler Datenstand und technische Belege

DE **6.909**, EN **6.897**, LA **6.897**, zusammen **20.703**, paarweise disjunkt,
nur a–z und Länge 3–9. EN/LA liegen 12 Wörter unter DE; der LA-Pool nach
Überschneidungs-/Variantenfilter begrenzt die gemeinsame Größe. Der EN-Pool
hätte 6.923 Kandidaten. Mittelwerte DE 6,577 / EN 6,659 / LA 6,988.
Die EN-Komfortgrenze wurde nach den zusätzlichen kurzen Ausschlüssen von
6,6 auf 6,7 offengelegt angehoben; die Zeichen-/Längeninvariante bleibt gleich.
Vier unabhängige Wörter aus der Vereinigung entsprechen 57,350 Bit,
Größenfaktor gegenüber DE 80,625 statt pauschal 81.

Deltas gegenüber `691860b`: **DE 224 entfernt / 3 hinzugefügt,
EN 287 / 218, LA 77 / 8**. Vollständige Entfernungen mit Gründen und vorherigen
LA-Glossen: [DE](languages/stage2-removed-de.tsv),
[EN](languages/stage2-removed-en.tsv), [LA](languages/stage2-removed-la.tsv).
Größenangleichung, Sprachabzüge, Variantenfilter und konservative Sperren
werden nicht pauschal als bestätigte A-Funde ausgegeben.
[Ausgabehashes und Zahlen](languages/stage2-statistics.json).

`cargo test -p kairos-wordlists` nach dem Ursachen-Korrekturdurchgang:
**Exit 0**, 12 DE-Tests + 23 EN/LA-Tests + 2 Doctests = **37 bestanden**.
Neuerzeugung mit `cargo run -p kairos-wordlists --example rebuild-languages`:
**Exit 0**. Ein früherer vollständiger Workspace-Testlauf vor dem
Korrekturdurchgang ergab **Exit 0, 145 bestanden, 7 ignoriert**.
Der frühere Clippy-/Fmt-Check war ebenfalls grün; er ist keine abschließende
Bestätigung des späteren Code-Diffs.

Die Abschlussprüfungen nach Rubrik-Präzisierung werden unten mit Exitcodes
dokumentiert. Die unabhängige semantische Prüfung bleibt der nächste Schritt.

## Abschlussprüfungen nach Rubrik-Präzisierung

Alle folgenden Läufe erfolgten am 25.10.2026 (Europe/Berlin) direkt im
vorgegebenen Worktree `t3-3ecd0667`, nach Laden von `Enter-KairosBuildEnv.ps1`.
Ein einziges vorhandenes `CARGO_TARGET_DIR=target/stage2-build` wurde verwendet;
alle Cargo-Läufe verwendeten das Debug-/Testprofil.

| Befehl | Exit | Ergebnis |
|---|---:|---|
| `cargo run -p kairos-wordlists --example rebuild-german` | 0 | DE neu erzeugt, 6.909 Wörter |
| `cargo run -p kairos-wordlists --example rebuild-languages` | 0 | EN/LA neu erzeugt, je 6.897 Wörter; Hashes unverändert |
| `cargo fmt --check` | 0 | Grün |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | Grün |
| `cargo test --workspace` | 0 | 140 Unit-/Integrationstests + 8 Doctests bestanden, 7 ignoriert, 0 Fehler |
| `cargo run -p kairos-wordlists --example rebuild-german -- --check` | 0 | Bytegleich zur Pipeline |
| `cargo run -p kairos-wordlists --example rebuild-languages -- --check` | 0 | Listen, Quellbindung, Hashes, Bericht und LA-Glossen reproduziert |
| `pwsh -NoProfile -File C:\Users\Axel_desktop\.t3\worktrees\kairos-password\t3-d9ce504b\target\verify\check-independent-findings.ps1` | 0 | Unverändertes Gate für alle 14 Fälle, `findings=0` |
| `node crates/kairos-wordlists/tools/prepare-english-senses.mjs --check` | 0 | WordNet-Auszug reproduziert: 9.558 Kandidaten, 8.924 Treffer, 634 Lücken |
| Zusätzlicher PowerShell-Datenabgleich | 0 | Sortierung, Eindeutigkeit, a–z, 3–9, Größen, Hashes und alle drei leeren Schnittmengen bestätigt; sämtliche Hinzufügungs-/Entfernungsregister exakt zum Ausgangshead `691860b` |
| Isolierte Ausführung des tatsächlichen Nebenbedeutungs-Gateblocks mit synthetischen Eingaben | 0 | 8 Fälle: korrekte A/B-Urteile angenommen, jeweils umgekehrte Urteile abgewiesen |

Die Wortlisten-Suite umfasst jetzt 12 DE-Tests, 24 EN/LA-Tests und 2 Doctests,
insgesamt 38 bestandene Tests. Die neuen Regressionen sichern `reptile` als B,
Tier-/Gegenstandsmetaphern und die getrennten sexuellen/vulgären/Slur-Klassen.
Bestehende Sperrregressionen bleiben wirksam. Bei den Workspace-Tests gab es
MSVC-LNK4099-Hinweise zu fehlenden OpenSSL-PDBs; der Lauf endete mit Exit 0.

Diese technischen Ergebnisse ersetzen keine unabhängige semantische Abnahme.
Die vorhandene zweite Probe bleibt unvollständig; ihre 449 offenen Zeilen
wurden nicht weiter bewertet und das vollständige Stichprobengate wird nicht
als bestanden ausgegeben. Nächster Schritt: frischer unabhängiger Verifier
am veröffentlichten PR-Head unter der präzisierten Rubrik.
