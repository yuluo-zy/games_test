//! delete_wall.rs — removes all resources/entities owned by a deleted wall.
//!
//! Source SHA256 B skeleton; PDB `2d9686b8...`; procedure `delete_wall` 0x140aac1e0 (1095 B).
//! FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x140aac1e0):
//!   - Consumes `OnWallDeleted` events.
//!   - For each wall: removes the wall entry from two `WallId -> Entity` maps
//!     (`WallEntities` and the wall-published entity map), despawns those entities via
//!     `Commands::despawn`, and removes the `WallId -> Vec<Entity>` plaster map entry,
//!     despawn-then-drain its entities.
//!   - Loop continues while the reader has events.

use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, ResMut};

use crate::resources::walls::public_walls::{PublicWalls, WallEntities, WallPlasters};
use crate::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted;

pub fn delete_wall(
    mut reader: EventReader<OnWallDeleted>,
    mut public_walls: ResMut<PublicWalls>,
    mut wall_plasters: ResMut<WallPlasters>,
    mut wall_entities: ResMut<WallEntities>,
    mut commands: Commands,
) {
    for OnWallDeleted(wall) in reader.read() {
        // O: remove WallId -> Entity maps and despawn.
        if let Some(entity) = wall_entities.map.remove(wall) {
            commands.entity(entity).despawn();
        }
        if let Some(entity) = public_walls.wall_entities_map.remove(wall) {
            commands.entity(entity).despawn();
        }
        // O: remove WallId -> Vec<Entity> plasters and despawn each.
        if let Some(plasters) = wall_plasters.map.remove(wall) {
            for entity in plasters {
                commands.entity(entity).despawn();
            }
        }
        // keep inner wall record removal out of the public cascade (owner separation).
        let _ = &mut public_walls;
    }
}
