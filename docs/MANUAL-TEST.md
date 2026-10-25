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
9. F11 zunächst ohne Brücke/KeePassXC auslösen. Frisches Passwort und KeePassXC-Bestätigungsdialog, leere URL mit Hinweis. Keine Eingabe ohne Bestätigung; Verbindungsfehler sichtbar. Der neue Auftrag ersetzt die bisherige Nacharbeit bewusst.
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

## F11 + Violentmonkey + KeePassXC (isolierte Testumgebung)

Diese Schritte sind **nicht bereits ausgeführt**. Keine echten Logins absenden.
Browserprofile, KeePassXC-Einstellungen und Testdatenbank richtet ausschließlich
der prüfende Nutzer selbst ein. Nur eine wegwerfbare Datenbank, keine echten
Passwörter. Je Schritt Browser-/VM-/KeePassXC-Version, Build-SHA und Ergebnis
notieren. Anleitung/Bedrohungsmodell: [browser/README.md](../browser/README.md).

1. Violentmonkey in einem neuen Testprofil installieren; `browser/kairos-bridge.user.js`
   als neues Script einfügen. Metadaten mit `@noframes`, Content-Injektion,
   GM-Grants und ausschließlich `127.0.0.1` als `@connect` prüfen.
2. Kairos starten. Ohne Pairing auf einer Testseite ein Formular fokussieren:
   F11 darf keinen Vorschlag haben. In Kairos „Browser-Brücke koppeln“ öffnen,
   Code anzeigen; nach zwei Minuten muss er ungültig sein. Neuen Code anzeigen,
   abbrechen, alten Code im VM-Menü versuchen: abgewiesen. Mit neuem Code koppeln.
   Nicht in ein DOM-Formular eingeben. Code verschwindet nach erfolgreicher Kopplung.
3. Seite mit `?testtoken=PUBLIC#PUBLIC` und Testpfad öffnen. Passwortfeld fokussieren,
   sofort F11. Nur Origin+Pfad im Vorschlag, keine Query/Fragment. URL-Feld bleibt
   bis zur ausdrücklichen Übernahme leer. Übernahme und Bestätigung getrennt prüfen.
4. Zweites Browserfenster/anderen Browser, Hintergrundtab, iframe und Editor
   prüfen. Alte/falsche HWND-/PID-Zuordnung oder mehr als fünf Sekunden ohne
   fokussierte Meldung: kein Vorschlag. Kurzzeitige Fokus-/Tabrennen sind bewusst
   nicht beweisbar: keine automatische Übernahme. URL manuell eingeben funktioniert.
5. E-Mail im Testformular setzen. Normale Fokusmeldungen enthalten keinen
   Benutzernamen. Über VM-Menü einmal ausdrücklich senden und sofort F11:
   nach Übernahme vorbefüllt. Späterer Heartbeat ohne Benutzername ersetzt ihn;
   keine dauerhafte GM-Speicherung. Passwortfeld wird niemals ausgelesen.
6. Eigene wegwerfbare KeePassXC-Datenbank entsperren und Browserintegration selbst
   aktivieren. Keine lokale Zuordnung vorhanden: „Eintragen“ führt zum geführten
   Kopplungsknopf. KeePassXC-Freigabe ablehnen: kein Tippen, sichtbarer Fehler.
   Für erfolgreichen Kopplungsfall einen frischen isolierten Durchlauf verwenden;
   danach getrennt Eintragen bestätigen. Keine automatische Wiederholung.
7. Eintrag mit Test-URL, -Benutzername und optional bestehender Gruppen-UUID
   anfordern. KeePassXC-Ergebnis und Datenbank prüfen. Erst „Jetzt ins gemerkte
   Feld tippen“ darf tippen; dasselbe Passwort vergleichen, kein Enter/Submit.
   Kopierhaken an/aus prüfen. Zweiter Klick darf nicht doppelt schreiben/tippen.
8. KeePassXC sperren, beenden, andere DB wählen, Zuordnung widerrufen, Timeout
   abwarten. Jeder Fehler sichtbar, kein Tippen. Nach Timeout kann ein Eintrag
   existieren: manuell prüfen. Kein automatischer Retry oder stiller Teilerfolg.
9. Nach angenommener Anfrage Zielfenster schließen/anderen nativen Control wählen.
   Tippen muss scheitern und auf bereits angenommene KeePassXC-Anfrage hinweisen.
   Browser-DOM-Feldwechsel innerhalb desselben HWND bleibt erkennbare Testgrenze.
10. Port vorher durch eigenen Dummy-Prozess belegen bzw. eigene isolierte DPAPI-
    Datei beschädigen: Brücke sichtbar nicht verfügbar, kein Klartext-Fallback.
    Kein neuer Token ersetzt eine vorhandene Datei. Widerruf/Backup-Umbenennung
    gemäß Browser-README ausschließlich an eigenen Testdateien prüfen.
11. Von einer Test-Webseite fetch/form/OPTIONS versuchen: kein CORS, Origin
    abgewiesen, keine Aktion. Testwerkzeug mit falschem Host/Origin/Token sowie
    gleicher Nonce muss 403 ohne Body erhalten. Niemals echte Tokens protokollieren.

Automatisch: `cargo test -p kairos-bridge`, `cargo test -p kairos --lib`,
`node --check browser/kairos-bridge.user.js`, `node browser/test-userscript.cjs`.
Der zusätzliche reale **nur lesende** Test ist absichtlich `#[ignore]`.
Kairos dafür beenden; bereits gepaartes isoliertes Script und vorhandene
Test-KeePassXC-Zuordnung verwenden, dann Vordergrund-Testseite aktivieren:

```powershell
$env:KAIROS_BRIDGE_TEST_ASSOCIATION_FILE = '<eigener isolierter DPAPI-Testpfad>'
$env:KAIROS_KEEPASSXC_TEST_ASSOCIATION_FILE = '<eigener isolierter KeePassXC-Testpfad>'
cargo test -p kairos --lib real_browser_context_and_keepassxc_association_read_only -- --ignored --test-threads=1
```

Test wartet höchstens 30 Sekunden auf echten frischen Kontext und prüft nur
`test-associate`. Er koppelt nicht, schreibt nicht in die Datenbank, tippt nicht
und verändert kein Profil. Echte Eintragsanlage bleibt Schritt 7 von Hand.
