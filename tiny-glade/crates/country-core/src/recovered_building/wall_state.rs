//! Game fields read by B PublicWalls::get and update_roof_spatial.
//! This is a projection of observed game state, not a hashbrown/Bevy ABI.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::recovered_building::wall_height_rules::{HeightCacheError, cached_wall_max_y};

/// Serialized global wall ID. Its internal generation/ownership is not guessed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WallId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallReadError {
    InvalidValidationCount,
    NonpositiveCurveLength,
}

/// Original PublicWalls::get checks count at state+0x10 and source Curve2 length at +0x30.
/// The count's original field name is not established.
pub fn validate_public_wall_read(
    count: u64,
    source_curve_length: f32,
) -> Result<(), WallReadError> {
    if count < 2 {
        return Err(WallReadError::InvalidValidationCount);
    }
    if !(source_curve_length > 0.0) {
        return Err(WallReadError::NonpositiveCurveLength);
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct WallShapeSnapshot([u8; 28]);
impl WallShapeSnapshot {
    pub fn from_observed(bytes: [u8; 28]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 28] {
        &self.0
    }
    pub fn tag(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }

    /// Circle compares four floats, rectangle six. Ordered equality preserves
    /// existing signed-zero bits; a NaN forces replacement and notification.
    /// Unsupported wall shapes are skipped by this consumer, not reclassified.
    pub fn synchronize_roof(&self, roof: &mut [u8; 88]) -> bool {
        let tag = self.tag();
        if tag > 1 {
            return false;
        }
        let count = if tag == 0 { 4 } else { 6 };
        let same = u32::from_le_bytes(roof[8..12].try_into().unwrap()) == tag
            && (0..count).all(|i| {
                let source = 4 + i * 4;
                let target = 12 + i * 4;
                f32::from_le_bytes(self.0[source..source + 4].try_into().unwrap())
                    == f32::from_le_bytes(roof[target..target + 4].try_into().unwrap())
            });
        if !same {
            roof[8..36].copy_from_slice(&self.0);
        }
        !same
    }
}

/// PDB/call-site observation of B public-wall record size 0x2a0.
/// Unused fields stay opaque; this does not declare the original Rust type ABI.
#[derive(Clone, Debug)]
pub struct PublicWallBuildingState([u8; 0x2a0]);
impl PublicWallBuildingState {
    pub fn from_observed(bytes: [u8; 0x2a0]) -> Self {
        Self(bytes)
    }
    pub fn validate_read(&self) -> Result<(), WallReadError> {
        validate_public_wall_read(
            u64::from_le_bytes(self.0[0x10..0x18].try_into().unwrap()),
            self.float_at(0x30),
        )
    }
    pub fn source_point_count(&self) -> u64 {
        u64::from_le_bytes(self.0[0x10..0x18].try_into().unwrap())
    }
    pub fn source_curve_length(&self) -> f32 {
        self.float_at(0x30)
    }
    pub fn published_source_curve_length(&self) -> f32 {
        self.float_at(0x28c)
    }
    pub fn max_y(&self) -> Result<f32, HeightCacheError> {
        cached_wall_max_y(self.0[0x108], self.float_at(0x10c))
    }
    pub fn max_y_gable(&self) -> Result<f32, HeightCacheError> {
        crate::recovered_building::wall_height_rules::cached_gable_max_y(
            self.0[0x110],
            self.float_at(0x114),
        )
    }
    pub fn shape(&self) -> WallShapeSnapshot {
        WallShapeSnapshot::from_observed(self.0[0x258..0x274].try_into().unwrap())
    }
    /// Original WallStyle::is_halftimbered VA 0x1408c5670: byte >= 2.
    pub fn is_halftimbered(&self) -> bool {
        self.0[0x299] >= 2
    }
    /// B snapping/telegraph preview offset field @+0x284 (semantic 待取证).
    pub fn float_0x284(&self) -> f32 {
        self.float_at(0x284)
    }
    /// B snapped-flag byte copied by the snapping systems (+0x29e; semantic
    /// 待取证).
    pub fn byte_0x29e(&self) -> u8 {
        self.0[0x29e]
    }
    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
}
