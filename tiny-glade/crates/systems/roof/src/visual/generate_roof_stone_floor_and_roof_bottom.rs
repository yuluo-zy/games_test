//! 石地板与屋顶底面生成（visual/generate_roof_stone_floor_and_roof_bottom.rs）。
//!
//! `generate_roof_stone_floor_and_roof_bottom`（0x1421a6d40，839B/204）。参数面
//! （mangled 名 O）：`void(EventReader<RecalculateRoofCmd>, Res<PublicWalls>,
//!  Commands)`；Roof 组件经 wall→roof 哈希表直接读 archetype（stride 0x58）。
//!
//! 机器码（fresh decomp-0x1421a6d40）：
//! - 逐 `RecalculateRoofCmd` 事件（stride 0x20）：wall 哈希表（keyed WallId）→
//!   实体位图校验 → wall→roof 哈希表 → Roof 组件；
//! - `(roof[8]&1)==0`（圆形）时调 `log2f(roof[0x14]·3.0)`（半径×3 的细分级别）；
//! - `DecoratorRank::new()` + `drawable::from()` 构造实例描述，向 Commands 延迟队列
//!   压 `entity_command::insert::<NaniInstanceDesc>`（0x38B：{insert fn, 16B 数据,
//!   tag 9, 0, entity, panic handler}）。
//!
//! 命令执行体（NaniInstanceDesc kind 9 / 白底实例的精确字段）依赖渲染侧
//! `NaniBundle<StoneFloorInstanceData>` 接线 —— **本批未实现（渲染边界）**；
//! CPU 侧控制流（事件→roof→log2f→入队）按机器码重建。
use country_core::resources::{
    roofs::{Roof, commands::RecalculateRoofCmd},
    walls::{WallId, public_walls::PublicWalls},
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
        // 圆形（roof[8]&1==0）：log2f(radius·3.0) —— 细分级别（机器码 O）。
        let mut level = 0.0f32;
        if (roof.observed()[8] & 1) == 0 {
            let radius = f32::from_le_bytes(roof.observed()[0x14..0x18].try_into().unwrap());
            level = (radius * 3.0).log2();
        }
        // 原命令 = Commands::entity(roof_entity).insert(NaniInstanceDesc)。
        // 渲染侧执行体（kind 9、白底 1,1,1）未接线 → 记录观测载荷。
        commands.queue(ObservedStoneFloorCommand {
            entity,
            wall: event.0,
            level,
        });
    }
}

/// 观测延迟命令载荷（decomp：`{insert::<NaniInstanceDesc> fn, 16B 数据, tag 9,
/// 0, entity, panic}`）。CPU 侧完成事件→roof→log2f→入队（机器码证据）；执行体
/// 属渲染/网格侧后置接线。
#[derive(Debug)]
struct ObservedStoneFloorCommand {
    entity: Entity,
    wall: WallId,
    /// `log2f(radius·3.0)`（仅圆形；矩形走 0）。
    level: f32,
}
impl bevy_ecs::system::Command for ObservedStoneFloorCommand {
    fn apply(self, world: &mut bevy_ecs::world::World) {
        // 原命令 fn（insert::<NaniInstanceDesc> + 16B 实例数据）属渲染侧；
        // CPU 侧完成入队与载荷采集，执行体待 NaniBundle 接线。
        let _ = (world, self.entity, self.wall, self.level);
    }
}
