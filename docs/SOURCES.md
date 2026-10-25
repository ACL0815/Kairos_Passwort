# Open-Source-Vorbilder

Vorbilder werden unabhängig von ihrer Lizenz als Modell genannt; übernommen werden Prinzipien, nicht Code. Einzige Ausnahme: Zufallsgenerator – hier wird das Crate `rand`/`getrandom` direkt verwendet.

Daten-/Asset-Übernahmen (Wortlisten, Schriftarten) nur mit redistributionsfähiger
Lizenz, Lizenz dokumentiert. Die Softwarelizenz eines Generators erlaubt nicht
automatisch die Weitergabe sämtlicher beigelegter Wörterbücher oder Schriften.
Herkunft, konkrete Version, Lizenztext und erforderliche Hinweise werden vor einer
Übernahme separat festgehalten. Im Scaffold sind keine fremden Wortlisten oder
Schriften enthalten; `zeroize` ist die für den API-Vertrag benötigte Abhängigkeit.

## Fester Index und Zuständigkeit

**Diesen Index ändert später niemand mehr.** Jede Bereichsdatei gehört genau
einem der folgenden PRs. Weitere Recherche, Bewertungen und neue Vorbilder
werden ausschließlich in der zugewiesenen Datei ergänzt. Mehrfachnennungen eines
Projekts sind zulässig, weil unterschiedliche Prinzipien betrachtet werden.

| Bereich | Datei | Einziger zuständiger PR | Vorgefüllte Vorbilder |
|---|---|---|---|
| Core | [core.md](sources/core.md) | PR-1 | rand/getrandom, KeePassXC Random.cpp, xkcdpass, diceware, pwgen-go |
| Wortlisten | [wordlists.md](sources/wordlists.md) | PR-2 | xkcdpass, diceware, pwgen-go, Password Tech |
| Speicher | [store.md](sources/store.md) | PR-5 | SQLCipher, KeePassXC, libsodium |
| Windows | [platform.md](sources/platform.md) | PR-4 | KeePassXC Clipboard.cpp, Password Tech |
| KeePassXC | [keepassxc.md](sources/keepassxc.md) | PR-6 | KeePassXC, keepassxc-browser |
| GUI | [gui.md](sources/gui.md) | PR-3 | egui/eframe, Password Tech, KeePassXC |
| Sprachen | [languages.md](sources/languages.md) | PR-7 | xkcdpass, diceware, pwgen-go |
| Profile | [profiles.md](sources/profiles.md) | PR-8 | Password Tech, xkcdpass, KeePassXC |
| Schutz | [protection.md](sources/protection.md) | PR-9 | KeePassXC Alloc.cpp/Bootstrap.cpp, Bitwarden memory-testing, libsodium, obfstr |
| Browser | [browser.md](sources/browser.md) | PR-10 | 2FAS, keepassxc-browser |

## Pflege der Bereichsdateien

Links verweisen auf feste Commits oder Tags. Lizenzangaben beziehen sich auf den
verlinkten Stand; abweichende Datei-/Datenlizenzen sind zusätzlich zu prüfen.
Die anfängliche Passung von 1–5 ist eine vorläufige Einschätzung für Kairos,
keine abgeschlossene Quellcodebewertung. Der zuständige PR nennt später die
konkreten Module, geprüften Prinzipien, Grenzen und die eigene Umsetzung.

Die ursprünglichen Recherchelinks und Aussagen bleiben unverändert in
[Grundkonzept.md](Grundkonzept.md) erhalten. Die Bereichstabellen normalisieren
wechselnde Download-/`latest`-Links auf nachprüfbare Quellstände.
