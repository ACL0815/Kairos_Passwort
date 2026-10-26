<img src="assets/icon.svg" width="64" height="64" alt="">

# Kairos Passwort

Wortpasswort-Generator für Windows x64 mit Rust und egui. Erzeugt ein Passwort,
macht es gut abschreibbar und übergibt es an ein zuvor gewähltes Feld.
Das [Grundkonzept](docs/Grundkonzept.md) bleibt die unveränderte Konzeptquelle.

## Funktionen

- Profile 0–12 mit kumulativen Regeln, unabhängigen Zufallsentscheidungen und Windows-Systemzufall über rand/OsRng.
- Profil 0 verwendet 6.907 deutsche Wörter. Ab Profil 1 stehen 20.697 unterschiedliche DE/EN/LA-Wörter bereit. Die Sprache wird je Wort unabhängig gewählt. „Nur deutsche Wörter“ überschreibt dies mit sichtbarem Hinweis auf die geringere Stärke.
- Entropie aus der tatsächlichen, nach Sperrliste verbleibenden Quelle. Die Konzeptziele 7.000/21.000 sind keine behaupteten Listengrößen. Vier Wörter mit festen Minuszeichen ergeben ohne Ausschlüsse etwa 51,0 bzw. 57,3 Bit.
- Optionale Rotation über 0–12 mit der jeweiligen Quelle, konservativem Minimum und **+0,0 Bit Profilbonus**. Manuelle Nacharbeit entwertet die Bewertung.
- Farbiger, bearbeitbarer Editor, geschütztes Kopieren mit 30-Sekunden-Frist, Fokus-Rücksprung und Unicode-Autotype ohne Enter/Submit.
- Eigene Sperrliste mit Wortschaltflächen und Verwaltung. Nur ausdrücklich gewählte Wörter werden gespeichert, keine Passwörter.
- KeePassXC-Eintrag nach Bestätigung und optionale Violentmonkey-Kontextvorschläge mit bewusster Kopplung.
- Frühe Prozesshärtung mit Sicherheitsreport; standardmäßig aktiver Windows-Bildschirmaufnahmeschutz.

Eigene Profile, externe Listen in SQLCipher/DPAPI und Integritätsmanifeste sind
als Bibliotheks-APIs vorhanden. Eine GUI zum Bearbeiten eigener Profile oder zum
Store-Import fehlt noch. [Roadmap](docs/ROADMAP.md) und
[Architektur](docs/ARCHITECTURE.md) beschreiben den Stand.

## Installation und Build

Noch kein veröffentlichter Installer. Mit Rust stable/MSVC (mindestens Rust 1.95
für den Workspace), Visual Studio C++ Build Tools und Windows SDK bauen.
SQLCipher/OpenSSL benötigt zusätzlich vollständiges natives Perl, etwa Strawberry
Perl; Git-Perl genügt nicht. Kein Wortlisten-Download zur Laufzeit.

Im betreuten Windows-Build die vorhandene Umgebung laden, nicht verändern:

```powershell
. '<Hauptcheckout>\target\Enter-KairosBuildEnv.ps1'
$kairosTargetDir = '<Buildverzeichnis>'
cargo fmt --check
cargo clippy --workspace --all-targets --target-dir $kairosTargetDir -- -D warnings
cargo test --workspace --target-dir $kairosTargetDir
node --check browser/kairos-bridge.user.js
node browser/test-userscript.cjs
cargo build --release -p kairos --target-dir $kairosTargetDir
& (Join-Path $kairosTargetDir 'release\kairos.exe')
```

Auf anderen Rechnern den Target-Pfad anpassen und MSVC/Perl selbst bereitstellen.
`Cargo.lock` gehört zur Anwendung. Die EXE kann in ein lokales Verzeichnis kopiert
werden; Netzlaufwerke brauchen wegen der DLL-Policies eine Kompatibilitätsprüfung.
Release-Verteilung und Signierung sind noch separat abzunehmen.

## Bedienung und Hotkeys

Alle globalen F-Tasten verwenden **Strg+Umschalt**.

| Taste/Aktion | Wirkung |
|---|---|
| Strg+Umschalt+F9 | Zielfeld merken, frisch generieren, Kompakteditor öffnen; danach bewusst „Einfügen“ wählen. |
| Strg+Umschalt+F10 | Frisch generieren, direkt ins gemerkte Feld tippen und geschützt kopieren. |
| Strg+Umschalt+F11 | Frisch generieren, URL/Benutzername/Gruppe bestätigen und KeePassXC-Eintrag anfordern. Erst ein neuer Klick tippt nach angenommener Anfrage. |
| Strg+R | Neues Passwort. |
| Strg+C / Kopieren | Ganzes aktuelles Passwort geschützt kopieren. |
| Kopieren und zurückspringen | Kopieren und zum ursprünglichen Fenster zurückkehren. |

Beim normalen Start ist „Einfügen“ bis zum ersten F9-Auftrag gesperrt. Belegte
Hotkeys erscheinen mit Fehler und Neuregistrierungsschaltfläche. `--popup`
startet kompakt. `--allow-screencapture` erlaubt Aufnahmen nur für diesen Start
und zeigt dauerhaft „Bildschirmaufnahme erlaubt“.

Die [Violentmonkey-Anleitung](browser/README.md) beschreibt Installation,
Pairing, KeePassXC-Testdatenbank und Wiederherstellung. Vorschläge werden bewusst
übernommen. Kairos sendet keine Formulare und betreibt keinen Cloud-Sync.

## Sperrliste und lokale Dateien

„Sperrliste verwalten“ bietet Wörter des aktuellen **unbearbeiteten** Passworts
zum Sperren an; Großschreibung und eingefügte Ziffern werden rückgängig gemacht.
Alternativ ein Wort eingeben oder Einträge entfernen. Unbearbeitete Ergebnisse
werden nach Änderungen neu erzeugt; manuelle Nacharbeit bleibt bis zum nächsten
Generieren erhalten. Lesefehler leeren und sperren weiter.
Standardpfad ist `%APPDATA%\Kairos\blocked-words.txt`.
`KAIROS_BLOCKED_WORDS_FILE` kann für Tests einen eigenen Dateipfad vorgeben.

Eine Zeile je Wort, 3–9 Buchstaben a–z. Rand-Leerraum und ASCII-Großschreibung
werden normalisiert, Duplikate entfernt. Keine Transliteration; UTF-8-BOM und
leere Zeilen sind erlaubt. Das Leselimit beträgt 1 MiB. Die Textdatei ist
unverschlüsselt, erbt Verzeichnisrechte und kann
Vorlieben verraten. Wörter bereits verwendeter Passwörter nicht sperren: Die Datei
verrät dann Passwortbestandteile. Sie enthält keine automatisch protokollierten Passwörter.
Ungültige/unlesbare Dateien sperren Generierung bis zur Korrektur und „Datei neu
laden“. Eine fehlende Datei ist leer. Eine App-Instanz je Datei zum Bearbeiten
nutzen; externe Änderungen ausdrücklich neu laden.

## Sicherheitsgrenzen

Der Sicherheitsreport zeigt jede Prozessmaßnahme und numerische Fehler.
Fehler sind sichtbar und nicht fatal. Härtung begrenzt WER-Snapshots, neue
Prozesshandles und DLL-Ladevorgänge. Vorhandene Handles, Administratoren mit
Debugprivileg und weitere Dump-Pfade bleiben möglich; Diagnosewerkzeuge können
beeinträchtigt werden. Kein Schutzversprechen gegen lokale Schadsoftware.

Der Editor hält vorab reserviertes `Zeroizing<String>`. `LockedSecret` ist
vorhanden, aber im editierbaren App-Puffer noch nicht integriert. Core,
egui-Layouts, Undo-Temporäre, Clipboard, Grafiktreiber und Zielprogramme können
weitere Kopien halten. Kein Passwortverlauf. Clipboard-Kennzeichnungen wirken
nur bei beachtender Software; Absturz oder dauerhafte Belegung können das Leeren
verhindern.

Windows-Annahme von Autotype bestätigt keine Feldzustellung. Browser-DOM-Felder
können dasselbe HWND teilen. Fokuswechsel/Teileingaben bleiben möglich; keine
automatische Wiederholung. Aufnahmeschutz hängt von DWM, Windows-Version und
Capture-Pfad ab; Kameras, privilegierte Tools und manche Fernwartung sind
ausgenommen. Echte Teams-/Snipping-/KeePassXC-Abnahme bleibt Aufgabe der
[manuellen Gesamtcheckliste](docs/MANUAL-TEST.md).

## Open-Source-Vorbilder und Lizenzen

Prinzipien werden modelliert, kein fremder Anwendungscode kopiert. rand/getrandom
liefert Systemzufall und Gleichverteilung als direkte Ausnahme. KeePassXC,
Password Tech und xkcdpass liefern dokumentierte Prinzipien; kompatible
Bibliotheken sind reguläre Abhängigkeiten. [Quellenindex](docs/SOURCES.md).

Projektcode steht unter [MIT](LICENSE-MIT) oder [Apache-2.0](LICENSE-APACHE).
Daten/Schriften haben eigene Bedingungen. DE-Daten sind CC0-1.0; EN/LA-Herkunft
und vollständige Hinweise stehen in
[NOTICE.txt](crates/kairos-wordlists/data/languages/NOTICE.txt). Die App zeigt
auch diese Hinweise und Fontlizenzen unter „Lizenzen und Wortherkunft“.
