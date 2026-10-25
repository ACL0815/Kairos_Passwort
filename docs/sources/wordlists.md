# Eingebettete deutsche Wortliste

Zuständig: **nur PR-2** (`feat/wordlist-de`), Prio 1, Basis `main` @ `42a2137`.
Abruf und Bewertung: **25.10.2026**. Die Umsetzung übernimmt ausschließlich
Wortdaten unter CC0-1.0; fremder Implementierungscode wurde nicht kopiert.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| dys2p/wordlists-de | [Rohlisten und Regeln, Commit `6ef31b9`](https://github.com/dys2p/wordlists-de/tree/6ef31b9aefb8735a7b066592393d12843ec502cd) | **Daten:** Unlicense ODER CC0-1.0 ODER BSD-3-Clause, siehe [LICENSE](https://github.com/dys2p/wordlists-de/blob/6ef31b9aefb8735a7b066592393d12843ec502cd/LICENSE); gewählt CC0 | `de-7776-v1.txt`, `de-2048-v1.txt`, `README.md` | Kuratierte deutsche ASCII-Wörter als Ausgangsbasis; kurze bekannte Wörter und Ausschluss reiner Eigennamen/negativer Begriffe. Upstream bezeichnet die großen Listenregeln ausdrücklich als nicht lückenlos erfüllt. | 5 |
| enz/german-wordlist | [`words`, Commit `1071d87`](https://github.com/enz/german-wordlist/blob/1071d87d9535b57b80ec076c699f2578a28ee9dc/words) | **Daten:** CC0-1.0, [COPYING](https://github.com/enz/german-wordlist/blob/1071d87d9535b57b80ec076c699f2578a28ee9dc/COPYING) | `words`, `README.md` | Lizenzklarer Kandidatenpool, ein Wort pro Zeile; Scrabble-orientierte Regeln grenzen Namen und Abkürzungen ein. Die Maintainer warnen vor Fehlern, daher nur redaktionell gewählte Ergänzungen. | 4 |
| xkcdpass | [`generate_wordlist`, Commit `b981361`](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py) | **Code:** BSD-3-Clause. **Daten:** `ger-long`/`ger-short` laut [README](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/README.rst) von dys2p unter BSD-3; `ger-anlx` aus GPL-v3-Quelle; andere Listen separat | `generate_wordlist`, `xkcdpass/static/` | Zeichen-/Längenfilter und Deduplizierung vor der Zufallsauswahl; reale Quellgröße für Sicherheitswerte. Datenlizenzen pro Liste behandeln. | 5 |
| diceware / Uli Fouquet | [Wortliste, Commit `89534ea`](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/diceware/wordlists/wordlist_de_8k.txt) | **Code:** GPL-3.0-or-later. **Dateien `wordlist_de.txt` und `wordlist_de_8k.txt`:** CC-BY-3.0 laut [COPYRIGHT](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/COPYRIGHT); keine Übernahme | `wordlist_de_8k.txt` | Einfaches Listenformat und transparente Quellgröße; sichtbare Eigennamen und unpassende Einträge der 8k-Liste verhindern eine ungeprüfte Übernahme. | 3 |
| pwgen-go | [Generator, Commit `5a53dce`](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist/generate/main.go) | **Code:** MIT, [LICENSE](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/LICENSE). **Daten:** englische EFF-Daten, unabhängig vom Code; nicht übernommen | `internal/wordlist/eff_long.go`, `wordlist.go`, `generate/main.go` | Generierte Stringarrays machen Wörter ohne Laufzeitdatei verfügbar und halten die Daten getrennt vom Generator. Der geprüfte Stand nutzt generierten Go-Code, kein `go:embed`. | 4 |
| passalong-core 0.3.0 | [`crypto/words.rs`, feste Crate-Version](https://docs.rs/passalong-core/0.3.0/src/passalong_core/crypto/words.rs.html) | **Code:** Apache-2.0 ([Crate](https://docs.rs/crate/passalong-core/0.3.0)); englische EFF-Daten nicht übernommen | `LIST`, `word_list()` | `include_str!` plus `OnceLock<Vec<&'static str>>` indexiert eingebettete Wörter einmalig und ohne einzelne Wortallokationen. | 5 |
| Password Tech | [Quellstand `c77cade`](https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29) | **Code:** GPL-2.0-or-later ([Lizenz](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/license.txt)); separate deutsche Download-Wortliste hier nicht lizenzverifiziert und nicht übernommen | `src/passw/`, Wörterbuchverarbeitung | Datenquellen von der Passworterzeugung trennen und Unicode beim Import berücksichtigen. Für den eingebetteten deutschen ASCII-Bestand sind weitere Importfunktionen unnötig. | 3 |
| EFF | [Deep Dive, datierter Artikel Juli 2016](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) (Dokument, kein Git-Stand) | Artikel: CC-BY-4.0 gemäß [EFF-Richtlinie](https://www.eff.org/copyright); **keine Wortdaten übernommen** | Lange Liste; separate kurze Liste mit speziellen Präfixen | 3–9 Buchstaben, Schreibbarkeit und manuelle Prüfung sensibler Begriffe. Eindeutige Drei-Zeichen-Präfixe und Mindest-Editierabstand 3 gehören zur separaten kurzen Liste. | 5 |
| Rust-Standardbibliothek 1.80.0 | [`std/sync/once_lock.rs`, Tag 1.80.0](https://github.com/rust-lang/rust/blob/1.80.0/library/std/src/sync/once_lock.rs) | MIT OR Apache-2.0; nur Standard-API genutzt | `OnceLock`, außerdem eingebautes `include_str!` | Thread-sichere einmalige Initialisierung; kein Buildskript und keine weitere Laufzeit-Crate für den Index nötig. | 5 |

Weitere geprüfte Datenalternativen, nicht übernommen:

| Projekt | Permalink / Stand | Datenlizenz | Datei/Modul | Prinzip und Ablehnungsgrund | Passung 1–5 |
|---|---|---|---|---|---|
| klamann/diceware-dereko | [Commit `e193146`](https://github.com/klamann/diceware-dereko/tree/e193146a24218ed5377cb0a27d82dd8dcc805e4) | CC-BY-NC-3.0 als Upstream-Bedingung; die bedingte Public-Domain-Erklärung des Autors hebt diese nicht verlässlich auf ([README](https://github.com/klamann/diceware-dereko/blob/e193146a24218ed5377cb0a27d82dd8dcc805e4/README.md)) | `alternatives/diceware-dereko-limit-8.txt` | Frequenzbasierte Auswahl mit kurzer Variante ist interessant; NC-Bedingungen verhindern eine uneingeschränkte Distribution im Projekt. | 2 |
| Wiktionary | [Hinweise zur Weiternutzung](https://de.wiktionary.org/wiki/Hilfe:Hinweise_f%C3%BCr_Leser), Recherche 25.10.2026; kein übernommener Dump | CC-BY-SA, konkrete Dump-Lizenz vor Verwendung zu prüfen | Artikeldaten, kein ausgewähltes Listenartefakt | Wortarten und Lemmata wären hilfreich. Eigenständige Extraktion und Copyleft-Daten sind unnötig, da zwei brauchbare CC0-Quellen vorhanden sind. | 1 |
| Leipzig-Frequenzlisten | [Offizielles Korpusportal](https://wortschatz.uni-leipzig.de/en/download), Recherche 25.10.2026; kein übernommenes Korpus | Korpusabhängig; keine pauschale Lizenzübertragung auf Wortdaten | Wortfrequenzen einzelner Korpora | Häufigkeit wäre ein Zusatzsignal; keine hier ausgewählte, lizenzverifizierte und redaktionell geeignete Liste benötigt. Nicht mit IDS/DeReKo gleichsetzen. | 2 |
| OpenThesaurus | [Offizielle Download-/Lizenzseite](https://www.openthesaurus.de/about/download), Recherche 25.10.2026; Export-URL veränderlich | Daten wahlweise CC-BY-SA-4.0 oder LGPL laut Downloadseite; keine Daten übernommen | `openthesaurus.txt` | Synonymgruppen liefern viele Wörter, benötigen aber weitere Kuratierung und haben gegenüber vorhandenen CC0-Quellen keinen Vorteil. | 2 |

Veränderliche Portale sind hier ausdrücklich nur verworfene Alternativen;
sie sind keine Eingaben der reproduzierbaren Pipeline. Deren drei Eingaben
sind sämtlich auf vollständige Commit-IDs und SHA-256 festgelegt.

## Entscheidung und eigene Umsetzung

Die D1-Nachbesserung aus Verifier A sperrt `drink`, `bowle`, `hanf` und
`prost` über die bestehende Pipeline. Vollbestandssuche, neutrale
Vergleichsfälle und aktualisierte Stufenzahlen stehen im
[Kuratierungsbericht](../../crates/kairos-wordlists/data/curation-review.md).

**Übernommen:** die CC0-Daten von dys2p und eine explizite Auswahl aus dem
CC0-Wörterbuch von Markus Enzenberger und Beitragenden. Die ungeänderten
Rohquellen, festen Revisionen, Original-Lizenztexte und SHA-256-Werte sind
in der Crate vorhanden. Der große enz-Bestand liegt platzsparend als Gzip
vor; der Hash wird auf den dekomprimierten Originalbytes geprüft.

Aus 9.824 dys2p-Rohzeilen werden nach `a-z`, Länge 3–9 und Deduplizierung
6.867 Wörter. Aus beiden Quellen sind 1.306 Einträge kategorisiert gesperrt;
782 Sperren greifen in der Basis, sodass 6.085 Wörter übrig bleiben.
1.966 gegen die enz-Rohquelle geprüfte Ergänzungskandidaten kommen hinzu.
Ein konservativer Endungsfilter entfernt 921 Schreibvarianten. Das Ergebnis:
**7.130 Wörter**, durchschnittlich **6,565 Buchstaben**, davon 5.453 aus der
Basis und 1.677 Ergänzungen. Die Ergänzungen dürfen höchstens sieben
Buchstaben haben. Großgeschriebene
Wörter werden nur ASCII-kleingeschrieben; Umlaute/ß werden ausgeschlossen
und niemals umgeschrieben. Jedes Laufzeitwort liefert `Language::German`.

Die kleine Rust-Pipeline nutzt Standard-Sets zur Sortierung/Deduplizierung;
`flate2` und `sha2` sparen eigene Kompressions-/Prüfsummenimplementierungen
und sind nur Entwicklungsabhängigkeiten. Der Laufzeitcode nutzt das bei
passalong-core verifizierte Prinzip aus eingebettetem Text und einmaligem
Index; der Algorithmus wurde eigenständig formuliert.

**Nicht übernommen:** fremder Implementierungscode, Generatorfunktionen,
ungeprüfte Wörterbuchreste, xkcdpass-GPL-Daten und Daten mit ungeklärter
Lizenz. Ein Buildskript oder PHF würde für `word(index)` zusätzlichen Code
ohne nötigen Nutzen bringen. Es gibt weder eine Rückwärtssuche noch einen
Grund für einen perfekten Hash. Die 21.000 Wörter aus dem Konzept bleiben
ein Beispiel; dieser PR dokumentiert den kleineren, geprüften Bestand.

**Bewusste Grenzen:** Präfixfreiheit, eindeutige Drei-Zeichen-Präfixe und
ein Mindest-Editierabstand werden nicht zugesichert. Die Zusammenführung
und kurze Wortformen heben solche Eigenschaften der Ursprungslisten auf.
Feste Trenner halten Wortgrenzen beim Abschreiben eindeutig. Redaktionelle
Kuratierung bildet keine vollständige Klassifikation aller möglichen
Nebenbedeutungen; gewöhnliche Wörter mit Namenshomographen bleiben erlaubt.

## Entscheidungen nach dem unabhängigen Review

Der Endungsfilter bevorzugt kürzere Schreibweisen gegenüber Varianten mit
`e`, `n`, `s`, `en`, `er`, `es`, `em` oder `r`. Er verhindert beispielsweise
`tag/tage/tages`. Er ist keine linguistische Stammworterkennung und kann
vorsorglich auch unabhängige Wörter wie `auto/autor` ausdünnen. Ein eigenes
Morphologie-System wäre hier unverhältnismäßig. Die geringfügig höhere
mittlere Länge ist offengelegt; wir erreichen die Mindestgröße nicht durch
zusätzliche kurze Flexionsformen. 31,4 % der Wörter haben acht oder neun
Buchstaben; Ziel sechs ist mit 6,565 nicht exakt erreicht und begründet.

Die Ausschlusskategorien `vulgär`, `beleidigend`, `belastend`, `Eigenname` und
`Schreibbarkeit` sind in der Pipeline festgelegt. Sie gelten für beide
Quellen und werden gegen deren Vereinigung geprüft. Weltanschauungen werden
nicht als Belastung klassifiziert: `atheismus`, `glaube` und `engel` bleiben
enthalten. Beanstandete Rauschmittel-/Gewaltbegriffe und problematische
Schreibvarianten sind zusätzlich durch unabhängige Regressionstests gesperrt.

Nur originale Kleinschreibung oder ein großer Anfangsbuchstabe werden für
enz-Kandidaten anerkannt, keine Großbuchstaben-Abkürzungen. Das bestätigt
keine Bedeutung automatisch; diese bleibt eine redaktionelle Entscheidung.
Reine englische Ergänzungen wurden gestrichen. Bekannte deutsche Lehnwörter
bleiben erlaubt; **PR-7 muss seine englischen und weiteren Sprachlisten um
alle bereits deutschen Einträge bereinigen**, bevor eine Sprachmischung
freigegeben wird. Gleichlautende Wörter dürfen keinen Sprachbonus erzeugen.

Ordinale Ablage ermöglicht reproduzierbare Diffs. Die spätere randomisierte
Speicherung aus dem Grundkonzept ist keine Voraussetzung dieser eingebetteten
Quelle; bei gleichverteilter Indexwahl ist die Reihenfolge entropieneutral.
Die komprimierte vollständige Rohquelle bleibt für Offline-Reproduktion
erhalten. Ein Auszug würde diese Prüfung schwächen, ohne die lizenzierte
Originalquelle lokal belegbar zu halten. Eine strengere Mittelwertgrenze
von 6,5 wird zugunsten der erläuterten Auswahl nicht erzwungen; der Test
sichert maximal 6,6 und die reale Wortzahl sowie die Endungsregel ab.

API, vollständiger Filterbericht, Tests und alle lokalen Belege:
[`kairos-wordlists/README.md`](../../crates/kairos-wordlists/README.md),
[`Provenienz`](../../crates/kairos-wordlists/data/PROVENANCE.md),
[`Ausschlüsse mit Grund`](../../crates/kairos-wordlists/data/rejected.tsv),
[`Positivliste`](../../crates/kairos-wordlists/data/supplement.txt),
[`Pipeline`](../../crates/kairos-wordlists/tools/pipeline.rs).
