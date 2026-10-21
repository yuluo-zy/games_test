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
//! 尚未完整恢复DormerHole与后续视觉载荷，不能据本source宣称完整视觉验收。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastMeshData;
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::{Circle2d, Rectangle2d};

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
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

/// B1407be730：恢复真实flat/边界门与动态collider；后续洞/视觉载荷仍待完整恢复。
pub fn generate_trap_doors(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    derived: Res<DerivedDecoratorSubtypes>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    for (wall, decorators) in decorator_storage.iter_walls() {
        let Some(roof_entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(roof) = roofs.get(roof_entity) else {
            continue;
        };
        // 原bec64是JE跳过，只处理flat；与其它老虎窗的相反门不能混用。
        if !roof.is_flat() {
            continue;
        }
        for (id, info) in decorators.iter_modified() {
            let subtype = derived.get(id).0;
            let Some(geometry) = trap_door_geometry(subtype) else {
                continue;
            };
            let transform = cached_transforms.transform(id);
            if !trap_door_enclosure_accepts(&transform, roof) {
                continue;
            }
            // 原bef7d先取得真实entry，后续构造失败保留source/flags部分mutation。
            let blueprint =
                blueprints.insert_for(id, DecoratorStorageAddr::Wall(wall), info.state());
            let data = RaycastMeshData::new_cuboid_at(
                Vec3::new(TRAP_DOOR_CUBOID_A, 0.5, TRAP_DOOR_CUBOID_A),
                Vec3::ZERO,
            );
            // 原dataClone新建outer Arc，内部SharedShape仍同owner，不重建第二primitive。
            let collision = Arc::new(data.clone());
            let interaction = Arc::new(data);
            blueprint.set_positioned_colliders(
                collision,
                interaction,
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_cached_transform(transform);
            // 下面保留旧视觉接口；零洞载荷/旧outline尚未接完整原业务，不能计作已恢复。
            let _hole =
                crate::utils::DormerHoleSsbo::cottage([0; 16], id, [0; 16], [0.0, 0.0], 0.0);
            blueprint.set_outline((0.0, 0.0, 0.0));
            let mut builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            builder.mesh_name = geometry.mesh;
            builder.nani_name = geometry.nani;
            let _ = &mut transient;
            let _ = &builder;
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
            state: 1,
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
        world.insert_resource(DecoratorTransientVisualState::default());
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
        assert_eq!(blueprint.interaction_source_flags, 1);
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
                RaycastGeoSource::decorator_interaction(id, addr, 1),
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
}
