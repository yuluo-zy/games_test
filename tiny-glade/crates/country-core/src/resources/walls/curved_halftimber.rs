//! Original clear_canvas140a7b2da/2e6 clears two48B hash maps in order.
//! Callee140aaae80 and remove_wall_id140aa9380 prove both exact key/value types.
//! Field names describe their storage order; full geometry producer is separate.
use bevy_ecs::{component::Component,resource::Resource,entity::Entity};
use std::collections::HashMap;
use super::WallId;

/// 原半木实体携带的颜色键与直接索引标志；非零标志按原路径直接查色。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurvedHalfTimber {
    pub key: u64,
    pub direct: u8,
}

#[derive(Resource,Clone,Debug,Default)]
pub struct CurvedHalfTimberDrawables {
    pub primary:HashMap<WallId,Vec<Entity>>,
    pub secondary:HashMap<WallId,Vec<Entity>>,
}
impl CurvedHalfTimberDrawables {
    /// Retain table allocations/hash state; drop every actual owned entity list.
    /// This original clear does not queue despawns; entity lifecycle consumers
    /// remain responsible for the native remove_wall_id+Commands policy.
    pub fn clear(&mut self) {self.primary.clear();self.secondary.clear();}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canvas_clear_drops_nonempty_lists_keeps_table_capacity_and_never_despawns() {
        let mut world=bevy_ecs::world::World::new();
        let first=world.spawn_empty().id();let second=world.spawn_empty().id();
        let mut owner=CurvedHalfTimberDrawables::default();
        owner.primary.insert(WallId(7),vec![first,second]);
        owner.secondary.insert(WallId(19),vec![second]);
        let capacity=(owner.primary.capacity(),owner.secondary.capacity());
        owner.clear();
        assert!(owner.primary.is_empty());assert!(owner.secondary.is_empty());
        assert_eq!((owner.primary.capacity(),owner.secondary.capacity()),capacity);
        assert!(world.get_entity(first).is_ok());assert!(world.get_entity(second).is_ok());
    }
}
