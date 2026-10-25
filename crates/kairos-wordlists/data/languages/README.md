# Englisch, Latein und die kombinierte Quelle

**Arbeitsstand mit STOP, keine fachliche Freigabe.** Die zweite eigene
Abnahmeprobe findet nach einem Korrekturversuch erneut Sperr-/Zweifelfälle.
Technische Prüfungen und das ursprüngliche 14-Wörter-Gate sind grün.
[Stop-Regel, konkrete Funde und nächster Entscheidungspunkt](../curation-diagnosis.md).

PR #7 ist auf PR #2 gestapelt. Die deutsche Liste und deren Regeln bleiben
bytegleich bei **7.130 Wörtern**. Aktuelle Endgrößen, Mittelwerte, Abzüge und
SHA-256 stehen im erzeugten [Filterbericht](report.txt). Die drei Endlisten
sind paarweise disjunkt; `combined()` erhält das Sprachflag jedes Eintrags.
Im gesicherten Stand: EN/LA **je 6.966**, insgesamt **21.062** Wörter,
Mittel EN **6,541**, LA **6,990**. Die strengere Kuratierung erhöht die Längen;
die Tests begrenzen EN nun auf 6,6 und LA auf 7,0 statt bisher 6,5/6,8.

```rust
use kairos_core::{Language, WordSource};
let source: &dyn WordSource = kairos_wordlists::combined();
assert_eq!(source.language(1), Language::English);
assert_eq!(source.len(), kairos_wordlists::german().len()
    + kairos_wordlists::english().len() + kairos_wordlists::latin().len());
```

`german()`, `english()`, `latin()` und `combined()` liefern gemeinsam
`&'static EmbeddedWordList`. `OnceLock` initialisiert jede Quelle einmal.
Die Laufzeit benötigt keine externen Dateien und kein Netzwerk. Die
Kuratierungsdateien und Rohglossen werden nicht in den Laufzeitindex eingebettet.
`word(index)` und `language(index)` verwenden dieselben Indexgrenzen.

## Semantische Freigabe vor jeder Auswahl

Die zweite Abnahme hat weitere Krankheiten, Verletzungen, Genitalien,
Vulgärmarker, Alkoholbegriffe, Waffen und Abwertungen nachgewiesen, obwohl
Reproduktion und Tests grün waren. Diagnose, verbindliche Rubrik und die eigene
400er-Stichprobe stehen in [curation-diagnosis.md](../curation-diagnosis.md).
Der Keyword-Wächter allein ist kein semantischer Freigabenachweis.

[semantic-decisions.tsv](semantic-decisions.tsv) enthält für **alle 9.558 EN-
und 11.632 LA-Kandidaten** eine Entscheidung, einschließlich bestehender
Endwörter, bisheriger Ausschlüsse und sämtlicher Reserve-/Nachrückerkandidaten.
Spalten ohne Kopfzeile: **Wort | Sprache | Entscheidung | Klasse | Begründung |
Quell-SHA-256**, getrennt durch TAB. Sortierung ordinal nach Sprache, dann Wort.
`allow` verlangt Klasse `neutral`; `block` verlangt eine oder mehrere der Klassen
`Krankheit`, `Verletzung`, `Tod-Trauer`, `Gewalt-Waffen`, `Sexualitaet`,
`Drogen-Alkohol`, `Abwertung`, `Vulgaer`, `Zweifel` (mit Komma getrennt).
**Im Zweifel raus.** Eine problematische Nebenbedeutung genügt zur Sperre.

EN bindet an die geprüfte Schreibweise; die Mitgliedschaft in den unveränderten
Rohquellen wird separat geprüft. LA bindet an sämtliche decodierbaren
Originaldatensätze derselben Schreibweise, einschließlich seltener Frequenzen
außerhalb A–D, mit Zeilennummer, Frequenz und vollständiger Originalglosse.
Eine A–D-Bedeutung ist weiterhin Voraussetzung für die Kandidatenaufnahme.
Ein einzelner neutraler Hauptsinn kann problematische Nebensinne nicht aufheben.

Die Pipeline prüft die vollständige Kandidatenabdeckung und Quellfingerprints,
bevor sie Wörter auswählt. Fehlende, doppelte, ungeordnete oder ungültige
Entscheidungen, unbekannte Wörter und geänderte Glossen brechen den Build ab.
`allow` ist notwendig, zusätzlich gelten weiterhin die bisherigen redaktionellen
Sperren, Schreibbarkeitsregeln, Sprachabzüge und Variantenfilter. Ein erlaubter
Rohkandidat muss deshalb nicht in der Endliste stehen. Die semantischen
Einzelentscheidungen sind redaktionelle Bewertungen; Vollständigkeit des
Registers ist technisch prüfbar, allgemeine sprachliche Fehlerfreiheit nicht.

Der vorhandene Keyword-Wächter bleibt ein Zusatznetz. Er prüft Krankheiten,
Verletzungen, Verwesung und Tod über Tokens/Wortfamilien und explizite sensible
Glossen. Die Markierungsformen `rude word`, `sometimes rude`, `rude term`,
`rude expression`, `vulgar`, `obscene` und weitere explizite Genitalbegriffe
werden zusätzlich erkannt. Neutrale Fragmente wie `rot` in `rotation` oder
`penis` in `peninsula` sind keine Bedeutungsentscheidungen.

Deutsch wird nach derselben Rubrik geprüft, Funde aber nur in einem separaten
[Bericht für PR #2](de-findings-report.tsv) gemeldet: 299 Sperr-/Zweifelkandidaten,
einschließlich redaktioneller Grenzfälle. Diese PR ändert weder deutsche Daten
noch Regeln. Die Große-Hälften-Klassifikation und Freigabe aller nicht genannten
Einträge hat sich als unzureichender Qualitätsnachweis erwiesen; siehe Stop-Befund.

## Quellen und deterministisches Ranking

**EN:** Joseph Bonneau/EFF, drei Listen von 2016 mit 10.368 Rohzeilen und
8.301 eindeutigen passenden Wörtern; dazu 1.257 ausdrücklich ausgewählte
Wordnik-Ergänzungen. Die vollständige komprimierte Wordnik-Datei prüft nur die
Quellenmitgliedschaft dieser [Ergänzungen](english-supplement.txt). Ihr übriger
Rohbestand ist kein freigegebener Nachrückerpool. 1.226 bestehende redaktionelle
Sperren bleiben zusätzlich wirksam. Nach vollständiger semantischer Freigabe
und deutschem Sprachabzug gilt: kürzer zuerst, dann ordinal.

**LA:** William A. Whitaker und Beitragende, WORDS, fester Commit
`1f2f0fb0867a896d7b9284a03d615ed635d6f992`, 39.335 Rohdatensätze. Der begrenzte
Decoder rekonstruiert unterstützte Nominative, regelmäßige/deponentische erste
Personen und unveränderliche Wörter. Er übernimmt keine nackten Stämme, keine
fehlenden `zzz`-Stämme, keine reinen Namen und keine nicht unterstützten Muster.
11.632 passende Schreibweisen haben mindestens einen Datensatz A–D.
2.352 bestehende redaktionelle Sperren bleiben zusätzlich wirksam. Bekannte
Quelldatenfehler und irritierende Formen sind damit ausgeschlossen. Der Decoder
ist kein vollständiger Morphologie- oder Rechtschreibprüfer.

Deutsch hat Vorrang vor Englisch, endgültiges Englisch vor Latein. Nach
Freigabe, Sperren und Sprachabzug bevorzugt LA Frequenz A vor B vor C vor D,
dann kürzere und ordinale Schreibweisen. Identische vollständige erste Glossen
werden auf die zuerst gerankte Schreibweise reduziert. Das kann auch Synonyme
entfernen und ist keine vollständige linguistische Lemmatisierung. Der
alphabetische Tie-Break beeinflusst die Vokabularzusammensetzung am Größenlimit.
Er verändert die Gleichverteilung beim späteren Ziehen aus der Endliste nicht.

Alle ausgewählten LA-Wörter haben im [Bedeutungsregister](latin-glosses.tsv)
Originalzeilen, Frequenzen und alle Glossen; mehrere Datensätze stehen mit
` | ` getrennt in derselben Zeile. Dieses Register wird ebenfalls reproduziert.
Bei Alltagswörtern sind unterschiedliche Sprachverwendungen möglich; das
Sprachflag bezeichnet die zugewiesene Verwendung, nicht die Wortgeschichte.
Die tatsächlich eingebetteten Schreibweisen bleiben vollständig disjunkt.

## Größen und unabhängige Wortwahl

Angestrebt werden 7.130 pro Sprache. Reicht der vollständig geprüfte Pool
nicht, werden EN und LA auf den gemeinsamen verfügbaren Umfang gekürzt und
die Abweichung dokumentiert; DE bleibt bei 7.130. Die tatsächliche Größe liefert
`WordSource::len()` und der [Filterbericht](report.txt).

Jede Wortposition soll unabhängig einen gleichverteilten Index aus
`0..combined().len()` ziehen. Die Ablage verschränkt die verfügbaren Listen
Deutsch/Englisch/Latein deterministisch. Bei gleichen Größen hat jede Sprache
Wahrscheinlichkeit 1/3 und vier Wortpositionen liefern gegenüber einer
Einzelquelle den Faktor 3^4 = 81. Bei abweichenden Größen ist die Sprachverteilung
entsprechend den tatsächlichen Größen gewichtet; der Faktor 81 gilt dann nicht.
Die Wortentropie ist stets `n * log2(combined().len())` bei festem Trenner.
Ein gesonderter Sprachbonus darf nicht nochmals addiert werden.
Aktuell: vier Wörter **57,449 Bit**, Vergleichsfaktor zur deutschen Quelle
**76,145**. Der Faktor 81 gilt für diesen verkleinerten Arbeitsstand nicht.

Sechs Buchstaben bleiben ein Komfortziel. Wörter mit 3–9 ASCII-Kleinbuchstaben
werden zugelassen, keine Transliteration. Ungebräuchlichere LA-Lemmata und
gängige EN-Flexionsformen bleiben möglich. Allgemeine Präfixfreiheit,
eindeutige Dreizeichenpräfixe und Mindest-Editierabstände werden nicht zugesichert.
Trenner halten Wortgrenzen eindeutig. Auswahl und Profilregeln gehören zu Core.

## Reproduzierbare Offline-Pipeline

```powershell
. 'C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1'
cargo run -p kairos-wordlists --example rebuild-languages
cargo run -p kairos-wordlists --example rebuild-languages -- --check
cargo run -p kairos-wordlists --example rebuild-german -- --check
```

`--check` schreibt nichts. Quellen und Lizenzdateien werden gegen
[SHA256SUMS](upstream/SHA256SUMS) geprüft, ebenso der unveränderte deutsche Hash,
Ergänzungen, Sperren, sämtliche semantischen Entscheidungen, beide Endlisten,
die separat freigegebenen Ergebnis-Hashes, Bericht und Bedeutungsregister.
Bei einer beabsichtigten Änderung: neue Kandidaten und alle Bedeutungen prüfen,
Entscheidungsdatei aktualisieren, erzeugen, Wortdiff bewerten und erst danach
Ergebnis-Hashes und Dokumentation freigeben. Die Pipeline aktualisiert die
freigegebenen Ergebnis-Hashes absichtlich nicht selbst.

18 sprachbezogene Integrationstests, zusammen mit DE **29 Integrationstests
und zwei Rustdoc-Tests**, prüfen Daten- und API-Invarianten, Quelle/Hash/Register,
Decoder, Wächter, die 14 unabhängigen Re-Verifier-Fälle, Freigabe jedes
Endworts und Fehlerpfade der vollständigen semantischen Entscheidungsdatei.
Die Tests ersetzen die redaktionelle Bedeutungsprüfung nicht.

Originale und Datenlizenzen: [PROVENANCE.md](PROVENANCE.md).
Die damaligen Klassifikationseingaben lassen sich unter Windows mit
`pwsh -NoProfile -File crates/kairos-wordlists/tools/prepare-review.ps1`
aus dem Repo-Wurzelverzeichnis rekonstruieren; die Hashes stehen in
[review-coverage.tsv](review-coverage.tsv). Die erzeugten Eingabedateien sind
Hilfsartefakte und keine von der Pipeline konsumierten Freigaben.
Quellenbewertung: [languages.md](../../../../docs/sources/languages.md).
[NOTICE.txt](NOTICE.txt) enthält Attribution, Änderungen und alle drei vollständigen
Datenlizenztexte. Die spätere Release-Integration muss diese Angaben mit der
EXE ausliefern; diese Verdrahtung liegt außerhalb dieser PR.
