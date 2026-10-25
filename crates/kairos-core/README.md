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
  Entropieberechnung. Die Struktur bereitet eigene Profile vor; diese PR fügt
  keinen weiteren Konfigurationsmodus hinzu.
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
eingebaute Regeln leihen Literale, künftige eigene Regeln können zur Laufzeit
erzeugte Strings besitzen. `profile_rules` liefert nur eingebaute Regeln.
Bevor PR-8 eigene Regeln an die Berechnung übergibt, muss deren Konstruktor
mindestens Wortzahl ≥ 1, Trennerzahl je Lücke ≥ 1 und einen nichtleeren,
eindeutigen ASCII-Sonderzeichenpool validieren. Die heutigen Regeln erfüllen
diese Voraussetzungen; ein Test sichert ihre Invarianten ab.

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
berücksichtigt das. Ein unbekannter Profilwechsel bringt hier keinen Bonus.

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
