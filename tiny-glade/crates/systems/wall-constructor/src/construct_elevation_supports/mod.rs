//! 高程支撑构造（construct_elevation_supports/mod.rs）。
//!
//! 5 系统（fresh decomp 全落盘）：
//! - `overhang_subscribe_to_tabloids`（0x140786440）
//! - `detect_overhang_support_segments`（0x1407867d0）
//! - `construct_circle_supports`（0x140788160）
//! - `construct_rectangle_columns`（0x140788a80）
//! - `construct_rectangle_brackets`（0x1407896b0）
use bevy_ecs::prelude::*;

pub mod circle_bottom_layer;
pub mod circle_categorize;
pub mod circle_pillars;
pub mod circle_turret;
pub mod find_overlap_segments;
pub mod maintain_stone_column_colliders;
pub mod rectangle_bottom_layer;
pub mod rectangle_large_brackets;
pub mod rectangle_pillars;
pub mod rectangle_small_brackets;
pub mod util_cross_brick_pillar;
pub mod util_pillar_construction;

/// B `OverhangSupportDirtySubscriptions`（检测订阅资源）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct OverhangSupportDirtySubscriptions {
    pub dirty: hashbrown::HashMap<u64, u32>,
}

/// B `overhang_subscribe_to_tabloids`（0x140786440）：订阅悬挑支撑 tabloid。
/// **未恢复（壳登记）**：tabloid subscribe 接线待 render/宿主装配层（不称已恢复）。
pub fn overhang_subscribe_to_tabloids() {}

/// B `detect_overhang_support_segments`（0x1407867d0）：检测悬挑支撑段
/// （find_overhang_support_segments 0x141248620 43KB 链）。
pub fn detect_overhang_support_segments(
    mut subscriptions: ResMut<OverhangSupportDirtySubscriptions>,
    mut cache: ResMut<crate::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache>,
) {
    // 脏订阅 → 悬挑段查找（43KB 链的几何细节登记）。
    for (&wall, _dirty) in subscriptions.dirty.iter() {
        let _segments =
            crate::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments(
                &[],
                &mut cache,
                wall,
            );
    }
}

/// B `construct_circle_supports`（0x140788160）。
/// 候选参数面：`(EventReader<RecalculateCircleSupports>, Res<AssetSsboLibrary>,
///  Res<PublicWalls>, Res<UiSignifierStream>, Res<RaycastWorld>, …)`。
/// **未恢复（壳登记）**：机器码已反编译（re-analyst-C RE-ANALYSIS），
/// circle_categorize 判别链 + 环向支柱实现待补（不称已恢复）。
pub fn construct_circle_supports() {}

/// B `construct_rectangle_columns`（0x140788a80）。
/// **未恢复（壳登记）**：矩形柱构造（rectangle_pillars 族）实现待补
/// （re-analyst-C RE-ANALYSIS 已给契约，不称已恢复）。
pub fn construct_rectangle_columns() {}

/// B `construct_rectangle_brackets`（0x1407896b0）。
/// **未恢复（壳登记）**：矩形托架构造（rectangle_small/large_brackets 族）
/// 实现待补（不称已恢复）。
pub fn construct_rectangle_brackets() {}
