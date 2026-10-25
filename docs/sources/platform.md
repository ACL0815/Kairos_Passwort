# Windows: Zwischenablage, Hotkeys und Fokus

Zuständig: **nur PR-4** (`feat/platform-hotkeys`). Die Crate bleibt unabhängig
von Core und GUI. Die Bewertung ist vorläufig.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| KeePassXC Clipboard.cpp | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/gui/Clipboard.cpp` | Sensible Inhalte kennzeichnen, Verlauf/Cloud ausschließen, Timeout und Besitzwechsel berücksichtigen | 5 |
| Password Tech | [Zwischenablage](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/util/SecureClipboard.cpp) | GPL-2.0-or-later | `src/util/SecureClipboard.cpp` | Kurze Lebensdauer und explizite Nutzeraktion für Kopieren in einer Windows-App | 4 |
| KeePassXC | [2.8.0](https://github.com/keepassxreboot/keepassxc/tree/2.8.0/src) | GPL-2.0 OR GPL-3.0; Dateilizenzen beachten | `src/autotype/`, `src/gui/` | Fokusziel, Fensterwechsel und Autotype als getrennte Betriebssystemaktionen behandeln | 4 |

Zwei Hotkey-Abläufe: Popup zum Nachbearbeiten/1–2× Neugenerieren sowie
Generieren + ins aktuelle Feld schreiben + kopieren. Rücksprung zum vorherigen
Fenster und Timeout dürfen keine fremden, inzwischen neu kopierten Inhalte löschen.
