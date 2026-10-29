# Windows: Zwischenablage, Hotkeys und Fokus

Zuständig: PR-4 (`feat/platform-hotkeys`) und additive Integration PR-W.
Die Crate bleibt unabhängig von Core und GUI. Quellenstand/Abruf: **25.10.2026**. Bewertungen bezeichnen die
Passung für Kairos, keine Codeübernahme. Ausschließlich Prinzipien wurden
übertragen; verwendete Rust-Abhängigkeiten sind gesondert genannt.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| KeePassXC Clipboard.cpp | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp) | GPL-2.0 OR GPL-3.0 (Dateikopf) | `Clipboard::setText`, `clearCopiedText` | Sensible Inhalte kennzeichnen; Timeout darf zwischenzeitlich ersetzte Inhalte nicht löschen | 5 |
| Password Tech | [Zwischenablage](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/util/SecureClipboard.cpp) | GPL-2.0-or-later | `src/util/SecureClipboard.cpp` | Kurze Lebensdauer und explizite Nutzeraktion für Kopieren in einer Windows-App | 4 |
| KeePassXC Autotype | [2.8.0 Windows](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/windows/AutoTypeWindows.cpp), [Ablauf](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp) | Beide Dateien: GPL-2.0 OR GPL-3.0 (Dateiköpfe) | `activeWindow`, `raiseWindow`, `sendChar`, `executeAutoTypeActions` | Fensteridentität getrennt von Texteingabe behandeln; bei geändertem Vordergrund abbrechen; Unicode-Ereignisse paaren | 5 |
| arboard 3.7.0 | [Commit `5f5c863`](https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/src/platform/windows.rs), [Manifest](https://github.com/1Password/arboard/blob/5f5c8637911ba219edee619105a19b4f74e66a9f/Cargo.toml) | MIT OR Apache-2.0; Windows-Abhängigkeit `clipboard-win`: BSL-1.0 | `SetExtWindows`, `add_clipboard_exclusions`, `Clipboard::open` | Alle drei Ausschlüsse sind schon implementiert; kurze Clipboard-Sperre und begrenzte Retries bei Belegung | 4 |
| clipboard-win 5.4.3 | [Versionierte Crate](https://docs.rs/crate/clipboard-win/5.4.3/source/src/raw.rs), [Manifest](https://docs.rs/crate/clipboard-win/5.4.3/source/Cargo.toml) | BSL-1.0 | `raw::{set_without_clear,seq_num,empty}`, `Clipboard` | Registrierte Formate und Sequenznummer sind zugänglich; atomare Besitzprüfung braucht dieselbe Clipboard-Sperre | 3 |
| global-hotkey 0.9.0 | [Commit `b4185ab`](https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/src/lib.rs), [Windows](https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/src/platform_impl/windows/mod.rs), [egui](https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/egui.rs), [winit](https://github.com/tauri-apps/global-hotkey/blob/b4185ab2f37b62e4d1bd2b8410fcf93e338f55fc/examples/winit.rs) | MIT OR Apache-2.0 | `GlobalHotKeyManager`, `GlobalHotKeyEvent`, `RegisterHotKey`-Backend | Wiederverwendung der Registrierung mit `MOD_NOREPEAT`; Manager und Message-Pump auf demselben Thread, Ereignisse zur App weiterreichen | 5 |
| keyboard-types 0.8.3 | [Commit `24446d6`](https://github.com/rust-windowing/keyboard-types/blob/24446d6fe974bca4303affd07072939f4a7aab6f/src/code.rs) | MIT OR Apache-2.0 | `Code::from_str`, `Modifiers` | Standardisierte Tastencodes und vorhandenen Key-Parser verwenden; eigene Logik nur für sichere Kombinationen | 5 |
| win-hotkeys 0.5.1 | [Tag `v0.5.1`](https://github.com/iholston/win-hotkeys/blob/v0.5.1/src/hook.rs), [Manager](https://github.com/iholston/win-hotkeys/blob/v0.5.1/src/manager.rs) | MIT | `KeyboardHook`, `HotkeyManager::register_channel` | Channel entkoppelt Eingabe und Aktion; ein globaler Low-Level-Tastaturhook erfasst für Kairos mehr Eingaben als nötig | 2 |
| enigo 0.6.1 | [Tag `v0.6.1`](https://github.com/enigo-rs/enigo/blob/v0.6.1/src/win/win_impl.rs) | MIT | `Keyboard::text`, `send_input` | Unicode-Down/Up und Prüfung der angenommenen Ereigniszahl; Text-Debuglogging ist für Geheimnisse ungeeignet | 3 |
| AutoHotkey 2.0.30 | [Tag `v2.0.30`](https://github.com/AutoHotkey/AutoHotkey/blob/v2.0.30/source/keyboard_mouse.cpp) | GPL-2.0 (Projekt) | `SendUnicodeChar`, `SendKeys` | Modifier verändern Texteingabe; Unicode und Shortcuts getrennt behandeln, keinen komplexen Script-/Tastaturzustandsautomaten nachbauen | 4 |
| KeePass 2.61.1 | [Offizielles Source-Archiv 2.61.1](https://sourceforge.net/projects/keepass/files/KeePass%202.x/2.61.1/KeePass-2.61.1-Source.zip/download), [Auto-Type-Doku](https://keepass.info/help/base/autotype.html), [Integrität](https://keepass.info/integrity.html) | GPL-2.0-or-later | `ClipboardUtil.cs::ClearIfOwner`, `ClipboardUtil.Windows.cs::AttachIgnoreFormatsW`; `AutoType.cs`, `SendInputExt/SiEngineWin.cs` | Clipboard nur bei noch passendem Hash leeren; No-Persist-Marker und Unicode-/Privileggrenzen beachten; keine Sequenzsprache übernehmen | 4 |
| Bitwarden Desktop 2026.10.1 | [Tag `desktop-v2026.10.1`: Hauptprozess](https://github.com/bitwarden/clients/blob/desktop-v2026.10.1/apps/desktop/src/platform/main/clipboard.main.ts), [Rust-Backend](https://github.com/bitwarden/clients/blob/desktop-v2026.10.1/apps/desktop/desktop_native/core/src/clipboard/arboard_backend.rs) | GPL-3.0 (Projekt; keine Übernahme) | `clipboard.write`, `before-quit`, `clipboard_set` | Rust-Backend setzt Windows-Cloud-/History-Ausschlüsse; beim Beenden nur den noch eigenen Clipboard-Text entfernen | 3 |
| eframe 0.37.0 / winit 0.30.13 | [eframe](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/src/epi.rs), [winit](https://github.com/rust-windowing/winit/blob/v0.30.13/src/event_loop.rs) | eframe: MIT OR Apache-2.0; winit: Apache-2.0 | `App::logic`, `Context::request_repaint`, `EventLoopProxy` | Ereignis erst einreihen, GUI dann gezielt wecken; Betriebssystemdienst benötigt keine GUI-Abhängigkeit | 5 |
| windows-sys 0.61.2 | [Commit `32c3144`](https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/sys/Cargo.toml) | MIT OR Apache-2.0 | `Win32::{System,UI}`, Win32-Bindings | Offizielle FFI-Typen und Aufrufe nutzen, statt eigene ABI-Deklarationen zu pflegen | 5 |
| zeroize 1.8.2 | [Commit `c100874`](https://github.com/RustCrypto/utils/blob/c100874101bfd584870de5dde1b13dd92a17bf48/zeroize/src/lib.rs) | MIT OR Apache-2.0 | `Zeroize` | Compilerbeständiges Überschreiben eigener, noch nicht an Windows übergebener Geheimnispuffer | 5 |

## Bewertung und konkrete Übertragung

**Zwischenablage:** KeePassXC und Password Tech liefern das Prinzip „nur noch
eigenen Inhalt leeren“. Kairos behält dafür weder Klartext wie KeePassXC noch
einen Passwort-Hash wie Password Tech oder KeePass. Bitwardens Abgleich beim
Beenden bestätigt das Lebensdauerprinzip, sein gespeicherter Klartext und seine
Desktop-/Tresorarchitektur werden nicht übernommen. Stattdessen wird eine pro
Schreibvorgang eindeutige, nicht geheime Marke unter derselben Clipboard-Sperre
geprüft. Die Sequenznummer wird nur diagnostisch beobachtet: Auf ihren Endwert
vor/nach `CloseClipboard` oder nach synthetisierten Formaten wird nicht vertraut. Eine erneut
kopierte identische Zeichenfolge bleibt damit ebenfalls unangetastet. Das ist
eine eigene Umsetzung des Prinzips, kein kryptografischer Eigentumsnachweis gegen
böswillige lokale Programme. Ein Fremdprogramm, das alle Formate einschließlich
der Marke wiederherstellt, stellt denselben markierten Eintrag wieder her. Gegen
absichtlich manipulierte Marker gibt es keine Schutzgrenze. Schutzflags stehen vor dem Text. Die DWORD-Werte
folgen Microsoft, nicht ungeprüft einer Qt-Serialisierung aus dem Vorbild.

`arboard` wurde tatsächlich auf seine Windows-Erweiterungen geprüft: Es kann alle
drei Ausschlüsse setzen. Seine öffentliche API bietet jedoch keine gemeinsame
Sperre für Schreiben, Receipt-Erfassung und späteres Prüfen/Leeren. Sein
Windows-Backend behandelt fehlgeschlagene Formatregistrierung zudem optional.
`clipboard-win` stellt Low-Level-Operationen bereit, steht aber unter BSL-1.0,
die in der expliziten Nutzervorgabe für Abhängigkeiten nicht enthalten ist;
`arboard` zieht diese Abhängigkeit unter Windows ebenfalls ein. Deshalb werden
beide nur als Modell bewertet. Die kleine eigene Win32-Grenze nutzt
`windows-sys`; Bild-, HTML-, Linux- und verzögertes Clipboard-Rendering entfallen.

Die Lizenzliste stammt aus dem ausdrücklichen PR-4-Arbeitsauftrag vom 25.10.2026:
MIT/Apache/BSD/Zlib/ISC/MPL-2.0 für Crate-Abhängigkeiten; beliebige Lizenzen nur als
Prinzipienquellen. Sie ist auch in der [PR-Beschreibung](https://github.com/ACL0815/Kairos_Passwort/pull/3)
festgehalten. Der unveränderliche Quellenindex wird dafür nicht erweitert.

**Autotype/Fokus:** Von KeePassXC, KeePass, AutoHotkey und enigo übernehmen wir
Unicode-Paare, Modifier-Beachtung, Zielprüfung und klare Fehlerbehandlung als
Prinzipien. Kairos wartet auf das physische Loslassen der Modifier und der
Hotkey-Haupttaste, statt sie
synthetisch aufzuheben und später wiederherzustellen. Es gibt keine Tab-/Enter-
Sequenzen, keinen layoutabhängigen Ersatzpfad, kein `AttachThreadInput`, keine
Focus-Lock-Manipulation und keine Wiederholung bei teilweiser Eingabe.
Der Fokus-Rücksprung wartet begrenzt auf die asynchrone Aktivierung und wird vor
dem Zerstören des App-Fensters angefordert. Prozess-Token liefern die
Integritätsstufen; höhere Ziele oder fehlgeschlagene Abfragen werden abgewiesen.
Angenommene `SendInput`-Ereignisse beweisen keine Zustellung im Zielfeld.
enigo ist bewusst keine Abhängigkeit: `Keyboard::text` loggt im geprüften Stand
den Text auf Debug-Level. Die eigene Batch-Allokation wird ohne Reallokation
dimensioniert und ihre Scan-Einheiten beim Drop mit `zeroize` gelöscht.

Native Fenster- und Control-Identität wird vor dem Popup erfasst und vor der
Eingabe erneut geprüft; Autotype verfällt nach fünf Sekunden. `SendInput` kann
ein Ziel nicht atomar adressieren, und mehrere Browser-DOM-Felder können ein
HWND teilen. Diese Grenzen sind dokumentiert, keine behauptete Feldgarantie.
Das spätere Browserprotokoll in PR-6 ist dafür der passendere Integrationspunkt.

**Hotkeys/GUI:** `global-hotkey`, `keyboard-types`, `windows-sys` und das bereits
gepinnte `zeroize` werden als normale kompatible Crates eingebunden. Hotkey-
Registrierung, Key-Codes und ABI werden wiederverwendet. `win-hotkeys` bleibt
Vergleichsquelle; wir benötigen keinen Low-Level-Hook für sämtliche Tastatureingaben.
Die eigene Schicht ergänzt nur Aktionen, Konflikt-/AltGr-Prüfung, Ziel-Snapshot,
begrenzten Channel und Lebensdauer des Message-Threads. Die eframe/winit-Prinzipien
führen zu einem Wakeup-Callback ohne GUI-Crate-Abhängigkeit. Der einmalige globale
`global-hotkey`-Receiver bleibt exklusiv beim Dienst.
`GetMessageW` blockiert im Leerlauf; `PostThreadMessageW(WM_QUIT)` beendet die
Schleife. Das neueste Ereignis ersetzt den alten Slot; ein fehlendes Fokusziel
bleibt zusammen mit der Aktion im Ereignis. Ein fünf Sekunden altes Ereignis
wird vor der GUI-Verarbeitung verworfen.

`keyboard-types` ist direkt erforderlich, damit Parsing ohne Windows-Backend
kompiliert; Features sind minimiert und Cargo vereinheitlicht beide 0.8-Bezüge.
`windows-sys 0.61` bleibt neben dem von `global-hotkey` benötigten 0.59: Die aktuelle
FFI bietet Default-Initialisierung für die verwendeten Win32-Strukturen und
vermeidet zusätzliche eigene unsafe-Initialisierungen. Zwischen den beiden
Versionen werden keine FFI-Strukturen ausgetauscht.

Die Defaults Ctrl+Shift+F9/F10/F11 vermeiden Ctrl+Alt und Zeichenpositionen;
F12 und Win-Chords werden ausgeschlossen. Druckbare Ctrl+Alt-Chords einschließlich
Shift werden konservativ wegen AltGr abgewiesen. Registrierungskonflikte mit
anderen Anwendungen bleiben möglich und werden gemeldet.
Mindestens zwei Modifier verhindern insbesondere globale Übernahmen von
Ctrl+C/V/X/Z/A und einfachen Alt-Menü-Accelerators.

Für den offiziellen KeePass-Snapshot `KeePass-2.61.1-Source.zip` nennt die
[Integritätsseite](https://keepass.info/integrity.html) am Abrufdatum 5.333.853 Bytes
und SHA-256 `711BD9EC7076661678469E51BAE876DEBAC07A08C32F1B1C6B3B8FE5E761C13A`.

## Microsoft-Verträge und Belegprüfung

### Desktop-Integration (Abruf 29.10.2026)

| Primärquelle | Lizenz / Vertrag | Modelliertes Prinzip |
|---|---|---|
| [Run und RunOnce](https://learn.microsoft.com/en-us/windows/win32/setupapi/run-and-runonce-registry-keys) | Microsoft-Dokumentation, CC-BY-4.0 | Nur HKCU Run, exe separat quoten, feste Argumente; keine Administratorrechte und keine Registrierung ohne ausdrückliche Aktion. |
| [auto-launch](https://docs.rs/auto-launch/latest/auto_launch/) | MIT, Crate-Dokumentation | Benutzerbezogene Windows-Registrierung ist optional. Nur Prinzip: eigene kleine API liest zusätzlich Original-Kommando und StartupApproved, damit ein Umzug oder eine Windows-Sperre sichtbar wird. Keine Abhängigkeit/Codeübernahme. |
| [Shell Links](https://learn.microsoft.com/en-us/windows/win32/shell/links), [IShellLinkW](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishelllinkw) | Microsoft-Dokumentation, CC-BY-4.0 | COM-Apartment je Aufruf, IShellLinkW-Ziel/Arbeitsverzeichnis/Beschreibung/Icon und IPersistFile-Speicherung. Nur festes persönliches Linkziel, keine Verzeichnisbereinigung; vorhandene fremde Dateien bleiben bestehen. |
| [CreateMutexW](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createmutexw), [Kernel object namespaces](https://learn.microsoft.com/en-us/windows/win32/termserv/kernel-object-namespaces), [CreateEventW](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw) | Microsoft-Dokumentation, CC-BY-4.0 | Benannter Mutex als Lebensdauerwächter, Session-Namespace plus Benutzer-SID; Auto-Reset-Events puffern Öffnen/Popup auch vor GUI-Start. Blockierender Listener wird beim Drop beendet und gejoint. Kein HWND-Suchen und keine sichtbaren IPC-Fenster. |

`StartupApproved\\Run` ist kein hier belegter stabiler Microsoft-API-Vertrag.
Kairos interpretiert ausschließlich bekannte 12-Byte-REG_BINARY-Marker (2/6
aktiv, 3/7 deaktiviert), behandelt andere Werte als unbekannt und schreibt diesen
Schlüssel **nie**. Die tatsächliche Task-Manager-Interaktion bleibt manuell offen.
Der Run-Wert bleibt getrennt von der Windows-Freigabe sichtbar. Ein abweichender
Pfad wird nicht beim Start oder Tray-Klick automatisch ersetzt.

Alle drei zusätzlichen Bibliotheken `tray-icon`, dessen `muda` und `winresource`
sind reguläre permissive Build-/Laufzeitabhängigkeiten; Windows-COM verwendet
die offiziellen `windows`-Bindings. Die Tests verwenden ausschließlich eigene
Registry-Wertnamen, Kernel-Namen und TempDir-Pfade und belegen ihre Entfernung.
Kein fremder Anwendungscode wurde übernommen.

| Vertrag | Fester Dokumentationsstand | Konsequenz |
|---|---|---|
| Clipboard-Schutzformate | [Commit `3aee48a`, Clipboard Formats](https://github.com/MicrosoftDocs/win32/blob/3aee48ac3f48e187d17bdfd56951741737331749/desktop-src/dataxchg/clipboard-formats.md), [offizielle Seite](https://learn.microsoft.com/en-us/windows/win32/dataxchg/clipboard-formats#cloud-clipboard-and-clipboard-history-formats) | Ein registriertes Ausschlussformat mit beliebigen Daten sowie zwei serialisierte DWORD(0) verhindern vorgesehenen Verlauf/Sync |
| Clipboard-Sperre / Sequenz | [GetClipboardSequenceNumber `d43984d`](https://github.com/MicrosoftDocs/sdk-api/blob/d43984d7c322f7d7d35704d6d1ad36dc3539703b/sdk-api-src/content/winuser/nf-winuser-getclipboardsequencenumber.md), [OpenClipboard am selben Stand](https://github.com/MicrosoftDocs/sdk-api/blob/d43984d7c322f7d7d35704d6d1ad36dc3539703b/sdk-api-src/content/winuser/nf-winuser-openclipboard.md) | Nur unter derselben Sperre prüfen und leeren; eine Sequenzbeobachtung allein autorisiert kein Leeren; für Schreiben nach EmptyClipboard ein gültiges Owner-Fenster verwenden |
| SendInput / Unicode / UIPI | [SendInput `d43984d`](https://github.com/MicrosoftDocs/sdk-api/blob/d43984d7c322f7d7d35704d6d1ad36dc3539703b/sdk-api-src/content/winuser/nf-winuser-sendinput.md), [KEYBDINPUT `0ca0b89`](https://github.com/MicrosoftDocs/sdk-api/blob/0ca0b898c7df5c24c886832ea2b90af226b52713/sdk-api-src/content/winuser/ns-winuser-keybdinput.md) | Vorhandener Tastaturzustand bleibt relevant; `wVk=0`, Unicode über `wScan`; angenommene Ereignisse zählen; keine privilegierte Eingabe erzwingen |
| Vordergrund / Control | [SetForegroundWindow `6268112`](https://github.com/MicrosoftDocs/sdk-api/blob/626811245f736a21b35d56030aa27048b1c76bb5/sdk-api-src/content/winuser/nf-winuser-setforegroundwindow.md), [GetGUIThreadInfo `d8335f6`](https://github.com/MicrosoftDocs/sdk-api/blob/d8335f67a4657aa01eb87c1b0f63863fe01530ee/sdk-api-src/content/winuser/nf-winuser-getguithreadinfo.md) | Windows darf Aktivierung verweigern; `hwndFocus` ist eine native Fensteridentität, keine DOM-Feld-ID |
| Hotkey / AltGr | [RegisterHotKey `d43984d`](https://github.com/MicrosoftDocs/sdk-api/blob/d43984d7c322f7d7d35704d6d1ad36dc3539703b/sdk-api-src/content/winuser/nf-winuser-registerhotkey.md), [Tastaturleitfaden `48e6f97`](https://github.com/MicrosoftDocs/win32/blob/48e6f97f72445079ad7dad073a645f9ff694c0b0/desktop-src/uxguide/inter-keyboard.md) | Ctrl+Alt kann AltGr abfangen; Win-Chords und F12 vermeiden; kein universell konfliktfreier Default |
| Integritätsstufen / Token | [GetTokenInformation am SDK-Stand `d43984d`](https://github.com/MicrosoftDocs/sdk-api/blob/d43984d7c322f7d7d35704d6d1ad36dc3539703b/sdk-api-src/content/securitybaseapi/nf-securitybaseapi-gettokeninformation.md), [MIC](https://learn.microsoft.com/en-us/windows/win32/secauthz/mandatory-integrity-control) | Integritätslabel aus Prozess-Token abfragen; Vergleich vor Eingabe; bei höherem oder unbekanntem Ziel abbrechen |

Microsoft-Quellen sind API-Dokumentation unter CC-BY-4.0, keine übernommene
Programmbibliothek. Die ältere UX-Leitlinie beschreibt AltGr nur für bestimmte
Sprachversionen; daraus leiten wir die konservative Konfiguration ab.

Stichproben durch den PR-Worker: KeePassXC-Dateiköpfe und Unicode-/Clipboard-Pfade, arboard 3.7.0
einschließlich Cargo-Metadaten, global-hotkey 0.9.0 einschließlich Windows-Backend
und egui-Beispiel, enigos Text-Debuglog, Password Techs Digest-Prüfung sowie die
offiziellen Microsoft-Formate/SendInput-/Fokusregeln sowie Bitwardens Rust-Backend.
Crate-Commit-IDs wurden mit
den veröffentlichten `.cargo_vcs_info.json`-Dateien abgeglichen. Nicht geprüfte
Cloud-Sync-/Desktop-Interaktionen sind nicht durch die Quellenprüfung bewiesen.
Reale Clipboard-Formate, Windows-Synthese, Timer-Ersetzung und Timeout wurden
zusätzlich auf einer privaten unsichtbaren Windowstation getestet. Sie dient
nicht als Beleg für globale Hotkeys oder Fokus-/Eingabe auf einem interaktiven Desktop.

## PR-W: ausdrückliche Bestätigung nach Bearbeitung

| Projekt | Permalink | Lizenz | Datei | Prinzip | Passung + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| KeePassXC 2.8.0 | [AutoType.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp) | GPL-2.0 OR GPL-3.0, Dateiheader | Ziel vor Auswahlfenster, Abbruch bei Vordergrundwechsel | Fensteridentität über Bearbeitungsschritt bewahren | 5/5; nur Prinzip. Kairos erneuert nach explizitem Einfügeklick die Frist und prüft weiter das ursprüngliche native Control | 25.10.2026 |
| eframe 0.37.0 | [App::logic](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/src/epi.rs) | MIT OR Apache-2.0 | App::logic | Hotkeys bei verborgenem Hauptfenster nach gezieltem Wakeup | 5/5; App-Integration, keine GUI-Abhängigkeit der Plattform | 25.10.2026 |

Die additive API FocusTarget::restore_and_type_text ist ausschließlich für
einen neuen Nutzerklick nach Editorbearbeitung. Erst restore() mit
unveränderter Fenster-/Prozess-/Thread-/Control-Identität, dann eine neue
Fünf-Sekunden-Frist. Es gibt keinen Recapture und keine Übernahme eines nun
zufällig aktiven Ziels. Alte automatische Hotkey-Aufträge verwenden weiterhin
type_text und verfallen. is_current_process ermöglicht der App, sich selbst
als Ziel auszuschließen. Kein fremder Anwendungscode wurde übernommen.
Die App-Grenzen sind per Fake prüfbar; ein zusätzlicher ignorierter nativer
Test prüft Ablauf, Controlwechsel und explizite Eingabe nach Bearbeitungszeit.

## PR-11: Bildschirmfreigabe und Aufnahmen, Prio 7

Standardmäßig aktiver Schutz mit explizitem `--allow-screencapture` pro Start.
Die folgenden Quellen wurden als Prinzipien modelliert. Kein fremder
Anwendungscode wurde kopiert; `raw-window-handle` und `windows-sys` werden als
normale Rust-Abhängigkeiten verwendet.

| Projekt | Permalink / Primärquelle | Lizenz | Datei / Vertrag | Prinzip | Passung + Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| KeePassXC 2.8.0 | [main.cpp, CLI und Top-Level-Policy](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp#L88-L100), [WinUtils.cpp, Win32-Aufruf](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp#L51-L63), [MainWindow.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/MainWindow.cpp) | GPL-2.0 OR GPL-3.0 laut geprüften Dateiheadern | `allow-screencapture`, `setPreventScreenCapture`, `setAllowScreenCapture` | Standardmäßig ausschließen, bewusst pro Start freigeben und Top-Level-Fenster einbeziehen | 5/5. Direkt passende Windows-Policy; Qt-Ereignisfilter/Implementierung nicht übernommen. Kairos ergänzt Hook und Statusrückgabe in eigenem Rust-Code | 25.10.2026 |
| Microsoft | [SetWindowDisplayAffinity](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity), [GetWindowDisplayAffinity](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity) | Dokumentation CC-BY-4.0 | `WDA_EXCLUDEFROMCAPTURE`, `WDA_MONITOR`, `WDA_NONE`, Rückgabewerte | Eigenes Top-Level-Fenster ausschließen und gesetzte Affinität zurücklesen | 5/5. Ausschluss ab Windows 10 2004/Build 19041. Ältere Systeme erhalten explizit `WDA_MONITOR`; Fehler werden nicht als Schutz dargestellt. DWM erforderlich, keine Garantie gegen beliebige Aufnahmewege | 25.10.2026 |
| Microsoft | [SetWindowsHookExW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw), [CallWndProc](https://learn.microsoft.com/en-us/windows/win32/winmsg/callwndproc), [WM_WINDOWPOSCHANGING](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging) | Dokumentation CC-BY-4.0 | Threadgebundener `WH_CALLWNDPROC`, `CWPSTRUCT`, `WINDOWPOS` | Vor Anzeige neuer nativer Fenster handeln, statt nur den Start-HWND zu schützen | 5/5. Deckt spätere eframe-Viewports und native Dialoge im GUI-Thread ab. Kein globaler Hook, keine Injection. Zusätzliche GUI-Threads benötigen eigenen Guard | 25.10.2026 |
| eframe 0.37.0 / raw-window-handle 0.6.2 | [CreationContext](https://github.com/emilk/egui/blob/0.37.0/crates/eframe/src/epi.rs), [WindowHandle](https://docs.rs/raw-window-handle/0.6.2/raw_window_handle/struct.WindowHandle.html) | MIT OR Apache-2.0 | `HasWindowHandle`, `RawWindowHandle::Win32` | OS-Handle an die Lebensdauer des besitzenden Fensters binden | 5/5. Root-Handle ohne Titel-/Vordergrundsuche; Plattform bleibt ohne eframe-/winit-Abhängigkeit | 25.10.2026 |
| winit 0.30.13 | [Windows-Ereignisfenster](https://github.com/rust-windowing/winit/blob/v0.30.13/src/platform_impl/windows/event_loop.rs#L913-L963) | Apache-2.0 | `create_event_target_window` | Technische Sichtbarkeit eines inhaltslosen Ereignis-HWND von sichtbarer App-Oberfläche unterscheiden | 5/5 zur Grenzprüfung. 0×0, layered, `WS_VISIBLE` per Stiländerung für WM_PAINT-Zustellung; kein angezeigtes Passwortfenster und kein Vorbildcode übernommen | 25.10.2026 |

Kairos zeigt Freigabe, fehlenden Schutz und Monitor-Fallback getrennt und dauerhaft
oberhalb des scrollbaren Editors an. Fehler aus einem späteren Fenster bleiben
sichtbar, auch wenn das Hauptfenster erfolgreich geschützt wurde. Die tatsächliche
Capture-Wirkung hängt vom verwendeten Aufnahmeweg ab. Kameras, privilegierte
Admin-/Treiber-Tools und manche Fernwartungsprogramme werden nicht abgedeckt.
Die automatisierten Tests lesen reale Anzeigeaffinität eigener Fenster zurück;
Teams/Snipping-Tool-Abnahme und ein tatsächlicher Alt-Windows-Lauf sind gesondert
zu dokumentieren. [MANUAL-TEST.md](../MANUAL-TEST.md) enthält die Schritte.

Reine `GWL_STYLE`-/`WS_VISIBLE`-Änderungen werden vom Show-Hook nicht erfasst.
Das inhaltslose winit-Ereignisfenster benötigt keinen Passwortschutz; künftige
Inhaltsfenster auf diesem Weg müssen vor der Stiländerung explizit geschützt
werden. Die App registriert mit `CaptureProtection::set_wakeup` einen Root-Repaint
für Hook-Fehler, damit deren Status sofort als Banner erscheint. F9, F11 und
Browser-Pairing nutzen denselben geschützten Root. Übersprungene
verschachtelte Show-Nachrichten führen konservativ zu einem Fehlerstatus.
