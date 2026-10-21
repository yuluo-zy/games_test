//! B `decorator_visual/chimney_roof.rs` — 屋顶烟囱装饰视觉生成（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407c0910（generate_roof_chimneys，32KB）+ 骨架
//! callees（O）：DecoratorStorage::iter（0x140a1e5b0）、DecoratorStorageAddr
//! （0x140a1c2a0）、Decorators::iter_modified（0x140a1b070）、
//! DerivedDecoratorInfo::subtype（0x140b26970）、impl$8::from（0x140b26a70）、
//! DecoratorExtraInfo::as_chimney（0x1408392d0）、RaycastMeshData::new_cuboid、
//! PublicWallState::max_y、CachedDecoratorTransforms::transform、
//! blueprint {set_colliders, insert}。
//!
//! 2026-10-19 复核的新事实（O）：
//! - 屋顶烟囱子类型 = **0x15**（0x1407c0910 行 501 `subtype != 0x15 → skip`）；
//! - 烟囱高 = `as_chimney().f + 0.1`（0x142925984 = 0.1；行 519
//!   `fVar51 = *pfVar23 + fVar10`）；
//! - 立方体尺寸两轴 = 0.85（0x3f59999a；行 615-622 写入 new_cuboid 的
//!   `{fVar51, 0x3f59999a, 0x3f59999a}`）；
//! - 视觉 nani 名 `"nani_smoke"`（0x142c21224，len 10），Smoke 载荷
//!   `(id, 0, ?)` 与 `as_chimney().smoke`（字节 +4，行 520/687）；
//! - 蓝图 `+0x120`（0x28B=10×u32）烟囱参数块：`{2, …}`（行 690-710）。
//!
//! U 登记：① 烟囱高/位置几何的其余槽位 ② 参数排序 ③ 蓝图 +0x120 参数块
//! 各字段语义。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};

/// 屋顶烟囱子类型（O：0x1407c0910 行 501）。
pub const ROOF_CHIMNEY_SUBTYPE: u8 = 0x15;
/// 烟囱高附加（O：0x142925984 = 0.1）。
pub const CHIMNEY_HEIGHT_EXTRA: f32 = 0.1;
/// 立方体截面/高（O：0x3f59999a = 0.85）。
pub const CHIMNEY_CUBOID_SIZE: f32 = 0.85;
/// 视觉 nani 名（O：0x142c21224，len 10）。
pub const CHIMNEY_NANI_SMOKE: &str = "nani_smoke";

/// 屋顶烟囱几何参数（O 公式）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofChimney {
    /// 立方体尺寸（两轴 0.85，高度轴另计）。
    pub cuboid: [f32; 3],
    /// 烟囱总高 = extra.f + 0.1。
    pub height: f32,
    /// 烟雾字节（`ChimneyExtra::smoke`，O：+4 字段）。
    pub smoke: u8,
}

/// 由 `ChimneyExtra` 计算屋顶烟囱几何（O）。
pub fn roof_chimney_geometry(extra_height: f32, smoke: u8) -> RoofChimney {
    RoofChimney {
        cuboid: [
            CHIMNEY_CUBOID_SIZE,
            extra_height + CHIMNEY_HEIGHT_EXTRA,
            CHIMNEY_CUBOID_SIZE,
        ],
        height: extra_height + CHIMNEY_HEIGHT_EXTRA,
        smoke,
    }
}

/// 0x1407c0910（O 结构）：生成屋顶烟囱装饰视觉。
pub fn generate_roof_chimneys(
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    _walls: Res<PublicWalls>,
    derived: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    let derived = derived.as_view();
    for (wall, decorators) in decorator_storage.iter() {
        for (id, info) in decorators.iter_modified() {
            // 原112a先required读子类型；完整原body没有额外目的地tag前门。
            if derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0
                != ROOF_CHIMNEY_SUBTYPE
            {
                continue;
            }
            let transform = cached_transforms.transform(id);
            let blueprint = blueprints.insert_for(
                id,
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(wall),
                info.state(),
            );
            let extra = info.extra().as_chimney();
            let chimney = roof_chimney_geometry(extra.f, extra.smoke);
            // 原局部cuboid中心为零，世界中心沿Y抬高半个实际烟囱高度。
            let collider = std::sync::Arc::new(
                country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(
                    glam::Vec3::from_array(chimney.cuboid),
                    glam::Vec3::ZERO,
                ),
            );
            let position = glam::Vec3::new(
                transform.translation.x,
                transform.translation.y + chimney.height * 0.5,
                transform.translation.z,
            );

            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&collider),
                collider,
                &position,
                &transform.rotation,
            );
            blueprint.add_chimney(chimney.height);
        }
    }
}

/// 判别（O：dst=屋顶 且 subtype=0x15）。
pub fn is_roof_chimney(dst_tag: u8, subtype: u8) -> bool {
    dst_tag == 2 && subtype == ROOF_CHIMNEY_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_matches_machine_constants() {
        let c = roof_chimney_geometry(1.4, 3);
        assert!((c.height - 1.5).abs() < 1e-6);
        assert_eq!(c.cuboid, [0.85, 1.5, 0.85]);
        assert_eq!(c.smoke, 3);
    }

    #[test]
    fn predicate_matches_decomp() {
        assert!(is_roof_chimney(2, 0x15));
        assert!(!is_roof_chimney(2, 0x13));
        assert!(!is_roof_chimney(0, 0x15));
    }
}


#[cfg(test)]
mod canonical_roof_collider_regressions {
    use super::*;
    use crate::blueprint::BlueprintMeshSlot;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{ChimneyExtra, DecoratorExtraInfo, DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, Decorators},
        decorator_subtype::{DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes},
    };
    use glam::{Quat, Vec3};
    use std::sync::Arc;

    #[test]
    fn roof_chimney_uses_native_y_height_zero_local_center_and_shared_arc() {
        let mut world = World::new();
        let mut info = DecoratorInfo {
            decorator_id: 9,
            u20: 2 << 16,
            rank: 1,
            state: 1,
            ..Default::default()
        };
        info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
            f: 1.4,
            smoke: 3,
        }));
        let mut bucket = Decorators::default();
        bucket.insert(DecoratorId(9), info);
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(2), bucket);
        world.insert_resource(storage);
        let transform = CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(0.4),
            translation: Vec3::new(4., 7., -3.),
            scale: Vec3::ONE,
        };
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(9, transform);
        world.insert_resource(cached);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(9, DecoratorSubtype(21));
        world.insert_resource(subtypes);
        // 动态烟囱不读取loader的球体记录，空库仍必须生成真实cuboid。
        world.insert_resource(DecoratorMeshLibrary::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(DecoratorBlueprints::default());
        world.run_system_once(generate_roof_chimneys).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = &blueprints.map[&9];
        assert_eq!(blueprint.storage_addr, Some(country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(WallId(2))));
        assert_eq!(blueprint.interaction_source_flags, 1);
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
            panic!("实际烟囱collision")
        };
        let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
            panic!("实际烟囱interaction")
        };
        assert!(Arc::ptr_eq(&collision.mesh, &interaction.mesh));
        assert_eq!(
            collision
                .mesh
                .shape_ref()
                .unwrap()
                .as_cuboid()
                .unwrap()
                .half_extents
                .as_slice(),
            &[0.425, 0.75, 0.425]
        );
        assert_eq!(collision.mesh.local_transform().translation, Vec3::ZERO);
        assert_eq!(collision.position, Vec3::new(4., 7.75, -3.));
        assert_eq!(collision.rotation, transform.rotation);
        assert_eq!(interaction.position, collision.position);
        assert_eq!(interaction.rotation, collision.rotation);
        assert!(blueprint.colliders.is_empty());
    }
}
