# Manipulationserkennung und Speicherhärtung

Zuständig: **nur PR-9** (`feat/protection`, auf PR-1).
Die Modellkandidaten sind vorläufig; es wird kein Schutzcode übernommen.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| KeePassXC Alloc.cpp | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Alloc.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/core/Alloc.cpp` | Speicherfreigabe und garantiertes Überschreiben unterscheiden | 5 |
| KeePassXC Bootstrap.cpp | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/core/Bootstrap.cpp` | Dump-/Prozesszugriffsbeschränkungen vor sensibler Verarbeitung setzen | 5 |
| Bitwarden memory-testing | [Quellstand des Konzepts](https://github.com/bitwarden/sdk-internal/blob/5cf919837e83b71c2508bf99344ea57cf5f84e88/crates/memory-testing/src/lib.rs) | GPL-3.0-only OR LicenseRef-Bitwarden-SDK | `crates/memory-testing/src/lib.rs` | Release-Dumps mit bekanntem Testwert, mehreren Kodierungen und Positivkontrolle untersuchen | 5 |
| libsodium | [Speicherfunktionen](https://github.com/jedisct1/libsodium/blob/60b751cd14418fae2d17e6f1003e43622b3e5a25/src/libsodium/sodium/utils.c) | ISC | `src/libsodium/sodium/utils.c` | Optimierungsfestes Löschen und gesperrte Puffer mit geprüften Rückgabewerten | 5 |
| obfstr | [Quellstand](https://github.com/CasualX/obfstr/tree/9e56c1938dfd3bd65fe305fbafcb57394c701356) | MIT | `src/lib.rs` | String-Verschleierung als begrenzte zusätzliche Hürde bewerten | 3 |

Windows-Maßnahmen: VirtualLock, Dump-Beschränkung und kurze Klartextlebensdauer.
UI-, Undo- und Zwischenablagekopien sind gesondert zu prüfen. Fehlende Dump-
Treffer beweisen keine vollständige Löschung. Die Manipulationserkennung
externer Listen benötigt eine definierte Vertrauensbasis; Verschlüsselung oder
Verschleierung allein ist noch kein Nachweis unveränderter Daten.
