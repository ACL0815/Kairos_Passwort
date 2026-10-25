# Lesende LA-Klassifikation Stufe 2

Modell codex/gpt-6.1-sol, reasoningEffort=high, serviceTier=priority.
Dies sind die Vorschläge des Kindes; die übernehmende Adjudikation steht in
stage2-decisions.tsv. A-nahe polyseme Vorschläge werden dort von gesicherten
A-Fällen unterschieden.

Vollständig gelesen: **8.680 eindeutige, zu Prüfungsbeginn erlaubte LA-Wörter mit sämtlichen 9.796 Rohglossen** — davon **6.966 Endwörter und 1.714 Reserven**. Die 2.952 damals bereits gesperrten Kandidaten wurden nicht wieder geöffnet. Keine Stichprobe, keine Weiterdelegation, keine Datei- oder Git-/PR-Mutation.

Die Lesung erfolgte fortlaufend in Blöcken von überwiegend 300 Wörtern:

- `review-input-1.tsv`: 4.434 erlaubte LA-Wörter, 5.093 Rohglossen; `abacus` bis `jungo`, Dateizeilen 5–14.160.
- `review-input-2.tsv`: 4.246 erlaubte LA-Wörter, 4.703 Rohglossen; `junior` bis `zothecula`, Dateizeilen 5–14.035.
- Alle 11.632 LA-Kandidaten technisch zugeordnet; sämtliche Rohglossen-Quellhashes stimmen. Auch die sechsten Spalten der abschließenden LA-Entscheidungen stimmen mit den Inputs überein.

Während der Lesung änderte sich der Bestand parallel: **`servula` wurde gesperrt; `servula` und `nexum` wurden aus der Endliste entfernt.** Abschlussstand: 6.964 Endwörter, 8.679 erlaubte Kandidaten, 2.953 Sperren. Die abschließenden 6.964 Zeilen von `latin-glosses.tsv` stimmen vollständig mit den gelesenen Rohglossen überein.

SHA-256, jeweils vollständige Datei:

```text
review-input-1.tsv unverändert
bcdc12c89c58941e8477de06e4e07cd634c84920c0d819526b59a8d797f15a80
review-input-2.tsv unverändert
385f6c740dd0e159c4c5f15c43a554fe7d538737d1030f88c80a8efd41866a9b

latin.txt Anfang
95d8b89f37c794ce6c2ea446f59551aff7cb08ce98e087fa6269630d0e0d0987
latin.txt Abschluss
ff8630b8a52cba6912c1f852d630802ba28fbac9239b07afab5b2d816b235123
latin-glosses.tsv Anfang
835c166ea7d5907b807c31e6a08c45e797ace1e9a5676593e6eb7494859c83a8
latin-glosses.tsv Abschluss
3aeeb028a022e8a73ba98221e272ebfd1d0c3eb35a3c164d705d9b92d36b6848
semantic-decisions.tsv Anfang
e2b7f51f134ce27745cf3813c73d10f0a1bdd3fcc9e1652cf71e511fdfeccb86
semantic-decisions.tsv Abschluss
04531cda42be001e5b25e142fb1dceb10092d46b2f1153bddae2ca8ea447bfb1
```

**Neue, weiterhin offene A-Funde: 99 Wörter — 46 abschließende Endwörter und 53 Reserven.** In den folgenden TSV-Zeilen bedeutet `E` Endwort, `R` erlaubte Reserve; `I1:…` beziehungsweise `I2:…` bezeichnet die Dateizeile im jeweiligen Review-Input. Die Zahlen in Klammern sind originale Glossennummern.

```tsv
abjuratio	A	Verbrechen/Meineid	E; I1:113 (112): „forswearing, denial under oath; perjury“ — ausdrücklicher Meineidssinn.
abjuro	A	Verbrechen/Meineid	E; I1:114 (110): „deny on oath (falsely) … perjure“ — falsches Schwören.
adfinis	A	Verbrechen/Beihilfe	R; I1:512 (903): „relation (by marriage); neighbor; accomplice“ — zusätzlich Bezeichnung eines Mittäters.
adjutor	A	Verbrechen/Beihilfe	E; I1:568 (1081): „assistant, deputy; accomplice“ — strafbezogener Helfersinn neben allgemeiner Hilfe.
adlego	A	Verbrechen/Anstiftung	R; I1:574 (1104): „depute/send as agent … suborn“ — subornieren, zu unrechtmäßigem Handeln bestimmen.
admissum	A	Verbrechen	R; I1:597 (1172): „crime, offense“ — unmittelbare Deliktsbezeichnung.
affinis	A	Verbrechen/Beihilfe	E; I1:909 (2181): „relation (by marriage); neighbor; accomplice“ — Mittätersinn.
allego	A	Verbrechen/Anstiftung	E; I1:1154 (2668): „depute/send as agent … suborn“ — Subornierung.
ambitus	A	Verbrechen/Bestechung	E; I1:1297 (2946): „canvass, bribery“ — Bestechung als belegte Bedeutung.
ammissum	A	Verbrechen	R; I1:1350 (3085): „crime, offense“ — Schreibvariante mit eigenem Deliktsbeleg.
amoveo	A	Verbrechen/Diebstahl	R; I1:1377 (3165): „move/take/put away, remove, steal“ — zusätzlich stehlen.
averto	A	Verbrechen/Diebstahl und Unterschlagung	R; I1:2602 (5880): „withdraw; steal, misappropriate“ — Aneignungsdelikte.
avorto	A	Verbrechen/Diebstahl und Unterschlagung	R; I1:2624 (5913): „withdraw; steal, misappropriate“ — entsprechender Variantenbeleg.
callim	A	Verbrechen/Betrug	R; I1:4071 (7630): „secretly … covertly; by fraud“ — ausdrücklich betrügerisches Vorgehen.
captio	A	Verbrechen/Betrug	R; I1:4220 (8135): „deception/trick/fraud“ — Betrug gehört zur unmittelbaren Bedeutungsgruppe.
clam	A	Verbrechen/Betrug	R; I1:4834 (10271): „secretly … covertly; by fraud“ — betrügerischer Neben­sinn.
clepo	A	Verbrechen/Diebstahl	R; I1:4909 (10398): „steal; take away secretly“ — Stehlen als Hauptsinn.
commissum	A	Verbrechen	R; I1:5294 (11495): „undertaking … thing entrusted/confiscated; crime“ — zusätzlicher Deliktssinn.
complex	A	Verbrechen/Beihilfe	E; I1:5369 (11781): „accomplice; confederate, participant“ — eigenständige Mittäterglosse.
congero	A	Verbrechen/Dieb; abwertende Menschenbezeichnung	E; I1:5575 (12635): „thief“ — zusätzliche seltenere Personenglüsse neben Sammeln/Anhäufen.
conjuro	A	Verbrechen/Verschwörung	E; I1:5602 (12772): „join in an oath/plot; conspire, plot“ — Verschwörungssinn.
consensio	A	Verbrechen/Verschwörung	R; I1:5738 (13341): „agreement … unanimity; plot, conspiracy“ — zusätzlicher Verschwörungssinn.
consensus	A	Verbrechen/Verschwörung	R; I1:5740 (13337): „agreement … conclusion, plot, conspiracy“ — Verschwörung ausdrücklich enthalten.
consentio	A	Verbrechen/Verschwörung	R; I1:5742 (13346): „act together; plot, conspire, combine“ — zugehörige Verbglosse.
conspiro	A	Verbrechen/Verschwörung	R; I1:5777 (13517): „plot/conspire/unite“ — Verschwören in der führenden Bedeutungsgruppe.
convena	A	Abwertende Menschenbezeichnung	E; I1:5884 (14043): „refugees … immigrants … tramps“ — beanstandet wird der abwertende Personensinn „tramps“.
defraudo	A	Verbrechen/Betrug und Raub	R; I1:6915 (16217): „cheat, defraud, deceive; rob (of)“ — ausdrückliche Deliktssinne.
defrudo	A	Verbrechen/Betrug und Raub	R; I1:6920 (16231): „cheat, defraud, deceive; rob (of)“ — eigener Variantenbeleg.
delictum	A	Verbrechen	R; I1:7008 (16446): „fault/offense/misdeed/crime/transgression“ — Delikt als unmittelbare Bedeutung.
divisor	A	Verbrechen/Wahlbestechung	E; I1:7798 (18369): „a candidate's agent hired to distribute bribes“ — konkreter Bestechungshelfer.
dominor	A	Unterdrückung/Despotismus	E; I1:7873 (18432): „be master/despot/in control“ — ausdrücklich despotischer Herrschaftssinn.
emercor	A	Verbrechen/Bestechung	E; I1:8672 (19044): „bribe; win (over) by bribing … procure favors by bribes“ — eindeutiger Hauptsinn.
factor	A	Verbrechen/Täter	R; I1:9759 (20236): „maker; perpetrator (of a crime)“ — ausdrücklicher Straftätersinn.
falsitas	A	Verbrechen/Betrug	R; I1:9829 (20307): „falsehood, untruth, fraud, deceit“ — Betrug ausdrücklich belegt.
falsum	A	Verbrechen/Betrug	R; I1:9830 (20299): „falsehood, untruth, fraud, deceit“ — entsprechender Substantivsinn.
fraus	A	Verbrechen/Betrug	R; I1:10678 (21030): „fraud … offense, crime“ — sowohl Betrug als auch Verbrechen ausdrücklich.
frustro	A	Verbrechen/Raub und Betrug	R; I1:10826 (21118): „rob/defraud/cheat“ — zusätzliche vollständige Rohglosse enthält Deliktssinne.
frustror	A	Verbrechen/Raub und Betrug	R; I1:10827 (21120): „rob/defraud/cheat“ — entsprechende deponente Variante.
furtivus	A	Verbrechen/Diebesgut	R; I1:10923 (21251): „stolen; secret, furtive“ — gestohlen als unmittelbare Bedeutung.
furtum	A	Verbrechen/Diebstahl	R; I1:10924 (21248): „theft … stolen article“ — Diebstahl und Diebesgut.
hereticus	A	Religiös herabsetzende Menschenbezeichnung	E; I1:12504 (22103): „heretic; teacher of false doctrine“ — personenbezogene Abwertung als Lehrer falscher Lehre.
illex	A	Verbrechen/Betrug	R; I1:13033 (22696): „false, fraudulent“; außerdem (22694) „lawless“ — belegter Betrugssinn.
illicitus	A	Verbrechen/Rechtswidrigkeit	E; I1:13038 (22703): „forbidden, unlawful, illicit“ — ausdrücklich rechtswidrig.
juratus	A	Verbrechen/Verschwörung	E; I2:13 (25030): „being under oath … conspired“ — zusätzlicher Verschwörungssinn.
juro	A	Verbrechen/Verschwörung	E; I2:18 (25027): „swear … conspire“ — Verschwören zusätzlich zum Schwören.
largior	A	Verbrechen/Bestechung	R; I2:837 (25320): „give bribes/presents corruptly“ — korrupte Zuwendungen.
largitio	A	Verbrechen/Bestechung	E; I2:840 (25326): „generosity … bribery“ — ausdrücklicher Bestechungssinn.
largitor	A	Verbrechen/Bestecher	E; I2:841 (25329): „liberal giver; briber“ — Personenbezeichnung für einen Bestecher.
libero	A	Sklaverei/Freilassung	E; I2:1183 (25628): „free; acquit, absolve; manumit“ — ausdrücklich Freilassung aus Sklaverei.
liberta	A	Sklaverei/Freigelassene	R; I2:1184 (25639): „freedwoman; ex-slave“ — frühere Sklavin.
libertina	A	Sklaverei/Freigelassener	E; I2:1186 (25643): „freedman“ — Freigelassenenstatus.
libertus	A	Sklaverei/Freigelassener	R; I2:1187 (25640): „freedman; ex-slave“ — früherer Sklave.
lupinus	A	Verbrechen/Falschgeld	E; I2:1566 (26046): „lupin; fake money“ — eigenständige Falschgeldbedeutung.
lytrum	A	Unterdrückung/Unfreiheit und Lösegeld	E; I2:1624 (26120): „ransom (pl.)“ — Lösegeld für Freikauf aus Unfreiheit.
manumitto	A	Sklaverei/Freilassung	E; I2:1845 (26452): „release, free, set free/at liberty, emancipate“ — spezifisches Freilassungsverb der manumissio-Familie.
merces	A	Verbrechen/Bestechung	E; I2:2139 (26803): „pay … rent, price; bribe“ — Bestechungsgeld als zusätzliche Bedeutung.
munus	A	Verbrechen/Bestechung	E; I2:2709 (27463): „service … gift … bribes (pl.)“ — pluralischer Bestechungssinn.
nexum	A	Sklaverei/Schuldknechtschaft	R; I2:3095 (27866): „debtor bondman for non-payment“ — Schuldner wird wegen Nichtzahlung unfrei.
noxia	A	Verbrechen	R; I2:3241 (28069): „crime, fault“ — unmittelbarer Verbrechenssinn.
peccatum	A	Verbrechen/Vergehen	R; I2:4563 (29600): „lapse, misdemeanor; transgression“ — neben Sünde ausdrücklich strafbares Vergehen.
peculator	A	Verbrechen/Veruntreuung	E; I2:4570 (29621): „embezzler of public money“ — Veruntreuer öffentlicher Gelder.
peculatus	A	Verbrechen/Veruntreuung	E; I2:4571 (29620): „embezzlement of public money or property“ — eindeutiges Delikt.
pejeratus	A	Verbrechen/Falscher Eid	R; I2:4602 (29673): „jus pejeratum => false oath“ — ausdrückliche Falscheid-Wendung.
pejero	A	Verbrechen/Meineid	E; I2:4603 (29672): „swear falsely; swear false oath“ — falsches Schwören.
pejerosus	A	Verbrechen/Meineid	E; I2:4604 (29674): „perjured“ — meineidig.
pejuratus	A	Verbrechen/Falscher Eid	R; I2:4605 (29676): „jus pejuratum => false oath“ — entsprechender Variantenbeleg.
pejuro	A	Verbrechen/Meineid	R; I2:4606 (29675): „swear falsely; swear false oath“ — falsches Schwören.
pejurosus	A	Verbrechen/Meineid	R; I2:4607 (29677): „perjured“ — meineidig.
perjuro	A	Verbrechen/Meineid	E; I2:4785 (30037): „swear falsely“ — falsches Schwören.
perjurus	A	Verbrechen/Meineid	E; I2:4786 (30036): „perjured; false, lying“ — meineidig.
piaculum	A	Verbrechen	R; I2:5030 (30499): „expiatory offering or rite; sin; crime“ — zusätzlicher ausdrücklicher Verbrechenssinn.
proditio	A	Verbrechen/Verrat	R; I2:5909 (31790): „treason, betrayal“ — Hochverrat/Verrat.
proditor	A	Abwertende Menschenbezeichnung/Verräter	E; I2:5910 (31792): „traitor“ — unmittelbare herabsetzende Personenbezeichnung.
proscribo	A	Unterdrückung/Proskription	E; I2:6072 (32051): „proscribe, deprive of property“ — Ächtung und Eigentumsentzug.
pudenter	A	Sexualität/Keuschheit	E; I2:6193 (32251): „bashfully … chastely“ — sexuelle Enthaltsamkeit als Nebenbedeutung.
pudicitia	A	Sexualität/Keuschheit	E; I2:6196 (32254): „chastity; modesty; purity“ — Keuschheit ausdrücklich enthalten.
pudicus	A	Sexualität/Keuschheit	E; I2:6197 (32253): „chaste, modest; virtuous; pure“ — ausdrücklich keusch.
puella	A	Sklaverei/Sklavin	R; I2:6200 (32256): „girl … sweetheart; slavegirl“ — Sklavinnenbedeutung ausdrücklich vorhanden.
purus	A	Sexualität/Sexuelle Reinheit	R; I2:6323 (32371): „chaste, unpolluted by sex“ — expliziter sexueller Nebensinn.
satelles	A	Verbrechen/Beihilfe	E; I2:8147 (34217): „attendant … accomplice, abettor“ — Mittäter und Tathelfer.
scelus	A	Verbrechen	R; I2:8252 (34331): „crime … evil deed“ — Verbrechen als Hauptsinn.
simonia	A	Verbrechen/Kirchliche Korruption	E; I2:9014 (35179): „simony; buying/selling of a benefice/ecclesiastical position“ — Ämterkauf.
sodalis	A	Verbrechen/Beihilfe und Verschwörung	E; I2:9363 (35340): „companion … accomplice, conspirator“ — entsprechende Personenbedeutungen.
sons	A	Verbrechen/Straftäter	R; I2:9432 (35446–35447): „criminal“; „guilty, criminal“ — unmittelbare Täterbezeichnung.
subjector	A	Verbrechen/Fälscher	E; I2:10247 (36090): „forger; substitutor“ — Fälscher als Personenbedeutung.
sublego	A	Verbrechen/Diebstahl	R; I2:10257 (36103): „pick up from the ground, steal away“ — heimliches Wegnehmen.
subripio	A	Verbrechen/Diebstahl	R; I2:10301 (36185): „snatch away, steal“ — ausdrücklicher Diebstahlssinn.
suppilo	A	Verbrechen/Diebstahl	R; I2:10523 (36579): „steal; pilfer“ — stehlen, entwenden.
susceptor	A	Verbrechen/Beherbergen von Dieben	E; I2:10582 (36658): „one who harbors (gamblers/thieves)“ — zusätzliche konkrete Tathelfersphäre.
tempto	A	Verbrechen/Bestechung	E; I2:10950 (37003): „test, try; urge; worry; bribe“ — zusätzlich bestechen.
tollo	A	Verbrechen/Diebstahl	R; I2:11259 (37373): „remove, steal“ — Stehlen als belegter Sinn.
unimanus	A	Behinderungsbezeichnung	E; I2:12165 (38224): „one-handed“ — Bezeichnung der Einhändigkeit.
usarius	A	Sklaverei/Sklavennutzung	R; I2:12517 (38283): „used by one other than owner … (object/slave)“ — Sklave als nutzbares Eigentum.
usuarius	A	Sklaverei/Sklavennutzung	R; I2:12532 (38298): „used by one other than owner … (object/slave)“ — entsprechender eigener Variantenbeleg.
usurpatio	A	Verbrechen/Unrechtmäßige Aneignung	E; I2:12536 (38304): „illegally taking possession“ — ausdrücklich rechtswidrige Besitzergreifung.
venalis	A	Verbrechen/Bestechlichkeit	E; I2:12745 (38567): „open to the influence of bribes“ — Bestechlichkeit.
venditor	A	Verbrechen/Bestechung	R; I2:12755 (38583): „one who sells for bribes or corrupt payments“ — korrupte Verkäufe.
vilicus	A	Sklaverei/Sklave als Gutsverwalter	R; I2:13016 (38905): „farm overseer (slave/free)“ — Sklavenstatus ausdrücklich Teil der Personenbedeutung.
villicus	A	Sklaverei/Sklave als Gutsverwalter	R; I2:13027 (38919): „farm overseer (slave/free)“ — eigener Variantenbeleg.
```

Der zu Beginn offene, inzwischen erledigte Befund:

```tsv
servula	A	Sklaverei/Sklavin	I2:8745 (34969): „young servant girl“ — gemäß verbindlicher Vorgabe Sklavin; anfangs E/allow, abschließend blockiert und aus der Endliste entfernt.
```

**Strittige B-Fälle und relevante Fehlalarmgrenzen**, jeweils mit konkretem Bedeutungsbeleg:

```tsv
absurde	B	Milde Wertung	I1:273 (292): „preposterously, absurdly, inappropriately“ — milde Wertung gemäß ausdrücklicher Vorgabe.
apsurde	B	Milde Wertung	I1:1929 (4309): „preposterously, absurdly, inappropriately“ — gleichbedeutende Schreibvariante.
adgravo	B	Belastung/Druck	I1:535 (979): „aggravate … weigh down, oppress; make heavier“ — Belastungs- und Gewichtskontext; kein eindeutiger despotischer Herrschaftssinn.
aggravo	B	Belastung/Druck	I1:961 (2281): „aggravate … weigh down, oppress; make heavier“ — entsprechender Belastungskontext.
gravo	B	Belastung/Druck	I1:11891 (21726): „load/weigh down; burden, oppress“ — Hauptsinn belasten; A-nahe Nebenbedeutung im Druckkontext.
ingravo	B	Belastung/Belästigung	I1:13486 (23784): „aggravate, make worse, weigh down, oppress, molest“ — „molest“ im Verschlimmerungs-/Belastungskontext; kein belegter sexueller Sinn.
onero	B	Belastung	I2:3701 (28707): „load, burden; oppress“ — Beladen/Belasten als Hauptsinn.
onerosus	B	Beschwerlichkeit	I2:3702 (28709): „oppressive; burdensome; onerous“ — beschwerlich und belastend; kein konkret belegter Unterdrücker.
premo	B	Druck/Bewegung	I2:5746 (31595): „press, press hard, pursue; oppress; overwhelm“ — neutrales Pressen als Hauptsinn; A-Nähe der Nebenbedeutung.
duritia	B	Härte/Strenge	I1:8140 (18612): „hardness … hardship, oppressiveness; strictness, rigor“ — Härte und Strenge; kein eindeutiges Unterdrückungsgeschehen.
durities	B	Härte/Strenge	I1:8141 (18613): „hardness … hardship, oppressiveness; strictness, rigor“ — entsprechender Variantenbeleg.
adparitio	B	Dienerschaft	I1:647 (1328): „service, attendance; servants, attendants“ — Dienst und Begleitung, ohne Sklavensinn.
apparitio	B	Dienerschaft	I1:1838 (4109): „service, attendance; servants, attendants“ — gleicher ausdrücklich neutraler Dienstkontext.
ancillor	B	Dienerschaft	I1:1461 (3379): „act as handmaid … be subservient/at beck and call“ — unterwürfige Dienerschaft; keine ausdrücklich belegte Sklavin.
calator	B	Dienerschaft	I1:4036 (7477): „personal attendant, servant, footman; servant of a priest“ — persönlicher beziehungsweise priesterlicher Diener.
kalator	B	Dienerschaft	I2:76 (25091): „personal attendant, servant, footman“ — eigener neutraler Variantenbeleg.
creatura	B	Dienerschaft	I1:6171 (14701): „creation; creature … servant (late Latin)“ — Diener-Nebensinn ohne Sklaverei.
famulor	B	Dienerschaft	I1:9854 (20327): „be a servant, attend“ — dienen und betreuen.
era	B	Haushalt/Dienerschaft	I1:8987 (19327): „mistress; lady of the house … servants“ — Hausherrin; keine sexuell verstandene Geliebte und kein expliziter Sklavensinn.
hera	B	Haushalt/Dienerschaft	I1:12482 (22075): „mistress; lady of the house … servants“ — entsprechender Haushaltskontext.
pediseca	B	Dienerschaft	I2:4586 (29657): „female attendant; waiting woman, waitress; handmaiden“ — Bedienstete.
pedisequa	B	Dienerschaft	I2:4588 (29660): „female attendant … waitress; handmaiden“ — Bedienstete; keine belegte Sklavin.
cerdo	B	Beruf/Beiläufiger Namensgebrauch	I1:4525 (9115): „artisan; craftsman; cobbler … proper name especially of slaves“ — Handwerker; Zusatz bezeichnet Namensgebrauch, keine Bedeutung „Sklave“.
dama	B	Tier/Beiläufiger Namensgebrauch	I1:6590 (15613): „fallow/red-deer … doe; slave name“ — Hirsch beziehungsweise Antilope; Sklavenname als Verwendungskontext.
animal	B	Tier	I1:1555 (3526): „animal, living thing … beast, brute; insect“ — „brute“ bezeichnet hier ein Tier.
asellus	B	Tier	I1:2161 (5021): „(small/young) ass, donkey“ — Esel; keine belegte Menschenbeleidigung.
asina	B	Tier	I1:2169 (5030): „she-ass“ — Eselstute.
catella	B	Tier	I1:4378 (8602): „puppy (female), young/little bitch; lap dog“ — „bitch“ ist durch den Kontext als Hündin bestimmt.
situs	B	Ort/Zustand mit A-naher Nebenbedeutung	I2:9108 (35277–35278): „situation, position, site … mold“; „positioned, situated“ — verbindlicher B-Fall.
subalaris	B	Neutrale Anatomie	I2:10214 (36006, 36008): „carrying/stuck under the arm“; „little wing“ — anatomischer Arm beziehungsweise Flügel; kein belegter Waffensinn.
vesica	B	Neutrale Anatomie	I2:12910 (38747): „bladder; balloon“ — Blase als Organ beziehungsweise Ballon.
poples	B	Neutrale Anatomie	I2:5426 (30951): „knee“ — Knie.
urinator	B	Tätigkeit/Tauchen	I2:12492 (38275): „diver“ — Taucher.
urino	B	Bewegung/Tauchen	I2:12493 (38272): „dive, plunge into water“ — ins Wasser tauchen.
urinor	B	Bewegung/Tauchen	I2:12494 (38273): „dive, plunge into water“ — eigener Variantenbeleg.
tribolus	B	Botanik	I2:11486 (37711): „spiny plant; caltrop (Tribulus terrestris)“ — „caltrop“ wird botanisch eindeutig bestimmt.
tribulus	B	Botanik	I2:11488 (37717): „spiny plant; caltrop (Tribulus terrestris)“ — entsprechender Pflanzenbeleg.
seviratus	B	Amt/Numeral	I2:8778 (35021): „sexvirate … member of a board of six men“ — Sechsmänneramt.
sexennis	B	Alter/Numeral	I2:8781 (35033): „six years old“ — sechsjährig.
sexprimus	B	Amt/Numeral	I2:8782 (35034): „member of council of six provincial officials“ — Mitglied eines Sechsergremiums.
vulgaris	B	Alltäglichkeit	I2:13326 (39212): „usual, common, commonplace, everyday“ — gewöhnlich; kein Vulgaritätsmarker.
papaver	B	Botanik	I2:4273 (29273): „poppy; poppy-seed“ — Pflanze und Samen; keine verfügbare Opium-/Rauschgiftglosse.
fermento	B	Lebensmittel/Prozess	I1:10054 (20519): „leaven; cause fermentation in; aerate (soil)“ — Sauerteig/Gärung beziehungsweise Bodenbelüftung; kein belegtes alkoholisches Getränk.
fermentum	B	Lebensmittel/Prozess	I1:10055 (20518): „fermentation, leavening … yeast“ — Gärung und Hefe, ohne konkreten Alkoholsinn.
sobrietas	B	Nüchternheit	I2:9349 (35304): „sobriety, freedom from intoxication … staidness“ — Nüchternheit beziehungsweise besonnene Lebensführung.
utriculus	B	Behälter	I2:12562 (38336): „wineskin, leather bottle“ — neutraler Lederbehälter; A-Nähe durch Weinverwendung.
panaces	B	Heilpflanze	I2:4233 (29220–29221): „plant, supposed to cure all diseases; panacea“ — Heilpflanze; Krankheit steht als Anwendungsziel im Kontext.
panax	B	Heilpflanze	I2:4236 (29223): „plant, supposed to cure all diseases“ — Heilpflanze.
tinnitus	B	Geräusch	I2:11205 (37329): „ringing, clanging, jangling“ — Geräusch; keine medizinische Diagnose in den verfügbaren Glossen.
saliva	B	Gewöhnliche Körperfunktion	I2:8009 (34049): „spittle; distinctive flavor“ — Speichel; keine Exkremente oder vulgäre Verwendung.
sudo	B	Gewöhnliche Körperfunktion	I2:10395 (36332): „sweat, perspire“ — Schwitzen.
sudor	B	Gewöhnliche Körperfunktion	I2:10397 (36337): „sweat; hard labor“ — Schweiß beziehungsweise Anstrengung.
inlex	B	Lockmittel mit seltener A-naher Nebenbedeutung	I1:13532 (23889 F, 23890 C): „lawless“ neben „one who entices/allures; decoy“ — selten markierte Gesetzlosigkeitsnähe; neutraler Lockmittel-/Lockvogelsinn.
adfingo	B	Erfinden/Nachahmen	I1:511 (902): „embellish, counterfeit, forge; claim wrongly“ — Ausschmücken/Erfinden ohne konkret bezeichnetes Fälschungsdelikt.
affingo	B	Erfinden/Nachahmen	I1:908 (2180): „embellish, counterfeit, forge“ — entsprechender Erfindungs-/Zuschreibungskontext.
adsimilo	B	Nachahmen	I1:715 (1538): „make like; compare; counterfeit, simulate, imitate … act a part“ — nachahmen/simulieren; kein konkretes Delikt.
adsimulo	B	Nachahmen	I1:716 (1543): „make like … simulate, imitate … act a part“ — entsprechender Variantenkontext.
assimilo	B	Nachahmen	I1:2234 (5186): „make like; compare … imitate … act a part“ — Nachbildung und Rollenhandeln.
assimulo	B	Nachahmen	I1:2236 (5192): „make like … counterfeit, simulate, imitate“ — Nachahmung ohne konkret bezeichnetes Fälschungsobjekt.
fingo	B	Formen/Erfinden	I1:10256 (20685–20690): „mold, form, shape … make up (story/excuse); pretend, pose; forge, counterfeit“ — Gestalten und Erfinden; A-nahe Fälschungswörter im Fiktionskontext.
similo	B	Nachahmen	I2:9011 (35166): „imitate, copy; pretend … simulate; counterfeit; feint“ — nachahmen und vortäuschen.
simulo	B	Nachahmen	I2:9029 (35192): „imitate, copy; pretend … simulate; counterfeit; feint“ — entsprechender Variantenbeleg.
ficticius	B	Fiktion	I1:10149 (20619): „fictitious; artificial; counterfeit, not genuine“ — fiktiv/künstlich/unecht.
fictitius	B	Fiktion	I1:10154 (20623): „fictitious; artificial; counterfeit … feigned, pretended“ — entsprechender Fiktionskontext.
fictus	B	Fiktion	I1:10156 (20616): „feigned, false; counterfeit“ — vorgetäuscht/unecht; kein konkret bezeichnetes Delikt.
confessus	B	Eingeständnis mit A-nahem Personen-Nebensinn	I1:5524 (12421 E, 12423 C): „one who admits/confesses liability/crime“ neben „admitted, acknowledged … obvious“ — seltene Personenverwendung; Hauptsinn anerkannt/eingestanden.
conventio	B	Vereinbarung/Historisches Familienrecht	I1:5888 (14061–14062): „wife passing into control of husband“; „agreement, compact, covenant“ — historische Rechtsform; kein ausdrücklich gewaltsamer oder versklavender Sinn.
repetunda	B	Rechtliche Wiedererlangung	I2:7261 (33442): „recovery … of extorted money“ — Rückforderung erpressten Geldes; bezeichnet die Wiedererlangung.
redemptio	B	Rückkauf/Erlösung	I2:6892 (33171): „redemption, buying back, ransoming; deliverance“ — Rückkauf/Befreiung als Hauptkontext; A-nahe Lösegeldverwendung.
redimo	B	Rückkauf/Erlösung	I2:6898 (33177–33180): „buy back … redeem; atone for; ransom; rescue/save“ — kaufen, zurückerlangen und erlösen; A-nahe Freikaufsverwendung.
restinguo	B	Löschen mit A-naher Nebenbedeutung	I2:7397 (33578): „extinguish, quench, put out; exterminate, destroy“ — Hauptsinn löschen; zerstörerische Nebenbedeutung ohne bezeichnetes Lebewesen.
pileum	B	Kleidung/Historischer Gebrauch	I2:5065 (30547): „felt cap … by manumited slaves; freedom/liberty; beret“ — Kappe; Freigelassene sind Trägerkontext.
pileus	B	Kleidung/Historischer Gebrauch	I2:5066 (30546): „felt cap … by manumited slaves“ — entsprechender Kleidungsbeleg.
pilleum	B	Kleidung/Historischer Gebrauch	I2:5069 (30550): „felt cap … by manumited slaves“ — entsprechender Kleidungsbeleg.
pilleus	B	Kleidung/Historischer Gebrauch	I2:5070 (30549): „felt cap … by manumited slaves“ — entsprechender Kleidungsbeleg.
pasca	B	Getränk/Historischer Gebrauch	I2:4401 (29443): „vinegar mixed in water … Roman soldiers … also for slaves“ — Essigwasser; Soldaten/Sklaven sind Nutzerkontext.
secubitus	B	Wohnen/Schlafen	I2:8473 (34546): „sleeping apart from one's spouse or lover“ — getrenntes Schlafen; kein belegter Sexualakt.
secubo	B	Wohnen/Schlafen	I2:8474 (34545): „sleep apart … sleep alone; live alone“ — allein schlafen/leben.
pudor	B	Scham/Bescheidenheit	I2:6198 (32255): „decency, shame; sense of honor; modesty; bashfulness“ — keine sexuelle Glosse.
voluptas	B	Freude/Genuss	I2:13242 (39173): „pleasure, delight, enjoyment“ — allgemeiner Genuss; keine sexuelle Glosse.
gentilis	B	Religion/Zugehörigkeit	I1:11353 (21455–21459): „non-Jew … not of one's faith“; „of same gens“ — Zugehörigkeit beziehungsweise Außenperspektive anderer Religionen; keine personenbezogene Beschimpfung belegt.
haeresis	B	Religion/Lehrsystem	I1:12088 (21889): „philosophical/religious school of thought/sect; heresy/heretical doctrine“ — Lehrsystem; keine herabsetzende Personenbezeichnung.
paganus	B	Religion/Zivilstatus	I2:4171 (29132–29134): „pagan; countryman, peasant; civilian“ — Religionszugehörigkeit beziehungsweise ziviler Status.
scisma	B	Religiöse Trennung	I2:8335 (34398): „schism/split/deep divide … refusal to submit to Pope“ — religiöse Spaltung; keine Beschimpfung.
schisma	B	Religiöse Trennung	I2:8285 (34348): „schism/split/deep divide“ — entsprechender Variantenbeleg.
dictator	B	Historisches Amt	I1:7416 (17616): „Roman magistrate … appointed in emergency“ — konkret beschriebenes römisches Amt.
dictatura	B	Historisches Amt	I1:7419 (17620): „dictatorship, office of dictator“ — zugehörige Amtsbezeichnung.
effectus	B	Ausführung	I1:8338 (18781): „execution, performance; effect“ — Durchführung/Wirkung.
esecutio	B	Ausführung/Rechtsdurchsetzung	I1:9199 (19405): „performance, carrying out; enforcement (law)“ — Durchführung und Rechtsdurchsetzung, kein Hinrichtungssinn.
executio	B	Ausführung/Rechtsdurchsetzung	I1:9445 (19737): „performance, carrying out; enforcement (law)“ — entsprechender Beleg.
exsecutio	B	Ausführung/Rechtsdurchsetzung	I1:9613 (20010): „performance, carrying out; enforcement (law)“ — entsprechender Variantenbeleg.
```

Klar belegte A-Wortfamilien beziehungsweise Regellücken:

- **Meineid und Schreibvarianten:** `abjuratio`/`abjuro`, `pejer-`, `pejur-`, `perjur-`; die Varianten benötigen jeweils ihre eigenen Quellenbelege.
- **Bestechung außerhalb offensichtlicher Deliktsnamen:** `ambitus`, `divisor`, `emercor`, `largior`/`largitio`/`largitor`, `merces`, `munus`, `tempto`, `venalis`, `venditor`.
- **Sklaverei einschließlich positiver Freilassungssinne:** `servula`, `nexum`, `liberta`/`libertina`/`libertus`, `libero`, `manumitto`, `puella`, `usarius`/`usuarius`, `vilicus`/`villicus`.
- **Zusätzliche Rohglossen derselben Schreibweise:** besonders `congero` → „thief“, `lupinus` → „fake money“, `complex` → „accomplice“, `susceptor` → Beherbergen von Dieben. Die neutrale erste Glosse reicht zur Prüfung nicht.
- **Erlaubte Deliktsreserven:** unter anderem `admissum`/`ammissum`, `defraudo`/`defrudo`, `delictum`, `fraus`, `furtum`, `noxia`, `scelus`, `sons`, `subripio`, `suppilo`.
- **Weitere Kategorien:** `simonia` für Ämterkauf, `hereticus` für religiöse Personenabwertung, `unimanus` für Behinderungsbezeichnungen sowie die expliziten Keuschheits-/Sexualitätssinne bei `pudenter`, `pudicitia`, `pudicus`, `purus`.
