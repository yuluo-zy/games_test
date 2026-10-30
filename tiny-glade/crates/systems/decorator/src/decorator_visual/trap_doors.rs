//! B `decorator_visual/trap_doors.rs` — 活板门装饰视觉生成。
//!
//! 证据：同B原字节 0x1407be730（generate_trap_doors，完整7587B）+ 骨架
//! callees（O）：iter_walls / iter_modified、Roof::is_flat、
//! PublicWalls::roof_entity、CachedDecoratorTransforms::transform、
//! DormerHoleSsbo::cottage、RaycastMeshData::new_cuboid、
//! Rectangle2d::contains_point_padding、Circle2d::intersect_circle、
//! DecoratorBlueprint::{set_colliders,set_outline} + DecoratorBlueprints::insert、
//! WindowVisualEntityBuilder::{new,custom_shader}、ColorId eq。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 活板门子类型 ∈ **{0x05, 0x0a}**（0x1407be730 行 424：
//!   `subtype != 0x0a && subtype != 0x05 → skip`）；
//! - 两套网格名（0x142a85fa8/0x142a85fc4）：`"decorators/trap_door_cottage"`、
//!   `"decorators/trap_door_gothic"`；视觉 nani 名 `"nani_balcony_door"`
//!   （0x142a85fdf，len 17）；
//! - 立方体尺寸常量 1.3（0x3fa66666，行 454）与 1.125（0x3f900000，行 764）。
//!
//! 本轮原字节复核：flat-only、enclosure交点None才允许、动态cuboid完整尺寸
//! (1.3,.5,1.3)，dataClone形成不同outer/sameinner Arc，真实entry/source/pose。
//! 已恢复完整builder、颜色/透明标记及canonical DormerHole输出；非空World经
//! fullvisual实体与碰撞回收测试通过，真实GPU场景验收由宿主另行完成。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources;
#[cfg(test)]
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastMeshData;
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::{Circle2d, Rectangle2d};

use crate::blueprint::DecoratorBlueprints;

use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 农舍活板门子类型（O：行 424）。
pub const TRAP_DOOR_SUBTYPE_COTTAGE: u8 = 0x05;
/// 哥特活板门子类型（O：行 424）。
pub const TRAP_DOOR_SUBTYPE_GOTHIC: u8 = 0x0a;
/// 原动态cuboid的完整X/Z尺寸（bits3fa66666）。
pub const TRAP_DOOR_CUBOID_A: f32 = 1.3;
/// 原后续DormerHole的1.125常量；不是collider的Y尺寸。
pub const TRAP_DOOR_CUBOID_B: f32 = 1.125;
/// 农舍活板门网格名（O：0x142a85fa8）。
pub const TRAP_DOOR_MESH_COTTAGE: &str = "decorators/trap_door_cottage";
/// 哥特活板门网格名（O：0x142a85fc4）。
pub const TRAP_DOOR_MESH_GOTHIC: &str = "decorators/trap_door_gothic";
/// 视觉 nani 名（O：0x142a85fdf，len 17）。
pub const TRAP_DOOR_NANI: &str = "nani_balcony_door";

/// 活板门子类型判别（O：{5, 10}）。
pub fn is_trap_door_subtype(subtype: u8) -> bool {
    subtype == TRAP_DOOR_SUBTYPE_COTTAGE || subtype == TRAP_DOOR_SUBTYPE_GOTHIC
}

/// 子类型 → 网格名（O）。
pub fn trap_door_mesh(subtype: u8) -> Option<&'static str> {
    match subtype {
        TRAP_DOOR_SUBTYPE_COTTAGE => Some(TRAP_DOOR_MESH_COTTAGE),
        TRAP_DOOR_SUBTYPE_GOTHIC => Some(TRAP_DOOR_MESH_GOTHIC),
        _ => None,
    }
}

/// 保留既有视觉参数接口；真实collider完整尺寸为(1.3,.5,1.3)。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrapDoorGeometry {
    pub mesh: &'static str,
    pub nani: &'static str,
    pub cuboid_a: f32,
    pub cuboid_b: f32,
}

/// 由子类型计算活板门几何（O）。
pub fn trap_door_geometry(subtype: u8) -> Option<TrapDoorGeometry> {
    trap_door_mesh(subtype).map(|mesh| TrapDoorGeometry {
        mesh,
        nani: TRAP_DOOR_NANI,
        cuboid_a: TRAP_DOOR_CUBOID_A,
        cuboid_b: TRAP_DOOR_CUBOID_B,
    })
}

/// 原bee76..bef30使用原Roof enclosure本体，不能代换为剖面或bbox。
fn trap_door_enclosure_accepts(transform: &CachedDecoratorTransform, roof: &Roof) -> bool {
    let bytes = roof.observed();
    let read = |offset: usize| {
        f32::from_le_bytes(bytes[offset..offset + 4].try_into().expect("固定Roof字段"))
    };
    let point = Vec2::new(transform.translation.x, transform.translation.z);
    if bytes[8] & 1 != 0 {
        // 原直接复制Rectangle2d，不额外调用会检查axis/尺寸的构造器。
        let rectangle = Rectangle2d {
            axis: Vec2::new(read(0x0c), read(0x10)),
            center: Vec2::new(read(0x14), read(0x18)),
            size: Vec2::new(read(0x1c), read(0x20)),
        };
        rectangle.contains_point_padding(point, Vec2::splat(f32::from_bits(0xbfee147a)))
    } else {
        let circle = Circle2d {
            center: Vec2::new(read(0x0c), read(0x10)),
            radius: read(0x14),
            rotation: read(0x18),
        };
        let probe = Circle2d::new_rotated(point, f32::from_bits(0x3f6e147a), circle.rotation);
        // 原bef21比较tag0、bef30 JNE跳过：交点Some拒绝，None才允许。
        circle.intersect_circle(probe).is_none()
    }
}

/// 原 B1407be730：按flat/enclosure门生成真实碰撞、描边、视觉builder和屋顶洞。
pub fn generate_trap_doors(
    derived_resources: DerivedDecoratorInfoResources,
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    _signifiers: Res<country_core::systems::ui::signifiers::UiSignifierStream>,
) {
    let derived = derived_resources.as_view();
    for (wall, decorators) in decorator_storage.iter_walls() {
        let Some(roof_entity) = walls.roof_entity(wall) else { continue; };
        let Ok(roof) = roofs.get(roof_entity) else { continue; };
        if !roof.is_flat() { continue; }
        for (id, info) in decorators.iter_modified() {
            let subtype = derived.subtype(country_core::resources::walls::decorator::DecoratorId(id)).0;
            let Some(geometry) = trap_door_geometry(subtype) else { continue; };
            let transform = cached_transforms.transform(id);
            if !trap_door_enclosure_accepts(&transform, roof) { continue; }
            // 原bef7d先取得真实entry；后续失败仍保留source/flags的已发生修改。
            let blueprint = blueprints.insert_for(id, DecoratorStorageAddr::Wall(wall), info.state());
            let data = RaycastMeshData::new_cuboid_at(
                Vec3::new(TRAP_DOOR_CUBOID_A, 0.5, TRAP_DOOR_CUBOID_A), Vec3::ZERO,
            );
            blueprint.set_positioned_colliders(Arc::new(data.clone()), Arc::new(data),
                &transform.translation, &transform.rotation);
            blueprint.set_cached_transform(transform);
            let mut builder = WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            builder.scale = transform.scale;
            builder.mesh_name = geometry.mesh;
            builder.nani_name = geometry.nani;
            // 原Info+28的999是未指定颜色哨兵，转真实ColorId0；不读独立map key。
            builder.color_id = Some(country_core::resources::color_id::ColorId(
                if info.u28 == 999 { 0 } else { info.u28 },
            ));
            builder.semi_transparent = info.state() != 0;
            blueprint.visual_entities.push(builder);
            let q = transform.rotation;
            // 原旋转公式保留非单位四元数语义，不归一化，也不换成假定单位的叉积简式。
            let length_sq = q.z * q.z + (q.y * q.y + q.x * q.x);
            let dot = q.z * 1.0 + (q.y * 0.0 + q.x * 0.0);
            let twice_dot = dot + dot;
            let twice_w = q.w + q.w;
            let forward_x = twice_w * (q.y * 1.0 - q.z * 0.0)
                + (twice_dot * q.x + (q.w * q.w - length_sq) * 0.0);
            let forward_z = twice_w * (q.x * 0.0 - q.y * 0.0)
                + (twice_dot * q.z + (q.w * q.w - length_sq) * 1.0);
            blueprint.dormer_holes.push(country_core::resources::walls::dormers::DormerHoleSsbo::cottage(
                wall.0 as u32, transform.translation, Vec3::new(1.125, 0.375, 1.125),
                [forward_x, forward_z],
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtype_predicate_matches_decomp() {
        assert!(is_trap_door_subtype(0x05));
        assert!(is_trap_door_subtype(0x0a));
        assert!(!is_trap_door_subtype(0x04));
        assert!(!is_trap_door_subtype(0x09));
        assert!(!is_trap_door_subtype(0x0b));
    }

    #[test]
    fn mesh_names_match_machine_strings() {
        assert_eq!(
            trap_door_mesh(0x05),
            Some("decorators/trap_door_cottage")
        );
        assert_eq!(trap_door_mesh(0x0a), Some("decorators/trap_door_gothic"));
        assert_eq!(trap_door_mesh(0x09), None);
    }

    #[test]
    fn geometry_constants() {
        let g = trap_door_geometry(0x0a).unwrap();
        assert_eq!(g.cuboid_a, 1.3);
        assert_eq!(g.cuboid_b, 1.125);
        assert_eq!(g.nani, "nani_balcony_door");
    }
}

#[cfg(test)]
mod canonical_trap_dynamic_tests {
    use super::*;
    use crate::blueprint::{
        BlueprintMeshSlot, PrevDecoratorBlueprints, instantiate_decorator_colliders,
    };
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorExtraInfo, DecoratorId, DecoratorInfo, WindowExtra},
        decorator_collision::DecoratorPhysicsColliders,
        decorator_storage::{DecoratorStorageAddr, Decorators},
        decorator_subtype::DecoratorSubtype,
    };
    use country_core::systems::collision::world::{RaycastGeoSource, RaycastWorld};
    use glam::{Quat, Vec3};
    use std::{
        panic::{AssertUnwindSafe, catch_unwind},
        sync::Arc,
    };

    fn roof(rectangle: bool, height01: f32, radius: f32) -> Roof {
        let mut bytes = [0u8; 88];
        bytes[..8].copy_from_slice(&7u64.to_le_bytes());
        bytes[8..12].copy_from_slice(&u32::from(rectangle).to_le_bytes());
        let fields = if rectangle {
            [
                (0x0c, 1.0f32),
                (0x10, 0.0),
                (0x14, 0.0),
                (0x18, 0.0),
                (0x1c, 4.0),
                (0x20, 4.0),
            ]
        } else {
            [
                (0x0c, 0.0f32),
                (0x10, 0.0),
                (0x14, radius),
                (0x18, 0.0),
                (0x1c, 0.0),
                (0x20, 0.0),
            ]
        };
        for (offset, value) in fields.into_iter().chain([(0x30, height01)]) {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(bytes)
    }

    fn fixture(roof: Roof, position: Vec3) -> (World, Entity, CachedDecoratorTransform) {
        let mut world = World::new();
        let entity = world.spawn(roof).id();
        let mut walls = PublicWalls::default();
        walls.associate_roof(WallId(7), entity);
        world.insert_resource(walls);
        let mut info = DecoratorInfo {
            decorator_id: 41,
            ty: 0x38,
            rank: 2,
            state: 0,
            u20: 2 << 16,
            ..Default::default()
        };
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0)));
        let mut bucket = Decorators::default();
        bucket.insert(DecoratorId(41), info);
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(7), bucket);
        world.insert_resource(storage);
        let mut derived = DerivedDecoratorSubtypes::default();
        derived.insert(41, DecoratorSubtype(5));
        world.insert_resource(derived);
        let transform = CachedDecoratorTransform {
            translation: position,
            rotation: Quat::from_rotation_y(0.5),
            scale: Vec3::new(2.0, 3.0, 4.0),
        };
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(41, transform);
        world.insert_resource(cached);
        world.insert_resource(DecoratorBlueprints::default());
        world.init_resource::<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>();
        world.init_resource::<country_core::systems::ui::signifiers::UiSignifierStream>();
        // 不注册MeshLibrary：原trap只用动态primitive及真实ID owner。
        (world, entity, transform)
    }

    #[test]
    fn actual_trap_system_clones_data_into_two_outer_owners_and_reaches_live_physics() {
        let (mut world, _, transform) = fixture(roof(true, 0.0, 3.0), Vec3::new(0.0, 2.0, 0.0));
        world.run_system_once(generate_trap_doors).unwrap();
        let current = world.resource::<DecoratorBlueprints>();
        let blueprint = current.get(41).unwrap();
        assert_eq!(
            blueprint.storage_addr,
            Some(DecoratorStorageAddr::Wall(WallId(7)))
        );
        assert_eq!(blueprint.interaction_source_flags, 0);
        assert_eq!(blueprint.u138, 0);
        assert_eq!(blueprint.transform, Some(transform));
        assert!(blueprint.colliders.is_empty());
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
            panic!("实际碰撞槽")
        };
        let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
            panic!("实际交互槽")
        };
        assert!(!Arc::ptr_eq(&collision.mesh, &interaction.mesh));
        assert!(std::ptr::eq(
            collision.mesh.shape_ref().unwrap(),
            interaction.mesh.shape_ref().unwrap()
        ));
        assert_eq!(collision.mesh.hash, interaction.mesh.hash);
        let cuboid = collision.mesh.shape_ref().unwrap().as_cuboid().unwrap();
        assert_eq!(
            [
                cuboid.half_extents.x,
                cuboid.half_extents.y,
                cuboid.half_extents.z
            ],
            [0.65, 0.25, 0.65]
        );
        assert_eq!(collision.position, transform.translation);
        assert_eq!(collision.rotation, transform.rotation);
        let mut physics = RaycastWorld::new();
        let mut maps = DecoratorPhysicsColliders::default();
        instantiate_decorator_colliders(
            &mut maps,
            current,
            &PrevDecoratorBlueprints::default(),
            &mut physics,
        );
        let id = DecoratorId(41);
        let addr = DecoratorStorageAddr::Wall(WallId(7));
        for (instance, mesh, source) in [
            (
                &maps.first[&id],
                &collision.mesh,
                RaycastGeoSource::decorator(id, addr),
            ),
            (
                &maps.second[&id],
                &interaction.mesh,
                RaycastGeoSource::decorator_interaction(id, addr, 0),
            ),
        ] {
            let registered =
                physics.get_collider_mesh(instance.registry_index, instance.generation);
            assert!(Arc::ptr_eq(&registered.data, mesh));
            assert_eq!(registered.source, source);
            let actual = physics.get_collider(instance.handle());
            assert_eq!(actual.position, transform.translation);
            assert_eq!(actual.rotation, transform.rotation);
        }
    }

    #[test]
    fn actual_roof_and_subtype_gates_precede_missing_id_or_cache_failures() {
        for missing_component in [false, true] {
            let (mut world, entity, _) = fixture(roof(true, 0.5, 3.0), Vec3::ZERO);
            world.insert_resource(DerivedDecoratorSubtypes::default());
            world.insert_resource(CachedDecoratorTransforms::default());
            if missing_component {
                world.entity_mut(entity).remove::<Roof>();
            }
            world.run_system_once(generate_trap_doors).unwrap();
            assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        }
        let (mut world, _, _) = fixture(roof(true, 0.0, 3.0), Vec3::ZERO);
        world.insert_resource(DerivedDecoratorSubtypes::default());
        assert!(
            catch_unwind(AssertUnwindSafe(|| world
                .run_system_once(generate_trap_doors)
                .unwrap()))
            .is_err()
        );
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        let (mut world, _, _) = fixture(roof(true, 0.0, 3.0), Vec3::ZERO);
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(41, DecoratorSubtype(4));
        world.insert_resource(CachedDecoratorTransforms::default());
        world.run_system_once(generate_trap_doors).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(41, DecoratorSubtype(10));
        assert!(
            catch_unwind(AssertUnwindSafe(|| world
                .run_system_once(generate_trap_doors)
                .unwrap()))
            .is_err()
        );
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
    }

    #[test]
    fn actual_enclosure_gates_keep_original_padding_and_circle_option_polarity() {
        for (enclosure, position, accepted) in [
            (roof(true, 0.0, 3.0), Vec3::new(1.07, 0.0, 0.0), true),
            (
                roof(true, 0.0, 3.0),
                Vec3::new(f32::from_bits(1.07f32.to_bits() + 1), 0.0, 0.0),
                false,
            ),
            (roof(false, 0.0, 3.0), Vec3::ZERO, true),
            (roof(false, 0.0, 3.0), Vec3::new(2.5, 0.0, 0.0), false),
            (
                roof(false, 0.0, f32::from_bits(0x3f6e147a)),
                Vec3::ZERO,
                false,
            ),
        ] {
            let (mut world, _, _) = fixture(enclosure, position);
            world.run_system_once(generate_trap_doors).unwrap();
            assert_eq!(
                world.resource::<DecoratorBlueprints>().get(41).is_some(),
                accepted
            );
        }
    }
    fn install_fullvisual(world: &mut World) {
        world.init_resource::<crate::blueprint::PrevDecoratorBlueprints>();
        world.init_resource::<crate::decorator_visual_state::DecoratorVisualState>();
        world.init_resource::<country_core::resources::walls::wall_holes::WallHoles>();
        world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
        world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<country_core::systems::collision::WindowAutoClutterCandidates>();
        world.init_resource::<crate::decorator_visual::deco_differential_state::WallAttachedDecoTracker<crate::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>>();
        world.init_resource::<crate::decorator_visual::deco_differential_state::WallAttachedDecoTracker<crate::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>>();
        world.insert_resource(country_core::asset_libraries::mesh_library::AssetMeshLibrary::new());
        world.insert_resource(country_core::asset_libraries::shader_library::SharedAssetShaderLibrary(std::sync::Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new())));
        // CPU descriptor fixture，未创建真实GPU资源或验证画面。
        world.insert_resource(country_core::asset_libraries::texture_library::AssetTextureLibrary::new(&country_core::render::test_device::TestDevice(0)));
        world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();
    }

    #[test]
    fn trap_builder_and_nonzero_hole_reach_actual_fullvisual_and_entity_reclamation() {
        for (subtype, color) in [(5, 999), (10, 123)] {
            let (mut world, _, transform) = fixture(roof(true, 0.0, 3.0), Vec3::new(0.0, 2.0, 0.0));
            world.resource_mut::<DerivedDecoratorSubtypes>().insert(41, DecoratorSubtype(subtype));
            world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(7)).unwrap()
                .get_mut(DecoratorId(41)).unwrap().u28 = color;
            world.run_system_once(generate_trap_doors).unwrap();
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            assert!(blueprint.outline.is_none(), "不写未消费的legacy outline");
            let builder = &blueprint.visual_entities[0];
            assert_eq!(builder.mesh_name, trap_door_mesh(subtype).unwrap());
            assert_eq!(builder.nani_name, TRAP_DOOR_NANI);
            assert_eq!(builder.scale, transform.scale);
            assert_eq!(builder.color_id, Some(country_core::resources::color_id::ColorId(if color == 999 { 0 } else { color })));
            assert!(!builder.semi_transparent);
            assert!(!builder.balcony_door, "nani类别不代表原条件组件标志");
            let hole = blueprint.dormer_holes[0];
            assert_eq!(hole.0[..8], [0.0f32.to_bits(), 2.0f32.to_bits(), 0.0f32.to_bits(), 7, 1.125f32.to_bits(), 0.375f32.to_bits(), 1.125f32.to_bits(), 0]);
            assert_eq!(hole.0[10..], [1.0f32.to_bits(), (-50.0f32).to_bits()]);
            assert!(f32::from_bits(hole.0[8]).abs() > 0.4);
            let crate::blueprint::BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else { panic!("真实collider"); };
            let weak = Arc::downgrade(&collision.mesh);
            install_fullvisual(&mut world);
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            let state = world.resource::<crate::decorator_visual_state::DecoratorVisualState>();
            let entity = state.persistent_entities_of(41).unwrap()[0];
            assert!(state.persistent.get(41).unwrap().is_sealed());
            assert_eq!(world.get::<country_core::render::nani::systems::NaniInstanceDesc>(entity).unwrap().mesh,
                country_core::render::nani::systems::NaniMeshSelector::Named(country_core::name_hash::NameHash::compute(trap_door_mesh(subtype).unwrap())));
            assert!(world.get::<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData>>(entity).is_some());
            assert_eq!(world.get::<country_core::utils::components::transform::Transform>(entity).unwrap().scale, transform.scale);
            assert_eq!(world.get::<country_core::resources::color_id::ColorId>(entity).unwrap().0, if color == 999 { 0 } else { color });
            let mut previous = crate::blueprint::PrevDecoratorBlueprints::default();
            world.resource::<DecoratorBlueprints>().copy_blueprints_to_prev(&mut previous);
            world.insert_resource(previous);
            // 同一蓝图下一帧不重复spawn，移除后真实fullvisual销毁原entity和两个collider。
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            assert_eq!(world.resource::<crate::decorator_visual_state::DecoratorVisualState>().persistent_entities_of(41), Some([entity].as_slice()));
            world.resource_mut::<DecoratorBlueprints>().map.clear();
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            assert!(world.get_entity(entity).is_err());
            assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
            assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
            *world.resource_mut::<crate::blueprint::PrevDecoratorBlueprints>() = Default::default();
            assert!(weak.upgrade().is_none());
        }
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-trap-forward-native-vectors.rs"));

    #[test]
    fn trap_hole_forward_matches_64_original_sse_vectors_including_nonunit_quaternions() {
        for &(quaternion, forward) in TRAP_FORWARD_NATIVE {
            let (mut world, _, mut transform) = fixture(roof(true, 0.0, 3.0), Vec3::ZERO);
            transform.rotation = glam::Quat::from_array(quaternion.map(f32::from_bits));
            world.resource_mut::<CachedDecoratorTransforms>().insert(41, transform);
            world.run_system_once(generate_trap_doors).unwrap();
            assert_eq!(world.resource::<DecoratorBlueprints>().get(41).unwrap().dormer_holes[0].0[8..10], forward, "原quat bits {quaternion:?}");
        }
    }

    #[test]
    fn trap_preview_keeps_real_visual_but_original_skip_byte_prevents_both_physics_upserts() {
        let (mut world, _, _) = fixture(roof(true, 0.0, 3.0), Vec3::ZERO);
        world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(7)).unwrap().get_mut(DecoratorId(41)).unwrap().state = 1;
        world.run_system_once(generate_trap_doors).unwrap();
        let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!(blueprint.u138, 1, "原insert第五参写Arc+138/data+128");
        assert_eq!(blueprint.interaction_source_flags, 0, "不能把state混进Arc+139/data+129");
        assert!(blueprint.visual_entities[0].semi_transparent);
        install_fullvisual(&mut world);
        world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
        assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
        assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
        let entity = world.resource::<crate::decorator_visual_state::DecoratorVisualState>().persistent_entities_of(41).unwrap()[0];
        assert!(world.get::<country_core::components::SemiTransparent>(entity).is_some());
    }

}
