//! Game-owned raw wall publication collection (B publisher/Curve remap).
//! Containers, float operations and curve ownership reuse existing libraries.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::recovered_building::{
    building_wall_publication::{WallPublicationMetrics, publication_metrics},
    building_wall_state::WallShapeSnapshot,
    surface::{Curve2, CurveError},
};
use std::sync::Arc;

/// Original terrain field is a 512 × 512 u16 float grid, normalized to the
/// separately stored height bounds. The sampling-fix flag is an explicit input:
/// its original DLL-owned global default has not been inferred.
#[derive(Clone, Debug)]
pub struct TerrainHeightGrid512 {
    pub encoded: Arc<[u16]>,
    pub minimum_y: f32,
    pub maximum_y: f32,
    pub sampling_fix: bool,
}
impl TerrainHeightGrid512 {
    pub fn sample_at_world_xz(&self, x: f32, z: f32) -> f32 {
        assert_eq!(
            self.encoded.len(),
            512 * 512,
            "original terrain grid extent"
        );
        let coord = |world: f32| {
            let u = world / 130.0 + 0.5;
            if self.sampling_fix {
                // MINSS source 1 wins on NaN; MAXSS source 0 wins on NaN.
                let u = if u < 1.0 { u } else { 1.0 };
                let p = u * 512.0 - 0.5;
                let p = if p > 0.0 { p } else { 0.0 };
                let index = p as u32;
                (
                    index.min(511) as usize,
                    index.saturating_add(1).min(511) as usize,
                    p - index as f32,
                )
            } else {
                let u = if u.is_nan() || u < 0.0 {
                    0.0
                } else if u > 1.0 {
                    1.0
                } else {
                    u
                };
                // Legacy policy multiplies by 512 and clamps base index to
                // 510; at the upper boundary its fraction can reach 2.
                let p = u * 512.0;
                let index = (p as u32).min(510);
                (index as usize, index as usize + 1, p - index as f32)
            }
        };
        let (x0, x1, tx) = coord(x);
        let (z0, z1, tz) = coord(z);
        let decode = |x: usize, z: usize| {
            let u = self.encoded[z * 512 + x] as f32 / 65535.0;
            u * self.maximum_y + (1.0 - u) * self.minimum_y
        };
        // Original utils::lerp, not an alternate bilinear evaluation order.
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        lerp(
            lerp(decode(x0, z0), decode(x0, z1), tz),
            lerp(decode(x1, z0), decode(x1, z1), tz),
            tx,
        )
    }
}

#[derive(Clone, Debug)]
pub struct RawWallPublicationSource {
    /// Source points, normalized parameters and length are the original 2D
    /// Curve representation, rather than a pre-resampled world outline.
    pub curve: Curve2,
    /// Publisher passes `tag == 1` to force an odd point count per segment.
    /// This controls segmentation; it does not close or deduplicate the curve.
    pub curve_kind_tag: u32,
    /// Original source +0x58. Positive values form a terrain lower bound;
    /// otherwise the lower bound is the original f32 -2.7 constant.
    pub terrain_floor: f32,
    pub shape: WallShapeSnapshot,
    pub style: u8,
    pub extra: f32,
    pub special: bool,
    /// Authored scalar consumed by the path-intersection preparation owner.
    /// Its original field name and creation default remain unestablished.
    pub observed_path_scalar: f32,
    /// Authored ClutterVersion; it selects actual construction policy.
    pub clutter_version: u8,
    /// Original source flag copied by publication; its game name is unknown.
    pub observed_source_flag_64: u8,
}
#[derive(Clone, Debug)]
pub struct CollectedWallPublication {
    pub outline: Arc<Vec<[f32; 3]>>,
    pub outline_u: Vec<f32>,
    pub mapped_curve_length: f32,
    /// PublicWallState B +0x28c retains raw 2D length, not mapped 3D length.
    pub original_curve_length: f32,
    pub terrain_y: Vec<f32>,
    pub relative_heights: Vec<f32>,
    pub metrics: WallPublicationMetrics,
}
#[derive(Debug, PartialEq)]
pub enum CollectionError {
    InvalidSource,
    Curve(CurveError),
    ExcessiveSampleCount,
}

/// B 14146bb70's game sampling policy; source segment endpoints are preserved,
/// including a repeated closing point. No closing edge is fabricated.
pub fn collect_raw_wall_publication(
    source: &RawWallPublicationSource,
    terrain: &TerrainHeightGrid512,
) -> Result<CollectedWallPublication, CollectionError> {
    if source.curve.points.len() < 2 || !(source.curve.length > 0.0) {
        return Err(CollectionError::InvalidSource);
    }
    let floor = if source.terrain_floor > 0.0 {
        source.terrain_floor
    } else {
        f32::from_bits(0xc02ccccd)
    };
    let mut outline = Vec::with_capacity(source.curve.points.len());
    let map = |p: [f32; 2]| {
        let y = terrain.sample_at_world_xz(p[0], p[1]);
        let y = if y.is_nan() || floor > y { floor } else { y };
        [p[0], y, p[1]]
    };
    outline.push(map(source.curve.points[0]));
    for segment in source.curve.points.windows(2) {
        let [a, b] = [segment[0], segment[1]];
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let length = (dz * dz + dx * dx).sqrt();
        let epsilon = f32::from_bits(0x322bcc77);
        let length = if length > epsilon { length } else { epsilon };
        let step = 0.5 / length;
        let step = if step < 1.0 { step } else { 1.0 };
        let n = (1.0 / step).round() as u64;
        let mut count = n.wrapping_add(1).max(2);
        if source.curve_kind_tag == 1 && count & 1 == 0 {
            count |= 1;
        }
        // The original allocator cannot materialize impossible sizes either.
        // Reject before conversion/allocation instead of hanging the runtime.
        if count > isize::MAX as u64 / 12 || !length.is_finite() {
            return Err(CollectionError::ExcessiveSampleCount);
        }
        for j in 1..count {
            let t = j as f32 / (count as f32 - 1.0);
            outline.push(map([
                b[0] * t + a[0] * (1.0 - t),
                b[1] * t + a[1] * (1.0 - t),
            ]));
        }
    }
    // Original try_new_from_points< Vec3 > calculates 3D arc length and U.
    let mut mapped_curve_length = 0.0;
    let mut outline_u = Vec::with_capacity(outline.len());
    let mut previous = outline[0];
    for p in &outline {
        let x = previous[0] - p[0];
        let y = previous[1] - p[1];
        let z = previous[2] - p[2];
        mapped_curve_length += (z * z + (y * y + x * x)).sqrt();
        outline_u.push(mapped_curve_length);
        previous = *p;
    }
    if mapped_curve_length == 0.0 || !mapped_curve_length.is_finite() {
        return Err(CollectionError::Curve(CurveError::NonFiniteLength));
    }
    for u in &mut outline_u {
        *u /= mapped_curve_length;
    }
    let terrain_y: Vec<_> = outline
        .iter()
        .map(|p| terrain.sample_at_world_xz(p[0], p[2]))
        .collect();
    let relative_heights: Vec<_> = outline
        .iter()
        .zip(&terrain_y)
        .map(|(p, y)| p[1] - y)
        .collect();
    let metrics = publication_metrics(&relative_heights, &outline, source.extra, source.special);
    Ok(CollectedWallPublication {
        outline: Arc::new(outline),
        outline_u,
        mapped_curve_length,
        original_curve_length: source.curve.length,
        terrain_y,
        relative_heights,
        metrics,
    })
}

/// B publisher 140a8c0e5..140a8c13e: a terrain-dirty candidate only republishes
/// when the mapped outline actually changes under the original floor policy.
pub fn terrain_requires_republication(
    outline: &[[f32; 3]],
    source_floor: f32,
    terrain: &TerrainHeightGrid512,
) -> bool {
    outline.iter().any(|p| {
        let sampled = terrain.sample_at_world_xz(p[0], p[2]);
        if p[1] > source_floor {
            sampled != p[1]
        } else {
            sampled > p[1]
        }
    })
}

/// Separate original tabloid subscribers; their source field names remain
/// unresolved. Primary routes low-relative walls directly to updates; secondary
/// builds the candidates later checked with terrain_requires_republication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirtyPublicationSubscription {
    Primary,
    Secondary,
}
pub fn dirty_tile_candidate(
    subscription: DirtyPublicationSubscription,
    minimum_relative_height: f32,
    already_queued_lazy: bool,
) -> bool {
    match subscription {
        DirtyPublicationSubscription::Primary => minimum_relative_height <= 0.5,
        DirtyPublicationSubscription::Secondary => {
            minimum_relative_height <= 0.5 || !already_queued_lazy
        }
    }
}
