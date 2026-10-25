# Verschlüsselte externe Wortlisten

Zuständig: **nur PR-5** (`feat/encrypted-store`). Abrufdatum aller unten
bewerteten Quellen: **25.10.2026**. Übernommen werden Prinzipien; fremder
Implementierungscode und fremde Wortlistendaten werden nicht kopiert.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| SQLCipher | [v4.19.0](https://github.com/sqlcipher/sqlcipher/tree/v4.19.0) | BSD-3-Clause; SQLite Public Domain | `src/crypto.c`, `src/sqlcipher.h` | Verschlüsselung und Seitenauthentisierung vollständig an den etablierten Codec delegieren; Schlüssel vor dem ersten Seitenzugriff setzen | 5 |
| rusqlite | [v0.41.0](https://github.com/rusqlite/rusqlite/tree/v0.41.0), [Build](https://github.com/rusqlite/rusqlite/blob/v0.41.0/libsqlite3-sys/build.rs) | MIT | `Connection`, `Transaction`, `libsqlite3-sys/build.rs` | Parametrisierte SQL-Abfragen und atomare Transaktionen; Codec und OpenSSL als gebündelte Abhängigkeiten statt eigener Datenbank-FFI | 5 |
| SQLite3 Multiple Ciphers | [v2.6.1](https://github.com/utelle/SQLite3MultipleCiphers/tree/v2.6.1), [Rust-Quellpaket 205.1.0](https://docs.rs/sqlite3mc-src/205.1.0%2Bsqlite3mc-2.5.1-sqlite-3.53.4/sqlite3mc_src/) | MIT; SQLite Public Domain, einzelne Komponenten CC0/Apache-2.0/Unlicense | Amalgamation, `sqlite3mc_src::source_dir()` | Eine eingebettete Cipher-Engine könnte OpenSSL vermeiden; das Quellpaket allein ersetzt aber keine sichere native Rust-API | 2 |
| WalletKit SQLite | [0.28.0 / 301fa6d9](https://github.com/worldcoin/walletkit/tree/301fa6d9758703c1e621ca957b82ff7f17645f01/crates/walletkit-sqlite) | MIT; SQLite3MC-Komponenten separat | `build.rs`, `Connection`, `Statement` | Ein sicherer SQLite3MC-Wrapper existiert; dessen Build lädt aber separat C-Quellen mit curl und verlangt Rust 1.98.1 | 3 |
| libSQL | [0.9.30 / 0653c578](https://github.com/tursodatabase/libsql/tree/0653c5788d77ef16a97c56ff3e9fdc11717a72d9) | MIT; Komponenten separat | `EncryptionConfig`, `libsql-ffi/build.rs` | Konfigurierbare Verschlüsselung direkt in der Rust-Datenbank-API; zusätzliche CMake-/Engine-Schicht gegen den Nutzen abwägen | 2 |
| OpenSSL | [openssl-3.6.3](https://github.com/openssl/openssl/tree/openssl-3.6.3) | Apache-2.0 | `NOTES-WINDOWS.md`, `NOTES-PERL.md` | Kryptografie als gepflegte Bibliothek bauen; vollständiges natives Perl und MSVC-Tools als explizite Voraussetzungen behandeln | 5 |
| Strawberry Perl | [SP_54221_64bit](https://github.com/StrawberryPerl/Perl-Dist-Strawberry/releases/tag/SP_54221_64bit) | Perl Artistic-1.0-Perl OR GPL-1.0-or-later; Distribution enthält separat lizenzierte Werkzeuge | Portable ZIP, `perl/bin/perl.exe` | Vollständiges Buildwerkzeug lokal entpacken, vorher offiziellen SHA-256 prüfen; keine Systeminstallation und keine Auslieferung mit Kairos | 4 |
| Chromium os_crypt | [130.0.6723.58](https://github.com/chromium/chromium/blob/130.0.6723.58/components/os_crypt/sync/os_crypt_win.cc) | BSD-3-Clause | `OSCryptImpl::Init`, DPAPI-Helfer | Einen zufälligen Datenschlüssel separat per DPAPI schützen; Entschlüsselungsfehler ernst nehmen | 5 |
| windows-dpapi | [v0.2.0 / 4ac261cc](https://github.com/sheridans/windows-dpapi/blob/4ac261cc789ab046dd8bac3e914f49b26b8b7d77/src/lib.rs) | MIT OR Apache-2.0 | `encrypt_data`, `decrypt_data` | Dünne CurrentUser-Fassade spart Aufrufcode; der native Ausgabepuffer muss jedoch vor `LocalFree` gelöscht werden | 3 |
| keyring-rs | [v4.2.0](https://github.com/open-source-cooperative/keyring-rs/tree/v4.2.0) | MIT OR Apache-2.0 | `Entry::set_secret`, `Entry::get_secret` | Betriebssystemeigenen Secret Store nutzen, wenn kein eigener DPAPI-Dateivertrag erforderlich ist | 3 |
| KeePassXC | [2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp) | GPL-2.0-or-later, Dateiheader beachten | `PassphraseGenerator::setWordList` | Vor der Verwendung trimmen und deduplizieren; einen stabilen Wortbestand bereitstellen | 4 |
| KeePassXC Quick Unlock | [2.8.0-beta1](https://github.com/keepassxreboot/keepassxc/blob/2.8.0-beta1/src/quickunlock/WindowsHello.cpp) | GPL-2.0-or-later | `WindowsHello::setKey`, `getKey` | Ein Benutzerfreigabe-Gate kann entsperrte Schlüssel begrenzen; anderer Vertrag als automatischer Importzugriff | 2 |
| Bitwarden Desktop | [desktop-v2026.9.1](https://github.com/bitwarden/clients/blob/desktop-v2026.9.1/apps/desktop/desktop_native/secure_memory/src/dpapi.rs) | GPL-3.0 | `CryptProtectMemory` / `CryptUnprotectMemory` | Laufzeitschutz von Speicher und dauerhafte Schlüsselablage getrennt beurteilen | 2 |
| libsodium | [Speicherfunktionen](https://github.com/jedisct1/libsodium/blob/60b751cd14418fae2d17e6f1003e43622b3e5a25/src/libsodium/sodium/utils.c) | ISC | `src/libsodium/sodium/utils.c` | Schlüsselpuffer kurz halten, Sperr-/Löschfehler und Kopien berücksichtigen | 4 |
| zeroize | [zeroize-v1.8.2](https://github.com/RustCrypto/utils/tree/zeroize-v1.8.2/zeroize) | Apache-2.0 OR MIT | `Zeroizing`, `Zeroize` | Eigene Rust-Schlüsselpuffer beim Freigeben überschreiben, fremde native Puffer zusätzlich behandeln | 5 |
| rand | [0.9.5](https://github.com/rust-random/rand/tree/0.9.5/src/seq) | MIT OR Apache-2.0 | `SliceRandom::shuffle`, `StdRng::try_from_rng`, `OsRng` | Bewährte gleichverteilte Shuffle-Implementierung mit Betriebssystem-Zufallsseed verwenden; Zufallsfehler abbrechen | 5 |
| xkcdpass | [9db43954 / xkcdpass-1.30.0](https://github.com/redacted/XKCD-password-generator/blob/9db439542d9a156f8a9db56323b459670175c132/xkcdpass/xkcd_password.py) | BSD-3-Clause; Wortlisten separat | `generate_wordlist` | UTF-8-Zeilen, Zeichensatz-/Längenfilter und Mengenbildung gegen doppelte Wörter | 5 |
| diceware (Uli Fouquet) | [98fa09af / v1.0.1](https://github.com/ulif/diceware/blob/98fa09afd7ab28b6075d771b2c44b0e00479e72f/diceware/wordlist.py) | GPL-3.0-or-later; Wortlisten separat | `WordList.refine_entry` | Nummernpräfix und Wort getrennt lesen; die Würfelziffern in Kairos zusätzlich streng validieren | 4 |
| Password Tech | [c77cade1 / v3.6.1](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/passw/PasswGen.cpp) | GPL-2.0; Wortlisten separat | `PasswordGenerator::LoadWordListFile`, `StringFileStreamW` | Kodierung/BOM, Längenfilter, Kleinschreibung und Duplikate als bewusste Importentscheidungen behandeln | 4 |

## Backendentscheidung und Build-Beleg

Gewählt ist **rusqlite 0.41.0 mit `bundled-sqlcipher-vendored-openssl`**.
`Cargo.lock` fixiert `libsqlite3-sys 0.39.1`, SQLCipher **4.19.0** über
`sqlcipher-src 419.0.6` und OpenSSL **3.6.3**. Die erste Recherche betrachtete
rusqlite 0.40.2 mit SQLCipher 4.14.0; die anschließende Stichprobe der tatsächlichen
0.41.0-Abhängigkeiten belegt den neueren Bundle-Stand. Kein eigenes `build.rs`,
keine generierten eigenen Bindings und keine Kopie eines SQLite-Forks sind nötig.
Nur der binäre Aufruf von `sqlite3_key` nutzt die von rusqlite exportierten
Bindings, um Klartext-Schlüssel in SQL-Strings zu vermeiden.

SQLite3MC wäre ohne OpenSSL attraktiv. Das geprüfte Rust-Quellpaket liefert
jedoch nur C-Dateien; eine eigene native Adapter-/Buildschicht würde die
Eigenarbeit vergrößern. SQLCipher mit anderem Crypto-Backend hätte denselben
Nachteil gegenüber dem von rusqlite unterstützten Feature. libSQL bringt eine
größere API samt CMake-Build mit; für 0.9.30 dokumentiert der
[Windows-Verschlüsselungsfix](https://github.com/tursodatabase/libsql/pull/2265)
zusätzlich ein konkretes Buildproblem. Diese Alternativen werden nicht eingebaut.

Eine zusätzliche lokale Quellprüfung von `walletkit-sqlite 0.28.0` bestätigt einen
sicheren High-Level-Wrapper für SQLite3MC. Er verwendet eine eigene schmalere
Abfrage-/Wert-API und lädt im Build per externem `curl` eine separat
prüfsummengebundene SQLite3MC-2.3.2-/SQLite-3.51.3-Amalgamation nach.
Sein Engine-Stand liegt hinter dem geprüften SQLCipher-Bundle. Das spart OpenSSL,
vergrößert aber gegenüber rusqlite die Wartungs- und
Buildabhängigkeit von einem spezialisierten Wrapper samt zusätzlichem
Release-Download. Für diesen PR wird deshalb die etablierte rusqlite-API mit
dem lokal nachgewiesenen Standard-MSVC-Bauweg bevorzugt.

rusqlite setzt Rust **mindestens 1.89** voraus; die Crate deklariert diese MSRV.
Der lokale Prüflauf verwendet Rust 1.99.1. WalletKit verlangt 1.98.1; beide
Varianten passen zur geprüften Umgebung. Ausschlaggebend gegen WalletKit sind
Wrapper-API, zusätzlicher Download und Engine-Stand. Der historische
Rust-1.80-Kommentar im Root-Manifest gehört nicht zum Änderungsumfang dieses PRs.

Windows-Builds benötigen vollständiges natives Perl; Git-Perl ist unzureichend.
Portables Strawberry Perl **5.42.2.1** kann unter einem lokalen Buildpfad
entpackt werden. SHA-256 des offiziellen ZIP-Downloads:
`32d83be90cf04b807cfb9477482bc36302cdee6f5b04cf57e81adecbd8f07898`.
Der Hash ist vor dem Entpacken zu prüfen; `perl -MLocale::Maketext::Simple`
prüft das für OpenSSL benötigte Modul. Die
[offiziellen Release-Metadaten](https://github.com/StrawberryPerl/strawberryperl.com/blob/gh-pages/releases.json)
veröffentlichen Download und Prüfsumme. Der abschließende Build-/Testbeleg steht
im PR. Die CI wählt das vorhandene Strawberry Perl explizit und prüft dessen
Modul; NASM ist optional. Maschinenlokale SDK-/Perl-Ergänzungen sind im PR als
Prüfbeleg dokumentiert; sie sind keine zusätzliche Voraussetzung des Crates.

Der vollständige lokale MSVC-Bauweg ist anschließend belegt: `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo test --workspace` jeweils Exitcode **0**, insgesamt **20 bestandene Tests**
(19 Unit-/Integrationstests und ein kompiliertes Rustdoc-Beispiel).
Beim Test-Linken meldet OpenSSL fehlende native PDB-Debugsymbole (`LNK4099`),
ohne den erfolgreichen Lauf zu verhindern. Die negativen Schlüsseltests erzeugen
erwartete SQLCipher-HMAC-Fehlermeldungen ohne Schlüssel- oder Wortinhalte.

Die direkten Laufzeitabhängigkeiten sind MIT/Apache-2.0 beziehungsweise MIT.
SQLCipher ist BSD-3-Clause, OpenSSL Apache-2.0 und SQLite Public Domain.
Das transitive Quellpaket `sqlcipher-src` nennt zusätzlich `blessing` und WTFPL
für mitgelieferte SQLite-/LibTomCrypt-Quellen; der native Build kompiliert gemäß
`libsqlite3-sys/build.rs` nur die SQLCipher-Amalgamation mit OpenSSL, **kein
LibTomCrypt**. Dessen enthaltene Quellen sind laut
[Paketbeschreibung](https://docs.rs/crate/sqlcipher-src/419.0.6%2Bsqlcipher-4.19.0-sqlite-3.53.4-libtomcrypt-1.18.2)
Public Domain oder WTFPL. Es werden keine fremden Wortlisten weitergegeben.

## Übernahme und Grenzen

- **SQLCipher/rusqlite:** Codec, Seiten-HMAC, parametrisierte Abfragen und
  Transaktionen werden als Bibliotheken genutzt. Keine selbst entworfene
  Dateiverschlüsselung. Temp-Tabellen bleiben im RAM; der Test liest ein echtes
  Rollback-Journal und sucht darin die bekannten Wörter.
  Der mit `Zeroizing` geschützte Raw-Key-Übergabepuffer vermeidet die unnötige
  Passphrasen-KDF. `cipher_memory_security` wird vor der Schlüsselübergabe
  aktiviert und verifiziert. Lade-Deduplizierung erfolgt direkt in SQL.
- **Chromium:** Separater Zufallsschlüssel plus DPAPI wird als Architektur
  übernommen. Browser-Preferences, eigene AES-Schicht und automatisches Ersetzen
  nicht entschlüsselbarer Schlüssel werden nicht übernommen.
- **Windows-DPAPI:** Offizielle `windows-sys`-Bindings mit CurrentUser und
  `CRYPTPROTECT_UI_FORBIDDEN`. Der kleine Adapter löscht die Windows-Ausgabe vor
  `LocalFree`. `windows-dpapi` 0.2.0 erledigt das laut geprüftem Quelltext nicht,
  deshalb wird es trotz kleinerer Aufruf-API nicht verwendet. Credential Manager
  passt nicht zum geforderten DPAPI-Blob unter `%APPDATA%\Kairos`.
- **KeePassXC/Bitwarden:** Stabile deduplizierte Wortquelle wird übernommen.
  Windows Hello und RAM-Verschlüsselung werden hier nicht implementiert; sie
  lösen andere Freigabe-/Laufzeitanforderungen. Weitergehende Speicherhärtung
  bleibt PR-9. DPAPI ist keine Grenze gegen Schadcode desselben Benutzers.
- **libsodium/zeroize:** Explizite Löschung eigener Schlüsselkopien wird
  übernommen; `zeroize = "=1.8.2"` bleibt unverändert. Native SQLCipher-Schlüssel
  verwaltet der Codec. Parser-Mengen, Betriebssystem und Abhängigkeiten können
  weitere Klartextkopien von Wortlistendaten halten; vollständige RAM-Bereinigung
  wird nicht behauptet.
  Der eigene Schlüssel liegt in `Box<Zeroizing<[u8; 32]>>`; Besitzerverschiebungen
  verschieben dessen Bytes nicht. Frühere externe Aufruferkopien bleiben PR-9-
  beziehungsweise Aufruferverantwortung.
- **xkcdpass/diceware/Password Tech:** Zeilenformat, Filter und Deduplizierung
  werden modelliert. Kairos ergänzt strenge 4–6 Würfelziffern aus `1..6`, UTF-8-BOM,
  CRLF und explizite Ablehnungszähler. Kein PGP-Hüllenparser, UTF-16-/ANSI-Fallback,
  CSV oder stilles Zerlegen beliebiger Mehrwortzeilen. Standard ist `a..z`,
  Kleinschreibung, Länge 3–9; Umlaute/ß werden nicht transliteriert. Die Regeln
  sind konfigurierbar und im [Crate-README](../../crates/kairos-store/README.md)
  vollständig beschrieben.
  Die Eingabereihenfolge bleibt bis zum Shuffle erhalten. Großbuchstaben in der
  erlaubten Zeichenmenge sind ungültige Regeln; `Other("de"/"en"/"la")` wird auf
  die entsprechende eingebaute Sprache normalisiert.
- **rand:** `OsRng` erzeugt den 256-Bit-Schlüssel und initialisiert fallibel einen
  `StdRng` für das unveränderte Bibliotheks-Shuffle. Keine alphabetischen
  Wortindizes und kein Entropiebonus aus der Speicherreihenfolge.

`rand` ist wie in PR-1 auf **=0.9.5** mit `os_rng`/`unbiased` festgelegt; `std_rng`
liefert zusätzlich den aus dem Betriebssystem initialisierten Shuffle-Generator.
Es gibt keine zweite rand-Version für den Store.

Implementierung: `crates/kairos-store/src/{lib,import,key}.rs`; Nachweise stehen
in rustdoc sowie `tests/store.rs`. Listenname, Importzeit und SHA-256 beziehen
sich auf denselben einmal eingelesenen Quellpuffer. Dateiimporte speichern den
kanonischen UTF-8-Quellpfad verschlüsselt mit; In-Memory-Importe haben keinen
Pfad. Der spätere Vergleich mit extern veränderten Dateien ist noch nicht
implementiert. Die Vorbilder wurden anhand ihrer offenen Quellmodule bewertet.
Reverse Engineer Anything wurde gemäß seiner Abgrenzung für vollständige
Quellrepos nicht als Binär-/Runtime-Werkzeug ausgeführt; es gab kein zu
rekonstruierendes ausgeliefertes Artefakt.

Primäre Windows-API-Verträge:
[`CryptProtectData`](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata),
[`CryptUnprotectData`](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptunprotectdata).
Diese API-Dokumentation ergänzt die fest versionierten Projektvorbilder.
