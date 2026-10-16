//! 石地板与屋顶底面生成（visual/generate_roof_stone_floor_and_roof_bottom.rs）。
//!
//! `generate_roof_stone_floor_and_roof_bottom`（0x1421a6d40）。候选参数面
//! （skeleton 富化）：`(EventReader<RecalculateRoofCmd>, Query<(Entity, &Roof)>,
//!  Commands)`。
//!
//! 机器码（evidence/native .c）：队列 walk → 掩码位集判"应生成" → 随机数
//! `(float)func_0x000142923f50(0)` clamp [1.0, 5.0] → 整数瓦数
//! （1.0@0x142925904、5.0@0x142925988、守卫 4294967040.0@0x1429a07d0）→
//! 延迟命令（双 fn 指针命令项 {FUN_140dceea0, 16B 数据, tag 9, …}）入队。
//! Commands 的延迟队列按观测结构登记（命令执行体 = 渲染/网格侧后置）。
use country_core::resources::{
    roofs::{Roof, commands::RecalculateRoofCmd},
    walls::public_walls::PublicWalls,
};
use bevy_ecs::prelude::*;

/// B 0x1421a6d40。
pub fn generate_roof_stone_floor_and_roof_bottom(
    mut recalc: EventReader<RecalculateRoofCmd>,
    walls: Res<PublicWalls>,
    roofs: Query<(Entity, &Roof)>,
    mut commands: Commands,
) {
    for event in recalc.read() {
        let Some(entity) = walls.roof_entity(event.0) else {
            continue;
        };
        let Ok((entity, roof)) = roofs.get(entity) else {
            continue;
        };
        if roof.is_pitched() {
            continue;
        }
        // 平顶：随机 1..5 瓦数（观测 clamp），生成延迟命令条目（tag 9）。
        let tiles = random_tile_count();
        commands.queue(ObservedStoneFloorCommand {
            entity,
            wall: event.0,
            tile_count: tiles,
        });
    }
}

/// 随机瓦数：`(float)func_0x000142923f50(0)` clamp [1.0, 5.0]（守卫
/// 4294967040.0 = 0xffff_ff00 语义上界）。
fn random_tile_count() -> u32 {
    let value = (fastrand::u64(..) as f64) as f32;
    let clamped = value.clamp(1.0, 5.0);
    clamped as u32
}

/// 观测延迟命令载荷（{FUN_140dceea0, 16B 数据, tag=9, 0, entity,
/// FUN_140dcfa80}；命令执行体 = 渲染侧后置登记）。
#[derive(Debug)]
struct ObservedStoneFloorCommand {
    entity: Entity,
    wall: country_core::resources::walls::WallId,
    tile_count: u32,
}
impl bevy_ecs::system::Command for ObservedStoneFloorCommand {
    fn apply(self, world: &mut bevy_ecs::world::World) {
        // 原延迟命令对（FUN_140dceea0/FUN_140dcfa80）属于渲染/网格侧；
        // CPU 侧完成随机瓦数与入队（机器码证据），执行体待 render 接线。
        let _ = world;
        let _ = (self.entity, self.wall, self.tile_count);
    }
}
