//! CPU ownership and real ECS publication consumers for B 0x140a8b530.
//! The ordinary pipeline consumes the named WorldRaster buffers and owns one
//! PublicWalls resource. The former isolated CPU API is a compatibility facade.

use bevy_ecs::{prelude::*, system::SystemParam};
use hashbrown::HashMap;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tabloid::Subscriber;

use crate::resources::walls::publication_source::CollectionError;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use crate::resources::{
    walls::{
        InnerWalls, WallId,
        public_walls::{
            OnWallChanged, OnWallDeleted, PublicWalls, WallPublicationDirty, WallPublicationEvent,
            WallSpatialHash,
        },
    },
    world_raster_defs::{Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster},
};
use crate::systems::wall::wall_state::path_preparation::PathRasters512;
use crate::utils::dirty_tiles::DirtyTiles;

pub use crate::resources::walls::public_walls::{
    PublicWallState as PublishedWallSource, WallPublicationDelta,
};

/// Derived snapshot of the authoritative TerrainHeightsData owner. Production
/// refreshes this view from that owner; isolated CPU callers may supply a view.
#[derive(Resource)]
pub struct WallPublicationTerrain(pub TerrainHeightGrid512);

impl WallPublicationTerrain {
    pub fn from_height_data(source: &crate::resources::terrain_height::TerrainHeightsData) -> Self {
        Self(TerrainHeightGrid512 {
            encoded: Arc::from(source.heights()),
            minimum_y: source.min_height(),
            maximum_y: source.max_height(),
            sampling_fix: utils::float_grid::CENTERED_PATH
                .load(std::sync::atomic::Ordering::Relaxed),
        })
    }
}

/// Safe storage adaptation for the original publisher's TerrainHeightsData
/// input. This cache never becomes another terrain editor or readback owner.
pub fn synchronize_wall_publication_terrain(
    source: Res<crate::resources::terrain_height::TerrainHeightsData>,
    mut view: ResMut<WallPublicationTerrain>,
) {
    let sampling_fix = utils::float_grid::CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed);
    if source.is_changed() || view.0.sampling_fix != sampling_fix {
        *view = WallPublicationTerrain::from_height_data(&source);
    }
}

/// Old direct CPU caller compatibility. It delegates all ownership/algorithms
/// to PublicWalls; new game consumers use PublicWalls itself.
#[derive(Resource, Default)]
pub struct WallPublicationState(PublicWalls);
impl Deref for WallPublicationState {
    type Target = PublicWalls;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for WallPublicationState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl WallPublicationState {
    pub fn get(&self, id: WallId) -> Result<Option<&PublishedWallSource>, &CollectionError> {
        if let Some(error) = self.0.failures().get(&id) {
            return Err(error);
        }
        Ok(self.0.get(id))
    }
    pub fn failures(&self) -> &HashMap<WallId, CollectionError> {
        self.0.failures()
    }
    pub fn consume(&mut self, inner: &mut InnerWalls, terrain: &TerrainHeightGrid512) {
        self.0.consume(inner, terrain);
    }
}

/// Original WallLazySubscriber: Path first, Terrain second. The producer's
/// wrapper @1408c630a/6322 subscribes the existing canonical owners in this
/// order; no independent publisher is introduced by this derived consumer.
#[derive(Resource)]
pub struct WallLazySubscriber {
    pub primary: Subscriber<DirtyTiles>,
    pub secondary: Subscriber<DirtyTiles>,
}
/// Compatibility name for the same resource identity.
pub use WallLazySubscriber as WallPublicationSubscriptions;

/// Original startup::startup_walls::once_startup_walls, merged here with its
/// actual publication consumer. run_unsafe@1408c61d0 inlines the complete body:
/// PathRaster.subscribe, TerrainHeightsData.publisher.subscribe, Commands insert.
/// add_game_startup_systems@140941110 installs it in the PreStartup chain.
pub fn once_startup_walls(
    mut terrain: ResMut<crate::resources::terrain_height::TerrainHeightsData>,
    mut path: ResMut<PathRaster>,
    mut commands: Commands,
) {
    let primary = path.subscribe();
    let secondary = terrain.subscribe();
    commands.insert_resource(WallLazySubscriber { primary, secondary });
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
    mut inner: ResMut<InnerWalls>,
    terrain: Res<WallPublicationTerrain>,
    rasters: WallPathRasterSources,
    mut cache: Local<WallPathRasterCache>,
    mut dirty: ResMut<WallPublicationDirty>,
    mut public: ResMut<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    mut changed: EventWriter<OnWallChanged>,
    mut deleted: EventWriter<OnWallDeleted>,
) {
    let sampling_fix = terrain.0.sampling_fix;
    if cache
        .paths
        .as_ref()
        .is_none_or(|p| p.sampling_fix != sampling_fix)
        || rasters.path.is_changed()
        || rasters.cobblestone1.is_changed()
        || rasters.cobblestone2.is_changed()
        || rasters.cobblestone3.is_changed()
    {
        cache.paths = Some(
            PathRasters512::from_encoded(
                [
                    Arc::from(rasters.path.current_buffer()),
                    Arc::from(rasters.cobblestone1.current_buffer()),
                    Arc::from(rasters.cobblestone2.current_buffer()),
                    Arc::from(rasters.cobblestone3.current_buffer()),
                ],
                sampling_fix,
            )
            .expect("all four path rasters have the declared 512x512 extent"),
        );
    }
    public.consume_with_paths(
        &mut inner,
        &terrain.0,
        cache.paths.as_ref(),
        &dirty,
        &spatial,
    );
    dirty.primary.clear();
    dirty.secondary.clear();
    for event in &public.last_delta.events {
        match *event {
            WallPublicationEvent::Changed(id) => {
                changed.write(OnWallChanged(id));
            }
            WallPublicationEvent::Deleted(id) => {
                deleted.write(OnWallDeleted(id));
            }
        }
    }
}

/// Preserved for existing isolated core CPU callers. The integrated pipeline
/// uses publish_public_wall_state and its real event resources instead.
pub fn publish_wall_sources(
    mut inner: ResMut<InnerWalls>,
    terrain: Res<WallPublicationTerrain>,
    mut published: ResMut<WallPublicationState>,
) {
    published.consume(&mut inner, &terrain.0);
}

#[cfg(test)]
mod subscription_tests {
    use super::*;
    use crate::resources::{terrain_height::TerrainHeightsData, world_raster::DirtyTilePublisher};
    use bevy_app::{App, PreStartup, Update};

    #[test]
    fn startup_subscribes_the_same_path_and_terrain_readback_owners() {
        let mut app = App::new();
        app.insert_resource(TerrainHeightsData::<
            crate::resources::terrain_height::TerrainHeightSsbo,
        >::new());
        app.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        app.init_resource::<WallPublicationDirty>();
        app.add_systems(PreStartup, once_startup_walls);
        app.add_systems(Update, collect_wall_publication_dirty);
        app.update();
        let mut pixels = vec![0u8; 512 * 512];
        pixels[0] = 7;
        let mut staged = vec![0u8; pixels.len()];
        {
            let mut path = app.world_mut().resource_mut::<PathRaster>();
            path.write_composite(Some(&pixels));
            path.copy_to_staging(0, &mut staged);
        }
        app.update();
        {
            let mut dirty = app.world_mut().resource_mut::<WallPublicationDirty>();
            assert!(!dirty.primary.is_empty());
            assert!(dirty.secondary.is_empty());
            dirty.primary.clear();
        }
        let mut heights = vec![0u16; 512 * 512];
        heights[0] = u16::MAX;
        app.world_mut()
            .resource_mut::<TerrainHeightsData>()
            .copy_from_gpu(&heights);
        app.update();
        let dirty = app.world().resource::<WallPublicationDirty>();
        assert!(dirty.primary.is_empty());
        assert!(!dirty.secondary.is_empty());
    }
}
