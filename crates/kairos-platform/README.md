# kairos-platform

Windows-Dienste für globale Hotkeys, Unicode-Autotype, Fokus-Rücksprung und
Zwischenablage mit Verlauf-/Cloud-Ausschluss. Die Crate ist unabhängig von Core,
KeePassXC und GUI. Die App liefert den Text ausschließlich nach einer Nutzeraktion.
Andere Betriebssysteme erhalten `PlatformError::UnsupportedPlatform`; Konfiguration
und Konfliktprüfung bleiben plattformunabhängig nutzbar.

## API und Integration

```rust,no_run
use kairos_platform::{copy_secret, Action, GlobalHotkeys, HotkeyBinding};
use std::time::Duration;

# fn example() -> Result<(), kairos_platform::PlatformError> {
let hotkeys = GlobalHotkeys::new(&HotkeyBinding::defaults())?;
// Aus der GUI heraus abfragen, ohne eine zweite GUI-Eventloop zu starten:
if let Some(event) = hotkeys.try_recv()? {
    match event.action {
        Action::OpenEditor => {
            // event.target vor dem Öffnen des Editors in der App behalten.
            // Nach Kopieren oder Schließen: event.target.restore()?
        }
        Action::GenerateTypeCopy => {
            let text = "Fester-Dummy+Äpfel7"; // Die App generiert den echten Text.
            event.target.type_text(text)?;
            let receipt = copy_secret(text, Some(Duration::from_secs(30)))?;
            // Bei App-Ende optional receipt.clear_if_current()? aufrufen.
            let _ = receipt;
        }
        Action::GenerateTypeKeePassXC => {
            // Dieselbe geprüfte target.type_text(text)-API; Protokoll folgt in PR-6.
        }
    }
}
# Ok(())
# }
```

`GlobalHotkeys::with_wakeup(&bindings, move || ctx.request_repaint())` weckt eine
eframe-App auch aus dem Hintergrund. Der Callback läuft auf dem Hotkey-Thread;
UI-Zustand und Textverarbeitung bleiben im App-Kontext. Alternativ lässt sich
ein winit-`EventLoopProxy` im Callback benachrichtigen. Die Bibliothek hängt von
keinem dieser GUI-Frameworks ab. Der Manager besitzt einen eigenen Win32-Thread
mit Message-Pump. Drop meldet Hotkeys ab und wartet auf dessen Ende.

Es gibt höchstens ein ausstehendes Ereignis; weitere Tastendrücke werden verworfen.
Pro Prozess ist ein Dienst erlaubt. Die App muss den prozessglobalen Receiver von
`global-hotkey` exklusiv diesem Dienst überlassen und darf keinen fremden
`set_event_handler` installieren. Bei einem Registrierungsfehler werden schon
registrierte Kombinationen zurückgenommen. Fehler enthalten keine Geheimnisse.

## Konfigurierbare Hotkeys und deutsches Layout

| Aktion | Standard |
|---|---|
| `OpenEditor` | `Ctrl+Shift+F9` |
| `GenerateTypeCopy` | `Ctrl+Shift+F10` |
| `GenerateTypeKeePassXC` | `Ctrl+Shift+F11` |

Kombinationen mit `"Ctrl+Shift+F9".parse()` erstellen und über `HotkeyBinding`
zuordnen. Ein bis drei unterschiedliche Aktionen sind erlaubt. Der Parser nutzt
`keyboard-types::Code`, also `KeyQ`/`Digit1` statt lokalisierter Tastennamen.
Zulässig sind Buchstaben, Ziffern und F1–F24 mit Ctrl oder Alt, optional Shift;
F12, Alt+F4, Win-Kombinationen und reine Shift-Kombinationen sind gesperrt.

Die Defaults nutzen Funktionstasten und kein Ctrl+Alt. Windows kann Ctrl+Alt als
AltGr interpretieren; auf deutschem Layout würde etwa Ctrl+Alt+Q das `@` abfangen.
Der Parser lehnt deshalb alle druckbaren Ctrl+Alt-Kombinationen einschließlich
Shift+Ctrl+Alt ab. Diese konservative Entscheidung folgt dem
[Microsoft-Tastaturleitfaden](https://github.com/MicrosoftDocs/win32/blob/48e6f97f72445079ad7dad073a645f9ff694c0b0/desktop-src/uxguide/inter-keyboard.md).
Eine universell konfliktfreie Belegung gibt es nicht. `HotkeyUnavailable` muss
die spätere App mit einer Umbelegung beantworten. Lokale Konflikte werden schon
vor der Windows-Registrierung erkannt.

## Zwischenablage und Lebensdauer

`copy_secret(text, clear_after)` setzt unter einer einzigen Zwischenablagesperre
zuerst `ExcludeClipboardContentFromMonitorProcessing`,
`CanIncludeInClipboardHistory = DWORD(0)` und
`CanUploadToCloudClipboard = DWORD(0)`, dann eine nicht geheime Besitzmarke und
zuletzt `CF_UNICODETEXT`. Fehlt eine Schutzinformation, wird kein Text publiziert.
Ein Fehler nach Beginn des Schreibens rollt die eigene Transaktion unter derselben
Sperre zurück. Vorherige Zwischenablageinhalte werden dabei nicht wiederhergestellt.
Eine eigene unsichtbare Message-Only-Window dient als gültiger Clipboard-Owner.

Der Rückgabewert `ClipboardReceipt` hält nur Sequenznummer und Besitzmarke, weder
Text noch Hash. Der Timeout prüft beides unter der Clipboard-Sperre. Auch eine neue
Kopie desselben Textes ist ein fremder Schreibvorgang für den alten Timeout.
Ein einziger Timerthread verwaltet den aktuellen Auftrag und wiederholt bei
vorübergehend belegter Zwischenablage den Löschversuch. Seine Warteschlange ist
auf einen Auftrag begrenzt; bei Überlast scheitert eine neue Kopie sicher und
meldet `WorkerUnavailable`. `None` deaktiviert den Timeout ausdrücklich,
`Some(Duration::ZERO)` macht ihn sofort fällig. Maximal 24 Stunden sind erlaubt.
Der Timer läuft nur, solange der Prozess lebt; die App sollte ihre letzte Receipt
beim geregelten Beenden explizit leeren.

Es entstehen keine temporären UTF-16-Strings: Text wird direkt in die vorbereitete
Win32-Allokation geschrieben. Nicht übergebene Puffer und die Scan-Einheiten im
Autotype-Puffer werden mit `zeroize` überschrieben. Nach erfolgreicher Übergabe
gehört der Clipboard-Speicher Windows; vollständige Löschung aller Betriebssystem-
oder Fremdkopien wird nicht behauptet. Schutzformate wirken nur bei Software, die
sie respektiert. Die Zwischenablage ist kein Schutz gegen lokale Schadsoftware.

## Fokus und Autotype

Das Ereignis erfasst vor der GUI-Benachrichtigung Vordergrund-HWND, Prozess,
Thread und fokussiertes natives Control. Für manuelle Aufrufe steht
`FocusTarget::capture()` bereit. Das Ziel muss vor dem Popup erfasst werden.
`event.target.type_text(text)` erzwingt die Verwendung dieses Ziels und vermeidet
eine freistehende Funktion, die nach einem Fensterwechsel still neu erfassen würde.

Autotype prüft das Ziel während des Wartens und unmittelbar vor `SendInput` erneut.
Ctrl, Alt, Shift und beide Win-Tasten müssen mindestens 20 ms freigegeben sein;
nach zwei Sekunden wird abgebrochen. Nach fünf Sekunden ist ein Autotype-Ziel
abgelaufen. Ein Fenster-/Controlwechsel, ein zerstörtes Ziel, aktive Menüs sowie
unvollständige Eingabe führen zu einem Fehler. Es gibt keine Wiederholung des
Geheimtexts und keinen automatischen Clipboard-Fallback. Eingabe erfolgt als ein
Batch aus Unicode-Down/Up-Paaren, einschließlich UTF-16-Surrogatpaaren, mit
`KEYEVENTF_UNICODE` und `wVk = 0`, unabhängig vom Tastaturlayout. Tab, Enter und
andere Steuerzeichen werden abgewiesen. Text ist auf 1024 UTF-16-Einheiten begrenzt.

`restore()` darf das ursprüngliche Fenster aktivieren, wenn aktuell die eigene App
im Vordergrund ist oder das Ziel bereits aktiv ist. Windows kann dies verweigern;
Rückgabewert und tatsächlicher Fokus werden geprüft. Es gibt keine synthetische
Alt-Taste, kein `AttachThreadInput` und keine Änderung des Foreground-Lock-Timeouts.
Eine andere inzwischen aktive Anwendung wird nicht verdrängt.

**Windows-Grenzen:** `SendInput` hat keinen Zielparameter. Zwischen dem letzten
Fokuscheck und der tatsächlichen Zustellung bleibt ein Rennen mit programmatischen
Fokuswechseln. Mehrere DOM-Felder im Browser oder Controls einer Canvas-App können
dasselbe native HWND besitzen. Diese API kann dann keinen Feldwechsel erkennen.
UIPI kann Eingabe in höher privilegierte Apps blockieren. `InputIncomplete` kann
Teiltext bedeuten; die App darf ihn nicht automatisch erneut senden. Die App muss
diese Grenzen vor Aktivierung von Autotype verständlich machen; die Browserbrücke
in PR-6 kann später eine genauere Feldadressierung liefern.

## Dummy-Demo und Prüfungen

In jeder PowerShell-Sitzung zuerst die bereitgestellte Build-Umgebung laden:

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-platform --example hotkey_demo -- 120
```

Ein leeres Testfeld fokussieren. F9 öffnet einen nativen Dummy-Popup; OK kopiert
geschützt mit 15 Sekunden Timeout, Schließen/Abbrechen springt zurück. F10 tippt
denselben festen Dummy und kopiert. F11 tippt den Dummy und meldet den erreichten
PR-6-Hook; es wird noch keine KeePassXC-Verbindung aufgebaut. Der Demo-Popup ist
kein Passworteditor; Nachbearbeiten/Neugenerieren wird in der App verdrahtet.
Die Demo beendet sich nach der angegebenen Sekundenzahl. Kein Generator, kein Enter.

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Unit-Tests prüfen Parsing, AltGr-/Doppelbelegungen, Löschentscheidung, Eingabegrenzen,
geheimnisfreie Fehler und Unicode-Input-Paare ohne Desktop-Eingriff. Vier echte
Windows-Tests sind mit `#[ignore]` versehen. Sie ersetzen die aktuelle Zwischenablage,
registrieren globale Hotkeys oder fokussieren eigene native Testfelder und senden
festen Dummy-Text. Nur auf einem ruhenden interaktiven Testdesktop einzeln ausführen:

```powershell
cargo test -p kairos-platform --test windows_interaction -- --ignored --test-threads=1
```

Zusätzlich manuell mit deutschem und englischem Layout prüfen: AltGr+Q bleibt `@`,
die drei Defaults lösen genau ihre Aktion aus, gehaltene Modifier verhindern Text,
Fensterwechsel bricht ab, nach einer neuen Fremdkopie lässt der alte Timeout sie
unverändert, Win+V zeigt keinen Dummy-Verlaufseintrag, und Cloud-Sync übernimmt ihn
nicht. Für ein erhöhtes Ziel muss ein normal gestarteter Prozess kontrolliert
scheitern. Diese Desktop-/Cloud-Checks sind kein Bestandteil von `cargo test`.

## Open-Source-Vorbilder (modellieren, nicht kopieren)

- KeePassXC 2.8.0 und Password Tech: sensible Zwischenablage, begrenzte Lebensdauer,
  Eigentumsprüfung sowie Trennung von Fensteraktivierung und Eingabe.
- `global-hotkey 0.9.0` wird als MIT/Apache-Abhängigkeit genutzt, einschließlich
  Registrierung und Message-Pump-Konzept. `keyboard-types 0.8.3` liefert Key-Codes.
- `arboard 3.7.0` kann alle drei Windows-Ausschlüsse setzen. Seine öffentliche API
  bietet jedoch keine gemeinsame Transaktion für Receipt-Prüfung und Leeren;
  außerdem nutzt es unter Windows `clipboard-win` (BSL-1.0, außerhalb der
  vorgegebenen Lizenzliste). Beide werden daher nur als Modell bewertet.
- enigo liefert das Unicode-/SendInput-Prinzip. Version 0.6.1 enthält einen
  Debug-Logaufruf mit dem Text; daher keine Abhängigkeit für Geheimtexte.
- `windows-sys 0.61.2` liefert die offiziellen Win32-Bindings. `zeroize 1.8.2`
  bleibt die unveränderte Workspace-Abhängigkeit für eigene Geheimnispuffer.

Feste Permalinks, Dateilizenzen, Bewertungen und bewusst verworfene Ansätze stehen
in [platform.md](../../docs/sources/platform.md). Prinzipien wurden übertragen;
fremder Quellcode wurde nicht kopiert.
