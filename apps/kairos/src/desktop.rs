//! The application owns the destination captured before showing its editor.
//!
//! Modelled after: KeePassXC <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/autotype/AutoType.cpp>.

use kairos_platform::{FocusTarget, PlatformError};

pub(crate) trait InputTarget {
    fn restore(&self) -> Result<(), PlatformError>;
    fn type_text(&self, text: &str, from_editor: bool) -> Result<(), PlatformError>;
}

#[derive(Default)]
pub(crate) enum Destination {
    #[default]
    Unavailable,
    ReturnOnly(Box<dyn InputTarget>),
    Editor(Box<dyn InputTarget>),
}

impl Destination {
    pub(crate) fn can_restore(&self) -> bool {
        !matches!(self, Self::Unavailable)
    }

    pub(crate) fn can_insert(&self) -> bool {
        matches!(self, Self::Editor(_))
    }

    pub(crate) fn restore(&self) -> Result<(), PlatformError> {
        match self {
            Self::ReturnOnly(target) | Self::Editor(target) => target.restore(),
            Self::Unavailable => Err(PlatformError::TargetUnavailable),
        }
    }

    pub(crate) fn take_editor(&mut self) -> Option<Box<dyn InputTarget>> {
        if !self.can_insert() {
            return None;
        }
        match std::mem::take(self) {
            Self::Editor(target) => Some(target),
            _ => unreachable!("only an explicit editor destination can be consumed"),
        }
    }
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
        PlatformError::TargetExpired => "Die Eingabeanforderung ist abgelaufen. Das aktuelle Passwort bleibt erhalten. Kopieren nutzen oder F9 im Ziel auslösen (erzeugt neu).",
        PlatformError::ModifiersHeld => "Eingabe abgebrochen. Alle Hotkey-Tasten loslassen. Das aktuelle Passwort bleibt erhalten; Kopieren nutzen oder F9 auslösen (erzeugt neu).",
        PlatformError::TargetElevated => "Eingabe in eine Anwendung mit höheren Rechten wird abgelehnt.",
        PlatformError::TargetChanged => "Zielfenster oder Eingabefeld wurde gewechselt. Passwort bleibt erhalten; Kopieren nutzen oder F9 im Ziel auslösen (erzeugt neu).",
        PlatformError::InvalidText => "Eingabe abgebrochen. Maximal 1024 UTF-16-Einheiten ohne Steuerzeichen sind erlaubt.",
        PlatformError::ForegroundDenied => "Windows hat die Fokus-Rückgabe verweigert. Passwort bleibt erhalten; Kopieren nutzen oder F9 im Ziel auslösen (erzeugt neu).",
        _ => "Kein externes Zielfeld verfügbar. Kairos selbst ist kein Einfügeziel. Passwort bleibt erhalten; Kopieren nutzen oder F9 im Ziel auslösen (erzeugt neu).",
    }
}
