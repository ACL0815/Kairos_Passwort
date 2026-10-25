# Englisch und Latein

Zuständig: **nur PR-7** (`feat/languages-en-la`, auf PR-2).
Zuerst Englisch, danach Latein oder eine vergleichbar leicht schreibbare Sprache.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| xkcdpass | [Mehrsprachige Quellen](https://github.com/redacted/XKCD-password-generator/tree/b981361292ae7328313e116178acacc90680df5e/xkcdpass) | BSD-3-Clause (Code); Daten je Liste verschieden | `xkcdpass/static/` | Herkunft und Lizenz je Sprache; Normalisierung und sprachübergreifende Duplikate | 5 |
| diceware | [Wortlisten](https://github.com/ulif/diceware/tree/89534ea7b2c2d090e76307d55497a27ff90cc3d9/diceware/wordlists) | GPL-3.0 (Code); Daten separat prüfen | `diceware/wordlists/` | Sprachen als austauschbare Quellen mit expliziten Größen behandeln | 4 |
| pwgen-go | [EFF-Quellen](https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/wordlist) | MIT (Code); EFF-Wortdaten separat | `internal/wordlist/` | Englische Quellen und Metadaten in der Anwendung einbetten | 4 |

Diese Kandidaten belegen noch keine geeignete lateinische Liste. Vor deren
Aufnahme ist eine eigenständige Datenquellen- und Lizenzprüfung erforderlich.
Die Sprachwahl erfolgt je Wort unabhängig; Listen müssen überschneidungsfrei
sein. Sprach-Flag und effektive Quellgröße bleiben für den Core nachvollziehbar.
