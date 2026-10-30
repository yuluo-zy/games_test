//! 玩家楼梯的图到几何计算，沿既有模块逐段恢复原 B 业务。
//!
//! 已按完整原码恢复权威图、对齐、形状/样条、失败回退到本轮装配请求。
//! 本轮装饰节点已恢复原交互蓝图、轮廓及墙洞；实际宿主 specialization 与图编辑仍需各自闭包；
//! 不通过空资源、默认请求或本地简化样条代替尚未接线的上游。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::{
    AdjustedStairEdge, AlignKind, PlayermadeStairsAssemblyParams, PlayermadeStairsAssemblyRequests,
    StairNodeAnchor, StairWallAnchor,
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

/// 原 14080beb0：按实际墙法线与权威地址生成有序偏好，不提前排序或修复 NaN。
fn find_alignment_preferences(
    from: &StairPoint, to: &StairPoint, delta: Vec2,
    storage: &DecoratorStorage, walls: &PublicWalls,
) -> Vec<(AlignKind, f32)> {
    use utils::comparative_space::ComparativeSpace;
    let Some(normal) = from.wall_normal else { return Vec::new(); };
    let from_info = storage.get(from.addr).and_then(|bucket| bucket.get(from.id))
        .expect("alignment preferences: missing source decorator");
    from_info.dst_as_wall();
    let from_tag = from_info.dst_tag();
    let from_u = from_info.wall_space_coord().u;
    let to_info = storage.get(to.addr).and_then(|bucket| bucket.get(to.id))
        .expect("alignment preferences: missing target decorator");
    let from_wall = from.addr.as_wall();
    let same_wall = matches!(to.addr, DecoratorStorageAddr::Wall(wall) if wall == from_wall)
        && to_info.dst_tag() < 2;
    if !same_wall {
        let mut options = Vec::with_capacity(3);
        let direction = delta.normalize_or_zero();
        let sign = (direction.y * normal.x - direction.x * normal.y).signum();
        let turn = Vec2::new((-normal.y) * sign, normal.x * sign);
        let straight_dot = direction.y * normal.y + direction.x * normal.x;
        let turn_dot = direction.y * turn.y + direction.x * turn.x;
        let factor = if straight_dot > 0.0 { f32::from_bits(0x3ed4_13cd) } else { 1.0 };
        options.push((AlignKind { tag: 0, first: normal, ..AlignKind::default() }, straight_dot * factor));
        options.push((AlignKind { tag: 1, byte_2: from_tag ^ 1, first: normal, second: turn,
                                  ..AlignKind::default() }, turn_dot));
        // 原第三项直接 XOR 两个符号位与 score，不重新求 dot。
        options.push((AlignKind { tag: 1, byte_2: from_tag ^ 1, first: normal, second: -turn,
                                  ..AlignKind::default() }, -turn_dot));
        return options;
    }
    let square_length = delta.y * delta.y + delta.x * delta.x;
    if square_length < f32::from_bits(0x3586_37bd) {
        return vec![(AlignKind { tag: 0, first: normal, ..AlignKind::default() }, 1.0)];
    }
    let wall = walls.get(from_wall).expect("alignment preferences: missing wall");
    let space = utils::wall_space::WallSpace {
        is_loop: wall.shape().tag() != 2, length: wall.wall_space_length(),
    };
    let difference = space.subtract(to_info.wall_space_coord().u, from_u);
    let mut options = Vec::with_capacity(1);
    // 原先 signum 再取负；(-difference).signum() 会改变 NaN 的符号。
    let sign = -difference.signum();
    let mut turn = Vec2::new((-normal.y) * sign, normal.x * sign);
    if from_tag == 0 { turn = -turn; }
    options.push((AlignKind { tag: 1, byte_2: from_tag ^ 1, first: normal, second: turn,
                              ..AlignKind::default() }, 1.0));
    options
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

/// 原 closure1 的 28 字节临时值；这不是发布给下游的 AdjustedStairEdge。
fn aligned_stair_control_point(
    points: &std::collections::HashMap<DecoratorId, StairPoint>,
    alignments: &petgraph::graphmap::DiGraphMap<DecoratorId, AlignKind, std::hash::RandomState>,
    from: DecoratorId,
    to: DecoratorId,
) -> (Vec2, AlignKind) {
    let point = points.get(&from).expect("stair control point: missing point");
    let alignment = *alignments.edge_weight(from, to)
        .expect("stair control point: missing alignment");
    // 原 140285a90 在复制 kind 后再次 required 借同一条有向边，才求宽度偏移。
    let offset = alignments.edge_weight(from, to)
        .expect("stair control point: missing alignment").position_offset(point.width);
    (Vec2::new(point.position.x, point.position.z) + offset, alignment)
}

/// 同一帧真实 owner 的借用分组；不建立另一份墙、缓存或高度权威。
struct StairRequestSources<'a> {
    storage: &'a DecoratorStorage,
    cached: &'a CachedDecoratorTransforms,
    walls: &'a PublicWalls,
    stairs: &'a StairsState,
    terrain: &'a TerrainHeightsData<VisualTerrainHeightSsbo>,
}

/// 原 source983..1078 的短出射请求，同时供孤点和整条路径失败后的首边使用。
fn make_short_stair_request(
    sources: &StairRequestSources<'_>,
    points: &std::collections::HashMap<DecoratorId, StairPoint>,
    endpoint_ids: [DecoratorId; 2],
    dirty_edges: &std::collections::HashSet<StairGraphEdge>,
) -> Option<PlayermadeStairsAssemblyParams> {
    let [a, b] = endpoint_ids;
    let from = points.get(&a).expect("short stair request: missing first point");
    let to = points.get(&b).expect("short stair request: missing second point");
    let (direction, height_extra, horizontal_extra) = if let Some(normal) = from.wall_normal {
        (Vec3::new(normal.x, 0.0, normal.y), -1.0, -0.0)
    } else {
        // 原 basis 保留两个负零，且不把有限但未归一的缓存 Quat 擅自归一化。
        (sources.cached.transform(a.0).rotation * Vec3::new(-0.0, -0.0, -1.0), 1.0, 0.0)
    };
    let start = Vec2::new(from.position.x, from.position.z);
    let direction_xz = Vec2::new(direction.x, direction.z);
    let end = (start + direction_xz) + Vec2::splat(horizontal_extra);
    // 此处唯一的 None 就是原两点 CurveTry(true) 的 Err；不能改用 compute 默认值。
    let curve = utils::curve::Curve2::try_new_from_points(vec![start, end], true).ok()?;
    let heights = vec![from.position.y, (from.position.y + direction.y) + height_extra];
    let widths = vec![from.width; 2];
    let tangents = vec![direction_xz.normalize_or_zero(); 2];
    let (kind, flag) = determine_segment_type(&curve.points, &heights);
    let mut sub_segments = StairSubSegments::new();
    sub_segments.push(StairSubSegment { kind, flag, count: 1 });
    let mut terrain_contacts = bitvec::vec::BitVec::<usize, bitvec::order::Lsb0>::repeat(false, 2);
    // 原只采中心点并作 ordered 严格比较；没有左右半宽、-.025 或抬高 height。
    terrain_contacts.set(0, sources.terrain.sample_at_world_xz(start.x, start.y) + 0.1 > heights[0]);
    terrain_contacts.set(1, sources.terrain.sample_at_world_xz(end.x, end.y) + 0.1 > heights[1]);
    let segment = StairSegment::new(a, b);
    let first_info = sources.storage.get(from.addr).and_then(|bucket| bucket.get(from.id))
        .expect("short stair request: missing first decorator");
    let first_node = derive_stair_node_params(first_info, sources.walls, sources.stairs, from.addr, from.id);
    // 原 281fdc 派生首端以后，282007 才 required 读取第二端；保留失败先后。
    let second_info = sources.storage.get(to.addr).and_then(|bucket| bucket.get(to.id))
        .expect("short stair request: missing second decorator");
    let second_node = derive_stair_node_params(second_info, sources.walls, sources.stairs, to.addr, to.id);
    let dirty = dirty_edges.contains(&StairGraphEdge::from(segment));
    Some(PlayermadeStairsAssemblyParams {
        curve, heights, widths, tangents,
        auxiliary_samples: Vec::new(), interpolated_curve: None,
        nodes: [first_node, second_node], terrain_contacts, segment, sub_segments, dirty,
    })
}

/// 原14080478x..4960：墙附着节点按完整有向邻接生成四类交互外形。
/// 返回(原四项判别、局部Z偏移、是否允许墙洞)，不把None邻接漏出计数。
fn stair_decorator_interaction_kind(
    address: DecoratorStorageAddr,
    id: DecoratorId,
    width: f32,
    requests: &PlayermadeStairsAssemblyRequests,
) -> (u8, f32, bool) {
    if address.try_as_wall().is_none() { return (0, 0.0, false); }
    let mut outgoing = 0usize;
    let mut straight = false;
    let mut turn = false;
    let mut flags = 0u8;
    for (_, _, alignment) in requests.alignments.edges_directed(id, petgraph::Direction::Outgoing) {
        outgoing += 1;
        if alignment.tag != 2 {
            if alignment.tag == 0 {
                straight = true;
                flags |= alignment.flag | alignment.byte_2;
            } else {
                turn = true;
                flags |= alignment.flag;
            }
        }
    }
    if flags & 1 != 0 { return (3, width * 0.5, false); }
    if straight {
        assert!(!(turn && outgoing != 0), "assertion failed: !any_normal");
        return (1, 0.5, true);
    }
    if outgoing == 0 { (1, 0.5, true) }
    else if turn { (2, width * 0.5, false) }
    else { (0, 0.0, false) }
}

fn stair_decorator_interaction_dimensions(kind: u8, width: f32) -> Vec3 {
    let expanded = width + f32::from_bits(0x3dcc_cccd);
    match kind {
        0 => Vec3::new(expanded + f32::from_bits(0x3dcc_cccd), 1.0, 0.95),
        1 => Vec3::new(expanded + f32::from_bits(0x3dcc_cccd), 1.0, 1.0),
        2 => Vec3::new(0.65, 1.0, expanded + f32::from_bits(0x3dcc_cccd)),
        3 => Vec3::new(expanded, 0.65, expanded),
        _ => unreachable!("原四项stair interaction跳表"),
    }
}

/// 原SSE四元数向量乘法的完整非单位公式；不归一化或使用单位四元数简式。
fn rotate_stair_decorator_local(rotation: glam::Quat, local: Vec3) -> Vec3 {
    let q = rotation;
    let length_sq = q.z * q.z + (q.y * q.y + q.x * q.x);
    let dot = q.z * local.z + (q.y * local.y + q.x * local.x);
    let twice_dot = dot + dot;
    let scalar = q.w * q.w - length_sq;
    let twice_w = q.w + q.w;
    Vec3::new(
        twice_w * (q.y * local.z - q.z * local.y) + (twice_dot * q.x + scalar * local.x),
        twice_w * (q.z * local.x - q.x * local.z) + (twice_dot * q.y + scalar * local.y),
        twice_w * (q.x * local.y - q.y * local.x) + (twice_dot * q.z + scalar * local.z),
    )
}

/// 原1408050b5..52eb：只有真实平顶规则建筑上的墙节点才补顶端缺口。
fn stair_decorator_wall_hole(
    address: DecoratorStorageAddr,
    info: &DecoratorInfo,
    width: f32,
    walls: &PublicWalls,
    roofs: &Query<&country_core::resources::roofs::roof_shape::Roof>,
) -> Option<country_core::resources::walls::wall_holes::RawWallHole> {
    let wall_id = address.try_as_wall()?;
    let wall = walls.get(wall_id)?;
    let roof = roofs.get(walls.roof_entity(wall_id)?).ok()?;
    if (2..=4).contains(&info.dst_tag()) || !wall.is_premade_shape() || !roof.is_flat() { return None; }
    // 原flat style的Option niche3回到Matching，而非把None当Override。
    let bytes: [u8; 12] = roof.observed()[0x40..0x4c].try_into().expect("原Roof flat style");
    let tag = if bytes[0] == 3 { 0 } else { bytes[0] };
    let wooden = match tag {
        0 => wall.building_projection().is_halftimbered(),
        1 => false,
        _ => bytes[1] & 1 != 0,
    };
    let maximum_y = wall.max_y();
    let coord = info.wall_space_coord();
    if !(maximum_y - coord.y < f32::from_bits(0x3f59_999a)) { return None; }
    let rim = if wooden { f32::from_bits(0x3f42_8f5d) } else { f32::from_bits(0x3f28_f5c3) };
    Some(country_core::resources::walls::wall_holes::RawWallHole {
        center: [coord.u, (maximum_y - rim) + 5.0], dimensions: [width, 10.0], kind: 4,
    })
}

/// 原140803dd0/9024B：生成节点的交互primitive/compound、描边和必要墙洞。
/// 楼梯砖视觉由独立stairs_assemble消费assembly requests，不在此发明第二套模型。
pub fn generate_playermade_stair_decorators(
    storage: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    requests: Res<PlayermadeStairsAssemblyRequests>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    walls: Res<PublicWalls>,
    roofs: Query<&country_core::resources::roofs::roof_shape::Roof>,
    _signifiers: Res<country_core::systems::ui::signifiers::UiSignifierStream>,
    demo: Res<country_core::systems::backend::input::translate_input::DemoModeEnabled>,
) {
    // 原全局三态只在值1时返回；现代宿主借用与请求producer相同的唯一Demo owner。
    if demo.0 { return; }
    let buckets = storage.iter_walls().map(|(wall, bucket)| (DecoratorStorageAddr::Wall(wall), bucket))
        .chain(storage.iter_terrain_decorators().map(|bucket| (DecoratorStorageAddr::Terrain, bucket)))
        .chain(storage.iter_stair_decorators().map(|bucket| (DecoratorStorageAddr::Stair, bucket)));
    for (address, bucket) in buckets {
        for (raw_id, info) in bucket.iter_modified() {
            if info.ty != b'?' { continue; }
            let id = DecoratorId(raw_id);
            let transform = cached.transform(raw_id);
            let blueprint = blueprints.insert_for(raw_id, address, info.state());
            // 原1408047bd：insert返回Arc+16的数据指针，data+129即Arc+139独立交互位。
            blueprint.interaction_source_flags = 1;
            let extra = info.extra().as_stair();
            let height = f32::from_le_bytes(extra.0[..4].try_into().expect("原楼梯高度"));
            let width = f32::from_le_bytes(extra.0[4..8].try_into().expect("原楼梯宽度"));
            let (kind, offset, creates_wall_hole) = stair_decorator_interaction_kind(address, id, width, &requests);
            let mut outline = transform;
            outline.translation += rotate_stair_decorator_local(transform.rotation, Vec3::new(0.0, 0.25, offset));
            blueprint.set_cached_transform(outline);
            let expanded = width + f32::from_bits(0x3dcc_cccd);
            let dimensions = stair_decorator_interaction_dimensions(kind, width);
            let base = country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(dimensions, Vec3::new(0.0, 0.0, offset));
            let mesh = if (2..=3).contains(&info.dst_tag()) {
                let panel_y = (-0.25 - height) + 0.5;
                let panel = country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(
                    Vec3::new(expanded + f32::from_bits(0x3dcc_cccd), 1.0, 1.0),
                    Vec3::new(panel_y * 0.0, panel_y, panel_y * 0.0),
                );
                let top = country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(
                    Vec3::new(0.87, 1.8, 0.3), Vec3::new(0.0, 1.3, offset + 0.0),
                );
                // 原Vec为base/top/panel，hash折叠顺序不能交换；外层为identity pose。
                country_core::systems::collision::world::RaycastMeshData::new_compound(
                    vec![base, top, panel], [0, 0, 0, 0, 0, 0, 0, 1.0f32.to_bits()],
                )
            } else {
                if info.dst_tag() == 4 { unreachable!(); }
                base
            };
            blueprint.set_interaction_mesh(std::sync::Arc::new(mesh), &transform.translation, &transform.rotation);
            if creates_wall_hole {
                if let Some(hole) = stair_decorator_wall_hole(address, info, width, &walls, &roofs) {
                    blueprint.wall_holes.push(hole);
                }
            }
            // 原80531x明确unset collision，节点只保留上面的交互mesh。
            blueprint.collision_mesh = crate::blueprint::BlueprintMeshSlot::Disabled;
        }
    }
}

/// 原 false140276e80 / true14027e470：权威图到完整本轮请求和双向几何图。
/// X 只控制旧 dirty 的继承；实际宿主 specialization 由原注册链另行绑定。
pub fn generate_playermade_stair_requests<const X: bool>(
    storage: Res<DecoratorStorage>,
    cached: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData<VisualTerrainHeightSsbo>>,
    stairs: Res<StairsState>,
    mut requests: ResMut<PlayermadeStairsAssemblyRequests>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    demo: Res<country_core::systems::backend::input::translate_input::DemoModeEnabled>,
) {
    // 现代 adapter 复用 App 的唯一 Demo owner，原 body 入口即返回。
    if demo.0 { return; }
    let old_dirty = if X {
        requests.requests.iter().filter(|request| request.dirty)
            .map(|request| StairGraphEdge::from(request.segment)).collect()
    } else { std::collections::HashSet::new() };
    requests.requests.clear();
    let points = collect_stair_points(&storage, &cached, &walls, &stairs);
    let mut alignments = std::mem::take(&mut requests.alignments);
    let mut adjusted_edges = std::mem::take(&mut requests.adjusted_edges);
    // 原 GraphMap::clear 清节点和边并保 allocation，不能只清边或换新图。
    alignments.clear();
    adjusted_edges.clear();
    for raw_id in stairs.graph().nodes() {
        let id = DecoratorId(raw_id);
        let neighbors: Vec<_> = stairs.graph().neighbors(raw_id).collect();
        let mut candidates = Vec::new();
        for raw_neighbor in neighbors {
            let neighbor = DecoratorId(raw_neighbor);
            let from = points.get(&id).expect("stair requests: missing source point");
            let to = points.get(&neighbor).expect("stair requests: missing neighbor point");
            let delta = Vec2::new(to.position.x - from.position.x, to.position.z - from.position.z);
            let preferences = find_alignment_preferences(from, to, delta, &storage, &walls);
            if preferences.is_empty() {
                // 反向节点自有偏好；这里不能顺手覆盖 neighbor→id。
                alignments.add_edge(id, neighbor, AlignKind { tag: 2, ..AlignKind::default() });
            } else {
                candidates.push((neighbor, preferences));
            }
        }
        for (neighbor, alignment) in find_best_neighbor_alignments(&candidates, 0).0 {
            alignments.add_edge(id, neighbor, alignment);
        }
    }
    propagate_alignment_flags(&mut alignments);
    let sources = StairRequestSources {
        storage: &storage, cached: &cached, walls: &walls, stairs: &stairs, terrain: &terrain,
    };
    let mut remaining = stairs.graph().clone();
    let mut path = std::collections::VecDeque::new();
    let mut control_points = Vec::new();
    let mut failed_edges = Vec::new();
    // 原每请求 Bump、逐路径 reset；保用户 owned Vec Spline，复用本轮 scratch Vec。
    // owned Spline 的 allocation retention 与原 Bump 不同，不能声称 allocator 等价。
    loop {
        let seed = remaining.all_edges().next().map(|(a, b, _)| (a, b));
        let Some((raw_a, raw_b)) = seed else { break; };
        path.clear();
        control_points.clear();
        remaining.remove_edge(raw_a, raw_b);
        path.push_back(DecoratorId(raw_a));
        path.push_back(DecoratorId(raw_b));
        let mut front_is_wall = false;
        let mut back_is_wall = false;
        let mut closed = false;
        loop {
            let a = *path.front().expect("stair requests: empty path front");
            if storage.lookup_decorator_info(a).is_some_and(|info| info.is_wall_dst()) {
                front_is_wall = true;
                break;
            }
            if a == *path.back().expect("stair requests: empty path back") {
                closed = true;
                break;
            }
            let Some(b) = remaining.neighbors(a.0).next() else { break; };
            remaining.remove_edge(a.0, b);
            path.push_front(DecoratorId(b));
        }
        loop {
            let a = *path.back().expect("stair requests: empty path back");
            if storage.lookup_decorator_info(a).is_some_and(|info| info.is_wall_dst()) {
                back_is_wall = true;
                break;
            }
            if a == *path.front().expect("stair requests: empty path front") {
                closed = true;
                break;
            }
            let Some(b) = remaining.neighbors(a.0).next() else { break; };
            remaining.remove_edge(a.0, b);
            path.push_back(DecoratorId(b));
        }
        if front_is_wall && back_is_wall && path.len() == 2 {
            let a = path[0].min(path[1]);
            let b = path[0].max(path[1]);
            let from = points.get(&a).expect("wall stair request: missing first point");
            let to = points.get(&b).expect("wall stair request: missing second point");
            if let (DecoratorStorageAddr::Wall(first_wall), DecoratorStorageAddr::Wall(second_wall)) = (from.addr, to.addr) {
                if first_wall == second_wall {
                    let first_info = storage.get(from.addr).and_then(|bucket| bucket.get(a))
                        .expect("wall stair request: missing first decorator");
                    let second_info = storage.get(to.addr).and_then(|bucket| bucket.get(b))
                        .expect("wall stair request: missing second decorator");
                    // 原 wrapping 门只排除已知2/3/4；不能吞未知 tag 的后续失败。
                    if first_info.dst_tag().wrapping_sub(5) <= 252 && second_info.dst_tag().wrapping_sub(5) <= 252 {
                        let (first_position, first_alignment) = aligned_stair_control_point(&points, &alignments, a, b);
                        let (second_position, second_alignment) = aligned_stair_control_point(&points, &alignments, b, a);
                        let first_direction = first_alignment.outgoing_dir();
                        let second_direction = second_alignment.outgoing_dir();
                        adjusted_edges.add_edge(a, b, AdjustedStairEdge { position: first_position, outgoing_dir: first_direction });
                        adjusted_edges.add_edge(b, a, AdjustedStairEdge { position: second_position, outgoing_dir: second_direction });
                        let nodes = [derive_stair_node_params(first_info, &walls, &stairs, from.addr, a),
                            derive_stair_node_params(second_info, &walls, &stairs, to.addr, b)];
                        let wall = walls.get(from.addr.as_wall()).expect("wall stair request: missing wall");
                        let output = walk_shape(wall, [first_alignment, second_alignment],
                            [first_direction, -second_direction], [from.width, to.width],
                            [first_info.wall_space_coord(), second_info.wall_space_coord()]);
                        match utils::curve::Curve2::try_new_from_points(output.points, true) {
                            Ok(curve) => requests.requests.push(compute_stair_assembly_params(
                                StairAssemblyInput { nodes, points: [from, to],
                                    alignments: [first_alignment, second_alignment],
                                    positions: [first_position, second_position], directions: [first_direction, -second_direction],
                                    ordered_ids: [a, b], curve },
                                StairAssemblyOverrides { tangents: output.tangents, widths: output.widths,
                                    heights: output.heights, sub_segments: output.sub_segments }, &terrain, &old_dirty,
                            )),
                            Err(_) => failed_edges.push([a, b]),
                        }
                        continue;
                    }
                }
            }
        }
        let mut start_tangent = None;
        for (&from, &to) in path.iter().zip(path.iter().skip(1)) {
            let (position, alignment) = aligned_stair_control_point(&points, &alignments, from, to);
            if !closed && control_points.is_empty() {
                start_tangent = match alignment.tag {
                    0 => Some(alignment.first), 1 => Some(alignment.second), _ => None,
                };
            }
            control_points.push(position);
        }
        let spline = if closed {
            // 原 caller source817 在 ctor 前独立断言，并 Debug 输出真实 ID path。
            assert!(control_points.len() >= 3, "A loop must have at least 3 points; {path:?}");
            let spline = Spline2D::from_periodic_loop(&control_points);
            let first = *control_points.first().expect("closed stair path: missing first control point");
            control_points.push(first);
            spline
        } else {
            let a = *path.back().expect("open stair path: missing last node");
            let b = *path.get(path.len() - 2).expect("open stair path: missing previous node");
            let (last, alignment) = aligned_stair_control_point(&points, &alignments, a, b);
            let end_tangent = match alignment.tag {
                0 => Some(-alignment.first), 1 => Some(-alignment.second), _ => None,
            };
            control_points.push(last);
            Spline2D::from_control_points_with_tangents(&control_points, start_tangent, end_tangent)
        };
        let mut any_success = false;
        for (index, control_pair) in control_points.windows(2).enumerate() {
            let mut dense = densely_sample_spline_segment(&spline, index);
            let mut a = *path.get(index).expect("stair spline: missing first node");
            let mut b = *path.get(index + 1).expect("stair spline: missing second node");
            let mut positions = [control_pair[0], control_pair[1]];
            let mut outgoing = [dense.start_tangent, dense.end_tangent];
            if a > b {
                std::mem::swap(&mut a, &mut b);
                dense.points.reverse();
                positions.reverse();
                outgoing.reverse();
            }
            let Ok(curve) = utils::curve::Curve2::try_new_from_points(dense.points, true) else { continue; };
            let from = points.get(&a).expect("stair spline: missing first point");
            let to = points.get(&b).expect("stair spline: missing second point");
            let first_alignment = *alignments.edge_weight(a, b).expect("stair spline: missing first alignment");
            let second_alignment = *alignments.edge_weight(b, a).expect("stair spline: missing second alignment");
            adjusted_edges.add_edge(a, b, AdjustedStairEdge { position: positions[0], outgoing_dir: outgoing[0] });
            adjusted_edges.add_edge(b, a, AdjustedStairEdge { position: positions[1], outgoing_dir: outgoing[1] });
            let first_info = storage.get(from.addr).and_then(|bucket| bucket.get(a))
                .expect("stair spline: missing first decorator");
            let second_info = storage.get(to.addr).and_then(|bucket| bucket.get(b))
                .expect("stair spline: missing second decorator");
            let nodes = [derive_stair_node_params(first_info, &walls, &stairs, from.addr, a),
                derive_stair_node_params(second_info, &walls, &stairs, to.addr, b)];
            requests.requests.push(compute_stair_assembly_params(
                StairAssemblyInput { nodes, points: [from, to], alignments: [first_alignment, second_alignment],
                    positions, directions: [outgoing[0], -outgoing[1]], ordered_ids: [a, b], curve },
                StairAssemblyOverrides { tangents: None, widths: None, heights: None, sub_segments: None },
                &terrain, &old_dirty,
            ));
            any_success = true;
        }
        // 原不是每段 Err 均补短请求：只有整条 path 没有产出才补首 pair 一次。
        if !any_success {
            if let (Some(&a), Some(&b)) = (path.front(), path.get(1)) {
                failed_edges.push([a, b]);
            }
        }
    }
    // 孤点取原权威图，不能用已经消完所有边的 remaining 图。
    for raw_id in stairs.graph().nodes() {
        if stairs.graph().neighbors(raw_id).next().is_none() {
            if let Some(request) = make_short_stair_request(&sources, &points, [DecoratorId(raw_id); 2], &old_dirty) {
                requests.requests.push(request);
            }
        }
    }
    for endpoint_ids in failed_edges {
        if let Some(request) = make_short_stair_request(&sources, &points, endpoint_ids, &old_dirty) {
            requests.requests.push(request);
        }
    }
    let dirty_nodes: std::collections::HashSet<_> = requests.requests.iter().filter(|request| request.dirty)
        .flat_map(|request| { let (a, b) = request.segment.nodes(); [a, b] }).collect();
    if !dirty_nodes.is_empty() {
        for request in &mut requests.requests {
            let (a, b) = request.segment.nodes();
            if dirty_nodes.contains(&a) || dirty_nodes.contains(&b) { request.dirty = true; }
        }
    }
    requests.alignments = alignments;
    requests.adjusted_edges = adjusted_edges;
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

/// 原140806110：九组资源维护同一楼梯图、碰撞与归档；附加资源只承接
/// core::_DEV_OVERRIDE_IS_DEMO_MODE@143634d90，不是开发者热键@143634cd8。
/// 失效节点先入栈，记录态逐个剔除获准的叶邻居，之后按原lane0识别过期边。
#[allow(clippy::too_many_arguments)]
pub fn maintain_stair_graph(
    history: Res<country_core::resources::history::History>,
    physics: Res<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    mut stairs: ResMut<StairsState>,
    mut structural: ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,
    mut raycast: ResMut<country_core::systems::collision::world::RaycastWorld>,
    mut colliders: ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
    demo: Res<country_core::DevOverrideIsDemoMode>,
) {
    use country_core::systems::collision::world::{dispose_collider_mesh_instance, remove_collider};
    // 原140806149..153：main从dev.demo写入的bool非零才返回，不依赖internal_hotkeys。
    if demo.0 { return; }
    let recording = history.is_recording();
    let mut culled = Vec::new();
    // 原try_lookup miss可剔除；索引命中却无桶必须在任何图修改前panic。
    let mut pending: Vec<_> = stairs.graph().nodes().filter(|&id| {
        match storage.try_lookup_storage(DecoratorId(id)) {
            None => true,
            Some(addr) => storage.get(addr).expect("maintain_stair_graph: missing storage")
                .get(DecoratorId(id)).is_none(),
        }
    }).collect();
    while let Some(id) = pending.pop() {
        if recording {
            for neighbor in stairs.graph().neighbors(id) {
                // 原len*8-8<8同时要求len非零，恰为degree1，不是degree<=1。
                // petgraph0.8.1无向Neighbors的size_hint即邻接slice.len，匹配原O(1)读取。
                if stairs.graph().neighbors(neighbor).size_hint().0 == 1
                    && archivist.is_decorator_cull_allowed(neighbor)
                {
                    culled.push(neighbor);
                    pending.push(neighbor);
                }
            }
        }
        stairs.remove_node(id);
        let endpoint = stairs.endpoint();
        if !endpoint.is_unset() && endpoint.node() == id {
            stairs.unset_endpoint();
        }
    }
    // 原集合只来自lane0；不能把结构采样自身额外失效的边补进来，改变原生命周期。
    let stale: std::collections::HashSet<_> = colliders.edge_colliders[0].keys().copied()
        .filter(|edge| { let (a, b) = edge.nodes(); !stairs.contains_edge_or_stub(a, b) })
        .collect();
    colliders.structural_colliders.retain(|(edge, _), instance| {
        if !stale.contains(edge) { return true; }
        dispose_collider_mesh_instance(&mut raycast, instance);
        false
    });
    for edge in stale {
        // 原先structural retain，后三lane依次remove，再删segment_counts。
        // 结构图节点由真实ColliderDiff消费者清理，不能提前伪造额外remove_node。
        for lane in &mut colliders.edge_colliders {
            remove_collider(&mut raycast, lane, &edge);
        }
        structural.parts_mut().segment_counts.remove(&edge);
    }
    for id in culled {
        if let Some(addr) = storage.try_lookup_storage(DecoratorId(id)) {
            archivist.archive_current_decorators_at_with_owners(addr, &storage,
                &raycast, &physics, crate::apply_decorator_op::LABEL_REMOVE);
            // 原806bff只写位置Option判别2(None)，xyz为inactive，不伪造实际零位姿。
            let mut extra = [0; 32];
            extra[12] = 2;
            let op = crate::apply_decorator_op::DecoratorOp {
                kind: crate::apply_decorator_op::DecoratorOp::KIND_REMOVE,
                decorator_id: DecoratorId(id), extra, ..Default::default()
            };
            crate::apply_decorator_op::apply_decorator_op(&op, 0, addr,
                &mut storage, Some(&mut archivist), None, &events);
        }
    }
}

#[cfg(test)]
mod maintain_stair_graph_recovery_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{
        resources::{history::History, stairs::{stairs_collider::StairColliderRegistry,
            stairs_state::ObservedStairEndpoint, structural_graph::StairsStructuralGraph},
            walls::{WallId, decorator::DeferredDecoratorOpEvents,
                decorator_collision::DecoratorPhysicsColliders,
                decorator_archivist::LiveDecoratorArchivist}},
        systems::{backend::input::translate_input::DemoModeEnabled,
            collision::world::{RaycastWorld, RaycastMeshData, RaycastGeoSource, upsert_collider}},
    };
    use std::{panic::{catch_unwind, AssertUnwindSafe}, sync::Arc};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-demo-gates-native-cases.rs"));

    fn world(recording: bool, allowed: bool) -> World {
        let mut world = World::new();
        let mut history: History = History::empty_glade();
        if recording { history.start_input_recording(0).unwrap(); }
        world.insert_resource(history);
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DecoratorPhysicsColliders>();
        let mut archivist = LiveDecoratorArchivist::default();
        if allowed { archivist.decorator_cull_allow_any(); }
        world.insert_resource(archivist);
        world.init_resource::<StairsState>();
        world.init_resource::<StairsStructuralGraph>();
        world.init_resource::<RaycastWorld>();
        world.init_resource::<StairColliderRegistry>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.insert_resource(DemoModeEnabled(false));
        world
    }

    fn place(world: &mut World, id: u32) {
        world.resource_mut::<DecoratorStorage>().apply_decorator_place(
            DecoratorStorageAddr::Terrain, DecoratorId(id), &DecoratorInfo {
                decorator_id: id, ty: 5, rank: 1, u28: 0x1234_5678,
                ..Default::default()
            });
    }

    fn endpoint(world: &mut World, id: u32) {
        let mut bytes = [0; 24];
        bytes[..4].copy_from_slice(&1u32.to_le_bytes());
        bytes[16..20].copy_from_slice(&id.to_le_bytes());
        world.resource_mut::<StairsState>().try_set_endpoint(ObservedStairEndpoint::from_observed(bytes));
    }

    #[test]
    fn demo_gate_precedes_corrupt_storage_and_all_graph_mutation() {
        let mut world = world(true, true);
        world.resource_mut::<StairsState>().add_node(7);
        world.resource_mut::<DecoratorStorage>().decorator_to_storage
            .insert(DecoratorId(7), DecoratorStorageAddr::Wall(WallId(99)));
        world.resource_mut::<DemoModeEnabled>().0 = true;
        world.run_system_once(maintain_stair_graph).unwrap();
        assert!(world.resource::<StairsState>().graph().contains_node(7));
        world.resource_mut::<DemoModeEnabled>().0 = false;
        assert!(catch_unwind(AssertUnwindSafe(|| world.run_system_once(maintain_stair_graph).unwrap())).is_err());
        assert!(world.resource::<StairsState>().graph().contains_node(7));
    }

    #[test]
    fn original_main_two_independent_gates_preserve_same_type_id_and_actual_maintain_behavior() {
        use country_core::{DevOverrideIsDemoMode, game_config::GameConfig,
            resources::{history::wall_edit::WallHistoryMetadata, walls::decorator::anchors::{
                DecoratorPlaceRequestObserved, decorator_source_from_place}}};
        assert_eq!(std::any::TypeId::of::<DemoModeEnabled>(),std::any::TypeId::of::<DevOverrideIsDemoMode>());
        for &(demo,hotkeys,enters,place_u45,wall_source69) in ORIGINAL_DEMO_GATES {
            let mut config=GameConfig::default(); config.dev.demo=demo; config.dev.internal_hotkeys=hotkeys;
            let mut world=world(false,false);
            // 原两个writer分别取dev+6和dev+2；本系统的唯一demo owner只接前者。
            world.insert_resource(DemoModeEnabled(config.dev.demo)); world.insert_resource(config);
            world.resource_mut::<StairsState>().add_node(7);
            world.run_system_once(maintain_stair_graph).unwrap();
            assert_eq!(!world.resource::<StairsState>().graph().contains_node(7),enters,
                "原demo{demo}/hotkeys{hotkeys}的实际维护门");
            let placed=decorator_source_from_place(&DecoratorPlaceRequestObserved([0;88]),
                world.resource::<DevOverrideIsDemoMode>().0,0,false).unwrap();
            assert_eq!(placed.info.bytes()[0x45],place_u45);
            assert_eq!(WallHistoryMetadata::for_new_wall(world.resource::<DevOverrideIsDemoMode>().0).source_69,wall_source69);
        }
    }

    #[test]
    fn required_bucket_failure_precedes_removing_already_collected_missing_nodes() {
        let mut world = world(false, false);
        world.resource_mut::<StairsState>().add_node(1);
        world.resource_mut::<StairsState>().add_node(2);
        world.resource_mut::<DecoratorStorage>().decorator_to_storage
            .insert(DecoratorId(2), DecoratorStorageAddr::Wall(WallId(99)));
        assert!(catch_unwind(AssertUnwindSafe(|| world.run_system_once(maintain_stair_graph).unwrap())).is_err());
        assert_eq!(world.resource::<StairsState>().graph().node_count(), 2);
        assert!(world.resource::<LiveDecoratorArchivist>().special1.is_none());
    }

    #[test]
    fn missing_record_removes_node_and_endpoint_but_does_not_invent_storage_cleanup() {
        let mut world = world(false, true);
        place(&mut world, 7);
        world.resource_mut::<DecoratorStorage>().get_mut(DecoratorStorageAddr::Terrain).unwrap()
            .decorators.remove(&7);
        world.resource_mut::<StairsState>().add_node(7);
        endpoint(&mut world, 7);
        world.run_system_once(maintain_stair_graph).unwrap();
        let stairs = world.resource::<StairsState>();
        assert!(!stairs.graph().contains_node(7));
        assert!(stairs.endpoint().is_unset());
        assert_eq!(world.resource::<DecoratorStorage>().try_lookup_storage(DecoratorId(7)), Some(DecoratorStorageAddr::Terrain));
        assert!(world.resource::<DeferredDecoratorOpEvents>().with_events(|events| events.is_empty()));
    }

    #[test]
    fn recording_and_cull_permission_both_gate_actual_leaf_decorator_removal() {
        for (recording, allowed) in [(false, true), (true, false), (true, true)] {
            let mut world = world(recording, allowed);
            for id in [2, 3, 4] { place(&mut world, id); }
            {
                let mut stairs = world.resource_mut::<StairsState>();
                stairs.add_edge(1, 2); stairs.add_edge(1, 3); stairs.add_edge(3, 4);
            }
            endpoint(&mut world, 1);
            world.run_system_once(maintain_stair_graph).unwrap();
            let stairs = world.resource::<StairsState>();
            assert!(!stairs.graph().contains_node(1));
            assert!(stairs.endpoint().is_unset());
            assert!(stairs.graph().contains_edge(3, 4));
            let culled = recording && allowed;
            assert_eq!(stairs.graph().contains_node(2), !culled);
            assert_eq!(world.resource::<DecoratorStorage>().try_lookup_storage(DecoratorId(2)).is_none(), culled);
            world.resource::<DeferredDecoratorOpEvents>().with_events(|events| {
                assert_eq!(events.len(), usize::from(culled));
                if culled {
                    let event = events[0];
                    assert_eq!(u64::from_le_bytes(event[..8].try_into().unwrap()), 4);
                    assert_eq!(u32::from_le_bytes(event[0x60..0x64].try_into().unwrap()), 2);
                    assert_eq!(event[0x7c], 2); // 原None位置载荷，不把未初始化xyz解释成实际零位置。
                    assert_eq!(event[0x80], 0);
                    assert_eq!(event[0x81], 5);
                }
            });
            if recording { assert!(world.resource::<History>().pending().unwrap().edits.is_empty()); }
        }
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-maintain-stair-graph-original-predicate-cases.rs"));

    #[test]
    fn real_graph_neighbor_degrees_match_original_instruction_and_policy_outputs() {
        for &(degree, allowed, expected_cull) in ORIGINAL_STAIR_CULL_PREDICATES {
            let mut world = world(true, allowed);
            place(&mut world, 2);
            if degree == 0 {
                world.resource_mut::<StairsState>().add_node(1);
                world.resource_mut::<StairsState>().add_node(2);
            } else {
                world.resource_mut::<StairsState>().add_edge(1, 2);
                for neighbor in 3..=degree + 1 {
                    place(&mut world, neighbor);
                    world.resource_mut::<StairsState>().add_edge(2, neighbor);
                }
            }
            world.run_system_once(maintain_stair_graph).unwrap();
            assert_eq!(!world.resource::<StairsState>().graph().contains_node(2), expected_cull,
                "原degree/policy分支：degree={degree}, allow={allowed}");
            assert_eq!(world.resource::<DecoratorStorage>().try_lookup_storage(DecoratorId(2)).is_none(), expected_cull);
        }
    }

    #[test]
    fn cull_archives_both_real_decorator_collision_owners_before_removal() {
        let mut world = world(true, true);
        place(&mut world, 2);
        world.resource_mut::<StairsState>().add_edge(1, 2);
        let first = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::Y));
        let second = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::X));
        let mut raycast = world.remove_resource::<RaycastWorld>().unwrap();
        let mut maps = world.remove_resource::<DecoratorPhysicsColliders>().unwrap();
        upsert_collider(&mut raycast, &mut maps.first, DecoratorId(2),
            RaycastGeoSource::decorator(DecoratorId(2), DecoratorStorageAddr::Terrain),
            first.clone(), Vec3::new(3.0, 4.0, 5.0), glam::Quat::IDENTITY);
        upsert_collider(&mut raycast, &mut maps.second, DecoratorId(2),
            RaycastGeoSource::decorator_interaction(DecoratorId(2), DecoratorStorageAddr::Terrain, 0),
            second.clone(), Vec3::new(-2.0, 1.0, 8.0), glam::Quat::IDENTITY);
        world.insert_resource(raycast); world.insert_resource(maps);
        world.run_system_once(maintain_stair_graph).unwrap();
        let archivist = world.resource::<LiveDecoratorArchivist>();
        assert!(Arc::ptr_eq(&archivist.collision_a[&2].1.data, &first));
        assert!(Arc::ptr_eq(&archivist.collision_b[&2].1.data, &second));
        // 原归档取实际rapier世界位姿，包含mesh local center，不是传入external位姿。
        assert_eq!(archivist.collision_a[&2].0.translation, Vec3::new(3.0, 5.0, 5.0));
        assert_eq!(archivist.collision_b[&2].0.translation, Vec3::new(-1.0, 1.0, 8.0));
        // 原此处仅装饰存储移除；装饰物理实体由后续fullvisual消费，不额外提前清它。
        assert_eq!(world.resource::<DecoratorPhysicsColliders>().first.len(), 1);
    }

    #[test]
    fn stale_lane_zero_disposes_all_real_stair_lanes_and_samples_only_for_its_edges() {
        let mut world = world(false, false);
        place(&mut world, 5); place(&mut world, 6);
        world.resource_mut::<StairsState>().add_edge(5, 6);
        let stale = StairGraphEdge::new(1, 2);
        let live = StairGraphEdge::new(5, 6);
        let structural_only = StairGraphEdge::new(91, 92);
        let mut raycast = world.remove_resource::<RaycastWorld>().unwrap();
        let mut registry = world.remove_resource::<StairColliderRegistry>().unwrap();
        let mut removed = Vec::new();
        let data = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        for lane in &mut registry.edge_colliders {
            for edge in [stale, live] {
                upsert_collider(&mut raycast, lane, edge, RaycastGeoSource::PlatformStair,
                    data.clone(), Vec3::ZERO, glam::Quat::IDENTITY);
                if edge == stale { removed.push(lane[&edge]); }
            }
        }
        for (edge, sample) in [(stale, 0), (stale, 1), (live, 0), (structural_only, 3)] {
            upsert_collider(&mut raycast, &mut registry.structural_colliders, (edge, sample),
                RaycastGeoSource::PlatformStair, data.clone(), Vec3::ZERO, glam::Quat::IDENTITY);
            if edge == stale { removed.push(registry.structural_colliders[&(edge, sample)]); }
        }
        world.resource_mut::<StairsStructuralGraph>().parts_mut().segment_counts.insert(stale, 2);
        world.resource_mut::<StairsStructuralGraph>().parts_mut().segment_counts.insert(live, 1);
        world.resource_mut::<StairsStructuralGraph>().parts_mut().segment_counts.insert(structural_only, 4);
        world.insert_resource(raycast); world.insert_resource(registry);
        world.run_system_once(maintain_stair_graph).unwrap();
        let registry = world.resource::<StairColliderRegistry>();
        for lane in &registry.edge_colliders { assert_eq!(lane.len(), 1); assert!(lane.contains_key(&live)); }
        assert_eq!(registry.structural_colliders.len(), 2);
        assert!(registry.structural_colliders.contains_key(&(structural_only, 3)));
        let counts = world.resource::<StairsStructuralGraph>().segment_counts();
        assert!(!counts.contains_key(&stale)); assert_eq!(counts[&live], 1);
        assert_eq!(counts[&structural_only], 4);
        for collider in removed {
            assert!(world.resource_mut::<RaycastWorld>().get_rapier_collider(collider.slot, collider.collider).is_none());
        }
        // 第二次同owner执行应保持活边、孤立结构记录与图一致，不制造新事件。
        world.run_system_once(maintain_stair_graph).unwrap();
        assert_eq!(world.resource::<StairColliderRegistry>().structural_colliders.len(), 2);
    }
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
        public.consume_with_paths(&mut inner, &terrain, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
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
        walls.consume_with_paths(&mut inner, &terrain, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
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
        let mut walls = PublicWalls::default(); walls.consume_with_paths(&mut inner, &terrain, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
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

    fn preference_fixture(side: u8, target_tag: u8) -> (DecoratorStorage, PublicWalls, StairPoint, StairPoint) {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let (walls, wall) = published_wall();
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (raw_id, tag, u) in [(29, side, 1.0f32), (7, target_tag, 4.0f32)] {
            let mut info = stair_info(tag, 2.0, 1.5);
            info.dst_wall = u64::from(u.to_bits()) | (u64::from(2.0f32.to_bits()) << 32);
            let addr = if tag == 3 { DecoratorStorageAddr::Terrain } else { DecoratorStorageAddr::Wall(wall) };
            storage.apply_decorator_place(addr, DecoratorId(raw_id), &info);
            cached.insert(raw_id, CachedDecoratorTransform { rotation: glam::Quat::IDENTITY,
                translation: Vec3::new(u, 2.0, 0.0), scale: Vec3::ONE });
            stairs.add_node(raw_id);
        }
        stairs.add_edge(29, 7);
        let points = collect_stair_points(&storage, &cached, &walls, &stairs);
        (storage, walls, points[&DecoratorId(29)], points[&DecoratorId(7)])
    }

    #[test]
    fn preference_none_skips_all_owner_reads_and_samewall_tiny_skips_public_wall() {
        let (storage, _walls, from, terrain) = preference_fixture(0, 3);
        assert!(find_alignment_preferences(&terrain, &from, Vec2::X,
            &DecoratorStorage::default(), &PublicWalls::default()).is_empty());
        let (same_storage, _, same_from, same_to) = preference_fixture(0, 0);
        let result = find_alignment_preferences(&same_from, &same_to, Vec2::ZERO,
            &same_storage, &PublicWalls::default());
        assert_eq!(result.len(), 1); assert_eq!(result[0].0.tag, 0);
        assert_eq!(result[0].0.first, Vec2::NEG_Y); assert_eq!(result[0].1, 1.0);
        assert!(storage.get(from.addr).is_some());
    }

    #[test]
    fn preference_three_candidates_preserve_order_discount_and_negated_score_bits() {
        for side in [0, 1] {
            let (storage, walls, from, to) = preference_fixture(side, 3);
            let normal = from.wall_normal.unwrap();
            let result = find_alignment_preferences(&from, &to, normal * 100.0, &storage, &walls);
            assert_eq!(result.len(), 3);
            assert_eq!(result.iter().map(|item| item.0.tag).collect::<Vec<_>>(), vec![0, 1, 1]);
            assert_eq!(result[0].0.first, normal);
            assert_eq!(result[0].1.to_bits(), 0x3ed4_13cd);
            assert_eq!(result[1].0.byte_2, side ^ 1);
            assert_eq!(result[2].0.byte_2, side ^ 1);
            assert_eq!(result[2].0.second.x.to_bits(), result[1].0.second.x.to_bits() ^ 0x8000_0000);
            assert_eq!(result[2].0.second.y.to_bits(), result[1].0.second.y.to_bits() ^ 0x8000_0000);
            assert_eq!(result[2].1.to_bits(), result[1].1.to_bits() ^ 0x8000_0000);
            let zero = find_alignment_preferences(&from, &to, Vec2::ZERO, &storage, &walls);
            assert_eq!(zero.len(), 3);
            assert_eq!(zero[2].1.to_bits(), zero[1].1.to_bits() ^ 0x8000_0000);
        }
    }

    #[test]
    fn samewall_preference_uses_authoritative_coord_delta_and_keeps_ordered_nan_gate() {
        for side in [0, 1] {
            let (storage, walls, from, to) = preference_fixture(side, 0);
            // caller 位移故意不指向目标；samewall实际使用Info的u差，不从position猜。
            let result = find_alignment_preferences(&from, &to, Vec2::NEG_X, &storage, &walls);
            assert_eq!(result.len(), 1); assert_eq!(result[0].0.tag, 1);
            assert_eq!(result[0].0.byte_2, side ^ 1);
            assert_eq!(result[0].0.second, Vec2::X); assert_eq!(result[0].1, 1.0);
            let nan_delta = find_alignment_preferences(&from, &to, Vec2::splat(f32::NAN), &storage, &walls);
            assert_eq!(nan_delta[0].0.tag, 1);
        }
    }

    #[test]
    #[should_panic(expected = "destination is not a wall")]
    fn preferences_checks_source_dst_before_missing_target() {
        let (mut storage, walls, from, mut to) = preference_fixture(0, 0);
        storage.apply_decorator_place(from.addr, from.id, &stair_info(2, 0.0, 1.0));
        to.id = DecoratorId(999);
        find_alignment_preferences(&from, &to, Vec2::X, &storage, &walls);
    }

    #[test]
    #[should_panic(expected = "missing target decorator")]
    fn preferences_checks_target_before_source_wall_address() {
        let (mut storage, walls, mut from, mut to) = preference_fixture(0, 0);
        let info = *storage.get(from.addr).unwrap().get(from.id).unwrap();
        storage.apply_decorator_remove(from.addr, from.id);
        from.addr = DecoratorStorageAddr::Terrain;
        storage.apply_decorator_place(from.addr, from.id, &info);
        to.id = DecoratorId(999);
        find_alignment_preferences(&from, &to, Vec2::X, &storage, &walls);
    }

    #[test]
    fn samewall_preference_negates_signum_before_nan_payload_multiplication() {
        let (mut storage, walls, from, to) = preference_fixture(1, 0);
        let mut info = *storage.get(to.addr).unwrap().get(to.id).unwrap();
        info.dst_wall = (info.dst_wall & 0xffff_ffff_0000_0000) | u64::from(0x7fc1_2345u32);
        storage.apply_decorator_place(to.addr, to.id, &info);
        let result = find_alignment_preferences(&from, &to, Vec2::X, &storage, &walls);
        assert_eq!(result[0].0.tag, 1);
        if cfg!(target_arch = "x86_64") {
            assert_eq!(result[0].0.second.x.to_bits(), 0xffc0_0000);
            assert_eq!(result[0].0.second.y.to_bits(), 0xffc0_0000);
        } else {
            // 原 x86 保完整位断言；其它架构仅允许乘法生成的 NaN 符号不同。
            assert_eq!(result[0].0.second.x.to_bits() & 0x7fff_ffff, 0x7fc0_0000);
            assert_eq!(result[0].0.second.y.to_bits() & 0x7fff_ffff, 0x7fc0_0000);
        }
    }

    fn short_request_points(world: &World) -> std::collections::HashMap<DecoratorId, StairPoint> {
        collect_stair_points(world.resource::<DecoratorStorage>(),
            world.resource::<CachedDecoratorTransforms>(), world.resource::<PublicWalls>(),
            world.resource::<StairsState>())
    }

    fn run_short_request(
        world: &World, points: &std::collections::HashMap<DecoratorId, StairPoint>,
        ids: [DecoratorId; 2], dirty: &std::collections::HashSet<StairGraphEdge>,
    ) -> Option<PlayermadeStairsAssemblyParams> {
        make_short_stair_request(&StairRequestSources {
            storage: world.resource::<DecoratorStorage>(), cached: world.resource::<CachedDecoratorTransforms>(),
            walls: world.resource::<PublicWalls>(), stairs: world.resource::<StairsState>(),
            terrain: world.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>(),
        }, points, ids, dirty)
    }

    #[test]
    fn control_point_uses_directed_alignment_and_never_requires_target_point() {
        let world = compute_world([3.0, 9.0], [2.0, 8.0]);
        let mut points = short_request_points(&world);
        points.remove(&DecoratorId(7));
        let mut alignments = petgraph::graphmap::DiGraphMap::default();
        let kind = AlignKind { tag: 0, flag: 1, first: Vec2::X, ..AlignKind::default() };
        alignments.add_edge(DecoratorId(29), DecoratorId(7), kind);
        alignments.add_edge(DecoratorId(7), DecoratorId(29),
            AlignKind { tag: 2, ..AlignKind::default() });
        assert_eq!(aligned_stair_control_point(&points, &alignments, DecoratorId(29), DecoratorId(7)),
            (Vec2::new(2.0, 0.0), kind));
    }

    #[test]
    #[should_panic(expected = "missing point")]
    fn control_point_checks_point_before_missing_alignment() {
        aligned_stair_control_point(&Default::default(), &Default::default(), DecoratorId(29), DecoratorId(7));
    }

    #[test]
    fn short_failed_edge_keeps_first_geometry_and_actual_second_anchor() {
        let mut world = compute_world([3.0, 9.0], [2.0, 8.0]);
        world.resource_mut::<TerrainHeightsData<VisualTerrainHeightSsbo>>().set_height_range(3.0, 3.0);
        let points = short_request_points(&world);
        let mut dirty = std::collections::HashSet::new();
        dirty.insert(StairGraphEdge::new(29, 7));
        let result = run_short_request(&world, &points, [DecoratorId(29), DecoratorId(7)], &dirty).unwrap();
        assert_eq!(result.curve.points, vec![Vec2::ZERO, Vec2::new(0.0, -1.0)]);
        assert_eq!(result.heights, vec![3.0, 4.0]);
        assert_eq!(result.widths, vec![2.0, 2.0]);
        assert_eq!(result.tangents, vec![Vec2::NEG_Y; 2]);
        assert_eq!(result.nodes, [(StairNodeAnchor::Terrain { height: 3.0 }, 1),
            (StairNodeAnchor::Terrain { height: 9.0 }, 1)]);
        assert_eq!(result.segment.nodes(), (DecoratorId(7), DecoratorId(29)));
        assert_eq!(result.sub_segments.as_slice(), &[StairSubSegment { kind: 2, flag: 1, count: 1 }]);
        assert!(result.terrain_contacts[0]);
        assert!(!result.terrain_contacts[1]);
        assert!(result.dirty);
        assert!(result.auxiliary_samples.is_empty());
        assert!(result.interpolated_curve.is_none());
    }

    #[test]
    fn short_isolated_point_keeps_non_unit_cached_quaternion_and_nan_mask_policy() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let mut world = compute_world([0.0, 0.0], [1.0, 2.0]);
        world.resource_mut::<CachedDecoratorTransforms>().insert(29, CachedDecoratorTransform {
            rotation: glam::Quat::from_xyzw(0.0, 0.0, 0.0, 2.0),
            translation: Vec3::ZERO, scale: Vec3::ONE,
        });
        world.resource_mut::<TerrainHeightsData<VisualTerrainHeightSsbo>>().set_height_range(f32::NAN, f32::NAN);
        let points = short_request_points(&world);
        let result = run_short_request(&world, &points, [DecoratorId(29); 2], &Default::default()).unwrap();
        assert_eq!(result.curve.points, vec![Vec2::ZERO, Vec2::new(0.0, -4.0)]);
        assert_eq!(result.heights, vec![0.0, 1.0]);
        assert_eq!(result.sub_segments.as_slice(), &[StairSubSegment { kind: 1, flag: 1, count: 1 }]);
        assert_eq!(result.segment.nodes(), (DecoratorId(29), DecoratorId(29)));
        assert!(result.terrain_contacts.iter().all(|bit| !*bit));
    }

    #[test]
    fn short_wall_point_uses_real_normal_without_reading_cached_quaternion() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let (walls, wall_id) = published_wall();
        let mut world = compute_world([4.0, 0.0], [2.0, 2.0]);
        world.insert_resource(walls);
        world.resource_mut::<DecoratorStorage>().apply_decorator_remove(DecoratorStorageAddr::Terrain, DecoratorId(29));
        world.resource_mut::<DecoratorStorage>().apply_decorator_place(
            DecoratorStorageAddr::Wall(wall_id), DecoratorId(29), &stair_info(0, 4.0, 2.0));
        world.resource_mut::<CachedDecoratorTransforms>().insert(29, CachedDecoratorTransform {
            rotation: glam::Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
            translation: Vec3::new(1.0, 4.0, 2.0), scale: Vec3::ONE,
        });
        let points = short_request_points(&world);
        let normal = points[&DecoratorId(29)].wall_normal.unwrap();
        world.insert_resource(CachedDecoratorTransforms::default());
        let result = run_short_request(&world, &points, [DecoratorId(29); 2], &Default::default()).unwrap();
        assert_eq!(result.curve.points, vec![Vec2::new(1.0, 2.0), Vec2::new(1.0, 2.0) + normal]);
        assert_eq!(result.heights, vec![4.0, 3.0]);
        assert!(matches!(result.nodes[0].0, StairNodeAnchor::Wall(_)));
    }

    #[test]
    fn short_curve_failure_happens_before_late_decorator_lookups() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let mut world = compute_world([0.0, 0.0], [1.0, 1.0]);
        world.resource_mut::<CachedDecoratorTransforms>().insert(29, CachedDecoratorTransform {
            rotation: glam::Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
            translation: Vec3::ZERO, scale: Vec3::ONE,
        });
        let points = short_request_points(&world);
        world.resource_mut::<DecoratorStorage>().get_mut(DecoratorStorageAddr::Terrain).unwrap()
            .remove(DecoratorId(29));
        assert!(run_short_request(&world, &points, [DecoratorId(29); 2], &Default::default()).is_none());
    }

    #[test]
    #[should_panic(expected = "missing second point")]
    fn short_second_point_is_required_before_cached_direction() {
        let mut world = compute_world([0.0, 0.0], [1.0, 1.0]);
        let mut points = short_request_points(&world);
        points.remove(&DecoratorId(7));
        world.insert_resource(CachedDecoratorTransforms::default());
        run_short_request(&world, &points, [DecoratorId(29), DecoratorId(7)], &Default::default());
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn short_first_anchor_is_derived_before_second_decorator_lookup() {
        let mut world = compute_world([0.0, 0.0], [1.0, 1.0]);
        let points = short_request_points(&world);
        let mut storage = world.resource_mut::<DecoratorStorage>();
        let bucket = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap();
        let info = bucket.get_mut(DecoratorId(29)).unwrap();
        info.u20 = (info.u20 & 0xff00_ffff) | (4 << 16);
        bucket.remove(DecoratorId(7));
        drop(storage);
        run_short_request(&world, &points, [DecoratorId(29), DecoratorId(7)], &Default::default());
    }

    fn install_request_resources(world: &mut World) {
        world.insert_resource(PlayermadeStairsAssemblyRequests::default());
        world.insert_resource(country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::default());
        world.insert_resource(country_core::systems::backend::input::translate_input::DemoModeEnabled(false));
        world.resource_mut::<TerrainHeightsData<VisualTerrainHeightSsbo>>().set_height_range(-20.0, -20.0);
    }

    fn run_requests<const KEEP_DIRTY: bool>(world: &mut World) {
        use bevy_ecs::system::RunSystemOnce;
        world.run_system_once(generate_playermade_stair_requests::<KEEP_DIRTY>).unwrap();
    }

    fn request_graph_world(positions: &[Vec3], edges: &[(u32, u32)]) -> World {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (index, &position) in positions.iter().enumerate() {
            let id = (index + 1) as u32;
            storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(id),
                &stair_info(3, position.y, 1.0));
            cached.insert(id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: position, scale: Vec3::ONE,
            });
            stairs.add_node(id);
        }
        for &(a, b) in edges { stairs.add_edge(a, b); }
        let mut world = World::new();
        world.insert_resource(storage); world.insert_resource(cached); world.insert_resource(stairs);
        world.insert_resource(PublicWalls::default());
        world.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
        install_request_resources(&mut world);
        world
    }

    #[test]
    fn requests_open_spline_sorts_ids_with_geometry_and_publishes_both_directions() {
        let mut world = compute_world([0.0, 1.0], [1.0, 3.0]);
        install_request_resources(&mut world);
        run_requests::<false>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 1);
        let request = &requests.requests[0];
        assert_eq!(request.segment.nodes(), (DecoratorId(7), DecoratorId(29)));
        assert_eq!(request.curve.points.len(), 8);
        assert!((request.curve.points[0] - Vec2::new(4.0, 0.0)).length() < 0.0001);
        assert!(request.curve.points.last().unwrap().length() < 0.0001);
        assert!((request.heights[0] - 1.0).abs() < 0.0001);
        assert!(request.heights.last().unwrap().abs() < 0.0001);
        assert_eq!(request.widths[0], 3.0);
        assert_eq!(*request.widths.last().unwrap(), 1.0);
        assert_eq!(request.tangents[0], Vec2::NEG_X);
        assert_eq!(*request.tangents.last().unwrap(), Vec2::NEG_X);
        assert_eq!(requests.alignments.edge_weight(DecoratorId(7), DecoratorId(29)).unwrap().tag, 2);
        assert_eq!(requests.alignments.edge_weight(DecoratorId(29), DecoratorId(7)).unwrap().tag, 2);
        let forward = requests.adjusted_edges.edge_weight(DecoratorId(7), DecoratorId(29)).unwrap();
        let backward = requests.adjusted_edges.edge_weight(DecoratorId(29), DecoratorId(7)).unwrap();
        assert_eq!(forward.position, Vec2::new(4.0, 0.0));
        assert_eq!(backward.position, Vec2::ZERO);
        assert!((forward.outgoing_dir.normalize_or_zero() - Vec2::NEG_X).length() < 0.0001);
        assert!((backward.outgoing_dir.normalize_or_zero() - Vec2::X).length() < 0.0001);
    }

    #[test]
    fn requests_keep_dirty_only_for_true_and_propagate_one_fixed_hop() {
        let mut world = request_graph_world(&[
            Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(8.0, 0.0, 0.0), Vec3::new(12.0, 0.0, 0.0),
        ], &[(1, 2), (2, 3), (3, 4)]);
        let points = short_request_points(&world);
        let dirty = std::collections::HashSet::from([StairGraphEdge::new(1, 2)]);
        let previous = run_short_request(&world, &points, [DecoratorId(1), DecoratorId(2)], &dirty).unwrap();
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(previous);
        run_requests::<true>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 3);
        let dirty_for = |a, b| requests.requests.iter()
            .find(|request| request.segment == StairSegment::new(DecoratorId(a), DecoratorId(b))).unwrap().dirty;
        assert!(dirty_for(1, 2));
        assert!(dirty_for(2, 3));
        assert!(!dirty_for(3, 4));
        let allocation = requests.requests.as_ptr();
        let capacity = requests.requests.capacity();
        run_requests::<false>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 3);
        assert!(requests.requests.iter().all(|request| !request.dirty));
        assert_eq!(requests.requests.capacity(), capacity);
        assert_eq!(requests.requests.as_ptr(), allocation);
    }

    #[test]
    fn requests_demo_gate_preserves_current_requests_and_scratch_graphs() {
        let mut world = request_graph_world(&[Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)], &[(1, 2)]);
        run_requests::<false>(&mut world);
        let allocation = world.resource::<PlayermadeStairsAssemblyRequests>().requests.as_ptr();
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().alignments.add_edge(DecoratorId(99), DecoratorId(100),
            AlignKind { tag: 2, ..AlignKind::default() });
        world.resource_mut::<country_core::systems::backend::input::translate_input::DemoModeEnabled>().0 = true;
        run_requests::<true>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 1);
        assert_eq!(requests.requests.as_ptr(), allocation);
        assert!(requests.alignments.contains_edge(DecoratorId(99), DecoratorId(100)));
        assert_eq!(requests.adjusted_edges.edge_count(), 2);
    }

    #[test]
    fn requests_closed_spline_consumes_original_edges_and_keeps_isolated_nodes_separate() {
        let mut world = request_graph_world(&[
            Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 3.0), Vec3::new(10.0, 0.0, 0.0),
        ], &[(1, 2), (2, 3), (3, 1)]);
        run_requests::<false>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 4);
        assert_eq!(requests.requests.iter().filter(|request| request.segment.nodes().0 == request.segment.nodes().1).count(), 1);
        assert_eq!(requests.adjusted_edges.edge_count(), 6);
        assert_eq!(world.resource::<StairsState>().graph().edge_count(), 3);
        assert!(requests.requests.iter().all(|request| request.curve.points.iter().all(|point| point.is_finite())));
        assert_eq!(requests.requests.last().unwrap().segment.nodes(), (DecoratorId(4), DecoratorId(4)));
    }

    #[test]
    fn requests_same_wall_curve_failure_keeps_published_edges_and_builds_short_request() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let (walls, wall_id) = published_wall();
        let mut world = request_graph_world(&[Vec3::ZERO, Vec3::new(4.0, 0.0, 0.0)], &[(1, 2)]);
        world.insert_resource(walls);
        for id in [1, 2] {
            world.resource_mut::<DecoratorStorage>().apply_decorator_remove(DecoratorStorageAddr::Terrain, DecoratorId(id));
            world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Wall(wall_id),
                DecoratorId(id), &stair_info(0, 4.0, 1.0));
            world.resource_mut::<CachedDecoratorTransforms>().insert(id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: Vec3::new(1.0, 4.0, 2.0), scale: Vec3::ONE,
            });
        }
        run_requests::<false>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 1);
        assert_eq!(requests.adjusted_edges.edge_count(), 2);
        assert_eq!(requests.requests[0].heights, vec![4.0, 3.0]);
        assert!(matches!(requests.requests[0].nodes[0].0, StairNodeAnchor::Wall(_)));
        assert_eq!(requests.requests[0].segment.nodes(), (DecoratorId(1), DecoratorId(2)));
    }

    #[test]
    fn requests_duplicate_control_points_reach_original_short_fallback() {
        // 依赖 paired canonical native Spline2D：不能用本系统 finite 门掩盖共享缺口。
        let mut world = request_graph_world(&[Vec3::ZERO, Vec3::ZERO], &[(1, 2)]);
        run_requests::<false>(&mut world);
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 1);
        assert_eq!(requests.requests[0].curve.points, vec![Vec2::ZERO, Vec2::new(0.0, -1.0)]);
        assert_eq!(requests.requests[0].heights, vec![0.0, 1.0]);
        assert_eq!(requests.requests[0].segment.nodes(), (DecoratorId(1), DecoratorId(2)));
        assert_eq!(requests.adjusted_edges.edge_count(), 0);
    }

    fn install_stair_fullvisual(world: &mut World) {
        world.init_resource::<crate::blueprint::PrevDecoratorBlueprints>();
        world.init_resource::<crate::decorator_visual_state::DecoratorVisualState>();
        world.init_resource::<country_core::resources::walls::wall_holes::WallHoles>();
        world.init_resource::<country_core::systems::collision::world::RaycastWorld>();
        world.init_resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<country_core::systems::collision::WindowAutoClutterCandidates>();
        world.init_resource::<crate::decorator_visual::deco_differential_state::WallAttachedDecoTracker<crate::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>>();
        world.init_resource::<crate::decorator_visual::deco_differential_state::WallAttachedDecoTracker<crate::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>>();
        world.insert_resource(country_core::asset_libraries::mesh_library::AssetMeshLibrary::new());
        world.insert_resource(country_core::asset_libraries::shader_library::SharedAssetShaderLibrary(std::sync::Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new())));
        // CPU descriptor fixture，未创建真实GPU资源或验证画面。
        world.insert_resource(country_core::asset_libraries::texture_library::AssetTextureLibrary::new(&country_core::render::test_device::TestDevice(0)));
        world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-visual-native-vectors.rs"));

    fn visual_alignments(case: u8) -> PlayermadeStairsAssemblyRequests {
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        let kinds: &[(u8, u8)] = match case {
            0 => &[], 1 => &[(0, 0)], 2 => &[(0, 1)], 3 => &[(1, 0)],
            4 => &[(1, 1)], 5 => &[(2, 0)], 6 => &[(2, 0), (1, 0)],
            7 => &[(0, 0), (1, 0)], _ => unreachable!(),
        };
        for (i, &(tag, flag)) in kinds.iter().enumerate() {
            requests.alignments.add_edge(DecoratorId(41), DecoratorId(50 + i as u32),
                AlignKind { tag, flag, ..AlignKind::default() });
        }
        requests
    }

    #[test]
    fn stair_visual_profile_and_shape_switch_match_42_original_sse_vectors() {
        use country_core::resources::walls::WallId;
        for &(case, width, kind, offset, hole, dims) in STAIR_VISUAL_NATIVE {
            let width = f32::from_bits(width);
            let requests = visual_alignments(case);
            let actual = stair_decorator_interaction_kind(DecoratorStorageAddr::Wall(WallId(7)), DecoratorId(41), width, &requests);
            assert_eq!((actual.0, actual.1.to_bits(), actual.2), (kind, offset, hole));
            assert_eq!(stair_decorator_interaction_dimensions(kind, width).to_array().map(f32::to_bits), dims);
        }
        let mut incoming = visual_alignments(0);
        incoming.alignments.add_edge(DecoratorId(90), DecoratorId(41), AlignKind::default());
        assert_eq!(stair_decorator_interaction_kind(DecoratorStorageAddr::Wall(WallId(7)), DecoratorId(41), 1.0, &incoming), (1, 0.5, true));
        assert!(std::panic::catch_unwind(|| stair_decorator_interaction_kind(
            DecoratorStorageAddr::Wall(WallId(7)), DecoratorId(41), 1.0, &visual_alignments(7))).is_err());
    }

    fn stair_visual_world(tag: u8) -> World {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let mut world = World::new();
        let mut storage = DecoratorStorage::default();
        let mut info = stair_info(tag, 0.8, 1.2);
        info.ty = b'?';
        storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(41), &info);
        world.insert_resource(storage);
        let mut transforms = CachedDecoratorTransforms::default();
        transforms.insert(41, CachedDecoratorTransform {
            translation: Vec3::new(3.0, 2.0, 4.0), rotation: glam::Quat::from_rotation_y(0.7),
            scale: Vec3::new(2.0, 3.0, 4.0),
        });
        world.insert_resource(transforms);
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<country_core::systems::ui::signifiers::UiSignifierStream>();
        world.insert_resource(country_core::systems::backend::input::translate_input::DemoModeEnabled(false));
        world
    }

    #[test]
    fn stair_visual_original_compound_reaches_fullvisual_and_reclaims_live_interaction() {
        use bevy_ecs::system::RunSystemOnce;
        use crate::blueprint::{BlueprintMeshSlot, PrevDecoratorBlueprints};
        use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        for tag in [2, 3] {
            let mut world = stair_visual_world(tag);
            world.run_system_once(generate_playermade_stair_decorators).unwrap();
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).unwrap();
            assert!(blueprint.visual_entities.is_empty(), "原节点不生成第二套楼梯砖模型");
            assert!(matches!(blueprint.collision_mesh, BlueprintMeshSlot::Disabled));
            let BlueprintMeshSlot::Mesh(mesh) = &blueprint.interaction_mesh else { panic!("不能丢进legacy interaction字段"); };
            let weak = std::sync::Arc::downgrade(&mesh.mesh);
            let compound = mesh.mesh.shape_ref().unwrap().as_compound().unwrap();
            assert_eq!(compound.shapes().len(), 3);
            let centers: Vec<_> = compound.shapes().iter().map(|(pose, _)| [pose.translation.x, pose.translation.y, pose.translation.z]).collect();
            assert_eq!(centers[0], [0.0, 0.0, 0.0]);
            assert_eq!(centers[1], [0.0, 1.3, 0.0]);
            assert_eq!(centers[2][1], (-0.25f32 - 0.8) + 0.5);
            assert_eq!(mesh.position, Vec3::new(3.0, 2.0, 4.0));
            assert_eq!(blueprint.transform.unwrap().translation, Vec3::new(3.0, 2.25, 4.0));
            install_stair_fullvisual(&mut world);
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            let maps = world.resource::<DecoratorPhysicsColliders>();
            assert!(maps.first.is_empty());
            assert_eq!(maps.second.len(), 1);
            let instance = maps.second[&DecoratorId(41)];
            let handle = instance.handle();
            assert_eq!(world.resource::<country_core::systems::collision::world::RaycastWorld>().get_collider_mesh(instance.registry_index, instance.generation).source,
                country_core::systems::collision::world::RaycastGeoSource::decorator_interaction(DecoratorId(41), DecoratorStorageAddr::Terrain, 1));
            assert_eq!(world.resource::<country_core::systems::collision::world::RaycastWorld>().get_collider(handle).position, Vec3::new(3.0, 2.0, 4.0));
            let mut previous = PrevDecoratorBlueprints::default();
            world.resource::<DecoratorBlueprints>().copy_blueprints_to_prev(&mut previous);
            world.insert_resource(previous);
            world.resource_mut::<DecoratorBlueprints>().map.clear();
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
            *world.resource_mut::<PrevDecoratorBlueprints>() = Default::default();
            assert!(weak.upgrade().is_none());
        }
    }

    #[test]
    fn stair_visual_demo_and_type_gates_precede_cache_and_extra_failures() {
        use bevy_ecs::system::RunSystemOnce;
        let mut world = stair_visual_world(3);
        world.resource_mut::<country_core::systems::backend::input::translate_input::DemoModeEnabled>().0 = true;
        world.insert_resource(CachedDecoratorTransforms::default());
        world.run_system_once(generate_playermade_stair_decorators).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
        world.resource_mut::<country_core::systems::backend::input::translate_input::DemoModeEnabled>().0 = false;
        world.resource_mut::<DecoratorStorage>().get_mut(DecoratorStorageAddr::Terrain).unwrap().get_mut(DecoratorId(41)).unwrap().ty = 0;
        world.run_system_once(generate_playermade_stair_decorators).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
    }

    fn regular_stair_visual_world(style: u8, flat_style: u8, material: u8) -> (World, country_core::resources::walls::WallId, Entity) {
        use country_core::resources::walls::{InnerWalls, WallState, publication_fields::WallShapeSnapshot, publication_source::TerrainHeightGrid512};
        let mut world = stair_visual_world(0);
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[-2.0,-2.0],[2.0,-2.0],[2.0,2.0],[-2.0,2.0]], true).unwrap();
        let state = WallState::new(curve, 4.0, style, 0, false);
        let mut snapshot = state.snapshot().clone();
        let mut shape = [0u8;28]; shape[..4].copy_from_slice(&1u32.to_le_bytes());
        for (offset,value) in [(4,1.0f32),(8,0.0),(12,0.0),(16,0.0),(20,4.0),(24,4.0)] {
            shape[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        let mut inner = InnerWalls::new();
        let wall_id = inner.new_wall(WallState::from_snapshot(snapshot, 0));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &TerrainHeightGrid512 {encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0,sampling_fix:false},
            None, &Default::default(), &Default::default());
        assert!(public.get(wall_id).unwrap().is_premade_shape());
        let mut roof = [0u8;88]; roof[..8].copy_from_slice(&wall_id.0.to_le_bytes());
        roof[8..36].copy_from_slice(&shape); roof[0x40]=flat_style; roof[0x41]=material;
        let roof_entity = world.spawn(country_core::resources::roofs::roof_shape::Roof::from_observed(roof)).id();
        public.associate_roof(wall_id, roof_entity);
        world.insert_resource(public);
        let mut info = stair_info(0, 0.8, 1.2); info.ty=b'?';
        info.dst_wall = u64::from(1.0f32.to_bits()) | (u64::from(3.5f32.to_bits())<<32);
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(DecoratorStorageAddr::Wall(wall_id), DecoratorId(41), &info);
        world.insert_resource(storage);
        (world,wall_id,roof_entity)
    }

    #[test]
    fn stair_visual_wall_hole_uses_real_regular_wall_flat_roof_material_and_original_ordered_gap() {
        use bevy_ecs::system::RunSystemOnce;
        for (style,flat,material,wooden) in [(0,0,0,false),(2,0,0,true),(2,1,0,false),(0,2,1,true),(2,3,0,true)] {
            let (mut world,wall,_) = regular_stair_visual_world(style,flat,material);
            let maximum = world.resource::<PublicWalls>().get(wall).unwrap().max_y();
            world.run_system_once(generate_playermade_stair_decorators).unwrap();
            let hole = world.resource::<DecoratorBlueprints>().get(41).unwrap().wall_holes[0];
            assert_eq!(hole.center[0], 1.0);
            assert_eq!(hole.center[1].to_bits(), ((maximum - if wooden {f32::from_bits(0x3f428f5d)} else {f32::from_bits(0x3f28f5c3)})+5.0).to_bits());
            assert_eq!(hole.dimensions,[1.2,10.0]); assert_eq!(hole.kind,4);
            install_stair_fullvisual(&mut world);
            world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
            assert!(world.resource::<country_core::resources::walls::wall_holes::WallHoles>().storage.contains_key(&wall));
        }
        for roof_missing in [false,true] {
            let (mut world,_,roof_entity) = regular_stair_visual_world(0,0,0);
            if roof_missing {world.entity_mut(roof_entity).remove::<country_core::resources::roofs::roof_shape::Roof>();}
            else {world.entity_mut(roof_entity).get_mut::<country_core::resources::roofs::roof_shape::Roof>().unwrap().observed_mut()[0x30..0x34].copy_from_slice(&0.5f32.to_le_bytes());}
            world.run_system_once(generate_playermade_stair_decorators).unwrap();
            assert!(world.resource::<DecoratorBlueprints>().get(41).unwrap().wall_holes.is_empty());
        }
        let (mut world,wall,_) = regular_stair_visual_world(0,0,0);
        world.resource_mut::<DecoratorStorage>().get_mut(DecoratorStorageAddr::Wall(wall)).unwrap().get_mut(DecoratorId(41)).unwrap().dst_wall =
            u64::from(1.0f32.to_bits()) | (u64::from(f32::NAN.to_bits())<<32);
        world.run_system_once(generate_playermade_stair_decorators).unwrap();
        assert!(world.resource::<DecoratorBlueprints>().get(41).unwrap().wall_holes.is_empty());
    }

    #[test]
    fn stair_visual_original_failure_points_keep_only_prior_mutations() {
        use bevy_ecs::system::RunSystemOnce;
        for bad_extra in [false, true] {
            let mut world = stair_visual_world(if bad_extra { 3 } else { 4 });
            if bad_extra {
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let info = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap().get_mut(DecoratorId(41)).unwrap();
                info.set_extra(country_core::resources::walls::decorator::DecoratorExtraInfo::Window(
                    country_core::resources::walls::decorator::WindowExtra(0)));
            }
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world.run_system_once(generate_playermade_stair_decorators).unwrap();
            })).is_err());
            let blueprint = world.resource::<DecoratorBlueprints>().get(41).expect("原insert先于失败");
            assert_eq!(blueprint.interaction_source_flags, 1, "原独立标记先于as_stair与目的判别失败");
            assert_eq!(blueprint.transform.is_none(), bad_extra);
            assert!(matches!(blueprint.collision_mesh, crate::blueprint::BlueprintMeshSlot::Unspecified), "原collision Disabled在末尾，不能提前写");
            assert!(matches!(blueprint.interaction_mesh, crate::blueprint::BlueprintMeshSlot::Unspecified));
        }
    }

    #[test]
    fn stair_visual_preview_skip_and_permanent_interaction_flag_are_independent_across_frames() {
        use bevy_ecs::system::RunSystemOnce;
        use crate::blueprint::{BlueprintMeshSlot, PrevDecoratorBlueprints};
        use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
        let mut world = stair_visual_world(3);
        world.resource_mut::<DecoratorStorage>().get_mut(DecoratorStorageAddr::Terrain).unwrap().get_mut(DecoratorId(41)).unwrap().state = 1;
        world.run_system_once(generate_playermade_stair_decorators).unwrap();
        let preview = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!((preview.u138, preview.interaction_source_flags), (1, 1));
        assert!(preview.transform.is_some());
        let BlueprintMeshSlot::Mesh(mesh) = &preview.interaction_mesh else { panic!("预览仍构造原交互载荷"); };
        let weak = std::sync::Arc::downgrade(&mesh.mesh);
        install_stair_fullvisual(&mut world);
        world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
        assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
        assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
        let mut previous = PrevDecoratorBlueprints::default();
        world.resource::<DecoratorBlueprints>().copy_blueprints_to_prev(&mut previous);
        world.insert_resource(previous);
        let mut normal = *world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Terrain).unwrap().get(DecoratorId(41)).unwrap();
        normal.state = 0;
        world.resource_mut::<DecoratorStorage>().apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(41), &normal);
        world.run_system_once(generate_playermade_stair_decorators).unwrap();
        let normal = world.resource::<DecoratorBlueprints>().get(41).unwrap();
        assert_eq!((normal.u138, normal.interaction_source_flags), (0, 1));
        world.run_system_once(crate::blueprint::instantiate_blueprints).unwrap();
        assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
        assert_eq!(world.resource::<DecoratorPhysicsColliders>().second.len(), 1);
        *world.resource_mut::<PrevDecoratorBlueprints>() = Default::default();
        assert!(weak.upgrade().is_none(), "旧预览mesh不应被活物理世界持有");
    }

}
