//! Session grading configuration (B `load_glade_grading` @0x1408bf9c0).
//!
//! The original writes `ImageFormationConfigs`, not a GPU `GradingLut`.
//! PDB system parameters and the three 0x40-byte writes agree. It reads the
//! theme's `grading.json`, falls back to default grading on an I/O error,
//! and unwraps malformed JSON. Halloween2024 uses the fixed cinema grade.
//! A saved image override applies only to slots 1 and 2; slot 0 remains the
//! glade default. LUT image allocation belongs to `GradingLuts::load`.
use std::path::{Path, PathBuf};

use bevy_ecs::prelude::{Res, ResMut};
use serde::Deserialize;

use crate::resources::{
    image_formation::{ImageFormationConfig, ImageFormationConfigs},
    session::{ActiveSession, SessionMeta},
};

pub const GRADING_JSON: &str = "grading.json";
pub const HALLOWEEN_GRADING: &str = "RoastMeat_Cinema_709_1";

/// Native ColorGradingState JSON fields, confirmed by shipped glade files
/// and serializer @0x140af00e0. Lens/artistic settings are not in this file.
#[derive(Deserialize)]
struct GladeGrading {
    bypass_color: bool,
    current_grading_lut: String,
    grading_strength: f32,
    ev_shift: f32,
    contrast: f32,
    saturation: f32,
}

fn from_grading(grading: GladeGrading) -> ImageFormationConfig {
    ImageFormationConfig {
        name: grading.current_grading_lut,
        params_18: [
            grading.grading_strength,
            grading.ev_shift,
            grading.contrast,
            grading.saturation,
        ],
        flag_28: u8::from(grading.bypass_color),
        // @1408bf9c0 clears the entire camera-lens/artistic tail of all slots.
        params_30: [0.0; 2],
        word_38: 0,
        flag_3c: 0,
        flag_3d: 0,
        ..ImageFormationConfig::default_plain()
    }
}

/// `assets_root` is already `assets/glade/<theme>` (SessionMeta::assets_root).
pub fn grading_json_path(assets_root: impl AsRef<Path>) -> PathBuf {
    assets_root.as_ref().join(GRADING_JSON)
}

/// Apply native glade defaults and saved-session override, preserving the
/// selected config kind. Read failures use default; parse errors are fatal.
pub fn load_session_grading(
    configs: &mut ImageFormationConfigs,
    meta: &SessionMeta,
    assets_root: impl AsRef<Path>,
) {
    let grading = if meta.is_halloween2024() {
        // .rdata @142aa0190 = [0.75, 0, 1, 1.06]; the native mutator scan
        // compares tag zero (Halloween2024), not all Halloween mutators.
        GladeGrading {
            bypass_color: false,
            current_grading_lut: HALLOWEEN_GRADING.to_owned(),
            grading_strength: 0.75,
            ev_shift: 0.0,
            contrast: 1.0,
            saturation: 1.06,
        }
    } else {
        match std::fs::read_to_string(grading_json_path(assets_root)) {
            Ok(text) => serde_json::from_str(&text).expect("deserialize glade grading"),
            Err(_) => GladeGrading {
                bypass_color: false,
                current_grading_lut: "default".to_owned(),
                grading_strength: 1.0,
                ev_shift: 0.0,
                contrast: 1.0,
                saturation: 1.0,
            },
        }
    };
    let config = from_grading(grading);
    configs.configs = [config.clone(), config.clone(), config];
    if let Some(saved) = &meta.image {
        // Native iter_non_default_mut skips index 0 in the final loop.
        configs.configs[1] = saved.clone();
        configs.configs[2] = saved.clone();
    }
}

pub fn load_glade_grading(mut configs: ResMut<ImageFormationConfigs>, session: Res<ActiveSession>) {
    load_session_grading(&mut configs, &session.meta, session.assets_root());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::session::SessionMutator;

    #[test]
    fn glade_defaults_and_saved_image_have_distinct_slots() {
        let root = (std::env::var_os("TG_INSTALL_ROOT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade"))).join("assets/glade/summer");
        let mut configs = ImageFormationConfigs::default();
        configs.kind = 2;
        let saved = ImageFormationConfig {
            name: "cold".to_owned(),
            ..ImageFormationConfig::default()
        };
        let meta = SessionMeta {
            image: Some(saved.clone()),
            ..SessionMeta::default()
        };
        load_session_grading(&mut configs, &meta, root);
        assert_eq!(configs.configs[0].name, "lush");
        assert_eq!(configs.configs[0].params_18, [0.4, 0.3, 1.05, 1.02]);
        assert_eq!(configs.configs[0].flag_3d, 0);
        assert_eq!(configs.configs[1], saved);
        assert_eq!(configs.configs[2], saved);
        assert_eq!(configs.kind, 2);
    }

    #[test]
    fn halloween2024_uses_native_constants_and_read_errors_fall_back() {
        let mut configs = ImageFormationConfigs::default();
        let mut meta = SessionMeta::default();
        meta.mutators.push(SessionMutator::Halloween2024);
        load_session_grading(&mut configs, &meta, "absent/glade");
        assert_eq!(configs.configs[0].name, HALLOWEEN_GRADING);
        assert_eq!(configs.configs[0].params_18, [0.75, 0.0, 1.0, 1.06]);
        meta.mutators[0] = SessionMutator::Halloween2025;
        load_session_grading(&mut configs, &meta, "absent/glade");
        assert_eq!(configs.configs[0].name, "default");
        assert_eq!(configs.configs[0].params_18, [1.0, 0.0, 1.0, 1.0]);
    }
}
