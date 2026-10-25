# Violentmonkey-Brücke und F11

PR-6b (Priorität 3) und PR-10 (Priorität 6) verbinden Seitenkontext mit dem
KeePassXC-Hotkey. Bewusst bestätigter Userscript-Prototyp; eine echte Extension
mit `activeTab` und Native Messaging bleibt der Zielpfad.

## Einrichten

1. Kairos starten. Unter „Browser-Brücke koppeln“ einen Kopplungscode anzeigen.
2. Violentmonkey selbst in einem **isolierten Testprofil** installieren. Neues
   Script anlegen, [kairos-bridge.user.js](kairos-bridge.user.js) vollständig
   einfügen und speichern. Keine `@require`-Bibliotheken.
3. HTTP(S)-Testseite öffnen. Im **Violentmonkey-Erweiterungsmenü** „Kairos: mit
   Kopplungscode verbinden“ ausführen, den Kairos-Code eingeben. Niemals in ein
   Seitenformular oder auf Aufforderung der Seite. Code gilt zwei Minuten und
   höchstens fünf Versuche; neuer Code ersetzt nur den unverbrauchten Code.
4. Leeres Passwortfeld fokussieren, **Strg+Umschalt+F11** drücken. Kairos merkt
   das Plattformziel und generiert frisch. Ein höchstens fünf Sekunden alter,
   zum lokalen Vordergrund-Browser-HWND/PID passender Kontext wird als Vorschlag
   angeboten. URL bleibt bis zur geprüften Übernahme leer; sonst manuell eingeben.
   Query/Fragment werden immer entfernt. HWND/PID sind kein aktiver-Tab-Beweis.
5. Benutzername optional selbst eingeben. Alternativ im fokussierten Formular
   über den gesonderten VM-Menüpunkt „Formular-Benutzername einmal senden“ prüfen
   und sofort F11 drücken. Folgende Heartbeats enthalten keinen Benutzernamen
   und ersetzen diesen Kontext. Keine Benutzername-Persistenz.
6. Optional Name und 32-stellige hexadezimale UUID einer **bestehenden** Gruppe
   eingeben; ohne beides Standardgruppe. Nicht vorhandene UUID kann in KeePassXC
   auf Standardgruppe zurückfallen. Keine neue Gruppe wird angelegt.
7. URL/Benutzername/Zielfeld prüfen, „Eintragen in KeePassXC“. KeePassXC muss
   laufen, die gewünschte Testdatenbank entsperrt und Browserintegration vom
   Nutzer selbst aktiviert sein. Fehlende Zuordnung: „KeePassXC jetzt koppeln“,
   Freigabe dort, anschließend Eintragen separat bestätigen.
8. Nach Annahme den Eintrag in KeePassXC prüfen, dann „Jetzt ins gemerkte Feld
   tippen“. Der zweite Klick erneuert die Eingabefreigabe nach langen Dialogen.
   Optional wird nach angenommener Eingabe geschützt kopiert. Kein Formular-Submit.

## Datenschutz und Protokoll

Nur `127.0.0.1:19471`, keine LAN-/IPv6-Bindung. Fester Port für Script und exakten
Host-Check. Port belegt oder DPAPI kaputt: Brücke sichtbar deaktiviert, manuelle
F11-URL weiter möglich. Keine automatische Portwahl/Klartext-Ersatzablage.

Nach bewusster Kopplung meldet das Script `origin + pathname` bei Feldfokus,
Fensterfokus und `visibilitychange`, zusätzlich alle zwei Sekunden bei sichtbarem,
fokussiertem oberstem Dokument. Keine Frames. **Auch Pfade können Geheimnisse,
Dokumentnamen oder IDs enthalten.** Script auf sensiblen Sites deaktivieren bzw.
`@match` auf erlaubte Sites begrenzen. Standard ist die breite HTTP(S)-Freigabe.
Keine Passwortauslese, keine URL-/Benutzername-Logs, kein Verlauf. Nach Autotype
kann die Zielseite das eingetragene Passwort selbstverständlich lesen.

`POST /v1/pair`, `/v1/context`, `/v1/clear`: `application/json`,
`X-Kairos-Token` mit 256 Bit Zufall, JSON mit Unixsekunden-`timestamp` und
32-stelliger zufälliger Hex-`nonce`, optional `url`/`username`. Keine Cookies
(`anonymous: true`). **Bearer-Token, kein HMAC, keine Transportverschlüsselung
oder Serveridentität.** Normale Origins (auch `null`), Referer, doppelte kritische
Header, Chunking, OPTIONS und unbekannte JSON-Felder werden abgewiesen.
Exakter Host `127.0.0.1:19471`, kein CORS, Responses nur 204/403 ohne Body.
Bei `/v1/pair` authentisiert ausschließlich der kurzlebige Code den Request;
das Script erzeugt separat einen neuen 256-Bit-`client_token` im Body. Erst nach
DPAPI-Persistenz und 204 wird dieser im GM-Speicher abgelegt. Der Code wird
verbraucht und ist danach kein gültiger Request-Schlüssel. Keine Secrets in
Serverantworten. Ein verlorenes 204 kann bewusste Wiederherstellung erfordern.

Zeitfenster ±30 Sekunden. Nonces 61 Sekunden im laufenden Server, maximal 1024;
bei vollem Cache ablehnen statt vorzeitig entfernen. Neustart setzt den Cache
zurück; Replay-Abwehr über Neustarts hinaus ist keine Zusage. Maximal 12 KiB
Header+Body, 8 KiB Body, 24 Header; Lese-Gesamtfrist eine Sekunde, einzelne
Lese-/Schreiboperation 200 ms. Serielle Bearbeitung begrenzt Ressourcen, lokale
Überlastung kann Kontext verhindern. Server entfernt URL-Daten nach fünf
Sekunden. Der angezeigte F11-Entwurf hält seinen bei F11 erfassten Vorschlag bis
zum Schließen; danach erfolgt keine neue Browserprüfung.

## Bedrohungsmodell und Grenzen

| Risiko | Schutz und bewusste Grenze |
|---|---|
| Bösartige Webseite | Host-Prüfung gegen DNS-Rebinding, kein normaler Origin, Custom-Header, Token/Nonce und POST-Framing. Kein unprivilegierter DOM-/`postMessage`-Zugang zu GM. CORS allein genügt nicht. Seite kann Inhalt und eigene URL beeinflussen: Vorschläge immer bestätigen. Keine Passwortantwort. |
| Anderer lokaler Prozess | Ohne Schlüssel ist Token-Raten praktisch ausgeschlossen; Header beweisen keine Prozessidentität. Ein Prozess, der den Port zuerst belegt, kann den Token abfangen. Lokales Mitlesen und Malware im selben Windows-Konto oder Administratoren sind keine geschützte Grenze. |
| Manipuliertes Userscript / Extension | Kann GM-Schlüssel lesen und Kontext fälschen. Pairing schützt nicht gegen den gepaarten Client. Script prüfen, keine fremden Updates/`@require`, `@match` begrenzen. `@connect` allein ist keine garantierte Engine-Sicherheitsgrenze. |
| Falsches Fenster / Tab / DOM-Feld | Lokal erfasstes HWND/PID nur Hinweis, Userscript ohne globale Tabs-API. Fokus-/Navigationsrennen und Feldwechsel innerhalb desselben HWND nicht erkennbar. URL und ursprüngliches Ziel ausdrücklich prüfen. Unbekanntes Browserprogramm liefert keinen Vorschlag. |
| KeePassXC-Ablehnung / Timeout | Kein Tippen. Schreibfehler kann bereits einen Eintrag hinterlassen: dort prüfen, kein automatischer Retry. `Accepted` belegt keine Eintrags-ID oder Speicherung auf Platte; Hash-Prüfung beseitigt keine spätere DB-Auswahl. |
| Autotype-/Clipboard-Fehler nach Annahme | Sichtbares Teilergebnis, Ziel einmalig verbraucht. Kein automatischer Retry, kein Löschen eines möglicherweise angelegten Eintrags. Passwort bleibt zur bewussten manuellen Kopie erhalten. |

GM-Schlüssel: `kairos-bridge-v1-token`. Desktopablage:
`%APPDATA%\Kairos\browser-bridge.dpapi`, insert-only DPAPI CurrentUser. Der
bestehende DPAPI-Association-Adapter wird mit separater Datei/festem Marker
verwendet; keine KeePassXC-Schlüsselwiederverwendung. GM-Speicher ist keine
DPAPI-Zusage: Browserexport/-Backup/-Sync kann Kopien enthalten.

Bewusster Widerruf: Script deaktivieren, Schlüssel im VM-Menü vergessen, Kairos
beenden, eigene `browser-bridge.dpapi` selbst als Backup umbenennen, dann neu
starten/koppeln. Kairos löscht/ersetzt keine Zuordnung. Verlorener GM-Schlüssel:
derselbe Ablauf. KeePassXC hat die separate `keepassxc-association.dpapi`;
Widerruf dort selbst und Backup-Umbenennung gemäß Crate-README. Keine automatische
Neukopplung bei bestehender, beschädigter oder widerrufener Zuordnung.

## Prüfung und Open-Source-Vorbilder

`node --check browser/kairos-bridge.user.js` und `node browser/test-userscript.cjs`
prüfen das echte Script im Mock-Modell. Das ersetzt keinen VM-Kompatibilitätstest.
Falls die Engine automatisch Origin/Referer oder `Sec-Fetch-Site: cross-site`
anfügt, sicher ablehnen, keine Schutzprüfung abschalten. Echter ignorierter
Lesetest und manuelle Schritte: [MANUAL-TEST.md](../docs/MANUAL-TEST.md).

Modelled after: KeePassXC (Association/Eingabefreigabe), 2FAS Auth (bewusstes
Pairing), Syncthing (Host-/CSRF-Schranken), Violentmonkey (isolierte GM-API).
Bitwarden Native Messaging als späterer Extension-Pfad. Kein Implementierungscode
kopiert, kompatible Crates unverändert eingebunden. Tabellen mit Permalinks,
Lizenzen und Bewertungen: [browser.md](../docs/sources/browser.md) und
[keepassxc.md](../docs/sources/keepassxc.md).
