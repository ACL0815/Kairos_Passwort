# Princeton WordNet 3.0

Abruf 25.10.2026. Unverändertes Originalarchiv
[WordNet-3.0.tar.gz](https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz),
Container-SHA-256 `640db279c949a88f61f851dd54ebbb22d003f8b90b85267042ef85a3781d3a52`.
Die [Original-Lizenz](LICENSE) erlaubt Verwendung, Änderung und Weitergabe
unter Beibehaltung von Copyright und Bedingungen. Sie wird zusätzlich in
`../../NOTICE.txt` vollständig weitergegeben. WordNet 3.0 Copyright 2006
by Princeton University. Keine Empfehlung durch Princeton.

`candidate-senses.tsv.gz` ist ein von Kairos erzeugter, gekennzeichneter
Bedeutungsauszug für genau die 9.558 zulässigen EN-Kandidaten. Der Auszug
enthält keine kopierte Implementierung. Jede Zeile hat Wort, TAB, Quelltext.
Der Quelltext enthält das Wort, die Mitgliedschaft in der bereits vendorten
EFF Large Wordlist und alle zugeordneten WordNet-Definitionen mit Wortart,
Synset-Offset, passendem Lemma und lexikographischer Kategorie
(`lex_filenum`/`lexname`, insbesondere `noun.person`).
Die Offset-IDs beziehen sich auf die
jeweilige `dict/data.*`-Datei im Originalarchiv.

Der eigene Extraktor `../../../../tools/prepare-english-senses.mjs` liest
unveränderte `data.noun`, `data.verb`, `data.adj`, `data.adv` und die vier
Ausnahmedateien sowie `lexnames`. Er berücksichtigt exakte Schreibweisen, Ausnahmen und die
einfachen dokumentierten WordNet-Suffixersetzungen. Er vereinigt alle
vorhandenen Treffer, statt nur eine erste Bedeutung auszuwählen. Dies kann
zusätzliche morphologische Homographen aufnehmen. Beispiele in Definitionen
sind keine zusätzlichen Bedeutungen des Eingabeworts. Eine redaktionelle
A/B-Entscheidung bleibt erforderlich.

8.924 Kandidaten besitzen mindestens einen Treffer, 634 haben ausdrücklich
`NO_ENTRY`. Diese Lücken werden nicht als WordNet-Freigabe ausgegeben.
7.772 Kandidaten sind in EFF Large enthalten. Mitgliedschaft ist ein externes
Kurationssignal, keine Zusicherung der A/B-Eignung. Die vorhandenen direkten
EFF-Quellen werden weiterhin unter der dokumentierten CC-BY-4.0-Freigabe
verteilt; ältere Kopien mit CC BY 3.0 werden nicht als neue Quelle übernommen.

Reproduktion ohne Netzwerk, vom Repository aus:

```powershell
node crates/kairos-wordlists/tools/prepare-english-senses.mjs --check
```

Ohne `--check` schreibt derselbe Befehl den Auszug neu. Das Manifest prüft
die dekomprimierten Archiv-/Auszugsbytes und die Lizenz. Jede semantische
EN-Entscheidung bindet den gesamten zugehörigen Quelltext per SHA-256.
Neue Kandidaten, geänderte Quellen oder Bedeutungen erfordern neue explizite
Entscheidungen. Node ist nur für die erneute Extraktion erforderlich;
die beiden Cargo-Pipelines und das Programm arbeiten ohne Node und Netzwerk.

Zusätzliche Befundnotizen stehen in `../../english-extra-evidence.tsv`.
Die 16 kurzen deutschen Feststellungen und Primärquellenlinks werden in
den Auszug aufgenommen und ebenfalls an die semantischen Fingerprints gebunden.
Dies ist kein Import eines vollständigen Wiktionary-Wörterbuchs.
Großgeschriebene WordNet-Lemmata werden nicht pauschal kleingeschrieben;
dadurch können beispielsweise Monatsnamen sichtbare NO_ENTRY-Lücken bleiben.

Der separate Personenbeleg bindet sämtliche gelesenen `noun.person`-Sinne
für 669 zuvor erlaubte Kandidaten. Er kann keine Personensinne erkennen,
die im WordNet-Original fehlen. Der bestätigte Gegenbeleg `reptile`
führte in der zweiten frischen Probe zum
[verbindlichen STOP](../../../curation-stage-two.md).
EFF-Mitgliedschaft und vorhandene WordNet-Einträge sind keine vollständige
semantische Abnahme nach der Kairos-Rubrik.
