//! B freehand arches: actual mask -> endpoint profile -> walker -> CPU records
//! and the actual third-height/resmoothing cache consumers.
use crate::{
    building_wall_arches::{
        emit_arch_brick, ConstructGateCmd120Projection, WallArchesError, WallArchesOutput,
    },
    building_wall_segmentation::WallPathSegmentationMask,
    building_wall_state::WallId,
    building_wall_structures::{
        materialize_wall_segments, MaterializedWallSegment, WallSegmentSource, WallStructuresError,
    },
    surface::Curve2,
};
use glam::Vec3;
#[derive(Clone, Copy)]
pub struct FreehandWallArchesInput<'a> {
    pub masks: &'a [WallPathSegmentationMask],
    pub prepared_positions: &'a [[f32; 3]],
    pub prepared_u: &'a [f32],
    pub path_values: &'a [f32],
    pub public_284: f32,
    pub public_288: f32,
    pub observed_294: u8,
    pub clutter_version: u8,
    pub source_curve_length: f32,
    pub wall_id: WallId,
    pub has_roof: bool,
}
fn maxnum(a: f32, b: f32) -> f32 {
    if a.is_nan() || b > a {
        b
    } else {
        a
    }
}
fn clamp(x: f32) -> f32 {
    let x = if x >= 0. { x } else { 0. };
    if x <= 1. {
        x
    } else {
        1.
    }
}
fn length(v: Vec3) -> f32 {
    ((v.x * v.x + v.y * v.y) + v.z * v.z).sqrt()
}
fn curve(points: &[[f32; 3]]) -> Result<Curve2, WallArchesError> {
    Curve2::try_new(points.iter().map(|p| [p[0], p[2]]).collect(), false)
        .map_err(|_| WallArchesError::InvalidInput)
}
fn slice(c: &Curve2, a: f32, b: f32) -> Result<Curve2, WallArchesError> {
    let (ia, _) = c.coord_at_u(a);
    let (ib, _) = c.coord_at_u(b);
    let start = c.pos_at_u(a);
    let end = c.pos_at_u(b);
    // Original slice policy omits exactly equal adjoining endpoint samples.
    let first = ia + if start == c.points[ia + 1] { 2 } else { 1 };
    let last = if end == c.points[ib] {
        ib.saturating_sub(1)
    } else {
        ib
    };
    let mut pts = vec![start];
    if first <= last {
        pts.extend_from_slice(&c.points[first..=last]);
    }
    pts.push(end);
    Curve2::try_new(pts, false).map_err(|_| WallArchesError::InvalidInput)
}
fn highest(s: &MaterializedWallSegment) -> [f32; 2] {
    let mut h = s.heights[0] + s.positions[0][1];
    let mut y = s.positions[0][1];
    for (a, p) in s.heights.iter().zip(&s.positions) {
        let v = *a + p[1];
        if h < v {
            y = p[1];
        }
        h = maxnum(h, v);
    }
    [h, y]
}
fn profile(
    s: &mut MaterializedWallSegment,
    wall_float: f32,
    prepared_count: usize,
    override_height: Option<[f32; 2]>,
) -> Result<[f32; 2], WallArchesError> {
    let n = s.positions.len();
    if n < 2 || s.heights.len() != n {
        return Err(WallArchesError::InvalidInput);
    }
    let [maximum, ground] = override_height.unwrap_or_else(|| highest(s));
    let boundary = s.range.start == 0 || s.range.end == prepared_count as i32;
    let end_only = s.range.start != 0;
    let mut xyzlength = 0.;
    for p in s.positions.windows(2) {
        xyzlength += length(Vec3::from(p[1]) - Vec3::from(p[0]));
    }
    let mut trim = [0., 1.];
    if boundary {
        let c = curve(&s.positions)?;
        if c.length > 0.1875 {
            let f = (0.1875 / c.length).min(f32::from_bits(0x3f733333));
            trim = if end_only {
                [0., 1. - f]
            } else if s.range.end == prepared_count as i32 {
                let f = f.min(f32::from_bits(0x3ef33333));
                [f, 1. - f]
            } else {
                [f, 1.]
            };
            let c = slice(&c, trim[0], trim[1])?;
            for (i, p) in s.positions.iter_mut().enumerate() {
                let pos = c.pos_at_u(i as f32 / (n as f32 + (-1.)));
                p[0] = pos[0];
                p[2] = pos[1];
            }
        }
    }
    let mut hs = Vec::with_capacity(n);
    for (i, (h, p)) in s.heights.iter().zip(&s.positions).enumerate() {
        let q = clamp(*h / wall_float);
        let q = q * 0.5 + (1. - q) * 0.;
        let t = i as f32 / (n as f32 + (-1.));
        let e = 2. * (t + (-0.5)).abs();
        let cube = (e * e) * e;
        let cubic = 1. - (1. - q) * cube;
        let weight = if boundary {
            let alpha = clamp((xyzlength - 2.) / (1. - 2.));
            let side = if end_only { 1. - t } else { t };
            let quad = 1. - (1. - q) * (side * side);
            (1. - alpha) * cubic + alpha * quad
        } else {
            cubic
        };
        let lower = ((maxnum(maximum + (-3.5), ground) + (-1.)) - p[1]).max(0.);
        hs.push((maximum - p[1]) * weight + (1. - weight) * lower);
    }
    s.heights = hs;
    Ok(trim)
}
fn walk(
    s: &MaterializedWallSegment,
    wall: WallId,
    source_length: f32,
    ordinal: u64,
    out: &mut Vec<crate::building_wall_constructor::WallBrickInstance>,
) {
    let pts: Vec<_> = s
        .positions
        .iter()
        .zip(&s.heights)
        .map(|(p, h)| Vec3::new(p[0], p[1] + *h, p[2]))
        .collect();
    let mut total = 0.;
    for p in pts.windows(2) {
        total += length(p[1] - p[0]);
    }
    let count = (total / f32::from_bits(0x3f0f5c29) + 0.5).floor().max(1.) as usize;
    let step = total / count as f32;
    let mut previous = pts[0];
    let mut previous_u = s.points_u[0];
    let mut residual = 0.;
    let mut emitted = 0;
    for (i, p) in pts.windows(2).enumerate() {
        let dist = length(p[1] - p[0]);
        let mut remaining = residual + dist;
        if step < remaining {
            let mut traversed = 0.;
            loop {
                traversed = (step - residual).max(0.) + traversed;
                let t = traversed / dist;
                let next = p[0] * (1. - t) + p[1] * t;
                let u = t * (s.points_u[i + 1] - s.points_u[i]) + s.points_u[i];
                out.push(emit_arch_brick(
                    previous,
                    next,
                    [previous_u, u],
                    wall,
                    source_length,
                    ordinal,
                    emitted,
                ));
                emitted += 1;
                residual = (residual - step).max(0.);
                remaining = (dist - traversed) + residual;
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
            *pts.last().unwrap(),
            [previous_u, *s.points_u.last().unwrap()],
            wall,
            source_length,
            ordinal,
            emitted,
        ));
    }
}
/// Whole freehand CPU owner. No captured profile, trim, walker or RNG inputs.
pub fn construct_freehand_wall_arches(
    input: FreehandWallArchesInput<'_>,
    arch: &mut [f32],
    third: &mut [f32],
) -> Result<WallArchesOutput, WallArchesError> {
    let n = input.prepared_positions.len();
    if n < 2
        || input.prepared_u.len() != n
        || input.path_values.len() != n
        || arch.len() != n
        || third.len() != n
        || !input.public_284.is_finite()
        || input.public_284 <= 0.
        || !input.public_288.is_finite()
    {
        return Err(WallArchesError::InvalidInput);
    }
    let mut out = WallArchesOutput {
        instances: Vec::new(),
        gate_commands: Vec::new(),
        segment_count: 0,
    };
    let mut all = Vec::new();
    for mask in input.masks {
        let mut ss = materialize_wall_segments(WallSegmentSource {
            mask,
            segment_values: input.path_values,
            arch_heights: arch,
            prepared_positions: input.prepared_positions,
            prepared_u: input.prepared_u,
            regular: false,
            closed: false,
        })?;
        let override_height = if input.observed_294 != 0 {
            let mut h = 0.;
            let mut y = 0.;
            for s in &ss {
                for (a, p) in s.heights.iter().zip(&s.positions) {
                    let v = *a + p[1];
                    if h < v {
                        y = p[1];
                    }
                    h = maxnum(h, v);
                }
            }
            Some([h, y])
        } else {
            None
        };
        let defer = input.has_roof && ss.len() == 1;
        for s in &mut ss {
            out.segment_count += 1;
            let trim = profile(s, input.public_284, n, override_height)?;
            if defer {
                out.gate_commands.push(ConstructGateCmd120Projection {
                    segment: s.clone(),
                    wall_id: input.wall_id,
                    ordinal: out.segment_count,
                });
            }
            walk(
                s,
                input.wall_id,
                input.source_curve_length,
                out.segment_count,
                &mut out.instances,
            );
            all.push((s.clone(), trim));
        }
    }
    for (s, trim) in &all {
        let c = if *trim == [0., 1.] {
            None
        } else {
            Some(curve(&s.positions)?)
        };
        for (i, h) in s.heights.iter().enumerate() {
            let value = if let Some(c) = &c {
                let t = i as f32 / (s.heights.len() as f32 + (-1.));
                let u = clamp((t - trim[0]) / (trim[1] - trim[0]));
                let (j, f) = c.coord_at_u(u);
                (s.heights[j + 1] - s.heights[j]) * f + s.heights[j]
            } else {
                *h
            };
            let index = s.range.index_at(i).map_err(WallStructuresError::from)?;
            let slot = third.get_mut(index).ok_or(WallArchesError::InvalidInput)?;
            *slot = maxnum(*slot, value);
        }
    }
    if input.clutter_version == 0
        && input.public_288 + f32::from_bits(0x3f6b851f) > input.public_284
    {
        let mut raised = arch.to_vec();
        for (s, _) in &all {
            let mut maximum = s.heights[0]
                + input.prepared_positions
                    [s.range.index_at(0).map_err(WallStructuresError::from)?][1];
            for (i, h) in s.heights.iter().enumerate() {
                let index = s.range.index_at(i).map_err(WallStructuresError::from)?;
                maximum = maxnum(maximum, *h + input.prepared_positions[index][1]);
            }
            maximum += f32::from_bits(0x3f6b851f);
            for i in 0..s.heights.len() {
                let j = s.range.index_at(i).map_err(WallStructuresError::from)?;
                raised[j] = maxnum(raised[j], maximum);
            }
        }
        for _ in 0..15 {
            let old = raised.clone();
            for i in 1..n - 1 {
                raised[i] = (((old[i - 1] + old[i + 1]) * 0.5 - old[i]) * 0.5) + old[i];
            }
        }
        for (i, a) in arch.iter_mut().enumerate() {
            *a = maxnum(*a, raised[i]);
        }
    }
    Ok(out)
}
