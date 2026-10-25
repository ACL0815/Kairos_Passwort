# Lesende EN-Restsuche und deutsche Triage Stufe 2

Modell codex/gpt-6.1-sol, reasoningEffort=high, serviceTier=priority.
Dies sind Vorschläge des Kindes. Die übernehmende Adjudikation und
konservative B-Ausschlüsse werden getrennt in stage2-decisions.tsv und
stage2-german-triage.tsv dokumentiert.

Die DE-Triage ist vollständig: **224 A-Entscheidungen und 75 B-Entscheidungen, zusammen genau 299 Meldungen**. Die EN-Restsuche ergibt **299 neue A-Funde: 223 aktuelle Endwörter und 76 erlaubte Reserven**. Alle 299 EN-Schreibweisen standen beim Schlussabgleich weiterhin auf `allow`.

Die A-Auswahl ist konservativ. Bei seltenen nichtsexuellen Fach- und Homographenbedeutungen kann die Rubrik eine B-Entscheidung zulassen; solche belegten Bedeutungen sind von bloßen Assoziationen zu unterscheiden. Die gesondert aufgeführten B-Fälle verkaufe ich nicht als bestätigte A-Verstöße.

**Abdeckung und Eingabestand.** Ausschließlich gelesen; keine Dateien geschrieben, keine Git-/PR-Mutationen, keine Weiterdelegation.

- Rubrik vollständig gelesen.
- EN: alle **8.150 erlaubten Schreibweisen** gesichtet, einschließlich aller **1.186 Reserven**. Die tatsächlich gelesene Endliste hatte **6.964**, nicht 6.966 Wörter.
- Entscheidungsdatei: EN-Zeilen **1–9.558**; 8.150 `allow`, 1.408 `block`.
- Bedeutungsdatei: Zeilen **1–9.558**, `aardvark` bis `zucchini`. Für den erlaubten Bestand wurden sämtliche **29.118 WordNet-Zuordnungen** mit mehreren gezielten Bedeutungsfiltern durchsucht und die ausgegebenen Verdachtsbedeutungen semantisch bewertet. Beispiele, Negationen und unpassende Lemma-Ableitungen wurden dabei getrennt.
- **7.560 erlaubte Wörter mit WordNet-Eintrag, 590 ohne Eintrag.** Zwei dieser Lücken wurden extern mit A-Beleg geschlossen (`hardhead`, `slinky`); zusätzlich wurden drei fehlende Nebenbedeutungen vorhandener WordNet-Wörter extern belegt (`mare`, `sow`, `yuppie`).
- **Grenze der EN-Abdeckung:** Keine vollständige menschliche Einzelabnahme aller 29.118 Zuordnungen und keine externe Bedeutungsrecherche für die übrigen 588 `NO_ENTRY`-Schreibweisen. Die Restsuche über den gesamten Bestand ist erfolgt; ein vollständiges semantisches Freigabesiegel folgt daraus nicht.
- DE: Zeilen **1–299**, `abgang` bis `zwicken`, vollständig zugeordnet. Gegenprüfung: keine fehlenden oder doppelt zugeordneten Wörter.

SHA-256 der Eingaben:

```text
curation-rubric.md
65ea55de3e60a534da3243a3fe3600e3fecead5338a86daa818cd193c4e05159

english.txt — Zeilen 1–6964, aardvark … zoology
3e02576c2017d2d4f8f6a2aeba2e0fac6a7436890f364fdb7d332f13345b2f08

stage2-english-input.tsv — Zeilen 1–9558
437eb4e63cd557b5865f5550e13c40769c4522ba07874193340ba12da1ea5955

de-findings-report.tsv — Zeilen 1–299
9db842fe931d76b48d48707ec8dd3c1722173303d8fe330dcdd4ada712f564ed

semantic-decisions.tsv — Beginn der Untersuchung
04531cda42be001e5b25e142fb1dceb10092d46b2f1153bddae2ca8ea447bfb1

semantic-decisions.tsv — letzter Abgleich aller 299 EN-Funde
611f1fd11761b0dc43fc752259529d61ea2eba2a1c6dfc85cdf34ec0306eb3b5
```

Die Entscheidungsdatei wurde während meiner Untersuchung anderweitig verändert. Bedeutungsdatei, EN-Endliste und DE-Bericht blieben bei den erfassten Kontrolllesungen unverändert. Die 299 EN-Funde wurden gegen den zuletzt genannten Entscheidungsstand erneut auf `allow` geprüft.

Die folgenden TSV-Blöcke haben jeweils die verlangten vier Spalten. `noun:…`, `verb:…` usw. bezeichnen **WordNet 3.0**; die deutschen Bedeutungsangaben sind kurze Paraphrasen.

EN-A: Sexualität, Genitalien und entsprechende Nebenbedeutungen — 71 Wörter.

```tsv
ablaze	A	Sexualitaet	sexuell erregt; adj:01726021:ablaze
aflame	A	Sexualitaet	sexuell erregt; adj:01726021:aflame
came	A	Sexualitaet	Orgasmus erleben; verb:02197360:come
camping	A	Sexualitaet	etwas mit sexueller Qualität versehen; verb:00525805:camp
chase	A	Sexualitaet	jemanden sexuell oder romantisch verfolgen; verb:02535093:chase
chasing	A	Sexualitaet	jemanden sexuell oder romantisch verfolgen; verb:02535093:chase
coil	A	Sexualitaet	Verhütungsmittel in der Gebärmutter; noun:03066130:coil
congress	A	Sexualitaet	Geschlechtsverkehr; noun:00845523:congress
decent	A	Sexualitaet	sexuellen Verhaltenskonventionen entsprechend; adj:00682932:decent
deviate	A	Sexualitaet_Abwertung	Person mit insbesondere sexuell als abweichend bezeichnetem Verhalten; noun:10419047:deviate
dish	A	Sexualitaet	sexuell verführerisch aussehende Frau; noun:10613996:dish
easiest	A	Sexualitaet	sexuell ungehemmt; adj:00361837:easy
egg	A	Genitalien	Hoden; noun:05524615:egg
entire	A	Sexualitaet	bei Haustieren sexuell fortpflanzungsfähig; adj:02137394:entire
exciting	A	Sexualitaet	sexuell erregen; verb:01762283:excite
fall	A	Sexualitaet	Keuschheit verlieren; verb:02566092:fall
feel	A	Sexualitaet	manuelle Genitalstimulation; noun:00854538:feel
felt	A	Sexualitaet	Genitalien mit den Händen berühren; verb:01210152:feel
flirt	A	Sexualitaet	Verhalten zum Erregen sexuellen Interesses; noun:00512522:flirt
gave	A	Sexualitaet	in Geschlechtsverkehr einwilligen; verb:00108747:give
give	A	Sexualitaet	in Geschlechtsverkehr einwilligen; verb:00108747:give
given	A	Sexualitaet	in Geschlechtsverkehr einwilligen; verb:00108747:give
giving	A	Sexualitaet	in Geschlechtsverkehr einwilligen; verb:00108747:give
glamour	A	Sexualitaet	Schönheit oder Charme mit Sex-Appeal; noun:04685195:glamour
ground	A	Sexualitaet	erotisch anzügliche Beckenbewegungen beim Tanzen; verb:02048682:grind
had	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:have
has	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:have
have	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:have
honor	A	Sexualitaet	weibliche Keuschheit; noun:04849972:honor
hook	A	Sexualitaet	sexuelle Dienste anbieten; verb:00781652:hook
jazz	A	Sexualitaet	Geschlechtsverkehr haben; verb:01426397:jazz
juicy	A	Sexualitaet	sexuell anzüglich; adj:02132224:juicy
kissing	A	Sexualitaet	Vorspiel ohne Genitalkontakt; noun:00854000:kissing
knew	A	Sexualitaet	Geschlechtsverkehr haben; verb:01426397:know
know	A	Sexualitaet	Geschlechtsverkehr haben; verb:01426397:know
light	A	Sexualitaet	sexuell ungehemmt; adj:00361837:light
love	A	Sexualitaet	sexuelle Aktivitäten einschließlich Geschlechtsverkehr; noun:00846515:love
luscious	A	Sexualitaet	stark sexuell anziehend; adj:02132967:luscious
made	A	Sexualitaet	zu Sex verführen; verb:01428578:make
make	A	Sexualitaet	zu Sex verführen; verb:01428578:make
making	A	Sexualitaet	zu Sex verführen; verb:01428578:make
moon	A	Vulgaer	jemandem das entblößte Gesäß zeigen; verb:02141840:moon
neck	A	Sexualitaet	mit sexueller Leidenschaft küssen oder liebkosen; verb:01426153:neck
oomph	A	Sexualitaet	sexuelle Anziehungskraft; noun:04686935:oomph
pair	A	Sexualitaet	Geschlechtsverkehr beziehungsweise Paarung vollziehen; verb:01428853:pair
peach	A	Sexualitaet	sexuell verführerisch aussehende Frau; noun:10613996:peach
pet	A	Sexualitaet	erotisch streicheln; verb:01226600:pet
petted	A	Sexualitaet	erotisch streicheln; verb:01226600:pet
player	A	Sexualitaet	Person mit gleichzeitig wechselnden sexuellen Partnern; noun:10440580:player
pure	A	Sexualitaet	sexuell jungfräulich; adj:00361125:pure
pureness	A	Sexualitaet	weibliche Keuschheit; noun:04849972:pureness
purity	A	Sexualitaet	weibliche Keuschheit; noun:04849972:purity
relation	A	Sexualitaet	Geschlechtsverkehr; noun:00845523:relation
rode	A	Sexualitaet	kopulieren; verb:01429322:ride
scarf	A	Sexualitaet	beim Sich-Strangulieren masturbieren; verb:01431132:scarf
serve	A	Sexualitaet	sich paaren; verb:01428011:serve
service	A	Sexualitaet	sich paaren; verb:01428011:service
serving	A	Sexualitaet	sich paaren; verb:01428011:serve
siren	A	Sexualitaet	als gefährlich verführerisch bezeichnete Frau; noun:10055410:siren
slinky	A	Sexualitaet	sexuell anziehende Kleidung beziehungsweise sexuell anzüglicher Tanz; Cambridge:slinky
swing	A	Sexualitaet	promiskuitiven Sex beziehungsweise Partnertausch betreiben; verb:01427483:swing
swung	A	Sexualitaet	promiskuitiven Sex beziehungsweise Partnertausch betreiben; verb:01427483:swing
take	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:take
taking	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:take
thong	A	Sexualitaet	minimales Kleidungsstück von Stripteasetänzern; noun:03464053:thong
took	A	Sexualitaet	Geschlechtsverkehr haben, archaisch; verb:01427127:take
tread	A	Sexualitaet	sich paaren; verb:01427907:tread
treading	A	Sexualitaet	sich paaren; verb:01427907:tread
trod	A	Sexualitaet	sich paaren; verb:01427907:tread
trodden	A	Sexualitaet	sich paaren; verb:01427907:tread
union	A	Sexualitaet	Paarung eines männlichen und weiblichen Individuums zur Fortpflanzung; noun:00847340:union
```

EN-A: Ausscheidung — 6 Wörter.

```tsv
empty	A	Ausscheidung	aus dem Körper ausscheiden; verb:00073343:empty
flow	A	Ausscheidung	menstruieren; verb:00062203:flow
spew	A	Ausscheidung	Speichel, Schleim oder Auswurf ausstoßen; verb:00101956:spew
stale	A	Ausscheidung	urinieren, bei Rindern und Pferden; verb:00072897:stale
sweat	A	Ausscheidung	Schweiß durch Hautporen ausscheiden; verb:00067545:sweat
water	A	Ausscheidung	Urin; noun:14855724:water
```

EN-A: Beleidigung, Abwertung und vulgäre beziehungsweise religiös herabsetzende Bedeutungen — 29 Wörter.

```tsv
boy	A	Ethnische_Abwertung	ethnische Herabsetzung eines schwarzen Mannes; noun:09637837:boy
buggy	A	Abwertung_Krankheit	Slangbezeichnung für geistig nicht normal; adj:02074929:buggy
child	A	Abwertung	unreifer kindischer Mensch; noun:09918554:child
crumb	A	Beleidigung	verachtenswerter Mensch; noun:10539715:crumb
cult	A	Religioese_Abwertung	als extremistisch oder falsch bezeichnete Religion beziehungsweise Sekte; noun:05948537:cult
dragon	A	Abwertung	äußerst unangenehme, streng wachende Frau; noun:10029729:dragon
earthy	A	Sexualitaet_Vulgaer	unverblümt auf Sex bezogen; adj:00683531:earthy; ergänzend Cambridge:earthy
fox	A	Abwertung	hinterhältiger täuschender Mensch; noun:10022759:fox
gawk	A	Beleidigung	unbeholfener dummer Mensch; noun:10274639:gawk
ham	A	Abwertung	unfähiger übertreibender Schauspieler; noun:10158010:ham
hardhead	A	Beleidigung	Bezeichnung für einen wenig intelligenten Menschen; Collins:hardhead, American English 1, Sinn 2
mare	A	Beleidigung	respektlose Bezeichnung für eine Frau; Cambridge:mare, WOMAN
name	A	Beleidigung	verleumderisches oder beleidigendes Wort; noun:06720964:name
operator	A	Abwertung	skrupelloser Mensch, der Schwierigkeiten geschickt umgeht; noun:10194566:operator
profane	A	Vulgaer_Religioese_Abwertung	durch Fluchen oder anstößige Sprache gekennzeichnet; adj:00425002:profane
queen	A	Beleidigung	beleidigende Bezeichnung eines offen homosexuellen Mannes; noun:10076033:queen
sheep	A	Beleidigung	ängstlicher wehrloser Dummkopf; noun:10588074:sheep
shout	A	Beleidigung	jemanden mit unflätiger oder beleidigender Sprache angehen; verb:00845299:shout
slang	A	Beleidigung	mit grober Sprache beschimpfen; verb:00845658:slang
slum	A	Abwertung	Verwendung beim Besuch ärmerer Milieus ausdrücklich als herablassend und unsensibel bezeichnet; verb:02492694:slum
sneak	A	Beleidigung	hinterhältiger und verachtenswerter Mensch; noun:10616048:sneak
sow	A	Beleidigung	abwertende Slangbezeichnung für eine verachtete, oft dicke Frau; Wiktionary:sow, English noun Sinn 2
taco	A	Ethnische_Abwertung	ethnische Herabsetzung einer Person mexikanischer Herkunft; noun:09722898:taco
throwaway	A	Beleidigung	als teilweise beleidigend markierte Bezeichnung eines verlassenen obdachlosen Jungen; noun:10663137:throwaway
user	A	Abwertung	Person, die andere egoistisch oder unethisch ausnutzt; noun:10072546:user
wildcat	A	Abwertung	grausam räuberischer Mensch; noun:09845589:wildcat
wolf	A	Abwertung	grausam räuberischer Mensch; noun:09845589:wolf
yuppie	A	Abwertung	abwertende Personenbezeichnung für wohlhabende konsumorientierte Stadtberufstätige; Cambridge:yuppie, disapproving
wrongdoer	A	Abwertung	Übeltäter beziehungsweise Verletzer moralischer oder staatlicher Gesetze; noun:09633969:wrongdoer
```

Die extern ergänzten Bedeutungen sind direkt belegt bei [Cambridge: slinky](https://dictionary.cambridge.org/us/dictionary/english/slinky), [Collins: hardhead](https://www.collinsdictionary.com/dictionary/english/hardhead), [Cambridge: mare, WOMAN](https://dictionary.cambridge.org/dictionary/english/mare?topic=insults-and-abuse), [Wiktionary: sow, englisches Substantiv Sinn 2](https://en.wiktionary.org/wiki/sow#Noun) und [Cambridge: yuppie](https://dictionary.cambridge.org/us/dictionary/english/yuppie?q=yuppie). Die zusätzliche Absicherung für `earthy` betrifft ausdrücklich Sex; bei `profane` nennt Cambridge auch anstößige, respektlose oder grobe Sprache. [Cambridge: earthy](https://dictionary.cambridge.org/dictionary/english/earthy), [Cambridge: profane](https://dictionary.cambridge.org/us/dictionary/english/profane).

EN-A: Unterdrückung, Zwang und Folterkontext — 10 Wörter.

```tsv
crimp	A	Unterdrueckung	Männer durch Täuschung oder Zwang in Schiffs- oder Militärdienst bringen; noun:09978566:crimp
dictator	A	Unterdrueckung	tyrannisch handelnde Person; noun:09824135:dictator
quash	A	Unterdrueckung	mit Gewalt oder Einschüchterung niederschlagen; verb:02424128:quash
railroad	A	Unterdrueckung	durch Zwang oder Drohungen nötigen; verb:02506181:railroad
rendition	A	Unterdrueckung_Folter	Gefangene in Länder überstellen, in denen Folter erlaubt ist; noun:00421321:rendition
squeeze	A	Unterdrueckung	etwas durch Zwang oder Einschüchterung erlangen; verb:02241107:squeeze
squeezing	A	Unterdrueckung	etwas durch Zwang oder Einschüchterung erlangen; verb:02241107:squeeze
subdued	A	Unterdrueckung	mit Gewalt oder Einschüchterung unterwerfen; verb:02424128:subdue
subduing	A	Unterdrueckung	mit Gewalt oder Einschüchterung unterwerfen; verb:02424128:subdue
subject	A	Unterdrueckung	zum Unterwerfen oder Dienen zwingen; verb:02496816:subject
```

EN-A: Verbrechen und ausdrücklich deliktische Bedeutungen — 62 Wörter.

```tsv
apache	A	Verbrechen	Pariser Gangster; noun:09607208:apache
bagged	A	Verbrechen	unrechtmäßig an sich nehmen; verb:02292432:bag
bagging	A	Verbrechen	unrechtmäßig an sich nehmen; verb:02292432:bag
bribe	A	Verbrechen	bestechen; verb:02284096:bribe
buy	A	Verbrechen	bestechen; verb:02284096:buy
caper	A	Verbrechen	Verbrechen, insbesondere Raub; noun:00782792:caper
coyote	A	Verbrechen	Menschenschmuggler über die US-Grenze; noun:09973903:coyote
defraud	A	Verbrechen	durch Betrug berauben; verb:02572119:defraud
don	A	Verbrechen	Oberhaupt einer organisierten Verbrecherfamilie; noun:10025295:don
embezzle	A	Verbrechen	anvertrautes Eigentum veruntreuen; verb:02292535:embezzle
fake	A	Verbrechen	eine Kopie in Täuschungsabsicht herstellen; verb:01654271:fake
falsify	A	Verbrechen	in Täuschungsabsicht manipulieren; verb:02576921:falsify
family	A	Verbrechen	organisierte Vereinigung von Gangstern; noun:08246302:family
fiddle	A	Verbrechen	den Arbeitgeber betrügen und bestehlen; verb:02292989:fiddle
fiddling	A	Verbrechen	den Arbeitgeber betrügen und bestehlen; verb:02292989:fiddle
forge	A	Verbrechen	eine Kopie in Täuschungsabsicht herstellen; verb:01654271:forge
hacker	A	Verbrechen	in Computersysteme zum Stehlen, Ändern oder Zerstören von Informationen eindringen; noun:10155222:hacker
illicitly	A	Verbrechen	auf illegale Weise; adv:00154536:illicitly
jailbird	A	Verbrechen	wiederholt inhaftierter Verbrecher; noun:10219121:jailbird
jailbreak	A	Verbrechen	Gefängnisausbruch; noun:00059989:jailbreak
job	A	Verbrechen	Verbrechen, insbesondere Raub; noun:00782792:job
kite	A	Verbrechen	betrügerisch im Betrag veränderter Bankscheck; noun:13382471:kite
launder	A	Verbrechen	illegal erlangte Gelder waschen; verb:00188949:launder
lift	A	Verbrechen	illegal an sich nehmen; verb:02277138:lift
lifter	A	Verbrechen	Ladendieb; noun:09866661:lifter
lifting	A	Verbrechen	illegal an sich nehmen; verb:02277138:lift
perjurer	A	Verbrechen	vorsätzlich falscher Zeuge beziehungsweise Meineidiger; noun:10416909:perjurer
perjury	A	Verbrechen	falsche Aussage unter Eid als Straftat; noun:00772381:perjury
plot	A	Verbrechen	geheimer, insbesondere illegaler Plan; noun:05907682:plot
plotted	A	Verbrechen	heimlich etwas gewöhnlich Illegales planen; verb:00708128:plot
poach	A	Verbrechen	illegal jagen; verb:01143266:poach
price	A	Verbrechen	Bestechungskosten; noun:13303759:price
prize	A	Verbrechen	illegal erlangte Güter oder Gelder; noun:13262663:prize
prowler	A	Verbrechen	herumschleichende Person mit gewöhnlich rechtswidrigen Absichten; noun:10486679:prowler
push	A	Verbrechen_Drogen	illegale Waren wie Drogen verkaufen; verb:02245555:push
pushing	A	Verbrechen_Drogen	illegale Waren wie Drogen verkaufen; verb:02245555:push
relieve	A	Verbrechen	durch Stehlen wegnehmen; verb:02312882:relieve
relieving	A	Verbrechen	durch Stehlen wegnehmen; verb:02312882:relieve
rig	A	Verbrechen	betrügerische Unternehmung; noun:00780148:rig
rigging	A	Verbrechen	betrügerisch manipulieren; verb:02351239:rig
rook	A	Verbrechen	durch Betrug berauben; verb:02572119:rook
run	A	Verbrechen	illegal handeln, etwa mit Waffen oder Alkohol; verb:02242256:run
running	A	Verbrechen	illegal handeln, etwa mit Waffen oder Alkohol; verb:02242256:run
scam	A	Verbrechen	betrügerisches Geschäftsmodell; noun:00754280:scam
sharpie	A	Verbrechen	professioneller betrügender Kartenspieler; noun:09894909:sharpie
shoplift	A	Verbrechen	im Geschäft stehlen; verb:02277303:shoplift
smuggler	A	Verbrechen	ohne Abgaben Waren ein- oder ausführende Person; noun:10615334:smuggler
smuggling	A	Verbrechen	verbotene oder abgabenpflichtige Waren heimlich einführen; noun:01112132:smuggling
squat	A	Verbrechen	eine Wohnung illegal besetzen; verb:02649712:squat
squatted	A	Verbrechen	eine Wohnung illegal besetzen; verb:02649712:squat
squatting	A	Verbrechen	eine Wohnung illegal besetzen; verb:02649712:squat
steal	A	Verbrechen	ohne Zustimmung des Eigentümers wegnehmen; verb:02321757:steal
stole	A	Verbrechen	ohne Zustimmung des Eigentümers wegnehmen; verb:02321757:steal
theft	A	Verbrechen	rechtswidriges Wegnehmen; noun:00780889:theft
thieving	A	Verbrechen	zum Diebstahl neigend; adj:01225294:thieving
thievish	A	Verbrechen	zum Diebstahl neigend; adj:01225294:thievish
torch	A	Verbrechen	böswillig durch Brandstiftung verbrennen; verb:00379280:torch
traffic	A	Verbrechen	illegal handeln; verb:02261123:traffic
treason	A	Verbrechen	Straftat gegen die eigene Regierung; noun:00782072:treason
trespass	A	Verbrechen	unrechtmäßig fremdes Grundstück betreten; verb:02571251:trespass
tribute	A	Verbrechen	unter Gewaltandrohung erpresste Zahlung an Gangster; noun:00784755:tribute
unlawful	A	Verbrechen	gesetzeswidrig beziehungsweise gesetzlich verboten; adj:01396047:unlawful
```

EN-A: Drogen und Alkohol — 15 Wörter.

```tsv
chalk	A	Drogen	Amphetaminderivat Methedrine; noun:03754295:chalk
cobbler	A	Alkohol	alkoholisches Mischgetränk mit Wein oder Spirituose; noun:07913774:cobbler
dime	A	Drogen	Zehn-Dollar-Päckchen illegaler Drogen; noun:03198819:dime
dot	A	Drogen	Straßenname für LSD; noun:02675657:dot
fifth	A	Alkohol	Spirituosenmenge von einem Fünftel US-Gallone; noun:13619475:fifth
filler	A	Drogen	Tabakfüllung einer Zigarre; noun:03338009:filler
flip	A	Alkohol	alkoholisches Mischgetränk mit geschlagenem Ei; noun:07914887:flip
generic	A	Alkohol	Verschnittwein ohne vorherrschende Rebsorte; noun:07899976:generic
huff	A	Drogen	Freizeitdrogen inhalieren; verb:01200245:huff
malt	A	Alkohol	stark alkoholhaltiges Lagerbier; noun:07889274:malt
pane	A	Drogen	Straßenname für LSD; noun:02675657:pane
schnapps	A	Alkohol	starke Spirituose; noun:07905474:schnapps
sour	A	Alkohol	Cocktail aus Spirituose, Zitrussaft und Zucker; noun:07918028:sour
suds	A	Alkohol	dysphemistische Bezeichnung für Bier; noun:07887192:suds
superman	A	Drogen	Straßenname für LSD; noun:02675657:superman
```

EN-A: Gewalt, Waffen, Töten und konkrete militärische Bedeutungen — 65 Wörter.

```tsv
automatic	A	Waffen	automatische Pistole; noun:02760658:automatic
blazing	A	Gewalt_Waffen	schnell und wiederholt schießen; verb:01135086:blaze
boots	A	Gewalt_Folter	Fußfolterinstrument beziehungsweise entsprechende Folter; noun:00422261:boot
bulldog	A	Gewalt	bösartig und heftig angreifen; verb:01119030:bulldog
canister	A	Waffen	mit Geschossen gefüllte Schusswaffenmunition; noun:02976641:canister
cat	A	Gewalt_Waffen	neunschwänzige Peitsche; noun:02985606:cat
crimson	A	Gewalt	durch Gewalt oder Blutvergießen gekennzeichnet; adj:00248560:crimson
destruct	A	Waffen	eigene Rakete beziehungsweise eigenen Flugkörper zerstören; verb:01620688:destruct
dress	A	Toeten	zum Verkauf oder Verzehr töten und herrichten; verb:00542809:dress
engine	A	Waffen	Kriegsgerät wie Rammbock, Katapult oder Geschütz; noun:03288003:engine
eradicate	A	Toeten	in großer Zahl töten; verb:00470701:eradicate
flight	A	Gewalt_Waffen	einen Vogel im Flug schießen; verb:02484771:flight
flypaper	A	Toeten	vergiftetes oder klebriges Papier zum Töten von Fliegen; noun:14872572:flypaper
frying	A	Toeten	durch elektrischen Strom hinrichten; verb:02484957:fry
girdle	A	Toeten	durch ringförmiges Durchtrennen der Versorgungsbahnen töten; verb:01608122:girdle
goad	A	Gewalt	stechen oder wie mit einem spitzen Stock antreiben; verb:01240935:goad
grape	A	Waffen	aus einer Kanone gemeinsam verschossene kleine Geschosse; noun:03453547:grape
hassle	A	Gewalt	ungeordnete Schlägerei; noun:01172441:hassle
hatchet	A	Waffen	Kampfaxt; noun:04449966:hatchet
lather	A	Gewalt	mit Peitsche oder Stock schwer schlagen; verb:01411085:lather
lurk	A	Gewalt	im Versteck auf einen Angriff warten; verb:01138204:lurk
magazine	A	Waffen	Patronenmagazin einer Schusswaffe; noun:02973017:magazine
manual	A	Waffen	militärische Gewehrgriffe beziehungsweise Gewehrdrill; noun:00895135:manual
nest	A	Waffen	Geschützstellung; noun:03819217:nest
nuclear	A	Waffen	auf zerstörerischer Kernenergie beruhende Waffen betreffend; adj:00610532:nuclear
occupier	A	Gewalt_Unterdrueckung	Angehöriger einer militärischen Besatzungsmacht; noun:10369955:occupier
occupy	A	Gewalt_Unterdrueckung	militärisch zur Eroberung und Besetzung einmarschieren; verb:01126360:occupy
onset	A	Gewalt_Waffen	militärischer Angriff mit Waffen; noun:00972621:onset
overlaid	A	Toeten	durch Daraufliegen töten; verb:01326890:overlay
overlay	A	Toeten	durch Daraufliegen töten; verb:01326890:overlay
overlying	A	Toeten	durch Daraufliegen töten; verb:01326890:overlie
pellet	A	Waffen	aus einer Feuerwaffe verschossenes festes Geschoss; noun:04205759:pellet
poking	A	Gewalt	mit Hand, Faust oder schwerem Gegenstand hart schlagen; verb:01414626:poke
predator	A	Gewalt	Person, die auf der Suche nach Beute angreift; noun:10293172:predator
prime	A	Waffen	Zündmittel vor Detonation oder Schuss einsetzen; verb:00408624:prime
quarrel	A	Waffen	Armbrustbolzen; noun:04031481:quarrel
raven	A	Gewalt	mit Gewalt erlangen oder ergreifen; verb:01213908:raven
removed	A	Toeten	vorsätzlich und geplant töten; verb:02482425:remove
removing	A	Toeten	vorsätzlich und geplant töten; verb:02482425:remove
repulsion	A	Gewalt	einen Angriff zurückschlagen; noun:00961001:repulsion
rumble	A	Gewalt	Kampf rivalisierender Jugendbanden; noun:01176649:rumble
silo	A	Waffen	unterirdische Anlage zum Lagern und Starten ballistischer Raketen; noun:04220344:silo
spoil	A	Gewalt	mit Gewalt, besonders im Krieg, erbeutete Wertsachen; noun:13262462:spoil
spoiler	A	Gewalt	Plünderer beziehungsweise Kriegsbeutenehmer; noun:10443170:spoiler
steel	A	Waffen	Schwert beziehungsweise lange Hieb- oder Stichwaffe; noun:04373894:steel
submarine	A	Gewalt_Waffen	mit einem U-Boot angreifen; verb:01119803:submarine
surprise	A	Gewalt	plötzlich oder im Sturm angreifen; verb:01126051:surprise
tank	A	Waffen	gepanzerter militärischer Kampfwagen mit Kanone; noun:04389033:tank
tuck	A	Waffen	gerades zweischneidiges Schwert; noun:04054361:tuck
wage	A	Gewalt	Krieg oder Schlachten führen; verb:02376089:wage
waged	A	Gewalt	Krieg oder Schlachten führen; verb:02376089:wage
wages	A	Gewalt	Krieg oder Schlachten führen; verb:02376089:wage
wheel	A	Gewalt_Folter	Folterinstrument zum Strecken oder Verstümmeln; noun:04039041:wheel
zero	A	Waffen	Visiereinstellung einer Schusswaffe; noun:04614505:zero
lock	A	Waffen	Mechanismus zum Zünden einer Schusswaffenladung; noun:03683341:lock
jacket	A	Waffen	Metallmantel bestimmter Munition; noun:03590475:jacket
lead	A	Waffen	Vorhaltewinkel beim Zielen mit einer Schusswaffe; noun:13893129:lead
live	A	Waffen	mit Explosivstoff geladen; adj:01424596:live
platform	A	Waffen	militärische Struktur oder Fahrzeug als Waffenträger; noun:03962525:platform
breeches	A	Waffen	Plural von breech als hinterer Verschlussbereich eines Waffenlaufs; noun:02895881:breech
field	A	Gewalt	Schlachtfeld; noun:08506641:field
foot	A	Gewalt_Militaer	Infanterieeinheit mit zu Fuß kämpfenden Soldaten; noun:08390157:foot
theater	A	Gewalt_Militaer	Gebiet aktiver militärischer Operationen; noun:08551628:theater
vane	A	Waffen	Leitflosse eines Pfeils, einer Bombe oder Rakete; noun:04520784:vane
unloaded	A	Waffen	bei Waffen ausdrücklich ohne Munition; adj:01424868:unloaded
```

`grape` steht hier wegen der **eigenständigen Geschossbedeutung**, nicht wegen des Zeichenfragments `rape`.

EN-A: Krankheit, Verletzung, Eiter und Beeinträchtigungen — 24 Wörter.

```tsv
broke	A	Verletzung	einen Knochen brechen; verb:00107739:break
catching	A	Krankheit	sich infizieren; noun:00077981:catching
game	A	Behinderung	an Füßen oder Beinen beeinträchtigt; adj:01018788:game
halt	A	Behinderung	an Füßen oder Beinen beeinträchtigt; adj:01018788:halt
mice	A	Verletzung	Pluralzuordnung zu mouse als geschwollener Bluterguss durch Schlag ans Auge; noun:14289387:mouse
mouse	A	Verletzung	geschwollener Bluterguss durch Schlag ans Auge; noun:14289387:mouse
maturing	A	Eiter	zum Reifen und Absondern von Eiter bringen; verb:00097179:mature
nebula	A	Krankheit	pathologische Trübung der Hornhaut; noun:04682184:nebula
radiation	A	Krankheit	Strahlenkrankheit; noun:14308472:radiation
slice	A	Verletzung	Schnittwunde; noun:14287113:slice
smartness	A	Verletzung_Schmerz	Wund-, Verbrennungs- oder Entzündungsschmerz; noun:14331873:smartness
sterile	A	Fortpflanzungsbeeintraechtigung	fortpflanzungsunfähig; adj:01002377:sterile
sterility	A	Fortpflanzungsbeeintraechtigung	Unfruchtbarkeit beziehungsweise Unfähigkeit zu Empfängnis oder Zeugung; noun:14045507:sterility
sterilize	A	Fortpflanzungsbeeintraechtigung	unfruchtbar machen; verb:00061595:sterilize
unhearing	A	Behinderung	vollständig gehörlos; adj:00682521:unhearing
vertigo	A	Krankheit_Symptom	Drehschwindel mit Fallgefühl; noun:14372513:vertigo
absence	A	Krankheit	kurzer Bewusstseinsverlust bei Petit-mal-Epilepsie; noun:14088412:absence
bunt	A	Krankheit	Weizenkrankheit mit durch Pilzsporen ersetzten Körnern; noun:14284026:bunt
founder	A	Krankheit	Entzündung des Huflamellengewebes; noun:14348895:founder
mosaic	A	Krankheit	virale Pflanzenkrankheit; noun:14280775:mosaic
rust	A	Krankheit	Rostkrankheit von Pflanzen; noun:14277655:rust
wilt	A	Krankheit	Welkekrankheit von Pflanzen; noun:14283178:wilt
deafening	A	Verletzung_Behinderung	gehörlos machen; verb:02189940:deafen
certified	A	Krankheit_Abwertung	als geisteskrank einzustufen; adj:02075594:certified
```

EN-A: Tod, Leichen und Nachlass — 17 Wörter.

```tsv
asleep	A	Tod	tot; adj:00095873:asleep
clay	A	Leiche	menschlicher Leichnam; noun:05218119:clay
departure	A	Tod	euphemistische Bezeichnung für Tod; noun:07333649:departure
exit	A	Tod	euphemistische Bezeichnung für Tod; noun:07333649:exit
going	A	Tod	euphemistische Bezeichnung für Tod; noun:07333649:going
last	A	Tod	Zeitpunkt des Lebensendes; noun:15143276:last
leave	A	Tod	nach dem eigenen Tod Hinterbliebene zurücklassen; verb:00360092:leave
left	A	Tod	nach dem eigenen Tod Hinterbliebene zurücklassen; verb:00360092:leave
monument	A	Tod	Grabgewölbe; noun:04076713:monument
release	A	Tod	euphemistische Bezeichnung für Tod; noun:07333649:release
repose	A	Tod	tot daliegen; verb:01548072:repose
sleep	A	Tod	euphemistische Bezeichnung für Tod; noun:13962765:sleep
vault	A	Tod	Grabkammer; noun:04523690:vault
viewing	A	Leiche_Trauer	Totenwache über einem Leichnam vor der Bestattung; noun:01029883:viewing
will	A	Tod	testamentarische Verfügung für den Todesfall; noun:06544142:will
willed	A	Tod	nach dem eigenen Tod testamentarisch hinterlassen; verb:02229055:will
willing	A	Tod	nach dem eigenen Tod testamentarisch hinterlassen; verb:02229055:will
```

Die Summe der EN-Blöcke ist **71 + 6 + 29 + 10 + 62 + 15 + 65 + 24 + 17 = 299**.

**Strittige EN-B-Fälle und verworfene A-Indizien.** Diese Liste benennt relevante Grenzfälle; sie ist keine vollständige Aufzählung aller übrigen erlaubten Wörter.

- **Unpassende Lemma-Zuordnung:** `cola → colon` belegt keine Darmbedeutung von `cola`. `coping → cop` belegt keine Diebstahlsbedeutung; die entsprechende Form wäre `copping`. `dined` und `dining` erhalten durch `din` keine Artilleriebedeutung; dessen Formen wären `dinned` und `dinning`. `camper → camp` und `liver → live` liefern keine gesicherte sexuelle beziehungsweise Explosivbedeutung des gewöhnlichen Substantivs.
- **`glasses`: B hinsichtlich des ausgegebenen Drogenindizes.** Die Zuordnung zum Stoffnamen `glass` für Methamphetamin beweist keine entsprechende Bedeutung der Pluralform `glasses`.
- **Neutrale Anatomie/Biologie:** `bladder`, `knee`, `gender`, `male`, `estrogen`, `spore`, `stamina`, `septum`, `vestibule`, `dug`. Anatomische oder biologische Definitionen und die bloße Erwähnung eines Organs in einem Beispiel begründen hier keinen zusätzlich festgestellten sexuellen Gebrauch der Schreibweise.
- **Neutrale Bewegung und Zuwendung:** `pouncing`, `swoop`, `caress`, `kisser`. Die vorliegenden Definitionen ergeben keinen zwingenden sexuellen oder beleidigenden Sinn. Anders liegt es bei den ausdrücklich erotischen Definitionen von `pet` und `neck`.
- **Slang allein genügt nicht:** `sister`, `suit`, `tummy`. Die gelesenen Bedeutungen sind nicht als vulgäre oder beleidigende Verwendung belegt.
- **Milde Wertung beziehungsweise Beschreibung von Unhöflichkeit:** `crust`, `short`, `shorter`, `low`, `lower`, `saucy`, `sauciness`, `tackiness`, `graffiti`. „Unhöflichkeit“, „geschmacklos“ oder „roh ausgeführt“ ist nicht automatisch ein Marker dafür, dass das betreffende Wort selbst vulgär oder eine Personenbeleidigung ist.
- **Versorgung und Hilfsmittel:** `napkin`, `throne`, `latrine`, `laxative`, `bandage`-ähnliche Versorgungssinne von `patch`, ferner `nurse`, `nursing`, `cure`, `remedy`, `prognosis`, `neurology`, `urology`, `urologist`, `aspirin`, `ibuprofen`. Ausscheidungszweck, Behandlung oder Diagnostik allein wurde nicht als Ausscheidung beziehungsweise Krankheit des Wortes gewertet.
- **Medikamenten-/Quellenlücke:** `prozac`, `ritalin` wurden nicht allein aufgrund ihrer Namen zu Rauschdrogen erklärt. Die gebundene Datei enthält für beide `NO_ENTRY`; eine missbräuchliche Nebenbedeutung ist damit nicht belegt.
- **Tabak als illustratives Material:** `chew`, `cud`, `plug` habe ich aus der vorläufigen A-Auswahl genommen. Die Definition eines kaubaren Klumpens mit Tabak als Beispiel ist für diese Auswahl kein hinreichend sauberer eigener Drogensinn.
- **Neutraler Werkzeuggebrauch:** `hew` beschreibt zunächst das Bearbeiten beziehungsweise Fällen mit einer Axt; ohne zusätzlichen Menschen-/Tiergewaltbeleg bleibt dies B.
- **Freiheit ist keine Sklaverei:** `free`, `liberty`, `unshackle`; ebenso begründet gewöhnliche Dienerschaft oder hierarchische Unterordnung allein keinen Sklavereibefund. Die A-Liste nennt dagegen konkrete Zwangs- und Unterwerfungsdefinitionen.
- **Justiz und Rechtsnähe:** `alibi`, `arrest`, `exonerate`, `detective`, `jailer`, `jailhouse`, `parole`, `probation`, `security`, `record`, `reward` sind nicht allein wegen der Erwähnung von Verbrechen A. `jaywalker` ist als Verkehrsregelverletzer belegt, aber damit nicht sicher ein Wort für ein Verbrechen im hier verlangten Sinn. `wrongful` kann eine zivilrechtliche Rechtsverletzung bezeichnen; ich habe es nicht als bestätigten Verbrechensbefund ausgegeben.
- **`joyride`: B auf Basis der gebundenen Quelle.** Dort ist eine Vergnügungsfahrt definiert; Fahrzeugdiebstahl darf nicht ergänzt werden, wenn er in dieser Definition fehlt.
- **Weitere neutrale Hauptsinne mit Fach-/Kontextbezug:** `aide`, `aviation`, `gauge`, `khaki`, `plate`, `post`, `quarters`, `rank`, `sector`, `support`, `umbrella`, `unarmored`, `wing`; ferner `anise`, `hop`, `ferment`, `vinegar`, `vanilla`, `glycerin`, `iodine`. Militärische Verwendung, ein denkbares Material oder technische Alkoholchemie allein ist kein zusätzlicher bestätigter A-Sinn.

Für **Sklaverei im engeren Sinn** ergab die gezielte EN-Suche keinen weiteren sicher belegten erlaubten Sklaven-/Leibeigenschaftsbegriff. Das ist das Ergebnis dieser Restsuche, keine Aussage über die Vollständigkeit aller denkbaren externen Wörterbuchbedeutungen.

Für DE folgt die vollständige Zuordnung. Jedes Wort steht genau einmal unter seiner Hauptkategorie; zusätzliche A-Gründe sind teilweise mitgenannt. **† kennzeichnet eine konservative Grenzentscheidung**, insbesondere bei biologischen, militärischen oder schwach spöttischen Begriffen. Diese Kennzeichnung soll solche Entscheidungen nicht als ebenso eindeutigen Beleg wie etwa `eichel`, `stuhl` oder `degen` ausgeben.

**DE-A: Sexualität, Genitalien und vulgäre Körpermetaphern — 38 Wörter.**

- `akt` — Geschlechtsakt.
- `angemacht`, `anmachen`, `gebaggert` — sexuelle Anmache.
- `balz` — sexuelles Werbeverhalten.
- `beiwohnen`, `besorgen`, `geblasen`, `genagelt`, `knallen`, `verkehr` — belegte sexuelle Handlungen beziehungsweise Geschlechtsverkehr; `geblasen` insbesondere Oralsex.
- `ehebett`† — gebräuchlicher Bezug auf ehelichen Geschlechtsverkehr.
- `eichel` — Penisglied; `eier` — Hoden.
- `eizelle`†, `keimzelle`† — unmittelbar geschlechtliche Fortpflanzungszellen; konservative biologische Auswahl.
- `erregen`, `feucht` — sexuelle Erregung beziehungsweise sexuelle Feuchtigkeit.
- `geoutet`† — auch Veröffentlichung sexueller Orientierung.
- `gepaart`, `paarung` — geschlechtliche Vereinigung, auch bei Tieren.
- `gezeugt` — sexuelle Zeugung.
- `gummi` — Kondom.
- `hammel` — kastrierter Schafbock; zusätzlich Personenbeschimpfung.
- `hengst` — auch sexuell potenter Mann.
- `kimme` — vulgäre Gesäßspaltenbedeutung; zusätzlich Waffenvisierung.
- `lust`, `trieb` — auch sexuelles Verlangen beziehungsweise Geschlechtstrieb.
- `mops` — saloppe Brustbezeichnung.
- `potenz` — sexuelle Leistungsfähigkeit.
- `ritze` — vulgäre Genital-/Gesäßspaltenbedeutung.
- `rohr`, `rute` — Penisbezeichnungen; zusätzlich Waffenlauf beziehungsweise Schlaginstrument.
- `sack` — Hodensack und grobe Personenbeschimpfung.
- `same` — männlicher Samen.
- `sinnlich` — auch sexuell-erotisch.
- `steif` — auch erigiert.
- `strich` — Straßenprostitution.

**DE-A: Abwertung, Beleidigung und grobe Personen-/Körperbezeichnungen — 45 Wörter.**

- `affe`, `esel`, `gimpel`, `hohl`, `kalb`, `kamel`, `pfosten` — abwertende Dummheits- beziehungsweise Unfähigkeitsbezeichnungen.
- `aufziehen`† — jemanden hänseln; konservative Auswahl des persönlich herabsetzenden Gebrauchs.
- `clown` — lächerlicher Mensch.
- `ferkel` — schmutziger beziehungsweise unanständiger Mensch.
- `fett` — herabsetzende Personenbeschreibung.
- `fink` — schimpfende Bezeichnung eines schlechten Menschen.
- `flasche`, `niete`, `pflaume` — unfähiger beziehungsweise versagender Mensch.
- `flegel`, `klotz`, `rohling` — grober, ungezogener beziehungsweise gewalttätiger Mensch.
- `geek`† — konservativ wegen der spöttisch abwertenden Personenverwendung; neutrale Selbstbezeichnungen ändern diese Triage nicht.
- `geier`, `schakal` — habgieriger beziehungsweise skrupelloser Ausnutzer.
- `gurke`, `hund` — personenbezogene Schimpfverwendungen.
- `herziehen` — über jemanden abwertend schimpfen.
- `kauz`†, `vogel`† — spöttisch abwertende Bezeichnung eines sonderbaren Menschen.
- `klappe` — grobe Mundbezeichnung, insbesondere im Beschimpfungsgebrauch.
- `klops` — abwertend dicker Mensch.
- `kuh`, `pute` — abwertende Frauenbezeichnungen.
- `kuli` — historisch rassistisch abwertende Personenbezeichnung.
- `landei` — herabsetzende Bezeichnung eines Menschen vom Land.
- `landratte`†, `langohr`†, `lausbub`† — spöttische beziehungsweise tadelnde Personenbezeichnungen; konservativ A.
- `lappen` — schwacher beziehungsweise verachteter Mensch.
- `macke`†, `meise` — abwertender Gebrauch für einen vermeintlichen geistigen Defekt.
- `mimose` — abwertend überempfindlicher Mensch.
- `pfeife` — unfähiger Mensch; zusätzlich Tabakrauchgerät.
- `schaf` — einfältiger Mensch.
- `schlange` — hinterhältige Frau.
- `wurm` — herabsetzende Personenbezeichnung; zusätzlich Darmparasit.
- `wurst`† — verächtlich unbedeutender Mensch.
- `zwerg` — herabsetzende Verwendung für einen kleinwüchsigen Menschen.

**DE-A: Gewalt, Waffen, Kampf und militärische Unterwerfung — 64 Wörter.**

- `abreibung` — Prügel; `abzug` — Waffenabzug.
- `aikido` — Kampfkunst.
- `anfallen`, `anfielen` — tätlich angreifen.
- `batterie` — Artillerieeinheit.
- `beil` — auch Hiebwaffe.
- `besatzung`, `besetzen` — militärische Besatzung beziehungsweise Unterwerfung.
- `bogen` — Schusswaffe.
- `bollwerk`†, `burg`†, `kastell`† — ausdrücklich zur militärischen Verteidigung bestimmte Anlagen; konservative Auswahl.
- `boxen`, `boxkampf`, `boxring`, `boxte`, `geboxt` — Faustkampf beziehungsweise unmittelbar dafür bestimmte Kampfstätte.
- `brisant` — hohe Sprengwirkung.
- `clinchen` — körperliche Umklammerung im Boxkampf.
- `coup`† — auch Staatsstreich.
- `degen` — Stichwaffe.
- `dreschen`, `gehauen`, `hauen` — auch körperlich schlagen beziehungsweise verprügeln.
- `drillen`†, `gedrillt`† — militärischer Zwangsdrill; konservative Auswahl.
- `einfall` — militärische Invasion.
- `einnehmen`, `einnimmt`, `erobern` — militärisch erobern.
- `entern`, `kapern` — gewaltsam ein Schiff oder Fahrzeug übernehmen.
- `fechten`, `fochten`, `gefochten` — mit Waffen kämpfen.
- `feuer` — Beschuss.
- `garde`†, `legion`† — militärische Verbände.
- `geladen` — auch schussbereit geladene Waffe.
- `gerammt`, `rammen`, `rammt` — gewaltsam gegen jemanden stoßen beziehungsweise anfahren.
- `gerangel`, `ringkampf` — körperlicher Kampf.
- `gerte` — auch Schlaginstrument.
- `gewappnet`, `wappnen` — bewaffnet beziehungsweise bewaffnen.
- `kadett`† — militärischer Offiziersanwärter.
- `klinge` — auch Schwert-/Dolchklinge.
- `knarren` — Schusswaffen.
- `kugel`, `pfeil` — Waffengeschosse.
- `magazin` — Munitionsbehälter einer Schusswaffe.
- `paladin`† — bewaffneter Ritter beziehungsweise Gefolgsmann.
- `parieren` — auch Waffenangriff abwehren.
- `punch` — Faustschlag.
- `rakete` — auch militärischer Flugkörper.
- `schellen` — Ohrfeigen.
- `sturm` — militärischer Angriff.
- `treten`, `tritt` — körperlicher Schlag mit dem Fuß.
- `visier` — Zielvorrichtung einer Schusswaffe.
- `vorhut`† — vorgeschobener militärischer Truppenteil.

**DE-A: Krankheit, Verletzung und Beeinträchtigung — 29 Wörter.**

- `abgehackt`, `abhauen` — Körperteile abtrennen.
- `anfahren`, `umfahren` — Menschen durch Fahrzeugkontakt verletzen.
- `anstecken` — Infektion übertragen.
- `benommen`† — beeinträchtigter Bewusstseinszustand; konservative Symptomauswahl.
- `beule` — verletzungsbedingte Schwellung.
- `blase` — auch Hautblase; die neutrale Harnblase ist nicht der Ausschlussgrund.
- `brechen` — Knochen brechen; außerdem Erbrechen.
- `bruch`, `fraktur` — Knochenbruch; `bruch` außerdem Hernie.
- `buckel` — auch pathologische Rückenverformung.
- `darnieder` — gebräuchlich für krankes Daniederliegen.
- `flechte`, `pilz` — Hautkrankheit beziehungsweise Pilzinfektion.
- `geblendet` — Augenlicht genommen.
- `hinken`, `hinkt`, `humpelt` — beeinträchtigter Gang.
- `kneifen`, `zwicken` — schmerzhaftes Zusammendrücken von Haut/Körperteilen.
- `kratzen`, `riss` — Haut-/Gewebeverletzung.
- `pickel` — entzündliche Hauterhebung.
- `reizung` — auch entzündliche beziehungsweise schmerzhafte Gewebereaktion.
- `stumm` — auch krankheitsbedingte Sprechunfähigkeit.
- `taube` — auch flektierte Form für fehlendes Hörvermögen.
- `tick` — unwillkürliche wiederkehrende Bewegung oder Lautäußerung.
- `toll` — auch tollwütig.

**DE-A: Tod, Leiche, Trauer, Töten und Nachlass — 19 Wörter.**

- `abgang` — auch Fehlgeburt.
- `elegie` — auch Totenklage.
- `erbe`, `erbfolge`, `erbin`, `erbschaft`, `erbteil`, `erbten`, `geerbt`, `nachlass`, `vererben`, `vermachen` — ausdrücklich auch Hinterlassenschaft beziehungsweise Rechtsnachfolge nach einem Todesfall.
- `erledigen`, `umlegen` — jemanden töten.
- `gefallen` — im Krieg getötet.
- `geist`, `jenseits` — Totengeist beziehungsweise Totenreich.
- `kreuz` — auch Hinrichtungsinstrument.
- `sektion` — Untersuchung eines Leichnams.

**DE-A: Drogen, Alkohol und Rausch — 25 Wörter.**

- `bar`, `beisel`, `bodega`, `kneipe`, `minibar`, `saloon`, `taverne` — ausdrücklich auch Alkoholausschank, Weinlokal oder Alkoholvorrat.
- `blau`, `breit`, `voll` — betrunken beziehungsweise berauscht.
- `bock`, `cidre`, `enzian`, `korn`, `trunk` — alkoholische Getränke beziehungsweise entsprechende Getränkebedeutung.
- `fahne` — Alkoholgeruch im Atem.
- `gelage`, `umtrunk` — alkoholbezogener gemeinsamer Konsum beziehungsweise Saufgelage.
- `getrunken`, `trinken` — auch eigenständiger Gebrauch für Alkoholkonsum.
- `gras`, `kraut` — Cannabis.
- `kippen` — Zigaretten.
- `stoff` — Rauschgift.
- `winzer` — Herstellung von Wein.

**DE-A: Ausscheidung — 4 Wörter.**

- `gespuckt`† — Speichel ausgestoßen; konservative Ausscheidungsauswahl.
- `losung` — Kot von Wildtieren.
- `periode`† — Menstruation beziehungsweise ausgeschiedene Monatsblutung; konservative physiologische Auswahl.
- `stuhl` — Darmausscheidung.

Die DE-A-Kategorien ergeben **38 + 45 + 64 + 29 + 19 + 25 + 4 = 224 Wörter**.

**DE-B: Milde Wertungen und Beschreibungen — 14 Wörter.**

- `abklatsch` — minderwertige Nachahmung; kein notwendiges Menschen-Schimpfwort.
- `albern` — milde Wertung „töricht“.
- `barsch` — schroff/unfreundlich.
- `billig` — auch geringe Qualität.
- `bissig` — scharf spöttisch; kein zwingender Personenbeleidigungsbegriff.
- `eselei` — törichte Handlung, milde Wertung.
- `feige` — Eigenschaftswertung; die Meldung darf daraus nicht das andere Wort `Feigling` machen.
- `frech` — unverschämt, milde Wertung.
- `gaul` — abwertende Tierbezeichnung; der Bericht belegt damit keine abwertende Menschenbezeichnung.
- `infantil`, `kindisch` — milde Unreife-Wertungen in der gemeldeten Verwendung.
- `karikatur` — satirische Darstellung; Herabsetzung ist möglicher Gebrauch, nicht notwendig die Wortbedeutung.
- `reizfigur` — Person, die negative Reaktionen auslöst; die Bezeichnung ist nicht schon eine Beschimpfung.
- `weltfremd` — mangelnder Wirklichkeitsbezug, milde Wertung.

**DE-B: Neutrale Hauptsinne mit militärischem, historischem oder spezialisiertem Kontext — 27 Wörter.**

- `abmarsch`, `anmarsch` — Weggehen beziehungsweise Herankommen.
- `abwehr`, `deckung` — allgemeine Abwehr beziehungsweise Schutz/Abdeckung.
- `alliierte` — Verbündete; politische oder militärische Bündniszugehörigkeit allein ist kein Gewaltwort.
- `ansturm` — auch gewöhnlicher großer Andrang.
- `division` — unter anderem mathematische Teilung; militärischer Fachsinn muss nicht die Freigabe verhindern.
- `drohne` — Tier beziehungsweise unbemanntes Fluggerät; Bewaffnung ist eine zusätzliche Ausführung, keine notwendige Eigenschaft.
- `einheit` — allgemeine Einheit.
- `einsatz` — allgemeine Verwendung/Tätigkeit.
- `einziehen`, `einzog`, `einzug` — Einzug/Einziehen in neutralen Hauptsinnen; Einberufung als spezialisierter militärischer Nebensinn.
- `entladen`, `entlud` — Last entfernen. Eine Waffe als mögliches Objekt macht das allgemeine Verb nicht zwingend zum Waffenwort.
- `flanke` — Körper-/Raumseite beziehungsweise Sportsinn; militärischer Kontext ist spezialisiert.
- `graben` — Erdvertiefung beziehungsweise graben; Schützengraben als militärischer Kontext.
- `granat` — Edelstein; seltener historischer Geschosssinn darf nach der Seltenheitsregel B bleiben.
- `haudegen` — gebräuchlich erfahrener, robuster alter Praktiker; historischer Kämpfersinn ist kein zwingender Ausschluss.
- `helm` — neutrale Schutzausrüstung.
- `kampagne` — organisierte Aktion; historischer Feldzugsinn ist nicht zwingend.
- `kriegen` — gewöhnlich bekommen; das Kriegsverb ist selten/altertümlich.
- `marsch` — Gehbewegung oder Musikform; militärische Verwendung allein genügt nicht.
- `parade` — Schau/Präsentation; Truppenschau ist keine notwendige Gewalthandlung.
- `wehrhaft` — fähig, sich zu wehren; kein notwendiger Waffen- oder Gewaltsinn.
- `zug` — viele neutrale Hauptsinne; militärische Teileinheit als Fachsinn.
- `zuschlag` — vor allem Aufpreis beziehungsweise Zuschlagserteilung; der gemeldete körperliche Schlag ist kein zwingender Ausschluss des neutralen Hauptsinns.

**DE-B: Versorgung, Untersuchung und Hilfsmittel — 9 Wörter.**

- `bandage` — Versorgung/Hilfsmittel; keine Wunde selbst.
- `beatmen` — neutrale Versorgung.
- `befund` — Untersuchungsergebnis, das auch unauffällig sein kann.
- `bettruhe` — Ruhe-/Versorgungsmaßnahme.
- `binde` — auch Menstruationshilfsmittel; das Hilfsmittel ist keine Ausscheidung.
- `klopapier`, `klosett`, `toilette` — Hygienehilfsmittel beziehungsweise Einrichtung; der Ausscheidungszweck ist keine eigene Ausscheidungsbedeutung.
- `zapfhahn` — neutrales Werkzeug für Flüssigkeiten, auch für nichtalkoholische.

**DE-B: Gewöhnliche Empfindungen, Vorgänge und Zustände — 14 Wörter.**

- `atemlos` — etwa nach normaler Anstrengung; keine notwendige Krankheit.
- `blues` — Musik beziehungsweise niedergeschlagene Stimmung; nicht automatisch klinische Depression.
- `gezittert` — kann Kälte, Aufregung oder gewöhnliche Muskelreaktion ausdrücken.
- `jucken`, `juckt` — Empfindung, die nicht notwendig Krankheit ist.
- `keim` — auch Pflanzenkeim; selbst ein Mikroorganismus ist nicht notwendig Krankheitserreger.
- `kribbeln` — gewöhnliche Empfindung; nicht notwendig eine diagnostizierte Störung.
- `schub` — neutraler Bewegungs-/Kraftbegriff; Krankheitsschub ist ein kontextgebundener Nebensinn.
- `schuppen` — auch normale Hautablösung; keine notwendige Krankheit.
- `stumpf` — gewöhnlich nicht scharf beziehungsweise wenig empfindsam; keine notwendige Behinderungsbezeichnung.
- `wehen` — gewöhnlicher Geburtsvorgang; Schmerz allein macht ihn nicht zur Verletzung.
- `zucken`, `zuckt` — gewöhnliche Bewegung/Reaktion; keine notwendige Krankheit.
- `zuziehen` — allgemeines Erwerben beziehungsweise Schließen; die Objekte „Krankheit“ und „Verletzung“ sind keine eigene Krankheitsbedeutung des Verbs.

**DE-B: Allgemeine Erinnerung, Klage und Material — 6 Wörter.**

- `asche` — allgemeiner Verbrennungsrückstand; Einäscherung ist ein möglicher Herkunftskontext.
- `gedenken` — erinnern; auch lebender Menschen oder Ereignisse.
- `heulen`, `heult` — allgemein weinen beziehungsweise einen Laut ausstoßen; Trauer um Tote ist kein notwendiger Sinn.
- `klage`, `klagt` — Beschwerde, Schmerz- oder Unmutsäußerung; kein notwendiger Todes-/Trauersinn.

**DE-B: Behälter, Rechnung und allgemeiner Flüssigkeitsvorgang — 5 Wörter.**

- `gezapft`, `zapfen`, `zapft` — Flüssigkeit entnehmen; Bier ist ein mögliches Objekt, kein notwendiger Bestandteil der Wortbedeutung.
- `pulle` — saloppe Flaschenbezeichnung; eine Alkoholflasche ist zunächst eine mögliche Füllung.
- `zeche` — Rechnung für konsumierte Speisen/Getränke; Alkohol ist nicht notwendig.

Die DE-B-Kategorien ergeben **14 + 27 + 9 + 14 + 6 + 5 = 75 Wörter**. Damit sind sämtliche **299 DE-Meldungen genau einmal** erfasst.
