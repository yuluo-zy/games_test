//! Canonical owner of each roof's dormer tile allocation.
//!
//! B generate_dormer_roofs@1407cb1a0 stores a WallId and the actual
//! SparseInstanceBufferAllocation<RoofTileSsbo> in this map. Removing an entry
//! queues its handle through the same sparse buffer's weak retirement link.
//! Default is the original empty map; this type does not generate geometry.
use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::render::sparse_instance_buffer::SparseInstanceBufferAllocation;
use crate::resources::roof_tile_ssbo::RoofTileSsbo;
use crate::resources::walls::WallId;

/// Roof wall identity to its sole, non-cloneable dormer tile allocation.
/// The key retains all64 bits of the canonical WallId.
#[derive(Resource, Debug, Default)]
pub struct DormerTiles {
    pub allocations: HashMap<WallId, SparseInstanceBufferAllocation<RoofTileSsbo>>,
}
