//! B wall mask -> materialized wall segments, before actual lintel consumers.
//! This game owner reuses the recovered LoopRange/coordinate projection.
use crate::building_wall_segmentation::{
    WallLoopRange, WallPathSegmentationMask, WallSegmentationError,
};
use glam::Vec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallStructuresError {
    InvalidInput,
    Segmentation(WallSegmentationError),
}
impl From<WallSegmentationError> for WallStructuresError {
    fn from(e: WallSegmentationError) -> Self {
        Self::Segmentation(e)
    }
}
#[derive(Clone, Copy)]
pub struct WallSegmentSource<'a> {
    pub mask: &'a WallPathSegmentationMask,
    pub segment_values: &'a [f32],
    pub arch_heights: &'a [f32],
    pub prepared_positions: &'a [[f32; 3]],
    pub prepared_u: &'a [f32],
    /// Original shape<2 flag controls the arch cap policy.
    pub regular: bool,
    /// Original PublicWall::u_space returns shape!=2 and period1.
    pub closed: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MaterializedWallSegment {
    pub heights: Vec<f32>,
    pub positions: Vec<[f32; 3]>,
    pub points_u: Vec<f32>,
    /// Original source mask local ordinal interval, exclusive end.
    pub source_indices: [usize; 2],
    pub range: WallLoopRange,
}
fn min_source(a: f32, b: f32) -> f32 {
    if a > b {
        b
    } else {
        a
    }
}
fn periodic_subtract(a: f32, b: f32, closed: bool) -> f32 {
    let mut v = a - b;
    if closed {
        if v < -0.5 {
            v += 1.0;
        } else if v >= 0.5 {
            v -= 1.0;
        }
    }
    v
}
fn project(v: f32, closed: bool) -> f32 {
    if closed {
        let r = v % 1.0;
        if r < 0.0 {
            r + 1.0
        } else {
            r
        }
    } else {
        v.max(0.0).min(1.0)
    }
}
/// Whole original14122ff90 output owner. No captured segment/height result input.
pub fn materialize_wall_segments(
    input: WallSegmentSource<'_>,
) -> Result<Vec<MaterializedWallSegment>, WallStructuresError> {
    let mask = input.mask;
    if mask.flags.len() != mask.coords.len()
        || mask.flags.len() != mask.range.len()
        || input.prepared_positions.len() != input.prepared_u.len()
        || mask.range.modulus <= 0
        || input.segment_values.len() < mask.range.modulus as usize
        || input.arch_heights.len() < mask.range.modulus as usize
        || input.prepared_positions.len() < mask.range.modulus as usize
    {
        return Err(WallStructuresError::InvalidInput);
    }
    let anchors: Vec<usize> = mask
        .flags
        .iter()
        .enumerate()
        .filter(|(_, f)| **f & 0xf != 0)
        .map(|(i, _)| i)
        .collect();
    let mut output = Vec::new();
    for pair in anchors.windows(2) {
        let begin = pair[0];
        let end = pair[1];
        let indices: Vec<usize> = (begin..=end)
            .map(|i| mask.range.index_at(i))
            .collect::<Result<_, _>>()?;
        let mut low_y = 0.0f32;
        let mut low_arch = f32::MAX;
        for &index in &indices {
            low_y = min_source(low_y, input.prepared_positions[index][1]);
            if input.regular {
                low_arch = min_source(low_arch, input.arch_heights[index]);
            }
        }
        let adjustment = (low_y.abs() + (-0.75)).max(0.0);
        let cap = low_arch + 0.92;
        let mut heights = Vec::new();
        let mut positions = Vec::new();
        let mut points_u = Vec::new();
        for &index in &indices {
            let mut height = input.segment_values[index] + adjustment;
            if input.regular {
                height = min_source(
                    height,
                    (input.arch_heights[index] - input.prepared_positions[index][1])
                        + f32::from_bits(0xbf75c290),
                );
                height = min_source(height, cap);
            }
            heights.push(height);
            positions.push(input.prepared_positions[index]);
            points_u.push(input.prepared_u[index]);
        }
        for (source_index, target) in [(begin, 0), (end, positions.len() - 1)] {
            let coord = mask.coords[source_index];
            let index = coord.point_index as usize;
            let a = mask.range.index_at(index)?;
            let b = mask.range.index_at(index + 1)?;
            let t = coord.fraction;
            // The private LoopSlice sampler141208d10 uses difference-based
            // interpolation. Reuse Vec3 operators, preserving its order; this
            // dependency version's weighted Vec3::lerp rounds differently.
            let first = Vec3::from_array(input.prepared_positions[a]);
            let second = Vec3::from_array(input.prepared_positions[b]);
            positions[target] = ((second - first) * t + first).to_array();
            let d = periodic_subtract(input.prepared_u[b], input.prepared_u[a], input.closed);
            points_u[target] = project(t * d + input.prepared_u[a], input.closed);
        }
        let range = WallLoopRange {
            start: mask.range.start + begin as i32,
            end: mask.range.start + end as i32 + 1,
            modulus: mask.range.modulus,
        };
        output.push(MaterializedWallSegment {
            heights,
            positions,
            points_u,
            source_indices: [begin, end + 1],
            range,
        });
    }
    Ok(output)
}

/// Actual caller preparation for one segment, before14-argument support owner.
#[derive(Clone, Debug, PartialEq)]
pub struct LintelSupportSource {
    pub heights: Vec<f32>,
    pub ground_positions: Vec<[f32; 3]>,
    pub raised_positions: Vec<[f32; 3]>,
    pub flags: Vec<u8>,
    pub source_coords: Vec<crate::building_wall_segmentation::ThresholdedCurveCoord>,
}
/// Originalconstruct_lintels14076e65c..e800 plus its map closure141204400.
/// The two scalars are authoritative Public+284/+288, not inferred defaults.
pub fn prepare_lintel_support(
    segment: &MaterializedWallSegment,
    public_prepared_positions: &[[f32; 3]],
    public_284: f32,
    public_288: f32,
) -> Result<LintelSupportSource, WallStructuresError> {
    let n = segment.positions.len();
    if n < 2
        || segment.heights.len() != n
        || segment.range.len() != n
        || public_prepared_positions.len() < segment.range.modulus as usize
    {
        return Err(WallStructuresError::InvalidInput);
    }
    let mut highest = segment.heights[0] + segment.positions[0][1];
    let mut selected_ground_y = segment.positions[0][1];
    for (height, point) in segment.heights.iter().zip(&segment.positions) {
        let total = *height + point[1];
        if highest < total {
            highest = total;
            selected_ground_y = point[1];
        }
    }
    let base = (public_284 + 0.5).max(public_288);
    let heights: Vec<_> = segment
        .positions
        .iter()
        .map(|point| ((selected_ground_y - point[1]) + (-0.2)).max(0.0) + base)
        .collect();
    let mut raised_positions = Vec::new();
    for (index, height) in heights.iter().enumerate() {
        let point = public_prepared_positions[segment.range.index_at(index)?];
        // Private map closure adds height*Vec3::Y to public uncut positions.
        raised_positions.push((Vec3::from_array(point) + Vec3::Y * *height).to_array());
    }
    let mut flags = vec![0; n];
    flags[0] = 8;
    flags[n - 1] = 8;
    let mut source_coords = vec![
        crate::building_wall_segmentation::ThresholdedCurveCoord {
            point_index: 0,
            fraction: 0.
        };
        n
    ];
    source_coords[n - 1] = crate::building_wall_segmentation::ThresholdedCurveCoord {
        point_index: (n - 2) as u32,
        fraction: 1.,
    };
    Ok(LintelSupportSource {
        heights,
        ground_positions: segment.positions.clone(),
        raised_positions,
        flags,
        source_coords,
    })
}

/// Originalprivate utils::rdp caller options:epsilon=.2,endpoint-distance=1.
/// The min-endpoint rule and short-left shortcut are private author policies,
/// so a generic dependency RDP algorithm is not substituted.
pub fn rdp_support_indices(points: &[[f32; 3]]) -> Vec<usize> {
    let n = points.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0];
    }
    if n < 3 {
        return vec![0, n - 1];
    }
    let first = Vec3::from_array(points[0]);
    let last = Vec3::from_array(points[n - 1]);
    let chord = (first - last).length();
    let mut initial = chord < 100000000.;
    let mut ranges = vec![(0, n - 1)];
    let mut output = vec![0];
    while let Some((begin, end)) = ranges.pop() {
        if end - begin + 1 <= 2 {
            output.push(end);
            continue;
        }
        let a = Vec3::from_array(points[begin]);
        let z = Vec3::from_array(points[end]);
        let delta = z - a;
        let denominator = delta.length_squared().max(f32::from_bits(0x1e3ce508));
        let mut distance = 0.0f32;
        let mut relative = 0;
        for index in begin + 1..end {
            let point = Vec3::from_array(points[index]);
            let from_a = point - a;
            let from_z = point - z;
            let t = delta.dot(from_a) / denominator;
            // This private utility projects onto the infinite line; unclamped.
            let projected = z * t + a * (1.0 - t);
            let d = (point - projected).length();
            if d > distance && from_a.length_squared() >= 1. && from_z.length_squared() >= 1. {
                distance = d;
                relative = index - begin;
            }
        }
        if distance > 0.2 || (initial && distance > chord) {
            let split = begin + relative;
            initial = false;
            ranges.push((split, end));
            if relative < 3 {
                output.push(split);
            } else {
                ranges.push((begin, split));
            }
        } else {
            output.push(end);
        }
    }
    output
}

#[derive(Clone, Debug, PartialEq)]
pub struct WallWoodenPlankInstance64B {
    pub affine: [f32; 12],
    pub segment_ordinal: u32,
    pub source_index: u32,
    pub wall_id: u32,
}
impl WallWoodenPlankInstance64B {
    pub fn observed_bytes(&self) -> [u8; 64] {
        let mut b = [0; 64];
        for (i, x) in self.affine.iter().enumerate() {
            b[i * 4..i * 4 + 4].copy_from_slice(&x.to_le_bytes());
        }
        b[48..52].copy_from_slice(&self.segment_ordinal.to_le_bytes());
        b[52..56].copy_from_slice(&self.source_index.to_le_bytes());
        b[56..60].copy_from_slice(&self.wall_id.to_le_bytes());
        b
    }
}
#[derive(Clone, Copy)]
struct Grower {
    index: usize,
    point: Vec3,
    direction: Option<Vec3>,
    length: f32,
    stopped: bool,
}
#[derive(Clone, Copy)]
struct Proposal {
    index: usize,
    point: Vec3,
    direction: Vec3,
    length: f32,
    cost: f32,
    stopped: bool,
}
fn length_yxz(v: Vec3) -> f32 {
    ((v.y * v.y + v.x * v.x) + v.z * v.z).sqrt()
}
fn propose(g: Grower, index: usize, points: &[[f32; 3]], flags: &[u8]) -> Option<Proposal> {
    if g.stopped {
        return None;
    }
    let p = Vec3::from_array(points[index]);
    let d = p - g.point;
    let length = (d.x * d.x + d.z * d.z).sqrt();
    let direction = d * (1. / length_yxz(d));
    let total = length + g.length;
    if total > f32::from_bits(0x40499999) {
        return None;
    }
    let cost = if g.length >= 1. {
        total * (length_yxz(g.direction.unwrap_or(direction) - direction) + 0.5)
    } else {
        0.
    };
    Some(Proposal {
        index,
        point: p,
        direction,
        length,
        cost,
        stopped: flags[index] & 15 != 0,
    })
}
fn consume(g: &mut Grower, p: Proposal) {
    g.index = p.index;
    g.point = p.point;
    g.direction = Some(p.direction);
    g.length = p.length + g.length;
    g.stopped = p.stopped;
}
fn affine_instance(
    x: glam::Vec4,
    y: glam::Vec4,
    z: glam::Vec4,
    t: glam::Vec4,
    wall_id: u64,
    ordinal: u32,
    source_index: u32,
) -> WallWoodenPlankInstance64B {
    WallWoodenPlankInstance64B {
        affine: [x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z, t.x, t.y, t.z],
        segment_ordinal: ordinal,
        source_index,
        wall_id: wall_id as u32,
    }
}
fn emit_plank(
    start: Vec3,
    end: Vec3,
    wall_id: u64,
    ordinal: u32,
    base_index: usize,
    out: &mut Vec<WallWoodenPlankInstance64B>,
) {
    use glam::Vec4;
    let a = Vec3::new(start.x, start.y + f32::from_bits(0xbe666667), start.z);
    let b = Vec3::new(end.x, end.y + f32::from_bits(0xbe666667), end.z);
    let delta = b - a;
    let length = length_yxz(delta);
    let d = delta * (1. / length);
    let side = Vec3::Y.cross(d);
    let dir = d.extend(d.z);
    let side = side.extend(side.z);
    let zero = Vec4::ZERO;
    let tail = Vec4::new(0., 0., 0., f32::from_bits(0x3e666667));
    let x = side * tail + dir * tail + Vec4::new(0., f32::from_bits(0x3e666667), 0., 0.);
    let y = dir * Vec4::new(length, length, length, 0.) + zero + side * zero;
    let z = dir * zero
        + zero
        + side
            * Vec4::new(
                f32::from_bits(0x3eaccccc),
                f32::from_bits(0x3eaccccc),
                f32::from_bits(0x3eaccccc),
                0.,
            );
    let t = dir * zero + zero + side * zero + a.extend(a.z);
    let index = ((start.x * 1000.) as i32)
        .wrapping_add((start.z * 1000.) as i32)
        .wrapping_add(base_index as i32) as u32;
    out.push(affine_instance(x, y, z, t, wall_id, ordinal, index));
}
fn walk_lintel(
    heights: &[f32],
    ground: &[[f32; 3]],
    wall_id: u64,
    ordinal: u32,
    base_index: usize,
    out: &mut Vec<WallWoodenPlankInstance64B>,
) {
    let points: Vec<_> = ground
        .iter()
        .zip(heights)
        .map(|(p, h)| Vec3::new(p[0], p[1] + *h, p[2]))
        .collect();
    let mut total = 0.;
    for p in points.windows(2) {
        total += length_yxz(p[1] - p[0]);
    }
    let count = (total / f32::from_bits(0x40499999) + 0.5).floor().max(1.) as usize;
    let step = total / count as f32;
    let mut previous = points[0];
    let mut residual = 0.;
    for pair in points.windows(2) {
        let length = length_yxz(pair[1] - pair[0]);
        let mut remaining = residual + length;
        if step < remaining {
            let mut traversed = 0.;
            loop {
                let distance = (step - residual).max(0.);
                traversed = distance + traversed;
                let fraction = traversed / length;
                let point = pair[1] * fraction + pair[0] * (1. - fraction);
                emit_plank(previous, point, wall_id, ordinal, base_index, out);
                residual = (residual - step).max(0.);
                remaining = (length - traversed) + residual;
                previous = point;
                if !(step < remaining) {
                    break;
                }
            }
        }
        residual = remaining;
    }
    if step * 0.1 < residual {
        emit_plank(
            previous,
            *points.last().unwrap(),
            wall_id,
            ordinal,
            base_index,
            out,
        );
    }
}
fn extend_xz(
    points: &[[f32; 3]],
    mut index: usize,
    mut fraction: f32,
    mut distance: f32,
    right: bool,
) -> (usize, f32) {
    loop {
        let a = points[index];
        let b = points[index + 1];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let length = (dx * dx + dz * dz).sqrt();
        if right {
            let remainder = (1. - fraction) * length - distance;
            if remainder >= 0. {
                return (index, 1. - remainder / length);
            }
            if index + 2 == points.len() {
                return (index, 1.);
            }
            distance = -remainder;
            index += 1;
            fraction = 0.;
        } else {
            let remainder = ((fraction + (-1.)) + 1.) * length - distance;
            if remainder >= 0. {
                return (index, remainder / length);
            }
            if index == 0 {
                return (0, 0.);
            }
            distance = -remainder;
            index -= 1;
            fraction = 1.;
        }
    }
}
fn add_diagonal(
    source: &LintelSupportSource,
    anchor: usize,
    right: bool,
    budget: f32,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<WallWoodenPlankInstance64B>,
) {
    use glam::Vec4;
    let coord = source.source_coords[anchor];
    let index = coord.point_index as usize;
    let fraction = coord.fraction;
    let sample = |i: usize, t: f32| {
        let a = Vec3::from_array(source.ground_positions[i]);
        let b = Vec3::from_array(source.ground_positions[i + 1]);
        (b - a) * t + a
    };
    let sample_height =
        |i: usize, t: f32| (source.heights[i + 1] - source.heights[i]) * t + source.heights[i];
    let height = sample_height(index, fraction) + f32::from_bits(0xbe666667);
    let length = (budget + f32::from_bits(0xbe866666)).min(height * 0.4);
    let extension = (length + f32::from_bits(0x3de66667)) + f32::from_bits(0x3e400000);
    let (target_index, target_fraction) =
        extend_xz(&source.ground_positions, index, fraction, extension, right);
    let a = sample(index, fraction) + Vec3::Y * ((height - length) + f32::from_bits(0xbe400000));
    let b = sample(target_index, target_fraction)
        + Vec3::Y * (sample_height(target_index, target_fraction) + f32::from_bits(0xbde66667));
    let delta = b - a;
    let full_length = length_yxz(delta);
    let d = delta * (1. / full_length);
    let c = d.cross(Vec3::Y);
    let side = c / length_yxz(c);
    let third = side.cross(d);
    let side = side.extend(side.z);
    let dir = d.extend(d.z);
    let third = third.extend(third.z);
    let scale = f32::from_bits(0x3e866666);
    let tail = Vec4::new(0., 0., 0., scale);
    let zero = Vec4::ZERO;
    let x = side * Vec4::splat(scale) + dir * tail + third * tail;
    let y =
        dir * Vec4::new(
            full_length + (-0.2),
            full_length + (-0.2),
            full_length + (-0.2),
            0.,
        ) + side * zero
            + third * zero;
    let z = dir * zero + side * zero + third * Vec4::new(scale, scale, scale, 0.);
    let t = dir * zero + side * zero + third * zero + a.extend(a.z);
    out.push(affine_instance(x, y, z, t, wall_id, ordinal, anchor as u32));
}
fn emit_range(
    source: &LintelSupportSource,
    begin: usize,
    end: usize,
    wall_id: u64,
    ordinal: u32,
    out: &mut Vec<WallWoodenPlankInstance64B>,
) {
    if begin == end {
        return;
    }
    let indices = rdp_support_indices(&source.raised_positions[begin..=end]);
    for pair in indices.windows(2) {
        let a = begin + pair[0];
        let b = begin + pair[1];
        walk_lintel(
            &source.heights[a..=b],
            &source.ground_positions[a..=b],
            wall_id,
            ordinal,
            begin,
            out,
        );
    }
}
/// Complete place_support_beams domain produced by actual lintel caller flags.
pub(crate) fn lintel_support_instances(
    source: &LintelSupportSource,
    wall_id: u64,
    ordinal: u32,
) -> Vec<WallWoodenPlankInstance64B> {
    let end = source.heights.len() - 1;
    let init = |i| Grower {
        index: i,
        point: Vec3::from_array(source.raised_positions[i]),
        direction: None,
        length: 0.,
        stopped: false,
    };
    let mut left = init(0);
    let mut right = init(end);
    let mut next_left = 1;
    let mut next_right = end - 1;
    let mut lp = propose(left, next_left, &source.raised_positions, &source.flags);
    let mut rp = propose(right, next_right, &source.raised_positions, &source.flags);
    while left.index < right.index {
        match (lp, rp) {
            (None, None) => break,
            (Some(p), r) if r.is_none() || r.unwrap().cost > p.cost => {
                consume(&mut left, p);
                next_left += 1;
                lp = if next_left <= end {
                    propose(left, next_left, &source.raised_positions, &source.flags)
                } else {
                    None
                };
            }
            (_, Some(p)) => {
                consume(&mut right, p);
                rp = if next_right > 0 {
                    next_right -= 1;
                    propose(right, next_right, &source.raised_positions, &source.flags)
                } else {
                    None
                };
            }
            _ => unreachable!(),
        }
    }
    let total = left.length + right.length;
    let join = left.index == right.index
        && total >= 1.
        && total <= f32::from_bits(0x40499999)
        && ((left.index != 0
            && propose(left, 0, &source.raised_positions, &source.flags).is_some())
            || (right.index != end
                && propose(right, left.index, &source.raised_positions, &source.flags).is_some()));
    let mut out = Vec::new();
    let (emit_left, emit_right) = if join {
        emit_range(source, 0, end, wall_id, ordinal, &mut out);
        (true, true)
    } else {
        let a = left.length >= 1.;
        let b = right.length >= 1.;
        if a {
            emit_range(source, 0, left.index, wall_id, ordinal, &mut out);
        }
        if b {
            emit_range(source, right.index, end, wall_id, ordinal, &mut out);
        }
        (a, b)
    };
    if left.index != right.index {
        if emit_left {
            add_diagonal(
                source,
                0,
                true,
                left.length * 0.85,
                wall_id,
                ordinal,
                &mut out,
            );
        }
        if emit_right {
            add_diagonal(
                source,
                end,
                false,
                right.length * 0.85,
                wall_id,
                ordinal,
                &mut out,
            );
        }
    } else if (emit_left || emit_right) && total >= f32::from_bits(0x3fb570a3) {
        add_diagonal(source, 0, true, total * 0.5, wall_id, ordinal, &mut out);
        add_diagonal(source, end, false, total * 0.5, wall_id, ordinal, &mut out);
    }
    out
}
/// Native third-authority writer; range-relative indices, not segment ordinals.
pub fn accumulate_lintel_third_heights(
    segments: &[MaterializedWallSegment],
    third: &mut [f32],
) -> Result<(), WallStructuresError> {
    for s in segments {
        if s.heights.len() != s.range.len() {
            return Err(WallStructuresError::InvalidInput);
        }
        for i in 0..s.heights.len() {
            if s.range.index_at(i)? >= third.len() {
                return Err(WallStructuresError::InvalidInput);
            }
        }
    }
    for s in segments {
        for (i, h) in s.heights.iter().enumerate() {
            let index = s.range.index_at(i)?;
            let new = *h + f32::from_bits(0xbde66667);
            let old = third[index];
            third[index] = if old.is_nan() || new > old { new } else { old };
        }
    }
    Ok(())
}
/// True raw mask/prepared/cache -> segments -> support selection ->64B records.
pub fn construct_lintel_supports(
    input: WallSegmentSource<'_>,
    public_284: f32,
    public_288: f32,
    wall_id: u64,
    third: &mut [f32],
) -> Result<Vec<WallWoodenPlankInstance64B>, WallStructuresError> {
    let mut segments = materialize_wall_segments(input)?;
    let mut output = Vec::new();
    for (index, s) in segments.iter_mut().enumerate() {
        let source = prepare_lintel_support(s, input.prepared_positions, public_284, public_288)?;
        output.extend(lintel_support_instances(
            &source,
            wall_id,
            (index + 1) as u32,
        ));
        // Original14076e800 replaces the segment's height Vec before its later
        // third-authority writer. The materializer's old values are consumed.
        s.heights = source.heights;
    }
    accumulate_lintel_third_heights(&segments, third)?;
    Ok(output)
}
