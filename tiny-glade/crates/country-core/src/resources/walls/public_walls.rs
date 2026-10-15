//! Public wall ownership, associations and publication notifications.
//! B's maps use hashbrown; their native layout is left to that dependency.
use bevy_ecs::{entity::Entity, event::Event, resource::Resource};
use hashbrown::{HashMap, HashSet};

use super::{InnerWalls, WallId};
use crate::recovered_building::{
    wall_collection::{
        CollectionError, DirtyPublicationSubscription, TerrainHeightGrid512, dirty_tile_candidate,
        terrain_requires_republication,
    },
    wall_path_preparation::{PathPreparationError, PathRasters512},
};
use crate::utils::dirty_tiles::DirtyTiles;

pub use super::public_wall_state::PublicWallState;
pub use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OnWallChanged(pub WallId);
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OnWallDeleted(pub WallId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallPublicationEvent {
    Changed(WallId),
    Deleted(WallId),
}

#[derive(Default, Debug)]
pub struct WallPublicationDelta {
    pub changed: HashSet<WallId>,
    pub removed: HashSet<WallId>,
    /// Preserves deleted-before-created-before-updated order, including repeated
    /// changed notices when an ID appears in both authority source sets.
    pub events: Vec<WallPublicationEvent>,
}

/// The two independent original subscriptions are intentionally kept distinct.
/// The producer must provide the real dirty tiles; clear does not manufacture
/// an all-dirty terrain update.
#[derive(Resource, Default, Debug)]
pub struct WallPublicationDirty {
    pub primary: DirtyTiles,
    pub secondary: DirtyTiles,
}

#[derive(Resource, Default)]
pub struct PublicWalls {
    records: HashMap<WallId, PublicWallState>,
    roofs: HashMap<WallId, Entity>,
    failures: HashMap<WallId, CollectionError>,
    path_failures: HashMap<WallId, PathPreparationError>,
    pub last_delta: WallPublicationDelta,
}

impl PublicWalls {
    /// B 0x140a91ef0: a present row must have a valid simplified Curve2.
    pub fn get(&self, wall: WallId) -> Option<&PublicWallState> {
        self.records
            .get(&wall)
            .inspect(|state| state.assert_valid())
    }
    /// B 0x140a92010 is an unchecked *validation* lookup, still returning None
    /// for a missing ID. The old chapter's description as unwrap was incorrect.
    pub fn get_unchecked(&self, wall: WallId) -> Option<&PublicWallState> {
        self.records.get(&wall)
    }
    pub fn get_mut(&mut self, wall: WallId) -> Option<&mut PublicWallState> {
        let state = self.records.get_mut(&wall)?;
        state.assert_valid();
        Some(state)
    }
    pub fn all(&self) -> impl Iterator<Item = (WallId, &PublicWallState)> {
        self.records.iter().map(|(&id, state)| (id, state))
    }
    pub fn wall_ids(&self) -> impl Iterator<Item = WallId> + '_ {
        self.records.keys().copied()
    }
    pub fn has_roof(&self, wall: WallId) -> bool {
        self.roofs.contains_key(&wall)
    }
    pub fn roof_entity(&self, wall: WallId) -> Option<Entity> {
        self.roofs.get(&wall).copied()
    }
    pub fn associate_roof(&mut self, wall: WallId, entity: Entity) -> Option<Entity> {
        self.roofs.insert(wall, entity)
    }
    pub fn remove_roof(&mut self, wall: WallId) -> Option<Entity> {
        self.roofs.remove(&wall)
    }
    pub fn replace_roof_entities(&mut self, roofs: HashMap<WallId, Entity>) -> Vec<WallId> {
        let changes = self
            .wall_ids()
            .filter(|id| self.roofs.contains_key(id) != roofs.contains_key(id))
            .collect();
        self.roofs = roofs;
        changes
    }
    pub fn failures(&self) -> &HashMap<WallId, CollectionError> {
        &self.failures
    }
    pub fn path_failures(&self) -> &HashMap<WallId, PathPreparationError> {
        &self.path_failures
    }

    pub fn consume(&mut self, inner: &mut InnerWalls, terrain: &TerrainHeightGrid512) {
        self.consume_with_paths(
            inner,
            terrain,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
    }

    /// B 0x140a8b530 processes removal, creation and editing sets separately.
    /// 0x140b772b0 is hashbrown::RawTable::remove_entry, not an animation test.
    /// The original invalid-source guard skips rebuilding and keeps any old row.
    pub fn consume_with_paths(
        &mut self,
        inner: &mut InnerWalls,
        terrain: &TerrainHeightGrid512,
        paths: Option<&PathRasters512>,
        dirty: &WallPublicationDirty,
        spatial: &WallSpatialHash,
    ) {
        let mut changes = inner.take_changes();
        self.last_delta = WallPublicationDelta::default();
        for id in spatial.candidates(&dirty.primary) {
            if let Some(state) = self.get(id) {
                if dirty_tile_candidate(
                    DirtyPublicationSubscription::Primary,
                    state.geometry.metrics.minimum_relative_height,
                    changes.updated.contains(&id),
                ) {
                    changes.updated.insert(id);
                }
            }
        }
        for id in spatial.candidates(&dirty.secondary) {
            if let Some(state) = self.get(id) {
                if dirty_tile_candidate(
                    DirtyPublicationSubscription::Secondary,
                    state.geometry.metrics.minimum_relative_height,
                    changes.updated.contains(&id),
                ) && terrain_requires_republication(
                    &state.geometry.outline,
                    state.authority.elevation(),
                    terrain,
                ) {
                    changes.updated.insert(id);
                }
            }
        }
        for id in changes.removed {
            self.failures.remove(&id);
            self.path_failures.remove(&id);
            if self.records.remove(&id).is_some() {
                self.last_delta.removed.insert(id);
                self.last_delta
                    .events
                    .push(WallPublicationEvent::Deleted(id));
            }
        }
        for (ids, adding) in [(changes.created, true), (changes.updated, false)] {
            for id in ids {
                let Some(authority) = inner.get(id) else {
                    continue;
                };
                let source = authority.source();
                if source.curve.points.len() < 2 || source.curve.length <= 0.0 {
                    self.failures.insert(id, CollectionError::InvalidSource);
                    continue;
                }
                // Preserve the original duplicate-created side effects: changed
                // is emitted and the old public row removed before the assertion.
                if adding && self.records.contains_key(&id) {
                    self.last_delta.changed.insert(id);
                    self.last_delta
                        .events
                        .push(WallPublicationEvent::Changed(id));
                    self.records.remove(&id);
                    assert!(!adding, "assertion failed: !is_adding");
                }
                let mut record = match PublicWallState::collect(authority, terrain) {
                    Ok(record) => record,
                    Err(error) => {
                        self.failures.insert(id, error);
                        continue;
                    }
                };
                self.path_failures.remove(&id);
                if let Some(paths) = paths {
                    if let Err(error) = record.prepare_paths(paths) {
                        self.path_failures.insert(id, error);
                    }
                }
                self.records.insert(id, record);
                self.failures.remove(&id);
                self.last_delta.changed.insert(id);
                self.last_delta
                    .events
                    .push(WallPublicationEvent::Changed(id));
            }
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct WallEntities(pub HashMap<WallId, Entity>);

impl WallEntities {
    pub fn wall_entity(&self, wall: WallId) -> Option<Entity> {
        self.0.get(&wall).copied()
    }
}

/// Original delete_wall owns and drains every plaster child of the deleted wall.
#[derive(Resource, Default, Debug)]
pub struct WallPlasters(pub HashMap<WallId, Vec<Option<Entity>>>);
