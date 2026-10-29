//! 拱构造（construct_arches/mod.rs + profile.rs）。
//!
//! `construct_arches`（0x14075f680，149KB）。候选参数面（skeleton 富化）：
//! `(EventReader<OnWallChanged>, EventReader<CreateRoofCmd>,
//!  EventReader<SwitchRoofTypeCmd>, ResMut<PublicWalls>,
//!  ResMut<WallPathSegmentationMasks>, …)`。
//!
//! 机器码要点（building-arches findings G05 冻结 130 例/4159 条 96B）：
//! - 消费 136B 处理掩码输入；shape_tag 0/1；has_roof 关系；
//!   Pub294!=0 时选跨段最高 (height+groundY)；第三数组跨段 maxnum 累计；
//!   闭合轮廓 first→last 复制（无 −0.1125）。
//! - `if style == 2` → 跳过共享拱装配路径，直达 MaterialStats 记录
//!   （set_arches_segments_count/add_arches_length/add_gate_length/
//!   set_gates_segments_count）—— **shape2 分支未冻结**（C 版 wooden_arch_frame
//!   未做原码差分；reviewer 登记为未覆盖/边界）。
//! - 产出（W-B2 落盘）：96B WallBrickInstance 记录 + ConstructGateCmd120
//!   （→ construct_gates 消费）+ segment_count → 输出资源写回。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::{
        Roof,
        commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    },
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;

pub mod profile;

/// 处理掩码资源（B `WallPathSegmentationMasks`：每墙 136B 记录的
/// 结构化投影；segmentation 族写回、arches/lintels 消费）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct WallPathSegmentationMasks {
    /// 每墙 136B 掩码（分段系统产出；arches/lintels 逐段消费）。
    pub masks: hashbrown::HashMap<u64, Vec<WallPathSegmentationMask>>,
}

/// 构造输出（渲染边界消费；W-B2 落盘）。
#[derive(bevy_ecs::resource::Resource, Default)]
pub struct WallConstructorOutput {
    /// Actual per-source sparse owners; CPU vectors below are legacy producer
    /// seams and must not be treated as uploaded instances.
    pub gpu: crate::gpu_output::ConstructorGpuOutput,
    /// 96B 砖实例（arches/construct_walls 产出）。
    pub brick_instances: Vec<[u8; 96]>,
    /// 64B 木板实例（fences/lintels 产出）。
    pub plank_instances: Vec<[u8; 64]>,
    /// 门构造命令（arches 产出 → construct_gates 消费）。
    pub gate_commands: Vec<[u8; 128]>,
    /// 门扇 136B spawn payload（gates 产出 → 渲染边界）。
    pub door_spawn_payloads: Vec<[u8; 136]>,
}

/// B 0x14075f680。
pub fn construct_arches(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    mut walls: ResMut<PublicWalls>,
    mut masks: ResMut<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>,
    mut arch_segments: ResMut<country_core::resources::walls::arches::ArchSegments>,
    roofs: Query<&Roof>,
    mut previous: ResMut<country_core::resources::walls::arches::BuildingHadArchesLastFrame>,
    mut recalculate: EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    mut output: ResMut<WallConstructorOutput>,
    mut gates: EventWriter<crate::construct_gates::systems::ConstructGateCmd>,
) {
    let mut recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0))
        .collect();
    recalc.sort_unstable();
    recalc.dedup();
    let mut exhausted = false;
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        // Native76047b calls PublicWalls::has_roof, including its failure-map
        // and generation test. Query validation belongs only to the tail below.
        let has_roof = walls.has_roof(id);

        let wall = walls
            .get_mut(id)
            .expect("arch event must reference a published wall");
        let storage = ssbo
            .get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
            .expect("canonical BrickTransformSsbo must exist before arch construction");
        output.gpu.clear_bricks(storage, wall_id);
        let outcome = if wall.building_projection().observed_296() != 0 {
            country_core::systems::wall::construction::arches::WallArchesOutput::default()
        } else {
            let segments = masks
                .masks
                .get(&wall_id)
                .expect("arches require processed masks for the source wall");
            wall.construct_arches(id, segments, has_roof)
                .expect("invalid or unrecovered authoritative arch input")
        };
        if wall.building_projection().observed_296() == 0 {
            let mask_list = masks.masks.get_mut(&wall_id).expect("arches require processed masks for the source wall");
            assert_eq!(mask_list.len(), outcome.pillar_caches.len());
            for (mask, (proposals, scores, keys)) in mask_list.iter_mut().zip(&outcome.pillar_caches) {
                mask.pillar_proposals.clone_from(proposals);
                mask.pillar_scores.clone_from(scores);
                mask.pillar_keys.clone_from(keys);
            }
        }
        {
            let stats = &outcome.material_stats;
            wall.material_stats
                .set_arches_segments_count(stats.arches_segments_count());
            wall.material_stats.add_arches_length(stats.arches_length);
            wall.material_stats.add_gate_length(stats.gates_length);
            wall.material_stats
                .set_gates_segments_count(stats.gates_count as u32);
            wall.material_stats.add_sewer_length(stats.sewer_length);
            wall.material_stats
                .set_sewer_segments_count(stats.sewer_count as u32);
            arch_segments.0.insert(id, outcome.segments);
            // W-B2：96B 实例 + ConstructGateCmd120 落盘。
            exhausted |= output.gpu.append_bricks(
                storage,
                crate::gpu_output::ConstructorKey {
                    wall: wall_id,
                    part: crate::gpu_output::ConstructorPart::Arches,
                },
                &outcome.instances,
            ) != 0;
            for gate in &outcome.gate_commands {
                gates.write(crate::construct_gates::systems::ConstructGateCmd {
                    wall: gate.wall_id.0,
                    ordinal: gate.ordinal,
                    segment: gate.segment.clone(),
                });
            }
            let _ = outcome.segment_count;
        }
    }
    if exhausted {
        log::error!(target:"system_wall_constructor::construct_arches",
            "Arch constructor ran out of bricks");
    }
    // Original762ce0..7630a5 scans all public rows every invocation, not only
    // this frame's event IDs. Only a live, non-flat associated roof participates.
    for (id, _) in walls.all() {
        let Some(entity) = walls
            .roof_entity(id)
            .filter(|entity| entity.to_bits() >> 32 != 0)
        else {
            continue;
        };
        let Ok(roof) = roofs.get(entity) else {
            continue;
        };
        if roof.is_flat() {
            continue;
        }
        let now = arch_segments
            .0
            .get(&id)
            .is_some_and(|segments| segments.len() >= 2);
        let before = previous.0.get(&id.0).copied().unwrap_or(false);
        if before != now {
            recalculate.write(
                country_core::resources::roofs::commands::RecalculateRoofCmd {
                    entity,
                    wall: id,
                    kind: 3,
                },
            );
        }
        previous.0.insert(id.0, now);
    }
}

#[cfg(test)]
mod pillar_producer_tests {
    use super::*;
    use bevy_ecs::schedule::Schedule;
    use country_core::{
        geometry::instanced_wall::BrickTransformSsbo,
        render::{sparse_instance_buffer::SparseInstanceBuffer, ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice}},
        resources::{roofs::{geometry::surface::Curve2, commands::{RecalculateRoofCmd, RecalculateWallBricks}},
            walls::{InnerWalls, arches::{ArchSegments, BuildingHadArchesLastFrame}, publication_source::TerrainHeightGrid512},
            world_raster::DirtyTilePublisher,
            world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster}},
        systems::wall::{construction::segmentation::{wall_path_segmentation, WallSegmentationInput},
            registration::register_wall_cpu_resources,
            wall_state::publish_public_wall_state::{publish_public_wall_state, WallPublicationTerrain}},
    };
    struct CpuDevice;
    impl GraphicsDevice for CpuDevice {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
            Ok(Buffer { id: 813, byte_size: desc.size as u64, ..Default::default() })
        }
        fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage("此测试不提供 GPU 上传"))
        }
    }
    #[test]
    fn real_arch_segments_feed_stone_pillars() {
        let mut world = World::new();
        register_wall_cpu_resources(&mut world);
        world.init_resource::<Events<CreateRoofCmd>>();
        world.init_resource::<Events<SwitchRoofTypeCmd>>();
        world.init_resource::<Events<RecalculateRoofCmd>>();
        world.init_resource::<Events<RecalculateWallBricks>>();
        world.init_resource::<Events<crate::construct_gates::systems::ConstructGateCmd>>();
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0., maximum_y:0., sampling_fix:false }));
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        path.write_composite(Some(&vec![255;512*512])); world.insert_resource(path);
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.init_resource::<ArchSegments>();
        world.init_resource::<BuildingHadArchesLastFrame>();
        world.init_resource::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>();
        world.init_resource::<WallConstructorOutput>();
        let library = AssetSsboLibrary::new();
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&CpuDevice, None));
        world.insert_resource(library);
        let id = world.resource_mut::<InnerWalls>().new_wall_w_params(
            Curve2::try_new(vec![[-6.,0.],[-1.,0.],[3.,2.],[7.,2.]], false).unwrap(), 1.2, 0, 0, false);
        let mut publication = Schedule::default(); publication.add_systems(publish_public_wall_state); publication.run(&mut world);
        let masks = {
            let walls = world.resource::<PublicWalls>(); let wall = walls.get(id).unwrap();
            assert_eq!(wall.building_projection().observed_296(), 0);
            let path = wall.prepared_path.as_ref().unwrap();
            wall_path_segmentation(WallSegmentationInput {prepared_positions:&path.positions, path_values:&path.segment_values,
                shape:wall.shape(), observed_special_flag:wall.authority.source_flag_65()}).unwrap()
        };
        world.resource_mut::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>().masks.insert(id.0, masks);
        let mut source = Schedule::default(); source.add_systems(construct_arches); source.run(&mut world);
        let count: usize = world.resource::<crate::construct_lintels::WallPathSegmentationMasksMinusStairs>().masks[&id.0].iter().map(|m|m.pillar_proposals.len()).sum();
        assert!(count > 0, "真实拱段必须生成非空柱候选");
        let before: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        let mut place = Schedule::default(); place.add_systems(crate::place_pillars::place_pillars); place.run(&mut world);
        let stats = &world.resource::<PublicWalls>().get(id).unwrap().material_stats;
        assert_eq!(stats.pillar_placements.len(), count);
        let after: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert!(after > before, "真实石柱必须写入砖分配");
    }
}
