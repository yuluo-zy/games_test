//! B PhotomodeState constructor (0x1408705b0) and enable reset (0x140870570).
//! Offsets document native provenance; this type does not promise its Rust ABI.
use crate::resources::photo_mode::PhotoModeCameraMode;
use bevy_ecs::resource::Resource;

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PhotomodeState {
    /// B +0, initialized to 1 in demo and 3 in full game.
    pub scalar: f32,
    /// B +4; do_photomode writes the focus camera tag or u32::MAX.
    pub index: u32,
    /// B +8; meaning not yet established by a recovered consumer.
    pub flag_08: bool,
    /// B +9; preserved across enabling photomode.
    pub camera_mode: PhotoModeCameraMode,
    /// B +0xa; consumed by hide_ui.
    pub hide_ui: bool,
    /// B +0xb; match_free_camera_to_normal is called only while false.
    pub free_camera_initialized: bool,
    /// B +0xc; reset on each entry; init_walk_camera sets it true.
    pub walk_camera_initialized: bool,
    /// B +0xd; gates focus-marker drawing in do_photomode.
    pub show_focus_marker: bool,
}

impl Default for PhotomodeState {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PhotomodeState {
    /// B 0x1408705b0: +4..+0xb is 0x00000000ffffffff; +0xc..+0xd is 0x0100.
    pub fn new(demo: bool) -> Self {
        Self {
            scalar: if demo { 1.0 } else { 3.0 },
            index: u32::MAX,
            flag_08: false,
            camera_mode: PhotoModeCameraMode::TiltShift,
            hide_ui: false,
            free_camera_initialized: false,
            walk_camera_initialized: false,
            show_focus_marker: true,
        }
    }

    /// B 0x140870570. Camera selection, free-camera initialization and marker
    /// visibility survive; the walk camera is initialized again on next use.
    pub fn on_enable_photomode(&mut self, demo: bool) {
        self.scalar = if demo { 1.0 } else { 3.0 };
        self.index = u32::MAX;
        self.flag_08 = false;
        self.hide_ui = false;
        self.walk_camera_initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabling_resets_only_the_native_fields() {
        let mut state = PhotomodeState {
            scalar: 9.0,
            index: 3,
            flag_08: true,
            camera_mode: PhotoModeCameraMode::Walk,
            hide_ui: true,
            free_camera_initialized: true,
            walk_camera_initialized: true,
            show_focus_marker: false,
        };
        state.on_enable_photomode(false);
        assert_eq!(
            state,
            PhotomodeState {
                camera_mode: PhotoModeCameraMode::Walk,
                free_camera_initialized: true,
                show_focus_marker: false,
                ..PhotomodeState::new(false)
            }
        );
    }

    #[test]
    fn constructor_matches_demo_and_full_defaults() {
        assert_eq!(PhotomodeState::default().scalar, 3.0);
        assert_eq!(PhotomodeState::new(true).scalar, 1.0);
        assert_eq!(PhotomodeState::default().index, u32::MAX);
        assert!(PhotomodeState::default().show_focus_marker);
        let mut state = PhotomodeState::default();
        state.on_enable_photomode(true);
        assert_eq!(state.scalar, 1.0);
    }
}
