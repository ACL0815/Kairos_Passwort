# GUI und Passwortdarstellung

Zuständig: **nur PR-3** (`feat/gui`). Verbindlicher visueller Rahmen:
[UI-STYLE.md](../design/UI-STYLE.md). Recherche und Abruf **25.10.2026**.
Passung 1–5 ist die Bewertung für diese GUI. Bewertet werden Prinzipien;
kein fremder Anwendungscode wurde übernommen. Normale Crate-Abhängigkeiten
und das ausdrücklich lizenzierte Schrift-Asset sind gesondert genannt.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| egui/eframe 0.37.0 | [Tag](https://github.com/emilk/egui/tree/0.37.0), [Code-Editor](https://github.com/emilk/egui/blob/0.37.0/crates/egui_demo_lib/src/demo/code_editor.rs) | MIT OR Apache-2.0; Assets separat | `eframe::App`, `TextEdit::layouter`, `LayoutJob` | Native Rust-Schleife und farbige Bereiche mit explizitem UI-Zustand; normale Crate-Abhängigkeit | 5 |
| Password Tech | [Windows-Oberfläche](https://github.com/cthoeing/passwordtech/tree/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/main) | GPL-2.0-or-later | `src/main/` | Direkter Generator-Ablauf, editierbares Ergebnis und wenige Kopieraktionen | 4 |
| KeePassXC 2.8.0 | [PasswordWidget](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/PasswordWidget.cpp), [Clipboard](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp) | Die verlinkten Dateien GPL-2.0 OR GPL-3.0 laut Header | `PasswordWidget`, `Clipboard::setText`, `clearCopiedText` | Generierung, Darstellung und Kopierlebensdauer getrennt; Windows-Kennzeichnungen und Timeout für sensible Inhalte | 5 |
| KeePassXC 2.7.12 | [EntryPreviewWidget](https://github.com/keepassxreboot/keepassxc/blob/2.7.12/src/gui/EntryPreviewWidget.cpp#L263-L297) | GPL-2.0-or-later laut Projektlizenz; Dateiheader beachten | `EntryPreviewWidget::setPasswordVisible` | Zeichentypen farblich unterscheidbar machen und eine dunkle Palette berücksichtigen | 5 |
| Bitwarden browser-v2026.10.0 | [Farbige Inline-Anzeige](https://github.com/bitwarden/clients/blob/browser-v2026.10.0/apps/browser/src/autofill/overlay/inline-menu/pages/list/autofill-inline-menu-list.ts#L411-L444) | GPL-3.0; verlinkter Pfad außerhalb `bitwarden_license` | `buildColorizedPasswordElement` | Ziffern und Sonderzeichen mit eigenen Klassen darstellen; reine Prinzipienreferenz | 4 |
| egui 0.37.0 TextEdit | [Builder](https://github.com/emilk/egui/blob/0.37.0/crates/egui/src/widgets/text_edit/builder.rs), [State](https://github.com/emilk/egui/blob/0.37.0/crates/egui/src/widgets/text_edit/state.rs) | MIT OR Apache-2.0 | `password`, `clear_undoer`, `store`, eigener Layouter | Passwortmodus verhindert normales Kopieren; Undo muss separat geleert werden. Ein eigener sichtbarer Layouter erzeugt Klartext-Layouts | 5 |
| catppuccin-egui | [Commit ffb92d2](https://github.com/catppuccin/egui/blob/ffb92d2da71b72bde41d83bbc0a46917de97b486/src/lib.rs) | MIT | `set_theme`, `make_widget_visual` | Semantische Rollen für Hintergrund, Text, Auswahl und Fokus statt zufälliger Einzelwerte | 2 |
| arboard 3.7.0 | [Commit 5f5c863](https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/src/platform/windows.rs) | MIT OR Apache-2.0 | `SetExtWindows`, `Set::text`, `OpenClipboard` | Beide Windows-Ausschlussformate über erprobte Crate setzen; der Windows-Adapter besitzt keinen HWND-Eigentümer | 5 |
| clipboard-win 5.4.3 | [Versionierter Quellstand](https://docs.rs/crate/clipboard-win/5.4.3/source/src/raw.rs) | MIT laut Crate-Metadaten | `Clipboard`, `raw::seq_num`, `get_string`, `get_vec`, `empty` | RAII-Sperre für eine gemeinsame Prüfung und bedingte Löschung; normale Windows-Abhängigkeit | 5 |

## Übertragung und Grenzen

Übernommen werden der kurze Generatorablauf von Password Tech, lesbare
Zeichenkategorien von KeePassXC/Bitwarden und eguis standardisierte Widgets.
Kairos übernimmt weder Qt-/VCL-/Web-Komponenten noch Generatorverläufe oder
Passworttresore. Bearbeitung wird ausdrücklich **nicht** mit der ursprünglichen
Generatorentropie bewertet. Die App zeigt die Entropie des aktuellen
Core-Ergebnisses und nach einer Eingabe „bearbeitet – nicht mehr bewertbar“.

Catppuccin wird nicht eingebunden. Der geprüfte Stand unterstützt egui-Versionen
bis 0.33; eine zweite egui-Version wäre unnötig. Die eigene kleine Palette
folgt den verbindlichen Kairos-Farben. eframe verwendet den Glow-Renderer ohne
Persistenz, normalen Clipboard-Export oder Inspektionsserver.

Beim eigenen Layouter ist der Passworttext absichtlich sichtbar. Das ist für
das Abschreiben nötig, erzeugt aber Klartext in eguis Layout-/Galley-Daten.
`password(true)` ersetzt keine Speicherhärtung. Der Undoer wird nach jedem Frame
geleert, seine temporären `String`-Snapshots können wir über die öffentliche API
nicht sicher überschreiben. Die verbleibenden UI-/OS-Kopien sind im
[Crate-README](../../apps/kairos/README.md) dokumentiert und PR-9 vorbehalten.

Die Übergangs-Zwischenablage nutzt arboard nur für Schreiben mit den beiden
Windows-Schutzformaten. Unter einer anschließenden OS-Sperre wird der Inhalt
mit dem gerade kopierten Text verglichen und eine pro Kopie neue Markierung
gesetzt. Markierung und Sequenznummer begrenzen die spätere Löschung auf
unveränderte eigene Inhalte. Prüfung und Leeren sind atomar gegenüber fremden
Clipboard-Schreibern. Ein eigener Worker hält den Timeout unabhängig von der
Fensterdarstellung. Der Adapter wird später durch `kairos-platform` ersetzt.

Stichproben am Original: Header und Windows-Formate in KeePassXC 2.8.0,
`PasswBoxChange`/`ShowPasswInfo` und Lizenzheader des gepinnten Password-Tech-
`Main.cpp`, arboards Windows-Implementierung und Cargo-VCS-Commit, eguis
TextEdit-Undoer sowie JetBrains README/OFL. Der native Clipboard-Test prüft
die tatsächlichen Formate und den Erhalt späterer Inhalte unter Windows.

## Passwortschrift

| Projekt | Permalink mit Commit/Tag | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| JetBrains Mono NL 2.304 | [TTF](https://github.com/JetBrains/JetBrainsMono/blob/v2.304/fonts/ttf/JetBrainsMonoNL-Regular.ttf), [README](https://github.com/JetBrains/JetBrainsMono/blob/v2.304/README.md), [OFL](https://github.com/JetBrains/JetBrainsMono/blob/v2.304/OFL.txt) | SIL OFL 1.1 | `JetBrainsMonoNL-Regular.ttf` | Statische Monospace-Datei ohne Ligaturen; markierte Null und eigenständige Formen für 1/l/I erleichtern Abschreiben | 5 |
| Atkinson Hyperlegible Mono | [Google-Fonts-Commit 4dc806e](https://github.com/google/fonts/tree/4dc806e91cf5eb55855d69823eee36507234ef8b/ofl/atkinsonhyperlegiblemono) | SIL OFL 1.1, dort `OFL.txt` | `AtkinsonHyperlegibleMono[wght].ttf` | Lesbarkeit und Zeichenunterscheidung als Entwurfsziel; Alternative mit variabler Schriftdatei | 4 |
| Source Code Pro | [Google-Fonts-Commit 4dc806e](https://github.com/google/fonts/tree/4dc806e91cf5eb55855d69823eee36507234ef8b/ofl/sourcecodepro) | SIL OFL 1.1, dort `OFL.txt` | `SourceCodePro[wght].ttf` | Bewährte Monospace-Schrift; Alternativglyphen müssten im tatsächlichen Renderer geprüft werden | 4 |
| B612 Mono | [Google-Fonts-Commit 4dc806e](https://github.com/google/fonts/tree/4dc806e91cf5eb55855d69823eee36507234ef8b/ofl/b612mono) | SIL OFL 1.1, dort `OFL.txt` | `B612Mono-Regular.ttf` | Für lesbare technische Anzeigen konzipiert; 0/O ohne eigene visuelle Prüfung nicht als überlegen belegt | 2 |
| Fira Mono | [Google-Fonts-Commit 4dc806e](https://github.com/google/fonts/tree/4dc806e91cf5eb55855d69823eee36507234ef8b/ofl/firamono) | SIL OFL 1.1, dort `OFL.txt` | `FiraMono-Regular.ttf` | Ruhige Monospace-Abstände; keine gleich starke belegte NL-Auswahl wie JetBrains | 3 |
| Hack 3.003 | [Tag](https://github.com/source-foundry/Hack/tree/v3.003), [Lizenz](https://github.com/source-foundry/Hack/blob/v3.003/LICENSE.md) | MIT plus Bitstream Vera License; DejaVu-Anteil gemeinfrei | `ttf/Hack-Regular.ttf`, `CHANGELOG.md` | Optimierte Zeichenformen und entfernte fi/fl-Ligaturen; zusätzliche Lizenz-/Variantenprüfung gegenüber einer klaren OFL-NL-Datei | 3 |

**Auswahl:** Die unveränderte JetBrains Mono **NL Regular v2.304** wird
eingebettet. [JetBrains](https://www.jetbrains.com/lp/mono/) belegt die gepunktete
Null gegenüber O und die Unterscheidung von 1/l/I; das gepinnte README belegt
die ligaturfreie NL-Variante. Eine Herstellerzusage zu 5/S wird nicht behauptet.
Die Ziffernfarbe unterstützt diese Unterscheidung zusätzlich. Die anderen
Schriften sind geeignete Alternativen, bieten im geprüften Stand aber keinen
belegten Vorteil, der weitere Assets oder Laufzeit-Konfiguration rechtfertigt.

Nur dieses Schrift-Asset wird übernommen. Herkunft, SHA-256 und unveränderter
vollständiger Lizenztext liegen unter
[`apps/kairos/assets/fonts/`](../../apps/kairos/assets/fonts/README.md).
Eine Systeminstallation ist nicht erforderlich. Das lokale Stock-Referenzbild
wurde nur betrachtet und bleibt außerhalb des Repositorys.

Die drei Recherche-Aufträge liefen auf `codex/gpt-6-luna`, jeweils mit
`reasoningEffort=xhigh` und `serviceTier=priority`. Schrift, Generator-UIs und
egui/Sicherheit wurden unabhängig untersucht. Ihre Ergebnisse wurden nicht
als Code übernommen; unbelegte Produkteigenschaften wurden verworfen.
