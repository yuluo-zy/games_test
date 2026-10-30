//! 原B完整位置检查14080f470/8143B及必要1203B脊线/200B角吸附。
//! 原判别11有效，0至10携带真实失败原因；验证入口供同世界蓝图反馈消费。
//! 主剔除接真实墙事件、活桶、物理归档和延迟删除；楼梯独立路径另行验收。

use bevy_ecs::prelude::*;
use country_core::resources::history::History;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::WallId;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo, DecoratorType};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::public_wall_state::PublicWallState;

use crate::apply_decorator_op::{DecoratorOp, apply_decorator_op};
use crate::blueprint::DecoratorBlueprints;

/// 原判别11表示有效；其余判别保存验证器实际消费的失败载荷。
/// 枚举名是本项目的语义命名，不伪造原生内存布局。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorLocationCheckResult {
    Valid,
    Invalid(DecoratorLocationInvalidReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorLocationInvalidReason {
    WallCutoutBoundary {
        wall: WallId,
        coord: CurveInternalCoord,
    },
    RoofBounds {
        position: Vec3,
        direction: Vec3,
        wall: WallId,
    },
    WallEnds {
        left_oob: bool,
        right_oob: bool,
        wall: WallId,
    },
    SharpWallTurn {
        wall: WallId,
        coord: CurveInternalCoord,
    },
    WallTop {
        padding: f32,
        wall: WallId,
    },
    WallCorner {
        wall: WallId,
        rectangle: Rectangle2d,
        corner: usize,
    },
    BalconyUpperClearance {
        wall: WallId,
    },
    FlatRoofForbidden,
    GableBandForbidden,
    MissingRoof,
    WallLowerBoundary {
        wall: WallId,
    },
}

impl DecoratorLocationInvalidReason {
    pub fn native_tag(self) -> u8 {
        match self {
            Self::WallCutoutBoundary { .. } => 0,
            Self::RoofBounds { .. } => 1,
            Self::WallEnds { .. } => 2,
            Self::SharpWallTurn { .. } => 3,
            Self::WallTop { .. } => 4,
            Self::WallCorner { .. } => 5,
            Self::BalconyUpperClearance { .. } => 6,
            Self::FlatRoofForbidden => 7,
            Self::GableBandForbidden => 8,
            Self::MissingRoof => 9,
            Self::WallLowerBoundary { .. } => 10,
        }
    }
}

impl DecoratorLocationCheckResult {
    pub fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }
    pub fn get_invalid_reason(&self) -> Option<&DecoratorLocationInvalidReason> {
        match self {
            Self::Valid => None,
            Self::Invalid(reason) => Some(reason),
        }
    }
    pub fn native_tag(self) -> u8 {
        match self {
            Self::Valid => 11,
            Self::Invalid(reason) => reason.native_tag(),
        }
    }
}

use crate::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds;
use crate::decorator_visual::gothic_dormer::gothic_dormer_profile_curve;
use crate::ui_systems::decorator_interaction_intent::{BalconyDoorSnap, snap_balcony_door};
use country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve;
use country_core::resources::roofs::geometry::surface::RoofSection;
use country_core::resources::roofs::{Roof, RoofIntersectionSection};
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView,
};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastWorld;
use glam::{Vec2, Vec3};
use utils::comparative_space::ComparativeSpace;
use utils::curve::{CurveCoordSemanticThresholded3D, CurveInternalCoord, curve_slice::CurveSlice};
use utils::geometry::{Circle2d, Rectangle2d};
use utils::wall_space::{WallSpace, WallSpaceCoord};

/// B14080f470/8143B。所有输入均来自当前装饰的真实存储地址和同一世界；
/// 必需资源缺失保留原断言，地形与楼梯的直接有效分支不读取额外资源。
#[allow(clippy::too_many_arguments)]
pub fn is_decorator_location_valid(
    id: DecoratorId,
    info: &DecoratorInfo,
    dst: DecoratorStorageAddr,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    derived: DerivedDecoratorInfoView<'_>,
    transforms: &CachedDecoratorTransforms,
    raycast: &RaycastWorld,
) -> DecoratorLocationCheckResult {
    use DecoratorLocationCheckResult::{Invalid, Valid};
    use DecoratorLocationInvalidReason as Reason;
    match info.dst_tag() {
        3 | 4 => Valid,
        2 => {
            let wall_id = dst.as_wall();
            let Some(entity) = walls.roof_entity(wall_id) else {
                return Invalid(Reason::MissingRoof);
            };
            // 原早期屋顶分支使用必需Query，而后面的墙顶检查允许查询缺失。
            let roof = roofs.get(entity).expect("decorator roof query failed");
            let subtype = derived.subtype(id).0;
            if subtype == 2 || subtype == 8 {
                let transform = transforms.transform(id.0);
                let bounds = derived
                    .mesh_library()
                    .collision_record(DecoratorLibKey::new(subtype, info.rank()))
                    .bounds;
                let height = bounds.top - bounds.bottom;
                let width = if subtype == 8 {
                    match info.rank() {
                        2 => 1.2,
                        3 => 1.54,
                        _ => 0.72,
                    }
                } else {
                    bounds.footprint.size.x
                };
                if is_out_of_roof_bounds(&transform, width, height, info.rank(), roof) {
                    let direction = if subtype == 8 {
                        location_unit3(transform.rotation * Vec3::Z)
                    } else {
                        Vec3::ZERO
                    };
                    return Invalid(Reason::RoofBounds {
                        position: location_elevated(transform.translation, height),
                        direction,
                        wall: wall_id,
                    });
                }
                if subtype == 8 {
                    let forward = transform.rotation * Vec3::Z;
                    let probe = location_unit2(Vec2::new(forward.x, forward.z));
                    let result = check_if_dormer_ridge_is_within_roof_bounds(
                        subtype,
                        info,
                        transform.translation,
                        probe,
                        wall_id,
                        walls,
                        roofs,
                    );
                    if !result.is_valid() {
                        return result;
                    }
                }
            }
            if roof.is_flat() && !info.decorator_type().can_place_on_roof() {
                return Invalid(Reason::FlatRoofForbidden);
            }
            let coord = info.wall_space_coord();
            if 1.9 > coord.y && roof.is_gable() {
                let sides = roof.gable_sides().expect("gable roof has no gable sides");
                for side in sides {
                    let lower = f32::from(side) * 0.25;
                    let upper = (f32::from(side) + 1.0) * 0.25;
                    if coord.u >= lower && upper > coord.u {
                        return Invalid(Reason::GableBandForbidden);
                    }
                }
            }
            Valid
        }
        _ => is_wall_decorator_location_valid(
            id, info, dst, walls, roofs, derived, transforms, raycast,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn is_wall_decorator_location_valid(
    id: DecoratorId,
    info: &DecoratorInfo,
    dst: DecoratorStorageAddr,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    derived: DerivedDecoratorInfoView<'_>,
    transforms: &CachedDecoratorTransforms,
    raycast: &RaycastWorld,
) -> DecoratorLocationCheckResult {
    use DecoratorLocationCheckResult::{Invalid, Valid};
    use DecoratorLocationInvalidReason as Reason;
    let subtype = derived.subtype(id).0;
    // 原f56c在判断subtype1之前已读取缓存；其它墙分支也不能吞缺失缓存。
    let transform = transforms.transform(id.0);
    let coord = info.wall_space_coord();
    if subtype == 1 {
        let wall_id = dst.as_wall();
        let Some(entity) = walls.roof_entity(wall_id) else {
            return Invalid(Reason::MissingRoof);
        };
        let roof = roofs.get(entity).expect("decorator roof query failed");
        let bounds = derived
            .mesh_library()
            .collision_record(DecoratorLibKey::new(1, info.rank()))
            .bounds;
        let height = bounds.top - bounds.bottom;
        if is_out_of_roof_bounds(
            &transform,
            bounds.footprint.size.x,
            height,
            info.rank(),
            roof,
        ) {
            return Invalid(Reason::RoofBounds {
                position: location_elevated(transform.translation, height),
                direction: Vec3::ZERO,
                wall: wall_id,
            });
        }
    }
    // 原墙分支再次读取当前子类型，随后先读取collision，再required墙和路径。
    let subtype_value = derived.subtype(id);
    let subtype = subtype_value.0;
    let bounds = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(subtype, info.rank()))
        .bounds;
    let width = bounds.footprint.size.x;
    let height = bounds.top - bounds.bottom;
    let wall_id = dst.as_wall();
    let wall = walls.get(wall_id).expect("decorator wall does not exist");
    let space = WallSpace {
        is_loop: wall.shape().tag() != 2,
        length: wall.wall_space_length(),
    };
    let points = wall
        .prepared_path
        .as_ref()
        .expect("decorator wall path has not been prepared")
        .positions
        .as_slice();
    let center = location_curve_coord(points, space.project_clamp(0.0 + coord.u));
    let half_width = width * 0.5;
    let left = location_curve_coord(points, space.project_clamp(coord.u - half_width));
    let right = location_curve_coord(points, space.project_clamp(coord.u + half_width));
    let center_point = location_sample(points, center);
    // Y沿原左乘补数、右乘参数顺序插值，与XYZ反馈采样的加数顺序独立。
    let base_y =
        points[center.segment][1] * (1.0 - center.t) + points[center.segment + 1][1] * center.t;
    if subtype == 7 {
        let tangent = location_unit3(
            Vec3::from_array(points[center.segment + 1]) - Vec3::from_array(points[center.segment]),
        );
        let probe = location_unit2(Vec2::new(-tangent.z, tangent.x));
        let result = check_if_dormer_ridge_is_within_roof_bounds(
            subtype,
            info,
            coord.to_world_space(wall.simplified_curve_2d()),
            probe,
            wall_id,
            walls,
            roofs,
        );
        if !result.is_valid() {
            return result;
        }
    }
    let right_oob = space.project(coord.u + half_width).is_none();
    let left_oob = space.project(coord.u - half_width).is_none();
    if left_oob || right_oob {
        return Invalid(Reason::WallEnds {
            left_oob,
            right_oob,
            wall: wall_id,
        });
    }
    for sample in [left, center, right] {
        let cut = location_height(&wall.segment_heights, sample);
        let threshold = match subtype {
            4 | 9 => coord.y + if cut > 0.0 { height * -0.5 } else { 0.2 },
            6 => {
                coord.y
                    + gothic_half_dormer_profile_curve(info.rank())
                        .last()
                        .expect("gothic half dormer profile is empty")
                        .y
            }
            11 | 12 => coord.y,
            16 => coord.y + 0.3,
            25 => coord.y + 0.2,
            _ => coord.y + height * -0.5,
        };
        if base_y + cut > threshold {
            return Invalid(if cut > 0.0 {
                Reason::WallCutoutBoundary {
                    wall: wall_id,
                    coord: sample,
                }
            } else {
                Reason::WallLowerBoundary { wall: wall_id }
            });
        }
    }
    if subtype != 3 && wall.shape().tag() >= 2 {
        let a = location_sample(points, left);
        let b = center_point;
        let c = location_sample(points, right);
        let ab = location_unit2(Vec2::new(b.x - a.x, b.z - a.z));
        let bc = location_unit2(Vec2::new(c.x - b.x, c.z - b.z));
        if 0.6 > ab.y * bc.y + ab.x * bc.x {
            return Invalid(Reason::SharpWallTurn {
                wall: wall_id,
                coord: center,
            });
        }
    }
    if subtype == 4 || subtype == 9 {
        let tangent = location_unit3(
            Vec3::from_array(points[center.segment + 1]) - Vec3::from_array(points[center.segment]),
        );
        let probe = location_unit2(Vec2::new(tangent.x, tangent.z));
        if matches!(
            snap_balcony_door(
                true,
                info,
                Vec3::new(center_point.x, coord.y, center_point.z),
                probe,
                coord.u,
                wall_id,
                wall,
                roofs,
                derived.mesh_library(),
                true,
                raycast
            ),
            BalconyDoorSnap::RejectedUpperClearance
        ) {
            return Invalid(Reason::BalconyUpperClearance { wall: wall_id });
        }
    }
    if !subtype_value.can_be_on_corner() {
        if let Some(rectangle) = wall.is_rectangle() {
            for corner in 0..4 {
                let corner_u = space.project_clamp(rectangle.get_point_u(corner) * space.length);
                if should_snap_to_corner(
                    info.decorator_type(),
                    wall.building_projection().is_halftimbered(),
                    wall,
                    WallSpaceCoord::new_unchecked(corner_u, coord.y),
                    coord,
                    width,
                ) {
                    return Invalid(Reason::WallCorner {
                        wall: wall_id,
                        rectangle,
                        corner,
                    });
                }
            }
        }
    }
    if wall.shape().tag() < 2 {
        let Some(roof) = walls
            .roof_entity(wall_id)
            .and_then(|entity| roofs.get(entity).ok())
        else {
            return Valid;
        };
        let use_arch_heights = if subtype == 3 {
            false
        } else if roof.is_gable() {
            let rectangle = location_roof_rectangle(roof);
            let sides = roof.gable_sides().expect("gable roof has no gable sides");
            let edge = rectangle.get_closest_edge_id(Vec2::new(center_point.x, center_point.z));
            edge == usize::from(sides[0]) || edge == usize::from(sides[1])
        } else {
            false
        };
        if !use_arch_heights {
            if subtype != 3
                && (info.ty == 0x3b
                    || info.ty == 0x3f
                    || (info.decorator_type().is_window() && !roof.is_flat()))
            {
                if coord.y + height * -0.25 > wall.max_y() {
                    return Invalid(Reason::WallTop {
                        padding: 0.0,
                        wall: wall_id,
                    });
                }
                return Valid;
            }
            let padding = if roof.is_flat() {
                if subtype == 3 { 0.79 } else { 0.69 }
            } else {
                0.1
            };
            let offset = if subtype == 11 || subtype == 12 {
                padding
            } else {
                height * -0.5
            };
            if coord.y - offset > wall.max_y() - padding {
                return Invalid(Reason::WallTop {
                    padding,
                    wall: wall_id,
                });
            }
            return Valid;
        }
    }
    let arch_heights = wall
        .arch_segment_heights()
        .expect("wall arch heights have not been calculated");
    for sample in [left, right] {
        let ceiling = location_height(arch_heights, sample);
        let padding = if wall.shape().tag() < 2 || info.decorator_type().is_window() {
            0.25
        } else {
            0.67
        };
        let offset = if subtype == 11 || subtype == 12 {
            -padding
        } else {
            height * 0.5
        };
        if coord.y + offset > ceiling - padding {
            return Invalid(Reason::WallTop {
                padding,
                wall: wall_id,
            });
        }
    }
    Valid
}

fn location_curve_coord(points: &[[f32; 3]], distance: f32) -> CurveInternalCoord {
    let start = CurveInternalCoord { segment: 0, t: 0.0 };
    let mut slice = CurveSlice::<[f32; 3], _, CurveCoordSemanticThresholded3D, Vec2, _>
        ::from_coord_range_with_remap(points, start, start, |_, p| Vec2::new(p[0], p[2]));
    // 原maxss的第二操作数为+0：NaN及-0选择该+0，不引入鲁棒采样器。
    slice.extend_right_until(if distance > 0.0 { distance } else { 0.0 }, |_| false);
    slice.end()
}

fn location_sample(points: &[[f32; 3]], coord: CurveInternalCoord) -> Vec3 {
    Vec3::from_array(points[coord.segment + 1]) * coord.t
        + Vec3::from_array(points[coord.segment]) * (1.0 - coord.t)
}
fn location_height(heights: &[f32], coord: CurveInternalCoord) -> f32 {
    let first = heights[coord.segment];
    (heights[coord.segment + 1] - first) * coord.t + first
}
fn location_unit2(value: Vec2) -> Vec2 {
    value * (1.0 / (value.y * value.y + value.x * value.x).sqrt())
}
fn location_unit3(value: Vec3) -> Vec3 {
    value * (1.0 / ((value.y * value.y + value.x * value.x) + value.z * value.z).sqrt())
}
fn location_elevated(position: Vec3, amount: f32) -> Vec3 {
    Vec3::new(
        amount * 0.0 + position.x,
        amount + position.y,
        amount * 0.0 + position.z,
    )
}

/// 原Roof枚举的矩形载荷解码；几何算法仍由同一Rectangle2d实现。
fn location_roof_rectangle(roof: &Roof) -> Rectangle2d {
    let bytes = &roof.observed()[8..0x24];
    assert_eq!(
        u32::from_le_bytes(bytes[..4].try_into().unwrap()),
        1,
        "gable roof enclosure is not a rectangle"
    );
    location_rectangle_payload(&bytes[4..])
}
fn location_rectangle_payload(bytes: &[u8]) -> Rectangle2d {
    let field = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    Rectangle2d {
        axis: Vec2::new(field(0), field(4)),
        center: Vec2::new(field(8), field(12)),
        size: Vec2::new(field(16), field(20)),
    }
}

/// `auto_allow_decorator_culling`（0x14080d3d0，O 结构）：录制中自动放行
/// 全部归档装饰的剔除（`decorator_cull_allow` 行 302 内联 → allowed_ids）。
pub fn auto_allow_decorator_culling<C: Send + Sync + 'static>(
    history: Res<History>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
) {
    if !history.is_recording() {
        return;
    }
    // 0x14080d3d0 尾跳 0x140b2bd30 decorator_cull_allow。
    archivist.decorator_cull_allow();
}

/// 原14080d540：只检查仍存在的受影响墙，遍历当前桶而非归档中的旧 ID。
/// 收集阶段不删除；首次越界备份先于缓存位置读取，全部检查结束后才应用删除。
pub fn cull_oob_decorators(
    walls: Res<PublicWalls>,
    history: Res<History>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<
        country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    >,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut heights: EventReader<country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd>,
    mut changes: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    mut roofs_edited: EventReader<country_core::resources::roofs::commands::EditRoofCmd>,
    roofs: Query<&Roof>,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
    derived: DerivedDecoratorInfoResources,
    transforms: Res<CachedDecoratorTransforms>,
    mut scratch: Local<CullScratch>,
) {
    let CullScratch {
        walls: affected,
        removals,
    } = &mut *scratch;
    affected.clear();
    removals.clear();
    // 原141264200保留 PublicWalls 中的归档墙键；special 桶不属于此系统。
    affected.extend(
        archivist
            .walls
            .keys()
            .map(|wall| WallId(wall.0))
            .filter(|wall| walls.get_unchecked(*wall).is_some()),
    );
    for wall in heights
        .read()
        .map(|event| WallId(event.0.wall.0))
        .chain(changes.read().map(|event| event.0))
        .chain(roofs_edited.read().map(|event| WallId(event.0.wall_id)))
    {
        if walls.get_unchecked(wall).is_some() {
            affected.insert(wall);
        }
    }
    let recording = history.is_recording();
    for &wall in affected.iter() {
        let address = DecoratorStorageAddr::Wall(wall);
        let Some(bucket) = storage.wall_decorators(wall) else {
            continue;
        };
        let mut backed_up = false;
        for (&id, info) in &bucket.decorators {
            if !cull_one(
                id,
                info,
                wall,
                &archivist,
                &transforms,
                &blueprints,
                &walls,
                &roofs,
                derived.as_view(),
                &archival.raycast_world,
            ) {
                continue;
            }
            if recording && !backed_up {
                archivist.archive_current_decorators_at_with_resources(
                    address,
                    &storage,
                    &archival,
                    "oob backup",
                );
                backed_up = true;
            }
            // 原缺失缓存会在归档后失败；不能用零位置或 Info 的局部坐标代替。
            removals.insert(
                country_core::resources::walls::decorator::DecoratorLocator::new(address, id),
                transforms.position(id),
            );
        }
    }
    for (locator, position) in removals.drain() {
        assert!(archivist.is_decorator_cull_allowed(locator.decorator));
        blueprints.remove(locator.decorator);
        // apply 的预览归档始终执行；同地址已存在的首快照和碰撞载荷不可覆盖。
        archivist.archive_current_decorators_at_with_resources(
            locator.addr,
            &storage,
            &archival,
            crate::apply_decorator_op::LABEL_REMOVE,
        );
        let info = *storage
            .get(locator.addr)
            .and_then(|bucket| bucket.get(DecoratorId(locator.decorator)))
            .expect("culled decorator lost its owning record");
        let op = DecoratorOp::remove_at(DecoratorId(locator.decorator), info, position, !recording);
        // 原 EDX=0；!recording 位于位置载荷+12，即事件+0x7c，不能混入+0x80。
        apply_decorator_op(
            &op,
            0,
            locator.addr,
            &mut storage,
            Some(&mut archivist),
            None,
            &events,
        );
    }
}

/// 复用逐帧临时表的容量；键仍是原完整定位器，不改变去重或候选集合语义。
#[derive(Default)]
pub struct CullScratch {
    walls: hashbrown::HashSet<WallId>,
    removals: hashbrown::HashMap<country_core::resources::walls::decorator::DecoratorLocator, Vec3>,
}

/// 原类型门仅含建筑装饰56..63且排除62；有效位置仍需真实蓝图碰撞检查。
#[allow(clippy::too_many_arguments)]
fn cull_one(
    id: u32,
    info: &DecoratorInfo,
    wall: WallId,
    archivist: &country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    transforms: &CachedDecoratorTransforms,
    blueprints: &DecoratorBlueprints,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    derived: DerivedDecoratorInfoView<'_>,
    raycast: &RaycastWorld,
) -> bool {
    if !archivist.is_decorator_cull_allowed(id) {
        return false;
    }
    let family = info.ty.wrapping_sub(0x38);
    if family >= 8 || family == 6 {
        return false;
    }
    if !is_decorator_location_valid(
        DecoratorId(id),
        info,
        DecoratorStorageAddr::Wall(wall),
        walls,
        roofs,
        derived,
        transforms,
        raycast,
    )
    .is_valid()
    {
        return true;
    }
    let Some(blueprint) = blueprints.get(id) else {
        return false;
    };
    let crate::blueprint::BlueprintMeshSlot::Mesh(mesh) = &blueprint.collision_mesh else {
        return false;
    };
    let shape = crate::blueprint::validation_overlap_shape(mesh);
    // 原ordered=true：同墙、非自身且ID更小的真实碰撞对象才构成阻挡。
    raycast
        .decorator_overlaps_with_others(&shape, DecoratorId(id), wall, true)
        .is_some()
}

/// 原B1412b6c30/1412b6420两个单态：先收集完整定位器，再归档并删除。
/// 保留ResMut包装，空候选帧不会因借出普通&mut而伪造owner变更。
#[allow(clippy::too_many_arguments)]
pub fn cull_stair_attached_decorators_impl(
    history: Res<History>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<
        country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    >,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
    transforms: Res<CachedDecoratorTransforms>,
    mut should_remove: impl FnMut(&DecoratorInfo) -> bool,
) {
    use country_core::resources::walls::decorator::DecoratorLocator;
    let mut removals = hashbrown::HashMap::<DecoratorLocator, Vec3>::new();
    for bucket in storage.iter_stair_decorators() {
        for (&id, info) in bucket.decorators() {
            // 原初筛没有许可时跳过；仅apply前的第二次检查是断言。
            if !archivist.is_decorator_cull_allowed(id) || !should_remove(info) {
                continue;
            }
            // 原1412b6d84..6db4：无缓存时保留(-0,-1000,-0)，不能panic或取局部坐标。
            let position = transforms
                .try_position(id)
                .unwrap_or(Vec3::new(-0.0, -1000.0, -0.0));
            removals.insert(
                DecoratorLocator::new(DecoratorStorageAddr::Stair, id),
                position,
            );
        }
    }
    for (locator, position) in removals {
        assert!(
            archivist.is_decorator_cull_allowed(locator.decorator),
            "assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)"
        );
        // 原先移出/释放蓝图Arc，再经apply归档与删除；全体收集失败时不先删任何实体。
        blueprints.remove(locator.decorator);
        archivist.archive_current_decorators_at_with_resources(
            locator.addr,
            &storage,
            &archival,
            crate::apply_decorator_op::LABEL_REMOVE,
        );
        let info = *storage
            .get(locator.addr)
            .and_then(|bucket| bucket.get(DecoratorId(locator.decorator)))
            .expect("culled stair decorator lost its owning record");
        let op = DecoratorOp::remove_at(
            DecoratorId(locator.decorator),
            info,
            position,
            !history.is_recording(),
        );
        apply_decorator_op(
            &op,
            0,
            locator.addr,
            &mut storage,
            Some(&mut archivist),
            None,
            &events,
        );
    }
}

/// 原14080ec10八参入口：图中任意合法边或stub均保留，不按类型近似判断。
pub fn cull_stair_attached_decorators(
    history: Res<History>,
    storage: ResMut<DecoratorStorage>,
    archivist: ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    blueprints: ResMut<DecoratorBlueprints>,
    stairs: Res<StairsState>,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
    transforms: Res<CachedDecoratorTransforms>,
) {
    cull_stair_attached_decorators_impl(
        history,
        storage,
        archivist,
        events,
        blueprints,
        archival,
        transforms,
        |info| {
            let info = info.dst_as_stair();
            !stairs.contains_edge_or_stub(info.dst_wall as u32, (info.dst_wall >> 32) as u32)
        },
    );
}

/// 原14080f040八参入口：请求+128的两个端点逐32位精确相等，非图边存在性检查。
pub fn cull_stair_attached_decorators_on_invalid_stairs(
    history: Res<History>,
    storage: ResMut<DecoratorStorage>,
    archivist: ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    blueprints: ResMut<DecoratorBlueprints>,
    requests: Res<
        country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
    >,
    archival: country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,
    transforms: Res<CachedDecoratorTransforms>,
) {
    cull_stair_attached_decorators_impl(
        history,
        storage,
        archivist,
        events,
        blueprints,
        archival,
        transforms,
        |info| {
            let info = info.dst_as_stair();
            !requests.requests.iter().any(|request| {
                let (a, b) = request.segment.nodes();
                a.0 == info.dst_wall as u32 && b.0 == (info.dst_wall >> 32) as u32
            })
        },
    );
}

/// B140811900/200B：只比较原墙空间的水平差，包含闭环投影。
/// halftimbered偏移为0.2，另一分支为0.46；阈值使用完整宽度的一半。
pub fn should_snap_to_corner(
    decorator_type: DecoratorType,
    halftimbered: bool,
    wall: &PublicWallState,
    a: WallSpaceCoord,
    b: WallSpaceCoord,
    threshold: f32,
) -> bool {
    if !decorator_type.should_offer_snap_to_corner() {
        return false;
    }
    let space = WallSpace {
        is_loop: wall.shape().tag() != 2,
        length: wall.wall_space_length(),
    };
    let offset = if halftimbered { 0.2 } else { 0.46 };
    space.subtract(a.u, b.u).abs() < threshold * 0.5 + offset
}

/// B140811440/1203B：以真实屋顶截面和原轮廓顶点检查老虎窗脊。
/// 此处缺失屋顶实体或组件均返回MissingRoof，区别于主检查的required查询。
#[allow(clippy::too_many_arguments)]
pub fn check_if_dormer_ridge_is_within_roof_bounds(
    subtype: u8,
    info: &DecoratorInfo,
    position: Vec3,
    probe: Vec2,
    wall_id: WallId,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
) -> DecoratorLocationCheckResult {
    use DecoratorLocationCheckResult::{Invalid, Valid};
    use DecoratorLocationInvalidReason as Reason;
    if !(subtype == 7 || subtype == 8) {
        return Valid;
    }
    let Some(roof) = walls
        .roof_entity(wall_id)
        .and_then(|entity| roofs.get(entity).ok())
    else {
        return Invalid(Reason::MissingRoof);
    };
    if u32::from_le_bytes(roof.observed()[8..12].try_into().unwrap()) == 0 {
        return Valid;
    }
    let profile = if subtype == 7 {
        gothic_half_dormer_profile_curve(info.rank())
    } else {
        gothic_dormer_profile_curve(info.rank())
    };
    let top = location_elevated(
        position,
        profile.first().expect("dormer profile is empty").y,
    );
    let invalid = || {
        Invalid(Reason::RoofBounds {
            position: top,
            direction: Vec3::new(probe.x, 0.0, probe.y),
            wall: wall_id,
        })
    };
    let point = Vec2::new(top.x, top.z);
    let closest = match roof.roof_shape_at_intersection_y(top.y) {
        RoofIntersectionSection::OutOfBounds => return invalid(),
        RoofIntersectionSection::Section(RoofSection::Rectangle { frame, dimensions }) => {
            Rectangle2d {
                axis: Vec2::new(frame[0], frame[1]),
                center: Vec2::new(frame[2], frame[3]),
                size: Vec2::from_array(dimensions),
            }
            .closest_point_on_perimeter(point)
        }
        RoofIntersectionSection::Section(RoofSection::Circle {
            frame,
            radius,
            word_0x18,
        }) => Circle2d::new_rotated(Vec2::from_array(frame), radius, f32::from_bits(word_0x18))
            .closest_point_on_perimeter(point),
        RoofIntersectionSection::RawSection(bytes) => {
            let field = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            if u32::from_le_bytes(bytes[..4].try_into().unwrap()) == 0 {
                Circle2d::new_rotated(Vec2::new(field(4), field(8)), field(12), field(16))
                    .closest_point_on_perimeter(point)
            } else {
                location_rectangle_payload(&bytes[4..]).closest_point_on_perimeter(point)
            }
        }
    };
    if top.y > (roof.height() + roof.floor_y()) + -0.1 {
        return invalid();
    }
    let normal = location_unit2(point - closest);
    let dot = normal.y * probe.y + normal.x * probe.x;
    // 原ucomiss/jbe将unordered也视为通过，不改成dot>=threshold。
    if 0.993 > dot { invalid() } else { Valid }
}

#[cfg(test)]
mod location_native_tests {
    use super::*;
    use bevy_ecs::system::SystemState;
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
    use country_core::resources::walls::decorator_subtype::{
        DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes,
    };
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::resources::walls::{InnerWalls, WallState};
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
    use std::sync::Arc;

    // 这些是显式测试输入，不作为游戏启动已拥有原资源的证据。
    struct Fixture {
        world: World,
        walls: PublicWalls,
        subtypes: DerivedDecoratorSubtypes,
        meshes: DecoratorMeshLibrary,
        cache: CachedDecoratorTransforms,
        raycast: RaycastWorld,
    }
    impl Fixture {
        fn new(subtype: u8, shape: u32) -> Self {
            let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::splat(8.0));
            let points = rectangle.as_points2().map(|p| [p.x, p.y]);
            let curve = Curve2::try_new(points.to_vec(), true).unwrap();
            let mut snapshot = WallState::new(curve, 3.0, 0, 0, false).snapshot().clone();
            let mut bytes = [0; 28];
            bytes[..4].copy_from_slice(&shape.to_le_bytes());
            for (index, value) in [1.0_f32, 0.0, 0.0, 0.0, 8.0, 8.0].into_iter().enumerate() {
                bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
            }
            snapshot.source.shape = WallShapeSnapshot::from_observed(bytes);
            snapshot.source.curve_kind_tag = shape;
            let mut inner = InnerWalls::new();
            inner.new_wall_with_id(WallId(42), WallState::from_snapshot(snapshot, 0));
            let mut walls = PublicWalls::default();
            walls.consume_with_paths(
                &mut inner,
                &TerrainHeightGrid512 {
                    encoded: vec![0; 512 * 512].into(),
                    minimum_y: 0.0,
                    maximum_y: 0.0,
                    sampling_fix: false,
                }, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
            let wall = walls.get_mut(WallId(42)).unwrap();
            wall.prepared_path = Some(PreparedWallPath {
                positions: vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [8.0, 0.0, 0.0]],
                points_u: vec![0.0, 0.5, 1.0],
                length: 8.0,
                segment_values: vec![0.0; 3],
            });
            wall.segment_heights = vec![0.0; 3];
            let mut subtypes = DerivedDecoratorSubtypes::default();
            subtypes.insert(7, DecoratorSubtype(subtype));
            let mut meshes = DecoratorMeshLibrary::default();
            meshes.collision.insert(
                DecoratorLibKey::new(subtype, 1),
                DecoratorCollision {
                    triangles: Vec::new(),
                    mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                    bounds: utils::geometry::Rectangle3d::new(
                        Vec3::ZERO,
                        Vec3::splat(2.0),
                        Vec2::X,
                    ),
                },
            );
            let mut cache = CachedDecoratorTransforms::default();
            cache.insert(
                7,
                CachedDecoratorTransform {
                    rotation: glam::Quat::IDENTITY,
                    translation: Vec3::new(4.0, 2.0, 0.0),
                    scale: Vec3::ONE,
                },
            );
            Self {
                world: World::new(),
                walls,
                subtypes,
                meshes,
                cache,
                raycast: RaycastWorld::new(),
            }
        }
        fn info(&self, tag: u8, u: f32, y: f32) -> DecoratorInfo {
            DecoratorInfo {
                dst_wall: u64::from(u.to_bits()) | (u64::from(y.to_bits()) << 32),
                decorator_id: 99,
                rank: 1,
                ty: 0x3f,
                u20: u32::from(tag) << 16,
                ..DecoratorInfo::default()
            }
        }
        fn check(
            &mut self,
            info: &DecoratorInfo,
            dst: DecoratorStorageAddr,
        ) -> DecoratorLocationCheckResult {
            let mut state = SystemState::<Query<&Roof>>::new(&mut self.world);
            let roofs = state.get(&self.world);
            is_decorator_location_valid(
                DecoratorId(7),
                info,
                dst,
                &self.walls,
                &roofs,
                DerivedDecoratorInfoView::new(&self.subtypes, &self.meshes),
                &self.cache,
                &self.raycast,
            )
        }
        fn add_roof(&mut self, flat: bool, gable: bool) {
            let mut bytes = [0_u8; 88];
            bytes[8..12].copy_from_slice(&1_u32.to_le_bytes());
            for (offset, value) in [
                (12, 1.0_f32),
                (16, 0.0),
                (20, 0.0),
                (24, 0.0),
                (28, 8.0),
                (32, 8.0),
                (48, if flat { 0.0 } else { 0.5 }),
                (52, if gable { 1.0 } else { 0.0 }),
            ] {
                bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            }
            let entity = self.world.spawn(Roof::from_observed(bytes)).id();
            self.walls.associate_roof(WallId(42), entity);
        }
    }

    #[test]
    fn terrain_and_stair_return_native_eleven_before_any_required_lookup() {
        let mut f = Fixture::new(0, 1);
        f.cache = CachedDecoratorTransforms::default();
        f.subtypes = DerivedDecoratorSubtypes::default();
        f.meshes = DecoratorMeshLibrary::default();
        for tag in [3, 4] {
            let info = f.info(tag, f32::NAN, f32::NAN);
            assert_eq!(
                f.check(&info, DecoratorStorageAddr::Terrain),
                DecoratorLocationCheckResult::Valid
            );
        }
    }
    #[test]
    fn native_result_schema_preserves_every_reason_payload_and_tag() {
        use DecoratorLocationInvalidReason::*;
        let wall = WallId(42);
        let coord = CurveInternalCoord {
            segment: 9,
            t: 0.25,
        };
        let rectangle = Rectangle2d::new_centered_axis_aligned(Vec2::ONE);
        let reasons = [
            WallCutoutBoundary { wall, coord },
            RoofBounds {
                position: Vec3::ONE,
                direction: Vec3::X,
                wall,
            },
            WallEnds {
                left_oob: true,
                right_oob: false,
                wall,
            },
            SharpWallTurn { wall, coord },
            WallTop {
                padding: 0.79,
                wall,
            },
            WallCorner {
                wall,
                rectangle,
                corner: 3,
            },
            BalconyUpperClearance { wall },
            FlatRoofForbidden,
            GableBandForbidden,
            MissingRoof,
            WallLowerBoundary { wall },
        ];
        for (index, reason) in reasons.into_iter().enumerate() {
            let result = DecoratorLocationCheckResult::Invalid(reason);
            assert_eq!(result.native_tag(), index as u8);
            assert!(!result.is_valid());
            assert_eq!(result.get_invalid_reason(), Some(&reason));
        }
        assert_eq!(DecoratorLocationCheckResult::Valid.native_tag(), 11);
    }
    #[test]
    fn absent_roof_is_reason_nine_without_reading_catalog_or_cache() {
        let mut f = Fixture::new(0, 1);
        f.subtypes = DerivedDecoratorSubtypes::default();
        f.cache = CachedDecoratorTransforms::default();
        let info = f.info(2, 0.0, 2.0);
        assert_eq!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
                .native_tag(),
            9
        );
    }
    #[test]
    fn wall_requires_actual_modified_id_cache_before_other_wall_work() {
        let mut f = Fixture::new(0, 1);
        let info = f.info(0, 4.0, 2.0);
        f.cache = CachedDecoratorTransforms::default();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
            ))
            .is_err()
        );
    }
    #[test]
    fn open_wall_checks_both_unclamped_endpoints() {
        let mut f = Fixture::new(0, 2);
        let info = f.info(0, -0.5, 2.0);
        assert_eq!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42))),
            DecoratorLocationCheckResult::Invalid(DecoratorLocationInvalidReason::WallEnds {
                wall: WallId(42),
                left_oob: true,
                right_oob: false,
            })
        );
    }
    #[test]
    fn positive_cut_and_zero_lower_boundary_are_distinct_native_reasons() {
        let mut f = Fixture::new(0, 2);
        f.walls
            .get_mut(WallId(42))
            .unwrap()
            .segment_heights
            .fill(1.0);
        let info = f.info(0, 4.0, 1.0);
        let result = f.check(&info, DecoratorStorageAddr::Wall(WallId(42)));
        assert!(matches!(result, DecoratorLocationCheckResult::Invalid(
            DecoratorLocationInvalidReason::WallCutoutBoundary { coord, .. })
            if coord.segment == 0 && coord.t == 0.75));
        f.walls
            .get_mut(WallId(42))
            .unwrap()
            .segment_heights
            .fill(0.0);
        let info = f.info(0, 4.0, 0.0);
        assert_eq!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
                .native_tag(),
            10
        );
    }
    #[test]
    fn real_three_point_path_turn_precedes_the_unready_arch_height_lookup() {
        let mut f = Fixture::new(0, 2);
        let wall = f.walls.get_mut(WallId(42)).unwrap();
        wall.prepared_path.as_mut().unwrap().positions =
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]];
        let info = f.info(0, 1.0, 2.0);
        let result = f.check(&info, DecoratorStorageAddr::Wall(WallId(42)));
        assert!(matches!(result, DecoratorLocationCheckResult::Invalid(
            DecoratorLocationInvalidReason::SharpWallTurn { coord, .. })
            if coord.segment == 0 && coord.t == 1.0));
    }
    #[test]
    fn first_actual_rectangle_corner_wins_before_optional_roof_check() {
        let mut f = Fixture::new(0, 1);
        let mut info = f.info(0, 0.5, 2.0);
        info.ty = 0x38;
        assert!(matches!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42))),
            DecoratorLocationCheckResult::Invalid(DecoratorLocationInvalidReason::WallCorner {
                wall: WallId(42),
                corner: 0,
                ..
            })
        ));
    }
    #[test]
    fn flat_roof_forbidden_and_gable_half_open_bands_use_current_type_and_coords() {
        let mut f = Fixture::new(0, 1);
        f.add_roof(true, false);
        let mut info = f.info(2, 0.125, 1.0);
        info.ty = 0x3a;
        assert_eq!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
                .native_tag(),
            7
        );
        f.add_roof(false, true);
        info.ty = 0x38;
        assert_eq!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
                .native_tag(),
            8
        );
        info.dst_wall = u64::from(0.25_f32.to_bits()) | (u64::from(1.0_f32.to_bits()) << 32);
        assert!(
            f.check(&info, DecoratorStorageAddr::Wall(WallId(42)))
                .is_valid()
        );
    }
    #[test]
    fn zero_segment_and_unordered_ridge_math_do_not_fabricate_finite_directions() {
        assert!(location_unit3(Vec3::ZERO).is_nan());
        assert!(location_unit2(Vec2::ZERO).is_nan());
        let points = [[0.0, 0.0, 0.0], [0.0, 5.0, 0.0]];
        assert!(location_curve_coord(&points, 0.0).t.is_nan());
        let amount = f32::INFINITY;
        let top = location_elevated(Vec3::ONE, amount);
        assert!(top.x.is_nan() && top.z.is_nan() && top.y.is_infinite());
    }

    // 以下验收使用真实 Bevy World、公共墙发布和原生 Rapier，不以人工候选表代替系统参数。
    use crate::blueprint::{BlueprintMeshSlot, DecoratorBlueprint, PositionedDecoratorMesh};
    use country_core::resources::roofs::commands::EditRoofCmd;
    use country_core::resources::walls::decorator::DeferredDecoratorOpEvents;
    use country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist;
    use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
    use country_core::resources::walls::public_walls::OnWallChanged;
    use country_core::systems::collision::world::{RaycastGeoSource, upsert_collider};
    use country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd;

    impl Fixture {
        fn into_cull_world(self, recording: bool) -> World {
            let mut world = self.world;
            world.insert_resource(self.walls);
            world.insert_resource(self.subtypes);
            world.insert_resource(self.meshes);
            world.insert_resource(self.cache);
            world.insert_resource(self.raycast);
            world.init_resource::<DecoratorStorage>();
            world.init_resource::<LiveDecoratorArchivist>();
            world.init_resource::<DecoratorBlueprints>();
            world.init_resource::<DecoratorPhysicsColliders>();
            world.init_resource::<DeferredDecoratorOpEvents>();
            world.init_resource::<Events<AdjustWallHeightCmd>>();
            world.init_resource::<Events<OnWallChanged>>();
            world.init_resource::<Events<EditRoofCmd>>();
            let mut history: History = History::empty_glade();
            if recording {
                history.start_input_recording(0).unwrap();
            }
            world.insert_resource(history);
            world
        }
    }
    fn cull_schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(cull_oob_decorators);
        schedule
    }
    fn put_cull_decorator(
        world: &mut World,
        address: DecoratorStorageAddr,
        id: u32,
        ty: u8,
        tag: u8,
    ) -> DecoratorInfo {
        // 正式记录的extra变体须与类型一致；历史会按原ty表重建该判别。
        let style = match ty {
            56..=58 => 6,
            59 => 4,
            60 => 7,
            61 => 5,
            _ => 0,
        };
        let info = DecoratorInfo {
            ty,
            u20: u32::from(tag) << 16,
            u40: style << 8,
            decorator_id: 9000 + id,
            rank: 1,
            ..Default::default()
        };
        world
            .resource_mut::<DecoratorStorage>()
            .apply_decorator_place(address, DecoratorId(id), &info);
        world.resource_mut::<CachedDecoratorTransforms>().insert(
            id,
            CachedDecoratorTransform {
                translation: Vec3::new(id as f32, 2.25, -3.5),
                rotation: glam::Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        );
        info
    }
    fn changed(world: &mut World, wall: u64) {
        world
            .resource_mut::<Events<OnWallChanged>>()
            .send(OnWallChanged(WallId(wall)));
    }
    fn has_cull_decorator(world: &World, address: DecoratorStorageAddr, id: u32) -> bool {
        world
            .resource::<DecoratorStorage>()
            .get(address)
            .is_some_and(|bucket| bucket.get(DecoratorId(id)).is_some())
    }
    fn allow_all(world: &mut World) {
        world
            .resource_mut::<LiveDecoratorArchivist>()
            .decorator_cull_allow_any();
    }

    #[test]
    fn cull_native_each_real_candidate_source_uses_current_wall_bucket_and_shared_queue() {
        let address = DecoratorStorageAddr::Wall(WallId(42));
        for source in 0..4 {
            let mut world = Fixture::new(0, 1).into_cull_world(true);
            // 旧快照中没有71：必须扫描活桶。归档的首状态本应在当前编辑之前。
            if source == 0 {
                world.resource_mut::<LiveDecoratorArchivist>().walls.insert(
                    country_core::resources::walls::publication_fields::WallId(42),
                    Default::default(),
                );
            }
            let info = put_cull_decorator(&mut world, address, 71, 56, 2);
            allow_all(&mut world);
            match source {
                1 => {
                    world
                        .resource_mut::<Events<AdjustWallHeightCmd>>()
                        .send(AdjustWallHeightCmd(
                            country_core::systems::wall::height_command::AdjustWallHeightRequest {
                                anchor: None,
                                mode: 0,
                                value: 2.0,
                                wall: country_core::resources::walls::publication_fields::WallId(
                                    42,
                                ),
                                observed_audio_flag: 0,
                            },
                        ));
                }
                2 => changed(&mut world, 42),
                3 => {
                    let mut raw = [0; 64];
                    raw[32..40].copy_from_slice(&42_u64.to_le_bytes());
                    world.resource_mut::<Events<EditRoofCmd>>().send(EditRoofCmd(
                        country_core::resources::roofs::geometry::edit_rules::RoofEditEvent::from_observed(&raw)));
                }
                _ => {}
            }
            let mut schedule = cull_schedule();
            schedule.run(&mut world);
            assert!(
                !has_cull_decorator(&world, address, 71),
                "candidate source {source}"
            );
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| {
                    assert_eq!(events.len(), 1);
                    let event = &events[0];
                    assert_eq!(&event[..8], &4_u64.to_le_bytes());
                    assert_eq!(&event[8..16], &[0; 8]);
                    assert_eq!(&event[16..24], &42_u64.to_le_bytes());
                    assert_eq!(
                        &event[0x18..0x60],
                        &crate::apply_decorator_op::info_to_bytes(info)
                    );
                    assert_eq!(&event[0x60..0x64], &71_u32.to_le_bytes());
                    for (index, value) in [71.0_f32, 2.25, -3.5].into_iter().enumerate() {
                        assert_eq!(
                            &event[0x70 + index * 4..0x74 + index * 4],
                            &value.to_le_bytes()
                        );
                    }
                    assert_eq!(event[0x7c], 0);
                    assert_eq!(event[0x80], 0);
                    assert_eq!(event[0x81], 56);
                });
            schedule.run(&mut world);
            assert_eq!(
                world
                    .resource::<DeferredDecoratorOpEvents>()
                    .with_events(|events| events.len()),
                1
            );
            if source == 0 {
                assert!(
                    world
                        .resource::<LiveDecoratorArchivist>()
                        .walls
                        .values()
                        .next()
                        .unwrap()
                        .items
                        .is_empty()
                );
            }
        }
    }

    #[test]
    fn cull_native_filters_public_presence_permissions_and_all_256_type_tags_without_touching_specials()
     {
        let address = DecoratorStorageAddr::Wall(WallId(42));
        let absent = DecoratorStorageAddr::Wall(WallId(999));
        let mut world = Fixture::new(0, 1).into_cull_world(false);
        for ty in 0_u8..=255 {
            put_cull_decorator(&mut world, address, 1000 + u32::from(ty), ty, 2);
        }
        put_cull_decorator(&mut world, address, 77, 56, 2); // 未允许，必须保留。
        for addr in [
            absent,
            DecoratorStorageAddr::Terrain,
            DecoratorStorageAddr::Stair,
        ] {
            put_cull_decorator(
                &mut world,
                addr,
                80 + match addr {
                    DecoratorStorageAddr::Wall(_) => 0,
                    DecoratorStorageAddr::Terrain => 1,
                    _ => 2,
                },
                56,
                2,
            );
        }
        {
            let mut archive = world.resource_mut::<LiveDecoratorArchivist>();
            archive.culling.allowed_ids.extend(1000..1256);
            archive.culling.allowed_ids.extend(80..83);
            archive.walls.insert(
                country_core::resources::walls::publication_fields::WallId(999),
                Default::default(),
            );
            archive.special1 = Some(Default::default());
            archive.special2 = Some(Default::default());
        }
        let mut schedule = cull_schedule();
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            0
        );
        changed(&mut world, 42);
        changed(&mut world, 999);
        changed(&mut world, 42);
        schedule.run(&mut world);
        for ty in 0_u8..=255 {
            let culled = (56..64).contains(&ty) && ty != 62;
            assert_eq!(
                has_cull_decorator(&world, address, 1000 + u32::from(ty)),
                !culled,
                "type {ty}"
            );
        }
        for (addr, id) in [
            (address, 77),
            (absent, 80),
            (DecoratorStorageAddr::Terrain, 81),
            (DecoratorStorageAddr::Stair, 82),
        ] {
            assert!(has_cull_decorator(&world, addr, id));
        }
        world
            .resource::<DeferredDecoratorOpEvents>()
            .with_events(|events| {
                assert_eq!(events.len(), 7);
                assert!(
                    events
                        .iter()
                        .all(|event| event[0x7c] == 1 && event[0x80] == 0)
                );
            });
    }

    #[test]
    fn cull_native_valid_location_needs_original_collision_slot_same_wall_lower_id_and_external_local_pose()
     {
        let address = DecoratorStorageAddr::Wall(WallId(42));
        for (blocker, blocker_addr, enabled, should_remove) in [
            (6, address, true, true),
            (7, address, true, false),
            (8, address, true, false),
            (6, DecoratorStorageAddr::Wall(WallId(999)), true, false),
            (6, DecoratorStorageAddr::Terrain, true, false),
            (6, address, false, false),
        ] {
            let mut world = Fixture::new(0, 1).into_cull_world(false);
            put_cull_decorator(&mut world, address, 7, 56, 3); // 原位置直接有效分支。
            let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.2), Vec3::X));
            let rotation = glam::Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
            let mut blueprint = DecoratorBlueprint::default();
            // external*local中心=(10,0,-1)，反序会是(11,0,0)，两者不重叠。
            let positioned = PositionedDecoratorMesh {
                mesh,
                position: Vec3::new(10.0, 0.0, 0.0),
                rotation,
            };
            blueprint.interaction_mesh = BlueprintMeshSlot::Mesh(positioned.clone());
            blueprint.collision_mesh = if enabled {
                BlueprintMeshSlot::Mesh(positioned)
            } else {
                BlueprintMeshSlot::Disabled
            };
            world
                .resource_mut::<DecoratorBlueprints>()
                .insert(7, blueprint);
            let blocker_mesh =
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.2), Vec3::ZERO));
            world.resource_scope(|world, mut raycast: Mut<RaycastWorld>| {
                let mut colliders = world.resource_mut::<DecoratorPhysicsColliders>();
                upsert_collider(
                    &mut raycast,
                    &mut colliders.first,
                    DecoratorId(blocker),
                    RaycastGeoSource::decorator(DecoratorId(blocker), blocker_addr),
                    blocker_mesh,
                    Vec3::new(10.0, 0.0, -1.0),
                    glam::Quat::IDENTITY,
                );
                raycast.flush_colliders();
            });
            allow_all(&mut world);
            changed(&mut world, 42);
            cull_schedule().run(&mut world);
            assert_eq!(
                !has_cull_decorator(&world, address, 7),
                should_remove,
                "blocker {blocker} {blocker_addr:?}, enabled {enabled}"
            );
            assert_eq!(
                world.resource::<DecoratorBlueprints>().get(7).is_none(),
                should_remove
            );
        }
    }

    #[test]
    fn cull_native_archives_before_missing_position_and_deletes_nothing_on_collection_failure() {
        let address = DecoratorStorageAddr::Wall(WallId(42));
        let mut world = Fixture::new(0, 1).into_cull_world(true);
        put_cull_decorator(&mut world, address, 71, 56, 2);
        put_cull_decorator(&mut world, address, 72, 56, 2);
        world.insert_resource(CachedDecoratorTransforms::default());
        allow_all(&mut world);
        changed(&mut world, 42);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cull_schedule().run(&mut world)
        }));
        assert!(result.is_err());
        assert!(has_cull_decorator(&world, address, 71));
        assert!(has_cull_decorator(&world, address, 72));
        assert_eq!(
            world
                .resource::<LiveDecoratorArchivist>()
                .walls
                .values()
                .next()
                .unwrap()
                .items
                .len(),
            2
        );
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            0
        );
    }

    #[test]
    fn cull_native_first_snapshot_retains_live_collider_payload_and_multiframe_history_undo_redo() {
        use country_core::resources::walls::decorator::DecoratorIdGen;
        use country_core::resources::walls::decorator_storage::DecoratorColliderMaps;
        let address = DecoratorStorageAddr::Wall(WallId(42));
        let mut world = Fixture::new(0, 1).into_cull_world(true);
        let first = put_cull_decorator(&mut world, address, 71, 56, 2);
        let second = put_cull_decorator(&mut world, address, 72, 57, 2);
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        let weak = Arc::downgrade(&mesh);
        world.resource_scope(|world, mut raycast: Mut<RaycastWorld>| {
            let mut colliders = world.resource_mut::<DecoratorPhysicsColliders>();
            upsert_collider(
                &mut raycast,
                &mut colliders.first,
                DecoratorId(71),
                RaycastGeoSource::decorator(DecoratorId(71), address),
                mesh,
                Vec3::new(71.0, 2.25, -3.5),
                glam::Quat::IDENTITY,
            );
        });
        allow_all(&mut world);
        changed(&mut world, 42);
        let mut schedule = cull_schedule();
        schedule.run(&mut world);
        {
            let archive = world.resource::<LiveDecoratorArchivist>();
            let bucket = archive.walls.values().next().unwrap();
            assert_eq!(bucket.items.len(), 2);
            assert_eq!(
                bucket.items[&71].0,
                crate::apply_decorator_op::info_to_bytes(first)
            );
            assert_eq!(
                bucket.items[&72].0,
                crate::apply_decorator_op::info_to_bytes(second)
            );
            assert_eq!(
                archive.collision_a[&71].0.translation,
                Vec3::new(71.0, 2.25, -3.5)
            );
            assert!(Arc::ptr_eq(
                archive.collision_a[&71].1.data(),
                &weak.upgrade().unwrap()
            ));
        }
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            2
        );
        let mut storage = world.remove_resource::<DecoratorStorage>().unwrap();
        let diff = world
            .resource_mut::<LiveDecoratorArchivist>()
            .extract_diff(&storage, false);
        assert_eq!(diff[&address].before.states.len(), 2);
        assert!(diff[&address].after.states.is_empty());
        let mut raycast = world.remove_resource::<RaycastWorld>().unwrap();
        let mut colliders = world
            .remove_resource::<DecoratorPhysicsColliders>()
            .unwrap();
        let mut maps = DecoratorColliderMaps {
            world: &mut raycast,
            collision_a: &mut colliders.first,
            collision_b: &mut colliders.second,
        };
        let mut ids = DecoratorIdGen::default();
        let events = world.resource::<DeferredDecoratorOpEvents>();
        storage.restore_decorators(&diff, true, &mut ids, &mut maps, events);
        assert_eq!(
            storage.get(address).unwrap().get(DecoratorId(71)),
            Some(&first)
        );
        assert_eq!(storage.lookup_storage(DecoratorId(72)), Some(address));
        storage.restore_decorators(&diff, false, &mut ids, &mut maps, events);
        assert!(storage.lookup_storage(DecoratorId(71)).is_none());
        assert!(storage.lookup_storage(DecoratorId(72)).is_none());
        drop(maps);
        drop(colliders);
        drop(raycast);
        assert!(
            weak.upgrade().is_none(),
            "真实物理表和归档释放后不得残留mesh owner"
        );
    }
}

#[cfg(test)]
mod stair_cull_native_tests {
    use super::*;
    use crate::blueprint::DecoratorBlueprint;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::{
        stairs::{
            stairs_assembly_params::{
                PlayermadeStairsAssemblyParams, PlayermadeStairsAssemblyRequests, StairNodeAnchor,
            },
            stairs_state::StairSegment,
        },
        walls::{
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator::DeferredDecoratorOpEvents, decorator_archivist::LiveDecoratorArchivist,
            decorator_collision::DecoratorPhysicsColliders,
        },
    };

    fn stair_world(recording: bool) -> World {
        let mut world = World::new();
        let mut history: History = History::empty_glade();
        if recording {
            history.start_input_recording(0).unwrap();
        }
        world.insert_resource(history);
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<StairsState>();
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.insert_resource(RaycastWorld::new());
        world
    }
    fn put(
        world: &mut World,
        addr: DecoratorStorageAddr,
        id: u32,
        a: u32,
        b: u32,
    ) -> DecoratorInfo {
        let info = DecoratorInfo {
            dst_wall: u64::from(a) | (u64::from(b) << 32),
            u20: 4 << 16,
            decorator_id: id + 1000,
            rank: 1,
            ..Default::default()
        };
        world
            .resource_mut::<DecoratorStorage>()
            .apply_decorator_place(addr, DecoratorId(id), &info);
        world
            .resource_mut::<DecoratorBlueprints>()
            .insert(id, DecoratorBlueprint::new());
        info
    }
    fn has(world: &World, addr: DecoratorStorageAddr, id: u32) -> bool {
        world
            .resource::<DecoratorStorage>()
            .get(addr)
            .is_some_and(|bucket| bucket.get(DecoratorId(id)).is_some())
    }
    fn allow(world: &mut World) {
        world
            .resource_mut::<LiveDecoratorArchivist>()
            .decorator_cull_allow_any();
    }
    fn request(a: u32, b: u32) -> PlayermadeStairsAssemblyParams {
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::crashy_new_from_points(vec![Vec2::ZERO, Vec2::X], true),
            heights: vec![0.0, 0.0],
            widths: vec![1.0, 1.0],
            tangents: vec![Vec2::X, Vec2::X],
            auxiliary_samples: vec![0.0, 0.0],
            interpolated_curve: None,
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 0); 2],
            terrain_contacts: [false, false].into_iter().collect(),
            segment: StairSegment::new(DecoratorId(a), DecoratorId(b)),
            sub_segments: Default::default(),
            dirty: false,
        }
    }

    #[test]
    fn stair_cull_invalid_1024_original_native_request_scans_use_actual_resource_and_shared_events()
    {
        let fixture = include_str!(
            "../../../../reports/module-recovery-2026-10-23/1412b6420-stair-cull-predicate.txt"
        );
        for (case, line) in fixture.lines().enumerate() {
            let row: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|word| u32::from_str_radix(word, 16).unwrap())
                .collect();
            let mut world = stair_world(false);
            let addr = DecoratorStorageAddr::Stair;
            put(&mut world, addr, 7, row[0], row[1]);
            allow(&mut world);
            for pair in row[3..3 + row[2] as usize * 2].chunks_exact(2) {
                world
                    .resource_mut::<PlayermadeStairsAssemblyRequests>()
                    .requests
                    .push(request(pair[0], pair[1]));
            }
            world
                .run_system_once(cull_stair_attached_decorators_on_invalid_stairs)
                .unwrap();
            let removed = *row.last().unwrap() != 0;
            assert_eq!(!has(&world, addr, 7), removed, "原请求扫描case {case}");
            assert_eq!(
                world.resource::<DecoratorBlueprints>().get(7).is_none(),
                removed
            );
            assert_eq!(
                world
                    .resource::<DeferredDecoratorOpEvents>()
                    .with_events(|events| events.len()),
                usize::from(removed)
            );
            if removed {
                assert!(
                    world
                        .resource::<LiveDecoratorArchivist>()
                        .special2
                        .is_some()
                );
            }
        }
    }

    #[test]
    fn stair_cull_original_option_position_bits_and_recording_flags_reach_shared_remove_event() {
        let fixture = include_str!(
            "../../../../reports/module-recovery-2026-10-23/1412b6420-stair-cull-predicate.positions.txt"
        );
        for recording in [false, true] {
            for line in fixture.lines() {
                let row: Vec<u32> = line
                    .split_ascii_whitespace()
                    .map(|word| u32::from_str_radix(word, 16).unwrap())
                    .collect();
                let mut world = stair_world(recording);
                put(&mut world, DecoratorStorageAddr::Stair, 7, 5, 9);
                allow(&mut world);
                if row[0] == 1 {
                    world.resource_mut::<CachedDecoratorTransforms>().insert(
                        7,
                        CachedDecoratorTransform {
                            translation: Vec3::new(
                                f32::from_bits(row[1]),
                                f32::from_bits(row[2]),
                                f32::from_bits(row[3]),
                            ),
                            rotation: glam::Quat::IDENTITY,
                            scale: Vec3::ONE,
                        },
                    );
                }
                world
                    .run_system_once(cull_stair_attached_decorators)
                    .unwrap();
                world
                    .resource::<DeferredDecoratorOpEvents>()
                    .with_events(|events| {
                        assert_eq!(events.len(), 1);
                        let event = &events[0];
                        assert_eq!(&event[..8], &4_u64.to_le_bytes());
                        assert_eq!(&event[8..16], &2_u64.to_le_bytes());
                        for (index, word) in row[4..].iter().enumerate() {
                            assert_eq!(
                                &event[0x70 + index * 4..0x74 + index * 4],
                                &word.to_le_bytes()
                            );
                        }
                        assert_eq!(event[0x7c], u8::from(!recording));
                        assert_eq!(event[0x80], 0);
                    });
            }
        }
    }

    #[test]
    fn stair_cull_actual_graph_edges_stubs_permission_and_storage_domains_are_distinct() {
        let mut world = stair_world(false);
        let stair = DecoratorStorageAddr::Stair;
        for (id, a, b) in [(1, 5, 9), (2, 12, 12), (3, 8, 9), (4, 20, 20)] {
            put(&mut world, stair, id, a, b);
        }
        put(&mut world, DecoratorStorageAddr::Terrain, 30, 8, 9);
        put(&mut world, DecoratorStorageAddr::Wall(WallId(42)), 31, 8, 9);
        // 未许可的损坏记录不能触发dst_as_stair断言。
        put(&mut world, stair, 5, 1, 2);
        world
            .resource_mut::<DecoratorStorage>()
            .get_mut(stair)
            .unwrap()
            .get_mut(DecoratorId(5))
            .unwrap()
            .u20 = 0;
        world
            .resource_mut::<LiveDecoratorArchivist>()
            .culling
            .allowed_ids
            .extend([1, 2, 3, 4, 30, 31]);
        world.resource_mut::<StairsState>().add_edge(5, 9);
        world.resource_mut::<StairsState>().add_node(12);
        world
            .run_system_once(cull_stair_attached_decorators)
            .unwrap();
        for id in [1, 2, 5] {
            assert!(has(&world, stair, id));
        }
        for id in [3, 4] {
            assert!(!has(&world, stair, id));
        }
        assert!(has(&world, DecoratorStorageAddr::Terrain, 30));
        assert!(has(&world, DecoratorStorageAddr::Wall(WallId(42)), 31));
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            2
        );
    }

    #[test]
    fn stair_cull_all_type_bytes_use_destination_and_malformed_collection_is_atomic() {
        let mut world = stair_world(false);
        let addr = DecoratorStorageAddr::Stair;
        for ty in 0_u8..=255 {
            let id = 1000 + u32::from(ty);
            put(&mut world, addr, id, 5, 9);
            world
                .resource_mut::<DecoratorStorage>()
                .get_mut(addr)
                .unwrap()
                .get_mut(DecoratorId(id))
                .unwrap()
                .ty = ty;
        }
        allow(&mut world);
        world
            .run_system_once(cull_stair_attached_decorators)
            .unwrap();
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(addr)
                .unwrap()
                .decorators()
                .is_empty()
        );
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            256
        );
        for invalid_requests in [false, true] {
            let mut world = stair_world(false);
            put(&mut world, addr, 7, 5, 9);
            put(&mut world, addr, 8, 6, 10);
            world
                .resource_mut::<DecoratorStorage>()
                .get_mut(addr)
                .unwrap()
                .get_mut(DecoratorId(8))
                .unwrap()
                .u20 = 0;
            allow(&mut world);
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    if invalid_requests {
                        world
                            .run_system_once(cull_stair_attached_decorators_on_invalid_stairs)
                            .unwrap();
                    } else {
                        world
                            .run_system_once(cull_stair_attached_decorators)
                            .unwrap();
                    }
                }))
                .is_err()
            );
            assert!(has(&world, addr, 7) && has(&world, addr, 8));
            assert_eq!(world.resource::<DecoratorBlueprints>().map.len(), 2);
            assert!(
                world
                    .resource::<LiveDecoratorArchivist>()
                    .special2
                    .is_none()
            );
            assert_eq!(
                world
                    .resource::<DeferredDecoratorOpEvents>()
                    .with_events(|events| events.len()),
                0
            );
        }
    }

    #[test]
    fn stair_cull_real_physics_archive_first_snapshot_history_undo_redo_and_unload() {
        use country_core::resources::walls::{
            decorator::DecoratorIdGen, decorator_storage::DecoratorColliderMaps,
        };
        use country_core::systems::collision::world::{
            RaycastGeoSource, RaycastMeshData, upsert_collider,
        };
        use std::sync::Arc;
        let mut world = stair_world(true);
        let addr = DecoratorStorageAddr::Stair;
        let first = put(&mut world, addr, 7, 5, 9);
        let second = put(&mut world, addr, 8, 5, 10);
        allow(&mut world);
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        let weak = Arc::downgrade(&mesh);
        world.resource_scope(|world, mut raycast: Mut<RaycastWorld>| {
            let mut colliders = world.resource_mut::<DecoratorPhysicsColliders>();
            upsert_collider(
                &mut raycast,
                &mut colliders.first,
                DecoratorId(7),
                RaycastGeoSource::decorator(DecoratorId(7), addr),
                mesh,
                Vec3::new(3., 4., 5.),
                glam::Quat::IDENTITY,
            );
        });
        world
            .run_system_once(cull_stair_attached_decorators)
            .unwrap();
        {
            let archive = world.resource::<LiveDecoratorArchivist>();
            let bucket = archive.special2.as_ref().unwrap();
            assert_eq!(bucket.items.len(), 2);
            assert_eq!(
                bucket.items[&7].0,
                crate::apply_decorator_op::info_to_bytes(first)
            );
            assert_eq!(
                bucket.items[&8].0,
                crate::apply_decorator_op::info_to_bytes(second)
            );
            assert_eq!(archive.collision_a[&7].0.translation, Vec3::new(3., 4., 5.));
            assert!(Arc::ptr_eq(
                archive.collision_a[&7].1.data(),
                &weak.upgrade().unwrap()
            ));
        }
        world
            .run_system_once(cull_stair_attached_decorators)
            .unwrap();
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| events.len()),
            2
        );
        // 同生产碰撞consumer卸载活句柄；归档仍独立持有真实mesh，不依赖已卸载的物理对象。
        world.init_resource::<crate::blueprint::PrevDecoratorBlueprints>();
        world
            .run_system_once(crate::blueprint::instantiate_blueprint_colliders)
            .unwrap();
        let maps = world.resource::<DecoratorPhysicsColliders>();
        assert!(maps.first.is_empty() && maps.second.is_empty());
        assert!(weak.upgrade().is_some());
        let mut storage = world.remove_resource::<DecoratorStorage>().unwrap();
        let diff = world
            .resource_mut::<LiveDecoratorArchivist>()
            .extract_diff(&storage, false);
        assert_eq!(diff[&addr].before.states.len(), 2);
        assert!(diff[&addr].after.states.is_empty());
        let mut raycast = world.remove_resource::<RaycastWorld>().unwrap();
        let mut colliders = world
            .remove_resource::<DecoratorPhysicsColliders>()
            .unwrap();
        let mut maps = DecoratorColliderMaps {
            world: &mut raycast,
            collision_a: &mut colliders.first,
            collision_b: &mut colliders.second,
        };
        let mut ids = DecoratorIdGen::default();
        let events = world.resource::<DeferredDecoratorOpEvents>();
        storage.restore_decorators(&diff, true, &mut ids, &mut maps, events);
        assert_eq!(storage.get(addr).unwrap().get(DecoratorId(7)), Some(&first));
        assert_eq!(storage.lookup_storage(DecoratorId(8)), Some(addr));
        storage.restore_decorators(&diff, false, &mut ids, &mut maps, events);
        assert!(storage.lookup_storage(DecoratorId(7)).is_none());
        drop(maps);
        drop(colliders);
        drop(raycast);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn stair_cull_noop_frames_preserve_real_owner_change_ticks() {
        for reason in 0..3 {
            for invalid_requests in [false, true] {
                let mut world = stair_world(false);
                if reason != 0 {
                    put(&mut world, DecoratorStorageAddr::Stair, 7, 5, 9);
                }
                if reason == 2 {
                    allow(&mut world);
                    world.resource_mut::<StairsState>().add_edge(5, 9);
                    world
                        .resource_mut::<PlayermadeStairsAssemblyRequests>()
                        .requests
                        .push(request(5, 9));
                }
                world.clear_trackers();
                if invalid_requests {
                    world
                        .run_system_once(cull_stair_attached_decorators_on_invalid_stairs)
                        .unwrap();
                } else {
                    world
                        .run_system_once(cull_stair_attached_decorators)
                        .unwrap();
                }
                assert!(!world.is_resource_changed::<DecoratorStorage>());
                assert!(!world.is_resource_changed::<LiveDecoratorArchivist>());
                assert!(!world.is_resource_changed::<DecoratorBlueprints>());
                assert_eq!(
                    world
                        .resource::<DeferredDecoratorOpEvents>()
                        .with_events(|events| events.len()),
                    0
                );
            }
        }
    }
}
