# Verschlüsselte externe Wortlisten

Zuständig: **nur PR-5** (`feat/encrypted-store`). SQLCipher, Sprach-Flags,
randomisierte Speicherung und Windows-DPAPI werden erst dort implementiert.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| SQLCipher | [Quellstand](https://github.com/sqlcipher/sqlcipher/tree/c4b275a47932888216bade83aff2bbc73df0ff85) | BSD-3-Clause; Komponentenbedingungen prüfen | `src/`, `LICENSE.md`, `LICENSE.txt` | Datenbankverschlüsselung über etablierten SQLCipher-Adapter, keine eigene Kryptografie | 5 |
| KeePassXC | [Browser-/Schlüsselgrenze](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/browser/BrowserService.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/browser/BrowserService.cpp` | Entschlüsselten Zustand und Zugriff auf gespeicherte Daten bewusst begrenzen | 3 |
| libsodium | [Speicherfunktionen](https://github.com/jedisct1/libsodium/blob/60b751cd14418fae2d17e6f1003e43622b3e5a25/src/libsodium/sodium/utils.c) | ISC | `src/libsodium/sodium/utils.c` | Schlüsselpuffer kurz halten, Sperr-/Löschfehler und Kopien berücksichtigen | 4 |

Für DPAPI ist die Windows-Schnittstelle die Implementierungsgrundlage.
Die Modellkandidaten begründen keine Übernahme ihres Codes. Import-,
Verschlüsselungs- und Integritätsfehler dürfen keine stillen Ersatzpfade öffnen.
