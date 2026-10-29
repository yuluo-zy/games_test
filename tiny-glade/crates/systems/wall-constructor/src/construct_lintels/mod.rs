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
//! 本轮补齐真实持久段owner：同一materializer/支持梁流程返回改高后的有序104B段，
//! 原14076f368保存到LintelSegments供碰撞consumer使用；普通墙分支用空Vec替换。
//! 64B木板仍复用当前重构稀疏buffer；原mask的pillar proposal缓存另属后续构造业务。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::commands::CreateRoofCmd,
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use country_core::systems::wall::construction::segmentation::WallPathSegmentationMask;
use country_core::systems::wall::construction::structures::WallSegmentSource;
use country_core::systems::wall::construction::structures::construct_lintel_supports_with_segments;

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
    mut lintel_segments: ResMut<country_core::resources::walls::arches::LintelSegments>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    // 原14076dcaf在读取事件前先取得真实WoodenPlankSsbo，缺失必须失败，空帧亦然。
    let storage = ssbo
        .get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
        .expect("canonical WoodenPlankSsbo must exist before lintel construction");
    let mut recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .collect();
    recalc.sort_unstable();
    recalc.dedup();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let wall = walls.get_mut(id).expect("lintel event must reference a published wall");
        let projection = wall.building_projection();
        let path = wall
            .prepared_path
            .as_ref()
            .expect("lintels require prepared wall path");
        let arch = wall
            .arch_segment_heights()
            .expect("lintels require actual arch height cache");
        if projection.observed_296() == 0 {
            // Native76e198..19d bypasses the complete ordinary-wall mask loop.
            wall.material_stats.set_lintels_segments_count(0);
            wall.material_stats.add_lintels_length(0.0);
            // 原14076e198跳到末端insert空Vec：禁用过梁时替换并释放旧三组数组。
            lintel_segments.0.insert(id, Vec::new());
            continue;
        }
        let segments = masks
            .masks
            .get_mut(&wall_id)
            .expect("lintels require processed masks for the source wall");
        let mut third = wall.segment_heights.clone();
        let mut instances = Vec::new();
        let mut stats_count = 0u64;
        let mut stats_length = 0.0f32;
        let mut retained_segments = Vec::new();
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
            let (planks, materialized) = construct_lintel_supports_with_segments(
                source, projection.float_0x284(), projection.float_0x288(), wall_id, &mut third,
            ).expect("invalid lintel authoritative input");
            stats_count += materialized.len() as u64;
            stats_length += mask.length;
            instances.extend(planks.iter().map(crate::gpu_output::wooden_plank));
            // 原 lintel 路径对每个改高后的段无条件采集两端，高度减 0.225。
            let mut cache = (std::mem::take(&mut mask.pillar_proposals),
                std::mem::take(&mut mask.pillar_scores), std::mem::take(&mut mask.pillar_keys));
            let mut unused_start = false;
            for segment in &materialized {
                country_core::systems::wall::construction::arches::collect_pillar_endpoints(
                    segment, &mut cache, &mut unused_start, true);
            }
            (mask.pillar_proposals, mask.pillar_scores, mask.pillar_keys) = cache;
            // 原Vec::extend保持mask/段顺序，保留真实改高后的段而非统计后丢弃。
            retained_segments.extend(materialized);
        }
        wall.segment_heights = third;
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
        // 原14076f2xx插入只在该墙完整构造成功后发生，旧段随替换释放。
        lintel_segments.0.insert(id, retained_segments);
    }
}

#[cfg(test)]
mod retained_lintel_tests {
    use super::*;
    use country_core::resources::{
        walls::{InnerWalls, arches::LintelSegments, publication_source::TerrainHeightGrid512},
        world_raster::DirtyTilePublisher,
        world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster},
    };
    use country_core::render::{sparse_instance_buffer::SparseInstanceBuffer, ssbo::rhapsody_boundary::{GraphicsDevice, BufferDesc, Buffer, BackendError}};
    use country_core::resources::roofs::{roof_buffers::WoodenPlankSsbo, geometry::surface::Curve2};
    use country_core::systems::wall::{registration::register_wall_cpu_resources, wall_state::publish_public_wall_state::{WallPublicationTerrain, publish_public_wall_state}};
    use country_core::systems::wall::construction::segmentation::{wall_path_segmentation, WallSegmentationInput};

    // 只提供真实稀疏缓冲的CPU阴影分配；上传明确失败，不把它当真实GPU验收。
    struct CpuShadowDevice;
    impl GraphicsDevice for CpuShadowDevice {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
            Ok(Buffer { id: 812, byte_size: desc.size as u64, ..Default::default() })
        }
        fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage("过梁CPU阴影测试不提供GPU上传"))
        }
    }

    fn fixture() -> (World, Schedule, Schedule) {
        let mut world = World::new();
        register_wall_cpu_resources(&mut world);
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false }));
        let mut path = PathRaster::new(DirtyTilePublisher::unique());
        path.write_composite(Some(&vec![255;512*512]));
        world.insert_resource(path);
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.init_resource::<LintelSegments>();
        world.init_resource::<WallPathSegmentationMasksMinusStairs>();
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        let library = AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&CpuShadowDevice, None));
        world.insert_resource(library);
        let mut publication = Schedule::default(); publication.add_systems(publish_public_wall_state);
        let mut construct = Schedule::default(); construct.add_systems((crate::gpu_output::clear_constructor_planks, construct_lintels).chain());
        (world, publication, construct)
    }

    #[test]
    fn actual_lintel_writer_keeps_profiled_segments_replaces_disabled_and_preserves_other_rows() {
        let (mut world, mut publication, mut construct) = fixture();
        let id = world.resource_mut::<InnerWalls>().new_wall_w_params(Curve2::try_new(vec![[-6.0,0.0],[-1.0,0.0],[3.0,2.0],[7.0,2.0]],false).unwrap(), 1.2, 0, 1, false);
        publication.run(&mut world);
        let mut old_points = None;
        for extra in [1.2, 2.4] {
            if extra != 1.2 {
                world.resource_mut::<InnerWalls>().update_wall_state_height(id, extra);
                publication.run(&mut world);
            }
            let masks = {
                let wall = world.resource::<PublicWalls>().get(id).unwrap();
                let path = wall.prepared_path.as_ref().unwrap();
                assert!(path.segment_values.iter().any(|v| *v > 0.0));
                wall_path_segmentation(WallSegmentationInput { prepared_positions: &path.positions, path_values: &path.segment_values, shape: wall.shape(), observed_special_flag: wall.authority.source_flag_65() }).unwrap()
            };
            assert!(!masks.is_empty());
            world.resource_mut::<WallPathSegmentationMasksMinusStairs>().masks.insert(id.0, masks);
            construct.run(&mut world);
            let retained = &world.resource::<LintelSegments>().0[&id];
            assert!(!retained.is_empty());
            assert!(retained.iter().all(|s| s.heights.len() == s.positions.len() && s.points_u.len() == s.positions.len()));
            let mut normals = Vec::new(); let mut triangles = Vec::new();
            for segment in retained { country_core::systems::collision::world::construct_arch_geo(segment, &mut normals, &mut triangles); }
            assert!(!triangles.is_empty()); assert_eq!(normals.len(), triangles.len());
            let current: Vec<_> = triangles.iter().flat_map(|t| t.points).flat_map(|p| p.to_array()).map(f32::to_bits).collect();
            if let Some(old) = old_points.replace(current.clone()) { assert!(current != old, "合法高度编辑必须改变真实过梁碰撞面"); }
            let library = world.resource::<AssetSsboLibrary>();
            assert!(!library.get::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().is_empty());
            // 同一system的EventReader已消费，静止帧不得追加或替换该有序Vec。
            let pointer = retained.as_ptr();
            construct.run(&mut world);
            assert_eq!(world.resource::<LintelSegments>().0[&id].as_ptr(), pointer);
        }
        let unrelated = country_core::resources::walls::WallId(id.0+100);
        let previous = world.resource::<LintelSegments>().0[&id].clone();
        world.resource_mut::<LintelSegments>().0.insert(unrelated, previous.clone());
        let authority = world.resource_mut::<InnerWalls>().remove(id);
        let (mut snapshot, freehand) = authority.into_snapshot();
        snapshot.source.clutter_version = 0;
        world.resource_mut::<InnerWalls>().new_wall_with_id(id, country_core::resources::walls::WallState::from_snapshot(snapshot, freehand));
        publication.run(&mut world);
        construct.run(&mut world);
        assert!(world.resource::<LintelSegments>().0[&id].is_empty());
        assert_eq!(world.resource::<LintelSegments>().0[&unrelated], previous);
    }

    #[test]
    fn actual_lintel_writer_missing_event_wall_is_an_original_failure() {
        let (mut world, _, _) = fixture();
        let mut construct = Schedule::default(); construct.add_systems(construct_lintels);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(country_core::resources::walls::WallId(999)));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| construct.run(&mut world))).is_err());
    }
    #[test]
    fn actual_lintel_writer_requires_original_storage_even_without_events() {
        let (mut world, _, _) = fixture();
        world.insert_resource(AssetSsboLibrary::new());
        let mut construct = Schedule::default(); construct.add_systems(construct_lintels);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| construct.run(&mut world))).is_err());
    }

    #[test]
    fn actual_lintel_writer_failure_keeps_last_complete_owned_segments_and_height_authority() {
        let (mut world, mut publication, mut construct) = fixture();
        let id = world.resource_mut::<InnerWalls>().new_wall_w_params(Curve2::try_new(vec![[-4.0,0.0],[0.0,0.0],[4.0,2.0]],false).unwrap(),1.2,0,1,false);
        publication.run(&mut world);
        let masks = {
            let wall = world.resource::<PublicWalls>().get(id).unwrap();
            let path = wall.prepared_path.as_ref().unwrap();
            wall_path_segmentation(WallSegmentationInput {prepared_positions:&path.positions,path_values:&path.segment_values,shape:wall.shape(),observed_special_flag:wall.authority.source_flag_65()}).unwrap()
        };
        world.resource_mut::<WallPathSegmentationMasksMinusStairs>().masks.insert(id.0,masks);
        construct.run(&mut world);
        let saved = world.resource::<LintelSegments>().0[&id].clone(); assert!(!saved.is_empty());
        let third = world.resource::<PublicWalls>().get(id).unwrap().segment_heights.clone();
        world.resource_mut::<WallPathSegmentationMasksMinusStairs>().masks.remove(&id.0);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(id));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||construct.run(&mut world))).is_err());
        assert_eq!(world.resource::<LintelSegments>().0[&id],saved);
        assert_eq!(world.resource::<PublicWalls>().get(id).unwrap().segment_heights,third);
    }

}
