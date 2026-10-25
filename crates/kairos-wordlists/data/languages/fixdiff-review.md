# Fixdiff-Review ENLA-Semantikkuration (gegen `ec6a956`)

## Urteil

**Nicht abnahmefähig.** Die Technik ist sauber: Entscheidungen werden deterministisch und quellgebunden konsumiert, und Lücken brechen den Build ab. Die semantische Richtigkeit ist aber weiter nicht belegt. Die bekannte Stop-Regel greift zu Recht: Die 5 bekannten Restfälle sind noch eingebettet. Meine eigene Stichprobe hat einen weiteren Zweifelfall gefunden und einen Grenzfall der Rubrik. Eine fachliche Abnahme spreche ich ausdrücklich nicht aus.

## Findings nach Schwere

**1. Blocker: Die 5 bekannten Restfälle sind weiter erlaubt und eingebettet.**
- Betroffen sind `pouncing`, `bladder`, `absurde`, `servula` und `situs`. Alle stehen in `data/languages/semantic-decisions.tsv` noch als `allow neutral`, z. B. `pouncing` in Zeile 6057.
- Das Gate `own-semantic-gate.log` endet mit Exit 1 und findings=5.
- `target/semantic-validation/summary.json` führt dieses Gate nicht auf. Die Aussage „alle Exit 0“ stimmt nur für die dort gelisteten Prüfungen.

**2. Hoch: Das implizite `allow` ist nicht belegt.**
- Alle 16.829 `allow`-Zeilen haben wörtlich dieselbe Begründung: „Semantische Einzelprüfung … keine Rubrikklasse festgestellt“.
- Eine Freigabe heißt damit nur, dass der Klassifikator geschwiegen hat. Eine wortbezogene Begründung gibt es nicht.
- Bei Englisch ist der Fingerprint nur `sha256(word)` (`tools/languages.rs:624`). Er bindet also keine Bedeutung. Für EN gibt es im Repo keine Glossenquelle, gegen die sich ein Urteil prüfen ließe.
- „Quellgebunden“ heißt bei EN deshalb nur „Wort unverändert“.

**3. Mittel: Sperrbegründungen sind teils nicht durch die Glosse gedeckt.**
- `la/generalis` ist gesperrt mit „Militärischer General“ (`semantic-decisions.tsv:14648`). Die Rohglosse in DICTLINE lautet nur „general, generic; shared by/common to a class/kind; of the nature of a thing“. Ein militärischer Sinn kommt darin nicht vor; die Begründung ist erfunden.
- Für die Sicherheit ist das harmlos, denn die Sperre schadet nicht. Es zeigt aber, dass die Urteile der Klassifikationskinder nicht mechanisch an die Glosse gebunden sind. Dieselbe Unzuverlässigkeit trifft auch die Freigaben (siehe Finding 2).

**4. Mittel: Fehlpositive und unklare Rubrikgrenzen kosten Poolgröße und Symmetrie.**
- **Schilde, Gewalt-Waffen:** rund 25 Schild-Wörter, darunter `scutum`, `clipeus`, `parma`, `umbo` und `en/shield`. Defensive Ausrüstung ist nicht automatisch belastend.
- **Militär-Nebensinn:** `testudo` (Schildkröte, Schildkrötenformation), `ductor` („leader, commander“).
- **Seltene Nebenglosse:** Hier wurde wegen eines seltenen Nebensinns gesperrt, obwohl die Hauptbedeutung neutral ist:
  - `patientia` (Hauptsinn „patience“, Zeile 17437)
  - `condicio` (Hauptsinn „agreement/terms“, Nebensinn „mistress“, Frequenz C)
  - `lustro` (Hauptsinn „purify/review“)
  - `capitulum` (Hauptsinn „chapter“)
  - `plaudo` („applaud“)
  - `virilis`
- **Englische Alltagswörter:** `en/dense`, `gone`, `survivor`, `sock`, `blew`.
- Nach „im Zweifel raus“ sind diese Sperren formal vertretbar. Sie drücken EN/LA aber auf 6.966, also unter die 7.130 von DE. Damit entfällt der Faktor 81 (jetzt 76,145), und die Sprachwahrscheinlichkeit ist nicht mehr 1/3. Die Rubrik legt die Grenzen nicht fest, etwa für Schutzausrüstung, Militärgeschichte, Sklaverei oder Kot/Schimmel.

**5. Niedrig: Kleinere technische Punkte.**
- **Tests gelockert:** Die Mittelwert-Schwellen wurden angehoben, von 6.5 auf 6.6 und von 6.8 auf 7.0 (`tests/languages.rs:20,26`). Die festen Größen 7.130 und 21.390 wurden durch tautologische Summen ersetzt (auch im Doctest in `src/lib.rs`). Größenänderungen fallen jetzt nur noch über die `.sha256`-Dateien auf.
- **Überausschluss:** Die Latein-Overlap-Prüfung läuft gegen die alte EN-Vorauswahl mit 7.130 Wörtern, nicht gegen die finalen 6.966 (`tools/languages.rs:752ff`). Das ist dokumentiert, deterministisch und harmlos.
- **Coverage-Nachweis:** Die Review-Eingaben `review-input-1.tsv` und `review-input-2.tsv` sind nur als Hash in `review-coverage.tsv` versioniert. `tools/prepare-review.ps1` kann sie neu erzeugen, schreibt dabei aber Dateien.
- **Repo-Hygiene:** Etwa 4.500 Zeilen Rechercheprotokolle (`research-*.md`) liegen unter `data/`.
- **Grenze der Quelle:** Whitaker deckt nur die eigenen Glossen ab. `tempto` erscheint ohne den klassischen Sinn „angreifen“. `subalare` (N 3 4) wird nicht in die Glossenunion von `subalaris` aufgenommen; die Schreibweise ist korrekt getrennt.

**Positiv geprüft:**
- `semantic_blocks` (`tools/languages.rs:111`) ist streng: sortierte und eindeutige Schlüssel, Enum-Prüfung, 64-stelliger Hex-Fingerprint, `allow` nur mit `neutral`, exakte Abdeckung (`seen == sources`).
- Unbekannte, fehlende oder geänderte Einträge führen zum Abbruch. Der Negativtest deckt alle diese Fälle ab.
- Für LA werden jetzt alle Frequenzen A–F in den Fingerprint gehasht. Damit ist die Ursache „seltener Nebensinn ignoriert“ für LA technisch behoben.

## Eigene Stichprobe

**Methode:**
- Seed: `kairos-opus-fixdiff-review-2026-10-25`
- Schlüssel je Wort: `SHA256(UTF8("<seed>|<en|la>|<wort>"))` als Hex in Kleinbuchstaben.
- Je Sprache aufsteigend nach Schlüssel sortiert, die ersten 100 genommen.
- Quelle sind die eingebetteten Listen `english.txt` und `latin.txt`.
- Für alle 100 LA-Wörter habe ich die vollständigen Zeilen aus `latin-glosses.tsv` gelesen. Auffällige Fälle habe ich gegen `DICTLINE.GEN.gz` geprüft.

**EN (100):** otherwise, shown, plethora, dozen, suit, repost, skincare, dislodge, debating, octane, authentic, mushily, gently, decline, flap, dotted, bring, broadly, primary, linoleum, expensive, subgroup, news, repacking, seismic, proximity, wobble, freemason, overtake, roamer, handled, short, silver, handprint, amicably, alongside, embody, endorse, violet, jockey, faced, asleep, nursing, married, overreact, appendage, lapel, booth, dubbed, clasp, amber, launch, dare, watch, renewable, mare, backlog, baggy, reusable, spectator, kofta, light, bullseye, tactful, loan, garnet, clanking, linguini, engross, gumming, gig, tapestry, emote, errant, deviation, just, clamor, recliner, activist, handler, bytes, muon, crayfish, recoup, unsorted, footing, upward, another, proving, grandson, observer, splotchy, fastball, path, overcast, baker, revert, sixties, spine, overgrown

**LA (100):** penetrale, uncia, semis, infreno, redimo, regificus, subtexo, conclusus, jampridem, feriatus, allego, recenseo, rigidus, passim, inferne, scisma, relucesco, quiete, promissor, siliqua, buteo, decoctio, subalaris, alacer, tempto, rectitudo, edictum, ortus, bigatus, sidereus, vitaliter, theoricus, animosus, unguis, rado, neptis, perpluo, verbosus, dependeo, oboediens, illinio, sifo, sirpo, recutio, pererro, assido, tributus, divisus, prorepo, incanus, obtorqueo, reverens, salus, solito, volans, incus, ammurmuro, pecuarius, excusatio, devolo, textum, vatis, platea, specialis, delineo, minuo, ammodum, acquiro, marinus, mirator, juxtim, delectio, zizania, diocesis, porcellus, conventus, huc, fideliter, sitiens, sartor, rubesco, altare, trivius, prope, sponsus, perquam, procurvus, jamdudum, beatifico, cuma, statura, taxus, monacha, valor, evidenter, tono, praealtus, prompte, qualitas, poeta

**Funde:**

| Wort | Sprache | Glosse | Klasse | Einordnung |
|---|---|---|---|---|
| `subalaris` | LA | Zeile 36006 D: „arm-carrying; carrying/stuck under the arm“ (`latin-glosses.tsv:6145`, Entscheidung `allow` in Zeile 19899) | Gewalt-Waffen, Zweifel | „arm-carrying“ lässt sich als „waffentragend“ lesen. Klassisch ist auch „telum subalare“ belegt, eine unter dem Arm verborgene Waffe (Nepos). Bei strenger Regel: raus. |
| `bullseye` | EN | Zentrum einer Zielscheibe | Gewalt-Waffen, Grenzfall | Bezug zu Schießen und Bogenschießen. Kein klarer Fund, aber inkonsistent: Schilde wurden gesperrt, dies nicht (Zeile 1052). |

Bewusst als neutral gewertet habe ich unter anderem:
- `nursing`: Pflege, Stillen
- `spine`, `appendage`, `incus`, `unguis`: normale Anatomie
- `redimo`: „ransom“ als Freikauf
- `allego`: „suborn“, kein Rubrikfall
- `altare`: Brandopfer, Religion
- `freemason`: Weltanschauung
- `seismic`, `taxus`, `zizania`

Ergebnis: 1 klarer Zweifelfall und 1 Grenzfall auf 200. Mit den früheren Stichproben (10/200, dann 5/200) liegt die Restquote grob bei 0,5–3 %. Das wären etwa 70–400 Wörter im ENLA-Pool.

## Determinismus und Tests

- Konsum und Prüfung der Entscheidungen sind deterministisch und fail-closed. Die drei neuen Tests passen dazu.
- Der Test `every_embedded_english_and_latin_word_has_a_source_bound_semantic_approval` (`tests/languages.rs:487`) beweist nur, dass die Pipeline alle Wörter abdeckt. Ob die Entscheidungen inhaltlich stimmen, beweist er nicht.
- `check-semantic-sample.ps1` misst nur die festen 200 Wörter aus `own-final-sample-v2.tsv`. Das Ergebnis lässt sich nicht verallgemeinern.
- Ich habe nichts gebaut und keine Datei geändert.

## Verbleibende Risiken

- Implizites `allow` heißt nur, dass der Klassifikator nichts gefunden hat. Für EN gibt es dazu keine Bedeutungsquelle.
- Die Rubrik lässt Grenzen offen: Schutzausrüstung, Militär, Sklaverei, Körperfunktionen, Schimmel/Verfall, „absurd“ als Abwertung. Dadurch bewerten Prüfer gleiche Fälle unterschiedlich (Schild gesperrt, Zielscheibe erlaubt).
- Begründungen können erfunden sein (`generalis`).
- Eine Stichprobe kann eine Restquote nicht auf null bringen. Selbst 0/200 begrenzt die Quote nur auf etwa 1,5 % (95 %), also rund 100 Wörter.

## Nächster Ansatz für den Lead

1. **Rubrik schärfen, bevor neu klassifiziert wird.** Je Klasse eine verbindliche Grenzliste mit Positiv- und Negativbeispielen. Darin ausdrücklich regeln:
   - Schilde, Rüstung und Militärämter
   - Sklaverei und Dienerschaft
   - Ausscheidungsorgane
   - Schimmel und Verfall
   - „absurd“ und „töricht“
   - Zielsport
2. **Bei EN eine Bedeutungsquelle einbinden** (WordNet oder Wiktionary, alle Senses). Den Fingerprint über diese Senses bilden, nicht über das Wort.
3. **Zwei unabhängige Klassifikationen pro Eintrag.** `allow` nur, wenn beide freigeben. Jede Abweichung wird `block/Zweifel`.
4. **Begründungen mechanisch prüfen.** Bei LA muss eine Sperrbegründung ein Glossenfragment zitieren, sonst wird sie zur Nachprüfung markiert. Freigaben brauchen eine kurze Begründung je Wort statt eines Standardtexts.
5. **Danach die Poolgröße bewusst entscheiden.** Entweder EN/LA < DE und den Faktor < 81 akzeptieren, oder neue neutrale Quellen nachziehen. Erst dann eine frische Stichprobe ziehen mit Ziel 0/200 und die Grenze von etwa 1,5 % offen ausweisen.
## Umgang mit dem Review durch den Implementierungs-Worker

Das einmalige Review lief mit `claudeAgent/claude-opus-5-5`,
`effort=medium`, `fastMode=false`, ausschließlich lesend und ohne
Weiterdelegation. Der vorstehende Text ist das unveränderte fachliche Urteil
des Review-Kinds. Die folgende Einordnung stammt vom Implementierungs-Worker.

**Übernommen:** Keine fachliche Abnahme. Die fünf eigenen Restfälle bleiben
offene Blocker nach der strengen Zweifelsregel. Die technische Vollständigkeit
der Entscheidungsdatei beweist keine richtige Semantik. Die unzureichend
belegten Freigaben, die fehlende englische Bedeutungsquelle und die nicht durch
die Rohglosse gedeckte Begründung bei `generalis` werden an die nächste
Diagnoseleiterstufe übergeben. Präzisierung der Zahl: Insgesamt gibt es 16.830
Freigaben; 16.829 verwenden denselben Standardtext, `indutia` hat eine eigene
Begründung. Das ändert den inhaltlichen Einwand nicht.

**Berichtsfehler behoben:** Die lokale maschinenlesbare
`target/semantic-validation/summary.json` enthält jetzt auch das eigene Gate
mit Exit 1. Die versionierten Diagnose- und Validierungsberichte unterschieden
bereits vor dem Review zwischen technischen grünen Prüfungen und gescheiterter
semantischer Abnahme. Alle Aussagen über den Endstand müssen beides nennen.

**Rubrikgrenzen an den Lead:** Defensive Ausrüstung, Militärämter, Zielsport,
Dienerschaft/Sklaverei und Körperfunktionen benötigen verbindliche Grenzen.
Seltene Nebenbedeutungen auszuschließen entspricht dem Auftrag; ihre Seltenheit
allein ist kein Grund, sie wieder freizugeben. Konservative Fehlpositive und
die dokumentierten Größen- und Längenänderungen bleiben überprüfbar bestehen.

**Zusätzliche Stichprobenhinweise nicht ungeprüft übernommen:** `bullseye` ist
ein Grenzfall für Zielsport. Bei `subalaris` sprechen das singuläre englische
`arm` und der Kontext `under the arm` zunächst für den anatomischen Arm. Das
im Review angeführte Substantiv `subalare` ist laut Review selbst ein eigener
Wörterbucheintrag. Ein Waffensinn von `subalaris` ist damit nicht sicher
nachgewiesen und muss fachlich geklärt werden. Beide Hinweise bleiben sichtbar,
ohne sie als bestätigte zusätzliche Verstöße auszugeben.

Die im Review genannten Hochrechnungen von 70 bis 400 Restwörtern werden nicht
als belastbare Schätzung übernommen: Die eigenen Proben betreffen verschiedene
Listenstände und enthalten redaktionelle Zweifelsfälle. Beobachtete Einzelfunde
und Probenquoten werden berichtet, aber nicht zu einer aktuellen Gesamtquote
zusammengezogen.

**Stop-Regel befolgt:** Nach dem bereits fehlgeschlagenen Korrekturversuch
erfolgen keine weiteren semantischen Änderungen und keine weitere Review-Runde.
Der gesicherte Stand wird mit offenen Befunden gepusht. Die nächste
Diagnoseleiterstufe entscheidet der Lead.
