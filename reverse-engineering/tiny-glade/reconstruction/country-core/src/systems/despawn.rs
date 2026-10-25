//! `systems/despawn.rs` — despawn all glade entities (level teardown).
//!
//! PDB: `despawn_glade_entities @0x1408b3510` (386 B).
//!
//! O: two Query branches (flag @+0x108); for each entity-id slot (stride 0xa0
//! or 0x48 arrays), issues `EntityCommands::despawn` for each stored entity,
//! stopping early when a slot's high word is 0.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::Gap;

/// `despawn_glade_entities` @0x1408b3510 (O).
pub fn despawn_glade_entities(mut _entities: Query<&Gap>, _commands: Commands) {
    todo!("recover despawn_glade_entities (entity-id arrays → despawn)")
}
