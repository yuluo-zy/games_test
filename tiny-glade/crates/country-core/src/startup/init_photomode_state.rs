//! 拍照模式状态初始化与进入时的字段重置。未知字段不假定业务含义。
use crate::resources::photo_mode::PhotoModeCameraMode;
use bevy_ecs::resource::Resource;

/// Original one-byte screenshot-message visibility owner, read by toolbar
/// `hide_ui`. The application constructor passes zero to its resource insert
/// at B 0x14059f3d7..3d9; the insert stores that byte at 0x14011d557.
/// This is application state, separate from each session's PhotomodeState reset.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SteamdeckScreenshotMsgScreen(pub bool);

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PhotomodeState {
    /// 拍照标量；demo 初值为 1，完整版为 3。
    pub scalar: f32,
    /// 焦点相机标签；u32::MAX 表示未选择。
    pub index: u32,
    /// 尚未确认含义的状态位；只保留已知重置行为。
    pub flag_08: bool,
    /// 相机模式；再次进入拍照模式时保留。
    pub camera_mode: PhotoModeCameraMode,
    /// 隐藏界面开关。
    pub hide_ui: bool,
    /// 自由相机是否已从普通相机初始化。
    pub free_camera_initialized: bool,
    /// 漫游相机是否初始化；每次进入拍照模式时重新初始化。
    pub walk_camera_initialized: bool,
    /// 是否显示焦点标记。
    pub show_focus_marker: bool,
}

/// Original OnEnter(Game) init_photomode_state, inlined in run_unsafe1408397a0.
/// This is a session reset, distinct from on_enable_photomode's selective reset.
/// Original demo is process-global; the host supplies the same immutable value.
pub fn init_photomode_state(
    mut state: bevy_ecs::prelude::ResMut<PhotomodeState>,
    config: bevy_ecs::prelude::Res<crate::game_config::GameConfig>,
) {
    *state = PhotomodeState::new(config.dev.demo);
}

impl Default for PhotomodeState {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PhotomodeState {
    /// 构造初始状态，demo 只影响拍照标量。
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

    /// 重置本次拍照状态，保留相机模式、自由相机初始化状态和标记开关。
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
