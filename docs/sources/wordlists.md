# Eingebettete deutsche Wortliste

Zuständig: **nur PR-2** (`feat/wordlist-de`). Noch keine Datenübernahme.
Die Tabelle ist der Startpunkt für die dokumentierte Auswahl-/Filterpipeline.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| xkcdpass | [Quellstand](https://github.com/redacted/XKCD-password-generator/tree/b981361292ae7328313e116178acacc90680df5e) | BSD-3-Clause (Code); Daten je Liste verschieden | `xkcdpass/static/` | Deutsche Kandidaten nach Schreibbarkeit, Länge und Duplikaten filtern; Provenienz je Liste | 5 |
| diceware | [Wortlisten](https://github.com/ulif/diceware/tree/89534ea7b2c2d090e76307d55497a27ff90cc3d9/diceware/wordlists) | GPL-3.0 (Code); Daten separat prüfen | `wordlist_de.txt`, `wordlist_de_8k.txt` | Listenformat, Gleichverteilung und Auswahlgröße nachvollziehbar dokumentieren | 5 |
| pwgen-go | [Eingebettete Listen](https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist) | MIT (Code); EFF-Wortdaten separat | `internal/wordlist/` | Einbettung und Metadaten unabhängig vom Generator organisieren | 4 |
| Password Tech | [Quellstand](https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29) | GPL-2.0-or-later (Code); Downloads separat | `src/`, Wörterbuchverarbeitung | Unicode-Wörter und importierbare Listen als eigene Datenquelle behandeln | 4 |

Vor Aufnahme einer Liste: Herkunft und unveränderlichen Stand, Anzahl vor/nach
Filterung, Normalisierung, ausgeschlossene Zeichen, Duplikatprüfung und
redistributionsfähige Datenlizenz dokumentieren. Keine Code-Lizenz pauschal
auf die Wörterbücher übertragen. Jedes Wort erhält `Language::German`.
