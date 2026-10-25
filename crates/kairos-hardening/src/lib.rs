//! Optional protection for short-lived secrets. Call before creating secrets.
//!
//! Modelled after: KeePassXC Bootstrap.cpp and Alloc.cpp
//! <https://github.com/keepassxreboot/keepassxc/tree/2.8.0/src/core>.
//! No process setting prevents an administrator or in-process code reading RAM.
//! Application integration is intentionally left to a later PR. See the README.

mod locked;
#[cfg(windows)]
mod windows;

pub use locked::LockedSecret;

/// A failed OS operation, with no secret data.
#[derive(Debug)]
pub enum HardeningError {
    InvalidCapacity,
    CapacityExceeded,
    Memory(region::Error),
}

impl std::fmt::Display for HardeningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCapacity => f.write_str("secret capacity must be 1..=65536 bytes"),
            Self::CapacityExceeded => f.write_str("secret exceeds fixed capacity"),
            Self::Memory(error) => write!(f, "memory protection failed: {error}"),
        }
    }
}

impl std::error::Error for HardeningError {}

/// A measure that was attempted, never an assurance of complete protection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Measure {
    WerHeapAndSnapshots,
    ProcessDacl,
    ExtensionPoints,
    ImageLoads,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Applied,
    /// Win32 error or HRESULT, retaining its original bits.
    Failed(u32),
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "inspect every hardening outcome before processing secrets"]
pub struct HardeningReport {
    pub measures: Vec<(Measure, Outcome)>,
}

impl HardeningReport {
    pub fn all_applied(&self) -> bool {
        !self.measures.is_empty() && self.measures.iter().all(|(_, s)| *s == Outcome::Applied)
    }
}

/// Applies process-local settings, reporting every failure instead of panicking.
///
/// Modelled after: KeePassXC Bootstrap.cpp
/// <https://github.com/keepassxreboot/keepassxc/blob/2.8.0/src/core/Bootstrap.cpp>.
/// Call once, early, on the application startup thread. Some settings cannot be
/// undone. Existing handles, elevated debuggers and WER LocalDumps remain outside
/// this protection. The process DACL can affect diagnostics and integrations.
pub fn harden_process() -> HardeningReport {
    #[cfg(windows)]
    {
        windows::harden()
    }
    #[cfg(not(windows))]
    {
        HardeningReport {
            measures: [
                Measure::WerHeapAndSnapshots,
                Measure::ProcessDacl,
                Measure::ExtensionPoints,
                Measure::ImageLoads,
            ]
            .map(|m| (m, Outcome::Unsupported))
            .to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_does_not_hide_partial_failure() {
        let mut report = HardeningReport { measures: vec![] };
        assert!(!report.all_applied());
        report
            .measures
            .push((Measure::ProcessDacl, Outcome::Applied));
        assert!(report.all_applied());
        for outcome in [Outcome::Failed(5), Outcome::Unsupported] {
            report.measures.push((Measure::ImageLoads, outcome));
            assert!(!report.all_applied());
            report.measures.pop();
        }
    }
}
