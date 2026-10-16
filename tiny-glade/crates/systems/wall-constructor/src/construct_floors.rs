//! 墙内地板构造（construct_floors.rs）。
//!
//! `construct_floors`（0x1407661e0）。候选参数面（skeleton 富化）：
//! `(Res<PublicWalls>, EventReader<OnWallChanged>, EventReader<OnWallDeleted>,
//!  Local<FloorStorage>, Commands)`。
//! 机器码（building-fences findings 下一 owner）：墙内地板（与屋顶地板
//! 路径分开）；Local `FloorStorage` 跨帧保留地板实体。
use country_core::resources::walls::public_walls::{OnWallChanged, OnWallDeleted, PublicWalls};
use bevy_ecs::prelude::*;

/// B `system_wall_constructor::construct_floors::FloorStorage`（Local）。
#[derive(Default, Debug)]
pub struct FloorStorage {
    /// 墙 id → 地板实体（跨帧保留）。
    pub entities: hashbrown::HashMap<u64, Entity>,
}

/// B 0x1407661e0。
pub fn construct_floors(
    mut changed: EventReader<OnWallChanged>,
    mut deleted: EventReader<OnWallDeleted>,
    walls: Res<PublicWalls>,
    mut storage: Local<FloorStorage>,
    mut commands: Commands,
) {
    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };
        // 墙内地板：闭合墙轮廓 → 地板网格（与屋顶 stone floor 路径分开；
        // 网格 spawn 的精确面数待 building-fences findings 冻结链接线）。
        let _ = wall.simplified_curve_2d().length;
        if !storage.entities.contains_key(&event.0 .0) {
            let entity = commands.spawn_empty().id();
            storage.entities.insert(event.0 .0, entity);
        }
    }
    for event in deleted.read() {
        if let Some(entity) = storage.entities.remove(&event.0 .0) {
            commands.entity(entity).despawn();
        }
    }
}
