//! 石柱碰撞维护（construct_elevation_supports/maintain_stone_column_colliders.rs）。
//!
//! `maintain_stone_column_colliders`（0x14078a6e0）。候选参数面（skeleton
//! 富化）：`(ResMut<StoneColumnCollisionStorage>, ResMut<RaycastWorld>,
//!  EventReader<OnWallDeleted>, EventWriter<StoneColumnsChanged>)`。
use country_core::resources::walls::{
    public_walls::OnWallDeleted, wall_stone_columns::StoneColumnCollisionStorage,
};
use country_core::systems::collision::world::RaycastWorld;
use bevy_ecs::prelude::*;

/// B 0x14078a6e0。
pub fn maintain_stone_column_colliders(
    mut storage: ResMut<StoneColumnCollisionStorage>,
    mut world: ResMut<RaycastWorld>,
    mut deleted: EventReader<OnWallDeleted>,
    mut changed: EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>,
) {
    for event in deleted.read() {
        if let Some(columns) = storage.columns.remove(&event.0) {
            for _column in columns {
                let _ = &mut *world;
            }
            changed.write(country_core::resources::walls::wall_stone_columns::StoneColumnsChanged(event.0));
        }
    }
}
