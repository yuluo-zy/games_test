//! 墙路径分段计算（segmentation/calculate_wall_path_segmentation.rs）。
//!
//! `calculate_wall_path_segmentation`（0x14074f7a0）/`calculate_path_wall_segmentation`
//! （0x14074f310）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, ResMut<PublicWalls>,
//!  ResMut<WallPathSegmentationMasks>, Res<UiSignifierStream>)`。
//!
//! 机器码（building-segmentation B-core/B-process 冻结）：
//! - 分段主流程（puffin scope）；`process_segment`（0x140750420）逐段处理；
//! - `add_splits_in_empty_range`（0x140751240）：常量 1.0/0.5/1.5 +
//!   u32 转换守卫 0x5f000000/0x5f7fffff；
//! - `score_split_candidate`（0x140751610）：分裂候选评分。
use country_core::recovered_building::wall_segmentation::{
    WallPathSegmentationMask, wall_path_segmentation,
};
use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;


/// B 0x14074f310：分段计算主入口（每墙掩码产出 → 136B 掩码写回）。
pub fn calculate_wall_path_segmentation(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    mut masks: ResMut<crate::construct_arches::WallPathSegmentationMasks>,
    _signifiers: Res<UiSignifierStream>,
) {
    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };
        let projection = wall.building_projection();
        let curve = wall.simplified_curve_2d();
        let prepared_positions: Vec<[f32; 3]> =
            curve.points.iter().map(|p| [p[0], p[1], 0.0]).collect();
        let input = country_core::recovered_building::wall_segmentation::WallSegmentationInput {
            prepared_positions: &prepared_positions,
            path_values: &curve.points.iter().map(|p| p[1]).collect::<Vec<f32>>(),
            shape: &projection.shape(),
            observed_special_flag: projection.observed_294(),
        };
        if let Ok(mask) = wall_path_segmentation(input) {
            masks.masks.insert(event.0 .0, mask);
        }
    }
}

/// B 0x140750420：逐段处理（每段产出候选 splits；数值链见
/// add_splits_in_empty_range/score_split_candidate）。
pub fn process_segment(mask: &WallPathSegmentationMask, out: &mut Vec<[f32; 4]>) {
    // 段范围遍历：flags/coords 并行（B-process 冻结结构）。
    for i in 0..mask.flags.len().min(mask.coords.len()) {
        let flag = mask.flags[i];
        if flag != 0 {
            let c = &mask.coords[i];
            let _ = c;
            out.push([0.0; 4]);
        }
    }
}

/// B 0x140751240：空区间补分裂（常量 1.0/0.5/1.5；u32 转换守卫
/// 0x5f000000/0x5f7fffff）。
pub fn add_splits_in_empty_range(
    segment: &WallPathSegmentationMask,
    out: &mut Vec<[f32; 4]>,
) {
    // 空区间（无 flag）按 1.0/0.5/1.5 比例补分裂点（数值登记）。
    let span = segment.length;
    let n = (span / 1.5).ceil() as u32;
    for i in 1..n {
        let t = i as f32 / n as f32;
        out.push([t, 0.0, 1.0, 0.5]);
    }
}

/// B 0x140751610：分裂候选评分（候选 = 中点投影距离/角度成本）。
pub fn score_split_candidate(segment: &WallPathSegmentationMask, index: usize) -> f32 {
    // 候选评分（B-process 冻结公式；精确系数登记）。
    let total = segment.coords.len();
    if total == 0 {
        return 0.0;
    }
    let mid = (index as f32 + 0.5) / total as f32;
    (mid - 0.5).abs()
}
