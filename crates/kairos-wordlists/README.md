# kairos-wordlists

Die deutsche Wortliste ist mit `include_str!` in die Anwendung eingebettet.
`german() -> &'static EmbeddedWordList` stellt `kairos_core::WordSource`
bereit: **7.130 eindeutige Wörter**, je Wort `Language::German`, nur `a`–`z`,
3–9 Buchstaben, durchschnittlich **6,565** Buchstaben. `OnceLock` erzeugt
einmalig einen Index aus geliehenen Zeichenketten. Die Anwendung benötigt
weder Wortlistendateien noch Netzwerkzugriff. Der Core-Vertrag bleibt unverändert.

```rust
use kairos_core::WordSource;
let words = kairos_wordlists::german();
assert_eq!(words.len(), 7_130);
assert_eq!(words.word(0), "aal");
```

## Auswahl und redaktionelle Regeln

Die Basis sind dys2p `de-7776-v1.txt` und `de-2048-v1.txt`. Nach Zeichen- und
Längenfilter sowie Deduplizierung bleiben 6.867 Wörter. Die redaktionellen
Entscheidungen stehen in [`data/rejected.tsv`](data/rejected.tsv): insgesamt
1.306 Sperren aus beiden Quellen, davon greifen 782 im Basisbestand.
Die Pipeline prüft genau fünf Kategorien:

| Kategorie | Bedeutung | Anzahl |
|---|---|---:|
| vulgär | Grobe/vulgäre Ausdrücke | 2 |
| beleidigend | Herabsetzende Ausdrücke | 17 |
| belastend | Insbesondere Gewalt, Krankheit, Tod und Rauschmittel | 707 |
| Eigenname | Reine Personen-, Orts- oder andere Eigennamen | 11 |
| Schreibbarkeit | Fehler, schwierige, seltene oder ungebräuchliche Schreibweisen | 569 |

Diese Kategorien sind redaktionelle Entscheidungen, keine Bewertung von
Menschen oder Weltanschauungen. `atheismus`, `glaube` und `engel` bleiben
beispielsweise gleichberechtigt enthalten. Gewöhnliche Wörter, die zugleich
Namen sein können (etwa `wolf` oder `iris`), gelten als gewöhnliche Wörter.

Zu den zunächst verbleibenden 6.085 Basiswörtern kommen 1.966 ausgewählte
Kandidaten aus enz/german-wordlist. Nur diese
[`Positivliste`](data/supplement.txt) wird zugelassen, der große Rohbestand
wird nicht vollständig zum Passwortvokabular. Ergänzungen sind auf sieben
Buchstaben begrenzt. Im Original muss ein Wort kleingeschrieben sein oder
nur einen großen Anfangsbuchstaben haben; reine Großbuchstaben-Abkürzungen
werden verworfen. Diese Prüfung ersetzt keine redaktionelle Namensprüfung.
ASCII-Großbuchstaben werden kleingeschrieben. `ä`, `ö`, `ü`, `ß`, Akzente,
Zahlen und Satzzeichen werden ausgeschlossen. Es gibt **keine Transliteration**.

Ein Endungsfilter entfernt anschließend 921 nahe Schreibvarianten. Er
verarbeitet kürzere Wörter zuerst, bei gleicher Länge ordinal. Existiert
bereits die kürzere Schreibweise, entfallen längere Varianten mit angehängtem
`e`, `n`, `s`, `en`, `er`, `es`, `em` oder `r`. So bleibt etwa `tag`, aber
nicht `tage`, `tags` oder `tages`. Dies ist eine konservative Schreibhilfe,
keine linguistische Behauptung über gemeinsame Lemmata: auch unabhängige
Wörter wie `auto`/`autor` können betroffen sein. Das spart ein eigenes
Morphologie-System. Übrig bleiben **5.453 Basiswörter und 1.677 Ergänzungen**.

Bekannte deutsche Lehnwörter und alleinstehende Wortformen bleiben enthalten,
etwa `cloud`, `laptop` und `gebogen`. Reine englische Ergänzungen wie `first`,
`range`, `page` und `type` wurden gestrichen. Für die spätere englische Liste
müssen **alle bereits deutschen Einträge vor der Freigabe entfernt werden**;
weitere Sprachlisten müssen ebenfalls paarweise disjunkt sein. Das Sprachflag
bezeichnet den deutschen Wortgebrauch, nicht die historische Herkunft.
Überlappende Sprachen liefern keinen zusätzlichen Entropiebonus.

Die Bewertung ist redaktionell und keine Garantie gegen jede regionale oder
kontextuelle Nebenbedeutung. Änderungen bleiben über die versionierten
Positiv-/Ausschlusslisten und den Endungsfilter nachvollziehbar. Eindeutige
Kurzpräfixe, allgemeine Präfixfreiheit, ein garantierter Editierabstand und
automatische Tippfehlerkorrektur werden nicht zugesichert. Feste Trenner
halten die Wortgrenzen eindeutig; dies ist kein direktes Würfelwörterbuch
mit genau `6^5` Einträgen. Gleichverteilte Indexwahl gehört zu `kairos-core`.

Die D1-Nachprüfung sperrt `drink`, `bowle`, `hanf` und `prost` dauerhaft
über die Pipeline-Eingaben. Die systematische Suche und die Bewertung
neutraler Teilworttreffer stehen im [Kuratierungsbericht](data/curation-review.md).

Die Mindestgröße von Profil 0 ist erreicht. Die beispielhaften 21.000 Wörter
von Profil 1 werden nicht mit ungeprüften Resten aufgefüllt. Vier unabhängig
gleichverteilt gezogene Wörter ergeben bei festem Trenner rund **51,199 Bit**.
Die durchschnittlichen **6,565 statt sechs Buchstaben** sind eine begründete
Zielabweichung: unterscheidbare Wörter und Grundformen haben Vorrang vor
kurzen Flexionsformen. 2.243 Wörter (31,4 %) haben acht oder neun Buchstaben.
Eine weitere Kürzung dieses Anteils wird nicht durch seltene Kurzformen oder
eine Wortzahl unter dem Mindestumfang erkauft.

## Reproduzierbare Offline-Pipeline

Alle Rohdaten und Lizenztexte sind lokal vorhanden. Die 8,64-MB-Datei `words`
ist verlustfrei als `words.gz` abgelegt (rund 1,90 MB). Der Quellhash bezieht
sich auf die **dekomprimierten Originalbytes** einschließlich Zeilenende.
`.gitattributes` bewahrt die Rohdatenbytes auch auf Windows.

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-wordlists --example rebuild-german
cargo run -p kairos-wordlists --example rebuild-german -- --check
```

Das erste Kommando erzeugt `data/german.txt`, das zweite prüft ohne Schreiben.
[`tools/pipeline.rs`](tools/pipeline.rs) ist die gemeinsame kleine
Implementierung für Werkzeug und Tests. Sie prüft zuerst die Rohquellen
gegen [`data/upstream/SHA256SUMS`](data/upstream/SHA256SUMS), filtert Zeichen,
Länge und Duplikate, validiert die Kuratierung und Originalschreibweisen,
wendet Sperren und Endungsfilter an und schreibt UTF-8 mit LF, genau einem
Wort pro Zeile und ordinaler Sortierung. Sperren gelten für beide Quellen;
sie können auch nur im enz-Bestand vorkommende Wörter dauerhaft sperren.
Unbekannte Kategorien, unbekannte oder doppelte Sperren sowie fehlerhafte
oder doppelte Ergänzungen brechen den Lauf ab.

| Stufe | Verbleibend | Veränderung |
|---|---:|---|
| dys2p, beide Rohlisten | 9.824 | 7.776 + 2.048 |
| Nur Kleinbuchstaben a–z | 9.824 | 0 entfernt |
| Länge 3–9 | 7.853 | 1.971 entfernt |
| Vereinigung, eindeutig | 6.867 | 986 Duplikate entfernt |
| Redaktionell bereinigte Basis | 6.085 | 782 Basis-Ausschlüsse |
| enz, Rohbestand | 685.829 | Nur Kandidatenpool |
| enz-Positivliste, 3–7 Buchstaben | 1.966 zusätzlich | Originalschreibweisen und Sperren geprüft |
| Vereinigte Vorauswahl | 8.051 | Keine Duplikate |
| Endungsfilter | **7.130** | 921 Schreibvarianten entfernt |
| Finale sortierte Liste | **7.130** | 5.453 Basiswörter + 1.677 Ergänzungen |

Längenverteilung: 3: 119; 4: 632; 5: 1.299; 6: 1.351; 7: 1.486;
8: 1.263; 9: 980.

Der freigegebene Ergebnis-Hash steht in [`data/german.sha256`](data/german.sha256):

```text
c740ce05d355b2296c2feb940733f84f9bc3e1972fcfad9cbfdf5e4236bf3175
```

Bei einer beabsichtigten Kuratierungsänderung: neu erzeugen, Wortdiff prüfen,
Anzahl/Statistik in Doku und Test anpassen und den Ergebnis-Hash nach Prüfung
aktualisieren. Die Pipeline aktualisiert den Prüfwert absichtlich nicht selbst.
Die Rohquellen-Hashes bleiben bei reiner Kuratierung unverändert.

Die sortierte Ablage dient Reproduzierbarkeit und Review. Das im Grundkonzept
genannte spätere randomisierte Speichern betrifft die künftige Speicherung;
bei gleichverteilter Indexwahl ändert die heutige Sortierung weder Auswahl-
wahrscheinlichkeit noch Entropie. Die vollständige komprimierte Rohquelle
bleibt bewusst erhalten, weil ein Auszug mit Download-Link die vollständige
Offline-Mitgliedsprüfung nicht leisten würde. `flate2` wird nicht als
Laufzeitabhängigkeit oder Entpacker in die EXE eingebunden.

## Herkunft und Datenlizenz

Abruf: **25.10.2026**. Original-Lizenzdateien und Projekt-READMEs liegen neben
den Rohdaten. Crate-Software: MIT OR Apache-2.0. Wortdaten: separat **CC0-1.0**.
Es werden keine Copyleft-Wortdaten übernommen.

| Quelle und fester Stand | Dateien | Gewählte Datenlizenz | Lokaler Lizenztext |
|---|---|---|---|
| [dys2p/wordlists-de, `6ef31b9aefb8735a7b066592393d12843ec502cd`](https://github.com/dys2p/wordlists-de/tree/6ef31b9aefb8735a7b066592393d12843ec502cd) | `de-7776-v1.txt`, `de-2048-v1.txt` | CC0-1.0; alternativ bietet Upstream Unlicense/BSD-3-Clause | [LICENSE](data/upstream/dys2p/LICENSE) |
| [Markus Enzenberger und Beitragende, german-wordlist, `1071d87d9535b57b80ec076c699f2578a28ee9dc`](https://github.com/enz/german-wordlist/tree/1071d87d9535b57b80ec076c699f2578a28ee9dc) | `words` (komprimiert) | CC0-1.0 | [COPYING](data/upstream/enz/COPYING) |

Original-Download-Permalinks und SHA-256-Werte stehen in
[`data/PROVENANCE.md`](data/PROVENANCE.md). Auswahl, Zusammenführung und
Endungsfilter sind Änderungen des Kairos-Projekts.

## Open-Source-Vorbilder

- **dys2p/wordlists-de:** deutsche ASCII-Basis und redaktionelle Auswahlregeln;
  zusätzliche Durchsicht, Längengrenze und Endungsfilter in Kairos.
- **enz/german-wordlist:** lizenzklarer Kandidatenpool, nur explizite Auswahl,
  da Wortspiele andere Qualitätsziele als Passphrasen haben.
- **xkcdpass:** Zeichen-/Längenfilter und Deduplizierung vor Auswahl; keine
  Generatorlogik und kein fremder Implementierungscode übernommen.
- **pwgen-go:** eingebettete Listen getrennt vom Generator halten; Rust nutzt
  `include_str!` statt generierter Quellcodearrays.
- **passalong-core 0.3.0:** Text mit `OnceLock` einmal indexieren und
  Wortreferenzen statt einzelner String-Allokationen halten; nur dieses Prinzip.
- **EFF/Diceware:** Schreibbarkeit und manuelle Prüfung; keine englischen
  Daten und keine Präfix-/Editierabstandsgarantie übernommen.

Feste Quellstände, dateispezifische Lizenzen und Bewertung:
[`docs/sources/wordlists.md`](../../docs/sources/wordlists.md).
`sha2` und `flate2` sind MIT/Apache-2.0-Abhängigkeiten ausschließlich für
Werkzeuge und Tests; zur Laufzeit genügen Core und Standardbibliothek.

## Tests

Elf Integrationstests und ein Rustdoc-Test prüfen mit
`cargo test -p kairos-wordlists` Anzahl, Eindeutigkeit, Zeichensatz, Längen,
Mittelwert, Sprache jedes Eintrags, Leerzeilen, Sortierung, Sperren,
Indexgrenzen, threadsichere Initialisierung, sortierte/eindeutige
Kuratierungseingaben und gültige Kategorien. Sie prüfen außerdem die
Originalschreibweise ohne Großbuchstaben-Abkürzungen, fehlende
Endungsnachbarn und eine unabhängige redaktionelle Regressionsliste.
Pipelinegleichheit, Ergebnis-Hash und sämtliche Rohquellen-Hashes werden
im Offline-Lauf geprüft. Der Mittelwert darf höchstens 6,6 betragen; die
engere ±0,5-Spanne um sechs wird zugunsten der erläuterten Grundformauswahl
nicht erzwungen.
