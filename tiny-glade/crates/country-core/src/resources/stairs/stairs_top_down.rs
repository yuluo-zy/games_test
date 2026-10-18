//! Original optional top-down stair mesh consumed by foliage exclusion.
//! The current BufferHost owns the actual managed buffer. No default resource
//! is installed: the stair mesh producer inserts/removes this value.
use bevy_ecs::resource::Resource;
use crate::rhapsody_contract::ManagedHandle;

#[derive(Resource, Clone, Debug)]
pub struct StairsTopDownMesh {
    pub buffer: ManagedHandle,
    /// B foliage consumer reads the actual vertex count at resource+0x28.
    pub vertex_count: u32,
}
