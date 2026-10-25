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

Eine Nebenbedeutung erzwingt nur dann A, wenn sie sexuell, vulgär/obszön oder
ein Slur ist: eine herabsetzende Bezeichnung aufgrund von Ethnie, Religion,
Behinderung, Geschlecht/Sexualität oder Herkunft. Eine bloß beleidigende
Tier- oder Gegenstandsmetapher bei neutraler Hauptbedeutung genügt nicht.
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
  Ein zusätzlicher sexueller, vulgärer/obszöner oder Slur-Sinn bleibt A.
- Neutrale Hauptbedeutungen mit nicht sexueller, nicht vulgärer/obszöner und
  nicht als Slur einzuordnender Nebenbedeutung nahe einer A-Klasse. Beispiel `situs`:
  Lage/Ort, daneben Vernachlässigung/Schimmel.

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

## Entscheidungen und Gates

`semantic-decisions.tsv` deckt weiterhin den ganzen EN/LA-Kandidatenpool
einschließlich Nachrückern ab. Neue A-Sperren erhalten eine konkrete Klasse
und einen wortbezogenen Bedeutungsbeleg. B-Fälle dürfen erlaubt bleiben.
Historische konservative Sperren und ihre Gründe bleiben nachvollziehbar.
Das Stichprobengate unterscheidet explizite `Nebenbedeutung:`-Kategorien:
`Sexualitaet`, `Vulgaer` und `Slur` verlangen A, `Tier-Gegenstandsmetapher`
verlangt B. Ein generischer Nebenbedeutungsgrund allein rechtfertigt kein A.

Die frische Abnahmeprobe umfasst je 300 EN- und LA-Endwörter, deterministisch
mit einem neuen dokumentierten Seed gewählt. Jedes Wort und alle zugeordneten
LA-Glossen werden gelesen. A, B und unauffällige neutrale Wörter werden
getrennt protokolliert. Nur eingebettete A-Fälle lassen das semantische Gate
fehlschlagen. Ein A-Fund erlaubt eine Ursachenbehebung und genau eine neue
frische Probe. Enthält auch diese A-Fälle, gilt STOP.

Eine Bedeutungsquelle oder externe Positivliste liefert überprüfbare Evidenz,
keine automatische A/B-Freigabe. Vollständige technische Abdeckung und grüne
Tests ersetzen die semantische Beurteilung nicht.
