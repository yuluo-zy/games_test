//! `roof_tiles.rs` — RoofTile/RoofTiles 组件（B `resources/roofs/roof_tiles.rs`）。
//!
//! 瓦片实例的实际 CPU 生成（assemble 圆形/矩形、RoofShingle 64 字节 GPU 记录）
//! 已由共享 `system-roof` 的 tiles 模块恢复（11,467 片圆形瓦 0 差异，
//! `evidence/roof-tiles/`）。`clone_behavior`（0x140002370）= 跨 6+ 域共享的
//! `xor eax,eax; ret` 空桩（返回 None）。
//!
//! B `assemble_roof_tiles` 的系统参数面（skeleton 富化候选）：
//! `(EventReader<RecalculateRoofCmd>, Query<(&mut RoofTiles, &Roof)>, Res<AssetSsboLibrary>, …)`
//! —— RoofTiles 是每个屋顶实体持有的装配记录组件。
use bevy_ecs::component::Component;

/// 屋顶瓦片实例组件（B `RoofTile`）。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTile {
    /// 共享空桩不携带业务状态；瓦片记录本体在 GPU/装配链中。
    pub(crate) _marker: (),
}

/// 装配后的屋顶瓦片记录组件（B `RoofTiles`）。载荷为
/// `tiles::RoofTileRecord` 的 64 字节观测投影（RoofShingle），保持与
/// `pipeline::assemble_roof_tiles_observed` 输出一致。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTiles {
    pub records: Vec<[u8; 64]>,
}
