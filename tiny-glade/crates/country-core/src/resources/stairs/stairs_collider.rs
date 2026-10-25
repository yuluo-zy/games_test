//! `stairs_collider.rs` — 楼梯碰撞体注册标记资源（B `resources/stairs/stairs_collider.rs`，
//! 1 胶水，无独立业务函数）。
//!
//! rapier 边界：实际碰撞体注册/撤除在 `systems/collision/world.rs`（R1 域）与
//! `resources/stairs/structural_graph`（本域）；本文件仅标记资源。
//! 章节：`chapters/resources/stairs/stairs_collider.rs.md`。
use bevy_ecs::resource::Resource;

/// 楼梯碰撞体注册标记（B `StairsCollider`，default 胶水）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsCollider;

/// B StairColliderRegistry: three edge-keyed ColliderRegistry owners followed
/// by the structural segment registry keyed by (edge, sample index).
///
/// 14060a9e0 drops four separate tables. 140b37180 removes each dirty edge
/// from the first three; create_structural_colliders_and_initial_graph passes
/// the fourth to upsert_collider<(StairGraphEdge,u32)> at 1407d2b79/1407d2ede.
/// These maps own lookup records, not the global RaycastWorld colliders.
/// Replacing this resource must not fabricate collider-disposal side effects.
#[cfg(feature = "native-physics")]
#[derive(Resource, Default)]
pub struct StairColliderRegistry {
    /// Native +0/+0x30/+0x60. Semantic labels for the three lanes remain
    /// unproved; their separate identities and exact edge keys are preserved.
    pub edge_colliders: [std::collections::HashMap<
        super::stairs_state::StairGraphEdge,
        crate::systems::collision::world::ColliderMeshInstance,
    >; 3],
    /// Native +0x90: the index is part of the key, so two subsegments on the
    /// same edge cannot overwrite each other's collider ownership.
    pub structural_colliders: std::collections::HashMap<
        (super::stairs_state::StairGraphEdge, u32),
        crate::systems::collision::world::ColliderMeshInstance,
    >,
}

#[cfg(feature = "native-physics")]
impl StairColliderRegistry {
    /// Native invalidate_dirty_stair_artifacts 140b37787/7d7/827: remove
    /// the dirty edge from all three distinct edge-keyed owners, in order.
    /// Structural segment retention and visual/RT/request cleanup are separate
    /// parts of that consumer and must not be replaced by clearing this map.
    pub fn remove_edge_colliders(
        &mut self,
        world: &mut crate::systems::collision::world::RaycastWorld,
        edge: super::stairs_state::StairGraphEdge,
    ) -> [Option<crate::systems::collision::world::SubworldColliderHandle>; 3] {
        std::array::from_fn(|index| {
            crate::systems::collision::world::remove_collider(
                world,
                &mut self.edge_colliders[index],
                &edge,
            )
        })
    }
}
