//! `restore_archived_decorators.rs` — 归档装饰恢复（0x140817a40，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140817a40-*.txt`，
//! 47KB）：
//! 1. `mark_mutable(storage)`（0x140a1c2b0）；
//! 2. ModeJustChanged 游标与墙擦除模式快速路径；
//! 3. `DecoratorArchivist::all_storages`（0x140b2bb80）→ 逐地址 `get_mut` +
//!    `clear_modification_tracking`（0x141257900）；
//! 4. `all_decorators`（0x140b2bb00）→ 逐装饰：地址桶缺失 → archivist 侧
//!    map 查（0x14085a560，U 身份）；`get_or_insert_default` +
//!    `DecoMoveToStorage::new`（0x140a1b430）+ 内嵌 map 逐条 insert（哈希
//!    XOR 掩码 _UNK_1429cd080/_UNK_1429cd090 = core SNAPSHOT_ID_MASK）+ 状态
//!    字节==2 → `d2s insert`（0x140a1b4e0）否则 `d2s update`（0x140a1b5a0）；
//! 5. `decorator_colliders`（0x140005c60）/`decorator_interactions`
//!    （0x1403c03c0）→ Arc 克隆 + 落位（FUN_141298110，U 身份）；
//! 6. 模式事件 → `extract_diff`（0x140b2cb30），返回表立即析构；不提交历史。
//!
//! core 对接：`restore_archived_decorators`（shared building modules，等价
//! extract_and_restore_to_storage 0x140b2c4b0）+ `DecoratorStorage` 桶 API +
//! `StairsState::restore_backup` + `InnerWalls::get`（机器码调用面）。
//! core 缺口登记：archivist `all_storages/all_decorators/extract_diff` 未在
//! core 实现（章节 O）——本模块以核心模型字段提供等价迭代
//! （[`archivist_all_storages`]/[`archivist_all_decorators`]），由集成 owner
//! 裁决是否上收 core。

use bevy_ecs::prelude::*;

use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist;
use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
use country_core::resources::events::ModeJustChanged;
use country_core::systems::collision::world::{RaycastWorld, upsert_archived_collider};
use country_core::systems::mode_manager::{AppMode, EraserLayer};
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::inner_walls::InnerWalls;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress;

/// archivist 全存储地址迭代（等价 `all_storages` 0x140b2bb80；core 模型面）。
pub fn archivist_all_storages<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
) -> Vec<DecoratorStorageAddress> {
    let mut out: Vec<DecoratorStorageAddress> = archivist
        .walls
        .keys()
        .map(|w| DecoratorStorageAddress::Wall(*w))
        .collect();
    if archivist.special1.is_some() {
        out.push(DecoratorStorageAddress::ObservedSpecial1);
    }
    if archivist.special2.is_some() {
        out.push(DecoratorStorageAddress::ObservedSpecial2);
    }
    out
}

/// archivist 全装饰迭代（等价 `all_decorators` 0x140b2bb00；core 模型面）。
pub fn archivist_all_decorators<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
) -> Vec<(DecoratorStorageAddress, u32)> {
    let mut out = Vec::new();
    for (wall, tracked) in &archivist.walls {
        for id in tracked.items.keys() {
            out.push((DecoratorStorageAddress::Wall(*wall), *id));
        }
    }
    if let Some(t) = &archivist.special1 {
        for id in t.items.keys() {
            out.push((DecoratorStorageAddress::ObservedSpecial1, *id));
        }
    }
    if let Some(t) = &archivist.special2 {
        for id in t.items.keys() {
            out.push((DecoratorStorageAddress::ObservedSpecial2, *id));
        }
    }
    out
}

/// 地址 → 存储桶（core 模型面）。
pub(crate) fn tracked_at<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
    address: DecoratorStorageAddress,
) -> Option<&country_core::resources::walls::decorator_archivist::changes::TrackedDecorators> {
    match address {
        DecoratorStorageAddress::Wall(w) => archivist.walls.get(&w),
        DecoratorStorageAddress::ObservedSpecial1 => archivist.special1.as_ref(),
        DecoratorStorageAddress::ObservedSpecial2 => archivist.special2.as_ref(),
    }
}

/// B 140817a40：原八参数合同，恢复装饰桶及两类碰撞，模式变化时丢弃差分。
pub fn restore_archived_decorators(
    mode: Res<AppMode>,
    inner_walls: Res<InnerWalls>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut storage: ResMut<DecoratorStorage>,
    mut colliders: ResMut<DecoratorPhysicsColliders>,
    mut world: ResMut<RaycastWorld>,
    mut stairs: ResMut<StairsState>,
    mut mode_changed: EventReader<ModeJustChanged>,
) {
    storage.mark_mutable();
    let changed = !mode_changed.is_empty();
    mode_changed.clear();
    if std::env::var_os("TG_M6_DECORATOR_TRACE").is_some() {
        eprintln!("M6_RESTORE before mode={mode:?} changed={changed} archive={:?} live={:?}",
            archivist_all_storages(&archivist),
            storage.per_wall.iter().map(|(wall, bucket)| (*wall, bucket.decorators.iter().map(|(id, info)| (*id, info.state)).collect::<Vec<_>>())).collect::<Vec<_>>());
    }
    if changed || !matches!(*mode, AppMode::Eraser(EraserLayer::Wall)) {
        for address in archivist_all_storages(&archivist) {
            let Some(bucket) = storage.get_mut(addr_from_address(address)) else { continue };
            bucket.modified_ids.clear();
            bucket.removed_ids.extend(bucket.decorators.keys().copied());
            let old = std::mem::take(&mut bucket.decorators);
            for (id, info) in old {
                // 141257a20..a89 扫描 0x50 字节桶项，直接读取项末字节
                //（DecoratorInfo +0x47）；值 2 跳至清表出口，不是仅由空迭代器生成的值。
                // 原 hashbrown 扫描序与当前 HashMap 的桶序不保证跨平台一致。
                if info.state == 2 { break; }
                storage.decorator_to_storage.remove(DecoratorId(id))
                    .expect("d2s remove in Decorators::clear: no such decorator");
            }
        }
        for address in archivist_all_storages(&archivist) {
            if let DecoratorStorageAddress::Wall(wall) = address {
                if inner_walls.get(WallId(wall.0)).is_none() { continue; }
            }
            let ids: Vec<_> = tracked_at(&archivist, address).unwrap().items.keys().copied().collect();
            storage.get_or_insert_default(addr_from_address(address));
            for id in &ids {
                overwrite_decorator_state_from_archivist(&mut storage, &archivist, address, *id);
            }
            for id in &ids {
                if let Some((pose, mesh)) = archivist.collision_a.get(id) {
                    upsert_archived_collider(&mut world, &mut colliders.first, DecoratorId(*id), mesh, pose);
                }
            }
            for id in &ids {
                if let Some((pose, mesh)) = archivist.collision_b.get(id) {
                    upsert_archived_collider(&mut world, &mut colliders.second, DecoratorId(*id), mesh, pose);
                }
            }
        }
    }
    stairs.restore_backup();
    // 原版模式变更时只提取并丢弃差分；历史提交由录制结束系统负责。
    if changed { drop(archivist.extract_diff(&storage, true)); }
    if std::env::var_os("TG_M6_DECORATOR_TRACE").is_some() {
        eprintln!("M6_RESTORE after archive={:?} live={:?}", archivist_all_storages(&archivist),
            storage.per_wall.iter().map(|(wall, bucket)| (*wall, bucket.decorators.iter().map(|(id, info)| (*id, info.state)).collect::<Vec<_>>())).collect::<Vec<_>>());
    }
}

/// 单装饰归档状态覆盖（0x140817a40 步骤 4 内联）：archivist 条目的
/// `ObservedDecoratorInfo` 解码为 `DecoratorInfo` → 目标桶 insert → 按
/// 插入返回的旧Option选择 d2s insert/update，不改输入flags。
///
/// 返回是否写入（归档条目缺失 → false，不伪造默认状态）。
pub fn overwrite_decorator_state_from_archivist<C: Send + Sync + 'static>(
    storage: &mut DecoratorStorage,
    archivist: &DecoratorArchivist<C>,
    address: DecoratorStorageAddress,
    id: u32,
) -> bool {
    let Some(tracked) = tracked_at(archivist, address) else {
        return false;
    };
    let Some(info) = tracked.items.get(&id) else {
        return false;
    };
    let decorator_info = crate::apply_decorator_op::bytes_to_info(&info.0);
    let addr = addr_from_address(address);
    let bucket = storage.get_or_insert_default(addr);
    let old = bucket.insert(DecoratorId(id), decorator_info);
    if old.is_none() {
        storage.decorator_to_storage.insert(DecoratorId(id), addr);
    } else {
        storage.decorator_to_storage.update(DecoratorId(id), addr);
    }
    true
}

/// DecoratorStorageAddress → DecoratorStorageAddr（core 模型桥；
/// recovered WallId → resources WallId 双类型转换）。
pub(crate) fn addr_from_address(address: DecoratorStorageAddress) -> DecoratorStorageAddr {
    match address {
        DecoratorStorageAddress::Wall(w) => DecoratorStorageAddr::Wall(WallId(w.0)),
        DecoratorStorageAddress::ObservedSpecial1 => DecoratorStorageAddr::Terrain,
        DecoratorStorageAddress::ObservedSpecial2 => DecoratorStorageAddr::Stair,
    }
}

/// 模型桥：WallId 保留（all_storages 墙键面）。
#[allow(dead_code)]
fn _wall_id_surface(_w: WallId) {}

const LOG_TARGET: &str = "system_decorator::restore_archived_decorators";

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_archivist::changes::{
        ObservedDecoratorInfo, TrackedDecorators,
    };
    use country_core::resources::walls::publication_fields::WallId as ArchivedWallId;

    #[test]
    fn wall_eraser_mode_event_restores_once_and_drains_archive() {
        let mut ecs = World::new();
        let mut archive = LiveDecoratorArchivist::default();
        let info = country_core::resources::walls::decorator::DecoratorInfo {
            decorator_id: 7, ..Default::default()
        };
        archive.special1 = Some(TrackedDecorators {
            items: [(7, ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)))].into(),
            ..Default::default()
        });
        let mut storage = DecoratorStorage::default();
        let address = DecoratorStorageAddr::Terrain;
        storage.get_or_insert_default(address).insert(DecoratorId(8),
            country_core::resources::walls::decorator::DecoratorInfo { decorator_id: 8, ..Default::default() });
        storage.decorator_to_storage.insert(DecoratorId(8), address);
        storage.mark_immutable();
        let mut stairs = StairsState::new();
        stairs.make_backup();
        ecs.insert_resource(storage); ecs.insert_resource(archive); ecs.insert_resource(stairs);
        ecs.insert_resource(AppMode::Eraser(EraserLayer::Wall)); ecs.insert_resource(InnerWalls::default());
        ecs.insert_resource(DecoratorPhysicsColliders::default()); ecs.insert_resource(RaycastWorld::new());
        ecs.insert_resource(Events::<ModeJustChanged>::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(restore_archived_decorators);
        schedule.run(&mut ecs);
        assert!(!ecs.resource::<DecoratorStorage>().immutable);
        assert!(ecs.resource::<DecoratorStorage>().lookup_decorator_info(DecoratorId(8)).is_some());
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_some());
        ecs.resource_mut::<Events<ModeJustChanged>>().send(ModeJustChanged {
            current: AppMode::Eraser(EraserLayer::Wall), previous: AppMode::default(),
        });
        schedule.run(&mut ecs);
        assert!(ecs.resource::<DecoratorStorage>().lookup_decorator_info(DecoratorId(8)).is_none());
        assert!(ecs.resource::<DecoratorStorage>().lookup_decorator_info(DecoratorId(7)).is_some());
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_none());
        ecs.resource_mut::<LiveDecoratorArchivist>().special1 = Some(TrackedDecorators::default());
        schedule.run(&mut ecs);
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_some());
    }

    #[test]
    fn restore_reinstates_both_collision_owners_at_archived_final_pose() {
        use country_core::systems::collision::world::{RaycastGeoSource, RaycastMesh, RaycastMeshData, upsert_collider};
        use std::sync::Arc;
        let id = DecoratorId(10);
        let address = DecoratorStorageAddr::Terrain;
        let info = country_core::resources::walls::decorator::DecoratorInfo { decorator_id: id.0, ..Default::default() };
        let mut archive = LiveDecoratorArchivist::default();
        archive.special1 = Some(TrackedDecorators {
            items: [(id.0, ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)))].into(),
            ..Default::default()
        });
        let data = Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::Y));
        let distinct_shape = Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::splat(2.0), glam::Vec3::ZERO));
        let pose = country_core::resources::walls::decorator_archivist::ArchivedColliderTransform {
            translation: glam::Vec3::new(2., 3., 4.), rotation: glam::Quat::IDENTITY,
            shape: distinct_shape.shape.as_ref().unwrap().0.clone(),
        };
        archive.collision_a.insert(id.0, (pose.clone(), RaycastMesh::new(data.clone(), RaycastGeoSource::decorator(id, address))));
        archive.collision_b.insert(id.0, (pose, RaycastMesh::new(data.clone(), RaycastGeoSource::decorator_interaction(id, address, 0))));
        let mut live_world = RaycastWorld::new();
        let mut live_maps = DecoratorPhysicsColliders::default();
        // 同一个注册 mesh/source 和最终位姿，但活 shape 与归档 shape 不同。
        // 恢复必须替换碰撞体，不能走仅比较 mesh 与位姿的快速路径。
        upsert_collider(&mut live_world, &mut live_maps.first, id,
            RaycastGeoSource::decorator(id, address), data.clone(), glam::Vec3::new(2., 2., 4.), glam::Quat::IDENTITY);
        upsert_collider(&mut live_world, &mut live_maps.second, id,
            RaycastGeoSource::decorator_interaction(id, address, 0), data.clone(), glam::Vec3::new(2., 2., 4.), glam::Quat::IDENTITY);
        for map in [&live_maps.first, &live_maps.second] {
            let live = live_world.into_transformed_shape(map[&id].handle());
            assert_eq!(live.position, glam::Vec3::new(2., 3., 4.));
            assert!(!Arc::ptr_eq(&live.shape, &distinct_shape.shape.as_ref().unwrap().0));
        }
        let mut stairs = StairsState::new(); stairs.make_backup();
        let mut ecs = World::new();
        ecs.insert_resource(archive); ecs.insert_resource(DecoratorStorage::default()); ecs.insert_resource(stairs);
        ecs.insert_resource(AppMode::default()); ecs.insert_resource(InnerWalls::default());
        ecs.insert_resource(live_maps); ecs.insert_resource(live_world);
        ecs.insert_resource(Events::<ModeJustChanged>::default());
        let mut schedule = Schedule::default(); schedule.add_systems(restore_archived_decorators);
        schedule.run(&mut ecs);
        assert_eq!(ecs.resource::<DecoratorStorage>().lookup_storage(id), Some(address));
        let maps = ecs.resource::<DecoratorPhysicsColliders>();
        let world = ecs.resource::<RaycastWorld>();
        for map in [&maps.first, &maps.second] {
            let instance = map[&id];
            assert!(Arc::ptr_eq(&world.get_collider_mesh(instance.registry_index, instance.generation).data, &data));
            let actual = world.get_collider(instance.handle());
            assert_eq!(actual.position, glam::Vec3::new(2., 3., 4.));
            assert_eq!(actual.rotation, glam::Quat::IDENTITY);
            assert!(Arc::ptr_eq(&world.into_transformed_shape(instance.handle()).shape, &distinct_shape.shape.as_ref().unwrap().0));
        }
    }

    #[test]
    fn overwrite_restores_entry_and_d2s() {
        let mut archivist = DecoratorArchivist::<()>::default();
        let info = country_core::resources::walls::decorator::DecoratorInfo {
            decorator_id: 5,
            state: 0,
            ..Default::default()
        };
        let mut tracked = TrackedDecorators::default();
        tracked.items.insert(
            5,
            ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
        );
        archivist.walls.insert(ArchivedWallId(9), tracked);

        let mut storage = DecoratorStorage::default();
        assert!(overwrite_decorator_state_from_archivist(
            &mut storage,
            &archivist,
            DecoratorStorageAddress::Wall(ArchivedWallId(9)),
            5
        ));
        let entry = storage
            .lookup_decorator_info(DecoratorId(5))
            .expect("restored entry");
        assert_eq!(entry.state, 0);
        assert_eq!(
            storage.lookup_storage(DecoratorId(5)),
            Some(DecoratorStorageAddr::Wall(WallId(9)))
        );
    }

    #[test]
    fn overwrite_returns_false_for_missing_archive_entry() {
        let archivist = DecoratorArchivist::<()>::default();
        let mut storage = DecoratorStorage::default();
        assert!(!overwrite_decorator_state_from_archivist(
            &mut storage,
            &archivist,
            DecoratorStorageAddress::Wall(ArchivedWallId(9)),
            5
        ));
        assert!(storage.lookup_decorator_info(DecoratorId(5)).is_none());
    }

    #[test]
    fn overwrite_uses_old_option_for_both_legal_flags_and_preserves_partial_failure() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        use country_core::resources::walls::decorator::DecoratorInfo;
        for flag in [0, 1] {
            let address = DecoratorStorageAddress::Wall(ArchivedWallId(9));
            let addr = DecoratorStorageAddr::Wall(WallId(9));
            let id = DecoratorId(5);
            let info = DecoratorInfo { decorator_id: id.0, state: flag, u45: 1 - flag, u28: 0x7fc1_2345, ..Default::default() };
            let mut archive = DecoratorArchivist::<()>::default();
            let mut tracked = TrackedDecorators::default();
            tracked.items.insert(id.0, ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)));
            archive.walls.insert(ArchivedWallId(9), tracked);
            let mut storage = DecoratorStorage::default();
            assert!(overwrite_decorator_state_from_archivist(&mut storage, &archive, address, id.0));
            assert_eq!(storage.lookup_decorator_info(id), Some(&info));
            assert_eq!(storage.lookup_storage(id), Some(addr));
            // 已有旧值必须走update，重复恢复不触发insert冲突。
            assert!(overwrite_decorator_state_from_archivist(&mut storage, &archive, address, id.0));
            assert_eq!(storage.lookup_decorator_info(id), Some(&info));
            let mut conflict = DecoratorStorage::default();
            conflict.decorator_to_storage.insert(id, DecoratorStorageAddr::Terrain);
            assert!(catch_unwind(AssertUnwindSafe(|| overwrite_decorator_state_from_archivist(&mut conflict, &archive, address, id.0))).is_err());
            assert_eq!(conflict.get(addr).unwrap().get(id), Some(&info));
            assert!(conflict.get(addr).unwrap().modified_ids.contains(&id.0));
        }
    }

}
