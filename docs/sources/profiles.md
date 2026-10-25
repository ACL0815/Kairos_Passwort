# Eigene Profile und automatische Rotation

Zuständig: **nur PR-8**, Prio **4**, Branch `feat/profile-rotation`, gestapelt
auf PR-1 (`feat/core-generator` @ `bd39d8a`). Auswertung und Abrufdatum aller
folgenden Quellen: **25.10.2026**, Zeitzone Europe/Berlin. Übertragen werden
Prinzipien, kein fremder Generatorcode. Reguläre kompatibel lizenzierte Crates
sparen Serialisierungscode; Systemzufall bleibt bei rand/getrandom aus PR-1.
Keine Wortlisten oder Assets übernommen.

| Projekt | Permalink mit Commit/Tag | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| pwgen-go | [5a53dce, config.go](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/config/config.go), [Funktionen](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/docs/pwgen_functions.md) | [MIT](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/LICENSE); Wortdaten separat | `Config`, `ProfileMap`, `Profile`, Templates | Profile als persistierbare Daten; Wortzahl, Ausgabeformat und Zufall trennen. | 4 |
| Password Tech | [c77cade, PasswGen.cpp](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/passw/PasswGen.cpp) | [GPL-2.0-or-later](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/license.txt), auch im Dateikopf | `PasswordGenerator::GetFormatPassw`, Format Password | Explizite Komposition von Wörtern, Zeichenklassen und Wiederholungszahlen. | 3 |
| xkcdpass | [b981361, xkcd_password.py](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py) | [BSD-3-Clause](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/LICENSE.BSD); Wortdaten separat | `generate_xkcdpassword`, `set_case`, `randomized_delimiter_join` | Wortzahl, Großschreibungsmodus und unabhängige Trennerwahl getrennt konfigurieren. | 4 |
| KeePassXC | [2.8.0, PassphraseGenerator.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp) | GPL-2.0-only OR GPL-3.0-only laut Dateikopf | `setWordCount`, `setWordCase`, `setWordSeparator`, `estimateEntropy` | Kleines Regelmodell mit expliziten Optionen und realer Quellgröße. | 4 |
| Bitwarden Clients | [c0ca72a, Optionsmodell](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/libs/tools/generator/core/src/data/default-passphrase-generation-options.ts) | [GPL-3.0 als Standardlizenz](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/LICENSE.txt); Bitwarden License nur im separaten Verzeichnis | `DefaultPassphraseGenerationOptions` | Wortzahl, Trenner, Großschreibung und Ziffer als einfache typisierte Optionen. | 5 |
| Serde | [v1.0.229](https://github.com/serde-rs/serde/tree/v1.0.229), [Manifest](https://github.com/serde-rs/serde/blob/v1.0.229/serde/Cargo.toml) | MIT OR Apache-2.0 laut Manifest | `Serialize`, `Deserialize`, Derive-Attribute | Serialisierung ableiten und beim Einlesen durch `TryFrom` dieselbe Profilvalidierung erzwingen. | 5 |
| serde_json | [v1.0.154, Manifest](https://github.com/serde-rs/json/blob/v1.0.154/Cargo.toml) | MIT OR Apache-2.0 laut Manifest | JSON-Adapter `to_string`/`from_str` | JSON-Roundtrips ohne eigenen Parser prüfen. | 5 |
| Crystal Diceware | [e59b6cc, diceware.cr](https://github.com/aloli-crystal/diceware/blob/e59b6cc6153dd1a4141b439f4319b7faa71e3830/src/diceware.cr) | [MIT](https://github.com/aloli-crystal/diceware/blob/e59b6cc6153dd1a4141b439f4319b7faa71e3830/LICENSE); Wortdaten separat | `Diceware.entropy` | `W * log2(N)` gilt für gleichverteilte, unabhängige Wortauswahl mit eindeutigen Ausgaben. | 3 |
| zxcvbn | [67c4ece, scoring.coffee](https://github.com/dropbox/zxcvbn/blob/67c4ece9efc40c9d0a1d7d995b2b22a91be500c2/src/scoring.coffee) | [MIT](https://github.com/dropbox/zxcvbn/blob/67c4ece9efc40c9d0a1d7d995b2b22a91be500c2/LICENSE.txt) | `most_guessable_match_sequence`, `estimate_guesses` | Ratezahl aus menschlichen Mustern ist eine andere Metrik als Generator-Min-Entropie. | 2 |

## Was übernehmen wir, was nicht, warum?

- **pwgen-go:** Persistierbare Parameter als Prinzip übernehmen. Keine Go-
  Templates, Sprig-Funktionen oder eigene Ausdruckssprache, denn unsere fünf
  Regelbausteine sind direkt prüfbar. Namen/IDs und die Dateiverwaltung bleiben
  Aufgabe von GUI/Store; Core speichert nur Regeln.
- **Password Tech:** Komposition aus expliziten Wort-/Zeichenentscheidungen
  übernehmen. Keine allgemeine Formatsprache, Permutationen oder variable
  Anzahlbereiche. Deren Ausgabekollisionen würden eine neue Entropieanalyse
  erfordern und helfen diesem Auftrag nicht.
- **xkcdpass:** Getrennte Wortzahl, Großschreibungsmodi und Trennerwahl übernehmen.
  Keine Acrostics und keinen optionalen schwachen Zufall. Kairos verwendet
  ausschließlich den System-CSPRNG und die bereits geprüften Unicode-Regeln.
- **KeePassXC:** Kleine typisierte Optionen und reale Quellgröße übernehmen.
  Keine Vault-/UI-Abhängigkeit und keine globale feste Trennerzeichenfolge;
  Kairos zieht jedes Zeichen im Pool unabhängig. Das Zufallsprinzip aus PR-1
  wird wiederverwendet.
- **Bitwarden:** Das kompakte Feldmodell übernehmen. Für Kairos werden der
  Trennerpool, die Anzahl je Lücke und die drei bestehenden Großschreibungsmodi
  genutzt. Keine Passwort-Historie oder Komplexitätsrichtlinie übernehmen;
  Abschreibkomfort und ehrliche Wahrscheinlichkeiten bestimmen die Regeln.
- **Serde:** Reguläre Crate mit Derives nutzen. `CustomProfile` besitzt private
  validierte Regeln, `ProfileRules` bleibt editierbare Rohdatenstruktur.
  Ein privates versioniertes V1-DTO mit `try_from` verhindert das Umgehen des
  Konstruktors beim Einlesen. `ProfileRules` selbst wird nicht serialisiert.
  Unbekannte Versionen/Felder werden abgelehnt, optionale Metadaten haben
  Defaults. Kein selbst geschriebener JSON-/TOML-Parser.
- **serde_json:** Nur als Testabhängigkeit für Roundtrip- und Fehlerprüfungen
  nutzen. GUI/Store wählen ihren Adapter selbst; Core erhält keine Dateipfade,
  Datenbankzugriffe oder automatische Speicherung.
- **Crystal Diceware:** Die einfache Wortmengenrechnung bestätigt das bestehende
  Core-Modell. Keine zusätzliche Würfel-, Hybrid- oder Wortlistenimplementierung
  übernehmen; die vorhandene rand-Integration deckt den Bedarf ab.
- **zxcvbn:** Die Unterscheidung von Ratezahlen und Auswahlbits übernehmen.
  Keinen Mustererkenner einbinden. Kairos kennt seine Erzeugungsverteilung und
  benötigt dafür keine Schätzung aus menschlichen Passwortmustern.

## Mathematische Primärquellen

| Quelle | Dauerhafter Link | Lizenz/Rechte | Abschnitt | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| Bonneau, *Guessing human-chosen secrets*, UCAM-CL-TR-819 (2012) | [Report/DOI](https://doi.org/10.48456/tr-819), [PDF](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-819.pdf) | © 2012 Joseph Bonneau; keine offene Lizenz im Report ausgewiesen | 3.1.1–3.1.3 und 3.2 | Shannon-Entropie, Min-Entropie und Guesswork unterscheiden; Min-Entropie ist `-log2(p_max)`. | 5 |
| Bonneau, *The Science of Guessing* (2012) | [DOI](https://doi.org/10.1109/SP.2012.49), [Symposium-PDF](https://www.ieee-security.org/TC/SP2012/papers/4681a538.pdf) | © 2012 Joseph Bonneau, under license to IEEE; keine weitere Lizenz behauptet | III–V | Schiefe menschliche Verteilungen verlangen eine zum Angreiferziel passende Metrik. | 4 |
| Massey, *Guessing and Entropy* (1994) | [DOI](https://doi.org/10.1109/ISIT.1994.394764), [ETH-PDF](https://www.isiweb.ee.ethz.ch/archive/massey_pub/pdf/BI633.pdf) | Keine offene Lizenz im PDF ausgewiesen | I–III | Erwartete optimale Rateposition ist nicht Shannon-Entropie. | 4 |
| Shannon, *A Mathematical Theory of Communication* (1948) | [DOI](https://doi.org/10.1002/j.1538-7305.1948.tb01338.x) | Keine offene Lizenz hier nachgewiesen | 6–7 | Erwarteter Informationswert ist von der wahrscheinlichsten Ausgabe zu unterscheiden. | 3 |

Übernommen werden die Metrikdefinitionen und ihre Unterscheidung, keine
empirischen Aussagen über menschliche Passwörter als Prognose für Kairos.
Die Mischungsformel ist eine eigene Ableitung aus dem Gesetz der totalen
Wahrscheinlichkeit. Sie ist keine angebliche Implementierung dieser Papers.

Mit `H_i` als **unterer** Schranke der Ausgabe-Min-Entropie von Profil i,
`m = min_i H_i` und n gleichwahrscheinlichen Einträgen gilt:

```text
P(x) = (1/n) * sum_i P(x | i) <= (1/n) * sum_i 2^(-H_i) <= 2^(-m)
H_min >= m

Wenn alle Ausgabemengen paarweise disjunkt sind:
P(x) <= 2^(-m)/n
H_min >= m + log2(n)
```

Das gilt auch für unterschiedlich lange Wörter, weil PR-1 konservative
Profil-Untergrenzen liefert; uniforme Ausgaben innerhalb eines Profils sind
für diese Ungleichungen nicht erforderlich. Die allgemeinere gewichtete
Schranke `log2(n) - log2(sum_i 2^(-H_i))` könnte bei Überlappungen stärker sein.
Wir verwenden dort bewusst m und null Bonus. Diese einfachere sichere
Untergrenze vermeidet eine zusätzliche Gleitkomma-/Überlappungsanalyse.

Disjunktheit wird nur durch unterschiedliche Wortanzahl, Trennerzahl je Lücke,
Ziffernanwesenheit oder disjunkte effektive Trennerpools bewiesen. Ein Wort
erhält keinen Trennerbeweis. Kann auch nur ein Paar überlappen, ist der Bonus
null. Zusätzlich ist für einen positiven Haken Gleichheit der konservativen
Profilstärken innerhalb von 1e-9 Bit erforderlich. Unterschiedlich starke
Profile erhalten m und **0 Bit Bonus**, obwohl eine disjunkte Mischung eine
größere Schranke hätte. Maximal 16 Einträge begrenzen den Bonus auf 4 Bit.
Bei unbekannter Auswahl und 16 gleich starken, disjunkten Profilen sind
die im Konzept genannten **bis +4 Bit** erreichbar. Die eingebauten Profile
0 und 1 überlappen vollständig; Rotation 0–12 erhält folglich keinen Bonus.
Ein schwaches Profil darf nicht durch den Durchschnitt, eine aktuell starke
Auswahl oder einen positiven Haken verdeckt werden. Die 157,1 Bit des Konzepts
sind ein theoretisches Beispiel; die aktuellen ASCII-Beweiskriterien stellen
keine 16 gleich starken Varianten von Profil 12 bereit. Eine öffentlich bekannte Profilwahl erhält über
`ProfileKnowledge::Known` **0 Bit** Bonus. Keine gewählte Profil-ID wird im
Ergebnis gespeichert; die Ausgabe und Segmente selbst verraten dennoch Muster.
Nebenkanäle wie verratene Länge können die Unbekanntheitsannahme entkräften.
Auch bewusstes wiederholtes Neu-Auswürfeln bis zu einem bevorzugten Muster
verletzt die Gleichverteilungsannahme für die endgültige Auswahl. Die spätere
GUI muss dann den Bonus entfernen und darf gezielte Auswahl innerhalb eines
Profils nicht als unveränderte Generatorverteilung ausgeben.

## Recherche und Prüfungen

Zwei Kinder liefen nur lesend ohne Weiterdelegation auf `codex/gpt-6-luna`,
jeweils `reasoningEffort=xhigh`, `serviceTier=priority`. A bewertete pwgen-go,
Password Tech, KeePassXC und Bitwarden. B prüfte Min-/Shannon-/Guessing-Entropie,
Bonneau, Massey, Shannon, Crystal Diceware und zxcvbn. Die Passungswerte hier
sind unsere Bewertung für den konkreten PR, nicht ungeprüft übernommene Urteile.

Eigene Stichproben betrafen pwgen-go `Config`/`Profile`, Password Tech
`PasswGen.cpp` einschließlich Dateilizenz, Bitwardens vier Optionsfelder und
Lizenzaufteilung, KeePassXC 2.8.0, xkcdpass sowie die tatsächlichen
Serde-/serde_json-Versionen und Manifestlizenzen. Bonneaus Report wurde direkt
gegen die Definitionen geprüft; Crystal Dicewares Entropiefunktion und die
zxcvbn-Lizenz wurden ebenfalls geöffnet. Lizenzangaben gelten für Software am
verlinkten Stand, nicht automatisch für beiliegende Daten.

Tests und Herleitung stehen im [Crate-README](../../crates/kairos-core/README.md).
Die Pflichtprüfungen sind `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo test --workspace`. Befehle, Exitcodes und das einmalige kritische Review
auf `claudeAgent/claude-opus-5-5`, `effort=medium`, `fastMode=false` werden in
der PR-Beschreibung dokumentiert. Gemeinsame Index-/Architektur-/Roadmapdateien
und fremde Bereichsdateien bleiben unverändert.
