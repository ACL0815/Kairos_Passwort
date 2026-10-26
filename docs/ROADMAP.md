# Roadmap und Integrationsstand

Stand 26.10.2026 (Europe/Berlin). Abschluss-Basis `main` bei
`e072f35d0323457bad8608cb2f5796645351ebbc`. Plan-Kürzel und GitHub-Nummern
sind verschieden. Status gegen die GitHub-PRs geprüft.

| GitHub | Plan | Status | Ergebnis |
|---|---|---|---|
| [#1](https://github.com/ACL0815/Kairos_Passwort/pull/1) | PR-1 Core | Gemergt | Profile 0–12, OsRng/Uniform, Entropie, Segmente, Ausschlüsse. |
| [#2](https://github.com/ACL0815/Kairos_Passwort/pull/2) | PR-2 DE | Gemergt | Deutsche Wortliste; aktueller Bestand 6.907. |
| [#3](https://github.com/ACL0815/Kairos_Passwort/pull/3) | PR-4 Plattform | Gemergt | Hotkeys, Clipboard, Fokus, Autotype. Desktop-Abnahme separat. |
| [#4](https://github.com/ACL0815/Kairos_Passwort/pull/4) | PR-3 GUI | Gemergt | egui-Editor, Hervorhebung, Passwortschrift. |
| [#5](https://github.com/ACL0815/Kairos_Passwort/pull/5) | PR-8 Profile | Gemergt | Eigene Regeln als Core-API, Serde und Rotation. |
| [#6](https://github.com/ACL0815/Kairos_Passwort/pull/6) | PR-5 Store | Gemergt | SQLCipher, DPAPI, Import, Snapshots. Noch keine Import-UI. |
| [#7](https://github.com/ACL0815/Kairos_Passwort/pull/7) | PR-7 EN/LA | Gemergt | EN/LA je 6.895, combined() disjunkt 20.697; redaktionelle Grenzen in Wortlistenberichten. |
| [#8](https://github.com/ACL0815/Kairos_Passwort/pull/8) | PR-W Verdrahtung | Gemergt | Echte Wortquelle, Plattformdienste, Hotkey-Editor, Rotation. |
| [#9](https://github.com/ACL0815/Kairos_Passwort/pull/9) | PR-6 KeePassXC | Gemergt | Named-Pipe-/NaCl-Client, DPAPI-Zuordnung, Eintragsanfrage. |
| [#10](https://github.com/ACL0815/Kairos_Passwort/pull/10) | PR-9a Härtung | Gemergt | Prozessmaßnahmen, LockedSecret, Store-Manifest und isolierte Tests. |
| [#11](https://github.com/ACL0815/Kairos_Passwort/pull/11) | PR-6b / PR-10 | Gemergt | F11-Fluss, Kontextbrücke, Violentmonkey-Pairing. |
| [#12](https://github.com/ACL0815/Kairos_Passwort/pull/12) | PR-11 Bildschirmschutz | Gemergt | Geschützte Fenster, --allow-screencapture, Statusbanner. |
| [#13](https://github.com/ACL0815/Kairos_Passwort/pull/13) | PR-9b obfstr | Offen, separat | Verschleierung, API unverändert, kein Entropiebonus. Hier keine Wortlisten-Dateien geändert. |
| Dieser PR (`feat/finalize`) | Abschluss | Implementiert; Review/CI gemäß PR-Belegen | Profilquelle, Deutsch-Override, Nutzer-Sperrliste, früher Härtungsaufruf, Report, Doku. |

## Restgrenzen und Folgearbeiten

1. Echte Desktop-Abnahme mit Build-SHA und Versionsangaben gemäß
   [MANUAL-TEST.md](MANUAL-TEST.md). Fokus, deutsches Layout, Win+V,
   Teams/Snipping Tool und echte KeePassXC-Einträge sind durch Unit-Tests oder
   Renderer-Screenshots nicht vollständig belegt.
2. Store-Import-UI mit ausdrücklicher Manifestmigration, Warnungen und optionaler
   Originalquellenprüfung. App öffnet Store bisher nicht; Bibliotheks-APIs bestehen.
3. Eigene Profile in der GUI bearbeiten/verwalten. Core unterstützt sie bereits.
4. LockedSecret für den editierbaren Puffer und direkte Erzeugung integrieren.
   UTF-8-TextBuffer-Adapter, sichtbare Lock-/Close-Fehler und reale Speicherprüfung
   nötig. Eine zusätzliche Spiegelkopie ersetzt den heutigen String nicht.
5. CI-Release-Probe, Verteilpaket, Daten-/Font-Hinweise und Signierung prüfen.
   Aktuelle Windows-CI prüft fmt/clippy/Tests, keinen installierten Release-Ablauf.
6. PR #13 separat reviewen/integrieren. Verschleierung erschwert einfache Analyse,
   schützt keine Geheimnisse vor entschlossenen lokalen Lesern und erhöht keine Bits.

Rotation nutzt je Profil die gefilterte Quelle und zeigt das Minimum ohne Bonus.
7.000/21.000 sind Konzeptziele; real zählen 6.907/20.697 minus Ausschlüsse.
Nachbearbeitung und wiederholte Auswahl nach Vorlieben können die ursprüngliche
Zufallsverteilung ändern. Die Sperrliste ist Klartext mit gewählten Wörtern,
kein Passwortverlauf und keine authentisierte Datei. Eine Instanz je Datei,
externe Änderungen neu laden. Prozess-/Capture-Härtung begrenzt nur unterstützte
Wege. Ältere Crate-READMEs beschreiben teilweise ihren damaligen Integrationsstand;
[ARCHITECTURE.md](ARCHITECTURE.md) beschreibt den App-Endstand.
