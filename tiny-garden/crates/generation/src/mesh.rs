//! Pure, meter-based mesh compilation. No Bevy types or GPU resources.
use crate::structure::{RoofInput, RoofRule, StructureRule};
use crate::{BlockLayout, BuildingLayout, Face};
use garden_domain::{Facade, Roof};
use garden_geometry::Rect;
use std::{collections::BTreeMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaterialKey {
    Stone,
    Plaster,
    Timber,
    Roof,
    Window,
}
impl MaterialKey {
    pub const ALL: [Self; 5] = [
        Self::Stone,
        Self::Plaster,
        Self::Timber,
        Self::Roof,
        Self::Window,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Stone => "stone",
            Self::Plaster => "plaster",
            Self::Timber => "timber",
            Self::Roof => "roof",
            Self::Window => "window",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometryProfile {
    pub wall_thickness: f64,
    pub frame_width: f64,
    pub eave: f64,
    pub roof_pitch_degrees: f64,
    pub max_roof_rise: f64,
    pub uv_meters_per_tile: f64,
}
impl Default for GeometryProfile {
    fn default() -> Self {
        Self {
            wall_thickness: 0.22,
            frame_width: 0.09,
            eave: 0.28,
            roof_pitch_degrees: 35.0,
            max_roof_rise: 3.0,
            uv_meters_per_tile: 1.0,
        }
    }
}
impl GeometryProfile {
    pub fn validate(self) -> Result<(), MeshError> {
        for (value, min, max) in [
            (self.wall_thickness, 0.05, 0.3),
            (self.frame_width, 0.02, 0.12),
            (self.eave, 0.0, 0.45),
            (self.roof_pitch_degrees, 15.0, 45.0),
            (self.max_roof_rise, 0.1, 4.0),
            (self.uv_meters_per_tile, 0.1, 10.0),
        ] {
            if !value.is_finite() || !(min..=max).contains(&value) {
                return Err(MeshError("invalid geometry profile"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshError(pub &'static str);
impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for MeshError {}

#[derive(Debug, Clone, Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}
impl MeshData {
    pub fn triangles(&self) -> usize {
        self.indices.len() / 3
    }
    pub fn validate(&self) -> Result<(), MeshError> {
        let n = self.positions.len();
        if n != self.normals.len()
            || n != self.uvs.len()
            || !self.indices.len().is_multiple_of(3)
            || self.indices.iter().any(|&i| i as usize >= n)
            || self
                .positions
                .iter()
                .flatten()
                .chain(self.normals.iter().flatten())
                .chain(self.uvs.iter().flatten())
                .any(|v| !v.is_finite())
        {
            return Err(MeshError("invalid vertex buffers"));
        }
        for normal in &self.normals {
            let length = normal.iter().map(|v| v * v).sum::<f32>();
            if (length - 1.0).abs() > 0.001 {
                return Err(MeshError("non-unit normal"));
            }
        }
        for triangle in self.indices.chunks_exact(3) {
            let a = self.positions[triangle[0] as usize].map(f64::from);
            let b = self.positions[triangle[1] as usize].map(f64::from);
            let c = self.positions[triangle[2] as usize].map(f64::from);
            let normal = self.normals[triangle[0] as usize].map(f64::from);
            if dot(cross(sub(b, a), sub(c, a)), normal) <= 1e-10 {
                return Err(MeshError("degenerate/reversed triangle"));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MeshBatch {
    pub material: MaterialKey,
    pub data: MeshData,
}
#[derive(Debug, Clone, Default)]
pub struct BuildingMesh {
    pub batches: Vec<MeshBatch>,
}
impl BuildingMesh {
    pub fn triangles(&self) -> usize {
        self.batches.iter().map(|b| b.data.triangles()).sum()
    }
    pub fn vertices(&self) -> usize {
        self.batches.iter().map(|b| b.data.positions.len()).sum()
    }
}

type Point = [f64; 3];
fn sub(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: Point, b: Point) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

struct Builder {
    batches: BTreeMap<MaterialKey, MeshData>,
    tile: f64,
}
impl Builder {
    fn polygon(&mut self, material: MaterialKey, points: &[Point], toward: Point) {
        let raw = cross(sub(points[1], points[0]), sub(points[2], points[0]));
        let length = dot(raw, raw).sqrt();
        let sign = if dot(raw, toward) < 0.0 { -1.0 } else { 1.0 };
        let normal = raw.map(|v| (v / length * sign) as f32);
        // Planar orthonormal UVs, in meters. No resize-dependent normalized UVs.
        let u = sub(points[1], points[0]);
        let ul = dot(u, u).sqrt();
        let u = u.map(|v| v / ul);
        let v = cross(normal.map(f64::from), u);
        let mesh = self.batches.entry(material).or_default();
        let start = mesh.positions.len() as u32;
        for &point in points {
            mesh.positions.push(point.map(|v| v as f32));
            mesh.normals.push(normal);
            mesh.uvs.push([
                (dot(point, u) / self.tile) as f32,
                (dot(point, v) / self.tile) as f32,
            ]);
        }
        for i in 1..points.len() - 1 {
            let (b, c) = if sign > 0.0 { (i, i + 1) } else { (i + 1, i) };
            mesh.indices
                .extend([start, start + b as u32, start + c as u32]);
        }
    }
    fn cuboid(&mut self, material: MaterialKey, min: Point, max: Point) {
        let [x, y, z] = min;
        let [a, b, c] = max;
        for (points, normal) in [
            (
                [[x, y, z], [a, y, z], [a, b, z], [x, b, z]],
                [0.0, 0.0, -1.0],
            ),
            (
                [[x, y, c], [a, y, c], [a, b, c], [x, b, c]],
                [0.0, 0.0, 1.0],
            ),
            (
                [[x, y, z], [x, y, c], [x, b, c], [x, b, z]],
                [-1.0, 0.0, 0.0],
            ),
            (
                [[a, y, z], [a, y, c], [a, b, c], [a, b, z]],
                [1.0, 0.0, 0.0],
            ),
            (
                [[x, b, z], [a, b, z], [a, b, c], [x, b, c]],
                [0.0, 1.0, 0.0],
            ),
            (
                [[x, y, z], [a, y, z], [a, y, c], [x, y, c]],
                [0.0, -1.0, 0.0],
            ),
        ] {
            self.polygon(material, &points, normal);
        }
    }
}

/// Sweep horizontal bands through openings; complexity depends on a wall's
/// openings/bands, never the whole scene. Supports interval unions as well.
pub fn wall_panels(width: f64, height: f64, openings: &[Rect]) -> Result<Vec<Rect>, MeshError> {
    let wall = Rect {
        x: 0.0,
        z: 0.0,
        width,
        depth: height,
    };
    if !wall.valid()
        || openings
            .iter()
            .any(|r| !r.valid() || !wall.contains(*r, 0.0))
    {
        return Err(MeshError("opening outside wall"));
    }
    let mut cuts = vec![0.0, height];
    for r in openings {
        cuts.extend([r.z, r.back()]);
    }
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    let mut panels = Vec::new();
    for band in cuts.windows(2) {
        let (bottom, top) = (band[0], band[1]);
        let mut occupied = openings
            .iter()
            .filter(|r| r.z < top && r.back() > bottom)
            .map(|r| (r.x, r.right()))
            .collect::<Vec<_>>();
        occupied.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut cursor = 0.0;
        for (left, right) in occupied {
            if left > cursor {
                panels.push(Rect {
                    x: cursor,
                    z: bottom,
                    width: left - cursor,
                    depth: top - bottom,
                });
            }
            cursor = cursor.max(right);
        }
        if cursor < width {
            panels.push(Rect {
                x: cursor,
                z: bottom,
                width: width - cursor,
                depth: top - bottom,
            });
        }
    }
    Ok(panels)
}

fn outward(face: Face) -> Point {
    match face {
        Face::Front => [0.0, 0.0, -1.0],
        Face::Back => [0.0, 0.0, 1.0],
        Face::Left => [-1.0, 0.0, 0.0],
        Face::Right => [1.0, 0.0, 0.0],
    }
}
fn along(face: Face) -> Point {
    match face {
        Face::Front => [1.0, 0.0, 0.0],
        Face::Back => [-1.0, 0.0, 0.0],
        Face::Left => [0.0, 0.0, -1.0],
        Face::Right => [0.0, 0.0, 1.0],
    }
}
fn wall_point(block: &BlockLayout, face: Face, a: f64, y: f64, inward: f64) -> Point {
    let r = block.footprint;
    match face {
        Face::Front => [r.x + a, y, r.z + inward],
        Face::Back => [r.right() - a, y, r.back() - inward],
        Face::Left => [r.x + inward, y, r.back() - a],
        Face::Right => [r.right() - inward, y, r.z + a],
    }
}
fn wall_quad(
    b: &mut Builder,
    material: MaterialKey,
    block: &BlockLayout,
    face: Face,
    r: Rect,
    depth: f64,
    reverse: bool,
) {
    let normal = outward(face).map(|v| if reverse { -v } else { v });
    b.polygon(
        material,
        &[
            wall_point(block, face, r.x, r.z, depth),
            wall_point(block, face, r.right(), r.z, depth),
            wall_point(block, face, r.right(), r.back(), depth),
            wall_point(block, face, r.x, r.back(), depth),
        ],
        normal,
    );
}

fn walls(
    b: &mut Builder,
    block: &BlockLayout,
    p: GeometryProfile,
    frames: bool,
    only_face: Option<Face>,
) -> Result<(), MeshError> {
    let material = match block.facade {
        Facade::Stone => MaterialKey::Stone,
        Facade::Plaster => MaterialKey::Plaster,
        Facade::Timber => MaterialKey::Timber,
    };
    for face in [Face::Front, Face::Back, Face::Left, Face::Right] {
        if only_face.is_some_and(|wanted| wanted != face) {
            continue;
        }
        let length = match face {
            Face::Front | Face::Back => block.footprint.width,
            _ => block.footprint.depth,
        };
        let windows = block
            .windows
            .iter()
            .filter(|w| w.key.face == face)
            .collect::<Vec<_>>();
        let openings = windows
            .iter()
            .map(|w| Rect {
                x: w.along_wall - w.width / 2.0,
                z: w.elevation - block.base_elevation - w.height / 2.0,
                width: w.width,
                depth: w.height,
            })
            .collect::<Vec<_>>();
        for mut panel in wall_panels(length, block.height, &openings)? {
            panel.z += block.base_elevation;
            wall_quad(b, material, block, face, panel, 0.0, false);
            wall_quad(b, material, block, face, panel, p.wall_thickness, true);
        }
        for (window, opening) in windows.into_iter().zip(openings) {
            let r = Rect {
                z: opening.z + block.base_elevation,
                ..opening
            };
            // Four actual reveals: wall aperture is not a dark decal on solid wall.
            for (a0, y0, a1, y1, normal) in [
                (r.x, r.z, r.x, r.back(), along(face)),
                (r.right(), r.z, r.right(), r.back(), along(face).map(|v| -v)),
                (r.x, r.z, r.right(), r.z, [0.0, 1.0, 0.0]),
                (r.x, r.back(), r.right(), r.back(), [0.0, -1.0, 0.0]),
            ] {
                b.polygon(
                    material,
                    &[
                        wall_point(block, face, a0, y0, 0.0),
                        wall_point(block, face, a1, y1, 0.0),
                        wall_point(block, face, a1, y1, p.wall_thickness),
                        wall_point(block, face, a0, y0, p.wall_thickness),
                    ],
                    normal,
                );
            }
            if !frames {
                continue;
            }
            wall_quad(
                b,
                MaterialKey::Window,
                block,
                face,
                r,
                p.wall_thickness + 0.015,
                false,
            );
            let f = p.frame_width.min(window.width * 0.2);
            for strip in [
                Rect {
                    x: r.x - f,
                    z: r.z - f,
                    width: f,
                    depth: r.depth + 2.0 * f,
                },
                Rect {
                    x: r.right(),
                    z: r.z - f,
                    width: f,
                    depth: r.depth + 2.0 * f,
                },
                Rect {
                    x: r.x,
                    z: r.z - f,
                    width: r.width,
                    depth: f,
                },
                Rect {
                    x: r.x,
                    z: r.back(),
                    width: r.width,
                    depth: f,
                },
                Rect {
                    x: r.x + r.width / 2.0 - f / 4.0,
                    z: r.z,
                    width: f / 2.0,
                    depth: r.depth,
                },
            ] {
                wall_quad(b, MaterialKey::Timber, block, face, strip, -0.045, false);
            }
        }
        for story in 1..block.stories {
            let y =
                block.base_elevation + f64::from(story) * block.height / f64::from(block.stories);
            wall_quad(
                b,
                MaterialKey::Timber,
                block,
                face,
                Rect {
                    x: 0.0,
                    z: y - 0.06,
                    width: length,
                    depth: 0.12,
                },
                -0.025,
                false,
            );
        }
    }
    Ok(())
}

fn roof(b: &mut Builder, block: &BlockLayout, p: GeometryProfile) {
    let r = block.footprint;
    let y = block.base_elevation + block.height;
    if block.effective_roof == Roof::Flat {
        // Slab under supported blocks is occluded; exposed differences are terraces.
        b.cuboid(
            MaterialKey::Stone,
            [r.x, y - 0.06, r.z],
            [r.right(), y, r.back()],
        );
        let t = 0.1;
        for (min, max) in [
            ([r.x, y, r.z], [r.right(), y + 0.3, r.z + t]),
            ([r.x, y, r.back() - t], [r.right(), y + 0.3, r.back()]),
            ([r.x, y, r.z + t], [r.x + t, y + 0.3, r.back() - t]),
            (
                [r.right() - t, y, r.z + t],
                [r.right(), y + 0.3, r.back() - t],
            ),
        ] {
            b.cuboid(MaterialKey::Stone, min, max);
        }
        return;
    }
    let structure = RoofRule.resolve(&RoofInput {
        footprint: r,
        kind: block.effective_roof,
        profile: p,
    });
    let along_z = structure.along_z;
    let [u0, u1] = structure.u;
    let [v0, v1] = structure.v;
    let half = structure.width() / 2.;
    let center = (u0 + u1) / 2.;
    let rise = structure.rise;
    let to_world = |u, h, v| structure.point(u, h, v);
    let a = to_world(u0, y, v0);
    let c = to_world(u1, y, v1);
    let d = to_world(u0, y, v1);
    let q = to_world(u1, y, v0);
    let [start, end] = structure.ridge;
    let e = to_world(center, y + rise, start);
    let f = to_world(center, y + rise, end);
    if (end - start).abs() < 1e-9 {
        b.polygon(MaterialKey::Roof, &[a, d, e], [0.0, 1.0, 0.0]);
        b.polygon(MaterialKey::Roof, &[q, e, c], [0.0, 1.0, 0.0]);
    } else {
        b.polygon(MaterialKey::Roof, &[a, d, f, e], [0.0, 1.0, 0.0]);
        b.polygon(MaterialKey::Roof, &[q, e, f, c], [0.0, 1.0, 0.0]);
    }
    if block.effective_roof == Roof::Hipped {
        b.polygon(MaterialKey::Roof, &[a, e, q], [0.0, 1.0, 0.0]);
        b.polygon(MaterialKey::Roof, &[d, c, f], [0.0, 1.0, 0.0]);
    } else {
        let material = match block.facade {
            Facade::Stone => MaterialKey::Stone,
            Facade::Plaster => MaterialKey::Plaster,
            Facade::Timber => MaterialKey::Timber,
        };
        let edge_y = y + rise * p.eave / half;
        for (v, normal) in [
            (
                v0 + p.eave,
                if along_z {
                    [0.0, 0.0, -1.0]
                } else {
                    [-1.0, 0.0, 0.0]
                },
            ),
            (
                v1 - p.eave,
                if along_z {
                    [0.0, 0.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0]
                },
            ),
        ] {
            if p.eave > 0.0 {
                b.polygon(
                    material,
                    &[
                        to_world(u0 + p.eave, y, v),
                        to_world(u1 - p.eave, y, v),
                        to_world(u1 - p.eave, edge_y, v),
                        to_world(u0 + p.eave, edge_y, v),
                    ],
                    normal,
                );
            }
            b.polygon(
                material,
                &[
                    to_world(u0 + p.eave, edge_y, v),
                    to_world(center, y + rise, v),
                    to_world(u1 - p.eave, edge_y, v),
                ],
                normal,
            );
        }
    }
    b.cuboid(
        MaterialKey::Timber,
        [r.x - p.eave, y - 0.12, r.z - p.eave],
        [r.right() + p.eave, y, r.back() + p.eave],
    );
}

pub fn compile_mesh(
    layout: &BuildingLayout,
    profile: GeometryProfile,
) -> Result<BuildingMesh, MeshError> {
    compile_mesh_with_frames(layout, profile, true)
}

/// External art kits supply their own frames and recessed infill. Apertures and
/// reveals still belong to the structural compiler, avoiding double geometry.
pub fn compile_mesh_with_frames(
    layout: &BuildingLayout,
    profile: GeometryProfile,
    frames: bool,
) -> Result<BuildingMesh, MeshError> {
    profile.validate()?;
    if layout.blocks.is_empty() || layout.blocks.len() > 3 {
        return Err(MeshError("invalid block count"));
    }
    let placement = layout.placement;
    if [placement.x, placement.z, placement.elevation, placement.yaw]
        .into_iter()
        .any(|v| !v.is_finite() || v.abs() > 10_000.0)
    {
        return Err(MeshError("placement outside presentation range"));
    }
    let mut b = Builder {
        batches: BTreeMap::new(),
        tile: profile.uv_meters_per_tile,
    };
    for block in &layout.blocks {
        let r = block.footprint;
        if !r.valid()
            || r.width > 100.0
            || r.depth > 100.0
            || r.width < 1.2
            || r.depth < 1.2
            || [r.x, r.z, block.height, block.base_elevation]
                .into_iter()
                .any(|v| !v.is_finite() || v.abs() > 1000.0)
            || !(1..=4).contains(&block.stories)
            || !(2.2..=16.0).contains(&block.height)
            || block.windows.len() > 1_000
        {
            return Err(MeshError("invalid block layout"));
        }
        for window in &block.windows {
            if window.key.block != block.block
                || window.key.building != layout.building
                || [
                    window.along_wall,
                    window.elevation,
                    window.width,
                    window.height,
                ]
                .into_iter()
                .any(|v| !v.is_finite())
            {
                return Err(MeshError("invalid window layout"));
            }
        }
        walls(&mut b, block, profile, frames, None)?;
        roof(&mut b, block, profile);
    }
    let batches = b
        .batches
        .into_iter()
        .map(|(material, data)| MeshBatch { material, data })
        .collect::<Vec<_>>();
    for batch in &batches {
        batch.data.validate()?;
    }
    Ok(BuildingMesh { batches })
}

/// Local-coordinate structural chunk. Inputs are resolved and validated by the
/// part planner; roof and wall chunks deliberately have separate dependencies.
pub fn compile_structure_part(
    block: &BlockLayout,
    profile: GeometryProfile,
    face: Option<Face>,
    frames: bool,
) -> Result<BuildingMesh, MeshError> {
    profile.validate()?;
    let mut b = Builder {
        batches: BTreeMap::new(),
        tile: profile.uv_meters_per_tile,
    };
    if let Some(face) = face {
        walls(&mut b, block, profile, frames, Some(face))?;
    } else {
        roof(&mut b, block, profile);
    }
    let batches = b
        .batches
        .into_iter()
        .map(|(material, data)| MeshBatch { material, data })
        .collect::<Vec<_>>();
    for batch in &batches {
        batch.data.validate()?;
    }
    Ok(BuildingMesh { batches })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile;
    use garden_domain::{
        BlockDraft, BlockId, Building, BuildingEdit, BuildingId, Stories, sample_building,
    };
    fn layout(w: f64, h: f64, roof: Roof) -> BuildingLayout {
        let mut draft = sample_building(BuildingId::new(1).unwrap(), w, h);
        draft.blocks[0].roof_intent = roof;
        compile(&Building::try_new(draft).unwrap())
    }
    #[test]
    fn all_roofs_and_size_extremes_have_finite_correctly_wound_geometry() {
        for roof in [Roof::Gabled, Roof::Hipped, Roof::Flat] {
            for (w, h) in [
                (1.2, 3.0),
                (3.0, 3.0),
                (6.0, 6.0),
                (10.0, 12.0),
                (100.0, 12.0),
            ] {
                let mesh = compile_mesh(&layout(w, h, roof), GeometryProfile::default()).unwrap();
                assert!(mesh.triangles() > 0 && mesh.batches.len() <= 5);
                for batch in mesh.batches {
                    batch.data.validate().unwrap();
                }
            }
        }
        let mut square = layout(6.0, 3.0, Roof::Hipped);
        square.blocks[0].footprint.depth = 6.0;
        compile_mesh(&square, GeometryProfile::default()).unwrap();
    }
    #[test]
    fn wall_sweep_removes_opening_area_and_handles_overlaps() {
        let holes = [
            Rect {
                x: 1.0,
                z: 1.0,
                width: 1.0,
                depth: 1.0,
            },
            Rect {
                x: 3.0,
                z: 1.0,
                width: 1.0,
                depth: 1.0,
            },
        ];
        let panels = wall_panels(6.0, 3.0, &holes).unwrap();
        assert_eq!(panels.iter().map(|r| r.area()).sum::<f64>(), 16.0);
        for (i, a) in panels.iter().enumerate() {
            assert!(holes.iter().all(|h| !a.intersects(*h)));
            assert!(panels[i + 1..].iter().all(|b| !a.intersects(*b)));
        }
        let overlapping = [holes[0], Rect { x: 1.5, ..holes[0] }];
        assert_eq!(
            wall_panels(6.0, 3.0, &overlapping)
                .unwrap()
                .iter()
                .map(|r| r.area())
                .sum::<f64>(),
            16.5
        );
    }
    #[test]
    fn meter_uvs_do_not_normalize_whole_wall_to_one_tile() {
        let mut layout = layout(10.0, 3.0, Roof::Flat);
        layout.blocks[0].windows.clear();
        let mesh = compile_mesh(&layout, GeometryProfile::default()).unwrap();
        let wall = mesh
            .batches
            .iter()
            .find(|b| b.material == MaterialKey::Plaster)
            .unwrap();
        assert!(wall.data.uvs.iter().flatten().any(|v| v.abs() >= 10.0));
    }
    #[test]
    fn facade_uv_phase_is_shared_across_all_cut_wall_panels() {
        let mesh =
            compile_mesh(&layout(6.0, 6.0, Roof::Gabled), GeometryProfile::default()).unwrap();
        let wall = mesh
            .batches
            .iter()
            .find(|b| b.material == MaterialKey::Plaster)
            .unwrap();
        for ((point, normal), uv) in wall
            .data
            .positions
            .iter()
            .zip(&wall.data.normals)
            .zip(&wall.data.uvs)
        {
            if *normal == [0.0, 0.0, -1.0] && point[2] == 0.0 && point[1] <= 6.0 {
                assert!((uv[0] - point[0]).abs() < 1e-5);
                assert!((uv[1] + point[1]).abs() < 1e-5);
            }
        }
    }
    #[test]
    fn stacking_has_valid_geometry_and_bounded_material_batches() {
        let base =
            Building::try_new(sample_building(BuildingId::new(1).unwrap(), 10.0, 3.0)).unwrap();
        let stacked = base
            .edited(BuildingEdit::AddBlock(BlockDraft {
                id: BlockId::new(2).unwrap(),
                parent: Some(BlockId::new(1).unwrap()),
                footprint: Rect {
                    x: 1.0,
                    z: 1.0,
                    width: 4.0,
                    depth: 3.0,
                },
                height: 6.0,
                stories: Stories::Locked(2),
                roof_intent: Roof::Hipped,
                facade: Facade::Stone,
            }))
            .unwrap();
        let mesh = compile_mesh(&compile(&stacked), GeometryProfile::default()).unwrap();
        assert!(mesh.batches.len() <= 5);
    }
    #[test]
    fn rejects_bad_layout_profile_and_buffers() {
        let mut bad = layout(6.0, 3.0, Roof::Gabled);
        bad.blocks[0].windows[0].width = -1.0;
        assert!(compile_mesh(&bad, GeometryProfile::default()).is_err());
        assert!(
            compile_mesh(
                &layout(6.0, 3.0, Roof::Flat),
                GeometryProfile {
                    eave: f64::NAN,
                    ..GeometryProfile::default()
                }
            )
            .is_err()
        );
        let bad = MeshData {
            positions: vec![[0.0; 3]],
            indices: vec![1, 1, 1],
            ..MeshData::default()
        };
        assert!(bad.validate().is_err());
    }
}
