# Englisch, Latein und die kombinierte Quelle

PR-7, Prio 4, gestapelt auf PR-2. Die deutsche Liste bleibt bytegleich.
Alle drei eingebetteten Quellen haben **je 7.130 eindeutige Wörter** und
sind paarweise disjunkt. `combined()` enthält **21.390 verschiedene Wörter**,
jeweils mit `Language::German`, `Language::English` oder `Language::Latin`.

```rust
use kairos_core::{Language, WordSource};
let source: &dyn WordSource = kairos_wordlists::combined();
assert_eq!(source.len(), 21_390);
assert_eq!(source.language(1), Language::English);
```

`german()`, `english()`, `latin()` und `combined()` geben gemeinsam
`&'static EmbeddedWordList` zurück. `word(index)` und `language(index)`
verwenden dieselben Grenzen; ungültige Indizes dürfen entsprechend dem
bestehenden Vertrag paniken. `OnceLock` initialisiert jede Quelle einmal.
Die Daten und Sprachflags sind stabil und benötigen weder Netzwerk noch
Dateizugriff zur Laufzeit. Es gibt keine neue Laufzeitabhängigkeit.

## Unabhängige Sprachwahl je Wort

Der Generator soll **für jede Wortposition unabhängig einen gleichverteilten
Index aus `0..combined().len()` ziehen**. Es wird keine Sprache für das ganze
Passwort festgelegt. Die gleich großen Listen geben jeder Sprache je Position
Wahrscheinlichkeit 1/3. Wegen der Disjunktheit sind die Möglichkeiten auch als
ausgegebene Schreibweisen verschieden: vier Wörter liefern gegenüber einer
Einzelquelle Faktor **3⁴ = 81**, also **6,340 Bit** zusätzlich.

Die kombinierte Quelle verschränkt die Listen deterministisch: Deutsch 0,
Englisch 0, Latein 0, Deutsch 1 usw. Damit gibt es keine Sprachblöcke. Dies ist
keine geheime oder zufällige Permutation. Bei gleichverteilter Indexwahl ist
die Reihenfolge entropieneutral; ein zusätzlicher Zufallsgenerator beim Laden
würde Komfort und Sicherheit nicht verbessern. Die sprachliche Zuordnung
bleibt als Flag erhalten. Die konkrete gleichverteilte Auswahl gehört zu Core;
die Crate verdrahtet weder Profile noch GUI außerhalb ihres Schreibbereichs.

Für Profil 1 sind bei vier Wörtern mit festem Trenner tatsächlich
`4 * log2(21390) = 57,539 Bit` verfügbar. Der Sprachgewinn ist darin bereits
enthalten und darf nicht noch einmal addiert werden. Aus 21.000 im Konzept
werden hier reale 21.390 Wörter; weitere Profilregeln berechnet Core getrennt.

## Quellen und Auswahl

**Englisch:** Die drei EFF-Listen von Joseph Bonneau/Electronic Frontier
Foundation bilden den Hauptbestand. Ihre 10.368 Zeilen ergeben nach `a-z`,
3–9 und Deduplizierung 8.301 Wörter. Dazu kommen genau 1.257 ausdrücklich
ausgewählte, gegen die MIT-Wortliste von Wordnik geprüfte kurze Wörter
([Positivliste](english-supplement.txt)). Der vollständige Wordnik-Pool ist
kein freigegebenes Passwortvokabular. 1.226 kategorisierte Sperren entfernen
unter anderem Belastendes, reine Namen, Marken und schwierige Schreibweisen.
Die EFF-Prüfung auf Profanität allein genügt für die strengere Kairos-Auswahl
nicht; beispielsweise bleiben `douche`, `badass` und `cancer` ausgeschlossen.
Die kontextgebundenen Labels `islamist` und `marxism` sind als
Schreibbarkeitsentscheidung für zufällige Passwortausgaben gesperrt, ohne die
bezeichneten Überzeugungen zu bewerten. `faith` bleibt erlaubt.
Nach Sperren entfernt derselbe Krankheits-/Wund-/Verwesungs-/Todeswächter wie
bei den englischen Lateinglossen neun weitere englische Kandidaten. Darunter
sind acht zuvor gewählte Wörter: `deafness`, `dizziness`, `dizzy`, `giddiness`,
`paralyze`, `rot`, `scabbed`, `scabby`; außerdem wird der sonst nachrückende
Verletzungsbegriff `hamstring` gesperrt. Nach 383 deutschen Überschneidungen
bleiben 7.940 Kandidaten.
Die Pipeline bevorzugt kurze Wörter, bei gleicher Länge ordinal, und wählt
7.130. Mittlere Länge: **6,284 Buchstaben**.

**Latein:** Whitaker's WORDS bietet ausreichend lizenzierte Daten. Deshalb
werden Esperanto/Italienisch nicht als Ausweichsprache benötigt. Der feste
Stand enthält 39.335 Datensätze mit Stämmen, Grammatik, Frequenz und englischen
Bedeutungen. Die Pipeline nimmt nur unterstützte nominativische Nomen und
Adjektive, die erste Person Präsens regelmäßiger bzw. deponentischer Verben
sowie unveränderliche Wortarten. Sie fügt ausschließlich die zum erkannten
Muster gehörende Endung an; Stämme werden nicht pauschal als Wörter ausgegeben.
Andere Flexionsmuster und unpersönliche Verben werden ausgelassen. Reine Namen
mit Kennzeichen `N` und großgeschriebene Stämme entfallen bereits im Decoder. Der WORDS-Platzhalter
`zzz` für einen fehlenden Stamm wird vor jeder Endungsbildung abgelehnt.

Es bleiben 11.632 unterschiedliche, passende Kandidaten mit Frequenz A–D.
Upstream bezeichnet A als sehr häufig, B als häufig, C als verbreitet und D
als weniger häufig (oberer Bestand von etwa 20.000). Diese Kennzeichen sind
Wörterbuchangaben, keine aktuelle Häufigkeitsmessung und keine Komfortgarantie.
2.352 versionierte Sperren betreffen Belastendes, Namen sowie Schreibbarkeit;
darunter archaische Formen, häufige `ad-/ac-`- und `i/j`-Doppelvarianten und
auffällige Quelldatenfehler wie `adnililo`. Der begrenzte Decoder würde aus
einem falsch/irregulär klassifizierten Eintrag auch `abnueo` ableiten; diese
Schreibweise ist ausdrücklich gesperrt. Der Decoder ist kein vollständiger
lateinischer Morphologie- oder Rechtschreibprüfer.

Die Durchsicht nutzt Bedeutungen und berücksichtigt auch belastende
Nebenbedeutungen konservativ. Gewöhnliche positive Begriffe wie `pax` (Frieden),
`aqua` (Wasser) und `amor` (Liebe) bleiben trotz einzelner kontextueller
Nebenbedeutungen erlaubt. Weltanschauungen werden nicht pauschal gesperrt.
Ein zusätzlicher deterministischer Glossenwächter entfernt 183 weitere Kandidaten
mit expliziten Markierungen/Bedeutungen wie `(rude)`, Genitalien, Urinieren,
Bordell, Zuhälter, Sodomie oder Einlauf. Er prüft alle berücksichtigten Glossen
eines Wortes einschließlich Nebenbedeutungen; es gibt keine Ausnahmeliste.
`vagina`, `semen` und `genital` sind zusätzlich als auf Deutsch/Englisch
problematische Schreibweisen gesperrt. Dies ist ein konservativer Markerfilter,
kein allgemeiner semantischer Klassifikator.

Die L1-Nachbesserung erweitert ihn um Krankheiten, Wunden, Eiter, Geschwüre,
Seuchen, Juckreiz/Aussatz, Verwesung und Tod einschließlich Flexionsformen,
typischer Symptome und expliziter Bestattungsbegriffe. Alle berücksichtigten
Nebenbedeutungen werden geprüft. Neue Klassen treffen ganze englische Tokens
oder ausdrücklich definierte Wortfamilien; `rotation`, `pestle`, `manger`,
`push`, `skill` und `stitch` bleiben als bloße Teilworttreffer erlaubt.
Konservative Mehrdeutigkeiten und sämtliche 121 entfernten lateinischen
Wörter mit Originalglossen sind im [Abnahmebericht](../curation-review.md)
und [Entfernungsregister](curation-removed-latin.tsv) dokumentiert.

Nach Sperren, Wächter und Überschneidungen stehen 8.895 Kandidaten bereit.
Von Wörtern mit identischer vollständiger erster Glosse bleibt jeweils eine
Schreibweise: höhere Frequenz, dann kürzer, dann ordinal. Das entfernt 889
Varianten; 8.006 Kandidaten bleiben. Die Regel kann vorsorglich auch echte
Synonyme entfernen und erkennt nicht alle linguistischen Varianten.
Abhängige Präfixformen wie `plo` und bekannte irritierende Schreibweisen wie
`arcs`/`apes` sind ausdrücklich gesperrt. Der Sprachflag bezeichnet den
zugewiesenen Wortgebrauch; ein gültiges lateinisches Wort kann zugleich im
gesamten englischen Wörterbuch vorkommen. Ein pauschaler Wordnik-Ausschluss
wird deshalb nicht angewandt; disjunkt bleiben die tatsächlich eingebetteten
Sprachen. Die Auswahl
bevorzugt A vor B vor C vor D, innerhalb der Gruppe kurze Wörter und ordinale
Reihenfolge. Am Größenlimit bevorzugt diese Regel alphabetisch frühe Wörter
gleicher Länge/Frequenz; sie ist kein Repräsentativitätsversprechen für die
Sprache. Englisch hat beispielsweise 508 gewählte Neunbuchstabenwörter von
`abdominal` bis `handbrake`. Diese Auswahlverzerrung des festen Vokabulars ist
öffentlich und verändert die Gleichverteilung innerhalb der Endliste nicht.
Die 7.130 Endwörter haben durchschnittlich **6,744 Buchstaben**.

Alle ausgewählten lateinischen Wörter sind im reproduzierten
[Bedeutungsregister](latin-glosses.tsv) mit Zeilennummer des Originaldatensatzes,
Frequenz und englischer Originalerklärung nachprüfbar. Mehrere Einträge einer
Schreibweise stehen mit ` | ` getrennt in derselben Zeile. Dieses Register
gehört zur Datenprüfung und wird nicht in die Laufzeitquelle eingebettet.

**Zielabweichungen:** Sechs Buchstaben bleiben ein Komfortziel. Die
Abweichungen Deutsch 6,565 und Latein 6,744 sind offengelegt. Die Variantenbereinigung
erhöht den lateinischen Mittelwert gegenüber dem ersten Stand 6,618 leicht;
unterscheidbare Schreibweisen und zuverlässigere Lemmata haben Vorrang vor
einer künstlich auf sechs gedrückten Länge. Der Test sichert maximal 6,8. Latein enthält
auch weniger verbreitete Lemmata; 7.130 einfache lateinische Alltagswörter werden
nicht behauptet. Keine Zusicherung allgemeiner Präfixfreiheit, eindeutiger
Dreizeichenpräfixe, eines Editierabstands oder völliger Abwesenheit jeder
regionalen/kontextuellen Nebenbedeutung. Durch Kombination und Filter gelten
solche Eigenschaften einzelner EFF-Kurzlisten nicht mehr. Wortgrenzen benötigen
Trenner. Gebräuchliche englische Flexionsformen bleiben erlaubt; der deutsche
Endungsfilter wird nicht ungeprüft auf andere Sprachen übertragen.

## Überschneidungen und tatsächliche Größen

Vorrang ist **Deutsch vor Englisch vor Latein**. Gleichlautende Wörter werden
nur der höchstpriorisierten enthaltenen Sprache zugerechnet; Herkunftsgeschichte
oder Lehnwortstatus ändern das nicht. Damit bleiben deutsche API und Daten aus
PR-2 stabil. Die Zahlen beziehen sich auf den kuratierten Pool vor der finalen
Größenbegrenzung, nicht auf die unbereinigten Wörterbücher.

| Schritt | Deutsch | Englisch | Latein |
|---|---:|---:|---:|
| Kuratierter Pool nach Glossenwächter, vor Sprachabzug | 7.130 | 8.323 | 9.097 |
| Wegen Deutsch entfernt | 0 | 383 | 83 |
| Wegen endgültigem Englisch entfernt | 0 | 0 | 119 |
| Verfügbare disjunkte Kandidaten | 7.130 | 7.940 | 8.895 |
| Wegen identischer erster Glosse entfernt | 0 | 0 | 889 |
| Nach Variantenbereinigung verfügbar | 7.130 | 7.940 | 8.006 |
| Wegen gleicher Zielgröße nicht gewählt | 0 | 810 | 876 |
| Endgröße | **7.130** | **7.130** | **7.130** |

Eine bereits bei der Kuratierung ausgeschlossene gemeinsame Schreibweise zählt
nicht noch einmal als Sprachüberschneidung. Deutsch und endgültiges Englisch
sind bereits disjunkt, weshalb die beiden lateinischen Abzüge keine gemeinsame
Schreibweise doppelt zählen. Die gemeinsame Quelle hat insgesamt 21.390 Wörter.

## Reproduzierbare Offline-Pipeline

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-wordlists --example rebuild-languages
cargo run -p kairos-wordlists --example rebuild-languages -- --check
cargo run -p kairos-wordlists --example rebuild-german -- --check
```

`--check` schreibt nichts. Es prüft Originalbytes und Lizenzdateien gegen
[SHA256SUMS](upstream/SHA256SUMS), den unveränderten deutschen Ergebnis-Hash,
Größen, Quellenmitgliedschaft der Ergänzungen, geordnete/eindeutige
Kuratierungseingaben und gültige Ausschlusskategorien. Anschließend vergleicht
es beide erzeugten Listen, ihre separat freigegebenen Ergebnis-Hashes,
[Filterbericht](report.txt) und das Bedeutungsregister mit den gespeicherten
Artefakten. Ungültige CLI-Argumente und zu kleine bereinigte Pools brechen ab.

Eine beabsichtigte Datenänderung: Pipeline neu erzeugen, Wortdiff und Register
prüfen, Zahlen/Statistiken in Doku und Tests anpassen und erst danach die
Ergebnis-Hashes freigeben. Die Pipeline schreibt diese Hashes absichtlich nicht
selbst. Quellenwechsel benötigen neue Provenienz, Revision und Lizenzprüfung.
Gzip spart Platz, seine Containerbytes dürfen variieren; maßgeblich bleiben die
dekomprimierten Originalbytes. Latin-1 der WORDS-Datei wird nur beim Werkzeug
gelesen, Laufzeitlisten sind ASCII mit LF. Es gibt keine Transliteration.

Fünfzehn zusätzliche Integrationstests prüfen vollständige paarweise Disjunktheit,
Endgrößen, Zeichen/Längen/Mittelwerte, Quellen- und Endprüfsummen, Register,
Sprachflags jedes Wortes, vollständige Union, Verschachtelung, Indexgrenzen,
threadsichere Einmalinitialisierung, ausgewählte Decoderbeispiele und
unabhängige redaktionelle Regressionen. Neue Prüfungen sichern insbesondere
fehlende `zzz`-Stämme, alle konkret beanstandeten Wörter, den Glossenwächter
und eindeutige erste Glossen gegen Varianten ab. Zusammen mit PR-2 sind es **26
Integrationstests und zwei Rustdoc-Tests**. Die Tests ersetzen keine vollständige
sprachwissenschaftliche oder semantische Begutachtung.

Originale, Lizenzen, Änderungen und alle Prüfsummen:
[PROVENANCE.md](PROVENANCE.md). Vorbilderbewertung und verworfene Alternativen:
[docs/sources/languages.md](../../../../docs/sources/languages.md).

Für die spätere Binärdistribution [NOTICE.txt](NOTICE.txt) mitliefern. Sie
bündelt die erforderlichen Namensnennungen, Änderungshinweise und kompletten
Lizenzen der englischen und lateinischen Daten. Die GUI-/Release-Integration
muss diese Datei neben die EXE bzw. in deren Lizenzanzeige aufnehmen; diese
Verdrahtung bleibt als Integrationsaufgabe außerhalb dieses Bereichs-PRs offen.
