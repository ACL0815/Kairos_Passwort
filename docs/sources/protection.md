# Prozesshärtung und Manipulationserkennung

Zuständig ist PR-9a, Prio 5, Branch `feat/protection`, Basis `main` bei
`fa35aded13b50a67a817db79258b1ba1c95a9d46`. Alle Abrufe/Bewertungen vom
**25.10.2026, Europe/Berlin**. Die Zahlen bewerten die Passung zum lokalen
Windows-Generator, keine allgemeine Produktsicherheit.

Es werden ausschließlich Prinzipien modelliert, unabhängig von Sprache oder
Lizenz. Es wurde kein Implementierungscode aus den Vorbildern kopiert. Bestehende
kompatible Crates werden als Abhängigkeiten genutzt. Die einzige für direkte
Codeübernahme freigegebene Ausnahme bleibt `rand`; hier war keine erforderlich.
Dateilizenzen gehen pauschalen Repository-Angaben vor. API-Dokumentation ist
keine Codequelle; dort existiert häufig kein Commit-Permalink.

## Bewertete Vorbilder

| Projekt | Permalink (Commit/Tag/Version) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 und Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| KeePassXC | [2.8.0 Bootstrap.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp) | GPL-2.0 OR GPL-3.0, Dateiheader | `src/core/Bootstrap.cpp` | Früh härten, Prozessrechte minimieren, OWNER RIGHTS beachten, Fehlschlag melden | **5**. Grundlage für DACL; kein Schutz gegen Admin-/Debugrechte | 2026-10-25 |
| KeePassXC | [2.8.0 Alloc.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Alloc.cpp) | GPL-2.0 OR GPL-3.0, Dateiheader | `src/core/Alloc.cpp` | Freigabe von garantiertem Überschreiben unterscheiden | **4**. Löschprinzip passend, globale C++-Operatoren passen nicht zur Rust-Besitzstruktur | 2026-10-25 |
| KeePass | [Sicherheitsdokumentation](https://keepass.info/help/base/security.html) | Dokumentation, keine OSS-Dateilizenz ausgewiesen | Prozessspeicher/Klartextkopien | UI, Runtime und OS können Kopien erzeugen | **5**. Maßgebliche Grenze der Schutzversprechen | 2026-10-25 |
| Bitwarden Desktop | [b449ac1d…](https://github.com/bitwarden/clients/blob/b449ac1d255784380d75ef026343198891286be7/apps/desktop/desktop_native/core/src/process_isolation/windows.rs) | GPL-3.0, Repository-Lizenz | `desktop_native/core/src/process_isolation/windows.rs` | Native Prozesshärtung kapseln und Fehler weiterreichen | **3**. Gute API-Grenze; Windows-Core-Dump-Deaktivierung dort nicht implementiert | 2026-10-25 |
| secmem-proc | [0.3.7](https://docs.rs/secmem-proc/0.3.7/secmem_proc/) | MIT OR Apache-2.0 | Windows-Prozess-DACL | Minimale Prozessrechte | **3**. OWNER-RIGHTS-Wirkung separat prüfen; kein pauschaler Dump-Schutz | 2026-10-25 |
| Chromium | [1554b574…](https://chromium.googlesource.com/chromium/src/+/1554b5743311a7850aebb2da0e0e04ac9f8ab968/docs/design/sandbox.md) | BSD-3-Clause | `docs/design/sandbox.md` | Mitigationen passend zu Prozessrolle und Zeitpunkt wählen | **4**. Gute Auswahlkriterien; Browser-Sandbox nicht auf Desktop-App übertragbar | 2026-10-25 |
| Firefox | [efd2d1e6…](https://searchfox.org/firefox-main/rev/efd2d1e6f77570ae6bc5294731a68101e45c81a3/browser/app/winlauncher/LauncherProcessWin.cpp) | MPL-2.0, Dateiheader | `browser/app/winlauncher/LauncherProcessWin.cpp` | Mitigations beim Start mit Architektur-/Kompatibilitätsausnahmen | **4**. Prozessattribute statt direktem SetProcessMitigationPolicy; zeigt Grenzen später Policies | 2026-10-25 |
| libsodium | [1.0.20 utils.c](https://github.com/jedisct1/libsodium/blob/1.0.20/src/libsodium/sodium/utils.c) | ISC | `src/libsodium/sodium/utils.c` | Eigene Seiten, Sperren und Scrubbing | **5**. Passendes Besitz-/Löschprinzip; keine native libsodium-Abhängigkeit nötig | 2026-10-25 |
| Rust secrets | [1.3.0 Quellstand](https://docs.rs/crate/secrets/1.3.0/source/src/secret.rs) | MIT OR Apache-2.0 | `secret.rs`, SecretBox/SecretVec | Feste geschützte Puffer und Löschung | **2**. Sperrfehler können paniken; passt nicht zur falliblen API | 2026-10-25 |
| Rust memsec | [0.7.0 mlock](https://docs.rs/memsec/0.7.0/memsec/fn.mlock.html) | MIT | `mlock`, `munlock`, `memzero` | VirtualLock mit prüfbarem bool | **3**. Besitz, eigene Seiten und Fehlerobjekte müssten zusätzlich gebaut werden | 2026-10-25 |
| Rust region | [3.0.2, fc89b0c7…](https://github.com/darfink/region-rs/tree/fc89b0c76d51bd13a591dca5fc9b9c8a0224aa18/src) | MIT, Crate-Manifest | `alloc.rs`, `lock.rs`, `os/windows.rs` | Eigene Seiten, fallibles Allokieren/Sperren/Entsperren | **5**. Gewählt und auf 3.0.2 fixiert, SHA aus .cargo_vcs_info.json geprüft. Löscht nicht selbst; Allocation-Drop prüft Free nur per Debug-Assertion | 2026-10-25 |
| Rust secrecy | [0.10.3](https://docs.rs/secrecy/0.10.3/secrecy/) | MIT OR Apache-2.0 | `SecretBox`, Zugriffstraits | Bewusstes Exponieren, Debug-Schutz, zeroize | **2**. Allein kein VirtualLock und keine eigenen gesperrten Seiten | 2026-10-25 |
| RustCrypto zeroize | [zeroize-v1.8.2](https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/src/lib.rs) | Apache-2.0 OR MIT | `Zeroize`, `Zeroizing` | Optimierungsfestes Überschreiben, Grenzen bei Moves/Reallokation | **5**. Bereits Core/Store, jetzt auch ganze LockedSecret-Allokation | 2026-10-25 |
| RustCrypto hmac | [0.12.1](https://docs.rs/crate/hmac/0.12.1/source/src/lib.rs) | MIT OR Apache-2.0 | `Mac`, `Hmac` | Standard-HMAC und zeitkonstante Tag-Verifikation | **5**. Gewählte Krypto-Abhängigkeit, kein selbst gebauter MAC | 2026-10-25 |
| minisign | [b85e15d45ac9eab34e44596fd309f5b07db9545c](https://github.com/jedisct1/minisign/blob/b85e15d45ac9eab34e44596fd309f5b07db9545c/README.md) | ISC, LICENSE am Commit | `README.md`, Signaturformat | Herausgeberidentität über vertrauten Ed25519-Public-Key | **3**. Für veröffentlichte Listen gut; Privatimporte haben keinen Herausgeberschlüssel | 2026-10-25 |
| OpenBSD signify | [af1d15b1…](https://github.com/openbsd/src/blob/af1d15b12c89b408915f77e38fc74d527327c0fc/usr.bin/signify/signify.c) | ISC, Dateiheader | `signify.c`, `signify.1` | Signierte Prüfsummenliste bindet Dateien | **3**. Gutes Manifestprinzip; Schlüsselvertrauen und Aktualität separat | 2026-10-25 |
| TUF Specification | [v1.0.36](https://github.com/theupdateframework/specification/blob/v1.0.36/tuf-spec.md) | Community Specification License 1.0, [Lizenz](https://github.com/theupdateframework/specification/blob/v1.0.36/LICENSE.md) | `tuf-spec.md` | Identitäten, Längen/Hashes, Rollen, Versionen, Ablaufzeiten | **4**. Für verteilte Updates/Anti-Rollback gut; vollständiges TUF für lokalen Import zu groß | 2026-10-25 |
| Sigstore Cosign | [v3.1.4](https://github.com/sigstore/cosign/blob/v3.1.4/specs/SIGNATURE_SPEC.md) | Apache-2.0 | `specs/SIGNATURE_SPEC.md` | Signaturen, Identität und Transparenznachweise | **2**. Für Releases passend, für lokale Listen unnötige Vertrauenskette | 2026-10-25 |
| Windows Code Integrity / Kataloge | [Code Integrity](https://learn.microsoft.com/en-us/windows/security/book/operating-system-security-system-security), [Kataloge](https://learn.microsoft.com/en-us/windows-hardware/drivers/install/catalog-files) | Windows-/Microsoft-Dokumentation, kein OSS-Code übernommen | Code Integrity, `.cat` | Signierte Hashkataloge und Codevertrauen unterscheiden | **1**. Schützt Textimporte nicht automatisch; Katalogprüfung wäre eigener Mechanismus | 2026-10-25 |
| KeePassXC KDBX4 | [2.8.0 HmacBlockStream](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/streams/HmacBlockStream.cpp), [Kdbx4Reader](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/format/Kdbx4Reader.cpp) | GPL-2.0 OR GPL-3.0, Dateiheader | `HmacBlockStream.cpp`, `Kdbx4Reader.cpp` | Header/Daten authentisieren, Index und Länge binden, Fehler melden | **5**. Gewähltes logisches Vorbild; kein KDBX-Code/-Format übernommen | 2026-10-25 |
| SQLCipher | [v4.19.0](https://github.com/sqlcipher/sqlcipher/tree/v4.19.0), [Design](https://www.zetetic.net/sqlcipher/design/) | BSD-3-Clause für CE; SQLite-Anteile Public Domain | Seiten-Codec, `cipher_integrity_check` | Verschlüsselte Seiten mit abgeleiteten MAC-Schlüsseln authentisieren | **5**. Bestehende Store-Grundlage; kein logischer Bestandsvergleich oder Anti-Replay | 2026-10-25 |
| WinGet Paket-Manifeste | [509f52b0…](https://github.com/microsoft/winget-pkgs/blob/509f52b0d41f15dfeaf086b8f36f35755f02a237/doc/manifest/schema/1.0.0/installer.md) | MIT, Repository; Installer separat | `doc/manifest/schema/1.0.0/installer.md` | SHA-256 je Objekt plus Metadaten | **4**. Einfaches Hashmodell; Referenzmanifest braucht Authentisierung | 2026-10-25 |
| Bitwarden SDK memory-testing | [5cf91983…](https://github.com/bitwarden/sdk-internal/blob/5cf919837e83b71c2508bf99344ea57cf5f84e88/crates/memory-testing/src/lib.rs) | GPL-3.0-only OR LicenseRef-Bitwarden-SDK, [Manifest](https://github.com/bitwarden/sdk-internal/blob/5cf919837e83b71c2508bf99344ea57cf5f84e88/crates/memory-testing/Cargo.toml) | `crates/memory-testing/src/lib.rs` | Bekannte Werte suchen, Testkopien löschen, Positivkontrolle | **5**. Gewähltes Testprinzip; Datei ist kein Windows-Dumpwriter. Kairos hält Suchwerte im Elternprozess | 2026-10-25 |
| Microsoft windows-sys | [0.61.2](https://docs.rs/windows-sys/0.61.2/windows_sys/Win32/System/Diagnostics/Debug/fn.MiniDumpWriteDump.html) | MIT OR Apache-2.0 | Win32-Bindings | Typisierte FFI statt eigener ABI-Deklarationen | **5**. Vorhandene Crate-Familie; Rückgaben/Handles bleiben unsere Verantwortung | 2026-10-25 |
| obfstr, PR-9b | [9e56c193…](https://github.com/CasualX/obfstr/tree/9e56c1938dfd3bd65fe305fbafcb57394c701356) | MIT | `src/lib.rs` | Verschleierung als begrenzte Hürde | **3**. Folge-PR nach PR #7; hier keine Wortlisten-/App-Änderungen | 2026-10-25 |

## Maßgebliche Win32-Dokumentation

| Projekt | Permalink | Lizenz | Datei/API | Prinzip | Passung und Bewertung | Abrufdatum |
|---|---|---|---|---|---|---|
| Microsoft Learn | [Prozessrechte](https://learn.microsoft.com/en-us/windows/win32/procthread/process-security-and-access-rights), [OWNER RIGHTS](https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-identifiers) | API-Dokumentation, kein Code übernommen | OpenProcess, DACL, S-1-3-4 | VM_READ, DUP_HANDLE und implizites Owner-WRITE_DAC berücksichtigen | **5**. Echte neue Handles testen; Alt-Handles und SeDebugPrivilege bleiben Grenzen | 2026-10-25 |
| Microsoft Learn | [OpenProcess](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess), [AdjustTokenPrivileges](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges) | API-Dokumentation, kein Code übernommen | SeDebugPrivilege, ERROR_NOT_ALL_ASSIGNED | Aktiviertes Debugprivileg umgeht den Security Descriptor; verfügbar ist nicht gleich aktiviert | **5**. Test protokolliert Elevation/Privileg und prüft Bypass sowie Verweigerung nach Deaktivierung im isolierten Kind | 2026-10-25 |
| Microsoft Learn | [VirtualFree](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree) | API-Dokumentation, kein Code übernommen | MEM_RELEASE | Gesamte VirtualAlloc-Reservierung mit Größe 0 freigeben und Fehler auswerten | **5**. close prüft den Aufruf explizit und erhält gleichzeitige Unlock-/Free-Fehler; Test injiziert ungültige Größe 1 | 2026-10-25 |
| Microsoft Learn | [VirtualLock](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock), [VirtualUnlock](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualunlock) | API-Dokumentation | Speicher-Locking | Rückgaben prüfen, Seiten exklusiv besitzen, vor Unlock löschen | **5**. Gewählt; kein Schutz gegen Lesen oder Dumps | 2026-10-25 |
| Microsoft Learn | [WerGetFlags](https://learn.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-wergetflags), [WerSetFlags](https://learn.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-wersetflags), [LocalDumps](https://learn.microsoft.com/en-us/windows/win32/wer/collecting-user-mode-dumps) | API-Dokumentation | WER-Flags, LocalDumps | Heap-/Snapshotdaten reduzieren, keine globale Dumpsperre behaupten | **5**. Nur pro Prozess, keine Registry. ERROR_NOT_FOUND vor erstem Set im frischen nativen Prozess lokal geprüft | 2026-10-25 |
| Microsoft Learn | [Mitigation API](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy), [Extension Points](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_extension_point_disable_policy), [Image Load](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy) | API-Dokumentation | Get/SetProcessMitigationPolicy | Lesen, gewünschte Bits ergänzen, zurücklesen | **5**. Früher Aufruf und App-Kompatibilität erforderlich | 2026-10-25 |
| Microsoft Learn | [CryptProtectData](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) | API-Dokumentation | CurrentUser-DPAPI | Lokalen Zufallsschlüssel geschützt speichern | **5**. Vorhandener Store-Key schützt auch abgeleiteten Manifest-Key; same-user-Angreifer kann DPAPI nutzen | 2026-10-25 |
| Microsoft Learn | [MiniDumpWriteDump](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/nf-minidumpapiset-minidumpwritedump), [MINIDUMP_TYPE](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ne-minidumpapiset-minidump_type) | API-Dokumentation | DbgHelp | FullMemory und getrennten Scanner nutzen | **5**. Selbst-Dump im synthetischen Kind wie beauftragt; Microsoft bevorzugt externen Helper gegen Loader-Deadlocks | 2026-10-25 |

Stichproben des PR-Workers prüfen Bootstrap.cpp, HmacBlockStream.cpp und deren
Dateiheader, Bitwardens Crate-Lizenz, den minisign-Commit, TUF-Lizenz sowie
region 3.0.2 im Registry-Quellstand einschließlich Backend, LockGuard und
Allocation-Drop. Die drei Luna-Recherchen decken Prozess-/Speicherschutz,
Integritätsmodelle und Dump-Tests ab.

## Entscheidungen für PR-9a

Die sechs Lehren des Grundkonzepts bleiben maßgeblich. Sicherer Zufall und
Gleichverteilung stammen aus Core/rand. Länge und wenige Kopien haben Vorrang.
Dieser PR ergänzt kurze Lebensdauer, explizites Überschreiben, gesperrte Seiten,
begrenzte Prozessrechte und einen messbaren Test. Er ergänzt keinen Verlauf,
keine eigene RAM-Verschlüsselung und keine Zwischenablageintegration.

`kairos_hardening::harden_process()` liefert einen Report für jede Maßnahme.
WER reduziert Heap-/Snapshotdaten. Die DACL sperrt neue gefährliche Handles auch
für den Besitzer, behält minimale Abfragerechte und wird zuletzt gesetzt.
Mitigationen für alte Extension Points und neue Remote-/Low-Integrity-Images
werden gelesen, ergänzt und zurückgelesen. DEP/ASLR/CFG sind Build-/Startfragen;
strikte Handles können Prozesse beenden. DynamicCode-, Signatur-, Win32k-,
Font- und ChildProcess-Sperren bleiben bis zur App-Kompatibilitätsprüfung aus.
Details stehen im [Hardening-README](../../crates/kairos-hardening/README.md).

`LockedSecret` kombiniert region mit zeroize, besitzt eigene Seiten und feste
Kapazität. Der Lock muss vor erster Nutzung gelingen. Ein bewusster `close`
macht Unlock-Fehler sichtbar. Die ganze Allokation wird zuerst überschrieben.
Keine automatische Core-Integration, kein Ersatz für UI-/Clipboard-Löschung.

### Welche Projekte könnte man für Manipulationserkennung modellieren?

Für lokale importierte Listen sind **KeePassXC KDBX4 und SQLCipher** die besten
Vorbilder für vollständige Authentisierung, Fehlerbehandlung und Schlüsseltrennung.
**Paket-Manifeste und TUF** ergänzen klar gebundene Objektidentitäten, Längen und
Inhaltshashes. Deshalb bindet unser HMAC-SHA-256-Manifest Quell-SHA-256 und den
Hash der tatsächlich gespeicherten Wörter samt Sprache und Reihenfolge sowie
alle bestehenden Listenmetadaten. Die Manifestdomäne v2 bindet zusätzlich
`user_version`, `application_id` und sämtliche Schemaobjekte aus `sqlite_master`,
einschließlich Triggern, Views, Indizes und Constraints. Es authentisiert auch den gesamten Listenbestand.
Das ist ein eigener lokaler Datentyp, kein KDBX- oder TUF-Format.

**minisign/signify** wären passend, wenn später ein Herausgeber Listen mit einem
vorab vertrauten Public Key veröffentlicht. **Sigstore** wäre eher für Releases
und nachvollziehbare Herstelleridentität sinnvoll. **Windows Code Integrity**
prüft Codevertrauen; Textimporte bekommen dadurch keine automatische Prüfung.
Keines dieser Modelle beantwortet ohne definierte Vertrauensbasis die Frage,
ob eine beliebige vom Nutzer gewählte Liste vertrauenswürdig ist.

Der HMAC-Schlüssel wird domänengetrennt per HMAC-PRF aus dem vorhandenen zufälligen
256-Bit-Store-Schlüssel abgeleitet. Der normale Store-Pfad schützt ihn dadurch
mit CurrentUser-DPAPI. Das spart eine zweite Schlüsseldatei und deren atomare
Migration. Explizite Schlüssel über `open_with_key` bleiben Aufruferverantwortung.
SQLCipher und Manifest werden in einer Transaktion aktualisiert; Prüfung und
Laden verwenden denselben Snapshot. Fehlend und verändert sind Warnzustände und
blockieren Metadatenanzeige, Laden und Import. Neue Stores nutzen Schema 2 und
werden von Schema-1-Binaries abgelehnt. Nur echte Schema-1-Stores ohne
Manifesttabelle erhalten `ManifestMissing` und können nach ausdrücklicher Prüfung
migriert werden. Ein entferntes Schema-2-Manifest ist `Changed` und kann nicht
per Baseline-Annahme akzeptiert werden. Migration vertraut keinem Altbestand automatisch.
Details zum Format und zur ausdrücklichen Baseline-Annahme stehen im
[Store-README](../../crates/kairos-store/README.md).

Ein HMAC ist keine Herausgebersignatur. Angreifer im Nutzerkontext können DPAPI
nutzen, den Prozess ändern oder gültige HMACs erzeugen. SQLCipher schützt bereits
Seiten; der Manifest-HMAC ergänzt logische Bestandsprüfung und Warnzustände,
keine neue Grenze gegen denselben Nutzer. Ein gemeinsam zurückgespielter gültiger
DB-/Schlüsselstand besteht die Prüfung. Revisionen oder lokale Zusatzdateien
allein lösen diesen Replay nicht. Echter Anti-Rollback benötigt einen externen
oder anderweitig nicht zurücksetzbaren Vertrauensanker. Vollständig entfernte
Store-Dateien sind ebenfalls nicht von einer Neuinstallation unterscheidbar.

Optional vergleicht `verify_original_sources()` die unveränderten Importbytes
mit dem Quellhash. Gelöscht, verschoben oder unlesbar wird als Hinweis getrennt
gemeldet. Kein automatischer Neuimport. Nur authentisierte Pfade werden gelesen;
die App muss mögliche Netzwerk-/Wechselmedienzugriffe bewusst auslösen.

## Dump-Test und Beweisgrenzen

Der ignorierte Release-Test erzeugt einen Laufzeit-Testwert als UTF-8 und
UTF-16LE direkt in LockedSecret. Er erzeugt drei FullMemory-Selbst-Dumps: mit den
lebenden Werten, nach `clear()` bei weiterhin belegten und gesperrten Seiten und
nach `close()` zur Suche nach Restkopien außerhalb der freigegebenen Allokationen.
Nur der zweite Dump prüft das Löschen unabhängig vom späteren Unmapping. In beiden
späteren Dumps bleiben getrennte lebende Positivkontrollen beider Kodierungen.
Der Elternprozess baut Suchmuster erst nach Ende des Kindes auf,
scannt mit überlappenden Fenstern und entfernt seine temporären Dumps.
Keine echten Passwörter, keine Literal-Suchwerte im Zielprozess und keine
Dump-Artefakte in CI. Der manuelle Befehl steht im Hardening-README.

Fehlende Treffer beweisen keine vollständige Löschung. Gesucht werden bekannte
zusammenhängende Bytes in erfassten Regionen zu einem Zeitpunkt. Temporäre
Stackkopien, andere Darstellungen und weitere Prozesse können unberücksichtigt
bleiben. Der Test misst keine UI, kein Undo, keine Zwischenablage und keine
Generatorintegration. VirtualLock verhindert weder Dump noch Debuggerlesen.
WER/DACL begrenzen bestimmte Zugriffswege; Adminrechte, bestehende Handles,
Kernelcode, Ruhezustand und in-process-Malware bleiben Grenzen.

**Die Prozess-DACL schützt nicht gegen Prozesse mit aktiviertem SeDebugPrivilege
oder gegen Administratoren, die es aktivieren können.** `Applied` im Report
bestätigt die Konfiguration und keine Grenze gegen diesen Angreifer. Der
Prozesstest protokolliert Elevation und Privilegstatus, prüft den Bypass bei
verfügbarem Privileg und deaktiviert es dann im isolierten Kindprozess.
Anschließend müssen alle zehn gefährlichen Rechte einschließlich der
Owner-Rechte verweigert werden. Elevation führt zu keinem pauschalen Test-Skip.

## Offene Integration und Folge-PR

Die App muss den Report und Integritätswarnungen sichtbar auswerten, Migration
bewusst anbieten und LockedSecret-Lebensdauer samt UI-/Clipboardkopien prüfen.
Das geschieht später. PR-9b übernimmt nach PR #7 die gesonderte Bewertung und
Integration von obfstr/Wortlistenschutz. Hier bleiben `apps`, `kairos-core`,
`kairos-platform`, `kairos-keepassxc` und `kairos-wordlists` unverändert.
