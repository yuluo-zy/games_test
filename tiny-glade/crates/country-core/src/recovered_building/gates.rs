//! Original gate CPU consumer. Game mesh, source fields and lifecycle policy
//! are projected separately from Bevy commands and Country asset containers.
//! This module is under recovery; whole-owner differential acceptance is pending.
use crate::recovered_building::{
    building_wall_state::WallId, building_wall_structures::MaterializedWallSegment,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateConstructionError {
    InvalidInput,
    InvalidCurve,
    UnrecoveredSupportConsumer,
}
#[derive(Clone, Copy)]
pub struct GateConstructionInput<'a> {
    pub segment: &'a MaterializedWallSegment,
    pub prepared_positions: &'a [[f32; 3]],
    pub prepared_u: &'a [f32],
    pub observed_public_288: f32,
    /// Actual Raw metadata69 -> Public29a byte; the publisher owns the binding.
    pub observed_public_29a: u8,
    pub wall_id: WallId,
    pub ordinal: u64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GateMeshProjection {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}
impl GateMeshProjection {
    fn add_quad(
        &mut self,
        points: [[f32; 3]; 4],
        normal: [f32; 3],
        uv: [[f32; 2]; 4],
        color: [f32; 3],
    ) {
        for i in [0, 1, 2, 0, 2, 3] {
            self.indices.push(self.positions.len() as u32);
            self.positions.push(points[i]);
            self.normals.push(normal);
            self.uvs.push(uv[i]);
            self.colors.push(color);
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WoodenGateProjection {
    pub mesh: GateMeshProjection,
    pub top_positions: Vec<[f32; 3]>,
    pub endpoint_directions: [[f32; 2]; 2],
    pub wall_id: WallId,
    pub seed: u32,
    pub observed_public_29a: u8,
}
/// True output data needed by the complete consumer; this stage has no input
/// for captured/native quads, mesh buffers or random draws.
#[derive(Clone, Debug, PartialEq)]
pub struct GateShapeProjection {
    pub leaves: Vec<WoodenGateProjection>,
    pub bottom_positions: Vec<[f32; 3]>,
    pub relative_heights: Vec<f32>,
    pub bottom_u: Vec<f32>,
    pub bottom_length: f32,
}
#[derive(Clone)]
pub(crate) struct GateCurve3 {
    pub(crate) positions: Vec<[f32; 3]>,
    pub(crate) points_u: Vec<f32>,
    pub(crate) length: f32,
}
impl GateCurve3 {
    pub(crate) fn new(points: &[[f32; 3]]) -> Result<Self, GateConstructionError> {
        let mut positions = Vec::new();
        for &p in points {
            if positions.last().is_none_or(|q| *q != p) {
                positions.push(p);
            }
        }
        if positions.len() < 2 {
            return Err(GateConstructionError::InvalidCurve);
        }
        let mut distance = 0.;
        let mut points_u = vec![0.];
        for p in positions.windows(2) {
            distance += length3(p[0], p[1]);
            points_u.push(distance);
        }
        if distance <= 0. || !distance.is_finite() {
            return Err(GateConstructionError::InvalidCurve);
        }
        for u in &mut points_u {
            *u /= distance;
        }
        Ok(Self {
            positions,
            points_u,
            length: distance,
        })
    }
    fn sample(&self, heights: &[f32], u: f32) -> ([f32; 3], f32) {
        let (i, t) = coordinate(&self.points_u, u);
        let a = self.positions[i];
        let b = self.positions[i + 1];
        let p = std::array::from_fn(|j| (b[j] - a[j]) * t + a[j]);
        (p, (heights[i + 1] - heights[i]) * t + heights[i])
    }
    /// Shared private-game Curve3 sampler, preserving the observed arithmetic.
    #[allow(dead_code)] // Recovered private sampler retained for future gate consumers.
    pub(crate) fn sample_position(&self, u: f32) -> [f32; 3] {
        let (i, t) = coordinate(&self.points_u, u);
        std::array::from_fn(|j| {
            (self.positions[i + 1][j] - self.positions[i][j]) * t + self.positions[i][j]
        })
    }
}
fn coordinate(us: &[f32], u: f32) -> (usize, f32) {
    let i = us
        .partition_point(|v| *v <= u)
        .saturating_sub(1)
        .min(us.len() - 2);
    (i, (u - us[i]) / (us[i + 1] - us[i]))
}
fn length3(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dy * dy + dx * dx + dz * dz).sqrt()
}
fn normal(points: &[[f32; 3]]) -> Result<[f32; 3], GateConstructionError> {
    let a = points[0];
    let b = points[points.len() - 1];
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let inverse = 1. / (dz * dz + dx * dx).sqrt();
    if !inverse.is_finite() || inverse <= 0. {
        return Err(GateConstructionError::InvalidCurve);
    }
    Ok([-inverse * dz, 0., inverse * dx])
}
fn offset_height(p: [f32; 3], height: f32) -> [f32; 3] {
    [p[0] + height * 0., p[1] + height, p[2] + height * 0.]
}
fn panel(
    mesh: &mut GateMeshProjection,
    a: ([f32; 3], f32),
    b: ([f32; 3], f32),
    normal: [f32; 3],
    us: [f32; 2],
    color: [f32; 3],
) {
    mesh.add_quad(
        [a.0, b.0, offset_height(b.0, b.1), offset_height(a.0, a.1)],
        normal,
        [[us[0], 0.], [us[1], 0.], [us[1], b.1], [us[0], a.1]],
        color,
    );
}
/// Native mesh branch. Support brick rows and Nani consumers remain a distinct
/// unresolved edge, so this result alone does not mean a gate was constructed.
pub fn recover_gate_shape(
    input: GateConstructionInput<'_>,
) -> Result<GateShapeProjection, GateConstructionError> {
    let old = &input.segment.positions;
    let old_heights = &input.segment.heights;
    if old.len() < 2
        || old.len() != old_heights.len()
        || input.segment.points_u.len() != old.len()
        || input.prepared_positions.len() < 2
        || input.prepared_positions.len() != input.prepared_u.len()
        || old
            .iter()
            .flatten()
            .chain(old_heights)
            .chain(input.prepared_u)
            .any(|v| !v.is_finite())
        || !input.observed_public_288.is_finite()
    {
        return Err(GateConstructionError::InvalidInput);
    }
    let original = GateCurve3::new(old)?;
    let mut minimum = f32::MAX;
    let mut maximum = 0f32;
    for (&p, &h) in old.iter().zip(old_heights) {
        minimum = minimum.min(p[1]);
        maximum = maximum.max(p[1] + h);
    }
    let floor = minimum.max(maximum - (input.observed_public_288 + 0.5));
    let a = old[0];
    let b = old[old.len() - 1];
    let bottom: Vec<[f32; 3]> = original
        .points_u
        .iter()
        .map(|&t| {
            [
                t * b[0] + (1. - t) * a[0],
                t * floor + (1. - t) * floor,
                t * b[2] + (1. - t) * a[2],
            ]
        })
        .collect();
    let mut curve = GateCurve3::new(&bottom)?;
    if curve.positions.len() != old.len() {
        return Err(GateConstructionError::InvalidCurve);
    }
    let heights: Vec<f32> = old_heights
        .iter()
        .zip(&curve.positions)
        .zip(old)
        .map(|((&h, p), old)| h - (p[1] - old[1]))
        .collect();
    let unshifted = curve.positions.clone();
    let normal_offset = normal(&curve.positions)?;
    for p in &mut curve.positions {
        for j in 0..3 {
            p[j] += normal_offset[j] * f32::from_bits(0x3e2c0832);
        }
    }
    let n = normal(&curve.positions)?;
    let wide = curve.length > 2.7;
    let gap = (0.05 / curve.length).min(0.1);
    let samples = if wide {
        (old.len() / 2).max(2)
    } else {
        old.len()
    };
    let intervals = if wide {
        vec![(0., 0.5 - gap, true), (0.5 + gap, 1., true)]
    } else {
        vec![(0., 1., false)]
    };
    let color = [
        f32::from_bits(0x3e2e7d57),
        f32::from_bits(0x3de978d5),
        f32::from_bits(0x3d0e8a72),
    ];
    let dark = [
        f32::from_bits(0x3dcb923a),
        f32::from_bits(0x3d883127),
        f32::from_bits(0x3ca64c2f),
    ];
    let top_positions: Vec<_> = curve
        .positions
        .iter()
        .zip(&heights)
        .map(|(&p, &h)| offset_height(p, h))
        .collect();
    let endpoint_directions = std::array::from_fn(|end| {
        let u = input.segment.points_u[if end == 0 { 0 } else { old.len() - 1 }];
        let (i, _) = coordinate(input.prepared_u, u);
        let a = input.prepared_positions[i];
        let b = input.prepared_positions[i + 1];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let dy = b[1] - a[1];
        let inverse = 1. / (dy * dy + dx * dx + dz * dz).sqrt();
        let dx = dx * inverse;
        let dz = dz * inverse;
        let inverse = 1. / (dz * dz + dx * dx).sqrt();
        [inverse * dx, inverse * dz]
    });
    let mut leaves = Vec::new();
    for (ordinal, (start, end, flip)) in intervals.into_iter().enumerate() {
        let mut mesh = GateMeshProjection::default();
        let mut previous = None;
        for j in 0..samples {
            let t = j as f32 / (samples as f32 - 1.);
            let u = t * end + (1. - t) * start;
            let point = curve.sample(&heights, u);
            if let Some((old_u, old_point)) = previous {
                let uv = |x| {
                    if flip && start == 0. {
                        end - x
                    } else if flip {
                        x - start
                    } else {
                        x
                    }
                };
                panel(
                    &mut mesh,
                    old_point,
                    point,
                    n,
                    [uv(old_u) * curve.length, uv(u) * curve.length],
                    color,
                );
            }
            previous = Some((u, point));
        }
        if wide && ordinal == 1 {
            let stops = [0.5 - gap, 0.5, 0.5 + gap];
            let mut previous = None;
            for (j, u) in stops.into_iter().enumerate() {
                let (mut p, h) = curve.sample(&heights, u);
                if j == 1 {
                    for k in 0..3 {
                        p[k] += n[k] * (-0.1);
                    }
                }
                if let Some((previous_u, previous_point)) = previous {
                    panel(
                        &mut mesh,
                        previous_point,
                        (p, h),
                        n,
                        [previous_u * curve.length, u * curve.length],
                        dark,
                    );
                }
                previous = Some((u, (p, h)));
            }
        }
        leaves.push(WoodenGateProjection {
            mesh,
            top_positions: top_positions.clone(),
            endpoint_directions,
            wall_id: input.wall_id,
            seed: input
                .wall_id
                .0
                .wrapping_mul(input.ordinal)
                .wrapping_add(ordinal as u64) as u32,
            observed_public_29a: input.observed_public_29a,
        });
    }
    Ok(GateShapeProjection {
        leaves,
        bottom_positions: unshifted,
        relative_heights: heights,
        bottom_u: curve.points_u,
        bottom_length: curve.length,
    })
}

/// Complete original support-row CPU consumer. The separate header/footer Nani
/// bundle owner is being recovered alongside this output, not represented by a
/// fake mesh or marker here.
pub fn recover_gate_support_bricks(
    input: GateConstructionInput<'_>,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<
    Vec<crate::recovered_building::building_wall_constructor::WallBrickInstance>,
    GateConstructionError,
> {
    use crate::recovered_building::{
        building_wall_production::{PlainWallProductionInput, produce_wall_kind},
        surface::Curve2,
    };
    let shape = recover_gate_shape(input)?;
    let old = &input.segment.positions;
    let minimum = old.iter().fold(f32::MAX, |y, p| y.min(p[1]));
    let highest = old
        .iter()
        .zip(&input.segment.heights)
        .fold(0f32, |y, (p, h)| y.max(p[1] + h));
    let scalar = input.observed_public_288 + 0.5;
    if highest - minimum <= scalar || shape.bottom_positions[0][1] <= 0. {
        return Ok(Vec::new());
    }
    let a = shape.bottom_positions[0];
    let b = shape.bottom_positions[shape.bottom_positions.len() - 1];
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let inv = 1. / (dx * dx + dz * dz).sqrt();
    let direction = [dx * inv, dz * inv];
    let rows = if highest - minimum - scalar >= 2.3 {
        1usize
    } else {
        let rows = ((highest - minimum - scalar) / 0.46).round().max(1.) as usize;
        let rows = rows.min(4);
        let offset = (rows as f32 * 0.56) * 0.8;
        let dz = (-direction[1]) * offset;
        let dx = direction[0] * offset;
        let ground_a = terrain.sample_at_world_xz(a[0] + dz, a[2] + dx);
        let ground_b = terrain.sample_at_world_xz(b[0] + dz, b[2] + dx);
        if (ground_a - ground_b).abs() - scalar >= 2.3 {
            1
        } else {
            rows
        }
    };
    let original = Curve2::try_new(
        shape
            .bottom_positions
            .iter()
            .map(|p| [p[0], p[2]])
            .collect(),
        false,
    )
    .map_err(|_| GateConstructionError::InvalidCurve)?;
    let trim = (0.23 / shape.bottom_length).min(0.1);
    let sliced = slice_gate_curve(&original, trim, 1. - trim)?;
    let offsets = [(direction[1] * 0.56) * 0.8, (direction[0] * 0.56) * 0.8];
    let second = vec![minimum + (-0.92); sliced.points.len()];
    let third = vec![0.; sliced.points.len()];
    let mut result = Vec::new();
    for i in 0..rows {
        let arch_height = i as f32 * (-0.46) + shape.bottom_positions[0][1];
        if arch_height <= 0. {
            continue;
        }
        let mut curve = sliced.clone();
        for p in &mut curve.points {
            p[0] -= i as f32 * offsets[0];
            p[1] += i as f32 * offsets[1];
        }
        let arch = vec![arch_height; curve.points.len()];
        let records = produce_wall_kind(
            PlainWallProductionInput {
                curve: &curve,
                arch_heights: &arch,
                second_heights: &second,
                third_heights: &third,
                holes: &[],
                holes_length: 0.,
                closed: false,
                seed: 0,
                brick_source_id:
                    crate::recovered_building::building_wall_construction_sources::brick_source_id(
                        input.wall_id,
                    ),
                base: 0.,
                row_end_perturbation: 0.2,
                width_padding: None,
                thickness_scale: None,
                wall_float: arch_height,
            },
            0,
        )
        .map_err(|_| GateConstructionError::UnrecoveredSupportConsumer)?;
        for mut r in records {
            // Actual GPUInstance::default followed by affine and seed/source
            // copies at141211560..1412115ee, not constructor brick deformation.
            r.deformation = [0.; 4];
            r.surface = [-100.; 3];
            r.wall_distance = [0.; 2];
            r.flags = 0;
            result.push(r);
        }
    }
    Ok(result)
}
fn slice_gate_curve(
    c: &crate::recovered_building::surface::Curve2,
    a: f32,
    b: f32,
) -> Result<crate::recovered_building::surface::Curve2, GateConstructionError> {
    let (ia, _) = c.coord_at_u(a);
    let (ib, _) = c.coord_at_u(b);
    let start = c.pos_at_u(a);
    let end = c.pos_at_u(b);
    let first = ia + if start == c.points[ia + 1] { 2 } else { 1 };
    let last = if end == c.points[ib] {
        ib.saturating_sub(1)
    } else {
        ib
    };
    let mut points = vec![start];
    if first <= last {
        points.extend_from_slice(&c.points[first..=last]);
    }
    points.push(end);
    crate::recovered_building::surface::Curve2::try_new(points, false)
        .map_err(|_| GateConstructionError::InvalidCurve)
}

#[derive(Clone, Debug, PartialEq)]
pub struct GateNaniProjection {
    pub rotation: [f32; 4],
    pub position: [f32; 3],
    pub scale: [f32; 3],
    /// Compatibility evidence name: Nani mesh descriptor discriminator at
    /// bundle+0x30 (0 = Named). This is neither a Solid attribute nor GPU flags.
    pub observed_word_30: u32,
    pub definition_id: u64,
    pub subset_kind: u8,
}
impl GateNaniProjection {
    /// Meaningful fields written by the real NaniBundle insert owner; omit
    /// padding, Bevy Entity identity and its deferred-command destructor.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for f in self
            .rotation
            .into_iter()
            .chain(self.position)
            .chain(self.scale)
        {
            bytes.extend_from_slice(&f.to_le_bytes());
        }
        bytes.extend_from_slice(&self.observed_word_30.to_le_bytes());
        bytes.extend_from_slice(&self.definition_id.to_le_bytes());
        bytes.push(self.subset_kind);
        bytes
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GateCpuOutput {
    pub bricks: Vec<crate::recovered_building::building_wall_constructor::WallBrickInstance>,
    pub leaves: Vec<WoodenGateProjection>,
    pub nanis: Vec<GateNaniProjection>,
}
fn facing_rotation(x: f32, z: f32) -> [f32; 4] {
    // The private helper's invalid reciprocal fallback is game policy; its
    // normalize/cross/matrix-to-quaternion operations reuse pinned glam.
    let inverse = 1. / (z * z + x * x).sqrt();
    let (x, z) = if inverse.is_finite() && inverse > 0. {
        (x * inverse, z * inverse)
    } else {
        (1., 0.)
    };
    let facing = glam::Vec3::new(x, 0., z);
    let right = glam::Vec3::Y.cross(facing);
    let right = right / right.length();
    glam::Quat::from_mat3(&glam::Mat3::from_cols(right, glam::Vec3::Y, facing)).to_array()
}
fn nani(
    rotation: [f32; 4],
    position: [f32; 3],
    scale: [f32; 3],
    definition_id: u64,
) -> GateNaniProjection {
    GateNaniProjection {
        rotation,
        position,
        scale,
        definition_id,
        observed_word_30: 0,
        subset_kind: 3,
    }
}
/// Raw command -> complete wooden-door CPU mesh, support bricks and Nani
/// outputs. Container insertion/mesh handles/Entity IDs are dependency owners.
pub fn construct_wooden_gate_doors(
    input: GateConstructionInput<'_>,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<GateCpuOutput, GateConstructionError> {
    let shape = recover_gate_shape(input)?;
    let bricks = recover_gate_support_bricks(input, terrain)?;
    let a = shape.bottom_positions[0];
    let b = shape.bottom_positions[shape.bottom_positions.len() - 1];
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let mut curve = GateCurve3 {
        positions: shape.bottom_positions.clone(),
        points_u: shape.bottom_u.clone(),
        length: shape.bottom_length,
    };
    let offset = normal(&curve.positions)?;
    for p in &mut curve.positions {
        for j in 0..3 {
            p[j] += offset[j] * f32::from_bits(0x3e2c0832);
        }
    }
    // The original footer reloads endpoints after the in-place mesh offset;
    // recomputing the delta from pre-offset values loses f32 cancellation bits.
    let fa = curve.positions[0];
    let fb = curve.positions[curve.positions.len() - 1];
    let fx = fb[0] - fa[0];
    let fy = fb[1] - fa[1];
    let fz = fb[2] - fa[2];
    let inverse = 1. / (fz * fz + (fy * fy + fx * fx)).sqrt();
    let forward = [fx * inverse, 0., fz * inverse];
    let facing = [(-inverse) * fz, 0., fx * inverse];
    let rotation = facing_rotation(facing[0], facing[2]);
    let mut nanis = Vec::new();
    let scalar = input.observed_public_288 + 0.5;
    let old = &input.segment.positions;
    let minimum = old.iter().fold(f32::MAX, |v, p| v.min(p[1]));
    let highest = old
        .iter()
        .zip(&input.segment.heights)
        .fold(0f32, |v, (p, h)| v.max(p[1] + h));
    if highest - minimum > scalar && a[1] > 0. {
        let mut large = highest - minimum - scalar >= 2.3;
        let inv = 1. / (dx * dx + dz * dz).sqrt();
        let direction = [dx * inv, dz * inv];
        if !large {
            let rows = (((highest - minimum - scalar) / 0.46).round().max(1.) as usize).min(4);
            let shift = (rows as f32 * 0.56) * 0.8;
            let ground_a = terrain
                .sample_at_world_xz(a[0] + (-direction[1]) * shift, a[2] + direction[0] * shift);
            let ground_b = terrain
                .sample_at_world_xz(b[0] + (-direction[1]) * shift, b[2] + direction[0] * shift);
            large = (ground_a - ground_b).abs() - scalar >= 2.3;
        }
        if large {
            let forward = [direction[0], 0., direction[1]];
            let n = [-direction[1], 0., direction[0]];
            let p: [f32; 3] = if old[old.len() - 1][1] <= old[0][1] {
                std::array::from_fn(|j| b[j] - forward[j] * f32::from_bits(0x3f547ae2))
            } else {
                std::array::from_fn(|j| a[j] + forward[j] * f32::from_bits(0x3f547ae2))
            };
            let p = std::array::from_fn(|j| p[j] + (n[j] * 0.56) * 0.5);
            nanis.push(nani(
                facing_rotation(n[0], n[2]),
                p,
                [1.; 3],
                0x6cd896c8b7173d6e,
            ));
        }
    }
    let maximum = shape.relative_heights.iter().copied().fold(0f32, f32::max);
    let raise = maximum * 0.4 + (-0.23);
    let wide = shape.bottom_length > 2.7;
    let (p, _) = curve.sample(&shape.relative_heights, if wide { 0.5 } else { 0.25 });
    for side in if wide { vec![0.25, -0.25] } else { vec![0.] } {
        let position = [
            p[0] + forward[0] * side + 0. * raise,
            p[1] + forward[1] * side + raise,
            p[2] + forward[2] * side + 0. * raise,
        ];
        nanis.push(nani(rotation, position, [1.; 3], 0xb75231fda9a1fa83));
    }
    Ok(GateCpuOutput {
        bricks,
        leaves: shape.leaves,
        nanis,
    })
}

/// True system-side selection. If more than half the segment's source points
/// lie below Y=0, the original consumer emits interior posts instead of doors.
pub fn construct_gate_cpu(
    input: GateConstructionInput<'_>,
    terrain: &crate::recovered_building::building_wall_collection::TerrainHeightGrid512,
) -> Result<GateCpuOutput, GateConstructionError> {
    let segment = input.segment;
    if segment.positions.len() < 2
        || segment.heights.len() != segment.positions.len()
        || segment
            .positions
            .iter()
            .flatten()
            .chain(&segment.heights)
            .any(|x| !x.is_finite())
    {
        return Err(GateConstructionError::InvalidInput);
    }
    let below = segment
        .positions
        .iter()
        .fold(0f32, |n, p| if p[1] < 0. { n + 1. } else { n });
    if below / segment.positions.len() as f32 <= 0.5 {
        return construct_wooden_gate_doors(input, terrain);
    }
    let mut nanis = Vec::new();
    let mut distance = 0.;
    for i in 1..segment.positions.len() - 1 {
        let a = segment.positions[i];
        let b = segment.positions[i + 1];
        let dx = a[0] - b[0];
        let dz = a[2] - b[2];
        let edge = (dz * dz + dx * dx).sqrt();
        distance += edge;
        if distance > 0.5 {
            let inverse = 1. / edge;
            nanis.push(nani(
                facing_rotation(dx * inverse, dz * inverse),
                [a[0], a[1] + (-0.2), a[2]],
                [1., segment.heights[i] + 0.2, 1.],
                0x7e4dd4fe71bf7165,
            ));
            distance = 0.;
        }
    }
    Ok(GateCpuOutput {
        bricks: Vec::new(),
        leaves: Vec::new(),
        nanis,
    })
}

/// Different source-event operations are retained explicitly. Real EventReader,
/// HashMap and Bevy EntityCommands are supplied by the runtime, not emulated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateInvalidation {
    Rebuild,
    Remove,
}
