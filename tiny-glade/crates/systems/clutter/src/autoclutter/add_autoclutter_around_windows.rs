//! B 0x1422cefe0：从真实墙装饰存储生成窗口周围的自动装饰区域。
use bevy_ecs::prelude::*;
use country_core::components::drawable::{AutoClutterSource, AutoClutterSourceIdTag};
use country_core::resources::walls::{
    cached_decorator_transforms::CachedDecoratorTransforms,
    decorator::{DecoratorId, DecoratorType}, decorator_storage::DecoratorStorage,
    decorator_subtype::DerivedDecoratorInfoResources, public_walls::PublicWalls,
};
use glam::{Vec2, Vec3};
use utils::curve::{curve_slice::CurveSlice, CurveCoordSemantic2D, CurveInternalCoord};
use super::{AutoClutterRegion, AutoClutterState, ClutterCandidateIndex, ClutterRegionId};

pub const DECORATOR_RIGHT: &str = "decorator_right";
pub const DECORATOR_LEFT: &str = "decorator_left";
pub const TRAP_DOOR: &str = "trap_door";

/// 原函数遍历每面墙的真实装饰集合；墙与装饰变化由区域 epoch 收敛。
pub fn add_autoclutter_around_windows(
    mut state: ResMut<AutoClutterState>,
    decorators: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    transforms: Res<CachedDecoratorTransforms>,
    derived: DerivedDecoratorInfoResources,
) {
    for (wall_id, decorators) in decorators.iter_walls() {
        let Some(wall) = walls.get(wall_id) else { continue };
        if !wall.is_premade_shape() { continue; }
        for (id, info) in decorators.iter_maybe_mut() {
            if !DecoratorType(info.ty).spawns_clutter() { continue; }
            let tag = info.dst_tag();
            let branch = if tag.wrapping_sub(2) < 3 { tag.wrapping_sub(1) } else { 0 };
            if branch == 0 {
                let points = wall.geometry.outline.as_slice();
                let first = CurveInternalCoord::new_for_point_count(0, 0.0, points.len());
                let mut slice = CurveSlice::<[f32; 3], _, CurveCoordSemantic2D, Vec2, _>::from_coord_range_with_remap(
                    points, first, first, |_, p| Vec2::new(p[0], p[2]));
                let coord = info.wall_space_coord();
                // MAXSS 遇 NaN 返回第二操作数 0。
                let distance = if coord.u > 0.0 { coord.u } else { 0.0 };
                slice.extend_right_until(distance, |_| false);
                let end = slice.end();
                let regions = wall_regions(points, end.segment, end.t, coord.y);
                for (name, (axis, origin)) in [(DECORATOR_RIGHT, regions[0]), (DECORATOR_LEFT, regions[1])] {
                    let key = ClutterRegionId(utils::calculate_hash(&(name, id as usize, info.decorator_id)));
                    let region = AutoClutterRegion {
                        source: AutoClutterSource::Id { tag: AutoClutterSourceIdTag::Native0, native_u64_08: wall_id.0, native_u32_10: id },
                        kind: 0, key, center: origin, basis: [axis, Vec2::new(4.0, 2.0)],
                        f34: id as i32, b38: 0, b39: info.u45,
                        candidate_index: ClutterCandidateIndex::NONE, epoch: state.epoch,
                    };
                    state.add_region(key, &region);
                }
            } else if branch == 1 {
                if matches!(derived.as_view().subtype(DecoratorId(id)).0, 5 | 10) {
                    let transform = transforms.transform(id);
                    for (index, (axis, origin)) in trap_regions(transform.rotation.to_array(), transform.translation).into_iter().enumerate() {
                        let key = ClutterRegionId(utils::calculate_hash(&(TRAP_DOOR, index as i32, id as usize, info.decorator_id)));
                        let region = AutoClutterRegion {
                            source: AutoClutterSource::Id { tag: AutoClutterSourceIdTag::Native0, native_u64_08: wall_id.0, native_u32_10: id },
                            kind: 0, key, center: origin, basis: [axis, Vec2::splat(1.5)],
                            f34: id as i32, b38: 1, b39: info.u45,
                            candidate_index: ClutterCandidateIndex::NONE, epoch: state.epoch,
                        };
                        state.add_region(key, &region);
                    }
                }
            } else {
                panic!("unsupported decorator destination for window autoclutter");
            }
        }
    }
}

/// 原 SIMD 运算顺序；128 个真实机器码样本逐位相同。
fn wall_regions(points: &[[f32; 3]], i: usize, t: f32, height: f32) -> [(Vec2, Vec3); 2] {
    let prev = points[i.saturating_sub(1)];
    let a = points[i];
    let b = points[i + 1];
    let dx = b[0] - prev[0];
    let dz = b[2] - prev[2];
    let inv = 1.0 / (dz * dz + dx * dx).sqrt();
    let (tx, tz) = if inv.is_finite() && inv >= 0.0 { (dx * inv, dz * inv) } else { (0.0, 0.0) };
    let x = b[0] * t + a[0] * (1.0 - t);
    let z = a[2] * (1.0 - t) + b[2] * t;
    let diagonal = Vec2::new(tx * 4.0 - tz * 2.0, tz * 4.0 + tx * 2.0);
    let right = Vec2::new(((tx * 4.72 - tz * 2.0) * 0.5 + x) + tz * -0.28,
                          ((tz * 4.72 + tx * 2.0) * 0.5 + z) + tx * 0.28);
    let left = Vec2::new(((-tx * 4.72 - tz * 2.0) * 0.5 + x) + tz * -0.28,
                         ((-tz * 4.72 + tx * 2.0) * 0.5 + z) + tx * 0.28);
    let axis = Vec2::new(tx, tz);
    let origin = |center: Vec2| Vec3::new(center.x + diagonal.x * -0.5, height + -0.0, center.y + diagonal.y * -0.5);
    [(axis, origin(right)), (axis, origin(left))]
}

/// 原四元数计算不假设单位长度；逐项保留 SIMD 的乘加顺序。
fn trap_regions(q: [f32; 4], position: Vec3) -> [(Vec2, Vec3); 2] {
    let [x, y, z, w] = q;
    let sum = z * z + (y * y + x * x);
    let scale = w * w - sum;
    let dot = ((x * 1.0 + y * 0.0) + z * 0.0) * 2.0;
    let tx = (w + w) * (y * 0.0 - z * 0.0) + (dot * x + scale * 1.0);
    let tz = (w + w) * (x * 0.0 - y * 1.0) + (dot * z + scale * 0.0);
    let a = tx * 1.5;
    let b = tz * 1.5;
    let diagonal = Vec2::new(a - b, b + a);
    [1.0f32, -1.0].map(|side| {
        let cx = (tx * 2.9) * side * 0.5 + (position.x + 0.0);
        let cz = (tz * 2.9) * side * 0.5 + (position.z + 0.0);
        (Vec2::new(tx, tz), Vec3::new(cx + diagonal.x * -0.5, (position.y + 0.5) + -0.0, cz + diagonal.y * -0.5))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_decorator_and_published_wall_produce_two_sources() {
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{
            InnerWalls, WallId, WallState,
            decorator::{DecoratorDst, DecoratorInfo, WallDecoratorDst},
            decorator_storage::DecoratorStorageAddr,
            decorator_subtype::{DerivedDecoratorSubtypes, DecoratorMeshLibrary},
            publication_fields::WallShapeSnapshot,
            publication_source::TerrainHeightGrid512,
        }};
        let wall = WallId(0x1234_5678_0000_002a);
        let curve = Curve2::try_new(vec![[-4., -4.], [4., -4.], [4., 4.], [-4., 4.]], true).unwrap();
        let mut snapshot = WallState::new(curve, 3.0, 0, 0, false).snapshot().clone();
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, value) in [0.0f32, 0.0, 1.25, 0.0, 8.0, 8.0].into_iter().enumerate() {
            bytes[4 + i * 4..8 + i * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
        snapshot.source.curve_kind_tag = 1;
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(wall, WallState::from_snapshot(snapshot, 0));
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false,
        }, None, &Default::default(), &Default::default());
        let mut info = DecoratorInfo { ty: 0x38, decorator_id: 0x9876, u45: 3, ..Default::default() };
        info.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord: utils::wall_space::WallSpaceCoord::new_unchecked(4.0, 2.0),
            platform: WallId(99), relative_y: 0.0, position: Vec3::ZERO,
            corner_attachment: 1, bottom_as_door: false,
        }));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).insert(DecoratorId(7), info);
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(walls);
        world.insert_resource(CachedDecoratorTransforms::default());
        world.insert_resource(DerivedDecoratorSubtypes::default());
        world.insert_resource(DecoratorMeshLibrary::default());
        world.insert_resource(AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() });
        world.init_resource::<country_core::systems::ui::signifiers::UiSignifierStream>();
        world.init_resource::<country_core::resources::gui::DebugGui>();
        world.run_system_once(add_autoclutter_around_windows).unwrap();
        let state = world.resource::<AutoClutterState>();
        assert_eq!(state.regions.len(), 2);
        for region in state.regions.values() {
            assert!(matches!(&region.source, AutoClutterSource::Id { tag: AutoClutterSourceIdTag::Native0, native_u64_08, native_u32_10: 7 } if *native_u64_08 == wall.0));
            assert_eq!((region.kind, region.f34, region.b38, region.b39), (0, 7, 0, 3));
        }
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            components::drawable::{AutoClutter, AutoClutterConvertableToDecoClutter},
            resources::{clutter_meshes::{ClutterMesh, ClutterMeshes, ClutterMarkedForDeletion},
                terrain_height::{TerrainHeightSsbo, TerrainHeightsData}, terrain_rock_mask::TerrainRockMaskSsbo,
                world_raster::DirtyTilePublisher, world_raster_defs::{PathRaster, WaterRaster, GardenRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster}},
            systems::{collision::world::RaycastWorld, wall::wall_state::wall_spatial_hash::WallSpatialHash}};
        use super::super::{LazyGridClutterCell, populate_autoclutter_regions::{self as pop, AutoClutterDirtySubscriptions, BushGrid, DirtyTilesResource, MeshGroupWeights, PopulateLocal}};
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        fixed[6].mesh = Some(country_core::asset_libraries::Handle::new());
        fixed[6].bounds = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(), Default::default()));
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(country_core::render::ssbo::MappedStorageBuffer::from_mapping(
            &mut [], country_core::render::ssbo::rhapsody_boundary::Buffer { id: 79, generation: 1, byte_size: 0x8000, ..Default::default() }));
        world.insert_resource(library);
        world.init_resource::<WallSpatialHash>();
        world.insert_resource(AutoClutterDirtySubscriptions::default());
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(BushGrid::default());
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        // 与上面发布墙时的零高度场一致；默认高度场的全零编码为 -2.5，
        // 会使窗口区域高出地形 4.5，按原 >4 门控被正常拒绝。
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(0.0, 0.0);
        world.insert_resource(terrain);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        world.init_resource::<DirtyTilesResource>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems((pop::populate_autoclutter_regions,
            super::super::process_autoclutter_candidates::process_autoclutter_candidates,
            super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let mut query = world.query::<(Entity, &AutoClutter, &AutoClutterConvertableToDecoClutter,
            &country_core::components::drawable::NaniInstanceDesc)>();
        let produced = query.iter(&world).collect::<Vec<_>>();
        assert!(!produced.is_empty(), "真实窗口区域必须到达非空消费者");
        assert!(produced.iter().all(|(_, _, object, _)| matches!(&object.source,
            AutoClutterSource::Id { tag: AutoClutterSourceIdTag::Native0, native_u64_08, native_u32_10: 7 } if *native_u64_08 == wall.0)));
        let entities: Vec<_> = produced.iter().map(|(entity, _, _, _)| *entity).collect();
        drop(produced);
        for &entity in &entities {
            world.get_mut::<AutoClutter>(entity).unwrap().progress = 1.0;
        }
        world.resource_mut::<DecoratorStorage>().per_wall.clear();
        {
            world.resource_mut::<country_core::resources::time::GameTime>().frame_count = 1;
            let tick = world.resource::<country_core::resources::time::GameTime>().frame_count as u8;
            let mut state = world.resource_mut::<AutoClutterState>();
            state.set_current_tick(tick);
            state.reset_autoclutter_for_new_frame();
        }
        world.run_system_once(add_autoclutter_around_windows).unwrap();
        schedule.run(&mut world);
        let mut retired = world.query::<(&AutoClutter, &ClutterMarkedForDeletion)>();
        assert_eq!(retired.iter(&world).count(), entities.len(), "原窗口移除必须退休全部已有实体");
        world.init_resource::<Events<country_core::systems::audio::PlayClutterSoundCmd>>();
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 0.3;
        let mut animation = Schedule::default();
        animation.add_systems(super::super::autoclutter_animation::autoclutter_animation);
        animation.run(&mut world);
        assert!(entities.into_iter().all(|entity| world.get_entity(entity).is_err()), "退役动画结束必须销毁实体和 Nani 组件");
    }
    #[test]
    fn trap_regions_match_native_machine_code() {
        let cases: &[([u32; 7], [[u32; 7]; 2])] = include!("../../../../../reports/module-recovery-2026-10-23/m06-trap-regions-native-cases.rs");
        for &(input, expected) in cases {
            let f = input.map(f32::from_bits);
            let got = trap_regions([f[0], f[1], f[2], f[3]], Vec3::new(f[4], f[5], f[6])).map(|(a, p)|
                [a.x, a.y, 1.5, 1.5, p.x, p.y, p.z].map(f32::to_bits));
            assert_eq!(got, expected);
        }
    }
    #[test]
    fn wall_regions_match_native_machine_code() {
        let cases: &[([u32; 11], usize, [[u32; 7]; 2])] = include!("../../../../../reports/module-recovery-2026-10-23/m06-window-regions-native-cases.rs");
        for &(input, i, expected) in cases {
            let f = input.map(f32::from_bits);
            let points = [[f[0], f[1], f[2]], [f[3], f[4], f[5]], [f[6], f[7], f[8]]];
            let got = wall_regions(&points, i, f[9], f[10]).map(|(a, p)|
                [a.x, a.y, 4.0, 2.0, p.x, p.y, p.z].map(f32::to_bits));
            assert_eq!(got, expected);
        }
    }
}
