# Englisch und Latein

Der aktuelle Abnahmestandard unterscheidet zwingende A-Ausschlüsse und
zulässige B-Fälle. Der historische STOP aus Stufe 1 und die Fortsetzung
werden getrennt dokumentiert. [Rubrik](../../crates/kairos-wordlists/data/curation-rubric.md),
[aktueller Prüfstand Stufe 2](../../crates/kairos-wordlists/data/curation-stage-two.md).

PR-7, **Prio 4**, `feat/languages-en-la`, **Base main nach Merge von #2**.
Abruf/historische Stichproben: **25.10.2026**.
Bewertung nach Prüfung der Primärquellen am 25.10.2026.

Code- und Datenlizenzen werden getrennt betrachtet. **Kein fremder
Implementierungscode wurde kopiert**. Übernommen sind nur lizenzierte Wortdaten
und Lizenztexte. Die Passung bewertet Umfang, Lizenz, Abschreibbarkeit und
Aufbereitungsaufwand zusammen. Alle Zeilen wurden am 25.10.2026 abgerufen.

| Projekt | Permalink / Stand | Lizenz | Datei/Modul | Prinzip / Entscheidung | Passung 1–5 + Bewertung |
|---|---|---|---|---|---|
| EFF / Joseph Bonneau | [Long, 18.07.2016](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), [Short 1](https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt), [Short 2](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt), [Methodik, Juli 2016](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) | **Daten: CC BY 4.0 gewählt**, laut aktueller [EFF-Freigabe](https://www.eff.org/copyright) für EFF-Originalmaterial, sofern nicht anders angegeben; ältere xkcdpass-Kopien nennen CC BY 3.0. | Drei Rohlisten von 2016, 7.776 + 1.296 + 1.296 Zeilen | Bekannte schreibbare Wörter, Prüfung vor Auswahl. Vereinigung nach `a-z`, 3–9: 8.301 verschiedene Wörter. Eigene strengere Belastungsprüfung nötig. Datierte URLs sind keine unveränderlichen Git-Commits; lokale Originalbytes durch SHA-256 festgelegt. | **5** – beste Passphrasenbasis; kurze Ergänzungen nötig. Dreizeichenpräfixe/Editierabstand der separaten Kurzliste gelten nach Kombination nicht. |
| Wordnik | [Wortliste, Commit 46e6215](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/wordlist-20210729.txt), [README](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/README.md) | **MIT**, Copyright (c) 2020 Wordnik, [Lizenz](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/LICENSE) | `wordlist-20210729.txt`, 198.422 Zeilen | Nur ausdrückliche Positivliste von 1.257 kurzen Wörtern; Mitgliedschaft gegen vollständige komprimierte Quelle geprüft. Keine automatische Aufnahme des Wortspiel-Wörterbuchs. | **4** – lizenzklarer Ergänzungspool; ungeprüft wegen seltenen/belastenden Wörtern ungeeignet. |
| Whitaker's WORDS | [Lexikon, Commit 1f2f0fb](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/DICTLINE.GEN), [Projekt](https://github.com/mk270/whitakers-words/tree/1f2f0fb0867a896d7b9284a03d615ed635d6f992) | **Individuelle permissive Freigabe für Programm und Daten**, [Originaltext](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/LICENCE.txt); keine CC0-/PD-Behauptung. | `DICTLINE.GEN`, 39.335 Datensätze; [Frequenz-/Grammatikbeschreibung](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/src/latin_utils/latin_utils-inflections_package.ads) | Gewählte Lateinquelle: Stämme, Grammatik, Frequenz und Bedeutungen. Begrenzter eigener Decoder für unterstützte Grundformen; 11.632 passende Kandidaten A–D vor Kuratierung. Keine Ada-Codeübernahme. | **4** – genug Daten und breite Weitergabefreigabe; Quelldatenfehler und ungewohnte Lemmata verlangen dokumentierte Grenzen. |
| xkcdpass | [Auswahlroutine, Commit b981361](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py), [README](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/README.rst) | **Code: BSD-3-Clause**, [Lizenz](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/LICENSE.BSD); Daten je Liste separat. | `generate_wordlist`, `xkcdpass/static/` | Mehrere Quellen vor unabhängiger Indexwahl vereinigen und deduplizieren; Zeichen/Länge vorher filtern. Kairos bewahrt zusätzlich Sprachflags und wendet deutschen Vorrang an. | **5** – direkt passendes Prinzip; Flattening allein verliert Sprachherkunft. |
| sts10/tidy | [README, Commit 449c187](https://github.com/sts10/tidy/blob/449c1875e8a8a6cb973640265486195cb7ae87e8/readme.markdown), [Lizenz](https://github.com/sts10/tidy/blob/449c1875e8a8a6cb973640265486195cb7ae87e8/LICENSE) | **Code: MIT**, keine Wortdaten mitgeliefert | Listenvereinigung, ASCII-/Längenfilter, Sperr-/Freigabelisten | Reproduzierbare Datenpflege vor Passworterzeugung; Prinzip übertragen, keine Logik kopiert, keine weitere Laufzeit-Crate. | **4** – passende Filterprinzipien; Sprachherkunft bleibt Aufgabe von Kairos. |
| Uli Fouquet/diceware | [README, Commit 89534ea](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/README.rst), [COPYRIGHT](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/COPYRIGHT) | **Code: GPL-3.0-or-later**, Wortdaten separat, z. B. Deutsch CC BY 3.0, Französisch CC0. | `diceware/wordlists/`, Mehrlisten-Muster | Dateispezifische Lizenzen und reale Größen. Feste Mehrlisten-Muster sind keine freie Sprachwahl pro Wort. Kein Code und keine Daten übernommen. | **3** – gute Metadatenvorlage; Generatorsemantik für diesen Auftrag weniger passend. |
| pwgen-go | [Listen, Commit 5a53dce](https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist), [Lizenz](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/LICENSE) | **Code: MIT**, EFF-Daten separat | `internal/wordlist/` | Eingebettete Daten vom Generator trennen. Rust setzt den PR-2-Ansatz mit `include_str!` fort. | **4** – einfaches Modell, keine Go-Codeübernahme. |
| passalong-core 0.3.0 | [words.rs, feste Version](https://docs.rs/passalong-core/0.3.0/src/passalong_core/crypto/words.rs.html), [Crate](https://docs.rs/crate/passalong-core/0.3.0) | **Code: Apache-2.0**, Daten separat | `LIST`, `word_list()` | Schon in PR-2 modelliertes `OnceLock`: Text einmal indexieren, geliehene Wortreferenzen mit Metadaten teilen. | **5** – Standardbibliothek genügt. |
| Orchard Street | [Long, v0.4.3 / Commit 4fd015f](https://github.com/sts10/orchard-street-wordlists/blob/4fd015fe9a8e50d837d9f54cb39883bb801da1ed/lists/orchard-street-long.txt), [README/Lizenz](https://github.com/sts10/orchard-street-wordlists/blob/4fd015fe9a8e50d837d9f54cb39883bb801da1ed/readme.markdown) | **Daten: CC BY-SA 4.0** | `orchard-street-long.txt`, 17.576 Wörter, 13.130 nach strengem Filter | Frequenz und redaktionelle Regeln sind nützlich. **Nicht übernommen:** ShareAlike vermeidbar, weil EFF plus gezielte MIT-Ergänzungen reichen. | **4** – großer geeigneter Pool, hier unnötige Copyleft-Entscheidung. |
| LiLa Lemma Bank | [Dump/README, Commit 1a1ee92](https://github.com/CIRCSE/LiLa_Lemma-Bank/tree/1a1ee929e3bca85781582482b7c65ee86f62d568) | **Daten: CC BY-SA 4.0**, laut README | `lila_db.sql.gz`, Lemmaklassen und Wortarten | Explizite Hauptlemmata und `PROPN` sind nützlich; keine Abschreib-/Häufigkeitsauswahl. **Nicht übernommen:** Whitaker reicht permissiv aus, keine SQL-Pipeline nötig. | **4** – guter lemmaorientierter Pool, aber vermeidbares ShareAlike. |
| Perseus / Lewis & Short | [XML, Commit 51b2f90](https://github.com/PerseusDL/lexica/blob/51b2f90b515d51ff1052bb9f0c5ec17fa7246859/CTS_XML_TEI/perseus/pdllex/lat/ls/lat.ls.perseus-eng2.xml), [Dateihinweise](https://github.com/PerseusDL/lexica/blob/51b2f90b515d51ff1052bb9f0c5ec17fa7246859/CTS_XML_TEI/perseus/pdllex/lat/ls/README.md) | **Daten: CC BY-SA 4.0**, Attribution und weitere Dateihinweise | `lat.ls.perseus-eng2.xml` | Kopfwort-/Bedeutungsfelder nützlich; große XML-Aufbereitung ohne Komfortkuratierung. **Nicht übernommen:** permissive Alternative vorhanden. | **3** – reiches Lexikon, hier unnötiger Aufwand. |
| Wiktionary / Wiktextract | [Latein-Index](https://kaikki.org/dictionary/Latin/index.html), [Wikimedia-Bedingungen](https://foundation.wikimedia.org/wiki/Terms_of_Use), veränderliche Portale | Grundsätzlich **CC BY-SA/GFDL**, importierte Inhalte ggf. abweichend; konkreten Dump separat prüfen. | Latein-JSONL; kein übernommener Dump | Wortarten/Grundformen, aber großer Formbestand und importabhängige Bedingungen. **Nicht übernommen**, keine Pipelineeingabe. | **2** – gegenüber Whitaker kein Nutzen für den nötigen Umfang. |
| Latin Diceware / Sebastian Możejko | [Signierte Wortdatei](https://www.theworld.com/~reinhold/diceware_latin.txt.asc), [Autoren-/Lizenzhinweis](https://theworld.com/~reinhold/diceware.html) | CC-BY laut Reinhold, genaue Version in der Datei nicht bezeichnet. | `diceware_latin.txt.asc`, 7.776 Würfelzuordnungen | Fertiges Format, aber viele Namen, Flexionsformen und Fragmente wie `astr`/`anim`. **Nicht übernommen**, weitere Lizenz- und Inhaltsprüfung nötig. | **2** – zu wenig Kuratierungspuffer. |
| esumbar/passphrase | [Latin-dice.txt, Commit 5faaa6d](https://github.com/esumbar/passphrase/blob/5faaa6dcf71f31bc6acad6f683f581408e0b5c32/database/raw_data/Latin-dice.txt), [README](https://github.com/esumbar/passphrase/blob/5faaa6dcf71f31bc6acad6f683f581408e0b5c32/README.md) | **Code: MIT**, eigenständige Datenherkunft/-lizenz nicht belegt. | `database/raw_data/Latin-dice.txt` | Mehrere Wortformen pro Würfelzeile; **nicht übernommen** wegen ungesicherter Datenlizenz und Flexionsfülle. | **2** – kein belegter Vorteil. |
| Original Diceware / Reinhold | [Englische Wortliste](https://theworld.com/~reinhold/diceware.wordlist.asc), [Datenlizenzhinweis](https://theworld.com/~reinhold/diceware.html) | Englische Liste **CC BY 4.0**, soweit geschützt; Webseitenlizenz separat. | `diceware.wordlist.asc` | Kürze hilfreich, aber auch Abkürzungen und Zeichenfolgen; streng gefiltert kleiner als Ziel. **Nicht übernommen.** | **2** – weniger passend als EFF plus Ergänzungen. |
| xkcdpass legacy / 12Dicts | [legacy, Commit 9db4395](https://github.com/redacted/XKCD-password-generator/blob/9db439542d9a156f8a9db56323b459670175c132/xkcdpass/static/legacy), [12Dicts-Erklärung](https://wordlist.aspell.net/12dicts-readme/) | Legacy nennt 12Dicts-PD-Herkunft; konkrete Unterliste unbenannt, manche 12Dicts-Listen hängen von AGID ab. | `xkcdpass/static/legacy` | Großer Pool, aber Herkunft weniger konkret und keine Passphrasen-Kuratierung. **Nicht übernommen**, nicht mit der BSD-Codelizenz gleichsetzen. | **2** – Wordnik ist für gezielte Ergänzungen klarer belegt. |
| Forcellini | [Tag v1.3 / Commit f7c5b64](https://github.com/latin-dict/Forcellini/tree/f7c5b6438a974557e6b22434671615faab6aa8f6) | Projekt nennt Public Domain; Anpassung CC0. | GoldenDict-Wörterbuchpaket | Historische Lemmata als Alternative; **nicht übernommen**, Paketextraktion und Bedeutungsprüfung bringen hier keinen Vorteil. | **2** – kein fertiges Komfortvokabular. |

## Entscheidung und Konzepttreue

Der aktuelle [A/B-Standard](../../crates/kairos-wordlists/data/curation-rubric.md) ersetzt „im Zweifel raus“.
Die [Stufe-2-Belege](../../crates/kairos-wordlists/data/curation-stage-two.md) trennen Semantik und technische
Reproduktion. Der historische STOP bleibt in der Diagnose zu `691860b` erhalten.

Zusätzlich gewählt ist Princeton WordNet 3.0 als englische Bedeutungsquelle.
[Originalarchiv](https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz),
[Original-Lizenz](https://wordnet.princeton.edu/license-and-commercial-use).
Der deterministische eigene Auszug bindet Definitionen, Wortart, Synset-Offset,
Lemma und EFF-Large-Mitgliedschaft. 8.924 von 9.558 Kandidaten haben Treffer,
634 bleiben explizite Lücken. Das sind Quellenbelege, keine automatische Freigabe.
Die gezielte EN-Restsuche durchsucht den gesamten erlaubten Bestand, beansprucht
aber keine menschliche Einzelabnahme jeder WordNet-Zuordnung.

Nach dem Merge von #2 basiert #7 auf main und setzt die deutsche Triage hier um.
DE hat 6.909 Wörter, EN/LA jeweils 6.897, die disjunkte Vereinigung
20.703. Vier unabhängig und gleichverteilt aus der Vereinigung gezogene Wörter ergeben bei festem Trenner **57,350 Bit**. Der Größenfaktor gegenüber DE ist **80,625**; der Sprachanteil ist bereits enthalten und darf nicht nochmals addiert werden.
Die Mittelwerte sind DE 6,577, EN 6,659, LA 6,988.

Quell- und Lizenzentscheidung: EFF CC BY 4.0, Wordnik MIT, Whitakers individuelle
permissive Freigabe und Princeton WordNet 3.0. Es besteht keine offene Copyleft-
Entscheidung für übernommene Daten. Keine fremde Implementierung wird in den
eigenen Generatorcode übernommen. Das originale WordNet-Archiv enthält auch
unbenutzte Upstream-Werkzeuge. Die Runtime verwendet ausschließlich die Endwörter.

Der Decoder und das Ranking bleiben begrenzt und deterministisch. Seltene
LA-Lemmata, morphologische Homographen und regionale EN-Bedeutungen sind
offengelegte Grenzen. Sämtliche verfügbaren LA-Glossen werden gebunden;
EN-NO_ENTRY-Zeilen bleiben sichtbar. Sprache bezeichnet den zugeordneten
Wortgebrauch. Vollständige allgemeine sprachliche Fehlerfreiheit wird nicht behauptet.

[Crate-README](../../crates/kairos-wordlists/README.md), [Auswahl und Prüfungen](../../crates/kairos-wordlists/data/languages/README.md),
[Provenienz und Hashes](../../crates/kairos-wordlists/data/languages/PROVENANCE.md), [Filterbericht](../../crates/kairos-wordlists/data/languages/report.txt),
[Originalglossen](../../crates/kairos-wordlists/data/languages/latin-glosses.tsv), [NOTICE](../../crates/kairos-wordlists/data/languages/NOTICE.txt).
Die spätere Release-Integration muss NOTICE mit der Anwendung ausliefern.

## Stufe 2: Bereit zur unabhängigen Prüfung (präzisierte Rubrik)

Der Lead stuft `reptile` als B ein: Eine Tier-/Gegenstandsmetapher bei neutraler
Hauptbedeutung erzwingt kein A. Nur sexuelle, vulgäre/obszöne oder Slur-Nebensinne
tun dies; die Hauptbedeutungen bleiben streng nach A-Klasse bewertet.
Bereits gesperrte Wörter bleiben gesperrt. Der frühere STOP-Grund ist damit
ersetzt, die zweite eigene Probe bleibt unvollständig. Keine neue semantische
Stichprobe durch den Implementierungsworker; ein frischer Verifier übernimmt
die unabhängige Prüfung. [Lead-Entscheid, Seeds und Prüfgrenzen](../../crates/kairos-wordlists/data/curation-stage-two.md).
