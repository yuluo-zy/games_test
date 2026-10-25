//! 石柱碰撞维护（construct_elevation_supports/maintain_stone_column_colliders.rs）。
//!
//! `maintain_stone_column_colliders`（0x14078a6e0）。候选参数面（skeleton
//! 富化）：`(ResMut<StoneColumnCollisionStorage>, ResMut<RaycastWorld>,
//!  EventReader<OnWallDeleted>, EventWriter<StoneColumnsChanged>)`。
//!
//! 2026-10-17 复核：旧实现（删记录 + 不触碰撞体）为**近似体**，已移除。
//! 真实逻辑需 rapier 碰撞体撤除（workspace 已有 rapier3d 0.22.0，但
//! country-core native-physics 后端暴露面未闭合）——**core-api-missing**。
use bevy_ecs::prelude::*;
use country_core::resources::walls::{
    public_walls::OnWallDeleted, wall_stone_columns::StoneColumnCollisionStorage,
};
use country_core::systems::collision::world::RaycastWorld;

/// B 0x14078a6e0。**未实现**：rapier 碰撞体撤除面待 country-core
/// native-physics 暴露后重建。
pub fn maintain_stone_column_colliders(
    _storage: ResMut<StoneColumnCollisionStorage>,
    _world: ResMut<RaycastWorld>,
    _deleted: EventReader<OnWallDeleted>,
    _changed: EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,
) {
    todo!(
        "未实现: maintain_stone_column_colliders 0x14078a6e0（rapier 碰撞体撤除；core native-physics 后端未暴露）"
    )
}
