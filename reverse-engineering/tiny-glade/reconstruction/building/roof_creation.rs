//! B create_roof: game initialization after successful public-wall resolution.
//! ECS spawn, deferred commands and subsequent animation/drawable bundles are
//! outside this recovered initialization range, never handwritten here.
use crate::{
    building_wall_state::{PublicWallBuildingState, WallId, WallReadError},
    roof_replay::ObservedReplayEvent,
    roof_state,
    wall_height_rules::HeightCacheError,
};

#[derive(Clone, Debug)]
pub struct RoofCreateRequestObserved([u8; 64]);
impl RoofCreateRequestObserved {
    /// Original create event record, including the ignored queue-message prefix.
    pub fn from_observed(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
    pub fn from_replay_event(event: &ObservedReplayEvent) -> Option<Self> {
        if !matches!(event, ObservedReplayEvent::Create { .. }) {
            return None;
        }
        let mut record = [0; 64];
        event.write_observed_fields(&mut record);
        Some(Self(record))
    }
    pub fn wall_id(&self) -> WallId {
        WallId(u64::from_le_bytes(self.0[8..16].try_into().unwrap()))
    }
    pub fn record(&self) -> &[u8; 64] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoofCreationError {
    WallRead(WallReadError),
    Height(HeightCacheError),
    UnsupportedWallShape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoofCreationOutcome {
    pub pitched: bool,
    /// Original create range emits a wall-ID notification only when gabled.
    pub gable_notification: Option<WallId>,
}

/// Connects the recovered create replay event and public-wall game state to
/// Roof::new. Appearance/style fields are copied, without guessing defaults.
pub fn initialize_resolved_roof(
    roof: &mut [u8; 88],
    request: &RoofCreateRequestObserved,
    wall: &PublicWallBuildingState,
) -> Result<RoofCreationOutcome, RoofCreationError> {
    wall.validate_read().map_err(RoofCreationError::WallRead)?;
    let shape = wall.shape();
    if shape.tag() > 1 {
        return Err(RoofCreationError::UnsupportedWallShape);
    }
    let max_y = wall.max_y().map_err(RoofCreationError::Height)?;
    let record = request.record();
    roof_state::new_observed(
        roof,
        record[8..16].try_into().unwrap(),
        record[32..60].try_into().unwrap(),
        shape.bytes(),
        &max_y.to_le_bytes(),
        record[16..20].try_into().unwrap(),
        u8::from(wall.is_halftimbered()),
        record[20..32].try_into().unwrap(),
    );
    let height_01 = f32::from_le_bytes(roof[0x30..0x34].try_into().unwrap());
    let ridge_01 = f32::from_le_bytes(roof[0x34..0x38].try_into().unwrap());
    Ok(RoofCreationOutcome {
        pitched: roof_state::ty_observed(height_01),
        gable_notification: roof_state::is_gable_observed(height_01, ridge_01)
            .then_some(request.wall_id()),
    })
}
