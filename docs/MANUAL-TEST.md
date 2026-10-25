# Manuelle Desktop-Prüfung

Nur unbenutzte Testpasswörter in leeren lokalen Testfeldern verwenden.
Keinen echten Login absenden. Kairos während der 30-Sekunden-Prüfung laufen lassen.
Prüfdatum, Build-SHA und Ergebnis je Schritt notieren. Diese Liste ist kein
bereits ausgeführter Abnahmebeleg. Alle F-Tasten unten mit **Strg+Umschalt** drücken.

1. Release-App starten. Vier deutsche Wörter und etwa 51,2 Bit bei Profil 0 prüfen. Profil 1 verwendet aktuell dieselbe Liste; keine fiktiven 21.000 Wörter erwarten.
2. Profile 0–12 und Trennerausschlüsse ausprobieren. Alle erlaubten Trenner ausgeschlossen muss einen sichtbaren Fehler ergeben.
3. „Profil bei jeder Generierung wechseln“ aktivieren. Zufall aus 0–12, Wiederholungen erlaubt, konservative Stärke und **+0,0 Bit** Bonus wegen Überlappung/ungleicher Stärke prüfen. Anschließend deaktivieren.
4. Einen lokalen Texteditor mit leerem Testfeld aktivieren. F9 drücken. Kompakter Editor erscheint; Text bearbeiten und ein- bis zweimal neu generieren. Nach Handänderung muss die Stärkebewertung entfallen.
5. Nach mehr als fünf Sekunden Bearbeitung „Einfügen“ wählen. Das ursprüngliche Editorfenster erhält den Fokus und den bearbeiteten Text ohne Enter/Tab. „Beim Einfügen auch kopieren“ einmal an-, einmal abwählen und Ergebnis prüfen. Ein zweiter Klick darf nicht doppelt tippen.
6. F9 wiederholen, deutschen Tastaturmodus wählen und nur den öffentlichen Testtext `PUBLIC-ÄÖÜäöüß@€#+'!?()_` eintippen. Einfügen muss diese Zeichen unverändert zustellen. AltGr+Q muss weiterhin `@` ergeben.
7. In einem leeren Browser-Testfeld F9 und Einfügen prüfen. Vorher im selben Feld bleiben. Mehrere DOM-Felder können dasselbe native Fenster teilen; Kairos kann einen Wechsel darin nicht zuverlässig erkennen.
8. In Editor und Browser jeweils F10 auslösen, während Kairos minimiert ist. Neues Passwort wird ins aktive Feld getippt und geschützt kopiert. Das Kairos-Fenster darf keinen Fokus anfordern. Zielinhalt und eingefügten Clipboard-Inhalt manuell vergleichen, nichts absenden.
9. In beiden Anwendungen F11 auslösen. Sichtbarer Hinweis „KeePassXC-Anbindung folgt (PR-6)“; keine Texteingabe und kein vermeintlich gespeicherter KeePassXC-Eintrag.
10. Aus dem Zielfeld F9 öffnen und „Kopieren und zurückspringen“ wählen. Es muss genau das gemerkte Fenster zurückkehren. Ziel schließen oder zu einem anderen nativen Eingabefeld wechseln: Einfügen muss abbrechen; keinen automatischen Wiederholungsversuch erwarten. Innerhalb desselben Browser-HWND gilt die Grenze aus Schritt 7.
11. F9 öffnen, danach eine andere Anwendung aktivieren. Kairos darf diese nicht eigenmächtig verdrängen. Bei einem während F10 gewechselten Ziel auf Abbruch achten. Bei Fehlern Taskleistenhinweis beachten und Kairos manuell öffnen; bei Teileingabe niemals automatisch erneut senden.
12. Kopieren wählen und Win+V öffnen. Das Testpasswort darf nicht im Windows-Verlauf stehen. Normales Einfügen vor Ablauf muss funktionieren. Nach mindestens 30 Sekunden darf die eigene unveränderte Kopie nicht mehr einfügbar sein.
13. Erneut aus Kairos kopieren, dann vor 30 Sekunden in einer anderen Anwendung `PUBLIC FOREIGN TEXT` kopieren. Nach Ablauf muss dieser fremde Text erhalten bleiben. Beim normalen App-Ende muss nur eine noch eigene Kopie geleert werden.
14. Bei bereits von einer anderen Testanwendung belegtem F9/F10/F11 starten. Sichtbaren Registrierungsfehler prüfen; Belegung freigeben und „Hotkeys erneut registrieren“ wählen. Dann „Lizenzen und Wortherkunft“ mit Schriftlizenzen und CC0-Herkunft der Wortdaten prüfen.

Windows kann Fokusaktivierung verweigern; erhöhte Zielprozesse werden abgewiesen.
`SendInput` bestätigt angenommene Ereignisse, keine geprüfte Feldzustellung.
Clipboard-Kennzeichnungen wirken nur bei beachtender Software; lokale Leser,
bereits gespeicherte Kopien, dauerhaft belegte Zwischenablage und Prozessabstürze
liegen außerhalb dieser Zusage. Bildschirmfreigabeschutz folgt in PR-11.
