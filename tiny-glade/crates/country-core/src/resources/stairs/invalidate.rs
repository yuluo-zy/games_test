//! `invalidate.rs` — 脏楼梯伪影失效系统（B `resources/stairs/invalidate.rs`，1 业务）。
//!
//! `invalidate_dirty_stair_artifacts`（0x140b37180，3455B，O）处理链：脏边 drain
//! （StairsState +0x148 集合）→ 每脏边：
//!   1. `remove_collider<StairGraphEdge>`（0x140252ad0）×3（碰撞体撤除——胶水登记，collision 域）；
//!   2. Arc drop（共享句柄——胶水）；
//!   3. RtWorld RtMesh 删除面（rt 域——胶水）；
//!   4. `contains_edge_or_stub`（0x140900e80）判定边是否仍存在 → 决定
//!      `StairsVisualState::remove_edge`（0x140aa8620）/`reset_edge`（0x140aa8a60）与
//!      装配缓存失效（胶水登记）。
//! 章节：`chapters/resources/stairs/invalidate.rs.md` + `collision-eco/ALGORITHMS.md` §3.5。
use crate::{
    resources::stairs::{
        stairs_state::StairsState,
        visual::StairsVisualState,
    },
};
use bevy_ecs::prelude::*;

/// 脏边失效：碰撞体/RT 网格撤除为胶水登记，本实现闭合
/// `contains_edge_or_stub` 判定 → 边视觉 remove/reset 的存储核心。
pub fn invalidate_dirty_stair_artifacts(
    mut stairs: ResMut<StairsState>,
    mut visual: ResMut<StairsVisualState>,
) {
    // 先收集脏边（drain 借用与后续 contains 查询互斥）。
    let edges: Vec<_> = stairs.drain_dirty_edges().collect();
    for edge in edges {
        let (a, b) = edge.nodes();
        // 胶水登记：remove_collider<StairGraphEdge>×3 + Arc drop + RtMesh 删
        // （collision 域 / rt 域待接线）。
        if stairs.contains_edge_or_stub(a, b) {
            // 边仍存在 → 重置视觉（重建）。
            visual.reset_edge(edge);
        } else {
            // 边已移除 → 撤除视觉。
            visual.remove_edge(edge);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        stairs::stairs_state::{ObservedStairEndpoint, StairsState},
        stairs::visual::StairsVisualState,
    };
    use bevy_ecs::prelude::*;

    fn stairs_with_edges(edges: &[(u32, u32)]) -> StairsState {
        let mut stairs = StairsState::empty(ObservedStairEndpoint::from_observed([0; 24]));
        for &(a, b) in edges {
            stairs.add_edge(a, b);
        }
        stairs
    }

    #[test]
    fn invalidate_resets_present_and_removes_absent_edge() {
        let mut world = World::new();
        world.insert_resource(stairs_with_edges(&[(1, 2), (3, 4)]));
        world.init_resource::<StairsVisualState>();
        // 制造一条脏边（3,4）并把它从权威图移除 → contains_edge_or_stub false。
        world
            .resource_mut::<StairsState>()
            .disconnect_edge(3, 4);
        // (1,2) 仍在图内 → 重置；(3,4) 已移除 → 撤除。
        let mut schedule = Schedule::default();
        schedule.add_systems(invalidate_dirty_stair_artifacts);
        schedule.run(&mut world);
        let visual = world.resource::<StairsVisualState>();
        let edge_present = crate::resources::stairs::stairs_state::StairGraphEdge::new(1, 2);
        let edge_removed = crate::resources::stairs::stairs_state::StairGraphEdge::new(3, 4);
        assert!(visual.contains(&edge_present), "present edge reset");
        assert!(!visual.contains(&edge_removed), "absent edge removed");
        assert!(world
            .resource_mut::<StairsState>()
            .drain_dirty_edges()
            .next()
            .is_none());
    }
}
