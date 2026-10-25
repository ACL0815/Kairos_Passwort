# Eigene Profile und automatische Rotation

Zuständig: **nur PR-8** (`feat/profile-rotation`, auf PR-1).
Die eingebauten Profile 0–12 gehören bereits zu PR-1.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| Password Tech | [Quellstand](https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29) | GPL-2.0-or-later | `src/main/`, Generator-Konfiguration | Konfiguration und Erzeugung trennen; eigene Regeln explizit speichern | 4 |
| xkcdpass | [Quellstand](https://github.com/redacted/XKCD-password-generator/tree/b981361292ae7328313e116178acacc90680df5e/xkcdpass) | BSD-3-Clause; Wortdaten separat | `xkcdpass/` | Wortanzahl, Trenner und Großschreibung als nachvollziehbare Parameter behandeln | 4 |
| KeePassXC | [2.8.0](https://github.com/keepassxreboot/keepassxc/tree/2.8.0/src) | GPL-2.0 OR GPL-3.0; Dateilizenzen beachten | `src/core/`, `src/gui/` | Profilzustand und sichtbare Generatorparameter konsistent halten | 4 |

Der automatische Wechsel pro Generierung ist eine eigene Zufallsentscheidung.
Bis zu +4 Bit gelten nur für 16 gleichwahrscheinliche, gleich starke Profile
mit überschneidungsfreien Ausgabemengen; bei bekanntem Profil beträgt der Bonus
0 Bit. Der optionale Haken darf diese Voraussetzungen nicht verschweigen.
