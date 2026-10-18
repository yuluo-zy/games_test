//! The two live decorator collider owners used by B replay_pop_bucket.
//! DecoratorPlugin @140343a3c inserts the two empty maps; later collision
//! producers populate these exact maps and the shared RaycastWorld.
use super::decorator::DecoratorId;
use crate::systems::collision::world::ColliderMeshInstance;
use bevy_ecs::resource::Resource;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct DecoratorPhysicsColliders {
    pub first: HashMap<DecoratorId, ColliderMeshInstance>,
    pub second: HashMap<DecoratorId, ColliderMeshInstance>,
}
