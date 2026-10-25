//! terrain_hover_idx.rs — 悬停地形笔画索引（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/terrain_hover_idx.rs`。反编译 `terrain_hover_idx`@0x140aff720（行 17）。
//! ECS 签名（O）：
//! ```
//! fn terrain_hover_idx(
//!   Res<AppMode>, (Res<AssetSsboLibrary>, Res<WaterRaster>, Res<CursorTerrainRaycast>),
//!   ResMut<HoveredTerrainStrokeId>)
//! ```
//! 业务（O）：仅特定 AppMode（位掩码 0x421 = bits{0,5,10}）；读鼠标回读 SSBO（`FrozenMap::get`，
//! 断言尺寸 0x6c）→ 笔画 id（+0x10）；`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 采样水位；
//! 水位 ≤0 或光标无效或距离 ≤0.5 时不变；否则写 `HoveredTerrainStrokeId {tag=1, id}`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::world_raster_defs::WaterRaster;
use crate::resources::terrain_editing::TerrainStrokeId;
use crate::systems::terrain::AppMode;
use crate::typed_gap::Gap;

/// 悬停笔画 id 资源（O 名；`HoveredTerrainStrokeId`；形状 `{tag, id, 帧 tick}`，I）。
#[derive(Resource, Clone, Debug, Default)]
pub struct HoveredTerrainStrokeId {
    pub tag: u32,
    pub id: TerrainStrokeId,
}

/// 悬停笔画索引系统（O：`terrain_hover_idx`@0x140aff720，行 17；签名见文件头）。
pub fn terrain_hover_idx(
    _mode: &AppMode,
    _ssbo_library: &Gap,
    _water: &WaterRaster,
    _cursor: &CursorTerrainRaycast,
    _hovered: &mut HoveredTerrainStrokeId,
) {
    unimplemented!("recover terrain_hover_idx readback + water-height gating")
}
