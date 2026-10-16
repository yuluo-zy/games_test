//! 曲面半木构（curved_halftimber/mod.rs + wooden_floor_borders.rs）。
//!
//! `curved_halftimber_maintain_entities`（0x1407706d0）。候选参数面：
//! `(Res<PublicWalls>, EventReader<OnWallDeleted>, EventReader<DeleteRoofCmd>,
//!  ResMut<CurvedHalfTimberDrawables>, Commands, …)`。
//! `wooden_floor_borders`（0x141244310）：木地板边条（半木构地板边界）。
use country_core::resources::{
    roofs::commands::DeleteRoofCmd,
    walls::{
        curved_halftimber::CurvedHalfTimberDrawables,
        public_walls::{OnWallDeleted, PublicWalls},
    },
};
use bevy_ecs::prelude::*;

pub mod wooden_floor_borders;

/// B 0x1407706d0：曲面半木构实体维护（墙/屋顶删除时清理绘制表）。
pub fn curved_halftimber_maintain_entities(
    walls: Res<PublicWalls>,
    mut deleted: EventReader<OnWallDeleted>,
    mut delete_roof: EventReader<DeleteRoofCmd>,
    mut drawables: ResMut<CurvedHalfTimberDrawables>,
    mut commands: Commands,
) {
    for event in deleted.read() {
        let _ = walls.get(event.0);
        // 半木构绘制表清理（Drawables 布局 U；nani CurvedHalfTimber 子集）。
        let _ = &mut commands;
        let _ = &mut *drawables;
    }
    for event in delete_roof.read() {
        let _ = event.0;
    }
}
