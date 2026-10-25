# kairos-core

Gemeinsamer API-Vertrag für Wortquellen, Profile 0–12, Generator, Entropie und
GUI-Segmente. Dieses Scaffold enthält nur Vertragstypen und Generator-Stubs.
`ProfileId` prüft den Wertebereich bereits; `generate` meldet `NotImplemented`,
`entropy_bits` ist noch nicht aufrufbar. PR-1 implementiert die Fachlogik.

Der Vertrag steht in [ARCHITECTURE.md](../../docs/ARCHITECTURE.md).
Der Core kennt weder Windows noch GUI, Datenbank oder KeePassXC.

## Open-Source-Vorbilder (modellieren, nicht kopieren)

Platzhalter für die Auswertung in PR-1: [core.md](../../docs/sources/core.md).
Profilrotation und Speicherhärtung werden später getrennt in
[profiles.md](../../docs/sources/profiles.md) und
[protection.md](../../docs/sources/protection.md) dokumentiert.
