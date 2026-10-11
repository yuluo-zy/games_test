//! B RoofShapeParams field mapping from original serialization instructions.
//! See evidence/multi-version/B-parameter-consumers and B-parameter-findings.md.
//! This adapter preserves bytes whose semantics/defaults are not established.
use tg_history::{RidgeDirection, RoofParameters};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoofParameterBlock([u8; 28]);

impl RoofParameterBlock {
    pub fn from_bytes(bytes: [u8; 28]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 28] {
        &self.0
    }
    pub fn from_roof(roof: &[u8; 88]) -> Self {
        Self(roof[0x24..0x40].try_into().unwrap())
    }
    pub fn write_to_roof(&self, roof: &mut [u8; 88]) {
        roof[0x24..0x40].copy_from_slice(&self.0);
    }

    /// Only confirmed serialized fields are replaced. Flat rim and padding remain.
    /// Missing rim defaults cannot be invented from older sample documents.
    pub fn replace_serialized_fields(&mut self, params: &RoofParameters) {
        for (offset, value) in [
            (0, params.tip_offset_01[0]),
            (4, params.tip_offset_01[1]),
            (8, params.profile_01),
            (12, params.height_01),
            (16, params.ridge_length_01),
            (20, params.eave_length_01),
        ] {
            self.0[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        self.0[24] = match params.ridge_dir {
            RidgeDirection::Width => 0,
            RidgeDirection::Length => 1,
        };
    }

    pub fn direction(&self) -> Option<RidgeDirection> {
        match self.0[24] {
            0 => Some(RidgeDirection::Width),
            1 => Some(RidgeDirection::Length),
            _ => None,
        }
    }

    pub fn profile_01(&self) -> f32 {
        self.float_at(8)
    }
    pub fn height_01(&self) -> f32 {
        self.float_at(12)
    }
    pub fn eave_length_01(&self) -> f32 {
        self.float_at(20)
    }
    fn float_at(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }

    /// RoofShapeParams::eave_length_ws, original B VA 0x1408e3c30, 62 bytes.
    /// Both zero terms and operation order are kept from original SSE lanes.
    pub fn eave_length_ws(&self) -> f32 {
        let eave = self.eave_length_01();
        let profile_root = self.profile_01().sqrt();
        let scaled_eave = 1.5 * eave;
        let x = scaled_eave + (1.0 - eave) * 0.0;
        // Legacy MOVSS xmm1,xmm2 preserves xmm1's sqrt(profile) upper lane.
        let y = profile_root + (1.0 - profile_root) * 0.5;
        y * x
    }
}
