# Leitstern
Passwortregeln mit Komplexität sind überholter Schwachsinn!
Maximiere den Komfort für User bei dem Passwort-Abschreiben und der Passwort-Generierung, ohne die Sicherheit zu kompromitieren.

# Hauptphase



Sprache Rust


Features:
- Erweiterbar über zusätzliche Wortlisten. Externe Wortlisten importierbar, in eine bessere version von sqllight mit verschlüsslung.
- Interne Wortlisten, eingebettet in die exe
- Mann soll es kopieren können oder kopieren und zurück springen
- Mann sollte es aufrufen können mit Tastenkombination und manuell Nachbearbeiten und in dem fall auch ein oder 2 mal neu generieren.
- Man sollte bestimmte Sonderzeichen ausschließen können
- Man sollte auch eine Tastenkombination haben, bei der man es generiert und direkt ins aktuelle feld schreibt und Kopiert.
- Man sollte auch eine Tastenkombination haben, bei der man es generiert und direkt ins aktuelle feld schreibt und in KeePassXC über die Browser Schnittstelle einfügt.
- In der Version 1 Randomisierung nutze den Windows-Systemgenerator und in der 2. zusätzlich https://github.com/rust-random/rand?utm_source=chatgpt.com (was auch immer mit weniger eigenem Code möglich ist, zuerst implemntieren)
- Bevorzuge Code zu modellieren, auch in anderer Sprache und dann darauf neu aufzubauen, statt bei 0 anzufangen. Das bedeutet bewerte den Code wie gut der für unser Projekt passt, und übertrage die besten Prinzipien in unsere Planung. (nicht den code selbst) Nutze dazu Reverse Engeneer Anything.
- PW Listen: Fange mit Deutsch an. Dann erweitere es um Englisch. Dann um eine Sprache wie z.B. Latein die wenig gesprochen wird, aber dennoch irgendwie leicht von Menschen verarbeitet/geschieben werden kann. Die sollten in der "DB" einen Flag haben für die Sprache aber später Randomisiert gespeichert sein. (Erst mal nur deutsch)
- Baue Grafische Highlights farblich ein, damit man Zahlen und Sonderzeichen irgendwie rausspringen hat. Nimm eine Schriftart die besonders gut für Passwörter ist. 
- 

# Was ist das Angebot an Wort Passwörtern?
## Meine gedanken und anweisungen
Erstmal sollen die Profile 0 bis 12 auswählbar sein, aber es soll später dann auch frei konfigurierbar sein.Erstmal sollen die Profile 0 bis 12 auswählbar sein, aber es soll später dann auch frei konfigurierbar sein.
Es soll später möglich sein, zwischen verschiedenen Profilen hin und her zu wechseln, sagen wir mal und es soll möglich sein, eigene Profile anzulegen. Das Hin-und-her-Wechseln wäre aber automatisch bei jeder Passwortgenerierung, damit man nicht immer dasselbe Muster hat. Es sollte, ich glaube, das bringt noch mal 4 Bit, das sollte irgendwie drangeschrieben werden, also so als Haken. Aber auch nicht in der ersten Version.Es soll später möglich sein, zwischen verschiedenen Profilen hin und her zu wechseln und eigene Profile anzulegen.
---
## Ab hier research
Wie viel bringt das Randomisieren konkret?

Bei vier Wörtern und drei Zwischenräumen:
Automatische Zufallsentscheidung pro Passwort	Gewinn gegenüber der jeweils bekannten festen Auswahl
Eine von drei gleich großen, überschneidungsfreien Sprachlisten für das gesamte Passwort wählen	Faktor 3 · 1,58 Bit
Für jedes Wort unabhängig aus diesen drei Listen wählen	Faktor 81 · 6,34 Bit
Einen von vier Trennern wählen und ihn überall verwenden	Faktor 4 · 2 Bit
Jeden der drei Trenner unabhängig aus vier Zeichen wählen	Faktor 64 · 6 Bit


Das ist die entscheidende Verbesserung: die einzelnen Bestandteile unabhängig randomisieren. „Diesmal Latein und überall Punkte“ schafft weniger Möglichkeiten als eine unabhängige Sprachwahl pro Wort und unabhängige Trenner.

Vier bequem erreichbare, gleichwahrscheinlich gezogene Trennzeichen liefern dabei einen echten Gewinn. Dass die Auswahl auf bequeme Zeichen begrenzt ist, ist bereits in der Rechnung berücksichtigt.
---

Das Hin- und Herwechseln wäre aber automatisch bei jeder Passwortgenerierung, damit man nicht immer dasselbe Muster hat. Das bringt, glaube ich, nochmal 4 Bit. Das sollte irgendwie drangeschrieben werden, also so als Haken, aber auch nicht in der ersten Version.

---
Sure, I can!, **die Praxiszahl steigt, wenn zusätzliche unbekannte Auswahl hinzukommt. Mehr Länge durch bekannte Anhänge oder Wiederholungen bringt keine zusätzlichen Bits.**

Die mittlere Spalte zeigt **berechnete Obergrenzen der rechts beschriebenen Beispielmodelle**, keine gemessenen Durchschnittswerte. **B** bezeichnet jeweils eines von **10.000 dem Angreifer bekannten Grundmustern passender Länge**; die Großschreibung folgt einer festen Regel. Die Zeichenlängen rechts bleiben die theoretischen Äquivalente zur Wortleiter.

| **Wortpasswortgenerator: kumulative Regeln und Stärke** | Praktische Sicherheit des Zeichenpassworts im beschriebenen Modell | Theoretische Sicherheit des Zeichenpassworts und Erklärung des Unterschieds |
|---|---:|---|
| **0 · 51,1 Bit:** Vier Zufallswörter aus 7.000, klein, feste Minuszeichen. | **≤13,3 Bit** | **8 Zeichen · 51,3 Bit** bei Zufallsauswahl. Modell: **B + festes `1!`**. Nur die 10.000 Grundmuster sind unbekannt; feste Ergänzungen liefern keine Auswahlbits. |
| **1 · 57,4 Bit:** Wortliste auf 21.000 unterschiedliche Wörter erweitern. | **≤16,6 Bit** | **9 Zeichen · 58,1 Bit** bei Zufallsauswahl. Modell: **B + festes `1` + eine unbekannte Ziffer + festes `!`**. Höchstens 10.000 × 10 Varianten. |
| **2 · 64,4 Bit:** Jeden der drei Trenner unabhängig aus `-+.,#` ziehen. | **≤19,9 Bit** | **10 Zeichen · 64,8 Bit** bei Zufallsauswahl. Modell: **B + festes `1` + zwei unbekannte Ziffern + festes `!`**. Höchstens 10.000 × 100 Varianten. |
| **3 · 72,0 Bit:** Zusätzlich eine Zufallsziffer an einer zufälligen inneren Wortposition. | **≤21,9 Bit** | **12 Zeichen · 78,1 Bit** bei Zufallsauswahl. Modell: **B + zwei unbekannte Ziffern + eines von vier Lieblingssonderzeichen**, alles an festen Positionen. Höchstens 10.000 × 100 × 4 Varianten. |
| **4 · 74,1 Bit:** Trennerauswahl auf acht Zeichen erweitern, zusätzlich `*!?`. | **≤21,9 Bit** | **12 Zeichen · 78,1 Bit.** Dasselbe menschliche Auswahlmodell; daher derselbe Praxiswert. Die Wortstufe gewinnt dagegen zusätzliche zufällige Trenner. |
| **5 · 75,8 Bit:** Auf zwölf Trenner erweitern, zusätzlich `_:;/`. | **≤21,9 Bit** | **12 Zeichen · 78,1 Bit.** Dasselbe Auswahlmodell. Mehr erlaubte Sonderzeichen helfen nicht, solange der Mensch weiterhin nur seine vier Lieblingszeichen verwendet. |
| **6 · 77,1 Bit:** Auf 16 Trenner erweitern, zusätzlich Klammern und beide Anführungszeichen. | **≤21,9 Bit** | **12 Zeichen · 78,1 Bit.** Unverändertes Auswahlmodell: Die theoretisch erlaubten Möglichkeiten werden weiterhin nur teilweise genutzt. |
| **7 · 78,0 Bit:** Auf 20 Trenner erweitern, zusätzlich `$%&=`. | **≤21,9 Bit** | **12 Zeichen · 78,1 Bit.** Unverändertes Auswahlmodell; deshalb kein zusätzlicher Praxisgewinn. |
| **8 · 82,0 Bit:** Jeden Wortanfang unabhängig zufällig groß oder klein schreiben. | **≤25,3 Bit** | **13 Zeichen · 84,8 Bit** bei Zufallsauswahl. Neues Modell: **B + drei unbekannte Ziffern + eines von vier Sonderzeichen**. Die zusätzliche Ziffer erhöht die Auswahl gegenüber dem Zweiziffernmodell um **3,32 Bit**. |
| **9 · 102,0 Bit:** Auf fünf Wörter erhöhen; bisherige Regeln beibehalten. | **≤28,6 Bit** | **16 Zeichen · 104,6 Bit** bei Zufallsauswahl. Neues Modell: **B + vier unbekannte Ziffern + eines von vier Sonderzeichen**. Höchstens 10.000 × 10.000 × 4 Varianten. |
| **10 · 122,0 Bit:** Auf sechs Wörter erhöhen; bisherige Regeln beibehalten. | **≤28,6 Bit** | **19 Zeichen · 124,3 Bit.** Weiterhin nur 10.000 längere Grundmuster, vier Ziffern und vier Schlusszeichen. **Längere Grundmuster vergrößern deren Anzahl hier nicht.** |
| **11 · 131,5 Bit:** Wortanfänge ersetzen durch genau einen zufällig positionierten Großbuchstaben je Wort. | **≤28,6 Bit** | **21 Zeichen · 137,5 Bit.** Dasselbe Auswahlmodell mit längeren Grundmustern. Feste Großschreibung liefert weiterhin keine zusätzlichen Auswahlbits. |
| **12 · 153,1 Bit:** Zwischen jedes Wortpaar zwei unabhängig gezogene Trenner aus den 20 setzen. | **≤28,6 Bit** | **24 Zeichen · 157,2 Bit.** Dasselbe Auswahlmodell. Bekannte Verlängerungen oder Wiederholungen erzeugen zusätzliche Zeichen, aber keine zusätzlichen unbekannten Entscheidungen. |
| **Optional · bis 157,1 Bit:** Zusätzlich unbekannte Auswahl aus 16 gleichwahrscheinlichen, gleich starken Profilen mit überschneidungsfreien Ausgabemengen. | **≤28,6 Bit** | **24 Zeichen · 157,2 Bit.** Der Profilbonus betrifft den Wortgenerator. Er beträgt höchstens **+4 Bit**; bei überlappenden Ausgaben weniger, bei bekanntem aktuellem Profil **0 Bit**. |

**Warum die Praxiswerte teilweise steigen und teilweise gleich bleiben:** Die Beispiele wechseln ausdrücklich von festen Ergänzungen zu einer, zwei, drei beziehungsweise vier unbekannten Ziffern. **Diese Auswahländerung erzeugt den Gewinn; die Längenvorgabe allein erzwingt ihn nicht.** Ein menschlicher Bauplan mit zwei Ziffern und vier Schlusszeichen bleibt auch bei 24 Zeichen bei höchstens **21,9 Bit**.

**Was bedeuten acht Bit?** **8 Bit = 256 gleichwahrscheinliche Möglichkeiten.** Die zuvor genannten **8,6 Bit** entstehen beispielsweise, wenn das Grundmuster bereits geleakt ist und nur zwei Ziffern sowie eines von vier Sonderzeichen unbekannt bleiben: **100 × 4 = 400 Möglichkeiten**.

Berechnungsbasis: Zeichenpasswörter verwenden 94 druckbare ASCII-Zeichen ohne Leerzeichen und müssen jede der vier Zeichenarten enthalten. Die Wortleiter verwendet weiterhin das Modell mit sechs Buchstaben je Wort, unabhängigen Zufallsentscheidungen und vollständig bekannten Erzeugungsregeln. **Die menschlichen Beispielmodelle erreichen kein praktisches Äquivalent zur ersten Wortstufe.**

Sure, I can!, **meine letzte Praxis-Spalte hatte keinen belastbaren Vorhersagewert: Ich hatte frei gewählte menschliche Baupläne verschiedenen Längen zugeordnet. Das war keine aus der Länge abgeleitete Sicherheitsprognose.**

Aus ersten Prinzipien folgt: Der optimale Angreifer prüft Passwörter nach ihrer Wahrscheinlichkeit — unter Berücksichtigung seiner Kenntnisse über Sprache, frühere Passwörter und Erzeugungsregeln. **Entscheidend ist die Anzahl benötigter Versuche.** Forschung zu menschlichen Passwörtern verwendet deshalb Wahrscheinlichkeitsverteilungen und konkrete Rateangriffe. [Technical reports: UCAM-CL-TR-819](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-819.html?utm_source=chatgpt.com)

Für einen vergleichbaren Zahlenwert verwende ich:

\[
G_{50}=\text{Versuche, um 50 % der betreffenden Passwortauswahl zu treffen}
\]

Bei **gleichverteilten Generatorausgaben mit \(H\) Bit** gilt:

\[
G_{50}=\frac{2^H}{2}
\]

Damit erhält deine Tabelle eine mathematisch belastbare Zuordnung. **Die mittlere Spalte zeigt den notwendigen praktischen Rateaufwand für ein gleich starkes Zeichenpasswort. Sie behauptet nicht, dass menschliche Passwörter dieser Länge ihn tatsächlich erreichen.**

| **Wortpasswortgenerator: kumulative Regeln und Stärke** | Praktisch erforderliches \(G_{50}\) des gleich starken Zeichenpassworts | Theoretische Sicherheit des Zeichenpassworts |
|---|---:|---|
| **0 · 51,1 Bit:** Vier Zufallswörter aus 7.000, klein, feste Minuszeichen. | **1,20 × 10¹⁵ Versuche** | **8 Zufallszeichen: 51,3 Bit** |
| **1 · 57,4 Bit:** Wortliste auf 21.000 unterschiedliche Wörter erweitern. | **9,72 × 10¹⁶ Versuche** | **9 Zufallszeichen: 58,1 Bit** |
| **2 · 64,4 Bit:** Jeden Trenner unabhängig aus fünf Zeichen ziehen. | **1,22 × 10¹⁹ Versuche** | **10 Zufallszeichen: 64,8 Bit** |
| **3 · 72,0 Bit:** Eine Zufallsziffer an einer zufälligen inneren Wortposition ergänzen. | **2,43 × 10²¹ Versuche** | **12 Zufallszeichen: 78,1 Bit** |
| **4 · 74,1 Bit:** Trennerauswahl auf acht Zeichen erweitern. | **9,96 × 10²¹ Versuche** | **12 Zufallszeichen: 78,1 Bit** |
| **5 · 75,8 Bit:** Auf zwölf Trenner erweitern. | **3,36 × 10²² Versuche** | **12 Zufallszeichen: 78,1 Bit** |
| **6 · 77,1 Bit:** Auf 16 Trenner erweitern. | **7,97 × 10²² Versuche** | **12 Zufallszeichen: 78,1 Bit** |
| **7 · 78,0 Bit:** Auf 20 Trenner erweitern. | **1,56 × 10²³ Versuche** | **12 Zufallszeichen: 78,1 Bit** |
| **8 · 82,0 Bit:** Jeden Wortanfang unabhängig zufällig groß oder klein schreiben. | **2,49 × 10²⁴ Versuche** | **13 Zufallszeichen: 84,8 Bit** |
| **9 · 102,0 Bit:** Auf fünf Wörter erhöhen; bisherige Regeln beibehalten. | **2,61 × 10³⁰ Versuche** | **16 Zufallszeichen: 104,6 Bit** |
| **10 · 122,0 Bit:** Auf sechs Wörter erhöhen; bisherige Regeln beibehalten. | **2,64 × 10³⁶ Versuche** | **19 Zufallszeichen: 124,3 Bit** |
| **11 · 131,5 Bit:** Genau einen zufällig positionierten Großbuchstaben je Wort statt zufälliger Wortanfänge verwenden. | **1,92 × 10³⁹ Versuche** | **21 Zufallszeichen: 137,5 Bit** |
| **12 · 153,1 Bit:** Zwischen jedes Wortpaar zwei unabhängig gezogene Trenner setzen. | **6,15 × 10⁴⁵ Versuche** | **24 Zufallszeichen: 157,2 Bit** |
| **Optional · 157,1 Bit:** Zusätzlich volle vier Bit durch unbekannte Auswahl aus 16 gleich starken, überschneidungsfreien Profilen. | **9,83 × 10⁴⁶ Versuche** | **24 Zufallszeichen: 157,2 Bit** |

Rechts steht jeweils die kürzeste mindestens gleich starke Zufallsalternative: 94 druckbare ASCII-Zeichen ohne Leerzeichen, jede der vier Zeichenarten mindestens einmal. Links gelten weiterhin sechs Buchstaben je Wort und unabhängige, gleichverteilte Ziehungen.

**Warum lässt sich die menschliche Praxiszahl nicht einfach mit der Länge erhöhen?**

Für den zuvor definierten Bauplan gilt:

\[
10.000\ \text{Grundmuster}\times100\ \text{Zahlen}\times4\ \text{Schlusszeichen}
=4.000.000\ \text{Möglichkeiten}
\]

Das sind höchstens **21,9 Auswahlbits** — bei zwölf wie bei zwanzig Zeichen. Die längeren Grundmuster bleiben dieselben **10.000 Auswahlmöglichkeiten**.

Eine weitere **unabhängig zufällige Ziffer** vervielfacht dagegen die Auswahl um zehn: **+3,32 Bit**. Eine bekannte Wiederholung oder ein festes `!` liefert **+0 Bit**.

**8 Bit** bedeuten 256 gleichwahrscheinliche Möglichkeiten; für 50 % Treffer genügen 128 Versuche. Die früher genannten **8,6 Bit** entsprechen 400 Möglichkeiten: beispielsweise zwei unbekannte Ziffern und vier mögliche Schlusszeichen bei bereits bekanntem Grundmuster.

Für tatsächliche menschliche Praxiswerte braucht die Tabelle gemessene Rateerfolge auf passenden Testdaten. **Eine allein aus der Länge steigende Bitkurve wäre Scheingenauigkeit.**


# Browser Phase
Schaue wie 2FAs und KeePassXC die Kommunikation mit dem Browser Plugin gestaltet haben und schaue wie man das einfach und sicher Adaptieren kann.
Erst mal via Violentmonkey


# Protection Phase Optional

## Nice to have
Möglichst Manipulationssicher Erkennen und warnen beim Start wenn die externen Wortlisten sich verändert haben. (welche Projekte könnte man dafür modellieren)
Externe Wortlisten verschlüsselt ablegen.




## Nutze einen Code und Wortlisten Schutz: https://github.com/CasualX/obfstr?utm_source=chatgpt.com


## Schaue bei KeePassXC was sinnvoll ist für ein PW generator
Sure, I can!, **Speicherhärtung hat echten Wert. Die Zwischenablage begrenzt ihren Nutzen während des Kopierens; sie macht den Schutz davor und danach keineswegs überflüssig.** Für einen Passwortgenerator stehen sicherer Zufall und wenige Datenkopien allerdings weiter oben als aufwendige RAM-Verschlüsselung.

Ich habe dafür KeePassXC 2.8.0 und Teile des Bitwarden-SDKs gelesen sowie KeePass’ Sicherheitsdokumentation verglichen.

**Das wichtigste First Principle: Jede zusätzliche Kopie verlängert die Angriffsfläche.**

Ein Passwort wandert beispielsweise durch Generator, Ergebnisvariable, UI-Feld, Undo-Verlauf, Zwischenablage und Zielformular. Jede dieser Stationen besitzt einen eigenen Speicher und eine eigene Lebensdauer. Das Löschen einer Station löscht die anderen Kopien nicht. KeePass dokumentiert ausdrücklich, dass Betriebssystem und Laufzeit zusätzliche Kopien erzeugen, die KeePass selbst nicht zuverlässig beseitigt. [KeePass](https://keepass.info/help/base/security.html?utm_source=chatgpt.com)

Daraus folgen sechs konkrete Lehren:

**1. Zufall korrekt erzeugen und korrekt in Zeichen übersetzen.**

KeePassXC verwendet in [Random.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/crypto/Random.cpp) Botan mit Betriebssystem-Zufallsquelle beziehungsweise kryptografischem Generator. Außerdem verwirft es bestimmte Zufallswerte, bevor es den Modulo-Operator verwendet: **Rejection Sampling verhindert ungleiche Zeichenwahrscheinlichkeiten.**

Ein kleines Beispiel: `Zufallsbyte % 62` bevorzugt einige Zeichen, weil 256 nicht durch 62 teilbar ist.

Für einen eigenen Generator:

- Betriebssystem-CSPRNG beziehungsweise im Browser `crypto.getRandomValues()` verwenden.
- Gleichverteilte Auswahl durch eine bewährte Funktion oder korrektes Rejection Sampling.
- Bei Fehlern abbrechen; keinen schwächeren Zufall als Ersatz einsetzen.
- Genügend Länge wählen: 24 unabhängig gleichverteilte Zeichen aus 62 Zeichen ergeben rund **143 Bit Entropie**. Zusätzliche Zufallsquellen selbst zusammenzubauen bringt hier keinen praktischen Gewinn. Die Browser-API liefert kryptografisch geeignete Zufallswerte. [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues?utm_source=chatgpt.com)

**2. Geheimnisse kurz halten und möglichst wenige Kopien erzeugen.**

Für einen nativen Generator würde ich einen veränderbaren, ausreichend großen Puffer vorab reservieren und nach Gebrauch überschreiben. Unnötige String-Konvertierungen, Vergrößerungen und Zwischenobjekte vermeiden: Beim Umkopieren bleiben sonst alte Speicherbereiche zurück.

Zur Lebensdauer gehören auch Produktentscheidungen: **kein Passwort in Logs, Fehlerberichten, Analyseereignissen oder einem ungefragten Generator-Verlauf.** Bitwarden besitzt ausdrücklich einen Generator-Verlauf – eine sinnvolle Wiederherstellungsfunktion, die aber zusätzliche Geheimnisse aufbewahrt. Für einen eigenständigen Generator würde ich diesen Komfort bewusst abwählen. [Bitwarden](https://bitwarden.com/help/password-and-generator-history/?utm_source=chatgpt.com)

**3. Speicherfreigabe und sicheres Löschen ausdrücklich unterscheiden.**

`free()`, ein zerstörtes Objekt oder eine geleerte Variable bedeuten nicht automatisch, dass die Bytes überschrieben wurden.

KeePassXC überschreibt in [Alloc.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Alloc.cpp) Speicher in seinen eigenen `delete`-Operatoren mit `Botan::secure_scrub_memory()`, bevor es ihn freigibt. Das ist eine konkrete Schutzmaßnahme für die darüber laufenden Freigaben.

Für eigene Software eine bewährte Löschfunktion verwenden: beispielsweise `sodium_memzero()`. Ein gewöhnliches `memset()` entfernt ein optimierender Compiler unter Umständen als vermeintlich unnötige Arbeit. **Auch korrektes Überschreiben beseitigt ausschließlich die betroffenen Bytes; zuvor entstandene Kopien bleiben eine eigene Aufgabe.** [Libsodium documentation](https://doc.libsodium.org/memory_management?utm_source=chatgpt.com)

**4. Verhindern, dass RAM ungewollt auf der Platte landet.**

Zwei unterschiedliche Maßnahmen:

- **Crash-Dumps beschränken:** KeePassXC setzt in [Bootstrap.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp) plattformabhängige Dump- und Prozesszugriffsbeschränkungen.
- **Geheimnispuffer gegen Auslagern sperren:** etwa mit `mlock()` beziehungsweise `VirtualLock()`, idealerweise über eine etablierte Bibliothek. Rückgabewerte prüfen: Die Sperre besitzt Betriebssystemgrenzen und schlägt bei überschrittenen Limits fehl.

Das schützt nicht automatisch UI-Kopien, sämtliche Stack-Inhalte oder Ruhezustandsabbilder. [Libsodium documentation](https://doc.libsodium.org/memory_management?utm_source=chatgpt.com)

**5. Die Zwischenablage als eigenen Sicherheitsbereich behandeln.**

KeePassXC macht in [Clipboard.cpp](https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/gui/Clipboard.cpp) mehr als zeitgesteuertes Leeren: Es setzt plattformspezifische Kennzeichnungen für sensible Inhalte und gegen Verlauf beziehungsweise Cloud-Synchronisierung.

Für einen Generator würde ich Kopieren ausschließlich auf Nutzeraktion auslösen, einen kurzen Timeout anbieten und unterstützte Verlauf-/Sync-Sperren setzen.

**Bereits ausgelesene oder anderswo gespeicherte Zwischenablage-Inhalte holt das spätere Leeren nicht zurück.** Diese Kennzeichnungen setzen zudem voraus, dass die beteiligte Software sie respektiert.

**6. Die Schutzwirkung messen.**

Bitwardens SDK enthält eigene [Speicher-Dump-Tests](https://github.com/bitwarden/sdk-internal/blob/5cf919837e83b71c2508bf99344ea57cf5f84e88/crates/memory-testing/src/lib.rs), einschließlich gezieltem Überschreiben zusätzlicher Testdatenkopien.

Die übertragbare Idee: Ein bekanntes Testpasswort erzeugen, anzeigen, kopieren und entfernen; anschließend im Release-Build nach Resten suchen, einschließlich unterschiedlicher Zeichenkodierungen. Eine Positivkontrolle stellt sicher, dass der Dump überhaupt brauchbar ist. Ein fehlender Treffer allein beweist keine vollständige Löschung.

**Wie viel bringt das trotz Zwischenablage?**

| Angriff | Nutzen der Speicherhärtung |
|---|---|
| Passwort erraten | Keiner; hier zählen Zufall und Länge |
| Später Speicherreste oder Dumps untersuchen | Direkter Nutzen |
| Zwischenablage während der Nutzung auslesen | RAM-Löschen verhindert diesen Zugriff nicht |
| Schadcode läuft bereits innerhalb des Passwortmanagers | Keine verlässliche Schutzgrenze |

**Meine Priorisierung:** Für einen einfachen Generator zuerst **CSPRNG → gleichverteilte Auswahl → ausreichende Länge → keine unnötige Speicherung → saubere Zwischenablage → sicheres Überschreiben und Dump-Schutz**.

Bei einem Passwortmanager besitzen langlebige Tresorschlüssel und viele gleichzeitig entschlüsselte Einträge zusätzlichen Schutzbedarf. RAM-Verschlüsselung wie bei KeePass erschwert bestimmte Speicheranalysen; ihr Schlüsselmanagement und die notwendigen Klartextphasen bestimmen ihre tatsächliche Wirkung. [KeePass](https://keepass.info/help/base/security.html?utm_source=chatgpt.com) Für einen Generator, der ein einzelnes Passwort nur kurz verarbeitet, würde ich zuerst Kopien und Lebensdauer reduzieren. Das liefert mehr Wirkung pro Implementierungsaufwand.


# Quellen für Wortlisten und modellieren von vorhandenem Code
Sure, I can!, **für eine bequem bedienbare Windows-App empfehle ich Password Tech. Wenn „nur Generator, kein Passwortmanager“ höchste Priorität hat, stehen `xkcdpass`, `diceware` und `pwgen-go` vorne.**

Die Suche mit **drei Luna-6-Agenten** und anschließender Quellenprüfung ergibt dieses Ranking. **Alle sechs arbeiten mit echten Wörtern und sind herunterladbar.** Reine Generatoren stehen zuerst; Terminalprogramme sind ausdrücklich gekennzeichnet.

| Rang | Programm und Download | Wörterbücher | Bedienung und Plattform | Bewertung |
|---:|---|---|---|---|
| **1** | **[xkcdpass](https://pypi.org/project/xkcdpass/#files)** — kostenlos, Open Source | Mehrere **deutsche** Wortlisten, englische EFF-Listen und weitere Sprachen. **Eigene Textdateien importierbar.** | **Terminal; Python erforderlich.** Windows, macOS, Linux. Download als Python-Paket, keine fertige grafische EXE. | **Bester reiner Generator für flexible Wörterbuch-Passwörter.** Wortanzahl, Wortlänge, Trenner und Großschreibung einstellbar. [PyPI](https://pypi.org/project/xkcdpass/?utm_source=chatgpt.com) |
| **2** | **[diceware von Uli Fouquet](https://pypi.org/project/diceware/#files)** — kostenlos, Open Source | **Deutsch enthalten**, englische EFF-Liste und weitere Sprachen. **Eigene Wortlisten importierbar.** | **Terminal; Python erforderlich.** Plattformunabhängiges Python-Paket. | **Beste einfache Diceware-Lösung.** Standardmäßig sechs Wörter; wahlweise Systemzufall oder Eingabe echter Würfelwürfe. [PyPI](https://pypi.org/project/diceware/?utm_source=chatgpt.com) |
| **3** | **[pwgen-go](https://github.com/gabe565/pwgen-go/releases/latest)** — kostenlos, Open Source | Eingebettete **englische EFF-Wortlisten**, lange Liste als Standard. | **Terminal; fertige Binärdateien.** macOS/Linux ausdrücklich dokumentiert; kein Python nötig. | **Beste schlanke Lösung ohne Python-Installation.** Reiner Generator mit Vorlagen, Wortanzahl und verschiedenen Ausgabeformaten. [GitHub](https://github.com/gabe565/pwgen-go?utm_source=chatgpt.com) |
| **4** | **[Password Tech, früher PWGen](https://sourceforge.net/projects/pwgen-win/files/Password%20Tech/)** — kostenlos, Open Source | **Eigene Wortlisten**, beliebige Sprachen und Unicode. Deutsche Wortliste separat erhältlich. | **Grafische Windows-App**, fertiger Installer. | **Meine praktische Windows-Empfehlung.** Leistungsfähiger Generator mit Stapelerzeugung und zahlreichen Regeln. Enthält zusätzlich einen Passwortsafe — deshalb hinter den reinen Generatoren. [pwgen-win.sourceforge.io](https://pwgen-win.sourceforge.io/?utm_source=chatgpt.com) |
| **5** | **[KeePassXC](https://keepassxc.org/download/)** — kostenlos, Open Source | Wörterbuch-Passphrasen; **zusätzliche eigene/deutsche Wortlisten einbindbar**. | **Grafische App:** Windows, macOS, Linux; auch portable Windows-Ausgabe. | **Beste grafische Alternative über mehrere Betriebssysteme.** Steht weiter unten, weil es ein vollständiger Passwortmanager ist. [keepassxc.org](https://keepassxc.org/docs/KeePassXC_GettingStarted?utm_source=chatgpt.com) |
| **6** | **[Bitwarden](https://bitwarden.com/download/)** — kostenlos nutzbar, Open Source | Zufällige Wort-Passphrasen mit einstellbarer Wortanzahl, Trenner, Großschreibung und optionaler Zahl. | **Grafische Apps:** Windows, macOS, Linux sowie Mobilgeräte. | Sinnvoll, wenn du Bitwarden bereits verwendest. Für einen eigenständigen Wörterbuch-Generator ist die gesamte Passwortverwaltung zusätzlicher Aufwand. [Bitwarden](https://bitwarden.com/download/?utm_source=chatgpt.com) |

**Für „herunterladen, anklicken, deutsche Wörter erzeugen“:** Nimm **Password Tech** und lade die [deutsche Wortliste](https://sourceforge.net/projects/pwgen-win/files/Word%20Lists/) dazu. Für einen **reinen Generator** mit deutschen Wörterbüchern ist **xkcdpass** meine erste Wahl; dafür brauchst du Python und das Terminal.

Reine Zeichen-/Zahlensalat-Generatoren stehen **unter sämtlichen genannten Kandidaten** und erfüllen deine Wörterbuchpflicht nicht. Beispielsweise dokumentiert [HiBARAs RandomPasswordGenerator](https://hibara.jp/en/software/randompasswordgenerator/) lediglich Zeichenlänge und Zeichensätze. [HiBARA Software](https://hibara.jp/en/software/randompasswordgenerator/?utm_source=chatgpt.com)

Als Einstellung für die englische EFF-Langliste: **sechs zufällig ausgewählte Wörter**. Das liefert rund **77 Bit Entropie**; ein zusätzliches Wort bringt mehr als dekorative Ersetzungen wie `a → @`. [diceware 1.0.1 documentation](https://diceware.readthedocs.io/en/stable/readme.html?utm_source=chatgpt.com)