# kairos-core

Wortpasswörter nach den Profilen 0–12, mit Windows-Systemzufall, konservativer
Entropieanzeige und UTF-8-Segmenten für die Hervorhebung. Core kennt weder GUI
noch Datenbank, Zwischenablage oder KeePassXC. Der bestehende
[API-Vertrag](../../docs/ARCHITECTURE.md) bleibt erhalten.

## Verwendung und additive API

`generate(&source, &config)` liefert `GeneratedPassword` oder einen Fehler ohne
Geheimnisinhalt. Die Quelle implementiert `WordSource`. Default ist Profil 0.
Wörter werden unabhängig mit Zurücklegen gezogen; Wiederholungen sind erlaubt.

- `profile_rules(ProfileId)` liefert gemeinsame Regeldaten für Generator und
  Entropieberechnung der eingebauten Profile.
- `profile_warnings(source.len(), &config)` meldet eine unterschrittene
  Listenempfehlung. Profil 0 empfiehlt 7.000, Profil 1–12 empfiehlt 21.000
  Wörter. Kleinere Quellen bleiben nutzbar; ihre reale Größe zählt.
- `entropy_bits(source_len, &config)` bildet das Sechs-Buchstaben-Modell ab.
  Es ist nicht für die Sicherheitsanzeige geeignet. Für leere Quellen und
  unzulässige Trennerausschlüsse gibt es `0.0` zurück.
- `entropy_bits_for(&source, &config)` prüft die tatsächlichen Wörter und liefert
  `Result<f64, GenerateError>`. `GeneratedPassword.entropy_bits` verwendet
  ausschließlich diese konservative Berechnung.

Die GUI muss `GeneratedPassword.entropy_bits` oder `entropy_bits_for` anzeigen.
`CaseRule`, `ProfileRules` und `ProfileWarning` sind `#[non_exhaustive]`, damit
spätere Ergänzungen additiv bleiben. Der Trennerpool ist `Cow<'static, str>`;
eingebaute Regeln leihen Literale, eigene Regeln können zur Laufzeit erzeugte
Strings besitzen. `profile_rules` liefert weiterhin nur eingebaute Regeln.

## Eigene Profile und optionale Rotation

PR-8 ergänzt `CustomProfile`, `ProfileRotation`, `generate_custom`,
`generate_rotating`, `entropy_bits_for_custom` und `entropy_bits_for_rotation`.
`GeneratorConfig`, `ProfileId`, `GeneratedPassword` und die Regeln 0–12 bleiben
unverändert. Ohne Aufruf der Rotations-API gibt es keinen Profilwechsel.

```rust
use kairos_core::{CaseRule, CustomProfile, ProfileId, ProfileRotation};

let own = CustomProfile::new(5, "-+.,", 1, true, CaseRule::RandomInitial)?;
let rotation = ProfileRotation::new(vec![ProfileId::new(0)?.into(), own])?;
let json = serde_json::to_string(&rotation)?;
let restored: ProfileRotation = serde_json::from_str(&json)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

`CustomProfile::new` verlangt 1–64 Wörter, 1–8 Trenner je Lücke und einen
nichtleeren Pool eindeutiger ASCII-Sonderzeichen. Die Obergrenzen begrenzen
Ressourcenverbrauch, nicht die errechnete Mindeststärke. Ein einzelnes Wort ist
zulässig; auch dann bleibt der Pool valide und darf nicht leer ausgeschlossen
werden. Buchstaben, Ziffern und Unicode-Zeichen sind als Trenner ausgeschlossen,
damit die Ausgabe eindeutig zerlegbar bleibt. Die Großschreibung hat dieselben
drei Modi wie die eingebauten Profile. Wörter und benötigte Innen-/Großschreib-
positionen prüft die Generierung gegen die tatsächliche Quelle. Kleine Quellen
bleiben zulässig; es gibt keine zusätzliche Komplexitätspflicht oder Mindestbitzahl.

`rules()` leiht unveränderliche Daten. Änderungen erfolgen an einem Klon mit
anschließendem `CustomProfile::try_from(rules)`. So lässt sich auch die reine
Listenempfehlung einstellen und mit `custom.warnings(source.len())` prüfen.
Die Empfehlung erzeugt keine Entropie; null deaktiviert sie.
`ProfileRules` bleibt ein editierbarer Rohdatentyp ohne Serialisierung.
Nur `CustomProfile` darf generieren.

Die Typen implementieren Serde. Eigene Profile erscheinen in JSON als Objekt
mit `version: 1`, `word_count`, `separator_pool`, `separators_per_gap`, `insert_digit`, `case`
und `recommended_source_len`; `case` ist `lowercase`, `random_initial` oder
`one_random_uppercase`. `recommended_source_len` hat beim Einlesen den Default 1;
die regelbestimmenden Felder und die Version sind erforderlich. Unbekannte
Versionen, Felder und Modi werden abgelehnt, damit keine Sicherheitsregel still
ignoriert wird. Ein privates `CustomProfileV1` trennt das Speicherformat von
`ProfileRules`. Version 1 bleibt ein stabiler Vertrag; spätere Formate brauchen
eine neue Version und explizite Migration unter Erhalt des V1-Lesepfads.
Neue optionale V1-Metadaten müssen einen Serde-Default erhalten. Alte Programme
dürfen unbekannte Formate ausdrücklich ablehnen. Eine Rotation ist ein Array
solcher Objekte. Auch beim Einlesen läuft dieselbe Validierung. Serde ist eine reguläre Abhängigkeit,
`serde_json` nur eine Testabhängigkeit. GUI/Store können den JSON-Adapter selbst
einbinden. Core schreibt weder Dateien noch Store. Aufrufer sollten die Größe
externer Dokumente vor dem Parsen begrenzen, denn Serde liest das Rohobjekt vor
der Regelvalidierung ein.

Eine Rotation enthält 1–16 Einträge, einschließlich Kopien eingebauter Profile.
Jeder Aufruf von `generate_rotating(&source, &rotation, &excluded, knowledge)`
zieht einen Eintrag erneut gleichverteilt über `OsRng` und rand-`Uniform`.
Wiederholungen des vorherigen Profils sind erlaubt und nötig für unabhängige
Ziehungen. Identische Einträge werden nicht dedupliziert; jeder Listenplatz hat
Gewicht `1/n`, Überschneidungen verhindern den Bonus. Alle Einträge müssen mit
derselben Quelle und den globalen Trennerausschlüssen funktionieren. Eine
ungültige Kombination wird vor der Zufallsauswahl vollständig abgelehnt.
Die Wörterbuchprüfung läuft pro Aufruf einmal, nicht einmal pro Profil.

`RotatedPassword` enthält `password: GeneratedPassword` und
`rotation_bonus_bits` für den späteren GUI-Haken. Es gibt keine ausgewählte
Profil-ID, kein Regelobjekt und keine zusätzliche Kopie des Geheimnisses im
Ergebnis. `password.entropy_bits` beschreibt die gesamte Rotation, unabhängig
von der konkreten Auswahl. Es darf deshalb zusammen mit dem Bonus angezeigt
werden, ohne das gezogene Profil in Metadaten festzuhalten.

## Herleitung des Profilbonus

Für Profil i begrenzt `H_i` aus der realen Quelle jede Ausgabewahrscheinlichkeit
von oben durch `2^(-H_i)`. Mit `m = min_i H_i` und gleichverteilter Profilwahl
gilt für jede mögliche Ausgabe x:

```text
P(X=x) = (1/n) * sum_i P(X=x | Profil=i)
       <= (1/n) * sum_i 2^(-H_i) <= 2^(-m)
H_min(X) = -log2(max_x P(X=x)) >= m

Bei paarweise disjunkten Ausgabemengen trägt höchstens ein Profil bei:
P(X=x) <= 2^(-m) / n
H_min(X) >= m + log2(n)
```

Die Implementierung beweist Disjunktheit paarweise durch unterschiedliche
Wortanzahl, unterschiedliche Trennerzahl pro Lücke, unterschiedliche Anwesenheit
einer Ziffer oder disjunkte **effektive** Trennerpools. Die Trennerbeweise gelten
nur bei mindestens zwei Wörtern. Alphabetische Quellwörter und sichtbare
ASCII-Trenner/Ziffern machen diese Merkmale eindeutig. Großschreibungsmodi
allein gelten konservativ nicht als Beweis. Zusätzlich müssen die konservativen
Profilstärken innerhalb von 1e-9 Bit übereinstimmen. Scheitert auch nur ein Paar
am Beweis oder unterscheiden sich die Stärken, ist der gesamte Bonus null.
Es wird kein Überlappungsgraph und keine
aufwendige Ausgabemengenanalyse gebaut.

16 gleich starke, disjunkte Profile liefern genau +4 Bit gegenüber einem
bekannten festen Profil. Wegen maximal 16 Einträgen beträgt der Bonus niemals
mehr als 4 Bit. Bei unterschiedlichen Stärken ist die Anzeige immer die
schwächste Profilstärke mit **0 Bit Bonus**, niemals die gerade gezogene Stärke
oder der Durchschnitt. So kann ein schwacher Rotationseintrag keinen positiven
Haken suggerieren, während die Sicherheit gegenüber einer starken festen
Auswahl sinkt. Disjunkte Profile mit 1 und 8 Bit hätten mathematisch 2 Bit
Min-Entropie; die API zeigt konservativ 1 Bit und keinen Bonus. Auch bei
Überschneidungen verwenden wir m ohne Bonus.
Die 157,1 Bit im Konzept sind ein theoretisches Beispiel, keine Zusage dieser
API. Mit den einfachen Beweiskriterien und ASCII-Pools stehen keine 16
gleich starken Varianten von Profil 12 zur Verfügung. Die vorhandenen Profile
0–12 erfüllen die Voraussetzungen ebenfalls nicht; 0 und 1 haben identische
Ausgaben.

`ProfileKnowledge::Unknown` setzt voraus, dass der Angreifer vor dem Raten
Quelle, vollständige Regelliste und Algorithmus kennt, aber keine zusätzliche
Information über die aktuelle Auswahl hat. `ProfileKnowledge::Known` entfernt
den Bonus unabhängig von Disjunktheit und liefert die schwächste feste
Profilentropie als sichere Untergrenze. Damit die GUI den Haken ehrlich setzt,
muss sie bei anderweitig verratener aktueller Auswahl `Known` verwenden.
Auch verratene Länge oder Struktur kann die Annahme entkräften. Das Passwort
selbst zeigt zwangsläufig sein Muster; Rotation soll das Raten eines noch
unbekannten Passworts erschweren, kein dem Angreifer vorliegendes Passwort
verschleiern. Segmente und Geheimnisse dürfen nicht geloggt werden.
Die Schranke gilt pro unabhängiger Generierung. Wählt ein Nutzer nach bewusstem
Neu-Auswürfeln ein bevorzugtes Muster aus, ist die endgültige Auswahl nicht mehr
gleichverteilt. Die GUI darf für eine solche Auswahl keinen Rotationshaken
versprechen und muss mindestens den Bonus entfernen (`Known`). Das ist noch
keine Garantie für eine unveränderte Profilentropie, wenn auch innerhalb eines
Profils gezielt nach Wörtern oder Mustern ausgewählt wird.

Die Rechnung ist eine Untergrenze der **Min-Entropie**, keine Shannon-Entropie
und keine Messung durchschnittlicher Rateversuche oder menschlicher Auswahl.
Bonneaus [UCAM-CL-TR-819](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-819.html)
begründet die getrennte Betrachtung von Wahrscheinlichkeiten und Rateerfolg;
die konkrete konservative Mischungsrechnung oben ist unsere Herleitung.

Die neuen Tests prüfen Konstruktor-/JSON-Validierung, alle Großschreibungsmodi,
Serialisierungs-Roundtrips, jede Disjunktheitsbedingung, Überlappungen und
Ausschlüsse, 16 Profile mit +4 Bit, ungleiche Stärken, bekanntes Profil und
CSPRNG-Fehler. Statistische Regressionen mit festen Seeds ziehen je 32.000-mal
aus 3, 7 und 16 Profilen und prüfen Chi-Quadrat sowie aufeinanderfolgende Paare.
Sie prüfen Auswahlfehler, nicht die kryptografische Qualität des Systemzufalls.
Für alle Profile 0–12 stimmen kopierte Regeln, Segmente, Entropie und gesetzte
Test-Seeds überein; die ursprünglichen Konzepttests bleiben erhalten.
Erschöpfende Tests zählen zusätzlich echte Generatorausgaben mit ihren
Wahrscheinlichkeiten. Sie prüfen die tatsächliche Mischungs-Min-Entropie und
leere Schnittmengen bei Bonus. Dazu gehören gleiche Gesamttrennerzahl bei
unterschiedlicher Wortzahl mit Ziffer, Ziffer/Großschreibung, variable Wortlängen
und nicht umwandelbare Wortanfänge. Versionsfehler, Defaults, Listenempfehlungen,
`Known`-Generierung und fehlendes `Debug`/`Clone` sind ebenfalls abgesichert.

Feste eigene Profile haben bewusst einen direkten Generierungsweg. Aufrufer
müssen dafür keine künstliche Rotation mit einem Eintrag anlegen. Die beiden
Ergebnisformen behalten das geforderte Feld `rotation_bonus_bits`; intern
teilen sie Quellenprüfung, Regeldaten und Geheimniskonstruktion.

## Profile und Ausschlüsse

Die Regeln sind kumulativ. Profil 1 ändert nur die Listenempfehlung. Alle
Trenner und Groß-/Kleinschreibentscheidungen werden unabhängig gezogen.

| Profil | Wörter | Trennerpool | Ziffer | Großschreibung | Trenner je Lücke |
|---|---:|---|---|---|---:|
| 0 | 4 | `-` | keine | klein | 1 |
| 1 | 4 | `-` | keine | klein | 1 |
| 2 | 4 | `-+.,#` | keine | klein | 1 |
| 3 | 4 | wie 2 | eine, innen | klein | 1 |
| 4 | 4 | zusätzlich `*!?` | eine, innen | klein | 1 |
| 5 | 4 | zusätzlich `_:;/` | eine, innen | klein | 1 |
| 6 | 4 | zusätzlich `(`, `)`, `"`, `'` | eine, innen | klein | 1 |
| 7 | 4 | zusätzlich `$%&=` | eine, innen | klein | 1 |
| 8 | 4 | 20 Zeichen | eine, innen | Anfang unabhängig groß/klein | 1 |
| 9 | 5 | 20 Zeichen | eine, innen | wie 8 | 1 |
| 10 | 6 | 20 Zeichen | eine, innen | wie 8 | 1 |
| 11 | 6 | 20 Zeichen | eine, innen | genau ein Großbuchstabe je Wort | 1 |
| 12 | 6 | 20 Zeichen | eine, innen | wie 11 | 2 |

Die Klammern in Profil 6 sind runde Klammern. Zusammen mit den beiden geraden
Anführungszeichen ergeben sie die geforderten vier Ergänzungen auf 16.
Ein Ziffernprofil wählt zuerst eines der Wörter gleichverteilt, dann eine seiner
inneren Grenzen zwischen Unicode-Skalaren und eine Ziffer `0`–`9`.
Bei sechs Buchstaben gibt es fünf innere Grenzen. Die Ziffer wird eingefügt,
kein Buchstabe wird ersetzt. Die Position der Großschreibung bezieht sich auf
die Buchstaben vor der Zifferneinfügung.

`excluded_chars` entfernt ausschließlich Trenner aus dem jeweiligen Pool.
Doppelte und poolfremde Einträge ändern nichts. Ein leerer Pool, einschließlich
des ausgeschlossenen festen `-` in Profil 0/1, ergibt `InvalidConfig`.

## Wörter, Unicode und ehrliche Entropie

Die Quelle bleibt während eines Aufrufs stabil. Vor jeder Generierung prüft der
Core die gesamte Quelle. Er schreibt ihre Wörter klein und verlangt danach
nichtleere, eindeutige alphabetische Einträge ohne verbleibende Großbuchstaben.
Ziffern, Trenner, Leerzeichen sowie Kollisionen wie `Haus`/`haus` führen zu
`InvalidSource`. Eine Quelle mit einem solchen Eintrag wird vollständig
abgelehnt und nicht still verkleinert. Externe Importer müssen diese
Voraussetzungen vor der Nutzung erfüllen. Die Prüfung benötigt O(N) Zeit und
O(N) Referenzen; bereits kleingeschriebene Wörter werden nur ausgeliehen.
Normalisierte Wörter sind Wörterbuchdaten, keine Kopien ausgewählter Passwörter.
Importer sollen NFC liefern. Übliche zerlegte lateinische Akzente (NFD) werden
als nichtalphabetische Folge abgelehnt; Core führt keine NFC-Normalisierung
oder vollständige Prüfung der Unicode-Normalform durch. `InvalidSource`
unterscheidet ungeeignete Quellen von `InvalidConfig` für Trennerausschlüsse.

Ziffernprofile benötigen mindestens zwei Unicode-Skalare je Wort. Profil 11/12
benötigt je Wort mindestens eine Position mit einer umkehrbaren Großschreibung
auf genau einen Unicode-Skalar. Deutsche Umlaute funktionieren. `ß` bleibt
klein und zählt nicht als Großschreibposition; so wird kein schwer abtippbares
`ẞ` eingefügt. Erweiterungen wie `ß` zu `SS`, `ﬀ` zu `FF` und nicht
umkehrbare Abbildungen wie finales `ς` zu `Σ` werden als Großschreibposition
ausgeschlossen. Bei Profil 8–10 bleibt ein nicht umwandelbarer Anfang gleich;
dafür werden keine Auswahlbits angerechnet. Wortpositionen zählen Skalare,
keine Graphemcluster; Highlight-Bereiche zählen UTF-8-Bytes.
Bei alphabetischen kombinierenden Zeichen anderer Schriftsysteme kann eine
Skalargrenze innerhalb eines sichtbaren Zeichens liegen. Deren komfortable
Unterstützung braucht eine Graphemprüfung oder passende Quellenfilter im
Sprach-/Importbereich; diese PR verspricht sie nicht.

Mit N Wörtern, W Ziehungen, S verbleibenden Trennern, K Trennern je Lücke,
I als minimaler Zahl innerer Grenzen und C als minimaler Zahl von
Großschreibpositionen gilt die konservative Schranke:

```text
H >= W * log2(N) + (W - 1) * K * log2(S)
     + log2(10 * W * I)              [falls eine Ziffer eingefügt wird]
     + W                            [zufälliger Anfang, alle umwandelbar]
     + W * log2(C)                  [genau ein Großbuchstabe je Wort]
```

Die letzten beiden Zeilen sind Alternativen. Ist mindestens ein Anfang nicht
umwandelbar, wird der Anfangsbonus insgesamt konservativ mit null angesetzt.
Es handelt sich um eine untere Schranke der Ausgabe-Min-Entropie
`-log2(maximale Ausgabewahrscheinlichkeit)`, nicht um die Shannon-Entropie einer
gemischten Längenverteilung. Für gleich lange, vollständig umwandelbare Wörter
ist der Wert exakt. Die einfache API nimmt sechs Buchstaben, I = 5 und C = 6 an.

Jede gültige Ausgabe lässt sich eindeutig in Trenner, eingefügte Ziffer und
kleingeschriebene Quellwörter zurückführen. Wiederholte Buchstaben verursachen
keine Positionskollision, weil der Großbuchstabe beziehungsweise die Ziffer
sichtbar an einer anderen Position steht. Die Prüfung verhindert kollidierende
Quellwörter und mehrdeutige Wortgrenzen. Bei nicht umwandelbaren Anfängen können
beide Zufallsentscheidungen dieselbe Ausgabe liefern; der konservative Wert
berücksichtigt das. Die API für ein festes Profil rechnet keinen Profilbonus an.

Die Tests reproduzieren die Konzepttabelle mit 7.000 beziehungsweise 21.000
verschiedenen Sechs-Buchstaben-Wörtern auf ±0,1 Bit. Statistische Tests prüfen
Wortwahl, Trenner, Ziffern, Wort-/Innenpositionen, Anfangspaare, doppelte Trenner
und Großschreibpositionen mit festen `StdRng`-Seeds. Der Produktionspfad bietet
keine öffentliche RNG-Injektion und verwendet ausschließlich `OsRng`.
Zusätzlich werden 128 Ausgaben mit variablen Wortlängen/Trennern und sämtliche
3.840 Ausgaben von Profil 12 für die Quelle `ab` bei festem `-` erschöpfend
geprüft. Eindeutigkeit und berechnete Min-Entropie stimmen überein.

## Zufall und Lebensdauer

`rand` 0.9.5 verwendet `OsRng` aus `rand_core` und `getrandom` 0.3.4. Unter
Windows 10+ ruft dessen Backend `ProcessPrng` auf. V1 und V2 des Konzepts fallen
damit zusammen. Für jede Auswahl kommt ein vorbereiteter `Uniform<usize>` zum
Einsatz; eigener Modulo- oder Rejection-Sampling-Code entfällt.

Da rand einen unfehlbaren `RngCore` für `Uniform::sample` erwartet, überbrückt
ein privater Adapter die fallible OS-API. Nach einem Fehler liefert er nur zum
Beenden des Samplers `MAX`; `draw` verwirft diesen Wert und liefert immer
`RandomSourceUnavailable`. Es gibt keinen Ersatzgenerator und kein Passwort
aus solchen Werten. Die Terminierung folgt aus dem geprüften Lemire-Pfad in
rand 0.9.5 und ist einschließlich eines Fehlers nach Zurückweisung getestet.
Grenzfälle betreffen beide Integerpfade auf einem 64-Bit-Windows-Ziel;
ein eigener 32-Bit-Zielbuild wurde nicht ausgeführt.
Darum ist diese Version exakt gepinnt. Ein Update muss den Adapter erneut
prüfen. `unbiased` ist zusätzlich aktiviert, falls später ein anderer
Integer-Sampling-Pfad der Crate genutzt wird. `StdRng` ist nur Testabhängigkeit.

Das Passwort entsteht direkt in einem `Zeroizing<String>`. Die Kapazität wird
vorab aus maximaler Wort-Bytebreite, möglicher Großschreibverlängerung, Ziffer
und Trennerzahl berechnet. Es gibt keinen temporären Passwort-String, `join`,
Klon oder Wachstum des Geheimnispuffers. Auch ein später OS-Fehler lässt den
teilgefüllten Puffer normal droppen und überschreiben. Segmente enthalten nur
Typ und Bytebereich; Großbuchstaben und Ziffern teilen die Wortsegmente.

`GeneratedPassword` implementiert weder `Debug` noch `Clone`.
`Zeroizing<String>` allein redigiert `Debug` allerdings nicht. Verbraucher
dürfen das öffentliche `secret`-Feld nicht loggen. Diese API kann Kopien durch
Aufrufer, UI, Zwischenablage, Register und Betriebssystem nicht verhindern.
Speicherhärtung bleibt PR-9 zugeordnet.
Auch Segmente dürfen nicht geloggt werden, da sie Wortlängen und Positionen
verraten. Zufallswerte und ausgewählte Indizes/Zeichen können kurzzeitig in
Stack oder Registern verbleiben; deren vollständiges Löschen ist mit diesem
Guard nicht zugesichert und bleibt Teil der späteren Speicherhärtung.

Der gesperrte gemeinsame Architekturtext beschreibt noch das Scaffold. Sein
Abgleich auf die implementierten APIs und die Anzeigeregel ist bei Integration
durch den zuständigen Lead nötig; diese Bereichs-PR ändert ihn nicht.

## Open-Source-Vorbilder

- [pwgen-go, Profilkonfiguration](https://github.com/gabe565/pwgen-go/blob/5a53dce1f9cbde823855ee6facf6889eb5275fde/internal/config/config.go)
  und [Bitwarden, Generatoroptionen](https://github.com/bitwarden/clients/blob/c0ca72a0c83cc8b7f990138ac04961cf63d985bc/libs/tools/generator/core/src/data/default-passphrase-generation-options.ts)
  liefern das Prinzip persistierbarer, kleiner Regelobjekte. Die allgemeinen
  Template-Sprachen von pwgen-go und
  [Password Tech](https://github.com/cthoeing/passwordtech/blob/c77cade188d9b307302d0f367c4f95ceb3aebc29/src/passw/PasswGen.cpp)
  werden zugunsten direkt prüfbarer Felder nicht übernommen.
- [Serde v1.0.229](https://github.com/serde-rs/serde/tree/v1.0.229)
  liefert als reguläre MIT/Apache-2.0-Crate die Serialisierung und validierte
  Deserialisierung. [serde_json v1.0.154](https://github.com/serde-rs/json/tree/v1.0.154)
  bleibt Testadapter. Die Bewertung, Lizenzen, mathematischen Primärquellen und
  Grenzen von PR-8 stehen in [profiles.md](../../docs/sources/profiles.md).
- [rand 0.9.5](https://github.com/rust-random/rand/blob/0.9.5/src/distr/uniform_int.rs)
  und [getrandom 0.3.4](https://github.com/rust-random/getrandom/blob/v0.3.4/src/backends/windows.rs)
  werden als Crates für gleichverteilte Auswahl und Systemzufall genutzt.
- [KeePassXC 2.8.0](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/PassphraseGenerator.cpp)
  und [Random.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp)
  liefern die Trennung von Quellmenge, Wortzahl, Ausgabe und sicherem Zufall.
- [xkcdpass](https://github.com/redacted/XKCD-password-generator/blob/b981361292ae7328313e116178acacc90680df5e/xkcdpass/xkcd_password.py)
  liefert das Prinzip, Filter und tatsächliche Quellgröße vor der Bitrechnung
  festzulegen.
- [zeroize 1.8.2](https://github.com/RustCrypto/utils/blob/zeroize-v1.8.2/zeroize/src/lib.rs)
  liefert den Drop-Guard und die Hinweise zu Vorabkapazität und Speicherkopien.

Die vollständige Bewertung einschließlich verworfener Kandidaten, Lizenzen und
Abrufdatum steht in [core.md](../../docs/sources/core.md). Übernommen wurden
Prinzipien und reguläre Crate-Abhängigkeiten, kein fremder Generatorcode,
keine Wörterbücher und keine Assets.

## Abschlussintegration mit mehreren Quellen

Die additive `generate_rotating_from_sources(&[&dyn WordSource], &rotation,
&excluded)` verlangt genau eine Quelle je Profileintrag in derselben Reihenfolge.
Jedes Paar wird vor der Zufallsauswahl geprüft. Das Ergebnis nennt stets das
Minimum der individuellen quellenabhängigen Grenzen und null Bonus, auch bei
Disjunktheit. Bestehende APIs mit einer gemeinsamen Quelle bleiben unverändert.
Die App verwendet dies für Deutsch bei Profil 0 und DE+EN+LA ab Profil 1,
jeweils nach Nutzer-Sperrliste. Die frühere Aussage zu identischen Ausgaben von
Profil 0/1 gilt für dieselbe übergebene Quelle, nicht für diese App-Zuordnung.
