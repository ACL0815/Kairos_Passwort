# Englisch und Latein

PR-7, **Prio 4**, `feat/languages-en-la`, **gestapelt auf #2** (`feat/wordlist-de`
@ `f35f842d50436492b31d4dfa6a789dbb66112d6f`). Abruf/Stichproben: **25.10.2026**.
Drei reine Recherche-Kinder: `codex/gpt-6-luna`, `reasoningEffort=xhigh`,
`serviceTier=priority`; Themen Englisch, Latein und mehrsprachige Vereinigung.
Keine Weiterdelegation und keine Schreibzugriffe dieser Kinder. Die folgende
Bewertung ist die Implementierungsentscheidung nach Primärquellenprüfung.

Code- und Datenlizenzen werden getrennt betrachtet. **Kein fremder
Implementierungscode wurde kopiert**. Übernommen sind nur lizenzierte Wortdaten
und Lizenztexte. Die Passung bewertet Umfang, Lizenz, Abschreibbarkeit und
Aufbereitungsaufwand zusammen. Alle Zeilen wurden am 25.10.2026 abgerufen.

| Projekt | Permalink / Stand | Lizenz | Datei/Modul | Prinzip / Entscheidung | Passung 1–5 + Bewertung |
|---|---|---|---|---|---|
| EFF / Joseph Bonneau | [Long, 18.07.2016](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), [Short 1](https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt), [Short 2](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt), [Methodik, Juli 2016](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) | **Daten: CC BY 4.0 gewählt**, laut aktueller [EFF-Freigabe](https://www.eff.org/copyright) für EFF-Originalmaterial, sofern nicht anders angegeben; ältere xkcdpass-Kopien nennen CC BY 3.0. | Drei Rohlisten von 2016, 7.776 + 1.296 + 1.296 Zeilen | Bekannte schreibbare Wörter, Prüfung vor Auswahl. Vereinigung nach `a-z`, 3–9: 8.301 verschiedene Wörter. Eigene strengere Belastungsprüfung nötig. Datierte URLs sind keine unveränderlichen Git-Commits; lokale Originalbytes durch SHA-256 festgelegt. | **5** – beste Passphrasenbasis; kurze Ergänzungen nötig. Dreizeichenpräfixe/Editierabstand der separaten Kurzliste gelten nach Kombination nicht. |
| Wordnik | [Wortliste, Commit 46e6215](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/wordlist-20210729.txt), [README](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/README.md) | **MIT**, Copyright (c) 2020 Wordnik, [Lizenz](https://github.com/wordnik/wordlist/blob/46e6215d0f90356afe9c8ba4be347e7e98cb425c/LICENSE) | `wordlist-20210729.txt`, 198.422 Zeilen | Nur ausdrückliche Positivliste von 1.257 kurzen Wörtern; Mitgliedschaft gegen vollständige komprimierte Quelle geprüft. Keine automatische Aufnahme des Wortspiel-Wörterbuchs. | **4** – lizenzklarer Ergänzungspool; ungeprüft wegen seltenen/belastenden Wörtern ungeeignet. |
| Whitaker's WORDS | [Lexikon, Commit 1f2f0fb](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/DICTLINE.GEN), [Projekt](https://github.com/mk270/whitakers-words/tree/1f2f0fb0867a896d7b9284a03d615ed635d6f992) | **Individuelle permissive Freigabe für Programm und Daten**, [Originaltext](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/LICENCE.txt); keine CC0-/PD-Behauptung. | `DICTLINE.GEN`, 39.335 Datensätze; [Frequenz-/Grammatikbeschreibung](https://github.com/mk270/whitakers-words/blob/1f2f0fb0867a896d7b9284a03d615ed635d6f992/src/latin_utils/latin_utils-inflections_package.ads) | Gewählte Lateinquelle: Stämme, Grammatik, Frequenz und Bedeutungen. Begrenzter eigener Decoder für unterstützte Grundformen; 11.635 passende Kandidaten A–D vor Kuratierung. Keine Ada-Codeübernahme. | **4** – genug Daten und breite Weitergabefreigabe; Quelldatenfehler und ungewohnte Lemmata verlangen dokumentierte Grenzen. |
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

Latein hat eine brauchbare redistribuierbare Quelle mit ausreichend Wörtern.
Deshalb sind Esperanto/Italienisch als Ausweichsprachen nicht erforderlich.
Übernommen werden EFF (CC BY 4.0), ausgewählte Wordnik-Daten (MIT) und WORDS
(individuelle permissive Freigabe). Es gibt **keine offene Copyleft-Entscheidung**.
Originaltexte, Namensnennungen und Änderungshinweise liegen bei und müssen bei
einer Binärdistribution mitgeliefert werden. Die Crate-Software bleibt MIT OR
Apache-2.0; diese Lizenz ersetzt keine Datenlizenz.

Deutsch bleibt bytegleich bei 7.134 Wörtern. Nach Kuratierung entfernt Englisch
383 deutsche Überschneidungen; Latein entfernt 83 deutsche und 121 endgültig
englische. Vorrang ist Deutsch vor Englisch vor Latein. Die anschließend gleich
großen Endlisten haben **je 7.134 Wörter**. `combined()` enthält **21.402
verschiedene Schreibweisen** und erhält die Sprache jedes Eintrags. Für jede
Passwortposition unabhängig einen gleichverteilten Index aus der Vereinigung zu
ziehen erfüllt die Konzeptrechnung: Faktor **3⁴ = 81** bei vier Wörtern, gegenüber
einer Einzelquelle. Eine zusätzliche Sprachziehung ist bei diesen gleich großen
Listen unnötig. Der Sprachgewinn ist in `4 * log2(21402) = 57,542 Bit` mit festem
Trenner schon enthalten und darf nicht doppelt addiert werden.

Die kombinierte Ablage verschränkt Deutsch/Englisch/Latein deterministisch,
ohne Sprachblöcke. Dies ist keine geheime Zufallspermutation; bei gleichverteilter
Indexwahl ist die Reihenfolge entropieneutral. Jeder Wortaufruf liefert das
passende Sprachflag. Core, Profile und GUI werden in diesem Bereichs-PR nicht
außerhalb der Crate geändert.

Mittlere Länge: Englisch **6,282**, Latein **6,618**, Deutsch unverändert **6,564**.
Sechs Buchstaben bleiben ein Komfortziel. Latein enthält auch weniger bekannte
Lemmata. Begrenzter Decoder, Quelldatenfehler und semantische Nebenbedeutungen
bleiben dokumentierte Grenzen; vollständige sprachwissenschaftliche Prüfung,
allgemeine Präfixfreiheit und garantierter Editierabstand werden nicht behauptet.
Neutrale gewöhnliche Begriffe wie `pax`, `aqua` und `amor` bleiben erlaubt.

Belege: [Crate-README](../../crates/kairos-wordlists/README.md),
[Auswahl, Zahlen, Schnittmengen und Offline-Pipeline](../../crates/kairos-wordlists/data/languages/README.md),
[Provenienz/Lizenztexte/Hashes](../../crates/kairos-wordlists/data/languages/PROVENANCE.md),
[Filterbericht](../../crates/kairos-wordlists/data/languages/report.txt),
[lateinische Bedeutungen und Originalzeilen](../../crates/kairos-wordlists/data/languages/latin-glosses.tsv).
