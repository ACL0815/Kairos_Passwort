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
