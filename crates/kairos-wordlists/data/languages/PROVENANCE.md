# Provenienz: Englisch und Latein

Abruf 25.10.2026. Die Hashes komprimierter Quellen beziehen sich auf die
dekomprimierten Bytes. Quellen und Originallizenzen liegen lokal; die
Wortlisten-Pipelines benötigen kein Netzwerk.

| Lokaler Pfad unter `upstream/` | Original-Download / fester Stand | SHA-256 der lokal geprüften Bytes (Rohdaten: Originalbytes) |
|---|---|---|
| `eff/large.txt` | [Original](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) | `addd35536511597a02fa0a9ff1e5284677b8883b83e986e43f15a3db996b903e` |
| `eff/short1.txt` | [Original](https://www.eff.org/files/2016/09/08/eff_short_wordlist_1.txt) | `8f5ca830b8bffb6fe39c9736c024a00a6a6411adb3f83a9be8bfeeb6e067ae69` |
| `eff/short2.txt` | [Original](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt) | `22b45c52e0bd0bbf03aa522240b111eb4c7c0c1d86c4e518e1be2a7eb2a625e4` |
| `whitaker/DICTLINE.GEN.gz` | [Original](https://raw.githubusercontent.com/mk270/whitakers-words/1f2f0fb0867a896d7b9284a03d615ed635d6f992/DICTLINE.GEN) | `8f6c0fc84d12859abc863eac84ddd49b7e039503855ca34c1d44a6e78abf7569` |
| `whitaker/LICENCE.txt` | [Original](https://raw.githubusercontent.com/mk270/whitakers-words/1f2f0fb0867a896d7b9284a03d615ed635d6f992/LICENCE.txt) | `de533c3fb7c4a54b6d2deb5e56624947850bd2121c1de206ba5ee59b72c119a3` |
| `whitaker/README.md` | [Original](https://raw.githubusercontent.com/mk270/whitakers-words/1f2f0fb0867a896d7b9284a03d615ed635d6f992/README.md) | `7383c8ef30f30286d2dbff58fb88ef659a48c6aadd16108e10368f8a03f0dbdc` |
| `eff/LICENSE.txt` | [Original](https://creativecommons.org/licenses/by/4.0/legalcode.txt) | `9ba9550ad48438d0836ddab3da480b3b69ffa0aac7b7878b5a0039e7ab429411` |
| `eff/COPYRIGHT-POLICY.txt` (eigener Auszug, nicht bytegleich zur Webseite) | [Quelle des Auszugs](https://www.eff.org/copyright) | `33c85f185938a5952915526dbaedac04812ed00d668136ab27f20f211f530c5f` |
| `wordnik/wordlist-20210729.txt.gz` | [Original](https://raw.githubusercontent.com/wordnik/wordlist/46e6215d0f90356afe9c8ba4be347e7e98cb425c/wordlist-20210729.txt) | `bfd1b4eb4ade1ba81e84c7e24248b9a1aecec9d9baa427453b367a83e30e0451` |
| `wordnik/LICENSE` | [Original](https://raw.githubusercontent.com/wordnik/wordlist/46e6215d0f90356afe9c8ba4be347e7e98cb425c/LICENSE) | `a568f0ac4c7ad5248ff748387d41e39519811b4a3e07d7438c9d1465dd4c9339` |
| `wordnik/README.md` | [Original](https://raw.githubusercontent.com/wordnik/wordlist/46e6215d0f90356afe9c8ba4be347e7e98cb425c/README.md) | `740feccbefca2ff908862884cd46fbfbd6b1787eff24ef69f2369bbef8ca704b` |
| `wordnet/WordNet-3.0.tar.gz` | Princeton WordNet 3.0 / eigener Auszug | `0d25346d1e6f1b053708274563eb4379171e14da21524fb12776ec0896a46e62` |
| `wordnet/LICENSE` | Princeton WordNet 3.0 / eigener Auszug | `7731175a77952e259390b496fab905e57118b8d19ad3a8383c67eee724ff443f` |
| `wordnet/candidate-senses.tsv.gz` | Princeton WordNet 3.0 / eigener Auszug | `439192525026792d32f95a7e68a8a1adbe425d3b88d22942a7db504ade4eb250` |

EFF: Joseph Bonneau und Electronic Frontier Foundation, 2016. Direkte EFF-Dateien
werden unter der vorhandenen ausdrücklichen CC-BY-4.0-Freigabe verteilt.
[EFF-Richtlinie](https://www.eff.org/copyright), [vollständige Lizenz](upstream/eff/LICENSE.txt).
Ältere Kopien nennen CC BY 3.0; sie werden hier nicht als neue Quelle verwendet.

Wordnik: Copyright (c) 2020 Wordnik, [MIT](upstream/wordnik/LICENSE).
Nur die expliziten Ergänzungen sind Kandidaten, keine ungeprüften Wörterbuchreste.

Whitaker und Beitragende: individuelle permissive Freigabe für Programm und Daten,
keine CC0-/Public-Domain-Behauptung. [Originaltext](upstream/whitaker/LICENCE.txt).
Der begrenzte eigene Decoder verwendet nur Wortdaten.

Princeton WordNet 3.0: Copyright 2006 by Princeton University. Das unveränderte
[Originalarchiv](https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz),
die [Originallizenz](upstream/wordnet/LICENSE) und der als eigener Auszug
gekennzeichnete Bedeutungsbestand sind enthalten. Der Extraktor übernimmt
keinen Princeton-Implementierungscode. Das Originalarchiv enthält auch die
unbenutzten Upstream-Werkzeuge. [Extraktionsverfahren und Containerhash](upstream/wordnet/README.md).

Kairos entfernt Würfelpräfixe, filtert Zeichen/Länge, vereinigt und dedupliziert,
wählt Ergänzungen, rekonstruiert unterstützte LA-Lemmata, bindet Bedeutungen,
wendet redaktionelle und A/B-Entscheidungen an und entfernt Überlappungen/Varianten.
Die Autoren unterstützen oder empfehlen diese Änderungen nicht.

| Sprache | Wörter | Mittelwert | SHA-256 |
|---|---:|---:|---|
| DE | 6.909 | 6,577 | `02911732a22dbba0967453121e60d245bc7452e13383d808acfbbef47efb2ed0` |
| EN | 6.897 | 6,659 | `8aae2952cab7bb2406864e7f87b685f8f5c3ced1d7c7b4f04d4a666f9253899a` |
| LA | 6.897 | 6,988 | `7ccd0262310397edb42e1c65e33d5a781a4426fd84ad14d7edf8bedee28fd8c3` |

Die Softwarelizenz MIT OR Apache-2.0 ersetzt keine dieser Datenlizenzen.
[NOTICE.txt](NOTICE.txt) enthält alle vier vollständigen Texte und Attributionen
zur Weitergabe mit der Anwendung. Keine übernommenen Daten erfordern Copyleft.
Die spätere NOTICE-Auslieferung liegt außerhalb dieses Crate-Auftrags.

## Reproduzierbarer Filterbericht

```text
German retained: 6909
Acceptance rubric A/B sha256: 65ea55de3e60a534da3243a3fe3600e3fecead5338a86daa818cd193c4e05159
Semantic decisions sha256: 09b47c892c5f4c18560129a3137e1999ff7d9751a761059e72c94dc51ebfbba0
English raw rows: 10368
English eligible distinct: 8301
English Wordnik selected additions: 1257
English WordNet covered: 8924; explicit no-entry: 634; EFF Large positive signals: 7772
English person meanings review: 669 candidates; sha256: 8fd57ff79446dd90c6a3fe2441dd6d827b3d471eb103874884a9d125275c6a4f
English semantic decisions: 9558; blocked: 1782
English editorial rejections: 1226
English additional semantic blocks: 1066
English disease/injury/decay/death guard removed: 0
English overlap removed (German): 343
English available after curation and overlap removal: 6923
Latin raw rows: 39335
Latin eligible distinct (frequency A-D, supported lemma patterns): 11632
Latin semantic decisions: 11632; blocked: 3095
Latin editorial rejections: 2352
Latin additional semantic blocks: 1472
Latin sensitive-gloss/spelling guard removed: 16
Latin overlap removed (German): 77
Latin overlap removed (English): 99
Latin available after curation and overlap removal: 7616
Latin identical-gloss variants removed: 719
Latin available after variant filter: 6897
Equal EN/LA size reduced to 6897: minimum of German target, reviewed English pool and Latin pool after overlap/variant filtering. German unchanged; Latin overlap exclusions retain the English preselection of 6909.
English final: 6897; mean length: 6.659; sha256: 8aae2952cab7bb2406864e7f87b685f8f5c3ced1d7c7b4f04d4a666f9253899a
Latin final: 6897; mean length: 6.988; sha256: 7ccd0262310397edb42e1c65e33d5a781a4426fd84ad14d7edf8bedee28fd8c3
Combined distinct: 20703
```

Aktueller Prüfstand und semantische Grenzen: [Stufe 2](../curation-stage-two.md).
