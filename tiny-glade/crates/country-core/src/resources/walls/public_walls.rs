//! Wall-to-entity lookup consumed by the B wall highlighter (0x140a91e00).
//! This resource is separate from PublicWalls and its full publication state.
use bevy_ecs::{entity::Entity, resource::Resource};
use hashbrown::HashMap;

use super::WallId;

#[derive(Resource, Default)]
pub struct WallEntities(pub HashMap<WallId, Entity>);

impl WallEntities {
    pub fn wall_entity(&self, wall: WallId) -> Option<Entity> {
        self.0.get(&wall).copied()
    }
}
