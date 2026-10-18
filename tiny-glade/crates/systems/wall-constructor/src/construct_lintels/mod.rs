//! 檐梁构造（construct_lintels/mod.rs + place_support_beams.rs）。
//!
//! `construct_lintels`（0x14076d9f0）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, EventReader<CreateRoofCmd>,
//!  ResMut<PublicWalls>, ResMut<WallPathSegmentationMasksMinusStairs>,
//!  Res<AssetSsboLibrary>, …)`。
//!
//! 机器码（building-structures findings，178 例/3104 条 64B 冻结）：
//! - 消费 136B 掩码 → materializer（14122ff90）→ 104B (0x68) 段；
//! - third 写回 = max(selGroundY−clipGroundY−.2, 0) + max(Pub284+.5, Pub288)，
//!   随后 +bitsbde66667(−0.112500004) 按 LoopRange 映射 prepared index
//!   与旧 third maxnum（oldNaN→new）；
//! - special 源 raw+54 夹 .3..2.5（有效下限 .8）。
//!
//! W-B1 接线：`materialize_wall_segments` + `construct_lintel_supports`
//! （systems/wall/construction/structures.rs 冻结链）→ 64B 木板输出落盘。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;
use country_core::systems::wall::construction::structures::WallSegmentSource;
use country_core::systems::wall::construction::structures::construct_lintel_supports;
use country_core::resources::{
    roofs::commands::CreateRoofCmd,
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use bevy_ecs::prelude::*;

pub mod place_support_beams;

/// B `WallPathSegmentationMasksMinusStairs`（lintels 专用掩码资源；
/// 楼梯段剔除后的 136B 掩码）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct WallPathSegmentationMasksMinusStairs {
    pub masks: hashbrown::HashMap<u64, Vec<WallPathSegmentationMask>>,
}

/// B 0x14076d9f0。
pub fn construct_lintels(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    walls: Res<PublicWalls>,
    mut masks: ResMut<WallPathSegmentationMasksMinusStairs>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0 .0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let projection = wall.building_projection();
        let Some(segments) = masks.masks.get(&wall_id) else {
            continue;
        };
        let curve = wall.simplified_curve_2d();
        let prepared_positions: Vec<[f32; 3]> = curve
            .points
            .iter()
            .map(|p| [p[0], p[1], 0.0])
            .collect();
        let prepared_u: Vec<f32> = curve.points.iter().map(|p| p[0]).collect();
        let segment_values: Vec<f32> = curve.points.iter().map(|p| p[1]).collect();
        let mut arch = vec![0.0f32; prepared_positions.len().max(2)];
        let mut third = vec![0.0f32; prepared_positions.len().max(2)];
        for mask in segments {
            let source = WallSegmentSource {
                mask,
                segment_values: &segment_values,
                arch_heights: &arch,
                prepared_positions: &prepared_positions,
                prepared_u: &prepared_u,
                regular: projection.shape().tag() < 2,
                // 闭合判定：掩码 LoopRange 的闭合性质（for_closed/for_open 族；
                // 当前以开环登记，精确判定待 range 接线）。
                closed: false,
            };
            if let Ok(planks) = construct_lintel_supports(
                source,
                projection.float_0x284(),
                projection.float_0x288(),
                wall_id,
                &mut third,
            ) {
                for plank in &planks {
                    output.plank_instances.push(plank.observed_bytes());
                }
            }
        }
        let _ = ssbo;
    }
}
