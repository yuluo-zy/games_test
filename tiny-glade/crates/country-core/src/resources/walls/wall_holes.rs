//! Actual hole ownership. Geometric finalization/hash policy is shared with the
//! native-validated game leaf; no duplicate approximation of that algorithm.
use super::WallId;
pub use crate::recovered_building::wall_holes::{
    FinalizedWallHole, HoleStorageError, HoleStoragePhase, PreviousWallHoles, RawWallHole,
    WallHoleStorage,
};
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

#[derive(Resource, Default, Debug)]
pub struct WallHoles {
    pub storage: HashMap<WallId, WallHoleStorage>,
}
#[derive(Resource, Default, Debug)]
pub struct PrevWallHoles {
    pub storage: HashMap<WallId, PreviousWallHoles>,
}

impl PrevWallHoles {
    pub fn wall_holes(&self, wall: WallId) -> Option<&PreviousWallHoles> {
        self.storage.get(&wall)
    }
}
impl WallHoles {
    pub fn add(&mut self, wall: WallId, hole: RawWallHole) -> Result<(), HoleStorageError> {
        self.storage.entry(wall).or_default().add(hole)
    }
    /// InProgressHoleStorage::extend stops at the original kind-5 terminator.
    pub fn extend(
        &mut self,
        wall: WallId,
        holes: impl IntoIterator<Item = RawWallHole>,
    ) -> Result<(), HoleStorageError> {
        for hole in holes {
            if hole.kind == 5 {
                break;
            }
            self.add(wall, hole)?;
        }
        Ok(())
    }
    /// Finalization retains the native checksum reuse policy, including its
    /// cached length/closed values when no hole geometry changed.
    pub fn finalize(
        &mut self,
        wall: WallId,
        length: f32,
        closed: bool,
    ) -> Result<bool, HoleStorageError> {
        self.storage
            .entry(wall)
            .or_default()
            .finalize(length, closed)
    }
    pub fn clear_for_next_frame(&mut self, previous: &mut PrevWallHoles) {
        previous.storage.clear();
        for (&wall, storage) in &mut self.storage {
            if let Some(snapshot) = storage.advance_frame() {
                previous.storage.insert(wall, snapshot);
            }
        }
    }
}
