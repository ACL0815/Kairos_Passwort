# Verbindlicher Abnahmestandard A/B

Stand 25.10.2026, Europe/Berlin. Dieser Standard ersetzt für Stufe 2 die
frühere Regel „im Zweifel raus“. Bestehende strengere Sperren dürfen bestehen
bleiben; sie werden nicht nachträglich als bestätigte A-Befunde ausgegeben.

## Klasse A: zwingend ausschließen

- Sexualität, Genitalien und Ausscheidungen.
- In der Quelle als vulgär, rude oder obscene markierte Verwendungen.
- Beleidigungen und abwertende Bezeichnungen für Menschen als Hauptbedeutung, einschließlich
  Dummheitslabels wie `bardus` und `morio`.
- Gewalt, Waffen und Töten.
- Tod, Leichen, Trauer und Verwitwung.
- Krankheit, Verletzung, Wunden, Eiter und Behinderungsbezeichnungen.
- Drogen, Alkohol und Rausch.
- Sklaverei und Unterdrückung. `servula` bedeutet Sklavin und ist A.
- Verbrechen.
- Religiös, ethnisch oder politisch herabsetzende beziehungsweise
  extremistische Bezeichnungen.

Eine Nebenbedeutung zählt nur dann als A, wenn das **Wort selbst** in einem
gängigen Wörterbuch (Duden, Oxford/Merriam-Webster, Lewis & Short) als
vulgär, obszön oder diskriminierend markiert ist oder überwiegend so verwendet
wird. Ein sexueller Gebrauch eines Grundverbs oder Allerweltsworts in einer
einzelnen Wendung genügt nicht. Ein Slur ist eine herabsetzende Bezeichnung
aufgrund von Ethnie, Religion, Behinderung, Geschlecht/Sexualität oder Herkunft.
Eine bloß beleidigende Tier- oder Gegenstandsmetapher bei neutraler
Hauptbedeutung genügt ebenfalls nicht.
Die oben genannten A-Klassen gelten weiterhin streng für Hauptbedeutungen.
Für LA werden alle verfügbaren Rohglossen derselben
decodierten Schreibweise gelesen, einschließlich seltener Frequenzen.
Ein Quellenmarker wird im Kontext bewertet: Das neutrale Adjektiv „rau“
ist keine Kennzeichnung einer Verwendung als vulgär.

## Klasse B: akzeptabel

- Neutrale Anatomie, etwa `bladder` und `knee`.
- Neutrale Bewegungs- und Tierverben, etwa `pounce` und `pouncing`.
- Milde Wertungen, etwa `absurd` und `absurde`.
- Tier- und Gegenstandsmetaphern mit neutraler Hauptbedeutung, auch wenn sie
  Menschen abwerten: `reptile`, `snake`, `pig`, `rat`, `cow`, `worm`, `weasel`.
  Ein zusätzlicher vulgärer/obszöner oder diskriminierender Gebrauch wird nach
  der oben festgelegten Wörterbuch-/Gebrauchsschwelle beurteilt.
- Neutrale Hauptbedeutungen mit einer Nebenbedeutung nahe einer A-Klasse,
  die diese Schwelle nicht erreicht. Beispiel `situs`:
  Lage/Ort, daneben Vernachlässigung/Schimmel.
- Ausdrückliche B-Entscheidungen des Leads: `nimmst`, `scio`, `squirrel`,
  `klage`, `kappe`. Alltagssprache und obskure Nebenbedeutungen allein
  rechtfertigen bei diesen Wörtern keinen A-Ausschluss.

Neutrale Versorgung, Werkzeuge, gewöhnliche Körperfunktionen außerhalb
der ausdrücklich ausgeschlossenen Ausscheidungen, Religionen und politische
Überzeugungen sind ohne einen belegten A-Sinn kein A-Befund. Dienerschaft
allein wird von Sklaverei unterschieden. Bloße Zeichenfragmente, etwa
`rape` in `grape`, begründen keine Wortbedeutung.

## Lead-Entscheid Rubrik-Präzisierung

Am 25.10.2026 (Europe/Berlin) hat der Lead die bisherige Regel für
beleidigende Nebenbedeutungen durch die oben präzisierte Regel ersetzt.
`reptile` ist B und bleibt enthalten. Bereits gesperrte Wörter bleiben
gesperrt; diese Präzisierung verlangt keine Rückfreigabe.

Begründung des Leads: Fast jedes Alltagssubstantiv kann als abwertende
Metapher dienen. Die Auswahl soll peinliche oder verletzende Passwörter
verhindern, ohne die Alltagssprache durch jede denkbare Metapher auszudünnen.
Als methodischer Vergleich dient die
[EFF-Wordlist-Kuratierung](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases);
dies ist eine eigene Rubrikentscheidung, keine behauptete EFF-Einzelfreigabe.

## Endgültiger Lead-Entscheid und Abnahme

Die Endstufenentscheidung vom 25.10.2026 (Europe/Berlin) ersetzt die frühere
Nebenbedeutungs- und Stichprobenregel. Zusätzlich gesperrt werden genau
DE `farbig`, `pritsche`, EN `lunchbox`, `pucker`, `joyride` und
LA `lascivio`, `punctim`: vom Lead bestätigte eindeutige Fälle mit
A-Hauptbedeutung oder gängigem vulgärem/diskriminierendem Gebrauch.
Die [Endstufenbegründung](curation-stage-two.md#lead-entscheid-endstufe)
dokumentiert auch die verworfene Stufe 3. Bestehende Sperren bleiben erhalten.

Abnahmestandard ist eine **unabhängige 900er-Stichprobe**, je 300 Wörter aus
DE, EN und LA, mit dokumentiertem Seed und Listenstand. Die A-Quote darf
insgesamt **höchstens 0,5 %** betragen (bei 900 Wörtern höchstens vier A-Fälle);
**kein einziger A-Fall nach Hauptbedeutung** ist zulässig. Die Quote wird über
alle 900 Wörter berechnet, nicht durch Zusammenlegen verschiedener Listenstände.

Als Restrisiko werden **ungefähr 0,4 %** nach der vorhandenen
[Verifier-Matrix](languages/curation-verifier-matrix.md#matrix) dokumentiert
(historisch 4/900 = 0,44 % auf `8696255` unter der damaligen Rubrik).
Das ist eine beobachtete Stichprobenquote, keine Garantie einer exakten
Restquote im gesamten Bestand und keine neue Abnahme des geänderten Heads.
Die sieben Sperren und die endgültige Wortschwelle sind der ausdrückliche
Lead-Entscheid; die historische Matrix wird nicht nachträglich umbewertet.
Eine Nutzer-Sperrliste, mit der Nutzer eigene Wörter ausschließen können,
ist als Folge-Feature geplant und in diesem Auftrag nicht implementiert.

## Entscheidungen und historische Gates

`semantic-decisions.tsv` deckt weiterhin den ganzen EN/LA-Kandidatenpool
einschließlich Nachrückern ab. Neue A-Sperren erhalten eine konkrete Klasse
und einen wortbezogenen Bedeutungsbeleg. B-Fälle dürfen erlaubt bleiben.
Historische konservative Sperren und ihre Gründe bleiben nachvollziehbar.
Das historische Stichprobengate unterscheidet explizite `Nebenbedeutung:`-Kategorien:
`Sexualitaet`, `Vulgaer` und `Slur` verlangen A, `Tier-Gegenstandsmetapher`
verlangt B. Ein generischer Nebenbedeutungsgrund allein rechtfertigt kein A.

Diese frühere Nulltoleranzprüfung umfasst nur je 300 EN- und LA-Endwörter
und ist kein Gate für den endgültigen 900er-Abnahmestandard. Ihre historischen
Proben und Fehlurteile bleiben nachvollziehbar; eine unvollständige oder
veraltete Probe wird weiterhin nicht als bestanden ausgegeben. Der
Implementierungsworker führt keine weitere semantische Runde durch.

Eine Bedeutungsquelle oder externe Positivliste liefert überprüfbare Evidenz,
keine automatische A/B-Freigabe. Vollständige technische Abdeckung und grüne
Tests ersetzen die semantische Beurteilung nicht.
