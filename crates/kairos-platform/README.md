# kairos-platform

Windows-Dienste für globale Hotkeys, Unicode-Autotype, Fokus-Rücksprung und
Zwischenablage mit Verlauf-/Cloud-Ausschluss. Die Crate ist unabhängig von Core,
KeePassXC und GUI. Die App liefert den Text ausschließlich nach einer Nutzeraktion.
Andere Betriebssysteme erhalten `PlatformError::UnsupportedPlatform`; Konfiguration
und Konfliktprüfung bleiben plattformunabhängig nutzbar.

## API und Integration

### Bildschirmfreigabe und Aufnahmen (PR-11, Prio 7)

`CaptureProtection::new(CapturePolicy::Protect)` vor `eframe::run_native`
auf dem GUI-Thread erstellen und bis zum App-Ende behalten. In dessen
Creation-Callback `guard.apply_to(creation)` aufrufen. Die Plattform leiht
den HWND über `raw-window-handle 0.6`; es gibt keinen Titel-/Vordergrundfenster-
Lookup und keine gespeicherte HWND-Kopie in der App. `set_window_capture_policy`
ist auch für einen geliehenen winit-Handle separat nutzbar.

Ein threadgebundener `WH_CALLWNDPROC`-Hook erfasst zusätzlich Top-Level-Fenster
vor `WM_SHOWWINDOW` bzw. `WM_WINDOWPOSCHANGING` mit `SWP_SHOWWINDOW`. Damit sind
auch später erzeugte native Viewports, nicht aktivierende Popups und eigene
Dialoge auf diesem GUI-Thread erfasst. Child-Controls werden übersprungen;
egui-Dialoge innerhalb eines Viewports teilen dessen Schutz. Der Guard ist
nicht `Send`; ein künftig zusätzlicher GUI-Thread benötigt einen eigenen Guard.
Message-Only-Fenster der Clipboard-/Hotkey-Worker enthalten keine sichtbaren
Passwörter und benötigen keine Anzeigeaffinität. Drop entfernt den Hook.

Reine `GWL_STYLE`-Änderungen mit `WS_VISIBLE` lösen die beobachteten Nachrichten
nicht aus. Das inhaltslose 0×0-Ereignisfenster von winit nutzt diesen Weg und
ist kein Passwortfenster. Ein künftiges Fenster mit sichtbarem Inhalt muss vor
einer solchen Stiländerung ausdrücklich `apply_to` verwenden. Bei zusätzlich
integrierten nativen Viewports nach deren Anzeige auch den Root neu zeichnen
lassen (`Context::request_repaint`), damit ein Hook-Fehler sofort im Banner steht.
Der heutige `--popup`-/F9-Editor verwendet denselben Root und dessen Repaints.
Eine während eines Affinitätsaufrufs übersprungene verschachtelte Anzeigenachricht
setzt den Status auf `Unavailable(NestedWindowShow)` statt Erfolg zu behaupten.

Ab Windows 10 2004 (Build 19041) wird `WDA_EXCLUDEFROMCAPTURE` verwendet,
auf älteren Systemen ausdrücklich `WDA_MONITOR`. Die Betriebssystemversion
kommt über `RtlGetVersion`, unabhängig vom App-Manifest. Jeder Set-Aufruf
wird mit `GetWindowDisplayAffinity` geprüft. `CaptureStatus` unterscheidet
`Pending`, `Excluded`, `MonitorOnly`, `Allowed` und `Unavailable(CaptureError)`.
Ein Fehler in irgendeinem Fenster bleibt im aggregierten Status sichtbar;
ein später erfolgreicher Aufruf verdeckt ihn nicht. Fehler enthalten nur
Operation und numerischen OS-Code. Bei fehlgeschlagenem Hook bleibt der
Status auch dann fehlerhaft, wenn der Root-Aufruf gelingt.

Die App schützt standardmäßig. `--allow-screencapture` wählt für diesen Start
`CapturePolicy::Allow`, setzt am Root `WDA_NONE` und installiert keinen Hook.
Neue Fenster behalten dann ihren ungeschützten Windows-Standard. Die GUI zeigt
dauerhaft „Bildschirmaufnahme erlaubt“. Bei fehlendem/fehlgeschlagenem Schutz
steht „Schutz nicht verfügbar“, beim älteren Fallback „Eingeschränkter
Aufnahmeschutz“. Der Hinweis steht außerhalb des scrollbaren Editors, auch im
`--popup`-Modus. Die Freigabe wird nicht gespeichert.
Schlägt bei bewusstem Opt-out die Bestätigung von `WDA_NONE` fehl, bleibt der
Freigabehinweis sichtbar und erklärt die nicht bestätigte Freigabe separat.

**Grenzen:** Anzeigeaffinität benötigt DWM und erfasst nur Windows-Aufnahmewege,
die diese Einstellung beachten. `WDA_MONITOR` kann eine leere/schwarze Fensterfläche
zurücklassen, statt das Fenster ganz auszublenden. Kameras, Admin-/Treiber-Tools
und manche Fernwartungs-/Capture-Verfahren können den Schutz umgehen. Ein
erfolgreicher API-Aufruf belegt keine universelle Unsichtbarkeit. Teams und
Snipping Tool sind mit [MANUAL-TEST.md](../../docs/MANUAL-TEST.md) am Zielsystem
zu prüfen. Die App bleibt bei Fehlern nutzbar und warnt sichtbar.

**Vorbild:** [KeePassXC 2.8.0, WinUtils.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/osutils/winutils/WinUtils.cpp)
modelliert den Win32-Aufruf; [main.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/main.cpp)
den gleichnamigen Startparameter und den Schutz aller Top-Level-Fenster.
Lizenz der Vorbilddateien ist GPL-2.0 OR GPL-3.0. Nur diese Prinzipien wurden
modelliert; kein fremder Anwendungscode wurde übernommen. Belege und Bewertung
stehen in [platform.md](../../docs/sources/platform.md).

`cargo test -p kairos-platform --test capture_protection` prüft reale Affinität
an eigenen Fenstern sowie spätere unabhängige/Owned-Popups vor dem Anzeigen,
erneutes Anzeigen, Child-Filterung, Hook-Abbau und Opt-out. Fenster stehen
außerhalb des sichtbaren Desktops und werden ohne Fokusanforderung gezeigt.
Die Tests verändern weder Zwischenablage noch Hotkeys und verwenden keine
echten Passwörter. Sie benötigen eine Windows-Sitzung mit DWM. Der Unit-Test
der Versionsentscheidung prüft auch die Buildgrenze 19040/19041; ein tatsächlicher
Alt-Windows-Lauf wird dadurch nicht ersetzt.

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
            // Auch bei fehlendem Fokusziel darf der Editor öffnen.
            // Vor Zerstören des App-Fensters bei Ok(target): target.restore()?
        }
        Action::GenerateTypeCopy => {
            let text = "Fester-Dummy+Äpfel7"; // Die App generiert den echten Text.
            event.target?.type_text(text)?;
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

Es gibt höchstens ein ausstehendes Ereignis; der neueste Tastendruck ersetzt den
alten. Ereignisse über fünf Sekunden werden vor Verarbeitung verworfen. Ein
fehlendes Fokusziel bleibt als `Err` im Ereignis erhalten, die Aktion geht nicht
verloren: `OpenEditor` kann auch ohne Zielfeld öffnen und kopieren. `try_recv()`
meldet nur einen ausgefallenen Dienst als Fehler. Die Message-Pump blockiert mit
`GetMessageW` und benötigt im Leerlauf keine regelmäßigen Aufwachvorgänge.
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
Zulässig sind Buchstaben, Ziffern und F1–F24 mit mindestens zwei Modifiern aus
Ctrl, Alt und Shift; bekannte Ctrl+C/V/X/Z/A- und Alt-Menübelegungen bleiben frei.
F12, Alt+F4, Win-Kombinationen und reine Shift-Kombinationen sind gesperrt.

Die Defaults nutzen Funktionstasten und kein Ctrl+Alt. Windows kann Ctrl+Alt als
AltGr interpretieren; auf deutschem Layout würde etwa Ctrl+Alt+Q das `@` abfangen.
Der Parser lehnt deshalb alle druckbaren Ctrl+Alt-Kombinationen einschließlich
Shift+Ctrl+Alt ab. Diese konservative Entscheidung folgt dem
[Microsoft-Tastaturleitfaden](https://github.com/MicrosoftDocs/win32/blob/48e6f97f72445079ad7dad073a645f9ff694c0b0/desktop-src/uxguide/inter-keyboard.md).
Eine universell konfliktfreie Belegung gibt es nicht. `HotkeyUnavailable` muss
die App sichtbar melden. PR-W bietet Freigabe der Belegung und erneute
Registrierung; eine Umbelegungs-UI folgt bei Bedarf. Lokale Konflikte werden schon
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
Text noch Hash. Der Timeout prüft die pro Schreibvorgang eindeutige Marke unter
der Clipboard-Sperre. Die beobachtete Sequenznummer dient nur der Diagnose;
Windows darf nach dem Schreiben Formate synthetisieren. Auch eine neue
Kopie desselben Textes ist ein fremder Schreibvorgang für den alten Timeout.
Ein einziger Timerthread verwaltet den aktuellen Auftrag und wiederholt bei
vorübergehend belegter Zwischenablage den Löschversuch höchstens 50-mal. Ein
gemeinsamer Auftrag mit Condvar wird durch neue Kopien ersetzt; keine wachsende
Warteschlange und keine Überlast durch rasches Kopieren. Permanente Fehler werden
nicht endlos wiederholt. Nach Standby wird spätestens beim nächsten Sekunden-Tick
auch die Wanduhr geprüft; ein Zurückstellen der Uhr verlängert die monotone Frist
nicht. `None` deaktiviert den Timeout ausdrücklich, `Some(Duration::ZERO)` macht
ihn sofort fällig. Die App soll `Some(DEFAULT_CLEAR_AFTER)` (30 Sekunden) verwenden;
maximal `MAX_CLEAR_AFTER` (zehn Minuten) ist erlaubt. Ein begründet deaktivierter
Timeout bleibt gemäß dem vorgegebenen `Option<Duration>`-Vertrag möglich.
`NotPresent` unterscheidet bereits leere Inhalte von `Replaced`; Lesefehler werden
als Fehler gemeldet und nicht als Besitzerwechsel verschwiegen.
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
`event.target?.type_text(text)` erzwingt die Verwendung dieses Ziels und vermeidet
eine freistehende Funktion, die nach einem Fensterwechsel still neu erfassen würde.

Autotype prüft das Ziel während des Wartens und unmittelbar vor `SendInput` erneut.
Ctrl, Alt, Shift, beide Win-Tasten und die Hotkey-Haupttaste müssen mindestens
20 ms freigegeben sein;
nach zwei Sekunden wird abgebrochen. Nach fünf Sekunden ist ein Autotype-Ziel
abgelaufen. Ein Fenster-/Controlwechsel, ein zerstörtes Ziel, aktive Menüs sowie
unvollständige Eingabe führen zu einem Fehler. Es gibt keine Wiederholung des
Geheimtexts und keinen automatischen Clipboard-Fallback. Eingabe erfolgt als ein
Batch aus Unicode-Down/Up-Paaren, einschließlich UTF-16-Surrogatpaaren, mit
`KEYEVENTF_UNICODE` und `wVk = 0`, unabhängig vom Tastaturlayout. Tab, Enter und
andere Steuerzeichen werden abgewiesen. Text ist auf 1024 UTF-16-Einheiten begrenzt.

`restore()` muss vor dem Zerstören des App-Fensters aufgerufen werden. Es darf das
ursprüngliche Fenster aktivieren, wenn aktuell die eigene App
im Vordergrund ist oder das Ziel bereits aktiv ist. Windows kann dies verweigern;
Rückgabewert und tatsächlicher Fokus werden geprüft; die asynchrone Aktivierung
erhält höchstens 500 ms in 10-ms-Schritten. Es gibt keine synthetische
Alt-Taste, kein `AttachThreadInput` und keine Änderung des Foreground-Lock-Timeouts.
Eine andere inzwischen aktive Anwendung wird nicht verdrängt.

**Windows-Grenzen:** `SendInput` hat keinen Zielparameter. Zwischen dem letzten
Fokuscheck und der tatsächlichen Zustellung bleibt ein Rennen mit programmatischen
Fokuswechseln. Mehrere DOM-Felder im Browser oder Controls einer Canvas-App können
dasselbe native HWND besitzen. Diese API kann dann keinen Feldwechsel erkennen.
Vor dem Senden werden beide Prozess-Integritätsstufen über deren Token abgefragt.
Ein höher privilegiertes Ziel wird mit `TargetElevated` abgewiesen; eine nicht
mögliche Abfrage führt ebenfalls zum Abbruch. Dennoch ist `Ok(())` ausschließlich
die Annahme der Ereignisse durch Windows, keine geprüfte Zustellung ins Feld.
Manche RDP-/VM-/Java-/Spiele-Oberflächen verarbeiten `VK_PACKET` nicht.
`InputIncomplete` kann Teiltext einschließlich eines einzelnen UTF-16-Surrogats
bedeuten; die App darf ihn nicht automatisch erneut senden oder löschen. Die App muss
diese Grenzen vor Aktivierung von Autotype verständlich machen; die Browserbrücke
in PR-6 kann später eine genauere Feldadressierung liefern.

Autotype erzeugt weitere Geheimniskopien in der Windows-Eingabewarteschlange und
im Zielfeld. Lokale Tastaturhooks können Unicode-Ereignisse mitlesen. Zusammen mit
Kopieren entsteht zusätzlich der Clipboard-Pfad; `zeroize` beseitigt diese fremden
Kopien nicht. Autotype ist kein Schutz gegen lokale Schadsoftware.

## Dummy-Demo und Prüfungen

In jeder PowerShell-Sitzung zuerst die bereitgestellte Build-Umgebung laden:

```powershell
# Im betreuten Build-Setup: . '<Hauptcheckout>\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-platform --example hotkey_demo -- 120
```

Ein leeres Testfeld fokussieren. Ctrl+Shift+F9 öffnet einen nativen Dummy-Popup;
OK kopiert geschützt mit 15 Sekunden Timeout, Schließen/Abbrechen springt zurück.
Ctrl+Shift+F10 tippt denselben festen Dummy und kopiert. Ctrl+Shift+F11 tippt den Dummy und meldet den erreichten
PR-6-Hook; es wird noch keine KeePassXC-Verbindung aufgebaut. Der Demo-Popup ist
kein Passworteditor; Nachbearbeiten/Neugenerieren wird in der App verdrahtet.
Die Demo beendet sich nach der angegebenen Sekundenzahl. Kein Generator, kein Enter.

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Unit-Tests prüfen Parsing, AltGr-/Doppelbelegungen, Löschentscheidung, Eingabegrenzen,
geheimnisfreie Fehler, Integritätsregeln, Release-/Aktivierungsfristen und
Unicode-Input-Paare ohne Desktop-Eingriff. Ein OS-Test fragt reale Prozess-Token ab.
Sechs echte
Windows-Tests sind mit `#[ignore]` versehen. Sie ersetzen die aktuelle Zwischenablage,
registrieren globale Hotkeys oder fokussieren eigene native Testfelder und senden
festen Dummy-Text. Nur auf einem ruhenden interaktiven Testdesktop einzeln ausführen:

```powershell
cargo test -p kairos-platform --test windows_interaction -- --ignored --test-threads=1
```

Ein zusätzlicher ignorierter Clipboard-Test kann unabhängig von der aktuellen
Nutzerzwischenablage laufen: Er erstellt eine private unsichtbare Windowstation
mit eigenem Desktop, ohne Desktopwechsel oder Eingabeinjektion. Er prüft reale
Schutzflags, synthetisierte CF_TEXT-/CF_LOCALE-Formate, neue identische Kopien,
20 rasche Timer-Ersetzungen und tatsächliches Timeout-Leeren. Allein in einem
eigenen Testprozess starten:

```powershell
cargo test -p kairos-platform --test isolated_clipboard -- --ignored
```

Zusätzlich manuell mit deutschem und englischem Layout prüfen: AltGr+Q bleibt `@`,
die drei Defaults lösen genau ihre Aktion aus, gehaltene Modifier verhindern Text,
Fensterwechsel bricht ab, nach einer neuen Fremdkopie lässt der alte Timeout sie
unverändert, Win+V zeigt keinen Dummy-Verlaufseintrag, und Cloud-Sync übernimmt ihn
nicht. Ein höher privilegiertes Ziel muss `TargetElevated` oder bei verweigerter
Token-Abfrage `TargetUnavailable` ergeben. Diese Desktop-/Cloud-Checks sind kein
Bestandteil von `cargo test` und noch nicht als ausgeführt belegt.

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

## Additive App-Integration aus PR-W

FocusTarget::restore_and_type_text(text) ist für einen ausdrücklichen neuen
Einfügeklick nach manueller Bearbeitung bestimmt. Es validiert den Text,
führt restore() auf dem ursprünglichen Snapshot aus und beginnt erst danach
eine frische Fünf-Sekunden-Frist für denselben Fenster-/Prozess-/Thread-/Control-
Datensatz. Kein neues Vordergrundziel wird erfasst. Alte automatische
Hotkey-Ereignisse dürfen diese API nicht zum Umgehen ihrer Frist verwenden.
Modifier-, Integritäts-, Unicode-, Teilerfolgs- und Fokusprüfungen bleiben aktiv.
is_current_process() erlaubt, in der App eigene Eingabeziele abzuweisen,
ohne Fensterhandles offenzulegen. Der bestehende type_text-Vertrag ist unverändert.

Modelled after: [KeePassXC AutoType.cpp 2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp).
Nur die Bewahrung der Zielidentität über einen interaktiven Auswahl-/Editor-
schritt wird modelliert; kein Code und keine Retry-Logik wird kopiert.
Der zusätzliche ignorierte Test
editor_confirmation_preserves_identity_after_editing_deadline prüft
Fristablauf, Ablehnung eines gewechselten Controls und explizite Unicode-Eingabe.
Insgesamt sind nun sechs interaktive Tests vorhanden, daneben der isolierte
Clipboard-Test. Ohne ausdrücklichen Lauf bleiben sie ungeprüft.

Die reale App nutzt GlobalHotkeys::with_wakeup und eframes App::logic.
F9 öffnet den Editor, F10 tippt und kopiert ohne Fokusanforderung für Kairos,
F11 zeigt vorerst nur den PR-6-Hinweis. Sie behält die letzte ClipboardReceipt
für bedingtes Leeren beim normalen App-Ende. Die Desktop-Checkliste steht in
[MANUAL-TEST.md](../../docs/MANUAL-TEST.md).
