# Einmaliges unabhängiges Review vor dem Ursachen-Korrekturdurchgang

Beauftragtes Modell claudeAgent/claude-opus-5-5, effort=medium, fastMode=false.
Das folgende Urteil gehört zu den unten genannten Vor-Korrektur-Hashes.
Die A-Befunde werden mit der ersten eigenen Probe in genau einem
Ursachen-Korrekturdurchgang behandelt. Keine weitere Review-Runde.

# Review des gestagten Fixdiffs (Stufe 2, EN/LA/DE-Wortlisten)

**Ergebnis: FAIL.** In der EN-Endliste stehen Wörter mit Klasse-A-Bedeutung, auch einer in meiner eigenen Stichprobe. LA zeigt in Stichprobe und Schlagwortsuche keinen A-Fall. Der Code hat keinen blockierenden Mangel. Vollständige sprachliche Fehlerfreiheit behaupte ich nicht.

## Wortlistenhashes
Alle drei stimmen mit den `.sha256`-Dateien überein.
- EN `0d7cff9c907a298b40e484f947f69fe777f64b7639a4aa4052ac038a077aa575`
- LA `d3417f6ef5185b0df23242632e57478855d594ab5be0efd48ce74724f03298df`
- DE `02911732a22dbba0967453121e60d245bc7452e13383d808acfbbef47efb2ed0`

Die Root-Probe `stage2-own-sample.tsv` (`1baa52ac…3683`) hat noch 600 unbewertete Zeilen. Das Gate scheitert derzeit also erwartungsgemäß.

## Eigene Stichprobe
- **Seed:** `20261025-review-worker-ab200-v1`
- **Verfahren:** `SHA256(UTF8(seed|lang|word))`, je Sprache die 100 kleinsten Hashes. Ich habe alle 200 Wörter mit dem vollständigen WordNet-Text bzw. allen LA-Glossen gelesen.
- Sie überschneidet sich in 6 Wörtern mit der Root-Probe.
- Gerechnet wurde nur im Speicher bzw. in `/tmp` außerhalb des Repos.

**EN (100):** aloud antics antihero anvil apple astir ate awake award belle blinker board briskness bytes carbon carpentry catchy celery ceremony choice coleslaw crawling crayfish creature deduce deferred detract devoutly disband dwell dweller earthlike ecology edgy elastic eloquence equator expansive faded fernlike fiction finch flight full generous grating gravitate grimace hardly hedge hoist ibex ionic jot lyrics mahogany manpower meow mink monsoon order overhear ozone paced parmesan perk plywood polymer proof provolone pumice recent reformist research reshape retail robotics ropelike safari sandworm seltzer shave smugly spent spongy spotted spry strenuous subsector swagger toy uncurled undoing unify unmarked varnish wasabi whirl wildly zesty

**LA (100):** abditus acipenser adequito advolo aeternum aevum alii aliusmodi amabilis ammodum animatus arbustum cenito cis citimus claro coena comprobo concino condono conrectio construo contexo cunela custodela debitus dilexio emax epula esurio evincio exaggero exatio femen fraternus funale glis gyrus harena incola incubito indictus infusio insculpo introrsus jugis juventa lacerta lapillus lectio magnes malva marmus menstruus minuo necopmus nox numerus occiduus oesypum oppido oppono patro pendulus perlito perplexus praecaveo praecingo pridem provolo puppis purpureus quoquo receptum recurvo refutatio renavigo renidesco salinum sancio seculum setius sibilo silens simulans sponsal statera subveho suppeto symbola tela tympanum umidus umor vanitas velivolus velut vitulus voco voluntas

| Sprache | A | B | N |
|---|---|---|---|
| EN | **1** | 5 | 94 |
| LA | 0 | 7 | 93 |

- **EN A:** `creature` mit `noun:09976551`: „a person who is controlled by others and is used to perform unpleasant or dishonest tasks“. Das ist eine abwertende Menschenbezeichnung, so wie die bereits als A gesperrten `user` und `operator`.
- **EN B:**
  - `wildly` `adv:00174987`: violent passion
  - `proof` `noun:13780180`: Alkoholgehalt
  - `safari` `noun:00309906`: Jagdreise
  - `swagger` `verb:01035199`: einschüchtern
  - `expansive` `adj:00909220`: Größenwahn
- **LA B:**
  - `femen`: Oberschenkel
  - `debitus`: „doomed, fated“
  - `perlito` und `condono`: Opfer
  - `vanitas`: „foolishness“
  - `puppis`: „poop“ als Schiffsheck
  - `tela`: Lemma „web“. Das Rohwort `telum` (Waffe) wird zu einer anderen Schreibweise decodiert, aber der häufige Plural „tela“ heißt „Waffen“. Ich werte das als Homograph-Hinweis, nicht als A.

Ich habe die DICTLINE-Rohzeilen für nox/noxa, coena/coenum, menstruus/menstruum, tela/telum, lacerta/lacertus und debitus gegengeprüft. Alle zugeordneten Glossen sind enthalten, die belasteten Nachbarn wie `telum`, `coenum`, `noxa`, `menstruum` und `menstruo` stehen nicht in der Liste.

## Curation-Befunde außerhalb der Stichprobe
Gefunden habe ich sie, indem ich die Personen-Synsets der EN-Endliste gezielt gelesen und mit den A-Sperren der eigenen Adjudikation verglichen habe.

1. **Hoch – `cat`.** `stage2-decisions.tsv` lässt das Wort ausdrücklich als B zu, die Begründung nennt aber nur die Peitsche. Übersehen ist `noun:09900153` „a spiteful woman gossip“. Das ist dieselbe Art Befund wie `dragon`, `mare` und `sow`, die als A gesperrt sind.
2. **Hoch – Dummheits- und Opfer-Bezeichnungen**, vergleichbar mit `sheep` (A):
   - `gull` `noun:09921409`: „gullible and easy to take advantage of“
   - `lamb` `noun:10243988`: „a person easily deceived or cheated“
3. **Hoch – abwertende Personenbezeichnungen**, vergleichbar mit `fox`, `sneak`, `user`, `operator`, `wolf` und `wrongdoer` (alle A):
   - `doormat` `10771636`: weak, ineffectual
   - `shark` `10586674`: ruthless, greedy, dishonest
   - `pretender` und `pseudo` `10201535`: deceitful pretenses
   - `lizard` `10274474`: lounge lizard
   - `renegade` `10007109`: disloyal person
   - `puppet` und `creature` `09976551`: Handlanger
4. **Mittel – `dirtiness`** `noun:04903678`: „obscenity in speech or writing“. Das ist ein vulgärer Nebensinn, vergleichbar mit `name`, `shout` und `slang` (A).
5. **A-Verdacht, Wörterbuch nötig – `skirt`** `09989045`: „informal terms for a (young) woman“. In Wörterbüchern gilt das teils als beleidigend, ähnlich wie `mare`. Das Wort liegt in der Root-Probe.

**Ursache:** Die EN-Sichtung lief über gezielte Klassenfilter. Abwertende Personensinne ohne Markerwort sind dabei durchgerutscht. Etwa 591 Endwörter haben zusammen rund 822 Sinne im Offset-Bereich der Personen-Synsets.

**Empfehlung:** Diese Sinne vollständig lesen. Damit das deterministisch geht, sollte der Extraktor das WordNet-Feld `lex_filenum` (Datei `noun.person`) mitschreiben. Derzeit verwirft er es.

## B-Hinweise
Das sind keine A-Behauptungen.
- **`certify`** `verb:00822262`: „declare legally insane“. Das Wort ist erlaubt, obwohl `certified` wegen desselben Sinns als A gesperrt ist. Nach der Rubrik ist das B, aber die beiden Entscheidungen widersprechen sich.
- **Uneinheitliche Behandlung gleichartiger Nebensinne:**
  - `stick` (Joint) und `key` (Kilo Rauschgift) sind erlaubt, `chalk`, `dot`, `pane` und `dime` sind vorsorglich gesperrt.
  - `throne` (Toilette) ist erlaubt, `latrine` ist gestrichen.
  - `seat` (Gesäß) ist erlaubt, `bottom` ist gesperrt, dort aber zusätzlich wegen des Sexualsinns.
- **Weitere Grenzfälle:**
  - `out`/`outing`: Outing der Homosexualität, keine Beleidigung
  - `dug`: Euter, Zitze
  - `snooper`, `boaster`, `tagalong`, `mouse`, `cub`: milde Wertungen
  - LA `confessus` (gestandenes Verbrechen) und `desultor` (wankelmütiger Liebhaber)
- **DE:** `klopapier`, `klosett` und `toilette` sind als B zugelassen, obwohl die Triage selbst einen „unmittelbaren Bezug zu Ausscheidungen“ vermerkt. Das sollte der Lead entscheiden.

## Technik
- **Funktioniert:**
  - `prepare-english-senses.mjs --check` endet mit Exit 0 und reproduziert die Zahlen 9558/8924/634/7772.
  - `git diff --cached --check` ist sauber.
  - `english_sources()` prüft Manifest, Reihenfolge, Präfix und exakte Poolabdeckung.
  - Der Fingerprint bindet den gesamten Quelltext.
  - Die Logik in `check-semantic-sample.ps1` für Ziehen, Überschreibschutz, veraltete Listen und Zählung ist korrekt.
  - Die Negativregressionen (14 Fälle plus `servula`) und die vier positiven B-Fälle sind getestet. Keines der Negativwörter steht in den Listen.
- **Mittel:** Die EN-Begründungen in `semantic-decisions.tsv` sind Textbausteine. Sie zitieren nur den ersten Sinn und schreiben „kein bestätigter A-Befund“, zum Beispiel `gull`: „aquatic bird“. Das klingt nach Einzelprüfung aller Sinne, die es so nicht gab.
- **Niedrig:**
  - Das Gate verlangt bei A/B-Zeilen keine Kategorie, `-` wird akzeptiert.
  - Der Extraktor findet nur kleingeschriebene WordNet-Lemmata und verwirft `lex_filenum`.
- **Nicht ausgeführt:** cargo build, test und clippy, wegen des Verbots von Schreibausgaben. Die vollständige Testsuite muss Root erneut laufen lassen.

Root behebt die A-Befunde. Danach ist nach der Stop-Regel genau eine neue frische Probe zulässig.

## Umgang mit dem einmaligen Review und abschließender STOP

Die 13 bestätigten Review-Wörter `cat creature gull lamb doormat shark
pretender pseudo lizard renegade puppet dirtiness skirt` wurden in dem
einen gemeinsamen Ursachen-Korrekturdurchgang ausgeschlossen.
Die technischen Hinweise wurden behandelt: `lex_filenum`/`lexname` bleibt
im Auszug erhalten; alle 669 noch erlaubten Kandidaten mit 938
`noun.person`-Zuordnungen einschließlich Reserven wurden vollständig
gelesen und quellgebunden erfasst. Das A/B-Gate verlangt bei A/B eine
ausdrückliche Kategorie und Begründung. Die EN-Standardtexte behaupten
keine vollständige Einzelprüfung sämtlicher Sinne mehr.
Großgeschriebene Quelllemmata werden weiterhin nicht pauschal normalisiert;
die dadurch entstehenden Quellenlücken bleiben sichtbar.

Milde Wertungen, neutrale Anatomie und seltene nicht sexuelle/nicht beleidigende
Nebenbedeutungen bleiben nach Rubrik B. Historische oder neue vorsichtige
B-Sperren werden getrennt dokumentiert. Deutsche Toiletten-/Hygienegegenstände
bleiben B, weil der bezeichnete Gegenstand nicht selbst eine Ausscheidung ist.

Es wurde kein zweites Review-Kind beauftragt. Die lokale Wortlisten-Suite
ergab nach Korrektur Exit 0 (35 Integrationstests und 2 Doctests).
Dies ist kein nachträgliches unabhängiges PASS des Reviewers.

**STOP:** Die zweite frische eigene Probe mit Seed
`20261025-kairos-stage2-own-ab-v2` enthält `reptile`.
[Merriam-Webster, Sinn 3](https://www.merriam-webster.com/dictionary/reptile)
belegt einen abwertenden Personensinn, den WordNet nicht enthält.
Das Wort wurde nach dem Fund nicht weiter korrigiert. Die Probe bleibt
ausdrücklich unvollständig; für LA wird kein Null-A-Ergebnis behauptet.
[Abschließender Befund und Prüfgrenzen](../curation-stage-two.md).
