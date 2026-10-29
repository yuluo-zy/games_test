//! B regular-wall arches: actual mask -> profile -> walker ->96B CPU records.
//! Reuse std/glam and the recovered mask/segment/source-ID owners.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::assign_op_pattern, clippy::neg_cmp_op_on_partial_ord)]
use crate::resources::walls::publication_fields::WallId;
use crate::systems::wall::construction::bricks::WallBrickInstance;
use crate::systems::wall::construction::parameters::brick_source_id;
use crate::systems::wall::construction::segmentation::WallPathSegmentationMask;
use crate::systems::wall::construction::structures::MaterializedWallSegment;
use crate::systems::wall::construction::structures::WallSegmentSource;
use crate::systems::wall::construction::structures::WallStructuresError;
use crate::systems::wall::construction::structures::materialize_wall_segments;
use glam::Vec3;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallArchesError {
    InvalidInput,
    UnsupportedShape,
    Structures(WallStructuresError),
}
impl From<WallStructuresError> for WallArchesError {
    fn from(e: WallStructuresError) -> Self {
        Self::Structures(e)
    }
}
#[derive(Clone, Copy)]
pub struct RegularWallArchesInput<'a> {
    pub masks: &'a [WallPathSegmentationMask],
    pub prepared_positions: &'a [[f32; 3]],
    pub prepared_u: &'a [f32],
    pub path_values: &'a [f32],
    pub shape_tag: u32,
    pub public_284: f32,
    /// Actual Public294 from Raw64; its authored semantic name is not established.
    pub observed_294: u8,
    pub source_curve_length: f32,
    pub wall_id: WallId,
    /// From validated PublicWalls->Roof relationship, not geometric guessing.
    pub has_roof: bool,
}
#[derive(Clone, Debug)]
pub struct ConstructGateCmd120Projection {
    pub segment: MaterializedWallSegment,
    pub wall_id: WallId,
    pub ordinal: u64,
}
#[derive(Clone, Debug, Default)]
pub struct WallArchesOutput {
    pub material_stats: super::arch_stats::WallArchMaterialStats,
    pub instances: Vec<WallBrickInstance>,
    pub gate_commands: Vec<ConstructGateCmd120Projection>,
    pub segment_count: u64,
    /// Original ArchSegments map owns these full104B records after profiling
    /// (140762a58..140762a99), independently of generated brick instances.
    pub segments: Vec<MaterializedWallSegment>,
    /// 按掩码顺序保存由原 arch 段端点生成的柱提案缓存。
    pub pillar_caches: Vec<(Vec<[u8; 28]>, Vec<f32>, Vec<usize>)>,
}
/// B 140760c20..1407610f8：按原 104B 段端点顺序归并柱提案。
pub fn collect_pillar_endpoints(
    segment: &MaterializedWallSegment,
    cache: &mut (Vec<[u8; 28]>, Vec<f32>, Vec<usize>),
    touched_start: &mut bool,
    lintel: bool,
) {
    let n = segment.heights.len();
    assert!(n > 0 && segment.positions.len() >= n && segment.points_u.len() >= n);
    let mut add = |key: usize, at: usize, adjacent: usize| {
        let p = segment.positions[at];
        let q = segment.positions[adjacent];
        let (dx, dz) = if at == 0 { (q[0] - p[0], q[2] - p[2]) }
                       else { (p[0] - q[0], p[2] - q[2]) };
        let inv = 1.0 / (dz * dz + dx * dx).sqrt();
        let score = if lintel { segment.heights[at] + f32::from_bits(0xbe666667) } else { segment.heights[at] };
        let values = [dx * inv, dz * inv, p[0], p[1], p[2], score, segment.points_u[at]];
        let mut proposal = [0u8; 28];
        for (chunk, value) in proposal.chunks_exact_mut(4).zip(values) { chunk.copy_from_slice(&value.to_le_bytes()); }
        if cache.2.last().copied() == Some(key) {
            let old = cache.1.last_mut().expect("原柱提案缺少分数");
            *old = if old.is_nan() || score < *old { score } else { *old };
            let previous = cache.0.last_mut().expect("原柱提案缺少记录");
            let previous_score = f32::from_le_bytes(previous[20..24].try_into().unwrap());
            if score > previous_score { *previous = proposal; }
        } else { cache.0.push(proposal); cache.1.push(score); cache.2.push(key); }
    };
    if lintel || segment.heights[0] > 0.3 {
        assert!(n >= 2);
        if !lintel { *touched_start |= segment.range.start.rem_euclid(segment.range.modulus) == 0; }
        add(segment.source_indices[0], 0, 1);
    }
    let length = segment.positions.windows(2).fold(0.0f32, |sum, pair| {
        let d = Vec3::from_array(pair[1]) - Vec3::from_array(pair[0]);
        sum + ((d.y*d.y + d.x*d.x) + d.z*d.z).sqrt()
    });
    if lintel || (segment.heights[n - 1] > 0.3 && length > 0.3
        && !(*touched_start && segment.range.end.rem_euclid(segment.range.modulus) == 1))
    {
        assert!(n >= 2);
        add(segment.source_indices[1] - 1, n - 1, n - 2);
    }
}
fn length_xyz(v: Vec3) -> f32 {
    ((v.x * v.x + v.y * v.y) + v.z * v.z).sqrt()
}
fn maxnum(a: f32, b: f32) -> f32 {
    if a.is_nan() || b > a { b } else { a }
}
fn minnum(a: f32, b: f32) -> f32 {
    if a.is_nan() || b < a { b } else { a }
}
fn clamp_ratio(x: f32) -> f32 {
    let v = if 0. <= x { x } else { 0. };
    if v <= 1. { v } else { 1. }
}
fn highest(segment: &MaterializedWallSegment) -> [f32; 2] {
    let mut highest = segment.heights[0] + segment.positions[0][1];
    let mut y = segment.positions[0][1];
    for (h, p) in segment.heights.iter().zip(&segment.positions) {
        let candidate = *h + p[1];
        if highest < candidate {
            y = p[1];
        }
        highest = maxnum(highest, candidate);
    }
    [highest, y]
}
fn regular_profile(
    segment: &mut MaterializedWallSegment,
    wall_float: f32,
    arch: &[f32],
    override_height: Option<[f32; 2]>,
) -> Result<(), WallArchesError> {
    let n = segment.positions.len();
    if n < 2 || segment.heights.len() != n {
        return Err(WallArchesError::InvalidInput);
    }
    let [maximum, ground] = override_height.unwrap_or_else(|| highest(segment));
    let mut heights = Vec::with_capacity(n);
    for (i, (h, p)) in segment.heights.iter().zip(&segment.positions).enumerate() {
        let ratio = clamp_ratio(*h / wall_float);
        let ratio = ratio * 0.5 + (1. - ratio) * 0.;
        let t = i as f32 / (n as f32 + (-1.));
        let edge = (t + (-0.5)).abs();
        let edge = edge + edge;
        let cube = (edge * edge) * edge;
        let weight = 1. - (1. - ratio) * cube;
        let lower = ((maxnum(maximum + (-3.5), ground) + (-1.)) - p[1]).max(0.);
        let raw = (maximum - p[1]) * weight + (1. - weight) * lower;
        let index = segment
            .range
            .index_at(i)
            .map_err(WallStructuresError::from)?;
        let cap = *arch.get(index).ok_or(WallArchesError::InvalidInput)? - p[1];
        heights.push(minnum(raw, cap));
    }
    segment.heights = heights;
    Ok(())
}
fn periodic_delta(a: f32, b: f32) -> f32 {
    let mut d = a - b;
    if d < -0.5 {
        d += 1.;
    } else if d >= 0.5 {
        d -= 1.;
    }
    d
}
fn project_period(v: f32) -> f32 {
    let r = v % 1.;
    if r < 0. { r + 1. } else { r }
}
pub(crate) fn emit_arch_brick(
    start: Vec3,
    end: Vec3,
    us: [f32; 2],
    wall: WallId,
    length: f32,
    ordinal: u64,
    index: u64,
) -> WallBrickInstance {
    let center = (start + end) * 0.5;
    let delta = end - start;
    let full = length_xyz(delta);
    let d = delta * (1. / full);
    let side = d.cross(Vec3::Y);
    let side = side * (1. / length_xyz(side));
    let up = side.cross(d);
    let x = d * full + up * 0. + side * 0.;
    let y = d * 0. + up * f32::from_bits(0x3ed70a3e) + side * 0.;
    let z = d * 0. + up * 0. + side * 0.7;
    // Real BrickTransformSsbo::default1408587c0: deformation0,surface[-100;3],flags0.
    let end_u = if us[1] < us[0] { us[1] + 1. } else { us[1] };
    WallBrickInstance {
        affine_columns: [x.to_array(), y.to_array(), z.to_array(), center.to_array()],
        deformation: [0.; 4],
        surface: [-100.; 3],
        seed: (ordinal as u32).wrapping_add(index as u32),
        material: brick_source_id(wall),
        wall_distance: [us[0] * length, end_u * length],
        flags: 0,
    }
}
fn walk_profile(
    segment: &MaterializedWallSegment,
    wall: WallId,
    length: f32,
    ordinal: u64,
    out: &mut Vec<WallBrickInstance>,
) {
    let points: Vec<_> = segment
        .positions
        .iter()
        .zip(&segment.heights)
        .map(|(p, h)| Vec3::new(p[0], p[1] + *h, p[2]))
        .collect();
    let mut total = 0.;
    for p in points.windows(2) {
        total += length_xyz(p[1] - p[0]);
    }
    let count = (total / f32::from_bits(0x3f0f5c29) + 0.5).floor().max(1.) as usize;
    let step = total / count as f32;
    let mut previous = points[0];
    let mut previous_u = segment.points_u[0];
    let mut residual = 0.;
    let mut emitted = 0u64;
    for (i, pair) in points.windows(2).enumerate() {
        let distance = length_xyz(pair[1] - pair[0]);
        let mut remaining = residual + distance;
        if step < remaining {
            let mut traversed = 0.;
            loop {
                traversed = (step - residual).max(0.) + traversed;
                let t = traversed / distance;
                let next = pair[0] * (1. - t) + pair[1] * t;
                let u = project_period(
                    t * periodic_delta(segment.points_u[i + 1], segment.points_u[i])
                        + segment.points_u[i],
                );
                out.push(emit_arch_brick(
                    previous,
                    next,
                    [previous_u, u],
                    wall,
                    length,
                    ordinal,
                    emitted,
                ));
                emitted += 1;
                residual = (residual - step).max(0.);
                remaining = (distance - traversed) + residual;
                previous = next;
                previous_u = u;
                if !(step < remaining) {
                    break;
                }
            }
        }
        residual = remaining;
    }
    if step * 0.1 < residual {
        out.push(emit_arch_brick(
            previous,
            *points.last().unwrap(),
            [previous_u, *segment.points_u.last().unwrap()],
            wall,
            length,
            ordinal,
            emitted,
        ));
    }
}
/// Complete regular(shape0/1) original producer. Source arrays remain explicit;
/// profile, walker points, seed/index and CPU instance metadata are internal.
/// Gate command payloads exclude Bevy EventId and event buffer bookkeeping.
pub fn construct_regular_wall_arches(
    input: RegularWallArchesInput<'_>,
    arch: &mut [f32],
    third: &mut [f32],
) -> Result<WallArchesOutput, WallArchesError> {
    if input.shape_tag > 1 {
        return Err(WallArchesError::UnsupportedShape);
    }
    let n = input.prepared_positions.len();
    if n < 2
        || input.prepared_u.len() != n
        || input.path_values.len() != n
        || arch.len() != n
        || third.len() != n
        || !input.public_284.is_finite()
        || input.public_284 <= 0.
        || !input.source_curve_length.is_finite()
    {
        return Err(WallArchesError::InvalidInput);
    }
    let mut out = WallArchesOutput {
        material_stats: Default::default(),
        instances: Vec::new(),
        gate_commands: Vec::new(),
        segment_count: 0,
        segments: Vec::new(),
        pillar_caches: Vec::new(),
    };
    let mut touched_start = false;
    let mut all = Vec::new();
    for mask in input.masks {
        let mut segments = materialize_wall_segments(WallSegmentSource {
            mask,
            segment_values: input.path_values,
            arch_heights: arch,
            prepared_positions: input.prepared_positions,
            prepared_u: input.prepared_u,
            regular: true,
            closed: true,
        })?;
        out.material_stats
            .observe_mask(mask.length, &segments, input.has_roof);
        let override_height = if input.observed_294 != 0 {
            let mut maximum = 0.;
            let mut ground = 0.;
            for s in &segments {
                for (h, p) in s.heights.iter().zip(&s.positions) {
                    let total = *h + p[1];
                    if maximum < total {
                        ground = p[1];
                    }
                    maximum = maxnum(maximum, total);
                }
            }
            Some([maximum, ground])
        } else {
            None
        };
        let defer = input.has_roof && segments.len() == 1;
        for s in &mut segments {
            out.segment_count += 1;
            regular_profile(s, input.public_284, arch, override_height)?;
            if defer {
                out.gate_commands.push(ConstructGateCmd120Projection {
                    segment: s.clone(),
                    wall_id: input.wall_id,
                    ordinal: out.segment_count,
                });
            }
            walk_profile(
                s,
                input.wall_id,
                input.source_curve_length,
                out.segment_count,
                &mut out.instances,
            );
        }
        let mut cache = (mask.pillar_proposals.clone(), mask.pillar_scores.clone(), mask.pillar_keys.clone());
        for segment in &segments {
            collect_pillar_endpoints(segment, &mut cache, &mut touched_start, false);
        }
        out.pillar_caches.push(cache);
        all.extend(segments);
    }
    for s in &all {
        for (i, h) in s.heights.iter().enumerate() {
            let index = s.range.index_at(i).map_err(WallStructuresError::from)?;
            let slot = third.get_mut(index).ok_or(WallArchesError::InvalidInput)?;
            *slot = maxnum(*slot, *h);
        }
    }
    third[n - 1] = third[0];
    arch[n - 1] = arch[0];
    out.segments = all;
    Ok(out)
}

#[cfg(test)]
mod pillar_cache_tests {
    use super::*;
    use crate::systems::wall::construction::segmentation::WallLoopRange;

    #[test]
    fn arch_and_lintel_endpoint_caches_follow_distinct_original_gates() {
        let segment = MaterializedWallSegment {
            heights: vec![0.3, 1.0], positions: vec![[0.0, 0.0, 0.0], [0.3, 0.0, 0.0]],
            points_u: vec![0.0, 0.5], source_indices: [0, 2],
            range: WallLoopRange { start: 0, end: 2, modulus: 4 },
        };
        let mut arch = (Vec::new(), Vec::new(), Vec::new());
        collect_pillar_endpoints(&segment, &mut arch, &mut false, false);
        assert!(arch.0.is_empty());
        let mut lintel = (Vec::new(), Vec::new(), Vec::new());
        collect_pillar_endpoints(&segment, &mut lintel, &mut false, true);
        assert_eq!(lintel.2, [0, 1]);
        assert_eq!(lintel.1, [0.3 + f32::from_bits(0xbe666667), 1.0 + f32::from_bits(0xbe666667)]);
        assert_eq!(f32::from_le_bytes(lintel.0[1][24..28].try_into().unwrap()), 0.5);
    }
}
