# GUI und Passwortdarstellung

Zuständig: **nur PR-3** (`feat/gui`). Verbindlicher visueller Rahmen:
[UI-STYLE.md](../design/UI-STYLE.md). Die Passung ist eine erste Einschätzung.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| egui/eframe | [Quellstand](https://github.com/emilk/egui/tree/bf5ac0dac137af9a4861611f3d2a77ad531f50e3) | MIT OR Apache-2.0; Assets separat | `crates/egui/`, `crates/eframe/` | Native Rust-Oberfläche, segmentierte Textdarstellung und klarer UI-Zustand | 5 |
| Password Tech | [Windows-Oberfläche](https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/main) | GPL-2.0-or-later | `src/main/` | Direkter Generator-Ablauf, editierbares Ergebnis und wenige Kopieraktionen | 4 |
| KeePassXC | [GUI 2.8.0](https://github.com/keepassxreboot/keepassxc/tree/2.8.0/src/gui) | GPL-2.0 OR GPL-3.0; Dateilizenzen beachten | `src/gui/` | Passwortlesbarkeit und Trennung von Generierung, Anzeige und Zwischenablage | 4 |

Profilwahl, Neugenerierung, manuelle Nachbearbeitung, farbige Zahlen/Sonderzeichen,
eingebettete Passwort-Schriftart, Kopieren und Kopieren + Zurückspringen bilden
den Funktionsumfang. Kein fremdes Stockbild übernehmen. Eine geeignete
Schriftlizenz muss vor dem Einbetten gesondert nachgewiesen werden.
