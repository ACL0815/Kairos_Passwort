# kairos-store

Externe Wortlisten werden einmal eingelesen, geprüft und in einer SQLCipher-Datei
gespeichert. `StoredWordList` liefert einen stabilen Snapshot als
`kairos_core::WordSource`. Auswahl ist nach Listen-ID, Sprache oder beidem möglich.
Bei überlappenden Listen erscheint jedes Wort einmal. Ohne Sprachfilter gewinnt
das zuerst gespeicherte Vorkommen mit seinem Sprach-Flag. Ein unbekannter Filter
liefert eine leere Quelle; Core behandelt leere Quellen gemäß API-Vertrag.

Ein vollständiges, als Doctest kompiliertes Beispiel steht in der
[Rustdoc des Moduls](src/lib.rs). `cargo doc --open -p kairos-store` erzeugt die
lokale API-Dokumentation.

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
Der Parser erhält die Reihenfolge des ersten Vorkommens bis zum expliziten Shuffle.
Ein erlaubter Zeichensatz mit Großbuchstaben, Leerraum oder Steuerzeichen ist
ungültig. Die Sprach-Tags `de`, `en` und `la` in `Language::Other` werden unabhängig
von der Großschreibung auf die entsprechenden eingebauten Varianten abgebildet.

`ImportReport` zählt jede Zeile genau einmal. Priorität der Ablehnungsgründe ist
leer, Kommentar, Format, Zeichensatz, zu kurz, zu lang, Duplikat. Ungültiges UTF-8
oder ungültige Regeln brechen den gesamten Import ab. Ohne gültige Wörter liefert
`EmptyImport` den Bericht; die Datenbank bleibt unverändert. Erfolgreiche Importe
legen immer eine neue Liste an, auch bei identischem Namen oder Quellhash.

Listen enthalten Name, Importzeit als Unix-Sekunden, Wortanzahl und SHA-256 der
unveränderten Quelldatei einschließlich BOM und Zeilenenden. Dieser Hash dient
der optionalen Prüfung der ursprünglichen Importdatei.
Dateiimporte speichern zusätzlich den kanonischen UTF-8-Quellpfad innerhalb der
verschlüsselten Metadaten. In-Memory-Importe haben keinen Quellpfad.
`verify_original_sources()` prüft die ursprüngliche Datei; verschobene Dateien benötigen dann
eine neue Zuordnung.

## Verschlüsselung und Schlüssel

`rusqlite` bündelt SQLCipher und OpenSSL statisch. Es gibt keine eigene
Kryptografie, keine externe DLL-Suche und keinen Klartext-Ersatzpfad.
SQLCipher verwendet seine Standardparameter mit AES-256 und Seiten-HMAC.
Der zufällige 256-Bit-Schlüssel wird mit `OsRng` erzeugt und über `sqlite3_key`
als Raw-Key-Literal mit expliziter Länge übergeben. Der feste hexadezimale
Übergabepuffer nutzt `Zeroizing` und tritt nicht in SQL-Anweisungen auf. Der
Raw-Key-Modus vermeidet die bei Zufallsschlüsseln unnötige Passphrasen-KDF.
Ein eingebettetes NUL schneidet den Schlüssel nicht ab.

Unter Windows liegen standardmäßig `words.db` und `database-key.dpapi` in
`%APPDATA%\Kairos\`. DPAPI verwendet CurrentUser und unterdrückt Dialoge.
Die verschlüsselte Schlüsseldatei wird mit `tempfile::persist_noclobber` erst nach
vollständigem Schreiben veröffentlicht und nie überschrieben. Verzeichnisse
erben die Zugriffsrechte ihres Elternverzeichnisses. Ein verlorener oder korrupter
Schlüssel wird bei bestehender Datenbank nicht neu erzeugt.

Eigene Schlüsselpuffer nutzen `Zeroizing`; ein `Box` hält den Schlüssel beim
Verschieben des Besitzers am selben Heap-Ort. Die native DPAPI-Ausgabe wird vor
`LocalFree` überschrieben. SQLCipher erhält bereits vor der Schlüsselübergabe
`cipher_memory_security = ON`, dessen Status anschließend geprüft wird.
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
RAM-/Dump-/Auslagerungshärtung liefert ergänzend `kairos-hardening` in PR-9a;
die App-Integration folgt separat. Keine Passwörter werden gespeichert.
Bei extern bereitgestellten Schlüsseln bleiben frühere Kopien des Aufrufers
einschließlich möglicher Stack-Kopien in dessen Verantwortung. Deduplizierung beim
Laden erfolgt in SQL und erzeugt keine zusätzliche Rust-Kopie jedes Wortes.
`AuthenticationFailed` bezeichnet einen falschen Schlüssel oder eine beschädigte
Datei; kryptografisch lassen sich diese beiden Ursachen nicht sicher trennen.

## Manipulationserkennung beim Start (PR-9a, Prio 5)

Die App soll unmittelbar nach `open` `verify_integrity()` auswerten und
`Changed` oder `ManifestMissing` als Warnung anzeigen. `load` und Import prüfen
zusätzlich selbst und liefern bei Warnungen `StoreError::IntegrityWarning`.
So können Aufrufer einen vergessenen Startup-Check nicht zum stillen Laden nutzen.
Datenbank-/SQLCipher-Fehler sind ebenfalls Fehler und kein erfolgreicher Check.
Die UI und der tatsächliche Startup-Aufruf sind Teil des späteren Integrations-PR.

Das Manifest liegt in `integrity_manifest` innerhalb der verschlüsselten DB.
Ein neuer leerer Store erhält einen authentisierten leeren Ausgangsstand.
Ältere Schema-1-Datenbanken erhalten nur die zusätzliche Tabelle, keinen
automatischen Vertrauensnachweis. Ein fehlender Datensatz bleibt auch nach
erneutem Öffnen `ManifestMissing`. `accept_current_integrity()` setzt ausdrücklich
den aktuellen Inhalt als vertrauenswürdig voraus. Diese Funktion ist nur für
bewusst bestätigte Migration/Wiederherstellung gedacht, niemals als automatische
Reaktion auf eine Warnung. Ein Neuimport geprüfter Originale in einen neuen Store
ist die bevorzugte Wiederherstellung.

Das versionierte Binärformat bindet Listenanzahl, Gesamtwortanzahl und je Liste
ID, Name, Importzeit, bytegenauen Quell-SHA-256, optionalen Quellpfad, Wortanzahl
und SHA-256 der gespeicherten Wortzeilen. Die Wort-Hashes binden Zeilen-ID,
Wortbytes und Sprache in stabiler ID-Reihenfolge. Diese Entscheidung bindet auch
die bestehende Reihenfolge und die Sprachwahl bei Überschneidungen. Sie ändert
das Import-Shuffle nicht. Bytefelder haben eine u64-Längenpräfixkodierung in
Little Endian, IDs/Zeitstempel sind i64 Little Endian. Pfadabwesenheit erhält ein
zusätzliches Optionsbyte. Die Manifestversion ist Teil der authentisierten
Domänenkennung. Importregeln werden bisher nicht gespeichert; geprüft wird der
tatsächliche normalisierte Bestand, keine rückwirkende Rekonstruktion des Imports.

RustCrypto `hmac`/`sha2` authentisieren das vollständige Manifest mit HMAC-SHA-256
und prüfen den Tag zeitkonstant. Der Manifest-Schlüssel wird mit einer eigenen
festen Domänenkennung als HMAC-PRF aus dem vorhandenen gleichverteilten 256-Bit-
Datenbankschlüssel abgeleitet. Im normalen `open`-Pfad ist damit auch dieser
Schlüssel durch CurrentUser-DPAPI geschützt. Die getrennte Domäne vermeidet
direkte Wiederverwendung des SQLCipher-Raw-Keys und eine zweite Schlüsseldatei
mit eigenem Lebenszyklus. `open_with_key` überträgt die Verantwortung für den
Schlüsselschutz ausdrücklich auf den Aufrufer. Der abgeleitete Schlüssel liegt
in einem festen `Box<Zeroizing<[u8; 32]>>`; kurzfristige Kopien in kryptografischen
Kontexten und nativen Bibliotheken können damit nicht vollständig gelöscht werden.

Importe prüfen den vorherigen Zustand, speichern Listen/Wörter und aktualisieren
das Manifest innerhalb einer IMMEDIATE-Transaktion. Fehler rollen alles zurück.
Verifikation und Laden lesen Manifest und Wörter aus derselben DB-Sicht.
Es gibt keinen separaten Manifestdatei-Rename und damit keinen Zwei-Dateien-
Commit. SHA-256 allein reicht nicht, wenn der Angreifer die Referenz ändern kann.

Modelled after: KeePassXC KDBX4 für authentisierte Metadaten und reihenfolgegebundene
Inhalte; TUF und Paket-Manifeste für Identität, Länge und Hash je Objekt;
SQLCipher für transaktionalen Seitenschutz. Kein Vorbildcode wurde kopiert.
Alle Kandidaten, Lizenzen und Bewertungen stehen in
[protection.md](../../docs/sources/protection.md).

Grenzen: Ein Angreifer mit Nutzerrechten kann DPAPI benutzen oder den laufenden
Prozess manipulieren und damit gültige Manifeste erzeugen. HMAC beweist keine
Herausgeberidentität und keine Qualität/Entropie einer importierten Liste.
SQLCipher authentisiert bereits Seiten; das Manifest ergänzt die logische
Bestandsprüfung und schafft keine neue Grenze gegen denselben Benutzer.
Ein alter, gemeinsam gültiger Stand von DB und Schlüssel besteht die Prüfung.
Ohne externen monotonen Vertrauensanker gibt es keinen Rollback-/Freeze-Schutz.
Eine vollständige Löschung des Stores kann wie ein neuer Store aussehen.

Die optionale Originaldateiprüfung ist davon getrennt. Geänderte Dateien liefern
`SourceStatus::Changed`, gelöschte oder unlesbare `Unavailable`, In-Memory-Importe
`NoSourcePath`. Sie importiert nie automatisch und ändert die DB nicht. Sie folgt
nur zuvor authentisierten Pfaden; die App soll sie wegen möglicher Netzwerk-/
Wechselmedienzugriffe ausschließlich bewusst für vertraute Pfade starten.

## Build und Prüfung

Die Crate setzt Rust **1.89 oder neuer** voraus; der dokumentierte lokale Lauf
verwendet Rust 1.99.1. Windows/MSVC benötigt C-Buildtools, Windows SDK, `nmake` und vollständiges natives
Perl für vendored OpenSSL. Git-Perl genügt nicht. NASM ist optional; OpenSSL wählt
ohne NASM `no-asm`. Die CI prüft das auf `windows-latest` vorhandene Strawberry
Perl ausdrücklich. Es erfolgt keine Systeminstallation.

Der Build muss aus einer für MSVC und das Windows SDK eingerichteten Umgebung
laufen. Bei mehreren Perl-Installationen wählt `OPENSSL_SRC_PERL` die vollständige
native Perl-EXE explizit aus. Deren Modulprüfung lautet:

```powershell
& $env:OPENSSL_SRC_PERL -MLocale::Maketext::Simple -e 'print "$^V OK\n"'
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Beispielsweise lässt sich portables Strawberry Perl 5.42.2.1 über das
[offizielle Release](https://github.com/StrawberryPerl/Perl-Dist-Strawberry/releases/tag/SP_54221_64bit)
beziehen. Vor dem Entpacken ist der von der
[Projektseite veröffentlichte SHA-256](https://github.com/StrawberryPerl/strawberryperl.com/blob/gh-pages/releases.json)
gegen `32d83be90cf04b807cfb9477482bc36302cdee6f5b04cf57e81adecbd8f07898` zu prüfen.
Archive und Buildwerkzeuge gehören ausschließlich unter `target/` oder
`%LOCALAPPDATA%\kairos-build-tools`.

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
die explizite Pufferlöschung. [rand](https://github.com/rust-random/rand/tree/0.9.5/src/seq)
liefert das Shuffle und den Systemzufall.
Es wurde kein fremder Implementierungscode kopiert. Konkrete Quellstände,
Lizenzen, Alternativen und Übernahmegrenzen stehen in
[store.md](../../docs/sources/store.md).
