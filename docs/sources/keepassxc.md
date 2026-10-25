# KeePassXC-Browser-Protokoll

Zuständig: **nur PR-6** (`feat/keepassxc-browser`, auf PR-4).
Client und Anwendung werden über eine von Core unabhängige Grenze verbunden.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| KeePassXC | [BrowserService 2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/browser/BrowserService.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/browser/BrowserService.cpp` | Kopplung, Nachrichtenvalidierung und Erstellen eines Eintrags über die bestehende Protokollgrenze | 5 |
| keepassxc-browser | [Client](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-browser/background/keepass.js) | GPL-3.0; Dateilizenz beachten | `keepassxc-browser/background/keepass.js` | Verbindungszustand und kryptografische Sitzungs-/Nachrichtenprüfung modellieren | 5 |

Vor der Implementierung sind der genaue Transport und die Kompatibilität mit
dem unterstützten KeePassXC-Stand zu prüfen. Hotkey-Ablauf: generieren, ins Feld
schreiben, Eintrag anlegen; Fehlermeldungen enthalten keine Geheimnisse.
