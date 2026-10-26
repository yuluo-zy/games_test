//! B's complete resolved tag>=2 base-cache consumer. Input positions are the
//! prepared Public Curve3 (+90/+98), not the initial terrain-mapped Curve3.
//! Standard Vec/allocator storage is reused; segmentation ownership is external.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::type_complexity)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::roofs::geometry::surface::CurveError;
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use ::utils::bytes::{read_f32_le as float_at, write_f32_le as write_float};
use ::utils::scalar::{clamp, max_f32, min_f32};

#[derive(Clone, Debug)]
pub struct FreeformWallHeightCache {
    pub maximum_y: f32,
    pub arch_segment_heights: Vec<f32>,
    /// The original allocation uses Public segment-owner count +d0, not +98.
    pub segment_heights: Vec<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreeformCacheError {
    RegularShape,
}

fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}

fn numeric_max(previous: f32, current: f32) -> f32 {
    if previous.is_nan() || current > previous {
        current
    } else {
        previous
    }
}

/// Executes the original shape guard and both freeform policies. The observed
/// +295 ClutterVersion byte selects uniform vs per-point heights; its default
/// and the producer of prepared positions/count remain the caller's contract.
/// The third array is freshly zeroed for the real segment-owner count even
/// when that count differs from the prepared point count.
pub fn freeform_wall_height_cache(
    prepared_positions: &[[f32; 3]],
    shape: &WallShapeSnapshot,
    constrained_extra: f32,
    clutter_version: u8,
    prepared_segment_count: usize,
) -> Result<FreeformWallHeightCache, FreeformCacheError> {
    if shape.tag() < 2 {
        return Err(FreeformCacheError::RegularShape);
    }
    let segment_heights = vec![0.0; prepared_segment_count];
    let (maximum_y, arch_segment_heights) = if clutter_version != 0 {
        let top = prepared_positions
            .iter()
            .fold(-2.5, |top, position| numeric_max(top, position[1]));
        let value = add(top, constrained_extra);
        let floor = f32::from_bits(0x3f75c290);
        let maximum = if value > floor { value } else { floor };
        (maximum, vec![maximum; prepared_positions.len()])
    } else {
        let heights: Vec<_> = prepared_positions
            .iter()
            .map(|position| {
                let value = add(position[1], constrained_extra);
                // Original MAXSS/MAXPS source wins on equality and NaN.
                if value > 0.5 { value } else { 0.5 }
            })
            .collect();
        let maximum = heights.iter().copied().fold(0.0, numeric_max);
        (maximum, heights)
    };
    Ok(FreeformWallHeightCache {
        maximum_y,
        arch_segment_heights,
        segment_heights,
    })
}

impl FreeformWallHeightCache {
    /// Only the two identified base/gable cache flags and values are projected.
    /// Arrays are owned by the typed result, not copied into an invented ABI.
    pub fn write_projection(&self, bytes: &mut [u8; 672]) -> PublicWallBuildingState {
        for offset in [0x108, 0x110] {
            bytes[offset..offset + 4].copy_from_slice(&1u32.to_le_bytes());
            bytes[offset + 4..offset + 8].copy_from_slice(&self.maximum_y.to_le_bytes());
        }
        PublicWallBuildingState::from_observed(*bytes)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PreviewRectangleObserved {
    pub axes: [f32; 2],
    pub center: [f32; 2],
    pub dimensions: [f32; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct PreviewCircleObserved {
    pub center: [f32; 2],
    pub radius: f32,
    pub angle: f32,
}

#[derive(Clone, Copy)]
pub struct ShapeHeightInputs<'a> {
    pub app_mode_tag: u32,
    /// Original AppMode tag6 subtype1 circle / subtype2 rectangle.
    pub app_mode_subtype: u8,
    pub rectangle: PreviewRectangleObserved,
    pub circle: PreviewCircleObserved,
    pub terrain: &'a TerrainHeightGrid512,
    pub delta_time: f32,
}

#[derive(Clone, Debug)]
pub struct AppliedShapeHeight {
    pub preview_curve: Curve2,
    pub sampled_positions: Vec<[f32; 3]>,
    pub sampled_u: Vec<f32>,
    pub sampled_length: f32,
    pub requested_extra: f32,
    pub target_world_height: f32,
}

#[derive(Clone, Debug)]
pub enum ShapeHeightOutcome {
    Skipped,
    Applied(AppliedShapeHeight),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeHeightError {
    Curve(CurveError),
    InvalidRectangle,
    ExcessiveSampleCount,
}

pub fn rectangle_curve(rectangle: PreviewRectangleObserved) -> Result<Curve2, ShapeHeightError> {
    let [ux, uz] = rectangle.axes;
    let [cx, cz] = rectangle.center;
    let [width, length] = rectangle.dimensions;
    if !(width * length > 0.0) {
        return Err(ShapeHeightError::InvalidRectangle);
    }
    let [nx, px, nz, pz] = [width * -0.5, width * 0.5, length * -0.5, length * 0.5];
    let p0 = [(nz * -uz + nx * ux) + cx, (nz * ux + nx * uz) + cz];
    let points = vec![
        p0,
        [(nx * ux + pz * -uz) + cx, (nx * uz + pz * ux) + cz],
        [(px * ux + pz * -uz) + cx, (px * uz + pz * ux) + cz],
        [(nz * -uz + px * ux) + cx, (nz * ux + px * uz) + cz],
        p0,
    ];
    Curve2::try_new(points, false).map_err(ShapeHeightError::Curve)
}

pub fn circle_curve(
    circle: PreviewCircleObserved,
    spacing: f32,
) -> Result<Curve2, ShapeHeightError> {
    let tau = f32::from_bits(0x40c90fdb);
    // The original callers select physical spacing (.2 for preview, .5 for
    // wall creation). The same private shape geometry rule serves both.
    let count = ((circle.radius * tau / spacing).ceil() as usize).max(3);
    if count > isize::MAX as usize / 8 {
        return Err(ShapeHeightError::ExcessiveSampleCount);
    }
    let phase = circle.angle % tau;
    let phase = if phase < 0.0 { phase + tau } else { phase };
    let points = (0..count)
        .map(|index| {
            let u = index as f32 / (count as f32 - 1.0);
            let angle = phase + (1.0 - u) * tau;
            [
                circle.center[0] + original_cos_call(angle) * circle.radius,
                circle.center[1] + original_sin_call(angle) * circle.radius,
            ]
        })
        .collect();
    Curve2::try_new(points, false).map_err(ShapeHeightError::Curve)
}

// The original calls cosf then sinf. LLVM otherwise combines adjacent Rust
// calls into platform sincosf, which differs by 1ULP on measured inputs.
// These wrappers preserve the dependency call boundary, not its math internals.
#[inline(never)]
fn original_cos_call(value: f32) -> f32 {
    value.cos()
}
#[inline(never)]
fn original_sin_call(value: f32) -> f32 {
    value.sin()
}

fn preview_terrain_map(
    curve: &Curve2,
    terrain: &TerrainHeightGrid512,
    ground: f32,
) -> Result<(Vec<[f32; 3]>, Vec<f32>, f32), ShapeHeightError> {
    let map = |p: [f32; 2]| {
        [
            p[0],
            numeric_max(terrain.sample_at_world_xz(p[0], p[1]), ground),
            p[1],
        ]
    };
    let mut positions = vec![map(curve.points[0])];
    for pair in curve.points.windows(2) {
        let [a, b] = [pair[0], pair[1]];
        let [dx, dz] = [b[0] - a[0], b[1] - a[1]];
        let length = max_f32((dz * dz + dx * dx).sqrt(), f32::from_bits(0x322bcc77));
        let step = min_f32(0.5 / length, 1.0);
        let count = ((1.0 / step).round() as usize).saturating_add(1).max(2);
        if count > isize::MAX as usize / 12 {
            return Err(ShapeHeightError::ExcessiveSampleCount);
        }
        for index in 1..count {
            let t = index as f32 / (count as f32 - 1.0);
            positions.push(map([
                b[0] * t + a[0] * (1.0 - t),
                b[1] * t + a[1] * (1.0 - t),
            ]));
        }
    }
    let mut length = 0.0;
    let mut us = Vec::with_capacity(positions.len());
    let mut previous = positions[0];
    for point in &positions {
        let [dx, dy, dz] = [
            previous[0] - point[0],
            previous[1] - point[1],
            previous[2] - point[2],
        ];
        length += (dz * dz + (dy * dy + dx * dx)).sqrt();
        us.push(length);
        previous = *point;
    }
    if length == 0.0 || !length.is_finite() {
        return Err(ShapeHeightError::Curve(CurveError::NonFiniteLength));
    }
    for u in &mut us {
        *u /= length;
    }
    Ok((positions, us, length))
}

/// Original B calc_shape_height, including preview shape generation, its own
/// terrain remap, placement policy, TimberframeConstructionParamsCache writes
/// and time_continuous_lerp. The observed byte windows cover only fields read
/// or written by this function; they do not claim whole resource ABI layouts.
pub fn calculate_shape_height(
    input: ShapeHeightInputs<'_>,
    shape_placement: &mut [u8; 68],
    timberframe_parameters: &mut [u8; 16],
    placement_tick: &mut u32,
    parameters_tick: &mut u32,
    current_tick: u32,
) -> Result<ShapeHeightOutcome, ShapeHeightError> {
    if input.app_mode_tag != 6 || !matches!(input.app_mode_subtype, 1 | 2) {
        *parameters_tick = current_tick;
        timberframe_parameters[..4].copy_from_slice(&0u32.to_le_bytes());
        return Ok(ShapeHeightOutcome::Skipped);
    }
    let (curve, extent) = if input.app_mode_subtype == 2 {
        (rectangle_curve(input.rectangle)?, 6.0)
    } else {
        (circle_curve(input.circle, f32::from_bits(0x3e4ccccd))?, 8.0)
    };
    let ground = float_at(shape_placement, 0x3c);
    let (positions, us, length) = preview_terrain_map(&curve, input.terrain, ground)?;
    let mut sum = 0.0;
    let mut maximum = 0.0;
    for p in &positions {
        sum += p[1];
        maximum = numeric_max(maximum, p[1]);
    }
    let source_mode = u32::from_le_bytes(shape_placement[8..12].try_into().unwrap());
    let wanted = if source_mode == 2 {
        extent
            + if shape_placement[0] == 0 {
                0.0
            } else {
                float_at(shape_placement, 4)
            }
    } else {
        (sum / positions.len() as f32 + extent) + f32::from_bits(0xbeeb851f)
    };
    let base_maximum = maximum + f32::from_bits(0xbeeb851f);
    let extra = clamp(
        wanted - base_maximum,
        f32::from_bits(0x3f333333),
        14.0,
    );
    *placement_tick = current_tick;
    write_float(shape_placement, 0x40, extra);
    let baseline = if ground == 0.0 { -2.5 } else { ground };
    *parameters_tick = current_tick;
    write_float(timberframe_parameters, 8, baseline);
    let target = (extra + maximum) + f32::from_bits(0xbeeb851f);
    let previous = if timberframe_parameters[0] == 0 {
        target
    } else {
        float_at(timberframe_parameters, 4)
    };
    let decay = (f32::from_bits(input.delta_time.to_bits() ^ 0x80000000) * 10.0).exp();
    let updated = (target - previous) * (1.0 - decay) + previous;
    timberframe_parameters[..4].copy_from_slice(&1u32.to_le_bytes());
    write_float(timberframe_parameters, 4, updated);
    write_float(timberframe_parameters, 12, updated - baseline);
    Ok(ShapeHeightOutcome::Applied(AppliedShapeHeight {
        preview_curve: curve,
        sampled_positions: positions,
        sampled_u: us,
        sampled_length: length,
        requested_extra: extra,
        target_world_height: target,
    }))
}
