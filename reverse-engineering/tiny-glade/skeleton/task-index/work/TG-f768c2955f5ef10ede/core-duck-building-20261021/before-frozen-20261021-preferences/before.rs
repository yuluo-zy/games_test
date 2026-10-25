//! 玩家楼梯的图到几何计算，沿既有模块逐段恢复原 B 业务。
//!
//! 已绑定完整原码：节点 fold、锚点派生、段类型、对齐组合搜索、装配参数计算。
//! 原 shape 遍历、完整请求 producer、装饰生成和图编辑仍按各自合同推进；
//! 本计算器不通过空资源或默认请求代替这些尚未接线的上游。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::{
    AlignKind, PlayermadeStairsAssemblyParams, StairNodeAnchor, StairWallAnchor,
    StairSubSegment, StairSubSegments,
};
use country_core::resources::stairs::stairs_state::{StairGraphEdge, StairSegment, StairsState};
use country_core::resources::terrain_height::{TerrainHeightsData, VisualTerrainHeightSsbo};
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorInfo, DecoratorRotationXZ};
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Vec2, Vec3};
use utils::natural_cubic::{DenseSplineSegment2D, Spline2D};

use crate::blueprint::DecoratorBlueprints;

/// 段类型阈值（O：0x1429cd064 = 2.5、0x142925950 = 0.25）。
const STEEP_SLOPE: f32 = 2.5;
const GENTLE_SLOPE: f32 = 0.25;

/// 原 0x14080c810：先验证高度，再取路径首尾，保留原有序比较与第二返回位。
pub fn determine_segment_type(points: &[Vec2], heights: &[f32]) -> (u8, u8) {
    let first_height = *heights.first().expect("determine_segment_type: empty heights");
    let last_height = *heights.last().expect("determine_segment_type: empty heights");
    let first = points.first().expect("determine_segment_type: empty points");
    let last = points.last().expect("determine_segment_type: empty points");
    let dx = first.x - last.x;
    let dz = first.y - last.y;
    let slope = (first_height - last_height).abs() / (dz * dz + dx * dx).sqrt();
    let kind = if slope > STEEP_SLOPE {
        3
    } else if slope > GENTLE_SLOPE {
        2
    } else {
        // 原 SETA/CMOVBE 对 NaN 取此分支；不能改成 <=2.5 外门。
        1
    };
    (kind, 1)
}

/// 0x140809ad0（O）：密集采样样条段（原属 stairs.rs 模块 451；已恢复于
/// utils::Spline2D::densely_sample_segment，此处直通）。
pub fn densely_sample_spline_segment(spline: &Spline2D, index: usize) -> DenseSplineSegment2D {
    spline.densely_sample_segment(index)
}

/// 原图节点到请求几何的实际本轮值；normal 与沿曲线的 tangent 不混用。
#[derive(Clone, Copy, Debug)]
struct StairPoint {
    addr: DecoratorStorageAddr,
    wall_normal: Option<Vec2>,
    position: Vec3,
    id: DecoratorId,
    width: f32,
}

/// 原 14048ce90 中翻转后交换两分量，保持墙两侧的真实法线。
fn stair_wall_normal(
    info: &DecoratorInfo,
    addr: DecoratorStorageAddr,
    walls: &PublicWalls,
) -> Vec2 {
    let wall = walls.get(addr.as_wall()).expect("stair point: missing wall");
    let tangent = info.wall_space_coord().get_tangent(wall.simplified_curve_2d());
    if info.dst_tag() == 0 {
        Vec2::new(tangent.y, -tangent.x)
    } else {
        Vec2::new(-tangent.y, tangent.x)
    }
}

/// 原 14048ce90 完整节点 fold；只构造本轮临时表，所有值来自同一权威 owner。
fn collect_stair_points(
    storage: &DecoratorStorage,
    cached: &CachedDecoratorTransforms,
    walls: &PublicWalls,
    stairs: &StairsState,
) -> std::collections::HashMap<DecoratorId, StairPoint> {
    stairs.graph().nodes().map(|raw_id| {
        let id = DecoratorId(raw_id);
        let addr = storage.try_lookup_storage(id).expect("stair point: missing storage address");
        let bucket = storage.get(addr).expect("stair point: missing storage");
        let info = bucket.get(id).expect("stair point: missing decorator");
        let transform = cached.transform(raw_id);
        let extra = info.extra().as_stair();
        let width = f32::from_le_bytes(extra.0[4..8].try_into().unwrap());
        let wall_normal = if info.dst_tag() <= 1 {
            Some(stair_wall_normal(info, addr, walls))
        } else {
            None
        };
        (id, StairPoint { addr, wall_normal, position: transform.translation, id, width })
    }).collect()
}

/// 原 0x14080c640 的真实锚点和度数，不把跳表索引当成业务输出。
pub fn derive_stair_node_params(
    info: &DecoratorInfo,
    walls: &PublicWalls,
    stairs: &StairsState,
    addr: DecoratorStorageAddr,
    id: DecoratorId,
) -> (StairNodeAnchor, u8) {
    // 原低字节转换会回绕；邻居未发布其它信息也不影响此真实图计数。
    let degree = stairs.graph().neighbors(id.0).count() as u8;
    let tag = info.dst_tag();
    let anchor = match tag {
        2 | 3 => {
            let extra = info.extra().as_stair();
            let height = f32::from_le_bytes(extra.0[..4].try_into().unwrap());
            if tag == 2 { StairNodeAnchor::Roof { height } }
            else { StairNodeAnchor::Terrain { height } }
        }
        4 => unreachable!(),
        _ => {
            info.dst_as_wall();
            let normal = stair_wall_normal(info, addr, walls);
            let angle = DecoratorRotationXZ::from_xz_forward_vector(normal).0;
            StairNodeAnchor::Wall(StairWallAnchor::new(addr.as_wall(), angle))
        }
    };
    (anchor, degree)
}

/// 0x14080beb0（O 结构，6+ 参面）：对齐偏好计算。
/// U：偏好准则（邻墙对齐）主体。
pub fn find_alignment_preferences(
    _p1: u64,
    _p2: u64,
    _p3: u64,
    _f4: f32,
    _f5: f32,
) -> Vec<(u32, f32)> {
    Vec::new() // U。
}

/// B 0x14080c360：遍历完整邻接候选组合，返回所有选择及其总分。
/// 冲突方向每次扣 10；严格大于才替换，保持原先后顺序与 NaN 分支。
pub fn find_best_neighbor_alignments(
    candidates: &[(DecoratorId, Vec<(AlignKind, f32)>)],
    used_directions: u8,
) -> (Vec<(DecoratorId, AlignKind)>, f32) {
    let Some(((id, options), remaining)) = candidates.split_first() else {
        return (Vec::new(), 0.0);
    };
    let mut best = Vec::new();
    let mut best_score = -f32::MAX;
    for &(kind, score) in options {
        let direction = match kind.tag {
            0 => 1,
            1 => {
                let left = kind.first.x * kind.second.y;
                let right = kind.first.y * kind.second.x;
                // 原 UCOMISS/SETBE 在 unordered 时也取 mask4。
                if left > right { 2 } else { 4 }
            }
            _ => unreachable!(),
        };
        let (mut choices, child_score) =
            find_best_neighbor_alignments(remaining, used_directions | direction);
        let penalty = if used_directions & direction != 0 { -10.0 } else { 0.0 };
        let total = (score + child_score) + penalty;
        if total > best_score {
            // 接管递归的真实 Vec；原算法在前部插入当前邻居，避免整表 clone。
            choices.insert(0, (*id, kind));
            best = choices;
            best_score = total;
        }
    }
    (best, best_score)
}

/// 原 request source530..584 的第二遍有向图旗位传播，先收集再统一修改。
fn propagate_alignment_flags(
    alignments: &mut petgraph::graphmap::DiGraphMap<DecoratorId, AlignKind, std::hash::RandomState>,
) {
    let mut updates = Vec::new();
    for node in alignments.nodes() {
        let mut has_straight = false;
        let mut has_positive = false;
        let mut has_nonpositive = false;
        for neighbor in alignments.neighbors(node) {
            let alignment = alignments.edge_weight(node, neighbor).expect("alignment graph: missing edge");
            match alignment.tag {
                0 => has_straight = true,
                1 => {
                    let cross = alignment.first.x * alignment.second.y
                        - alignment.first.y * alignment.second.x;
                    // 原先 signum 再有序比较：+0 为正；-0 与 NaN 走另一侧。
                    if cross.signum() > 0.0 { has_positive = true; }
                    else { has_nonpositive = true; }
                }
                _ => {}
            }
        }
        if !has_straight || !(has_positive || has_nonpositive) { continue; }
        let flags = u8::from(has_nonpositive) | (u8::from(has_positive) << 1);
        for neighbor in alignments.neighbors(node) {
            if alignments.edge_weight(node, neighbor).expect("alignment graph: missing edge").tag <= 1 {
                updates.push((node, neighbor, flags));
            }
        }
    }
    for (node, neighbor, flags) in updates {
        let alignment = alignments.edge_weight_mut(node, neighbor).expect("alignment graph: missing edge");
        match alignment.tag {
            2 => {}
            1 => alignment.flag = 1,
            _ => { alignment.flag = flags & 1; alignment.byte_2 = (flags >> 1) & 1; }
        }
    }
}

/// 0x140803dd0（O 结构）：生成玩家楼梯装饰视觉（walk_shape 消费 + 蓝图）。
pub fn generate_playermade_stair_decorators(
    _storage: Res<DecoratorStorage>,
    _derived: Res<DerivedDecoratorSubtypes>,
    _blueprints: ResMut<DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    // U：楼梯视觉生成主体待取证。
}

/// 0x140276e80 / 0x14027e470（O 结构，泛型 <false,true>）：生成玩家楼梯请求
/// （形状遍历 → 段类型 → 节点参数 → 装配参数）。
pub fn generate_playermade_stair_requests<const X: bool>() {
    // U：主体待取证。
}

/// 原 walk 返回五个真实 owner，None 与 Some(empty) 不合并。
struct WalkShapeResult {
    sub_segments: Option<StairSubSegments>,
    points: Vec<Vec2>,
    tangents: Option<Vec<Vec2>>,
    widths: Option<Vec<f32>>,
    heights: Option<Vec<f32>>,
}

impl WalkShapeResult {
    /// 仅用于原明确返回无路径的分支，不是失败转成功的替身。
    fn empty() -> Self {
        Self { sub_segments: None, points: Vec::new(), tangents: None, widths: None, heights: None }
    }
}

/// 原矩形 receiver 捕获状态，同一 stream 的所有角点共享，不逐角重置。
struct RectangleStairReceiver {
    endpoint_widths: [f32; 2],
    endpoint_heights: [f32; 2],
    need_height: bool,
    previous: Option<(Vec2, f32)>,
    points: Vec<Vec2>,
    tangents: Vec<Vec2>,
    widths: Vec<f32>,
    heights: Vec<f32>,
    sub_segments: StairSubSegments,
}

impl RectangleStairReceiver {
    /// 原 1412b2540 完整 callback：有序去重、四类段、梯段末行替换。
    fn receive(&mut self, record: utils::geometry::rectangle::RectangleSlicePoint) {
        let width = utils::float_grid::lerp(self.endpoint_widths[0], self.endpoint_widths[1], record.relative_u);
        let point = record.position + record.normal * (0.5 * width);
        if self.points.last().is_some_and(|&last| last == point) { return; }
        let tangent = Vec2::new(record.normal.y, -record.normal.x);
        if !self.need_height {
            self.points.push(point); self.tangents.push(tangent); self.widths.push(width);
            return;
        }
        let height = utils::float_grid::lerp(self.endpoint_heights[0], self.endpoint_heights[1], record.relative_u);
        if let Some((previous_point, previous_height)) = self.previous {
            let kind = if record.tag != 5 { 4 } else {
                let delta = previous_point - point;
                let slope = (previous_height - height).abs() / (delta.y * delta.y + delta.x * delta.x).sqrt();
                if slope > 2.5 { 3 } else if slope > 0.25 { 2 } else { 1 }
            };
            let merge = self.sub_segments.last().is_some_and(|last| {
                last.kind == kind && if kind < 2 { last.flag != 0 } else { last.kind >= 2 }
            });
            if merge && kind == 3 {
                *self.points.last_mut().expect("rectangle stair: missing point") = point;
                *self.tangents.last_mut().expect("rectangle stair: missing tangent") = tangent;
                *self.widths.last_mut().expect("rectangle stair: missing width") = width;
                *self.heights.last_mut().expect("rectangle stair: missing height") = height;
                self.previous = Some((point, height));
                return;
            }
            if merge {
                let last = self.sub_segments.last_mut().unwrap();
                last.count = last.count.wrapping_add(1);
            } else {
                self.sub_segments.push(StairSubSegment { kind, flag: 1, count: 1 });
            }
        }
        self.previous = Some((point, height));
        self.points.push(point); self.tangents.push(tangent);
        self.widths.push(width); self.heights.push(height);
    }
}

/// 原 140809cc0：真实墙周界或 source curve 的完整采样和 offset strip 裁剪。
fn walk_shape(
    wall: &PublicWallState, alignments: [AlignKind; 2], directions: [Vec2; 2],
    widths: [f32; 2], coordinates: [utils::wall_space::WallSpaceCoord; 2],
) -> WalkShapeResult {
    use utils::comparative_space::ComparativeSpace;
    use utils::wall_space::{WallSpace, WallSpaceCoord};
    let space = WallSpace { is_loop: wall.shape().tag() != 2, length: wall.wall_space_length() };
    let delta = space.subtract(coordinates[1].u, coordinates[0].u);
    if let Some(rectangle) = wall.is_rectangle() {
        let reciprocal = 1.0 / space.length;
        let mut start = coordinates[0].u * reciprocal;
        let mut end = coordinates[1].u * reciprocal;
        let mut endpoint_widths = widths;
        let mut endpoint_heights = [coordinates[0].y, coordinates[1].y];
        let inset = |i: usize| if alignments[i].tag == 1 && alignments[i].flag == 1 {
            0.5 * widths[i] * reciprocal
        } else { 0.0 };
        let mut start_offset = inset(0);
        let mut end_offset = inset(1);
        if delta < 0.0 {
            std::mem::swap(&mut start, &mut end);
            endpoint_widths.reverse(); endpoint_heights.reverse();
            std::mem::swap(&mut start_offset, &mut end_offset);
        }
        let padded_start = start + start_offset;
        let padded_end = end - end_offset;
        if (end - start).signum() * (padded_end - padded_start).signum() == 1.0 {
            start = padded_start; end = padded_end;
        }
        // canonical 参数构造器仍有 infinity 失败时序差异，交 root 修同一 owner。
        let start = utils::curve::curve_u_circular::CurveUCircular::<
            utils::curve::CurveCoordSemantic2D,
        >::new(start).get();
        let end = utils::curve::curve_u_circular::CurveUCircular::<
            utils::curve::CurveCoordSemantic2D,
        >::new(end).get();
        let mut receiver = RectangleStairReceiver {
            endpoint_widths, endpoint_heights, need_height: true, previous: None,
            points: Vec::new(), tangents: Vec::new(), widths: Vec::new(), heights: Vec::new(),
            sub_segments: StairSubSegments::new(),
        };
        rectangle.circular_slice_segment_points(start, end,
            &mut utils::geometry::rectangle::RectangleSliceState::default(),
            |record| receiver.receive(record));
        if delta < 0.0 {
            receiver.points.reverse(); receiver.tangents.reverse();
            for tangent in &mut receiver.tangents { *tangent = -*tangent; }
            receiver.widths.reverse(); receiver.heights.reverse(); receiver.sub_segments.reverse();
        }
        return WalkShapeResult {
            sub_segments: Some(receiver.sub_segments), points: receiver.points,
            tangents: Some(receiver.tangents), widths: Some(receiver.widths), heights: Some(receiver.heights),
        };
    }

    if alignments[0].tag != 1 { return WalkShapeResult::empty(); }
    let side_sign = if alignments[0].byte_2 != 0 { -1.0 } else { 1.0 };
    let magnitude = delta.abs();
    if !(magnitude >= f32::from_bits(0x3a83_126f)) { return WalkShapeResult::empty(); }
    let steps = (magnitude.ceil() as usize).max(1);
    let count = steps.wrapping_add(1);
    let mut points = Vec::with_capacity(count);
    let mut sampled_widths = Vec::new();
    let sign = delta.signum();
    let mut start_u = coordinates[0].u;
    if alignments[0].flag != 0 { start_u = space.project_clamp(start_u + sign * widths[0] * 0.5); }
    if alignments[1].tag != 1 { return WalkShapeResult::empty(); }
    let mut end_u = coordinates[1].u;
    if alignments[1].flag != 0 { end_u = space.project_clamp(end_u + sign * widths[1] * (-0.5)); }
    let endpoint_normals = directions.map(|direction| {
        let direction = direction.normalize_or_zero();
        Vec2::new(-direction.y, direction.x) * sign
    });
    let curve = wall.simplified_curve_2d();
    for i in 0..count {
        let t = (i as f32) / (steps as f32);
        let u = space.project_clamp(start_u + space.subtract(end_u, start_u) * t);
        let width = utils::float_grid::lerp(widths[0], widths[1], t);
        sampled_widths.push(width);
        let coordinate = WallSpaceCoord::new(&space, u, 0.0);
        let position = coordinate.get_pos_xz(curve);
        let normal = if i == 0 { endpoint_normals[0] }
            else if i == steps { endpoint_normals[1] }
            else { let tangent = coordinate.get_tangent(curve); Vec2::new(-tangent.y, tangent.x) };
        points.push(position + (normal * (width * 0.5)) * side_sign);
    }
    if points.len() <= 1 {
        return WalkShapeResult { points, ..WalkShapeResult::empty() };
    }
    let mut tangents: Vec<_> = (0..points.len()).map(|i| {
        let difference = if i == 0 { points[1] - points[0] }
            else if i + 1 == points.len() { points[i] - points[i - 1] }
            else { points[i + 1] - points[i - 1] };
        difference.normalize_or_zero()
    }).collect();

    loop {
        let mut removed = false;
        for i in 1..points.len() - 1 {
            let j = i + 1;
            let offset = |index: usize| {
                Vec2::new(-tangents[index].y, tangents[index].x)
                    * (sampled_widths[index] * 0.5 * 1.25) * side_sign
            };
            let a = points[i]; let b = points[j];
            let oa = offset(i); let ob = offset(j);
            let quad = if side_sign > 0.0 { [a + oa, a - oa, b - ob, b + ob] }
                else { [b + ob, b - ob, a - oa, a + oa] };
            let concave = (0..4).any(|corner| {
                let previous = quad[(corner + 3) % 4] - quad[corner];
                let next = quad[(corner + 1) % 4] - quad[corner];
                previous.x * next.y - previous.y * next.x > f32::from_bits(0x322b_cc77)
            });
            if !concave { continue; }
            points.remove(i); tangents.remove(i); sampled_widths.remove(i);
            tangents[i - 1] = if i == 1 { (points[1] - points[0]).normalize_or_zero() }
                else { (points[i] - points[i - 2]).normalize_or_zero() };
            tangents[i] = if i + 1 == points.len() {
                (points[i] - points[i - 1]).normalize_or_zero()
            } else { (points[i + 1] - points[i - 1]).normalize_or_zero() };
            removed = true;
            break;
        }
        if !removed { break; }
    }
    WalkShapeResult {
        sub_segments: None, points, tangents: Some(tangents),
        widths: Some(sampled_widths), heights: None,
    }
}

/// 0x140806110（O 结构，8 参面）：维护楼梯图（StairsState 消费 +
/// apply_decorator_op 应用）。U：图维护主体。
pub fn maintain_stair_graph(
    _storage: Res<DecoratorStorage>,
    _stairs: Res<country_core::resources::stairs::stairs_state::StairsState>,
    _commands: Commands,
) {
    // U。
}

/// 原计算器按所有权移动的四个覆盖项；Some(empty) 与 None 保持区分。
struct StairAssemblyOverrides {
    tangents: Option<Vec<Vec2>>,
    widths: Option<Vec<f32>>,
    heights: Option<Vec<f32>>,
    sub_segments: Option<StairSubSegments>,
}

/// 原调用点的几何参数分组；caller 给定端点、方向与有序 locator ID 不重新猜测。
struct StairAssemblyInput<'a> {
    nodes: [(StairNodeAnchor, u8); 2],
    points: [&'a StairPoint; 2],
    alignments: [AlignKind; 2],
    positions: [Vec2; 2],
    directions: [Vec2; 2],
    ordered_ids: [DecoratorId; 2],
    curve: utils::curve::Curve2,
}

/// 原 140807090 完整计算：地形采样、真实分段压缩、端点 padding 与旧 dirty 传播。
fn compute_stair_assembly_params(
    input: StairAssemblyInput<'_>,
    overrides: StairAssemblyOverrides,
    terrain: &TerrainHeightsData<VisualTerrainHeightSsbo>,
    dirty_edges: &std::collections::HashSet<StairGraphEdge>,
) -> PlayermadeStairsAssemblyParams {
    use bitvec::{order::Lsb0, vec::BitVec};
    use utils::curve::Curve2;
    let StairAssemblyInput {
        nodes, points, alignments, positions, directions, ordered_ids, mut curve,
    } = input;
    let StairAssemblyOverrides {
        tangents: tangent_override, widths: width_override,
        heights: height_override, sub_segments: segment_override,
    } = overrides;
    let start_height = points[0].position.y;
    let end_height = points[1].position.y;
    let start_width = points[0].width;
    let end_width = points[1].width;
    let point_count = curve.points.len();

    let mut tangents = tangent_override.unwrap_or_else(|| {
        (0..point_count).map(|i| {
            let delta = if i + 1 < point_count {
                curve.points[i + 1] - curve.points[i]
            } else {
                curve.points[i] - curve.points[i - 1]
            };
            // 原 game callback 用 raw normalize；零段必须保留 NaN。
            delta.normalize()
        }).collect()
    });
    let mut widths = width_override.unwrap_or_else(|| {
        curve.points_u.iter().map(|&u| u * end_width + (1.0 - u) * start_width).collect()
    });
    let mut terrain_contacts = BitVec::<usize, Lsb0>::repeat(false, point_count);
    let mut heights = height_override.unwrap_or_else(|| {
        curve.points.iter().zip(&curve.points_u).zip(&tangents).zip(&widths)
            .enumerate().map(|(i, (((&point, &u), &tangent), &width))| {
                // 原连续两次乘 .5，不改写成另一采样宽度。
                let offset = Vec2::new(-tangent.y, tangent.x) * (width * 0.5 * 0.5);
                let plus = point + offset;
                let minus = point - offset;
                let plus_height = terrain.sample_at_world_xz(plus.x, plus.y) + (-0.025);
                let minus_height = terrain.sample_at_world_xz(minus.x, minus.y) + (-0.025);
                let floor = plus_height.max(minus_height);
                let line = start_height * (1.0 - u) + end_height * u;
                terrain_contacts.set(i, floor + 0.1 > line);
                floor.max(line)
            }).collect()
    });

    // 必须在地形 callback 之后覆盖；短 override 保留真实逐次 bounds 失败。
    tangents[0] = directions[0].normalize_or_zero();
    tangents[point_count - 1] = directions[1].normalize_or_zero();

    let mut sub_segments = if let Some(segments) = segment_override {
        segments
    } else {
        let rows = std::mem::take(&mut curve.points).into_iter()
            .zip(tangents).zip(widths).zip(heights).zip(terrain_contacts.iter().by_vals());
        let mut kept_points = Vec::new();
        let mut kept_tangents = Vec::new();
        let mut kept_widths = Vec::new();
        let mut kept_heights = Vec::new();
        let mut kept_contacts = BitVec::<usize, Lsb0>::new();
        let mut segments = StairSubSegments::new();
        let mut previous: Option<(Vec2, f32)> = None;
        for ((((point, tangent), width), height), contact) in rows {
            if let Some((previous_point, previous_height)) = previous {
                let delta = previous_point - point;
                let slope = (previous_height - height).abs()
                    / (delta.y * delta.y + delta.x * delta.x).sqrt();
                let kind = if slope > 2.5 { 3 } else if slope > 0.25 { 2 } else { 1 };
                let can_merge = segments.last().is_some_and(|last| {
                    last.kind == kind && (kind >= 2 || last.flag != 0)
                });
                if can_merge && kind == 3 {
                    // 连续梯段只替换末行；分类仍使用相邻原输入，而非保留行。
                    *kept_points.last_mut().expect("stair collapse: missing point") = point;
                    *kept_tangents.last_mut().expect("stair collapse: missing tangent") = tangent;
                    *kept_widths.last_mut().expect("stair collapse: missing width") = width;
                    *kept_heights.last_mut().expect("stair collapse: missing height") = height;
                    let last = kept_contacts.len().checked_sub(1)
                        .expect("stair collapse: missing terrain contact");
                    kept_contacts.set(last, contact);
                    previous = Some((point, height));
                    continue;
                }
                if can_merge {
                    let last = segments.last_mut().unwrap();
                    last.count = last.count.wrapping_add(1);
                } else {
                    segments.push(StairSubSegment { kind, flag: 1, count: 1 });
                }
            }
            previous = Some((point, height));
            kept_points.push(point);
            kept_tangents.push(tangent);
            kept_widths.push(width);
            kept_heights.push(height);
            kept_contacts.push(contact);
        }
        curve = Curve2::crashy_new_from_points(kept_points, false);
        tangents = kept_tangents;
        widths = kept_widths;
        heights = kept_heights;
        terrain_contacts = kept_contacts;
        segments
    };

    let start_alignment = alignments[0];
    if start_alignment.tag == 0
        && (start_alignment.flag != 0 || start_alignment.byte_2 & 1 != 0)
    {
        sub_segments.insert(0, StairSubSegment {
            kind: start_alignment.flag ^ 1, flag: start_alignment.byte_2 ^ 1, count: 1,
        });
        curve.points.insert(0, positions[0] - start_alignment.first * start_width);
        curve = Curve2::crashy_new_from_points(std::mem::take(&mut curve.points), false);
        heights.insert(0, start_height);
        widths.insert(0, start_width);
        tangents.insert(0, directions[0]);
        terrain_contacts.insert(0, false);
    }
    let end_alignment = alignments[1];
    if end_alignment.tag == 0
        && (end_alignment.flag != 0 || end_alignment.byte_2 & 1 != 0)
    {
        sub_segments.push(StairSubSegment {
            kind: end_alignment.byte_2 ^ 1, flag: end_alignment.flag ^ 1, count: 1,
        });
        curve.points.push(positions[1] - end_alignment.first * end_width);
        curve = Curve2::crashy_new_from_points(std::mem::take(&mut curve.points), false);
        heights.push(end_height);
        widths.push(end_width);
        tangents.push(directions[1]);
        terrain_contacts.push(true);
    }

    let segment = StairSegment::new(ordered_ids[0], ordered_ids[1]);
    let dirty = dirty_edges.contains(&StairGraphEdge::from(segment));
    PlayermadeStairsAssemblyParams {
        curve, heights, widths, tangents,
        // 原本 producer 明确初始化 empty/None，不能据此冒称 mesh 消费者已完成。
        auxiliary_samples: Vec::new(), interpolated_curve: None,
        nodes, terrain_contacts, segment, sub_segments, dirty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    fn direction_kind(tag: u8, first: Vec2, second: Vec2) -> AlignKind {
        AlignKind { tag, first, second, ..AlignKind::default() }
    }

    #[test]
    fn alignment_search_chooses_the_global_non_conflicting_combination() {
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let turn = direction_kind(1, Vec2::X, Vec2::Y);
        let candidates = vec![
            (DecoratorId(7), vec![(straight, 9.0), (turn, 8.0)]),
            (DecoratorId(29), vec![(straight, 9.0)]),
        ];
        let before = candidates.clone();
        let (choices, score) = find_best_neighbor_alignments(&candidates, 0);
        assert_eq!(choices, vec![(DecoratorId(7), turn), (DecoratorId(29), straight)]);
        assert_eq!(score, 17.0);
        assert_eq!(candidates, before);
    }

    #[test]
    fn alignment_search_preserves_the_first_equal_score_candidate() {
        let first = direction_kind(0, Vec2::X, Vec2::ZERO);
        let second = direction_kind(0, Vec2::Y, Vec2::ZERO);
        let candidates = [(DecoratorId(3), vec![(first, 5.0), (second, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0),
            (vec![(DecoratorId(3), first)], 5.0));
    }

    #[test]
    fn alignment_empty_inputs_and_empty_candidate_lists_have_distinct_scores() {
        assert_eq!(find_best_neighbor_alignments(&[], 7), (vec![], 0.0));
        let candidates = [(DecoratorId(3), vec![])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0), (vec![], -f32::MAX));
    }

    #[test]
    fn alignment_unordered_cross_uses_mask_four_and_nan_score_never_wins() {
        let kind = direction_kind(1, Vec2::new(f32::NAN, 0.0), Vec2::Y);
        let candidates = [(DecoratorId(3), vec![(kind, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 4).1, -5.0);
        assert_eq!(find_best_neighbor_alignments(&candidates, 2).1, 5.0);
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let scores = [(DecoratorId(3), vec![(straight, f32::NAN), (straight, 1.0)])];
        assert_eq!(find_best_neighbor_alignments(&scores, 0),
            (vec![(DecoratorId(3), straight)], 1.0));
        let all_nan = [(DecoratorId(3), vec![(straight, f32::NAN)])];
        assert_eq!(find_best_neighbor_alignments(&all_nan, 0), (vec![], -f32::MAX));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn alignment_none_variant_retains_original_panic() {
        let none = AlignKind { tag: 2, ..AlignKind::default() };
        let _ = find_best_neighbor_alignments(&[(DecoratorId(3), vec![(none, 0.0)])], 0);
    }

    fn stair_info(tag: u8, height: f32, width: f32) -> DecoratorInfo {
        use country_core::resources::walls::decorator::{DecoratorExtraInfo, StairExtra};
        let mut info = DecoratorInfo {
            u20: u32::from(tag) << 16,
            dst_wall: u64::from(1.0f32.to_bits()) | (u64::from(2.0f32.to_bits()) << 32),
            ..DecoratorInfo::default()
        };
        let mut extra = [0; 20];
        extra[..4].copy_from_slice(&height.to_le_bytes());
        extra[4..8].copy_from_slice(&width.to_le_bytes());
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
        info
    }

    fn published_wall() -> (PublicWalls, country_core::resources::walls::WallId) {
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use country_core::resources::walls::{InnerWalls, WallState};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0], [4.0, 4.0]], false).unwrap();
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &terrain);
        assert!(public.get(wall).is_some());
        (public, wall)
    }

    #[test]
    fn actual_world_graph_points_and_both_wall_sides_use_published_curve() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let (walls, wall) = published_wall();
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (raw_id, tag, position) in [
            (29, 0, Vec3::new(8.0, 3.0, 9.0)),
            (7, 1, Vec3::new(2.0, 4.0, 5.0)),
            (60, 2, Vec3::new(1.0, 6.0, 3.0)),
            (90, 3, Vec3::new(5.0, 2.0, 7.0)),
        ] {
            let addr = if tag == 3 { DecoratorStorageAddr::Terrain }
                       else { DecoratorStorageAddr::Wall(wall) };
            storage.apply_decorator_place(addr, DecoratorId(raw_id), &stair_info(tag, 2.5, 1.75));
            cached.insert(raw_id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: position, scale: Vec3::ONE,
            });
            stairs.add_node(raw_id);
        }
        stairs.add_edge(29, 7);
        stairs.add_edge(29, 60);
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(cached);
        world.insert_resource(walls);
        world.insert_resource(stairs);
        let storage = world.resource::<DecoratorStorage>();
        let walls = world.resource::<PublicWalls>();
        let stairs = world.resource::<StairsState>();
        let points = collect_stair_points(storage, world.resource::<CachedDecoratorTransforms>(), walls, stairs);
        assert_eq!(points.len(), 4);
        for (id, expected) in [(29, Vec2::NEG_Y), (7, Vec2::Y)] {
            let point = points[&DecoratorId(id)];
            assert_eq!(point.id, DecoratorId(id));
            assert_eq!(point.width, 1.75);
            assert_eq!(point.wall_normal, Some(expected));
            let info = storage.get(point.addr).unwrap().get(point.id).unwrap();
            let (anchor, degree) = derive_stair_node_params(info, walls, stairs, point.addr, point.id);
            let anchor = anchor.try_as_wall().unwrap();
            assert_eq!(anchor.wall, wall);
            assert!((anchor.wall_normal() - expected).length() < 0.000002);
            assert_eq!(degree, if id == 29 { 2 } else { 1 });
        }
        assert_eq!(points[&DecoratorId(29)].position, Vec3::new(8.0, 3.0, 9.0));
        assert!(points[&DecoratorId(60)].wall_normal.is_none());
        assert!(points[&DecoratorId(90)].wall_normal.is_none());
        let point = points[&DecoratorId(90)];
        let info = storage.get(point.addr).unwrap().get(point.id).unwrap();
        assert_eq!(derive_stair_node_params(info, walls, stairs, point.addr, point.id),
                   (StairNodeAnchor::Terrain { height: 2.5 }, 0));
    }

    #[test]
    fn derive_preserves_height_bits_and_original_degree_byte_wrap() {
        let mut stairs = StairsState::new();
        stairs.add_node(700);
        for other in 0..256 { stairs.add_edge(700, other); }
        let height = f32::from_bits(0x7fc1_2345);
        for tag in [2, 3] {
            let (anchor, degree) = derive_stair_node_params(
                &stair_info(tag, height, 2.0), &PublicWalls::default(), &stairs,
                DecoratorStorageAddr::Terrain, DecoratorId(700),
            );
            assert_eq!(degree, 0);
            let actual = match anchor {
                StairNodeAnchor::Roof { height } | StairNodeAnchor::Terrain { height } => height,
                StairNodeAnchor::Wall(_) => panic!("意外墙锚点"),
            };
            assert_eq!(actual.to_bits(), height.to_bits());
        }
    }

    #[test]
    fn required_stair_point_failures_preserve_cache_before_extra_order() {
        use country_core::resources::walls::decorator::{DecoratorExtraInfo, WindowExtra};
        let mut storage = DecoratorStorage::default();
        let mut info = stair_info(3, 2.0, 1.0);
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0)));
        storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(7), &info);
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            collect_stair_points(&storage, &CachedDecoratorTransforms::default(), &PublicWalls::default(), &stairs);
        })).unwrap_err();
        let message = panic.downcast_ref::<String>().map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied()).unwrap();
        assert!(message.contains("CachedDecoratorTransforms::transform"));
    }

    #[test]
    #[should_panic(expected = "missing storage address")]
    fn required_stair_point_without_authoritative_address_panics() {
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        collect_stair_points(&DecoratorStorage::default(), &CachedDecoratorTransforms::default(),
                             &PublicWalls::default(), &stairs);
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn derive_stair_destination_is_unreachable() {
        derive_stair_node_params(&stair_info(4, 0.0, 1.0), &PublicWalls::default(),
                                 &StairsState::new(), DecoratorStorageAddr::Stair, DecoratorId(7));
    }

    #[test]
    fn segment_type_preserves_ordered_nan_and_both_original_return_bytes() {
        let line = [Vec2::ZERO, Vec2::X];
        for (heights, kind) in [([0.0, 0.25], 1), ([0.0, 2.5], 2),
                                ([0.0, 3.0], 3), ([0.0, f32::NAN], 1),
                                ([0.0, f32::INFINITY], 3)] {
            assert_eq!(determine_segment_type(&line, &heights), (kind, 1));
        }
        assert_eq!(determine_segment_type(&[Vec2::ZERO], &[4.0]), (1, 1));
        assert_eq!(determine_segment_type(&[Vec2::ZERO], &[0.0, 1.0]), (3, 1));
        assert_eq!(determine_segment_type(&line, &[0.0, 100.0, 0.25]), (1, 1));
    }

    #[test]
    #[should_panic(expected = "empty heights")]
    fn segment_type_checks_empty_heights_before_empty_points() {
        determine_segment_type(&[], &[]);
    }

    #[test]
    #[should_panic(expected = "empty points")]
    fn segment_type_requires_the_real_points_slice() {
        determine_segment_type(&[], &[1.0]);
    }

    fn compute_world(heights: [f32; 2], widths: [f32; 2]) -> World {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (i, raw_id) in [29, 7].into_iter().enumerate() {
            storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(raw_id),
                                           &stair_info(3, heights[i], widths[i]));
            cached.insert(raw_id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: Vec3::new(i as f32 * 4.0, heights[i], 0.0),
                scale: Vec3::ONE,
            });
            stairs.add_node(raw_id);
        }
        stairs.add_edge(29, 7);
        let mut visual = TerrainHeightsData::<VisualTerrainHeightSsbo>::new();
        visual.set_height_range(0.0, 511.0);
        for z in 0..512 {
            for x in 0..512 {
                visual.heights_mut()[z * 512 + x] = (z as u32 * 65535 / 511) as u16;
            }
        }
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(cached);
        world.insert_resource(stairs);
        world.insert_resource(PublicWalls::default());
        world.insert_resource(visual);
        // 同 World 保有不同的物理高度，防止误读默认 TerrainHeightSsbo 也通过。
        let mut physical = TerrainHeightsData::<
            country_core::resources::terrain_height::TerrainHeightSsbo,
        >::new();
        physical.set_height_range(-50.0, -50.0);
        world.insert_resource(physical);
        world
    }

    fn no_stair_overrides() -> StairAssemblyOverrides {
        StairAssemblyOverrides { tangents: None, widths: None, heights: None, sub_segments: None }
    }

    fn run_compute_case(
        world: &World, curve: utils::curve::Curve2, alignments: [AlignKind; 2],
        directions: [Vec2; 2], positions: [Vec2; 2], overrides: StairAssemblyOverrides,
        dirty: &std::collections::HashSet<StairGraphEdge>,
    ) -> PlayermadeStairsAssemblyParams {
        let storage = world.resource::<DecoratorStorage>();
        let walls = world.resource::<PublicWalls>();
        let stairs = world.resource::<StairsState>();
        let points = collect_stair_points(storage, world.resource::<CachedDecoratorTransforms>(), walls, stairs);
        let ordered_ids = [DecoratorId(29), DecoratorId(7)];
        let point_refs = [&points[&ordered_ids[0]], &points[&ordered_ids[1]]];
        let nodes = point_refs.map(|point| {
            let info = storage.get(point.addr).unwrap().get(point.id).unwrap();
            derive_stair_node_params(info, walls, stairs, point.addr, point.id)
        });
        compute_stair_assembly_params(
            StairAssemblyInput { nodes, points: point_refs, alignments, positions, directions, ordered_ids, curve },
            overrides, world.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>(), dirty,
        )
    }

    fn no_padding() -> [AlignKind; 2] {
        [AlignKind { tag: 2, ..AlignKind::default() }; 2]
    }

    #[test]
    fn compute_samples_actual_visual_grid_before_endpoint_direction_overwrite() {
        let world = compute_world([0.0, 0.0], [8.0, 8.0]);
        let curve = utils::curve::Curve2::try_new_from_points(vec![Vec2::ZERO, Vec2::new(4.0, 0.0)], false).unwrap();
        let result = run_compute_case(&world, curve, no_padding(), [Vec2::Y; 2],
            [Vec2::ZERO, Vec2::new(4.0, 0.0)], StairAssemblyOverrides {
                tangents: Some(vec![Vec2::X; 2]), widths: None, heights: None,
                sub_segments: Some(StairSubSegments::new()),
            }, &Default::default());
        let terrain = world.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>();
        let expected = terrain.sample_at_world_xz(0.0, 2.0) - 0.025;
        let wrong_after_overwrite = terrain.sample_at_world_xz(2.0, 0.0) - 0.025;
        let wrong_half_width = terrain.sample_at_world_xz(0.0, 4.0) - 0.025;
        assert!((result.heights[0] - expected).abs() < 0.0001);
        assert!((result.heights[0] - wrong_after_overwrite).abs() > 1.0);
        assert!((result.heights[0] - wrong_half_width).abs() > 1.0);
        assert_eq!(result.tangents, vec![Vec2::Y; 2]);
        assert!(result.terrain_contacts.iter().all(|bit| *bit));
        assert!(result.auxiliary_samples.is_empty());
        assert!(result.interpolated_curve.is_none());
    }

    #[test]
    fn compute_uses_curve_u_and_keeps_raw_zero_segment_nan() {
        let world = compute_world([0.0, 0.0], [1.0, 5.0]);
        let curve = utils::curve::Curve2::try_new_from_points(
            vec![Vec2::ZERO, Vec2::X, Vec2::X, Vec2::new(4.0, 0.0)], false,
        ).unwrap();
        let result = run_compute_case(&world, curve, no_padding(), [Vec2::ZERO; 2],
            [Vec2::ZERO, Vec2::new(4.0, 0.0)], StairAssemblyOverrides {
                tangents: None, widths: None, heights: Some(vec![0.0; 4]),
                sub_segments: Some(StairSubSegments::new()),
            }, &Default::default());
        assert_eq!(result.widths, vec![1.0, 2.0, 2.0, 5.0]);
        assert_eq!(result.tangents[0], Vec2::ZERO);
        assert!(result.tangents[1].is_nan());
        assert_eq!(result.tangents[3], Vec2::ZERO);
    }

    #[test]
    fn compute_collapses_steep_rows_and_rebuilds_real_curve_coordinates() {
        let world = compute_world([0.0, 8.1], [1.0, 4.0]);
        let curve = utils::curve::Curve2::try_new_from_points(
            (0..4).map(|x| Vec2::new(x as f32, 0.0)).collect(), false,
        ).unwrap();
        let mut dirty = std::collections::HashSet::new();
        dirty.insert(StairGraphEdge::new(7, 29));
        let result = run_compute_case(&world, curve, no_padding(), [Vec2::X; 2],
            [Vec2::ZERO, Vec2::new(3.0, 0.0)], StairAssemblyOverrides {
                tangents: Some(vec![Vec2::X; 4]), widths: Some(vec![1.0, 2.0, 3.0, 4.0]),
                heights: Some(vec![0.0, 4.0, 8.0, 8.1]), sub_segments: None,
            }, &dirty);
        assert_eq!(result.curve.points, vec![Vec2::ZERO, Vec2::new(2.0, 0.0), Vec2::new(3.0, 0.0)]);
        assert_eq!(result.curve.points_u, vec![0.0, 2.0 / 3.0, 1.0]);
        assert_eq!(result.widths, vec![1.0, 3.0, 4.0]);
        assert_eq!(result.heights, vec![0.0, 8.0, 8.1]);
        assert_eq!(result.sub_segments.as_slice(), &[
            StairSubSegment { kind: 3, flag: 1, count: 1 },
            StairSubSegment { kind: 1, flag: 1, count: 1 },
        ]);
        assert!(result.dirty);
        assert_eq!(result.segment.nodes(), (DecoratorId(7), DecoratorId(29)));
        assert!(result.terrain_contacts.iter().all(|bit| !*bit));
    }

    #[test]
    fn compute_keeps_some_empty_and_moves_override_allocations() {
        let world = compute_world([0.0, 1.0], [1.0, 2.0]);
        let widths = vec![8.0, 9.0, 10.0];
        let widths_pointer = widths.as_ptr();
        let heights = Vec::new();
        let result = run_compute_case(&world, utils::curve::Curve2::try_new_from_points(
            vec![Vec2::ZERO, Vec2::X], false,
        ).unwrap(), no_padding(), [Vec2::X; 2], [Vec2::ZERO, Vec2::X],
            StairAssemblyOverrides { tangents: Some(vec![Vec2::X; 2]), widths: Some(widths),
                heights: Some(heights), sub_segments: Some(StairSubSegments::new()) },
            &Default::default());
        assert!(result.heights.is_empty());
        assert!(result.sub_segments.is_empty());
        assert_eq!(result.curve.points.len(), 2);
        assert_eq!(result.widths.as_ptr(), widths_pointer);
        assert_eq!(result.widths, vec![8.0, 9.0, 10.0]);
    }

    #[test]
    fn compute_padding_uses_caller_positions_raw_dirs_and_opposite_end_flags() {
        let world = compute_world([1.0, 3.0], [2.0, 3.0]);
        for start_flag in [0, 1] { for start_side in [0, 1] {
            for end_flag in [0, 1] { for end_side in [0, 1] {
                let start_enabled = start_flag != 0 || start_side != 0;
                let end_enabled = end_flag != 0 || end_side != 0;
                let result = run_compute_case(&world, utils::curve::Curve2::try_new_from_points(
                    vec![Vec2::ZERO, Vec2::new(4.0, 0.0)], false,
                ).unwrap(), [
                    AlignKind { tag: 0, flag: start_flag, byte_2: start_side, first: Vec2::X, ..AlignKind::default() },
                    AlignKind { tag: 0, flag: end_flag, byte_2: end_side, first: Vec2::NEG_X, ..AlignKind::default() },
                ], [Vec2::new(2.0, 0.0), Vec2::new(0.0, 3.0)],
                    [Vec2::new(-2.0, 0.0), Vec2::new(6.0, 0.0)],
                    StairAssemblyOverrides { tangents: Some(vec![Vec2::X; 2]), widths: None,
                        heights: Some(vec![1.0, 3.0]), sub_segments: Some(StairSubSegments::new()) },
                    &Default::default());
                assert_eq!(result.curve.points.len(), 2 + usize::from(start_enabled) + usize::from(end_enabled));
                if start_enabled {
                    assert_eq!(result.curve.points[0], Vec2::new(-4.0, 0.0));
                    assert_eq!(result.tangents[0], Vec2::new(2.0, 0.0));
                    assert!(!result.terrain_contacts[0]);
                    assert_eq!(result.sub_segments[0], StairSubSegment {
                        kind: start_flag ^ 1, flag: start_side ^ 1, count: 1,
                    });
                }
                if end_enabled {
                    assert_eq!(*result.curve.points.last().unwrap(), Vec2::new(9.0, 0.0));
                    assert_eq!(*result.tangents.last().unwrap(), Vec2::new(0.0, 3.0));
                    assert!(result.terrain_contacts[result.terrain_contacts.len() - 1]);
                    assert_eq!(*result.sub_segments.last().unwrap(), StairSubSegment {
                        kind: end_side ^ 1, flag: end_flag ^ 1, count: 1,
                    });
                }
            }}
        }}
    }

    #[test]
    fn compute_nan_slopes_are_kind_one_and_long_counts_wrap_as_u16() {
        let world = compute_world([0.0, 0.0], [1.0, 1.0]);
        for (count, height) in [(3, f32::NAN), (65537, 0.0)] {
            let curve = utils::curve::Curve2::try_new_from_points(
                (0..count).map(|i| Vec2::new(i as f32, 0.0)).collect(), false,
            ).unwrap();
            let result = run_compute_case(&world, curve, no_padding(), [Vec2::X; 2],
                [Vec2::ZERO, Vec2::new((count - 1) as f32, 0.0)],
                StairAssemblyOverrides { tangents: None, widths: None, heights: Some(vec![height; count]),
                    sub_segments: None }, &Default::default());
            assert_eq!(result.sub_segments.as_slice(), &[
                StairSubSegment { kind: 1, flag: 1, count: (count as u16).wrapping_sub(1) },
            ]);
            assert_eq!(result.curve.points.len(), count);
        }
    }

    #[test]
    #[should_panic]
    fn compute_some_empty_tangents_preserves_required_bounds_failure() {
        let world = compute_world([0.0, 1.0], [1.0, 1.0]);
        run_compute_case(&world, utils::curve::Curve2::try_new_from_points(
            vec![Vec2::ZERO, Vec2::X], false,
        ).unwrap(), no_padding(), [Vec2::X; 2], [Vec2::ZERO, Vec2::X],
            StairAssemblyOverrides { tangents: Some(Vec::new()), ..no_stair_overrides() },
            &Default::default());
    }

    fn turn_alignment() -> AlignKind {
        AlignKind { tag: 1, first: Vec2::X, second: Vec2::Y, ..AlignKind::default() }
    }

    #[test]
    fn alignment_flags_preserve_signed_zero_nan_and_directed_payloads() {
        for (second, expected) in [(Vec2::Y, (0, 1)), (Vec2::NEG_Y, (1, 0)),
            (Vec2::ZERO, (0, 1)), (Vec2::new(0.0, -0.0), (1, 0)),
            (Vec2::splat(f32::NAN), (1, 0))] {
            let mut graph = petgraph::graphmap::DiGraphMap::<_, _, std::hash::RandomState>::default();
            let straight = AlignKind { tag: 0, flag: 9, byte_2: 9, first: Vec2::Y, ..AlignKind::default() };
            let turn = AlignKind { tag: 1, byte_2: 7, first: Vec2::X, second, ..AlignKind::default() };
            let none = AlignKind { tag: 2, flag: 8, byte_2: 8, ..AlignKind::default() };
            graph.add_edge(DecoratorId(29), DecoratorId(7), straight);
            graph.add_edge(DecoratorId(29), DecoratorId(60), turn);
            graph.add_edge(DecoratorId(29), DecoratorId(90), none);
            graph.add_edge(DecoratorId(7), DecoratorId(29), straight);
            propagate_alignment_flags(&mut graph);
            let actual = graph.edge_weight(DecoratorId(29), DecoratorId(7)).unwrap();
            assert_eq!((actual.flag, actual.byte_2), expected);
            assert_eq!(actual.first, Vec2::Y);
            let actual = graph.edge_weight(DecoratorId(29), DecoratorId(60)).unwrap();
            assert_eq!(actual.flag, 1); assert_eq!(actual.byte_2, 7);
            assert_eq!(*graph.edge_weight(DecoratorId(29), DecoratorId(90)).unwrap(), none);
            assert_eq!(*graph.edge_weight(DecoratorId(7), DecoratorId(29)).unwrap(), straight);
        }
    }

    #[test]
    fn alignment_flags_require_straight_and_accumulate_both_turn_sides() {
        let mut graph = petgraph::graphmap::DiGraphMap::<_, _, std::hash::RandomState>::default();
        let straight = AlignKind { tag: 0, first: Vec2::Y, ..AlignKind::default() };
        graph.add_edge(DecoratorId(1), DecoratorId(2), turn_alignment());
        graph.add_edge(DecoratorId(1), DecoratorId(3), AlignKind { second: Vec2::NEG_Y, ..turn_alignment() });
        graph.add_edge(DecoratorId(9), DecoratorId(10), straight);
        propagate_alignment_flags(&mut graph);
        assert_eq!(graph.edge_weight(DecoratorId(1), DecoratorId(2)).unwrap().flag, 0);
        assert_eq!(*graph.edge_weight(DecoratorId(9), DecoratorId(10)).unwrap(), straight);
        graph.add_edge(DecoratorId(1), DecoratorId(4), straight);
        propagate_alignment_flags(&mut graph);
        let actual = graph.edge_weight(DecoratorId(1), DecoratorId(4)).unwrap();
        assert_eq!((actual.flag, actual.byte_2), (1, 1));
    }

    fn rectangular_wall() -> (PublicWalls, country_core::resources::walls::WallId) {
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::resources::walls::{InnerWalls, WallState};
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let rectangle = utils::geometry::Rectangle2d { axis: Vec2::X, center: Vec2::ZERO, size: Vec2::new(4.0, 6.0) };
        let mut points: Vec<_> = rectangle.as_points2().into_iter().map(|p| p.to_array()).collect();
        points.push(points[0]);
        let curve = Curve2::try_new(points, false).unwrap();
        let mut snapshot = WallState::new(curve, 4.0, 0, 0, false).snapshot().clone();
        snapshot.source.shape = ShapeParameters::from_shape(Shape::Rectangle(rectangle)).to_snapshot();
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::from_snapshot(snapshot, 0));
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume(&mut inner, &terrain);
        (walls, id)
    }

    #[test]
    fn rectangle_walk_is_the_real_perimeter_stream_and_reverses_all_outputs() {
        use utils::wall_space::WallSpaceCoord;
        let (walls, id) = rectangular_wall();
        let wall = walls.get(id).unwrap();
        let length = wall.wall_space_length();
        let forward = walk_shape(wall, [turn_alignment(); 2], [Vec2::splat(f32::NAN); 2],
            [1.0, 2.0], [WallSpaceCoord::new_unchecked(length * 0.1, 0.0), WallSpaceCoord::new_unchecked(length * 0.4, 4.0)]);
        let backward = walk_shape(wall, [turn_alignment(); 2], [Vec2::ZERO; 2],
            [2.0, 1.0], [WallSpaceCoord::new_unchecked(length * 0.4, 4.0), WallSpaceCoord::new_unchecked(length * 0.1, 0.0)]);
        assert!(forward.points.len() > 2);
        assert!(forward.points.iter().copied().eq(backward.points.iter().rev().copied()));
        assert!(forward.tangents.as_ref().unwrap().iter().copied()
            .eq(backward.tangents.as_ref().unwrap().iter().rev().map(|p| -*p)));
        assert!(forward.widths.as_ref().unwrap().iter().eq(backward.widths.as_ref().unwrap().iter().rev()));
        assert!(forward.heights.as_ref().unwrap().iter().eq(backward.heights.as_ref().unwrap().iter().rev()));
        assert!(forward.sub_segments.as_ref().unwrap().iter().eq(backward.sub_segments.as_ref().unwrap().iter().rev()));
    }

    #[test]
    fn rectangle_inset_crossing_reverts_both_endpoints_together() {
        use utils::wall_space::WallSpaceCoord;
        let (walls, id) = rectangular_wall();
        let wall = walls.get(id).unwrap(); let length = wall.wall_space_length();
        let coords = [WallSpaceCoord::new_unchecked(length * 0.1, 0.0), WallSpaceCoord::new_unchecked(length * 0.15, 2.0)];
        let a = walk_shape(wall, [turn_alignment(); 2], [Vec2::X; 2], [10.0; 2], coords);
        let b = walk_shape(wall, [AlignKind { flag: 1, ..turn_alignment() }; 2], [Vec2::X; 2], [10.0; 2], coords);
        assert_eq!(a.points, b.points);
        assert_eq!(a.widths, b.widths); assert_eq!(a.heights, b.heights);
    }

    #[test]
    fn generic_walk_uses_source_curve_and_central_tangent_and_has_real_empty_gates() {
        use utils::wall_space::WallSpaceCoord;
        let (walls, id) = published_wall(); let wall = walls.get(id).unwrap();
        let coords = [WallSpaceCoord::new_unchecked(1.0, 3.0), WallSpaceCoord::new_unchecked(7.0, 8.0)];
        let result = walk_shape(wall, [turn_alignment(); 2], [Vec2::X, Vec2::Y], [0.0; 2], coords);
        assert_eq!(result.points.len(), 7);
        assert_eq!(result.points[3], Vec2::new(4.0, 0.0));
        let tangent = result.tangents.unwrap()[3];
        assert!((tangent - Vec2::ONE.normalize()).length() < 0.000001);
        assert_ne!(tangent, Vec2::Y);
        assert!(result.sub_segments.is_none()); assert!(result.heights.is_none());
        let no_first = walk_shape(wall, [AlignKind::default(), turn_alignment()], [Vec2::X; 2], [1.0; 2], coords);
        assert!(no_first.points.is_empty() && no_first.widths.is_none());
        let no_second = walk_shape(wall, [turn_alignment(), AlignKind::default()], [Vec2::X; 2], [1.0; 2], coords);
        assert!(no_second.points.is_empty() && no_second.tangents.is_none());
        let tiny = walk_shape(wall, [turn_alignment(); 2], [Vec2::X; 2], [1.0; 2],
            [coords[0], WallSpaceCoord::new_unchecked(1.0001, 0.0)]);
        assert!(tiny.points.is_empty());
    }

    fn receiver(need_height: bool) -> RectangleStairReceiver {
        RectangleStairReceiver { endpoint_widths: [0.0, 0.0], endpoint_heights: [0.0, 8.0],
            need_height, previous: None, points: Vec::new(), tangents: Vec::new(), widths: Vec::new(),
            heights: Vec::new(), sub_segments: StairSubSegments::new() }
    }

    fn slice_point(x: f32, u: f32, tag: u8) -> utils::geometry::rectangle::RectangleSlicePoint {
        utils::geometry::rectangle::RectangleSlicePoint { position: Vec2::new(x, 0.0),
            normal: Vec2::ZERO, relative_u: u, tag }
    }

    #[test]
    fn rectangle_receiver_preserves_height_gate_dedup_steep_replace_and_nan() {
        let mut flat = receiver(false);
        flat.receive(slice_point(0.0, 0.0, 5)); flat.receive(slice_point(0.0, 1.0, 4));
        assert_eq!(flat.points.len(), 1); assert!(flat.previous.is_none());
        assert!(flat.heights.is_empty() && flat.sub_segments.is_empty());
        let mut steep = receiver(true);
        for (x, u) in [(0.0, 0.0), (1.0, 0.5), (2.0, 1.0)] { steep.receive(slice_point(x, u, 5)); }
        assert_eq!(steep.points, vec![Vec2::ZERO, Vec2::new(2.0, 0.0)]);
        assert_eq!(steep.heights, vec![0.0, 8.0]);
        assert_eq!(steep.sub_segments.as_slice(), &[StairSubSegment { kind: 3, flag: 1, count: 1 }]);
        let mut nan = receiver(true); nan.endpoint_heights = [f32::NAN; 2];
        nan.receive(slice_point(0.0, 0.0, 5)); nan.receive(slice_point(1.0, 1.0, 5));
        assert_eq!(nan.sub_segments.as_slice(), &[StairSubSegment { kind: 1, flag: 1, count: 1 }]);
        nan.receive(slice_point(1.0, 0.5, 4));
        assert_eq!(nan.points.len(), 2);
        nan.receive(slice_point(2.0, 1.0, 4));
        assert_eq!(nan.sub_segments.last().unwrap().kind, 4);
    }

    #[test]
    fn generic_strip_repeatedly_prunes_concave_interior_and_preserves_endpoints() {
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::resources::walls::{InnerWalls, WallState};
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use utils::wall_space::WallSpaceCoord;
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0], [4.0, 1.0], [0.0, 1.0]], false).unwrap();
        let mut inner = InnerWalls::default();
        let id = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default(); walls.consume(&mut inner, &terrain);
        let wall = walls.get(id).unwrap();
        for side in [0, 1] {
            let result = walk_shape(wall, [AlignKind { byte_2: side, ..turn_alignment() }; 2],
                [Vec2::X, Vec2::NEG_X], [4.0; 2],
                [WallSpaceCoord::new_unchecked(0.0, 0.0), WallSpaceCoord::new_unchecked(9.0, 1.0)]);
            assert!(result.points.len() >= 2 && result.points.len() < 10);
            let sign = if side == 0 { 1.0 } else { -1.0 };
            assert_eq!(result.points[0], Vec2::new(0.0, 2.0 * sign));
            assert_eq!(*result.points.last().unwrap(), Vec2::new(0.0, 1.0 - 2.0 * sign));
            assert_eq!(result.points.len(), result.tangents.as_ref().unwrap().len());
            assert_eq!(result.points.len(), result.widths.as_ref().unwrap().len());
        }
    }

    #[test]
    fn rectangle_receiver_kind_four_wraps_original_u16_count() {
        let mut state = receiver(true);
        state.receive(slice_point(0.0, 0.0, 4)); state.receive(slice_point(1.0, 0.1, 4));
        assert_eq!(state.sub_segments[0].kind, 4);
        state.sub_segments[0].count = u16::MAX;
        state.receive(slice_point(2.0, 0.2, 4));
        assert_eq!(state.sub_segments[0].count, 0);
        assert_eq!(state.points.len(), 3);
    }

}
