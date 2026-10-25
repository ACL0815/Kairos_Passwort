//! The application owns the destination captured before showing its editor.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp>.

use kairos_platform::{FocusTarget, PlatformError};

pub(crate) trait InputTarget {
    fn restore(&self) -> Result<(), PlatformError>;
    fn type_text(&self, text: &str, from_editor: bool) -> Result<(), PlatformError>;
}

impl InputTarget for FocusTarget {
    fn restore(&self) -> Result<(), PlatformError> {
        FocusTarget::restore(self)
    }

    fn type_text(&self, text: &str, from_editor: bool) -> Result<(), PlatformError> {
        if from_editor {
            self.restore_and_type_text(text)
        } else {
            FocusTarget::type_text(self, text)
        }
    }
}

pub(crate) fn boxed_target(
    target: Result<FocusTarget, PlatformError>,
) -> Result<Box<dyn InputTarget>, PlatformError> {
    target.and_then(|target| {
        if target.is_current_process() {
            Err(PlatformError::TargetUnavailable)
        } else {
            Ok(Box::new(target) as Box<dyn InputTarget>)
        }
    })
}

pub(crate) fn input_error(error: PlatformError) -> &'static str {
    match error {
        PlatformError::InputIncomplete => "Windows hat nur einen Teil der Eingabe angenommen. Zielfeld prüfen; nicht automatisch erneut einfügen.",
        PlatformError::TargetExpired => "Die Eingabeanforderung ist abgelaufen. Hotkey erneut auslösen.",
        PlatformError::ModifiersHeld => "Eingabe abgebrochen. Bitte alle Hotkey-Tasten loslassen und erneut auslösen.",
        PlatformError::TargetElevated => "Eingabe in eine Anwendung mit höheren Rechten wird abgelehnt.",
        PlatformError::TargetChanged => "Zielfenster oder Eingabefeld wurde gewechselt. Eingabe abgebrochen; Hotkey im gewünschten Feld erneut auslösen.",
        PlatformError::InvalidText => "Eingabe abgebrochen. Maximal 1024 UTF-16-Einheiten ohne Steuerzeichen sind erlaubt.",
        _ => "Zielfenster nicht verfügbar oder Fokus-Rückgabe von Windows verweigert. Bitte Hotkey im gewünschten Feld erneut auslösen.",
    }
}
