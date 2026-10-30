//! 半木构纹理生成（utils/half_timber.rs）。
//!
//! 原 B 源：`crates/systems/wall-constructor/src/utils/half_timber.rs`。
//!
//! `half_timbering`（0x141218e30）已按原 B 恢复主流程，矩形与圆形若干完整
//! 原输入已逐字节对照；更多组合、失败与正式宿主链仍待验证。
//!
//! 窗工具三件（0x1412218a0 / 0x141221da0 / 0x141221f70）由真实墙装饰表、
//! subtype 和 bbox 查询接入主流程；投影输入仍供有界叶函数测试使用。
//! 窗口组合及正式宿主调用链尚需继续对照原执行。
//!
//! 常量（rne-read 直读 .rdata）：
//! - 0x142c1c880 = 0.038、0x142c1c884 = −0.038、0x142c1c888 = −0.14、
//!   0x142a80d40 = 0.14；
//! - 0x1429cd068 = 0.2、0x1429a0764 = 0.46、0x1429df16c = −0.2、
//!   0x142925970 = 0.4、0x142c1c88c = 0.093。
use glam::Vec2;
use glam::Vec3;
use utils::geometry::aabb::Aabb2;

/// 原半木生成器的真实业务输入；机器调用中的 slice 双字合并为借用。
pub struct HalfTimberInput<'a> {
    pub wall: &'a country_core::resources::walls::public_walls::PublicWallState,
    pub wall_id: country_core::resources::walls::WallId,
    pub decorators: &'a country_core::resources::walls::decorator_storage::DecoratorStorage,
    pub derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'a>,
    pub terrain: &'a country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo>,
    pub segment_heights: &'a [f32],
    pub path_heights: &'a [f32],
    pub roof: &'a country_core::resources::roofs::roof_shape::Roof,
    pub holes: &'a [country_core::resources::walls::wall_holes::FinalizedWallHole],
    pub demo_mode: bool,
}

/// B 0x141218e30：先经过 demo/原墙形门，再验证原缓冲及槽、锁定并清空。
/// 已有有界原输入逐字节对照，完整分支和正式宿主接线仍待验收。
pub fn half_timbering(
    input: HalfTimberInput<'_>,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &crate::gpu_output::ConstructorGpuOutput,
    instances: &mut Vec<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,
) {
    if input.demo_mode || input.wall.try_get_wall_shape().is_none() { return; }
    let gable_sides = input.roof.gable_sides();
    let flat = input.roof.is_flat();
    let storage = library.get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
        .expect("原半木木板缓冲区缺失");
    let mut writer = output.half_timber_append_writer(storage, input.wall_id.0);
    writer.clear();
    // 平顶原闭包只复制 hole kind 1、3、4 的 AABB，保留输入顺序。
    let flat_holes: Vec<Aabb2> = if flat {
        input.holes.iter().filter(|hole| matches!(hole.kind, 1 | 3 | 4))
            .map(|hole| Aabb2 { min: Vec2::new(hole.aabb[0], hole.aabb[1]),
                max: Vec2::new(hole.aabb[2], hole.aabb[3]) }).collect()
    } else { Vec::new() };
    // 原 141219089–343 在获取并清空 writer 后构造各层高度。
    let maximum = if gable_sides.is_some() { input.wall.max_y_gable() } else { input.wall.max_y() };
    let mut levels = country_core::resources::walls::split_height_into_skirts(maximum);
    if gable_sides.is_some() {
        levels.push(input.wall.max_y());
        levels.sort_by(|a, b| a.partial_cmp(b).expect("原半木裙层高度不可比较"));
    }
    let floor = input.wall.authority.elevation();
    levels.retain(|height| *height >= floor + 1.0);
    let top = if flat {
        (input.wall.max_y() + f32::from_bits(0xbeeb_851f)) + f32::from_bits(0xbe4c_cccd)
    } else { maximum };
    levels.push(top);
    levels.insert(0, floor);
    let areas = half_timber_areas(input.wall, &levels, gable_sides);
    // 原为每层建立独立状态及扣除区间，再将高度轮廓投影成二维曲线。
    let mut level_flags = vec![0u8; levels.len()];
    let mut spans: Vec<Vec<HalfTimberHeightSpan>> = vec![Vec::new(); levels.len()];
    let outline: Vec<Vec2> = input.wall.height_outline().iter()
        .map(|point| Vec2::new(point[0], point[2])).collect();
    let projected_curve = utils::curve::Curve2::try_new(outline, false)
        .expect("原半木高度轮廓无法构造曲线");
    half_timber_height_spans(&projected_curve, &levels,
        input.segment_heights, input.path_heights, &mut level_flags, &mut spans);
    // 原先为每个区域查询真实墙装饰表，再开始逐区域分割。
    let windows: Vec<_> = areas.iter().map(|area| overlapping_windows_from_storage(
        *area, input.wall.wall_space_length(), input.wall_id, input.decorators, &input.derived,
    )).collect();
    let mut write_failed = false;
    let mut circle_positions = std::collections::HashMap::new();
    for (area_index, (area, area_windows)) in areas.iter().zip(&windows).enumerate() {
        let mut splits = half_timber_initial_splits(*area, input.wall.is_circle().is_some());
        half_timber_merge_windows(&mut splits, area_windows, input.wall.wall_space_length());
        let mut positions = half_timber_split_positions(*area, &splits, area_windows, &flat_holes,
            input.wall.wall_space_length(), input.wall_id, input.decorators, &input.derived);
        if gable_sides.is_some() { half_timber_filter_gable_positions(&mut positions, *area, input.wall.max_y(), input.roof); }
        let level = levels.iter().position(|value| *value == area.min.y).unwrap_or(levels.len());
        let mut point_flags = vec![0u8; positions.len()];
        for (index, &position) in positions.iter().enumerate() {
            if half_timber_span_contains(&spans[level], position) {
                point_flags[index] = 1;
                continue;
            }
            // 原函数在继续处理下一点前立即生成此处竖梁。
            half_timber_vertical_plank(input.wall, input.wall_id, *area, position,
                index as u32, &mut writer, &mut write_failed);
        }
        // 原 14121b9ee：末点在逐点竖梁后追加，且默认不标记覆盖。
        if gable_sides.is_none() || !(input.wall.max_y() < area.max.y) {
            positions.push(area.max.x);
            point_flags.push(0);
        }
        // 原 14121aad0：只处理两端均未被高度区间覆盖的相邻分割点，
        // 平顶孔洞使用左闭右开中点门；各候选仍须按原顺序执行。
        for (index, pair) in positions.windows(2).enumerate() {
            if point_flags[index] != 0 || point_flags[index + 1] != 0 { continue; }
            let middle = pair[1] * 0.5 + pair[0] * 0.5;
            if flat_holes.iter().any(|hole| !(hole.max.x <= middle || middle < hole.min.x)) { continue; }
            let width = (pair[1] - pair[0]).abs();
            let mut generated = false;
            if width > 0.5 && width < 2.0 {
                for window in area_windows {
                    let bounds = Aabb2 { min: Vec2::new(window.aabb[0], window.aabb[1]),
                        max: Vec2::new(window.aabb[2], window.aabb[3]) };
                    if !((width - bounds.width()).abs() < 0.05) { continue; }
                    let center = bounds.center().x;
                    if !(pair[0] <= center && center <= pair[1]) { continue; }
                    if !check_real_windows(Vec2::new(middle, area.min.y - 0.1),
                        input.wall.wall_space_length(), input.wall_id, input.decorators, &input.derived) {
                        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
                        let shape = if let Some(circle) = input.wall.is_circle() {
                            Shape::Circle(circle)
                        } else { Shape::Rectangle(input.wall.is_rectangle().expect("原半木墙形缺失")) };
                        add_underwindow_pattern(window, area, pair[0], pair[1], &mut generated,
                            input.wall, &ShapeParameters::from_shape(shape), &mut writer, &mut write_failed,
                            input.wall_id, index as u32, input.wall.max_y());
                    }
                    break;
                }
            }
            if generated { continue; }
            let Some(rng) = half_timber_pair_rng(input.wall_id, area_index, pair[0], pair[1],
                area.height()) else { continue; };
            // 原 141762b70 的区间相交包含两端，且无序比较不相交。
            if spans[level].iter().any(|span| span.end >= pair[0] && pair[1] >= span.start) { continue; }
            let direction_sample = rng.f32();
            let interval = Aabb2 { min: Vec2::new(pair[0], area.min.y), max: Vec2::new(pair[1], area.max.y) };
            if area_windows.iter().any(|window| interval.intersects(Aabb2 {
                min: Vec2::new(window.aabb[0] + 0.05, window.aabb[1]),
                max: Vec2::new(window.aabb[2] - 0.05, window.aabb[3]),
            })) { continue; }
            half_timber_diagonal_plank(input.wall, input.wall_id, *area, pair[0], pair[1],
                index as u32, direction_sample, &mut writer, &mut write_failed);
        }
        // 原 14121b9f0：仅圆墙保留本区域分割位置，供后续曲面流程使用。
        if input.wall.is_circle().is_some() { circle_positions.insert(area_index, positions); }
    }
    // 原圆墙先给每个非空位置序列补零，随后再进入各区域的曲面构造。
    for positions in circle_positions.values_mut() {
        let first = positions.first().expect("原半木圆墙位置序列为空");
        if *first != 0.0 { positions.insert(0, 0.0); }
    }
    // 原 closure11 以原区域顺序展开上边界；地面区域额外展开下边界。
    let boundaries = half_timber_area_boundaries(&areas, floor);
    for (area_index, height, center) in boundaries {
        let rng = utils::rng::rng_from_seed_u64(input.wall_id.0 as u32 as u64);
        let mut enabled = half_timber_boundary_enabled(flat, input.wall.max_y(), height);
        if let Some(sides) = gable_sides {
            if !enabled {
                if let Some(rectangle) = input.wall.is_rectangle() {
                    let edge = rectangle.get_edge_id_at_u(center / input.wall.wall_space_length());
                    enabled = edge == usize::from(sides[0]) || edge == usize::from(sides[1]);
                }
            }
            let adjusted = if height == input.wall.max_y() { height + f32::from_bits(0xbe23_d70b) } else { height };
            if !enabled || adjusted == input.wall.max_y_gable() { continue; }
        }
        if !enabled { continue; }
        if let Some(rectangle) = input.wall.is_rectangle() {
            let length = input.wall.wall_space_length();
            let edge = rectangle.get_edge_id_at_u(center / length);
            let mut left = rectangle.get_point_u(edge) * length;
            let mut right = rectangle.get_point_u(edge + 1) * length;
            let adjusted_height = if height == input.wall.max_y() {
                height + f32::from_bits(0xbe23_d70b)
            } else { height };
            if !half_timber_clip_high_boundary(&mut left, &mut right, adjusted_height, input.wall.max_y(), input.roof) { continue; }
            let last_level = *levels.last().expect("原半木裙层为空");
            let last = flat && height == last_level;
            let level = levels.iter().position(|value| *value == height).unwrap_or(levels.len());
            let clipping = half_timber_boundary_clipping(left, right, length, adjusted_height,
                last_level, &flat_holes, &windows[area_index], &spans[level]);
            let segment_height = if last && input.roof.is_flat_w_no_rim() {
                adjusted_height - 0.2
            } else { adjusted_height };
            let mut fine_points = Vec::new();
            let mut segment_count = 0u32;
            for segment in clipping.result() {
                if segment.end - segment.start < 0.1 { continue; }
                half_timber_rectangle_boundary_plank(input.wall, input.wall_id, segment.start,
                    segment.end, segment_height, last, segment_count, &mut writer, &mut write_failed);
                segment_count = segment_count.wrapping_add(1);
                if last && input.roof.observed()[0x3d] == 0 {
                    let (p, q, normal) = half_timber_rectangle_corner_adjusted_endpoints(
                        input.wall, rectangle, segment.start, segment.end, segment_height);
                    half_timber_rectangle_top_plank(input.wall_id, p, q, normal,
                        segment_count, &mut writer, &mut write_failed);
                    fine_points.extend(half_timber_rectangle_fine_points(segment.start, segment.end, p, q, normal, &rng));
                }
            }
            half_timber_write_fine_points(&mut fine_points, input.wall_id, segment_count, segment_height,
                &rng, &mut writer, &mut write_failed);
        } else {
            let length = input.wall.wall_space_length();
            let positions = circle_positions.get(&area_index).expect("原半木圆墙区域位置缺失");
            let mut coordinates: Vec<f32> = positions.iter().map(|position| {
                let u = *position / length;
                assert!((0.0..=1.0).contains(&u), "原半木圆墙曲线坐标越界");
                u
            }).collect();
            coordinates.sort_unstable_by(|a, b| a.partial_cmp(b).expect("原半木圆墙曲线坐标不可比较"));
            coordinates.dedup_by(|a, b| *a == *b);
            let level = levels.iter().position(|value| *value == height).unwrap_or(levels.len());
            let adjusted_height = if height == input.wall.max_y() {
                height + f32::from_bits(0xbe23_d70b)
            } else { height };
            let mut fine_points = Vec::new();
            let mut segment_count = 0u32;
            for pair in coordinates.windows(2) {
                let mut left = pair[0] * length;
                let mut right = pair[1] * length;
                if !half_timber_clip_high_boundary(&mut left, &mut right, adjusted_height, input.wall.max_y(), input.roof) { continue; }
                let clipping = half_timber_boundary_clipping(left, right, length, adjusted_height,
                    *levels.last().expect("原半木裙层为空"), &flat_holes, &windows[area_index], &spans[level]);
                let last = flat && height == *levels.last().unwrap();
                let segment_height = if last && input.roof.is_flat_w_no_rim() {
                    adjusted_height - 0.2
                } else { adjusted_height };
                for segment in clipping.result() {
                    if segment.end - segment.start < 0.1 { continue; }
                    let circle = input.wall.is_circle().expect("原半木圆墙形缺失");
                    half_timber_circle_boundary(circle, length, segment.start, segment.end,
                        segment_height, last, area_index,
                        input.roof.observed()[0x3d] == 0, &flat_holes, instances);
                    segment_count = segment_count.wrapping_add(1);
                    if last && input.roof.observed()[0x3d] == 0 {
                        fine_points.extend(half_timber_circle_fine_points(input.wall, circle,
                            segment.start, segment.end, segment_height, &flat_holes));
                    }
                }
            }
            half_timber_write_fine_points(&mut fine_points, input.wall_id, segment_count, adjusted_height,
                &rng, &mut writer, &mut write_failed);
        }
    }
    if let Some(rectangle) = input.wall.is_rectangle() {
        half_timber_rectangle_corners(&input, rectangle, &flat_holes, &mut writer, &mut write_failed);
    }
}

/// 原 14121c438–c4f3：高于墙顶的边界须取对应矩形屋顶截面。
fn half_timber_clip_high_boundary(left: &mut f32, right: &mut f32, height: f32, maximum: f32,
    roof: &country_core::resources::roofs::roof_shape::Roof) -> bool {
    if !(maximum < height) { return true; }
    let t = utils::scalar::inverse_lerp(roof.floor_y(), roof.floor_y() + roof.height(), height);
    let t = if t.is_nan() { t } else { t.max(0.0).min(1.0) };
    let dimensions = match roof.shape_at(t) {
        country_core::resources::roofs::geometry::surface::RoofSection::Rectangle { dimensions, .. } => dimensions,
        _ => panic!("原半木山墙要求矩形屋顶截面"),
    };
    let axis = usize::from((!roof.observed()[0x3c]) & 1);
    let width = dimensions[axis] - 0.3;
    if width < 0.5 { return false; }
    let center = *right * 0.5 + *left * 0.5;
    *left = center - width * 0.5;
    *right = center + width * 0.5;
    true
}

/// 原 14121a623–a756：屋檐以上分割点按对应高度的真实屋顶截面过滤。
fn half_timber_filter_gable_positions(positions: &mut Vec<f32>, area: Aabb2, maximum: f32,
    roof: &country_core::resources::roofs::roof_shape::Roof) {
    if !(maximum < area.max.y) { return; }
    let t = utils::scalar::inverse_lerp(roof.floor_y(), roof.floor_y() + roof.height(), area.max.y);
    if t >= 1.0 { positions.clear(); return; }
    let t = if t.is_nan() { t } else { t.max(0.0).min(1.0) };
    let dimensions = match roof.shape_at(t) {
        country_core::resources::roofs::geometry::surface::RoofSection::Rectangle { dimensions, .. } => dimensions,
        _ => panic!("原半木山墙要求矩形屋顶截面"),
    };
    let axis = usize::from((!roof.observed()[0x3c]) & 1);
    let width = dimensions[axis] - 0.3;
    let width = if width > 0.0 { width } else { 0.0 };
    let center = area.max.x * 0.5 + area.min.x * 0.5;
    let half = width * 0.5;
    positions.retain(|position| center - half <= *position && *position <= center + half);
}

/// 原圆墙顶饰按角弧长度决定采样数，不在采样阶段消耗随机流。
fn half_timber_circle_fine_points(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    circle: utils::geometry::Circle2d, left: f32, right: f32, height: f32, holes: &[Aabb2],
) -> Vec<(f32, Vec3, Vec2)> {
    use utils::wall_space::WallSpaceCoord;
    let length = wall.wall_space_length();
    let scale = std::f32::consts::TAU / length;
    let midpoint = ((right + left) * 0.5) * scale
        - utils::circular_f32::CircularF32::new(circle.rotation).as_f32();
    let half = ((right - left) * 0.5) * scale;
    let (start_extension, end_extension, _) = half_timber_circle_extensions(length, left, right, holes);
    let start = (midpoint - half) - start_extension * scale;
    let end = (midpoint + half) + end_extension * scale;
    let count = ((((end - start).abs() * circle.radius) / 0.4).ceil() as u64).max(2) as usize;
    let denominator = count as f32 - 1.0;
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let t = i as f32 / denominator;
        let u = right * t + (1.0 - t) * left;
        // 14121df53 直接构造墙坐标，末端 length 仍指向闭合曲线末点。
        let point = WallSpaceCoord::new_unchecked(u, height).to_world_space(wall.simplified_curve_2d());
        let normal_u = u / circle.perimeter();
        assert!(!normal_u.is_nan(), "原半木圆墙法线坐标为 NaN");
        let normal_u = normal_u.max(0.0).min(1.0);
        let normal = circle.get_normal_at_u(normal_u);
        result.push((u, point, normal));
    }
    result
}

/// B 14121ce89 等：无洞窗的平顶圆墙边界曲面，后续细梁仍另行生成。
fn half_timber_circle_boundary(
    circle: utils::geometry::Circle2d, wall_length: f32, left: f32, right: f32,
    height: f32, last_level: bool, area_index: usize, initial: bool, holes: &[Aabb2],
    instances: &mut Vec<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,
) {
    use country_core::render::nani::instance_data::CurvedHalfTimberInstanceData;
    let scale = std::f32::consts::TAU / wall_length;
    let midpoint = ((right + left) * 0.5) * scale
        - utils::circular_f32::CircularF32::new(circle.rotation).as_f32();
    let half = ((right - left) * 0.5) * scale;
    let (start_extension, end_extension, flag) = half_timber_circle_extensions(wall_length, left, right, holes);
    let (start, end) = ((midpoint - half) - start_extension * scale, (midpoint + half) + end_extension * scale);
    let position = Vec3::new(circle.center.x, height, circle.center.y);
    if last_level {
        instances.push(CurvedHalfTimberInstanceData::new_curved_xz(position, circle.radius,
            0.68, f32::from_bits(0x3ea3_d70b), start, end, flag));
        if initial {
            let position = Vec3::new(circle.center.x, height + f32::from_bits(0x3f5c_28f6), circle.center.y);
            instances.push(CurvedHalfTimberInstanceData::new_curved_xz(position, circle.radius + 0.2,
                f32::from_bits(0x3df5_c290), f32::from_bits(0x3df5_c290), start, end, flag));
        }
    } else {
        let radius = (circle.radius + 0.28) + if area_index == 0 { 0.05 } else { 0.0 };
        instances.push(CurvedHalfTimberInstanceData::new_curved_xy(position, radius,
            0.1, f32::from_bits(0x3ea3_d70b), start, end));
    }
}

/// 原 14121d39a–d6f1：孔边与两端的双向环距小于 0.05 时各延长 0.1。
fn half_timber_circle_extensions(length: f32, left: f32, right: f32, holes: &[Aabb2]) -> (f32, f32, u8) {
    let wrap = |x: f32| {
        let x = x / length;
        assert!(!x.is_nan(), "原半木圆墙孔坐标为 NaN");
        let x = x % 1.0;
        if x < 0.0 { x + 1.0 } else { x }
    };
    let left = wrap(left); let right = wrap(right);
    let mut start = false; let mut end = false;
    for hole in holes {
        for point in [wrap(hole.min.x), wrap(hole.max.x)] {
            let near = |target: f32| {
                (if target < point { target + 1.0 } else { target }) - point < 0.05
                    || (if point < target { point + 1.0 } else { point }) - target < 0.05
            };
            start |= near(left);
            end |= near(right);
        }
    }
    (if start { 0.1 } else { 0.0 }, if end { 0.1 } else { 0.0 }, u8::from(start) | (u8::from(end) << 1))
}

/// 原 14121e7xx–f420 矩形角柱及 subtype3 装饰扣除与辅助梁。
fn half_timber_rectangle_corners(
    input: &HalfTimberInput<'_>, rectangle: utils::geometry::Rectangle2d, holes: &[Aabb2],
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    use utils::curve::{CurveCoordSemanticThresholded3D, CurveInternalCoord, curve_slice::CurveSlice};
    let points: Vec<Vec3> = input.wall.height_outline().iter().copied().map(Vec3::from_array).collect();
    assert!(points.len() >= 2, "原半木角柱曲线至少两点");
    let start = CurveInternalCoord::new_for_point_count(0, 0.0, points.len());
    for corner in 0..4 {
        let position = rectangle.get_point(corner) + rectangle.get_corner_normal(corner) * 0.2;
        let terrain = input.terrain.sample_at_world_xz(position.x, position.y);
        let floor = input.wall.authority.elevation();
        let floor = if terrain.is_nan() { floor } else if floor > terrain { floor } else { terrain };
        let distance = rectangle.get_point_u(corner) * rectangle.perimeter();
        let mut slice = CurveSlice::<Vec3, _, CurveCoordSemanticThresholded3D>::from_coord_range(
            points.as_slice(), start, start).remap_2d();
        slice.extend_right(distance);
        let coord = slice.end();
        let i = coord.segment;
        let a = input.segment_heights[i];
        let b = input.segment_heights[i + 1];
        let segment_height = (b - a) * coord.t + a;
        let a = points[i].y;
        let b = points[i + 1].y;
        let path_height = (b - a) * coord.t + a;
        let combined = path_height + segment_height;
        let bottom = if floor.is_nan() { combined } else if combined > floor { combined } else { floor };
        let mut top = if input.roof.is_flat() {
            (input.wall.max_y() + f32::from_bits(0xbeeb_851f)) + 0.1
        } else { input.wall.max_y() };
        if input.roof.is_flat_w_no_rim() { top += -0.2; }
        if input.roof.is_flat() && holes.iter().any(|hole| !(distance < hole.min.x || hole.max.x <= distance)) {
            top += -0.4;
        }
        let mut intervals = utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range(bottom, top);
        let mut decorations = Vec::new();
        if let Some(bucket) = input.decorators.get(DecoratorStorageAddr::Wall(input.wall_id)) {
            for (&id, info) in &bucket.decorators {
                if info.ty == b'8' {
                    let subtype = input.derived.subtype(country_core::resources::walls::decorator::DecoratorId(id));
                    if subtype.0 != 3 { continue; }
                    let bbox = input.derived.bbx_ls(subtype, info.rank);
                    let dst = info.dst_as_wall().dst_wall;
                    let center = Vec2::new(f32::from_bits(dst as u32), f32::from_bits((dst >> 32) as u32));
                    let bounds = Aabb2::from_center_dims(center, Vec2::new(bbox.footprint.size.x, bbox.top - bbox.bottom));
                    if bounds.contains(Vec2::new(distance, bounds.center().y)) {
                        decorations.push((bounds.min.y, bounds.max.y, bounds.width()));
                        intervals.subtract(utils::subtractive_1d_boolean::Interval {
                            start: bounds.min.y, end: bounds.max.y, upper_exclusive: false,
                        });
                    }
                }
            }
        }
        for (index, &(bottom, top, width)) in decorations.iter().enumerate() {
            for direction in rectangle.get_edge_dirs_from_corner_id(corner) {
                for y in [bottom, top] {
                    let translation = Vec3::new(position.x + direction.x * f32::from_bits(0xbe3a_5e35),
                        y + 0.0, position.y + direction.y * f32::from_bits(0xbe3a_5e35));
                    let rotation = utils::xz_rotation_bad(direction)
                        * glam::Quat::from_xyzw(f32::from_bits(0x3f35_04f3), 0.0, 0.0, f32::from_bits(0x3f35_04f3));
                    let transform = glam::Affine3A::from_scale_rotation_translation(
                        Vec3::new(0.4, width * 0.5 + 0.5, 0.2), rotation, translation);
                    let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
                        piece_seed: input.wall_id.0.wrapping_add(index as u64) as i32,
                        wall_id: input.wall_id.0 as u32, flags: 0 };
                    if writer.push(&record).is_err() { *write_failed = true; }
                }
            }
        }
        for (index, range) in intervals.result().iter().enumerate() {
            if !(0.1 < range.end - range.start) { continue; }
            let transform = glam::Affine3A::from_scale_rotation_translation(
                Vec3::new(0.4, (range.end - range.start).abs(), 0.4),
                utils::xz_rotation_bad(rectangle.axis), Vec3::new(position.x, range.start, position.y));
            let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
                piece_seed: input.wall_id.0.wrapping_add(index as u64) as i32,
                wall_id: input.wall_id.0 as u32, flags: 0 };
            if writer.push(&record).is_err() { *write_failed = true; }
        }
    }
}

/// 原排序后从左到右过滤相距小于 0.2 的相邻样点，再逐个抽样写细梁。
fn half_timber_write_fine_points(
    points: &mut [(f32, Vec3, Vec2)], wall_id: country_core::resources::walls::WallId,
    piece_index: u32, height: f32, rng: &fastrand::Rng,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("原半木细梁位置不可比较"));
    for index in 0..points.len() {
        let (_, p, normal) = points[index];
        if let Some((_, next, _)) = points.get(index + 1) {
            let d = p - *next;
            if ((d.z * d.z + d.x * d.x) + d.y * d.y).sqrt() < 0.2 { continue; }
        }
        let random = rng.f32();
        let lateral = random * 0.05 - (1.0 - random) * 0.05;
        let translation = Vec3::new(normal.x * 0.2 + ((-normal.y) * lateral + p.x),
            0.0f32 * 0.2 + (0.0 * lateral + height),
            normal.y * 0.2 + (normal.x * lateral + p.z));
        let transform = glam::Affine3A::from_scale_rotation_translation(
            Vec3::new(f32::from_bits(0x3df5_c290), f32::from_bits(0x3f5c_28f6), f32::from_bits(0x3df5_c290)),
            utils::xz_rotation_bad(normal), translation);
        let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
            piece_seed: wall_id.0.wrapping_add(piece_index as u64) as i32,
            wall_id: wall_id.0 as u32, flags: 0 };
        if writer.push(&record).is_err() { *write_failed = true; }
    }
}

/// 原顶层矩形边界靠近墙角时延伸对应世界端点。
fn half_timber_rectangle_corner_adjusted_endpoints(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: utils::geometry::Rectangle2d, left: f32, right: f32, height: f32,
) -> (Vec3, Vec3, Vec2) {
    use utils::wall_space::WallSpaceCoord;
    let curve = wall.simplified_curve_2d();
    let mut p = WallSpaceCoord::new_unchecked(left, height).to_world_space(curve);
    let mut q = WallSpaceCoord::new_unchecked(right, height).to_world_space(curve);
    let d = q - p;
    let reciprocal = 1.0 / ((d.z * d.z + d.y * d.y) + d.x * d.x).sqrt();
    let tangent = d * reciprocal;
    let step = tangent * f32::from_bits(0x3e10_d0c3);
    for corner in 0..4 {
        let x = rectangle.get_point_u(corner) * rectangle.perimeter();
        let min = x - 0.01;
        let max = x + 0.01;
        if !(left < min || max <= left) { p -= step; }
        else if min <= right && right < max { q += step; }
    }
    (p, q, Vec2::new(-tangent.z, tangent.x))
}

/// 原顶层矩形墙边装饰横梁，紧接主横梁写入。
fn half_timber_rectangle_top_plank(
    wall_id: country_core::resources::walls::WallId, p: Vec3, q: Vec3, normal: Vec2,
    index: u32,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    let delta = q - p;
    let distance = ((delta.z * delta.z + delta.y * delta.y) + delta.x * delta.x).sqrt();
    let translation = Vec3::new((p.x * 0.5 + q.x * 0.5) + normal.x * 0.2,
        (p.y * 0.5 + q.y * 0.5) + f32::from_bits(0x3f5c_28f6),
        (p.z * 0.5 + q.z * 0.5) + normal.y * 0.2);
    let transform = glam::Affine3A::from_scale_rotation_translation(
        Vec3::new(distance + 0.2, f32::from_bits(0x3df5_c290), f32::from_bits(0x3df5_c290)),
        utils::xz_rotation_bad(normal), translation);
    let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
        piece_seed: wall_id.0.wrapping_add(index as u64) as i32,
        wall_id: wall_id.0 as u32, flags: 0 };
    if writer.push(&record).is_err() { *write_failed = true; }
}

/// B 14121cf84–d27a：矩形边界的主横梁；后续细梁另按原流生成。
fn half_timber_rectangle_boundary_plank(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    wall_id: country_core::resources::walls::WallId, left: f32, right: f32,
    height: f32, last_level: bool, index: u32,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use utils::wall_space::WallSpaceCoord;
    let curve = wall.simplified_curve_2d();
    let p = WallSpaceCoord::new_unchecked(left, height).to_world_space(curve);
    let q = WallSpaceCoord::new_unchecked(right, height).to_world_space(curve);
    let delta = q - p;
    let reciprocal = 1.0 / ((delta.z * delta.z + delta.y * delta.y) + delta.x * delta.x).sqrt();
    let tangent = delta * reciprocal;
    let normal = Vec2::new(-tangent.z, tangent.x);
    let offset = if last_level { 0.05 } else { 0.2 };
    let translation = Vec3::new((p.x - tangent.x * 0.12) + normal.x * offset,
        height + f32::from_bits(0x3e23_d70b), (p.z - tangent.z * 0.12) + normal.y * offset);
    let rotation = utils::xz_rotation_bad(normal)
        * glam::Quat::from_xyzw(0.0, 0.0, -f32::from_bits(0x3f35_04f3), f32::from_bits(0x3f35_04f3));
    let distance = ((delta.z * delta.z + delta.y * delta.y) + delta.x * delta.x).sqrt();
    let depth = if last_level { f32::from_bits(0x3f11_eb86) } else { f32::from_bits(0x3ea3_d70b) };
    let transform = glam::Affine3A::from_scale_rotation_translation(
        Vec3::new(f32::from_bits(0x3ea3_d70b), distance + 0.2, depth), rotation, translation);
    let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
        piece_seed: wall_id.0.wrapping_add(index as u64) as i32,
        wall_id: wall_id.0 as u32, flags: if last_level { 4 } else { 0 } };
    if writer.push(&record).is_err() { *write_failed = true; }
}

/// 原边界先扣洞、窗口和跨周长镜像，再扣该层轮廓 span。
fn half_timber_boundary_clipping(left: f32, right: f32, length: f32, height: f32, last_level: f32,
    holes: &[Aabb2], windows: &[WindowAccessInfo], spans: &[HalfTimberHeightSpan])
    -> utils::subtractive_1d_boolean::SubtractiveBoolean1D {
    use utils::subtractive_1d_boolean::{Interval, SubtractiveBoolean1D};
    let mut clipping = SubtractiveBoolean1D::new_range(left, right);
    let mut cuts = Vec::new();
    for hole in holes {
        if height == last_level || !(height < hole.min.y || hole.max.y < height) { cuts.push(*hole); }
    }
    for window in windows {
        let bounds = Aabb2 { min: Vec2::new(window.aabb[0], window.aabb[1]),
            max: Vec2::new(window.aabb[2], window.aabb[3]) };
        let y = height + 0.2;
        if matches!(window.subtype, 6 | 9) {
            use country_core::resources::gothic_window_profile::{gothic_door_profile_curve, gothic_half_dormer_profile_curve};
            let mut points = if window.subtype == 6 { gothic_half_dormer_profile_curve(window.leaf) }
                else { gothic_door_profile_curve(window.leaf) };
            points.reverse();
            let first = points.first().expect("原哥特窗轮廓为空").y;
            let last = points.last().expect("原哥特窗轮廓为空").y;
            if !(y < bounds.min.y || bounds.max.y <= y) {
                for pair in points.windows(2) {
                    let a = utils::scalar::inverse_lerp(first, last, pair[0].y);
                    let b = utils::scalar::inverse_lerp(first, last, pair[1].y);
                    let low = bounds.min.y * (1.0 - a) + bounds.max.y * a;
                    let high = bounds.min.y * (1.0 - b) + bounds.max.y * b;
                    if y < low || high <= y { continue; }
                    let t = utils::scalar::inverse_lerp(low, high, y);
                    let mut half_width = pair[0].x * (1.0 - t) + pair[1].x * t;
                    if window.subtype == 9 && window.leaf == 1 { half_width += 0.14; }
                    if half_width > 0.05 {
                        let center = bounds.center().x;
                        cuts.push(Aabb2 { min: Vec2::new(center - half_width, bounds.min.y),
                            max: Vec2::new(center + half_width, bounds.max.y) });
                    }
                    break;
                }
            }
        } else if bounds.contains(Vec2::new(bounds.center().x, y)) { cuts.push(bounds); }
    }
    for cut in cuts {
        clipping.subtract(Interval { start: cut.min.x, end: cut.max.x, upper_exclusive: false });
        let center = cut.center().x;
        let half_width = cut.width() * 0.5;
        let shift = if !(center + half_width <= length) { Some(-length) }
            else if center < half_width { Some(length) } else { None };
        if let Some(shift) = shift {
            let shifted_center = center + shift;
            clipping.subtract(Interval { start: shifted_center - half_width, end: shifted_center + half_width, upper_exclusive: false });
        }
    }
    for span in spans {
        clipping.subtract(Interval { start: span.start, end: span.end, upper_exclusive: span.tag != 0 });
    }
    clipping
}

/// B 14121bd91–be8b：非山墙边界相对缓存墙顶的原比较门。
fn half_timber_boundary_enabled(flat: bool, maximum: f32, height: f32) -> bool {
    if flat { maximum != height }
    else { height < maximum - 0.2 || maximum + 0.2 <= height }
}

/// B 141fd16d0：每个区域先产出上边界，地面层再产出下边界。
fn half_timber_area_boundaries(areas: &[Aabb2], floor: f32) -> Vec<(usize, f32, f32)> {
    let mut result = Vec::new();
    for (index, area) in areas.iter().enumerate() {
        let center = area.center().x;
        result.push((index, area.max.y, center));
        if area.min.y == floor { result.push((index, area.min.y, center)); }
    }
    result
}

/// B 14121af3c–b5a2：一根区间斜梁；窗口相交由调用者处理。
fn half_timber_diagonal_plank(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    wall_id: country_core::resources::walls::WallId, area: Aabb2, left: f32, right: f32,
    index: u32, direction_sample: f32,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use utils::wall_space::WallSpaceCoord;
    let length = wall.wall_space_length();
    let wrap = |x: f32| {
        let t = x / length;
        assert!(!t.is_nan(), "原墙面坐标不得为 NaN");
        let r = t % 1.0;
        (if r < 0.0 { r + 1.0 } else { r }) * length
    };
    let (first_y, second_y) = if direction_sample > 0.5 {
        (area.min.y + 0.1, area.max.y + 0.1)
    } else { (area.max.y + 0.1, area.min.y + 0.1) };
    let first_u = wrap(left * 0.9 + right * 0.1);
    let second_u = wrap(left * f32::from_bits(0x3dcc_ccd0) + right * 0.9);
    let curve = wall.simplified_curve_2d();
    let p = WallSpaceCoord::new_unchecked(first_u, first_y).to_world_space(curve);
    let q = WallSpaceCoord::new_unchecked(second_u, second_y).to_world_space(curve);
    let u = (second_u * 0.5 + first_u * 0.5) / length;
    assert!((0.0..=1.0).contains(&u), "原墙面曲线坐标越界");
    let normal = if let Some(circle) = wall.is_circle() {
        let center = circle.get_pos_at_u(u);
        let dx = center.x - (p.x * 0.5 + q.x * 0.5);
        let dz = center.y - (p.z * 0.5 + q.z * 0.5);
        if !((dz * dz + dx * dx).sqrt() <= f32::from_bits(0x3d9f_be77)) { return; }
        circle.get_normal_at_u(u)
    } else { wall.is_rectangle().expect("原半木墙形缺失").get_normal_at_u(u) };
    let delta = q - p;
    let reciprocal = 1.0 / ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt();
    let direction = delta * reciprocal;
    let rotation = utils::geometry::rotation_from_normal(direction)
        * glam::Quat::from_xyzw(f32::from_bits(0x3f35_04f3), 0.0, 0.0, f32::from_bits(0x3f35_04f3));
    let translation = Vec3::new((normal.x * 0.15 + normal.x * 0.15) + p.x,
        (0.0f32 + 0.0) + p.y, (normal.y * 0.15 + normal.y * 0.15) + p.z);
    let distance = ((p.z - q.z) * (p.z - q.z)
        + ((p.y - q.y) * (p.y - q.y) + (p.x - q.x) * (p.x - q.x))).sqrt();
    let transform = glam::Affine3A::from_scale_rotation_translation(Vec3::new(0.15, distance, 0.15), rotation, translation);
    let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
        piece_seed: wall_id.0.wrapping_add(index as u64) as i32,
        wall_id: wall_id.0 as u32, flags: 0 };
    if writer.push(&record).is_err() { *write_failed = true; }
}

/// B 14121b659–b9e6：未被当前层高度区间覆盖的分割点立即写竖梁。
fn half_timber_vertical_plank(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    wall_id: country_core::resources::walls::WallId,
    area: Aabb2,
    position: f32,
    index: u32,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use utils::wall_space::WallSpaceCoord;
    let length = wall.wall_space_length();
    let t = position / length;
    assert!(!t.is_nan(), "原墙面坐标不得为 NaN");
    let r = t % 1.0;
    let wrapped = (if r < 0.0 { r + 1.0 } else { r }) * length;
    let point = WallSpaceCoord::new_unchecked(wrapped, area.min.y)
        .to_world_space(wall.simplified_curve_2d());
    let u = wrapped / length;
    assert!((0.0..=1.0).contains(&u), "原墙面曲线坐标越界");
    let (normal, depth) = if let Some(circle) = wall.is_circle() {
        (circle.get_normal_at_u(u), f32::from_bits(0x3d4c_ccce))
    } else {
        (wall.is_rectangle().expect("原半木墙形缺失").get_normal_at_u(u), 0.15)
    };
    let translation = Vec3::new((normal.x * 0.15 + normal.x * 0.15) + point.x,
        (0.0f32 + 0.0) + point.y, (normal.y * 0.15 + normal.y * 0.15) + point.z);
    let transform = glam::Affine3A::from_scale_rotation_translation(
        Vec3::new(0.15, (area.min.y - area.max.y).abs(), depth),
        utils::xz_rotation_bad(normal), translation);
    let record = WoodenPlankSsbo { transform: transform.into(), gate_seed: 0,
        piece_seed: wall_id.0.wrapping_add(index as u64) as i32,
        wall_id: wall_id.0 as u32, flags: 0 };
    if writer.push(&record).is_err() { *write_failed = true; }
}

/// 原逐层轮廓采样形成的12字节区间；tag 后续由半木主流程更新。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct HalfTimberHeightSpan { start: f32, end: f32, tag: u8 }

/// 原 14121b5f0–b631：flag0 含上界，其他值排除上界。
fn half_timber_span_contains(spans: &[HalfTimberHeightSpan], position: f32) -> bool {
    spans.iter().any(|span| span.start <= position
        && if span.tag == 0 { position <= span.end } else { position < span.end })
}

/// B 14121ad9e–aec0：区间种子及首抽概率门；调用者须保留返回的 RNG，
/// 后续几何继续使用同一随机流。
fn half_timber_pair_rng(
    wall_id: country_core::resources::walls::WallId,
    area_index: usize,
    left: f32,
    right: f32,
    height: f32,
) -> Option<fastrand::Rng> {
    let width = (right - left).abs();
    let ratio = width / height;
    if !(ratio >= 0.25) { return None; }
    let seed = (wall_id.0 as u32).wrapping_add(area_index as u32)
        .wrapping_add((left * 10.0) as u64 as u32)
        .wrapping_add((right * 10.0) as u64 as u32);
    let rng = utils::rng::rng_from_seed_u64(seed as u64);
    let random = rng.f32();
    let t = ((ratio - 0.25) * 4.0).max(0.0).min(1.0);
    let smooth = (3.0 - (t + t)) * (t * t);
    let threshold = smooth * 0.3 + (1.0 - smooth);
    if threshold <= random { None } else { Some(rng) }
}

/// 原 14121ddb0–df1a：矩形顶层细梁先在修正后的两端间生成随机采样点，
/// u 列用于后续排序，法线原样随每点保存；抽样继续消费调用者的 RNG。
fn half_timber_rectangle_fine_points(
    left: f32, right: f32, p: Vec3, q: Vec3, normal: Vec2, rng: &fastrand::Rng,
) -> Vec<(f32, Vec3, Vec2)> {
    let d = q - p;
    let distance = ((d.y * d.y + d.x * d.x) + d.z * d.z).sqrt();
    let count = ((distance / 0.4).ceil() as u64).max(2) as usize;
    let jitter = 0.05 / distance;
    let denominator = count as f32 - 1.0;
    let mut points = Vec::with_capacity(count);
    for i in 0..count {
        let t = i as f32 / denominator;
        let u = right * t + (1.0 - t) * left;
        let random = rng.f32();
        let t = t + (random * jitter - (1.0 - random) * jitter);
        let position = q * t + p * (1.0 - t);
        points.push((u, position, normal));
    }
    points
}

/// 原区域的初始三列分割记录；后续窗口合并会更新两侧界。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct HalfTimberSplit { position: f32, left: f32, right: f32 }

/// 原 141219c9d–dd4：低于阈值不产生初始分割，随后逐条追加。
/// 它必须在真实窗口查询之后执行；主调用暂不提前使用该子段。
fn half_timber_initial_splits(area: Aabb2, circle: bool) -> Vec<HalfTimberSplit> {
    let mut result = Vec::new();
    if !(area.max.y - area.min.y > 0.75) { return result; }
    let divisor = if circle { 2.0 } else { 2.5 };
    let count = (((area.max.x - area.min.x).abs() / divisor).ceil() as u64) as usize;
    let count_f = count as f32;
    for index in 0..count {
        let t = index as f32 / count_f;
        let position = (t * area.max.x) + ((1.0 - t) * area.min.x);
        result.push(HalfTimberSplit { position, left: position, right: position });
    }
    result
}

/// 原 141219e90–1a110：按窗口及分割顺序合并真实窗范围和跨界副本。
fn half_timber_merge_windows(splits: &mut [HalfTimberSplit], windows: &[WindowAccessInfo], wall_length: f32) {
    fn apply(split: &mut HalfTimberSplit, aabb: Aabb2) {
        if !aabb.contains(Vec2::new(split.position, aabb.center().y)) { return; }
        // 原 minnum 的平值选择已有 left，maxnum 的平值选择新 left。
        let left = if split.left.is_nan() { aabb.min.x }
            else if aabb.min.x.is_nan() || split.left <= aabb.min.x { split.left }
            else { aabb.min.x };
        split.left = left;
        split.right = if left.is_nan() { aabb.max.x }
            else if aabb.max.x.is_nan() || left >= aabb.max.x { left }
            else { aabb.max.x };
    }
    for window in windows.iter().filter(|window| window.shifted == 0) {
        let aabb = Aabb2 { min: Vec2::new(window.aabb[0], window.aabb[1]),
            max: Vec2::new(window.aabb[2], window.aabb[3]) };
        for split in splits.iter_mut() {
            apply(split, aabb);
            let center = aabb.center();
            let half = aabb.width() * 0.5;
            let shifted = if center.x + half > wall_length {
                Some(center.x - wall_length)
            } else if half > center.x {
                Some(center.x + wall_length)
            } else { None };
            if let Some(x) = shifted {
                apply(split, Aabb2::from_center_dims(Vec2::new(x, center.y),
                    Vec2::new(aabb.width(), aabb.height())));
            }
        }
    }
}

/// 原区域分割候选、实际窗/平顶洞过滤与相邻小间距平滑。
fn half_timber_split_positions(
    area: Aabb2, splits: &[HalfTimberSplit], windows: &[WindowAccessInfo], holes: &[Aabb2],
    wall_length: f32, wall: country_core::resources::walls::WallId,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    derived: &country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
) -> Vec<f32> {
    let mut positions = Vec::new();
    for split in splits {
        let mut pair = [split.position; 2];
        let count = if split.left != split.right {
            let left = split.left / wall_length;
            let right = split.right / wall_length;
            assert!(!left.is_nan() && !right.is_nan(), "原半木分割坐标为 NaN");
            let wrap = |x: f32| { let r = x % 1.0; if r < 0.0 { r + 1.0 } else { r } };
            // 原先计算右侧 fmod，再计算左侧。
            let r = wrap(right);
            let l = wrap(left);
            pair = [wall_length * l, wall_length * r];
            2
        } else { 1 };
        for &position in &pair[..count] {
            let outside = area.min.x > position || position > area.max.x;
            if outside {
                if check_real_windows(Vec2::new(position, area.center().y), wall_length, wall, storage, derived) {
                    continue;
                }
            } else if windows.iter().any(|window| {
                !(position < window.aabb[0] + 0.05 || window.aabb[2] - 0.05 <= position)
            }) {
                continue;
            }
            // 原 LAB_14121a440 在两条窗口路径之后共同检查平顶孔洞。
            if holes.iter().any(|hole| !(position < hole.min.x) && !(hole.max.x <= position)) { continue; }
            positions.push(position);
        }
    }
    positions.sort_by(|a, b| a.partial_cmp(b).expect("原半木分割坐标不可比较"));
    let original = positions.clone();
    if original.len() > 1 && (original[1] - positions[0]).abs() < 0.3 {
        positions[0] = positions[0] * 0.5 + original[1] * 0.5;
    }
    for index in 1..positions.len() {
        if index + 1 < original.len() && (original[index + 1] - positions[index]).abs() < 0.3 {
            positions[index] = positions[index] * 0.5 + original[index + 1] * 0.5;
        }
        if (original[index - 1] - positions[index]).abs() < 0.3 {
            positions[index] = positions[index] * 0.5 + original[index - 1] * 0.5;
        }
    }
    positions
}

fn half_timber_height_spans(
    curve: &utils::curve::Curve2, levels: &[f32], segment_heights: &[f32], path_heights: &[f32],
    flags: &mut [u8], result: &mut [Vec<HalfTimberHeightSpan>],
) {
    let count = ((curve.length / 0.3).ceil() as usize).max(2);
    let denominator = count as f32 - 1.0;
    for (level_index, &level) in levels.iter().enumerate() {
        let spans = &mut result[level_index];
        let mut index = 0usize;
        let mut position = 0.0f32;
        let mut previous_u = 0.0f32;
        for sample in 0..count {
            let u = sample as f32 / denominator;
            let delta = u - previous_u;
            assert!(delta >= 0.0, "原半木曲线采样必须有序");
            if index + 1 < curve.points_u.len() {
                let mut next_u = curve.points_u[index + 1];
                let mut length_u = (next_u - curve.points_u[index]).abs();
                let mut remainder = (1.0 - position) * length_u - delta;
                if remainder >= 0.0 {
                    position = 1.0 - remainder / length_u;
                } else {
                    position = 1.0;
                    while index + 2 < curve.points_u.len() {
                        let after = curve.points_u[index + 2];
                        length_u = (after - next_u).abs();
                        remainder += length_u;
                        index += 1;
                        if remainder >= 0.0 {
                            position = 1.0 - remainder / length_u;
                            break;
                        }
                        next_u = after;
                    }
                }
            }
            let inverse = 1.0 - position;
            let segment = position * segment_heights[index + 1] + inverse * segment_heights[index];
            let visible = if segment > 0.0 {
                let base = inverse * path_heights[index] + position * path_heights[index + 1];
                segment + base > level
            } else { false };
            if visible {
                let x = u * curve.length;
                if flags[level_index] != 0 {
                    let last = spans.last_mut().expect("原半木活跃区间缺失");
                    last.end = x + 0.0;
                    last.tag = 0;
                } else {
                    spans.push(HalfTimberHeightSpan { start: x, end: x, tag: 0 });
                }
            }
            flags[level_index] = u8::from(visible);
            previous_u = u;
        }
    }
}

/// 原 closure$4/$5 的墙面区域构造，按原形状、边和层高顺序。
fn half_timber_areas(
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    levels: &[f32],
    gable_sides: Option<[u8; 2]>,
) -> Vec<Aabb2> {
    if wall.is_circle().is_some() {
        levels.windows(2).map(|pair| Aabb2 {
            min: Vec2::new(0.0, pair[0]),
            max: Vec2::new(wall.wall_space_length(), pair[1]),
        }).collect::<Vec<_>>()
    } else {
        // 原 closure$5 先按四条边，再按层高窗口输出；山墙侧不裁至普通墙顶。
        let rectangle = wall.is_rectangle().expect("原半木墙形必须为矩形");
        let mut result = Vec::new();
        for side in 0..4 {
            let start = rectangle.get_point_u(side) * wall.wall_space_length();
            let end = rectangle.get_point_u(side + 1) * wall.wall_space_length();
            let is_gable = gable_sides.is_some_and(|sides| sides.contains(&(side as u8)));
            for pair in levels.windows(2) {
                // 原山墙侧也读取 max_y，因此保留未初始化时的失败顺序。
                let max_y = wall.max_y();
                let (lo, hi) = if is_gable {
                    (pair[0], pair[1])
                } else {
                    let min_original = |value: f32| {
                        if value.is_nan() { max_y }
                        else if max_y.is_nan() || value <= max_y { value }
                        else { max_y }
                    };
                    let lo = min_original(pair[0]);
                    if !(lo < max_y) { continue; }
                    (lo, min_original(pair[1]))
                };
                result.push(Aabb2 { min: Vec2::new(start, lo), max: Vec2::new(end, hi) });
            }
        }
        result
    }
}

/// B `roof_outline`（0x141217be0）：先检查形状，再取得真实库和墙分配。
/// 调用者完成半木后释放前一个写入器；此处重新锁住同一分配并追加。
pub fn roof_outline(
    wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    wall_id: country_core::resources::walls::WallId,
    roof: &country_core::resources::roofs::roof_shape::Roof,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &crate::gpu_output::ConstructorGpuOutput,
) {
    let Some(rectangle) = wall.is_rectangle() else { return };
    let storage = library.get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
        .expect("原屋顶轮廓木板缓冲区缺失");
    let mut writer = output.half_timber_append_writer(storage, wall_id.0);
    roof_outline_geometry(rectangle, wall_id, roof, &mut writer);
}

/// 四侧的唯一数值主体；上层持有与原版相同的分配锁。
fn roof_outline_geometry(
    rectangle: utils::geometry::Rectangle2d,
    wall_id: country_core::resources::walls::WallId,
    roof: &country_core::resources::roofs::roof_shape::Roof,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
) {
    use country_core::resources::roofs::geometry::surface::Curve2;
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls;
    use country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom;
    use glam::Affine3A;
    use utils::geometry::Rectangle2d;

    let (bottom_size, mut top_size) = get_roof_expanded_top_and_bottom(roof).as_rectangle();
    let direction = usize::from(roof.observed()[0x3c]);
    let bottom = Rectangle2d::new_centered_axis_aligned(Vec2::from_array(bottom_size));
    top_size[direction] = bottom_size[direction];
    let top = Rectangle2d::new_axis_aligned(
        Vec2::from_array(calc_roof_tip_center_ls(roof, [0.0, 0.0])),
        Vec2::from_array(top_size),
    );
    let profile = Curve2::try_new(roof.profile_curve_normalized().to_vec(), false)
        .expect("原屋顶轮廓曲线无效");
    let bottom_points = bottom.as_points2();
    let top_points = top.as_points2();
    let orientation = rectangle.orientation_mat2();
    let height = roof.height();
    let floor = roof.floor_y();
    let wall_word = wall_id.0 as u32;
    for side in 0..4 {
        let top_xz = rectangle.center + orientation * top_points[side];
        let bottom_xz = rectangle.center + orientation * bottom_points[side];
        let delta = top_xz - bottom_xz;
        let horizontal = Vec3::new(delta.x, 0.0, delta.y);
        let vertical = Vec3::new(height * 0.0, height, height * 0.0);
        let length_xz = (delta.y * delta.y + delta.x * delta.x).sqrt();
        let height_len = (vertical.x * vertical.x + vertical.y * vertical.y + vertical.z * vertical.z).sqrt();
        let world_profile = roof.profile_curve_ws(0.0, length_xz, height_len)
            .expect("原世界屋顶轮廓曲线无效");
        let divisions = world_profile.length.round().max(2.0) as usize;
        let cross = vertical.cross(horizontal);
        let cross_length = (cross.z * cross.z + cross.x * cross.x + cross.y * cross.y).sqrt();
        let offset = (cross * (1.0 / cross_length)) * 0.0;
        let base = Vec3::new(bottom_xz.x, 0.0, bottom_xz.y);
        let floor_offset = Vec3::new(floor * 0.0, floor, floor * 0.0);
        let axis = if direction == 0 { rectangle.axis } else { Vec2::new(-rectangle.axis.y, rectangle.axis.x) };
        let axis = axis * if side == 3 { 1.0 } else { -1.0 };
        for index in 0..divisions {
            let a = profile.pos_at_u(index as f32 / divisions as f32);
            let b = profile.pos_at_u((index + 1) as f32 / divisions as f32);
            let first = (horizontal * a[0] + vertical * a[1] + offset + base) + floor_offset;
            let second = (horizontal * b[0] + vertical * b[1] + offset + base) + floor_offset;
            let line = first - second;
            let length = ((line.y * line.y + line.x * line.x) + line.z * line.z).sqrt();
            let rotation = utils::geometry::rotation_from_normal(line * (1.0 / length));
            let transform: country_core::resources::render::Affine3Packed = Affine3A::from_scale_rotation_translation(
                Vec3::new(0.4, 0.25, length + 0.05), rotation, (first + second) * 0.5,
            ).into();
            let beam = WoodenPlankSsbo {
                transform,
                gate_seed: 0, piece_seed: wall_word as i32, wall_id: wall_word, flags: 1,
            };
            let _ = writer.push(&beam);
            let across = Vec2::new(second.x - first.x, second.z - first.z);
            let across = across * (1.0 / (across.y * across.y + across.x * across.x).sqrt());
            let support = WoodenPlankSsbo {
                transform: Affine3A::from_scale_rotation_translation(
                    Vec3::new(0.1875, 0.1875, 0.7), utils::xz_rotation_bad(axis),
                    first + Vec3::new(across.x, 0.0, across.y) * 0.25,
                ).into(),
                gate_seed: 0, piece_seed: wall_word as i32, wall_id: wall_word, flags: 1,
            };
            let _ = writer.push(&support);
        }
    }
}

#[cfg(test)]
mod roof_outline_tests {
    use super::roof_outline;
    use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::gpu_output::ConstructorGpuOutput;
    use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
    use country_core::resources::roofs::{geometry::surface::Curve2, roof_buffers::WoodenPlankSsbo, roof_shape::Roof};
    use country_core::resources::walls::{WallId, WallState, public_wall_state::PublicWallState, publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};

    struct Device;
    impl GraphicsDevice for Device {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
            Ok(Buffer { id: 1, byte_size: desc.size as u64, ..Default::default() })
        }
        fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
    }

    #[test]
    fn roof_outline_matches_b_original_rectangular_profile() {
        let fixtures: [&[u8]; 8] = [
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-1.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-2.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-3.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-4.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-5.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-6.bin")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-roof-outline-native-output-7.bin")),
        ];
        let mut mismatches = Vec::new();
        for (case, native) in fixtures.into_iter().enumerate() {
        let state = WallState::new(Curve2::try_new(vec![[-3., -2.], [3., -2.]], false).unwrap(), 1., 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, value) in if case % 4 < 2 { [1f32, 0., 0., 0., 6., 4.] } else { [0f32, 1., 0., 0., 6., 4.] }.into_iter().enumerate() { shape[4+i*4..8+i*4].copy_from_slice(&value.to_le_bytes()); }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let mut raw = [0u8; 88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32),(0x30,0.5),(0x34,1.0),(0x4c,2.0)] { raw[offset..offset+4].copy_from_slice(&value.to_le_bytes()); }
        if case >= 4 {
            for (offset, value) in [(0x24,0.2f32),(0x28,-0.3),(0x2c,0.2),(0x30,0.8),(0x38,0.7)] { raw[offset..offset+4].copy_from_slice(&value.to_le_bytes()); }
        }
        raw[0x3c] = (case % 2) as u8;
        let roof = Roof::from_observed(raw);
        let mut library = AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        roof_outline(&wall, WallId(123), &roof, &mut library, &output);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), native.len()/64);
        let mut all = Vec::new();
        for (index, record) in storage.data.iter().take(native.len()/64).enumerate() {
            let mut got = Vec::new();
            for col in record.transform.cols { for value in col.to_array() { got.extend(value.to_bits().to_le_bytes()); } }
            for value in [record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags] { got.extend(value.to_le_bytes()); }
            all.extend(got);
            let _ = index;
        }
        if all != native { mismatches.push(case); }
        // 同一个分配的尾块先追加；非矩形状态须保持既有记录不动。
        let saved = wall.authority.clone();
        let mut snapshot = saved.clone().into_snapshot().0;
        let mut freehand = [0u8; 28]; freehand[..4].copy_from_slice(&2u32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(freehand);
        wall.authority = WallState::from_snapshot(snapshot, 0);
        roof_outline(&wall, WallId(123), &roof, &mut library, &output);
        assert_eq!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), native.len()/64);
        wall.authority = saved;
        roof_outline(&wall, WallId(123), &roof, &mut library, &output);
        assert_eq!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), native.len()/32);
        }
        assert!(mismatches.is_empty(), "原实例差异样本: {mismatches:?}");
    }
    #[test]
    fn roof_outline_gates_resources_and_relocks_original_wall_owner() {
        use bevy_ecs::prelude::World;
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let state = WallState::new(Curve2::try_new(vec![[-3., -2.], [3., -2.]], false).unwrap(), 1., 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8; 28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 6., 4.].into_iter().enumerate() { shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes()); }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&WallState::from_snapshot(snapshot, 0), &terrain).unwrap();
        let mut raw = [0u8; 88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32),(0x30,0.5),(0x34,1.0),(0x4c,2.0)] { raw[offset..offset+4].copy_from_slice(&value.to_le_bytes()); }
        let roof = Roof::from_observed(raw);
        let rectangle = wall.authority.clone();
        let mut free = rectangle.clone().into_snapshot().0;
        let mut free_shape = [0u8; 28]; free_shape[..4].copy_from_slice(&2u32.to_le_bytes());
        free.source.shape = WallShapeSnapshot::from_observed(free_shape);
        wall.authority = WallState::from_snapshot(free, 0);
        let mut world = World::new();
        world.insert_resource(AssetSsboLibrary::new());
        world.insert_resource(ConstructorGpuOutput::default());
        let call = |world: &mut World, wall: &PublicWallState| {
            let mut library = world.remove_resource::<AssetSsboLibrary>().unwrap();
            let result = catch_unwind(AssertUnwindSafe(|| {
                let output = world.resource::<ConstructorGpuOutput>();
                roof_outline(wall, WallId(123), &roof, &mut library, output);
            }));
            world.insert_resource(library);
            result
        };
        // 原矩形守卫先于库查找：二者均缺也正常返回。
        assert!(call(&mut world, &wall).is_ok());
        wall.authority = rectangle;
        assert!(call(&mut world, &wall).is_err());
        world.resource::<AssetSsboLibrary>().register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        assert!(call(&mut world, &wall).is_err());
        world.resource_scope(|world, mut library: bevy_ecs::prelude::Mut<AssetSsboLibrary>| {
            let mut output = world.resource_mut::<ConstructorGpuOutput>();
            let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
            output.init_half_timber_planks(storage, 123);
            let mut writer = output.half_timber_writer(storage, 123);
            writer.push(&WoodenPlankSsbo::default()).unwrap();
        });
        call(&mut world, &wall).unwrap();
        world.resource_scope(|_world, mut library: bevy_ecs::prelude::Mut<AssetSsboLibrary>| {
            let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
            assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 57);
            assert_eq!(storage.data[0], WoodenPlankSsbo::default());
        });
        // 同类型的另一个物理缓冲不能借用原墙分配。
        let mut foreign = AssetSsboLibrary::new();
        foreign.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let output = world.resource::<ConstructorGpuOutput>();
        assert!(catch_unwind(AssertUnwindSafe(|| roof_outline(&wall, WallId(123), &roof, &mut foreign, output))).is_err());
    }

}

/// 窗装饰器记录的安全输入（调用方从 DecoratorStorage 取出）。
///
/// `raw` 的原件读取偏移：+0x0c（leaf 标志）、+0x0e（subtype 族）、
/// +0x44（子类型/朝向字节）、+0x46（窗类型字节 '8'/'9'/':'）。
#[derive(Clone, Copy, Debug)]
pub struct WindowAabbSource<'a> {
    /// 装饰器原始记录（至少 0x50 字节；这里保留 0x50）。
    pub raw: [u8; 0x50],
    /// `DecoratorInfo::dst_as_wall` 的挂点 (x0, y0)。
    pub attachment: [f32; 2],
    /// `DerivedDecoratorInfo::dims_wall_space` 的三浮点结果 (a, b, c)。
    pub bbx: [f32; 3],
    /// gothic profile 的墙面坐标点（raw[0x46]=='9' 时使用）。
    pub profile: &'a [[f32; 2]],
}

impl Default for WindowAabbSource<'_> {
    fn default() -> Self {
        Self {
            raw: [0; 0x50],
            attachment: [0.0; 2],
            bbx: [0.0; 3],
            profile: &[],
        }
    }
}

/// B `get_window_aabb`（0x141221da0，逐语句重建）。
///
/// - `raw[0x46] == '9'`（门类）：profile 末点 (px, py)、首点 (fx, fy)；
///   `aabb = [x0 − px, y0 + py, x0 + px, fy + 0.093 + y0]`。
///   `subtype == 9` 走 `gothic_door_profile_curve`，否则
///   `gothic_half_dormer_profile_curve(raw[0x44])`（两者由调用方在 profile
///   中给出——曲线函数属装饰器 crate）。
/// - 其他：
///   `width = max(bbx[0] + 0.2, 0.46)`；
///   `height = bbx[2] − bbx[1]`；
///   `raw[0x46]=='8'` → height += 0.2；
///   `raw[0x46]==':'` → `raw[0x44]==3` 时 width −= 0.2，height += 0.4；
///   `Aabb2::from_center_dims(center, (width, height))`。
pub fn get_window_aabb(window: &WindowAabbSource) -> Aabb2 {
    let subtype_tag = window.raw[0x46];
    if subtype_tag == b'9' {
        // 原函数对空哥特轮廓 unwrap 失败，不能伪造零尺寸窗洞。
        let first = window.profile.first().expect("原哥特窗轮廓为空");
        let last = window.profile.last().expect("原哥特窗轮廓为空");
        let [x0, y0] = window.attachment;
        return Aabb2 {
            min: Vec2::new(x0 - last[0], y0 + last[1]),
            max: Vec2::new(x0 + last[0], first[1] + f32::from_bits(0x3dbe76c9) + y0),
        };
    }
    let mut width = window.bbx[0] + f32::from_bits(0x3e4ccccd); // +0.2
    // 原 MAXSS 的源操作数是 0.46；输入 NaN 时也选该源值。
    if !(width > f32::from_bits(0x3eeb851f)) {
        width = f32::from_bits(0x3eeb851f); // 0.46
    }
    let mut height = window.bbx[2] - window.bbx[1];
    if subtype_tag == b'8' {
        height += f32::from_bits(0x3e4ccccd); // +0.2
    } else if subtype_tag == b':' {
        if window.raw[0x44] == 3 {
            width += f32::from_bits(0xbe4ccccd); // −0.2
        }
        height += f32::from_bits(0x3ecccccd); // +0.4
    }
    let center = Vec2::from_array(window.attachment);
    Aabb2::from_center_dims(center, Vec2::new(width, height))
}

/// B `overlapping_windows`（0x1412218a0）：对已取出的窗集合做 AABB 访问测试，
/// 返回 `ResourceAccessInfo` 投影（20B：aabb 16B + leaf + subtype + shifted）。
///
/// 调用方已完成原件 SIMD 掩码/子类型过滤（`byte[10] < 2 &&
/// (byte[0xe] − 0x38) < 3`）；本函数按原件逐窗展开与三向移位测试。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowAccessInfo {
    pub aabb: [f32; 4],
    /// 原 `pauStack_110[-1][0xc]`。
    pub leaf: u8,
    /// `DerivedDecoratorInfo::subtype`。
    pub subtype: u8,
    /// 0 = 直接相交；1 = 侧向移位后相交。
    pub shifted: u8,
}

impl WindowAccessInfo {
    pub fn observed_bytes(&self) -> [u8; 20] {
        let mut b = [0u8; 20];
        for (i, v) in self.aabb.iter().enumerate() {
            b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        b[16] = self.leaf;
        b[17] = self.subtype;
        b[18] = self.shifted;
        b
    }
}

fn aabb_array(aabb: Aabb2) -> [f32; 4] {
    [aabb.min.x, aabb.min.y, aabb.max.x, aabb.max.y]
}

/// B `overlapping_windows`（0x1412218a0）。
///
/// `query` 为待测墙 AABB（归一化/墙面空间），`expanded_limit` 为原件
/// `param_6`（0.038 外的横向可用余量）。
pub fn overlapping_windows(
    query: Aabb2,
    expanded_limit: f32,
    windows: &[WindowAabbSource],
    subtypes: &[u8],
) -> Vec<WindowAccessInfo> {
    let mut out = Vec::new();
    overlapping_windows_into(query, expanded_limit, windows, subtypes, &mut out);
    out
}

fn overlapping_windows_into(
    query: Aabb2, expanded_limit: f32, windows: &[WindowAabbSource], subtypes: &[u8],
    out: &mut Vec<WindowAccessInfo>,
) {
    for (window, &subtype) in windows.iter().zip(subtypes) {
        let mut aabb = get_window_aabb(window);
        // 原件只调整 x 两界：min.x +0.038、max.x −0.038（向内收 3.8cm）。
        aabb.min.x += f32::from_bits(0x3d1ba5e3);
        aabb.max.x += f32::from_bits(0xbd1ba5e3);
        if subtype == b'\t' && window.raw[0x0c] == 1 {
            // dormer 'g' 分支：min.x −0.14、max.x +0.14。
            aabb.min.x += f32::from_bits(0xbe0f5c29);
            aabb.max.x += f32::from_bits(0x3e0f5c29);
        }
        let leaf = window.raw[0x0c];
        if query.intersects(aabb) {
            out.push(WindowAccessInfo {
                aabb: aabb_array(aabb),
                leaf,
                subtype,
                shifted: 0,
            });
        }
        // 原件第二段：中心 ± 半宽 与 expanded_limit 比较后左右移位。
        let center = aabb.center();
        let half = aabb.half_dims();
        // 原 UCOMISS/JBE 对无序值也走分支。
        let shifted = if !((center.x + half.x) > expanded_limit) {
            if !(half.x > center.x) {
                continue;
            }
            Aabb2::from_center_dims(
                Vec2::new(center.x + expanded_limit, center.y),
                Vec2::new(aabb.width(), aabb.height()),
            )
        } else {
            Aabb2::from_center_dims(
                Vec2::new(center.x - expanded_limit, center.y),
                Vec2::new(aabb.width(), aabb.height()),
            )
        };
        if query.intersects(shifted) {
            out.push(WindowAccessInfo {
                aabb: aabb_array(shifted),
                leaf,
                subtype,
                shifted: 1,
            });
        }
    }
}

/// 原 1412218a0 从同一墙桶直接遍历装饰，按原类型门查真实 subtype 与 bbox。
fn overlapping_windows_from_storage(
    query: Aabb2, expanded_limit: f32, wall: country_core::resources::walls::WallId,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    derived: &country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
) -> Vec<WindowAccessInfo> {
    let mut out = Vec::new();
    visit_real_windows(wall, storage, derived, |window, subtype| {
        overlapping_windows_into(query, expanded_limit, &[window], &[subtype], &mut out);
        false
    });
    out
}

/// 两个原窗查询共用相同的实际桶、类型门及 bbox/profile 读取。
fn visit_real_windows(
    wall: country_core::resources::walls::WallId,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    derived: &country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
    mut visit: impl FnMut(WindowAabbSource<'_>, u8) -> bool,
) -> bool {
    use country_core::resources::walls::decorator::DecoratorId;
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    use country_core::resources::gothic_window_profile::{gothic_door_profile_curve, gothic_half_dormer_profile_curve};
    let Some(bucket) = storage.get(DecoratorStorageAddr::Wall(wall)) else { return false };
    for (&id, info) in &bucket.decorators {
        if !info.is_wall_dst() || !(b'8'..=b':').contains(&info.ty) { continue; }
        let subtype = derived.subtype(DecoratorId(id));
        let mut raw = [0u8; 0x50];
        // 原 bucket 尾部 -4 同时作为窗 rank/leaf。
        raw[0x0c] = info.rank;
        raw[0x44] = info.rank;
        raw[0x46] = info.ty;
        let profile = if info.ty == b'9' {
            let points = if subtype.0 == 9 { gothic_door_profile_curve(info.rank) }
                else { gothic_half_dormer_profile_curve(info.rank) };
            points.into_iter().map(|point| point.to_array()).collect::<Vec<_>>()
        } else { Vec::new() };
        let bbx = if info.ty == b'9' { [0.0; 3] } else {
            let bounds = derived.bbx_ls(subtype, info.rank);
            [bounds.footprint.size.x, bounds.bottom, bounds.top]
        };
        let coord = info.dst_as_wall().wall_space_coord();
        let window = WindowAabbSource { raw, attachment: [coord.u, coord.y], bbx, profile: &profile };
        if visit(window, subtype.0) { return true; }
    }
    false
}

fn check_real_windows(
    point: Vec2, expansion: f32, wall: country_core::resources::walls::WallId,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    derived: &country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView<'_>,
) -> bool {
    visit_real_windows(wall, storage, derived, |window, _| {
        check_for_overlap_with_all_windows(point, expansion, &[window])
    })
}

/// B `check_for_overlap_with_all_windows`（0x141221f70）：对每个窗调用
/// `get_window_aabb`，先测查询点是否在 AABB 内；否则按 `expansion` 左右
/// 移位中心 x（保持 y/宽高），再次测点。返回任一命中。
pub fn check_for_overlap_with_all_windows(
    point: Vec2,
    expansion: f32,
    windows: &[WindowAabbSource],
) -> bool {
    for window in windows {
        let aabb = get_window_aabb(window);
        if aabb.contains(point) {
            return true;
        }
        let center = aabb.center();
        let half_width = aabb.width() * 0.5;
        let shifted_x = if center.x + half_width > expansion {
            center.x - expansion
        } else if !(half_width > center.x) {
            continue;
        } else {
            center.x + expansion
        };
        let shifted = Aabb2::from_center_dims(
            Vec2::new(shifted_x, center.y),
            Vec2::new(aabb.width(), aabb.height()),
        );
        if shifted.contains(point) {
            return true;
        }
    }
    false
}

/// 原窗下图案的墙面端点前段；写入世界空间及梁实例仍由主函数完成。
fn underwindow_wall_segments(subtype: u8, top_y: f32, base_y: f32,
    left: f32, right: f32, floor_y: f32) -> Option<[[Vec2; 2]; 3]> {
    if subtype & 0x1e == 0x16 { return None; }
    let height = top_y - base_y;
    // MAXSS 的第二操作数为零，NaN 也取零。
    let height = if height > 0.0 { height } else { 0.0 };
    if !(height > 0.5 && height < 2.0) { return None; }
    let middle = right * 0.5 + left * 0.5;
    let top = top_y + 0.13;
    let lower = (0.2 + base_y) + if base_y == floor_y { f32::from_bits(0xbe23_d70b) } else { 0.0 };
    Some([
        [Vec2::new(left + 0.1, top), Vec2::new(middle + 0.1, lower)],
        [Vec2::new(middle - 0.1, lower), Vec2::new(right - 0.1, top)],
        [Vec2::new(left, top), Vec2::new(right, top)],
    ])
}

/// B 0x141fd1c30：窗下三根梁，按原墙面投影、弧度门和顺序写入。
/// `generated` 是原调用者的分支标志，`write_failed` 保留任一分配失败。
/// 主流程已调用；圆形通过门及更多输入仍须原执行对照。
pub fn add_underwindow_pattern(
    window: &WindowAccessInfo,
    base: &Aabb2,
    left: f32,
    right: f32,
    generated: &mut bool,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    shape: &country_core::systems::wall::valid_enclosure::ShapeParameters,
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_, country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    write_failed: &mut bool,
    wall_id: country_core::resources::walls::WallId,
    piece_index: u32,
    floor_y: f32,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use country_core::systems::wall::valid_enclosure::Shape;
    use utils::wall_space::WallSpaceCoord;
    let Some(segments) = underwindow_wall_segments(window.subtype, window.aabb[1], base.min.y, left, right, floor_y) else { return };
    *generated = true;
    let length = wall.wall_space_length();
    let curve = wall.simplified_curve_2d();
    for [a, b] in segments {
        let wrap = |x: f32| {
            let t = x / length;
            assert!(!t.is_nan(), "原墙面坐标不得为 NaN");
            let r = t % 1.0;
            (if r < 0.0 { r + 1.0 } else { r }) * length
        };
        let au = wrap(a.x);
        let bu = wrap(b.x);
        let p = WallSpaceCoord::new_unchecked(au, a.y).to_world_space(curve);
        let q = WallSpaceCoord::new_unchecked(bu, b.y).to_world_space(curve);
        let u = (bu * 0.5 + au * 0.5) / length;
        assert!((0.0..=1.0).contains(&u), "原墙面曲线坐标越界");
        let normal = shape.get_normal_at_u(u);
        if let Shape::Circle(circle) = shape.get_shape().expect("原窗下图案要求闭合形状") {
            let center = circle.get_pos_at_u(u);
            let dx = center.x - (q.x * 0.5 + p.x * 0.5);
            let dz = center.y - (q.z * 0.5 + p.z * 0.5);
            if (dz * dz + dx * dx).sqrt() > 0.078 { continue; }
        }
        let delta = q - p;
        let reciprocal = 1.0 / ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt();
        let direction = delta * reciprocal;
        let rotation = utils::geometry::rotation_from_normal(direction);
        let translation = glam::Vec3::new(
            (normal.x * 0.15 + normal.x * 0.15) + (p.x + q.x) * 0.5,
            (0.0f32 + 0.0) + (p.y + q.y) * 0.5,
            (normal.y * 0.15 + normal.y * 0.15) + (p.z + q.z) * 0.5,
        );
        let distance = ((p.z - q.z) * (p.z - q.z)
            + ((p.y - q.y) * (p.y - q.y) + (p.x - q.x) * (p.x - q.x))).sqrt();
        let affine = glam::Affine3A::from_scale_rotation_translation(
            glam::Vec3::new(0.15, 0.15, distance), rotation, translation,
        );
        let record = WoodenPlankSsbo {
            transform: affine.into(), gate_seed: 0,
            piece_seed: (wall_id.0 as u32).wrapping_add(piece_index) as i32,
            wall_id: wall_id.0 as u32, flags: 0,
        };
        if writer.push(&record).is_err() { *write_failed = true; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 真实出版墙体加同一双窗双洞原输入，比较全部木板而非只比较数量。
    fn assert_rect_multi_original(wall: &country_core::resources::walls::public_wall_state::PublicWallState,
        roof: &country_core::resources::roofs::roof_shape::Roof, original: &[u8], count: usize) {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
        use country_core::resources::walls::{WallId, decorator::DecoratorInfo,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey,
                DecoratorMeshLibrary, DecoratorOutline, DecoratorSubtype, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
        }
        let mut decorators = DecoratorStorage::default();
        for (id, x) in [(42, 0f32), (43, 4f32)] {
            decorators.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(123))).decorators.insert(id,
                DecoratorInfo { dst_wall: u64::from(x.to_bits()) | (u64::from(1f32.to_bits()) << 32),
                    rank: 1, ty: b'8', ..Default::default() });
        }
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(42, DecoratorSubtype(3)); subtypes.insert(43, DecoratorSubtype(3));
        let mut meshes = DecoratorMeshLibrary::default(); let mut collision = DecoratorCollision::empty();
        collision.bounds = utils::geometry::Rectangle3d::new(Vec3::new(0.,1.,0.), Vec3::new(1.,2.,1.), Vec2::X);
        meshes.insert_collision_and_interaction_as_same(DecoratorLibKey::new(3,1), collision,
            DecoratorOutline::None, DecoratorCollisionOutline::None);
        let holes = [
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [0.25,0.,0.75,2.], kind: 1, origin: 0 },
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [5.,0.,5.6,2.], kind: 4, origin: 0 },
        ];
        let mut terrain = country_core::resources::terrain_height::TerrainHeightsData::new(); terrain.set_height_range(0., 0.);
        let heights: Vec<_> = wall.prepared_path.as_ref().unwrap().positions.iter().map(|p| p[1]).collect();
        let input = HalfTimberInput { wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain,
            segment_heights: &wall.segment_heights, path_heights: &heights, roof, holes: &holes, demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), count);
        for (i, v) in storage.data.iter().take(count).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原真实出版双窗双洞木板 {i}");
        }
    }

    #[test]
    fn export_real_circle_gable_publication_for_original_probe() {
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_shape::Roof};
        use country_core::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let circle = utils::geometry::Circle2d { center: Vec2::ZERO, radius: 2., rotation: 0. };
        let source = circle.as_curve2(0.9);
        let curve = Curve2 { points: source.points.iter().map(|v| v.to_array()).collect(),
            points_u: source.points_u, length: source.length };
        let state = WallState::new(curve, 2.5, 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8;28]; shape[12..16].copy_from_slice(&2f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), false).unwrap();
        wall.prepare_paths(&rasters).unwrap();
        let mut raw = [0u8;88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,1.), (0x34,1.), (0x4c,wall.max_y())] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw); wall.update_gable_heights(Some(roof.observed())).unwrap();
        let source = wall.simplified_curve_2d(); let prepared = wall.prepared_path.as_ref().unwrap();
        let projection = wall.building_projection(); let bytes = projection.observed_bytes();
        let words = |data: &[f32]| data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let pairs = |data: &[[f32;2]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let triples = |data: &[[f32;3]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let tail: Vec<u32> = bytes[0x280..0x290].chunks_exact(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        println!("NATIVE_GABLE_INPUT={{\"wall_id\":123,\"shape\":{:?},\"roof\":{:?},\"curve2_points\":{:?},\"curve2_u\":{:?},\"curve2_length\":{},\"mapped_points\":{:?},\"mapped_u\":{:?},\"mapped_length\":{},\"prepared_points\":{:?},\"prepared_u\":{:?},\"prepared_length\":{},\"segment_values\":{:?},\"arch\":{:?},\"segment\":{:?},\"path_y\":{:?},\"public_280_words\":{:?},\"max_y\":{},\"max_y_gable\":{}}}",
            shape.to_vec(), raw.to_vec(), pairs(&source.points), words(&source.points_u), source.length.to_bits(),
            triples(&wall.geometry.outline), words(&wall.geometry.outline_u), wall.geometry.mapped_curve_length.to_bits(),
            triples(&prepared.positions), words(&prepared.points_u), prepared.length.to_bits(), words(&prepared.segment_values),
            words(wall.arch_segment_heights().unwrap()), words(&wall.segment_heights),
            words(&prepared.positions.iter().map(|p| p[1]).collect::<Vec<_>>()), tail, wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert_eq!(wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert!(roof.gable_sides().is_some());
        // 以同一真实出版对象进入生成器，对照原入口完整输出。
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
        use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorage,
            decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
        }
        let mut terrain_data = country_core::resources::terrain_height::TerrainHeightsData::new();
        terrain_data.set_height_range(0., 0.);
        let decorators = DecoratorStorage::default();
        let subtypes = DerivedDecoratorSubtypes::default(); let meshes = DecoratorMeshLibrary::default();
        let path_heights: Vec<_> = prepared.positions.iter().map(|p| p[1]).collect();
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &wall.segment_heights, path_heights: &path_heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-gable-published-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 7);
        for (i, v) in storage.data.iter().take(7).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原真实圆墙山墙木板 {i}");
        }
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-gable-published-curved.bin"));
        assert_eq!(curved.len(), 7);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected, "原真实圆墙山墙曲面 {i}");
        }
    }

    #[test]
    fn export_real_multilevel_flat_publication_for_original_probe() {
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_shape::Roof};
        use country_core::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let curve = Curve2::try_new(vec![[-2.,-1.], [-2.,1.], [2.,1.], [2.,-1.], [-2.,-1.]], false).unwrap();
        let state = WallState::new(curve, 6.0, 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8;28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), false).unwrap();
        wall.prepare_paths(&rasters).unwrap();
        let mut raw = [0u8;88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,0.), (0x34,1.), (0x4c,wall.max_y())] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw);
        wall.update_gable_heights(Some(roof.observed())).unwrap();
        let source = wall.simplified_curve_2d();
        let prepared = wall.prepared_path.as_ref().unwrap();
        let projection = wall.building_projection();
        let bytes = projection.observed_bytes();
        let words = |data: &[f32]| data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let pairs = |data: &[[f32;2]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let triples = |data: &[[f32;3]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let tail: Vec<u32> = bytes[0x280..0x290].chunks_exact(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        println!("NATIVE_GABLE_INPUT={{\"wall_id\":123,\"shape\":{:?},\"roof\":{:?},\"curve2_points\":{:?},\"curve2_u\":{:?},\"curve2_length\":{},\"mapped_points\":{:?},\"mapped_u\":{:?},\"mapped_length\":{},\"prepared_points\":{:?},\"prepared_u\":{:?},\"prepared_length\":{},\"segment_values\":{:?},\"arch\":{:?},\"segment\":{:?},\"path_y\":{:?},\"public_280_words\":{:?},\"max_y\":{},\"max_y_gable\":{}}}",
            shape.to_vec(), raw.to_vec(), pairs(&source.points), words(&source.points_u), source.length.to_bits(),
            triples(&wall.geometry.outline), words(&wall.geometry.outline_u), wall.geometry.mapped_curve_length.to_bits(),
            triples(&prepared.positions), words(&prepared.points_u), prepared.length.to_bits(), words(&prepared.segment_values),
            words(wall.arch_segment_heights().unwrap()), words(&wall.segment_heights),
            words(&prepared.positions.iter().map(|p| p[1]).collect::<Vec<_>>()), tail, wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert_rect_multi_original(&wall, &roof, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-multilevel-multi-wood.bin")), 67);
    }

    #[test]
    fn export_real_true_multilevel_flat_publication_for_original_probe() {
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_shape::Roof};
        use country_core::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let curve = Curve2::try_new(vec![[-2.,-1.], [-2.,1.], [2.,1.], [2.,-1.], [-2.,-1.]], false).unwrap();
        let state = WallState::new(curve, 8.0, 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8;28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), false).unwrap();
        wall.prepare_paths(&rasters).unwrap();
        let mut raw = [0u8;88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,0.), (0x34,1.), (0x4c,wall.max_y())] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw);
        wall.update_gable_heights(Some(roof.observed())).unwrap();
        let source = wall.simplified_curve_2d();
        let prepared = wall.prepared_path.as_ref().unwrap();
        let projection = wall.building_projection();
        let bytes = projection.observed_bytes();
        let words = |data: &[f32]| data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let pairs = |data: &[[f32;2]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let triples = |data: &[[f32;3]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let tail: Vec<u32> = bytes[0x280..0x290].chunks_exact(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        println!("NATIVE_GABLE_INPUT={{\"wall_id\":123,\"shape\":{:?},\"roof\":{:?},\"curve2_points\":{:?},\"curve2_u\":{:?},\"curve2_length\":{},\"mapped_points\":{:?},\"mapped_u\":{:?},\"mapped_length\":{},\"prepared_points\":{:?},\"prepared_u\":{:?},\"prepared_length\":{},\"segment_values\":{:?},\"arch\":{:?},\"segment\":{:?},\"path_y\":{:?},\"public_280_words\":{:?},\"max_y\":{},\"max_y_gable\":{}}}",
            shape.to_vec(), raw.to_vec(), pairs(&source.points), words(&source.points_u), source.length.to_bits(),
            triples(&wall.geometry.outline), words(&wall.geometry.outline_u), wall.geometry.mapped_curve_length.to_bits(),
            triples(&prepared.positions), words(&prepared.points_u), prepared.length.to_bits(), words(&prepared.segment_values),
            words(wall.arch_segment_heights().unwrap()), words(&wall.segment_heights),
            words(&prepared.positions.iter().map(|p| p[1]).collect::<Vec<_>>()), tail, wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert_rect_multi_original(&wall, &roof, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-true-multilevel-wood.bin")), 80);
        let mut rim_raw = raw; rim_raw[0x3d] = 1;
        let rim_roof = Roof::from_observed(rim_raw);
        assert_rect_multi_original(&wall, &rim_roof, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-true-multilevel-rim-wood.bin")), 41);
    }

    #[test]
    fn export_real_low_flat_publication_for_original_probe() {
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_shape::Roof};
        use country_core::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let curve = Curve2::try_new(vec![[-2.,-1.], [-2.,1.], [2.,1.], [2.,-1.], [-2.,-1.]], false).unwrap();
        let state = WallState::new(curve, 0.6, 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8;28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), false).unwrap();
        wall.prepare_paths(&rasters).unwrap();
        let mut raw = [0u8;88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,0.), (0x34,1.), (0x4c,wall.max_y())] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw);
        wall.update_gable_heights(Some(roof.observed())).unwrap();
        let source = wall.simplified_curve_2d();
        let prepared = wall.prepared_path.as_ref().unwrap();
        let projection = wall.building_projection();
        let bytes = projection.observed_bytes();
        let words = |data: &[f32]| data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let pairs = |data: &[[f32;2]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let triples = |data: &[[f32;3]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let tail: Vec<u32> = bytes[0x280..0x290].chunks_exact(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        println!("NATIVE_GABLE_INPUT={{\"wall_id\":123,\"shape\":{:?},\"roof\":{:?},\"curve2_points\":{:?},\"curve2_u\":{:?},\"curve2_length\":{},\"mapped_points\":{:?},\"mapped_u\":{:?},\"mapped_length\":{},\"prepared_points\":{:?},\"prepared_u\":{:?},\"prepared_length\":{},\"segment_values\":{:?},\"arch\":{:?},\"segment\":{:?},\"path_y\":{:?},\"public_280_words\":{:?},\"max_y\":{},\"max_y_gable\":{}}}",
            shape.to_vec(), raw.to_vec(), pairs(&source.points), words(&source.points_u), source.length.to_bits(),
            triples(&wall.geometry.outline), words(&wall.geometry.outline_u), wall.geometry.mapped_curve_length.to_bits(),
            triples(&prepared.positions), words(&prepared.points_u), prepared.length.to_bits(), words(&prepared.segment_values),
            words(wall.arch_segment_heights().unwrap()), words(&wall.segment_heights),
            words(&prepared.positions.iter().map(|p| p[1]).collect::<Vec<_>>()), tail, wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert_rect_multi_original(&wall, &roof, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-low-multi-wood.bin")), 52);
    }

    #[test]
    fn export_real_gable_publication_for_original_probe() {
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_shape::Roof};
        use country_core::resources::walls::{WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
        use std::sync::Arc;
        let curve = Curve2::try_new(vec![[-2.,-1.], [-2.,1.], [2.,1.], [2.,-1.], [-2.,-1.]], false).unwrap();
        let state = WallState::new(curve, 2.5, 2, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8;28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        let rasters = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0u8;512*512])), false).unwrap();
        wall.prepare_paths(&rasters).unwrap();
        let mut raw = [0u8;88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,1.), (0x34,1.), (0x4c,wall.max_y())] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw);
        wall.update_gable_heights(Some(roof.observed())).unwrap();
        let source = wall.simplified_curve_2d();
        let prepared = wall.prepared_path.as_ref().unwrap();
        let projection = wall.building_projection();
        let bytes = projection.observed_bytes();
        let words = |data: &[f32]| data.iter().map(|v| v.to_bits()).collect::<Vec<_>>();
        let pairs = |data: &[[f32;2]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let triples = |data: &[[f32;3]]| data.iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>();
        let tail: Vec<u32> = bytes[0x280..0x290].chunks_exact(4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).collect();
        println!("NATIVE_GABLE_INPUT={{\"wall_id\":123,\"shape\":{:?},\"roof\":{:?},\"curve2_points\":{:?},\"curve2_u\":{:?},\"curve2_length\":{},\"mapped_points\":{:?},\"mapped_u\":{:?},\"mapped_length\":{},\"prepared_points\":{:?},\"prepared_u\":{:?},\"prepared_length\":{},\"segment_values\":{:?},\"arch\":{:?},\"segment\":{:?},\"path_y\":{:?},\"public_280_words\":{:?},\"max_y\":{},\"max_y_gable\":{}}}",
            shape.to_vec(), raw.to_vec(), pairs(&source.points), words(&source.points_u), source.length.to_bits(),
            triples(&wall.geometry.outline), words(&wall.geometry.outline_u), wall.geometry.mapped_curve_length.to_bits(),
            triples(&prepared.positions), words(&prepared.points_u), prepared.length.to_bits(), words(&prepared.segment_values),
            words(wall.arch_segment_heights().unwrap()), words(&wall.segment_heights),
            words(&prepared.positions.iter().map(|p| p[1]).collect::<Vec<_>>()), tail, wall.max_y().to_bits(), wall.max_y_gable().to_bits());
        assert_eq!(prepared.positions.len(), wall.segment_heights.len());
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
        use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorage,
            decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
        }
        let mut terrain_data = country_core::resources::terrain_height::TerrainHeightsData::new();
        terrain_data.set_height_range(0., 0.);
        let decorators = DecoratorStorage::default();
        let subtypes = DerivedDecoratorSubtypes::default(); let meshes = DecoratorMeshLibrary::default();
        let path_heights: Vec<_> = prepared.positions.iter().map(|p| p[1]).collect();
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &wall.segment_heights, path_heights: &path_heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-gable-published-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 21);
        for (i, v) in storage.data.iter().take(21).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原真实发布输入山墙木板 {i}");
        }
        use country_core::resources::walls::{decorator::DecoratorInfo, decorator_storage::DecoratorStorageAddr,
            decorator_subtype::{DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey, DecoratorOutline, DecoratorSubtype}};
        let mut decorators = DecoratorStorage::default();
        decorators.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(123))).decorators.insert(42,
            DecoratorInfo { dst_wall: u64::from(1f32.to_bits()) << 32, rank: 1, ty: b'8', ..Default::default() });
        let mut subtypes = DerivedDecoratorSubtypes::default(); subtypes.insert(42, DecoratorSubtype(3));
        let mut meshes = DecoratorMeshLibrary::default(); let mut collision = DecoratorCollision::empty();
        collision.bounds = utils::geometry::Rectangle3d::new(Vec3::new(0.,1.,0.), Vec3::new(1.,2.,1.), Vec2::X);
        meshes.insert_collision_and_interaction_as_same(DecoratorLibKey::new(3,1), collision,
            DecoratorOutline::None, DecoratorCollisionOutline::None);
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &wall.segment_heights, path_heights: &path_heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-gable-published-subtype3-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 24);
        for (i, v) in storage.data.iter().take(24).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原真实发布输入 subtype3 山墙木板 {i}");
        }
        // 相同出版墙体，平顶、两个实际窗桶与两个不同类型的 finalized 洞。
        decorators.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(123))).decorators.insert(43,
            DecoratorInfo { dst_wall: u64::from(4f32.to_bits()) | (u64::from(1f32.to_bits()) << 32),
                rank: 1, ty: b'8', ..Default::default() });
        subtypes.insert(43, DecoratorSubtype(3));
        let mut flat_raw = raw; flat_raw[0x30..0x34].copy_from_slice(&0f32.to_le_bytes());
        let flat_roof = Roof::from_observed(flat_raw);
        let holes = [
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [0.25,0.,0.75,2.], kind: 1, origin: 0 },
            country_core::resources::walls::wall_holes::FinalizedWallHole { aabb: [5.,0.,5.6,2.], kind: 4, origin: 0 },
        ];
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &wall.segment_heights, path_heights: &path_heights, roof: &flat_roof, holes: &holes, demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-multi-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 59);
        for (i, v) in storage.data.iter().take(59).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原矩形双窗双洞木板 {i}");
        }
        // 同一墙窗洞输入改变屋顶版本字节，原版跳过顶饰与细梁。
        flat_raw[0x3d] = 1;
        let rim_roof = Roof::from_observed(flat_raw);
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &wall.segment_heights, path_heights: &path_heights, roof: &rim_roof, holes: &holes, demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-multi-rim-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 26);
        for (i, v) in storage.data.iter().take(26).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原矩形双窗双洞屋顶版本木板 {i}");
        }
    }

    #[test]
    fn circle_boundary_matches_original_three_64_byte_records() {
        let circle = utils::geometry::Circle2d { center: Vec2::ZERO, radius: 2., rotation: 0. };
        let length = f32::from_bits(0x41471c54);
        let right = f32::from_bits(0x3fe38e18);
        let mut actual = Vec::new();
        half_timber_circle_boundary(circle, length, 0., right, f32::from_bits(0x3feb851e), true, 0, true, &[], &mut actual);
        half_timber_circle_boundary(circle, length, 0., right, 0., false, 0, true, &[], &mut actual);
        let native = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-curved.bin"));
        for (index, source_index) in [0usize, 1, 14].into_iter().enumerate() {
            let v = &actual[index];
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = native[source_index*64..(source_index+1)*64].chunks_exact(4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原圆墙曲面第 {source_index} 条");
        }
        let seam = Aabb2 { min: Vec2::new(-0.4, 0.), max: Vec2::new(0.5, 2.) };
        assert_eq!(half_timber_circle_extensions(length, 0.5, right, &[seam]), (0.1, 0., 1));
        assert_eq!(half_timber_circle_extensions(length, 10.666642, 12.044416, &[seam]), (0., 0.1, 2));
        let mut seam_output = Vec::new();
        half_timber_circle_boundary(circle, length, 0.5, right, f32::from_bits(0x3feb851e), true, 0, true, &[seam], &mut seam_output);
        let native = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-seamhole-curved.bin"));
        for (index, v) in seam_output.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = native[index*64..(index+1)*64].chunks_exact(4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原跨缝洞曲面 {index}");
        }
        let hole = Aabb2 { min: Vec2::new(0.5, 0.), max: Vec2::new(1., 2.) };
        assert_eq!(half_timber_circle_extensions(length, 0., 0.5, &[hole]), (0.1, 0.1, 3));
        assert_eq!(half_timber_circle_extensions(length, 1., right, &[hole]), (0.1, 0., 1));
    }

    #[test]
    fn circle_full_entry_matches_original_36_wood_and_21_curved() {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_buffers::WoodenPlankSsbo, roof_shape::Roof};
        use country_core::resources::walls::{WallId, WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::path_preparation::PreparedWallPath;
        use country_core::resources::walls::decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorInfoView, DerivedDecoratorSubtypes};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
        }
        let source = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-source.bin"));
        let read = |offset: usize| f32::from_le_bytes(source[offset..offset+4].try_into().unwrap());
        let coords: Vec<_> = (0..14).map(|i| [read(i*8), read(i*8+4)]).collect();
        let us: Vec<_> = (0..14).map(|i| read(0x100+i*4)).collect();
        let length = f32::from_bits(0x41471c54);
        let curve = Curve2 { points: coords, points_u: us.clone(), length };
        let state = WallState::new(curve, 2.5, 0, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8; 28]; shape[12..16].copy_from_slice(&2f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        wall.prepared_path = Some(PreparedWallPath { positions: (0..14).map(|i|
            [read(0x200+i*12), read(0x204+i*12), read(0x208+i*12)]).collect(),
            points_u: us, length, segment_values: vec![0.; 14] });
        wall.regular_heights.as_mut().expect("原圆墙高度缓存缺失").maximum_y = 2.5;
        let mut terrain_data = country_core::resources::terrain_height::TerrainHeightsData::new();
        terrain_data.set_height_range(0., 0.);
        let decorators = country_core::resources::walls::decorator_storage::DecoratorStorage::default();
        let subtypes = DerivedDecoratorSubtypes::default(); let meshes = DecoratorMeshLibrary::default();
        let mut raw = [0u8; 88]; raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c,0.5f32), (0x30,0.), (0x34,1.), (0x4c,2.5)] {
            raw[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = Roof::from_observed(raw); let heights = [0.;14];
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 36);
        for (i, v) in storage.data.iter().take(36).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原圆墙木板 {i}");
        }
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-curved.bin"));
        assert_eq!(curved.len(), 21);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原圆墙曲面 {i}");
        }
        // 原入口确用真实两层装饰 hash 桶及 subtype 表，不注入窗口投影。
        use country_core::resources::walls::{decorator::DecoratorInfo,
            decorator_storage::DecoratorStorageAddr, decorator_subtype::DecoratorSubtype};
        let mut decorators = country_core::resources::walls::decorator_storage::DecoratorStorage::default();
        let dst = u64::from(0.2f32.to_bits()) | (u64::from(1f32.to_bits()) << 32);
        decorators.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(123))).decorators.insert(42,
            DecoratorInfo { dst_wall: dst, rank: 1, ty: b'9', ..Default::default() });
        let mut subtypes = DerivedDecoratorSubtypes::default(); subtypes.insert(42, DecoratorSubtype(9));
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: DerivedDecoratorInfoView::new(&subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-gothic-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 34);
        for (i, v) in storage.data.iter().take(34).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原哥特窗圆墙木板 {i}");
        }
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-gothic-curved.bin"));
        assert_eq!(curved.len(), 21);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原哥特窗圆墙曲面 {i}");
        }
        let hole = country_core::resources::walls::wall_holes::FinalizedWallHole {
            aabb: [-0.4, 0., 0.5, 2.], kind: 1, origin: 0,
        };
        let empty_decorators = country_core::resources::walls::decorator_storage::DecoratorStorage::default();
        let empty_subtypes = DerivedDecoratorSubtypes::default();
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &empty_decorators,
            derived: DerivedDecoratorInfoView::new(&empty_subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[hole], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-seamhole-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 33);
        for (i, v) in storage.data.iter().take(33).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原跨缝洞圆墙木板 {i}");
        }
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-seamhole-curved.bin"));
        assert_eq!(curved.len(), 21);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原跨缝洞圆墙曲面 {i}");
        }
        // 写入资源耗尽时仍执行其余原业务及曲面输出。
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &empty_decorators,
            derived: DerivedDecoratorInfoView::new(&empty_subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        library.get_mut::<WoodenPlankSsbo>().unwrap().free_chunks.write().clear();
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert_eq!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 0);
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-curved.bin"));
        assert_eq!(curved.len(), 21);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "耗尽后的原圆墙曲面 {i}");
        }
        // 仅留一个 32 槽块：成功前缀保留，后续失败不能终止曲面业务。
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &empty_decorators,
            derived: DerivedDecoratorInfoView::new(&empty_subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        library.get_mut::<WoodenPlankSsbo>().unwrap().free_chunks.write().truncate(1);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let runs = storage.to_draw_multi_instanced();
        assert_eq!(runs.iter().map(|r| r.1 as usize).sum::<usize>(), 32);
        let native_wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-wood.bin"));
        for (i, v) in storage.data[runs[0].0 as usize..runs[0].0 as usize + 32].iter().enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = native_wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "资源耗尽前原圆墙木板 {i}");
        }
        assert_eq!(curved.len(), 21);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "部分耗尽后的原圆墙曲面 {i}");
        }
        // 原 roof+3d 非零路径：不生成顶饰细梁，但保留曲面。
        raw[0x3d] = 1;
        let rim_roof = Roof::from_observed(raw);
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &empty_decorators,
            derived: DerivedDecoratorInfoView::new(&empty_subtypes, &meshes), terrain: &terrain_data,
            segment_heights: &heights, path_heights: &heights, roof: &rim_roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let wood = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-rim-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 7);
        for (i, v) in storage.data.iter().take(7).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = wood[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原圆墙屋顶版本木板 {i}");
        }
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-rim-curved.bin"));
        assert_eq!(curved.len(), 14);
        for (i, v) in curved.iter().enumerate() {
            let words = [v.position.x.to_bits(), v.position.y.to_bits(), v.position.z.to_bits(), v.pad_0c,
                v.x0.to_bits(), v.x1.to_bits(), v.y0.to_bits(), v.y1.to_bits(), v.w20, v.w24,
                v.w28.to_bits(), v.w2c, v.w30, v.w34, v.w38, v.w3c];
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words.as_slice(), expected.as_slice(), "原圆墙屋顶版本曲面 {i}");
        }
    }

    #[test]
    fn area_boundaries_follow_original_closure_order() {
        let areas = [Aabb2 { min: Vec2::new(0., 0.), max: Vec2::new(2., 1.84) },
            Aabb2 { min: Vec2::new(2., 1.84), max: Vec2::new(6., 3.) }];
        assert_eq!(half_timber_area_boundaries(&areas, 0.),
            [(0, 1.84, 1.), (0, 0., 1.), (1, 3., 4.)]);
        assert!(half_timber_boundary_enabled(true, 2.5, 1.84));
        assert!(!half_timber_boundary_enabled(true, 2.5, 2.5));
        assert!(half_timber_boundary_enabled(false, 2.5, 2.0));
        assert!(!half_timber_boundary_enabled(false, 2.5, 2.5));
        let top = f32::from_bits(0x3feb851e);
        let original_areas: Vec<_> = [(0., 2.), (2., 6.), (6., 8.), (8., 12.)].into_iter()
            .map(|(left, right)| Aabb2 { min: Vec2::new(left, 0.), max: Vec2::new(right, top) }).collect();
        let observed = half_timber_area_boundaries(&original_areas, 0.);
        let expected = [(0, top, 1.), (0, 0., 1.), (1, top, 4.), (1, 0., 4.),
            (2, top, 7.), (2, 0., 7.), (3, top, 10.), (3, 0., 10.)];
        assert_eq!(observed, expected);
        let rectangle = utils::geometry::Rectangle2d { center: Vec2::ZERO, axis: Vec2::X, size: Vec2::new(4., 2.) };
        for (center, expected_range) in [(1., [0.,2.]), (4., [2.,6.]), (7., [6.,8.]), (10., [8.,12.])] {
            let edge = rectangle.get_edge_id_at_u(center / 12.);
            let actual = [rectangle.get_point_u(edge) * 12., rectangle.get_point_u(edge + 1) * 12.];
            assert_eq!(actual.map(f32::to_bits), expected_range.map(f32::to_bits));
            for height in [top, 0.] {
                let hole = Aabb2 { min: Vec2::new(0.5, 0.), max: Vec2::new(1.5, 2.) };
                let clipped = half_timber_boundary_clipping(actual[0], actual[1], 12., height, top, &[hole], &[], &[]);
                let pieces: Vec<_> = clipped.result().iter().map(|v| [v.start.to_bits(), v.end.to_bits()]).collect();
                let expected = if center == 1. { vec![[0f32.to_bits(), 0.5f32.to_bits()], [1.5f32.to_bits(), 2f32.to_bits()]] }
                    else { vec![expected_range.map(f32::to_bits)] };
                assert_eq!(pieces, expected);
            }
        }
        let window = WindowAccessInfo { aabb: [0.5, 0., 1.5, 2.], leaf: 0, subtype: 0, shifted: 0 };
        let top_result = half_timber_boundary_clipping(0., 2., 12., top, top, &[], &[window], &[]);
        let floor_result = half_timber_boundary_clipping(0., 2., 12., 0., top, &[], &[window], &[]);
        assert_eq!(top_result.result().iter().map(|v| (v.start, v.end)).collect::<Vec<_>>(), [(0., 2.)]);
        assert_eq!(floor_result.result().iter().map(|v| (v.start, v.end)).collect::<Vec<_>>(), [(0., 0.5), (1.5, 2.)]);
        let gothic = WindowAccessInfo { leaf: 1, subtype: 6, ..window };
        let gothic_floor = half_timber_boundary_clipping(0., 2., 12., 0., top, &[], &[gothic], &[]);
        let gothic_bits: Vec<_> = gothic_floor.result().iter().map(|v| [v.start.to_bits(), v.end.to_bits()]).collect();
        assert_eq!(gothic_bits, [[0, 0x3f051eb8], [0x3fbd70a4, 0x40000000]]);
        let door = WindowAccessInfo { subtype: 9, ..gothic };
        let door_floor = half_timber_boundary_clipping(0., 2., 12., 0., top, &[], &[door], &[]);
        let door_bits: Vec<_> = door_floor.result().iter().map(|v| [v.start.to_bits(), v.end.to_bits()]).collect();
        assert_eq!(door_bits, [[0, 0x3ea3d70a], [0x3fd70a3e, 0x40000000]]);
    }

    #[test]
    fn gable_positions_match_original_six_area_checkpoint() {
        let bytes = *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-gable-roof.bin"));
        let roof = country_core::resources::roofs::roof_shape::Roof::from_observed(bytes);
        let cases = [
            (0., 0., 2., 2.5, vec![0.], vec![0., 2.]),
            (0., 2.5, 2., 3.5, vec![0.], vec![]),
            (2., 0., 6., 2.5, vec![2., 4.], vec![2., 4., 6.]),
            (6., 0., 8., 2.5, vec![6.], vec![6., 8.]),
            (6., 2.5, 8., 3.5, vec![6.], vec![]),
            (8., 0., 12., 2.5, vec![8., 10.], vec![8., 10., 12.]),
        ];
        for (left, bottom, right, top, mut positions, expected) in cases {
            let area = Aabb2 { min: Vec2::new(left, bottom), max: Vec2::new(right, top) };
            half_timber_filter_gable_positions(&mut positions, area, 2.5, &roof);
            if !(2.5 < top) { positions.push(right); }
            assert_eq!(positions, expected);
        }
        let bytes = *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-gable-wide-roof.bin"));
        let roof = country_core::resources::roofs::roof_shape::Roof::from_observed(bytes);
        for (left, right, positions, expected) in [(0., 4., vec![0., 2.], vec![2.]),
            (12., 16., vec![12., 14.], vec![14.])] {
            for top in [4.5, 6.5] {
                let mut positions = positions.clone();
                half_timber_filter_gable_positions(&mut positions,
                    Aabb2 { min: Vec2::new(left, 2.5), max: Vec2::new(right, top) }, 2.5, &roof);
                assert_eq!(positions, expected);
            }
        }
        let (mut left, mut right) = (0., 4.);
        assert!(half_timber_clip_high_boundary(&mut left, &mut right, 4.5, 2.5, &roof));
        assert_eq!([left.to_bits(), right.to_bits()], [0x3eb77774, 0x40691112]);
        let (mut left, mut right) = (12., 16.);
        assert!(half_timber_clip_high_boundary(&mut left, &mut right, 4.5, 2.5, &roof));
        assert_eq!([left.to_bits(), right.to_bits()], [0x4145bbbc, 0x417a4444]);
        let mut circle_roof = bytes;
        circle_roof[8..12].copy_from_slice(&0u32.to_le_bytes());
        let circle_roof = country_core::resources::roofs::roof_shape::Roof::from_observed(circle_roof);
        assert!(std::panic::catch_unwind(|| {
            let (mut left, mut right) = (0., 4.);
            half_timber_clip_high_boundary(&mut left, &mut right, 4.5, 2.5, &circle_roof)
        }).is_err());
    }

    #[test]
    fn pair_rng_first_draw_matches_original_entry() {
        let cases = [(0, 0., 2., 143u64, 0x3e399128), (1, 2., 4., 184, 0x3f601a2c),
            (1, 4., 6., 224, 0x3f6c811c), (2, 6., 8., 265, 0x3f48e6ba),
            (3, 8., 10., 306, 0x3f0f4e00), (3, 10., 12., 346, 0x3f47bee6)];
        for (area, left, right, seed, bits) in cases {
            assert_eq!(utils::rng::rng_from_seed_u64(seed).f32().to_bits(), bits);
            let result = half_timber_pair_rng(country_core::resources::walls::WallId(123),
                area, left, right, f32::from_bits(0x3feb851e));
            assert_eq!(result.is_some(), seed == 143);
            if let Some(rng) = result { assert_eq!(rng.f32().to_bits(), 0x3f2089a4); }
        }
    }

    #[test]
    fn rectangle_fine_points_match_original_pre_sort_checkpoint() {
        let z = f32::from_bits(0x3f921a18);
        let y = f32::from_bits(0x3feb851e);
        let rng = utils::rng::rng_from_seed_u64(123);
        let points = half_timber_rectangle_fine_points(0., 2., Vec3::new(-2., y, -z),
            Vec3::new(-2., y, z), Vec2::new(-1., 0.), &rng);
        let actual: Vec<[u32; 6]> = points.iter().map(|(u, p, n)|
            [u.to_bits(), p.x.to_bits(), p.y.to_bits(), p.z.to_bits(), n.x.to_bits(), n.y.to_bits()]).collect();
        assert_eq!(actual, [
            [0,0xbfffffff,0x3feb851e,0xbf95fe93,0xbf800000,0],
            [0x3ecccccd,0xc0000000,0x3feb851e,0xbf39169c,0xbf800000,0],
            [0x3f4ccccd,0xc0000000,0x3feb851e,0xbe624a10,0xbf800000,0],
            [0x3f99999a,0xc0000000,0x3feb851e,0x3e8ae190,0xbf800000,0],
            [0x3fcccccd,0xc0000000,0x3feb851e,0x3f2b33ca,0xbf800000,0],
            [0x40000000,0xc0000000,0x3feb851e,0x3f98347c,0xbf800000,0],
        ]);
    }

    #[test]
    fn vertical_plank_matches_original_entry_first_record() {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::{geometry::surface::Curve2, roof_buffers::WoodenPlankSsbo};
        use country_core::resources::walls::{WallId, WallState, public_wall_state::PublicWallState,
            publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> { unreachable!() }
        }
        let curve = Curve2::try_new(vec![[-2.,-1.], [-2.,1.], [2.,1.], [2.,-1.], [-2.,-1.]], false).unwrap();
        let state = WallState::new(curve, 2.5, 0, 0, false);
        let mut snapshot = state.into_snapshot().0;
        let mut shape = [0u8; 28];
        shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (i, v) in [1f32, 0., 0., 0., 4., 2.].into_iter().enumerate() {
            shape[4+i*4..8+i*4].copy_from_slice(&v.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let state = WallState::from_snapshot(snapshot, 0);
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0., maximum_y: 0., sampling_fix: false };
        let mut wall = PublicWallState::collect(&state, &terrain).unwrap();
        // 原完整入口样本的 prepared 曲线就是五个闭合点；普通 collect 会重新采样。
        wall.prepared_path = Some(country_core::systems::wall::wall_state::path_preparation::PreparedWallPath {
            positions: vec![[-2.,0.,-1.], [-2.,0.,1.], [2.,0.,1.], [2.,0.,-1.], [-2.,0.,-1.]],
            points_u: vec![0., 1. / 6., 0.5, 2. / 3., 1.], length: 12., segment_values: vec![0.; 5],
        });
        wall.regular_heights.as_mut().expect("原矩形墙高度缓存缺失").maximum_y = 2.5;
        let mut terrain_data = country_core::resources::terrain_height::TerrainHeightsData::new();
        terrain_data.set_height_range(0., 0.);
        let decorators = country_core::resources::walls::decorator_storage::DecoratorStorage::default();
        let subtypes = country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes::default();
        let meshes = country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::default();
        let derived = country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let mut roof_raw = [0u8; 88];
        roof_raw[8..36].copy_from_slice(&shape);
        for (offset, value) in [(0x2c, 0.5f32), (0x30, 0.), (0x34, 1.), (0x4c, 2.5)] {
            roof_raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        let roof = country_core::resources::roofs::roof_shape::Roof::from_observed(roof_raw);
        let heights = [0.; 5];
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived, terrain: &terrain_data, segment_heights: &heights, path_heights: &heights,
            roof: &roof, holes: &[], demo_mode: false };
        let mut buffer = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&Device, None);
        let allocation = buffer.new_allocation();
        let mut failed = false;
        {
            let mut writer = buffer.writer(&allocation);
            half_timber_vertical_plank(&wall, WallId(123), Aabb2 { min: Vec2::ZERO,
                max: Vec2::new(2., f32::from_bits(0x3feb851e)) }, 0., 0, &mut writer, &mut failed);
            half_timber_diagonal_plank(&wall, WallId(123), Aabb2 { min: Vec2::ZERO,
                max: Vec2::new(2., f32::from_bits(0x3feb851e)) }, 0., 2., 0,
                f32::from_bits(0x3f2089a4), &mut writer, &mut failed);
            half_timber_rectangle_boundary_plank(&wall, WallId(123), 0., 2.,
                f32::from_bits(0x3feb851e), true, 0, &mut writer, &mut failed);
            let (p, q, normal) = half_timber_rectangle_corner_adjusted_endpoints(&wall,
                wall.is_rectangle().unwrap(), 0., 2., f32::from_bits(0x3feb851e));
            half_timber_rectangle_top_plank(WallId(123), p, q, normal, 1, &mut writer, &mut failed);
            let rng = utils::rng::rng_from_seed_u64(123);
            let mut points = half_timber_rectangle_fine_points(0., 2., p, q, normal, &rng);
            half_timber_write_fine_points(&mut points, WallId(123), 1, f32::from_bits(0x3feb851e), &rng, &mut writer, &mut failed);
            half_timber_rectangle_corners(&input, wall.is_rectangle().unwrap(), &[], &mut writer, &mut failed);
        }
        assert!(!failed);
        let ranges = buffer.allocation_to_draw_multi_instanced(&allocation);
        assert_eq!(ranges.iter().map(|r| r.1).sum::<u32>(), 14);
        let record = &buffer.data[ranges[0].0 as usize];
        let words: Vec<u32> = record.transform.cols.iter()
            .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
            .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-oracle-flat-v2-wood.bin"));
        let expected: Vec<u32> = original[..64].chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
        assert_eq!(words, expected);
        let record = &buffer.data[ranges[0].0 as usize + 1];
        let words: Vec<u32> = record.transform.cols.iter()
            .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
            .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
        let expected: Vec<u32> = original[64..128].chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
        assert_eq!(words, expected, "原斜梁 64 字节");
        let record = &buffer.data[ranges[0].0 as usize + 2];
        let words: Vec<u32> = record.transform.cols.iter()
            .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
            .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
        let expected: Vec<u32> = original[7*64..8*64].chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
        assert_eq!(words, expected, "原边界横梁 64 字节");
        let record = &buffer.data[ranges[0].0 as usize + 3];
        let words: Vec<u32> = record.transform.cols.iter()
            .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
            .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
        let expected: Vec<u32> = original[8*64..9*64].chunks_exact(4)
            .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
        assert_eq!(words, expected, "原顶层装饰横梁 64 字节");
        for i in 0..6 {
            let record = &buffer.data[ranges[0].0 as usize + 4 + i];
            let words: Vec<u32> = record.transform.cols.iter()
                .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
            let expected: Vec<u32> = original[(9+i)*64..(10+i)*64].chunks_exact(4)
                .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原细梁第 {i} 条 64 字节");
        }
        for i in 0..4 {
            let record = &buffer.data[ranges[0].0 as usize + 10 + i];
            let words: Vec<u32> = record.transform.cols.iter()
                .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
            let expected: Vec<u32> = original[(53+i)*64..(54+i)*64].chunks_exact(4)
                .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原角柱第 {i} 条 64 字节");
        }
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new();
        half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 57);
        for (i, record) in storage.data.iter().take(57).enumerate() {
            let words: Vec<u32> = record.transform.cols.iter()
                .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4)
                .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原半木完整入口第 {i} 条 64 字节");
        }
        // 此原探针直接投影非零 path 数组，尚未证明来自完整上游 producer。
        let segment_heights = [1., 2., 3., 1., 0.];
        let path_heights = [0., 0.2, -0.1, 0.3, 0.];
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView::new(&subtypes, &meshes),
            terrain: &terrain_data, segment_heights: &segment_heights, path_heights: &path_heights,
            roof: &roof, holes: &[], demo_mode: false };
        let mut library = country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::new_storage_buffer(&Device, None));
        let mut output = crate::gpu_output::ConstructorGpuOutput::default();
        output.init_half_timber_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), 123);
        let mut curved = Vec::new(); half_timbering(input, &mut library, &output, &mut curved);
        assert!(curved.is_empty());
        let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-nonzero-flags-wood.bin"));
        assert_eq!(storage.to_draw_multi_instanced().iter().map(|r| r.1 as usize).sum::<usize>(), 29);
        for (i, v) in storage.data.iter().take(29).enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[i*64..(i+1)*64].chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原非零高度半木第 {i} 条 64 字节");
        }
        use country_core::resources::walls::{decorator::DecoratorInfo, decorator_storage::DecoratorStorageAddr,
            decorator_subtype::{DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey, DecoratorOutline, DecoratorSubtype}};
        let mut decorators = country_core::resources::walls::decorator_storage::DecoratorStorage::default();
        decorators.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(123))).decorators.insert(42,
            DecoratorInfo { dst_wall: u64::from(1f32.to_bits()) << 32, rank: 1, ty: b'8', ..Default::default() });
        let mut subtypes = country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes::default();
        subtypes.insert(42, DecoratorSubtype(3));
        let mut meshes = country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::default();
        let mut collision = DecoratorCollision::empty();
        collision.bounds = utils::geometry::Rectangle3d::new(Vec3::new(0.,1.,0.), Vec3::new(1.,2.,1.), Vec2::X);
        meshes.insert_collision_and_interaction_as_same(DecoratorLibKey::new(3,1), collision,
            DecoratorOutline::None, DecoratorCollisionOutline::None);
        let roof = country_core::resources::roofs::roof_shape::Roof::from_observed(
            *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-gable-roof.bin")));
        let input = HalfTimberInput { wall: &wall, wall_id: WallId(123), decorators: &decorators,
            derived: country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoView::new(&subtypes, &meshes),
            terrain: &terrain_data, segment_heights: &heights, path_heights: &heights, roof: &roof, holes: &[], demo_mode: false };
        let mut buffer = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&Device, None);
        let allocation = buffer.new_allocation();
        {
            let mut writer = buffer.writer(&allocation);
            half_timber_rectangle_corners(&input, wall.is_rectangle().unwrap(), &[], &mut writer, &mut failed);
        }
        let ranges = buffer.allocation_to_draw_multi_instanced(&allocation);
        assert_eq!(ranges.iter().map(|r| r.1).sum::<u32>(), 8);
        let original = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-rect-gable-subtype3-wood.bin"));
        for (i, v) in buffer.data[ranges[0].0 as usize..ranges[0].0 as usize + 8].iter().enumerate() {
            let words: Vec<u32> = v.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([v.gate_seed as u32, v.piece_seed as u32, v.wall_id, v.flags]).collect();
            let expected: Vec<u32> = original[(13+i)*64..(14+i)*64].chunks_exact(4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
            assert_eq!(words, expected, "原 subtype3 角点木板 {i}");
        }
    }

    #[test]
    fn actual_decorator_bucket_supplies_gothic_window_and_rank() {
        use country_core::resources::walls::{WallId,
            decorator::DecoratorInfo,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        let wall = WallId(17);
        let mut storage = DecoratorStorage::default();
        let u = 3.0f32.to_bits() as u64;
        let y = (2.0f32.to_bits() as u64) << 32;
        let info = DecoratorInfo { dst_wall: u | y, rank: 1, ty: b'9', ..Default::default() };
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).decorators.insert(42, info);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(42, DecoratorSubtype(9));
        let library = DecoratorMeshLibrary::default();
        let view = DerivedDecoratorInfoView::new(&subtypes, &library);
        let query = Aabb2 { min: Vec2::new(2.0, 0.0), max: Vec2::new(4.0, 4.0) };
        let result = overlapping_windows_from_storage(query, 12.0, wall, &storage, &view);
        assert_eq!(result.len(), 1);
        assert_eq!((result[0].leaf, result[0].subtype, result[0].shifted), (1, 9, 0));
        assert!(result[0].aabb[0] < 3.0 && result[0].aabb[2] > 3.0);
    }

    #[test]
    fn actual_window_uses_collision_bbox_from_the_derived_owner() {
        use country_core::resources::walls::{WallId, decorator::DecoratorInfo,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey,
                DecoratorMeshLibrary, DecoratorOutline, DecoratorSubtype, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        let wall = WallId(19);
        let mut storage = DecoratorStorage::default();
        let coord = u64::from(3.0f32.to_bits()) | (u64::from(2.0f32.to_bits()) << 32);
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall)).decorators.insert(7,
            DecoratorInfo { dst_wall: coord, rank: 1, ty: b'8', ..Default::default() });
        let mut subtypes = DerivedDecoratorSubtypes::default(); subtypes.insert(7, DecoratorSubtype(2));
        let mut library = DecoratorMeshLibrary::default();
        let mut collision = DecoratorCollision::empty();
        collision.bounds = utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::new(2.0, 1.0, 1.0), Vec2::X);
        library.insert_collision_and_interaction_as_same(DecoratorLibKey::new(2, 1), collision,
            DecoratorOutline::None, DecoratorCollisionOutline::None);
        let view = DerivedDecoratorInfoView::new(&subtypes, &library);
        let query = Aabb2 { min: Vec2::new(1.0, 0.0), max: Vec2::new(5.0, 4.0) };
        let result = overlapping_windows_from_storage(query, 12.0, wall, &storage, &view);
        assert_eq!(result.len(), 1);
        assert_eq!((result[0].leaf, result[0].subtype, result[0].shifted), (1, 2, 0));
        assert!((result[0].aabb[2] - result[0].aabb[0] - 2.124).abs() < 1e-6);
        assert!((result[0].aabb[3] - result[0].aabb[1] - 1.2).abs() < 1e-6);
    }

    #[test]
    fn half_timber_initial_splits_match_original_entry_checkpoint() {
        use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorage,
            decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorInfoView, DerivedDecoratorSubtypes}};
        let top = f32::from_bits(0x3feb851e);
        let storage = DecoratorStorage::default();
        let subtypes = DerivedDecoratorSubtypes::default();
        let library = DecoratorMeshLibrary::default();
        let view = DerivedDecoratorInfoView::new(&subtypes, &library);
        for (left, right, expected) in [
            (0.0, 2.0, vec![0.0]), (2.0, 6.0, vec![2.0, 4.0]),
            (6.0, 8.0, vec![6.0]), (8.0, 12.0, vec![8.0, 10.0]),
        ] {
            let area = Aabb2 { min: Vec2::new(left, 0.0), max: Vec2::new(right, top) };
            let actual = half_timber_initial_splits(area, false);
            assert_eq!(actual.len(), expected.len());
            for (record, value) in actual.iter().zip(expected) {
                assert_eq!([record.position.to_bits(), record.left.to_bits(), record.right.to_bits()],
                    [f32::to_bits(value); 3]);
            }
            // 空墙桶仍调用真实查询，再按原入口补区域尾端。
            let mut positions = half_timber_split_positions(area, &actual, &[], &[], 12.0, WallId(123), &storage, &view);
            positions.push(right);
            let expected_final = match left as i32 {
                0 => vec![0.0, 2.0], 2 => vec![2.0, 4.0, 6.0],
                6 => vec![6.0, 8.0], 8 => vec![8.0, 10.0, 12.0],
                _ => unreachable!(),
            };
            assert_eq!(positions, expected_final);
        }
        let length = f32::from_bits(0x41471c54);
        let area = Aabb2 { min: Vec2::ZERO, max: Vec2::new(length, top) };
        let hole = Aabb2 { min: Vec2::new(-0.4, 0.), max: Vec2::new(0.5, 2.) };
        let splits = half_timber_initial_splits(area, true);
        let mut positions = half_timber_split_positions(area, &splits, &[], &[hole], length, WallId(123), &storage, &view);
        positions.push(length);
        assert_eq!(positions.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            [0x3fe38e18, 0x40638e18, 0x40aaaa91, 0x40e38e18, 0x410e38cf, 0x412aaa91, 0x41471c54]);
    }

    #[test]
    fn half_timber_nonzero_height_spans_match_original_checkpoint() {
        let curve = utils::curve::Curve2::try_new(vec![
            Vec2::new(-2.0, -1.0), Vec2::new(-2.0, 1.0), Vec2::new(2.0, 1.0),
            Vec2::new(2.0, -1.0), Vec2::new(-2.0, -1.0),
        ], false).unwrap();
        let mut flags = vec![0u8; 2];
        let mut spans = vec![Vec::new(), Vec::new()];
        half_timber_height_spans(&curve, &[0.0, f32::from_bits(0x3feb851e)],
            &[1.0, 2.0, 3.0, 1.0, 0.0], &[0.0, 0.2, -0.1, 0.3, 0.0],
            &mut flags, &mut spans);
        assert_eq!(flags, [0, 0]);
        assert_eq!(spans[0].iter().map(|s| (s.start.to_bits(), s.end.to_bits(), s.tag)).collect::<Vec<_>>(),
            [(0, 0x413b13b1, 0)]);
        assert_eq!(spans[1].iter().map(|s| (s.start.to_bits(), s.end.to_bits(), s.tag)).collect::<Vec<_>>(),
            [(0x3fc4ec50, 0x40e27628, 0)]);
        // 同一原入口非零探针在四区域补尾端前的每点活跃标志。
        assert_eq!([0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0]
            .map(|position| half_timber_span_contains(&spans[0], position)),
            [true, true, true, true, true, true, false]);
        let expected: [&[(u32, u32)]; 8] = [
            &[(0, 0x3fc4ec50)], &[], &[], &[], &[(0x40e27628, 0x41000000)], &[],
            &[(0x41000000, 0x41400000)], &[(0x413b13b1, 0x41400000)],
        ];
        for (i, (left, right)) in [(0., 2.), (2., 6.), (6., 8.), (8., 12.)].into_iter().enumerate() {
            for (side, level) in [1usize, 0].into_iter().enumerate() {
                let mut clipping = utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range(left, right);
                for span in &spans[level] {
                    clipping.subtract(utils::subtractive_1d_boolean::Interval {
                        start: span.start, end: span.end, upper_exclusive: span.tag != 0,
                    });
                }
                let actual: Vec<_> = clipping.result().iter().map(|v| (v.start.to_bits(), v.end.to_bits())).collect();
                assert_eq!(actual.as_slice(), expected[i * 2 + side]);
            }
        }
    }

    #[test]
    fn half_timber_rectangle_areas_match_original_gable_closure() {
        use country_core::resources::walls::{InnerWalls, WallState,
            publication_source::TerrainHeightGrid512,
            public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash}};
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [12.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Rectangle(
            utils::geometry::Rectangle2d { center: Vec2::ZERO, axis: Vec2::X, size: Vec2::new(4.0, 2.0) }
        )).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let wall = walls.get_mut(id).unwrap();
        wall.regular_heights.as_mut().unwrap().maximum_y = 3.0;
        let areas = half_timber_areas(wall, &[0.0, 2.0, 5.0], Some([0, 2]));
        let expected = [
            [0.0,0.0,2.0,2.0], [0.0,2.0,2.0,5.0],
            [2.0,0.0,6.0,2.0], [2.0,2.0,6.0,3.0],
            [6.0,0.0,8.0,2.0], [6.0,2.0,8.0,5.0],
            [8.0,0.0,12.0,2.0], [8.0,2.0,12.0,3.0],
        ];
        let actual: Vec<[u32; 4]> = areas.iter().map(|a|
            [a.min.x.to_bits(), a.min.y.to_bits(), a.max.x.to_bits(), a.max.y.to_bits()]).collect();
        assert_eq!(actual, expected.map(|a| a.map(f32::to_bits)));
    }

    #[test]
    fn underwindow_writes_three_real_plank_records() {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice};
        use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::resources::walls::{WallId, WallState, public_wall_state::PublicWallState,
            publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        struct Device;
        impl GraphicsDevice for Device {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { id: 6, byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[std::ops::Range<usize>], _: usize) -> Result<(), BackendError> {
                Err(BackendError::invalid_usage("仅 CPU 影子夹具"))
            }
        }
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let authority = WallState::new(curve, 4.0, 0, 0, false);
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let wall = PublicWallState::collect(&authority, &grid).unwrap();
        let shape = ShapeParameters::from_shape(Shape::Rectangle(utils::geometry::Rectangle2d {
            center: Vec2::ZERO, axis: Vec2::X, size: Vec2::new(4.0, 2.0),
        }));
        let mut buffer = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&Device, None);
        let allocation = buffer.new_allocation();
        let window = WindowAccessInfo { aabb: [1.0, 1.0, 3.0, 2.0], leaf: 0, subtype: 0, shifted: 0 };
        let base = Aabb2 { min: Vec2::ZERO, max: Vec2::ONE };
        let mut generated = false;
        let mut failed = false;
        {
            let mut writer = buffer.writer(&allocation);
            add_underwindow_pattern(&window, &base, 1.0, 3.0, &mut generated, &wall, &shape,
                &mut writer, &mut failed, WallId(17), 4, 0.0);
        }
        assert!(generated && !failed);
        let ranges = buffer.allocation_to_draw_multi_instanced(&allocation);
        assert_eq!(ranges.iter().map(|r| r.1).sum::<u32>(), 3);
        for record in &buffer.data[ranges[0].0 as usize..ranges[0].0 as usize + 3] {
            assert_eq!((record.gate_seed, record.piece_seed, record.wall_id, record.flags), (0, 21, 17, 0));
            assert!(record.transform.cols.iter().all(|v| v.is_finite()));
        }
        let original = [
            [0xb299999a,0,0x3e19999b,0xbde25e19,0xbdcfad3c,0,0x3f800001,0xbf8b8520,0xb43d572c,0x3fcccccc,0x3f15c28f,0x3e99999a,0,21,17,0],
            [0x3299999a,0x3199999a,0x3e199999,0x3de25e15,0xbdcfad35,0x3199999a,0x3f800000,0x3f8b851f,0x343d572d,0x402ccccd,0x3f15c28f,0,0,21,17,0],
            [0x3219999a,0,0x3e199999,0,0xbe199999,0,0x3fffffff,0,0x34000000,0x400d9392,0x3f90a3d7,0x3e593924,0,21,17,0],
        ];
        for (index, record) in buffer.data[ranges[0].0 as usize..ranges[0].0 as usize + 3].iter().enumerate() {
            let words: Vec<u32> = record.transform.cols.iter()
                .flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags])
                .collect();
            assert_eq!(words, original[index], "第 {index} 根原木梁");
        }
        // 原圆形探针：直线端点与圆弧中点偏差超过门限，三段均跳过。
        let circle = ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {
            center: Vec2::ZERO, radius: 2.0, rotation: 0.0,
        }));
        generated = false;
        {
            let mut writer = buffer.writer(&allocation);
            writer.clear();
            add_underwindow_pattern(&window, &base, 1.0, 3.0, &mut generated, &wall, &circle,
                &mut writer, &mut failed, WallId(17), 4, 0.0);
        }
        assert!(generated && !failed);
        assert!(buffer.allocation_to_draw_multi_instanced(&allocation).is_empty());
        // 原圆墙曲线的短区间通过曲率门，比较三条完整记录。
        let source = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m06-half-main-circle-source.bin"));
        let read = |offset: usize| f32::from_le_bytes(source[offset..offset+4].try_into().unwrap());
        let curve = Curve2 { points: (0..14).map(|i| [read(i*8), read(i*8+4)]).collect(),
            points_u: (0..14).map(|i| read(0x100+i*4)).collect(), length: f32::from_bits(0x41471c54) };
        let circle_wall = PublicWallState::collect(&WallState::new(curve, 4.0, 0, 0, false), &grid).unwrap();
        let mut generated = false;
        {
            let mut writer = buffer.writer(&allocation); writer.clear();
            add_underwindow_pattern(&window, &base, 1.0, 1.8, &mut generated, &circle_wall, &circle,
                &mut writer, &mut failed, WallId(17), 4, 0.0);
        }
        assert!(generated && !failed);
        let ranges = buffer.allocation_to_draw_multi_instanced(&allocation);
        assert_eq!(ranges.iter().map(|r| r.1).sum::<u32>(), 3);
        let original = [
            [0x3de5f14e,0,0xbdcbb612,0x3dbf3da0,0xbd53aa61,0x3dd7ddc7,0xbe87ceb5,0xbf8b8520,0xbe994b89,0x3fe4019c,0x3f15c28f,0xbfaf3d6e,0,21,17,0],
            [0x3de5f148,0x3199999a,0xbdcbb619,0xbdbf3da6,0xbd53aa6a,0xbdd7ddc3,0xbe87cebe,0x3f8b8520,0xbe994b8e,0x3fd0835c,0x3f15c28f,0xbfc55962,0,21,17,0],
            [0x3de5f14a,0,0xbdcbb616,0,0xbe19999a,0x80000000,0xbf07ceb8,0,0xbf194b85,0x3fda4c03,0x3f90a3d7,0xbfba538c,0,21,17,0],
        ];
        for (index, record) in buffer.data[ranges[0].0 as usize..ranges[0].0 as usize + 3].iter().enumerate() {
            let words: Vec<u32> = record.transform.cols.iter().flat_map(|v| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()])
                .chain([record.gate_seed as u32, record.piece_seed as u32, record.wall_id, record.flags]).collect();
            assert_eq!(words, original[index], "原圆墙窗下第 {index} 根木梁");
        }
        // 原 push 返回失败时设置捕获标志，继续消费全部候选梁。
        // 使用全新空分配：清理已有三条的分配会把其块归还 free 栈。
        let mut buffer = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&Device, None);
        let allocation = buffer.new_allocation();
        buffer.free_chunks.write().clear();
        generated = false;
        {
            let mut writer = buffer.writer(&allocation);
            add_underwindow_pattern(&window, &base, 1.0, 3.0, &mut generated, &wall, &shape,
                &mut writer, &mut failed, WallId(17), 4, 0.0);
        }
        assert!(generated && failed);
        assert!(buffer.allocation_to_draw_multi_instanced(&allocation).is_empty());
    }

    #[test]
    fn underwindow_gates_and_three_wall_segments() {
        assert!(underwindow_wall_segments(0x16, 1.5, 0.0, 1.0, 3.0, 0.0).is_none());
        for height in [0.5, 2.0, f32::NAN] {
            assert!(underwindow_wall_segments(0, height, 0.0, 1.0, 3.0, 0.0).is_none());
        }
        let result = underwindow_wall_segments(0, 1.0, 0.0, 1.0, 3.0, 0.0).unwrap();
        assert_eq!(result, [[Vec2::new(1.1, 1.13), Vec2::new(2.1, f32::from_bits(0x3d23_d708))],
            [Vec2::new(1.9, f32::from_bits(0x3d23_d708)), Vec2::new(2.9, 1.13)],
            [Vec2::new(1.0, 1.13), Vec2::new(3.0, 1.13)]]);
        // 同一 B 原函数的有界 Rosetta 执行，逐位比较非常规坐标。
        let result = underwindow_wall_segments(0, 1.375, -0.125, -2.75, 6.625, -0.125).unwrap();
        let bits: Vec<u32> = result.iter().flat_map(|pair| pair.iter())
            .flat_map(|point| [point.x.to_bits(), point.y.to_bits()]).collect();
        assert_eq!(bits, [0xc029999a, 0x3fc0a3d7, 0x40026666, 0xbdae147c,
            0x3feb3333, 0xbdae147c, 0x40d0cccd, 0x3fc0a3d7,
            0xc0300000, 0x3fc0a3d7, 0x40d40000, 0x3fc0a3d7]);
    }

    fn window<'a>(tag: u8) -> WindowAabbSource<'a> {
        let mut raw = [0u8; 0x50];
        raw[0x46] = tag;
        WindowAabbSource {
            raw,
            attachment: [1.0, 2.0],
            bbx: [0.5, 0.1, 1.1],
            profile: &[],
        }
    }

    #[test]
    fn plain_window_uses_max_width_and_height_delta() {
        let aabb = get_window_aabb(&window(b'7'));
        // width = max(0.5+0.2, 0.46)=0.7；height=1.1−0.1=1.0。
        assert!((aabb.width() - 0.7).abs() < 1e-6);
        assert!((aabb.height() - 1.0).abs() < 1e-6);
        assert_eq!(aabb.center(), Vec2::new(1.0, 2.0));
    }

    #[test]
    fn subtype_8_and_colon_adjust_height() {
        let tall = get_window_aabb(&window(b'8'));
        assert!((tall.height() - 1.2).abs() < 1e-6);
        let mut colon = window(b':');
        colon.raw[0x44] = 3;
        let aabb = get_window_aabb(&colon);
        assert!((aabb.height() - 1.4).abs() < 1e-6);
        assert!((aabb.width() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn width_clamps_to_046() {
        let mut w = window(b'7');
        w.bbx[0] = 0.1;
        assert!((get_window_aabb(&w).width() - 0.46).abs() < 1e-6);
        w.bbx[0] = f32::NAN;
        assert!((get_window_aabb(&w).width() - 0.46).abs() < 1e-6);
    }

    #[test]
    fn gothic_profile_builds_from_last_and_first_points() {
        let profile = [[0.1, 0.2], [0.3, 0.5]];
        let mut w = window(b'9');
        w.profile = &profile;
        let aabb = get_window_aabb(&w);
        assert!((aabb.min.x - (1.0 - 0.3)).abs() < 1e-6);
        assert!((aabb.min.y - (2.0 + 0.5)).abs() < 1e-6);
        assert!((aabb.max.x - (1.0 + 0.3)).abs() < 1e-6);
        assert!((aabb.max.y - (0.2 + 0.093 + 2.0)).abs() < 1e-5);
    }

    #[test]
    fn overlap_check_detects_inside_and_shifted_points() {
        let w = window(b'7');
        assert!(check_for_overlap_with_all_windows(
            Vec2::new(1.0, 2.0),
            0.1,
            &[w]
        ));
        // 移位后中心 x=0.9，区间 [0.55,1.25]；点 1.2 命中。
        assert!(check_for_overlap_with_all_windows(
            Vec2::new(1.2, 2.0),
            0.1,
            &[w]
        ));
        assert!(!check_for_overlap_with_all_windows(
            Vec2::new(4.0, 4.0),
            0.1,
            &[w]
        ));
        // 原移位盒保留窗中心高度，不能用待测点高度替代。
        assert!(!check_for_overlap_with_all_windows(
            Vec2::new(0.0, 8.0), 1.0, &[w]
        ));
    }

    #[test]
    fn overlapping_windows_reports_shifted_flag() {
        let w = window(b'7');
        // 原窗中心1、余量1时向左移1，查询中心0命中移位记录。
        let query = Aabb2::from_center_dims(Vec2::new(0.0, 2.0), Vec2::new(0.1, 2.0));
        let hits = overlapping_windows(query, 1.0, &[w], &[b'7']);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].shifted, 1);
        let bytes = hits[0].observed_bytes();
        assert_eq!(bytes[17], b'7');
        // 原位与移位可同时命中，原函数会各追加一条。
        let query = Aabb2::from_center_dims(Vec2::new(1.0, 2.0), Vec2::new(0.1, 2.0));
        let hits = overlapping_windows(query, 0.0, &[w], &[b'7']);
        assert_eq!(hits.iter().map(|hit| hit.shifted).collect::<Vec<_>>(), [0, 1]);
    }
}
