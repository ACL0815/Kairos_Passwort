# kairos-hardening

Optionale Schutzbibliothek für den Windows-Wortpasswortgenerator. PR-9a liefert
die Bausteine; der Aufruf beim App-Start und die Verwendung in GUI/Core folgen
in einem eigenen Integrations-PR. Es gibt keine Systemeinstellungen oder Registry-
Änderungen. Die Reihenfolge des Grundkonzepts bleibt CSPRNG → Gleichverteilung →
Länge → keine unnötige Speicherung → Zwischenablage → Überschreiben/Dump-Schutz.

## Prozessstart

`harden_process() -> HardeningReport` einmal vor sensibler Verarbeitung aufrufen.
Jeder Eintrag enthält `Applied`, den unveränderten Win32-/HRESULT-Fehlercode oder
`Unsupported`. Die App muss unvollständigen Schutz sichtbar machen. Die Funktion
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
  Handles bleiben verwendbar. Administratoren mit SeDebugPrivilege, Kernelcode,
  Tokenzugriff und Schadcode im Prozess bleiben außerhalb der Schutzwirkung.
- ExtensionPointDisable deaktiviert alte DLL-Erweiterungspunkte. ImageLoad
  verbietet neue Remote-/Low-Integrity-Images und bevorzugt System32. Bestehende
  Flags bleiben erhalten, Rücklesen kontrolliert die angeforderten Bits.

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
`close(self)` gibt Entsperrfehler zurück. Drop kann Fehler nicht zurückgeben.
Die Allokation besitzt keine Debug-/Clone-Implementierung. Ausgeliehene Bytes
können Aufrufer weiterhin kopieren.

Für die App empfiehlt sich ein vorab dimensionierter Puffer, direkte Erzeugung
oder kurzlebige Übernahme aus `GeneratedPassword.secret`, danach sofortiges
Löschen des bisherigen `Zeroizing<String>`. Diese Übernahme verhindert keine
früheren Core-, Stack-, UI-, Undo- oder Zwischenablagekopien. Es gibt bewusst
keinen automatischen Konvertierungsweg, der eine neue Kopie verbirgt.

`region` 3.0.2 liefert geprüfte VirtualAlloc/VirtualLock/VirtualUnlock-Aufrufe
unter Windows. Seine Allocation-RAII-Freigabe prüft VirtualFree intern mit einer
Debug-Assertion; einen Freigabefehler kann diese API nicht an den Aufrufer geben.
Die App sollte `close` nutzen. `VirtualLock` hat ein kleines Working-Set-Limit,
verhindert keine Dumps, keine Zugriffe auf lesbare Seiten und keine
Ruhezustandsabbilder. Andere Betriebssysteme nutzen die region-Backends;
Prozesshärtung wird dort als `Unsupported` berichtet.

## Prüfung

`cargo test -p kairos-hardening` prüft Puffergrenzen, stabile Adressen bei Moves,
Löschen einschließlich Padding, unabhängige Seiten, explizites Entsperren,
Report-Fehler und die echten Prozessmaßnahmen in einem isolierten Kindprozess.
Der Kindprozess prüft auch, dass derselbe Benutzer keine neuen gefährlichen
Prozesshandles öffnen kann und QueryLimitedInformation weiter funktioniert.

Der ignorierte Test erzeugt ausschließlich synthetische Daten in einem eigenen
Kindprozess und schreibt zwei Full-Memory-Dumps dieses Prozesses:

```powershell
cargo test --release -p kairos-hardening --test memory_dump -- --ignored --nocapture
```

Ein Laufzeit-Seed erzeugt die bekannte ASCII-Testpasswortfolge direkt in zwei
LockedSecret-Puffern als UTF-8 und UTF-16LE. Der erste Dump enthält die noch
lebenden Puffer, der zweite entsteht nach `close`. Zusätzlich müssen beide
Kodierungen einer anderen, noch lebenden Positivkontrolle im zweiten Dump
gefunden werden. Erst der Elternprozess baut die Suchfolgen auf und scannt
mit überlappenden Lesefenstern. Passwörter stehen nicht als Literale, Argumente
oder Umgebungswerte im Kind. Der Seed ist ein Testprotokoll, kein Geheimnis und
kein produktiver Passwortgenerator. Temporäre Dumps werden auch bei normalen
Testpaniken entfernt; bei hartem Prozessabbruch kann Aufräumen ausbleiben.

Der Test benutzt MiniDumpWriteDump im eigenen Prozess, wie im Auftrag verlangt.
Microsoft empfiehlt für produktive Diagnose einen externen Helper, weil Selbst-
Dumps Loader-Deadlocks riskieren. Der Test ist deshalb manuell und ignoriert.
Er lässt keine unzugänglichen Regionen per IgnoreInaccessibleMemory aus.
Ein fehlender Treffer beweist keine vollständige Löschung. Gemessen werden nur
die bekannten zusammenhängenden Bytefolgen zu den beiden Zeitpunkten in den
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
