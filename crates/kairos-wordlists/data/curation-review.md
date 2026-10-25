# Abnahme-Nachbesserung (Verifier A)

Prüfung am 25.10.2026, Europe/Berlin. Ausgangsstand Deutsch: `f35f842`.
Alle Änderungen erfolgen in den Pipeline-Eingaben; `german.txt` wurde mit
`rebuild-german` neu erzeugt. Rohdaten und deren Prüfsummen bleiben gleich.

## D1: Alkohol und Rauschmittel im deutschen Vollbestand

Der vollständige Bestand von 7.134 Wörtern wurde mit zwei überlappenden,
großzügigen Begriffsmengen nach Alkohol, Getränken, Drogen, Konsum und
Rausch durchsucht. Dazu gehörten unter anderem alkohol/bier/wein/schnaps,
whisk/wodka/vodka/sekt/likoer/gin/rum/punsch/cider/cognac/brandy/drink,
cocktail/aperitif/absinth/tequila, tabak/zigarr/zigaret/rausch/drog/opium,
heroin/kokain/cannab/hasch/hanf/marihua/morph/nikotin/kiff/joint/ecstasy,
speed/crack/trip/pils/bock/malz/hopfen/ethanol/spirit/brau/fusel/promille,
bowle/suff/sauf/sucht/sherry/prost/rauch/pfeife/trink. Teilworttreffer
wurden redaktionell beurteilt, nicht automatisch verworfen.

| Treffer | Entscheidung | Begründung |
|---|---|---|
| `drink` | sperren | D1; alkoholassoziiertes Mischgetränk, neutrale Nebenlesart überwiegt die Regel nicht |
| `bowle` | sperren | gleichartiger alkoholassoziierter Getränkebegriff |
| `hanf` | sperren | vorsorglicher Ausschluss der Pflanze mit unmittelbarer Rauschmittelassoziation |
| `prost` | sperren | unmittelbar mit Alkoholkonsum verbundener Trinkspruch |
| `hopfen`, `malz` | behalten | landwirtschaftliche Pflanze bzw. Lebensmittelzutat, selbst kein Rauschmittel |
| `bock`, `steinbock`, `pfeife`, `rauch` | behalten | gewöhnliche Tier-, Instrument-/Gegenstands- bzw. Verbrennungsbedeutung |
| `trinken`, `trip`, `kurztrip` | behalten | allgemeine Flüssigkeitsaufnahme bzw. Reise, kein bestimmtes Rauschmittel |
| `blau`, `stoff`, `ginseng`, `matetee`, `klebstoff` | behalten | gewöhnliche Farbe, Material, Pflanze, Getränk bzw. Werkstoff; keine Sperre allein wegen möglicher Konsum- oder Slangbedeutung |
| übrige Teilworttreffer, z. B. `gingen`, `forum`, `insekt`, `export`, `sehnsucht` | behalten | zufällige Zeichenfolgen oder neutrale Bedeutung |

Vier neue Einträge in `rejected.tsv` unter `belastend`; die drei betroffenen
enz-Ergänzungen wurden zugleich aus `supplement.txt` entfernt, weil die
Pipeline gesperrte Ergänzungen als widersprüchliche Eingabe ablehnt.
Regressionen prüfen die vier Sperren und sieben neutrale Vergleichswörter.

Ergebnis: **7.130** Wörter (−4), **1.306** Sperren, davon **782** in der
dys2p-Basis. Nach Kuratierung 6.085 Basiswörter plus 1.966 Ergänzungen;
921 Endungsvarianten entfallen. Finale Herkunft: 5.453 Basiswörter und
1.677 Ergänzungen. Die vollständige Längenstatistik und der freigegebene
SHA-256 stehen im README und in `german.sha256`.

## L1: Krankheit, Wunde, Eiter, Verwesung und Tod

Ausgangsstand Englisch/Latein: `a158b94`. Zuerst wurde D1 aus #2 übernommen
(`4b10929`); die deutsche Liste bleibt danach gegenüber der neuen Stackbasis
bytegleich. Die EN-/LA-Pipeline leitet ihre gemeinsame Zielgröße aus dieser
deutschen Liste ab: **je 7.130**, zusammen **21.390**, paarweise disjunkt.

Ursache: Der bisherige Glossenwächter prüfte überwiegend sexuelle/vulgäre
Marker. Die Quelle und die erzeugten Glossen enthielten weiterhin explizite
Krankheits-/Wundbedeutungen. `sensitive_gloss` berücksichtigt jetzt zusätzlich
gemeinsame englische Klassen für Krankheit, Wunde, Eiter/Geschwür, Seuche,
Juckreiz/Aussatz, Verwesung und Tod. Die Klassen umfassen Flexionen sowie
ergänzende klinische Begriffe, Symptome und Bestattung. Sie werden auch auf
englische Kandidaten angewandt, jeweils **vor** Sprachabzug und Größenwahl.
Jede berücksichtigte lateinische Nebenbedeutung kann einen Eintrag sperren.

Die Volltextprüfung umfasste den gesamten erzeugten Glossenbestand und
englischen Pool, mit den geforderten Markern disease/sick/plague/pest/ulcer,
sore/wound/pus/suppurate/fester/itch/mange/scab/leprosy, rot/putrid/corpse,
death/die/kill und ergänzenden Synonymen. Zusätzliche Durchsicht erfasste
unter anderem hemorrhoids, morbid, inflammation, paralysis, tetanus, gout,
catarrh, prolapse, bruising, swelling und burial. Neue Nachrücker wurden
mitgeprüft; `hamstring` ist deshalb als Verletzungsbegriff ebenfalls erfasst.
Das Verfahren ist ein dokumentierter Markerfilter mit redaktioneller
Kontrolle, keine vollständige semantische Freigabe aller Nebenbedeutungen.

| Befund/Treffer | Entscheidung |
|---|---|
| `scabies` / `itch, mange`, `ulcero` / `cause to fester`, `suppuro` / `suppurate, fester under the surface` | durch Klassenwächter entfernen; Tests lesen zusätzlich die Originaldatensätze aus der gepinnten Quelle |
| `pusula`, `putesco`, `putidus`, `cariosus`, `marcidus`, `tabidus` | weitere Wund-/Eiter-/Verwesungsbedeutungen entfernen |
| `ficus` (Feige/Hämorrhoiden), `robur` (Eiche/Tetanus), `capitulum` (Kapitel/Hämorrhoiden) | trotz neutraler Hauptbedeutung konservativ entfernen; alle berücksichtigten Glossen gelten |
| `indies`, `nefastus`, `nundina` mit lateinischem `dies` in englischen Erklärungen | bewertete Fehlpositive des englischen die/dies-Markers; vorsorglich ausgeschlossen, keine per-Lemma-Ausnahmeliste |
| `bury` bei z. B. `defodio`/`pono`; `swelling`/`swollen` bei Wasser, Zunahme oder Schwangerschaft | bewertete neutrale Mehrdeutigkeiten; konservativer Ausschluss zugunsten einer einfachen, nachvollziehbaren Regel; ausreichend Restpool |
| `dies` als lateinisches Endwort (Tag) | bleibt: nur die Bedeutungen werden als Englisch geprüft, nicht die lateinische Schreibweise |
| `rotation`, `rotate`, `rotunda`, `pestle`, `manger`, `push`, `octopus`, `skill`, `stitch` | bleiben: keine beliebige Teilwortsuche für die neuen Klassen |
| `boil` (Kochen), `inflamed with love`, `grave, serious`, `by decree of fate` | neutrale/allgemeine Bedeutungen bleiben; die expliziten klinischen Marker werden getrennt erfasst |
| `antivirus`, `germless`, `germproof`, `undying`, `immortal`, `unwound` | neutrale Schutz-, Material- oder übertragene Bedeutungen bleiben |
| `tinnitus` mit Originalglosse `ringing, clanging, jangling`; `urinator` mit `diver` | lateinische Quellenbedeutung maßgeblich; heutige klinische Assoziation allein wird nicht als englische Glosse interpretiert |

**Englisch:** Aus der bisherigen Endliste entfallen genau **8** Wörter:
`deafness`, `dizziness`, `dizzy`, `giddiness`, `paralyze`, `rot`, `scabbed`,
`scabby`. Neu gewählt: `gumminess`, `habitable`, `hamburger`, `handbrake`.
Der Wächter entfernt **9** Kandidaten aus dem Gesamtpool, einschließlich
`hamstring`; es bleiben nach deutschem Sprachabzug **7.940**, davon werden
**810** wegen der gemeinsamen Zielgröße nicht gewählt.

**Latein:** Genau **121** bisherige Endwörter entfallen; **117** Wörter aus
dem verbleibenden Pool rücken deterministisch nach. Die vollständigen
Entfernungen samt alten Originalglossen, Frequenzen und Quellzeilen stehen
in [curation-removed-latin.tsv](languages/curation-removed-latin.tsv).
Das aktuelle [Bedeutungsregister](languages/latin-glosses.tsv) belegt auch
die Nachrücker. Der Glossen-/Schreibwächter entfernt nach den bestehenden
2.352 redaktionellen Sperren **183** Kandidaten (vorher 35). Nach Sprachabzug
bleiben **8.895**, nach 889 identischen Erstglossen **8.006**; davon werden
**876** wegen der Zielgröße nicht gewählt. Lizenzen und Rohquellen sind
unverändert, NOTICE benennt die zusätzliche Kuratierung.

Regressionen prüfen alle verlangten Klassen und Flexionen, neutrale
Teilworttreffer, die Originaldatensätze der drei L1-Wörter, alle gewählten
englischen Wörter und sämtliche gespeicherten lateinischen Bedeutungen.

## Lokale Abnahme am 25.10.2026

Windows mit der vorgeschriebenen `Enter-KairosBuildEnv.ps1`-Umgebung.

| Befehl | #2 nach D1 | #7 nach L1 |
|---|---:|---:|
| `cargo fmt --check` | 0 | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 0 |
| `cargo test --workspace` | 0; 11 Integrationstests + 1 Doctest | 0; 26 Integrationstests + 2 Doctests |
| `cargo run -p kairos-wordlists --example rebuild-german -- --check` | 0 | 0 |
| `cargo run -p kairos-wordlists --example rebuild-languages -- --check` | nicht vorhanden | 0 |

Der ursprüngliche Verifier-Check `check-curation.ps1` meldet nach der
Nachbesserung Exit 0 und keine der drei L1-Schreibweisen mehr im Endbestand.
Ein Zwischenlauf fand noch die alte kombinierte Wortzahl im Doctest;
nach Aktualisierung bestehen beide Doctests. Keine wiederholte Verletzung
dieses Sollverhaltens nach dem Fix.
