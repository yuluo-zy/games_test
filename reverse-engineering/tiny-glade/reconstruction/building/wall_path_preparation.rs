//! B sample_wall_path_intersections: explicit four encoded path raster owners.
//! Uses actual selected buffers for attenuation, smoothing and threshold cuts.
use crate::building_wall_collection::TerrainHeightGrid512;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PathRasters512 {
    encoded: [Arc<[u8]>; 4],
    pub sampling_fix: bool,
    empty: bool,
    normalized: [TerrainHeightGrid512; 4],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathPreparationError {
    RasterExtent,
    SourceArrayCount,
    SourceScalar,
    InvalidCurve,
}
impl PathRasters512 {
    /// No Default or manufactured clear buffers. These are the actual current
    /// four WorldRaster buffers selected by PathRastersResources::as_float_grid.
    pub fn from_encoded(
        encoded: [Arc<[u8]>; 4],
        sampling_fix: bool,
    ) -> Result<Self, PathPreparationError> {
        if encoded.iter().any(|v| v.len() != 512 * 512) {
            return Err(PathPreparationError::RasterExtent);
        }
        let empty = encoded.iter().all(|v| v.iter().all(|code| *code == 0));
        // u8/255 == (u8*257)/65535: both integer conversions are exact. Reuse
        // the already recovered private FloatGrid coordinate/bilinear policy;
        // no second generic grid implementation or container is reconstructed.
        let normalized = std::array::from_fn(|i| TerrainHeightGrid512 {
            encoded: Arc::from(
                encoded[i]
                    .iter()
                    .map(|v| u16::from(*v) * 257)
                    .collect::<Vec<_>>(),
            ),
            minimum_y: 0.0,
            maximum_y: 1.0,
            sampling_fix,
        });
        Ok(Self {
            encoded,
            sampling_fix,
            empty,
            normalized,
        })
    }
    pub fn encoded(&self) -> &[Arc<[u8]>; 4] {
        &self.encoded
    }
    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn sample_sum(&self, x: f32, z: f32) -> f32 {
        let values = self.normalized.each_ref().map(|grid| {
            let view = TerrainHeightGrid512 {
                sampling_fix: self.sampling_fix,
                ..grid.clone()
            };
            view.sample_at_world_xz(x, z)
        });
        clamp_unit(((values[0] + values[1]) + values[2]) + values[3])
    }
}

#[derive(Clone, Debug)]
pub struct PreparedWallPath {
    /// Public Curve3 +88/+90/+98. Consumers rebuild their own XZ Curve2.
    pub positions: Vec<[f32; 3]>,
    pub points_u: Vec<f32>,
    pub length: f32,
    /// Public Vec +c0/+c8/+d0; NOT the subsequent third array +f0/+f8/+100.
    pub segment_values: Vec<f32>,
}

/// The original producer attenuates path samples, smooths strengths and inserts
/// threshold crossings before consecutive deduplication and Curve3 rebuilding.
/// Measured all-zero resource bytes allow an equivalent empty branch. Relative
/// heights remain required because the original threshold masking reads them.
pub fn prepare_wall_path(
    mapped_positions: &[[f32; 3]],
    relative_heights: &[f32],
    observed_source_5c: f32,
    paths: &PathRasters512,
) -> Result<PreparedWallPath, PathPreparationError> {
    if mapped_positions.len() != relative_heights.len() {
        return Err(PathPreparationError::SourceArrayCount);
    }
    if !observed_source_5c.is_finite() {
        return Err(PathPreparationError::SourceScalar);
    }
    if mapped_positions.iter().flatten().any(|v| !v.is_finite()) {
        return Err(PathPreparationError::InvalidCurve);
    }
    if mapped_positions.len() < 2 {
        return Err(PathPreparationError::InvalidCurve);
    }
    let (positions, strengths) = if paths.is_empty() {
        let mut points = Vec::with_capacity(mapped_positions.len());
        for &p in mapped_positions {
            if points
                .last()
                .is_none_or(|previous: &[f32; 3]| *previous != p)
            {
                points.push(p);
            }
        }
        let strengths = vec![0.0; points.len()];
        (points, strengths)
    } else {
        prepare_nonempty(mapped_positions, relative_heights, paths)
    };
    if positions.len() < 2 {
        return Err(PathPreparationError::InvalidCurve);
    }
    let mut points_u = Vec::with_capacity(positions.len());
    let mut previous = positions[0];
    let mut length = 0.0;
    for &p in &positions {
        // glam is an existing dependency. Its finite dot/sqrt evaluation matches
        // the original Curve3 norm, verified against the untouched rebuild.
        length += (glam::Vec3::from_array(previous) - glam::Vec3::from_array(p)).length();
        points_u.push(length);
        previous = p;
    }
    if length == 0.0 || !length.is_finite() {
        return Err(PathPreparationError::InvalidCurve);
    }
    for u in &mut points_u {
        *u /= length;
    }
    let segment_values = strengths
        .into_iter()
        .map(|s| (1.0 - (-5.0 * s).exp()) * observed_source_5c)
        .collect();
    Ok(PreparedWallPath {
        positions,
        points_u,
        length,
        segment_values,
    })
}

fn clamp_unit(v: f32) -> f32 {
    let lo = if 0.0 > v { 0.0 } else { v };
    if 1.0 < lo { 1.0 } else { lo }
}
fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (glam::Vec3::from_array(a) - glam::Vec3::from_array(b)).length()
}
fn interpolate(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    std::array::from_fn(|i| b[i] * t + a[i] * (1.0 - t))
}
fn midpoint(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| b[i] * 0.5 + a[i] * 0.5)
}
fn append(points: &mut Vec<[f32; 3]>, strengths: &mut Vec<f32>, p: [f32; 3], s: f32) {
    if points.last().is_none_or(|old| *old != p) {
        points.push(p);
        strengths.push(s);
    }
}
fn prepare_nonempty(
    input: &[[f32; 3]],
    relative: &[f32],
    paths: &PathRasters512,
) -> (Vec<[f32; 3]>, Vec<f32>) {
    let mut sampled = Vec::with_capacity(input.len());
    let mut previous = [
        input[0][0] - (input[1][0] - input[0][0]),
        input[0][2] - (input[1][2] - input[0][2]),
    ];
    for p in input {
        let dx = previous[0] - p[0];
        let dz = previous[1] - p[2];
        let norm = 1.0 / (dz * dz + dx * dx).sqrt();
        let normal = [-dz * norm, dx * norm];
        let mut total = paths.sample_sum(p[0], p[2]);
        let mut denominator = 1.0;
        for layer in 0..4 {
            let t = (layer as f32 + 1.0) * 0.25;
            let weight = ((-t * t) + (-t * t)).exp();
            let offset = t * 0.5;
            let left = paths.sample_sum(p[0] + normal[0] * offset, p[2] + normal[1] * offset);
            let right = paths.sample_sum(p[0] - normal[0] * offset, p[2] - normal[1] * offset);
            let minimum = if right < left { right } else { left };
            total += minimum * (weight + weight);
            denominator += weight + weight;
        }
        sampled.push(total / denominator);
        previous = [p[0], p[2]];
    }
    let first_weight = f32::from_bits(0x3f1b4598);
    let second_weight = f32::from_bits(0x3e0a9555);
    let smooth: Vec<_> = (0..sampled.len())
        .map(|i| {
            let mut v = sampled[i];
            let mut d = 1.0;
            if i >= 1 {
                v += sampled[i - 1] * first_weight;
                d = f32::from_bits(0x3fcda2cc);
            }
            if i + 1 < sampled.len() {
                v += sampled[i + 1] * first_weight;
                d += first_weight;
            }
            if i >= 2 {
                v += sampled[i - 2] * second_weight;
                d += second_weight;
            }
            if i + 2 < sampled.len() {
                v += sampled[i + 2] * second_weight;
                d += second_weight;
            }
            v / d
        })
        .collect();
    let active: Vec<_> = smooth
        .iter()
        .zip(relative)
        .map(|(&s, &h)| if h > 0.5 { 0.0 } else { s })
        .collect();
    let threshold = f32::from_bits(0x3d4ccccd);
    let near_end = f32::from_bits(0x3f7fbe77);
    let minimum_distance = f32::from_bits(0x3dcccccd);
    let mut points = Vec::with_capacity(input.len());
    let mut strengths = Vec::with_capacity(input.len());
    let mut before = input[0];
    let mut old = active[0];
    for (&p, &s) in input.iter().zip(&active) {
        if old > threshold && s <= threshold {
            let t = (threshold - old) / (s - old);
            if t < near_end {
                let cross = interpolate(before, p, t);
                if distance(cross, before) > minimum_distance {
                    let mid = midpoint(before, cross);
                    append(
                        &mut points,
                        &mut strengths,
                        mid,
                        paths.sample_sum(mid[0], mid[2]),
                    );
                }
                append(&mut points, &mut strengths, cross, 0.0);
            }
            append(&mut points, &mut strengths, p, 0.0);
        } else if old <= threshold && s > threshold {
            let t = (threshold - old) / (s - old);
            if t < near_end {
                let cross = interpolate(before, p, t);
                append(&mut points, &mut strengths, cross, 0.0);
                if distance(cross, p) > minimum_distance {
                    let mid = midpoint(cross, p);
                    append(
                        &mut points,
                        &mut strengths,
                        mid,
                        paths.sample_sum(mid[0], mid[2]),
                    );
                }
            }
            append(&mut points, &mut strengths, p, s);
        } else if old > threshold && s > threshold {
            if (before[1] - p[1]).abs() > threshold {
                append(&mut points, &mut strengths, midpoint(before, p), s);
            }
            append(&mut points, &mut strengths, p, s);
        } else {
            append(&mut points, &mut strengths, p, 0.0);
        }
        before = p;
        old = s;
    }
    (points, strengths)
}
