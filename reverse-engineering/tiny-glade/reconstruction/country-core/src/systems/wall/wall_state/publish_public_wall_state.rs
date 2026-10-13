//! publish_public_wall_state.rs — publishes inner wall state to the public-wall map.
//!
//! Source SHA256 B skeleton; PDB `2d9686b8...`; procedure `publish_public_wall_state`
//! 0x140a8b530 (18515 bytes). FunctionSystem signature O.
//!
//! Business logic (O, decompiled entry + call chain):
//!   - Reads `InnerWalls` and synchronises changed/new inner walls into `PublicWalls`:
//!     `gather_lazy_update_sources` (closure$0) collects walls needing re-publication.
//!   - `process` (closures $1..$4) rebuilds the public wall record: source curve points,
//!     shape, height/terrain cache, `WallStyle::is_halftimbered`, `material_stats`,
//!     `SparseInstanceBuffer::allocate` for instanced walls, and maintenance of the
//!     `WallSpatialHash` segment membership.
//!   - Emits `OnWallChanged` / `OnWallDeleted` events consumed by calc_wall_bend,
//!     spatial_hash_update, delete_wall, structure consumers.
//!   - AnimateWallBulldozeBuffer membership is preserved across publication.
//!   - Debug-viz lines/spheres under a debug resource.
//!
//! The inner→public copy details and the spatial-hash feeding are typed gaps (WP11 state).

use bevy_ecs::event::EventReader;
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::{Component, Resource};
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use crate::resources::walls::inner_walls::InnerWalls;
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::WallId;
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// Published-change notification (O: consumed by calc_wall_bend, spatial_hash_update).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct OnWallChanged(pub WallId);

/// Published-delete notification (O: consumed by delete_wall, spatial_hash_update).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct OnWallDeleted(pub WallId);

/// Lazy publication sources collected by `gather_lazy_update_sources` (O).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LazyUpdateSource {
    pub wall: WallId,
    pub kind: u8,
}

/// `gather_lazy_update_sources` (O: closure$0 iterates inner walls).
pub fn gather_lazy_update_sources(
    inner: &InnerWalls,
    public: &PublicWalls,
) -> Vec<LazyUpdateSource> {
    inner
        .wall_ids()
        .into_iter()
        .map(|w| {
            let known = public.contains(w);
            LazyUpdateSource {
                wall: w,
                kind: if known { 1 } else { 0 },
            }
        })
        .collect()
}

/// O: full publish system.
pub fn publish_public_wall_state(
    mut inner_walls: ResMut<InnerWalls>,
    mut public_walls: ResMut<PublicWalls>,
    wall_spatial_hash: Res<WallSpatialHash>,
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut on_changed: EventWriter<OnWallChanged>,
    mut on_deleted: EventWriter<OnWallDeleted>,
) {
    let _ = (&mut inner_walls, &mut public_walls, &wall_spatial_hash, &terrain_heights);
    // collect sources
    let sources = gather_lazy_update_sources(&inner_walls, &public_walls);
    for source in sources {
        // process(): rebuild public record / emit events. Field mapping U -> typed gap.
        let _ = source;
        on_changed.write(OnWallChanged(source.wall));
    }
}
