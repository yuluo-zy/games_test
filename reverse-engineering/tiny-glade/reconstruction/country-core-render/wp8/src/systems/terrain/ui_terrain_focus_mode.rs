//! ui_terrain_focus_mode.rs — 地形聚焦模式 UI 系统（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/ui_terrain_focus_mode.rs`。反编译 `ui_terrain_focus_mode`@0x1409a4fb0
//! （行 0；闭包行 94/175/261/324/441/481/531）。
//! ECS 签名（O，FunctionSystem 记录，行 41 initialize 宿主）：
//! ```
//! fn ui_terrain_focus_mode(
//!   Res<AppMode>, Res<UiState>, Res<UiNavState>, Res<ActionSetStates>,
//!   EventWriter<RequestAppModeChange>, EventWriter<ToolSelectionCancelled>,
//!   EventWriter<OutlineWallCmd>, Res<CameraCursorRay>, Res<RaycastWorld>,
//!   ResMut<PublicWalls>, Res<DecoratorStorage>, EventWriter<EvRightClickedAutoClutter>,
//!   Res<History>,
//!   (Res<HoveredTerrainStrokeId>, Res<CursorTerrainRaycast>),
//!   (EventWriter<NotifyHintSystemTerrain>, Local<TerrainFocusModeOrigin>))
//! ```
//! 进入/退出地形聚焦模式的 UI 状态机（I）；细节 U。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 聚焦模式原点 Local（O 名；`TerrainFocusModeOrigin`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TerrainFocusModeOrigin(pub Gap);

/// 地形聚焦模式 UI 系统（O：0x1409a4fb0；签名见文件头）。
pub fn ui_terrain_focus_mode(
    _mode: &Gap,
    _ui: &Gap,
    _nav: &Gap,
    _actions: &Gap,
    _mode_change: &mut Vec<Gap>,
    _tool_cancelled: &mut Vec<Gap>,
    _outline_wall: &mut Vec<Gap>,
    _camera_ray: &Gap,
    _raycast: &Gap,
    _walls: &mut Gap,
    _decor: &Gap,
    _autoclutter: &mut Vec<Gap>,
    _history: &Gap,
    _origin: &mut TerrainFocusModeOrigin,
) {
    unimplemented!("recover ui_terrain_focus_mode focus state machine")
}
