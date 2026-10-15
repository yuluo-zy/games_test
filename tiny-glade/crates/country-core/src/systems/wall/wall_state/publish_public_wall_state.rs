//! CPU ownership and real ECS publication consumers for B 0x140a8b530.
//! The ordinary pipeline consumes the named WorldRaster buffers and owns one
//! PublicWalls resource. The former isolated CPU API is a compatibility facade.

use std::{ops::{Deref, DerefMut}, sync::Arc};
use bevy_ecs::{prelude::*, system::SystemParam};
use hashbrown::HashMap;
use tabloid::Subscriber;

use crate::recovered_building::{
    wall_collection::{CollectionError, TerrainHeightGrid512},
    wall_path_preparation::PathRasters512,
};
use crate::resources::{
    walls::{InnerWalls, WallId, public_walls::{PublicWalls, OnWallChanged, OnWallDeleted, WallPublicationDirty, WallPublicationEvent, WallSpatialHash}},
    world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster},
};
use crate::utils::dirty_tiles::DirtyTiles;

pub use crate::resources::walls::public_walls::{PublicWallState as PublishedWallSource, WallPublicationDelta};

/// Terrain's current encoded height buffer. No zero-height fallback exists.
#[derive(Resource)]
pub struct WallPublicationTerrain(pub TerrainHeightGrid512);

/// Old direct CPU caller compatibility. It delegates all ownership/algorithms
/// to PublicWalls; new game consumers use PublicWalls itself.
#[derive(Resource, Default)]
pub struct WallPublicationState(PublicWalls);
impl Deref for WallPublicationState {
    type Target = PublicWalls;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for WallPublicationState {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
impl WallPublicationState {
    pub fn get(&self, id: WallId) -> Result<Option<&PublishedWallSource>, &CollectionError> {
        if let Some(error) = self.0.failures().get(&id) { return Err(error); }
        Ok(self.0.get(id))
    }
    pub fn failures(&self) -> &HashMap<WallId, CollectionError> { self.0.failures() }
    pub fn consume(&mut self, inner: &mut InnerWalls, terrain: &TerrainHeightGrid512) {
        self.0.consume(inner, terrain);
    }
}

/// Original two independent tabloid cursors. The caller binds the actual
/// publisher owners; their still-unresolved original field names are not guessed.
#[derive(Resource)]
pub struct WallPublicationSubscriptions {
    pub primary: Subscriber<DirtyTiles>,
    pub secondary: Subscriber<DirtyTiles>,
}

pub fn collect_wall_publication_dirty(
    mut subscriptions: ResMut<WallPublicationSubscriptions>,
    mut dirty: ResMut<WallPublicationDirty>,
) {
    if let Some(batch) = subscriptions.primary.take() {
        dirty.primary.extend(&batch);
    }
    if let Some(batch) = subscriptions.secondary.take() {
        dirty.secondary.extend(&batch);
    }
}

#[derive(SystemParam)]
pub struct WallPathRasterSources<'w> {
    path: Res<'w, PathRaster>,
    cobblestone1: Res<'w, Cobblestone1Raster>,
    cobblestone2: Res<'w, Cobblestone2Raster>,
    cobblestone3: Res<'w, Cobblestone3Raster>,
}

/// Cached conversions of actual raster bytes. This is derived storage, not an
/// independently editable replacement path resource.
#[derive(Default)]
pub struct WallPathRasterCache {
    paths: Option<PathRasters512>,
}

pub fn publish_public_wall_state(
    mut inner: ResMut<InnerWalls>, terrain: Res<WallPublicationTerrain>,
    rasters: WallPathRasterSources, mut cache: Local<WallPathRasterCache>,
    mut dirty: ResMut<WallPublicationDirty>, mut public: ResMut<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    mut changed: EventWriter<OnWallChanged>, mut deleted: EventWriter<OnWallDeleted>,
) {
    let sampling_fix = terrain.0.sampling_fix;
    if cache.paths.as_ref().is_none_or(|p| p.sampling_fix != sampling_fix)
        || rasters.path.is_changed() || rasters.cobblestone1.is_changed()
        || rasters.cobblestone2.is_changed() || rasters.cobblestone3.is_changed() {
        cache.paths = Some(PathRasters512::from_encoded([
            Arc::from(rasters.path.current_buffer()),
            Arc::from(rasters.cobblestone1.current_buffer()),
            Arc::from(rasters.cobblestone2.current_buffer()),
            Arc::from(rasters.cobblestone3.current_buffer()),
        ], sampling_fix).expect("all four path rasters have the declared 512x512 extent"));
    }
    public.consume_with_paths(&mut inner, &terrain.0, cache.paths.as_ref(), &dirty, &spatial);
    dirty.primary.clear();
    dirty.secondary.clear();
    for event in &public.last_delta.events {
        match *event {
            WallPublicationEvent::Changed(id) => { changed.write(OnWallChanged(id)); }
            WallPublicationEvent::Deleted(id) => { deleted.write(OnWallDeleted(id)); }
        }
    }
}

/// Preserved for existing isolated core CPU callers. The integrated pipeline
/// uses publish_public_wall_state and its real event resources instead.
pub fn publish_wall_sources(
    mut inner: ResMut<InnerWalls>, terrain: Res<WallPublicationTerrain>,
    mut published: ResMut<WallPublicationState>,
) {
    published.consume(&mut inner, &terrain.0);
}
