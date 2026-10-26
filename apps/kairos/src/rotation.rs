//! Validated profile selection and small, password-free local preferences.

use std::{ffi::OsString, io, path::PathBuf};

use kairos_core::{GenerateError, ProfileId, ProfileRotation, RotatedPassword, WordSource};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection(Vec<u8>);

impl Default for Selection {
    fn default() -> Self {
        Self((0..=12).collect())
    }
}

impl Selection {
    pub fn new(mut ids: Vec<u8>) -> Result<Self, &'static str> {
        ids.sort_unstable();
        if !(2..=13).contains(&ids.len())
            || ids.iter().any(|&id| id > 12)
            || ids.windows(2).any(|pair| pair[0] == pair[1])
        {
            return Err("Mindestens zwei unterschiedliche Profile von 0 bis 12 wählen.");
        }
        Ok(Self(ids))
    }

    pub fn ids(&self) -> &[u8] {
        &self.0
    }

    pub fn set(&mut self, id: u8, selected: bool) -> Result<(), &'static str> {
        let mut ids = self.0.clone();
        ids.retain(|&value| value != id);
        if selected {
            ids.push(id);
        }
        *self = Self::new(ids)?;
        Ok(())
    }

    pub fn core_rotation(&self) -> ProfileRotation {
        ProfileRotation::new(
            self.0
                .iter()
                .map(|&id| ProfileId::new(id).expect("validated selection").into())
                .collect(),
        )
        .expect("validated selection length")
    }

    pub fn concept_minimum(&self) -> f64 {
        self.0
            .iter()
            .map(|&id| crate::strength::CONCEPT_BITS[id as usize])
            .fold(f64::INFINITY, f64::min)
    }
}

/// The core validates every selected profile/source pair and draws uniformly.
pub fn generate(
    selection: &Selection,
    german: &dyn WordSource,
    expanded: &dyn WordSource,
    german_only: bool,
    excluded: &[char],
) -> Result<RotatedPassword, GenerateError> {
    let sources: Vec<&dyn WordSource> = selection
        .ids()
        .iter()
        .map(|&id| {
            if crate::source::uses_german(ProfileId::new(id).expect("validated"), german_only) {
                german
            } else {
                expanded
            }
        })
        .collect();
    kairos_core::generate_rotating_from_sources(&sources, &selection.core_rotation(), excluded)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Preferences {
    version: u8,
    rotate_profiles: bool,
    rotation_profiles: Vec<u8>,
}

#[derive(Default)]
pub struct SettingsFile {
    pub path: Option<PathBuf>,
    pub notice: Option<&'static str>,
}

impl SettingsFile {
    pub fn configured() -> Self {
        Self {
            path: configured_path(
                std::env::var_os("KAIROS_SETTINGS_FILE"),
                std::env::var_os("APPDATA"),
            ),
            notice: None,
        }
    }

    /// Invalid data leaves rotation disabled and the default selection intact.
    pub fn load(&mut self) -> (bool, Selection) {
        let loaded = (|| -> io::Result<(bool, Selection)> {
            use std::io::Read;
            let path = self
                .path
                .as_ref()
                .ok_or_else(|| io::Error::other("no path"))?;
            let mut bytes = Vec::new();
            std::fs::File::open(path)?
                .take(4097)
                .read_to_end(&mut bytes)?;
            if bytes.len() > 4096 {
                return Err(io::Error::other("settings too large"));
            }
            let data: Preferences = serde_json::from_slice(&bytes)?;
            if data.version != 1 {
                return Err(io::Error::other("unsupported settings version"));
            }
            let selection = Selection::new(data.rotation_profiles).map_err(io::Error::other)?;
            Ok((data.rotate_profiles, selection))
        })();
        match loaded {
            Ok(preferences) => {
                self.notice = None;
                preferences
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => (false, Selection::default()),
            Err(_) => {
                self.notice = Some("Rotationseinstellungen nicht lesbar oder ungültig. Rotation ist aus; neue Auswahl wird erst bei einer bewussten Änderung gespeichert.");
                (false, Selection::default())
            }
        }
    }

    pub fn save(&mut self, enabled: bool, selection: &Selection) {
        self.notice = if self.write(enabled, selection).is_ok() {
            None
        } else {
            Some("Rotationsauswahl gilt für diesen Start. Speichern fehlgeschlagen; Einstellungsdatei und Schreibrechte prüfen.")
        };
    }

    fn write(&self, enabled: bool, selection: &Selection) -> io::Result<()> {
        use std::io::Write;
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| io::Error::other("no path"))?;
        let parent = path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or(std::path::Path::new("."));
        std::fs::create_dir_all(parent)?;
        let mut file = tempfile::NamedTempFile::new_in(parent)?;
        serde_json::to_writer_pretty(
            &mut file,
            &Preferences {
                version: 1,
                rotate_profiles: enabled,
                rotation_profiles: selection.ids().to_vec(),
            },
        )?;
        file.write_all(b"\n")?;
        file.as_file().sync_all()?;
        file.persist(path).map_err(|error| error.error)?;
        Ok(())
    }
}

fn configured_path(override_path: Option<OsString>, appdata: Option<OsString>) -> Option<PathBuf> {
    override_path
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            appdata
                .filter(|path| !path.is_empty())
                .map(|root| PathBuf::from(root).join("Kairos/settings.json"))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_rejects_singletons_duplicates_and_invalid_profiles_without_mutating() {
        for ids in [vec![], vec![4], vec![4, 4], vec![0, 13]] {
            assert!(Selection::new(ids).is_err());
        }
        let mut selection = Selection::new(vec![12, 4]).unwrap();
        assert_eq!(selection.ids(), [4, 12]);
        assert!(selection.set(4, false).is_err());
        assert_eq!(selection.ids(), [4, 12]);
        selection.set(8, true).unwrap();
        selection.set(4, false).unwrap();
        assert_eq!(selection.ids(), [8, 12]);
    }

    #[test]
    fn four_profile_selection_roundtrips_only_preferences_and_can_be_replaced() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("Kairos/settings.json");
        let mut file = SettingsFile {
            path: Some(path.clone()),
            notice: None,
        };
        assert_eq!(file.load(), (false, Selection::default()));
        let selection = Selection::new(vec![2, 8, 10, 12]).unwrap();
        file.save(true, &selection);
        assert!(file.notice.is_none());
        assert_eq!(file.load(), (true, selection.clone()));
        let data: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(data.as_object().unwrap().len(), 3);
        assert_eq!(data["rotation_profiles"], serde_json::json!([2, 8, 10, 12]));
        file.save(false, &selection);
        assert_eq!(file.load(), (false, selection));
    }

    #[test]
    fn invalid_or_unreadable_settings_disable_rotation_without_overwriting() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("settings.json");
        let mut file = SettingsFile {
            path: Some(path.clone()),
            notice: None,
        };
        for text in [
            r#"{"version":1,"rotate_profiles":true,"rotation_profiles":[4]}"#,
            r#"{"version":2,"rotate_profiles":true,"rotation_profiles":[2,4]}"#,
            r#"{"version":1,"rotate_profiles":true,"rotation_profiles":[2,4],"secret":"x"}"#,
            "invalid json",
        ] {
            std::fs::write(&path, text).unwrap();
            assert_eq!(file.load(), (false, Selection::default()));
            assert!(file.notice.is_some());
            assert_eq!(std::fs::read_to_string(&path).unwrap(), text);
        }
        std::fs::write(&path, vec![b' '; 4097]).unwrap();
        assert!(!file.load().0);
        assert!(file.notice.is_some());
        file.path = Some(root.path().to_owned());
        assert!(!file.load().0);
        file.save(true, &Selection::default());
        assert!(file.notice.is_some());
    }

    #[test]
    fn test_path_overrides_appdata_without_global_environment_mutation() {
        assert_eq!(
            configured_path(Some("custom.json".into()), Some("root".into())),
            Some("custom.json".into())
        );
        assert_eq!(
            configured_path(Some("".into()), Some("root".into())),
            Some(PathBuf::from("root/Kairos/settings.json"))
        );
        assert_eq!(configured_path(None, None), None);
    }

    struct OneWord(&'static str);

    impl WordSource for OneWord {
        fn len(&self) -> usize {
            1
        }
        fn word(&self, _: usize) -> &str {
            self.0
        }
        fn language(&self, _: usize) -> kairos_core::Language {
            kairos_core::Language::German
        }
    }

    #[test]
    fn four_selected_profiles_draw_only_their_patterns_with_profile_specific_sources() {
        let selection = Selection::new(vec![0, 9, 10, 12]).unwrap();
        assert_eq!(
            selection.core_rotation().profiles(),
            &[
                ProfileId::new(0).unwrap().into(),
                ProfileId::new(9).unwrap().into(),
                ProfileId::new(10).unwrap().into(),
                ProfileId::new(12).unwrap().into()
            ]
        );
        let german = OneWord("ahorn");
        let expanded = OneWord("cloud");
        for _ in 0..64 {
            let result = generate(&selection, &german, &expanded, false, &[]).unwrap();
            let words: Vec<_> = result
                .password
                .secret
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect();
            let plain: Vec<_> = words
                .iter()
                .map(|word| {
                    word.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<String>()
                        .to_lowercase()
                })
                .collect();
            let separator_count = result
                .password
                .secret
                .chars()
                .filter(|c| !c.is_alphanumeric())
                .count();
            match (words.len(), separator_count) {
                (4, 3) => {
                    assert_eq!(result.password.secret.as_str(), "ahorn-ahorn-ahorn-ahorn");
                    assert!(plain.iter().all(|word| word == "ahorn"));
                }
                (5, 4) | (6, 5) => {
                    assert!(plain.iter().all(|word| word == "cloud"));
                    assert!(words.iter().all(|word| word
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .skip(1)
                        .all(char::is_lowercase)));
                }
                (6, 10) => {
                    assert!(plain.iter().all(|word| word == "cloud"));
                    assert!(words.iter().all(|word| word
                        .chars()
                        .filter(|c| c.is_uppercase())
                        .count()
                        == 1));
                }
                _ => panic!("pattern outside selection"),
            }
            assert_eq!(result.rotation_bonus_bits, 0.0);
            assert_eq!(result.password.entropy_bits, 0.0);
        }
        for _ in 0..8 {
            let result = generate(&selection, &german, &expanded, true, &[]).unwrap();
            let letters = result
                .password
                .secret
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            assert!(letters.replace("ahorn", "").is_empty());
        }
    }

    #[test]
    fn only_selected_profiles_are_validated_and_their_core_bound_is_preserved() {
        let source = OneWord("ahorn");
        let selection = Selection::new(vec![9, 12]).unwrap();
        let excluded = ['-'];
        let minimum = selection
            .ids()
            .iter()
            .map(|&id| {
                kairos_core::entropy_bits_for(
                    &source,
                    &kairos_core::GeneratorConfig {
                        profile: ProfileId::new(id).unwrap(),
                        excluded_chars: excluded.to_vec(),
                    },
                )
                .unwrap()
            })
            .fold(f64::INFINITY, f64::min);
        for _ in 0..16 {
            let result = generate(&selection, &source, &source, false, &excluded).unwrap();
            let count = result
                .password
                .secret
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .count();
            let separators = result
                .password
                .secret
                .chars()
                .filter(|c| !c.is_alphanumeric())
                .count();
            assert!([(5, 4), (6, 10)].contains(&(count, separators)));
            assert_eq!(result.password.entropy_bits, minimum);
        }
        assert!(generate(
            &Selection::new(vec![0, 9]).unwrap(),
            &source,
            &source,
            false,
            &excluded
        )
        .is_err());
    }
}
