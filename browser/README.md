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
   und innerhalb von zehn Sekunden F11 drücken. Heartbeats senden keinen
   Benutzernamen; Kairos hält den einmal bestätigten Wert bei gleicher URL und
   Fensterzuordnung höchstens zehn Sekunden, ohne die Frist zu erneuern.
   Keine dauerhafte Benutzername-Persistenz.
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

Protokoll v2 akzeptiert ausschließlich `POST /v2/pair`, `/v2/context`,
`/v2/clear` und `/v2/probe` mit `application/json` (optional UTF-8-Charset).
JSON enthält Unixsekunden-`timestamp` als vorzeichenlose Ganzzahl und eine
32-stellige zufällige **kleingeschriebene** Hex-`nonce`, optional `url` und
`username`. Unbekannte JSON-Felder, einschließlich alter `client_token`, werden
abgewiesen. Alte v1-Pfade und jeder `X-Kairos-Token`-Header sind ungültig.

Jeder Request enthält `X-Kairos-HMAC` als genau 64 kleingeschriebene Hexzeichen.
Der Schlüssel wird aus kanonischem Base64url ohne Padding in **32 rohe Bytes**
decodiert. Die Signatur ist HMAC-SHA256 über folgende UTF-8-Zeichenfolge,
getrennt durch genau ein LF (0x0a), **ohne abschließendes LF**:

```text
kairos-bridge-v2
POST
<exakter Request-Pfad, z. B. /v2/context>
127.0.0.1:19471
<timestamp als kürzeste dezimale Ganzzahl>
<nonce exakt wie im JSON>
<SHA256 der exakten übertragenen UTF-8-Body-Bytes, 64 kleine Hexzeichen>
```

Keine JSON-Neusortierung oder URL-Normalisierung vor dem Hash. Auch Whitespace,
UTF-8-Zeichen und eine Änderung des Zeitstempels im Body verändern die Signatur.
Methode, Pfad und Host werden vor der Verarbeitung streng validiert und sind
zusätzlich signiert. Der Server vergleicht alle 32 MAC-Bytes konstantzeitig mit
`hmac::Mac::verify_slice`. Länge/Hexsyntax sind öffentliche Formatprüfungen.
Der gemeinsame [Testvektor](hmac-vector.json) wird von Rust, WebCrypto im echten
Userscript und unabhängig von Nodes `createHmac` geprüft.

Keine Cookies (`anonymous: true`), kein CORS, Responses nur 204/403 ohne Body.
Normale Origins (auch `null`), Referer, doppelte kritische Header, Chunking und
OPTIONS werden abgewiesen. Exakter Host `127.0.0.1:19471`. Fehlender Origin ist
für privilegierte GM-Anfragen erlaubt. Ein mitgesendeter Extension-Origin muss
exakt `chrome-extension://` plus 32 Zeichen `a`–`p` oder `moz-extension://` plus
UUID entsprechen. Header beweisen keine Extension- oder Prozessidentität.
`Sec-Fetch-Site` ist wegen unterschiedlicher Engines kein Auth-Signal.

**Pairing ohne Secret im HTTP-Kanal.** Kairos erzeugt 32 Zufallsbytes, zeigt sie
als 43-stelligen Base64url-Code und hält sie maximal zwei Minuten/fünf Versuche.
Der Nutzer überträgt ihn ausschließlich in das VM-Menü. `/v2/pair` enthält nur
Zeitstempel und Nonce und wird mit diesen rohen Code-Bytes signiert. Beide
Seiten berechnen lokal den separaten Dauerschlüssel
`HMAC-SHA256(code_bytes, UTF8("kairos-bridge-v2-client-key"))`. Diese feste
Domänentrennung unterscheidet Schlüsselableitung und Request-Signaturen.
Kairos speichert ihn insert-only unter DPAPI CurrentUser, bevor es 204 meldet.
Code und Dauerschlüssel werden **nie** in HTTP-Headern/-Bodies übertragen.
Nach Pairing wird der Code verworfen; normale Requests verwenden den abgeleiteten
Schlüssel. Kein Code-/Token-Fallback. v1-DPAPI-Marker werden nicht akzeptiert.

Vor dem Senden speichert das Script ausschließlich den abgeleiteten Schlüssel
vorläufig unter `kairos-bridge-v2-pending-key`. Nach 204 wird er unter
`kairos-bridge-v2-key` aktiviert. Unklarer Timeout/Netzwerkfehler bewahrt ihn,
ein 403 verwirft ihn. Bei verlorenem 204 im VM-Menü „Ausstehende Kopplung prüfen“
wählen. `/v2/probe` authentisiert einen frischen Request mit dem bereits
persistierten Schlüssel. Keine neue Pairing-/Schreibanfrage, kein automatischer
Retry. Pairing-Timeout zehn Sekunden. WebCrypto ist Voraussetzung; wenn die
Engine `crypto.subtle` auf HTTP-Seiten nicht bereitstellt, bleibt das Script
dort ohne Kontextmeldung. Pairing auf einer vertrauenswürdigen HTTPS-Testseite.

**Restgrenze beim Pairing.** Der manuelle Kanal (Kairos-Anzeige, geschützte
Zwischenablage, VM-Eingabe) muss vertrauenswürdig sein. Ein gefälschter
VM-Dialog oder Malware mit Zugriff darauf kann den Code stehlen. HTTP liefert
keine Serveridentität und keine authentisierten Antworten. Ein imitierter Port
kann 204 vortäuschen, Kontext lesen, Verfügbarkeit stören und aufgezeichnete
signierte Nachrichten innerhalb des Zeitfensters weiterleiten. Er bekommt
keinen Schlüssel zum Erzeugen neuer Nachrichten. Ein Pairing-Relay kann die
genau signierte Zuordnung auslösen, jedoch keinen eigenen Schlüssel einsetzen.
HMAC verschlüsselt keine URLs/Benutzernamen. Replay über Serverneustarts und
aktive Relays bleiben eine Grenze. KeePassXC nutzt stattdessen Native Messaging
und NaCl-Schlüssel/Association; 2FAS nutzt öffentliche Schlüssel/RSA-OAEP für
seinen Relay-Kanal. Deren Transport-/Identitätsgarantien werden hier nicht
behauptet. Für diese Stufe genügt der manuell übertragene hochentropische Code
als Besitznachweis; Native Messaging bleibt der spätere Zielpfad.

Jeder lokale Prozess mit Portzugriff, auch aus einem anderen Windows-Konto,
kann durch fünf falsche Pairing-HMACs das offene Pairing abbrechen. Der Nutzer
muss dann einen neuen Code erzeugen. Das Versuchslimit bleibt vor der HMAC-
Prüfung, damit falsche Besitznachweise ebenfalls gezählt werden. Lokale
Verfügbarkeit wird durch dieses Protokoll nicht garantiert.

Zeitfenster ±30 Sekunden. Nonces 61 Sekunden im laufenden Server, maximal 1024;
bei vollem Cache ablehnen statt vorzeitig entfernen. Neustart setzt den Cache
zurück; Replay-Abwehr über Neustarts hinaus ist keine Zusage. Maximal 12 KiB
Header+Body, 8 KiB Body, 48 Header; Lese-Gesamtfrist eine Sekunde, einzelne
Lese-/Schreiboperation 200 ms. Serielle Bearbeitung begrenzt Ressourcen, lokale
Überlastung kann Kontext verhindern. Server entfernt URL-Daten nach fünf
Sekunden. Der angezeigte F11-Entwurf hält seinen bei F11 erfassten Vorschlag bis
zum Schließen; danach erfolgt keine neue Browserprüfung.
Auch erzwungene Fokusmeldungen haben mindestens 500 ms Abstand; synthetische
DOM-Events werden ignoriert. Server: mindestens 250 ms zwischen Kontext-/Clear-
Anfragen vor Einfügen einer neuen Nonce. Temporäre Accept-Resets werden wieder
aufgenommen; ein beendeter Worker wird in Kairos sichtbar als ausgefallen gemeldet.

## Bedrohungsmodell und Grenzen

| Risiko | Schutz und bewusste Grenze |
|---|---|
| Bösartige Webseite | Host-Prüfung gegen DNS-Rebinding, kein normaler Origin, Custom-Header, HMAC/Nonce und POST-Framing. Kein unprivilegierter DOM-/`postMessage`-Zugang zu GM. CORS allein genügt nicht. Seite kann Inhalt und eigene URL beeinflussen: Vorschläge immer bestätigen. Keine Passwortantwort. |
| Anderer lokaler Prozess | Ein imitierter Port erhält nur zweckgebundene HMACs und Klartext-Kontext, keinen wiederverwendbaren Schlüssel. Keine Serveridentität, Vertraulichkeit oder Relay-Abwehr. Malware im selben Windows-Konto und Administratoren können Speicher/GM/DPAPI oder den manuellen Pairing-Kanal angreifen und sind keine geschützte Grenze. |
| Manipuliertes Userscript / Extension | Kann GM-Schlüssel lesen und Kontext fälschen. Pairing schützt nicht gegen den gepaarten Client. Script prüfen, keine fremden Updates/`@require`, `@match` begrenzen. `@connect` allein ist keine garantierte Engine-Sicherheitsgrenze. |
| Falsches Fenster / Tab / DOM-Feld | HWND/PID nur Hinweis, keine globale Tabs-API. Bei verzögerter Verarbeitung kann der Vorschlag zu einem anderen Fenster gehören als das früher erfasste Tippziel. Keine automatische Übernahme; ursprüngliches Ziel und URL prüfen. Tab-/DOM-Feldwechsel bleiben unerkennbar. Unbekanntes Browserprogramm liefert keinen Vorschlag. |
| KeePassXC-Ablehnung / Timeout | Kein Tippen. Schreibfehler kann bereits einen Eintrag hinterlassen: dort prüfen, kein automatischer Retry. `Accepted` belegt keine Eintrags-ID oder Speicherung auf Platte; Hash-Prüfung beseitigt keine spätere DB-Auswahl. |
| Autotype-/Clipboard-Fehler nach Annahme | Sichtbares Teilergebnis, Ziel einmalig verbraucht. Kein automatischer Retry, kein Löschen eines möglicherweise angelegten Eintrags. Passwort bleibt zur bewussten manuellen Kopie erhalten. |

GM-Schlüssel: `kairos-bridge-v2-key`. Desktopablage:
`%APPDATA%\Kairos\browser-bridge.dpapi`, insert-only DPAPI CurrentUser. Der
bestehende DPAPI-Association-Adapter wird mit separater Datei/festem Marker
verwendet; keine KeePassXC-Schlüsselwiederverwendung. GM-Speicher ist keine
DPAPI-Zusage: Browserexport/-Backup/-Sync kann Kopien enthalten. Rust puffert
Secrets in `Zeroizing`; JavaScript-Strings, CryptoKey und interne HMAC-/Runtime-
Kopien haben keine garantierte Speicherlöschung.

Bewusster Widerruf: Script deaktivieren, Schlüssel im VM-Menü vergessen, Kairos
beenden, eigene `browser-bridge.dpapi` selbst als Backup umbenennen, dann neu
starten/koppeln. Kairos löscht/ersetzt keine Zuordnung. Verlorener GM-Schlüssel:
derselbe Ablauf. KeePassXC hat die separate `keepassxc-association.dpapi`;
Widerruf dort selbst und Backup-Umbenennung gemäß Crate-README. Keine automatische
Neukopplung bei bestehender, beschädigter oder widerrufener Zuordnung.

## Prüfung und Open-Source-Vorbilder

`node --check browser/kairos-bridge.user.js` und `node browser/test-userscript.cjs`
prüfen das echte Script im Mock-Modell. Das ersetzt keinen VM-Kompatibilitätstest.
Chrome/Edge und Firefox zuerst real mit ihrer VM-Version abnehmen. Extension-
Origins/JSON-Charset sind getestet; gewöhnliche Web-Origins und Referer bleiben
verboten. Keine Schutzprüfung abschalten. Echter ignorierter
Lesetest und manuelle Schritte: [MANUAL-TEST.md](../docs/MANUAL-TEST.md).

Modelled after: KeePassXC (Association/Eingabefreigabe), 2FAS Auth (bewusstes
Pairing), Syncthing (Host-/CSRF-Schranken), Violentmonkey (isolierte GM-API).
Bitwarden Native Messaging als späterer Extension-Pfad. Kein Implementierungscode
kopiert, kompatible Crates unverändert eingebunden. Tabellen mit Permalinks,
Lizenzen und Bewertungen: [browser.md](../docs/sources/browser.md) und
[keepassxc.md](../docs/sources/keepassxc.md).
