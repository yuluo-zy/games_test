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
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::commands::CreateRoofCmd,
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;
use country_core::systems::wall::construction::structures::WallSegmentSource;
use country_core::systems::wall::construction::structures::construct_lintel_supports;

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
    mut walls: ResMut<PublicWalls>,
    mut masks: ResMut<WallPathSegmentationMasksMinusStairs>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let mut recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    recalc.sort_unstable();
    recalc.dedup();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get_mut(id) else {
            continue;
        };
        let projection = wall.building_projection();
        if projection.observed_296() == 0 {
            // Native76e198..19d bypasses the complete ordinary-wall mask loop.
            wall.material_stats.set_lintels_segments_count(0);
            wall.material_stats.add_lintels_length(0.0);
            continue;
        }
        let segments = masks
            .masks
            .get(&wall_id)
            .expect("lintels require processed masks for the source wall");
        let path = wall
            .prepared_path
            .as_ref()
            .expect("lintels require prepared wall path");
        let arch = wall
            .arch_segment_heights()
            .expect("lintels require actual arch height cache");
        let mut third = wall.segment_heights.clone();
        let mut instances = Vec::new();
        let mut stats_count = 0u64;
        let mut stats_length = 0.0f32;
        for mask in segments {
            let source = WallSegmentSource {
                mask,
                segment_values: &path.segment_values,
                arch_heights: arch,
                prepared_positions: &path.positions,
                prepared_u: &path.points_u,
                regular: projection.shape().tag() < 2,
                closed: projection.shape().tag() != 2,
            };
            // Observe the same pre-profile materializer; complete retained
            // LintelSegments/pillar-proposal ownership is not implied here.
            let materialized =
                country_core::systems::wall::construction::structures::materialize_wall_segments(
                    source,
                )
                .expect("invalid authoritative lintel materialization input");
            stats_count += materialized.len() as u64;
            stats_length += mask.length;
            if let Ok(planks) = construct_lintel_supports(
                source,
                projection.float_0x284(),
                projection.float_0x288(),
                wall_id,
                &mut third,
            ) {
                instances.extend(planks.iter().map(crate::gpu_output::wooden_plank));
            } else {
                panic!("invalid lintel authoritative input for wall {wall_id}");
            }
        }
        wall.segment_heights = third;
        let storage = ssbo
            .get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before lintel construction");
        output.gpu.append_planks(
            storage,
            crate::gpu_output::ConstructorKey {
                wall: wall_id,
                part: crate::gpu_output::ConstructorPart::Lintels,
            },
            &instances,
        );
        wall.material_stats
            .set_lintels_segments_count(stats_count as u32);
        wall.material_stats.add_lintels_length(stats_length);
    }
}
