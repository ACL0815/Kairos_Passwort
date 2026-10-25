# kairos-wordlists

Die deutsche Wortliste ist mit `include_str!` in die Anwendung eingebettet.
`german() -> &'static EmbeddedWordList` stellt `kairos_core::WordSource`
bereit: **7.299 eindeutige Wörter**, je Wort `Language::German`, nur `a`–`z`,
3–9 Buchstaben, durchschnittlich **6,468** Buchstaben. Ein `OnceLock` erzeugt
einmalig einen Index aus geliehenen Zeichenketten. Die Anwendung benötigt
weder Wortlistendateien noch Netzwerkzugriff. Der gemeinsame Core-Vertrag
bleibt unverändert.

```rust
use kairos_core::WordSource;

let words = kairos_wordlists::german();
assert_eq!(words.len(), 7_299);
assert_eq!(words.word(0), "aal");
```

## Auswahl und Grenzen

Die Basis sind die beiden dys2p-Listen `de-7776-v1.txt` und `de-2048-v1.txt`.
Nach dem Längenfilter und der Vereinigung bleiben 6.867 Wörter. Eine
redaktionelle Durchsicht entfernt 756 Einträge: belastende, vulgäre oder
beleidigende Begriffe, reine Eigennamen sowie auffällige Schreibfehler und
schwer verständliche Formen. Die Entscheidungen stehen einzeln mit Grund in
[`data/rejected.tsv`](data/rejected.tsv). Wörter mit gewöhnlicher Bedeutung,
die zugleich Namen sein können (etwa `wolf` oder `iris`), gelten als gewöhnliche
Wörter; reine Personen- oder Ortsnamen werden ausgeschlossen.

Die verbleibenden 6.111 Wörter werden um 1.188 einzeln ausgewählte Wörter aus
enz/german-wordlist ergänzt. Nur die Positivliste
[`data/supplement.txt`](data/supplement.txt) wird zugelassen; der große
Wörterbuchbestand wird **nicht vollständig** zum Passwortvokabular. Ergänzungen
sind auf höchstens sechs Buchstaben begrenzt. ASCII-Großbuchstaben des
Wörterbuchs werden kleingeschrieben; `ä`, `ö`, `ü`, `ß`, Akzente, Zahlen und
Satzzeichen bleiben ausgeschlossen. Es gibt **keine Transliteration**.

Bekannte Lehnwörter und übliche Wortformen sind enthalten, etwa `cloud`,
`biker`, `aale` und `gebogen`. Die Qualitätsbewertung ist redaktionell und
keine Garantie gegen jede regionale oder kontextuelle Nebenbedeutung. Neue
Meldungen werden durch Änderungen der Positiv-/Ausschlusslisten und erneute
Prüfung nachvollziehbar bearbeitet, nicht durch zusätzliche Laufzeitfilter.

Die Mindestgröße von Profil 0 ist erreicht. Die beispielhaften 21.000 Wörter
von Profil 1 werden bewusst nicht mit ungeprüften Wörterbuchresten aufgefüllt.
Die reale Größe muss in die Entropieberechnung eingehen: vier unabhängig
gleichverteilt gewählte Wörter ergeben bei festem Trenner rund 51,334 Bit.
Die durchschnittlichen 6,468 statt genau sechs Buchstaben sind der Kompromiss
zwischen gut schreibbaren Wörtern und ausreichendem Umfang.

Eindeutige Kurzpräfixe, Präfixfreiheit, ein garantierter Editierabstand und
automatische Tippfehlerkorrektur werden nicht zugesichert. Insbesondere
Wortformen teilen Präfixe. Der Generator muss Wortgrenzen mit Trennern
erhalten; diese Liste ist kein direktes Würfelwörterbuch mit genau `6^5`
Einträgen. Die gleichverteilte Indexwahl gehört zu `kairos-core`.

## Reproduzierbare Offline-Pipeline

Alle Rohdaten und Lizenztexte sind lokal vorhanden. Die 8,64-MB-Datei `words`
ist verlustfrei als `words.gz` abgelegt (rund 1,90 MB), damit nicht 685.829
Wörterbuchzeilen den Diff dominieren. Der Hash bezieht sich auf die
**dekomprimierten Originalbytes**, einschließlich originalem Zeilenende.
`.gitattributes` bewahrt die Rohdatenbytes auch auf Windows.

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-wordlists --example rebuild-german
cargo run -p kairos-wordlists --example rebuild-german -- --check
```

Das erste Kommando erzeugt `data/german.txt`, das zweite prüft ohne Schreiben.
Die kleine gemeinsame Implementierung unter
[`tools/pipeline.rs`](tools/pipeline.rs) wird vom Beispiel und den Tests
verwendet. Sie prüft zuerst die Rohquellen gegen
[`data/upstream/SHA256SUMS`](data/upstream/SHA256SUMS), filtert, wendet die
redaktionellen Entscheidungen an, validiert die Ergänzungen gegen deren
Rohquelle und schreibt ordinal sortierte Wörter als UTF-8 mit LF und genau
einem Wort pro Zeile. Ausgeschlossene Wörter dürfen auch über Ergänzungen
nicht zurückkehren. Fehlerhafte oder doppelte Kuratierungseinträge brechen
den Lauf ab.

| Stufe | Verbleibend | Veränderung |
|---|---:|---|
| dys2p, beide Rohlisten | 9.824 | 7.776 + 2.048 |
| Nur Kleinbuchstaben a–z | 9.824 | 0 entfernt |
| Länge 3–9 | 7.853 | 1.971 entfernt |
| Vereinigung, eindeutig | 6.867 | 986 doppelte Einträge entfernt |
| Redaktionelle Ausschlüsse | 6.111 | 756 entfernt |
| enz, Rohbestand | 685.829 | Nur Kandidatenpool |
| enz-Positivliste, 3–6 Buchstaben | 1.188 zusätzlich | Alle gegen die Rohquelle geprüft |
| Finale sortierte Liste | **7.299** | Keine Duplikate |

Längenverteilung: 3: 121; 4: 731; 5: 1.647; 6: 1.277; 7: 1.168;
8: 1.344; 9: 1.011.

Der freigegebene Ergebnis-Hash steht in
[`data/german.sha256`](data/german.sha256):

```text
74ec7d5ac9fa5f108df8d5b5d40f9f642358fc7aebb348e44cd3651769929845
```

Bei einer beabsichtigten Kuratierungsänderung: neu erzeugen, Diff der Wörter
prüfen, Anzahl/Statistik in Doku und Test anpassen und den Ergebnis-Hash nach
Prüfung aktualisieren. Die Pipeline aktualisiert diesen Prüfwert absichtlich
nicht selbst. Die Rohquellen-Hashes bleiben bei reiner Kuratierung unverändert.

## Herkunft und Datenlizenz

Abruf aller Daten: **25.10.2026**. Die ursprünglichen Lizenzdateien und
Projekt-READMEs liegen jeweils neben den Rohdaten. Die Software dieser Crate
bleibt unter MIT OR Apache-2.0; die Wortdaten werden separat unter **CC0-1.0**
genutzt. Es werden keine Copyleft-Wortdaten übernommen.

| Quelle und fester Stand | Übernommene Dateien | Gewählte Datenlizenz | Lokaler Lizenztext |
|---|---|---|---|
| [dys2p/wordlists-de, `6ef31b9aefb8735a7b066592393d12843ec502cd`](https://github.com/dys2p/wordlists-de/tree/6ef31b9aefb8735a7b066592393d12843ec502cd) | `de-7776-v1.txt`, `de-2048-v1.txt` | CC0-1.0; Upstream bietet alternativ Unlicense/BSD-3-Clause an | [LICENSE](data/upstream/dys2p/LICENSE) |
| [Markus Enzenberger und Beitragende, german-wordlist, `1071d87d9535b57b80ec076c699f2578a28ee9dc`](https://github.com/enz/german-wordlist/tree/1071d87d9535b57b80ec076c699f2578a28ee9dc) | `words` (komprimiert), nur die lokale Positivliste fließt ein | CC0-1.0 | [COPYING](data/upstream/enz/COPYING) |

Die vollständigen Download-Permalinks und SHA-256-Werte stehen in
[`data/PROVENANCE.md`](data/PROVENANCE.md). Die Auswahl und Zusammenführung
sind Änderungen durch das Kairos-Projekt, keine unveränderte Upstream-Liste.

## Open-Source-Vorbilder

- **dys2p/wordlists-de:** deutsche ASCII-Wörter und redaktionelle
  Ausschlussregeln als Grundlage; eigene zusätzliche Durchsicht und Längengrenze.
- **enz/german-wordlist:** lizenzklarer Kandidatenpool; nur ausdrückliche
  Auswahl, weil ein Spielwörterbuch andere Qualitätsziele hat.
- **xkcdpass:** Zeichensatz, Länge und Duplikate vor der Auswahl behandeln;
  keine Generatorlogik und kein fremder Implementierungscode übernommen.
- **pwgen-go:** eingebettete Listen als eigene Datenquelle vom Generator
  trennen. Rust nutzt dafür `include_str!` und `OnceLock`, keine generierten
  Quellcodearrays und keine zusätzliche Laufzeitabhängigkeit.
- **passalong-core 0.3.0:** Text einmal mit `OnceLock` indexieren und
  Wortreferenzen statt einzelner String-Allokationen halten; nur dieses Prinzip.
- **EFF/Diceware:** Schreibbarkeit und redaktionelle Prüfung als Vorbild;
  keine englischen Daten und keine Präfix-/Editierabstandsgarantie übernommen.

Feste Quellstände, dateispezifische Lizenzen, Bewertung und verworfene
Alternativen: [`docs/sources/wordlists.md`](../../docs/sources/wordlists.md).
`sha2` und `flate2` sind MIT/Apache-2.0-Abhängigkeiten ausschließlich für
Entwicklungswerkzeuge und Tests. Der Laufzeitpfad benötigt nur `kairos-core`
und die Standardbibliothek.

## Tests

`cargo test -p kairos-wordlists` prüft die reale Wortzahl, Eindeutigkeit,
Zeichensatz, Längen, durchschnittliche Länge, Sprache jedes Eintrags, fehlende
Leerzeilen, Sortierung, ausgeschlossene Wörter, beide Indexgrenzen,
threadsichere gemeinsame Initialisierung sowie Gleichheit mit der Pipeline
und den Ergebnis-Hash. Die Pipeline prüft dabei auch alle Rohquellen-Hashes.
