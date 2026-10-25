<img src="assets/icon.svg" width="64" height="64" alt="">

# Kairos Passwort

Komfortabler, sicherer Wortpasswort-Generator für Windows, geschrieben in Rust.

## Leitstern

> Maximiere den Komfort für User bei dem Passwort-Abschreiben und der Passwort-Generierung, ohne die Sicherheit zu kompromitieren.

Die unveränderte Konzeptquelle steht in [docs/Grundkonzept.md](docs/Grundkonzept.md).
Geplant sind deutsche Wortpasswörter, Profile 0–12, nachvollziehbare Entropie,
farbige Hervorhebung, Hotkeys und die Anbindung an KeePassXC.

## Status

Dieses Repository enthält das kompilierbare Fundament und den gemeinsamen
API-Vertrag. Generator, Wortlisten, Windows-Integration, verschlüsselter Speicher
und GUI sind noch nicht implementiert. Der Binary-Einstieg meldet diesen Status;
er erzeugt keine Passwörter. `generate` liefert `GenerateError::NotImplemented`,
`entropy_bits` ist ein ausdrücklich dokumentierter `todo!()`-Stub.

Die priorisierten Implementierungsschritte stehen in der [Roadmap](docs/ROADMAP.md).
[Architektur und API-Vertrag](docs/ARCHITECTURE.md),
[Open-Source-Vorbilder](docs/SOURCES.md) und
[GUI-Stil](docs/design/UI-STYLE.md) bilden die gemeinsame Arbeitsgrundlage.

## Build und Prüfung

Windows x64 mit Rustup, der Toolchain `stable-x86_64-pc-windows-msvc` und
Visual Studio Build Tools (Desktopentwicklung mit C++, Windows SDK) verwenden.
Das Scaffold wurde mit Rust/Cargo 1.80.0 geprüft und nutzt Edition 2021, Resolver 2.
`rust-toolchain.toml` wählt `stable`; CI installiert die jeweils verfügbare Stable-Version.

```powershell
rustup show
cargo --version
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
cargo run -p kairos
```

Falls eine bestehende Rustup-Installation in dieser Sitzung nicht im `PATH` liegt:

```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
```

`Cargo.lock` wird für die Anwendung eingecheckt. Die GUI-Abhängigkeit egui/eframe
und SQLCipher folgen in ihren eigenen PRs; das Scaffold bringt keine unbenutzten
GUI- oder Datenbankbibliotheken mit.

## Workspace

| Paket | Verantwortung |
|---|---|
| `kairos-core` | Gemeinsamer Vertrag, Generator, Profile und Entropie |
| `kairos-wordlists` | Eingebettete Wortlisten, zuerst Deutsch |
| `kairos-store` | Externe Wortlisten in SQLCipher, Schlüssel über Windows DPAPI |
| `kairos-platform` | Zwischenablage, Hotkeys, Autotype und Fokus-Rücksprung |
| `kairos-keepassxc` | Client für das KeePassXC-Browser-Protokoll |
| `kairos` | Geplante egui/eframe-GUI, verdrahtet alle Komponenten |
| `browser/` | Spätere Violentmonkey-Phase und lokale Brücke |

## Lizenz

Eigener Projektcode steht wahlweise unter [MIT](LICENSE-MIT) oder
[Apache-2.0](LICENSE-APACHE): `MIT OR Apache-2.0`.
Wortlisten, Schriftarten und weitere fremde Daten erhalten vor ihrer Aufnahme
eine dokumentierte, redistributionsfähige Lizenz; ihre Bedingungen gelten zusätzlich.
