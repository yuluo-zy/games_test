//! 悬挑段查找（construct_elevation_supports/find_overlap_segments.rs）。
//!
//! `find_overhang_support_segments`（0x141248620，43KB 大函数）+
//! `OverhangSupportFinderCache::new`（0x141248300）：悬挑支撑段检测缓存。
//!
//! 2026-10-17 复核：旧实现（只取缓存首项）为**近似体**，已移除。
//! fresh（`evidence/elevsys-0x1407867d0.c` 等）已确认：
//! `OverhangSupportFinderCache` 为屋顶和预制墙两个候选 Vec；
//! 输出合同和主要分支已有候选实现；完整原函数差分及生产接线尚未完成。

/// 原临时查找缓存的两个有序候选表。引用用墙ID和实体身份表示，
/// 使用者必须在同一次不可变查询期间解析，避免伪造原 ECS 存储地址。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct OverhangSupportFinderCache {
    pub roofs: Vec<(country_core::resources::walls::WallId, bevy_ecs::entity::Entity)>,
    pub walls: Vec<country_core::resources::walls::WallId>,
}

impl OverhangSupportFinderCache {
    /// B 0x141248300：按公共墙遍历顺序，分别收集有效屋顶和预制墙。
    pub fn new(
        public: &country_core::resources::walls::public_walls::PublicWalls,
        roofs: &bevy_ecs::system::Query<&country_core::resources::roofs::Roof>,
    ) -> Self {
        let mut cache = Self::default();
        for (id, wall) in public.all() {
            if let Some(entity) = public.roof_entity(id) {
                if roofs.get(entity).is_ok() {
                    cache.roofs.push((id, entity));
                }
            }
            if wall.is_premade_shape() {
                cache.walls.push(id);
            }
        }
        cache
    }
}

/// 原查找器输出的单段业务数据；字段采用安全类型而不复制私有Vec ABI。
/// 原记录 +0..+0x37 为两向量与曲线长度，+0x38 为墙ID，
/// +0x40/+0x44 为参数区间，+0x48 为标志。标志的业务名仍待确认。
#[derive(Clone, Debug)]
pub struct OverhangSegment {
    pub curve: country_core::resources::roofs::geometry::surface::Curve2,
    pub wall: country_core::resources::walls::WallId,
    pub interval: [utils::curve::CurveU; 2],
    pub flag: bool,
}

/// 原 closure0：把交点投影到曲线参数并按原比较次序限制到 [0,1]。
fn intersection_parameters(curve: &utils::curve::Curve2, points: &[glam::Vec2]) -> Vec<f32> {
    points.iter().map(|&point| {
        let u = curve.get_approx_u_from_pos(point);
        let lower = if 0.0 <= u { u } else { 0.0 };
        if lower <= 1.0 { lower } else { 1.0 }
    }).collect()
}

/// 原交点支线的高度门控：使用有序比较，NaN 不进入该支线。
fn overlap_height_gate(source_height: f32, target_height: f32, source_max_y: f32) -> bool {
    source_height + 0.6 <= target_height && target_height < source_max_y - 0.6
}

/// 原 0x1412497e3/7ec：两次 UCOMISS 的无序条件都跳过当前墙。
fn containment_height_gate(source_height: f32, target_height: f32, source_max_y: f32) -> bool {
    target_height >= source_height + 0.6 && source_max_y - 0.6 > target_height
}

/// 原相交候选分支：按圆周参数配对，按中点在目标形状内筛选并切片。
/// 调用者须先添加原版矩形角点，再传入完整的交点参数。
fn intersection_segments(
    curve: &utils::curve::Curve2,
    wall: country_core::resources::walls::WallId,
    mut parameters: Vec<f32>,
    minimum_length: f32,
    trim: bool,
    inside: impl Fn(glam::Vec2) -> bool,
) -> Vec<OverhangSegment> {
    parameters.sort_by(|a, b| a.partial_cmp(b).expect("原交点参数不能为NaN"));
    let wrap = |v: f32| {
        let v = v % 1.0;
        if v < 0.0 { v + 1.0 } else { v }
    };
    let to_owned = |c: &utils::curve::Curve2| country_core::resources::roofs::geometry::surface::Curve2 {
        points: c.points.iter().map(|p| p.to_array()).collect(),
        points_u: c.points_u.clone(),
        length: c.length,
    };
    if parameters.len() == 1 && minimum_length == 0.0 {
        let raw_end = -0.3 / curve.length + 1.0;
        assert!(!raw_end.is_nan(), "原单交点环绕参数不能为NaN");
        let end = wrap(raw_end);
        return vec![OverhangSegment { curve: to_owned(curve), wall,
            interval: [utils::curve::CurveU::from_raw(0.0), utils::curve::CurveU::from_raw(end)], flag: false }];
    }
    let mut result = Vec::new();
    for index in 0..parameters.len() {
        let mut start = parameters[index];
        let mut end = parameters[(index + 1) % parameters.len()];
        let forward_end = if end < start { end + 1.0 } else { end };
        let midpoint = wrap(forward_end * 0.5 + start * 0.5);
        let midpoint = utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(midpoint);
        if !inside(curve.get_pos_at_u(midpoint.get())) || !(minimum_length < (forward_end - start) * curve.length) {
            continue;
        }
        if trim {
            let inset = 0.3 / curve.length;
            start = wrap(start + inset);
            end = wrap(end - inset);
        }
        if let Ok(piece) = curve.circular_slice(start, end) {
            result.push(OverhangSegment { curve: to_owned(&piece), wall,
                interval: [utils::curve::CurveU::from_raw(start), utils::curve::CurveU::from_raw(end)], flag: false });
        }
    }
    result
}

/// 原交点表分支；保留哈希表遍历顺序和每对墙的完整交点顺序。
fn segments_from_intersections(
    target: country_core::resources::walls::WallId,
    target_wall: &country_core::resources::walls::public_walls::PublicWallState,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    intersections: &country_core::resources::shape_intersections::ShapeIntersections,
    trim: bool,
    minimum_length: f32,
) -> Vec<OverhangSegment> {
    use country_core::resources::glade_border::{BorderShape, GladeBorder};
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let target_shape = ShapeParameters::from_snapshot(target_wall.get_wall_shape()).get_shape().unwrap();
    let border = GladeBorder(match target_shape { Shape::Circle(v) => BorderShape::Circle(v), Shape::Rectangle(v) => BorderShape::Rectangle(v) });
    let target_height = target_wall.authority.elevation();
    let mut result = Vec::new();
    for (pair, points) in &intersections.0 {
        let other = if pair[0] == target { pair[1] } else if pair[1] == target { pair[0] } else { continue };
        let wall = walls.get(other).expect("交点引用的墙必须存在");
        if !wall.is_premade_shape() || !overlap_height_gate(wall.authority.elevation(), target_height, wall.max_y()) { continue; }
        let shape = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape().unwrap();
        // 原调用 0x141248c1e 使用固定采样间距 0x3ef5c290。
        let curve = match shape {
            Shape::Circle(circle) => circle.as_curve2(f32::from_bits(0x3ef5c290)),
            Shape::Rectangle(rectangle) => rectangle.as_curve2(),
        };
        let mut parameters = intersection_parameters(&curve, points);
        if trim && matches!(target_shape, Shape::Rectangle(_)) {
            if let Shape::Rectangle(rectangle) = shape {
                parameters.extend((0..=4).map(|index| rectangle.get_point_u(index)));
            }
        }
        result.extend(intersection_segments(&curve, other, parameters, minimum_length, trim, |point| border.is_pos_inside(point)));
    }
    result
}

/// 原屋顶山墙分支：仅矩形截面和有效双山墙边参与。
fn segments_from_roofs(
    target: country_core::resources::walls::WallId,
    target_wall: &country_core::resources::walls::public_walls::PublicWallState,
    cache: &OverhangSupportFinderCache,
    roofs: &bevy_ecs::system::Query<&country_core::resources::roofs::Roof>,
    trim: bool,
    minimum_length: f32,
) -> Vec<OverhangSegment> {
    use country_core::resources::roofs::{roof_shape::RoofIntersectionSection, geometry::surface::RoofSection};
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let target_shape = ShapeParameters::from_snapshot(target_wall.get_wall_shape());
    let mut result = Vec::new();
    for &(wall, entity) in &cache.roofs {
        if wall == target { continue; }
        let roof = roofs.get(entity).expect("悬挑缓存中的屋顶必须仍然有效");
        let section = roof.roof_shape_at_intersection_y(target_wall.authority.elevation());
        let rectangle = match section {
            RoofIntersectionSection::RawSection(raw) => match ShapeParameters::from_observed(raw).get_shape() {
                Some(Shape::Rectangle(rectangle)) => rectangle,
                _ => continue,
            },
            RoofIntersectionSection::Section(RoofSection::Rectangle { frame, dimensions }) =>
                utils::geometry::Rectangle2d { axis: glam::Vec2::new(frame[0],frame[1]), center: glam::Vec2::new(frame[2],frame[3]), size: glam::Vec2::from_array(dimensions) },
            _ => continue,
        };
        let Some(sides) = roof.gable_sides() else { continue };
        for side in sides {
            let i = side as usize;
            let line = [rectangle.get_point(i), rectangle.get_point(i + 1)];
            let Some([a,b]) = country_core::systems::utils_intersect::line_x_shape_overlap(
                line, utils::curve::CurveU::new(rectangle.get_point_u(i)),
                utils::curve::CurveU::new(rectangle.get_point_u(i + 1)), &target_shape, 0.0,
            ) else { continue };
            let mut start = a.get();
            let mut end = b.get();
            let perimeter = rectangle.perimeter();
            if !(minimum_length / perimeter <= (if end < start { end + 1.0 } else { end }) - start) { continue; }
            if trim {
                let wrap = |x: f32| { let x = x % 1.0; if x < 0.0 { x + 1.0 } else { x } };
                let inset = 0.3 / perimeter;
                start = wrap(start + inset);
                end = wrap(end - inset);
            }
            if let Ok(piece) = rectangle.as_curve2().circular_slice(start, end) {
                result.push(OverhangSegment {
                    curve: country_core::resources::roofs::geometry::surface::Curve2 {
                        points: piece.points.iter().map(|p| p.to_array()).collect(), points_u: piece.points_u, length: piece.length,
                    },
                    wall,
                    interval: [utils::curve::CurveU::from_raw(start),utils::curve::CurveU::from_raw(end)], flag:false,
                });
            }
        }
    }
    result
}

/// 原 finder 的四种形状包含判定；边界按原严格比较保留。
fn shape_is_contained(
    target: country_core::systems::wall::valid_enclosure::Shape,
    source: country_core::systems::wall::valid_enclosure::Shape,
) -> bool {
    use country_core::systems::wall::valid_enclosure::Shape;
    match (target, source) {
        (Shape::Rectangle(target), Shape::Circle(source)) => target.contains_point(source.center) && source.radius < target.signed_distance_to_closest_edge(source.center).abs(),
        (Shape::Circle(target), Shape::Circle(source)) => {
            let delta = source.center - target.center;
            target.radius > source.radius && target.radius > (delta.y * delta.y + delta.x * delta.x).sqrt()
        },
        (target, Shape::Rectangle(source)) => (0..4).all(|i| target.contains_point(source.get_point(i))),
    }
}

/// 原完整包含支线：圆柱源给完整圆环，矩形源给四个有向边。
fn segments_from_contained_walls(
    target: country_core::resources::walls::WallId,
    target_wall: &country_core::resources::walls::public_walls::PublicWallState,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    cache: &OverhangSupportFinderCache,
    trim: bool,
) -> Vec<OverhangSegment> {
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let target_shape = ShapeParameters::from_snapshot(target_wall.get_wall_shape()).get_shape().unwrap();
    let target_elevation = target_wall.authority.elevation();
    let wrap = |v: f32| { let v = v % 1.0; if v < 0.0 { v + 1.0 } else { v } };
    let to_owned = |c: &utils::curve::Curve2| country_core::resources::roofs::geometry::surface::Curve2 {
        points: c.points.iter().map(|p| p.to_array()).collect(), points_u: c.points_u.clone(), length: c.length,
    };
    let mut result = Vec::new();
    for &id in &cache.walls {
        if id == target { continue; }
        // get_unchecked 只跳过曲线校验，仍是安全的 HashMap 查找；缓存失效明确失败。
        let wall = walls.get_unchecked(id).expect("悬挑缓存中的墙必须仍然存在");
        if !containment_height_gate(wall.authority.elevation(), target_elevation, wall.max_y()) { continue; }
        let shape = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape().expect("缓存只包含预制墙");
        let contained = shape_is_contained(target_shape, shape);
        if !contained { continue; }
        match shape {
            Shape::Circle(circle) => {
                let raw_end = -0.3 / circle.perimeter() + 1.0;
                assert!(!raw_end.is_nan(), "原完整圆环参数不能为NaN");
                let curve = circle.as_curve2(f32::from_bits(0x3ef5c290));
                result.push(OverhangSegment { curve: to_owned(&curve), wall: id,
                    interval: [utils::curve::CurveU::from_raw(0.0), utils::curve::CurveU::from_raw(wrap(raw_end))], flag: true });
            }
            Shape::Rectangle(rectangle) => {
                let curve = rectangle.as_curve2();
                for i in 0..4 {
                    let mut start = rectangle.get_point_u(i);
                    let mut end = rectangle.get_point_u((i + 1) % 4);
                    if trim {
                        let inset = 0.3 / rectangle.perimeter();
                        start = wrap(start + inset);
                        end = wrap(end - inset);
                    }
                    if let Ok(piece) = curve.circular_slice(start, end) {
                        result.push(OverhangSegment { curve: to_owned(&piece), wall: id,
                            interval: [utils::curve::CurveU::from_raw(start),utils::curve::CurveU::from_raw(end)], flag: true });
                    }
                }
            }
        }
    }
    result
}

/// B 0x141248620：按原顺序合并交点、屋顶山墙和完整包含候选。
/// 不在此去重或按墙排序；源表迭代顺序和每个分支的输出顺序均保留。
pub fn find_overhang_support_segments(
    target: country_core::resources::walls::WallId,
    target_wall: &country_core::resources::walls::public_walls::PublicWallState,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    intersections: &country_core::resources::shape_intersections::ShapeIntersections,
    cache: &OverhangSupportFinderCache,
    roofs: &bevy_ecs::system::Query<&country_core::resources::roofs::Roof>,
    trim: bool,
    minimum_length: f32,
) -> Vec<OverhangSegment> {
    let mut result = segments_from_intersections(target, target_wall, walls, intersections, trim, minimum_length);
    result.extend(segments_from_roofs(target, target_wall, cache, roofs, trim, minimum_length));
    result.extend(segments_from_contained_walls(target, target_wall, walls, cache, trim));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::{prelude::*, system::RunSystemOnce};
    use country_core::resources::{roofs::{Roof, geometry::surface::Curve2}, walls::{InnerWalls, WallState, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512}};
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    /// 原 finder 完整入口执行至包含支线的形状检查，逐样本比较是否进入。
    #[test]
    fn containment_height_matches_native_finder_branch() {
        let rows: &[(u32, u32, u32, bool)] = &[
            (0x3f800000, 0x40400000, 0x40a00000, true),
            (0x3f800000, 0x3fcccccd, 0x40a00000, true),
            (0x3f800000, 0x40400000, 0x40666666, false),
            (0x3f800000, 0x00000000, 0x40a00000, false),
            (0x3f800000, 0x3f19999a, 0x40a00000, false),
            (0x3f800000, 0x3f800000, 0x40a00000, false),
            (0x3f800000, 0x40400000, 0x40a00000, true),
            (0x3f800000, 0x40a00000, 0x40a00000, false),
            (0x3f800000, 0x7f800000, 0x40a00000, false),
            (0x3f800000, 0xff800000, 0x40a00000, false),
            (0x3f800000, 0x7fc12345, 0x40a00000, false),
            (0x3f800000, 0x3fcccccc, 0x40a00000, false),
            (0x3f800000, 0x3fcccccd, 0x40a00000, true),
            (0x3f800000, 0x3fccccce, 0x40a00000, true),
            (0x3f800000, 0x40400000, 0x40666665, false),
            (0x3f800000, 0x40400000, 0x40666666, false),
            (0x3f800000, 0x40400000, 0x40666667, false),
            (0x00000000, 0x40400000, 0x40a00000, true),
            (0x3f19999a, 0x40400000, 0x40a00000, true),
            (0x3f800000, 0x40400000, 0x40a00000, true),
            (0x40400000, 0x40400000, 0x40a00000, false),
            (0x40a00000, 0x40400000, 0x40a00000, false),
            (0x7f800000, 0x40400000, 0x40a00000, false),
            (0xff800000, 0x40400000, 0x40a00000, true),
            (0x7fc12345, 0x40400000, 0x40a00000, false),
            (0x3f800000, 0x40400000, 0x00000000, false),
            (0x3f800000, 0x40400000, 0x3f19999a, false),
            (0x3f800000, 0x40400000, 0x3f800000, false),
            (0x3f800000, 0x40400000, 0x40400000, false),
            (0x3f800000, 0x40400000, 0x40a00000, true),
            (0x3f800000, 0x40400000, 0x7f800000, true),
            (0x3f800000, 0x40400000, 0xff800000, false),
            (0x3f800000, 0x40400000, 0x7fc12345, false),
        ];
        for &(source, target, maximum, expected) in rows {
            assert_eq!(containment_height_gate(f32::from_bits(source), f32::from_bits(target), f32::from_bits(maximum)), expected,
                "source={source:08x} target={target:08x} max={maximum:08x}");
        }
    }
    /// 原 finder 执行到形状包含分支的输出入口或下一个候选。
    #[test]
    fn shape_containment_matches_native_finder_branch() {
        use utils::geometry::{Circle2d, Rectangle2d};
        let circle = |x, radius| Shape::Circle(Circle2d { center: glam::Vec2::new(x, 0.0), radius, rotation: 0.0 });
        let rectangle = |x, size| Shape::Rectangle(Rectangle2d::new(glam::Vec2::new(x, 0.0), glam::Vec2::splat(size), glam::Vec2::X));
        let cases = [
            (circle(0.0, 5.0), circle(0.0, 2.0), true),
            (circle(0.0, 2.0), circle(0.0, 2.0), false),
            (circle(0.0, 5.0), circle(6.0, 1.0), false),
            (rectangle(0.0, 10.0), circle(0.0, 2.0), true),
            (rectangle(0.0, 4.0), circle(0.0, 2.0), false),
            (circle(0.0, 5.0), rectangle(0.0, 2.0), true),
            (circle(0.0, 1.0), rectangle(0.0, 2.0), false),
            (rectangle(0.0, 10.0), rectangle(0.0, 2.0), true),
            (rectangle(0.0, 2.0), rectangle(5.0, 2.0), false),
        ];
        for (target, source, expected) in cases { assert_eq!(shape_is_contained(target, source), expected, "target={target:?} source={source:?}"); }
    }
    #[test]
    fn original_height_branches_preserve_ordered_and_unordered_comparisons() {
        assert!(overlap_height_gate(2.0, 3.0, 5.0));
        assert!(!overlap_height_gate(2.0, 2.0, 5.0));
        assert!(!overlap_height_gate(2.0, f32::NAN, 5.0));
        assert!(!containment_height_gate(2.0, f32::NAN, 5.0));
        assert!(!containment_height_gate(2.0, 2.0, 5.0));
    }
    #[test]
    fn intersection_segments_preserve_cyclic_pair_order_and_short_cutoff() {
        let curve = utils::geometry::Rectangle2d::new(glam::Vec2::ZERO, glam::Vec2::splat(2.0), glam::Vec2::X).as_curve2();
        let id = country_core::resources::walls::WallId(9);
        let pieces = intersection_segments(&curve, id, vec![0.75, 0.25, 0.5], 0.0, false, |_| true);
        let intervals: Vec<_> = pieces.iter().map(|s| [s.interval[0].get(), s.interval[1].get()]).collect();
        assert_eq!(intervals, vec![[0.25, 0.5], [0.5, 0.75], [0.75, 0.25]]);
        assert!(intersection_segments(&curve, id, vec![0.25, 0.5], 100.0, false, |_| true).is_empty());
        assert!(intersection_segments(&curve, id, vec![0.25, 0.5], 0.0, false, |_| false).is_empty());
    }
    #[test]
    fn projects_intersections_into_curve_parameter_range() {
        let curve = utils::curve::Curve2::try_new(vec![glam::Vec2::ZERO, glam::Vec2::new(4.0, 0.0)], false).unwrap();
        assert_eq!(intersection_parameters(&curve, &[glam::Vec2::new(-2.0, 0.0), glam::Vec2::new(2.0, 1.0), glam::Vec2::new(8.0, 0.0)]), vec![0.0, 0.5, 1.0]);
    }
    #[test]
    fn intersection_branch_emits_single_crossing_curve_and_checks_owner() {
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let source = inner.new_wall(WallState::new(curve.clone(), 10.0, 0, 0, false));
        let target = inner.new_wall(WallState::new(curve, 10.0, 0, 0, false));
        inner.update_elevation(source, 1.0);
        inner.update_elevation(target, 4.0);
        inner.update_wall_state_enclosure(source, ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: glam::Vec2::ZERO, radius: 2.0, rotation: 0.0,
        })).to_snapshot());
        inner.update_wall_state_enclosure(target, ShapeParameters::from_shape(Shape::Rectangle(utils::geometry::Rectangle2d::new(
            glam::Vec2::ZERO, glam::Vec2::splat(5.0), glam::Vec2::X))).to_snapshot());
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut intersections = country_core::resources::shape_intersections::ShapeIntersections::default();
        intersections.0.insert([target, source], vec![glam::Vec2::new(2.0,0.0)]);
        let result = segments_from_intersections(target, public.get(target).unwrap(), &public, &intersections, false, 0.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].wall, source);
        assert_eq!(result[0].interval[0].get(), 0.0);
        assert!(result[0].curve.points.len() > 2);
        // 单交点原 finder 整段输出；原墙查询以已知有效墙桥接。
        assert_eq!(result[0].curve.points.iter().flat_map(|point| point.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), [0x40000000, 0x34bbbd2e, 0x3ff88fa5, 0xbef50f25, 0x3fe2ad3f, 0xbf6df033, 0x3fbf9e65, 0xbfa9c26a, 0x3f916cb4, 0xbfd2af10, 0x3f358ecb, 0xbfef5d39, 0x3e76dbf8, 0xbffe222b, 0xbe76dbf4, 0xbffe222b, 0xbf358ebb, 0xbfef5d3b, 0xbf916cae, 0xbfd2af14, 0xbfbf9e68, 0xbfa9c267, 0xbfe2ad3f, 0xbf6df033, 0xbff88fa4, 0xbef50f37, 0xc0000000, 0xb43bbd2e, 0xbff88fa5, 0x3ef50f2b, 0xbfe2ad40, 0x3f6df02e, 0xbfbf9e67, 0x3fa9c268, 0xbf916cb3, 0x3fd2af10, 0xbf358ec4, 0x3fef5d3a, 0xbe76dbfc, 0x3ffe222b, 0x3e76dbf0, 0x3ffe222b, 0x3f358ec1, 0x3fef5d3a, 0x3f916cb1, 0x3fd2af12, 0x3fbf9e65, 0x3fa9c26a, 0x3fe2ad3f, 0x3f6df032, 0x3ff88fa4, 0x3ef50f36, 0x40000000, 0x00000000]);
        assert_eq!(result[0].curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), [0x00000000, 0x3d1d89dd, 0x3d9d89dd, 0x3dec4ecc, 0x3e1d89d9, 0x3e44ec50, 0x3e6c4ec7, 0x3e89d8a0, 0x3e9d89d9, 0x3eb13b15, 0x3ec4ec51, 0x3ed89d8b, 0x3eec4ec5, 0x3f000000, 0x3f09d89e, 0x3f13b13b, 0x3f1d89d8, 0x3f276275, 0x3f313b13, 0x3f3b13b0, 0x3f44ec4e, 0x3f4ec4eb, 0x3f589d8a, 0x3f627627, 0x3f6c4ec5, 0x3f762762, 0x3f800000]);
        assert_eq!(result[0].curve.length.to_bits(), 0x414892b2);
        assert_eq!(result[0].interval.map(|v| v.get().to_bits()), [0x00000000, 0x3f79dfa1]);
        assert!(!result[0].flag);

        let mut world = World::new();
        world.insert_resource(public);
        world.insert_resource(intersections);
        let combined = world.run_system_once(move |public: Res<PublicWalls>, intersections: Res<country_core::resources::shape_intersections::ShapeIntersections>, roofs: Query<&Roof>| {
            let cache = OverhangSupportFinderCache { roofs: vec![], walls: vec![source, target] };
            find_overhang_support_segments(target, public.get(target).unwrap(), &public, &intersections, &cache, &roofs, false, 0.0)
        }).unwrap();
        assert_eq!(combined.len(), 2);
        assert_eq!([combined[0].flag,combined[1].flag], [false,true]);
        assert!(combined.iter().all(|segment| segment.wall == source));
    }
    #[test]
    fn gable_roof_branch_includes_closing_fourth_corner() {
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let target = inner.new_wall(WallState::new(curve, 8.0, 0, 0, false));
        inner.update_elevation(target, 4.0);
        inner.update_wall_state_enclosure(target, ShapeParameters::from_shape(Shape::Rectangle(utils::geometry::Rectangle2d::new(
            glam::Vec2::ZERO, glam::Vec2::splat(10.0), glam::Vec2::X))).to_snapshot());
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut bytes = [0u8;88];
        bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
        for (offset, value) in [(12,1.0f32),(16,0.0),(20,0.0),(24,0.0),(28,4.0),(32,2.0),(48,1.0),(52,1.0),(76,4.0)] {
            bytes[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        bytes[60] = 1;
        let mut world = World::new();
        let entity = world.spawn(Roof::from_observed(bytes)).id();
        let roof_wall = country_core::resources::walls::WallId(987);
        world.insert_resource(public);
        let result = world.run_system_once(move |public: Res<PublicWalls>, roofs: Query<&Roof>| {
            let cache = OverhangSupportFinderCache { roofs: vec![(roof_wall,entity)], walls: vec![] };
            find_overhang_support_segments(target, public.get(target).unwrap(), &public,
                &country_core::resources::shape_intersections::ShapeIntersections::default(), &cache, &roofs, false, 0.0)
        }).unwrap();
        // 第二边含闭合顶点4，超出线段的末端按原环绕到0并被逆序门控拒绝。
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].wall, roof_wall);
        // 原 finder 同屋顶样本的完整输出位型。
        assert_eq!(result[0].curve.points.iter().flat_map(|point| point.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), [0xc0000000, 0x3f800000, 0x40000000, 0x3f800000]);
        assert_eq!(result[0].curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), [0x00000000, 0x3f800000]);
        assert_eq!(result[0].curve.length.to_bits(), 0x40800000);
        assert_eq!(result[0].interval.map(|v| v.get().to_bits()), [0x3e2aaaab, 0x3f000000]);
        assert!(!result[0].flag);

    }
    #[test]
    fn contained_circle_and_rectangle_emit_full_ring_and_four_edges() {
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let target = inner.new_wall(WallState::new(curve.clone(), 10.0, 0, 0, false));
        let circular = inner.new_wall(WallState::new(curve.clone(), 10.0, 0, 0, false));
        let rectangular = inner.new_wall(WallState::new(curve, 10.0, 0, 0, false));
        for (id, elevation) in [(target,4.0),(circular,1.0),(rectangular,1.0)] { inner.update_elevation(id,elevation); }
        for (id, shape) in [
            (target, Shape::Rectangle(utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(10.0),glam::Vec2::X))),
            (circular, Shape::Circle(utils::geometry::Circle2d { center:glam::Vec2::ZERO,radius:2.0,rotation:0.0 })),
            (rectangular, Shape::Rectangle(utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(2.0),glam::Vec2::X))),
        ] { inner.update_wall_state_enclosure(id,ShapeParameters::from_shape(shape).to_snapshot()); }
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let cache = OverhangSupportFinderCache { roofs: vec![], walls: vec![target,circular,rectangular] };
        let pieces = segments_from_contained_walls(target,public.get(target).unwrap(),&public,&cache,false);
        assert_eq!(pieces.len(),5);
        assert_eq!(pieces[0].wall,circular);
        // 原 finder 完整圆环段；CRT 三角函数及 fmod 使用 macOS 桥接。
        let expected_points: &[u32] = &[0x40000000, 0x34bbbd2e, 0x3ff88fa5, 0xbef50f25, 0x3fe2ad3f, 0xbf6df033, 0x3fbf9e65, 0xbfa9c26a, 0x3f916cb4, 0xbfd2af10, 0x3f358ecb, 0xbfef5d39, 0x3e76dbf8, 0xbffe222b, 0xbe76dbf4, 0xbffe222b, 0xbf358ebb, 0xbfef5d3b, 0xbf916cae, 0xbfd2af14, 0xbfbf9e68, 0xbfa9c267, 0xbfe2ad3f, 0xbf6df033, 0xbff88fa4, 0xbef50f37, 0xc0000000, 0xb43bbd2e, 0xbff88fa5, 0x3ef50f2b, 0xbfe2ad40, 0x3f6df02e, 0xbfbf9e67, 0x3fa9c268, 0xbf916cb3, 0x3fd2af10, 0xbf358ec4, 0x3fef5d3a, 0xbe76dbfc, 0x3ffe222b, 0x3e76dbf0, 0x3ffe222b, 0x3f358ec1, 0x3fef5d3a, 0x3f916cb1, 0x3fd2af12, 0x3fbf9e65, 0x3fa9c26a, 0x3fe2ad3f, 0x3f6df032, 0x3ff88fa4, 0x3ef50f36, 0x40000000, 0x00000000];
        let expected_u: &[u32] = &[0x00000000, 0x3d1d89dd, 0x3d9d89dd, 0x3dec4ecc, 0x3e1d89d9, 0x3e44ec50, 0x3e6c4ec7, 0x3e89d8a0, 0x3e9d89d9, 0x3eb13b15, 0x3ec4ec51, 0x3ed89d8b, 0x3eec4ec5, 0x3f000000, 0x3f09d89e, 0x3f13b13b, 0x3f1d89d8, 0x3f276275, 0x3f313b13, 0x3f3b13b0, 0x3f44ec4e, 0x3f4ec4eb, 0x3f589d8a, 0x3f627627, 0x3f6c4ec5, 0x3f762762, 0x3f800000];
        assert_eq!(pieces[0].curve.points.iter().flat_map(|point| point.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), expected_points);
        assert_eq!(pieces[0].curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), expected_u);
        assert_eq!(pieces[0].curve.length.to_bits(), 0x414892b2);
        assert_eq!(pieces[0].interval.map(|v| v.get().to_bits()), [0x00000000, 0x3f79e371]);

        assert!(pieces.iter().all(|piece| piece.flag));
        assert_eq!(pieces.iter().filter(|piece| piece.wall==rectangular).count(),4);
        // 原 finder 返回的四个矩形段，逐位比较输出及顺序。
        let expected: &[(&[u32], &[u32], u32, [u32; 2])] = &[
            (&[0xbf800000, 0xbf800000, 0xbf800000, 0x3f800000][..], &[0x00000000, 0x3f800000][..], 0x40000000, [0x00000000, 0x3e800000]),
            (&[0xbf800000, 0x3f800000, 0x3f800000, 0x3f800000][..], &[0x00000000, 0x3f800000][..], 0x40000000, [0x3e800000, 0x3f000000]),
            (&[0x3f800000, 0x3f800000, 0x3f800000, 0xbf800000][..], &[0x00000000, 0x3f800000][..], 0x40000000, [0x3f000000, 0x3f400000]),
            (&[0x3f800000, 0xbf800000, 0xbf800000, 0xbf800000][..], &[0x00000000, 0x3f800000][..], 0x40000000, [0x3f400000, 0x00000000]),
        ];
        for (piece, &(points, parameters, length, interval)) in pieces.iter().filter(|piece| piece.wall == rectangular).zip(expected) {
            assert_eq!(piece.curve.points.iter().flat_map(|point| point.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), points);
            assert_eq!(piece.curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), parameters);
            assert_eq!(piece.curve.length.to_bits(), length);
            assert_eq!(piece.interval.map(|v| v.get().to_bits()), interval);
            assert!(piece.flag);
        }
        // 同一原几何打开收缩，核对四段输出及浮点位型。
        let trimmed = segments_from_contained_walls(target, public.get(target).unwrap(), &public, &cache, true);
        let expected_trimmed: &[(&[u32], &[u32], u32, [u32; 2])] = &[
            (&[0xbf800000, 0xbf333334, 0xbf800000, 0x3f333334][..], &[0x00000000, 0x3f800000][..], 0x3fb33334, [0x3d19999a, 0x3e59999a]),
            (&[0xbf333334, 0x3f800000, 0x3f333334, 0x3f800000][..], &[0x00000000, 0x3f800000][..], 0x3fb33334, [0x3e933333, 0x3eeccccd]),
            (&[0x3f800000, 0x3f333330, 0x3f800000, 0xbf333330][..], &[0x00000000, 0x3f800000][..], 0x3fb33330, [0x3f09999a, 0x3f366666]),
            (&[0x3f333330, 0xbf800000, 0xbf333330, 0xbf800000][..], &[0x00000000, 0x3f800000][..], 0x3fb33330, [0x3f49999a, 0x3f766666]),
        ];
        let rect_trimmed: Vec<_> = trimmed.iter().filter(|piece| piece.wall == rectangular).collect();
        assert_eq!(rect_trimmed.len(), expected_trimmed.len());
        for (piece, &(points, parameters, length, interval)) in rect_trimmed.iter().zip(expected_trimmed) {
            assert_eq!(piece.curve.points.iter().flat_map(|point| point.iter().map(|v| v.to_bits())).collect::<Vec<_>>(), points);
            assert_eq!(piece.curve.points_u.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), parameters);
            assert_eq!(piece.curve.length.to_bits(), length);
            assert_eq!(piece.interval.map(|v| v.get().to_bits()), interval);
            assert!(piece.flag);
        }

    }
    #[test]
    #[should_panic(expected = "悬挑缓存中的墙必须仍然存在")]
    fn stale_wall_cache_fails_safely() {
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let target = inner.new_wall(WallState::new(curve, 10.0, 0, 0, false));
        inner.update_wall_state_enclosure(target, ShapeParameters::from_shape(Shape::Circle(
            utils::geometry::Circle2d { center: glam::Vec2::ZERO, radius: 3.0, rotation: 0.0 },
        )).to_snapshot());
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let cache = OverhangSupportFinderCache { roofs: vec![], walls: vec![country_core::resources::walls::WallId(u64::MAX)] };
        segments_from_contained_walls(target, public.get(target).unwrap(), &public, &cache, false);
    }
    #[test]
    fn cache_collects_live_roofs_and_premade_walls_independently() {
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let first = inner.new_wall(WallState::new(curve.clone(), 4.0, 0, 0, false));
        let second = inner.new_wall(WallState::new(curve.clone(), 4.0, 0, 0, false));
        let third = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        inner.update_wall_state_enclosure(first, ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: glam::Vec2::ZERO, radius: 3.0, rotation: 0.0,
        })).to_snapshot());
        inner.mark_as_non_enclosed(second);
        inner.mark_as_non_enclosed(third);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut world = World::new();
        let roof = world.spawn(Roof::from_observed([0; 88])).id();
        let without_roof = world.spawn_empty().id();
        let stale = world.spawn_empty().id();
        world.despawn(stale);
        public.associate_roof(first, stale);
        public.associate_roof(second, roof);
        public.associate_roof(third, without_roof);
        world.insert_resource(public);
        let cache = world.run_system_once(|public: Res<PublicWalls>, roofs: Query<&Roof>| OverhangSupportFinderCache::new(&public, &roofs)).unwrap();
        assert_eq!(cache.walls, vec![first]);
        assert_eq!(cache.roofs, vec![(second, roof)]);
    }
}
