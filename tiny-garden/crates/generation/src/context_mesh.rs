//! 上下文结构的几何编译；统一三角/立方体基础，不在渲染层实现规则。
//! 地形和道路按固定区域分块，缓存键捕获实际边、局部高度块和样式。
use crate::{
    context::*,
    incremental::Cancellation,
    mesh::{BuildingMesh, MaterialKey, MeshBatch, MeshData, MeshError},
    strokes::{Curve, Opening, WallInput, compile_wall_resolved, cuboid, quad},
};
use garden_domain::{
    BuildingId,
    context::*,
    strokes::{Point, StrokeId, StrokeKind},
    terrain::{CELL, TILE_CELLS, TerrainDocument, TileId},
};
use garden_geometry::linear;
use std::{collections::BTreeSet, sync::Arc};
/// 强类型来源键；窗组合使用所有稳定成员编号，几何位置不作为身份。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DerivedGroup {
    Terrain(TileId),
    Road(TileId),
    Foundation(BuildingId),
    Assembly(WallAnchor, Vec<OpeningId>),
}
/// 仅在展示边界分配兼容编号，避免把大 BuildingId/BlockId 按位拼接造成碰撞。
/// 删除来源后释放键，序号不回退，因此异步旧结果不能冒充新组。
#[derive(Clone, Default)]
pub struct GroupRegistry {
    ids: std::collections::BTreeMap<DerivedGroup, StrokeId>,
    next: u64,
    live: BTreeSet<DerivedGroup>,
}
impl GroupRegistry {
    pub fn begin(&mut self) {
        self.live.clear();
    }
    pub fn id(&mut self, group: DerivedGroup) -> StrokeId {
        self.live.insert(group.clone());
        if let Some(id) = self.ids.get(&group) {
            return *id;
        }
        self.next = self
            .next
            .checked_add(1)
            .filter(|n| *n < (1 << 60))
            .expect("派生组序号已耗尽");
        let flag = match group {
            DerivedGroup::Terrain(_) => 1 << 63,
            DerivedGroup::Road(_) => 1 << 62,
            DerivedGroup::Foundation(_) => 1 << 61,
            DerivedGroup::Assembly(..) => 1 << 60,
        };
        let id = StrokeId(flag | self.next);
        self.ids.insert(group, id);
        id
    }
    pub fn finish(&mut self) {
        self.ids.retain(|key, _| self.live.contains(key));
        self.live.clear();
    }
}
pub fn is_terrain(id: StrokeId) -> bool {
    id.0 & (1 << 63) != 0
}
pub fn is_road(id: StrokeId) -> bool {
    id.0 == 0 || id.0 & (1 << 62) != 0
}
fn mesh(data: MeshData, material: MaterialKey) -> Result<BuildingMesh, MeshError> {
    if data.positions.is_empty() {
        return Ok(BuildingMesh::default());
    }
    data.validate()?;
    Ok(BuildingMesh {
        batches: vec![MeshBatch { data, material }],
    })
}
/// 根据变形后的真实三角形重算法线，之后仍使用共用的后台切线准备。
fn normals(data: &mut MeshData) {
    data.normals = vec![[0.; 3]; data.positions.len()];
    for t in data.indices.chunks_exact(3) {
        let a = data.positions[t[0] as usize];
        let b = data.positions[t[1] as usize];
        let c = data.positions[t[2] as usize];
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        for i in t {
            for (axis, value) in n.iter().enumerate() {
                data.normals[*i as usize][axis] += value;
            }
        }
    }
    for n in &mut data.normals {
        let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if l > 1e-9 {
            for a in n.iter_mut() {
                *a /= l;
            }
        } else {
            *n = [0., 1., 0.];
        }
    }
}
pub fn terrain_tile(
    id: TileId,
    terrain: &TerrainDocument,
    cancel: &Cancellation,
) -> Result<BuildingMesh, MeshError> {
    let mut data = MeshData::default();
    for z in 0..TILE_CELLS {
        cancel.check()?;
        for x in 0..TILE_CELLS {
            let a = [
                (id.0 * 16 + x as i32) as f64 * CELL,
                (id.1 * 16 + z as i32) as f64 * CELL,
            ];
            if a[0] < -25. || a[0] >= 25. || a[1] < -15.5 || a[1] >= 15.5 {
                continue;
            }
            let points = [
                [a[0], a[1]],
                [a[0], a[1] + CELL],
                [a[0] + CELL, a[1] + CELL],
                [a[0] + CELL, a[1]],
            ];
            let start = data.uvs.len();
            quad(
                &mut data,
                points.map(|p| {
                    [
                        p[0],
                        terrain.height(Point { x: p[0], z: p[1] }) - 0.005,
                        p[1],
                    ]
                }),
                [0., 1., 0.],
            );
            for (uv, p) in data.uvs[start..].iter_mut().zip(points) {
                *uv = [p[0] as f32, p[1] as f32];
            }
        }
    }
    normals(&mut data);
    mesh(data, MaterialKey::Plaster)
}
/// 只编译经过该块的道路边；高度不同的覆盖分别保留，不能平面合并成假连接。
pub fn road_tile(
    id: TileId,
    network: &PathNetwork,
    terrain: &TerrainDocument,
    cancel: &Cancellation,
) -> Result<BuildingMesh, MeshError> {
    let bounds = [
        id.0 as f64 * 4.,
        id.1 as f64 * 4.,
        (id.0 + 1) as f64 * 4.,
        (id.1 + 1) as f64 * 4.,
    ];
    let edges = network
        .edges
        .iter()
        .filter(|e| {
            let a = network.nodes[e.nodes[0]].point;
            let b = network.nodes[e.nodes[1]].point;
            bounds_overlap(
                bounds,
                [
                    a.x.min(b.x) - e.width / 2.,
                    a.z.min(b.z) - e.width / 2.,
                    a.x.max(b.x) + e.width / 2.,
                    a.z.max(b.z) + e.width / 2.,
                ],
            )
        })
        .collect::<Vec<_>>();
    let mut data = MeshData::default();
    for z in 0..16 {
        cancel.check()?;
        for x in 0..16 {
            let px = bounds[0] + x as f64 * CELL;
            let pz = bounds[1] + z as f64 * CELL;
            let center = [px + CELL / 2., pz + CELL / 2.];
            let mut levels = Vec::<f64>::new();
            for edge in &edges {
                let a = &network.nodes[edge.nodes[0]];
                let b = &network.nodes[edge.nodes[1]];
                let t = linear::projection(center, [a.point.x, a.point.z], [b.point.x, b.point.z])
                    .clamp(0., 1.);
                let q = linear::mix([a.point.x, a.point.z], [b.point.x, b.point.z], t);
                if linear::distance(center, q) <= edge.width / 2. {
                    let offset = (a.height - terrain.height(a.point)) * (1. - t)
                        + (b.height - terrain.height(b.point)) * t;
                    if !levels.iter().any(|o| (*o - offset).abs() < 0.001) {
                        levels.push(offset);
                    }
                }
            }
            levels.sort_by(f64::total_cmp);
            for offset in levels {
                quad(
                    &mut data,
                    [
                        [px, pz],
                        [px, pz + CELL],
                        [px + CELL, pz + CELL],
                        [px + CELL, pz],
                    ]
                    .map(|p| {
                        [
                            p[0],
                            terrain.height(Point { x: p[0], z: p[1] }) + offset + 0.015,
                            p[1],
                        ]
                    }),
                    [0., 1., 0.],
                );
            }
        }
    }
    normals(&mut data);
    mesh(data, MaterialKey::Plaster)
}
pub fn road_tiles(network: &PathNetwork) -> BTreeSet<TileId> {
    let mut tiles = BTreeSet::new();
    for edge in &network.edges {
        let a = network.nodes[edge.nodes[0]].point;
        let b = network.nodes[edge.nodes[1]].point;
        tiles.extend(TerrainDocument::tiles_in([
            a.x.min(b.x) - edge.width / 2.,
            a.z.min(b.z) - edge.width / 2.,
            a.x.max(b.x) + edge.width / 2.,
            a.z.max(b.z) + edge.width / 2.,
        ]));
    }
    tiles
}
pub fn boundary(
    line: &LinearStructure,
    arches: &[Opening],
    junctions: &[BoundaryJunction],
    terrain: &TerrainDocument,
    cancel: &Cancellation,
) -> Result<BuildingMesh, MeshError> {
    if line.kind == StrokeKind::Wall {
        let mut s = line.stroke.as_ref().clone();
        s.kind = StrokeKind::Wall;
        s.points = line.points.clone();
        let mut output = compile_wall_resolved(
            &WallInput {
                wall: Arc::new(s),
                paths: Vec::new(),
            },
            arches,
            cancel,
        )?;
        let mut corners = MeshData::default();
        for j in junctions
            .iter()
            .filter(|j| j.members.first().is_some_and(|m| m.0 == line.stroke.id))
        {
            cuboid(
                &mut corners,
                [j.point.x, j.height / 2., j.point.z],
                [j.width, j.height, j.width],
                [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            );
        }
        if !corners.positions.is_empty() {
            output.batches.push(MeshBatch {
                data: corners,
                material: MaterialKey::Stone,
            });
        }
        for batch in &mut output.batches {
            for p in &mut batch.data.positions {
                p[1] += (terrain.height(Point {
                    x: p[0] as f64,
                    z: p[2] as f64,
                }) + line.elevation_offset) as f32;
            }
            normals(&mut batch.data);
        }
        return Ok(output);
    }
    let curve = Curve::new(&line.points);
    let posts = (curve.length / 1.2).ceil() as usize;
    let mut data = MeshData::default();
    for i in 0..=posts {
        cancel.check()?;
        let endpoint = if i == 0 {
            line.control_ids.first()
        } else if i == posts {
            line.control_ids.last()
        } else {
            None
        };
        if endpoint.is_some_and(|cid| {
            junctions.iter().any(|j| {
                j.members.contains(&(line.stroke.id, *cid)) && j.members[0].0 != line.stroke.id
            })
        }) {
            continue;
        }
        let (p, _) = curve.sample(curve.length * i as f64 / posts as f64);
        let base = terrain.height(p) + line.elevation_offset;
        cuboid(
            &mut data,
            [p.x, base + line.stroke.height / 2., p.z],
            [0.14, line.stroke.height.max(0.12), 0.14],
            [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        );
    }
    for i in 0..posts {
        let (a, _) = curve.sample(curve.length * i as f64 / posts as f64);
        let (b, _) = curve.sample(curve.length * (i + 1) as f64 / posts as f64);
        let dx = b.x - a.x;
        let dz = b.z - a.z;
        let length = dx.hypot(dz);
        let dy = terrain.height(b) - terrain.height(a);
        let span = (length * length + dy * dy).sqrt();
        let axis = [dx / span, dy / span, dz / span];
        let z = [-dz / length, 0., dx / length];
        let y = [
            -z[2] * axis[1],
            z[2] * axis[0] - z[0] * axis[2],
            z[0] * axis[1],
        ];
        for fraction in [0.3, 0.8] {
            cuboid(
                &mut data,
                [
                    (a.x + b.x) / 2.,
                    (terrain.height(a) + terrain.height(b)) / 2.
                        + line.elevation_offset
                        + line.stroke.height * fraction,
                    (a.z + b.z) / 2.,
                ],
                [span, 0.09, 0.08],
                [axis, y, z],
            );
        }
    }
    mesh(data, MaterialKey::Timber)
}
pub fn assembly(
    facade: &FacadeStructure,
    members: &OpeningAssembly,
) -> Result<BuildingMesh, MeshError> {
    let Some(first) = members.openings.first() else {
        return Ok(BuildingMesh::default());
    };
    let last = members.openings.last().unwrap();
    let lo = first.along - first.width / 2.;
    let hi = last.along + last.width / 2.;
    let length = facade.surface.start.distance(facade.surface.end);
    let dx = (facade.surface.end.x - facade.surface.start.x) / length;
    let dz = (facade.surface.end.z - facade.surface.start.z) / length;
    let along = (lo + hi) / 2.;
    let p = Point {
        x: facade.surface.start.x + dx * along,
        z: facade.surface.start.z + dz * along,
    };
    let mut data = MeshData::default();
    for height in [
        first.elevation - first.height / 2. - 0.05,
        first.elevation + first.height / 2. + 0.05,
    ] {
        cuboid(
            &mut data,
            [p.x, facade.surface.base + height, p.z],
            [hi - lo + 0.12, 0.1, 0.13],
            [[dx, 0., dz], [0., 1., 0.], [-dz, 0., dx]],
        );
    }
    mesh(data, MaterialKey::Timber)
}
/// 地基仅填补实际地面到水平底座的差值，不移动或拉伸房屋主体。
pub fn foundation_mesh(
    building: &garden_domain::Building,
    base: f64,
    terrain: &TerrainDocument,
) -> Result<BuildingMesh, MeshError> {
    let b = building
        .blocks()
        .iter()
        .find(|b| b.parent.is_none())
        .unwrap();
    let r = b.footprint;
    let p = building.placement();
    let corners = [
        Point { x: r.x, z: r.z },
        Point {
            x: r.right(),
            z: r.z,
        },
        Point {
            x: r.right(),
            z: r.back(),
        },
        Point {
            x: r.x,
            z: r.back(),
        },
    ]
    .map(|q| world_point(q, p.x, p.z, p.yaw));
    let mut data = MeshData::default();
    for i in 0..4 {
        let a = corners[i];
        let b = corners[(i + 1) % 4];
        let length = a.distance(b);
        let steps = (length / CELL).ceil() as usize;
        for j in 0..steps {
            let x = linear::mix([a.x, a.z], [b.x, b.z], j as f64 / steps as f64);
            let z = linear::mix([a.x, a.z], [b.x, b.z], (j + 1) as f64 / steps as f64);
            let h0 = terrain.height(Point { x: x[0], z: x[1] });
            let h1 = terrain.height(Point { x: z[0], z: z[1] });
            if base - h0 < 0.005 && base - h1 < 0.005 {
                continue;
            }
            quad(
                &mut data,
                [
                    [x[0], h0 - 0.02, x[1]],
                    [x[0], base, x[1]],
                    [z[0], base, z[1]],
                    [z[0], h1 - 0.02, z[1]],
                ],
                [(b.z - a.z) / length, 0., -(b.x - a.x) / length],
            );
        }
    }
    // 水平顶面封住地基，内部凹地或门洞处也不会露出未整平的原地形。
    quad(
        &mut data,
        [corners[0], corners[3], corners[2], corners[1]].map(|q| [q.x, base, q.z]),
        [0., 1., 0.],
    );
    normals(&mut data);
    mesh(data, MaterialKey::Stone)
}
