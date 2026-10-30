<img src="assets/icon.svg" width="64" height="64" alt="">

# Kairos Passwort

Wortpasswort-Generator für Windows x64 mit Rust und egui. Erzeugt ein Passwort,
macht es gut abschreibbar und übergibt es an ein zuvor gewähltes Feld.
Das [Grundkonzept](docs/Grundkonzept.md) bleibt die unveränderte Konzeptquelle.

## Funktionen

- Profile 0–12 mit kumulativen Regeln, unabhängigen Zufallsentscheidungen und Windows-Systemzufall über rand/OsRng.
- Profil 0 verwendet 6.907 deutsche Wörter. Ab Profil 1 stehen 20.697 unterschiedliche DE/EN/LA-Wörter bereit. Die Sprache wird je Wort unabhängig gewählt. „Nur deutsche Wörter“ überschreibt dies mit sichtbarem Hinweis auf die geringere Stärke.
- Entropie aus der tatsächlichen, nach Sperrliste verbleibenden Quelle. Die Konzeptziele 7.000/21.000 sind keine behaupteten Listengrößen. Vier Wörter mit festen Minuszeichen ergeben ohne Ausschlüsse etwa 51,0 bzw. 57,3 Bit.
- Optionale Rotation aus mindestens zwei frei gewählten Profilen 0–12, mit der jeweiligen Quelle und konservativer Core-Bewertung. Auswahl und Aktivierung bleiben lokal gespeichert. Manuelle Nacharbeit entwertet die Bewertung.
- Farbiger, bearbeitbarer Editor, geschütztes Kopieren mit 30-Sekunden-Frist, Fokus-Rücksprung und Unicode-Autotype ohne Enter/Submit.
- Eigene Sperrliste mit Wortschaltflächen und Verwaltung. Nur ausdrücklich gewählte Wörter werden gespeichert, keine Passwörter.
- KeePassXC-Eintrag nach Bestätigung und optionale Violentmonkey-Kontextvorschläge mit bewusster Kopplung.
- Frühe Prozesshärtung mit Sicherheitsreport; standardmäßig aktiver Windows-Bildschirmaufnahmeschutz.
- System-Tray mit Öffnen, frisch generieren und geschützt kopieren, Autostart und Beenden. Globale Hotkeys bleiben bei verborgenem Fenster aktiv.
- Optionaler Autostart und persönlicher Windows-Startmenü-Eintrag ohne Administratorrechte; Einzelinstanz je Windows-Konto und Sitzung.

Eigene Profile, externe Listen in SQLCipher/DPAPI und Integritätsmanifeste sind
als Bibliotheks-APIs vorhanden. Eine GUI zum Bearbeiten eigener Profile oder zum
Store-Import fehlt noch. [Roadmap](docs/ROADMAP.md) und
[Architektur](docs/ARCHITECTURE.md) beschreiben den Stand.

## Profile und Stärke

„Profile erklärt“ und die kleinen **ⓘ**-Elemente erläutern jede Profilregel,
den Gewinn gegenüber dem Vorgänger und ein öffentliches Lehrbeispiel.
Diese Beispiele niemals als Passwort verwenden. Die Hilfen erklären auch
Trennerausschlüsse, Wortquellen, Sperrliste, Hotkeys, Zwischenablage,
Bildschirmschutz, Sicherheitsreport und KeePassXC-/Violentmonkey-Anbindung.

Die App trennt drei Angaben:

- **Theoretisch (Konzept):** feste Referenz aus beiden Tabellen des
  [Grundkonzepts](docs/Grundkonzept.md), von 51,1 bis 153,1 Bit. Dieses Modell
  verwendet 7.000 bzw. 21.000 Wörter mit sechs Buchstaben je Wort.
- **Tatsächlich:** konservative Min-Entropie aus dem Core für die reale,
  ggf. gefilterte Wortquelle und die effektiven Trennerregeln. Reale
  Listengröße, kürzere Wörter, „Nur Deutsch“, Sperrliste und Ausschlüsse
  erklären Abweichungen. Künftige Core-Trenneränderungen fließen automatisch ein.
- **Vergleich:** die kürzeste mindestens gleich starke Alternative aus
  gleichverteilt gezogenen Zeichen (94 druckbare ASCII-Zeichen ohne Leerzeichen,
  alle vier Zeichenarten enthalten). Inklusions-/Exklusionsrechnung zählt
  gültige Zeichenfolgen. **G50-Modell = 2^H / 2** Rateversuche, aus dem tatsächlichen
  Core-Wert berechnet und wissenschaftlich mit Dezimalkomma dargestellt.
  Für konservative Min-Entropie ist dies Vergleich/Untergrenze, keine gemessene
  mittlere Knackdauer und keine Prognose für menschliche Passwörter.

Mehr Länge durch bekannte Zusätze, feste Anhänge oder Wiederholungen bringt
keine zusätzlichen Bits. Nur unabhängige Zufallsentscheidungen vergrößern die
Auswahl. Nach manueller Bearbeitung erscheinen weder Stärkevergleich noch Bonus.

Unter „Profil bei jeder Generierung wechseln“ die gewünschten Profile anhaken,
etwa **2, 4, 8 und 12**. Mindestens zwei bleiben ausgewählt. Der Core zieht
bei jeder Generierung gleichverteilt; Wiederholungen sind möglich. Profil 0
verwendet Deutsch, ab Profil 1 die kombinierte Quelle, sofern „Nur deutsche
Wörter“ dies nicht überschreibt. Jedes ausgewählte Profil muss mit den
Ausschlüssen gültig bleiben. Ohne Profil 0/1 darf etwa Minus ausgeschlossen sein.

Die Anzeige bewertet die **gesamte Rotation**, nicht das gerade gezogene Profil.
Die Konzeptreferenz ist dabei das Minimum der Auswahl. Der Profilbonus wird
unverändert aus `rotation_bonus_bits` übernommen: höchstens `log2(Anzahl)`,
im aktuellen Mehrquellen-Core-Pfad **+0,0 Bit**, auch mit „Nur deutsche Wörter“. Ungleich starke oder überlappende
Ausgaben erhalten ebenfalls keinen Bonus. Keine vorhandene Auswahl garantiert
einen Bonus. Dafür wären gleich starke, nachweisbar überschneidungsfreie
Profile mit unbekannter aktueller Wahl nötig. Die optionalen **bis 157,1 Bit**
im Konzept setzen 16 solche Profile voraus (+4 Bit); die 13 vorhandenen
erlauben allein nach ihrer Anzahl höchstens +3,7 Bit.

Aktivierung und Profilnummern stehen in `%APPDATA%\Kairos\settings.json`
(Schema-Version 1, zusätzlich `desktop.close_to_tray` und `desktop.tray_notice_seen`), ohne Passwörter. `KAIROS_SETTINGS_FILE` legt für Tests einen
anderen Dateipfad fest. Fehlende Datei: Rotation aus, alle Profile vorausgewählt.
Ungültige/unlesbare Datei: Rotation aus und sichtbarer Hinweis, kein stilles
Überschreiben; erst eine bewusste Änderung speichert neu. Schreibfehler sind
sichtbar, die Auswahl gilt dann nur für den laufenden Start. Eine Instanz pro
Einstellungsdatei verwenden.

Tray-Änderungen und das Bestätigen des Tray-Hinweises überschreiben eine beim
Start ungültige/unlesbare Einstellungsdatei nicht. Sie gelten dann nur für den
laufenden Prozess, bis die Datei korrigiert oder eine neue Rotationsauswahl
bewusst gespeichert wird. Ältere Kairos-Versionen können das neue `desktop`-Feld
als unbekannt ablehnen; vor einem Downgrade die Einstellungsdatei sichern.

## Installation und Build

Noch kein veröffentlichter Installer. Mit Rust stable/MSVC (mindestens Rust 1.95
für den Workspace), Visual Studio C++ Build Tools und Windows SDK bauen.
SQLCipher/OpenSSL benötigt zusätzlich vollständiges natives Perl, etwa Strawberry
Perl; Git-Perl genügt nicht. Kein Wortlisten-Download zur Laufzeit.
Für die eingebettete Icon-Ressource wird der Ressourcencompiler `rc.exe` des
Windows SDK benötigt. `winresource` findet eine reguläre SDK-Installation;
bei separat entpacktem SDK `RC_PATH` auf dessen vollständigen `rc.exe`-Pfad setzen.

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

## Tray, Autostart und Startmenü

Ein Klick oder Doppelklick auf das K im Infobereich öffnet Kairos und fordert
den Vordergrund an. Windows kann die Fokusübernahme verweigern. Im Rechtsklick-
Menü stehen „Kairos öffnen“, „Neues Passwort kopieren“, „Mit Windows starten“
und „Beenden“. Neues Passwort kopieren verwendet das aktuelle Profil bzw. die
aktuelle Rotation, dieselben Clipboard-Ausschlüsse und dieselbe 30-s-Leerung
wie der Kopieren-Button. Während eines laufenden/angenommenen KeePassXC-Auftrags
wird kein neues Passwort erzeugt. Bei Fehlern wird das Fenster sichtbar.

Das Fenster-X verbirgt Kairos standardmäßig im Tray. Beim ersten Mal erscheint
ein einmaliger Hinweis mit ausdrücklicher Schaltfläche zum Verbergen. Unter
„Windows und Tray“ lässt sich „Beim Schließen in den Tray“ abschalten; dann
beendet das X die App. Ohne verfügbares Tray bleibt das Fenster sichtbar und X
beendet die App. Tray → Beenden beendet wirklich; eine laufende KeePassXC-Anfrage
wird zuerst abgeschlossen. F9/F10/F11 funktionieren auch bei verborgenem Fenster;
F10 fordert keinen Kairos-Fokus an. Der Capture-Schutz bleibt beim Wiederöffnen
aktiv. Der Opt-out gilt weiterhin nur für die zuerst gestartete Instanz.

„Mit Windows starten“ liest und schreibt nur den Wert `Kairos Passwort` unter
`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`: `"<voller EXE-Pfad>" --tray`.
Es gibt keine automatische Registrierung beim Programmstart. Ein anderer Pfad
oder andere Argumente werden angezeigt; „Pfad aktualisieren“ ist ausdrücklich
zu wählen. Eine Task-Manager-Deaktivierung bzw. unbekannte `StartupApproved`-
Markierung wird angezeigt und sperrt Änderungen in der App. Dort wieder
aktivieren; Kairos schreibt oder löscht `StartupApproved` niemals.

„Startmenü-Eintrag“ erstellt/entfernt nur `Kairos Passwort.lnk` im persönlichen
Programme-Ordner des Startmenüs (Known Folder `FOLDERID_Programs`, per
`SHGetKnownFolderPath` ermittelt, berücksichtigt also Umleitungen). Üblich ist
`%APPDATA%\Microsoft\Windows\Start Menu\Programs`; dieser Pfad dient nur als
Fallback, wenn die Windows-API fehlschlägt. Die Verknüpfung entsteht mit
Ziel, Arbeitsverzeichnis, Beschreibung und Icon der aktuellen EXE. Vorhandene
Dateien werden nicht überschrieben; entfernt werden nur Verknüpfungen mit
Kairos-Beschreibung. Nach einem EXE-Umzug den eigenen Link entfernen und neu anlegen.

| Startparameter | Verhalten |
|---|---|
| `--tray` | Ohne sichtbares Fenster direkt im Tray starten, Hotkeys aktiv. Mit `--popup` bleibt der Start verborgen, der Editor ist kompakt. |
| `--popup` | Kompakteditor. Eine zweite Instanz öffnet den vorhandenen Editor kompakt, ohne neues Passwort oder Einfügeberechtigung. |
| `--allow-screencapture` | Aufnahmen nur für diesen ersten Prozess erlauben. Ein zweiter Aufruf ändert die laufende Policy nicht. |
| `--remove-integration` | Run-Wert und eigenen Startmenü-Link ohne Fenster entfernen; Vorrang vor allen anderen Flags. Erfolg: Exitcode 0, Fehler: ungleich 0. Vor dem Löschen der portablen EXE ausführen. |

Ein sitzungsbezogener, nach Benutzer-SID benannter Mutex verhindert doppelte
Hotkey-Registrierungen. Ein zweiter normaler Start (auch `--tray`) signalisiert
„Fenster zeigen“ und endet mit 0. `--popup` wird als separates Ereignis
weitergereicht. Der ursprüngliche Zielauftrag und das Passwort bleiben erhalten.
Die lokale Registry und Startmenü-Ziele werden erst auf ausdrücklichen Wunsch
geändert; für eine portable Nutzung sind beide optional.

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
