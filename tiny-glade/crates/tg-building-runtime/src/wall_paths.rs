//! Actual path-raster resource -> original prepared wall curve owner.
use crate::{PublicWalls, WallCollectionState, WallSpatialDirty};
use bevy_ecs::prelude::*;
use country_core::recovered_building::{
    building_wall_path_preparation::{
        PathPreparationError, PathRasters512, PreparedWallPath, prepare_wall_path,
    },
    building_wall_state::WallId,
};
use std::collections::HashMap;

#[derive(Event, Clone)]
pub struct SetWallPathRasters(pub PathRasters512);
#[derive(Clone, Debug)]
pub struct PreparedWallRecord {
    pub path: PreparedWallPath,
    pub publication_revision: u64,
    pub raster_revision: u64,
    pub generation: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PendingWallPathReason {
    MissingPathResources,
    OriginalBranch(PathPreparationError),
}
#[derive(Resource, Default)]
pub struct WallPreparedPaths {
    pub records: HashMap<WallId, PreparedWallRecord>,
    pub pending: HashMap<WallId, PendingWallPathReason>,
    rasters: Option<PathRasters512>,
    raster_revision: u64,
}
impl WallPreparedPaths {
    pub fn current(&self, wall: WallId, publication_revision: u64) -> Option<&PreparedWallRecord> {
        self.records.get(&wall).filter(|record| {
            record.publication_revision == publication_revision
                && record.raster_revision == self.raster_revision
        })
    }
}
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallPreparedPaths>()
        .add_event::<SetWallPathRasters>()
        .add_systems(
            bevy_app::Update,
            prepare.in_set(crate::BuildingStage::PathPreparation),
        );
}
fn prepare(
    mut input: EventReader<SetWallPathRasters>,
    collection: Res<WallCollectionState>,
    walls: Res<PublicWalls>,
    mut prepared: ResMut<WallPreparedPaths>,
    mut dirty: EventWriter<WallSpatialDirty>,
) {
    for event in input.read() {
        prepared.rasters = Some(event.0.clone());
        prepared.raster_revision = prepared.raster_revision.wrapping_add(1);
    }
    prepared
        .records
        .retain(|id, _| collection.raw_sources.contains_key(id) && walls.walls.contains_key(id));
    prepared
        .pending
        .retain(|id, _| collection.raw_sources.contains_key(id) && walls.walls.contains_key(id));
    let mut ids: Vec<_> = collection.raw_sources.keys().copied().collect();
    ids.sort_unstable_by_key(|id| id.0);
    for id in ids {
        let Some(wall) = walls.walls.get(&id) else {
            continue;
        };
        if prepared.current(id, wall.publication_revision).is_some() {
            continue;
        }
        let Some(rasters) = prepared.rasters.as_ref() else {
            prepared.records.remove(&id);
            prepared
                .pending
                .insert(id, PendingWallPathReason::MissingPathResources);
            continue;
        };
        let relative: Vec<_> = wall
            .source
            .outline
            .iter()
            .zip(&wall.source.terrain_y)
            .map(|(point, y)| point[1] - y)
            .collect();
        let result = prepare_wall_path(
            &wall.source.outline,
            &relative,
            collection.raw_sources[&id].observed_path_scalar,
            rasters,
        );
        let old_generation = prepared
            .records
            .get(&id)
            .map_or(0, |record| record.generation);
        match result {
            Ok(path) => {
                let raster_revision = prepared.raster_revision;
                prepared.records.insert(
                    id,
                    PreparedWallRecord {
                        path,
                        publication_revision: wall.publication_revision,
                        raster_revision,
                        generation: old_generation.wrapping_add(1),
                    },
                );
                prepared.pending.remove(&id);
                dirty.write(WallSpatialDirty(id));
            }
            Err(error) => {
                if prepared.records.remove(&id).is_some() {
                    dirty.write(WallSpatialDirty(id));
                }
                prepared
                    .pending
                    .insert(id, PendingWallPathReason::OriginalBranch(error));
            }
        }
    }
}
