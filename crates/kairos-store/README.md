# kairos-store

Externe Wortlisten werden einmal eingelesen, geprüft und in einer SQLCipher-Datei
gespeichert. `StoredWordList` liefert einen stabilen Snapshot als
`kairos_core::WordSource`. Auswahl ist nach Listen-ID, Sprache oder beidem möglich.
Bei überlappenden Listen erscheint jedes Wort einmal. Ohne Sprachfilter gewinnt
das zuerst gespeicherte Vorkommen mit seinem Sprach-Flag. Ein unbekannter Filter
liefert eine leere Quelle; Core behandelt leere Quellen gemäß API-Vertrag.

```rust,no_run
use kairos_core::Language;
use kairos_store::{EncryptedStore, ImportRules, StorePaths};
use std::path::Path;

# fn main() -> Result<(), kairos_store::StoreError> {
let mut store = EncryptedStore::open(&StorePaths::for_current_user()?)?;
let imported = store.import_file(
    Path::new("meine-woerter.txt"),
    "Meine deutsche Liste",
    Language::German,
    &ImportRules::default(),
)?;
let source = store.load(Some(imported.list.id), Some(&Language::German))?;
# Ok(())
# }
```

## Importregeln

Eingabe ist UTF-8 mit optionalem führendem BOM, LF oder CRLF. Pro Zeile wird ein
Wort oder ein Diceware-Code mit 4–6 Ziffern aus `1..6`, Leerraum und genau einem
Wort akzeptiert. Führender und abschließender Leerraum wird entfernt. Leere Zeilen
und mit `#` beginnende Kommentarzeilen werden separat gezählt. Inline-Kommentare,
CSV, Mehrwortzeilen und zusätzliche Spalten werden abgewiesen.

Nach Unicode-Kleinschreibung gelten standardmäßig ausschließlich `a..z` und
3–9 Unicode-Zeichen. `ä`, `ö`, `ü`, `ß`, Ziffern und Satzzeichen werden verworfen,
nicht umgeschrieben. `ImportRules` erlaubt andere Zeichensätze und Längen.
Es gibt keine Unicode-Normalisierung oder sprachabhängige Transliteration.
Duplikate nach Kleinschreibung werden innerhalb jedes Imports entfernt.

`ImportReport` zählt jede Zeile genau einmal. Priorität der Ablehnungsgründe ist
leer, Kommentar, Format, Zeichensatz, zu kurz, zu lang, Duplikat. Ungültiges UTF-8
oder ungültige Regeln brechen den gesamten Import ab. Ohne gültige Wörter liefert
`EmptyImport` den Bericht; die Datenbank bleibt unverändert. Erfolgreiche Importe
legen immer eine neue Liste an, auch bei identischem Namen oder Quellhash.

Listen enthalten Name, Importzeit als Unix-Sekunden, Wortanzahl und SHA-256 der
unveränderten Quelldatei einschließlich BOM und Zeilenenden. Dieser Hash dient
PR-9 als Grundlage; PR-5 implementiert keinen Vergleich mit einer später
veränderten Quelldatei.

## Verschlüsselung und Schlüssel

`rusqlite` bündelt SQLCipher und OpenSSL statisch. Es gibt keine eigene
Kryptografie, keine externe DLL-Suche und keinen Klartext-Ersatzpfad.
SQLCipher verwendet seine Standardparameter mit AES-256 und Seiten-HMAC.
Der zufällige 256-Bit-Schlüssel wird mit `OsRng` erzeugt und über `sqlite3_key`
binär mit expliziter Länge übergeben. SQLCipher leitet daraus seine Arbeitsschlüssel
ab; ein eingebettetes NUL schneidet den Schlüssel nicht ab.

Unter Windows liegen standardmäßig `words.db` und `database-key.dpapi` in
`%APPDATA%\Kairos\`. DPAPI verwendet CurrentUser und unterdrückt Dialoge.
Die verschlüsselte Schlüsseldatei wird mit `tempfile::persist_noclobber` erst nach
vollständigem Schreiben veröffentlicht und nie überschrieben. Verzeichnisse
erben die Zugriffsrechte ihres Elternverzeichnisses. Ein verlorener oder korrupter
Schlüssel wird bei bestehender Datenbank nicht neu erzeugt.

Eigene Schlüsselpuffer nutzen `Zeroizing`; die native DPAPI-Ausgabe wird vor
`LocalFree` überschrieben. SQLCipher erhält `cipher_memory_security = ON`.
Temporäre Tabellen bleiben im RAM, Rollback-Journale werden vom Codec
verschlüsselt. Alle Wörter eines Imports erhalten erst nach `rand::shuffle`
ihre Zeilen-IDs. Dessen CSPRNG wird fallibel aus `OsRng` initialisiert.
Die Reihenfolge ist kein zusätzlicher Entropiebonus für Passwörter.

`StorePaths` erlaubt isolierte Pfade. `open_with_key` unterstützt explizit extern
verwaltete Schlüssel und plattformunabhängige Datenbanktests, speichert sie aber
nicht. Die verwaltete Schlüsselablage bricht außerhalb Windows ab.

DPAPI schützt vor dem Zugriff eines anderen Benutzerkontexts, nicht vor Code,
der bereits als derselbe Benutzer läuft. Importdateien bleiben beim Aufrufer im
Klartext. Parser und Deduplizierung benötigen vorübergehende Klartextwörter;
`StoredWordList` löscht seine eigenen Strings beim Freigeben. Vollständige
RAM-/Dump-/Auslagerungshärtung gehört zu PR-9. Keine Passwörter werden gespeichert.

## Build und Prüfung

Windows/MSVC benötigt C-Buildtools, Windows SDK, `nmake` und vollständiges natives
Perl für vendored OpenSSL. Git-Perl genügt nicht. NASM ist optional; OpenSSL wählt
ohne NASM `no-asm`. Die CI prüft das auf `windows-latest` vorhandene Strawberry
Perl ausdrücklich. Es erfolgt keine Systeminstallation.

Für die lokale Umgebung zuerst den im Arbeitsauftrag genannten
`Enter-KairosBuildEnv.ps1` laden. Danach das lokal entpackte Perl nur für diese
Sitzung voranstellen:

```powershell
$kairosPerlBin = Join-Path $env:LOCALAPPDATA 'kairos-build-tools\strawberry-perl-5.42.2.1\perl\bin'
$env:OPENSSL_SRC_PERL = Join-Path $kairosPerlBin 'perl.exe'
$env:PATH = "$kairosPerlBin;$env:PATH"
perl -MLocale::Maketext::Simple -e 'print "$^V OK\n"'
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Portable Strawberry Perl 5.42.2.1 wurde über das
[offizielle Release](https://github.com/StrawberryPerl/Perl-Dist-Strawberry/releases/tag/SP_54221_64bit)
bezogen. Vor dem Entpacken wird der von der
[Projektseite veröffentlichte SHA-256](https://github.com/StrawberryPerl/strawberryperl.com/blob/gh-pages/releases.json)
gegen `32d83be90cf04b807cfb9477482bc36302cdee6f5b04cf57e81adecbd8f07898` geprüft.
Archive und Buildwerkzeuge gehören ausschließlich unter `target/` oder
`%LOCALAPPDATA%\kairos-build-tools`.

Im vorliegenden Worktree fehlen zusätzlich die Windows-SDK-Header, obwohl
UCRT-Header und Linkbibliotheken vorhanden sind. Dafür wurde das offizielle
[Microsoft.Windows.SDK.CPP 10.0.22000.196](https://www.nuget.org/packages/Microsoft.Windows.SDK.CPP/10.0.22000.196)
nach Prüfung des SHA-512 aus dem NuGet-Katalog unter `target/windows-sdk-cpp`
entpackt. Nur für diese lokale Umgebung enthält `INCLUDE` dessen Verzeichnisse
`c/Include/10.0.22000.0/um` und `shared` sowie die vorhandenen MSVC-/UCRT-Includes.
Ein regulärer Windows-SDK-Build und `windows-latest` benötigen diesen Zusatz nicht.

Integrationstests verwenden Temp-Verzeichnisse und echte verschlüsselte Dateien.
Sie prüfen Roundtrip, Herkunftsmetadaten, Sprach-/Listenfilter, Duplikate,
konfigurierbare Regeln, UTF-8/BOM/CRLF, falsche Schlüssel, NUL im Schlüssel,
Ablehnung von Klartextdatenbanken, stabile randomisierte Speicherreihenfolge und
unter Windows DPAPI, beschädigte/fehlende Schlüssel sowie konkurrierende Anlage.

## Open-Source-Vorbilder (modellieren, nicht kopieren)

[SQLCipher](https://github.com/sqlcipher/sqlcipher/tree/v4.19.0) und
[rusqlite](https://github.com/rusqlite/rusqlite/tree/v0.41.0) liefern Verschlüsselung
und Transaktionen als Abhängigkeit.
[Chromium os_crypt](https://github.com/chromium/chromium/blob/130.0.6723.58/components/os_crypt/sync/os_crypt_win.cc)
begründet den separaten DPAPI-geschützten Zufallsschlüssel.
[xkcdpass](https://github.com/redacted/XKCD-password-generator/blob/9db439542d9a156f8a9db56323b459670175c132/xkcdpass/xkcd_password.py)
und [diceware](https://github.com/ulif/diceware/blob/98fa09afd7ab28b6075d771b2c44b0e00479e72f/diceware/wordlist.py)
begründen Textformate, Längenfilter und Deduplizierung.
[KeePassXC](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp)
begründet stabile Wortquellen;
[libsodium](https://github.com/jedisct1/libsodium/blob/60b751cd14418fae2d17e6f1003e43622b3e5a25/src/libsodium/sodium/utils.c)
die explizite Pufferlöschung. [rand](https://github.com/rust-random/rand/blob/0.8.5/src/seq/mod.rs)
liefert das Shuffle und den Systemzufall.
Es wurde kein fremder Implementierungscode kopiert. Konkrete Quellstände,
Lizenzen, Alternativen und Übernahmegrenzen stehen in
[store.md](../../docs/sources/store.md).
