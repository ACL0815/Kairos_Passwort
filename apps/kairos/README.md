# kairos

Native Windows-App mit eframe/egui 0.37.0. Profil 0 nutzt `german()` mit 6.907,
Profil 1–12 `combined()` mit 20.697 unterschiedlichen DE/EN/LA-Wörtern. Sprache
je Wort unabhängig, Deutsch-Override mit Hinweis auf fehlende Profil-1-Bits.
Die Sperrliste verkleinert die tatsächliche Quelle vor Auswahl und Bitrechnung.
Core-, Plattform- und Quellenverträge bleiben getrennt. Die App ist mit
KeePassXC, Browser-Brücke und Prozesshärtung verdrahtet; Store-Import bleibt API.

## Start und Bedienung

In PowerShell zuerst die bereitgestellte Build-Umgebung laden:

```powershell
# Den vom Projekt bereitgestellten Enter-KairosBuildEnv.ps1-Helfer laden.
# Beispiel: . '<Hauptcheckout>\target\Enter-KairosBuildEnv.ps1'
$kairosTargetDir = '<Buildverzeichnis>'
cargo run --target-dir $kairosTargetDir -p kairos
cargo run --target-dir $kairosTargetDir -p kairos -- --popup
cargo run --target-dir $kairosTargetDir -p kairos -- --allow-screencapture
cargo run --target-dir $kairosTargetDir -p kairos -- --popup --allow-screencapture
```

Profile 0–12 und 20 Trennerausschlüsse werden an Core übergeben. Strg+R oder
„Neu generieren“ ersetzt das Passwort. Bei sauberem Generatorergebnis erzeugen
neue Einstellungen sofort neu; manuelle Nacharbeit bleibt bis zur nächsten
Generierung erhalten. Die GUI trennt Konzeptreferenzen, quellenabhängige
Core-Metadaten und einen definierten Zufallszeichen-/G50-Vergleich. Nach
Änderungen steht „bearbeitet – nicht mehr bewertbar“.

„Profil bei jeder Generierung wechseln“ zieht bei jeder Generierung erneut
gleichverteilt aus mindestens zwei frei gewählten Profilen 0–12. Aktivierung und
Auswahl stehen ohne Passwörter in `%APPDATA%\Kairos\settings.json`, für Tests
über `KAIROS_SETTINGS_FILE` überschreibbar. Wiederholungen sind möglich. Die feste Profilwahl
ist währenddessen deaktiviert. Der Core prüft jede Profilquelle und liefert die
konservative Stärke der gesamten Rotation und `rotation_bonus_bits`.
Die App nutzt derzeit immer den Mehrquellen-Pfad, auch mit „Nur deutsche Wörter“.
Dieser vergibt aktuell für jede auswählbare Rotation **+0,0 Bit**. Ungleiche
Stärken oder überlappende Ausgaben verhindern einen Bonus zusätzlich. Die Anzeige nennt die
schwächste Stärke; eine Rotation kann gegenüber einem starken festen Profil
schwächer sein. Ausschlüsse müssen für alle Profile gelten, deshalb ist ein
ausgeschlossenes Minuszeichen bei ausgewähltem Profil 0 oder 1 ungültig.
„Profile erklärt“ zeigt Regeln, Konzeptgewinn, öffentliche Beispiele und
aktuelle Einzelwerte für alle 13 Profile. ⓘ bietet Hover- und Klick-Hilfe.
Im Kompaktmodus liegen „Nur deutsche Wörter“ und die Quellgrößen unter
„Wortquelle“; Profilwahl, Passwort und Stärke erhalten so mehr Platz.
Die Rechenmodelle und Dateifehler sind im
[Haupt-README](../../README.md#profile-und-stärke) beschrieben.

| Aktion | Bedienung und Ergebnis |
|---|---|
| Kopieren | Strg+C oder Schaltfläche kopiert das ganze aktuelle Passwort geschützt, auch nach Nacharbeit. |
| Einfügen in Kairos | Strg+V nur bei Fokus im Passworteditor; Steuerzeichen werden entfernt. |
| Kopieren und zurückspringen | Kopiert, gibt den Fokus über das vor App-/Popup-Öffnung erfasste Ziel zurück und minimiert erst nach erfolgreicher Rückgabe. Bei Fehler bleibt Kairos offen. |
| Strg+Umschalt+F9 | Merkt das externe Zielfeld vor dem Wakeup, generiert und zeigt den kompakten Editor. Nachbearbeiten und erneute Generierungen sind möglich. |
| Einfügen | Erst nach F9 im gewünschten Zielfeld aktiv. Bestätigt dieses Ziel nach der Bearbeitung, gibt Fokus zurück und tippt über die Plattform. Optional auch kopieren, standardmäßig an. Der Auftrag wird verbraucht; kein doppeltes Tippen durch einen zweiten Klick. |
| Strg+Umschalt+F10 | Generiert, tippt direkt ins erfasste Ziel und kopiert nach angenommener Eingabe. Fordert keinen Fokus für das Hauptfenster an. Fehler stehen im App-Hinweis und gegebenenfalls im Titel/Taskleistenhinweis. |
| Strg+Umschalt+F11 | Frisch generieren, URL/Benutzername/Gruppe bestätigen, KeePassXC-Anfrage; erst nach Annahme mit neuem Klick einmalig ins ursprüngliche Feld tippen. Browser-Brücke liefert nur ausdrücklich übernommene Vorschläge. |

Der Dienst wird einmal mit `GlobalHotkeys::with_wakeup` gestartet. Der Callback
fordert nur `request_repaint` an. `eframe::App::logic` verarbeitet das Ereignis
auch bei verborgenem Fenster. `ui` zeichnet den Editor. Kein zusätzlicher
Polling-Timer ist nötig. Belegte Hotkeys bzw. Registrierungsfehler erscheinen
mit einer Schaltfläche zum erneuten Registrieren. Die App akzeptiert kein
Eingabeziel im eigenen Prozess. Ohne erfassbares Ziel bleibt F9 zum Generieren
und Kopieren verfügbar. Ein Klick auf das schon offene Taskleistenfenster
ersetzt das frühere Ziel nicht; für ein neues Ziel F9 dort erneut auslösen.
Das beim normalen oder kompakten Start gemerkte Fenster berechtigt ausschließlich
zum Fokus-Rücksprung. „Einfügen“ bleibt dabei deaktiviert, damit ein Start aus
Terminal oder Explorer keine Eingabe dorthin freigibt. Erst F9 im beabsichtigten
Zielfeld erzeugt einen Editorauftrag. Ohne gemerktes Ziel bleibt auch „Kopieren
und zurückspringen“ deaktiviert.

## Fokus und Auto-Type

Das Ziel enthält Fenster-, Prozess-, Thread- und native Control-Identität,
keinen Text. Die App erfasst nach dem Öffnen des Editors kein neues Ziel.
Direktes F10 verwendet `FocusTarget::type_text` mit der Fünf-Sekunden-Frist.
Der ausdrücklich bestätigte Editor-Klick verwendet additiv
`restore_and_type_text`: Erst das Originalziel zurückgeben und prüfen, dann
eine frische Eingabefrist starten. So bleibt Nachbearbeitung auch nach mehr als
fünf Sekunden möglich. Modifier und Hotkey-Haupttaste müssen freigegeben sein;
höhere Integritätsstufen, zerstörte oder veränderte Ziele werden abgewiesen.

Es gibt keine automatische Wiederholung, keine Eingabe von Tab/Enter und
keinen Clipboard-Fallback nach Eingabefehlern. Nach angenommener Eingabe und
fehlgeschlagenem Kopieren erklärt der Hinweis den Teilerfolg; nur Kopieren
soll erneut versucht werden. Der Nutztext wird nicht in Fehlern ausgegeben.
Nach einem Einfügefehler bleibt die Nacharbeit erhalten und kann kopiert werden.
Ein erneutes F9 erzeugt ein neues Passwort; die Fehlermeldungen erklären dies.

**Grenzen:** Windows kann Fokusaktivierung verweigern. `SendInput` hat keinen
atomaren Zielparameter. Mehrere DOM-Felder können dasselbe native HWND teilen,
sodass ein Feldwechsel im Browser unbemerkt bleiben kann. Windows-Annahme ist
keine bestätigte Zustellung; `InputIncomplete` kann Teiltext hinterlassen.
Lokale Hooks, Eingabequeue und Zielfeld erzeugen weitere Geheimniskopien.
[MANUAL-TEST.md](../../docs/MANUAL-TEST.md) enthält die Desktop-Abnahme.

## Geheimnisse und Zwischenablage

Ein aktuelles Passwort liegt in `Zeroizing<String>`, ohne Logs, Dateispeicherung
oder Generatorverlauf. Der Puffer behält seine vorab reservierten 16 KiB und
wird vor Ersetzen und beim Freigeben überschrieben. Bis 4096 Unicode-Zeichen
passen ohne Reallokation hinein; Kopieren und Auto-Type akzeptieren höchstens
1024 UTF-16-Einheiten gemäß Plattformgrenze. Übergroße bzw. steuerzeichenhaltige
Core-Ergebnisse werden abgewiesen. egui-Persistenz ist deaktiviert.

Das einzeilige `TextEdit::password(true)` nutzt einen sichtbaren farbigen
Layouter mit visuellem Umbruch. Ziffern sind Bernstein, Trenner Magenta,
Großbuchstaben Cyan und unterstrichen. Gültige Core-UTF-8-Bytebereiche werden
nicht als Zeichenindizes interpretiert. Enter fügt keinen Zeilenumbruch ein.
Undo wird nach jedem Frame geleert, beim Neugenerieren der Editorzustand entfernt.
Normale eframe-Zwischenablage-Shortcuts sind deaktiviert. `ClipboardAccess`
ist die mockbare App-Grenze; keine Geheimtextkopie wird zum Worker geschickt.

Der arboard-Übergangsadapter ist ersetzt. Schreiben erfolgt ausschließlich mit
`kairos_platform::copy_secret(text, Some(DEFAULT_CLEAR_AFTER))`. Alle drei
Windows-Ausschlüsse stehen vor dem Text in derselben OS-Transaktion. Der
Plattform-Timer leert nach 30 Sekunden nur die noch eigene Schreibmarke, auch
bei minimierter GUI. Der Adapter behält nur die letzte `ClipboardReceipt` und
versucht beim normalen App-Ende deren bedingte Löschung. Fremde spätere Inhalte
bleiben erhalten. Es gibt keinen ungeschützten Schreibfallback. Strg+V liest
über clipboard-win unter RAII-Sperre in einen eigenen Zeroizing-Bytepuffer und
anschließend einen Zeroizing-String.

**Speichergrenzen:** egui erzeugt temporäre `String`-Snapshots, Paste-Ereignisse,
Layout-/Galley- und Renderdaten. Leeren des Undoers überschreibt nicht dessen
interne Strings. Windows, Grafikspeicher, Auslagerung, Dumps und fremde Programme
können weitere Kopien halten. Der Adapter erzeugt für egui ein normales
`Event::Paste(String)`; der darunterliegende Leser kann eigene Zwischenpuffer
anlegen. Kein vollständiger Lösch- oder RAM-Härtungsnachweis. Schutzformate
wirken nur bei beachtender Software. Ein Absturz oder dauerhaft belegtes Clipboard
kann die Löschung verhindern. Prozesshärtung ist beim Start integriert, Wortlistenschutz
in PR-9b. Der folgende Aufnahmeschutz begrenzt unterstützte Windows-Aufnahmewege.

## Bildschirmfreigabe und Aufnahmen

Der Schutz ist bei jedem Start an, auch mit `--popup` und nach F9. Ab Windows 10
2004 schließt `SetWindowDisplayAffinity(WDA_EXCLUDEFROMCAPTURE)` die Kairos-Fenster
aus unterstützten Aufnahmen aus. Ältere Windows-Versionen verwenden `WDA_MONITOR`
und können eine leere Fensterfläche hinterlassen. Root-HWND aus eframe und ein
GUI-Thread-Hook decken spätere Viewports und native Dialoge vor dem Anzeigen ab.
Der Herkunftsdialog ist ein egui-Fenster innerhalb des geschützten Root-Viewports.

Mit **`--allow-screencapture`** lässt sich der Schutz für diesen Start ausschalten.
Der dauerhaft sichtbare Hinweis **„Bildschirmaufnahme erlaubt“** steht über dem
Editor, auch in der Kompaktansicht. **„Schutz nicht verfügbar“** meldet fehlenden,
fehlgeschlagenen oder nicht bestätigten Schutz. Der Monitor-Fallback erhält einen
eigenen Hinweis. Neugenerieren und Hotkeys entfernen diese Hinweise nicht.

Anzeigeaffinität ist kein Schutz gegen Kameras, privilegierte Admin-/Treiber-
Werkzeuge oder alle Fernwartungsprogramme. Ein erfolgreicher Windows-Status
ersetzt keinen Test des verwendeten Teams-/Capture-Pfads. Für Tests nur frische,
nirgendwo verwendete Passwörter oder die öffentliche Demo verwenden.
[MANUAL-TEST.md](../../docs/MANUAL-TEST.md) enthält die Schritte mit/ohne Opt-out.
Künftige zusätzliche GUI-Threads benötigen jeweils einen eigenen Plattform-Guard.

## Schrift, Lizenzen und Wortherkunft

Die Gestaltung folgt [UI-STYLE.md](../../docs/design/UI-STYLE.md). Das lokale
Stock-Referenzbild wird nicht mitgeliefert. JetBrains Mono NL Regular 2.304 ist
unverändert mit SIL-OFL-Lizenz eingebettet. Markierte Null, unterschiedliche
`1/l/I` und fehlende Ligaturen helfen beim Abschreiben. `5/S` unterscheidet
zusätzlich die Ziffernfarbe. Herkunft, Hash und Lizenz stehen unter
[assets/fonts/](assets/fonts/README.md). eguis Ubuntu-/Hack-/Icon-Fallbacks
haben separate beigefügte Lizenztexte unter
[egui-defaults](assets/fonts/egui-defaults/README.md).

„Lizenzen und Wortherkunft“ zeigt neben den Schriftlizenzen die direkt aus
kairos-wordlists eingebettete `data/PROVENANCE.md` und die vollständigen
Original-Lizenzdateien von dys2p und enz/german-wordlist sowie EN/LA-Provenienz
und NOTICE.txt. Kairos wählt für die DE-Wortdaten CC0-1.0. EN/LA-Bedingungen
gelten gemäß NOTICE.txt; Auswahl, Zusammenführung und Filter sind dokumentiert.

## Tests und öffentliche Demo

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --target-dir $kairosTargetDir -- -D warnings
cargo test --target-dir $kairosTargetDir --workspace
cargo build --release -p kairos --target-dir $kairosTargetDir
cargo test --target-dir $kairosTargetDir -p kairos-platform --test isolated_clipboard -- --ignored
cargo test --target-dir $kairosTargetDir -p kairos-platform --test windows_interaction -- --ignored --test-threads=1
```

Die letzten zwei Befehle laufen nur ausdrücklich. Der isolierte Test nutzt eine
private unsichtbare Windowstation. Interaktionstests fokussieren echte Fenster,
registrieren Hotkeys und tippen ausschließlich öffentliche Fixtures; auf einem
ruhenden Testdesktop ausführen. GUI-Tests verwenden mockbare Clipboard- und
Zielgrenzen. Sie prüfen echte german()-Generierung, Hotkey-Aktionen, fehlende
Ziele, Abbruch/Teileingabe, Fokus-Rückgabe, Einfüge-Opt-out, Entropieentwertung,
Rotation, TextEdit-Undo und Ausgabe ohne normalen egui-Clipboard-Export.

Die Debug-Demo enthält öffentliche Lehrbeispiele je Profil. Die angezeigten
Metadaten stammen aus dem realen Core für die Einstellungen; der sichtbare Text
wird anschließend durch das feste, ausdrücklich unbrauchbare Beispiel ersetzt.
`KAIROS_DEMO_PROFILE` (0–12, sonst 8) wählt das Beispiel,
`KAIROS_DEMO_HELP=1` öffnet die Profilhilfe. Rotation lässt sich über eine eigene
`KAIROS_SETTINGS_FILE` mit z. B. Profilen 2/4/8/12 starten.
`KAIROS_DEMO_LARGE=1` startet die Demo für Gesamtaufnahmen mit 1000×1050 Punkten;
normale Builds und ihre Fenstergrößen bleiben davon unberührt.
Für offene Hilfefenster `KAIROS_DEMO_CAPTURE_TO=<PNG-Pfad>` statt
`EFRAME_SCREENSHOT_TO` verwenden: Dieser reine Demo-Export wartet mehrere
Renderframes ab und beendet anschließend die App.
`--release --features demo`
wird durch `compile_error!` abgewiesen. Eine Demo ist kein Zufallsgenerator;
globale Hotkeys sind darin deaktiviert.

```powershell
$env:EFRAME_SCREENSHOT_TO = Join-Path (Get-Location) 'target\gui-evidence\gui-demo.png'
cargo run --target-dir $kairosTargetDir -p kairos --features demo
Remove-Item Env:EFRAME_SCREENSHOT_TO
```

Der gezeigte Befehl rendert die native Demo, speichert nach dem zweiten Frame
und beendet sie. Für eine ausdrücklich lokale Aufnahme mit echter Wortquelle
lässt sich stattdessen `cargo run --release -p kairos --target-dir $kairosTargetDir --features eframe/__screenshot`
verwenden. Dieses bestehende eframe-Feature aktiviert den Renderer-Bildexport,
ohne Kairos' Demo einzuschalten. Der normale Release-Build aktiviert es nicht.
Screenshots bleiben lokal unter `target/`; verwendete Passwörter gehören nicht
in Screenshots oder Logs. Bildbelege mit echten Quellen dürfen ausschließlich
frisch erzeugte, nirgendwo verwendete Testpasswörter zeigen.

## Open-Source-Vorbilder

### F11 / KeePassXC und Browser

Strg+Umschalt+F11 öffnet einen eigenen Bestätigungsdialog mit frischem Passwort,
URL, Benutzername und optionaler bestehender Gruppe. KeePassXC läuft in einem
Worker; erst nach Annahme erlaubt ein neuer Klick die einmalige Eingabe über das
ursprüngliche `FocusTarget`. Ablehnung/Timeout tippt nicht, Fehler nach Annahme
nennen das Teilergebnis. Während Anfragen laufen sind neue Hotkey-Aufträge gesperrt.

Modelled after: KeePassXC-Association und bewusste Autotype-Freigabe, 2FAS-Pairing,
Syncthing-Host-/CSRF-Schutz und isolierte Violentmonkey-GM-APIs. Keine Übernahme
fremden Codes. Die neue Kontext-Crate verwendet den existierenden DPAPI-Adapter
mit separatem Blob/Marker. Anleitung und Grenzen:
[browser/README.md](../../browser/README.md), Quellen:
[browser.md](../../docs/sources/browser.md), [keepassxc.md](../../docs/sources/keepassxc.md).

- egui/eframe 0.37.0 (MIT OR Apache-2.0) liefert Widgets, LayoutJobs, `App::logic` und Wakeup-Vertrag.
- global-hotkey 0.9.0 (MIT OR Apache-2.0) modelliert das Weiterreichen nativer Ereignisse. Den bestehenden Plattformdienst verwendet die App als Bibliothek.
- KeePassXC 2.8.0 (verlinkte Dateien GPL-2.0 OR GPL-3.0) liefert Zielidentität vor dem Popup, Abbruch bei Fokuswechsel und separate Clipboard-Lebensdauer als Prinzipien.
- KeePassXC modelliert außerdem den standardmäßig aktiven Aufnahmeschutz und `--allow-screencapture`, belegt durch [main.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp) und [WinUtils.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp). Eigene Rust-Umsetzung, kein übernommener Anwendungscode.
- Password Tech (GPL-2.0-or-later) liefert den kurzen Generierungs-/Bearbeitungsablauf als Prinzip.
- pwgen-go liefert die Trennung eingebetteter Wortquellen und Generator als Prinzip. Wortdaten und Lizenzen stammen aus kairos-wordlists.
- clipboard-win 5.4.3 (BSL-1.0) bleibt ausschließlich als normaler lesender Windows-Clipboard-Adapter. arboard ist keine direkte App-Abhängigkeit mehr; eframe kann es transitiv weiterhin einbinden.
- JetBrains Mono NL 2.304 (SIL OFL 1.1) bleibt das ausdrücklich lizenzierte unveränderte Font-Asset.

Kein fremder Anwendungscode wurde kopiert. Versionierte Quellen, Lizenzbelege,
Bewertungen und verworfene Ansätze stehen in [gui.md](../../docs/sources/gui.md)
und [platform.md](../../docs/sources/platform.md).


## Abschlussintegration (26.10.2026)

`main` ruft `harden_process()` vor eframe auf. Sicherheitsreport und Warnstatus
stehen auch im kompakten/F11-Fluss bereit; fehlgeschlagene Maßnahmen bleiben
nicht fatal. Bestehende Handles, privilegierte Leser und manche Dumps bleiben
außerhalb des Schutzes. Der editierbare Zeroizing-String bleibt erhalten.
LockedSecret braucht einen eigenen UTF-8-TextBuffer-Adapter und direkte Erzeugung,
sonst würde lediglich eine weitere Passwortkopie entstehen. Diese Integration
bleibt Folgearbeit, einschließlich Lock-/Close-Fehleranzeige und Speichermessung.

„Sperrliste verwalten“ persistiert nur explizit gewählte Einzelwörter in
`%APPDATA%\Kairos\blocked-words.txt`. Testpfad über `KAIROS_BLOCKED_WORDS_FILE`.
ASCII-Kleinschreibung, 3–9 Buchstaben a–z, keine Transliteration. Großschreibung
und innere Ziffern werden bei Wortschaltflächen rückgängig gemacht; diese gibt
es nur für unbearbeitete Ergebnisse. Datei unverschlüsselt mit geerbten Rechten,
kann Vorlieben verraten. Schreibfehler erhalten bisherigen Filter; ungültige/
unlesbare Dateien sperren Generierung bis Korrektur/Neuladen. Eine Instanz je Datei.
Details und Gesamtprüfung im [Haupt-README](../../README.md) und
[MANUAL-TEST.md](../../docs/MANUAL-TEST.md).

Modelled after: xkcdpass für Filter vor Sampling/Entropie und KeePassXC Bootstrap
für frühe Prozesshärtung. Kein Vorbildcode kopiert. Permalinks/Lizenzen stehen in
[profiles.md](../../docs/sources/profiles.md), [gui.md](../../docs/sources/gui.md)
und [protection.md](../../docs/sources/protection.md).
