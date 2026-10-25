# Provenienz der deutschen Wortdaten

Abrufdatum: 2026-10-25. Datenlizenz der übernommenen Wortdaten: CC0-1.0.
Die Lizenzentscheidung betrifft diese Dateien unabhängig von der Lizenz des
Generators oder dieser Rust-Crate.

| Lokaler Pfad (unter `data/`) | Original-Download am festen Commit | SHA-256 der Originalbytes |
|---|---|---|
| `upstream/dys2p/de-7776-v1.txt` | [Rohquelle](https://raw.githubusercontent.com/dys2p/wordlists-de/6ef31b9aefb8735a7b066592393d12843ec502cd/de-7776-v1.txt) | `440fa02c65591328d6351435d3824c27b483a049f4eca0b13456d8c5090442e7` |
| `upstream/dys2p/de-2048-v1.txt` | [Rohquelle](https://raw.githubusercontent.com/dys2p/wordlists-de/6ef31b9aefb8735a7b066592393d12843ec502cd/de-2048-v1.txt) | `7965dc8c6b413ccb635d3021043365e18df0367bf5413a50a069a98addfe4e1d` |
| `upstream/enz/words.gz` (dekomprimiert: `words`) | [Rohquelle](https://raw.githubusercontent.com/enz/german-wordlist/1071d87d9535b57b80ec076c699f2578a28ee9dc/words) | `20095928ca70974fb5d86e44f10181cee16e2831b0d4b1a4ddd12c261aebc5e9` |

Die unveränderten Original-Lizenztexte:

- [dys2p LICENSE](upstream/dys2p/LICENSE), einschließlich vollständigem CC0-Text;
  [Original](https://github.com/dys2p/wordlists-de/blob/6ef31b9aefb8735a7b066592393d12843ec502cd/LICENSE).
  Das Projekt bietet Unlicense, CC0 und BSD-3 zur Wahl; Kairos wählt CC0.
- [enz COPYING](upstream/enz/COPYING);
  [Original](https://github.com/enz/german-wordlist/blob/1071d87d9535b57b80ec076c699f2578a28ee9dc/COPYING).
  Dank an Markus Enzenberger und die Beitragenden.

Die zugehörigen Original-READMEs sind ebenfalls beigelegt. Der enz-Rohbestand
hat 685.829 Zeilen und 8.644.058 Bytes; die Kompression ändert die Rohbytes
nicht. Eine Gzip-Neukompression darf andere Containerbytes erzeugen, sofern
der dekomprimierte SHA-256 unverändert bleibt.

`supplement.txt` und `rejected.tsv` sind die versionierten redaktionellen
Entscheidungen des Kairos-Projekts. Der vollständige Ausgangsbestand bleibt
erhalten, damit die Pipeline ohne Download reproduzierbar bleibt. Nur
`german.txt` wird in den Laufzeitcode eingebettet. Filterbericht, Änderungen
gegenüber den Quellen und Grenzen der Kuratierung stehen im [README](../README.md).
