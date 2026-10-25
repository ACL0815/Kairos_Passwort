# egui-Standardschriften

eframes Feature `default_fonts` bindet `epaint_default_fonts` 0.37.0 ein.
Diese Dateien sind die unveränderten Lizenztexte aus dessen Cargo-Paket.
Die Schriftdateien selbst werden von der normalen Crate-Abhängigkeit eingebettet.

Herkunft, Quellstand und einzelne Originaldateien:
[egui 0.37.0 / epaint_default_fonts/fonts](https://github.com/emilk/egui/tree/0.37.0/crates/epaint_default_fonts/fonts).
Abruf und Prüfung am 25.10.2026.

| Schrift/Asset | Lizenztext | Verwendung |
|---|---|---|
| Ubuntu Light | [UFL.txt](UFL.txt), Ubuntu Font Licence 1.0 | Proportionale UI-Schrift |
| Hack Regular | [Hack-Regular.txt](Hack-Regular.txt), MIT/Bitstream Vera, DejaVu-Hinweise | Monospace-Fallback hinter JetBrains Mono NL |
| egui-icons / emoji-icon-font | [emoji-icon-font-mit-license.txt](emoji-icon-font-mit-license.txt), MIT | Mitgelieferte egui-Sonderglyphen |
| Noto Emoji Regular | [OFL.txt](OFL.txt), SIL OFL 1.1 | Lizenz vorsorglich mitgeliefert; `monochrome_emoji_fonts` ist nicht aktiviert |

Kein Font wird auf dem System installiert. Bei einer späteren Distribution
sind dieser Ordner und die JetBrains-OFL-Hinweise mitzuliefern.
