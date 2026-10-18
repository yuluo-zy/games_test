//! 围栏构造（construct_fences/mod.rs + process_segment.rs）。
//!
//! `construct_fences`（0x140765830）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, Res<PublicWalls>, Res<AssetSsboLibrary>)`。
//! hot caller 0x140765ddc 对 Pub296==1 调 emit_fences（building-fences
//! findings）。
//!
//! W-B1 接线：`construct_fences_cpu`（systems/wall/construction/fences.rs 冻结链：
//! split_fence_angles 40° 切段 → 每段 FenceSegmentInput → 64B 木板 +
//! 柱 + 种子累计）→ 输出资源写回。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::systems::wall::construction::fences::FenceConstructionInput;
use country_core::systems::wall::construction::fences::construct_fences_cpu;
use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use bevy_ecs::prelude::*;
use glam::Vec3;

pub mod process_segment;

/// B 0x140765830：墙样式为栅栏（Pub296==1）时按段发射围栏柱/板。
pub fn construct_fences(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };
        let projection = wall.building_projection();
        // Pub296：栅栏样式字节（0x296 偏移）。
        if projection.observed_296() != 1 {
            continue;
        }
        let curve = wall.simplified_curve_2d();
        let raw_positions: Vec<[f32; 2]> = curve.points.iter().map(|p| [p[0], p[1]]).collect();
        let prepared_positions: Vec<[f32; 3]> =
            curve.points.iter().map(|p| [p[0], p[1], 0.0]).collect();
        let input = FenceConstructionInput {
            wall: country_core::resources::walls::publication_fields::WallId(event.0 .0),
            raw_positions: &raw_positions,
            prepared_positions: &prepared_positions,
            segment_values: &curve.points.iter().map(|p| p[1]).collect::<Vec<f32>>(),
            observed_public_284: projection.float_0x284(),
            observed_public_296: projection.observed_296(),
        };
        if let Ok(outcome) = construct_fences_cpu(input) {
            // 64B 木板实例落盘。
            for plank in &outcome.instances {
                output.plank_instances.push(plank.observed_bytes());
            }
            let _ = (
                outcome.posts,
                outcome.source_ranges,
                outcome.segment_seeds,
                outcome.fence_segments_count,
                outcome.fence_segments_seed,
            );
        }
        let _ = ssbo;
    }
}

/// B `split_curve_based_on_angle`（0x141fdf9e0）：40 度角阈值切段
/// （= systems::wall::construction::fences::split_fence_angles）。
pub fn split_curve_based_on_angle(points: &[Vec3]) -> Vec<Vec<Vec3>> {
    let xz: Vec<[f32; 3]> = points.iter().map(|p| p.to_array()).collect();
    let mut segments = Vec::new();
    if let Ok(indices) = country_core::systems::wall::construction::fences::split_fence_angles(&xz) {
        let mut start = 0usize;
        for &end in &indices {
            if end > start {
                segments.push(points[start..end].to_vec());
            }
            start = end;
        }
        if start < points.len() {
            segments.push(points[start..].to_vec());
        }
    }
    segments
}
