//! InnerWalls -> collected CPU geometry -> regular height cache, inside core.
//! This implements the recovered source/collection portion of B 0x140a8b530;
//! GPU allocation, full PublicWalls layout and terrain-dirty routing stay outside
//! this bounded consumer. No successful GPU state is manufactured.

use bevy_ecs::prelude::{Res, ResMut, Resource};
use hashbrown::{HashMap, HashSet};

use crate::recovered_building::{
    wall_cache::{RegularWallHeightCache, regular_wall_height_cache},
    wall_collection::{
        CollectedWallPublication, CollectionError, TerrainHeightGrid512,
        collect_raw_wall_publication,
    },
};
use crate::resources::walls::{InnerWalls, WallId, WallState};

/// Explicit CPU terrain input; it must be supplied by the terrain owner.
#[derive(Resource)]
pub struct WallPublicationTerrain(pub TerrainHeightGrid512);

pub struct PublishedWallSource {
    pub authority: WallState,
    pub geometry: CollectedWallPublication,
    /// Freehand/roof-modified heights require their separate recovered consumer.
    pub regular_heights: Option<RegularWallHeightCache>,
}

#[derive(Default)]
pub struct WallPublicationDelta {
    pub changed: HashSet<WallId>,
    pub removed: HashSet<WallId>,
}

/// CPU output resource, deliberately separate from the incomplete native
/// PublicWalls/instancing state. Failed rebuilds are visible to readers.
#[derive(Resource, Default)]
pub struct WallPublicationState {
    records: HashMap<WallId, PublishedWallSource>,
    failures: HashMap<WallId, CollectionError>,
    pub last_delta: WallPublicationDelta,
}

impl WallPublicationState {
    pub fn get(&self, id: WallId) -> Result<Option<&PublishedWallSource>, &CollectionError> {
        if let Some(error) = self.failures.get(&id) {
            return Err(error);
        }
        Ok(self.records.get(&id))
    }

    pub fn failures(&self) -> &HashMap<WallId, CollectionError> {
        &self.failures
    }

    /// Consume separate authority notices. The current source determines whether
    /// an ID survives a create/remove/explicit-restore batch. Iteration order is
    /// the dependency's set order; no original ordered event stream is claimed.
    pub fn consume(&mut self, inner: &mut InnerWalls, terrain: &TerrainHeightGrid512) {
        let changes = inner.take_changes();
        self.last_delta = WallPublicationDelta::default();
        let candidates: HashSet<_> = changes
            .created
            .into_iter()
            .chain(changes.updated)
            .chain(changes.removed)
            .collect();
        for id in candidates {
            let Some(authority) = inner.get(id) else {
                self.records.remove(&id);
                self.failures.remove(&id);
                self.last_delta.removed.insert(id);
                continue;
            };
            match collect_raw_wall_publication(authority.source(), terrain) {
                Ok(geometry) => {
                    let regular_heights = regular_wall_height_cache(
                        &geometry.outline,
                        &authority.source().shape,
                        geometry.metrics.constrained_extra,
                    )
                    .ok();
                    self.records.insert(
                        id,
                        PublishedWallSource {
                            authority: authority.clone(),
                            geometry,
                            regular_heights,
                        },
                    );
                    self.failures.remove(&id);
                    self.last_delta.changed.insert(id);
                }
                Err(error) => {
                    self.failures.insert(id, error);
                }
            }
        }
    }
}

/// Core ECS consumer of real resources; callers choose their schedule order.
pub fn publish_wall_sources(
    mut inner: ResMut<InnerWalls>,
    terrain: Res<WallPublicationTerrain>,
    mut published: ResMut<WallPublicationState>,
) {
    published.consume(&mut inner, &terrain.0);
}
