# kairos

Native Windows-App mit eframe/egui 0.37.0. Der normale Einstieg generiert aus
`kairos_wordlists::german()`, derzeit 7.130 eingebettete ASCII-Wörter mit
CC0-Datenlizenz. Profil 0 liefert vier Wörter und rund 51,2 Bit. Profil 1
verwendet dieselbe Quelle; die Empfehlung von 21.000 Wörtern erzeugt keine
zusätzlichen Auswahlbits. `german_source()` leiht die Liste ohne Wortkopien.
`KairosApp::new` akzeptiert weiter `Box<dyn WordSource>` für spätere kombinierte
Quellen oder Store-Snapshots. PR #7 und eine Store-Importoberfläche sind nicht
Bestandteil dieser Integration. Core-, Plattform- und Quellenverträge bleiben
getrennt; die KeePassXC-Crate ist noch ein Scaffold.

## Start und Bedienung

In PowerShell zuerst die bereitgestellte Build-Umgebung laden:

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos
cargo run -p kairos -- --popup
```

Profile 0–12 und 20 Trennerausschlüsse werden an Core übergeben. Strg+R oder
„Neu generieren“ ersetzt das Passwort. Bei sauberem Generatorergebnis erzeugen
neue Einstellungen sofort neu; manuelle Nacharbeit bleibt bis zur nächsten
Generierung erhalten. Die GUI zeigt ausschließlich die quellenabhängigen
Core-Metadaten. Nach Änderungen steht „bearbeitet – nicht mehr bewertbar“.

„Profil bei jeder Generierung wechseln“ zieht bei jeder Generierung erneut
gleichverteilt aus Profil 0–12. Wiederholungen sind möglich. Die feste Profilwahl
ist währenddessen deaktiviert. Der Core liefert die konservative Stärke der
gesamten Rotation und `rotation_bonus_bits`. Hier sind es **+0,0 Bit** wegen
überlappender Ausgaben und ungleicher Profilstärken. Die Anzeige nennt die
schwächste Stärke; eine Rotation kann gegenüber einem starken festen Profil
schwächer sein. Ausschlüsse müssen für alle Profile gelten, deshalb ist ein
ausgeschlossenes Minuszeichen in dieser Rotation ungültig.

| Aktion | Bedienung und Ergebnis |
|---|---|
| Kopieren | Strg+C oder Schaltfläche kopiert das ganze aktuelle Passwort geschützt, auch nach Nacharbeit. |
| Einfügen in Kairos | Strg+V nur bei Fokus im Passworteditor; Steuerzeichen werden entfernt. |
| Kopieren und zurückspringen | Kopiert, gibt den Fokus über das vor App-/Popup-Öffnung erfasste Ziel zurück und minimiert erst nach erfolgreicher Rückgabe. Bei Fehler bleibt Kairos offen. |
| Strg+Umschalt+F9 | Merkt das externe Zielfeld vor dem Wakeup, generiert und zeigt den kompakten Editor. Nachbearbeiten und erneute Generierungen sind möglich. |
| Einfügen | Bestätigt das ursprüngliche Ziel nach der Bearbeitung, gibt Fokus zurück und tippt über die Plattform. Optional auch kopieren, standardmäßig an. Der Auftrag wird verbraucht; kein doppeltes Tippen durch einen zweiten Klick. |
| Strg+Umschalt+F10 | Generiert, tippt direkt ins erfasste Ziel und kopiert nach angenommener Eingabe. Fordert keinen Fokus für das Hauptfenster an. Fehler stehen im App-Hinweis und gegebenenfalls im Titel/Taskleistenhinweis. |
| Strg+Umschalt+F11 | Sichtbarer Hinweis „KeePassXC-Anbindung folgt (PR-6)“. Keine Texteingabe, kein Kopieren und kein Tresoreintrag. |

Der Dienst wird einmal mit `GlobalHotkeys::with_wakeup` gestartet. Der Callback
fordert nur `request_repaint` an. `eframe::App::logic` verarbeitet das Ereignis
auch bei verborgenem Fenster. `ui` zeichnet den Editor. Kein zusätzlicher
Polling-Timer ist nötig. Belegte Hotkeys bzw. Registrierungsfehler erscheinen
mit einer Schaltfläche zum erneuten Registrieren. Die App akzeptiert kein
Eingabeziel im eigenen Prozess. Ohne erfassbares Ziel bleibt F9 zum Generieren
und Kopieren verfügbar. Ein Klick auf das schon offene Taskleistenfenster
ersetzt das frühere Ziel nicht; für ein neues Ziel F9 dort erneut auslösen.

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
kann die Löschung verhindern. Speicherhärtung folgt in PR-9a, Wortlistenschutz
in PR-9b. Fensterinhalte sind weiterhin in Bildschirmaufnahmen sichtbar;
PR-11 plant den gesonderten Bildschirmfreigabeschutz.

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
Original-Lizenzdateien von dys2p und enz/german-wordlist. Kairos wählt für die
Wortdaten CC0-1.0; Auswahl, Zusammenführung und Filter sind dort dokumentiert.

## Tests und öffentliche Demo

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release -p kairos
cargo test -p kairos-platform --test isolated_clipboard -- --ignored
cargo test -p kairos-platform --test windows_interaction -- --ignored --test-threads=1
```

Die letzten zwei Befehle laufen nur ausdrücklich. Der isolierte Test nutzt eine
private unsichtbare Windowstation. Interaktionstests fokussieren echte Fenster,
registrieren Hotkeys und tippen ausschließlich öffentliche Fixtures; auf einem
ruhenden Testdesktop ausführen. GUI-Tests verwenden mockbare Clipboard- und
Zielgrenzen. Sie prüfen echte german()-Generierung, Hotkey-Aktionen, fehlende
Ziele, Abbruch/Teileingabe, Fokus-Rückgabe, Einfüge-Opt-out, Entropieentwertung,
Rotation, TextEdit-Undo und Ausgabe ohne normalen egui-Clipboard-Export.

Die Debug-Demo enthält das öffentliche Beispiel `Ahorn-Wolke7-Kiesel-Falke`
und einen ausdrücklich fiktiven Wert von 82 Bit. `--release --features demo`
wird durch `compile_error!` abgewiesen. Eine Demo ist kein Zufallsgenerator;
globale Hotkeys sind darin deaktiviert.

```powershell
$env:EFRAME_SCREENSHOT_TO = Join-Path (Get-Location) 'target\gui-evidence\gui-demo.png'
cargo run -p kairos --features demo
Remove-Item Env:EFRAME_SCREENSHOT_TO
```

Der gezeigte Befehl rendert die native Demo, speichert nach dem zweiten Frame
und beendet sie. Für eine ausdrücklich lokale Aufnahme mit echter Wortquelle
lässt sich stattdessen `cargo run --release -p kairos --features eframe/__screenshot`
verwenden. Dieses bestehende eframe-Feature aktiviert den Renderer-Bildexport,
ohne Kairos' Demo einzuschalten. Der normale Release-Build aktiviert es nicht.
Screenshots bleiben lokal unter `target/`; verwendete Passwörter gehören nicht
in Screenshots oder Logs. Bildbelege mit echten Quellen dürfen ausschließlich
frisch erzeugte, nirgendwo verwendete Testpasswörter zeigen.

## Open-Source-Vorbilder

- egui/eframe 0.37.0 (MIT OR Apache-2.0) liefert Widgets, LayoutJobs, `App::logic` und Wakeup-Vertrag.
- global-hotkey 0.9.0 (MIT OR Apache-2.0) modelliert das Weiterreichen nativer Ereignisse. Den bestehenden Plattformdienst verwendet die App als Bibliothek.
- KeePassXC 2.8.0 (verlinkte Dateien GPL-2.0 OR GPL-3.0) liefert Zielidentität vor dem Popup, Abbruch bei Fokuswechsel und separate Clipboard-Lebensdauer als Prinzipien.
- Password Tech (GPL-2.0-or-later) liefert den kurzen Generierungs-/Bearbeitungsablauf als Prinzip.
- pwgen-go liefert die Trennung eingebetteter Wortquellen und Generator als Prinzip. Wortdaten und Lizenzen stammen aus kairos-wordlists.
- clipboard-win 5.4.3 (BSL-1.0) bleibt ausschließlich als normaler lesender Windows-Clipboard-Adapter. arboard ist keine direkte App-Abhängigkeit mehr; eframe kann es transitiv weiterhin einbinden.
- JetBrains Mono NL 2.304 (SIL OFL 1.1) bleibt das ausdrücklich lizenzierte unveränderte Font-Asset.

Kein fremder Anwendungscode wurde kopiert. Versionierte Quellen, Lizenzbelege,
Bewertungen und verworfene Ansätze stehen in [gui.md](../../docs/sources/gui.md)
und [platform.md](../../docs/sources/platform.md).
