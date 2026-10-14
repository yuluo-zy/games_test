//! Game-owned creation and animation values; dependency Vec/ECS storage excluded.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::derivable_impls)]
use crate::recovered_building::{
    building_roof_creation::{
        RoofCreateRequestObserved, RoofCreationError, RoofCreationOutcome, initialize_resolved_roof,
    },
    building_wall_state::PublicWallBuildingState,
    roof_state,
};

#[derive(Clone, Debug)]
pub struct RoofTileSimulation([u8; 24]);
impl RoofTileSimulation {
    pub fn from_observed(bytes: [u8; 24]) -> Self {
        Self(bytes)
    }
    /// Original initializes bytes 0..21; caller-owned padding stays unchanged.
    pub fn initialize(mut bytes: [u8; 24], max_y: f32) -> Self {
        bytes[..21].fill(0);
        bytes[4..8].copy_from_slice(&max_y.to_le_bytes());
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 24] {
        &self.0
    }
}

/// Semantic initialization result. Actual triangle/normal/UV element types and
/// their library container ABI are deliberately not fabricated.
#[derive(Clone, Copy, Debug, Default)]
pub struct EmptyRoofTriggerVolume;

#[derive(Clone, Debug)]
pub struct RoofAnimation([u8; 8]);
impl Default for RoofAnimation {
    fn default() -> Self {
        Self([0; 8])
    }
}
impl RoofAnimation {
    pub fn from_observed(bytes: [u8; 8]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 8] {
        &self.0
    }
    pub fn is_being_animated(&self) -> bool {
        self.value(0) != 1.0
    }
    pub fn deletion_finished(&self) -> bool {
        self.value(0) == 0.0
    }
    fn value(&self, offset: usize) -> f32 {
        f32::from_le_bytes(self.0[offset..offset + 4].try_into().unwrap())
    }
    /// Original update consumes resolved Roof, optional deletion marker and dt.
    /// Completion/marking remain separate from the actual EntityCommands call.
    pub fn advance(
        &mut self,
        roof: &[u8; 88],
        marked: bool,
        delta_time: f32,
        changed_tick: &mut u32,
        current_tick: u32,
    ) {
        let (primary, secondary) = if marked {
            (mul(delta_time, -4.0), mul(delta_time, -10.0))
        } else {
            let h = f32::from_le_bytes(roof[0x30..0x34].try_into().unwrap());
            let factor = if !roof_state::ty_observed(h) {
                2.0
            } else {
                1.0
            };
            let secondary = if self.value(0) > f32::from_bits(0x3f7d70a4) {
                mul(delta_time, f32::from_bits(0x40555555))
            } else {
                0.0
            };
            (
                mul(mul(delta_time, f32::from_bits(0x4036db6e)), factor),
                mul(secondary, factor),
            )
        };
        for (offset, delta) in [(0, primary), (4, secondary)] {
            if delta != 0.0 {
                *changed_tick = current_tick;
                let value = add(delta, self.value(offset));
                let nonnegative = if 0.0 > value { 0.0 } else { value };
                let bounded = if 1.0 < nonnegative { 1.0 } else { nonnegative };
                self.0[offset..offset + 4].copy_from_slice(&bounded.to_le_bytes());
            }
        }
    }
}
fn invalid(v: f32, a: f32, b: f32) -> f32 {
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
fn add(a: f32, b: f32) -> f32 {
    invalid(a + b, a, b)
}
fn mul(a: f32, b: f32) -> f32 {
    invalid(a * b, a, b)
}

#[derive(Clone, Debug)]
pub struct PreparedRoofGameplayComponents {
    pub roof: [u8; 88],
    pub tile_simulation: RoofTileSimulation,
    pub animation: RoofAnimation,
    pub empty_trigger_volume: EmptyRoofTriggerVolume,
    pub creation: RoofCreationOutcome,
}
/// Prepares confirmed gameplay components. Drawable/spire/support bundles and
/// entity spawn still belong to the next recovered game/engine adapter boundary.
pub fn prepare_resolved_roof_components(
    roof: [u8; 88],
    tile_bytes: [u8; 24],
    request: &RoofCreateRequestObserved,
    wall: &PublicWallBuildingState,
) -> Result<PreparedRoofGameplayComponents, RoofCreationError> {
    let mut roof = roof;
    let creation = initialize_resolved_roof(&mut roof, request, wall)?;
    let max_y = f32::from_le_bytes(roof[0x4c..0x50].try_into().unwrap());
    Ok(PreparedRoofGameplayComponents {
        roof,
        tile_simulation: RoofTileSimulation::initialize(tile_bytes, max_y),
        animation: RoofAnimation::default(),
        empty_trigger_volume: EmptyRoofTriggerVolume,
        creation,
    })
}

/// Setter deallocation strides: triangles and normals 36 bytes, UVs 24 bytes;
/// each setter asserts one entry per triangle. Standard Vec owns storage.
#[derive(Clone, Debug, Default)]
pub struct RoofTriggerVolume {
    triangles: Vec<[[f32; 3]; 3]>,
    normals: Vec<[[f32; 3]; 3]>,
    uvs: Vec<[[f32; 2]; 3]>,
}
impl RoofTriggerVolume {
    pub fn triangles(&self) -> &[[[f32; 3]; 3]] {
        &self.triangles
    }
    pub fn normals(&self) -> &[[[f32; 3]; 3]] {
        &self.normals
    }
    pub fn uvs(&self) -> &[[[f32; 2]; 3]] {
        &self.uvs
    }
    pub fn set_triangles(&mut self, triangles: Vec<[[f32; 3]; 3]>) {
        self.triangles = triangles;
    }
    pub fn set_normals(&mut self, normals: Vec<[[f32; 3]; 3]>) {
        self.normals = normals;
        assert_eq!(
            self.triangles.len(),
            self.normals.len(),
            "original normal count guard"
        );
    }
    pub fn set_uvs(&mut self, uvs: Vec<[[f32; 2]; 3]>) {
        self.uvs = uvs;
        assert_eq!(
            self.triangles.len(),
            self.uvs.len(),
            "original UV count guard"
        );
    }
}
