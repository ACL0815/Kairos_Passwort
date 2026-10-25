# Priorisierte Roadmap

Das Scaffold auf `main` ist die gemeinsame Ausgangsbasis. Die folgenden PRs
werden erst in den nachgelagerten Aufträgen angelegt. Der API-Vertrag aus
[ARCHITECTURE.md](ARCHITECTURE.md) gilt für alle parallelen Implementierungen.

## Welle 1

Alle von `main`, parallel, gegen API-Vertrag.

| Prio | PR | Branch | Crate | Basis | Abhängigkeiten | Kurzbeschreibung |
|---|---|---|---|---|---|---|
| P1 | PR-1 | `feat/core-generator` | `kairos-core` | `main` | API-Vertrag | CSPRNG über `rand`+`OsRng`/`getrandom` (= Windows-Systemgenerator; V1 und V2 des Konzepts fallen zusammen, weil das der Weg mit dem wenigsten eigenen Code ist), gleichverteilte Auswahl (Rejection Sampling über rand), Profile 0–12 datengetrieben exakt nach Konzepttabelle, Entropieberechnung aus realer Listengröße, Sonderzeichen-Ausschluss mit Neuberechnung, Segmente für Hervorhebung, zeroize. |
| P1 | PR-2 | `feat/wordlist-de` | `kairos-wordlists` | `main` | API-Vertrag | Deutsche Wortliste eingebettet in die EXE, Sprach-Flag je Wort, Auswahl-/Filterpipeline dokumentiert. |
| P2 | PR-3 | `feat/gui` | `apps/kairos` | `main` | API-Vertrag | egui-GUI, Profilwahl 0–12, Neu generieren, manuell nachbearbeiten, farbige Hervorhebung Zahlen/Sonderzeichen, Passwort-Schriftart eingebettet, Kopieren / Kopieren + zurückspringen, Stil nach UI-STYLE.md. |
| P2 | PR-4 | `feat/platform-hotkeys` | `kairos-platform` | `main` | API-Vertrag | Globale Hotkeys (Popup zum Nachbearbeiten/1–2× neu generieren; Generieren + ins aktuelle Feld schreiben + kopieren), Zwischenablage mit Verlauf-/Cloud-Ausschluss und Timeout, Fokus-Rücksprung. |
| P3 | PR-5 | `feat/encrypted-store` | `kairos-store` | `main` | API-Vertrag | Import externer Wortlisten in SQLCipher-DB, Sprach-Flag, randomisierte Speicherung, Schlüssel per Windows DPAPI. |

## Welle 2

Gestapelt auf den jeweiligen Welle-1-Branch.

| Prio | PR | Branch | Crate | Basis | Abhängigkeiten | Kurzbeschreibung |
|---|---|---|---|---|---|---|
| P3 | PR-6 | `feat/keepassxc-browser` | `kairos-keepassxc` | `feat/platform-hotkeys` | PR-4 | Hotkey generiert, schreibt ins Feld und legt Eintrag über KeePassXC-Browser-Schnittstelle an. |
| P4 | PR-7 | `feat/languages-en-la` | `kairos-wordlists` | `feat/wordlist-de` | PR-2 | Englisch, dann Latein (oder ähnliche leicht schreibbare Sprache), überschneidungsfrei, je Wort unabhängige Sprachwahl. |
| P4 | PR-8 | `feat/profile-rotation` | `kairos-core` | `feat/core-generator` | PR-1 | Eigene Profile, automatischer Profilwechsel je Generierung (+bis 4 Bit, als Haken anzeigbar). |
| P5 | PR-9 | `feat/protection` | `kairos-core` | `feat/core-generator` | PR-1 | Manipulationserkennung externer Wortlisten beim Start, obfstr, Speicherhärtung (VirtualLock, Dump-Beschränkung), Speicher-Dump-Test nach Bitwarden-Vorbild. |
| P6 | PR-10 | `feat/browser-violentmonkey` | `browser/` | `feat/keepassxc-browser` | PR-6 | Violentmonkey-Userscript + lokale Brücke nach Vorbild 2FAS/keepassxc-browser. |

## Integration

**Merge-Reihenfolge = Priorität.** Cargo.lock-Konflikte beim Rebase per
`cargo update -w` lösen. Welle-2-Branches werden auf ihren jeweiligen Basis-PR
gestapelt und nach dessen Merge auf `main` umgebased.

Die unabhängige Implementierung gegen den Vertrag ersetzt nicht den späteren
Integrationstest der Anwendung mit echten Komponenten. Bereichsquellen werden
nur im zugewiesenen PR aus [SOURCES.md](SOURCES.md) bearbeitet; der Index selbst
bleibt bei Folge-PRs unverändert. Änderungen am Vertrag bleiben additiv.
