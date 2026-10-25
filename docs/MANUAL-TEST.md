# Manuelle Desktop-Prüfung

Nur unbenutzte Testpasswörter in leeren lokalen Testfeldern verwenden.
Keinen echten Login absenden. Kairos während der 30-Sekunden-Prüfung laufen lassen.
Prüfdatum, Build-SHA und Ergebnis je Schritt notieren. Diese Liste ist kein
bereits ausgeführter Abnahmebeleg. Alle F-Tasten unten mit **Strg+Umschalt** drücken.

1. Release-App normal und mit `--popup` starten, auch aus einem Terminal. Vier deutsche Wörter und etwa 51,2 Bit bei Profil 0 prüfen. „Einfügen“ muss bis F9 deaktiviert bleiben; das Startfenster darf nur Ziel des Fokus-Rücksprungs sein. Profil 1 nutzt dieselbe Liste, keine fiktiven 21.000 Wörter.
2. Profile 0–12 und Trennerausschlüsse ausprobieren. Alle erlaubten Trenner ausgeschlossen muss einen sichtbaren Fehler ergeben.
3. „Profil bei jeder Generierung wechseln“ aktivieren. Zufall aus 0–12, Wiederholungen erlaubt, konservative Stärke und **+0,0 Bit** Bonus wegen Überlappung/ungleicher Stärke prüfen. Anschließend deaktivieren.
4. Einen lokalen Texteditor mit leerem Testfeld aktivieren. F9 drücken. Kompakter Editor erscheint; Text bearbeiten und ein- bis zweimal neu generieren. Nach Handänderung muss die Stärkebewertung entfallen.
5. Nach mehr als fünf Sekunden Bearbeitung „Einfügen“ wählen. Das ursprüngliche Editorfenster erhält den Fokus und den bearbeiteten Text ohne Enter/Tab. „Beim Einfügen auch kopieren“ einmal an-, einmal abwählen und Ergebnis prüfen. Ein zweiter Klick darf nicht doppelt tippen.
6. F9 wiederholen, deutschen Tastaturmodus wählen und nur den öffentlichen Testtext `PUBLIC-ÄÖÜäöüß@€#+'!?()_` eintippen. Einfügen muss diese Zeichen unverändert zustellen. AltGr+Q muss weiterhin `@` ergeben.
7. In einem leeren Browser-Testfeld F9 und Einfügen prüfen. Vorher im selben Feld bleiben. Mehrere DOM-Felder können dasselbe native Fenster teilen; Kairos kann einen Wechsel darin nicht zuverlässig erkennen.
8. In Editor und Browser jeweils F10 auslösen, während Kairos minimiert ist. Neues Passwort wird ins aktive Feld getippt und geschützt kopiert. Das Kairos-Fenster darf keinen Fokus anfordern. Zielinhalt und eingefügten Clipboard-Inhalt manuell vergleichen, nichts absenden.
9. In beiden Anwendungen F11 auslösen. Sichtbarer Hinweis „KeePassXC-Anbindung folgt (PR-6)“; aktuelle Nacharbeit bleibt erhalten, kein Einfügeziel wird freigegeben. Keine Texteingabe und kein vermeintlich gespeicherter KeePassXC-Eintrag.
10. Aus dem Zielfeld F9 öffnen und „Kopieren und zurückspringen“ wählen. Es muss genau das gemerkte Fenster zurückkehren. Ziel schließen oder zu einem anderen nativen Eingabefeld wechseln: Einfügen muss abbrechen; keinen automatischen Wiederholungsversuch erwarten. Innerhalb desselben Browser-HWND gilt die Grenze aus Schritt 7.
11. F9 öffnen, danach eine andere Anwendung aktivieren. Kairos darf diese nicht eigenmächtig verdrängen. Bei einem während F10 gewechselten Ziel auf Abbruch achten. Bei Fehlern Taskleistenhinweis beachten und Kairos manuell öffnen; bei Teileingabe niemals automatisch erneut senden.
12. Kopieren wählen und Win+V öffnen. Das Testpasswort darf nicht im Windows-Verlauf stehen. Normales Einfügen vor Ablauf muss funktionieren. Nach mindestens 30 Sekunden darf die eigene unveränderte Kopie nicht mehr einfügbar sein.
13. Erneut aus Kairos kopieren, dann vor 30 Sekunden in einer anderen Anwendung `PUBLIC FOREIGN TEXT` kopieren. Nach Ablauf muss dieser fremde Text erhalten bleiben. Beim normalen App-Ende muss nur eine noch eigene Kopie geleert werden.
14. Bei bereits von einer anderen Testanwendung belegtem F9/F10/F11 starten. Sichtbaren Registrierungsfehler prüfen; Belegung freigeben und „Hotkeys erneut registrieren“ wählen. Dann „Lizenzen und Wortherkunft“ mit Schriftlizenzen und CC0-Herkunft der Wortdaten prüfen.

Windows kann Fokusaktivierung verweigern; erhöhte Zielprozesse werden abgewiesen.
`SendInput` bestätigt angenommene Ereignisse, keine geprüfte Feldzustellung.
Clipboard-Kennzeichnungen wirken nur bei beachtender Software; lokale Leser,
bereits gespeicherte Kopien, dauerhaft belegte Zwischenablage und Prozessabstürze
liegen außerhalb dieser Zusage.

## PR-11: Bildschirmfreigabe und Aufnahmen (Prio 7)

Build-SHA, Windows-Version/Build, Grafiktreiber, Teams-/Snipping-Tool-Version,
Aufnahmeweg und Ergebnis dokumentieren. Nur frische, nirgendwo verwendete
Testpasswörter oder `cargo run -p kairos --features demo` verwenden. Für Teams
eine ausdrücklich eingerichtete Testsitzung nutzen; keine fremden Teilnehmer
ungefragt einladen. Ausgeführte API-Tests ersetzen diese Desktop-Prüfung nicht.

1. Kairos ohne Parameter starten. Auf dem lokalen Monitor muss das Fenster normal
   sichtbar bleiben. Mit Snipping Tool und separat **Win+Shift+S** einen Bereich
   aufnehmen, der Kairos und einen Teil des Desktops umfasst. Ab Windows 10 2004
   darf Kairos im Ergebnis nicht erscheinen; auf älterem Windows darf höchstens
   die leere Fensterfläche bleiben, mit „Eingeschränkter Aufnahmeschutz“ in der App.
2. Dasselbe mit `--popup` und mit über Strg+Umschalt+F9 geöffnetem Kompakteditor
   prüfen. Minimieren, zurückholen, Größe ändern und Neu generieren wiederholen.
   „Lizenzen und Wortherkunft“ öffnen; dessen Inhalt darf ebenfalls nicht in der
   Aufnahme stehen. Künftige native Viewports/Dialoge auch nach dem Start öffnen
   und prüfen. Automatisiert wird die Affinität solcher späteren Fenster geprüft.
3. In Teams erst den ganzen Bildschirm und dann, falls angeboten, das einzelne
   Kairos-Fenster teilen. Ergebnis am separaten empfangenden Testgerät prüfen.
   Ein geschütztes Fenster kann aus der Auswahlliste fehlen. Passwortinhalt darf
   über einen unterstützten Aufnahmeweg nicht sichtbar werden. Bei Sichtbarkeit
   Aufnahmeweg als nicht unterstützt dokumentieren; kein Schutzversprechen ableiten.
4. Kairos beenden und mit `--allow-screencapture` starten. „Bildschirmaufnahme
   erlaubt“ muss dauerhaft über dem Editor stehen. Schritt 1 und 3 wiederholen;
   Kairos samt öffentlichem Testinhalt muss jetzt aufnehmbar sein. Erneut mit
   `--popup --allow-screencapture`, kleinster Fensterhöhe, Scrollen und F9 prüfen.
5. Wieder ohne Freigabeparameter starten. Schutz muss erneut an sein; der vorherige
   Start darf ihn nicht dauerhaft abschalten. Bei einer tatsächlich nicht verfügbaren
   API/DWM-Sitzung muss „Schutz nicht verfügbar“ sichtbar sein. Erfolgs-, Opt-out-,
   Fehler- und Fallback-Hinweise werden zusätzlich durch UI-/Status-Tests geprüft;
   DWM nicht im laufenden Nutzerdesktop absichtlich abschalten.

Anzeigeaffinität benötigt DWM und hängt von Windows-Version und Aufnahmeweg ab.
Kein Schutz gegen Kameras, Admin-/Treiber-Tools oder jede Fernwartung. `WDA_MONITOR`
blendet Inhalte aus, garantiert aber keine unsichtbare Fensterfläche. Eigene
Renderer-Bildexporte (z. B. eframes Debug-Screenshot-Feature) umgehen den OS-Capture-
Pfad und sind kein Nachweis für oder gegen den Windows-Schutz.
