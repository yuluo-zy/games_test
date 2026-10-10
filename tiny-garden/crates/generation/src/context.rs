//! 上下文语义解析的唯一入口：不可变输入产生类型、网络、地基、立面及决策。
//! 预览与正式提交共同使用此模块；解析不读取 World，不创建网格，不修改玩家意图。
use crate::{
    Face,
    structure::{BuildingRule, StructureRule},
};
use garden_domain::{
    Building, BuildingId,
    context::*,
    strokes::{Point, Stroke, StrokeId, StrokeKind},
    terrain::{CELL, TerrainDocument},
};
use garden_geometry::linear::{self, SpatialGrid};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
#[derive(Debug, Clone, PartialEq)]
pub struct ContextInput {
    pub buildings: Vec<Arc<Building>>,
    pub strokes: Vec<Arc<Stroke>>,
    pub scene: Arc<SceneContext>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct LinearStructure {
    pub stroke: Arc<Stroke>,
    pub kind: StrokeKind,
    pub control_ids: Vec<u64>,
    pub points: Vec<Point>,
    pub elevation_offset: f64,
    pub style: BoundaryStyle,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SegmentSource {
    pub stroke: StrokeId,
    pub from: u64,
    pub to: u64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkNode {
    pub point: Point,
    pub height: f64,
    pub sources: Vec<SegmentSource>,
    pub anchors: Vec<(SegmentSource, Option<u64>)>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkEdge {
    pub source: SegmentSource,
    pub interval: [f64; 2],
    pub nodes: [usize; 2],
    pub width: f64,
    pub passable: bool,
    pub contributions: Vec<(SegmentSource, [f64; 2])>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PathNetwork {
    pub nodes: Vec<NetworkNode>,
    pub edges: Vec<NetworkEdge>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WallSurface {
    pub anchor: WallAnchor,
    pub start: Point,
    pub end: Point,
    pub base: f64,
    pub height: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DoorRequest {
    pub along: f64,
    pub width: f64,
    pub height: f64,
    pub sources: Vec<StrokeId>,
    pub segments: Vec<SegmentSource>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpeningAssembly {
    pub members: Vec<OpeningId>,
    pub openings: Vec<OpeningIntent>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FacadeStructure {
    pub surface: WallSurface,
    pub doors: Vec<DoorRequest>,
    pub assemblies: Vec<OpeningAssembly>,
    pub sleeping: Vec<OpeningId>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelationKey {
    Network(Vec<(SegmentSource, Option<u64>)>),
    Crossing(WallAnchor, Vec<SegmentSource>),
    Windows(WallAnchor, Vec<OpeningId>),
    Boundary(StrokeKind, Vec<(StrokeId, u64)>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Diagnostic {
    Steep(StrokeId),
    Sleeping(OpeningId),
    BlockedEntrance(WallAnchor),
}

/// 同族边界的端点连接。材料、高度和端点方向均兼容后才建立关系。
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryJunction {
    pub kind: StrokeKind,
    pub members: Vec<(StrokeId, u64)>,
    pub point: Point,
    pub width: f64,
    pub height: f64,
}
pub fn boundary_junctions(
    lines: &BTreeMap<StrokeId, LinearStructure>,
    terrain: &TerrainDocument,
    old: &DecisionState,
    rules: ContextRules,
) -> (Vec<BoundaryJunction>, Vec<ConnectionHint>) {
    let mut endpoints = Vec::new();
    for line in lines.values().filter(|l| l.kind != StrokeKind::Path) {
        let p = &line.stroke.points;
        let ids = &line.control_ids;
        for (point, neighbor, id, other) in [
            (p[0], p[1], ids[0], ids[1]),
            (
                *p.last().unwrap(),
                p[p.len() - 2],
                *ids.last().unwrap(),
                ids[ids.len() - 2],
            ),
        ] {
            let length = point.distance(neighbor);
            let dir = [
                (point.x - neighbor.x) / length,
                (point.z - neighbor.z) / length,
            ];
            endpoints.push((line, point, id, other, dir));
        }
    }
    let mut junctions: Vec<BoundaryJunction> = Vec::new();
    let mut hints = BTreeSet::new();
    for i in 0..endpoints.len() {
        for j in i + 1..endpoints.len() {
            let (a, p, aid, _, ad) = endpoints[i];
            let (b, q, bid, bother, bd) = endpoints[j];
            if a.stroke.id == b.stroke.id
                || a.kind != b.kind
                || ((a.style != b.style)
                    && a.style != BoundaryStyle::Auto
                    && b.style != BoundaryStyle::Auto)
            {
                continue;
            }
            if (a.stroke.height - b.stroke.height).abs() > 0.1
                || (terrain.height(p) + a.elevation_offset - terrain.height(q) - b.elevation_offset)
                    .abs()
                    > rules.height_tolerance
            {
                continue;
            }
            if ad[0] * bd[0] + ad[1] * bd[1] > 0.95 {
                continue;
            }
            let hint = ConnectionHint {
                endpoint: (a.stroke.id, aid),
                segment: (b.stroke.id, bid.min(bother), bid.max(bother)),
                kind: a.kind,
            };
            let limit = if old.connections.contains(&hint) {
                rules.connect_exit
            } else {
                rules.connect_enter
            };
            if p.distance(q) > limit {
                continue;
            }
            hints.insert(hint);
            let members = [(a.stroke.id, aid), (b.stroke.id, bid)];
            let center = Point {
                x: (p.x + q.x) / 2.,
                z: (p.z + q.z) / 2.,
            };
            if let Some(group) = junctions
                .iter_mut()
                .find(|g| g.kind == a.kind && g.members.iter().any(|m| members.contains(m)))
            {
                group.members.extend(members);
                group.members.sort();
                group.members.dedup();
                group.width = group.width.max(a.stroke.width).max(b.stroke.width);
            } else {
                junctions.push(BoundaryJunction {
                    kind: a.kind,
                    members: members.to_vec(),
                    point: center,
                    width: a.stroke.width.max(b.stroke.width),
                    height: a.stroke.height,
                });
            }
        }
    }
    // 一个新连接可能连接两组旧候选，先合并共享端点，再只生成一个公共接头。
    let mut i = 0;
    while i < junctions.len() {
        let mut j = i + 1;
        while j < junctions.len() {
            if junctions[i].kind == junctions[j].kind
                && junctions[i]
                    .members
                    .iter()
                    .any(|m| junctions[j].members.contains(m))
            {
                let other = junctions.remove(j);
                junctions[i].members.extend(other.members);
                junctions[i].members.sort();
                junctions[i].members.dedup();
                junctions[i].width = junctions[i].width.max(other.width);
                j = i + 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    for group in &mut junctions {
        let points = group
            .members
            .iter()
            .filter_map(|(id, cid)| {
                endpoints
                    .iter()
                    .find(|(l, _, c, _, _)| l.stroke.id == *id && c == cid)
                    .map(|(_, p, _, _, _)| *p)
            })
            .collect::<Vec<_>>();
        group.point = Point {
            x: points.iter().map(|p| p.x).sum::<f64>() / points.len() as f64,
            z: points.iter().map(|p| p.z).sum::<f64>() / points.len() as f64,
        };
    }
    (junctions, hints.into_iter().collect())
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolvedContext {
    pub linear: BTreeMap<StrokeId, LinearStructure>,
    pub network: PathNetwork,
    pub boundaries: Vec<BoundaryJunction>,
    pub wall_openings: BTreeMap<StrokeId, Vec<crate::strokes::Opening>>,
    pub foundations: BTreeMap<BuildingId, f64>,
    pub facades: BTreeMap<WallAnchor, FacadeStructure>,
    pub decisions: DecisionState,
    pub relations: Vec<RelationKey>,
    pub diagnostics: Vec<Diagnostic>,
}
fn xy(p: Point) -> [f64; 2] {
    [p.x, p.z]
}
fn point(p: [f64; 2]) -> Point {
    Point { x: p[0], z: p[1] }
}
/// 类型推断只修改返回决策；迟滞依赖的上一次类型属于可撤销数据。
pub fn linear_structure(
    stroke: &Arc<Stroke>,
    intent: &LinearIntent,
    old: Option<StrokeKind>,
    rules: ContextRules,
) -> LinearStructure {
    let kind = match intent.mode {
        StructureMode::Locked(kind) => kind,
        StructureMode::Auto => {
            let previous = old.unwrap_or(StrokeKind::Path);
            let path_limit = if previous == StrokeKind::Path {
                rules.path_exit
            } else {
                rules.path_enter
            };
            if stroke.height <= path_limit {
                StrokeKind::Path
            } else {
                let preference = match intent.style {
                    BoundaryStyle::Auto => 0.,
                    BoundaryStyle::Timber => 0.25,
                    BoundaryStyle::Stone => -0.25,
                };
                let wall_limit = if previous == StrokeKind::Wall {
                    rules.wall_exit
                } else {
                    rules.wall_enter
                } + preference;
                if stroke.height >= wall_limit || stroke.width > 0.8 {
                    StrokeKind::Wall
                } else {
                    StrokeKind::Fence
                }
            }
        }
    };
    LinearStructure {
        stroke: stroke.clone(),
        kind,
        control_ids: intent.control_ids.clone(),
        points: stroke.points.clone(),
        elevation_offset: intent.elevation_offset,
        style: intent.style,
    }
}
#[derive(Clone)]
struct Segment {
    source: SegmentSource,
    a: Point,
    b: Point,
    width: f64,
    offset: f64,
    cuts: Vec<(f64, Point)>,
}
fn segment_bounds(a: Point, b: Point, margin: f64) -> [f64; 4] {
    [
        a.x.min(b.x) - margin,
        a.z.min(b.z) - margin,
        a.x.max(b.x) + margin,
        a.z.max(b.z) + margin,
    ]
}
/// 网络先解析来源区间和节点；路面编译只消费此结果，不再自行决定连接。
pub fn path_network(
    lines: &BTreeMap<StrokeId, LinearStructure>,
    terrain: &TerrainDocument,
    old: &DecisionState,
    rules: ContextRules,
) -> (PathNetwork, Vec<ConnectionHint>, Vec<Diagnostic>) {
    let mut segments = Vec::new();
    for line in lines.values().filter(|l| l.kind == StrokeKind::Path) {
        for (i, pair) in line.stroke.points.windows(2).enumerate() {
            segments.push(Segment {
                source: SegmentSource {
                    stroke: line.stroke.id,
                    from: line.control_ids[i],
                    to: line.control_ids[i + 1],
                },
                a: pair[0],
                b: pair[1],
                width: line.stroke.width,
                offset: line.elevation_offset,
                cuts: vec![(0., pair[0]), (1., pair[1])],
            });
        }
    }
    let mut grid = SpatialGrid::<usize>::default();
    for (i, s) in segments.iter().enumerate() {
        grid.insert(i, segment_bounds(s.a, s.b, rules.connect_exit));
    }
    let mut decisions = BTreeSet::new();
    let mut unions = Vec::new();
    for i in 0..segments.len() {
        for j in grid
            .query(segment_bounds(
                segments[i].a,
                segments[i].b,
                rules.connect_exit,
            ))
            .into_iter()
            .filter(|j| *j > i)
        {
            let a = segments[i].clone();
            let b = segments[j].clone();
            if (a.offset - b.offset).abs() > rules.height_tolerance {
                continue;
            }
            let mut joins = linear::intersections(xy(a.a), xy(a.b), xy(b.a), xy(b.b));
            for (t, p, id) in [(0., a.a, a.source.from), (1., a.b, a.source.to)] {
                let hint = ConnectionHint {
                    endpoint: (a.source.stroke, id),
                    segment: (b.source.stroke, b.source.from, b.source.to),
                    kind: StrokeKind::Path,
                };
                let limit = if old.connections.contains(&hint) {
                    rules.connect_exit
                } else {
                    rules.connect_enter
                };
                let u = linear::projection(xy(p), xy(b.a), xy(b.b)).clamp(0., 1.);
                if p.distance(point(linear::mix(xy(b.a), xy(b.b), u))) <= limit {
                    joins.push((t, u));
                    decisions.insert(hint);
                }
            }
            for (u, p, id) in [(0., b.a, b.source.from), (1., b.b, b.source.to)] {
                let hint = ConnectionHint {
                    endpoint: (b.source.stroke, id),
                    segment: (a.source.stroke, a.source.from, a.source.to),
                    kind: StrokeKind::Path,
                };
                let limit = if old.connections.contains(&hint) {
                    rules.connect_exit
                } else {
                    rules.connect_enter
                };
                let t = linear::projection(xy(p), xy(a.a), xy(a.b)).clamp(0., 1.);
                if p.distance(point(linear::mix(xy(a.a), xy(a.b), t))) <= limit {
                    joins.push((t, u));
                    decisions.insert(hint);
                }
            }
            for (t, u) in joins {
                segments[i]
                    .cuts
                    .push((t, point(linear::mix(xy(a.a), xy(a.b), t))));
                segments[j]
                    .cuts
                    .push((u, point(linear::mix(xy(b.a), xy(b.b), u))));
                unions.push((i, t, j, u));
            }
        }
    }
    // 先保留来源顶点，再按连接关系合并节点。吸附不是简单丢弃重复 t 的坐标。
    let mut vertices = Vec::<(Point, f64, SegmentSource, Option<u64>)>::new();
    let mut cuts = Vec::new();
    for s in &mut segments {
        s.cuts.sort_by(|a, b| a.0.total_cmp(&b.0));
        s.cuts.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-8);
        let mut ids = Vec::new();
        for &(t, p) in &s.cuts {
            let id = vertices.len();
            let endpoint = if t < 1e-8 {
                Some(s.source.from)
            } else if t > 1. - 1e-8 {
                Some(s.source.to)
            } else {
                None
            };
            vertices.push((p, s.offset, s.source, endpoint));
            ids.push((t, id));
        }
        cuts.push(ids);
    }
    let mut parents = (0..vertices.len()).collect::<Vec<_>>();
    fn root(parents: &[usize], mut id: usize) -> usize {
        while parents[id] != id {
            id = parents[id];
        }
        id
    }
    for (i, t, j, u) in unions {
        let a = cuts[i]
            .iter()
            .min_by(|a, b| (a.0 - t).abs().total_cmp(&(b.0 - t).abs()))
            .unwrap()
            .1;
        let b = cuts[j]
            .iter()
            .min_by(|a, b| (a.0 - u).abs().total_cmp(&(b.0 - u).abs()))
            .unwrap()
            .1;
        let a = root(&parents, a);
        let b = root(&parents, b);
        parents[a.max(b)] = a.min(b);
    }
    let mut groups = BTreeMap::<usize, Vec<usize>>::new();
    for i in 0..vertices.len() {
        groups.entry(root(&parents, i)).or_default().push(i);
    }
    let mut result = PathNetwork::default();
    let mut remap = vec![0; vertices.len()];
    for members in groups.values() {
        let count = members.len() as f64;
        let p = Point {
            x: members.iter().map(|i| vertices[*i].0.x).sum::<f64>() / count,
            z: members.iter().map(|i| vertices[*i].0.z).sum::<f64>() / count,
        };
        let offset = members.iter().map(|i| vertices[*i].1).sum::<f64>() / count;
        let mut sources = members.iter().map(|i| vertices[*i].2).collect::<Vec<_>>();
        sources.sort();
        sources.dedup();
        let mut anchors = members
            .iter()
            .map(|i| (vertices[*i].2, vertices[*i].3))
            .collect::<Vec<_>>();
        anchors.sort();
        anchors.dedup();
        let id = result.nodes.len();
        for i in members {
            remap[*i] = id;
        }
        result.nodes.push(NetworkNode {
            point: p,
            height: terrain.height(p) + offset,
            sources,
            anchors,
        });
    }
    let mut diagnostics = Vec::new();
    for (i, s) in segments.iter().enumerate() {
        for pair in cuts[i].windows(2) {
            let ids = [remap[pair[0].1], remap[pair[1].1]];
            let a = &result.nodes[ids[0]];
            let b = &result.nodes[ids[1]];
            if ids[0] == ids[1] || a.point.distance(b.point) < 0.001 {
                continue;
            }
            let count = (a.point.distance(b.point) / CELL).ceil() as usize;
            let mut steep = false;
            let mut last = (a.point, a.height);
            let offsets = [
                a.height - terrain.height(a.point),
                b.height - terrain.height(b.point),
            ];
            for j in 1..=count {
                let t = j as f64 / count as f64;
                let p = point(linear::mix(xy(a.point), xy(b.point), t));
                let h = terrain.height(p) + offsets[0] + (offsets[1] - offsets[0]) * t;
                if (h - last.1).abs() / p.distance(last.0).max(1e-9)
                    > rules.max_slope_degrees.to_radians().tan()
                {
                    steep = true;
                }
                last = (p, h);
            }
            if steep {
                diagnostics.push(Diagnostic::Steep(s.source.stroke));
            }
            let contribution = (s.source, [pair[0].0, pair[1].0]);
            if let Some(edge) = result
                .edges
                .iter_mut()
                .find(|e| e.nodes == ids || e.nodes == [ids[1], ids[0]])
            {
                edge.width = edge.width.max(s.width);
                edge.passable &= !steep;
                edge.contributions.push(contribution);
            } else {
                result.edges.push(NetworkEdge {
                    source: s.source,
                    interval: contribution.1,
                    nodes: ids,
                    width: s.width,
                    passable: !steep,
                    contributions: vec![contribution],
                });
            }
        }
    }
    (result, decisions.into_iter().collect(), diagnostics)
}

/// 地基查询裁剪地形三角形，最高点包含旋转底面边界；不只采四角。
pub fn foundation(building: &Building, terrain: &TerrainDocument) -> f64 {
    if terrain.tiles.is_empty() {
        return building.placement().elevation;
    }
    foundation_with_intent(
        building,
        terrain,
        FoundationIntent {
            reference_y: building.placement().elevation,
            clearance: 0.,
        },
    )
}
/// 在相同地形三角形上裁剪旋转轮廓，取真实最高标高，建筑始终水平。
pub fn foundation_with_intent(
    building: &Building,
    terrain: &TerrainDocument,
    intent: FoundationIntent,
) -> f64 {
    if terrain.tiles.is_empty() {
        return intent.reference_y.max(0.) + intent.clearance;
    }
    let block = building
        .blocks()
        .iter()
        .find(|b| b.parent.is_none())
        .unwrap();
    let r = block.footprint;
    let p = building.placement();
    let polygon = vec![
        [r.x, r.z],
        [r.right(), r.z],
        [r.right(), r.back()],
        [r.x, r.back()],
    ]
    .into_iter()
    .map(|v| xy(world_point(point(v), p.x, p.z, p.yaw)))
    .collect::<Vec<_>>();
    let bounds = polygon.iter().fold(
        [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ],
        |a, v| {
            [
                a[0].min(v[0]),
                a[1].min(v[1]),
                a[2].max(v[0]),
                a[3].max(v[1]),
            ]
        },
    );
    let mut highest = f64::NEG_INFINITY;
    for z in (bounds[1] / CELL).floor() as i32..=(bounds[3] / CELL).ceil() as i32 {
        for x in (bounds[0] / CELL).floor() as i32..=(bounds[2] / CELL).ceil() as i32 {
            let a = [x as f64 * CELL, z as f64 * CELL];
            let b = [a[0] + CELL, a[1]];
            let c = [a[0], a[1] + CELL];
            let d = [a[0] + CELL, a[1] + CELL];
            for triangle in [[a, b, d], [a, d, c]] {
                for v in linear::clip_convex(polygon.clone(), &triangle) {
                    highest = highest.max(terrain.height(point(v)));
                }
            }
        }
    }
    intent
        .reference_y
        .max(if highest.is_finite() { highest } else { 0. })
        + intent.clearance
}
pub fn face(face: WallFace) -> Face {
    match face {
        WallFace::Front => Face::Front,
        WallFace::Back => Face::Back,
        WallFace::Left => Face::Left,
        WallFace::Right => Face::Right,
    }
}
/// 与正式墙面相同的沿墙方向，保持背面和左面锚点语义一致。
pub fn wall_surfaces(building: &Building, base: f64) -> Vec<WallSurface> {
    let p = building.placement();
    let mut out = Vec::new();
    for block in BuildingRule.resolve(building).blocks {
        let r = block.footprint;
        for (face, a, b) in [
            (WallFace::Front, [r.x, r.z], [r.right(), r.z]),
            (WallFace::Back, [r.right(), r.back()], [r.x, r.back()]),
            (WallFace::Left, [r.x, r.back()], [r.x, r.z]),
            (WallFace::Right, [r.right(), r.z], [r.right(), r.back()]),
        ] {
            out.push(WallSurface {
                anchor: WallAnchor {
                    building: building.id(),
                    block: block.block,
                    face,
                },
                start: world_point(point(a), p.x, p.z, p.yaw),
                end: world_point(point(b), p.x, p.z, p.yaw),
                base: base + block.base_elevation,
                height: block.height,
            });
        }
    }
    out
}
pub(super) fn doors(
    surface: &WallSurface,
    network: &PathNetwork,
    terrain: &TerrainDocument,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<DoorRequest> {
    let length = surface.start.distance(surface.end);
    let mut doors: Vec<DoorRequest> = Vec::new();
    for edge in network.edges.iter().filter(|e| e.passable) {
        let a = &network.nodes[edge.nodes[0]];
        let b = &network.nodes[edge.nodes[1]];
        let tangent = [
            surface.end.x - surface.start.x,
            surface.end.z - surface.start.z,
        ];
        let path = [b.point.x - a.point.x, b.point.z - a.point.z];
        let cosine = (tangent[0] * path[0] + tangent[1] * path[1]).abs()
            / length
            / b.point.distance(a.point);
        if cosine >= 0.85 {
            continue;
        }
        for (t, u) in
            linear::intersections(xy(surface.start), xy(surface.end), xy(a.point), xy(b.point))
        {
            let p = point(linear::mix(xy(a.point), xy(b.point), u));
            let offset = (a.height - terrain.height(a.point)) * (1. - u)
                + (b.height - terrain.height(b.point)) * u;
            if (terrain.height(p) + offset - surface.base).abs() > 0.2 {
                diagnostics.push(Diagnostic::BlockedEntrance(surface.anchor));
                continue;
            }
            let width = (edge.width / (1. - cosine * cosine).sqrt() + 0.2).min(4.);
            let along = t * length;
            if along - width / 2. < 0.15
                || along + width / 2. > length - 0.15
                || surface.height < 2.2
            {
                continue;
            }
            if let Some(existing) = doors
                .iter_mut()
                .find(|d| (d.along - along).abs() < (d.width + width) / 2.)
            {
                let lo = (existing.along - existing.width / 2.).min(along - width / 2.);
                let hi = (existing.along + existing.width / 2.).max(along + width / 2.);
                if hi - lo <= 4. {
                    existing.along = (lo + hi) / 2.;
                    existing.width = hi - lo;
                    existing
                        .sources
                        .extend(edge.contributions.iter().map(|(s, _)| s.stroke));
                    existing
                        .segments
                        .extend(edge.contributions.iter().map(|(s, _)| *s));
                }
            } else {
                doors.push(DoorRequest {
                    along,
                    width,
                    height: 2.,
                    sources: edge.contributions.iter().map(|(s, _)| s.stroke).collect(),
                    segments: edge.contributions.iter().map(|(s, _)| *s).collect(),
                });
            }
        }
    }
    doors.sort_by(|a, b| a.along.total_cmp(&b.along));
    for d in &mut doors {
        d.sources.sort();
        d.sources.dedup();
        d.segments.sort();
        d.segments.dedup();
    }
    doors
}
pub fn facade(
    surface: WallSurface,
    input: &ContextInput,
    network: &PathNetwork,
    diagnostics: &mut Vec<Diagnostic>,
) -> FacadeStructure {
    let doors = doors(&surface, network, &input.scene.terrain, diagnostics);
    facade_with_doors(surface, input, doors, diagnostics)
}
pub(super) fn facade_with_doors(
    surface: WallSurface,
    input: &ContextInput,
    doors: Vec<DoorRequest>,
    diagnostics: &mut Vec<Diagnostic>,
) -> FacadeStructure {
    let length = surface.start.distance(surface.end);
    let mut active = Vec::new();
    let mut sleeping = Vec::new();
    for window in input
        .scene
        .openings
        .values()
        .filter(|w| w.host == surface.anchor)
    {
        let valid = window.along - window.width / 2. >= 0.15
            && window.along + window.width / 2. <= length - 0.15
            && window.elevation - window.height / 2. >= 0.15
            && window.elevation + window.height / 2. <= surface.height - 0.15
            && !doors.iter().any(|d| {
                (d.along - window.along).abs() < (d.width + window.width) / 2. + 0.1
                    && window.elevation - window.height / 2. < d.height + 0.1
            });
        if valid {
            active.push(window.clone());
        } else {
            sleeping.push(window.id);
            diagnostics.push(Diagnostic::Sleeping(window.id));
        }
    }
    active.sort_by(|a, b| a.along.total_cmp(&b.along).then(a.id.cmp(&b.id)));
    let mut assemblies: Vec<OpeningAssembly> = Vec::new();
    for window in active {
        let join = assemblies.last().is_some_and(|group| {
            let last = group.openings.last().unwrap();
            let first = &group.openings[0];
            let was = input
                .scene
                .decisions
                .window_groups
                .iter()
                .any(|g| g.contains(&window.id) && g.contains(&last.id));
            let gap = window.along - window.width / 2. - (last.along + last.width / 2.);
            group.members.len() < 4
                && window.style == first.style
                && (window.elevation - first.elevation).abs() <= 0.08
                && (window.height - first.height).abs() <= 0.1
                && gap
                    <= if was {
                        input.scene.rules.window_exit
                    } else {
                        input.scene.rules.window_enter
                    }
                && window.along + window.width / 2. - (first.along - first.width / 2.) <= 5.
        });
        if join {
            let group = assemblies.last_mut().unwrap();
            group.members.push(window.id);
            group.openings.push(window);
        } else {
            assemblies.push(OpeningAssembly {
                members: vec![window.id],
                openings: vec![window],
            });
        }
    }
    FacadeStructure {
        surface,
        doors,
        assemblies,
        sleeping,
    }
}
/// 全部语义阶段有向执行；诊断可描述休眠/不可通行，不伪装成几何失败。
pub fn resolve(input: &ContextInput) -> ResolvedContext {
    resolve_cached(input, &mut super::context_cache::ContextCache::default())
}
/// 缓存保存精确语义输入；预览使用独立副本，不向正式索引写入候选关系。
pub fn resolve_cached(
    input: &ContextInput,
    cache: &mut super::context_cache::ContextCache,
) -> ResolvedContext {
    cache.begin();
    let mut out = ResolvedContext::default();
    let mut strokes = input.strokes.iter().collect::<Vec<_>>();
    strokes.sort_by_key(|s| s.id);
    for stroke in strokes {
        let intent = input
            .scene
            .linear
            .get(&stroke.id)
            .cloned()
            .unwrap_or_else(|| LinearIntent::legacy(stroke));
        let line = linear_structure(
            stroke,
            &intent,
            input.scene.decisions.kinds.get(&stroke.id).copied(),
            input.scene.rules,
        );
        out.decisions.kinds.insert(stroke.id, line.kind);
        out.linear.insert(stroke.id, line);
    }
    let (network, connections, diagnostics) = cache.network(&out.linear, input);
    out.network = network;
    out.decisions.connections = connections;
    let (boundaries, hints) = boundary_junctions(
        &out.linear,
        &input.scene.terrain,
        &input.scene.decisions,
        input.scene.rules,
    );
    out.decisions.connections.extend(hints);
    for j in &boundaries {
        out.relations
            .push(RelationKey::Boundary(j.kind, j.members.clone()));
    }
    out.boundaries = boundaries;
    for junction in &out.boundaries {
        for (id, point_id) in &junction.members {
            if let Some(line) = out.linear.get_mut(id) {
                if line.control_ids.first() == Some(point_id) {
                    line.points[0] = junction.point;
                }
                if line.control_ids.last() == Some(point_id) {
                    *line.points.last_mut().unwrap() = junction.point;
                }
            }
        }
    }
    for line in out.linear.values().filter(|l| l.kind == StrokeKind::Wall) {
        let mut wall = line.stroke.as_ref().clone();
        wall.kind = StrokeKind::Wall;
        wall.points = line.points.clone();
        let mut paths = Vec::new();
        for edge in out.network.edges.iter().filter(|e| e.passable) {
            let a = &out.network.nodes[edge.nodes[0]];
            let b = &out.network.nodes[edge.nodes[1]];
            let offset = (a.height - input.scene.terrain.height(a.point) + b.height
                - input.scene.terrain.height(b.point))
                / 2.;
            if (offset - line.elevation_offset).abs() > input.scene.rules.height_tolerance {
                continue;
            }
            // 同一融合边仍带有每条原笔画的来源，门洞不能只归属于第一条道路。
            for (source, _) in &edge.contributions {
                paths.push(Arc::new(Stroke {
                    id: source.stroke,
                    kind: StrokeKind::Path,
                    points: vec![a.point, b.point],
                    width: edge.width,
                    height: 0.,
                }));
            }
        }
        let input = crate::strokes::WallInput {
            wall: Arc::new(wall),
            paths,
        };
        let mut openings = crate::strokes::openings(&input);
        for opening in &mut openings {
            opening.sources.sort();
            opening.sources.dedup();
        }
        out.wall_openings.insert(line.stroke.id, openings);
    }
    out.diagnostics = diagnostics;
    for node in &out.network.nodes {
        if node.sources.len() > 1 {
            out.relations
                .push(RelationKey::Network(node.anchors.clone()));
        }
    }
    let mut buildings = input.buildings.iter().collect::<Vec<_>>();
    buildings.sort_by_key(|b| b.id());
    for building in buildings {
        let base = cache.foundation(building, &input.scene);
        out.foundations.insert(building.id(), base);
        for surface in wall_surfaces(building, base) {
            let f = cache.facade(surface, input, &out.network, &mut out.diagnostics);
            for group in &f.assemblies {
                if group.members.len() > 1 {
                    out.decisions.window_groups.push(group.members.clone());
                    out.relations.push(RelationKey::Windows(
                        f.surface.anchor,
                        group.members.clone(),
                    ));
                }
            }
            for door in &f.doors {
                out.relations.push(RelationKey::Crossing(
                    f.surface.anchor,
                    door.segments.clone(),
                ));
            }
            out.facades.insert(f.surface.anchor, f);
        }
    }
    for window in input.scene.openings.values() {
        if !out.facades.contains_key(&window.host) {
            out.diagnostics.push(Diagnostic::Sleeping(window.id));
        }
    }
    out.relations.sort();
    out.relations.dedup();
    cache.prune(input);
    out
}

/// 按射线穿过的高度场网格单元前进，精确测试实际显示的两个三角形。
/// 避免每次鼠标移动扫描整片地形，也不使用可能漏过尖峰的粗略步进。
pub fn terrain_hit(
    origin: [f64; 3],
    direction: [f64; 3],
    terrain: &TerrainDocument,
) -> Option<[f64; 3]> {
    if origin
        .iter()
        .chain(direction.iter())
        .any(|v| !v.is_finite())
    {
        return None;
    }
    let mut near: f64 = 0.;
    let mut far = f64::INFINITY;
    for (i, lo, hi) in [(0, -25., 25.), (1, -1.51, 3.01), (2, -15.5, 15.5)] {
        if direction[i].abs() < 1e-12 {
            if origin[i] < lo || origin[i] > hi {
                return None;
            }
        } else {
            let a = (lo - origin[i]) / direction[i];
            let b = (hi - origin[i]) / direction[i];
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
    }
    if far < near {
        return None;
    }
    let position = |t: f64| {
        [
            origin[0] + direction[0] * t,
            origin[1] + direction[1] * t,
            origin[2] + direction[2] * t,
        ]
    };
    let mut t = near;
    for _ in 0..1024 {
        if t > far {
            return None;
        }
        let p = position(t + 1e-8);
        let x = (p[0] / CELL).floor() as i32;
        let z = (p[2] / CELL).floor() as i32;
        let mut end = far;
        for (axis, cell) in [(0, x), (2, z)] {
            if direction[axis].abs() > 1e-12 {
                let border = (cell + if direction[axis] > 0. { 1 } else { 0 }) as f64 * CELL;
                let crossing = (border - origin[axis]) / direction[axis];
                if crossing > t + 1e-9 {
                    end = end.min(crossing);
                }
            }
        }
        let a = [x as f64 * CELL, terrain.vertex(x, z), z as f64 * CELL];
        let b = [
            (x + 1) as f64 * CELL,
            terrain.vertex(x + 1, z),
            z as f64 * CELL,
        ];
        let c = [
            x as f64 * CELL,
            terrain.vertex(x, z + 1),
            (z + 1) as f64 * CELL,
        ];
        let d = [
            (x + 1) as f64 * CELL,
            terrain.vertex(x + 1, z + 1),
            (z + 1) as f64 * CELL,
        ];
        let hit = [[a, d, b], [a, c, d]]
            .into_iter()
            .filter_map(|triangle| linear::ray_triangle(origin, direction, triangle, 1e-10))
            .filter(|hit| *hit >= t - 1e-7 && *hit <= end + 1e-7)
            .min_by(f64::total_cmp);
        if let Some(hit) = hit {
            return Some(position(hit));
        }
        if end == far {
            break;
        }
        t = end + 1e-8;
    }
    None
}

/// 将最终立面开口写入派生布局；权威窗户仍在 SceneContext 中，休眠不会删除来源。
/// 显示开口槽位与语义来源的专用映射；位置改变不重编号，合并/拆分产生新部件。
/// 一墙最多 4064 个来源区段，新旧集合同时保留也小于 0x4000 个槽位。
#[derive(Clone, Default)]
pub struct OpeningParts {
    slots: BTreeMap<(WallAnchor, Vec<SegmentSource>), u16>,
}
impl OpeningParts {
    pub fn update(&mut self, context: &ResolvedContext) {
        let mut next = BTreeMap::new();
        for (host, facade) in &context.facades {
            // 本次分配保留全部旧槽，避免新部件冒充同时退出的旧部件。
            let mut used = self
                .slots
                .iter()
                .filter(|(key, _)| key.0 == *host)
                .map(|(_, slot)| *slot)
                .collect::<BTreeSet<_>>();
            for door in &facade.doors {
                let key = (*host, door.segments.clone());
                let slot = self.slots.get(&key).copied().unwrap_or_else(|| {
                    let slot = (0..0x4000)
                        .find(|slot| !used.contains(slot))
                        .expect("来源区段上限保证开口槽位充足");
                    used.insert(slot);
                    slot
                });
                next.insert(key, slot);
            }
        }
        self.slots = next;
    }
    pub fn slot(&self, host: WallAnchor, door: &DoorRequest) -> u16 {
        self.slots[&(host, door.segments.clone())]
    }
}

pub fn apply_layout(layout: &mut crate::BuildingLayout, context: &ResolvedContext) {
    let mut parts = OpeningParts::default();
    parts.update(context);
    apply_layout_with_parts(layout, context, &parts);
}
/// 正式增量生成使用已发布的来源映射，预览和离线生成仍可使用相同布局入口。
pub fn apply_layout_with_parts(
    layout: &mut crate::BuildingLayout,
    context: &ResolvedContext,
    parts: &OpeningParts,
) {
    if let Some(base) = context.foundations.get(&layout.building) {
        layout.placement.elevation = *base;
    }
    for block in &mut layout.blocks {
        for wall_face in [
            WallFace::Front,
            WallFace::Back,
            WallFace::Left,
            WallFace::Right,
        ] {
            let host = WallAnchor {
                building: layout.building,
                block: block.block,
                face: wall_face,
            };
            let Some(facade) = context.facades.get(&host) else {
                continue;
            };
            let facing = face(wall_face);
            let windows = facade
                .assemblies
                .iter()
                .flat_map(|a| &a.openings)
                .collect::<Vec<_>>();
            block.windows.retain(|w| {
                w.key.face != facing
                    || (!windows
                        .iter()
                        .any(|m| (w.along_wall - m.along).abs() < (w.width + m.width) / 2. + 0.15)
                        && !facade.doors.iter().any(|d| {
                            (w.along_wall - d.along).abs() < (w.width + d.width) / 2. + 0.15
                                && w.elevation - block.base_elevation - w.height / 2.
                                    < d.height + 0.1
                        }))
            });
            for w in windows {
                block.windows.push(crate::WindowPlacement {
                    key: crate::PartKey {
                        building: layout.building,
                        block: block.block,
                        face: facing,
                        story: 0,
                        slot: 0x8000 + w.id.0 as u16,
                    },
                    along_wall: w.along,
                    elevation: block.base_elevation + w.elevation,
                    width: w.width,
                    height: w.height,
                });
            }
            for d in &facade.doors {
                block.windows.push(crate::WindowPlacement {
                    key: crate::PartKey {
                        building: layout.building,
                        block: block.block,
                        face: facing,
                        story: 0,
                        slot: 0x4000 + parts.slot(host, d),
                    },
                    along_wall: d.along,
                    elevation: block.base_elevation + d.height / 2.,
                    width: d.width,
                    height: d.height,
                });
            }
        }
        block.windows.sort_by_key(|w| w.key);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{
        sample_building,
        terrain::{BrushKind, BrushSample},
    };
    fn stroke(id: u64, a: Point, b: Point) -> Arc<Stroke> {
        Arc::new(Stroke {
            id: StrokeId(id),
            kind: StrokeKind::Path,
            points: vec![a, b],
            width: 1.,
            height: 0.,
        })
    }
    #[test]
    fn types_use_saved_hysteresis_and_locked_intents() {
        let mut s = stroke(1, Point { x: 0., z: 0. }, Point { x: 4., z: 0. });
        let mut i = LinearIntent::legacy(&s);
        i.mode = StructureMode::Auto;
        Arc::make_mut(&mut s).height = 0.12;
        assert_eq!(
            linear_structure(&s, &i, Some(StrokeKind::Path), ContextRules::default()).kind,
            StrokeKind::Path
        );
        assert_eq!(
            linear_structure(&s, &i, Some(StrokeKind::Fence), ContextRules::default()).kind,
            StrokeKind::Wall
        ); // 宽笔画优先实墙
        Arc::make_mut(&mut s).width = 0.4;
        assert_eq!(
            linear_structure(&s, &i, Some(StrokeKind::Fence), ContextRules::default()).kind,
            StrokeKind::Fence
        );
    }
    #[test]
    fn crossings_height_separation_and_source_order_are_stable() {
        let a = stroke(1, Point { x: -2., z: 0. }, Point { x: 2., z: 0. });
        let b = stroke(2, Point { x: 0., z: -2. }, Point { x: 0., z: 2. });
        let mut input = ContextInput {
            buildings: Vec::new(),
            strokes: vec![a, b],
            scene: Arc::default(),
        };
        let first = resolve(&input);
        assert_eq!(first.network.edges.len(), 4);
        assert!(first.network.nodes.iter().any(|n| n.sources.len() == 2));
        input.strokes.reverse();
        assert_eq!(first, resolve(&input));
        let scene = Arc::make_mut(&mut input.scene);
        let mut intent = LinearIntent::legacy(&input.strokes[0]);
        intent.elevation_offset = 1.;
        scene.linear.insert(StrokeId(2), intent);
        assert_eq!(resolve(&input).network.edges.len(), 2);
    }
    #[test]
    fn door_sleeps_original_window_and_terrain_moves_foundation() {
        let b = Arc::new(
            Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6., 3.)).unwrap(),
        );
        let root = b.blocks()[0].id;
        let host = WallAnchor {
            building: b.id(),
            block: root,
            face: WallFace::Front,
        };
        let mut scene = SceneContext::default();
        scene.openings.insert(
            OpeningId(1),
            OpeningIntent {
                id: OpeningId(1),
                host,
                along: 3.,
                elevation: 1.4,
                width: 0.9,
                height: 1.2,
                style: 0,
            },
        );
        let mut input = ContextInput {
            buildings: vec![b],
            strokes: vec![stroke(1, Point { x: 3., z: -2. }, Point { x: 3., z: 2. })],
            scene: Arc::new(scene),
        };
        let first = resolve(&input);
        assert_eq!(first.facades[&host].sleeping, vec![OpeningId(1)]);
        input.strokes.clear();
        assert!(resolve(&input).facades[&host].sleeping.is_empty());
        Arc::make_mut(&mut input.scene).terrain = input
            .scene
            .terrain
            .brushed(BrushSample {
                center: Point { x: 3., z: 1. },
                radius: 1.5,
                amount: 0.2,
                kind: BrushKind::Raise,
            })
            .unwrap();
        assert!(resolve(&input).foundations[&BuildingId::new(1).unwrap()] > 0.);
    }
}
