//! 墙发布 → 装饰存储迁移衔接（RE-DECORATOR-STORAGE §5，O 链）。
//!
//! 证据：`publish_public_wall_state` 0x140a8b530 发 OnWallChanged/OnWallDeleted
//! （16B EventInstance 行）→ `DecoratorStorage::on_wall_split` 0x140a1f590 /
//! `on_walls_merged` 0x140a1ea40 / `on_wall_removed` 0x140a1f550 → per_wall 探针
//! + 0x90B memmove + merge_with + 子墙插入 + d2s 索引更新。
//!
//! 当前 crate 的墙发布事件仅携带 `WallId`（无分裂/合并拓扑）；本系统把
//! **OnWallDeleted → on_wall_removed** 接通（真实删除的装饰清理链）。分裂/合并
//! 迁移（on_wall_split/on_walls_merged，需子墙 id + u + 0x28B 记录语义）待
//! `wall_compound_edit.rs`（79 函数，第三批卡A2）恢复后接线；期间已登记 U。
use bevy_ecs::prelude::*;

use crate::resources::walls::{
    decorator_storage::DecoratorStorage,
    public_walls::{OnWallChanged, OnWallDeleted},
};

/// OnWallDeleted → `DecoratorStorage::on_wall_removed`（真实删除清理链，O 衔接）。
/// immutable 置位（快照/恢复）期间跳过——原 immutable 门控语义。
pub fn decorator_wall_events(
    mut storage: ResMut<DecoratorStorage>,
    mut deleted: EventReader<OnWallDeleted>,
    _changed: EventReader<OnWallChanged>,
) {
    if storage.immutable {
        return;
    }
    for &OnWallDeleted(id) in deleted.read() {
        storage.on_wall_removed(id);
    }
}

/// OnWallChanged 的装饰迁移接线登记点（U）：分裂/合并拓扑未在事件中携带；
/// wall_compound_edit.rs 恢复后在此调用 on_wall_split/on_walls_merged。
pub fn decorator_wall_change_pending(_changed: EventReader<OnWallChanged>) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::decorator::DecoratorInfo;

    /// OnWallDeleted → on_wall_removed 清理链（RE §5 接线）。
    #[test]
    fn deleted_wall_clears_decorator_storage() {
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<OnWallChanged>>();
        world.init_resource::<Events<OnWallDeleted>>();
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage.apply_decorator_place(
                crate::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(
                    crate::resources::walls::WallId(7),
                ),
                crate::resources::walls::decorator::DecoratorId(11),
                &DecoratorInfo {
                    dst_wall: 7,
                    decorator_id: 11,
                    ..DecoratorInfo::default()
                },
            );
        }
        world
            .resource_mut::<Events<OnWallDeleted>>()
            .send(OnWallDeleted(crate::resources::walls::WallId(7)));
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_wall_events);
        schedule.run(&mut world);
        // 源墙桶被删、d2s 注销。
        assert!(
            world
                .resource::<DecoratorStorage>()
                .get(crate::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(
                    crate::resources::walls::WallId(7),
                ))
                .is_none()
        );
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .lookup_storage(crate::resources::walls::decorator::DecoratorId(11)),
            None
        );
    }

    /// immutable 置位期间跳过（原 immutable 门控语义）。
    #[test]
    fn immutable_storage_skips_deleted_cleanup() {
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<OnWallChanged>>();
        world.init_resource::<Events<OnWallDeleted>>();
        world.resource_mut::<DecoratorStorage>().mark_immutable();
        world
            .resource_mut::<Events<OnWallDeleted>>()
            .send(OnWallDeleted(crate::resources::walls::WallId(1)));
        let mut schedule = Schedule::default();
        schedule.add_systems(decorator_wall_events);
        schedule.run(&mut world); // 不 panic。
    }
}
