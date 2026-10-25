# kairos-hardening

Optionale Schutzbibliothek für den Windows-Wortpasswortgenerator. PR-9a liefert
die Bausteine; der Aufruf beim App-Start und die Verwendung in GUI/Core folgen
in einem eigenen Integrations-PR. Es gibt keine Systemeinstellungen oder Registry-
Änderungen. Die Reihenfolge des Grundkonzepts bleibt CSPRNG → Gleichverteilung →
Länge → keine unnötige Speicherung → Zwischenablage → Überschreiben/Dump-Schutz.

## Prozessstart

`harden_process() -> HardeningReport` einmal vor sensibler Verarbeitung aufrufen.
Jeder Eintrag enthält `Applied`, einen typisierten Fehler (`OsError::Win32` oder
`OsError::HResult`) mit unverändertem Code oder `Unsupported`. Die App muss
unvollständigen Schutz sichtbar machen. Die Funktion
panikt nicht bei OS-Fehlern und ändert nur ihren eigenen Prozess. Manche Policies
sind nicht rückgängig zu machen. Die App muss die Kompatibilität mit Hotkeys,
Eingabemethoden, Barrierefreiheit und nativen Bibliotheken separat prüfen.

- WER behält bestehende Flags und ergänzt NOHEAP, NO_HEAP_ON_QUEUE sowie die
  Sperre von Crash-/Hang-Snapshots. Rücklesen prüft die wirksamen Flags.
  Das ist keine globale Dumpsperre. WER LocalDumps, Debugger, manuelle Dumps,
  Kernel-Dumps und andere Crashhandler können weiter Speicher aufnehmen.
- Die Prozess-DACL verweigert neue Handles für VM_READ, VM_WRITE, VM_OPERATION,
  CREATE_THREAD, DUP_HANDLE, WRITE_DAC und WRITE_OWNER. OWNER RIGHTS unterdrückt
  das implizite WRITE_DAC des Besitzers. QueryLimitedInformation, READ_CONTROL
  und Synchronize bleiben erlaubt. Die DACL ersetzt die bestehende DACL und
  kann Diagnosewerkzeuge beeinträchtigen. Bereits geöffnete oder geerbte
  Handles bleiben verwendbar. Neue Handles mit QUERY_INFORMATION, SUSPEND_RESUME
  oder TERMINATE werden ebenfalls nicht erlaubt. Ein gewöhnlicher Task-Manager
  desselben Nutzers kann den Prozess daher möglicherweise nicht mehr beenden;
  ein zuvor geöffneter Handle oder erhöhte Debugrechte bleiben Ausnahmen.
  **Der Schutz wirkt nicht gegen Prozesse mit aktiviertem SeDebugPrivilege
  oder gegen Administratoren, die dieses Privileg aktivieren können.**
  OpenProcess überspringt dann die DACL-Prüfung. Elevation allein ist kein
  Nachweis für ein aktiviertes Privileg. `ProcessDacl: Applied` bestätigt die
  gesetzte DACL, keine Zugriffsverweigerung gegenüber privilegierten Prozessen.
  Kernelcode, Tokenzugriff und Schadcode im Prozess bleiben ebenfalls außerhalb
  der Schutzwirkung.
- ExtensionPointDisable deaktiviert alte DLL-Erweiterungspunkte. ImageLoad
  verbietet neue Remote-/Low-Integrity-Images und bevorzugt System32. Bestehende
  Flags bleiben erhalten, Rücklesen kontrolliert die angeforderten Bits.
  NoRemoteImages kann spätere DLL-Loads von UNC-/Netzlaufwerken verhindern;
  PreferSystem32 verändert die DLL-Suche. Portable Installationen auf Netzmedien
  brauchen deshalb eine eigene Kompatibilitätsprüfung vor der App-Integration.

DEP/ASLR/CFG müssen bereits zum Build und Prozessstart passen. Nachträgliches
Setzen kann fehlende Compilerinstrumentierung oder bereits geladene Images
nicht nachholen. Strikte Handleprüfung beendet bei ungültigen Handles den Prozess
und bleibt nach Abwägung zunächst aus. DynamicCode-Verbot, Microsoft-only-
Signaturpolicy, Win32k-Sperre, Font-Verbot und ChildProcess-Verbot sind für eine
Desktop-App ohne Integrationstest zu einschneidend. Die Bewertung samt Chromium-
und Firefox-Vorbildern steht in [protection.md](../../docs/sources/protection.md).

## Gesperrter Puffer

`LockedSecret::new(capacity)` reserviert 1–65536 nutzbare Bytes in eigenen,
seitenweise gerundeten `READ_WRITE`-Seiten. Die Seiten werden vor der ersten
Geheimniskopie gesperrt. Fehlschlag liefert `Err`, es gibt keinen ungesperrten
Ersatzpuffer. `extend_from_slice` und `replace` wachsen nie; Überlänge lässt den
bisherigen Wert unverändert. `clear` und Drop überschreiben die ganze Allokation,
auch ungenutztes Padding, mit `zeroize`, bevor sie entsperrt/freigegeben wird.
`close(self)` gibt Entsperrfehler und unter Windows auch VirtualFree-Fehler
zurück. Drop kann Fehler nicht zurückgeben.
Die Allokation besitzt keine Debug-/Clone-Implementierung. Ausgeliehene Bytes
können Aufrufer weiterhin kopieren.

`LockedSecret` ist mit dem gewählten region-Backend `!Send` und `!Sync` und bleibt
am erzeugenden Thread. Die spätere GUI-/Worker-Integration muss den Puffer dort
anlegen und verwenden. Es gibt keine ungeprüfte `unsafe impl Send`-Zusicherung.
`HardeningError::Memory` enthält derzeit `region::Error`; die öffentliche
Fehlerstruktur ist damit bewusst an die gepinnte Version 3.0.2 gekoppelt. Vor
einer stabilen externen API muss diese Kopplung neu bewertet werden.

Für die App empfiehlt sich ein vorab dimensionierter Puffer, direkte Erzeugung
oder kurzlebige Übernahme aus `GeneratedPassword.secret`, danach sofortiges
Löschen des bisherigen `Zeroizing<String>`. Diese Übernahme verhindert keine
früheren Core-, Stack-, UI-, Undo- oder Zwischenablagekopien. Es gibt bewusst
keinen automatischen Konvertierungsweg, der eine neue Kopie verbirgt.

`region` 3.0.2 liefert geprüfte VirtualAlloc/VirtualLock/VirtualUnlock-Aufrufe
unter Windows. `close` ersetzt dort die Allocation-RAII-Freigabe durch einen
explizit geprüften VirtualFree-Aufruf, auch wenn VirtualUnlock fehlschlägt.
`HardeningError::MemoryRelease` erhält den Freigabefehler und einen eventuell
gleichzeitigen Unlock-Fehler. Bei fehlgeschlagener Freigabe bleiben die bereits
gelöschten Seiten reserviert; es gibt keinen zweiten Freigabeversuch im Drop.
Der gewöhnliche Drop nutzt weiterhin region, das Freigabefehler nur per
Debug-Assertion prüft. Die App sollte deshalb `close` nutzen. Auf anderen
Plattformen meldet `close` weiterhin nur Unlock-Fehler. `VirtualLock` hat ein
kleines Working-Set-Limit, verhindert keine Dumps, keine Zugriffe auf lesbare Seiten und keine
Ruhezustandsabbilder. Andere Betriebssysteme nutzen die region-Backends;
Prozesshärtung wird dort als `Unsupported` berichtet.

## Prüfung

`cargo test -p kairos-hardening` prüft Puffergrenzen, stabile Adressen bei Moves,
Löschen einschließlich Padding, unabhängige Seiten, explizites Entsperren und
Freigeben einschließlich realer OS-Fehlerinjektion,
Report-Fehler und die echten Prozessmaßnahmen in einem isolierten Kindprozess.
Der Kindprozess protokolliert Elevation sowie initialen und verfügbaren
SeDebugPrivilege-Status. Falls verfügbar, wird der dokumentierte DACL-Bypass
mit aktiviertem Privileg für alle zehn gefährlichen Rechte geprüft. Anschließend
wird das Privileg ausschließlich im wegwerfbaren Kindprozess deaktiviert.
Alle zehn Rechte müssen dann ERROR_ACCESS_DENIED liefern, einschließlich
WRITE_DAC und WRITE_OWNER für den Prozessbesitzer. Diese Prüfung wird auch
auf erhöhten CI-Runnern niemals übersprungen. QueryLimitedInformation muss
in beiden Kontexten funktionieren. Ohne verfügbares Debugprivileg ist nur
der privilegierte Bypass nicht prüfbar; die Schutzprüfung bleibt vollständig.

Der ignorierte Test erzeugt ausschließlich synthetische Daten in einem eigenen
Kindprozess und schreibt drei Full-Memory-Dumps dieses Prozesses:

```powershell
cargo test --release -p kairos-hardening --test memory_dump -- --ignored --nocapture
```

Ein Laufzeit-Seed erzeugt die bekannte ASCII-Testpasswortfolge direkt in zwei
LockedSecret-Puffern als UTF-8 und UTF-16LE. Der erste Dump enthält die Werte.
Der zweite entsteht nach `clear()`, während dieselben Seiten weiter belegt und
gesperrt bleiben; dort müssen beide Werte fehlen. Damit prüft er das Löschen
vor dem Unmapping. Der dritte entsteht nach `close()` und prüft zusätzlich auf
Restkopien außerhalb der freigegebenen Puffer. In beiden späteren Dumps müssen
beide Kodierungen einer anderen, noch lebenden Positivkontrolle auffindbar sein.
Erst der Elternprozess baut die Suchfolgen auf und scannt
mit überlappenden Lesefenstern. Passwörter stehen nicht als Literale, Argumente
oder Umgebungswerte im Kind. Der Seed ist ein Testprotokoll, kein Geheimnis und
kein produktiver Passwortgenerator. Temporäre Dumps werden auch bei normalen
Testpaniken entfernt; bei hartem Prozessabbruch kann Aufräumen ausbleiben.

Der Test benutzt MiniDumpWriteDump im eigenen Prozess, wie im Auftrag verlangt.
Microsoft empfiehlt für produktive Diagnose einen externen Helper, weil Selbst-
Dumps Loader-Deadlocks riskieren. Der Test ist deshalb manuell und ignoriert.
Er lässt keine unzugänglichen Regionen per IgnoreInaccessibleMemory aus.
Ein fehlender Treffer beweist keine vollständige Löschung. Gemessen werden nur
die bekannten zusammenhängenden Bytefolgen zu den drei Zeitpunkten in den
erfassten Bereichen. GUI, Generator, Clipboard und Zielprogramme sind kein Teil
dieses Tests. Eigene Speicherlöschung ersetzt deren spätere Prüfung nicht.

## Open-Source-Vorbilder

Modelled after: KeePassXC Bootstrap.cpp/Alloc.cpp für frühe Prozesshärtung und
explizites Löschen, libsodium für eigene gesperrte Allokationen und Bitwarden SDK
memory-testing für Dump-Suche und Testhygiene. Es wurde kein Implementierungscode
aus diesen Projekten übernommen. `region`, `zeroize`, RustCrypto `hmac` im Store
und Win32-Bindings werden als kompatible Crates verwendet. Die geprüften
Quellstände, Lizenzen und Alternativen stehen vollständig in
[protection.md](../../docs/sources/protection.md).
