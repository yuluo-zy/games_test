//! 原B `decorator_visual/arrow_slit.rs`，主生成器1407bbee0/63KB。
//! 原文件SHA 6cf90892319dda7d481045d9c1cfbef261a8a87b20e5fc280804f39ac5345daf。
//! 墙空间帽骨、碰撞、墙洞、玻璃与半老虎窗必须共同发布到唯一蓝图owner。
use super::{cottage_wall_window::window_builder, gothic_wall_window::get_gothic_hat_bones};
use crate::blueprint::DecoratorBlueprints;
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        color_id::ColorId,
        walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            dormers::DormerHoleSsbo,
            public_walls::PublicWalls,
            wall_holes::RawWallHole,
        },
    },
    systems::ui::signifiers::signifier_create::UiSignifierStream,
    utils::components::transform::Transform,
};
use glam::{Vec2, Vec3};

pub const ARROW_SLIT_TYPE: u8 = 0x3a;
pub fn is_arrow_slit(info: &DecoratorInfo) -> bool {
    info.decorator_type().0 == ARROW_SLIT_TYPE
}

/// 原表是独立墙洞尺寸，不是四点曲线。rank3追加两个不同尺寸的洞。
const HOLES_ONE: [[f32; 2]; 1] = [[0.5, 0.5]];
const HOLES_TWO: [[f32; 2]; 1] = [[f32::from_bits(0x3ecc_cccd), 1.5]];
const HOLES_THREE: [[f32; 2]; 2] = [
    [f32::from_bits(0x3ecc_cccd), f32::from_bits(0x3fd9_999a)],
    [f32::from_bits(0x3f1a_e148), f32::from_bits(0x3f0c_cccd)],
];
fn slit_holes(rank: u8) -> &'static [[f32; 2]] {
    match rank {
        1 => &HOLES_ONE,
        2 => &HOLES_TWO,
        3 => &HOLES_THREE,
        _ => panic!("internal error: entered unreachable code"),
    }
}

fn arrow_slit_rotation(first: Vec3, second: Vec3) -> glam::Quat {
    let delta = second - first;
    let inv = 1.0 / (delta.z * delta.z + delta.x * delta.x).sqrt();
    utils::xz_projective_rotation_bad(Vec3::new(-inv * delta.z, 0.0, delta.x * inv))
}

/// 原五组参数：箭窗直接由墙空间构建，不能换成缓存位姿近似。
pub fn generate_arrow_slits(
    storage: Res<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
    walls: Res<PublicWalls>,
    _signifiers: Res<UiSignifierStream>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    let view = derived.as_view();
    for (wall_id, bucket) in storage.iter_walls() {
        for id in bucket.iter_modified_ids() {
            let info = bucket
                .get(DecoratorId(id))
                .expect("modified decorator is missing from its storage");
            if !is_arrow_slit(info) {
                continue;
            }
            let subtype = view.subtype(DecoratorId(id)).0;
            if subtype == 23 && info.rank() == 1 {
                continue;
            }
            let wall = walls
                .get(wall_id)
                .expect("called `Option::unwrap()` on a `None` value");
            let key = DecoratorLibKey::new(subtype, info.rank());
            let bounds = view.mesh_library().collision_record(key).bounds;
            let coord = info.dst_as_wall().wall_space_coord();
            let bones =
                get_gothic_hat_bones(wall, info, coord.u, coord.y, f32::from_bits(0x3f4c_cccd));
            let rotation = arrow_slit_rotation(bones.first, bones.second);
            let pose = Transform {
                rotation,
                translation: bones.second * 0.5 + bones.first * 0.5,
                scale: Vec3::ONE,
            };
            let blueprint =
                blueprints.insert_for(id, DecoratorStorageAddr::Wall(wall_id), info.state());
            let mesh = match info.rank() {
                1 => "decorators/arrow_slit_1x1",
                2 => "decorators/arrow_slit_1x2",
                3 if subtype == 23 => "decorators/arrow_slit_1x3_halfdormer",
                3 => "decorators/arrow_slit_1x3",
                _ => panic!("internal error: entered unreachable code"),
            };
            let half = bounds.footprint.size.x * 0.5;
            let mut brick = [0u8; 24];
            let words = [
                info.decorator_id,
                wall_id.0 as u32,
                (coord.u - half).to_bits(),
                (half + coord.u).to_bits(),
                bounds.footprint.size.x.to_bits(),
            ];
            for (slot, word) in brick[..20].chunks_exact_mut(4).zip(words) {
                slot.copy_from_slice(&word.to_le_bytes());
            }
            let semi = info.state() != 0;
            let mut frame = window_builder(
                DecoratorId(id),
                pose,
                mesh,
                if info.u28 == 999 {
                    "nani_solid_vertex_color_bricks"
                } else {
                    "nani_solid_vertex_color_window"
                },
                Some(ColorId(info.u28)),
                semi,
            );
            frame.brick_color = Some(brick);
            blueprint.visual_entities.push(frame);
            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&view.mesh_library().collision_record(key).mesh),
                std::sync::Arc::clone(&view.mesh_library().interaction_record(key).mesh),
                &pose.translation,
                &pose.rotation,
            );
            for dimensions in slit_holes(info.rank()) {
                blueprint.wall_holes.push(RawWallHole {
                    center: [coord.u, coord.y],
                    dimensions: *dimensions,
                    kind: 0,
                });
            }
            let premade = wall.shape().tag() < 2;
            let mut outline = pose;
            if premade {
                outline.translation += rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e0f_5c29));
            }
            blueprint.set_cached_transform(country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform {
                rotation: outline.rotation, translation: outline.translation, scale: outline.scale });
            if premade {
                let mesh = match info.rank() {
                    1 => "decorators/arrow_slit_1x1_glass",
                    2 => "decorators/arrow_slit_1x2_glass",
                    3 => "decorators/arrow_slit_1x3_glass",
                    _ => unreachable!(),
                };
                let mut glass = window_builder(
                    DecoratorId(id),
                    pose,
                    mesh,
                    "nani_solid_vertex_color_window_glass",
                    Some(ColorId(info.extra().as_window().0)),
                    semi,
                );
                glass.window_light = Some(u64::from(id) + u64::from(info.rank()) * 1000);
                blueprint.visual_entities.push(glass);
            }
            if subtype == 23 {
                let mesh = match info.rank() {
                    2 => "decorators/arrow_slit_1x2_hat",
                    3 => "decorators/arrow_slit_1x3_hat",
                    _ => panic!("internal error: entered unreachable code"),
                };
                let mut hat = window_builder(
                    DecoratorId(id),
                    pose,
                    mesh,
                    "nani_gothic_window_bricks",
                    Some(ColorId(999)),
                    semi,
                );
                hat.brick_color = Some(brick);
                let values = [
                    bones.side[0],
                    bones.side[1],
                    bones.side[2],
                    bones.side[3],
                    bones.edge[0],
                    bones.edge[1],
                    bones.first.x,
                    bones.first.y,
                    bones.first.z,
                    bones.center.x,
                    bones.center.y,
                    bones.center.z,
                    bones.second.x,
                    bones.second.y,
                    bones.second.z,
                ];
                let mut deformation = [0u8; 60];
                for (slot, value) in deformation.chunks_exact_mut(4).zip(values) {
                    slot.copy_from_slice(&value.to_le_bytes());
                }
                hat.gothic_window_brick_deformation = Some(deformation);
                blueprint.visual_entities.push(hat);
                let extent = bounds.extent();
                let forward = rotation * Vec3::Z;
                let middle_y = (bounds.bottom + bounds.top) * 0.5;
                let center = pose.translation + Vec3::new(middle_y * 0.0, middle_y, middle_y * 0.0);
                let first = center + forward * 0.5 * 0.5;
                blueprint.dormer_holes.push(DormerHoleSsbo::gothic(
                    wall_id.0 as u32,
                    first,
                    Vec3::new(
                        f32::from_bits(0x3f33_3333),
                        f32::from_bits(0x4022_8f5c),
                        extent.z + 0.5,
                    ),
                    [forward.x, forward.z, 2.5, f32::from_bits(0x3d4c_cccd)],
                ));
            }
        }
    }
}

#[cfg(test)]
mod original_world_tests {
    use super::super::flag::full_visual_test_support::*;
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    #[test]
    fn original_arrow_native_rotation_bits() {
        let cases: &[([u32; 6], [u32; 4])] = include!(
            "../../../../../reports/module-recovery-2026-10-23/m01-arrow-pose-native-cases.rs"
        );
        for (index, (input, expected)) in cases.iter().enumerate() {
            let first = Vec3::from_array(std::array::from_fn(|i| f32::from_bits(input[i])));
            let second = Vec3::from_array(std::array::from_fn(|i| f32::from_bits(input[i + 3])));
            assert_eq!(
                arrow_slit_rotation(first, second)
                    .to_array()
                    .map(f32::to_bits),
                *expected,
                "原箭孔旋转case {index}"
            );
        }
    }
    #[test]
    fn every_rank_and_halfdormer_publishes_real_holes_glass_deformation_and_releases() {
        for subtype in [22, 23] {
            for (rank, state) in (1..=3).flat_map(|rank| [0, 1].map(|state| (rank, state))) {
                let mut info = DecoratorInfo {
                    ty: 0x3a,
                    rank,
                    state,
                    u28: 55,
                    decorator_id: 19,
                    ..Default::default()
                };
                info.dst_wall = u64::from(4f32.to_bits()) | (u64::from(2f32.to_bits()) << 32);
                info.set_extra(
                    country_core::resources::walls::decorator::DecoratorExtraInfo::Window(
                        country_core::resources::walls::decorator::WindowExtra(77),
                    ),
                );
                let mut world = scene(
                    info,
                    subtype,
                    DecoratorStorageAddr::Wall(country_core::resources::walls::WallId(7)),
                );
                world.run_system_once(generate_arrow_slits).unwrap();
                if subtype == 23 && rank == 1 {
                    assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
                    continue;
                }
                let bp = world.resource::<DecoratorBlueprints>().get(41).unwrap();
                assert_eq!(bp.visual_entities.len(), if subtype == 23 { 3 } else { 2 });
                assert_eq!(bp.wall_holes.len(), if rank == 3 { 2 } else { 1 });
                assert_eq!(
                    bp.visual_entities[1].window_light,
                    Some(41 + u64::from(rank) * 1000)
                );
                assert_eq!(bp.visual_entities[1].color_id, Some(ColorId(77)));
                assert_eq!(bp.dormer_holes.len(), usize::from(subtype == 23));
                if subtype == 23 {
                    assert!(
                        bp.visual_entities[2]
                            .gothic_window_brick_deformation
                            .is_some()
                    );
                }
                let entities = instantiate(&mut world);
                assert_eq!(entities.len(), if subtype == 23 { 3 } else { 2 });
                assert!(
                    world
                        .get::<country_core::resources::window_lights::WindowLight>(entities[1])
                        .is_some()
                );
                assert_unload(&mut world, &entities);
            }
        }
    }
}
