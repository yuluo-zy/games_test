//! B GladeSettings: the eleven-key serde schema at 0x142aceb88 and native
//! default at 0x1408c2240. Padding and Rust enum niches are not settings.
use bevy_ecs::prelude::Resource;
use serde::Deserialize;
use std::path::Path;

/// Fields retain the original serde sequence order. Map/sequence omissions use
/// the same defaults; unknown map keys are ignored and duplicates are errors.
#[derive(Resource, Clone, Debug, PartialEq, Deserialize)]
#[serde(default)]
pub struct GladeSettings {
    /// Native +1c, consumed as a byte by ivy_leaf_spawner.
    pub ivy_leaves: bool,
    pub ivy_flowers: bool,
    /// Native +00; B's parser explicitly deserializes an unsigned 64-bit value.
    pub leaf_particle_count: u64,
    pub weather_fog: f32,
    pub rain: bool,
    pub butterflies: bool,
    pub wind_strength: f32,
    pub vegetation_around_buildings: bool,
    pub default_building_color_id: u32,
    pub default_freeform_wall_color_id: u32,
    pub default_roof_color_id: u32,
}

impl Default for GladeSettings {
    fn default() -> Self {
        Self {
            ivy_leaves: true,
            ivy_flowers: false,
            leaf_particle_count: 0,
            weather_fog: 0.01,
            rain: false,
            butterflies: false,
            wind_strength: 1.0,
            vegetation_around_buildings: true,
            default_building_color_id: 8,
            default_freeform_wall_color_id: 0,
            default_roof_color_id: 0,
        }
    }
}

impl GladeSettings {
    /// Native Path::join takes the selected glade directory, not a settings
    /// filename. Original read_to_string/serde errors and JSON null yield None.
    pub fn try_load(glade_directory: impl AsRef<Path>) -> Option<Self> {
        let json = std::fs::read_to_string(glade_directory.as_ref().join("settings.json")).ok()?;
        serde_json::from_str::<Option<Self>>(&json).ok().flatten()
    }

    /// B 0x1408bbbc0 falls back to the complete default on any failed load.
    pub fn load(glade_directory: impl AsRef<Path>) -> Self {
        Self::try_load(glade_directory).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glade_settings_original_schema_defaults_and_file_loading() {
        let defaults = GladeSettings::default();
        assert_eq!(defaults.weather_fog.to_bits(), 0x3c23_d70a);
        assert_eq!(defaults.wind_strength.to_bits(), 0x3f80_0000);
        assert!(defaults.ivy_leaves && defaults.vegetation_around_buildings);
        assert!(!defaults.ivy_flowers && !defaults.rain && !defaults.butterflies);
        assert_eq!(
            (
                defaults.leaf_particle_count,
                defaults.default_building_color_id,
                defaults.default_freeform_wall_color_id,
                defaults.default_roof_color_id
            ),
            (0, 8, 0, 0)
        );
        assert_eq!(
            serde_json::from_str::<GladeSettings>("{}").unwrap(),
            defaults
        );
        assert_eq!(
            serde_json::from_str::<GladeSettings>("[]").unwrap(),
            defaults
        );
        let all_fields = r#"{"ivy_leaves":false,"ivy_flowers":true,"leaf_particle_count":123,"weather_fog":0.25,"rain":true,"butterflies":true,"wind_strength":0.75,"vegetation_around_buildings":false,"default_building_color_id":2,"default_freeform_wall_color_id":3,"default_roof_color_id":4}"#;
        let actual: GladeSettings = serde_json::from_str(all_fields).unwrap();
        let sequential: GladeSettings =
            serde_json::from_str("[false,true,123,0.25,true,true,0.75,false,2,3,4]").unwrap();
        assert_eq!(actual, sequential);
        assert_eq!(actual.default_roof_color_id, 4);
        let partial: GladeSettings =
            serde_json::from_str(r#"{"ivy_leaves":false,"unknown":{"nested":[1,2]}}"#).unwrap();
        assert_eq!(
            partial,
            GladeSettings {
                ivy_leaves: false,
                ..defaults.clone()
            }
        );
        for invalid in [
            r#"{"ivy_leaves":null}"#,
            r#"{"ivy_leaves":1}"#,
            r#"{"ivy_leaves":true,"ivy_leaves":false}"#,
            r#"{"leaf_particle_count":-1}"#,
            r#"{"default_roof_color_id":4294967296}"#,
            "{} trailing",
        ] {
            assert!(serde_json::from_str::<GladeSettings>(invalid).is_err());
        }
        let directory = tempfile::tempdir().unwrap();
        assert_eq!(GladeSettings::try_load(directory.path()), None);
        assert_eq!(GladeSettings::load(directory.path()), defaults);
        let path = directory.path().join("settings.json");
        std::fs::write(&path, all_fields).unwrap();
        assert_eq!(GladeSettings::load(directory.path()), actual);
        for invalid in ["null", "{", r#"{"wind_strength":"bad"}"#] {
            std::fs::write(&path, invalid).unwrap();
            assert_eq!(GladeSettings::try_load(directory.path()), None);
            assert_eq!(GladeSettings::load(directory.path()), defaults);
        }
    }
}
