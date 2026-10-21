from pathlib import Path
w=Path(__file__).parent
p=w/'candidate/tiny-glade/crates/systems/decorator/src/cull_oob_decorators.rs'
s=(w/'before/tiny-glade/crates/systems/decorator/src/cull_oob_decorators.rs').read_text()
a=s.index('/// 越界检查结果'); b=s.index('/// `auto_allow_decorator_culling`')
body=r'''/// 原判别11表示有效；其余判别保存验证器实际消费的失败载荷。
/// 枚举名是本项目的语义命名，不伪造原生内存布局。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorLocationCheckResult {
    Valid,
    Invalid(DecoratorLocationInvalidReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecoratorLocationInvalidReason {
    WallCutoutBoundary { wall: WallId, coord: CurveInternalCoord },
    RoofBounds { position: Vec3, direction: Vec3, wall: WallId },
    WallEnds { left_oob: bool, right_oob: bool, wall: WallId },
    SharpWallTurn { wall: WallId, coord: CurveInternalCoord },
    WallTop { padding: f32, wall: WallId },
    WallCorner { wall: WallId, rectangle: Rectangle2d, corner: usize },
    BalconyUpperClearance { wall: WallId },
    FlatRoofForbidden,
    GableBandForbidden,
    MissingRoof,
    WallLowerBoundary { wall: WallId },
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
    pub fn is_valid(self) -> bool { matches!(self, Self::Valid) }
    pub fn get_invalid_reason(&self) -> Option<&DecoratorLocationInvalidReason> {
        match self { Self::Valid => None, Self::Invalid(reason) => Some(reason) }
    }
    pub fn native_tag(self) -> u8 {
        match self { Self::Valid => 11, Self::Invalid(reason) => reason.native_tag() }
    }
}

use country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve;
use country_core::resources::roofs::{Roof, RoofIntersectionSection};
use country_core::resources::roofs::geometry::surface::RoofSection;
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
use crate::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds;
use crate::decorator_visual::gothic_dormer::gothic_dormer_profile_curve;
use crate::ui_systems::decorator_interaction_intent::{BalconyDoorSnap, snap_balcony_door};

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
                let bounds = derived.mesh_library()
                    .collision_record(DecoratorLibKey::new(subtype, info.rank())).bounds;
                let height = bounds.top - bounds.bottom;
                let width = if subtype == 8 {
                    match info.rank() { 2 => 1.2, 3 => 1.54, _ => 0.72 }
                } else { bounds.footprint.size.x };
                if is_out_of_roof_bounds(&transform, width, height, info.rank(), roof) {
                    let direction = if subtype == 8 {
                        location_unit3(transform.rotation * Vec3::Z)
                    } else { Vec3::ZERO };
                    return Invalid(Reason::RoofBounds {
                        position: location_elevated(transform.translation, height),
                        direction, wall: wall_id,
                    });
                }
                if subtype == 8 {
                    let forward = transform.rotation * Vec3::Z;
                    let probe = location_unit2(Vec2::new(forward.x, forward.z));
                    let result = check_if_dormer_ridge_is_within_roof_bounds(
                        subtype, info, transform.translation, probe, wall_id, walls, roofs,
                    );
                    if !result.is_valid() { return result; }
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
        _ => is_wall_decorator_location_valid(id, info, dst, walls, roofs, derived, transforms, raycast),
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
        let bounds = derived.mesh_library()
            .collision_record(DecoratorLibKey::new(1, info.rank())).bounds;
        let height = bounds.top - bounds.bottom;
        if is_out_of_roof_bounds(&transform, bounds.footprint.size.x, height, info.rank(), roof) {
            return Invalid(Reason::RoofBounds {
                position: location_elevated(transform.translation, height),
                direction: Vec3::ZERO, wall: wall_id,
            });
        }
    }
    // 原墙分支再次读取当前子类型，随后先读取collision，再required墙和路径。
    let subtype_value = derived.subtype(id);
    let subtype = subtype_value.0;
    let bounds = derived.mesh_library()
        .collision_record(DecoratorLibKey::new(subtype, info.rank())).bounds;
    let width = bounds.footprint.size.x;
    let height = bounds.top - bounds.bottom;
    let wall_id = dst.as_wall();
    let wall = walls.get(wall_id).expect("decorator wall does not exist");
    let space = WallSpace { is_loop: wall.shape().tag() != 2, length: wall.wall_space_length() };
    let points = wall.prepared_path.as_ref().expect("decorator wall path has not been prepared")
        .positions.as_slice();
    let center = location_curve_coord(points, space.project_clamp(0.0 + coord.u));
    let half_width = width * 0.5;
    let left = location_curve_coord(points, space.project_clamp(coord.u - half_width));
    let right = location_curve_coord(points, space.project_clamp(coord.u + half_width));
    let center_point = location_sample(points, center);
    // Y沿原左乘补数、右乘参数顺序插值，与XYZ反馈采样的加数顺序独立。
    let base_y = points[center.segment][1] * (1.0 - center.t)
        + points[center.segment + 1][1] * center.t;
    if subtype == 7 {
        let tangent = location_unit3(Vec3::from_array(points[center.segment + 1])
            - Vec3::from_array(points[center.segment]));
        let probe = location_unit2(Vec2::new(-tangent.z, tangent.x));
        let result = check_if_dormer_ridge_is_within_roof_bounds(
            subtype, info, coord.to_world_space(wall.simplified_curve_2d()),
            probe, wall_id, walls, roofs,
        );
        if !result.is_valid() { return result; }
    }
    let right_oob = space.project(coord.u + half_width).is_none();
    let left_oob = space.project(coord.u - half_width).is_none();
    if left_oob || right_oob {
        return Invalid(Reason::WallEnds { left_oob, right_oob, wall: wall_id });
    }
    for sample in [left, center, right] {
        let cut = location_height(&wall.segment_heights, sample);
        let threshold = match subtype {
            4 | 9 => coord.y + if cut > 0.0 { height * -0.5 } else { 0.2 },
            6 => coord.y + gothic_half_dormer_profile_curve(info.rank())
                .last().expect("gothic half dormer profile is empty").y,
            11 | 12 => coord.y,
            16 => coord.y + 0.3,
            25 => coord.y + 0.2,
            _ => coord.y + height * -0.5,
        };
        if base_y + cut > threshold {
            return Invalid(if cut > 0.0 {
                Reason::WallCutoutBoundary { wall: wall_id, coord: sample }
            } else { Reason::WallLowerBoundary { wall: wall_id } });
        }
    }
    if subtype != 3 && wall.shape().tag() >= 2 {
        let a = location_sample(points, left);
        let b = center_point;
        let c = location_sample(points, right);
        let ab = location_unit2(Vec2::new(b.x - a.x, b.z - a.z));
        let bc = location_unit2(Vec2::new(c.x - b.x, c.z - b.z));
        if 0.6 > ab.y * bc.y + ab.x * bc.x {
            return Invalid(Reason::SharpWallTurn { wall: wall_id, coord: center });
        }
    }
    if subtype == 4 || subtype == 9 {
        let tangent = location_unit3(Vec3::from_array(points[center.segment + 1])
            - Vec3::from_array(points[center.segment]));
        let probe = location_unit2(Vec2::new(tangent.x, tangent.z));
        if matches!(snap_balcony_door(true, info,
            Vec3::new(center_point.x, coord.y, center_point.z), probe,
            coord.u, wall_id, wall, roofs, derived.mesh_library(), true, raycast),
            BalconyDoorSnap::RejectedUpperClearance)
        {
            return Invalid(Reason::BalconyUpperClearance { wall: wall_id });
        }
    }
    if !subtype_value.can_be_on_corner() {
        if let Some(rectangle) = wall.is_rectangle() {
            for corner in 0..4 {
                let corner_u = space.project_clamp(rectangle.get_point_u(corner) * space.length);
                if should_snap_to_corner(info.decorator_type(), wall.building_projection().is_halftimbered(),
                    wall, WallSpaceCoord::new_unchecked(corner_u, coord.y), coord, width)
                {
                    return Invalid(Reason::WallCorner { wall: wall_id, rectangle, corner });
                }
            }
        }
    }
    if wall.shape().tag() < 2 {
        let Some(roof) = walls.roof_entity(wall_id).and_then(|entity| roofs.get(entity).ok()) else {
            return Valid;
        };
        let use_arch_heights = if subtype == 3 { false } else if roof.is_gable() {
            let rectangle = location_roof_rectangle(roof);
            let sides = roof.gable_sides().expect("gable roof has no gable sides");
            let edge = rectangle.get_closest_edge_id(Vec2::new(center_point.x, center_point.z));
            edge == usize::from(sides[0]) || edge == usize::from(sides[1])
        } else { false };
        if !use_arch_heights {
            if subtype != 3 && (info.ty == 0x3b || info.ty == 0x3f
                || (info.decorator_type().is_window() && !roof.is_flat()))
            {
                if coord.y + height * -0.25 > wall.max_y() {
                    return Invalid(Reason::WallTop { padding: 0.0, wall: wall_id });
                }
                return Valid;
            }
            let padding = if roof.is_flat() {
                if subtype == 3 { 0.79 } else { 0.69 }
            } else { 0.1 };
            let offset = if subtype == 11 || subtype == 12 { padding } else { height * -0.5 };
            if coord.y - offset > wall.max_y() - padding {
                return Invalid(Reason::WallTop { padding, wall: wall_id });
            }
            return Valid;
        }
    }
    let arch_heights = wall.arch_segment_heights().expect("wall arch heights have not been calculated");
    for sample in [left, right] {
        let ceiling = location_height(arch_heights, sample);
        let padding = if wall.shape().tag() < 2 || info.decorator_type().is_window() { 0.25 } else { 0.67 };
        let offset = if subtype == 11 || subtype == 12 { -padding } else { height * 0.5 };
        if coord.y + offset > ceiling - padding {
            return Invalid(Reason::WallTop { padding, wall: wall_id });
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
    Vec3::new(amount * 0.0 + position.x, amount + position.y, amount * 0.0 + position.z)
}

/// 原Roof枚举的矩形载荷解码；几何算法仍由同一Rectangle2d实现。
fn location_roof_rectangle(roof: &Roof) -> Rectangle2d {
    let bytes = &roof.observed()[8..0x24];
    assert_eq!(u32::from_le_bytes(bytes[..4].try_into().unwrap()), 1,
        "gable roof enclosure is not a rectangle");
    location_rectangle_payload(&bytes[4..])
}
fn location_rectangle_payload(bytes: &[u8]) -> Rectangle2d {
    let field = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    Rectangle2d { axis: Vec2::new(field(0), field(4)),
        center: Vec2::new(field(8), field(12)), size: Vec2::new(field(16), field(20)) }
}

'''
s=s[:a]+body+s[b:]
a=s.index('/// `should_snap_to_corner`');b=s.index('#[cfg(test)]',a)
s=s[:a]+r'''/// B140811900/200B：只比较原墙空间的水平差，包含闭环投影。
/// halftimbered偏移为0.2，另一分支为0.46；阈值使用完整宽度的一半。
pub fn should_snap_to_corner(
    decorator_type: DecoratorType,
    halftimbered: bool,
    wall: &PublicWallState,
    a: WallSpaceCoord,
    b: WallSpaceCoord,
    threshold: f32,
) -> bool {
    if !decorator_type.should_offer_snap_to_corner() { return false; }
    let space = WallSpace { is_loop: wall.shape().tag() != 2, length: wall.wall_space_length() };
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
    if !(subtype == 7 || subtype == 8) { return Valid; }
    let Some(roof) = walls.roof_entity(wall_id).and_then(|entity| roofs.get(entity).ok()) else {
        return Invalid(Reason::MissingRoof);
    };
    if u32::from_le_bytes(roof.observed()[8..12].try_into().unwrap()) == 0 { return Valid; }
    let profile = if subtype == 7 { gothic_half_dormer_profile_curve(info.rank()) }
        else { gothic_dormer_profile_curve(info.rank()) };
    let top = location_elevated(position, profile.first().expect("dormer profile is empty").y);
    let invalid = || Invalid(Reason::RoofBounds {
        position: top, direction: Vec3::new(probe.x, 0.0, probe.y), wall: wall_id,
    });
    let point = Vec2::new(top.x, top.z);
    let closest = match roof.roof_shape_at_intersection_y(top.y) {
        RoofIntersectionSection::OutOfBounds => return invalid(),
        RoofIntersectionSection::Section(RoofSection::Rectangle { frame, dimensions }) => {
            Rectangle2d { axis: Vec2::new(frame[0], frame[1]),
                center: Vec2::new(frame[2], frame[3]), size: Vec2::from_array(dimensions) }
                .closest_point_on_perimeter(point)
        }
        RoofIntersectionSection::Section(RoofSection::Circle { frame, radius, word_0x18 }) => {
            Circle2d::new_rotated(Vec2::from_array(frame), radius, f32::from_bits(word_0x18))
                .closest_point_on_perimeter(point)
        }
        RoofIntersectionSection::RawSection(bytes) => {
            let field = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
            if u32::from_le_bytes(bytes[..4].try_into().unwrap()) == 0 {
                Circle2d::new_rotated(Vec2::new(field(4), field(8)), field(12), field(16))
                    .closest_point_on_perimeter(point)
            } else { location_rectangle_payload(&bytes[4..]).closest_point_on_perimeter(point) }
        }
    };
    if top.y > (roof.height() + roof.floor_y()) + -0.1 { return invalid(); }
    let normal = location_unit2(point - closest);
    let dot = normal.y * probe.y + normal.x * probe.x;
    // 原ucomiss/jbe将unordered也视为通过，不改成dot>=threshold。
    if 0.993 > dot { invalid() } else { Valid }
}

''' + s[b:]
p.write_text(s)
