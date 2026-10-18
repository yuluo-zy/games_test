//! One ECS owner for the already recovered tg-terrain stroke map/ID policy.
//! The original startup initializer is separate; explicit state is required.
use bevy_ecs::resource::Resource;
use std::ops::{Deref, DerefMut};
pub use tg_terrain::{schema::StrokeId, stroke::TerrainStroke};

#[derive(Resource, Clone, Debug)]
pub struct TerrainEditingData(pub tg_terrain::stroke::TerrainEditingData);
impl Deref for TerrainEditingData {
    type Target = tg_terrain::stroke::TerrainEditingData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TerrainEditingData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TerrainEditingData {
    /// B W3 borrows the actual stroke map and the same resource's +0x40 flag.
    /// Calling through ResMut preserves its original unconditional change tick.
    pub fn rasterization_parts(
        &mut self,
    ) -> (&std::collections::HashMap<StrokeId, TerrainStroke>, &mut bool) {
        (&self.0.strokes, &mut self.0.rasterized)
    }
}
