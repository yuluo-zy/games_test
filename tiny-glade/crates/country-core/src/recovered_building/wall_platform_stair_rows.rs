//! Actual platform stair render curves and shared96B wall producer consumer.
//! Geometry/colliders/holes are separate already-proven original consumers.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_clamp, clippy::neg_cmp_op_on_partial_ord)]
use crate::recovered_building::{
    building_wall_construction_sources::brick_source_id,
    building_wall_freeform_cache::{
        PreviewCircleObserved, PreviewRectangleObserved, circle_curve, rectangle_curve,
    },
    building_wall_platform_stairs::WallPlatformStairCell,
    building_wall_production::{
        PlainWallProductionInput, WallProductionBranch, WallProductionError, WallTopBounds,
        produce_wall,
    },
    building_wall_state::{WallId, WallShapeSnapshot},
    surface::Curve2,
};
use glam::Vec2;
#[derive(Clone, Debug, PartialEq)]
pub enum WallPlatformStairRowsError {
    UnsupportedShape,
    InvalidInput,
    Production(WallProductionError),
}
#[derive(Clone, Debug)]
pub struct WallPlatformStairFootprint {
    pub curve: Curve2,
    pub normal_coordinates: Vec<f32>,
}
fn project(u: f32) -> f32 {
    let r = u % 1.0;
    if r < 0.0 { r + 1.0 } else { r }
}
fn boundaries(r: PreviewRectangleObserved) -> [f32; 5] {
    let [w, l] = r.dimensions;
    let sum = w + l;
    let perimeter = sum + sum;
    [
        0.,
        l / perimeter,
        sum / perimeter,
        (l + l + w) / perimeter,
        1.,
    ]
}
fn rectangle_closest(r: PreviewRectangleObserved, curve: &Curve2, p: [f32; 2]) -> f32 {
    let us = boundaries(r);
    let p = Vec2::from_array(p);
    let mut best = f32::MAX;
    let mut answer = 0.;
    for i in 0..4 {
        let a = Vec2::from_array(curve.points[i]);
        let b = Vec2::from_array(curve.points[i + 1]);
        let d = b - a;
        let t = ((p - a).dot(d) / d.length_squared().max(1e-20))
            .max(0.)
            .min(1.);
        let q = b * t + a * (1. - t);
        let distance = (p - q).length();
        if distance < best {
            best = distance;
            answer = (1. - t) * us[i] + t * us[i + 1];
        }
    }
    answer
}
fn circle_closest(c: PreviewCircleObserved, p: [f32; 2]) -> f32 {
    let d = Vec2::new(p[0] - c.center[0], p[1] - c.center[1]);
    let inv = 1. / d.length();
    let x = d.x * inv;
    let z = d.y * inv;
    let tau = f32::from_bits(0x40c90fdb);
    let phase = c.angle % tau;
    let phase = if phase < 0. { phase + tau } else { phase };
    project(1. - (z.atan2(x) - phase) / tau)
}
fn append_slice(curve: &Curve2, a: f32, b: f32, out: &mut Vec<[f32; 2]>) {
    let (i, _) = curve.coord_at_u(a);
    let (j, _) = curve.coord_at_u(b);
    let first = curve.pos_at_u(a);
    let last = curve.pos_at_u(b);
    let begin = i + if first == curve.points[i + 1] { 2 } else { 1 };
    let stop = if last == curve.points[j] {
        j.saturating_sub(1)
    } else {
        j
    };
    out.push(first);
    if begin <= stop {
        out.extend_from_slice(&curve.points[begin..=stop]);
    }
    out.push(last);
}
fn circular_slice(curve: &Curve2, a: f32, b: f32) -> Option<Curve2> {
    let mut points = Vec::new();
    if a < b {
        append_slice(curve, a, b, &mut points);
    } else {
        if a != 1. {
            append_slice(curve, a, 1., &mut points);
        }
        if b != 0. {
            let mut tail = Vec::new();
            append_slice(curve, 0., b, &mut tail);
            if !points.is_empty() {
                points.pop();
            }
            if points.last() == tail.first() {
                points.pop();
            }
            points.extend(tail);
        }
    }
    Curve2::try_new(points, false).ok()
}
fn circle(shape: &WallShapeSnapshot) -> PreviewCircleObserved {
    let f = |at| f32::from_le_bytes(shape.bytes()[at..at + 4].try_into().unwrap());
    PreviewCircleObserved {
        center: [f(4), f(8)],
        radius: f(12),
        angle: f(16),
    }
}
fn rectangle(shape: &WallShapeSnapshot) -> PreviewRectangleObserved {
    let f = |at| f32::from_le_bytes(shape.bytes()[at..at + 4].try_into().unwrap());
    PreviewRectangleObserved {
        axes: [f(4), f(8)],
        center: [f(12), f(16)],
        dimensions: [f(20), f(24)],
    }
}
pub fn platform_stair_footprint(
    shape: &WallShapeSnapshot,
    cell: &WallPlatformStairCell,
) -> Result<Option<WallPlatformStairFootprint>, WallPlatformStairRowsError> {
    let first = [cell.vertices[0][0], cell.vertices[0][2]];
    let last = [cell.vertices[3][0], cell.vertices[3][2]];
    let (source, start, end) = match shape.tag() {
        0 => {
            let c = circle(shape);
            let source =
                circle_curve(c, 0.2).map_err(|_| WallPlatformStairRowsError::InvalidInput)?;
            (source, circle_closest(c, first), circle_closest(c, last))
        }
        1 => {
            let r = rectangle(shape);
            let source =
                rectangle_curve(r).map_err(|_| WallPlatformStairRowsError::InvalidInput)?;
            let a = rectangle_closest(r, &source, first);
            let b = rectangle_closest(r, &source, last);
            (source, a, b)
        }
        _ => return Err(WallPlatformStairRowsError::UnsupportedShape),
    };
    let Some(mut curve) = circular_slice(&source, start, end) else {
        return Ok(None);
    };
    let normal_coordinates = if shape.tag() == 1 {
        if curve.points.len() == 3
            && ((Vec2::from_array(curve.points[0]) - Vec2::from_array(curve.points[1])).length()
                < 0.05
                || (Vec2::from_array(curve.points[1]) - Vec2::from_array(curve.points[2])).length()
                    < 0.05)
        {
            curve = Curve2::try_new(vec![curve.points[0], curve.points[2]], false)
                .map_err(|_| WallPlatformStairRowsError::InvalidInput)?;
        }
        let mut us = vec![start];
        let b = boundaries(rectangle(shape));
        let target = if end < start { end + 1. } else { end };
        for &u in &b[1..4] {
            if start < u && u < target {
                us.push(u);
            }
        }
        if start < 1. && 1. < target {
            us.push(0.);
        }
        for &u in &b[1..4] {
            let extended = u + 1.;
            if start < extended && extended < target {
                us.push(extended + (-1.));
            }
        }
        us.push(end);
        us
    } else {
        let target = if end < start { end + 1. } else { end };
        curve
            .points_u
            .iter()
            .map(|u| project(u * target + (1. - u) * start))
            .collect()
    };
    Ok(Some(WallPlatformStairFootprint {
        curve,
        normal_coordinates,
    }))
}
#[inline(never)]
fn original_cos(x: f32) -> f32 {
    x.cos()
}
#[inline(never)]
fn original_sin(x: f32) -> f32 {
    x.sin()
}
fn normal(shape: &WallShapeSnapshot, u: f32) -> [f32; 2] {
    if shape.tag() == 0 {
        let c = circle(shape);
        let tau = f32::from_bits(0x40c90fdb);
        let phase = c.angle % tau;
        let phase = if phase < 0. { phase + tau } else { phase };
        let angle = phase + (1. - u) * tau;
        return [original_cos(angle), original_sin(angle)];
    }
    let r = rectangle(shape);
    let [x, z] = r.axes;
    let edge = [[-x, -z], [-z, x], [x, z], [z, -x]];
    let b = boundaries(r);
    let corner = if u == 0. || u == 1. {
        Some(0)
    } else {
        (1..4).find(|&i| u == b[i])
    };
    if let Some(i) = corner {
        let a = Vec2::from_array(edge[(i + 3) % 4]);
        let n = a + Vec2::from_array(edge[i]);
        let inv = 1. / n.length();
        let n = n * inv * f32::from_bits(0x3fb504f3);
        n.to_array()
    } else {
        edge[if u < b[1] {
            0
        } else if u < b[2] {
            1
        } else if u < b[3] {
            2
        } else {
            3
        }]
    }
}
pub fn platform_stair_gpu_instance(source: &[u8; 96]) -> [u8; 96] {
    let mut out = [0; 96];
    out[..48].copy_from_slice(&source[..48]);
    for at in [64, 68, 72] {
        out[at..at + 4].copy_from_slice(&(-100f32).to_le_bytes());
    }
    out[76..84].copy_from_slice(&source[76..84]);
    out[92..96].copy_from_slice(&16u32.to_le_bytes());
    out
}
pub fn platform_stair_rows(
    shape: &WallShapeSnapshot,
    wall: WallId,
    cell: &WallPlatformStairCell,
) -> Result<Vec<[u8; 96]>, WallPlatformStairRowsError> {
    let Some(foot) = platform_stair_footprint(shape, cell)? else {
        return Ok(Vec::new());
    };
    let base = cell.vertices[4][1] + f32::from_bits(0xbeeb851f);
    let original_bottom = cell.vertices[4][1];
    let mut output = Vec::new();
    for i in 0..cell.rise_count {
        let t = if cell.rise_count == 2 {
            i as f32 * 0.75
        } else {
            i as f32 / (cell.rise_count as f32 + (-1.))
        };
        let height = cell.render_bottom * t + cell.top * (1. - t);
        if !(height > 0.) {
            continue;
        }
        let offset = t * cell.depth + (1. - t) * f32::from_bits(0x3ee56042);
        let mut points = foot.curve.points.clone();
        for (p, &u) in points.iter_mut().zip(&foot.normal_coordinates) {
            let n = normal(shape, u);
            p[0] += n[0] * offset;
            p[1] += n[1] * offset;
        }
        points.dedup();
        let Ok(curve) = Curve2::try_new(points, false) else {
            continue;
        };
        let n = curve.points.len();
        let arch = vec![height; n];
        let second = vec![base; n];
        let third = vec![0.; n];
        let instances = produce_wall(
            PlainWallProductionInput {
                curve: &curve,
                arch_heights: &arch,
                second_heights: &second,
                third_heights: &third,
                holes: &[],
                holes_length: 0.,
                closed: false,
                seed: 0,
                brick_source_id: brick_source_id(wall),
                base,
                row_end_perturbation: 0.2,
                width_padding: None,
                thickness_scale: None,
                wall_float: height - original_bottom,
            },
            WallProductionBranch {
                kind: 0,
                top: WallTopBounds::PLAIN,
                fence_fit: false,
            },
        )
        .map_err(WallPlatformStairRowsError::Production)?;
        output.extend(
            instances
                .iter()
                .map(|i| platform_stair_gpu_instance(&i.observed_bytes())),
        );
    }
    Ok(output)
}
