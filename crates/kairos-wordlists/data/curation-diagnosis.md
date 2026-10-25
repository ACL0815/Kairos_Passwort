# Diagnose und Ursachenfix (Abnahmerunde 2)

## Lead-Entscheid Endstufe

Der Lead hat am 25.10.2026 (Europe/Berlin) die Diagnoseleiter beendet.
Basis bleibt `8696255` mit genau sieben zusätzlich bestätigten Sperren.
Die lokale, unveröffentlichte Stufe 3 in `t3-29ba4794` wird verworfen:
CC-BY-SA-Filterdaten bringen Weitergabepflichten, ungefähr 26 % Übersperrung
und unter der eskalierenden Nebenbedeutungs-Nulltoleranz keinen Abnahmegewinn.
Weder Daten noch Änderungen dieser Stufe werden übernommen; der fremde
Worktree bleibt unverändert.

Der endgültige Standard verlangt eine unabhängige 900er-Stichprobe mit
A-Quote höchstens 0,5 % und ohne A-Fall nach Hauptbedeutung. Alltagssprache
wird nach der wortbezogenen Wörterbuch-/Gebrauchsschwelle beurteilt.
[Verbindliche Rubrik](curation-rubric.md#endgültiger-lead-entscheid-und-abnahme),
[Umsetzung und verbleibendes Risiko](curation-stage-two.md#lead-entscheid-endstufe),
[historische Verifier-Matrix](languages/curation-verifier-matrix.md#matrix).

## Historischer Stufe-1-Bericht

**Historischer Bericht der Stufe 1 bei `691860b`.** Der damalige STOP bleibt
unten unverändert nachvollziehbar. Der aktuelle verbindliche Standard steht
in [curation-rubric.md](curation-rubric.md); Fortsetzung, Entscheidungen und
aktuelle Abnahme in [curation-stage-two.md](curation-stage-two.md).

**Arbeitsstand mit STOP, keine fachliche Abnahme.** Nach vollständiger
Kandidatenklassifikation und einem Korrekturversuch findet die zweite eigene
Abnahmeprobe erneut Sperr-/Zweifelfälle derselben Klassen. Gemäß Auftrag werden
keine weiteren semantischen Korrekturen vorgenommen. Der Stand wird mit Belegen
gepusht; die nächste Diagnoseleiterstufe entscheidet der Lead. Technische
Reproduzierbarkeit und das 14-Wörter-Gate sind grün, die semantische Abnahme nicht.

Ausgangspunkt: PR #7, `ec6a9562dbda9bab3742b6d3f766d8f11d289a6b`,
25.10.2026, Europe/Berlin. Deutsche Daten und deutsche Filterregeln gehören
zu PR #2 und werden hier nicht verändert.

## Fehlerursache mit Gegenbelegen

Der bisherige Ansatz verwechselt einen endlichen lexikalischen Wächter mit
einer Bedeutungsprüfung. Die grünen Reproduktionsprüfungen beweisen, dass
dieselben Regeln dieselben Bytes erzeugen. Sie beweisen keine semantische
Eignung dieser Bytes. Nach zwei fehlgeschlagenen Abnahmen ist eine weitere
Erweiterung um einzelne Fundwörter als alleiniger Fix ausgeschlossen.

1. **Polysemie:** `follis` bedeutet neben Beutel/Blasebalg auch Skrotum;
   `botulus` ist trotz neutralem Wurst-Hauptsinn ausdrücklich als `rude word`
   markiert. Pflanzen- und Materialwörter wie
   `abies`, `fraxinus`, `cornum` oder `cupressus` bezeichnen außerdem Waffen.
   Das harmlose erste Wort einer Glosse genügt nicht als Freigabe.
2. **Nicht abgeschlossene Wortfamilien:** `motio` nennt `shivering, ague`;
   `tardipes` enthält `lame`, während der Wächter nur `lameness` kannte.
   `inustum` bezeichnet `burned parts; burns`. Neue Synonyme und unregelmäßige
   Flexionsformen sind kein durch zusätzliche Teilwörter lösbares Problem.
3. **Markierungsvarianten:** `(rude)` und `(somewhat rude)` wurden erkannt,
   `(rude word)` bei `botulus` und `(sometimes rude)` bei `rima` nicht.
4. **Fehlende Bedeutungsbereiche:** `dagger`/`hasta` (Waffen),
   `bardus`/`morio` (Abwertung), `viduitas` (Witwenschaft/Trauer),
   `hydromel`/`potor` (Met/Trinker) liegen außerhalb des bisherigen
   Krankheits-/Verletzungswächters. Die Forderung war bereits breiter.
5. **Ungeprüftes Nachrücken:** Nach der Löschung wurden die nächsten Wörter
   nach Frequenz/Länge/Ordinalwert gewählt. `tardipes` und `viduitas` rückten
   neu nach. Die Auswahlposition ist kein semantisches Gütesiegel.
6. **Quellumfang:** Der alte Glossenaufbau sammelte nur Datensätze A–D.
   Seltenere Nebenbedeutungen derselben Schreibweise konnten fehlen.
   Die neue Quellbindung umfasst alle decodierbaren Rohdatensätze einer
   Kandidatenschreibweise, sobald mindestens einer die Frequenz A–D hat.
   Beispiel der eigenen Stichprobe: `coactio`, A-D-Hauptsinn Sammlung,
   zusätzliche F-Glosse `disease of animals`.
   Im vollständigen Kandidatenpool betreffen die vormals ausgelassenen
   **528 Nicht-A-D-Datensätze insgesamt 495 Schreibweisen**. Das neue
   Prüfmaterial umfasst für 11.632 LA-Kandidaten insgesamt **13.699
   Rohdatensätze**. Dies sind Abdeckungszahlen, keine Zahl belastender Glossen.

Die Re-Verifier-Matrix und bewerteten Such-TSVs in
`t3-d9ce504b/target/verify/` sowie die erste Abnahme in
`t3-3a556a4a/target/verify/verifier-a.md` wurden als Fehlerbelege gelesen.
Das unabhängige Gate prüft 13 LA-Schreibweisen und EN `dagger`.
Vor dem Neugenerieren selbst reproduziert: unverändertes
`check-independent-findings.ps1`, **Exit 1, findings=14**; alle 14 genannten
Schreibweisen befinden sich im Ausgangsbestand.

## Explizite Rubrik

Jede Kandidatenschreibweise wird einzeln geprüft. Bei LA werden sämtliche
zugeordneten Rohglossen einschließlich Nebenbedeutungen gelesen. Ein Treffer
in einer Bedeutung genügt zur Sperre. Die Klassen sind `Krankheit`
(einschließlich Symptome), `Verletzung` (einschließlich Schmerz),
`Tod-Trauer` (einschließlich Bestattung, Verwaisung, Witwenschaft),
`Gewalt-Waffen` (einschließlich Waffenteile und militärischer Gewalt),
`Sexualitaet` (einschließlich Genitalien, Paarung und explizitem Slang),
`Drogen-Alkohol`, `Abwertung` (einschließlich abwertender Personenbezeichnungen)
und `Vulgaer` (einschließlich Ausscheidungen). **Im Zweifel raus:** Klasse
`Zweifel`, mit konkretem Grund. Mehrere Klassen werden mit Komma getrennt.

Ein bloßes Zeichenfragment ist keine Bedeutung: `grape` enthält keine
Vergewaltigungsbedeutung, `password` keine Schwertbedeutung. Feuer, Wärme,
Materialverarbeitung, medizinische Versorgung, gewöhnliche Schwangerschaft/
Mutterschaft, Liebe und negative Situationen
allein werden nicht automatisch zu Krankheit/Verletzung/Sexualität. Eine
explizite problematische Nebenbedeutung hebt einen neutralen Hauptsinn aber auf.
Konservative Zweifelsperren werden als solche erkennbar gespeichert, nicht als
bewiesene Krankheiten oder Schimpfwörter ausgegeben.

## Eigene Diagnose-Stichprobe mit Abgleich

Seed: `20261025-kairos-causefix-own-v1`. Auswahl getrennt für EN und LA:
`SHA256(UTF8(seed + "|" + language + "|" + word))`, Labels `en` und `la`;
je 200 kleinste Hashes, ohne Wiederholung. Grundgesamtheit ist der vollständige
zulässige Rohkandidatenpool vor redaktionellen Sperren und Wächter, **nicht nur
die bereits eingebettete Liste**: EN 9.558, LA 11.632. Der gesamte unverwendete
Wordnik-Rohbestand ist kein Nachrückerpool; nur die 1.257 expliziten Ergänzungen
gehören zu den englischen Kandidaten.

Alle 400 gezogenen Einträge wurden selbst gelesen und semantisch bewertet.
Die unabhängige Auswahl wurde beim Abgleich nicht verändert. Die Erstbewertung
ergab EN 31 und LA 60 Sperren. Beim anschließenden Bedeutungsabgleich wurden
die vier EN-Nebensinne `dock`, `engaged`, `french`, `spotty` ergänzt;
LA `indutia` (Waffenstillstand) wurde als neutral korrigiert, `perdo` als
Zweifelsfall ergänzt und `matrix` als Zweifel statt gesicherter Sexualbedeutung
gekennzeichnet. Die folgende Tabelle verwendet diese abschließend bewertete
Stichprobe; sie ist keine verblindete unabhängige Zweitklassifikation.
Die individuellen Entscheidungen, Quelltexte, Auswahlhashes und Ergebnisse des
unveränderten alten Rust-Wächters stehen in
[`languages/own-semantic-sample.tsv`](languages/own-semantic-sample.tsv).
Spalten: Wort, Sprache, Entscheidung, Klasse, Begründung, alter Wächter,
Auswahlhash, Quellhash, vollständiger Quelltext.

| Sprache | Stichprobe | Rubriksperren inkl. Zweifel | davon alter Wächter erkannt | verfehlt | Trefferquote unter Rubriksperren |
|---|---:|---:|---:|---:|---:|
| EN | 200 | 35 | 1 | 34 | 2,86 % |
| LA | 200 | 60 | 10 | 50 | 16,67 % |
| zusammen | 400 | 95 | 11 | 84 | 11,58 % |

Das sind gemessene Quoten dieser Stichprobe und dieser konservativen Rubrik,
keine garantierten Populationswerte und keine Schätzung allein eindeutiger
Krankheitsbegriffe. Die alten redaktionellen Sperrdateien wurden bei dieser
isolierten Messung der **Keyword-Recall** absichtlich nicht mitgerechnet.
Schon gesperrte Wörter sind Teil der Stichprobe. Die Werte dürfen daher nicht
als Häufigkeit belastender Wörter in den alten Endlisten gelesen werden.
Auch Zweifelsfälle wie EN `pint` (Maßeinheit/Bierportion) und LA `dirrumpo`
(bersten/zerreißen, unklare Verletzungslesart) zählen sichtbar mit.
Ohne sämtliche als `Zweifel` markierten Fälle bleiben EN **30 Sperren,
1 erkannt, 29 verfehlt (3,33 %)** und LA **54 Sperren, 10 erkannt,
44 verfehlt (18,52 %)**. Der grundlegende Befund hängt somit nicht an den
vorsorglichen Zweifelsentscheidungen.
Die erste isolierte Messung des alten Keyword-Prädikats gegen sämtliche neuen
Rohglossen markierte noch 12 der 60 LA-Sperren (20 %). Im tatsächlich alten
Pipelineumfang A–D entfallen zusätzlich `calcitro` und `coactio`, weil ihre
belastenden Glossen E beziehungsweise F haben. Die Tabelle und die boolesche
Spalte im Einzelregister bilden deshalb den **alten tatsächlichen
Glossen-/Schreibwächter vor redaktionellen Sperren** ab: nur 10 von 60.

## Deterministischer Ursachenfix

Die versionierte Datei `languages/semantic-decisions.tsv` enthält Einzelzeilen
für **jeden** EN-/LA-Kandidaten: Wort, Sprache, `allow`/`block`, Klasse,
Begründung, SHA-256 des geprüften Quelltexts. EN bindet an die Schreibweise,
LA an alle Originalzeilen/Frequenzen/Glossen in ursprünglicher Reihenfolge.
Die Rohquellen sind weiterhin separat durch das vorhandene Hashmanifest fixiert.

Die Pipeline verlangt exakte Abdeckung des gesamten Pools, auch aller Reserven
und bereits anderweitig gesperrter Kandidaten. Unbekannte Wörter, doppelte oder
ungeordnete Zeilen, ungültige Klassen, fehlende Entscheidungen und geänderte
Quellbedeutungen führen zu einem Fehler. Erst danach wirken redaktionelle
Sperren, semantische Sperren, der zusätzliche Keyword-Wächter, Sprachabzug,
Variantenfilter und die deterministische Auswahl. Eine Quellenänderung oder
neue Ergänzung kann somit nicht stillschweigend ungeprüfte Wörter nachrücken.

Der Keyword-Wächter bleibt als Zusatznetz bestehen und erkennt nun auch
`rude word`, `rude term`, `sometimes rude`, `rude expression`, `vulgar`,
`obscene` sowie explizite zuvor übersehene Genitalbezeichnungen.
Er ersetzt keine Einzelentscheidung. Regressionen prüfen alle 14 bekannten
Fälle, jede eingebettete Freigabe samt Quellhash und die Fehlerpfade bei
fehlenden, veränderten oder ungültigen Entscheidungen.

Die semantische Bewertung bleibt eine redaktionelle Entscheidung mit
sprachlicher Unsicherheit. Vollständige Dateiabdeckung ist technisch beweisbar;
die inhaltliche Richtigkeit jeder Entscheidung wird zusätzlich durch eigene
Stichprobe und das unabhängige Fixdiff-Review kontrolliert.

## Gesamtprüfung, Korrekturdurchgang und Stop-Regel

Zwei Read-only-Klassifikatoren prüften je 14.160 Einträge. Hälfte 1:
4.437 EN, 6.214 LA, 3.509 DE; Hälfte 2: 5.121 EN, 5.418 LA, 3.621 DE.
Das ergibt alle **28.320** beauftragten Einträge: **9.558 EN, 11.632 LA,
7.130 DE**. Die erste Hälfte wurde nach einem ehrlichen Teilbericht fortgesetzt;
ein abgeschnittener Ausgabeabschnitt wurde vom selben Kind vollständig
nachgeliefert. Keine weiteren Klassifikationskinder, keine Weiterdelegation,
keine Schreibzugriffe der Kinder. Eingabehashes und Abdeckung stehen in
`languages/review-coverage.tsv`; die mit `research-` beginnenden Berichte
enthalten die gelieferten Sperren und Freigabeerklärungen.

Vier nicht zum Kandidatenpool gehörende Zeilen aus Hälfte 2 wurden nicht
materialisiert (drei davon bezeichnete das Kind selbst als nicht anwendbar).
Die unzulässige Kategorie `Raserei` bei `fanaticus` wurde auf `Zweifel`
normalisiert. `indutia` bleibt wie EN `truce` neutral: ein Waffenstillstand
beendet Gewalt. Bloße Zahlfragmente (`sexennis`, `sextans`), normale Werkzeuge,
medizinische Hilfe und reine Wörterbuchbeispiele begründen allein keine Sperre.

Die erste eigene **Endlistenprobe** (je 100 EN/LA, Seed
`20261025-kairos-causefix-own-final-v1`) fand **10 Sperr-/Zweifelfälle**.
Belege: `languages/own-final-sample.tsv`. Daraufhin wurde genau ein
Korrekturdurchgang durchgeführt: **232 zusätzliche semantische Sperren**
im vollständigen Kandidatenpool, mit individuellen Bedeutungsgründen in
`languages/semantic-correction.tsv`. Die Hilfssuchen lieferten 149 und 206
Kandidaten, deren vollständige Glossen einzeln gelesen wurden; harmlose
Teilwort-/Kontexttreffer wurden nicht automatisch gesperrt. Die Suchmuster
sind kein neuer Pipelinefilter. Beispiele wirklicher Inkonsistenzen waren
`calumnia`/`kalumnia`, `neglegens`/`neclegens`, `demolio`/`demolior` sowie
`com-`/`con-`-Varianten. Normale Bedeutungspaare wie `libella`/EN libel,
`spearmint`/spear, `gallus`/englischer Slang, Wetterkälte, Rechenausdrücke,
Instrumente, Heilung und die Abwesenheit von Gewalt bleiben semantisch neutral.

Die zweite eigene Endlistenprobe (je 100 EN/LA, neuer Seed
`20261025-kairos-causefix-own-final-v2`) findet **erneut 5 Sperr-/Zweifelfälle**:
EN `pouncing`, `bladder`; LA `absurde`, `servula`, `situs`.
Belege samt sämtlichen 200 gelesenen Wörtern/LA-Glossen:
`languages/own-final-sample-v2.tsv`. Die Auswahl benutzt dieselbe
SHA-256-Methode wie die Diagnoseprobe, jetzt auf den tatsächlich eingebetteten
Endlisten. Die 100 kleinsten Hashes je Sprache werden ohne Wiederholung gelesen.
Die TSV-Spalten sind Auswahlhash, Wort, Sprache, vollständiger Quelltext,
Entscheidung, Klasse, Begründung.

**STOP ausgelöst:** Die zweite semantische Probe ist fehlgeschlagen. Die
fünf Wörter werden in diesem Arbeitsstand bewusst nicht durch eine weitere
Reparatur entfernt. Die technische Abdeckung aller Kandidaten und gültige
Quellfingerprints verhindern ungeprüfte Quellenänderungen, können aber falsche
inhaltliche Freigaben nicht erkennen. Die Klassifikation großer Hälften mit
impliziter Freigabe aller nicht genannten Wörter war trotz vollständiger
Lesebestätigung kein ausreichender Qualitätsnachweis. Eine weitere Leiterstufe
muss die Granularität und Absicherung der Bedeutungsentscheidungen beurteilen;
ein weiteres Vergrößern einer Keywordliste löst dieses Problem nicht.

## Gesicherter Daten- und Prüfstand

Die Entscheidungsdatei hat **21.190** Zeilen: EN 9.558 (1.408 block),
LA 11.632 (2.952 block). Nach bestehenden Regeln, semantischen Sperren,
Zusatzwächter, Sprachabzug und Variantenfilter stehen EN 7.214 und LA 6.966
Kandidaten zur Verfügung. EN und LA werden auf das Minimum **6.966** gekürzt;
DE bleibt **7.130**, `combined()` hat **21.062** disjunkte Schreibweisen.
Der lateinische Sprachabzug bleibt konservativ gegen die ursprüngliche
EN-Vorauswahl von 7.130 wirksam, auch nach der Größenkürzung; dadurch werden
keine zuvor ausgeschlossenen Schreibweisen wiederhergestellt. Maximale mögliche
Kardinalität wird damit nicht beansprucht.

Gegenüber `ec6a956`: EN **687 entfernt, 523 hinzugefügt**; LA **974 entfernt,
810 hinzugefügt**; DE **0 Änderungen**. Mittelwerte EN **6,541**, LA **6,990**,
DE unverändert **6,565**. Die höhere Länge ist eine Konsequenz der strengeren
Kuratierung; die Tests erlauben EN höchstens 6,6 und LA höchstens 7,0.
Die alten Komfortgrenzen 6,5/6,8 wurden offengelegt angepasst. Vier Wörter aus
der Vereinigung ergeben **57,449 Bit** bei festem Trenner; der Vergleichsfaktor
zur deutschen Einzelquelle ist **76,145**, nicht mehr 81.

**DE nur gemeldet:** `languages/de-findings-report.tsv` enthält **299**
gemeldete Sperr-/Zweifelkandidaten zur Prüfung auf PR #2, beispielsweise
`abzug` (Waffenabzug), `abreibung` (Prügel), `akt` (Geschlechtsakt),
`anstecken` (Infektion). Das sind redaktionelle Meldungen einschließlich
Grenzfällen, keine Behauptung von 299 unabhängig gesicherten harten Verstößen.
Deutsche Listen, Hashes, Quellen, Sperren, Ergänzungen, Pipeline und Tests sind
gegenüber dem Ausgangshead unverändert.

Am gesicherten Arbeitsstand: `cargo fmt --check` **0**;
`cargo clippy --workspace --all-targets -- -D warnings` **0**;
`cargo test --workspace` **0**, **29 Integrationstests + 2 Doctests**;
`rebuild-german -- --check` **0**; `rebuild-languages -- --check` **0**;
unverändertes Re-Verifier-`check-independent-findings.ps1` **0**, `findings=0`.
Die eigene semantische Abnahme bleibt **FAIL**. Ausführliche Befehlslogs liegen
im eigenen Worktree unter `target/semantic-validation/`.

## Einmaliges Fixdiff-Review und Übergabe

Das vorgeschriebene lesende Review mit Claude Opus 5.5
(`effort=medium`, `fastMode=false`) urteilt **nicht abnahmefähig**.
Es bestätigt die fünf offenen Restfälle und die Ursache der unzureichenden
Freigaben: 16.829 von 16.830 `allow`-Entscheidungen haben nur einen Standardtext;
für EN ist lediglich die Schreibweise, keine Bedeutungsquelle gebunden.
Die Sperrbegründung bei `generalis` ist nicht durch die Rohglosse gedeckt.
Diese Befunde bleiben aufgrund der Stop-Regel offen; ein weiterer Versuch
wird nicht begonnen. Der Berichtsfehler, dass die lokale `summary.json` nur
die sechs grünen technischen Prüfungen enthielt, wurde um das eigene Gate
mit Exit 1 ergänzt.

Die zusätzliche Review-Stichprobe umfasst 100 EN und 100 LA mit Seed
`kairos-opus-fixdiff-review-2026-10-25`. Der Reviewer meldet `subalaris` als
Zweifel und `bullseye` als Grenzfall. Der Waffensinn von `subalaris` ist wegen
des anatomischen Kontexts und des gesonderten Eintrags `subalare` nicht sicher
belegt. Diese Hinweise werden zur Klärung übergeben, nicht als bestätigte
zusätzliche Verstöße gezählt. Proben verschiedener Listenstände werden nicht
zu einer Gesamtquote oder einer Hochrechnung verbleibender Wörter kombiniert.
Vollständiger Review-Text und Umgang mit allen Befunden:
[fixdiff-review.md](languages/fixdiff-review.md).
