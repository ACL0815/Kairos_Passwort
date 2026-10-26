# Core: Generator und Entropie

Zuständig: **nur PR-1** (`feat/core-generator`). Auswertung am **25.10.2026**.
Der Vertrag bleibt additiv kompatibel. Es wurden Prinzipien modelliert und
reguläre Crates verwendet; kein fremder Generatorcode wurde kopiert. Es wurden
keine Wortlisten oder Assets übernommen.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| rand | [0.9.5, Integer-Verteilungen](https://github.com/rust-random/rand/blob/0.9.5/src/distr/uniform_int.rs) | [MIT OR Apache-2.0](https://github.com/rust-random/rand/blob/0.9.5/Cargo.toml) | `UniformInt::sample`, `UniformUsize::sample` | Vorbereitete `Uniform`-Verteilungen nutzen Multiply-and-reject nach Lemire. Gleichverteilung bleibt Aufgabe der Crate. | 5 |
| getrandom | [v0.3.4, Windows](https://github.com/rust-random/getrandom/blob/v0.3.4/src/backends/windows.rs) | [MIT OR Apache-2.0](https://github.com/rust-random/getrandom/blob/v0.3.4/Cargo.toml) | `fill_inner`, `ProcessPrng`; [`fill`](https://github.com/rust-random/getrandom/blob/v0.3.4/src/lib.rs) | Systemzufall ohne eigene Win32-Bindings. Fallible Aufrufe müssen bis zum Abbruch weitergereicht werden. | 5 |
| KeePassXC | [2.8.0, Random.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp) | GPL-2.0-only OR GPL-3.0-only laut Dateikopf | `Random::randomUInt` | Zufallswerte vor der Bereichsabbildung zurückweisen. Nur das Prinzip wird übertragen, die Implementierung übernimmt rand. | 5 |
| KeePassXC | [2.8.0, PassphraseGenerator.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp) | GPL-2.0-only OR GPL-3.0-only laut Dateikopf | `setWordList`, `estimateEntropy`, `generatePassphrase` | Eindeutige Quellmenge und reale Größe sind Grundlage für unabhängige Wortziehungen und `W * log2(N)`. Regeln und Formatierung sind getrennt. | 5 |
| xkcdpass | [b981361, xkcd_password.py](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py) | [BSD-3-Clause](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/LICENSE.BSD); Wortdaten separat | `generate_wordlist`, `verbose_reports` | Erst filtern/deduplizieren, dann die tatsächliche Quellgröße zählen. Sonderregeln benötigen ihre eigene Ausgabemengenrechnung. | 5 |
| diceware, Uli Fouquet | [89534ea, README](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/README.rst) | [GPL-3.0-or-later](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/LICENSE); Wortdaten separat | Hinweis zu Prefix-Codes; [`WordList`](https://github.com/ulif/diceware/blob/89534ea7b2c2d090e76307d55497a27ff90cc3d9/diceware/wordlist.py) | Ohne eindeutige Wortgrenzen können verschiedene Wortfolgen dieselbe Ausgabe bilden. Explizite Trenner außerhalb des Wortalphabets verhindern das. | 5 |
| pwgen-go | [5a53dce, README](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/README.md) | [MIT](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/LICENSE); Wortdaten separat | Dokumentierte Profile und Templates, eingebettete EFF-Quellen | Benannte Profile als Daten und Quellwahl getrennt halten. Hier wurde die dokumentierte Oberfläche bewertet, kein interner Algorithmus behauptet. | 3 |
| Bitwarden SDK, Rust-Generator | [rust-v4.0.0, passphrase.rs](https://github.com/bitwarden/sdk-internal/blob/rust-v4.0.0/crates/bitwarden-generators/src/passphrase.rs) | [GPL-3.0-only OR Bitwarden SDK License](https://github.com/bitwarden/sdk-internal/blob/rust-v4.0.0/crates/bitwarden-generators/Cargo.toml) | `PassphraseGeneratorRequest`, `validate_options`, `gen_words` | Validierung vor Auswahl und Ziehen mit Zurücklegen. Der Generator filtert Bindestrich-Wörter für eindeutige Wortgrenzen. | 4 |
| zeroize | [zeroize-v1.8.2, lib.rs](https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/src/lib.rs) | [MIT OR Apache-2.0](https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/Cargo.toml) | `String::zeroize`, `Zeroizing::drop`, Stack/Heap Zeroing Notes | Drop-Guard ab der ersten geheimen Ausgabe; Bytekapazität vorab reservieren. Vorherige Kopien und Reallokationen kann ein Guard nicht beseitigen. | 5 |
| Bitwarden SDK, Geheimnisformatierung | [f76b514, sensitive.rs](https://github.com/bitwarden/sdk-internal/blob/f76b51450582cff9be767c9d9f454c42a84a56c1/crates/bitwarden-sensitive-value/src/sensitive.rs) | [GPL-3.0 OR Bitwarden SDK License 2.0](https://github.com/bitwarden/sdk-internal/blob/f76b51450582cff9be767c9d9f454c42a84a56c1/LICENSE) | `Sensitive<T>`, `ExposeSensitive`, Formatierung | Geheimnisformatierung unterbinden oder redigieren. Formatierungsschutz und Speicherbereinigung sind getrennte Aufgaben. | 4 |
| rpassword | [v7.5.4, lib.rs](https://github.com/conradkleinespel/rpassword/blob/v7.5.4/src/lib.rs) | [Apache-2.0](https://github.com/conradkleinespel/rpassword/blob/v7.5.4/Cargo.toml) | `read_password_with_config`, `FeedbackState::into_password` | Verdeckte Terminaleingabe liefert einen normalen `String`, keinen automatisch geschützten Geheimnispuffer. | 2 |

## Auswahl und Grenzen

**rand/getrandom werden regulär eingebunden.** Produktiv werden nur `os_rng`
und `unbiased` aktiviert. Das aufgelöste `rand_core` 0.9.5 stellt `OsRng`
über `getrandom` bereit. Die API ist zusätzlich am
[rand-0.9.5-Tag mit rand_core 0.9.3](https://github.com/rust-random/rand/blob/0.9.5/rand_core/src/os.rs)
belegt; die tatsächlich verwendete 0.9.5 wurde im Cargo-Quellcache gelesen.
Unter Windows 10+ nutzt die aufgelöste getrandom-Version 0.3.4
`ProcessPrng`, kein selbst geschriebenes `BCryptGenRandom`-Binding. Der private
Adapter auf `Uniform::sample` reicht jeden OS-Fehler als
`RandomSourceUnavailable` weiter. Ein MAX-Sentinel beendet nur rand-intern die
Zurückweisung; sein Ergebnis verlässt die fehlerhafte Auswahl nie. Die
Terminierung ist im Code begründet; beide Integerpfade einschließlich
Grenzbereichen werden auf einem 64-Bit-Windows-Ziel getestet. Ein eigener
32-Bit-Zielbuild wurde nicht ausgeführt.
Die genutzte rand-Version ist dafür exakt gepinnt.

Der alternative Weg `StdRng::try_from_os_rng` spart diesen Adapter, führt aber
einen zusätzlichen, aus dem OS gesäten Generator ein. Für die hier geforderten
direkten Systemziehungen wird er nicht verwendet. `StdRng` existiert nur in
Tests mit festen Seeds. Die frühere Konzeptaufteilung V1 Windows / V2 rand
entfällt, weil `rand::rngs::OsRng` bereits den Windows-Systemgenerator anbindet.
`Uniform::sample` ist im geprüften Stand exakt gleichverteilt; `random_range`
hat einen anderen Integer-Pfad. Das zusätzliche `unbiased`-Feature verhindert
auch dort die dokumentierte kleine Verzerrung, falls dieser Pfad später
genutzt wird. Es gibt keine eigene `% n`-Auswahl.

**KeePassXC, xkcdpass, diceware und Bitwarden liefern Prinzipien.** Übernommen
werden unabhängige Ziehungen mit Zurücklegen, explizite Regelwerte und eine
bereinigte, ausgabeseitig eindeutige Quellmenge. Kairos lehnt ungültige Quellen
vollständig ab, statt ihre Größe unbemerkt zu ändern. Großschreibkollisionen
werden nach dem Kleinschreiben geprüft. Kein Keeper-/Vault-Modell, feste
Zahlenanhänge, Pflichtzeichen-Gruppen oder alternative schwache RNGs werden
übernommen. xkcdpass' Acrostic-Schätzung und die Würfelquellen von diceware
passen nicht zu unseren Profilen. pwgen-go bestätigt den Nutzen benannter
Profile; dessen freie Template-Sprache bleibt außerhalb dieser PR.

**zeroize bleibt die einzige Speicher-Crate.** Der bestehende Pin 1.8.2 bleibt
unverändert. `GeneratedPassword` erhält weder `Debug` noch `Clone`;
Compile-fail-Doctests sichern das ab. Das öffentliche `Zeroizing<String>` ist
selbst nicht redigiert und darf vom Aufrufer nicht geloggt werden. Die
Bitwarden-Redaktionsidee wird durch den vorhandenen Vertrag berücksichtigt,
kein zusätzlicher Wrapper oder Debug-Bypass wird eingeführt. `rpassword` löst
eine Eingabeaufgabe und wird nicht als Generatorabhängigkeit eingebunden.
RAM-Sperren und Dump-Tests bleiben dem Schutzbereich zugeordnet.

## Ausgabemengen und Konzeptentscheidungen

Die ausführlichen Regeln und Formel stehen im
[Crate-README](../../crates/kairos-core/README.md). `ProfileRules` bildet die
13 Profile als Tabelle ab; Generator und Entropie verwenden dieselben Daten.
Additiv kommen `entropy_bits_for`, `profile_rules` und `profile_warnings` dazu.
Bestehende Signaturen und Felder bleiben unverändert. Die neuen Profiltypen
sind nicht erschöpfend; der Pool kann über `Cow` auch Laufzeitdaten besitzen.
Eigene Regeln müssen vor Verwendung positive Wort-/Lückenzahlen sowie einen
nichtleeren, eindeutigen ASCII-Sonderzeichenpool validieren. Der Warnhinweis für
Profil 1 sagt, wenn 21.000 Wörter unterschritten sind; er erhöht die Bits nicht.

- Profil 6 ergänzt genau `(`, `)`, `"`, `'`. Runde Klammern plus zwei gerade
  Anführungszeichen erfüllen die Tabellenzahl 16 mit vier neuen Zeichen.
- Eine innere Position bedeutet eine Grenze zwischen zwei Buchstaben. Sechs
  Unicode-Skalare besitzen fünf solcher Grenzen. Wortwahl, Position und Ziffer
  sind getrennte Ziehungen. Für unterschiedliche Wortlängen ist die Wortwahl
  gleichverteilt; Positionen sind innerhalb des gewählten Wortes gleichverteilt.
- Kleinschreibung vor Auswahl verhindert `Haus`/`haus`-Mehrfachgewichtung;
  Kollisionen, nichtalphabetische Wörter und unbrauchbare Profile liefern
  `InvalidSource`; Trennerausschlüsse bleiben `InvalidConfig`. Umkehrbare
  Großschreibung vermeidet Ausgabe-Kollisionen. `ß` bleibt zum bequemen
  Abschreiben klein; expandierende und nicht umkehrbare Positionen werden
  nicht als Großschreibauswahl verwendet. Importer sollen NFC liefern.
- `entropy_bits(N, cfg)` ist das dokumentierte Modell mit sechs Buchstaben.
  `entropy_bits_for` verwendet minimale Innen- und Großschreibpositionszahlen
  über der gesamten Quelle. Das ergibt eine konservative Schranke der
  Min-Entropie; bei gleichen Längen und vollständig umwandelbaren Wörtern ist
  sie exakt. Ein nicht umwandelbarer Anfang liefert keinen angerechneten Bonus.
- Ziffern und Trenner können nicht aus Quellwörtern stammen. Daher lassen sie
  sich aus der Ausgabe eindeutig entfernen und die Wörter nach umkehrbarer
  Kleinschreibung rekonstruieren. Wiederholte Buchstaben oder Wortziehungen
  erzeugen keine zusätzlichen Ausgabekollisionen. Ein Profilbonus wird nicht
  behauptet. Die GUI muss den quellabhängigen Wert anzeigen, niemals die
  einfache Sechs-Buchstaben-Modellfunktion.

## Prüfung und Rechercheprotokoll

### Ergänzung: unterschiedliche zufällige Trenner (26.10.2026)

Sind alle zufällig gezogenen Trenner eines Passworts gleich, zieht der Core
das gesamte Trennertupel erneut. Das gilt für Profile 2–12 und eigene Profile,
bei Profil 12 über alle zehn Positionen gemeinsam. Gleiche Paare sind erlaubt.
Mit effektivem Poolumfang `n >= 2` und Positionszahl `k >= 2` bleiben genau
`n^k - n` gleichwahrscheinliche Tupel; der Entropieanteil ist daher
`log2(n^k - n)` statt `k * log2(n)`. Gemeinsames Rejection Sampling erhält die
Gleichverteilung; nur eine Position neu zu ziehen täte dies nicht.
Der Code berechnet die Formel ohne Potenzüberlauf mit `ln_1p`.
Bei `n = 1` sowie `k < 2` entfällt die Regel; feste Profile 0/1 bleiben gleich.
Jeder Versuch gelingt mit Wahrscheinlichkeit mindestens 1/2. Zufall kommt
weiterhin ausschließlich aus rand/OsRng; Fehler werden zurückgegeben.

Alle Entropie-APIs einschließlich Rotation und Profilbonus verwenden die
gemeinsame korrigierte Schranke. Die bisherigen Disjunktheitsbeweise gelten
weiter für Teilmengen der bisherigen Ausgaben. Die GUI übernimmt bereits
`GeneratedPassword.entropy_bits` für Bitwert und Stärkering. In-place-Ersetzung
von ASCII-Trennern erhält Segmentbereiche und UTF-8-Hervorhebung ohne App-Diff.
Die Konzeptwerte sind angepasst (Profil 2: 64,398191 → 64,339297 Bit;
Trenneranteil 6,965784 → 6,906891 Bit). Die Regression prüft alle 13 Werte auf
±0,000001 Bit, sechs binäre Dreiertupel per Chi-Quadrat mit festem Seed,
96 eindeutige Wort-/Trennerausgaben, alle 1.022 erlaubten binären Zehnertupel
von Profil 12, Einzeichenpools, wiederholte Zurückweisung, OS-Fehler und
unveränderte Unicode-Segmente. Erschöpfende Rotationsprüfungen berücksichtigen
die bedingten Wahrscheinlichkeiten und prüfen Bonusentscheidungen nach der
Korrektur. Die 3.840 Ausgaben mit festem Trenner bleiben unverändert.

Die drei angeforderten Recherchen liefen auf `codex/gpt-6-luna`, jeweils
`reasoningEffort=xhigh`, `serviceTier=priority`, nur lesend ohne Weiterdelegation.
A prüfte Passphrasen und Kollisionen, B Zufall und Gleichverteilung,
C Geheimnislebensdauer. Stichproben am tatsächlich eingebundenen Stand betrafen
rand 0.9.5, getrandom 0.3.4, KeePassXC 2.8.0, xkcdpass, diceware,
Bitwarden-Generator/-Wrapper und zeroize. Die pwgen-go-Oberfläche wurde
zusätzlich direkt gelesen. Das Review korrigierte eine zwischenzeitlich auf
39 Zeichen gekürzte diceware-ID zurück auf die ursprüngliche, per GitHub-API
bestätigte vollständige ID `89534ea7b2c2d090e76307d55497a27ff90cc3d9`.

Die Tests erzeugen künstliche Quellen mit 7.000 beziehungsweise 21.000
eindeutigen Sechs-Buchstaben-Wörtern und prüfen die angepassten Konzeptwerte
wie oben beschrieben. Sie prüfen außerdem Quellenfehler, Ausschlüsse, Unicode-Bytebereiche,
konservative Werte, Speichergröße, Fehler der Zufallsquelle auch nach
teilweiser Ausgabe sowie statistische Verteilungen mit festen Seeds.
Nach dem ursprünglichen Review kamen erschöpfende Prüfungen von Wort-/Trennerausgaben
und allen 3.840 Ausgaben von Profil 12 für `ab` mit festem Trenner hinzu.
Sie prüfen Eindeutigkeit und die angegebene Ausgabe-Min-Entropie am Generator.
Die Pflichtprüfungen sind `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo test --workspace`; Ergebnisse und Review-Nachbesserungen stehen in der PR.

Genau ein kritisches Review lief auf `claudeAgent/claude-opus-5-5`,
`effort=medium`, `fastMode=false`. Die Erweiterbarkeit der Profiltypen wurde
nachgebessert. Stack-/Registerreste und Graphemgrenzen anderer Schriften sind
im README als Grenzen dokumentiert und bleiben den Folgearbeiten zugeordnet.
Der Pin bleibt mit sichtbarer Manifestbegründung bestehen. Der gemeinsame
Architekturtext ist gemäß Schreibverbot unverändert; sein Scaffold-Abgleich
muss bei der Integration durch den zuständigen Lead erfolgen.

Die Lizenzangaben beziehen sich auf die verlinkten Softwarestände. Wörterbücher
und andere Daten können abweichende Lizenzen haben und werden hier nicht
redistribuiert. Die Sonderausnahme zur direkten Nutzung von rand/getrandom und
die Bereichsgrenzen stehen unverändert in [SOURCES.md](../SOURCES.md).
