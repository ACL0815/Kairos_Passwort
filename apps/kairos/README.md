# kairos

Native Windows-Oberfläche mit eframe/egui 0.37.0. Das Passwort ist unmittelbar
bearbeitbar. Ziffern erscheinen in Bernstein, Trenner in Magenta,
Großbuchstaben in Cyan und Wörter in hellem Blauweiß. Die Core-Segmente
verwenden UTF-8-Bytebereiche; Umlaute verschieben die Hervorhebung nicht.

Profilwahl 0–12 und die 20 ausschließbaren Trenner werden direkt in
`GeneratorConfig` übertragen. Eine Änderung erzeugt ein neues Ergebnis.
Die Bitanzeige und der Stärke-Ring zeigen ausschließlich `GeneratedPassword`-
Metadaten. Nach manueller Bearbeitung lautet der Status dauerhaft
„bearbeitet – nicht mehr bewertbar“, bis erneut generiert wird.

Auf der Scaffold-Basis liefert Core noch `NotImplemented`, und die Wortlisten-
Crate enthält keine Quelle. Die App zeigt das verständlich an und ruft den
panikenden `entropy_bits`-Stub nicht auf. Es gibt keinen Ersatzgenerator.
Die spätere Integration reicht die echte Quelle als `Box<dyn WordSource>` an
`KairosApp::new` und ersetzt `MissingWordSource` im Einstieg. Außer dem
Core-Vertrag werden keine unveröffentlichten APIs vorausgesetzt.

## Start und Bedienung

Unter PowerShell vor jedem Cargo-Aufruf die vorgegebene Build-Umgebung laden:

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos
cargo run -p kairos -- --popup
```

„Neu generieren“ oder Strg+R erzeugt erneut. Strg+C und „Kopieren“ kopieren
immer das ganze Passwort. Strg+V fügt Text an der Cursorposition ein.
„Kopieren und zurückspringen“ minimiert das Fenster erst nach erfolgreichem
Kopieren. Windows wählt damit wieder das zuvor aktive Fenster; es wird keine
Eingabe simuliert. Ob Windows tatsächlich das gewünschte Zielfenster aktiviert,
hängt auch von dessen Fokusregeln ab. `show_popup` ist der Einstieg für die
spätere Hotkey-Verdrahtung und stellt ein minimiertes Fenster wieder her.

## Geheimnisse und Zwischenablage

Die Anwendung hält ein aktuelles Passwort in `Zeroizing<String>`, ohne Logs,
Dateispeicherung oder Generatorverlauf. Der Puffer wird vor dem Ersetzen und
beim Freigeben überschrieben. egui-Persistenz ist deaktiviert.

Das sichtbar farbige Feld nutzt `TextEdit::password(true)` mit eigenem Layouter.
Dadurch exportiert egui den Feldinhalt nicht über die normale Kopieraktion
oder eine Textauswahl-Meldung. Der Undoer wird nach **jedem** Frame geleert,
beim Neugenerieren wird der Editorzustand entfernt. Es gibt absichtlich kein
Undo/Redo für Passwörter. Der normale eframe-Zwischenablagepfad ist deaktiviert;
Kopieren und Einfügen laufen über den App-Adapter.

**Grenzen:** egui erzeugt beim Bearbeiten kurzlebige `String`-Snapshots und
speichert sichtbaren Text in Layout-/Galley-Daten und Renderdaten. Der Undoer
wird geleert, seine internen Strings werden durch egui aber nicht sicher
überschrieben. Auch Einfüge-Ereignisse, frühere String-Allokationen, Glyphen,
Grafikspeicher, Windows-Zwischenablage, Auslagerung und Dumps lassen sich hier
nicht vollständig löschen. Diese GUI beansprucht keine vollständige
Speicherhärtung; dafür ist PR-9 vorgesehen. Accessibility-Anbindung und
Inspektionsserver sind nicht aktiviert, um keine zusätzliche Klartextausgabe
des eigenen Layouters zu eröffnen.

Der **Übergangsadapter** in `clipboard.rs` wird im späteren Verdrahtungs-PR
durch `kairos-platform` ersetzt. Er verwendet arboard 3.7.0 mit Windows-
Kennzeichnungen gegen Verlauf und Cloud-Sync. Ein Worker löscht nach 30 s
auch bei minimiertem Fenster. Die Prüfung von Sequenznummer und eigener Markierung
sowie das Leeren erfolgen unter derselben Windows-Zwischenablagesperre.
Spätere Inhalte anderer Anwendungen bleiben erhalten. Bei einer belegten
Zwischenablage wird die Timeout-Löschung erneut versucht. Beim regulären
Beenden erfolgt ein letzter Löschversuch. Ein Prozessabsturz beendet den
Worker und kann den Timeout verhindern. Kopierfehler lassen das Fenster
offen; es gibt keinen ungeschützten Fallback. Programme müssen die Windows-
Kennzeichnungen respektieren; bereits ausgelesene Kopien werden nicht entfernt.

## Schrift und Gestaltung

Die Gestaltung folgt verbindlich [UI-STYLE.md](../../docs/design/UI-STYLE.md).
Das lokale Stock-Referenzbild wird nicht mitgeliefert. Die unveränderte
**JetBrains Mono NL Regular 2.304** ist eingebettet. Ihre markierte Null,
unterschiedlichen Formen für `1/l/I` und die Monospace-Abstände helfen beim
Abschreiben. Die explizite NL-Variante besitzt keine Ligaturen. `5/S` bleiben
zusätzlich durch die Ziffernfarbe unterscheidbar. Auswahl und Alternativen
stehen in [gui.md](../../docs/sources/gui.md); der vollständige Lizenztext
liegt in [assets/fonts/OFL.txt](assets/fonts/OFL.txt).

## Tests und öffentliche Demo

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p kairos windows_privacy_flags_timeout_and_foreign_replacement -- --ignored --test-threads=1
```

Der letzte Test schreibt ausschließlich öffentliche Fixtures in die echte
Windows-Zwischenablage und ist deshalb im normalen Testlauf ignoriert.
Er prüft beide Schutzformate, Timeout und den Erhalt späterer Inhalte.
Unit-Tests prüfen Farbzuteilung, mehrbyteige Segmente, Ausschlüsse, Fehler und
Entropieentwertung. Ein egui-Frame-Test tippt in das echte TextEdit, kontrolliert
Undo-Löschung und rendert Vollansicht sowie Popup.

Die optionale Debug-Demo ist ausschließlich für UI-Prüfung und Screenshots.
Sie ist deutlich markiert und enthält das öffentliche Beispiel
`Ähre-Wolke7-Kiesel-Möwe`. Die 82 Bit sind ein **fiktiver Anzeigewert**.
Bei `--release --features demo` bricht die Kompilierung absichtlich ab.

```powershell
cargo run -p kairos --features demo
$env:EFRAME_SCREENSHOT_TO = Join-Path (Get-Location) 'target\gui-evidence\gui-demo.png'
cargo run -p kairos --features demo
Remove-Item Env:EFRAME_SCREENSHOT_TO
```

Der eframe-Screenshotmodus speichert die tatsächlich gerenderte native App
nach dem zweiten Frame und beendet sie. Ohne `demo` steht diese Funktion
nicht zur Verfügung. Screenshots bleiben lokal unter `target/`.

## Open-Source-Vorbilder

- **egui/eframe 0.37.0**, MIT/Apache-2.0. Native Schleife, LayoutJobs und expliziter Widgetzustand.
- **KeePassXC 2.8.0**, dateibezogen GPL-2.0 OR GPL-3.0. Lesbarkeit und Zwischenablage als eigener Lebenszyklus.
- **Password Tech**, GPL-2.0-or-later. Unmittelbar erreichbare Generierungs- und Kopieraktionen.
- **arboard 3.7.0**, MIT/Apache-2.0. Bestehende Windows-Zwischenablageimplementierung als normale Crate-Abhängigkeit.
- **JetBrains Mono 2.304**, SIL OFL 1.1. Unverändertes Font-Asset mit mitgeliefertem Lizenztext.

Es wurde kein fremder Anwendungscode kopiert. Konkrete Quellstände, Module,
Lizenzbelege, bewertete Alternativen und die Grenzen der Übertragung sind
vollständig in [gui.md](../../docs/sources/gui.md) aufgeführt.
