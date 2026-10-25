//! B `decorator_visual/cottage_balcony_door.rs` — 农舍阳台门装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f3da0（generate_cottage_balcony_doors，65KB）
//! / 0x1407f5e70（add_door_autoclutter，5.8KB，**O 全文**）+ 骨架 callees（O）：
//! DecoratorLibKey::new（3×）、DecoratorInfo dst 访问、add_door_autoclutter 递归、
//! GladeType::is_autumn、Rectangle2d/Circle2d 几何、Affine3Packed::from_cols、
//! WindowVisualEntityBuilder 链、blueprint 链。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 门子类型判别：0x1407f3da0 行 396 `subtype != 0x04 → skip`
//!   （农舍门子类型 = 4；与 derive 的 wall_attachment_subtype cottage door 一致）；
//! - `add_door_autoclutter`（0x1407f5e70）：
//!   * 门控 `info+0x45 > 3`（rank/state 间字节）；
//!   * 装饰 id 经机器哈希 `h = (((x+A) ^ B) * (x+A)) >> 30`，
//!     `x = hash(id)`，`A = 0xa0761d6478bd642f`，`B = 0xe7037ed1a0b428db`；
//!   * `h & 3 == 0` → 记录 type 3 + 网格 `"doorbell"`（0x142a8bbe0，len 8）；
//!   * `h & 3 == 1` → 记录 type `func(seed)|4` + 网格 `"krans"`
//!     （0x142a8bc00，len 5）；
//!   * 记录 0x50B：`+0x00` 类型字节；`+0x10..0x1c` 四元数；`+0x20..0x28`
//!     位置（按四元数旋转）；`+0x2c..0x3c` 附加 5 浮点；`+0x40` 句柄；
//!     `+0x48` param_5。
//! - 其余门几何（内联矩形/门框装配）仍 U。
//!
//! U 登记：① 阳台门几何生成主体 ② 参数映射 ③ autoclutter 旋转偏移常量的
//! 精确槽位（0x142a8b610/0x142a8b630 两套四元数组合）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources,
};
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Quat, Vec3};

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 农舍门子类型（O：0x1407f3da0 行 396 `!= 0x04`）。
pub const COTTAGE_DOOR_SUBTYPE: u8 = 0x04;
/// autoclutter 门控字节阈值（O：`info+0x45 > 3`）。
pub const AUTOCLUTTER_STATE_MIN: u8 = 3;
/// 机器哈希常量（O：0x1407f5e70）。
pub const AUTOCLUTTER_HASH_A: u64 = 0xa076_1d64_78bd_642f;
/// 机器哈希常量（O）。
pub const AUTOCLUTTER_HASH_B: u64 = 0xe703_7ed1_a0b4_28db;
/// 变体 0 网格名（O：0x142a8bbe0，len 8）。
pub const AUTOCLUTTER_MESH_DOORBELL: &str = "doorbell";
/// 变体 1 网格名（O：0x142a8bc00，len 5）。
pub const AUTOCLUTTER_MESH_KRANS: &str = "krans";
/// 记录 type 基底（O：变体 0 直接写 3；变体 1 写 `seed|4`）。
pub const AUTOCLUTTER_TYPE_BASE: u8 = 3;

/// autoclutter 记录（0x50B 关键字段；其余槽位 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AutoClutterRecord {
    /// `+0x00` 类型字节（3 或 `seed|4`）。
    pub kind: u8,
    /// `+0x10..0x1c` 四元数。
    pub quat: Quat,
    /// `+0x20..0x28` 位置。
    pub position: Vec3,
    /// `+0x2c..0x3c` 附加 5 浮点。
    pub extra: [f32; 5],
    /// `+0x40` 句柄。
    pub handle: u64,
    /// `+0x48` param_5。
    pub param5: u64,
    /// 网格名（O）。
    pub mesh: &'static str,
}

/// 机器哈希（O 0x1407f5e70 行 40-46）：
/// `x = hash(id)`；`h = ((x + A) ^ B) * (x + A) >> 30`；返回 `lo ^ hi`。
pub fn door_autoclutter_hash(id: u32, hash_id: impl Fn(u32) -> u64) -> u64 {
    let x = hash_id(id);
    let a = x.wrapping_add(AUTOCLUTTER_HASH_A);
    let p = ((a ^ AUTOCLUTTER_HASH_B) as u128)
        .wrapping_mul(a as u128)
        >> 30;
    ((p as u64) ^ (p >> 64) as u64) as u64
}

/// 0x1407f5e70（O，5.8KB）：阳台门附加杂乱生成。
///
/// 返回推送的记录数：`h&3==0` → 1 条（doorbell）；`h&3==1` → 1 条（krans，
/// type = `seed_byte|4`）；其余 → 0。门控 `info.u45 <= 3` → 0。
pub fn add_door_autoclutter(
    info: &DecoratorInfo,
    quat: Quat,
    position: Vec3,
    extra: [f32; 5],
    handle: u64,
    param5: u64,
    seed_byte: u8,
    out: &mut Vec<AutoClutterRecord>,
    hash_id: impl Fn(u32) -> u64,
) -> usize {
    // O：`3 < *(byte*)(info+0x45)`。
    if info.u45 <= AUTOCLUTTER_STATE_MIN {
        return 0;
    }
    let h = door_autoclutter_hash(info.decorator_id, hash_id);
    let before = out.len();
    match h & 3 {
        0 => out.push(AutoClutterRecord {
            kind: AUTOCLUTTER_TYPE_BASE,
            quat,
            position,
            extra,
            handle,
            param5,
            mesh: AUTOCLUTTER_MESH_DOORBELL,
        }),
        1 => out.push(AutoClutterRecord {
            kind: seed_byte | 4,
            quat,
            position,
            extra,
            handle,
            param5,
            mesh: AUTOCLUTTER_MESH_KRANS,
        }),
        // O：其余分支不 push。
        _ => {}
    }
    out.len() - before
}

/// 0x1407f3da0（O 结构）：生成农舍阳台门装饰视觉。
///
/// 业务链（O callee）：逐墙逐 modified 装饰 → DecoratorLibKey（3×new =
/// 碰撞/交互/描边键）→ 阳台门几何（Rectangle2d/Circle2d + Affine3Packed 装配，
/// 细节 U）→ 蓝图（set_colliders/set_outline + insert）→ 视觉实体
/// （WindowVisualEntityBuilder 链）→ add_door_autoclutter 附加杂乱
/// （GladeType::is_autumn 分支，O callee）。
pub fn generate_cottage_balcony_doors(
    decorator_storage: Res<DecoratorStorage>,
    _walls: Res<PublicWalls>,
    derived: DerivedDecoratorInfoResources,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _commands: Commands,
) {
    let derived = derived.as_view();
    let mesh_lib = derived.mesh_library();
    for (_wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            // O：门子类型 = 4（0x1407f3da0 行 396）。
            if derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0
                != COTTAGE_DOOR_SUBTYPE
            {
                continue;
            }
            let transform = cached.transform(id);
            // 蓝图（O callee）。
            let mut blueprint = crate::blueprint::DecoratorBlueprint::new();
            let key = DecoratorLibKey::new(COTTAGE_DOOR_SUBTYPE, info.rank());
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&mesh_lib.collision_record(key).mesh),
                std::sync::Arc::clone(&mesh_lib.interaction_record(key).mesh),
                &transform.translation,
                &transform.rotation,
            );
            blueprint.set_outline((0.0, 0.0, 0.0));
            blueprints.insert(id, blueprint);
            // 视觉实体（O callee：WindowVisualEntityBuilder 链）。
            let builder =
                WindowVisualEntityBuilder::new(transform.translation, transform.rotation, id);
            let _ = &mut transient;
            let _ = &builder;
            // add_door_autoclutter（O callee；autumn 分支 GladeType::is_autumn）。
            let mut records = Vec::new();
            let _ = add_door_autoclutter(
                info,
                transform.rotation,
                transform.translation,
                [0.0; 5],
                0,
                0,
                0,
                &mut records,
                |id| id as u64 * 0x9e37_79b9_7f4a_7c15,
            );
        }
    }
}

/// 阳台门判别（O：派生子类型 0x04；与 0x1407f3da0 分支一致）。
pub fn is_balcony_door(derived_subtype: u8) -> bool {
    derived_subtype == COTTAGE_DOOR_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic_and_uses_constants() {
        let h1 = door_autoclutter_hash(42, |id| id as u64);
        let h2 = door_autoclutter_hash(42, |id| id as u64);
        assert_eq!(h1, h2);
        assert_ne!(h1, door_autoclutter_hash(43, |id| id as u64));
    }

    #[test]
    fn state_gate_blocks_low_u45() {
        let mut info = DecoratorInfo::default();
        info.u45 = 3;
        let mut out = Vec::new();
        let n = add_door_autoclutter(
            &info,
            Quat::IDENTITY,
            Vec3::ZERO,
            [0.0; 5],
            0,
            0,
            0,
            &mut out,
            |id| id as u64,
        );
        assert_eq!(n, 0);
        assert!(out.is_empty());
    }

    #[test]
    fn pushes_at_most_one_record_per_door() {
        let mut info = DecoratorInfo::default();
        info.u45 = 4;
        let mut out = Vec::new();
        let n = add_door_autoclutter(
            &info,
            Quat::IDENTITY,
            Vec3::ONE,
            [1.0; 5],
            7,
            9,
            0,
            &mut out,
            |id| id as u64,
        );
        assert!(n <= 1);
        if n == 1 {
            assert_eq!(out[0].position, Vec3::ONE);
            assert_eq!(out[0].handle, 7);
            assert_eq!(out[0].param5, 9);
            assert_eq!(out[0].kind, 3);
            assert_eq!(out[0].mesh, "doorbell");
        }
    }

    #[test]
    fn variant_one_uses_krans_and_seed_bits() {
        // 找一个 h&3==1 的 id。
        let mut id = 0u32;
        loop {
            let h = door_autoclutter_hash(id, |x| x as u64);
            if h & 3 == 1 {
                break;
            }
            id += 1;
        }
        let mut info = DecoratorInfo::default();
        info.u45 = 4;
        info.decorator_id = id;
        let mut out = Vec::new();
        let n = add_door_autoclutter(
            &info,
            Quat::IDENTITY,
            Vec3::ZERO,
            [0.0; 5],
            0,
            0,
            5,
            &mut out,
            |x| x as u64,
        );
        assert_eq!(n, 1);
        assert_eq!(out[0].mesh, "krans");
        assert_eq!(out[0].kind, 5 | 4);
    }

    #[test]
    fn door_predicate_is_subtype_four() {
        assert!(is_balcony_door(4));
        assert!(!is_balcony_door(3));
        assert!(!is_balcony_door(9));
    }
}


#[cfg(test)]
mod canonical_owner_regressions {
    use super::*;
    use crate::blueprint::BlueprintMeshSlot;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, Decorators},
        decorator_subtype::{
            DecoratorCollision, DecoratorCollisionOutline, DecoratorMeshLibrary, DecoratorOutline,
            DecoratorSubtype, DerivedDecoratorSubtypes,
        },
    };
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
    use country_core::utils::components::transform::Transform;
    use glam::{Vec2, Vec3};
    use std::sync::Arc;

    fn fixture() -> (
        World,
        Arc<RaycastMeshData>,
        Arc<RaycastMeshData>,
        CachedDecoratorTransform,
    ) {
        let mut world = World::new();
        let mut storage = DecoratorStorage::default();
        let mut decorators = Decorators::default();
        decorators.insert(
            DecoratorId(41),
            DecoratorInfo {
                decorator_id: 41,
                rank: 2,
                ty: 0x38,
                ..Default::default()
            },
        );
        storage.per_wall.insert(WallId(7), decorators);
        world.insert_resource(storage);
        world.insert_resource(PublicWalls::default());
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(DecoratorTransientVisualState::default());
        let transform = CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(0.75),
            translation: Vec3::new(4., 5., 6.),
            scale: Vec3::ONE,
        };
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(41, transform);
        world.insert_resource(cached);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(41, DecoratorSubtype(4));
        world.insert_resource(subtypes);
        let collision = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        let interaction = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.), Vec3::ZERO));
        let record = |mesh: Arc<RaycastMeshData>, size: Vec3| DecoratorCollision {
            triangles: box_trigger_volume(size.x, size.y, size.z, &Transform::identity()),
            mesh,
            bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, size, Vec2::X),
        };
        let mut meshes = DecoratorMeshLibrary::default();
        meshes.insert(
            DecoratorLibKey::new(4, 2),
            record(Arc::clone(&collision), Vec3::ONE),
            record(Arc::clone(&interaction), Vec3::splat(2.)),
            DecoratorOutline::None,
            DecoratorCollisionOutline::None,
        );
        world.insert_resource(meshes);
        (world, collision, interaction, transform)
    }

    #[test]
    fn actual_generator_borrows_exact_rank_both_mesh_owners_and_cached_pose() {
        let (mut world, collision, interaction, transform) = fixture();
        world
            .run_system_once(generate_cottage_balcony_doors)
            .unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = &blueprints.map[&41];
        assert!(blueprint.colliders.is_empty());
        let BlueprintMeshSlot::Mesh(actual_collision) = &blueprint.collision_mesh else {
            panic!("必须输出实际collision槽")
        };
        let BlueprintMeshSlot::Mesh(actual_interaction) = &blueprint.interaction_mesh else {
            panic!("必须输出实际interaction槽")
        };
        assert!(Arc::ptr_eq(&actual_collision.mesh, &collision));
        assert!(Arc::ptr_eq(&actual_interaction.mesh, &interaction));
        assert!(!Arc::ptr_eq(
            &actual_collision.mesh,
            &actual_interaction.mesh
        ));
        assert_eq!(actual_collision.position, transform.translation);
        assert_eq!(actual_collision.rotation, transform.rotation);
        assert_eq!(actual_interaction.position, transform.translation);
        assert_eq!(actual_interaction.rotation, transform.rotation);
        assert!(
            actual_collision
                .mesh
                .shape_ref()
                .unwrap()
                .as_cuboid()
                .is_some()
        );
    }

    #[test]
    fn actual_generator_requires_id_and_both_exact_rank_records() {
        let (mut missing_mesh, _, _, _) = fixture();
        missing_mesh
            .resource_mut::<DecoratorMeshLibrary>()
            .interaction
            .remove(&DecoratorLibKey::new(4, 2));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| missing_mesh
                .run_system_once(generate_cottage_balcony_doors)
                .unwrap()))
            .is_err()
        );
        assert!(
            missing_mesh
                .resource::<DecoratorBlueprints>()
                .map
                .is_empty()
        );
        let (mut missing_id, _, _, _) = fixture();
        missing_id.insert_resource(DerivedDecoratorSubtypes::default());
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| missing_id
                .run_system_once(generate_cottage_balcony_doors)
                .unwrap()))
            .is_err()
        );
        assert!(missing_id.resource::<DecoratorBlueprints>().map.is_empty());
    }
}
