//! Actual RecalculateWallHoles reader in the original wall publisher.
//! InnerWalls lookup uses standard owner APIs; duplicates remain ordered.
use crate::recovered_building::building_wall_state::WallId;
pub fn forward_recalculation(wall: WallId, authoritative_wall_exists: bool) -> Option<WallId> {
    authoritative_wall_exists.then_some(wall)
}
