# Architektur und API-Vertrag

Stand 26.10.2026, Basis `main` bei `e072f35d0323457bad8608cb2f5796645351ebbc`
mit dem Abschluss-PR. Bestehende Core-Signaturen, Feldtypen und Semantik bleiben
kompatibel; Erweiterungen sind additiv.

## Crate-Graph

| Verbraucher | Direkte Kairos-Abhängigkeiten | Aufgabe |
|---|---|---|
| `kairos-core` | keine | WordSource, Profile 0–12, eigene Regeln, Rotation, CSPRNG, Entropie, Segmente |
| `kairos-wordlists` | core | Eingebettete DE/EN/LA-Quellen und disjunkte Vereinigung |
| `kairos-store` | core | SQLCipher/DPAPI, Import, Snapshots und Integritätsmanifest |
| `kairos-platform` | keine | Clipboard, Fokus/Autotype, globale Hotkeys, CaptureProtection |
| `kairos-keepassxc` | keine | Named-Pipe-/NaCl-Client und DPAPI-Zuordnung |
| `kairos-bridge` | keepassxc | Loopback-Kontextdienst und separate DPAPI-Ablage |
| `kairos-hardening` | keine | Prozessmaßnahmen, Report und LockedSecret |
| `apps/kairos` | core, wordlists, platform, keepassxc, bridge, hardening | egui/eframe, Quellenwahl, Sperrliste, Desktop-/KeePassXC-Aktionen |

`browser/` enthält Violentmonkey-Script und Node-Tests. Die App öffnet keinen
Store und importiert keine externen Listen. Eigene Profile sind Core-API,
noch kein GUI-Editor. Store speichert Wörter, keine Passwörter.

## Core, Quellen und Entropie

`WordSource` ist ein objektsicherer stabiler Index mit `len`, `word`, `language`
und `is_empty`. Nach Kleinschreibung eindeutige, nichtleere alphabetische Wörter
sind Pflicht. Core prüft Quelle und Regeln vor der Auswahl. `ProfileId` validiert
0–12, `GeneratorConfig` enthält Profil und Trennerausschlüsse. `GeneratedPassword`
enthält `Zeroizing<String>`, UTF-8-Bytebereiche und Ausgabe-Min-Entropie, kein
Debug/Clone. Auch Segmente verraten Muster und gehören nicht ins Log.

`generate` und `generate_custom` verwenden OsRng/rand-Uniform mit Zurücklegen
und unabhängigen Entscheidungen. Zufallsfehler brechen ab. `entropy_bits_for`
und Ergebnis-Metadaten verwenden reale Listengröße, effektiven Trennerpool und
die kleinste innere Einfüge-/Großschreibauswahl. `entropy_bits(source_len, ...)`
bleibt ein Sechs-Buchstaben-Modell, keine GUI-Stärkeanzeige. Nach Bearbeitung
entfallen Entropie und Bonus. Fehler enthalten keine Geheimtexte.

Profil 0 nutzt `german()` mit 6.907, Profil 1–12 `combined()` mit 20.697 Wörtern
(DE 6.907, EN/LA je 6.895). Disjunkte Vereinigung und uniforme Wortindizes ergeben
unabhängige Sprache je Wort. Kein separater Sprachbonus. „Nur deutsche Wörter“
überschreibt alle Profile und warnt vor fehlenden Profil-1-Auswahlbits.
7.000/21.000 bleiben Konzeptziele.

`FilteredSource` hält die behaltenen Indizes, leiht Wörter/Sprachflags und
verwendet die gefilterte Größe für Auswahl und Entropie. Leer ergibt
`EmptySource`, keine Ersatzquelle. `ProfileRotation` enthält 1–16 gleichgewichtete
validierte Regeln. Bestehende APIs mit einer Quelle bleiben unverändert.
`generate_rotating_from_sources` erhält additiv genau eine Quelle je Profil in
passender Reihenfolge, prüft alle Paare und zieht erst danach. Ergebnis ist das
Minimum aller quellenabhängigen Grenzen mit null Bonus, auch bei Disjunktheit.
Die App rotiert über 0–12 mit Deutsch für 0 und DE+EN+LA für 1–12; beim Override
überall Deutsch. Wiederholungen erlaubt. Die Schranke gilt auch bei Überlappung.
Gewählte Profil-ID und Einzelstärke werden nicht zusätzlich ausgegeben.

## Startup, Sperrliste und Puffer

`main` ruft zuerst `harden_process()` auf, vor Fokusaufnahme, Argumenten,
CaptureProtection und eframe. Report an die App; jede Maßnahme ist im
Sicherheitsdialog sichtbar. Fehlende, nicht unterstützte oder fehlgeschlagene
Maßnahmen erzeugen einen sichtbaren Status, bleiben nicht fatal. Keine globalen
Registry-/Systemänderungen.

Startup lädt `%APPDATA%\Kairos\blocked-words.txt`, überschreibbar durch
`KAIROS_BLOCKED_WORDS_FILE`. Ein Wort je Zeile, 3–9 ASCII-Buchstaben;
Rand-Leerraum/ASCII-Kleinschreibung und Deduplizierung, keine Transliteration.
Leere Zeilen sind erlaubt. Ungültige/unlesbare Dateien sperren Generierung bis
Korrektur/Neuladen, fehlende Datei ist leer. Wortschaltflächen gibt es nur bei
unbearbeiteten Generatorergebnissen. Großschreibung/Ziffern werden rückgängig
gemacht. Nur das ausdrücklich ausgewählte Wort wird gespeichert, nie das ganze
Passwort; manuelle Eingabe akzeptiert ebenfalls nur ein Wort.

BTreeSet liefert sortierte Zeilen. Schreiben synchronisiert eine NamedTempFile
im selben Verzeichnis und ersetzt das Ziel mit `persist`. Fehler behalten den
bisherigen Filter. Klartextdatei mit geerbten Elternrechten, keine
Manipulationsauthentisierung; Vorlieben können sichtbar werden. Eine Instanz je
Datei, externe Änderungen neu laden, keine konkurrierende Zusammenführung.

Der Editor behält vorab reserviertes `Zeroizing<String>` (16 KiB/4096 Unicode-
Zeichen), gelöscht vor Ersetzen und bei Drop. LockedSecret ist noch nicht
integriert. Dessen feste Bytekapazität braucht einen UTF-8-Bearbeitungsadapter
und separate Lock-/Close-Fehlerpfade. Eine zusätzliche gesperrte Spiegelkopie
würde den String nicht beseitigen. Direkte Core-Erzeugung und geprüfter
TextBuffer-Adapter bleiben Folgearbeit. egui erzeugt weiter String-/Galley-/
Renderkopien. Undo wird je Frame geleert, Persistenz und normale eframe-Clipboard-
Shortcuts sind aus. Kein vollständiger Lösch-/Dump-Schutz.

## Desktop, Browser und Schutzgrenzen

Ziele sind `Unavailable`, `ReturnOnly`, `Editor`. Start erlaubt nur Rücksprung;
erst Strg+Umschalt+F9 gibt einen Editorauftrag frei. Hotkey-Thread erfasst das
Ziel vor Wakeup, App::logic verarbeitet auch verborgen. F10 generiert/tippt
ohne Kairos-Fokus und kopiert nach Windows-Annahme. Aufträge werden einmalig
verbraucht. Fokus/Modifier/Integrität werden geprüft; DOM-Felder mit gleichem
HWND bleiben ununterscheidbar. Keine automatische Wiederholung bei Teileingabe.

Clipboard nutzt `copy_secret` mit drei Ausschlussformaten in derselben
Transaktion. Timer/normaler App-Ende-Pfad leeren nur eine noch eigene Kopie;
fremde neue Inhalte bleiben. Andere Leser, Absturz oder dauerhafte Sperren
begrenzen die Wirkung.

F11 erzeugt frisch und öffnet den Bestätigungsfluss. Brückenvorschläge bleiben
bis zur Übernahme unverbindlich. Loopback bindet nur 127.0.0.1:19471 und prüft
Host/Origin/HMAC/Zeit/Nonce. Bewusstes Pairing mit zweiminütigem Code, keine
generierten Passwörter im Kontextdienst. KeePassXC verwendet getrennte DPAPI-
Zuordnung und Worker. Erst nach Annahme erlaubt ein neuer Klick einmalige
Eingabe ins Originalziel. Annahme beweist keine bestimmte Zieldatenbank/Gruppe;
nach Timeout kann ein Eintrag existieren. Kein Retry.
[Browser-Anleitung](../browser/README.md).

CaptureProtection schützt Root und spätere Top-Level-Fenster desselben GUI-
Threads. Ab Windows 10 2004 WDA_EXCLUDEFROMCAPTURE, älterer Fallback WDA_MONITOR.
`--allow-screencapture` gilt für einen Start mit Dauerbanner. DWM und beachtender
Capture-Pfad nötig; Kameras, privilegierte Tools und manche Fernwartung bleiben
ausgenommen. Prozesshärtung schützt nicht gegen vorhandene Handles, aktive
Debugprivilegien oder alle Dumps; DACL kann Diagnose/Beenden erschweren.

Tests und [Desktop-Checkliste](MANUAL-TEST.md) ergänzen sich. Ignorierte Tests
und nicht ausgeführte Desktop-Schritte sind keine bestandene Abnahme.
[Open-Source-Vorbilder](SOURCES.md).
