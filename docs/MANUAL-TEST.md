# Manuelle Gesamtcheckliste

Nur frische, nirgendwo verwendete Testpasswörter und leere Testfelder verwenden.
Keine echten Logins absenden. Build-SHA, Windows-Build, relevante App-Versionen,
Datum und Ergebnis je Schritt notieren. **Kein bereits ausgeführter Abnahmebeleg.**
Alle F-Tasten unten mit **Strg+Umschalt** drücken.

1. **Start/Quellen.** Normal und mit --popup starten. Profil 0 Deutsch/6.907,
   Profil 1 DE+EN+LA/20.697, etwa 51,0 bzw. 57,3 Bit ohne Sperrung. Deutsch-Override
   zeigt 6.907 und Hinweis auf fehlende Profil-1-Bits. Profile 2–12,
   Trennerausschlüsse, alle Trenner ausgeschlossen, Rotation mit Minimum/+0,0 Bit
   prüfen. Bearbeitung entwertet Stärke; Einstellungen bewahren die Nacharbeit.
2. **Härtungsreport.** Vier Maßnahmen mit Ergebnis und ggf. numerischem Fehler
   prüfen. Teilfehler dürfen nicht als voller Erfolg erscheinen; App bleibt
   bedienbar. Hotkeys, DLL-Laden, Eingabemethoden und Diagnosekompatibilität mit
   aktivierter Härtung prüfen. Keine Systemrichtlinien zur Fehlerinjektion ändern.
3. **Sperrliste.** Eigenen KAIROS_BLOCKED_WORDS_FILE-Testpfad verwenden. Wort des
   unbearbeiteten Passworts sperren; Ausgabe/Quellgröße/Entropie prüfen.
   Großschreibung/innere Ziffer werden rückgängig gemacht. Nach Handänderung
   keine Wortschaltflächen. „ AHORN “/Duplikate normalisieren, Mehrwort-/Ziffern-
   eingabe ablehnen. Neustart, Entfernen, Neustart. Datei enthält nur gewählte
   Wörter, nie das Passwort. Ungültige/unlesbare Testdatei sperrt Generierung
   bis Korrektur/Neuladen. Schreibfehler erhält bisherigen Filter mit Hinweis.
4. **F9/Fokus.** Leeres Editor-Testfeld fokussieren, F9, länger als fünf Sekunden
   bearbeiten, Einfügen. Originalziel/identischer Text, kein Enter/Tab. Kopierhaken
   an/aus, zweiter Klick. Start ohne F9 erlaubt kein Einfügen. „Kopieren und
   zurückspringen“ kehrt nur zum Originalfenster zurück; fremde aktive App wird
   nicht verdrängt. Ziel schließen/wechseln muss abbrechen.
5. **Autotype/F10.** Öffentliches Muster `PUBLIC-ÄÖÜäöüß@€#+'!?()_` über F9
   auf deutschem Layout prüfen; AltGr+Q bleibt @. F10 bei minimiertem Kairos in
   Editor und Browser. Tippen/Clipboard identisch, kein Kairos-Fokus. Erhöhtes
   Ziel/gehaltene Modifier abbrechen. Teileingaben nicht wiederholen. DOM-Wechsel
   im gleichen HWND bleibt eine Schutzgrenze.
6. **Clipboard/Win+V/Hotkeys.** Kopieren und vor 30 s einfügen; Win+V ohne Passwort.
   Nach 30 s eigene Kopie leer. Später kopierter fremder öffentlicher Text bleibt
   nach Frist und normalem App-Ende erhalten. F9/F10/F11 mit eigener Testbelegung
   starten, Fehler prüfen, freigeben und „Hotkeys erneut registrieren“.
7. **Snipping Tool.** Ohne Opt-out lokal sichtbar, Aufnahme mit Snipping Tool und
   Win+Shift+S samt Desktop. Ab Windows 10 2004 Kairos ausgeschlossen; älterer
   Monitor-Fallback mit leerer Fläche und Hinweis. Normal, --popup, F9/F11 sowie
   Pairing-, Sperrlisten-, Lizenz- und Sicherheitsdialog prüfen. Fehlende API warnt.
8. **Teams/Opt-out.** Eigene Testsitzung, Bildschirm und ggf. Fenster teilen;
   separater Testempfänger prüft. Mit --allow-screencapture, auch --popup, erneut
   testen. Dauerbanner beim Scrollen, F11 und Pairing sichtbar, öffentlicher
   Inhalt aufnehmbar. Neustart ohne Flag schützt wieder. Capture-Pfad und
   Windows-/Teams-/Tool-/Treiber-Version dokumentieren.
9. **Violentmonkey.** Eigenes Testbrowserprofil und wegwerfbare KeePassXC-DB nach
   [Browser-Anleitung](../browser/README.md) selbst einrichten. Chrome/Edge und
   Firefox separat. Ohne Pairing kein Vorschlag; Code nach zwei Minuten/Abbruch
   ungültig. Code nur im VM-Menü. Unterbrochene Antwort über „Ausstehende Kopplung
   prüfen“ aufnehmen. Falsche HMAC/Host/Origin/Nonce abweisen. Schutz nicht abschalten.
10. **Kontext.** Fokussierte Testseite mit Pfad/?Query/#Fragment, F11 binnen 5 s.
    Vorschlag nur Origin+Pfad, erst nach Übernahme im Feld. Benutzername nur nach
    Menüaktion, höchstens 10 s. Hintergrundtab, iframe, anderer Browser/Editor und
    veraltete Meldung kein gültiger Vorschlag. Script liest keine Passwörter aus.
11. **F11/KeePassXC.** Ohne KeePassXC Fehler ohne Tippen. Im isolierten Lauf
    koppeln, Ablehnung separat, dann Eintrag mit Test-URL/-Benutzer und ggf.
    bestehender Gruppen-UUID bestätigen. DB selbst kontrollieren. Erst „Jetzt ins
    gemerkte Feld tippen“ tippt dasselbe Passwort einmalig. Kopierhaken, zweiten
    Klick, Sperre/DB-Wechsel, Widerruf/Timeout prüfen. Kein Retry; nach Timeout
    kann ein Eintrag bestehen. Keine echten Tresore verwenden.
12. **Lizenzen/Fehler.** Dialog zeigt DE- und EN/LA-Provenienz, Datenhinweise und
    Schriftlizenzen. Nur eigene Test-DPAPI-Dateien/Ports für Korruption/Portbelegung
    verwenden. Kein Klartext-Fallback/automatischer Ersatz bestehender Zuordnung.
    Ignorierte echte Tests nur isoliert gemäß Crate-/Browser-README ausführen.

SendInput bestätigt Windows-Annahme, keine Feldzustellung. Clipboard-/Capture-
Schutz hängt von beachtender Software ab. Kameras, privilegierte Tools, bestehende
Handles und weitere RAM-/UI-/OS-/Zielkopien sind nicht vollständig abgedeckt.
Renderer-Screenshots sind UI-Belege, kein OS-Capture-Nachweis. Rust-Tests und
`node --check browser/kairos-bridge.user.js` / `node browser/test-userscript.cjs`
prüfen Zustände/Protokolle und ersetzen die Desktop-Abnahme nicht.
