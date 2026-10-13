//! ui_draw_terrain.rs — 绘制地形 UI 系统（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/ui_draw_terrain.rs`。反编译 `ui_draw_terrain`@0x140a9fdc0（行 40）。
//! ECS 签名（O，FunctionSystem 记录）：
//! ```
//! fn ui_draw_terrain(
//!   Res<AppMode>, Res<CursorTerrainRaycast>, Res<AssetSsboLibrary>, Res<WaterRaster>,
//!   ResMut<PendingTerrainHistory>, Res<UiState>, ResMut<TerrainEditingData>,
//!   ResMut<DecoratorArchivist>, EventWriter<EvTerrainStrokeChanged>,
//!   EventWriter<NotifyHintSystemTerrain>, EventWriter<NotifyHintSystemToolWasJustUsed>,
//!   (Res<GladeBorder>, EventWriter<DisplayOutOfBorder>, EventWriter<FeedbackInputOutOfBorder>),
//!   Local<DrawState>, Local<CacheMouseRaycastHeight>, EventWriter<PlayTerrainSound>)
//! ```
//! 业务（I）：高亮/绘制模式下点按创建新笔画（`new_stroke`）并发 `EvTerrainStrokeChanged`；
//! 内部 UI 状态机细节 U。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::terrain_editing::TerrainEditingData;
use crate::resources::world_raster_defs::WaterRaster;
use crate::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged;
use crate::systems::terrain::{
    AppMode, DecoratorArchivist, GladeBorder, NotifyHintSystemTerrain,
    NotifyHintSystemToolWasJustUsed, PendingTerrainHistory, PlayTerrainSound, UiState,
};
use crate::typed_gap::Gap;

/// 缓存鼠标射线高度 Local（O 名；`CacheMouseRaycastHeight`）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CacheMouseRaycastHeight(pub Gap);

/// 绘制地形 UI 系统（O：0x140a9fdc0，行 40；签名见文件头）。
pub fn ui_draw_terrain(
    _mode: &AppMode,
    _cursor: &Gap,
    _ssbo_library: &Gap,
    _water: &WaterRaster,
    _pending: &mut PendingTerrainHistory,
    _ui: &UiState,
    _editing: &mut TerrainEditingData,
    _archivist: &mut DecoratorArchivist,
    _stroke_changed: &mut Vec<EvTerrainStrokeChanged>,
    _notify: &mut Vec<NotifyHintSystemTerrain>,
    _notify_tool: &mut Vec<NotifyHintSystemToolWasJustUsed>,
    _border: &GladeBorder,
    _draw_state: &mut Gap,
    _cache: &mut CacheMouseRaycastHeight,
    _sound: &mut Vec<PlayTerrainSound>,
) {
    unimplemented!("recover ui_draw_terrain draw-mode stroke creation")
}
