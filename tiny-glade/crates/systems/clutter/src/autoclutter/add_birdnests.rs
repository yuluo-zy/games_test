//! add_birdnests（B 主基线；0x1422c9c70，源码 37–141；closure$0 L84、closure$1 L105）。
//!
//! fresh 反编译（2026-10-17，--worker shared）控制流：
//! 1. puffin 范围；
//! 2. `Subscriber::take<DirtyTiles>`（0x141247c90）→ 复制 0x200 字节位集 → 合入本地
//!    `DirtyTiles::new_clear`；
//! 3. 遍历真实 `BirdNestCandidateLocations`（stride 0x40），用真实 GlobalId
//!    查已处理集；脏格控制碰撞缓存失效，不产生候选。
//! 4. 屋顶截面轮廓/碰撞缓存通过后输出真实 Arc source、Birdnest mesh 和目标墙。
//! 碰撞交线到候选数组的上游构建仍由 collision owner 单独恢复。

use bevy_ecs::prelude::*;
use country_core::systems::collision::{BirdNestCandidateLocations, world::RaycastWorld};
use country_core::utils::dirty_tiles::DirtyTiles;
use std::sync::Mutex;
use tabloid::Subscriber;

use super::{AutoClutterPayload, AutoClutterSpawn, AutoClutterState, ResolvedAutoClutterObject};

/// `BirdNestLazySubscriber`（原版 `add_birdnests::BirdNestLazySubscriber`）：`add_birdnests` 的
/// `ResMut` 参数（校验串 0x142f330c3），由 Startup 系统 [`birdnests_subscribe_to_tabloids`] 经
/// `Commands::insert_resource` 在运行期插入（原版值 = 一个 8 B `Subscriber<DirtyTiles>` 句柄，
/// `*param + 0` 处被 `Subscriber::take` 取用）。Subscriber 含 Cell 非 Sync，以 Mutex 包裹。
#[derive(Resource)]
pub struct BirdNestLazySubscriber(pub Mutex<Subscriber<DirtyTiles>>);

/// 原系统 Local：以完整 GlobalId 缓存屋顶碰撞结果。
#[derive(Default)]
pub struct BirdNestCollisionCache(std::collections::HashMap<u64, bool>);

/// B 140960b20：FMA 后按 i32 饱和截断，再用无符号位检查 64×64。
fn birdnest_dirty_cell(point: glam::Vec2) -> Option<[u32; 2]> {
    let scale = f32::from_bits(0x3efc0fc1);
    let x = point.x.mul_add(scale, 32.0) as i32 as u32;
    let y = point.y.mul_add(scale, 32.0) as i32 as u32;
    ((x | y) < 64).then_some([x, y])
}

/// 原 1422ca8c0：只把处于同一高度截面的屋顶轮廓算作碰撞。
fn roof_blocks_birdnest(roof: &country_core::resources::roofs::roof_shape::Roof, position: glam::Vec3, radius: f32) -> bool {
    use country_core::resources::roofs::{geometry::surface::RoofSection, roof_shape::RoofIntersectionSection};
    use utils::geometry::{Circle2d, Rectangle2d};
    let point = glam::Vec2::new(position.x, position.z);
    let section = roof.roof_shape_at_intersection_y(position.y);
    let closest = match section {
        RoofIntersectionSection::OutOfBounds => return false,
        RoofIntersectionSection::Section(RoofSection::Circle { frame, radius, word_0x18 }) =>
            Circle2d { center: glam::Vec2::from_array(frame), radius, rotation: f32::from_bits(word_0x18) }.closest_point_on_perimeter(point),
        RoofIntersectionSection::Section(RoofSection::Rectangle { frame, dimensions }) =>
            Rectangle2d { axis: glam::Vec2::new(frame[0], frame[1]), center: glam::Vec2::new(frame[2], frame[3]),
                size: glam::Vec2::from_array(dimensions) }.closest_point_on_perimeter(point),
        RoofIntersectionSection::RawSection(raw) => {
            let f = |at: usize| f32::from_le_bytes(raw[at..at+4].try_into().unwrap());
            let tag = u32::from_le_bytes(raw[0..4].try_into().unwrap());
            if tag == 2 { return false; }
            if tag == 0 {
                Circle2d { center: glam::Vec2::new(f(4), f(8)), radius: f(12), rotation: f(16) }.closest_point_on_perimeter(point)
            } else {
                Rectangle2d { axis: glam::Vec2::new(f(4), f(8)), center: glam::Vec2::new(f(12), f(16)),
                    size: glam::Vec2::new(f(20), f(24)) }.closest_point_on_perimeter(point)
            }
        }
    };
    let delta = closest - point;
    // UCOMISS/JBE：相等及无序均继续扫描，不构成碰撞。
    radius * 0.7 > (delta.y * delta.y + delta.x * delta.x).sqrt()
}

/// `birdnests_subscribe_to_tabloids`（Startup 第三成员；PDB 无独立符号，系统名串 0x143040d48；
/// FunctionSystem 的 `run_without_applying_deferred` 0x141cde5d0 内联了函数体）。
///
/// 机器码证据：参数 `(ResMut<RaycastWorld>, Commands)`（`validate_param_unsafe` 0x1422c2ea0 只校验
/// `ResMut<RaycastWorld>` 串 0x142f3317d）；体为
/// `commands.insert_resource(BirdNestLazySubscriber(raycast.subscribe_to_changes(1)))`——
/// `RaycastWorld::subscribe_to_changes`（0x1408cdc20）实参 `DL = 1`（槽 1），随后
/// `Commands::insert_resource`（类型元数据 0x143041ff8）。
pub fn birdnests_subscribe_to_tabloids(mut raycast: ResMut<RaycastWorld>, mut commands: Commands) {
    commands.insert_resource(BirdNestLazySubscriber(Mutex::new(raycast.subscribe_to_changes(1))));
}

/// add_birdnests（0x1422c9c70）：消费真实候选并使用订阅脏格更新碰撞缓存。
pub fn add_birdnests(
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    mut state: ResMut<AutoClutterState>,
    sub: ResMut<BirdNestLazySubscriber>,
    candidates: Res<BirdNestCandidateLocations>,
    meshes: Res<country_core::resources::clutter_meshes::ClutterMeshes>,
    deleted: Res<country_core::systems::ui_clutter::DeletedAutoClutter>,
    mut cache: Local<BirdNestCollisionCache>,
) {
    // O：Subscriber::take → 复制 0x200B 位集 → new_clear 合入；以合入位集为遍历源。
    // 原无资源位集回退：take 无结果即空位集（无脏格可处理）。
    let mut tiles = country_core::utils::dirty_tiles::DirtyTiles::new_clear();
    if let Some(reader) = sub.0.lock().unwrap().take() {
        tiles.extend(&*reader);
    }
    let mesh = meshes.mesh(country_core::resources::clutter_meshes::ClutterMeshTy::Birdnest);
    for candidate in &candidates.0 {
        if deleted.is_deleted(candidate.global_id) { continue; }
        let position = glam::Vec3::new(candidate.position.x + 0.0,
            candidate.position.y + 0.15, candidate.position.z + 0.0);
        let dirty = birdnest_dirty_cell(glam::Vec2::new(position.x, position.z))
            .is_some_and(|[x, y]| (tiles.rows()[y as usize] >> x) & 1 != 0);
        let blocked = if !dirty {
            cache.0.get(&candidate.global_id).copied()
        } else { None }.unwrap_or_else(|| {
            let blocked = roofs.iter().any(|roof| roof_blocks_birdnest(roof, position, mesh.radius_xz));
            cache.0.insert(candidate.global_id, blocked);
            blocked
        });
        if blocked { continue; }
        assert!(mesh.mesh.is_some(), "鸟巢实际 mesh 尚未加载");
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 4,
            payload: AutoClutterPayload::Object(ResolvedAutoClutterObject {
                metadata: mesh.into(),
                source: country_core::components::drawable::AutoClutterSource::Points(candidate.points.clone()),
                // 原判别为 OnBuilding/无 UV；零仅填充 inactive 坐标字节。
                destination: country_core::components::drawable::AutoClutterDestination::OnBuilding {
                    native_u32_04: 0, coordinates: glam::Vec2::ZERO,
                    wall: country_core::resources::walls::WallId(candidate.destination_wall),
                },
            }),
            f18: 1,
            key: candidate.global_id,
            marker: 3,
            selector_value: 0,
            position,
            host_id: candidate.destination_wall,
            f48: u64::from(position.x.to_bits())
                | (u64::from(position.y.to_bits()) << 32),
            f50: u64::from(position.z.to_bits()),
            f54: candidate.forward.x.to_bits(),
            f58: candidate.forward.y.to_bits(),
            f5c: candidate.forward.z.to_bits(),
            scale: 1.0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;

    /// 从实际屋顶碰撞生产、交线图到鸟巢实体的候选验收；几何由两个相交的浅斜面组成。
    #[test]
    fn roof_colliders_generate_birdnest_and_retire_it_on_removal() {
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            components::drawable::{AutoClutter, AutoClutterConvertableToDecoClutter, NaniInstanceDesc},
            geometry::instanced_wall::BrickTransformSsbo,
            render::{sparse_instance_buffer::SparseInstanceBuffer, ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice}},
            resources::{roofs::{Roof, RoofTriggerVolume, geometry::surface::Curve2}, walls::{InnerWalls, WallId, WallState,
                publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512, public_walls::PublicWalls}},
            systems::collision::{MeshIntersectionGraph, intersect_meshes::{initialize_mesh_intersection_subscriber, find_mesh_mesh_intersections_system},
                world::update_roof_rapier_colliders}};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 1, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
                Err(BackendError::invalid_usage("测试设备不执行 GPU 上传"))
            }
        }
        let mut inner = InnerWalls::new();
        for id in [42u64, 43] {
            let curve = Curve2::try_new(vec![[-4., -4.], [4., -4.], [4., 4.], [-4., 4.]], true).unwrap();
            let (mut snapshot, freehand) = WallState::new(curve, 3., 0, 0, false).into_snapshot();
            let mut shape = [0u8; 28];
            shape[..4].copy_from_slice(&1u32.to_le_bytes());
            for (i, f) in [1.0f32, 0.0, 0.0, 0.0, 8.0, 8.0].into_iter().enumerate() {
                shape[4+i*4..8+i*4].copy_from_slice(&f.to_le_bytes());
            }
            snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
            snapshot.source.curve_kind_tag = 1;
            inner.new_wall_with_id(WallId(id), WallState::from_snapshot(snapshot, freehand));
        }
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &TerrainHeightGrid512 { encoded: vec![0;512*512].into(),
            minimum_y: 0., maximum_y: 0., sampling_fix: false }, None, &Default::default(), &Default::default());
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(RaycastWorld::new());
        world.run_system_once(initialize_mesh_intersection_subscriber).unwrap();
        world.run_system_once(birdnests_subscribe_to_tabloids).unwrap();
        world.init_resource::<MeshIntersectionGraph>();
        world.init_resource::<BirdNestCandidateLocations>();
        world.init_resource::<AutoClutterState>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<Events<country_core::systems::audio::PlayClutterSoundCmd>>();
        world.insert_resource(birdnest_meshes());
        let library = AssetSsboLibrary::new();
        library.register::<BrickTransformSsbo>(SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&Device, None));
        world.insert_resource(library);
        let mut roofs = Vec::new();
        for (id, slope) in [(42u64, 0.2f32), (43, -0.2)] {
            let mut bytes = [0u8;88];
            bytes[..8].copy_from_slice(&id.to_le_bytes());
            // 原 Roof::is_flat 由 height_01 判断；平顶 tag2 的 flag=1
            // 按交线资格会被拒绝。两片浅斜面均属于非平顶输入。
            bytes[0x30..0x34].copy_from_slice(&1.0f32.to_le_bytes());
            bytes[0x4c..0x50].copy_from_slice(&3.0f32.to_le_bytes());
            let points = [glam::Vec3::new(-3., 2.-slope, -1.), glam::Vec3::new(-3., 2.+slope, 1.),
                glam::Vec3::new(3., 2.-slope, -1.), glam::Vec3::new(3., 2.+slope, 1.)];
            let mut volume = RoofTriggerVolume::default();
            volume.set_triangles(vec![[points[0], points[1], points[2]], [points[2], points[1], points[3]]]);
            volume.set_normals(vec![[glam::Vec3::Y;3];2]);
            volume.set_uvs(vec![[glam::Vec2::ZERO;3];2]);
            roofs.push(world.spawn((Roof::from_observed(bytes), volume)).id());
        }
        let mut producer = Schedule::default();
        producer.add_systems(update_roof_rapier_colliders);
        producer.run(&mut world);
        world.resource_mut::<RaycastWorld>().flush_colliders();
        world.run_system_once(find_mesh_mesh_intersections_system).unwrap();
        assert!(!world.resource::<BirdNestCandidateLocations>().0.is_empty(),
            "真实碰撞交线必须生成鸟巢候选：图边数 {}，各边候选数 {:?}",
            world.resource::<MeshIntersectionGraph>().inner().edge_count(),
            world.resource::<MeshIntersectionGraph>().inner().all_edges().map(|(_, _, edge)| edge.bird_nests.len()).collect::<Vec<_>>());
        let mut consumer = Schedule::default();
        consumer.add_systems((add_birdnests, super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        consumer.run(&mut world);
        let mut query = world.query::<(Entity, &AutoClutter, &AutoClutterConvertableToDecoClutter, &NaniInstanceDesc)>();
        let entities: Vec<_> = query.iter(&world).map(|(entity, _, _, _)| entity).collect();
        assert!(!entities.is_empty(), "真实鸟巢候选必须到达 Nani");
        for &entity in &entities { world.get_mut::<AutoClutter>(entity).unwrap().progress = 1.0; }
        world.entity_mut(roofs[0]).remove::<RoofTriggerVolume>();
        producer.run(&mut world);
        world.resource_mut::<RaycastWorld>().flush_colliders();
        world.run_system_once(find_mesh_mesh_intersections_system).unwrap();
        assert!(world.resource::<BirdNestCandidateLocations>().0.is_empty());
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        consumer.run(&mut world);
        assert!(entities.iter().all(|&entity| world.get::<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>(entity).is_some()));
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 0.3;
        world.run_system_once(super::super::autoclutter_animation::autoclutter_animation).unwrap();
        assert!(entities.into_iter().all(|entity| world.get_entity(entity).is_err()));
    }

    #[test]
    fn dirty_cell_matches_original_fma_and_saturating_cast() {
        let cases: &[([u32; 2], u32, [u32; 2])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-bird-cell-native-cases.rs"));
        for &(bits, tag, cell) in cases {
            let point = glam::Vec2::new(f32::from_bits(bits[0]), f32::from_bits(bits[1]));
            assert_eq!(birdnest_dirty_cell(point), (tag != 0).then_some(cell), "输入 {bits:?}");
        }
    }

    fn birdnest_meshes() -> country_core::resources::clutter_meshes::ClutterMeshes {
        use country_core::resources::clutter_meshes::{ClutterMesh, ClutterMeshes};
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        fixed[9].mesh = Some(country_core::asset_libraries::Handle::new());
        ClutterMeshes::from_parts(fixed, vec![])
    }

    #[test]
    fn roof_perimeter_rejection_uses_actual_height_and_strict_radius() {
        let mut bytes = [0u8; 88];
        bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
        bytes[0x4c..0x50].copy_from_slice(&1.0f32.to_le_bytes());
        let roof = country_core::resources::roofs::roof_shape::Roof::from_observed(bytes);
        assert!(roof_blocks_birdnest(&roof, glam::Vec3::new(1.8, 1.0, 0.0), 1.0));
        assert!(!roof_blocks_birdnest(&roof, glam::Vec3::new(0.1, 1.0, 0.0), 1.0));
        assert!(!roof_blocks_birdnest(&roof, glam::Vec3::new(1.8, 0.0, 0.0), 1.0));
    }

    #[test]
    fn dirty_cells_cannot_manufacture_birdnests_without_real_candidates() {
        let mut publisher = tabloid::Publisher::<DirtyTiles>::default();
        let subscriber = publisher.subscribe();
        let mut tiles = DirtyTiles::new_clear();
        tiles.set_cell(31, 27);
        publisher.publish(tiles).unwrap();
        let mut world = World::new();
        world.insert_resource(BirdNestLazySubscriber(Mutex::new(subscriber)));
        world.init_resource::<AutoClutterState>();
        world.init_resource::<BirdNestCandidateLocations>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        world.insert_resource(birdnest_meshes());
        world.run_system_once(add_birdnests).unwrap();
        assert!(world.resource::<AutoClutterState>().spawn_candidates.is_empty());
    }

    #[test]
    fn real_birdnest_candidate_reaches_manager_with_arc_and_wall() {
        use country_core::components::drawable::{AutoClutterConvertableToDecoClutter, AutoClutterDestination, AutoClutterSource};
        use country_core::systems::collision::BirdNestCandidateLocation;
        let mut publisher = tabloid::Publisher::<DirtyTiles>::default();
        let mut world = World::new();
        world.insert_resource(BirdNestLazySubscriber(Mutex::new(publisher.subscribe())));
        world.init_resource::<AutoClutterState>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<Events<country_core::systems::audio::PlayClutterSoundCmd>>();
        world.insert_resource(birdnest_meshes());
        let points: std::sync::Arc<[glam::Vec3]> = vec![glam::Vec3::ZERO, glam::Vec3::X].into();
        world.insert_resource(BirdNestCandidateLocations(vec![BirdNestCandidateLocation {
            points: points.clone(), global_id: 0x8765_4321_1234_5678, destination_wall: 99,
            unknown_20: 0, position: glam::Vec3::new(2.0, 3.0, 4.0), forward: glam::Vec3::Z,
        }]));
        let mut schedule = Schedule::default();
        schedule.add_systems((add_birdnests, super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let spawn = &world.resource::<AutoClutterState>().spawn_candidates[0];
        assert_eq!(spawn.key, 0x8765_4321_1234_5678);
        assert_eq!(spawn.position, glam::Vec3::new(2.0, 3.15, 4.0));
        assert_eq!(spawn.marker, 3);
        let mut query = world.query::<(Entity, &AutoClutterConvertableToDecoClutter)>();
        let objects: Vec<_> = query.iter(&world).collect();
        assert_eq!(objects.len(), 1);
        let entity = objects[0].0;
        assert!(matches!(&objects[0].1.source, AutoClutterSource::Points(found) if std::sync::Arc::ptr_eq(found, &points)));
        assert!(matches!(objects[0].1.destination, AutoClutterDestination::OnBuilding { wall, .. } if wall.0 == 99));
        drop(objects);
        world.get_mut::<country_core::components::drawable::AutoClutter>(entity).unwrap().progress = 1.0;
        world.resource_mut::<BirdNestCandidateLocations>().0.clear();
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        schedule.run(&mut world);
        assert!(world.get::<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>(entity).is_some());
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 0.3;
        let mut animation = Schedule::default();
        animation.add_systems(super::super::autoclutter_animation::autoclutter_animation);
        animation.run(&mut world);
        assert!(world.get_entity(entity).is_err());
        assert_eq!(std::sync::Arc::strong_count(&points), 1);
    }

    #[test]
    fn dirty_cell_rechecks_cached_real_roof_collision() {
        use country_core::systems::collision::BirdNestCandidateLocation;
        let mut publisher = tabloid::Publisher::<DirtyTiles>::default();
        let mut world = World::new();
        world.insert_resource(BirdNestLazySubscriber(Mutex::new(publisher.subscribe())));
        world.init_resource::<AutoClutterState>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        world.insert_resource(birdnest_meshes());
        world.insert_resource(BirdNestCandidateLocations(vec![BirdNestCandidateLocation {
            points: vec![glam::Vec3::ZERO].into(), global_id: 42, destination_wall: 99, unknown_20: 0,
            position: glam::Vec3::new(2.0, 3.0, 4.0), forward: glam::Vec3::Z,
        }]));
        let mut schedule = Schedule::default();
        schedule.add_systems(add_birdnests);
        schedule.run(&mut world);
        assert_eq!(world.resource::<AutoClutterState>().spawn_candidates.len(), 1);
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        let mut bytes = [0u8; 88];
        bytes[0x0c..0x10].copy_from_slice(&2.2f32.to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&4.0f32.to_le_bytes());
        bytes[0x14..0x18].copy_from_slice(&0.2f32.to_le_bytes());
        bytes[0x4c..0x50].copy_from_slice(&(3.0f32 + 0.15).to_le_bytes());
        world.spawn(country_core::resources::roofs::roof_shape::Roof::from_observed(bytes));
        schedule.run(&mut world);
        assert_eq!(world.resource::<AutoClutterState>().spawn_candidates.len(), 1, "未标脏沿用缓存");
        world.resource_mut::<AutoClutterState>().spawn_candidates.clear();
        let [x, y] = birdnest_dirty_cell(glam::Vec2::new(2.0, 4.0)).unwrap();
        let mut dirty = DirtyTiles::new_clear();
        dirty.set_cell(x as i32, y as i32);
        publisher.publish(dirty).unwrap();
        schedule.run(&mut world);
        assert!(world.resource::<AutoClutterState>().spawn_candidates.is_empty(), "标脏必须重查真实屋顶");
    }

    /// Startup 系统经 `Commands::insert_resource` 插入 `BirdNestLazySubscriber`（延迟到命令应用后才存在）。
    #[test]
    fn birdnests_subscribe_inserts_the_lazy_subscriber_resource() {
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        assert!(!world.contains_resource::<BirdNestLazySubscriber>());
        world.run_system_once(birdnests_subscribe_to_tabloids).unwrap();
        let sub = world.resource::<BirdNestLazySubscriber>();
        assert!(sub.0.lock().unwrap().take().is_none(), "订阅之前的发布不可见");
    }

    /// 缺 `RaycastWorld` 时参数校验失败，不插入资源。
    #[test]
    fn birdnests_subscribe_requires_the_host_raycast_world() {
        let mut world = World::new();
        assert!(world.run_system_once(birdnests_subscribe_to_tabloids).is_err());
        assert!(!world.contains_resource::<BirdNestLazySubscriber>());
    }
}
