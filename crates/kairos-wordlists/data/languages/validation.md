# Prüfstand: Diagnoseleiter Stufe 1, STOP

**Historischer Prüfstand, kein aktueller STOP-Vermerk.** Die nachfolgende
Matrix gehört zu `691860b`. Der neue Standard und der aktuelle Prüfstand
werden in [Stufe 2](../curation-stage-two.md) dokumentiert.

25.10.2026, Europe/Berlin. Ausgangshead `ec6a9562dbda9bab3742b6d3f766d8f11d289a6b`.
Ausführung ausschließlich im eigenen Worktree `t3-139781e4` mit der Build-Umgebung
aus `C:\Users\Axel_desktop\.t3\projects\kairos-password\target\Enter-KairosBuildEnv.ps1`.
Im Hauptcheckout wurden keine Arbeitsdateien geschrieben. Cargo-/TEMP-Ausgaben
liegen im eigenen Worktree unter `target/`; ausführliche Logs unter
`target/semantic-validation/`.

| Prüfung | Befehl | Exit | Befund |
|---|---|---:|---|
| Format | `cargo fmt --check` | 0 | Grün |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | Grün |
| Workspace-Tests | `cargo test --workspace` | 0 | 29 Integrationstests + 2 Doctests bestanden, 0 fehlgeschlagen/ignoriert |
| DE-Reproduktion | `cargo run -p kairos-wordlists --example rebuild-german -- --check` | 0 | 7.130 unveränderte Wörter |
| EN/LA-Reproduktion | `cargo run -p kairos-wordlists --example rebuild-languages -- --check` | 0 | Quellen, Lizenzen, alle Kandidatenentscheidungen, Ausgaben, Hashes, Bericht und Glossen bytegleich reproduziert |
| Re-Verifier-Gate | unverändertes `t3-d9ce504b/target/verify/check-independent-findings.ps1` | 0 | `findings=0`, alle 14 bekannten Schreibweisen fehlen |
| Eigene zweite Abnahme | `pwsh -NoProfile -File crates/kairos-wordlists/tools/check-semantic-sample.ps1` | **1** | **5 verbliebene Sperr-/Zweifelfälle, STOP** |
| DE-Isolation | `git diff --exit-code ec6a956 --` mit allen deutschen Listen/Hashes/Quellen/Sperren/Ergänzungen/Pipeline/Tests | 0 | Keine Änderungen |
| Whitespace | `git diff --cached --check` | 0 | Grün |

Die ursprüngliche Gate-Reproduktion vor der Änderung ergab Exit **1** mit
**14** Funden. Die eigene erste Endlistenprobe nach der Gesamtklassifikation
ergab **10** Sperr-/Zweifelfunde. Der anschließende einzige Korrekturdurchgang
ergänzte **232** semantische Sperren im ganzen Kandidatenpool. Die zweite eigene
Probe findet erneut **5** Fälle. Es werden keine weiteren semantischen
Reparaturen ausgeführt. Dieser Arbeitsstand ist **fachlich nicht abgenommen**.

| Artefakt | Anzahl | SHA-256 |
|---|---:|---|
| DE, unverändert | 7.130 | `c740ce05d355b2296c2feb940733f84f9bc3e1972fcfad9cbfdf5e4236bf3175` |
| EN | 6.966 | `511107e3b370cb4c184ef9c5fc0675fbe226fbbe06ee533f1eba6a6047784881` |
| LA | 6.966 | `95d8b89f37c794ce6c2ea446f59551aff7cb08ce98e087fa6269630d0e0d0987` |
| Vollständige semantische Entscheidungen | 21.190 | `e2b7f51f134ce27745cf3813c73d10f0a1bdd3fcc9e1652cf71e511fdfeccb86` |

EN/LA sind untereinander und gegenüber DE disjunkt; `combined()` enthält alle
21.062 Wörter mit korrekten Flags. Gegenüber dem Ausgangshead: EN 687 entfernt,
523 hinzugefügt; LA 974 entfernt, 810 hinzugefügt; DE 0 Änderungen. Die eigenen
Nachprüfungsfälle `pouncing`, `bladder`, `absurde`, `servula`, `situs` sind
weiter enthalten und in `own-final-sample-v2.tsv` einzeln begründet. Keiner dieser
Fälle wurde nach Auslösen der Stop-Regel durch eine weitere Reparatur versteckt.

DE wurde vollständig mitgeprüft. Die 299 Meldungen einschließlich Zweifelsfällen
stehen in `de-findings-report.tsv`; Änderungen daran gehören auf PR #2.
Die vollständige Diagnose einschließlich Stichproben-Seeds, Keyword-Recall,
Konfliktentscheidungen, Grenzen und Übergabe steht in
[curation-diagnosis.md](../curation-diagnosis.md).

Das einmalige lesende Opus-Review urteilt **nicht abnahmefähig** und bestätigt
die offenen semantischen Blocker. Die lokale maschinenlesbare Zusammenfassung
enthält nach dem Review auch das eigene Gate mit Exit 1. Der vollständige
Review-Text einschließlich Einordnung und offener Übergabe steht in
[fixdiff-review.md](fixdiff-review.md). Nach der Stop-Regel wurden keine weiteren
semantischen Änderungen vorgenommen; die technisch geprüften Quelltexte und
die konsumierte Entscheidungsdatei sind seit der Prüfung unverändert.
