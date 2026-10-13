//! ui_edit_terrain.rs — 编辑地形 UI 系统（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/ui_edit_terrain.rs`。反编译 `ui_edit_terrain`@0x140aef1f0（行 34）。
//! ECS 签名（O，FunctionSystem 记录）：
//! ```
//! fn ui_edit_terrain(
//!   Res<AppMode>, Res<Modifiers>, Res<TerrainEditingData>, Res<UiState>,
//!   ResMut<TerrainStrokeParamsCache>, ResMut<PendingTerrainHistory>, ResMut<DecoratorArchivist>,
//!   EventWriter<EditTerrainStrokeCmd>, EventWriter<CursorCaptureCmd>,
//!   EventWriter<EnableArrowGizmoStretch>, EventWriter<AddTerrainHighlightCmd>,
//!   (EventWriter<NotifyHintSystemTerrain>, EventWriter<NotifyHintSystemTerrainSteepness>,
//!    Local<TerrainSteepnessEditAmount>),
//!   EventReader<MainCameraMoved>,
//!   (EventReader<CursorMovedScreenSpace>, Res<CursorTerrainRaycast>),
//!   Res<HoveredTerrainStrokeId>)
//! ```
//! 资源：`MouseActivityTimer::new`@行 211、`TerrainStrokeParamsCache::default`@行 22、
//! `TerrainSteepnessEditAmount::default`（O）。
//! 业务（I）：根据鼠标落点 + hover 笔画 id 生成 `EditTerrainStrokeCmd`/`SetTerrainStrokePtsCmd`；
//! UI 状态机细节 U。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::terrain_editing::TerrainEditingData;
use crate::systems::terrain::edit_terrain_stroke::{
    EditTerrainStrokeCmd, SetTerrainStrokePtsCmd,
};
use crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId;
use crate::systems::terrain::{
    AppMode, CursorCaptureCmd, CursorMovedScreenSpace, DecoratorArchivist, EnableArrowGizmoStretch,
    MainCameraMoved, Modifiers, NotifyHintSystemTerrain, NotifyHintSystemTerrainSteepness,
    PendingTerrainHistory, UiState,
};
use crate::typed_gap::Gap;

/// 鼠标活动计时器（O 名；`MouseActivityTimer::new`@行 211）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct MouseActivityTimer(pub Gap);

/// 笔画参数缓存（O 名；`TerrainStrokeParamsCache::default`@行 22）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainStrokeParamsCache(pub Gap);

/// 陡度编辑量 Local（O 名；`TerrainSteepnessEditAmount`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TerrainSteepnessEditAmount(pub Gap);

/// 编辑地形 UI 系统（O：0x140aef1f0，行 34；签名见文件头）。
pub fn ui_edit_terrain(
    _mode: &AppMode,
    _modifiers: &Modifiers,
    _editing: &TerrainEditingData,
    _ui: &UiState,
    _params_cache: &mut TerrainStrokeParamsCache,
    _pending: &mut PendingTerrainHistory,
    _archivist: &mut DecoratorArchivist,
    _edit_cmd: &mut Vec<EditTerrainStrokeCmd>,
    _capture: &mut Vec<CursorCaptureCmd>,
    _gizmo: &mut Vec<EnableArrowGizmoStretch>,
    _highlight: &mut Vec<Gap>,
    _steepness: &mut TerrainSteepnessEditAmount,
    _camera_moved: &Gap,
    _cursor_moved: &Gap,
    _cursor: &Gap,
    _hovered: &HoveredTerrainStrokeId,
) {
    unimplemented!("recover ui_edit_terrain edit-command generation")
}
