# Eingebettete deutsche Wortliste

Ursprüngliche Umsetzung in **PR #2** (`feat/wordlist-de`), inzwischen gemergt.
Die deutsche A/B-Nachtriage der 299 Meldungen erfolgt gemäß Auftrag in PR #7.
[Standard und aktueller Prüfstand](../../crates/kairos-wordlists/data/curation-stage-two.md).
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

Die ursprüngliche Umsetzung aus PR #2 ist gemergt. PR #7 übernimmt main und
setzt die A/B-Nachtriage um. Von 299 Meldungen werden 224 zur Sperrung adjudiziert
und 75 als B beibehalten; konservative Grenzen sind im Einzelbericht kenntlich.
Nach der Filteränderung sichtbare Formen wurden zusätzlich geprüft.
[Rubrik und Belege](../../crates/kairos-wordlists/data/curation-stage-two.md).

Von 9.824 Rohzeilen bleiben 6.867 passende eindeutige dys2p-Kandidaten. Die
Sperrliste enthält 1.566 Wörter, davon greifen 982 in der Basis. 5.885 kuratierte
Basiswörter plus 1.906 explizite enz-Ergänzungen ergeben nach 882 entfernten
Endungsvarianten **6.909 Wörter**. Davon stammen 5.283 aus der Basis und
1.626 aus der Ergänzung. Der Mittelwert beträgt **6,577 Buchstaben**.

Die Daten sind CC0, die Rohquellen und Original-Lizenzen bleiben unverändert.
Der große enz-Rohbestand ist kein ungeprüfter Passwortpool. ASCII-Regeln und
die begrenzte Endungsheuristik bleiben erhalten. Die A-Sperren wirken auf
Pipeline-Eingaben, nicht durch Handänderungen der Endliste. Vollständige
Wortdeltas und Nachrücker sind unter der Stufe-2-Dokumentation verlinkt.

Die Runtime indiziert eingebetteten Text einmalig mit OnceLock. Nur
Entwicklungswerkzeuge brauchen flate2/sha2. Zufallsauswahl gehört zu Core.
Keine allgemeine Präfixfreiheit, kein Mindest-Editierabstand und keine automatische
Tippkorrektur werden zugesichert. Trenner halten Wortgrenzen eindeutig.

[Aktuelles README](../../crates/kairos-wordlists/README.md), [Provenienz](../../crates/kairos-wordlists/data/PROVENANCE.md),
[299er-Triage](../../crates/kairos-wordlists/data/languages/stage2-german-triage.tsv),
[Nachrücker](../../crates/kairos-wordlists/data/languages/stage2-german-followup.tsv).
