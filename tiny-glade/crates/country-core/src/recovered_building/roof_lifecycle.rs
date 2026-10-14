//! Links recovered creation, committed deletion marks, animation and removal.
//! Library command flushes are explicit boundaries, not an invented scheduler.
use crate::recovered_building::{
    building_roof_components::{PreparedRoofGameplayComponents, prepare_resolved_roof_components},
    building_roof_creation::{RoofCreateRequestObserved, RoofCreationError},
    building_roof_deletion::{RoofDeletionMark, match_deletion_request, should_despawn},
    building_wall_state::{PublicWallBuildingState, WallId},
};

#[derive(Clone, Debug)]
pub struct ResolvedRoofLifecycle {
    /// Supplied by the engine; no local entity-generation algorithm.
    pub entity: u64,
    pub components: PreparedRoofGameplayComponents,
    pub marked_for_deletion: bool,
    pub animation_tick: u32,
}
impl ResolvedRoofLifecycle {
    pub fn prepare(
        entity: u64,
        roof: [u8; 88],
        tile_padding: [u8; 24],
        request: &RoofCreateRequestObserved,
        wall: &PublicWallBuildingState,
    ) -> Result<Self, RoofCreationError> {
        Ok(Self {
            entity,
            components: prepare_resolved_roof_components(roof, tile_padding, request, wall)?,
            marked_for_deletion: false,
            animation_tick: 0,
        })
    }
    pub fn consume_delete_requests(&self, requests: &[WallId]) -> Option<RoofDeletionMark> {
        match_deletion_request(requests, self.entity, &self.components.roof)
    }
    /// Called after the engine commits insertion of RoofMarkedForDeletion.
    pub fn deletion_mark_committed(&mut self) {
        self.marked_for_deletion = true;
    }
    /// Returns the original removal intent; actual despawn and association
    /// refresh must use the engine's command/update order.
    pub fn advance(&mut self, dt: f32, current_tick: u32) -> Option<u64> {
        self.components.animation.advance(
            &self.components.roof,
            self.marked_for_deletion,
            dt,
            &mut self.animation_tick,
            current_tick,
        );
        should_despawn(self.marked_for_deletion, &self.components.animation).then_some(self.entity)
    }
}
