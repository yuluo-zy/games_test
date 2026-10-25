//! Test harness helpers: event collectors and stand-ins for systems outside this module.

use crate::types::*;
use bevy_ecs::prelude::*;

/// Every event of type `E` observed by [`collect`], in order.
#[derive(Resource)]
pub struct Seen<E: Event>(pub Vec<E>);

impl<E: Event> Default for Seen<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

pub fn collect<E: Event + Clone>(mut reader: EventReader<E>, mut seen: ResMut<Seen<E>>) {
    seen.0.extend(reader.read().cloned());
}

/// HARNESS stand-in for the (out-of-module) SpawnPrefabCmd consumer that creates PrefabInstanceData.
pub fn consume_spawns(mut reader: EventReader<SpawnPrefabCmd>, mut commands: Commands) {
    for s in reader.read() {
        commands
            .entity(Entity::from_bits(s.field_10_unknown))
            .insert(PrefabInstanceData::at(s.position));
    }
}
