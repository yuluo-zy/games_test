//! CPU reference implementation: arc-length brick layout and a path influence
//! field. Real geometry holes, no fragment-discard-only pick mismatch.
use crate::{
    incremental::Cancellation,
    mesh::{BuildingMesh, MaterialKey, MeshBatch, MeshData, MeshError},
};
use garden_domain::strokes::{Point, Stroke, StrokeId, StrokeKind};
use std::sync::Arc;

pub const BRICKS_PER_BATCH: usize = 96;
#[derive(Debug, Clone, PartialEq)]
pub struct WallInput {
    pub wall: Arc<Stroke>,
    pub paths: Vec<Arc<Stroke>>,
}
pub fn wall_inputs(strokes: &[Arc<Stroke>]) -> Vec<WallInput> {
    strokes
        .iter()
        .filter(|s| s.kind == StrokeKind::Wall)
        .map(|wall| WallInput {
            wall: wall.clone(),
            paths: strokes
                .iter()
                .filter(|p| p.kind == StrokeKind::Path && p.affects(wall))
                .cloned()
                .collect(),
        })
        .collect()
}
pub fn distance_to_segment(p: Point, a: Point, b: Point) -> f64 {
    let dx = b.x - a.x;
    let dz = b.z - a.z;
    let denom = dx * dx + dz * dz;
    if denom < 1e-12 {
        return p.distance(a);
    }
    let t = (((p.x - a.x) * dx + (p.z - a.z) * dz) / denom).clamp(0., 1.);
    p.distance(Point {
        x: a.x + t * dx,
        z: a.z + t * dz,
    })
}
pub fn path_field(p: Point, paths: &[Arc<Stroke>]) -> f64 {
    paths
        .iter()
        .map(|s| {
            let d = s
                .points
                .windows(2)
                .map(|w| distance_to_segment(p, w[0], w[1]))
                .fold(f64::INFINITY, f64::min);
            (s.width * 0.5 - d).max(0.)
        })
        .fold(0., f64::max)
}
// Only segments crossing the local wall line may open it. A parallel road
// touching the wall remains a surface, not an accidental door.
fn crossing_field(p: Point, dir: [f64; 3], paths: &[Arc<Stroke>]) -> f64 {
    paths
        .iter()
        .flat_map(|path| {
            path.points.windows(2).filter_map(move |w| {
                let dx = w[1].x - w[0].x;
                let dz = w[1].z - w[0].z;
                let length = dx.hypot(dz);
                let side = |q: Point| -(q.x - p.x) * dir[2] + (q.z - p.z) * dir[0];
                (length > 1e-6
                    && (dx * dir[0] + dz * dir[2]).abs() / length < 0.85
                    && side(w[0]) * side(w[1]) <= 1e-6)
                    .then(|| (path.width * 0.5 - distance_to_segment(p, w[0], w[1])).max(0.))
            })
        })
        .fold(0., f64::max)
}
pub struct Curve {
    pub points: Vec<Point>,
    cumulative: Vec<f64>,
    pub length: f64,
}
impl Curve {
    pub fn new(points: &[Point]) -> Self {
        let mut cumulative = vec![0.];
        for p in points.windows(2) {
            cumulative.push(cumulative.last().unwrap() + p[0].distance(p[1]));
        }
        Self {
            points: points.to_vec(),
            length: *cumulative.last().unwrap(),
            cumulative,
        }
    }
    pub fn sample(&self, s: f64) -> (Point, [f64; 3]) {
        let s = s.clamp(0., self.length);
        let i = self
            .cumulative
            .partition_point(|x| *x <= s)
            .saturating_sub(1)
            .min(self.points.len() - 2);
        let a = self.points[i];
        let b = self.points[i + 1];
        let length = b.distance(a);
        let t = (s - self.cumulative[i]) / length;
        (
            Point {
                x: a.x + (b.x - a.x) * t,
                z: a.z + (b.z - a.z) * t,
            },
            [(b.x - a.x) / length, 0., (b.z - a.z) / length],
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Opening {
    pub start: f64,
    pub end: f64,
    pub rise: f64,
    pub sources: Vec<StrokeId>,
}
pub fn openings(input: &WallInput) -> Vec<Opening> {
    let curve = Curve::new(&input.wall.points);
    let n = (curve.length / 0.08).ceil() as usize;
    let step = curve.length / n as f64;
    let mut ranges = Vec::new();
    let mut start = None;
    for i in 0..=n {
        let (p, dir) = curve.sample(i as f64 * step);
        let occupied = crossing_field(p, dir, &input.paths) > 0.;
        if occupied && start.is_none() {
            start = Some(i);
        }
        if (!occupied || i == n)
            && let Some(begin) = start.take()
        {
            let from = ((begin as f64 - 0.5) * step - 0.1).max(0.);
            let to = ((i as f64 + 0.5) * step + 0.1).min(curve.length);
            let sources = input
                .paths
                .iter()
                .filter(|p| {
                    (begin..=i).any(|j| {
                        let (point, dir) = curve.sample(j as f64 * step);
                        crossing_field(point, dir, std::slice::from_ref(p)) > 0.
                    })
                })
                .map(|p| p.id)
                .collect();
            if to - from > 4. || from < 0.15 || to > curve.length - 0.15 {
                continue;
            }
            ranges.push(Opening {
                start: from,
                end: to,
                rise: (input.wall.height * 0.7).min(2.2),
                sources,
            });
        }
    }
    let mut merged: Vec<Opening> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut()
            && range.start <= last.end
        {
            last.end = last.end.max(range.end);
            last.sources.extend(range.sources);
            last.sources.sort();
            last.sources.dedup();
        } else {
            merged.push(range);
        }
    }
    merged.retain(|o| o.end - o.start <= 4.);
    merged
}
fn arch_height(s: f64, openings: &[Opening]) -> f64 {
    openings
        .iter()
        .filter(|o| s >= o.start && s <= o.end)
        .map(|o| {
            let t = (2. * (s - o.start) / (o.end - o.start) - 1.).clamp(-1., 1.);
            o.rise * (1. - t * t).max(0.).sqrt()
        })
        .fold(0., f64::max)
}
pub(crate) fn quad(mesh: &mut MeshData, p: [[f64; 3]; 4], normal: [f64; 3]) {
    let start = mesh.positions.len() as u32;
    mesh.positions.extend(p.map(|v| v.map(|x| x as f32)));
    mesh.normals.extend([normal.map(|x| x as f32); 4]);
    mesh.uvs.extend([[0., 0.], [0., 1.], [1., 1.], [1., 0.]]);
    mesh.indices
        .extend([start, start + 1, start + 2, start, start + 2, start + 3]);
}
pub(crate) fn cuboid(mesh: &mut MeshData, center: [f64; 3], size: [f64; 3], axes: [[f64; 3]; 3]) {
    let point = |q: [f64; 3]| {
        std::array::from_fn(|i| {
            center[i] + (0..3).map(|a| axes[a][i] * size[a] * q[a]).sum::<f64>()
        })
    };
    // Counter-clockwise faces, one unit normal per face.
    for (axis, sign, coords) in [
        (
            0,
            1.,
            [
                [0.5, -0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
            ],
        ),
        (
            0,
            -1.,
            [
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
                [-0.5, -0.5, -0.5],
            ],
        ),
        (
            1,
            1.,
            [
                [-0.5, 0.5, -0.5],
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
            ],
        ),
        (
            1,
            -1.,
            [
                [-0.5, -0.5, 0.5],
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
            ],
        ),
        (
            2,
            1.,
            [
                [-0.5, -0.5, 0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
            ],
        ),
        (
            2,
            -1.,
            [
                [0.5, -0.5, -0.5],
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
            ],
        ),
    ] {
        quad(mesh, coords.map(point), axes[axis].map(|n| n * sign));
    }
}
fn batch(out: &mut BuildingMesh, data: &mut MeshData, material: MaterialKey) {
    if !data.positions.is_empty() {
        out.batches.push(MeshBatch {
            material,
            data: std::mem::take(data),
        });
    }
}
pub fn compile_wall(input: &WallInput, cancel: &Cancellation) -> Result<BuildingMesh, MeshError> {
    compile_wall_resolved(input, &openings(input), cancel)
}
/// 正式和候选消费已解析的开口区间；几何编译不能另外运行一套穿越规则。
pub fn compile_wall_resolved(
    input: &WallInput,
    arches: &[Opening],
    cancel: &Cancellation,
) -> Result<BuildingMesh, MeshError> {
    input
        .wall
        .validate()
        .map_err(|_| MeshError("invalid stroke"))?;
    let curve = Curve::new(&input.wall.points);
    let rows = (input.wall.height / 0.26).ceil() as usize;
    let columns = (curve.length / 0.48).ceil() as usize;
    let dx = curve.length / columns as f64;
    let dy = input.wall.height / rows as f64;
    let mut out = BuildingMesh::default();
    let mut data = MeshData::default();
    let mut count = 0;
    for row in 0..rows {
        cancel.check()?;
        // Half-brick stagger, clipped at ends. Stable variation uses semantic row/column.
        let stagger = if row % 2 == 1 { dx * 0.5 } else { 0. };
        for col in 0..=columns {
            let from = (col as f64 * dx - stagger).max(0.);
            let to = ((col + 1) as f64 * dx - stagger).min(curve.length);
            if to - from < 0.04 {
                continue;
            }
            // Conservative trim at the highest sampled opening avoids bricks
            // sticking into the arch; stepped reveals are intentional brickwork.
            let cut = (0..=4)
                .map(|i| arch_height(from + (to - from) * i as f64 / 4., arches))
                .fold(0., f64::max);
            let bottom = (row as f64 * dy + 0.012).max(cut + 0.035);
            let top = (row + 1) as f64 * dy - 0.012;
            if top - bottom < 0.035 {
                continue;
            }
            let (p, dir) = curve.sample((from + to) * 0.5);
            let noise = ((input.wall.id.0.wrapping_mul(73856093)
                ^ (row as u64 * 19349663)
                ^ (col as u64 * 83492791))
                % 101) as f64
                / 100.;
            cuboid(
                &mut data,
                [p.x, (top + bottom) * 0.5, p.z],
                [
                    to - from - 0.015,
                    top - bottom,
                    input.wall.width * (0.95 + noise * 0.1),
                ],
                [dir, [0., 1., 0.], [-dir[2], 0., dir[0]]],
            );
            count += 1;
            if count % BRICKS_PER_BATCH == 0 {
                batch(&mut out, &mut data, MaterialKey::Stone);
            }
        }
    }
    for opening in arches {
        cancel.check()?;
        let steps = 24;
        let mut last = None;
        for i in 0..=steps {
            let angle = std::f64::consts::PI * i as f64 / steps as f64;
            let s = (opening.start + opening.end) * 0.5
                - (opening.end - opening.start) * 0.5 * angle.cos();
            let (p, _) = curve.sample(s);
            let pos = [p.x, opening.rise * angle.sin() + 0.08, p.z];
            if let Some(a) = last {
                let a: [f64; 3] = a;
                let delta = std::array::from_fn::<_, 3, _>(|j| pos[j] - a[j]);
                let length = delta.iter().map(|v| v * v).sum::<f64>().sqrt();
                if length > 0.02 {
                    let x = delta.map(|v| v / length);
                    let horizontal = x[0].hypot(x[2]);
                    if horizontal < 1e-6 {
                        continue;
                    }
                    let z = [-x[2] / horizontal, 0., x[0] / horizontal];
                    let y = [
                        -x[1] * x[0] / horizontal,
                        horizontal,
                        -x[1] * x[2] / horizontal,
                    ];
                    cuboid(
                        &mut data,
                        std::array::from_fn(|j| (a[j] + pos[j]) * 0.5),
                        [length * 0.97, 0.14, input.wall.width + 0.06],
                        [x, y, z],
                    );
                    count += 1;
                    if count % BRICKS_PER_BATCH == 0 {
                        batch(&mut out, &mut data, MaterialKey::Stone);
                    }
                }
            }
            last = Some(pos);
        }
    }
    batch(&mut out, &mut data, MaterialKey::Stone);
    for b in &out.batches {
        b.data.validate()?;
    }
    Ok(out)
}
/// Union mask tiles: overlapping strokes share one surface cell, not z-fighting
/// ribbons. This is a field union, not a semantic road-network graph.
pub fn compile_paths(
    paths: &[Arc<Stroke>],
    cancel: &Cancellation,
) -> Result<BuildingMesh, MeshError> {
    for path in paths {
        path.validate().map_err(|_| MeshError("invalid stroke"))?;
    }
    let mut out = BuildingMesh::default();
    let mut data = MeshData::default();
    let step = 0.25;
    if paths.is_empty() {
        return Ok(out);
    }
    for x in -100..100 {
        cancel.check()?;
        for z in -62..62 {
            let p = Point {
                x: (x as f64 + 0.5) * step,
                z: (z as f64 + 0.5) * step,
            };
            if path_field(p, paths) > 0. {
                let x = x as f64 * step;
                let z = z as f64 * step;
                quad(
                    &mut data,
                    [
                        [x, 0.015, z],
                        [x, 0.015, z + step],
                        [x + step, 0.015, z + step],
                        [x + step, 0.015, z],
                    ],
                    [0., 1., 0.],
                );
                if data.positions.len() >= 384 {
                    batch(&mut out, &mut data, MaterialKey::Plaster);
                }
            }
        }
    }
    batch(&mut out, &mut data, MaterialKey::Plaster);
    for b in &out.batches {
        b.data.validate()?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn line(id: u64, kind: StrokeKind, a: Point, b: Point) -> Arc<Stroke> {
        Arc::new(Stroke {
            id: StrokeId(id),
            kind,
            points: vec![a, b],
            width: 1.2,
            height: 2.4,
        })
    }
    #[test]
    fn arc_length_sampling_and_path_union_are_deterministic() {
        let c = Curve::new(&[
            Point { x: 0., z: 0. },
            Point { x: 3., z: 0. },
            Point { x: 3., z: 4. },
        ]);
        assert_eq!(c.length, 7.);
        assert_eq!(c.sample(5.).0, Point { x: 3., z: 2. });
        let path = line(
            2,
            StrokeKind::Path,
            Point { x: 2., z: -2. },
            Point { x: 2., z: 2. },
        );
        let a = compile_paths(std::slice::from_ref(&path), &Cancellation::default()).unwrap();
        let b = compile_paths(&[path.clone(), path], &Cancellation::default()).unwrap();
        assert_eq!(a.vertices(), b.vertices());
        assert_eq!(a.batches[0].data.positions, b.batches[0].data.positions);
    }
    #[test]
    fn crossing_paths_construct_real_holes_and_unrelated_paths_do_not_invalidate() {
        let wall = line(
            1,
            StrokeKind::Wall,
            Point { x: 0., z: 0. },
            Point { x: 6., z: 0. },
        );
        let path = line(
            2,
            StrokeKind::Path,
            Point { x: 3., z: -3. },
            Point { x: 3., z: 3. },
        );
        let remote = line(
            3,
            StrokeKind::Path,
            Point { x: 12., z: 8. },
            Point { x: 14., z: 8. },
        );
        let input = wall_inputs(&[wall.clone(), path.clone(), remote]);
        assert_eq!(input[0].paths.len(), 1);
        let arches = openings(&input[0]);
        assert_eq!(arches.len(), 1);
        assert_eq!(arches[0].sources, vec![path.id]);
        let mesh = compile_wall(&input[0], &Cancellation::default()).unwrap();
        assert!(mesh.vertices() > 0);
        for b in &mesh.batches {
            b.data.validate().unwrap();
            assert!(b.data.positions.len() <= BRICKS_PER_BATCH * 24);
        }
        assert!(
            openings(&WallInput {
                wall,
                paths: vec![]
            })
            .is_empty()
        );
        let cancel = Cancellation::default();
        cancel.cancel();
        assert!(compile_wall(&input[0], &cancel).is_err());
    }
    #[test]
    fn parallel_roads_and_crossings_at_wall_ends_do_not_form_arches() {
        let wall = line(
            1,
            StrokeKind::Wall,
            Point { x: 0., z: 0. },
            Point { x: 6., z: 0. },
        );
        for path in [
            line(
                2,
                StrokeKind::Path,
                Point { x: 2., z: 0.2 },
                Point { x: 4., z: 0.2 },
            ),
            line(
                3,
                StrokeKind::Path,
                Point { x: 0., z: -2. },
                Point { x: 0., z: 2. },
            ),
        ] {
            assert!(
                openings(&WallInput {
                    wall: wall.clone(),
                    paths: vec![path]
                })
                .is_empty()
            );
        }
    }
}
