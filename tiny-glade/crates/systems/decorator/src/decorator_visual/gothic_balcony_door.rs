//! 原B1407f0900哥特阳台门；门阶1407f3340复用已经逐指令核验的采样叶。
use super::{
    cottage_balcony_door::{add_door_autoclutter, needs_bottom_rail, rank_index},
    cottage_wall_window::window_builder,
    door_stairs_assemble_bricks::DoorStairsAssemblyParams,
    gothic_wall_window::{brick_color, get_gothic_hat_bones},
};
use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        color_id::ColorId,
        session::ActiveSession,
        terrain_height::TerrainHeightsData,
        walls::{
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator::DecoratorId,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            inner_wall_state::WallStyle,
            public_walls::PublicWalls,
            wall_holes::RawWallHole,
        },
        world_raster::RasterBufferRef,
        world_raster_defs::WaterRaster,
    },
    utils::components::transform::Transform,
};
use glam::{Quat, Vec3};
use utils::geometry::Rectangle3d;
#[path = "door_stairs_geometry.rs"]
mod door_stairs_geometry;
pub use door_stairs_geometry::sample_door_stairs;
pub const GOTHIC_DOOR_SUBTYPE: u8 = 9;
const DOORS: [&str; 3] = [
    "decorators/balcony_door_gothic_rank1",
    "decorators/balcony_door_gothic_rank2",
    "decorators/balcony_door_gothic_rank3",
];
const HATS: [&str; 3] = [
    "decorators/balcony_door_gothic_rank1_hat",
    "decorators/balcony_door_gothic_rank2_hat",
    "decorators/balcony_door_gothic_rank3_hat",
];
const RAILS: [&str; 3] = [
    "decorators/gothic_balcony_door_rank1_rails",
    "decorators/gothic_balcony_door_rank2_rails",
    "decorators/gothic_balcony_door_rank3_rails",
];
// 原rank表依次为三个洞的y偏移、宽、高；用原位型避免十进制转写漂移。
const HOLES: [[u32; 9]; 3] = [
    [
        0xbe4ccccd, 0x3f90a3d7, 0x4023d70a, 0x3f91eb85, 0x3f4f5c29, 0x3ecccccd, 0x3fab851f,
        0x3ed1eb85, 0x3ecccccd,
    ],
    [
        0xbed1eb85, 0x3fb33333, 0x40147ae1, 0x3f7ae148, 0x3fa7ae14, 0x3ef5c28f, 0x3fab851f,
        0x3f4ccccd, 0x3ea3d70a,
    ],
    [
        0xbea8f5c3, 0x40028f5c, 0x4028f5c3, 0x3f933333, 0x3fcf5c29, 0x3ec7ae14, 0x3fb5c28f,
        0x3f800000, 0x3eeb851f,
    ],
];

pub fn check_for_door_stairs(
    terrain: &TerrainHeightsData,
    water: &WaterRaster,
    transform: &CachedDecoratorTransform,
    bounds: &Rectangle3d,
    blueprint: &mut DecoratorBlueprint,
) -> bool {
    let water_grid = RasterBufferRef::<512>::new(water.current_buffer());
    let Some(words) = sample_door_stairs(
        transform.rotation,
        transform.translation,
        bounds.footprint.size.x,
        bounds.bottom,
        bounds.top,
        |point| terrain.sample_at_world_xz(point.x, point.y),
        |point| water_grid.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0),
    ) else {
        return false;
    };
    blueprint.door_stairs_assembly = Some(Box::new(DoorStairsAssemblyParams {
        words: words.map(f32::to_bits),
    }));
    true
}

/// 原1407f10c7..1407f1142与1407f11a8..1407f11f9；未进入库查询的纯数值段。
pub(super) fn gothic_pose_from_samples(first: Vec3, second: Vec3) -> Transform {
    let dx = second.x - first.x;
    let dz = second.z - first.z;
    let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt();
    Transform {
        rotation: utils::xz_projective_rotation_bad(Vec3::new(
            -reciprocal * dz,
            0.0,
            dx * reciprocal,
        )),
        translation: second * 0.5 + first * 0.5,
        scale: Vec3::ONE,
    }
}

pub fn generate_gothic_balcony_doors(
    session: Res<ActiveSession>,
    walls: Res<PublicWalls>,
    storage: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    mut blueprints: ResMut<DecoratorBlueprints>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WaterRaster>,
) {
    let view = derived.as_view();
    let library = view.mesh_library();
    for (wall_id, decorators) in storage.iter_walls() {
        for modified in decorators.iter_modified_ids() {
            let id = DecoratorId(modified);
            let info = decorators.get(id).expect("modified装饰记录缺失");
            if view.subtype(id).0 != GOTHIC_DOOR_SUBTYPE {
                continue;
            }
            let blueprint =
                blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let wall = walls.get(wall_id).expect("阳台门所属墙缺失");
            let coord = info.dst_as_wall().wall_space_coord();
            let key = DecoratorLibKey::new(GOTHIC_DOOR_SUBTYPE, info.rank());
            let bounds = library.collision_record(key).bounds;
            let width = bounds.footprint.size.x;
            let bones = get_gothic_hat_bones(wall, info, coord.u, coord.y, width);
            let pose = gothic_pose_from_samples(bones.first, bones.second);
            let depth = bounds.extent().z;
            let collision_position = pose.translation
                + pose.rotation * (Vec3::new(depth * 0.0, depth * 0.0, depth) * 0.5);
            blueprint.set_positioned_colliders(
                library.collision_record(key).mesh.clone(),
                library.interaction_record(key).mesh.clone(),
                &collision_position,
                &pose.rotation,
            );
            let cached = CachedDecoratorTransform {
                rotation: pose.rotation,
                translation: pose.translation,
                scale: pose.scale,
            };
            blueprint.set_cached_transform(cached);
            let rank = rank_index(info.rank());
            for hole in HOLES[rank].chunks_exact(3) {
                blueprint.wall_holes.push(RawWallHole {
                    center: [coord.u, f32::from_bits(hole[0]) + coord.y],
                    dimensions: [f32::from_bits(hole[1]), f32::from_bits(hole[2])],
                    kind: 0,
                });
            }
            // 原1407f0ebc附近只在哥特门将占位色999映射到0。
            let color = Some(ColorId(if info.u28 == 999 { 0 } else { info.u28 }));
            let semi = info.state() != 0;
            let mut door = window_builder(id, pose, DOORS[rank], "nani_balcony_door", color, semi);
            door.balcony_door = true;
            blueprint.visual_entities.push(door);
            let mut hat = window_builder(
                id,
                pose,
                HATS[rank],
                "nani_gothic_window_bricks",
                color,
                semi,
            );
            hat.gothic_window_brick_deformation = Some(bones.deformation_bytes());
            hat.brick_color = Some(brick_color(
                info,
                wall_id.0 as u32,
                coord.u,
                width,
                WallStyle(wall.authority.style()).is_halftimbered(),
            ));
            blueprint.visual_entities.push(hat);
            add_door_autoclutter(info, &pose, blueprint, id, wall_id, &session);
            if (info.u20 >> 8) as u8 == 1
                && !check_for_door_stairs(&terrain, &water, &cached, &bounds, blueprint)
                && needs_bottom_rail(wall, info, &pose, &terrain)
            {
                blueprint.visual_entities.push(window_builder(
                    id,
                    pose,
                    RAILS[rank],
                    "nani_solid_vertex_color_window",
                    color,
                    semi,
                ));
            }
        }
    }
}
pub fn is_gothic_door(subtype: u8) -> bool {
    subtype == GOTHIC_DOOR_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::world_raster::DirtyTilePublisher;
    use glam::{Vec2, Vec3};

    fn bounds() -> Rectangle3d {
        Rectangle3d::new(Vec3::ZERO, Vec3::new(2.0, 2.0, 1.0), Vec2::X)
    }

    #[test]
    fn actual_sampling_rejection_preserves_existing_box() {
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 2.0);
        let water = WaterRaster::new(DirtyTilePublisher::unique());
        let transform = CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(0.0, 5.0, 0.0),
            scale: Vec3::ONE,
        };
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.door_stairs_assembly =
            Some(Box::new(DoorStairsAssemblyParams { words: [37; 8] }));
        let previous = std::ptr::from_ref(&**blueprint.door_stairs_assembly.as_ref().unwrap());
        assert!(!check_for_door_stairs(
            &terrain,
            &water,
            &transform,
            &bounds(),
            &mut blueprint
        ));
        let retained = blueprint.door_stairs_assembly.as_ref().unwrap();
        assert_eq!(std::ptr::from_ref(&**retained), previous);
        assert_eq!(retained.words, [37; 8]);
    }

    #[test]
    fn actual_height_and_water_owners_replace_with_all_eight_original_fields() {
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 2.0);
        // 两种真实插值模式都在原点读抬升行，在前方z=1.06读低行。
        terrain.heights_mut()[255 * 512..257 * 512].fill(u16::MAX);
        let mut water = WaterRaster::new(DirtyTilePublisher::unique());
        water.composite_mut().fill(255);
        let transform = CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        };
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.door_stairs_assembly =
            Some(Box::new(DoorStairsAssemblyParams { words: [37; 8] }));
        let previous = std::ptr::from_ref(&**blueprint.door_stairs_assembly.as_ref().unwrap());
        assert!(check_for_door_stairs(
            &terrain,
            &water,
            &transform,
            &bounds(),
            &mut blueprint
        ));
        let output = blueprint.door_stairs_assembly.as_ref().unwrap();
        assert_ne!(std::ptr::from_ref(&**output), previous);
        assert_eq!(
            output.words.map(f32::from_bits),
            [0.0, 0.0, 0.0, 1.0, 2.0, -0.8, 0.0, 2.0]
        );
    }
}

#[cfg(test)]
mod original_gothic_producer_tests {
    use super::super::cottage_balcony_door::original_balcony_tests::world;
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorageAddr};

    #[test]
    fn all_three_ranks_emit_original_holes_door_hat_and_deformation_without_cached_owner() {
        for rank in 1..=3 {
            let mut world = world(9, rank);
            world
                .run_system_once(generate_gothic_balcony_doors)
                .unwrap();
            let bp = world.resource::<DecoratorBlueprints>().get(7).unwrap();
            assert_eq!(
                bp.storage_addr,
                Some(DecoratorStorageAddr::Wall(WallId(42)))
            );
            assert_eq!(bp.wall_holes.len(), 3);
            assert_eq!(bp.visual_entities.len(), 2);
            assert!(bp.transform.is_some());
            let door = &bp.visual_entities[0];
            let hat = &bp.visual_entities[1];
            assert_eq!(door.mesh_name, DOORS[rank as usize - 1]);
            assert_eq!(door.nani_name, "nani_balcony_door");
            assert!(door.balcony_door);
            assert_eq!(hat.mesh_name, HATS[rank as usize - 1]);
            assert_eq!(hat.nani_name, "nani_gothic_window_bricks");
            assert!(!hat.balcony_door);
            assert!(hat.gothic_window_brick_deformation.is_some());
            assert_eq!(hat.brick_color.unwrap()[..8], [1, 0, 0, 0, 42, 0, 0, 0]);
            assert_eq!(door.color_id, Some(ColorId(0x55667788)));
            assert!(door.semi_transparent);
            for (hole, bits) in bp
                .wall_holes
                .iter()
                .zip(HOLES[rank as usize - 1].chunks_exact(3))
            {
                assert_eq!(hole.center, [4.0, 2.0 + f32::from_bits(bits[0])]);
                assert_eq!(
                    hole.dimensions,
                    [f32::from_bits(bits[1]), f32::from_bits(bits[2])]
                );
                assert_eq!(hole.kind, 0);
            }
        }
    }

    #[test]
    fn exact_bottom_byte_controls_actual_rail_and_profile_uses_mapped_curve() {
        use country_core::resources::walls::{decorator::DecoratorId, public_walls::PublicWalls};
        for subtype in [4, 9] {
            for bottom_byte in [0_u8, 1, 2] {
                let mut world = world(subtype, 1);
                let mut walls = world.resource_mut::<PublicWalls>();
                let wall = walls.get_mut(WallId(42)).unwrap();
                // 原mappedCurve3由世界路径抬升；raw Curve2仍平面，必须只用mapped高度判断栏杆。
                for point in std::sync::Arc::make_mut(&mut wall.geometry.outline) {
                    point[1] = 1.0;
                }
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let info = storage
                    .per_wall
                    .get_mut(&WallId(42))
                    .unwrap()
                    .get_mut(DecoratorId(7))
                    .unwrap();
                info.u20 = (info.u20 & !0xff00) | (u32::from(bottom_byte) << 8);
                if subtype == 4 {
                    world
                        .run_system_once(
                            super::super::cottage_balcony_door::generate_cottage_balcony_doors,
                        )
                        .unwrap();
                } else {
                    world
                        .run_system_once(generate_gothic_balcony_doors)
                        .unwrap();
                }
                let bp = world.resource::<DecoratorBlueprints>().get(7).unwrap();
                let ordinary = if subtype == 4 { 1 } else { 2 };
                assert_eq!(
                    bp.visual_entities.len(),
                    ordinary + usize::from(bottom_byte == 1),
                    "subtype {subtype} bottom {bottom_byte}"
                );
                assert!(bp.door_stairs_assembly.is_none());
                if bottom_byte == 1 {
                    let rail = bp.visual_entities.last().unwrap();
                    assert!(rail.mesh_name.ends_with("rank1_rails"));
                    assert!(!rail.balcony_door);
                    assert_eq!(rail.nani_name, "nani_solid_vertex_color_window");
                }
            }
        }
    }
}
