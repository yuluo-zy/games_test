//! 原B哥特墙窗：完整主生成器1407e87b0/10658B及帽骨1407eb500/1182B。
//! 原证与数值表见本任务gothic-main-chain-20261021。主生成器只发布蓝图，
//! 实体/GPU上传仍由唯一fullvisual/Nani宿主消费，不在这里复制运行时owner。
//! 尚未恢复的独立半老虎窗剖面函数保留原有边界，不是本主函数的callee。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use country_core::{
    name_hash::NameHash,
    resources::{
        color_id::ColorId,
        gothic_window_profile::gothic_half_dormer_profile_curve,
        walls::{
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator::{DecoratorId, DecoratorInfo},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            dormers::DormerHoleSsbo,
            inner_wall_state::WallStyle,
            public_walls::{PublicWallState, PublicWalls},
            wall_holes::RawWallHole,
        },
    },
};
use glam::{Vec2, Vec3};
use utils::{
    comparative_space::ComparativeSpace,
    curve::Curve3,
    swizzlers::xvy,
    wall_space::{CurveSampling, WallSpaceCoord},
};

use super::cottage_wall_window::{wall_space, window_builder};
use crate::blueprint::DecoratorBlueprints;
use ::utils::scalar::inverse_lerp;

/// 半径表（O：DAT_142c2b180 = {0.25, 0.3, 0.29}，read_memory 核验）。
pub const GOTHIC_HALF_DORMER_RADIUS: [f32; 3] = [0.25, 0.3, 0.29];
/// 默认半径（O：0x142c2b0c8 = 0.38，read_memory 核验）。
pub const GOTHIC_HALF_DORMER_RADIUS_DEFAULT: f32 = 0.38;

/// `gothic_half_dormer_radius`（0x1412ac9b0 内联，O）。
///
/// 半径选择（radare2 反汇编核验）：`is_circle()==Some && shape_kind==7` →
/// `GOTHIC_HALF_DORMER_RADIUS[rank-1]`（rank 1..3，否则 panic）；其余 → 0.38。
/// ⚠ RE-ANALYSIS §4.4 "is_circle? 0.38 : 表" 方向反了，以本实现为准。
pub fn gothic_half_dormer_radius(rank: u8, is_circle: bool, shape_kind: u8) -> f32 {
    if is_circle && shape_kind == 7 && (1..=3).contains(&rank) {
        GOTHIC_HALF_DORMER_RADIUS[rank as usize - 1]
    } else {
        GOTHIC_HALF_DORMER_RADIUS_DEFAULT
    }
}

/// 0x1407eb160（O 结构 + fresh 核对）：哥特半老虎窗剖面世界空间计算。
///
/// 主体 `0..2` for_each 构建两条剖面 `Curve3` 压入 `out`（index 语义 U）。
/// 每段：`gothic_half_dormer_profile_curve(rank)` 取点表 → 逐点世界变换（U，
/// wall-space 面）→ max_y 截断（O 结构）→ `Curve::try_new_from_points<Vec3,3D>`。
/// U：精确 wall-space 变换（B raw 偏移缺 country_core PublicWallState）、
/// 0..2 索引语义、逐语句寄存器流。
pub fn calc_gothic_half_dormer_profile_ws(
    out: &mut Vec<Curve3>,
    rank: u8,
    max_y: f32,
    is_circle: bool,
    shape_kind: u8,
) {
    // O：`0..2` for_each（两次 closure_0，0x1407eb274 / 0x1407eb282）。
    for _ in 0..2 {
        let profile = gothic_half_dormer_profile_curve(rank);
        // U：半径在此函数逐点变换中经 wall 法线应用（0x1412ac9b0 输出
        // `Vec3(fVar9*nx+lerp_x, h+lerp_y, fVar9*ny+lerp_z)`）；当前以 RE-ANALYSIS
        // 简式承载，半径仅登记。
        let _radius = gothic_half_dormer_radius(rank, is_circle, shape_kind);

        // 逐点世界变换（U，wall-space 面）：RE-ANALYSIS 简式 `Vec3(u, h*max_y, 0)`。
        // ⚠ 精确变换含 inverse_lerp(0,last,u) 的墙点 A/B lerp 与 C/D 法线
        // （raw offsets 缺 country_core PublicWallState），未实现。
        let world_points: Vec<Vec3> = profile
            .iter()
            .map(|p| Vec3::new(p.x, p.y * max_y, 0.0))
            .collect();

        // max_y 截断（O 结构，closure$0 0x1407e7dd0）：折线裁剪到 y >= max_y，
        // 段跨越处插 `y==max_y` 截断点（`t=(max_y-prev.y)/(cur.y-prev.y)`）。
        // 寄存器别名打散，精确流 U，以语义裁剪承载。
        let mut points: Vec<Vec3> = Vec::with_capacity(world_points.len() + 1);
        let mut prev: Option<Vec3> = None;
        for cur in &world_points {
            if let Some(p) = prev {
                if (p.y >= max_y) != (cur.y >= max_y) {
                    let t = (max_y - p.y) / (cur.y - p.y);
                    points.push(Vec3::new(
                        p.x + (cur.x - p.x) * t,
                        max_y,
                        p.z + (cur.z - p.z) * t,
                    ));
                }
            }
            if cur.y >= max_y {
                points.push(*cur);
            }
            prev = Some(*cur);
        }

        // O：`Curve::try_new_from_points<Vec3, CurveCoordSemantic3D>()` +
        // unwrap（closure$0 LAB_1407e80e4）→ 压入 out（stride 0x38）。
        let curve = Curve3::try_new_from_points(points, false).unwrap();
        out.push(curve);
    }
}

/// 原60B输出的语义字段；不依赖本机Vec3对齐或原生容器布局。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GothicHatBones {
    pub side: [f32; 4],
    pub first: Vec3,
    pub center: Vec3,
    pub second: Vec3,
    pub edge: [f32; 2],
}

impl GothicHatBones {
    // 主函数发布前把edge前移；GPU载荷不是帽骨结构的内存拷贝。
    fn deformation_bytes(self) -> [u8; 60] {
        let values = [
            self.side[0],
            self.side[1],
            self.side[2],
            self.side[3],
            self.edge[0],
            self.edge[1],
            self.first.x,
            self.first.y,
            self.first.z,
            self.center.x,
            self.center.y,
            self.center.z,
            self.second.x,
            self.second.y,
            self.second.z,
        ];
        let mut bytes = [0; 60];
        for (slot, value) in bytes.chunks_exact_mut(4).zip(values) {
            slot.copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }
}

/// 原1182B：借用唯一墙曲线，保留投影、选段和失败的先后顺序。
pub fn get_gothic_hat_bones(
    wall: &PublicWallState,
    info: &DecoratorInfo,
    u: f32,
    y: f32,
    half_base: f32,
) -> GothicHatBones {
    let mut half = half_base;
    if let Some(circle) = wall.is_circle() {
        let t = inverse_lerp(0.95, 1.6, circle.radius);
        match info.rank() {
            1 => half += -0.05,
            2 | 3 => {
                let c = t.clamp(0.0, 1.0);
                // 原零乘项不能省略：非有限值时仍参与该数值合同。
                let adjustment = if info.rank() == 2 {
                    (0.0 * c + (1.0 - c) * 0.21) + 0.1
                } else {
                    (0.0 * c + (1.0 - c) * 0.158) + 0.15
                };
                half -= adjustment;
            }
            _ => panic!("get_gothic_hat_bones: unexpected rank"),
        }
    } else {
        match info.rank() {
            1 => {}
            2 => half += -0.05,
            3 => half += -0.1,
            _ => panic!("get_gothic_hat_bones: unexpected rank"),
        }
    }
    half *= 0.5;
    let space = wall_space(wall);
    let first_u = space.project_clamp(u - half);
    let second_u = space.project_clamp(u + half);
    let curve = wall.simplified_curve_2d();
    let first = WallSpaceCoord::new_unchecked(first_u, y).to_world_space(curve);
    let second = WallSpaceCoord::new_unchecked(second_u, y).to_world_space(curve);
    let delta_at = |point: Vec3| {
        let approx = curve
            .approximate_u(Vec2::new(point.x, point.z))
            .clamp(0.0, 1.0);
        let (segment, _) = curve.coord_at_u(approx);
        let a = Vec2::from_array(curve.points[segment]);
        let b = Vec2::from_array(curve.points[segment + 1]);
        b - a
    };
    let first_delta = delta_at(first);
    let second_delta = delta_at(second);
    // 原版先完成两端的required选段，之后才转换未经投影的中心点。
    let center = WallSpaceCoord::new_unchecked(u, y).to_world_space(curve);
    let center_delta = delta_at(center);
    let normal = |delta: Vec2| {
        let reciprocal = 1.0 / (delta.y * delta.y + delta.x * delta.x).sqrt();
        [-delta.y * reciprocal, delta.x * reciprocal]
    };
    let first_normal = normal(first_delta);
    let second_normal = normal(second_delta);
    let center_normal = normal(center_delta);
    GothicHatBones {
        side: [
            first_normal[0],
            first_normal[1],
            center_normal[0],
            center_normal[1],
        ],
        first: xvy(Vec2::new(first.x, first.z), y),
        center,
        second: xvy(Vec2::new(second.x, second.z), y),
        edge: second_normal,
    }
}

const HAT_NAMES: [&str; 3] = [
    "decorators/window_gothic_1x1_hat",
    "decorators/window_gothic_2x1_hat",
    "decorators/window_gothic_3x1_hat",
];
const FULL_HAT_NAMES: [&str; 3] = [
    "decorators/window_gothic_1x1_hat_full",
    "decorators/window_gothic_2x1_hat_full",
    "decorators/window_gothic_3x1_hat_full",
];
const FRAME_NAMES: [&str; 3] = [
    "decorators/window_gothic_1x1",
    "decorators/window_gothic_2x1",
    "decorators/window_gothic_3x1",
];
const GLASS_NAMES: [&str; 3] = [
    "decorators/window_gothic_1x1_glass",
    "decorators/window_gothic_2x1_glass",
    "decorators/window_gothic_3x1_glass",
];
const RANK_WIDTHS: [f32; 3] = [
    f32::from_bits(0x3f73_8716),
    f32::from_bits(0x3fbf_3e03),
    f32::from_bits(0x3ff6_4e8b),
];
const DORMER_TAIL_A: [f32; 3] = [
    2.0,
    f32::from_bits(0x400c_cccd),
    f32::from_bits(0x3fec_cccd),
];
const DORMER_TAIL_B: [f32; 3] = [
    f32::from_bits(0xbc34_3958),
    f32::from_bits(0xbe06_24dd),
    f32::from_bits(0x3d71_a9fc),
];

/// 原三张9浮点表：每行依次为中心Y偏移、宽、高，不从网格包围盒近似。
fn window_holes(rank: u8) -> [[f32; 3]; 3] {
    let bits = match rank {
        1 => [
            [0xbe93_3333, 0x3f32_fec5, 0x3faa_c083],
            [0x3f0a_5e35, 0x3f16_bb99, 0x3efd_2f1b],
            [0x3f68_9375, 0x3eb8_4b5d, 0x3eb0_a3d7],
        ],
        2 => [
            [0xbdc8_b43a, 0x3f95_8106, 0x3fd9_999a],
            [0x3f80_4189, 0x3f8b_e76d, 0x3f07_ae14],
            [0x3fad_0e56, 0x3f1c_ed91, 0x3e99_999a],
        ],
        3 => [
            [0x3e6b_851f, 0x3fc1_0625, 0x4016_6666],
            [0x3fca_3d71, 0x3f9f_1aa0, 0x3ec2_8f5c],
            [0x3fea_3d71, 0x3f50_20c5, 0x3eae_147b],
        ],
        _ => panic!("generate_gothic_wall_windows: unexpected rank"),
    };
    bits.map(|row| row.map(f32::from_bits))
}

fn brick_color(
    info: &DecoratorInfo,
    wall: u32,
    u: f32,
    width: f32,
    halftimbered: bool,
) -> [u8; 24] {
    let half = width * 0.5;
    let words = [
        info.decorator_id,
        wall,
        (u - half).to_bits(),
        (half + u).to_bits(),
        width.to_bits(),
    ];
    let mut bytes = [0; 24];
    for (slot, word) in bytes[..20].chunks_exact_mut(4).zip(words) {
        slot.copy_from_slice(&word.to_le_bytes());
    }
    bytes[20] = u8::from(halftimbered);
    bytes
}

/// 原10658B四参系统：只从真实墙和canonical目录生成本id的完整蓝图。
pub fn generate_gothic_wall_windows(
    derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    mut blueprints: ResMut<DecoratorBlueprints>,
) {
    let view = derived.as_view();
    for (wall_id, decorators) in storage.iter_walls() {
        for (modified, info) in decorators.iter_modified() {
            let id = DecoratorId(modified);
            let subtype = view.subtype(id).0;
            if subtype & 0x1e != 6 || info.dst_tag() > 1 {
                continue;
            }
            let wall = walls
                .get(wall_id)
                .expect("called `Option::unwrap()` on a `None` value");
            let key = DecoratorLibKey::new(subtype, info.rank());
            let mut bounds = view.mesh_library().collision_record(key).bounds;
            // 原版先覆盖局部包围盒宽度；非法rank仍由后续帽骨的实际门负责失败。
            let width = match info.rank() {
                3 => RANK_WIDTHS[2],
                2 => RANK_WIDTHS[1],
                _ => RANK_WIDTHS[0],
            };
            bounds.footprint.size.x = width;
            let coord = info.dst_as_wall().wall_space_coord();
            let bones = get_gothic_hat_bones(wall, info, coord.u, coord.y, width);
            let dx = bones.second.x - bones.first.x;
            let dz = bones.second.z - bones.first.z;
            let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt();
            let rotation = utils::xz_projective_rotation_bad(Vec3::new(
                -reciprocal * dz,
                0.0,
                dx * reciprocal,
            ));
            let mut pose = country_core::utils::components::transform::Transform {
                rotation,
                translation: bones.second * 0.5 + bones.first * 0.5,
                scale: Vec3::ONE,
            };
            let blueprint =
                blueprints.insert_for(modified, DecoratorStorageAddr::Wall(wall_id), info.state());
            let halftimbered = WallStyle(wall.authority.style()).is_halftimbered();
            let color = ColorId(info.u28);
            let brick = brick_color(info, wall_id.0 as u32, coord.u, width, halftimbered);
            let index = usize::from(info.rank() - 1);
            let full_hat = subtype == 7 || halftimbered;
            let hat_name = if full_hat {
                FULL_HAT_NAMES[index]
            } else {
                HAT_NAMES[index]
            };
            let semi = info.state() != 0;
            let mut hat = window_builder(
                id,
                pose,
                hat_name,
                if info.rank() == 1 {
                    "nani_solid_vertex_color_bricks"
                } else {
                    "nani_gothic_window_bricks"
                },
                Some(color),
                semi,
            );
            if info.rank() != 1 {
                hat.gothic_window_brick_deformation = Some(bones.deformation_bytes());
            }
            hat.brick_color = Some(brick);
            blueprint.visual_entities.push(hat);

            if wall.shape().tag() < 2 {
                let glass_color = ColorId(info.window_extra().0);
                let albedo = NameHash::compute("window_glass");
                pose.translation +=
                    pose.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3e0f_5c29));
                let mut glass = window_builder(
                    id,
                    pose,
                    GLASS_NAMES[index],
                    "nani_solid_vertex_color_window_glass",
                    Some(glass_color),
                    semi,
                );
                glass.window_light = Some(u64::from(modified) + 1000 * u64::from(info.rank()));
                glass.albedo_map = Some(albedo.0);
                blueprint.visual_entities.push(glass);
            }
            let mut frame = window_builder(
                id,
                pose,
                FRAME_NAMES[index],
                if color.0 == 999 {
                    "nani_solid_vertex_color_bricks"
                } else {
                    "nani_solid_vertex_color_window"
                },
                Some(color),
                semi,
            );
            let mut frame_brick = brick;
            frame_brick[20] = 0;
            frame.brick_color = Some(frame_brick);
            blueprint.visual_entities.push(frame);

            // 两个required查表及Arc增加都在实际视觉发布之后；失败不回滚前面的帽/玻璃/窗框。
            let collision = Arc::clone(&view.mesh_library().collision_record(key).mesh);
            let interaction = Arc::clone(&view.mesh_library().interaction_record(key).mesh);
            let depth = bounds.extent().z;
            let collider_offset =
                pose.rotation * (Vec3::new(depth * 0.0, depth * 0.0, depth) * 0.5);
            let position = pose.translation + collider_offset;
            blueprint.set_positioned_colliders(collision, interaction, &position, &pose.rotation);
            blueprint.set_cached_transform(CachedDecoratorTransform {
                rotation: pose.rotation,
                translation: pose.translation,
                scale: Vec3::ONE,
            });

            for [offset_y, width, height] in window_holes(info.rank()) {
                let source = info.dst_as_wall().wall_space_coord();
                blueprint.wall_holes.push(RawWallHole {
                    center: [source.u, offset_y + source.y],
                    dimensions: [width, height],
                    kind: 0,
                });
            }
            if subtype == 7 && wall.shape().tag() < 2 {
                let extent = bounds.extent();
                if extent.x > 0.0 {
                    let mid_y = (bounds.bottom + bounds.top) * 0.5;
                    let forward = pose.rotation * Vec3::Z;
                    let center = pose.translation + Vec3::new(mid_y * 0.0, mid_y, mid_y * 0.0);
                    let first = forward * 0.5 * 0.5 + center;
                    blueprint.dormer_holes.push(DormerHoleSsbo::gothic(
                        wall_id.0 as u32,
                        first,
                        Vec3::new(extent.x, extent.y + 0.0, f32::from_bits(0x3f90_a3d7)),
                        [
                            forward.x,
                            forward.z,
                            DORMER_TAIL_A[index],
                            DORMER_TAIL_B[index],
                        ],
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod original_main_tests {
    use super::*;
    use crate::blueprint::BlueprintMeshSlot;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{
        resources::{
            roofs::geometry::surface::Curve2,
            walls::{
                InnerWalls, WallId, WallState,
                decorator::{DecoratorDst, WallDecoratorDst},
                decorator_subtype::{
                    DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype,
                    DerivedDecoratorSubtypes,
                },
                publication_fields::WallShapeSnapshot,
                publication_source::TerrainHeightGrid512,
            },
        },
        systems::collision::world::RaycastMeshData,
    };

    const WALL: WallId = WallId(0x1234_5678_0000_002a);
    const STORED_WORD: u32 = 0x89ab_cdef;

    fn published_wall(points: Vec<[f32; 2]>, closed: bool, shape: u32, style: u8) -> PublicWalls {
        let curve = Curve2::try_new(points, closed).unwrap();
        let mut snapshot = WallState::new(curve, 3.0, style, 0, false)
            .snapshot()
            .clone();
        let mut shape_bytes = [0; 28];
        shape_bytes[..4].copy_from_slice(&shape.to_le_bytes());
        for (index, value) in [0.0_f32, 0.0, 1.25, 0.0, 8.0, 8.0].into_iter().enumerate() {
            shape_bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape_bytes);
        snapshot.source.curve_kind_tag = shape;
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WALL, WallState::from_snapshot(snapshot, 0));
        let mut walls = PublicWalls::default();
        walls.consume(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.0,
                maximum_y: 0.0,
                sampling_fix: false,
            },
        );
        assert!(walls.get(WALL).is_some());
        walls
    }

    fn producer_world(subtype: u8, rank: u8, shape: u32, style: u8, state: u8) -> World {
        let rectangle = utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(8.0));
        let corners = rectangle.as_points2();
        // 原Rectangle角点先沿左竖边；本夹具的坐标oracle明确要求u=4落底横边。
        // 显式构造同一真实闭矩形的底边起点与+X方向，不修改原几何算法或期望。
        let walls = published_wall(
            [corners[0], corners[3], corners[2], corners[1]]
                .map(|p| p.to_array())
                .to_vec(),
            true,
            shape,
            style,
        );
        let curve = walls.get(WALL).unwrap().simplified_curve_2d();
        assert_eq!(curve.points[0], [-4.0, -4.0]);
        assert_eq!(curve.points[1], [4.0, -4.0]);
        assert_eq!(
            WallSpaceCoord::new_unchecked(4.0, 2.0).to_world_space(curve),
            Vec3::new(0.0, 2.0, -4.0),
        );
        let mut info = DecoratorInfo {
            rank,
            state,
            decorator_id: STORED_WORD,
            u28: 0x5566_7788,
            u30: 0x1020_3040,
            u40: 6 << 8,
            ..DecoratorInfo::default()
        };
        info.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord: WallSpaceCoord::new_unchecked(4.0, 2.0),
            platform: WallId(99),
            relative_y: 0.0,
            position: Vec3::new(123.0, 456.0, 789.0),
            corner_attachment: 1,
            bottom_as_door: false,
        }));
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(WALL))
            .insert(DecoratorId(7), info);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(7, DecoratorSubtype(subtype));
        let mut library = DecoratorMeshLibrary::default();
        let bounds = utils::geometry::Rectangle3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(4.0, 2.0, 1.0),
            Vec2::X,
        );
        let key = DecoratorLibKey::new(subtype, rank);
        library.collision.insert(
            key,
            DecoratorCollision {
                triangles: Vec::new(),
                mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                bounds,
            },
        );
        // 交互bounds故意不同：原碰撞位移必须取最初collision的depth，不能偷读此值。
        library.interaction.insert(
            key,
            DecoratorCollision {
                triangles: Vec::new(),
                mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO)),
                bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::splat(9.0), Vec2::X),
            },
        );
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(storage);
        world.insert_resource(subtypes);
        world.insert_resource(library);
        world.insert_resource(DecoratorBlueprints::default());
        world
    }

    fn change_info(world: &mut World, change: impl FnOnce(&mut DecoratorInfo)) {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        change(
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(WALL))
                .get_mut(DecoratorId(7))
                .unwrap(),
        );
    }

    fn run_fails(world: &mut World) {
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.run_system_once(generate_gothic_wall_windows).unwrap();
        }));
        assert!(failure.is_err());
    }

    #[test]
    fn all_native_ranks_subtypes_and_wall_styles_publish_hat_glass_frame_with_real_arcs() {
        for subtype in [6, 7] {
            for rank in 1..=3 {
                for style in [0, 2] {
                    let mut world = producer_world(subtype, rank, 1, style, 1);
                    world.run_system_once(generate_gothic_wall_windows).unwrap();
                    let blueprints = world.resource::<DecoratorBlueprints>();
                    let blueprint = blueprints.get(7).unwrap();
                    let index = usize::from(rank - 1);
                    assert_eq!(
                        blueprint.storage_addr,
                        Some(DecoratorStorageAddr::Wall(WALL))
                    );
                    assert_eq!(blueprint.interaction_source_flags, 1);
                    assert_eq!(blueprint.visual_entities.len(), 3);
                    let hat = &blueprint.visual_entities[0];
                    let glass = &blueprint.visual_entities[1];
                    let frame = &blueprint.visual_entities[2];
                    assert_eq!(
                        hat.mesh_name,
                        if subtype == 7 || style >= 2 {
                            FULL_HAT_NAMES[index]
                        } else {
                            HAT_NAMES[index]
                        }
                    );
                    assert_eq!(
                        hat.nani_name,
                        if rank == 1 {
                            "nani_solid_vertex_color_bricks"
                        } else {
                            "nani_gothic_window_bricks"
                        }
                    );
                    assert_eq!(hat.gothic_window_brick_deformation.is_some(), rank != 1);
                    assert_eq!(hat.color_id, Some(ColorId(0x5566_7788)));
                    let brick = hat.brick_color.unwrap();
                    assert_eq!(
                        u32::from_le_bytes(brick[..4].try_into().unwrap()),
                        STORED_WORD
                    );
                    assert_eq!(u32::from_le_bytes(brick[4..8].try_into().unwrap()), 42);
                    assert_eq!(brick[20], u8::from(style >= 2));
                    assert_eq!(glass.mesh_name, GLASS_NAMES[index]);
                    assert_eq!(glass.nani_name, "nani_solid_vertex_color_window_glass");
                    assert_eq!(glass.color_id, Some(ColorId(0x1020_3040)));
                    assert_eq!(glass.window_light, Some(7 + 1000 * u64::from(rank)));
                    assert_eq!(glass.albedo_map, Some(NameHash::compute("window_glass").0));
                    assert!(
                        glass.brick_color.is_none()
                            && glass.gothic_window_brick_deformation.is_none()
                    );
                    assert_eq!(frame.mesh_name, FRAME_NAMES[index]);
                    assert_eq!(frame.nani_name, "nani_solid_vertex_color_window");
                    assert_eq!(frame.brick_color.unwrap()[20], 0);
                    for builder in &blueprint.visual_entities {
                        assert_eq!(builder.decorator_id, 7);
                        assert!(builder.semi_transparent);
                        assert_eq!(builder.scale, Vec3::ONE);
                        assert!(
                            builder.clothline_cloth.is_none() && builder.wall_flag_cloth.is_none()
                        );
                    }
                    assert!((hat.translation.z + 4.0).abs() < 0.000001);
                    assert!((glass.translation.z + 3.86).abs() < 0.000001);
                    assert_eq!(frame.translation, glass.translation);
                    assert_eq!(blueprint.transform.unwrap().translation, frame.translation);
                    let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
                        panic!("缺真实碰撞")
                    };
                    let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
                        panic!("缺真实交互")
                    };
                    let meshes = world.resource::<DecoratorMeshLibrary>();
                    let key = DecoratorLibKey::new(subtype, rank);
                    assert!(Arc::ptr_eq(
                        &collision.mesh,
                        &meshes.collision_record(key).mesh
                    ));
                    assert!(Arc::ptr_eq(
                        &interaction.mesh,
                        &meshes.interaction_record(key).mesh
                    ));
                    assert!((collision.position.z + 3.36).abs() < 0.000001);
                    assert_eq!(collision.position, interaction.position);
                    assert_eq!(blueprint.wall_holes.len(), 3);
                    assert_eq!(blueprint.dormer_holes.len(), usize::from(subtype == 7));
                    assert!(
                        blueprint.cloth_info.is_none()
                            && blueprint.window_auto_clutter_candidates.is_empty()
                    );
                }
            }
        }
    }

    #[test]
    fn native_three_hole_tables_are_exact_finite_words() {
        let expected = [
            [
                [0xbe93_3333, 0x3f32_fec5, 0x3faa_c083],
                [0x3f0a_5e35, 0x3f16_bb99, 0x3efd_2f1b],
                [0x3f68_9375, 0x3eb8_4b5d, 0x3eb0_a3d7],
            ],
            [
                [0xbdc8_b43a, 0x3f95_8106, 0x3fd9_999a],
                [0x3f80_4189, 0x3f8b_e76d, 0x3f07_ae14],
                [0x3fad_0e56, 0x3f1c_ed91, 0x3e99_999a],
            ],
            [
                [0x3e6b_851f, 0x3fc1_0625, 0x4016_6666],
                [0x3fca_3d71, 0x3f9f_1aa0, 0x3ec2_8f5c],
                [0x3fea_3d71, 0x3f50_20c5, 0x3eae_147b],
            ],
        ];
        for rank in 1..=3 {
            let mut world = producer_world(6, rank, 1, 0, 0);
            world.run_system_once(generate_gothic_wall_windows).unwrap();
            let blueprints = world.resource::<DecoratorBlueprints>();
            for (hole, row) in blueprints
                .get(7)
                .unwrap()
                .wall_holes
                .iter()
                .zip(expected[usize::from(rank - 1)])
            {
                assert_eq!(hole.center[0].to_bits(), 4.0_f32.to_bits());
                assert_eq!(
                    hole.center[1].to_bits(),
                    (f32::from_bits(row[0]) + 2.0).to_bits()
                );
                assert_eq!(hole.dimensions.map(f32::to_bits), [row[1], row[2]]);
                assert_eq!(hole.kind, 0);
            }
            assert!(
                blueprints
                    .get(7)
                    .unwrap()
                    .visual_entities
                    .iter()
                    .all(|builder| !builder.semi_transparent)
            );
        }
    }

    #[test]
    fn freeform_uses_unshifted_pose_without_glass_extra_and_special_color_changes_only_frame_shader()
     {
        let mut world = producer_world(7, 3, 2, 0, 0);
        change_info(&mut world, |info| {
            info.u40 = 0;
            info.u28 = 999;
        });
        world.run_system_once(generate_gothic_wall_windows).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 2);
        let hat = &blueprint.visual_entities[0];
        let frame = &blueprint.visual_entities[1];
        assert_eq!(hat.translation, frame.translation);
        assert_eq!(hat.nani_name, "nani_gothic_window_bricks");
        assert_eq!(frame.nani_name, "nani_solid_vertex_color_bricks");
        assert!(
            blueprint
                .visual_entities
                .iter()
                .all(|builder| builder.window_light.is_none() && builder.albedo_map.is_none())
        );
        assert!(blueprint.dormer_holes.is_empty());
        assert_eq!(blueprint.wall_holes.len(), 3);
    }

    #[test]
    fn actual_gate_masks_subtype_and_rejects_destination_before_required_wall_or_mesh_lookup() {
        for (subtype, destination_tag) in [(0, 0_u8), (5, 1), (6, 2), (7, 255)] {
            let mut world = producer_world(subtype, 1, 1, 0, 0);
            change_info(&mut world, |info| {
                info.u20 = (info.u20 & !0x00ff_0000) | (u32::from(destination_tag) << 16);
            });
            world.insert_resource(PublicWalls::default());
            world.insert_resource(DecoratorMeshLibrary::default());
            world.run_system_once(generate_gothic_wall_windows).unwrap();
            assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        }
        let mut world = producer_world(0x26, 1, 1, 0, 2);
        world.run_system_once(generate_gothic_wall_windows).unwrap();
        assert_eq!(
            world
                .resource::<DecoratorBlueprints>()
                .get(7)
                .unwrap()
                .visual_entities[0]
                .mesh_name,
            "decorators/window_gothic_1x1_hat"
        );
        let blueprints = world.resource::<DecoratorBlueprints>();
        assert_eq!(blueprints.get(7).unwrap().interaction_source_flags, 2);
        assert!(blueprints.get(7).unwrap().visual_entities[0].semi_transparent);
    }

    #[test]
    fn real_bent_wall_uses_independent_center_normal_and_non_circle_rank_one_keeps_full_width() {
        let walls = published_wall(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], false, 2, 0);
        let info = DecoratorInfo {
            rank: 1,
            ..DecoratorInfo::default()
        };
        let bones = get_gothic_hat_bones(walls.get(WALL).unwrap(), &info, 1.5, 3.0, 2.0);
        assert_eq!(bones.first, Vec3::new(0.5, 3.0, 0.0));
        assert_eq!(bones.center, Vec3::new(1.0, 3.0, 0.5));
        assert_eq!(bones.second, Vec3::new(1.0, 3.0, 1.0));
        assert_eq!(bones.side, [-0.0, 1.0, -1.0, 0.0]);
        assert_eq!(bones.edge, [-1.0, 0.0]);
        let raw = bones.deformation_bytes();
        assert_eq!(
            u32::from_le_bytes(raw[8..12].try_into().unwrap()),
            (-1.0_f32).to_bits()
        );
        assert_eq!(
            u32::from_le_bytes(raw[24..28].try_into().unwrap()),
            0.5_f32.to_bits()
        );
        assert_eq!(
            u32::from_le_bytes(raw[36..40].try_into().unwrap()),
            1.0_f32.to_bits()
        );
    }

    #[test]
    fn circle_rank_one_uses_its_own_width_adjustment() {
        let walls = published_wall(vec![[0.0, 0.0], [10.0, 0.0]], false, 0, 0);
        let info = DecoratorInfo {
            rank: 1,
            ..DecoratorInfo::default()
        };
        let bones = get_gothic_hat_bones(walls.get(WALL).unwrap(), &info, 4.0, 2.0, 2.0);
        assert!((bones.first.x - 3.025).abs() < 0.000001);
        assert!((bones.second.x - 4.975).abs() < 0.000001);
    }

    #[test]
    fn missing_initial_collision_or_invalid_rank_fails_before_blueprint_insert() {
        let mut missing = producer_world(6, 1, 1, 0, 0);
        missing
            .resource_mut::<DecoratorMeshLibrary>()
            .collision
            .clear();
        run_fails(&mut missing);
        assert!(missing.resource::<DecoratorBlueprints>().map.is_empty());
        let mut invalid = producer_world(6, 0, 1, 0, 0);
        run_fails(&mut invalid);
        assert!(invalid.resource::<DecoratorBlueprints>().map.is_empty());
    }

    #[test]
    fn missing_premade_window_extra_keeps_already_published_hat_and_no_later_outputs() {
        let mut world = producer_world(6, 2, 1, 0, 0);
        change_info(&mut world, |info| info.u40 = 0);
        run_fails(&mut world);
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 1);
        assert_eq!(
            blueprint.visual_entities[0].mesh_name,
            "decorators/window_gothic_2x1_hat"
        );
        assert!(blueprint.transform.is_none() && blueprint.wall_holes.is_empty());
        assert!(matches!(
            blueprint.collision_mesh,
            BlueprintMeshSlot::Unspecified
        ));
    }

    #[test]
    fn missing_interaction_keeps_all_three_visuals_without_colliders_transform_or_holes() {
        let mut world = producer_world(7, 2, 1, 0, 1);
        world
            .resource_mut::<DecoratorMeshLibrary>()
            .interaction
            .clear();
        run_fails(&mut world);
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = blueprints.get(7).unwrap();
        assert_eq!(blueprint.visual_entities.len(), 3);
        assert!(
            blueprint.transform.is_none()
                && blueprint.wall_holes.is_empty()
                && blueprint.dormer_holes.is_empty()
        );
        assert!(matches!(
            blueprint.collision_mesh,
            BlueprintMeshSlot::Unspecified
        ));
        assert!(matches!(
            blueprint.interaction_mesh,
            BlueprintMeshSlot::Unspecified
        ));
    }

    #[test]
    fn gothic_dormer_uses_real_bounds_center_wall_id_and_native_rank_tail_tables() {
        let tails = [
            [0x4000_0000, 0xbc34_3958],
            [0x400c_cccd, 0xbe06_24dd],
            [0x3fec_cccd, 0x3d71_a9fc],
        ];
        for rank in 1..=3 {
            let mut world = producer_world(7, rank, 1, 0, 0);
            world.run_system_once(generate_gothic_wall_windows).unwrap();
            let blueprints = world.resource::<DecoratorBlueprints>();
            let hole = blueprints.get(7).unwrap().dormer_holes[0].0;
            assert_eq!(hole[3], 42);
            let native_widths = [0x3f73_8716, 0x3fbf_3e03, 0x3ff6_4e8b];
            assert_eq!(
                &hole[4..8],
                &[
                    native_widths[usize::from(rank - 1)],
                    2.0_f32.to_bits(),
                    0x3f90_a3d7,
                    0
                ]
            );
            assert_eq!(&hole[10..12], &tails[usize::from(rank - 1)]);
            assert!((f32::from_bits(hole[1]) - 3.0).abs() < 0.000001);
            assert!((f32::from_bits(hole[2]) + 3.61).abs() < 0.000001);
        }
        let mut world = producer_world(7, 1, 1, 0, 0);
        world
            .resource_mut::<DecoratorMeshLibrary>()
            .collision
            .get_mut(&DecoratorLibKey::new(7, 1))
            .unwrap()
            .bounds
            .footprint
            .size
            .x = f32::NAN;
        world.run_system_once(generate_gothic_wall_windows).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        assert_eq!(blueprints.get(7).unwrap().dormer_holes[0].0[4], 0x3f73_8716);
        assert_eq!(blueprints.get(7).unwrap().wall_holes.len(), 3);
    }
}
