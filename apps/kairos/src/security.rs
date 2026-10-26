//! Modelled after: KeePassXC's early bootstrap with explicit protection outcomes.
//! <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp>.

use eframe::egui;
use kairos_hardening::{HardeningReport, Measure, OsError, Outcome};

pub fn summary(report: Option<&HardeningReport>) -> &'static str {
    match report {
        Some(report) if report.all_applied() => "Prozesshärtung angewendet",
        Some(_) => "Prozesshärtung unvollständig",
        None => "Prozesshärtung nicht gemeldet",
    }
}

fn outcome(outcome: Outcome) -> String {
    match outcome {
        Outcome::Applied => "angewendet".into(),
        Outcome::Unsupported => "nicht unterstützt".into(),
        Outcome::Failed(OsError::Win32(code)) => format!("fehlgeschlagen (Win32 {code})"),
        Outcome::Failed(OsError::HResult(code)) => {
            format!("fehlgeschlagen (HRESULT 0x{:08X})", code as u32)
        }
    }
}

pub fn view(ui: &mut egui::Ui, report: Option<&HardeningReport>) {
    ui.heading(summary(report));
    if let Some(report) = report {
        for &(measure, result) in &report.measures {
            let name = match measure {
                Measure::WerHeapAndSnapshots => "WER-Heap und Snapshots",
                Measure::ProcessDacl => "Prozesszugriffsrechte (DACL)",
                Measure::ExtensionPoints => "Alte DLL-Erweiterungspunkte",
                Measure::ImageLoads => "DLL-Laderichtlinien",
            };
            ui.label(format!("{name}: {}", outcome(result)));
        }
    }
    ui.label("Die Maßnahmen begrenzen unterstützte Zugriffe. Administratoren mit Debugrechten, vorhandene Handles und manche Dump-Wege bleiben außerhalb des Schutzes.");
    ui.label("Der Editor nutzt einen vorab reservierten Zeroizing-Puffer. LockedSecret ist noch nicht integriert; Core, egui, Betriebssystem und Zielprogramme können weitere Kopien halten.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_empty_partial_and_failed_reports_are_visible() {
        assert_eq!(summary(None), "Prozesshärtung nicht gemeldet");
        for measures in [
            vec![],
            vec![(Measure::ImageLoads, Outcome::Unsupported)],
            vec![
                (Measure::ProcessDacl, Outcome::Applied),
                (Measure::ImageLoads, Outcome::Failed(OsError::Win32(5))),
            ],
        ] {
            assert_eq!(
                summary(Some(&HardeningReport { measures })),
                "Prozesshärtung unvollständig"
            );
        }
        assert_eq!(
            summary(Some(&HardeningReport {
                measures: vec![(Measure::ImageLoads, Outcome::Applied)]
            })),
            "Prozesshärtung angewendet"
        );
        assert!(outcome(Outcome::Failed(OsError::Win32(5))).contains("Win32 5"));
        assert!(outcome(Outcome::Failed(OsError::HResult(-2147467259))).contains("0x80004005"));
    }
}
