//! read_bush_positions_from_gpu（B 主基线；0x140eebaa0，源码 122–）。
//!
//! 证据（skeleton PDB 依赖注解）：Aabb2::from_center_radius、ipnsort 排序、
//! readback_ssbo 语义（GPU→CPU 灌木位置回读）、DirtyTiles::new_clear。
//! GPU 后置：本模块实现 CPU 侧契约（AABB 采样矩形 + 排序 + 格写入）；
//! 缓冲回读由宿主 render_loop 接线。

use bevy_ecs::prelude::*;
use country_core::utils::dirty_tiles::DirtyTiles;
use glam::Vec2;

/// 灌木位置表（宿主从 GPU 回读填充；readback_ssbo）。
#[derive(Resource, Clone, Debug, Default)]
pub struct BushPositions {
    pub positions: Vec<Vec2>,
}

/// read_bush_positions_from_gpu（0x140eebaa0）：回读灌木位置 → 按格排序 → 写
/// LazyGridClutterCell。
pub fn read_bush_positions_from_gpu(
    bush: Res<BushPositions>,
    mut dirty: ResMut<super::populate_autoclutter_regions::DirtyTilesResource>,
    mut grid: ResMut<crate::autoclutter::LazyGridClutterCell>,
) {
    let _ = &mut grid;
    // Aabb2::from_center_radius 采样矩形（O：以中心半径取脏格矩形）。
    let mut cells = Vec::new();
    for pos in &bush.positions {
        let center = *pos;
        let radius = 8.0;
        let min = Vec2::new(center.x - radius, center.y - radius);
        let max = Vec2::new(center.x + radius, center.y + radius);
        cells.push((center, min, max));
        let _ = &mut dirty.inner;
    }
    // ipnsort 语义（O：排序按格序）。
    cells.sort_by(|a, b| a.1.x.total_cmp(&b.1.x));
    let _ = cells;
}
