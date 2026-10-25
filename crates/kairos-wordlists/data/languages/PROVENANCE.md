# Provenienz: Englisch und Latein

Abrufdatum: **25.10.2026**. Alle Quellbytes und Original-Lizenztexte liegen lokal.
Die Hashes komprimierter Dateien beziehen sich auf die dekomprimierten Originalbytes.
Die Pipeline braucht kein Netzwerk.

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

EFF: **Joseph Bonneau und Electronic Frontier Foundation**, Listen von 2016.
Gewählt ist **CC BY 4.0** aufgrund der aktuellen ausdrücklichen Freigabe für
EFF-Originalmaterial auf der [EFF-Lizenzseite](https://www.eff.org/copyright).
Die ältere xkcdpass-Kopie nennt CC BY 3.0; Kairos bezieht die Dateien direkt
von EFF und wählt deren aktuelle Freigabe. Der vollständige
[CC-BY-4.0-Text](upstream/eff/LICENSE.txt) und ein gekennzeichneter
[Auszug der Freigabe](upstream/eff/COPYRIGHT-POLICY.txt) liegen bei. Die URLs
sind datierte Veröffentlichungen, keine unveränderlichen Git-Commits; die
SHA-256-Werte legen die tatsächlich übernommenen Bytes fest.

Wordnik: **Copyright (c) 2020 Wordnik**, [MIT](upstream/wordnik/LICENSE).
Der Rohbestand ist ein Wörterbuch für Wortspiele; nur die ausdrückliche
[Positivliste](english-supplement.txt) wird aufgenommen. Die MIT-Datei und
das [Original-README](upstream/wordnik/README.md) gehören zum festen Commit.

Latein: **William A. Whitaker (1936–2010) und Beitragende**, WORDS,
Commit `1f2f0fb0867a896d7b9284a03d615ed635d6f992`. Die
[Originalfreigabe](upstream/whitaker/LICENCE.txt) gestattet jede Nutzung von
Programm und Daten einschließlich Weitergabe und Verkauf. Dies ist eine
**individuelle permissive Freigabe**, keine CC0-/Public-Domain-Erklärung und
keine pauschale MIT-Lizenz. Die Wortdaten werden mit Namensnennung und
unverändertem Lizenztext verteilt. Es wurde kein WORDS-Implementierungscode
übernommen. Die Datei hat 39.335 Datensätze; Dokumentationen mit anderen
Ständen nennen rund 39.000.

Änderungen von Kairos: Würfelpräfixe entfernt, ASCII-/Längenfilter,
Deduplizierung, positive Ergänzungsauswahl, kategorisierte Ausschlüsse,
begrenzte Rekonstruktion lateinischer Grundformen, Vorrang Deutsch vor
Englisch vor Latein und Auswahl auf EN/LA jeweils 6.966 Wörter bei unverändertem DE-Umfang von 7.130. Stammfelder werden nicht pauschal übernommen; ein begrenzter Decoder und
versionierte redaktionelle Sperren bestimmen die lateinischen Wörter.

Die Freigaben betreffen die Wortdaten getrennt von der Crate-Software
(MIT OR Apache-2.0). Für eine Binärdistribution die Angaben dieses Dokuments
und die drei oben verlinkten Lizenztexte zusammen mit den Wortdaten bzw.
der Anwendung mitliefern, insbesondere die EFF-Attribution und den
Änderungshinweis. Es besteht keine Unterstützung oder Empfehlung durch
die ursprünglichen Autoren. Keine übernommenen Daten erfordern Copyleft. Die zusammengefasste
[NOTICE.txt](NOTICE.txt) enthält Attribution, Änderungshinweise und alle drei
Lizenztexte der neuen Wortdaten zur Weitergabe. Die spätere GUI-/Release-Integration
muss diese Datei der EXE beilegen; diese Packaging-Verdrahtung liegt außerhalb
des Schreibbereichs dieses Crate-PRs.

## Reproduzierbarer Filterbericht

Arbeitsstand mit STOP, keine semantische Abnahme; siehe [Diagnose](../curation-diagnosis.md).

```text
German retained: 7130
Semantic decisions sha256: e2b7f51f134ce27745cf3813c73d10f0a1bdd3fcc9e1652cf71e511fdfeccb86
English raw rows: 10368
English eligible distinct: 8301
English Wordnik selected additions: 1257
English semantic decisions: 9558; blocked: 1408
English editorial rejections: 1226
English additional semantic blocks: 757
English disease/injury/decay/death guard removed: 0
English overlap removed (German): 361
English available after curation and overlap removal: 7214
Latin raw rows: 39335
Latin eligible distinct (frequency A-D, supported lemma patterns): 11632
Latin semantic decisions: 11632; blocked: 2952
Latin editorial rejections: 2352
Latin additional semantic blocks: 1388
Latin sensitive-gloss/spelling guard removed: 16
Latin overlap removed (German): 77
Latin overlap removed (English): 108
Latin available after curation and overlap removal: 7691
Latin identical-gloss variants removed: 725
Latin available after variant filter: 6966
Equal EN/LA size reduced to 6966: minimum of German target, reviewed English pool and Latin pool after overlap/variant filtering. German unchanged; Latin overlap exclusions retain the English preselection of 7130.
English final: 6966; mean length: 6.541; sha256: 511107e3b370cb4c184ef9c5fc0675fbe226fbbe06ee533f1eba6a6047784881
Latin final: 6966; mean length: 6.990; sha256: 95d8b89f37c794ce6c2ea446f59551aff7cb08ce98e087fa6269630d0e0d0987
Combined distinct: 21062
```

Die Ergebnisprüfwerte stehen in [english.sha256](english.sha256) und [latin.sha256](latin.sha256). Sie fixieren den reproduzierbaren Arbeitsstand und behaupten keine semantische Freigabe. Rohquellen und Lizenztexte bleiben unverändert.
