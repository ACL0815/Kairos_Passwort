# Priorisierte Roadmap

Stand 25.10.2026. `main` bei `fa35aded13b50a67a817db79258b1ba1c95a9d46`
enthält Generator, deutsche Wortliste, GUI, Plattformdienste, Rotation und Store.
Die fachlichen PR-Kürzel unterscheiden sich von den GitHub-Nummern.
[ARCHITECTURE.md](ARCHITECTURE.md) beschreibt die implementierten Verträge.

## Umgesetzt und Integration

| Prio | Plan-PR | GitHub | Branch | Status und Umfang |
|---|---|---|---|---|
| 1 | PR-1 | [#1](https://github.com/ACL0815/Kairos_Passwort/pull/1) | `feat/core-generator` | Gemergt. Profile 0–12, rand/OsRng, konservative quellenabhängige Entropie, Segmente, Ausschlüsse. |
| 1 | PR-2 | [#2](https://github.com/ACL0815/Kairos_Passwort/pull/2) | `feat/wordlist-de` | Gemergt. 7.130 kuratierte deutsche ASCII-Wörter, Sprachflag, reproduzierbare Pipeline, CC0-Daten. |
| 2 | PR-4 | [#3](https://github.com/ACL0815/Kairos_Passwort/pull/3) | `feat/platform-hotkeys` | Gemergt mit Desktop-Prüfauflagen. Globale Hotkeys, geschützte Clipboard-Transaktion, Timeout, Unicode-Autotype und Fokusidentität. |
| 2 | PR-3 | [#4](https://github.com/ACL0815/Kairos_Passwort/pull/4) | `feat/gui` | Gemergt. egui-Oberfläche, Profilwahl, Bearbeitung und Hervorhebung. Der Übergangsadapter wird in PR-W ersetzt. |
| 4 | PR-8 | [#5](https://github.com/ACL0815/Kairos_Passwort/pull/5) | `feat/profile-rotation` | Gemergt. Eigene Profile und optionale gleichverteilte Rotation, ehrlicher Bonus und konservative Mischungsentropie. |
| 3 | PR-5 | [#6](https://github.com/ACL0815/Kairos_Passwort/pull/6) | `feat/encrypted-store` | Gemergt. Import, SQLCipher, DPAPI, stabile WordSource-Snapshots. Noch keine Store-Oberfläche. |
| 2 | PR-W | [#8](https://github.com/ACL0815/Kairos_Passwort/pull/8) | `feat/app-wiring` | Implementiert, Review-Nachbesserung. Echte deutsche Quelle, Plattform-Clipboard, Fokus-Rücksprung, F9/F10/F11 im eframe-Loop, Rotationshaken, Herkunftsdialog und manuelle Desktop-Checkliste. |
| 4 | PR-7 | [#7](https://github.com/ACL0815/Kairos_Passwort/pull/7) | `feat/languages-en-la` | Offen, in Nachbesserung. Englisch/Latein und `combined()` sind nicht auf der hier verwendeten main-Basis. |

## Nächste PRs

| Prio | Plan-PR | Bereich / geplanter Branch | Abhängigkeiten | Status und Umfang |
|---|---|---|---|---|
| 3 | PR-6 | `kairos-keepassxc` / `feat/keepassxc-browser` | PR-4 und PR-W | Geplant. Kopplung und Eintrag über KeePassXC-Browser-Protokoll. F11 zeigt bis dahin ausdrücklich „KeePassXC-Anbindung folgt (PR-6)“. |
| 5 | PR-9a | `feat/protection-hardening` | PR-1, PR-5, PR-W | Geplant. Manipulationserkennung externer Listen mit Quellhash-Vergleich, Speicherhärtung (VirtualLock, Dump-Beschränkung), Speicher-Dump-Tests mit Positivkontrolle. |
| 5 | PR-9b | `feat/wordlist-protection` | PR-7 | Geplant. obfstr und Wortlistenschutz nach Abschluss der mehrsprachigen Datenbasis. Keine zusätzlichen Entropiebits durch Verschleierung behaupten. |
| 6 | PR-10 | `browser/` / `feat/browser-violentmonkey` | PR-6 | Geplant. Violentmonkey-Userscript und lokale Brücke nach 2FAS-/KeePassXC-Browser-Prinzipien. |
| 7 | PR-11 | `feat/screencapture-protection` | Nach PR-W | Geplant. „Unsichtbar bei Bildschirmfreigabe“ mit `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)`, Vorbild KeePassXC. Expliziter Startparameter `--allow-screencapture`. Windows-Grenzen, nicht unterstützte Capture-Pfade und Fehlermeldung prüfen. PR-W enthält diesen Schutz noch nicht. |

## Integrationsregeln und Abnahme

Merge-Reihenfolge folgt Priorität und Abhängigkeiten. Neue Folgebranches starten
vom integrierten `main`; alte gestapelte Branches werden vor dem Merge abgeglichen.
`Cargo.lock` wird mit Cargo aufgelöst. Der Core-Vertrag bleibt additiv.
Die App akzeptiert `Box<dyn WordSource>` für einen späteren Wechsel auf
`combined()` oder Store-Snapshots. Beides braucht einen eigenen Integrationsauftrag.

Bestandene Unit-Tests ersetzen keine Desktop-Abnahme. Die echte Zustellung,
Fokus-Rückgabe, das deutsche Layout und Win+V sind mit
[MANUAL-TEST.md](MANUAL-TEST.md) zu prüfen. Bereits vorhandene offene
Desktop-Befunde werden durch diesen PR nicht pauschal als erledigt gewertet.
Bereichsquellen bleiben in den zugewiesenen Dateien aus [SOURCES.md](SOURCES.md).
