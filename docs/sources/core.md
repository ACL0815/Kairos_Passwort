# Core: Generator und Entropie

Zuständig: **nur PR-1** (`feat/core-generator`). Vorläufige Modellkandidaten;
die Detailauswertung ergänzt dieser PR. Der Vertrag bleibt additiv kompatibel.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| rand | [Quellstand](https://github.com/rust-random/rand/tree/9e7d328f60d6372ae22927f37d1a816fc980f25e) | MIT OR Apache-2.0 | `src/`, Verteilungen und Zufallsquellen | Bewährte gleichverteilte Auswahl direkt als Crate verwenden; Versions-API vor Integration prüfen | 5 |
| getrandom | [Quellstand](https://github.com/rust-random/getrandom/tree/aa04c59adb35a3fb7e62fefea15700a61c7b4d8e) | MIT OR Apache-2.0 | `src/`, Betriebssystem-Backends | Windows-System-CSPRNG über die Crate-Anbindung verwenden; bei Fehlern abbrechen | 5 |
| KeePassXC Random.cpp | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp) | GPL-2.0 OR GPL-3.0; Dateilizenz beachten | `src/crypto/Random.cpp` | Systemzufall und Rejection Sampling als Entwurfsprinzip; keine Modulo-Verzerrung | 5 |
| xkcdpass | [Quellstand](https://github.com/redacted/XKCD-password-generator/tree/b981361292ae7328313e116178acacc90680df5e) | BSD-3-Clause; Wortdaten separat | `xkcdpass/` | Filter und tatsächliche Quellgröße vor der Entropiebetrachtung festlegen | 5 |
| diceware | [Quellstand](https://github.com/ulif/diceware/tree/89534ea7b2c2d090e76307d55497a27ff90cc3d9) | GPL-3.0; Wortdaten separat | `diceware/`, `diceware/wordlist.py` | Unabhängige Wortziehungen und klare Trennung von Quelle und Ausgabe | 5 |
| pwgen-go | [Quellstand](https://github.com/gabe565/pwgen-go/tree/5a53dce1f9cbde823855ee6facf6889eb5275fde) | MIT; Wortdaten separat | `internal/wordlist/` | Kleiner Generator mit eingebetteten Quellen und getrennten Ausgabeformaten | 4 |

Die Sonderausnahme zur direkten Nutzung von rand/getrandom steht in
[SOURCES.md](../SOURCES.md). Die anderen Projekte liefern ausschließlich Modelle.
Entropie hängt von der realen Quelle, den Filtern und den erlaubten unabhängigen
Entscheidungen ab; Konzeptzahlen werden nicht als Konstanten übernommen.
