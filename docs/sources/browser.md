# Violentmonkey und lokale Brücke

Zuständig: **nur PR-10** (`feat/browser-violentmonkey`, auf PR-6).
Diese Datei startet die spätere Protokoll-/Sicherheitsanalyse.

| Projekt | Link (Permalink mit Commit/Tag) | Lizenz | Datei/Modul | Übertragbares Prinzip | Passung 1–5 |
|---|---|---|---|---|---|
| 2FAS Browser Extension | [Quellstand](https://github.com/twofas/2fas-browser-extension/tree/9ad6e83d12792d0b3d8d66f018ace6765ebc3139) | GPL-3.0; Dateilizenzen beachten | `src/background/`, `src/background/events/` | Browser-Ereignisse, Kopplung und vertrauenswürdige Nachrichtenflüsse voneinander trennen | 4 |
| keepassxc-browser | [Protokollclient](https://github.com/keepassxreboot/keepassxc-browser/blob/c0c42f557a27c4b05c3928820659c65987583a04/keepassxc-browser/background/keepass.js) | GPL-3.0; Dateilizenz beachten | `keepassxc-browser/background/keepass.js` | Authentifizierte Verbindung, Nachrichtenvalidierung und Lebenszyklus der lokalen Brücke | 5 |

Ein Userscript besitzt andere Vertrauensgrenzen als eine Browser-Erweiterung.
Transport, erlaubte Ursprünge, Kopplung, Nachrichtenformate und Fehlerzustände
müssen vor der Umsetzung festgelegt werden. Die Vorbilder liefern Prinzipien;
ihre Erweiterungsarchitektur wird nicht ungeprüft auf Violentmonkey übertragen.
