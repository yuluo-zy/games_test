//! 曲面半木构（curved_halftimber/mod.rs + wooden_floor_borders.rs）。
//!
//! `curved_halftimber_maintain_entities`（0x1407706d0）。候选参数面：
//! `(Res<PublicWalls>, EventReader<OnWallDeleted>, EventReader<DeleteRoofCmd>,
//!  ResMut<CurvedHalfTimberDrawables>, Commands, …)`。
//! `wooden_floor_borders`（0x141244310）：木地板边条（半木构地板边界）。
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::commands::DeleteRoofCmd,
    walls::{
        curved_halftimber::CurvedHalfTimberDrawables,
        public_walls::{OnWallDeleted, PublicWalls},
    },
};

pub mod wooden_floor_borders;

/// B 0x1407706d0。**未实现（U）**：`CurvedHalfTimberDrawables` 在 country-core
/// 为 `Gap`（布局 U），无法执行原版“遍历 stride 0x10 drawables + 每墙 0x28
/// 砖表 → despawn 旧实体 → 逐砖 spawn `NaniBundle<CurvedHalfTimberInstanceData>`
/// + `CurvedHalfTimber`（0xd8 命令槽，@0xa0 来源字节 2/1）”。旧空循环已替换为
/// 显式 U（不伪造清理成功）。
pub fn curved_halftimber_maintain_entities(
    walls: Res<PublicWalls>,
    mut deleted: EventReader<OnWallDeleted>,
    mut delete_roof: EventReader<DeleteRoofCmd>,
    mut drawables: ResMut<CurvedHalfTimberDrawables>,
    mut commands: Commands,
) {
    let _ = (
        &walls,
        &mut deleted,
        &mut delete_roof,
        &mut *drawables,
        &mut commands,
    );
    todo!(
        "未实现: curved_halftimber_maintain_entities 0x1407706d0（CurvedHalfTimberDrawables core 类型为 Gap；drawables/砖表/实体字段 U）"
    )
}
