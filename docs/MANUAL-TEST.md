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
   Nacharbeit bei Sperren/Entfernen/gültigem Neuladen erhalten; ungültiges Neuladen
   leert sie. UTF-8-BOM akzeptieren; Warnung zu Wörtern verwendeter Passwörter prüfen.
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
13. **Profilhilfe und Stärke.** Profil 0, 2 und 12 normal sowie kompakt wählen.
    Theoretisch (Konzept) 51,1 / 64,4 / 153,1 Bit; tatsächlicher Wert aus der
    realen Quelle separat. Vergleich und G50 mit Dezimalkomma / Zehnerpotenz
    sichtbar. ⓘ per Hover und Klick sowie per Tastatur öffnen/schließen.
    ⓘ innerhalb der Profil-Auswahlliste öffnet die passende Profilhilfe, ohne
    das ausgewählte Generierungsprofil zu ändern. Hilfetitel und Grundsatz
    im Dialog jeweils nur einmal, Info-Symbole ohne Überlappung.
    „Profile erklärt“: alle 13 Profile, neue Regel, Konzeptgewinn, tatsächlicher
    Unterschied, öffentliches Beispiel und Grundsatz zu bekannten Zusätzen
    prüfen. Tabelle und Bedienungshilfen aufklappen, schmale Fenster scrollen;
    Passwort darf nicht durch Hilfen unlesbar werden. „Nur Deutsch“, Sperrliste
    und Trennerausschluss ändern Core-Werte, Konzeptwerte bleiben fest.
    Nach Bearbeitung kein G50-/Zeichenvergleich und kein Bonus; Nacharbeit bei
    Einstellungswechsel erhalten. ⓘ für alle im README genannten Funktionen
    auffindbar, auch Bildschirmschutz/--allow-screencapture und KeePassXC-Dialog.
14. **Freie Rotationsauswahl und Persistenz.** Eigenen `KAIROS_SETTINGS_FILE`-
    Testpfad verwenden. Rotation aktivieren, genau 2/4/8/12 wählen, mehrfach
    generieren; nur passende Muster, keine behauptete aktuelle Profilnummer.
    Konzeptminimum 64,4 Bit, tatsächliche konservative Rotationsgrenze und
    Profilbonus aus dem Core (aktuell +0,0 Bit). Bonus-ⓘ erklärt Ungleichheit,
    Überlappung, Mehrquellen-Grenze und bedingte +4 Bit aus dem Konzept.
    Auf zwei reduzieren: letzten beiden Haken nicht entfernbar. Wieder auf vier
    erweitern. Neustart: Aktivierung und genau diese vier Profile bleiben.
    Datei enthält ausschließlich Version, Aktivierung, Profilnummern und die
    beiden Tray-Einstellungen (`desktop`); keine Passwörter.
    Mit 9/12 Minus ausschließen: gültig; zusätzlich 0 aktivieren: sichtbarer
    Generierungsfehler, keine stille Ersatzwahl. Deutsch-Override für alle.
    Fehlende, ungültige, zu große und unlesbare Testdatei sowie Schreibfehler
    testen; Hinweis und dokumentierter Fallback, kein automatisches
    Überschreiben ungültiger Daten. Aktivierung aus/an erhält die Auswahl.

15. **Tray und Schließen (manuell offen).** Normal starten, Tooltip „Kairos
    Passwort“, K-Icon, Linksklick und Doppelklick prüfen. X: einmaligen Hinweis
    bestätigen, Fenster verschwindet aus Taskleiste, Tray bleibt. Wieder öffnen,
    erneut X: kein Hinweis. Neustart mit eigenem Einstellungs-Testpfad: Verhalten
    bleibt. Haken „Beim Schließen in den Tray“ aus → X beendet. Tray → Beenden
    beendet auch bei Haken an. Während einer KeePassXC-Anfrage wartet Beenden
    auf das Ergebnis. Tray-Ausfall darf kein unerreichbares Fenster hinterlassen.
16. **Hotkeys im Tray (manuell offen).** `--tray` ohne sichtbaren Fensterblitz
    starten. Im leeren Testeditor F9: frisch, kompakt, Originalziel; nach bewusstem
    Klick einfügen. X zum Verbergen, im Testeditor F10: frisch tippen + geschützt
    kopieren, Kairos bleibt verborgen und stiehlt keinen Fokus. X/F11: sichtbare
    KeePassXC-Bestätigung, kein automatisches Speichern/Tippen. Hotkey-Konflikte
    und Dienstfehler prüfen. Tray → Neues Passwort kopieren: aktuelles Profil/
    Rotation und Filter; Win+V ausgeschlossen, nach 30 s eigene Kopie leer,
    späterer fremder Text bleibt. Generierungs-/Clipboard-Fehler öffnen das
    Fenster; laufender/angenommener F11-Auftrag verhindert Ersetzung.
17. **Capture nach Verbergen (manuell offen).** Mit Schutz X/Tray/erneutes Öffnen
    mehrfach; Snipping Tool und Teams nach Schritten 7/8. F9/F11 aus `--tray`
    ebenfalls geschützt. Separater Start mit `--tray --allow-screencapture`:
    weiterhin aufnehmbar, Dauerbanner beim Öffnen. Zweite Instanz mit anderem
    Opt-out ändert die Policy der ersten nicht.
18. **Autostart und Startmenü (manuell offen, isoliertes Windows-Testkonto).**
    Keine Einträge im produktiven Benutzerkonto für diese Abnahme erstellen.
    „Mit Windows starten“ in App/Tray wechselseitig an/aus, Registry-Zeile mit
    Leerzeichen und Umlauten/`--tray` prüfen. Ab-/Anmelden: verborgen, Hotkeys
    aktiv. Im Task-Manager deaktivieren: Hinweis/gesperrte Änderung, Bytes bleiben
    unverändert; dort wieder aktivieren. EXE verschieben: anderer Pfad sichtbar,
    erst „Pfad aktualisieren“ ersetzt ihn. Startmenü-Haken: .lnk-Ziel,
    Arbeitsverzeichnis, Beschreibung, Icon; öffnen, entfernen. Eine fremde Datei
    am gleichen Testpfad muss unberührt bleiben. ⓘ aller drei Einstellungen prüfen.
19. **Einzelinstanz und Entfernen (manuell offen, isoliertes Testkonto).**
    Verborgene erste Instanz, zweiter normaler/`--tray`-Start → erstes Fenster
    sichtbar, zweite endet 0, keine Hotkey-Konflikte. Zweiter `--popup`-Start →
    bestehender Editor kompakt, kein neues Passwort, keine neue Einfügefreigabe.
    Gleichzeitige Starts und Neustart nach Absturz prüfen. `--remove-integration`
    (auch zusammen mit anderen Flags) ohne Fenster: nur Run-Wert und eigene .lnk
    weg, Exitcode 0; erneut idempotent. Fremde .lnk nicht entfernen; Fehler liefert
    ungleich 0. `StartupApproved` bleibt unverändert. Eigene Testeinträge abschließend
    entfernen und kontrollieren; keine produktiven Integrationseinträge anfassen.

Automatisiert: `desktop_integration` arbeitet ausschließlich mit
`KairosTest-<pid>-run` und `.lnk`/EXE in TempDir, RAII-Aufräumen auch bei Panik.
`--nocapture` gibt die bereinigten Namen/Pfade aus. Der Einzelinstanz-Test nutzt
`Local\KairosTest-<pid>-instance`, schließt alle Kernel-Handles und prüft, dass
der Mutex danach nicht mehr geöffnet werden kann. App-Tests verwenden eguis
`run_logic` ohne Fenster und eine Fake-Zwischenablage. Sie belegen den verborgenen
Aktionspfad, ersetzen die oben markierten Desktop-Prüfungen aber nicht.

SendInput bestätigt Windows-Annahme, keine Feldzustellung. Clipboard-/Capture-
Schutz hängt von beachtender Software ab. Kameras, privilegierte Tools, bestehende
Handles und weitere RAM-/UI-/OS-/Zielkopien sind nicht vollständig abgedeckt.
Renderer-Screenshots sind UI-Belege, kein OS-Capture-Nachweis. Rust-Tests und
`node --check browser/kairos-bridge.user.js` / `node browser/test-userscript.cjs`
prüfen Zustände/Protokolle und ersetzen die Desktop-Abnahme nicht.
