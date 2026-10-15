//! B add 0x1408fe280 and point-removal predicate 0x1412f54b1..15.
use super::ivy_storage::{IvyId, IvyPointId};
use bevy_ecs::prelude::Resource;
use glam::Vec3;
use perchance::PerchanceContext;
use std::collections::HashMap;

/// Native insert at 0x14094421c passes state [21,0], perchance seed 10.
/// This stream is distinct from IvyStorage's spawn/selection RNG.
#[derive(Resource, Clone)]
pub struct IvyLeafRng(pub PerchanceContext);
impl Default for IvyLeafRng {
    fn default() -> Self {
        Self(PerchanceContext::new(10))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IvyLeaf {
    pub point_a: IvyPointId,
    pub point_b: IvyPointId,
    pub ivy: IvyId,
    pub source_18: u64,
    pub position: Vec3,
    pub vector_2c: Vec3,
    pub branch_direction: Vec3,
    pub size: f32,
    pub branch_thickness: f32,
    pub growth: f32,
    pub growth_delay: f32,
    pub frame: u32,
    pub vector_58: Vec3,
}
#[derive(Clone, Copy, Debug)]
pub struct IvyLeafInput {
    pub position: Vec3,
    pub vector_2c: Vec3,
    pub vector_58: Vec3,
    pub branch_direction: Vec3,
    pub ivy: IvyId,
    pub point_a: IvyPointId,
    pub point_b: IvyPointId,
    pub size: f32,
    pub growth_delay: f32,
    pub frame: u32,
    pub source_18: u64,
}
#[derive(Resource, Clone, Debug, Default)]
pub struct IvyLeafStorage {
    leaves: HashMap<IvyId, Vec<IvyLeaf>>,
}
impl IvyLeafStorage {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, input: IvyLeafInput) {
        self.leaves.entry(input.ivy).or_default().push(IvyLeaf {
            point_a: input.point_a,
            point_b: input.point_b,
            ivy: input.ivy,
            source_18: input.source_18,
            position: input.position,
            vector_2c: input.vector_2c,
            branch_direction: input.branch_direction,
            size: input.size,
            branch_thickness: 0.0,
            growth: 0.0,
            growth_delay: input.growth_delay,
            frame: input.frame,
            vector_58: input.vector_58,
        });
    }
    pub fn get(&self, ivy: IvyId) -> Option<&[IvyLeaf]> {
        self.leaves.get(&ivy).map(Vec::as_slice)
    }
    pub fn all_leaves(&self) -> impl Iterator<Item = &IvyLeaf> {
        self.leaves.values().flatten()
    }
    pub fn leaf_count(&self) -> usize {
        self.leaves.values().map(Vec::len).sum()
    }
    pub fn all_leaves_mut(&mut self) -> impl Iterator<Item = &mut IvyLeaf> {
        self.leaves.values_mut().flatten()
    }
    /// Remove only leaves touching this point; preserve order and the ivy key.
    pub fn delete_affected_leaves(&mut self, ivy: IvyId, point: IvyPointId) {
        if let Some(leaves) = self.leaves.get_mut(&ivy) {
            leaves.retain(|leaf| leaf.point_a != point && leaf.point_b != point);
        }
    }
    /// Original inline uses map.get_mut then Vec::clear for one ivy. It does
    /// not clear the entire storage or remove the map entry.
    pub fn delete_all_leaves(&mut self, ivy: IvyId) {
        if let Some(leaves) = self.leaves.get_mut(&ivy) {
            leaves.clear();
        }
    }
}
