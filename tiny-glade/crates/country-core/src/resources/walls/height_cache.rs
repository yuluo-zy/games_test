//! B init_wall_height_and_arch_segment_height: regular circle/rectangle cache.
//! Positions are the published, resampled world outline, not raw control points.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::if_same_then_else)]
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Clone, Debug)]
pub struct RegularWallHeightCache {
    pub maximum_y: f32,
    pub arch_segment_heights: Vec<f32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheBuildError {
    UnsupportedShape,
}

/// Original 14078b865..14078bc96. Geometry containers/allocator are reused.
/// Gable profile modifications are a subsequent consumer; this is the base
/// cache required before Roof creation and spatial synchronization.
pub fn regular_wall_height_cache(
    positions: &[[f32; 3]],
    shape: &WallShapeSnapshot,
    constrained_extra: f32,
) -> Result<RegularWallHeightCache, CacheBuildError> {
    if shape.tag() >= 2 {
        return Err(CacheBuildError::UnsupportedShape);
    }
    let top = positions.iter().fold(-2.5f32, |old, p| {
        // LLVM maxnum: unordered inputs select the numeric operand, a tie
        // selects the previous value (including its signed zero).
        if old.is_nan() {
            p[1]
        } else if p[1] > old {
            p[1]
        } else {
            old
        }
    });
    let value = add(add(top, constrained_extra), f32::from_bits(0xbeeb851f));
    // MAXSS source operand 0x3f75c290 wins on unordered input and equality.
    let floor = f32::from_bits(0x3f75c290);
    let maximum_y = if value > floor { value } else { floor };
    Ok(RegularWallHeightCache {
        maximum_y,
        arch_segment_heights: vec![maximum_y; positions.len()],
    })
}
fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
impl RegularWallHeightCache {
    /// Both caches receive the same base maximum; only identified fields change.
    pub fn write_projection(&self, bytes: &mut [u8; 672]) -> PublicWallBuildingState {
        for offset in [0x108, 0x110] {
            bytes[offset..offset + 4].copy_from_slice(&1u32.to_le_bytes());
            bytes[offset + 4..offset + 8].copy_from_slice(&self.maximum_y.to_le_bytes());
        }
        PublicWallBuildingState::from_observed(*bytes)
    }
}

/// Final scalar store 14078c404..14078c42d. The preceding per-arch gable
/// profile remains separate; callers must not claim these heights include it.
pub fn gable_maximum(base_maximum: f32, roof: Option<&[u8; 88]>) -> f32 {
    let Some(roof) = roof else {
        return base_maximum;
    };
    let h = f32::from_le_bytes(roof[0x30..0x34].try_into().unwrap());
    let ridge = f32::from_le_bytes(roof[0x34..0x38].try_into().unwrap());
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) != 1
        || !crate::resources::roofs::roof_shape::is_gable_observed(h, ridge)
    {
        return base_maximum;
    }
    add(
        add(
            base_maximum,
            crate::resources::roofs::geometry::surface::height(roof),
        ),
        f32::from_bits(0x3eeb851f),
    )
}
