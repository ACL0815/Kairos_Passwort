# Windows: Zwischenablage, Hotkeys und Fokus

Zuständig: **nur PR-4** (`feat/platform-hotkeys`). Die Crate bleibt unabhängig
von Core und GUI. Quellenstand/Abruf: **25.10.2026**. Bewertungen bezeichnen die
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
