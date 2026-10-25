//! Synchronizes platform stair colliders, publishes dirty masks, and serves rays.
//! Queries cover platform geometry; callers select the downstream subscriptions.
use crate::{
    BuildingStage, RemoveWall,
    wall_platform_stairs::{
        WallCpuPlatformStairs, WallPlatformStairEntities, WallPlatformStairState,
    },
};
use bevy_ecs::prelude::*;
use country_core::utils::dirty_tiles::DirtyTileRect;
use country_core::utils::dirty_tiles::DirtyTiles64;
use country_core::systems::wall::construction::platform_collision::PlatformColliderUpdate;
use country_core::systems::wall::construction::platform_collision::PlatformStairColliderWorld;
use country_core::systems::wall::construction::platform_rapier_backend::PlatformColliderDiff;
use country_core::systems::wall::construction::platform_rapier_backend::PlatformRapierBackend;
use country_core::systems::wall::construction::platform_rapier_backend::PlatformRapierError;
use country_core::systems::wall::construction::platform_rapier_backend::PlatformRapierHandle;
use country_core::systems::wall::construction::platform_rapier_backend::PlatformRayHit;
use country_core::resources::walls::publication_fields::WallId;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

#[derive(Event, Clone)]
pub struct SetPlatformCollisionDirtyTiles {
    pub rows: [u64; 64],
}
#[derive(Event, Clone, Copy)]
pub struct ClearPlatformCollisionDirtyTiles;
/// Clears the dirty mask while retaining initialized grid ownership.
#[derive(Event, Clone, Copy)]
pub struct ResetPlatformCollisionDirtyTiles;
#[derive(Event, Clone, Debug)]
pub struct PlatformCollisionDirtyTilesPublished {
    pub subworld: u8,
    pub rows: [u64; 64],
}
#[derive(Clone, Debug)]
pub struct PlatformCollisionRasterWrite {
    pub world_rect: [f32; 4],
    pub cells: DirtyTileRect,
}
#[derive(Default)]
pub struct PlatformCollisionRasterState {
    pub tiles: Option<DirtyTiles64>,
    /// Real dependency-produced old/new AABBs awaiting an explicit grid owner.
    pub queued_world_rects: Vec<[f32; 4]>,
    pub rasterized: Vec<PlatformCollisionRasterWrite>,
    pub revision: u64,
    pub needs_publication: bool,
    pub published_rows: Vec<[u64; 64]>,
}
impl PlatformCollisionRasterState {
    fn consume_rect(&mut self, rect: [f32; 4]) {
        let Some(tiles) = self.tiles.as_mut() else {
            self.queued_world_rects.push(rect);
            return;
        };
        let cells = tiles.rasterize_world_rect(rect);
        self.rasterized.push(PlatformCollisionRasterWrite {
            world_rect: rect,
            cells,
        });
        self.revision = self.revision.wrapping_add(1);
        self.needs_publication = true;
    }
    fn set_rows(&mut self, rows: [u64; 64]) {
        self.tiles = Some(DirtyTiles64::from_rows(rows));
        for rect in std::mem::take(&mut self.queued_world_rects) {
            self.consume_rect(rect);
        }
        self.revision = self.revision.wrapping_add(1);
    }
}
#[derive(Clone, Debug)]
pub enum PlatformCollisionPending {
    MissingDirtyTileAuthority,
    StairsSourcePending,
    BackendInvariant(PlatformRapierError),
}
#[derive(Clone, Debug)]
pub struct PlatformCollisionChange {
    pub wall: WallId,
    pub update: PlatformColliderUpdate<PlatformRapierHandle>,
}
type DirtyRasterSink = Box<dyn FnMut([f32; 4]) + Send + Sync>;
type LivePlatformWorld = PlatformStairColliderWorld<PlatformRapierBackend<DirtyRasterSink>>;
#[derive(Resource)]
pub struct WallPlatformCollision {
    pub world: LivePlatformWorld,
    pub raster: Arc<Mutex<PlatformCollisionRasterState>>,
    pub pending: HashMap<WallId, PlatformCollisionPending>,
    pub changes: Vec<PlatformCollisionChange>,
    source_generations: HashMap<WallId, u64>,
}
impl Default for WallPlatformCollision {
    fn default() -> Self {
        let raster = Arc::new(Mutex::new(PlatformCollisionRasterState::default()));
        let sink = raster.clone();
        let sink: DirtyRasterSink = Box::new(move |rect| {
            sink.lock()
                .expect("platform DirtyTiles owner poisoned")
                .consume_rect(rect);
        });
        Self {
            world: PlatformStairColliderWorld::new(PlatformRapierBackend::new(sink)),
            raster,
            pending: HashMap::new(),
            changes: Vec::new(),
            source_generations: HashMap::new(),
        }
    }
}
#[derive(Event, Clone)]
pub struct PlatformColliderChanged(pub PlatformColliderDiff);
#[derive(Event, Clone, Copy)]
pub struct QueryPlatformRay {
    pub request_id: u64,
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub maximum_time_of_impact: f32,
    pub solid: bool,
}
#[derive(Clone, Debug)]
pub enum PlatformRayOutcome {
    Hit(PlatformRayHit),
    Miss,
    Pending(PlatformCollisionPending),
}
#[derive(Event, Clone, Debug)]
pub struct PlatformRayResult {
    pub request_id: u64,
    pub outcome: PlatformRayOutcome,
}
/// Selects which collision subworld publications invalidate walls.
/// Until supplied, dirty publications remain queued.
#[derive(Event, Clone, Copy)]
pub struct SetPlatformCollisionWallSubscription {
    pub subworld_mask: u32,
}
#[derive(Resource, Default)]
pub struct PlatformCollisionWallSubscription {
    pub subworld_mask: Option<u32>,
    pub pending_subworlds: Vec<u8>,
    queued: HashMap<u8, DirtyTiles64>,
}

/// Register after PlatformStairs (including ApplyDeferred), before WallStructures.
/// Parent plugin registers this module in its PlatformCollision stage.
pub fn register(app: &mut bevy_app::App) {
    app.init_resource::<WallPlatformCollision>()
        .add_event::<SetPlatformCollisionDirtyTiles>()
        .add_event::<ClearPlatformCollisionDirtyTiles>()
        .add_event::<ResetPlatformCollisionDirtyTiles>()
        .add_event::<PlatformCollisionDirtyTilesPublished>()
        .init_resource::<PlatformCollisionWallSubscription>()
        .add_event::<SetPlatformCollisionWallSubscription>()
        .add_event::<PlatformColliderChanged>()
        .add_event::<QueryPlatformRay>()
        .add_event::<PlatformRayResult>()
        .add_systems(
            bevy_app::Update,
            (configure, sync, flush, publish_wall_updates, queries)
                .chain()
                .in_set(BuildingStage::PlatformCollision),
        );
}
fn configure(
    mut sets: EventReader<SetPlatformCollisionDirtyTiles>,
    mut clears: EventReader<ClearPlatformCollisionDirtyTiles>,
    mut resets: EventReader<ResetPlatformCollisionDirtyTiles>,
    collision: Res<WallPlatformCollision>,
) {
    let mut raster = collision
        .raster
        .lock()
        .expect("platform DirtyTiles owner poisoned");
    raster.rasterized.clear();
    raster.published_rows.clear();
    for event in sets.read() {
        raster.set_rows(event.rows);
    }
    if resets.read().count() != 0 {
        if let Some(tiles) = raster.tiles.as_mut() {
            tiles.clear();
        }
        raster.needs_publication = false;
        raster.revision = raster.revision.wrapping_add(1);
    }
    if clears.read().count() != 0 {
        raster.tiles = None;
        raster.revision = raster.revision.wrapping_add(1);
    }
}
fn sync(
    stairs: Res<WallPlatformStairState>,
    entities: Res<WallPlatformStairEntities>,
    geometry: Query<&WallCpuPlatformStairs>,
    mut removed: EventReader<RemoveWall>,
    mut collision: ResMut<WallPlatformCollision>,
    mut report: ResMut<crate::BuildingReport>,
) {
    collision.pending.clear();
    collision.changes.clear();
    let deleted: HashSet<_> = removed.read().map(|e| e.0).collect();
    let authority = collision
        .raster
        .lock()
        .expect("platform DirtyTiles owner poisoned")
        .tiles
        .is_some();
    for wall in &stairs.pending {
        if !deleted.contains(wall) {
            collision
                .pending
                .insert(*wall, PlatformCollisionPending::StairsSourcePending);
        }
    }
    let mut ready = HashSet::new();
    let mut ids: Vec<_> = entities.0.keys().copied().collect();
    ids.sort_unstable_by_key(|w| w.0);
    for wall in ids {
        if deleted.contains(&wall) {
            continue;
        }
        if stairs.pending.contains(&wall) {
            collision
                .pending
                .insert(wall, PlatformCollisionPending::StairsSourcePending);
            continue;
        }
        let Some(cpu) = entities.0.get(&wall).and_then(|e| geometry.get(*e).ok()) else {
            continue;
        };
        if !authority {
            collision
                .pending
                .insert(wall, PlatformCollisionPending::MissingDirtyTileAuthority);
            continue;
        }
        ready.insert(wall);
        if collision.source_generations.get(&wall) == Some(&cpu.generation) {
            continue;
        }
        match collision.world.update(wall, &cpu.geometry) {
            Ok(update) => {
                collision.source_generations.insert(wall, cpu.generation);
                collision
                    .changes
                    .push(PlatformCollisionChange { wall, update });
            }
            Err(error) => {
                ready.remove(&wall);
                collision
                    .pending
                    .insert(wall, PlatformCollisionPending::BackendInvariant(error));
                report
                    .faults
                    .push(format!("Platform collider {}: {error:?}", wall.0));
            }
        }
    }
    let mut stale: Vec<_> = collision
        .world
        .colliders
        .keys()
        .filter(|wall| !ready.contains(wall))
        .copied()
        .collect();
    stale.sort_unstable_by_key(|w| w.0);
    for wall in stale {
        match collision.world.remove(wall) {
            Ok(update) => {
                collision
                    .changes
                    .push(PlatformCollisionChange { wall, update });
                collision.source_generations.remove(&wall);
            }
            Err(error) => {
                collision
                    .pending
                    .insert(wall, PlatformCollisionPending::BackendInvariant(error));
                report
                    .faults
                    .push(format!("Platform collider cleanup {}: {error:?}", wall.0));
            }
        }
    }
    collision
        .source_generations
        .retain(|wall, _| ready.contains(wall));
}
fn flush(
    mut collision: ResMut<WallPlatformCollision>,
    mut changed: EventWriter<PlatformColliderChanged>,
    mut tiles_published: EventWriter<PlatformCollisionDirtyTilesPublished>,
) {
    // Publish and clear the dirty mask before refreshing queries, so downstream
    // consumers observe the same completed collider update.
    let dirty = collision.world.backend.dirty;
    {
        let mut raster = collision
            .raster
            .lock()
            .expect("platform DirtyTiles owner poisoned");
        if (dirty || raster.needs_publication)
            && let Some(tiles) = raster.tiles.as_mut()
        {
            let snapshot = *tiles.take_and_clear().rows();
            raster.published_rows.push(snapshot);
            tiles_published.write(PlatformCollisionDirtyTilesPublished {
                subworld: 7,
                rows: snapshot,
            });
            raster.needs_publication = false;
        }
    }
    collision.world.backend.flush_queries();
    for diff in std::mem::take(&mut collision.world.backend.diffs) {
        changed.write(PlatformColliderChanged(diff));
    }
}

fn queries(
    collision: Res<WallPlatformCollision>,
    mut requests: EventReader<QueryPlatformRay>,
    mut results: EventWriter<PlatformRayResult>,
) {
    let authority = collision
        .raster
        .lock()
        .expect("platform DirtyTiles owner poisoned")
        .tiles
        .is_some();
    for query in requests.read() {
        let outcome = if !authority {
            PlatformRayOutcome::Pending(PlatformCollisionPending::MissingDirtyTileAuthority)
        } else if let Some((_, pending)) = collision.pending.iter().min_by_key(|(wall, _)| wall.0) {
            // A miss is not authoritative while required geometry or a stale
            // collider invariant remains unresolved in this query subworld.
            PlatformRayOutcome::Pending(pending.clone())
        } else {
            match collision.world.backend.cast_ray(
                query.origin,
                query.direction,
                query.maximum_time_of_impact,
                query.solid,
            ) {
                Some(hit) => PlatformRayOutcome::Hit(hit),
                None => PlatformRayOutcome::Miss,
            }
        };
        results.write(PlatformRayResult {
            request_id: query.request_id,
            outcome,
        });
    }
}

/// Publishes spatial invalidation candidates for subscribed subworlds.
/// Collision intersections still come from the physics query backend.
fn publish_wall_updates(
    mut published: EventReader<PlatformCollisionDirtyTilesPublished>,
    mut selects: EventReader<SetPlatformCollisionWallSubscription>,
    mut subscription: ResMut<PlatformCollisionWallSubscription>,
    mut wall_dirty: EventWriter<crate::TerrainDirtyTiles>,
) {
    for select in selects.read() {
        subscription.subworld_mask = Some(select.subworld_mask);
    }
    for event in published.read() {
        let mask = DirtyTiles64::from_rows(event.rows);
        subscription
            .queued
            .entry(event.subworld)
            .or_insert_with(DirtyTiles64::new_clear)
            .extend(&mask);
    }
    subscription.pending_subworlds.clear();
    let Some(selection) = subscription.subworld_mask else {
        subscription.pending_subworlds = subscription.queued.keys().copied().collect();
        subscription.pending_subworlds.sort_unstable();
        return;
    };
    let mut combined = DirtyTiles64::new_clear();
    for (subworld, mask) in std::mem::take(&mut subscription.queued) {
        if subworld < 17 && selection & (1u32 << subworld) != 0 {
            combined.extend(&mask);
        }
    }
    if combined.rows().iter().any(|row| *row != 0) {
        wall_dirty.write(crate::TerrainDirtyTiles {
            subscription: country_core::resources::walls::publication_source::DirtyPublicationSubscription::Primary,
            mask: *combined.rows(),
            // Primary subscriptions do not use the secondary lazy-dirty filter.
            already_lazy_dirty: HashSet::new(),
        });
    }
}
