//! B14127cf70：墙、屋顶、地形与楼梯四分支的真实装饰交互意图。
//! 原完整8546B及四项相对跳表已独立校验；安全typed输出只承载激活载荷，
//! 不沿用旧0x98伪布局。目的、独立ID、合并rank与原rank分别保留。
use crate::collision_detection::{WallIntersectCandidate, intersect_wall_decorators_for_intent};
use crate::merge_proposal_roof_decorators::{
    find_best_roof_decorator_merge_position, intersect_roof_decorators,
};
use crate::merge_proposal_wall_decorators::{
    WallMergeEnv, find_best_wall_decorator_merge_position_in, wall_space_of,
};
use crate::ui_systems::calculate_decorator_dst::RaycastDecoratorDst;
use crate::ui_systems::move_ops::MoveDstCandidate;
use bevy_ecs::system::Query;
use country_core::resources::roofs::Roof;
use country_core::resources::stairs::stairs_assembly_params::{
    PlayermadeStairsAssemblyParams, PlayermadeStairsAssemblyRequests,
};
use country_core::resources::walls::WallId;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{
    DecoratorId, DecoratorInfo, DecoratorRotationXZ, RoofDecoratorDst, StairDecoratorDst,
    TerrainDecoratorDst,
};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorInfoView,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::utils::debug_viz::{DebugLine, DebugSphere};
use glam::{Quat, Vec2, Vec3};
use utils::comparative_space::ComparativeSpace;
use utils::wall_space::{CurveSampling, WallSpaceCoord};

/// 原候选+0 quaternion、+10目的、+20世界点、+2c Option屋顶坐标。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorIntentCandidate {
    pub rotation: Quat,
    pub destination: RaycastDecoratorDst,
    pub position: Vec3,
    pub roof_coordinate: Option<Vec2>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecoratorIntentMerge {
    pub target: DecoratorId,
    pub rank: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AcceptedDecoratorIntent {
    /// 原+00的源地址与+10的目标地址独立；跨墙、屋顶、地形迁移不可混用。
    pub storage: DecoratorStorageAddr,
    pub target_storage: DecoratorStorageAddr,
    pub destination: MoveDstCandidate,
    pub merge: Option<DecoratorIntentMerge>,
    pub decorator_id: DecoratorId,
    pub position: Vec3,
    pub rank: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorInteractionIntent {
    Accepted(AcceptedDecoratorIntent),
    /// 原tag3只有世界位置；不能以普通无效朝向替代这条图标反馈。
    UnsupportedTerrain {
        position: Vec3,
    },
    /// 原tag4保留完整原朝向与世界位置。
    Rejected {
        rotation: Quat,
        position: Vec3,
    },
}

/// B原参数所有权不变：roofs、stairs、storage、derived两owner、cached均直接借用。
/// allow_merge也参与原阳台门转门门禁；flat_only还影响Wall/WallCompound选择。
#[allow(clippy::too_many_arguments)]
pub fn calculate_decorator_interaction_intent(
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    stairs: &PlayermadeStairsAssemblyRequests,
    candidate: DecoratorIntentCandidate,
    decorator_id: DecoratorId,
    info: &DecoratorInfo,
    excluded: &[u32],
    storage: &DecoratorStorage,
    derived: DerivedDecoratorInfoView<'_>,
    cached: &CachedDecoratorTransforms,
    storage_addr: DecoratorStorageAddr,
    allow_merge: bool,
    flat_only: bool,
    stream: &mut UiSignifierStream,
    world: &RaycastWorld,
) -> DecoratorInteractionIntent {
    assert!(
        candidate.position.is_finite(),
        "assertion failed: pos.is_finite()"
    );
    assert!(
        candidate.rotation.is_finite(),
        "assertion failed: rotation.is_finite()"
    );
    let rejected = || DecoratorInteractionIntent::Rejected {
        rotation: candidate.rotation,
        position: candidate.position,
    };
    let accepted = |destination, merge| {
        DecoratorInteractionIntent::Accepted(AcceptedDecoratorIntent {
            storage: storage_addr,
            target_storage: match candidate.destination {
                RaycastDecoratorDst::Wall(wall) => DecoratorStorageAddr::Wall(wall),
                RaycastDecoratorDst::Roof { payload, .. } => DecoratorStorageAddr::Wall(WallId(payload)),
                RaycastDecoratorDst::Terrain => DecoratorStorageAddr::Terrain,
                RaycastDecoratorDst::StairTop(_) => DecoratorStorageAddr::Stair,
                RaycastDecoratorDst::Invalid => unreachable!("无效候选不能被接受"),
            },
            destination,
            merge,
            decorator_id,
            position: candidate.position,
            rank: info.rank,
        })
    };
    match candidate.destination {
        RaycastDecoratorDst::Wall(wall_id) => {
            let wall = walls.get(wall_id).unwrap();
            let space = wall_space_of(wall);
            let subtype = crate::derive_decorator_subtype::derive_decorator_subtype(
                info,
                storage_addr,
                derived,
                walls,
                roofs,
            );
            let metrics = derived
                .mesh_library()
                .collision_record(DecoratorLibKey::new(subtype.0, info.rank));
            let width = metrics.bounds.footprint.size.x;
            let height = metrics.bounds.top - metrics.bounds.bottom;
            let curve = wall.simplified_curve_2d();
            let source_u = intent_clamp01(
                curve.approximate_u(Vec2::new(candidate.position.x, candidate.position.z)),
            );
            let source_coord =
                WallSpaceCoord::new_unchecked(source_u * curve.length, candidate.position.y);
            let mut coordinate = source_coord;
            let mut corner = 0;
            let mut platform = WallId(0);
            let compound = wall.shape().tag() < 2 || !flat_only;
            if allow_merge {
                if let Some(bucket) = storage.wall_decorators(wall_id) {
                    let shape = WallIntersectCandidate {
                        coord: coordinate,
                        along_width: width,
                        vertical_extent: height,
                    };
                    let intersections = intersect_wall_decorators_for_intent(
                        bucket,
                        decorator_id,
                        shape,
                        &space,
                        derived,
                    );
                    let env = WallMergeEnv {
                        mesh_lib: derived.mesh_library(),
                        derived_subtypes: derived.subtype_library(),
                        wall_space: &space,
                        current_id: decorator_id,
                    };
                    if let Some(proposal) = find_best_wall_decorator_merge_position_in(
                        wall_id,
                        &intersections,
                        info,
                        storage,
                        &env,
                        shape,
                        excluded,
                    ) {
                        if space.project(0.0 + proposal.coord.u).is_none() {
                            return rejected();
                        }
                        return accepted(
                            MoveDstCandidate::Wall {
                                storage: DecoratorStorageAddr::Wall(wall_id),
                                coordinate: proposal.coord,
                                platform: proposal.destination.platform,
                                corner_attachment: proposal.destination.corner_attachment,
                                bottom_as_door: proposal.destination.bottom_as_door,
                                compound: proposal.compound,
                            },
                            Some(DecoratorIntentMerge {
                                target: DecoratorId(proposal.target_id),
                                rank: proposal.info.rank,
                            }),
                        );
                    }
                }
            }
            if wall.shape().tag() == 1 {
                let bytes = wall.shape().bytes();
                let f = |o: usize| f32::from_le_bytes(bytes[o..o + 4].try_into().unwrap());
                let rectangle = utils::geometry::Rectangle2d {
                    axis: Vec2::new(f(4), f(8)),
                    center: Vec2::new(f(12), f(16)),
                    size: Vec2::new(f(20), f(24)),
                };
                for id in 0..4 {
                    let u = space.project_clamp(rectangle.get_point_u(id) * curve.length);
                    if crate::cull_oob_decorators::should_snap_to_corner(
                        info.decorator_type(),
                        wall.building_projection().is_halftimbered(),
                        wall,
                        WallSpaceCoord::new_unchecked(u, source_coord.y),
                        source_coord,
                        width,
                    ) {
                        coordinate.u = u;
                        corner = id as u8 + 1;
                        platform = WallId(0);
                        break;
                    }
                }
            }
            // 原支持探针的位置仍由光标在原曲线上的投影取得，不用角吸附后的u代换。
            let (segment, t) = curve.coord_at_u(source_u);
            let a = curve.points[segment];
            let b = curve.points[segment + 1];
            let position = Vec3::new(
                b[0] * t + a[0] * (1.0 - t),
                candidate.position.y,
                b[1] * t + a[1] * (1.0 - t),
            );
            let direction = DecoratorRotationXZ::negative_z_from_quat(candidate.rotation);
            let snap = snap_balcony_door(
                false,
                info,
                position,
                Vec2::new(direction.x, direction.z),
                source_coord.u,
                wall_id,
                wall,
                roofs,
                derived.mesh_library(),
                false,
                world,
            );
            let mut bottom_as_door = false;
            match snap {
                BalconyDoorSnap::AdjacentFlatRoof { wall, height }
                | BalconyDoorSnap::WallOutline { wall, height }
                | BalconyDoorSnap::ColliderSource13 { wall, height } => {
                    let old_support = info.try_dst_as_wall().is_some_and(|old| {
                        let dst = old.destination();
                        let wall = dst.as_wall();
                        wall.platform.is_valid() || wall.bottom_as_door
                    });
                    if old_support || allow_merge {
                        coordinate.y = height;
                        platform = wall;
                        bottom_as_door = matches!(snap, BalconyDoorSnap::WallOutline { .. });
                    }
                }
                BalconyDoorSnap::Unavailable | BalconyDoorSnap::RejectedUpperClearance => {}
            }
            if space.project(0.0 + coordinate.u).is_none() {
                return rejected();
            }
            accepted(
                MoveDstCandidate::Wall {
                    storage: DecoratorStorageAddr::Wall(wall_id),
                    coordinate,
                    platform,
                    corner_attachment: corner,
                    bottom_as_door,
                    compound,
                },
                None,
            )
        }
        RaycastDecoratorDst::Roof { payload, .. } => {
            if !info.decorator_type().can_place_on_roof() {
                return rejected();
            }
            let wall_id = WallId(payload);
            // 原required wall/roof/query三层读取分别失败，不借空屋顶掩盖注册缺口。
            walls.get(wall_id).unwrap();
            let roof = roofs.get(walls.roof_entity(wall_id).unwrap()).unwrap();
            if flat_only && !roof.is_flat() {
                return rejected();
            }
            let ty = info.ty;
            let mut rotation = if (!ty & 0x3e) == 0 || (!ty & 0x38) != 0 {
                DecoratorRotationXZ::from_quat(candidate.rotation).0
            } else {
                0.0
            } - roof.rotation_yaw_radians();
            let mut coord = candidate.roof_coordinate.unwrap();
            let mut merge = None;
            if ty == 0x3d {
                if !roof.is_flat() {
                    use country_core::systems::wall::shapes::shape_placement::{
                        RoofRidgeSnap, snap_to_roof_ridge,
                    };
                    // 原第一次以4.0距离绘制可吸附脊；第二次才用1.0吸附。
                    match snap_to_roof_ridge(roof, candidate.position, 4.0) {
                        RoofRidgeSnap::Point(point) => {
                            DebugSphere::new(point).radius(0.2).solid().show(stream)
                        }
                        RoofRidgeSnap::Segment { a, b, .. } => {
                            DebugLine::new(vec![a + Vec3::Y * 0.2, b + Vec3::Y * 0.2])
                                .line_width(5.0)
                                .dashed()
                                .show(stream)
                        }
                        RoofRidgeSnap::None => {}
                    }
                    match snap_to_roof_ridge(roof, candidate.position, 1.0) {
                        RoofRidgeSnap::None => {}
                        RoofRidgeSnap::Point(_) => {
                            coord = Vec2::new(0.05, 2.05);
                            rotation = 0.0;
                        }
                        RoofRidgeSnap::Segment { t, .. } => {
                            coord = Vec2::new(t, 2.05);
                            rotation = 0.0;
                        }
                    }
                }
            } else {
                if coord.y > 1.0 && !roof.is_flat() {
                    return rejected();
                }
                let subtype = crate::derive_decorator_subtype::derive_decorator_subtype(
                    info,
                    storage_addr,
                    derived,
                    walls,
                    roofs,
                );
                let collision = derived
                    .mesh_library()
                    .collision_record(DecoratorLibKey::new(subtype.0, info.rank));
                if allow_merge && !roof.is_flat() && info.decorator_type().max_rank() > 1 {
                    if let Some(bucket) = storage.wall_decorators(wall_id) {
                        let intersections = intersect_roof_decorators(
                            info,
                            candidate.position,
                            candidate.rotation,
                            collision.bounds.footprint.size.x,
                            collision.bounds.top - collision.bounds.bottom,
                            bucket,
                            derived,
                            cached,
                            decorator_id,
                        );
                        if let Some(proposal) = find_best_roof_decorator_merge_position(
                            info,
                            coord,
                            candidate.position,
                            roof,
                            bucket,
                            derived,
                            &intersections,
                        ) {
                            coord = proposal.coord;
                            rotation = proposal.rotation.0;
                            merge = Some(DecoratorIntentMerge {
                                target: proposal.target_id,
                                rank: proposal.info.rank,
                            });
                        }
                    }
                }
            }
            accepted(
                MoveDstCandidate::Roof(RoofDecoratorDst {
                    coord,
                    yaw: DecoratorRotationXZ(rotation),
                }),
                merge,
            )
        }
        RaycastDecoratorDst::Terrain => {
            if !info.decorator_type().can_place_on_terrain() {
                return DecoratorInteractionIntent::UnsupportedTerrain {
                    position: candidate.position,
                };
            }
            accepted(
                MoveDstCandidate::Terrain(TerrainDecoratorDst {
                    position_xz: Vec2::new(candidate.position.x, candidate.position.z),
                    yaw: DecoratorRotationXZ::from_quat(candidate.rotation),
                }),
                None,
            )
        }
        RaycastDecoratorDst::StairTop(packed) => {
            // 原此层调用can_place_on_roof，不是另一个名字相似的stair_top谓词。
            if !info.decorator_type().can_place_on_roof() {
                return rejected();
            }
            let a = packed as u32;
            let b = (packed >> 32) as u32;
            let request = stairs
                .requests
                .iter()
                .find(|request| {
                    let (first, second) = request.segment.nodes();
                    first.0 == a && second.0 == b
                })
                .unwrap_or_else(|| {
                    panic!("missing stair request ({a}, {b}): {:?}", stairs.requests)
                });
            let coordinate = intent_stair_coordinate(
                request,
                Vec2::new(candidate.position.x, candidate.position.z),
            );
            accepted(
                MoveDstCandidate::Stair(StairDecoratorDst {
                    first_id: a,
                    second_id: b,
                    coordinate: coordinate.0,
                    yaw: DecoratorRotationXZ::from_quat(candidate.rotation)
                        .offset(-DecoratorRotationXZ::from_xz_forward_vector(coordinate.1).0),
                }),
                None,
            )
        }
        RaycastDecoratorDst::Invalid => unreachable!("invalid raycast decorator destination"),
    }
}

fn intent_clamp01(value: f32) -> f32 {
    // 原MAXSS 0,value后MINSS 1,result，unordered返回第二operand。
    let lower = if value < 0.0 { 0.0 } else { value };
    if lower > 1.0 { 1.0 } else { lower }
}

/// 原4轮8点夹逼：以横向cross绝对值+沿切线残差平方定位，不能换最近线段。
fn intent_stair_coordinate(
    request: &PlayermadeStairsAssemblyParams,
    position: Vec2,
) -> (Vec2, Vec2) {
    let sample = |u: f32| {
        let c = request.curve.get_coord_at_u(u);
        let a = request.tangents[c.segment];
        let b = request.tangents[c.segment + 1];
        let tangent = Vec2::new((b.x - a.x) * c.t + a.x, (b.y - a.y) * c.t + a.y);
        let a = request.curve.points[c.segment];
        let b = request.curve.points[c.segment + 1];
        let delta = b * c.t + a * (1.0 - c.t) - position;
        let cross = tangent.x * delta.y - tangent.y * delta.x;
        let x = delta.x + cross * tangent.y;
        let y = delta.y - cross * tangent.x;
        cross.abs() + (y * y + x * x)
    };
    let mut lower = 0.0;
    let mut upper = 1.0;
    let mut values = [0.0; 8];
    values[0] = sample(0.0);
    values[7] = sample(1.0);
    for _ in 0..4 {
        let range = upper - lower;
        for i in 1..7 {
            values[i] = sample((i as f32 * f32::from_bits(0x3e124925)) * range + lower);
        }
        let mut best = 0;
        for i in 1..8 {
            if values[i].partial_cmp(&values[best]).unwrap() == std::cmp::Ordering::Less {
                best = i;
            }
        }
        let a = best.saturating_sub(1);
        let b = best.min(6) + 1;
        let next_lower = (a as f32 * f32::from_bits(0x3e124925)) * range + lower;
        let next_upper = (b as f32 * f32::from_bits(0x3e124925)) * range + lower;
        values[0] = values[a];
        values[7] = values[b];
        lower = next_lower.min(next_upper);
        upper = next_lower.max(next_upper);
    }
    let u = (upper + lower) * 0.5;
    assert!(
        u >= 0.0 && u <= 1.0,
        "original stair coordinate outside0..1"
    );
    let c = request.curve.get_coord_at_u(u);
    let a = request.tangents[c.segment];
    let b = request.tangents[c.segment + 1];
    let tangent = Vec2::new((b.x - a.x) * c.t + a.x, (b.y - a.y) * c.t + a.y);
    let inverse = 1.0 / (tangent.y * tangent.y + tangent.x * tangent.x).sqrt();
    let normal = if inverse.is_finite() && inverse > 0.0 {
        Vec2::new(-tangent.y * inverse, tangent.x * inverse)
    } else {
        Vec2::new(-0.0, 0.0)
    };
    let a = request.curve.points[c.segment];
    let b = request.curve.points[c.segment + 1];
    let center = b * c.t + a * (1.0 - c.t);
    let half_width = ((request.widths[c.segment + 1] - request.widths[c.segment]) * c.t
        + request.widths[c.segment])
        * 0.5;
    let dx = position.x - center.x;
    let dz = position.y - center.y;
    (
        Vec2::new((dz * normal.y + dx * normal.x) / half_width, u),
        normal,
    )
}

/// Observed B Snap result; semantic Rust values carry no native ABI promise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BalconyDoorSnap {
    AdjacentFlatRoof {
        wall: WallId,
        height: f32,
    },
    WallOutline {
        wall: WallId,
        height: f32,
    },
    /// The selected original ray result has source tag13; its name is unknown.
    ColliderSource13 {
        wall: WallId,
        height: f32,
    },
    Unavailable,
    RejectedUpperClearance,
}

impl BalconyDoorSnap {
    pub fn native_tag(self) -> u8 {
        match self {
            Self::AdjacentFlatRoof { .. } => 0,
            Self::WallOutline { .. } => 1,
            Self::ColliderSource13 { .. } => 2,
            Self::Unavailable => 3,
            Self::RejectedUpperClearance => 4,
        }
    }
}

/// B14127f0e0: preserve the five states, actual footprint/support sources and
/// finite source13 ray. The caller supplies the existing direction unchanged.
/// Current UI calculate is separately unrestored; location uses true/true and
/// observes only RejectedUpperClearance before continuing its other checks.
#[allow(clippy::too_many_arguments)]
pub fn snap_balcony_door(
    require_upper_clearance: bool,
    info: &DecoratorInfo,
    world_pos: Vec3,
    probe_direction_xz: Vec2,
    wall_distance: f32,
    wall_id: WallId,
    wall: &PublicWallState,
    roofs: &Query<&Roof>,
    meshes: &DecoratorMeshLibrary,
    prefer_wall_outline: bool,
    world: &RaycastWorld,
) -> BalconyDoorSnap {
    let (subtype, baseline_subtype) = match info.decorator_type().0 {
        0x38 => (4, 0),
        0x39 => (9, 6),
        _ => return BalconyDoorSnap::Unavailable,
    };
    // Both lookups precede all geometry/query work; missing original data is
    // an invariant failure, rather than a default box or unavailable result.
    let full_height = snap_collision_height(meshes, DecoratorLibKey::new(subtype, info.rank()));
    let baseline_height =
        snap_collision_height(meshes, DecoratorLibKey::new(baseline_subtype, info.rank()));
    let half_height = full_height * 0.5;
    let world_xz = Vec2::new(world_pos.x, world_pos.z);
    let probe = world_xz + probe_direction_xz * 0.2;
    // Public+280 comes from raw source+58, retained in this public snapshot.
    let elevation = wall.authority.elevation();
    let lower = snap_max_number(elevation - 0.4, world_pos.y - half_height - 0.4);
    let mut selected = None;
    for roof in roofs.iter() {
        if !roof.is_flat() || roof.wall_id() == wall_id {
            continue;
        }
        let support_y = roof.floor_y() - 0.79;
        if !(world_pos.y > support_y && support_y >= lower) {
            continue;
        }
        if !snap_roof_contains(roof, world_xz, probe) {
            continue;
        }
        selected = Some(BalconyDoorSnap::AdjacentFlatRoof {
            wall: roof.wall_id(),
            height: snap_max_number(elevation, support_y) + half_height,
        });
        break;
    }
    let selected = match selected {
        Some(selected) => selected,
        None => {
            let outline = wall.height_outline();
            let (i, t) = snap_curve_coord(outline, snap_nonnegative(wall_distance));
            let outline_y =
                snap_nonnegative(outline[i][1] + (outline[i + 1][1] - outline[i][1]) * t);
            let ray_base_y = if prefer_wall_outline {
                world_pos.y - half_height
            } else {
                world_pos.y + baseline_height * -0.5
            };
            let threshold = if prefer_wall_outline {
                ray_base_y - 0.1
            } else {
                ray_base_y
            };
            if outline_y > threshold {
                BalconyDoorSnap::WallOutline {
                    wall: wall_id,
                    height: snap_max_number(half_height, outline_y + half_height + 0.05),
                }
            } else {
                let ray_xz = world_xz + probe_direction_xz * 0.5;
                let hit = world.raycast_simple_with_max_toi(
                    Vec3::new(ray_xz.x, ray_base_y + 0.5, ray_xz.y),
                    Vec3::new(-0.0, -1.0, -0.0),
                    1.0,
                    0x1000,
                );
                if hit.handle[0] as u8 != 13 {
                    return BalconyDoorSnap::Unavailable;
                }
                BalconyDoorSnap::ColliderSource13 {
                    wall: wall_id,
                    height: snap_max_number(half_height, hit.position.y + half_height + 0.05),
                }
            }
        }
    };
    if !require_upper_clearance {
        return selected;
    }
    let (i, t) = snap_curve_coord(wall.height_outline(), snap_nonnegative(wall_distance));
    let heights = wall
        .arch_segment_heights()
        .expect("wall upper height cache is missing");
    // Original checks only the two selected indices, not whole-vector equality.
    let ceiling = heights[i] + (heights[i + 1] - heights[i]) * t;
    let mut roof_margin = 0.0;
    // A fresh query traversal is required even after selecting a foreign roof.
    for roof in roofs.iter() {
        if roof.is_flat() && roof.wall_id() == wall_id {
            roof_margin = -0.79;
        }
    }
    if world_pos.y + half_height >= ceiling + roof_margin {
        BalconyDoorSnap::RejectedUpperClearance
    } else {
        selected
    }
}

fn snap_collision_height(meshes: &DecoratorMeshLibrary, key: DecoratorLibKey) -> f32 {
    let collision = meshes.collision_record(key);
    // 同一canonical记录的真实垂直界；不复制三角形或重造bbox。
    collision.bounds.top - collision.bounds.bottom
}

fn snap_roof_contains(roof: &Roof, point: Vec2, probe: Vec2) -> bool {
    let bytes = roof.observed();
    let f = |at| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    if bytes[8] & 1 != 0 {
        // Borrow the actual footprint. Rectangle::new would add unrelated
        // validation that the original contains_point_padding call lacks.
        let footprint = utils::geometry::Rectangle2d {
            axis: Vec2::new(f(12), f(16)),
            center: Vec2::new(f(20), f(24)),
            size: Vec2::new(f(28), f(32)),
        };
        footprint.contains_point_padding(point, Vec2::splat(0.8))
            && footprint.contains_point_padding(probe, Vec2::splat(0.8))
    } else {
        let footprint = utils::geometry::Circle2d::new(Vec2::new(f(12), f(16)), f(20));
        footprint.signed_distance(point) < 0.4 && footprint.signed_distance(probe) < 0.0
    }
}

// MAXSS's right operand wins ties/unordered; the native NaN select keeps the
// numeric left operand when the right one isNaN.
fn snap_max_number(left: f32, right: f32) -> f32 {
    if right.is_nan() || left > right {
        left
    } else {
        right
    }
}
fn snap_nonnegative(value: f32) -> f32 {
    if value > 0.0 { value } else { 0.0 }
}

// The original921B CurveSlice specialization over projected [[f32;3]] input.
// Keep its f32 residual/endpoint order, without allocating Vec<Vec3> or casting
// array storage. The same borrowed publication feeds both samplings.
fn snap_curve_coord(points: &[[f32; 3]], distance: f32) -> (usize, f32) {
    assert!(!points.is_empty(), "original curve is empty");
    points.get(1).expect("original curve needs a second point");
    assert!(distance >= 0.0, "invalid extension distance");
    let length = |i: usize| {
        let x = points[i + 1][0] - points[i][0];
        let z = points[i + 1][2] - points[i][2];
        (z * z + x * x).sqrt()
    };
    let mut i = 0;
    let mut segment_length = length(i);
    let mut balance = segment_length - distance;
    loop {
        if balance >= 0.0 {
            return (i, 1.0 - balance / segment_length);
        }
        if i + 2 >= points.len() {
            return (i, 1.0);
        }
        i += 1;
        segment_length = length(i);
        balance += segment_length;
    }
}

#[cfg(test)]
mod snap_balcony_door_regressions {
    use super::*;
    use bevy_ecs::world::World;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::WallState;
    use country_core::resources::walls::height_cache::RegularWallHeightCache;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::systems::collision::world::{
        ColliderMeshInstance, ColliderUpsert, RaycastGeoSource, RaycastMeshData, upsert_collider,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    // Actual same-World Bevy Roof query, mesh resource and real Rapier world.
    // Populated fixture data is not the missing game mesh-loader integration.
    fn scene() -> World {
        let mut ecs = World::new();
        let mut meshes = DecoratorMeshLibrary::default();
        for (subtype, height) in [(4, 2.0), (0, 4.0), (9, 4.0), (6, 6.0)] {
            meshes.insert_collision_and_interaction_as_same(
                DecoratorLibKey::new(subtype, 1),
                {
                    use country_core::resources::walls::decorator_subtype::DecoratorCollision;
                    use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
                    use country_core::utils::components::transform::Transform;
                    let size = Vec3::new(1.0, height, 1.0);
                    let center = size * 0.5;
                    let transform = Transform { translation: center, ..Transform::identity() };
                    DecoratorCollision {
                        triangles: box_trigger_volume(1.0, height, 1.0, &transform),
                        mesh: Arc::new(RaycastMeshData::new_cuboid_at(size, center)),
                        bounds: utils::geometry::Rectangle3d::new(center, size, Vec2::X),
                    }
                },
                country_core::resources::walls::decorator_subtype::DecoratorOutline::None,
                country_core::resources::walls::decorator_subtype::DecoratorCollisionOutline::None,
            );
        }
        ecs.insert_resource(meshes);
        ecs.insert_resource(RaycastWorld::new());
        ecs
    }
    fn info(dtype: u8) -> DecoratorInfo {
        DecoratorInfo {
            ty: dtype,
            rank: 1,
            ..Default::default()
        }
    }
    fn published_wall(elevation: f32, ceiling: f32) -> PublicWallState {
        let points = (0..=16)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / 16.0;
                [-4.0 + 4.0 * angle.cos(), 4.0 * angle.sin()]
            })
            .collect();
        let authority = WallState::new(Curve2::try_new(points, false).unwrap(), 3.0, 0, 2, false);
        let (mut snapshot, freehand) = authority.into_snapshot();
        let mut shape = [0u8; 28];
        shape[4..8].copy_from_slice(&(-4.0f32).to_le_bytes());
        shape[12..16].copy_from_slice(&4.0f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        snapshot.source.terrain_floor = elevation;
        let authority = WallState::from_snapshot(snapshot, freehand);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let mut wall = PublicWallState::collect(&authority, &terrain).unwrap();
        wall.regular_heights = Some(RegularWallHeightCache {
            maximum_y: ceiling,
            arch_segment_heights: vec![ceiling; wall.height_outline().len()],
        });
        wall
    }
    fn roof(wall: u64, floor: f32, rectangle: bool) -> Roof {
        let mut shape = [0u8; 28];
        if rectangle {
            shape[..4].copy_from_slice(&1u32.to_le_bytes());
            shape[4..8].copy_from_slice(&1.0f32.to_le_bytes());
            shape[20..24].copy_from_slice(&1.0f32.to_le_bytes());
            shape[24..28].copy_from_slice(&1.0f32.to_le_bytes());
        } else {
            shape[12..16].copy_from_slice(&1.0f32.to_le_bytes());
        }
        Roof::new(
            wall.to_le_bytes(),
            [0; 28],
            shape,
            floor.to_le_bytes(),
            [0; 4],
            0,
            [0; 12],
        )
    }
    fn invoke(
        ecs: &mut World,
        wall: &PublicWallState,
        info: &DecoratorInfo,
        pos: Vec3,
        upper: bool,
        prefer_outline: bool,
    ) -> BalconyDoorSnap {
        let mut state = ecs.query::<&Roof>();
        let roofs = state.query(&*ecs);
        snap_balcony_door(
            upper,
            info,
            pos,
            Vec2::X,
            0.0,
            WallId(1),
            wall,
            &roofs,
            ecs.resource::<DecoratorMeshLibrary>(),
            prefer_outline,
            ecs.resource::<RaycastWorld>(),
        )
    }
    fn insert_surface(ecs: &mut World, height: f32) -> HashMap<u32, ColliderMeshInstance> {
        let mut instances = HashMap::new();
        let mut physics = ecs.resource_mut::<RaycastWorld>();
        assert!(matches!(
            upsert_collider(
                &mut physics,
                &mut instances,
                1,
                RaycastGeoSource::Slot13,
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.2), Vec3::ZERO)),
                Vec3::new(0.5, height, 0.0),
                glam::Quat::IDENTITY
            ),
            ColliderUpsert::Inserted { .. }
        ));
        physics.flush_colliders();
        instances
    }

    #[test]
    fn unsupported_type_does_not_consult_missing_keys() {
        let mut ecs = scene();
        ecs.resource_mut::<DecoratorMeshLibrary>().collision.clear();
        let out = invoke(
            &mut ecs,
            &published_wall(0.0, 10.0),
            &info(0),
            Vec3::Y * 3.0,
            true,
            true,
        );
        assert_eq!(out, BalconyDoorSnap::Unavailable);
    }

    #[test]
    fn dtype_rank_and_second_bbox_lookup_are_strict() {
        let mut ecs = scene();
        ecs.spawn(roof(2, 3.0, false));
        let wall = published_wall(0.0, 10.0);
        let a = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        let b = invoke(&mut ecs, &wall, &info(0x39), Vec3::Y * 3.0, false, true);
        let BalconyDoorSnap::AdjacentFlatRoof { height: a, .. } = a else {
            panic!("cottage");
        };
        let BalconyDoorSnap::AdjacentFlatRoof { height: b, .. } = b else {
            panic!("gothic");
        };
        assert!((b - a - 1.0).abs() < 1e-6);
        let mut rank2 = info(0x38);
        rank2.rank = 2;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                invoke(&mut ecs, &wall, &rank2, Vec3::Y * 3.0, false, true)
            }))
            .is_err()
        );
        ecs.resource_mut::<DecoratorMeshLibrary>()
            .collision
            .remove(&DecoratorLibKey::new(0, 1));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true)
            }))
            .is_err()
        );
    }

    #[test]
    fn first_other_flat_roof_preserves_id_and_elevation_not_max_y() {
        let mut ecs = scene();
        ecs.spawn(roof(1, 3.0, false));
        let mut pitched = roof(9, 3.0, false);
        pitched.observed_mut()[0x30..0x34].copy_from_slice(&0.2f32.to_le_bytes());
        ecs.spawn(pitched);
        ecs.spawn(roof(2, 3.0, false));
        ecs.spawn(roof(3, 3.2, false));
        let wall = published_wall(0.0, 20.0);
        let out = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        assert_eq!(
            out,
            BalconyDoorSnap::AdjacentFlatRoof {
                wall: WallId(2),
                height: 3.0 - 0.79 + 1.0
            }
        );
    }

    #[test]
    fn rectangle_padding_is_inclusive_and_circle_probe_is_strict() {
        let rectangle = roof(2, 3.0, true);
        assert!(snap_roof_contains(
            &rectangle,
            Vec2::new(0.9, 0.0),
            Vec2::new(0.9, 0.0)
        ));
        assert!(!snap_roof_contains(
            &rectangle,
            Vec2::new(0.9001, 0.0),
            Vec2::ZERO
        ));
        let circle = roof(2, 3.0, false);
        assert!(snap_roof_contains(
            &circle,
            Vec2::new(1.25, 0.0),
            Vec2::new(0.9, 0.0)
        ));
        assert!(!snap_roof_contains(
            &circle,
            Vec2::new(1.5, 0.0),
            Vec2::ZERO
        ));
        assert!(!snap_roof_contains(&circle, Vec2::new(0.8, 0.0), Vec2::X));
    }

    #[test]
    fn upper_compare_uses_input_y_halfheight_and_fresh_same_wall_roof_query() {
        let mut ecs = scene();
        let mut current_roof = roof(1, 50.0, false);
        current_roof.observed_mut()[0x30..0x34].copy_from_slice(&0.2f32.to_le_bytes());
        let current_roof = ecs.spawn(current_roof).id();
        ecs.spawn(roof(2, 3.0, false));
        let mut wall = published_wall(0.0, 4.125);
        let expected = BalconyDoorSnap::AdjacentFlatRoof {
            wall: WallId(2),
            height: 3.0 - 0.79 + 1.0,
        };
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            expected
        );
        wall.regular_heights
            .as_mut()
            .unwrap()
            .arch_segment_heights
            .fill(4.0);
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            BalconyDoorSnap::RejectedUpperClearance
        );
        wall.regular_heights
            .as_mut()
            .unwrap()
            .arch_segment_heights
            .fill(4.5);
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            expected
        );
        ecs.component_mut::<Roof>(current_roof)
            .unwrap()
            .observed_mut()[0x30..0x34]
            .copy_from_slice(&0.0f32.to_le_bytes());
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true),
            BalconyDoorSnap::RejectedUpperClearance
        );
        wall.regular_heights = None;
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true),
            expected
        );
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, true, true)
            }))
            .is_err()
        );
    }

    #[test]
    fn baseline_bbox_and_preference_select_different_real_branches() {
        let mut ecs = scene();
        let wall = published_wall(0.0, 10.0);
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 1.5, false, false),
            BalconyDoorSnap::WallOutline {
                wall: WallId(1),
                height: 1.05
            }
        );
        assert_eq!(
            invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 1.5, false, true),
            BalconyDoorSnap::Unavailable
        );
    }

    #[test]
    fn real_source13_pose_is_bounded_to_one_unit() {
        let mut ecs = scene();
        let _instances = insert_surface(&mut ecs, 1.75);
        let wall = published_wall(0.0, 10.0);
        let out = invoke(&mut ecs, &wall, &info(0x38), Vec3::Y * 3.0, false, true);
        let BalconyDoorSnap::ColliderSource13 { wall, height } = out else {
            panic!("actual source13");
        };
        assert_eq!(wall, WallId(1));
        assert!((height - 2.9).abs() < 1e-5);
        let mut beyond = scene();
        let _instances = insert_surface(&mut beyond, 0.25);
        let wall = published_wall(0.0, 10.0);
        assert!(
            !beyond
                .resource::<RaycastWorld>()
                .raycast_simple(Vec3::new(0.5, 2.5, 0.0), Vec3::NEG_Y, 0x1000)
                .is_miss()
        );
        assert_eq!(
            invoke(&mut beyond, &wall, &info(0x38), Vec3::Y * 3.0, false, true),
            BalconyDoorSnap::Unavailable
        );
    }

    #[test]
    fn projected_distance_keeps_f32_endpoint_zero_and_overrun_policy() {
        let points = [[0.0, 0.0, 0.0], [3.0, 100.0, 4.0], [6.0, 200.0, 8.0]];
        assert_eq!(snap_curve_coord(&points, 2.5), (0, 0.5));
        assert_eq!(snap_curve_coord(&points, 5.0), (0, 1.0));
        assert_eq!(snap_curve_coord(&points, 50.0), (1, 1.0));
        assert_eq!(
            snap_curve_coord(&points, snap_nonnegative(f32::NAN)),
            (0, 0.0)
        );
        assert_eq!(snap_curve_coord(&points, snap_nonnegative(-2.0)), (0, 0.0));
        let zero = [[0.0, 0.0, 0.0], [0.0, 4.0, 0.0], [1.0, 5.0, 0.0]];
        let (i, t) = snap_curve_coord(&zero, 0.0);
        assert_eq!(i, 0);
        assert!(t.is_nan());
        assert_eq!(snap_curve_coord(&zero, 0.5), (1, 0.5));
        for count in [0, 1] {
            assert!(std::panic::catch_unwind(|| snap_curve_coord(&points[..count], 0.0)).is_err());
        }
        assert_eq!(snap_max_number(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(snap_max_number(2.0, f32::NAN), 2.0);
    }
}

use country_core::render::extract::WorldComponentWrite as _;

#[cfg(test)]
mod original_intent_tests {
    use super::*;
    use bevy_ecs::prelude::World;
    use country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor;
    use country_core::resources::stairs::stairs_state::StairSegment;

    pub struct StairIntentOracleCase {
        pub points: &'static [[u32; 2]],
        pub tangents: &'static [[u32; 2]],
        pub widths: &'static [u32],
        pub points_u: &'static [u32],
        pub length: u32,
        pub position: [u32; 3],
        pub expected: [u32; 2],
    }
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../reports/module-recovery-2026-10-23/m01-intent-stair-native-vectors.rs"
    ));
    fn request(case: &StairIntentOracleCase) -> PlayermadeStairsAssemblyParams {
        let points: Vec<_> = case
            .points
            .iter()
            .map(|p| Vec2::from_array(p.map(f32::from_bits)))
            .collect();
        let mut curve = utils::curve::Curve2::crashy_new_from_points(points, false);
        curve.points_u = case.points_u.iter().copied().map(f32::from_bits).collect();
        curve.length = f32::from_bits(case.length);
        PlayermadeStairsAssemblyParams {
            heights: vec![0.0; curve.points.len()],
            curve,
            widths: case.widths.iter().copied().map(f32::from_bits).collect(),
            tangents: case
                .tangents
                .iter()
                .map(|p| Vec2::from_array(p.map(f32::from_bits)))
                .collect(),
            auxiliary_samples: vec![],
            interpolated_curve: None,
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 0); 2],
            terrain_contacts: Default::default(),
            segment: StairSegment::new(DecoratorId(7), DecoratorId(12)),
            sub_segments: Default::default(),
            dirty: false,
        }
    }
    #[test]
    fn full_native_stair_intent_search_matches_128_original_complete_executions() {
        for (index, case) in STAIR_INTENT_NATIVE.iter().enumerate() {
            let request = request(case);
            let p = Vec3::from_array(case.position.map(f32::from_bits));
            let got = intent_stair_coordinate(&request, Vec2::new(p.x, p.z)).0;
            assert_eq!(
                [got.x.to_bits(), got.y.to_bits()],
                case.expected,
                "原完整函数case{index}"
            );
        }
    }
    #[test]
    fn real_world_intent_routes_stair_terrain_and_failure_without_default_owners() {
        let mut world = World::new();
        let mut roof_query = world.query::<&Roof>();
        let roofs = roof_query.query(&world);
        let walls = PublicWalls::default();
        let storage = DecoratorStorage::default();
        let subtypes =
            country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes::default();
        let meshes = DecoratorMeshLibrary::default();
        let derived = DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let cached = CachedDecoratorTransforms::default();
        let physics = RaycastWorld::default();
        let mut stream = UiSignifierStream::default();
        let mut stairs = PlayermadeStairsAssemblyRequests::default();
        stairs.requests.push(request(&STAIR_INTENT_NATIVE[6]));
        let candidate = DecoratorIntentCandidate {
            rotation: Quat::IDENTITY,
            destination: RaycastDecoratorDst::StairTop(7 | (12u64 << 32)),
            position: Vec3::from_array(STAIR_INTENT_NATIVE[6].position.map(f32::from_bits)),
            roof_coordinate: None,
        };
        let info = DecoratorInfo {
            ty: 0x3c,
            rank: 1,
            decorator_id: 0xfeedbeef,
            ..Default::default()
        };
        let intent = calculate_decorator_interaction_intent(
            &walls,
            &roofs,
            &stairs,
            candidate,
            DecoratorId(177),
            &info,
            &[],
            &storage,
            derived,
            &cached,
            DecoratorStorageAddr::Stair,
            false,
            false,
            &mut stream,
            &physics,
        );
        let DecoratorInteractionIntent::Accepted(accepted) = intent else {
            panic!("实际楼梯必须接受")
        };
        assert_eq!(accepted.decorator_id, DecoratorId(177));
        assert_eq!(accepted.merge, None);
        let MoveDstCandidate::Stair(stair) = accepted.destination else {
            panic!("不能伪成Terrain")
        };
        assert_eq!(
            [stair.coordinate.x.to_bits(), stair.coordinate.y.to_bits()],
            STAIR_INTENT_NATIVE[6].expected
        );
        let terrain = DecoratorIntentCandidate {
            destination: RaycastDecoratorDst::Terrain,
            ..candidate
        };
        let DecoratorInteractionIntent::Accepted(terrain_accepted) =
            calculate_decorator_interaction_intent(
                &walls,
                &roofs,
                &stairs,
                terrain,
                DecoratorId(177),
                &info,
                &[],
                &storage,
                derived,
                &cached,
                DecoratorStorageAddr::Terrain,
                false,
                false,
                &mut stream,
                &physics,
            )
        else {
            panic!("原允许类型必须可落地")
        };
        assert!(
            matches!(terrain_accepted.destination,MoveDstCandidate::Terrain(TerrainDecoratorDst{position_xz,..}) if position_xz==Vec2::new(candidate.position.x,candidate.position.z))
        );
        assert_eq!(terrain_accepted.storage, DecoratorStorageAddr::Terrain);
        let unsupported = DecoratorInfo { ty: 0x38, ..info };
        assert!(matches!(
            calculate_decorator_interaction_intent(
                &walls,
                &roofs,
                &stairs,
                terrain,
                DecoratorId(177),
                &unsupported,
                &[],
                &storage,
                derived,
                &cached,
                DecoratorStorageAddr::Terrain,
                false,
                false,
                &mut stream,
                &physics
            ),
            DecoratorInteractionIntent::UnsupportedTerrain { .. }
        ));
        stairs.requests.clear();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                calculate_decorator_interaction_intent(
                    &walls,
                    &roofs,
                    &stairs,
                    candidate,
                    DecoratorId(177),
                    &info,
                    &[],
                    &storage,
                    derived,
                    &cached,
                    DecoratorStorageAddr::Stair,
                    false,
                    false,
                    &mut stream,
                    &physics,
                )
            }))
            .is_err()
        );
        assert!(stream.take_entries().is_empty());
    }
    #[test]
    fn real_world_parent_intent_connects_wall_corner_and_merge_roof_merge_flat_and_missing_gates() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        use country_core::resources::walls::decorator::{DecoratorDst, WallDecoratorDst};
        use country_core::resources::walls::decorator_subtype::{
            DecoratorCollision, DecoratorSubtype, DerivedDecoratorSubtypes,
        };
        use country_core::resources::walls::{
            InnerWalls, WallState, publication_fields::WallShapeSnapshot,
            publication_source::TerrainHeightGrid512,
        };
        use country_core::systems::collision::world::RaycastMeshData;
        use std::sync::Arc;
        use utils::geometry::{Rectangle2d, Rectangle3d};
        let wall_id = WallId(7);
        let address = DecoratorStorageAddr::Wall(wall_id);
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[-4., -3.], [-4., 3.], [4., 3.], [4., -3.], [-4., -3.]],
            false,
        )
        .unwrap();
        let state = WallState::new(curve, 3.0, 0, 0, false);
        let (mut snapshot, freehand) = state.into_snapshot();
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 8., 6.].into_iter().enumerate() {
            shape[4 + i * 4..8 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(wall_id, WallState::from_snapshot(snapshot, freehand));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: false,
        };
        let paths=country_core::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(std::array::from_fn(|_|Arc::from(vec![0u8;512*512])),false).unwrap();
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(
            &mut inner,
            &terrain,
            Some(&paths),
            &Default::default(),
            &Default::default(),
        );
        let mut bytes = [0; 88];
        bytes[..8].copy_from_slice(&7u64.to_le_bytes());
        bytes[8..36].copy_from_slice(&shape);
        for (i, v) in [0f32, 0., 0.5, 0.6, 0.3, 0.2, 0.].into_iter().enumerate() {
            bytes[0x24 + i * 4..0x28 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        let mut world = World::new();
        let roof_entity = world.spawn(Roof::from_observed(bytes)).id();
        walls.associate_roof(wall_id, roof_entity);
        let mut meshes = DecoratorMeshLibrary::default();
        for subtype in [0, 1, 2, 3, 4, 24] {
            for rank in [1, 2] {
                meshes.collision.insert(
                    DecoratorLibKey::new(subtype, rank),
                    DecoratorCollision {
                        triangles: vec![],
                        mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                        bounds: Rectangle3d {
                            footprint: Rectangle2d {
                                axis: Vec2::X,
                                center: Vec2::ZERO,
                                size: Vec2::new(if rank == 1 { 1.5 } else { 2.4 }, 0.1),
                            },
                            bottom: 0.,
                            top: 2.,
                        },
                    },
                );
            }
        }
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(9, DecoratorSubtype(0));
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(
            9,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            },
        );
        let mut storage = DecoratorStorage::default();
        let physics = RaycastWorld::new();
        let stairs = PlayermadeStairsAssemblyRequests::default();
        let mut stream = UiSignifierStream::default();
        let mut incoming = DecoratorInfo {
            ty: 0x38,
            rank: 1,
            state: 1,
            ..Default::default()
        };
        incoming.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord: WallSpaceCoord::new_unchecked(3., 8.),
            platform: WallId(0),
            relative_y: 0.,
            position: Vec3::ZERO,
            corner_attachment: 0,
            bottom_as_door: false,
        }));
        let mut target = incoming;
        target.dst_wall = u64::from(2f32.to_bits()) | (u64::from(8f32.to_bits()) << 32);
        storage.apply_decorator_place(address, DecoratorId(9), &target);
        let mut query = world.query::<&Roof>();
        let run = |world: &World,
                   storage: &DecoratorStorage,
                   subtypes: &DerivedDecoratorSubtypes,
                   info: &DecoratorInfo,
                   candidate: DecoratorIntentCandidate,
                   allow,
                   flat,
                   stream: &mut UiSignifierStream,
                   query: &mut bevy_ecs::query::QueryState<&Roof>| {
            calculate_decorator_interaction_intent(
                &walls,
                &query.query(world),
                &stairs,
                candidate,
                DecoratorId(177),
                info,
                &[],
                storage,
                DerivedDecoratorInfoView::new(subtypes, &meshes),
                &cached,
                address,
                allow,
                flat,
                stream,
                &physics,
            )
        };
        let candidate = DecoratorIntentCandidate {
            rotation: Quat::IDENTITY,
            destination: RaycastDecoratorDst::Wall(wall_id),
            position: Vec3::new(-4., 8., 0.),
            roof_coordinate: None,
        };
        let DecoratorInteractionIntent::Accepted(merged) = run(
            &world,
            &storage,
            &subtypes,
            &incoming,
            candidate,
            true,
            false,
            &mut stream,
            &mut query,
        ) else {
            panic!("墙合并")
        };
        assert_eq!(
            merged.merge,
            Some(DecoratorIntentMerge {
                target: DecoratorId(9),
                rank: 2
            })
        );
        let MoveDstCandidate::Wall { coordinate, .. } = merged.destination else {
            panic!("墙目的")
        };
        assert_eq!(coordinate.u, 2.45);
        let corner = DecoratorIntentCandidate {
            position: Vec3::new(-4., 8., -2.9),
            ..candidate
        };
        let DecoratorInteractionIntent::Accepted(corner) = run(
            &world,
            &storage,
            &subtypes,
            &incoming,
            corner,
            false,
            false,
            &mut stream,
            &mut query,
        ) else {
            panic!("角吸附")
        };
        assert!(matches!(
            corner.destination,
            MoveDstCandidate::Wall {
                coordinate: WallSpaceCoord { u: 0.0, .. },
                corner_attachment: 1,
                ..
            }
        ));
        // 原Place传入的基线仍为WallCompound；Roof候选另存，不能先改Info掩盖派生差异。
        let mut place_baseline = incoming;
        place_baseline.set_destination(DecoratorDst::WallCompound(*incoming.destination().as_wall()));
        let place_roof = DecoratorIntentCandidate {
            destination: RaycastDecoratorDst::Roof { sub: 0, payload: 7 },
            position: Vec3::Y,
            roof_coordinate: Some(Vec2::new(0.7, 0.6)),
            ..candidate
        };
        let DecoratorInteractionIntent::Accepted(baseline_roof) = run(
            &world, &storage, &subtypes, &place_baseline, place_roof, false, false,
            &mut stream, &mut query,
        ) else { panic!("原WallCompound基线的Roof候选") };
        assert_eq!(baseline_roof.storage, address);
        assert_eq!(baseline_roof.target_storage, address);
        assert!(matches!(baseline_roof.destination, MoveDstCandidate::Roof(_)));
        incoming.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::new(0.7, 0.6),
            yaw: DecoratorRotationXZ(0.),
        }));
        target.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::new(0.25, 0.6),
            yaw: DecoratorRotationXZ(0.7),
        }));
        storage.apply_decorator_place(address, DecoratorId(9), &target);
        subtypes.insert(9, DecoratorSubtype(2));
        let roof = DecoratorIntentCandidate {
            destination: RaycastDecoratorDst::Roof { sub: 0, payload: 7 },
            position: Vec3::Y,
            roof_coordinate: Some(Vec2::new(0.7, 0.6)),
            ..candidate
        };
        let DecoratorInteractionIntent::Accepted(merged) = run(
            &world,
            &storage,
            &subtypes,
            &incoming,
            roof,
            true,
            false,
            &mut stream,
            &mut query,
        ) else {
            panic!("屋顶合并")
        };
        assert_eq!(
            merged.merge,
            Some(DecoratorIntentMerge {
                target: DecoratorId(9),
                rank: 2
            })
        );
        let MoveDstCandidate::Roof(destination) = merged.destination else {
            panic!("屋顶目的")
        };
        assert_eq!(destination.yaw, DecoratorRotationXZ(0.7));
        let DecoratorInteractionIntent::Accepted(unmerged) = run(
            &world,
            &storage,
            &subtypes,
            &incoming,
            roof,
            false,
            false,
            &mut stream,
            &mut query,
        ) else {
            panic!("屋顶普通放置")
        };
        assert_eq!(unmerged.merge, None);
        assert!(
            matches!(unmerged.destination,MoveDstCandidate::Roof(RoofDecoratorDst{coord,..}) if coord==Vec2::new(0.7,0.6))
        );
        assert!(matches!(
            run(
                &world,
                &storage,
                &subtypes,
                &incoming,
                roof,
                true,
                true,
                &mut stream,
                &mut query
            ),
            DecoratorInteractionIntent::Rejected { .. }
        ));
        storage.apply_decorator_remove(address, DecoratorId(9));
        let DecoratorInteractionIntent::Accepted(no_target) = run(
            &world,
            &storage,
            &subtypes,
            &incoming,
            roof,
            true,
            false,
            &mut stream,
            &mut query,
        ) else {
            panic!("删除后仍可放置")
        };
        assert_eq!(no_target.merge, None);
        // 原Accepted同时保留两个定位器。用不同源域验证，避免同墙夹具把迁移遗漏藏住。
        let mut terrain_clutter = DecoratorInfo { ty: 0, rank: 1, ..Default::default() };
        terrain_clutter.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::ZERO, yaw: DecoratorRotationXZ(0.0),
        }));
        for target in [candidate, roof] {
            let DecoratorInteractionIntent::Accepted(migration) = calculate_decorator_interaction_intent(
                &walls, &query.query(&world), &stairs, target, DecoratorId(177), &terrain_clutter, &[],
                &storage, DerivedDecoratorInfoView::new(&subtypes, &meshes), &cached,
                DecoratorStorageAddr::Terrain, false, false, &mut stream, &physics,
            ) else { panic!("跨域目标应可接受") };
            assert_eq!(migration.storage, DecoratorStorageAddr::Terrain);
            assert_eq!(migration.target_storage, DecoratorStorageAddr::Wall(wall_id));
            if let MoveDstCandidate::Wall { storage, .. } = migration.destination {
                assert_eq!(storage, DecoratorStorageAddr::Wall(wall_id));
            }
        }
        let invalid = DecoratorIntentCandidate {
            position: Vec3::new(f32::NAN, 1., 0.),
            ..roof
        };
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(
                &world,
                &storage,
                &subtypes,
                &incoming,
                invalid,
                false,
                false,
                &mut stream,
                &mut query
            )))
            .is_err()
        );
        assert!(stream.take_entries().is_empty());
    }
}
