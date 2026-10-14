//! wall_roof_hashmap business policies over standard HashMap, not hash probing.
use crate::recovered_building::building_wall_state::{
    PublicWallBuildingState, WallId, WallReadError,
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssociationError {
    InvalidWall(WallReadError),
    DuplicateRoof,
    InvalidShapeDuringPhase13,
    InvalidShapeDuringPhase14,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssociationAction {
    Insert,
    KeepDeleting,
    RequestDeletion,
}

/// Handles successful and missing public-wall lookups. Replacement
/// is allowed for a roof already marked deleting; duplicate live roofs fail.
pub fn association_action(
    wall: Option<&PublicWallBuildingState>,
    marked: bool,
    phase: u32,
    previous_present: bool,
) -> Result<AssociationAction, AssociationError> {
    if let Some(wall) = wall {
        wall.validate_read()
            .map_err(AssociationError::InvalidWall)?;
        if wall.shape().tag() < 2 {
            if !marked && previous_present {
                return Err(AssociationError::DuplicateRoof);
            }
            return Ok(AssociationAction::Insert);
        }
    }
    if marked {
        return Ok(AssociationAction::KeepDeleting);
    }
    match phase {
        13 => Err(AssociationError::InvalidShapeDuringPhase13),
        14 => Err(AssociationError::InvalidShapeDuringPhase14),
        _ => Ok(AssociationAction::RequestDeletion),
    }
}

pub struct ResolvedRoofAssociation<'a> {
    pub wall_id: WallId,
    pub entity: u64,
    pub marked: bool,
    pub public_wall: Option<&'a PublicWallBuildingState>,
}
#[derive(Clone, Debug, Default)]
pub struct RoofAssociations {
    entities: HashMap<WallId, u64>,
}
impl RoofAssociations {
    pub fn entity(&self, wall: WallId) -> Option<u64> {
        self.entities.get(&wall).copied()
    }
    pub fn has_roof(&self, wall: WallId) -> bool {
        self.entities.contains_key(&wall)
    }
    /// Consumes caller-resolved query rows. Actual Query order/filtering remains
    /// Bevy's responsibility. Source map iteration is not emulated.
    pub fn rebuild<'a>(
        &mut self,
        rows: impl IntoIterator<Item = ResolvedRoofAssociation<'a>>,
        public_wall_order: &[WallId],
        phase: u32,
    ) -> Result<AssociationChanges, AssociationError> {
        let mut entities = HashMap::new();
        let mut invalid_roof_requests = Vec::new();
        for row in rows {
            let action = association_action(
                row.public_wall,
                row.marked,
                phase,
                entities.contains_key(&row.wall_id),
            )?;
            match action {
                AssociationAction::Insert => {
                    entities.insert(row.wall_id, row.entity);
                }
                AssociationAction::KeepDeleting => {}
                AssociationAction::RequestDeletion => invalid_roof_requests.push(row.wall_id),
            }
        }
        let wall_presence_changes = public_wall_order
            .iter()
            .copied()
            .filter(|id| self.has_roof(*id) != entities.contains_key(id))
            .collect();
        self.entities = entities;
        Ok(AssociationChanges {
            invalid_roof_requests,
            wall_presence_changes,
        })
    }
}
#[derive(Clone, Debug)]
pub struct AssociationChanges {
    pub invalid_roof_requests: Vec<WallId>,
    pub wall_presence_changes: Vec<WallId>,
}
