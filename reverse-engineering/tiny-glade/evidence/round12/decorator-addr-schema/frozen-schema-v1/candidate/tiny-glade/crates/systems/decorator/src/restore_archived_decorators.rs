//! OriginalB restore140817a40/set42, with its actual eight system parameters.
//! mark_mutable runs first; ModeJustChanged is consumed once. With no new
//! event the wall eraser skips storage/collision restoration, but stairs always
//! restore. Other paths clear tracked buckets with their d2s indices, skip
//! deleted authoritative walls, and restore both actual collision owners.
//! A mode change extracts and drops the original diff; it publishes no history.

use bevy_ecs::prelude::*;

use country_core::resources::events::ModeJustChanged;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist;
use country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress;
use country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::inner_walls::InnerWalls;
use country_core::systems::collision::world::{RaycastWorld, upsert_archived_collider};
use country_core::systems::mode_manager::{AppMode, EraserLayer};

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

/// Original system contract; register this concrete system in set42.
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
    // Original140817a9f precedes even the fast path. The event cursor is
    // consumed once; an event forces restore and final archive extraction.
    storage.mark_mutable();
    let changed = !mode_changed.is_empty();
    mode_changed.clear();
    if changed || !matches!(*mode, AppMode::Eraser(EraserLayer::Wall)) {
        restore_storage_and_colliders(
            &mut storage,
            &archivist,
            &inner_walls,
            &mut colliders,
            &mut world,
        );
    }
    // This always runs, including the wall eraser/no-event fast path.
    stairs.restore_backup();
    if changed {
        // Original140818969 extracts with the restored storage and flag1;
        // 140b743e0 immediately drops the returned table. No history publish.
        drop(archivist.extract_diff_against_storage(&storage, true));
    }
}

fn restore_storage_and_colliders(
    storage: &mut DecoratorStorage,
    archivist: &LiveDecoratorArchivist,
    inner_walls: &InnerWalls,
    colliders: &mut DecoratorPhysicsColliders,
    world: &mut RaycastWorld,
) {
    for address in archivist_all_storages(archivist) {
        clear_live_bucket(storage, addr_from_address(address));
    }
    // Restore wall buckets only if their authoritative wall still exists;
    // special terrain/stair buckets have no InnerWalls lookup.
    for address in archivist_all_storages(archivist) {
        if let DecoratorStorageAddress::Wall(wall) = address {
            if inner_walls.get(WallId(wall.0)).is_none() {
                continue;
            }
        }
        let ids: Vec<_> = tracked_at(archivist, address)
            .unwrap()
            .items
            .keys()
            .copied()
            .collect();
        // get_or_insert_default runs even for an empty archived bucket.
        storage.get_or_insert_default(addr_from_address(address));
        for id in &ids {
            overwrite_decorator_state_from_archivist(storage, archivist, address, *id);
        }
        // Original completes channel A for this bucket, then channel B.
        for (saved, live) in [
            (&archivist.collision_a, &mut colliders.first),
            (&archivist.collision_b, &mut colliders.second),
        ] {
            for id in &ids {
                if let Some((transform, mesh)) = saved.get(id) {
                    upsert_archived_collider(world, live, DecoratorId(*id), mesh, transform);
                }
            }
        }
    }
}

/// Original141257900 clears modified, extends removed by all live IDs, takes
/// the item table, and removes indexed live entries until the niche2 sentinel.
/// Taking before the removal loop preserves its partial state on a panic.
fn clear_live_bucket(storage: &mut DecoratorStorage, address: DecoratorStorageAddr) {
    let Some(bucket) = storage.get_mut(address) else {
        return;
    };
    bucket.modified_ids.clear();
    bucket.removed_ids.extend(bucket.decorators.keys().copied());
    let old = std::mem::take(&mut bucket.decorators);
    for (id, info) in old {
        if info.state == 2 {
            break;
        }
        storage
            .decorator_to_storage
            .remove(DecoratorId(id))
            .expect("d2s remove in Decorators::clear: no such decorator");
    }
}

/// 单装饰归档状态覆盖（0x140817a40 步骤 4 内联）：archivist 条目的
/// `ObservedDecoratorInfo` 解码为 `DecoratorInfo` → 目标桶 insert → 按
/// `state==2` 选择 d2s insert/update。
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
    // Native compares the returned Option<DecoratorInfo> niche, not incoming
    // state. Niche2 means None and must take the checked d2s insertion path.
    if old.is_none() {
        storage.decorator_to_storage.insert(DecoratorId(id), addr);
    } else {
        assert!(
            storage.decorator_to_storage.get(DecoratorId(id)).is_some(),
            "d2s update: no such {}",
            id
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_archivist::changes::{
        ObservedDecoratorInfo, TrackedDecorators,
    };
    use country_core::resources::walls::publication_fields::WallId as ArchivedWallId;

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
    fn overwrite_existing_item_rejects_missing_index_without_inserting_one() {
        let mut archive = DecoratorArchivist::<()>::default();
        let info = country_core::resources::walls::decorator::DecoratorInfo {
            decorator_id: 5,
            ..Default::default()
        };
        archive.special1 = Some(TrackedDecorators {
            items: [(
                5,
                ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
            )]
            .into(),
            ..Default::default()
        });
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(DecoratorId(5), info);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            overwrite_decorator_state_from_archivist(
                &mut storage,
                &archive,
                DecoratorStorageAddress::ObservedSpecial1,
                5,
            )
        }));
        assert!(result.is_err());
        assert!(storage.lookup_storage(DecoratorId(5)).is_none());
        assert!(
            storage
                .get(DecoratorStorageAddr::Terrain)
                .unwrap()
                .get(DecoratorId(5))
                .is_some()
        );
    }

    #[test]
    fn mode_event_overrides_wall_eraser_once_and_restores_stairs_on_both_paths() {
        let mut ecs = World::new();
        let mut archive = LiveDecoratorArchivist::default();
        let info = country_core::resources::walls::decorator::DecoratorInfo {
            decorator_id: 7,
            state: 0,
            ..Default::default()
        };
        archive.special1 = Some(TrackedDecorators {
            items: [(
                7,
                ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
            )]
            .into(),
            ..Default::default()
        });
        let mut storage = DecoratorStorage::default();
        let address = DecoratorStorageAddr::Terrain;
        storage.get_or_insert_default(address).insert(
            DecoratorId(8),
            country_core::resources::walls::decorator::DecoratorInfo {
                decorator_id: 8,
                ..Default::default()
            },
        );
        storage.decorator_to_storage.insert(DecoratorId(8), address);
        storage.mark_immutable();
        let mut stairs = StairsState::new();
        stairs.add_edge(1, 2);
        stairs.make_backup();
        stairs.add_edge(2, 3);
        ecs.insert_resource(storage);
        ecs.insert_resource(archive);
        ecs.insert_resource(stairs);
        ecs.insert_resource(AppMode::Eraser(EraserLayer::Wall));
        ecs.insert_resource(InnerWalls::default());
        ecs.insert_resource(DecoratorPhysicsColliders::default());
        ecs.insert_resource(RaycastWorld::new());
        ecs.insert_resource(Events::<ModeJustChanged>::default());
        // Keep the same SystemState/EventReader between frames.
        let mut schedule = Schedule::default();
        schedule.add_systems(restore_archived_decorators);
        schedule.run(&mut ecs);
        assert!(!ecs.resource::<DecoratorStorage>().immutable);
        assert!(
            ecs.resource::<DecoratorStorage>()
                .lookup_decorator_info(DecoratorId(8))
                .is_some()
        );
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_some());
        assert!(!ecs.resource::<StairsState>().contains_edge_or_stub(2, 3));
        ecs.resource_mut::<Events<ModeJustChanged>>()
            .send(ModeJustChanged::default());
        schedule.run(&mut ecs);
        let restored = ecs.resource::<DecoratorStorage>();
        assert!(restored.lookup_decorator_info(DecoratorId(8)).is_none());
        assert!(restored.lookup_decorator_info(DecoratorId(7)).is_some());
        assert!(restored.get(address).unwrap().removed_ids.contains(&8));
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_none());
        // No new event: wall eraser keeps a newly installed archive for later.
        ecs.resource_mut::<LiveDecoratorArchivist>().special1 = Some(TrackedDecorators::default());
        schedule.run(&mut ecs);
        assert!(ecs.resource::<LiveDecoratorArchivist>().special1.is_some());
    }

    #[test]
    fn vanished_wall_is_cleared_but_not_reinserted_and_channels_restore_exact_owners() {
        use country_core::systems::collision::world::{
            OffsetColliderTransform, RaycastGeoSource, RaycastMesh, RaycastMeshData,
        };
        use std::sync::Arc;
        let mut archive = LiveDecoratorArchivist::default();
        let make_tracked = |id| {
            let info = country_core::resources::walls::decorator::DecoratorInfo {
                decorator_id: id,
                ..Default::default()
            };
            TrackedDecorators {
                items: [(
                    id,
                    ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
                )]
                .into(),
                ..Default::default()
            }
        };
        archive.walls.insert(ArchivedWallId(9), make_tracked(9));
        archive.special1 = Some(make_tracked(10));
        let data = Arc::new(RaycastMeshData::new_cuboid(
            glam::Vec3::splat(0.5),
            glam::UVec3::ONE,
        ));
        let shape = data.shape_ref().unwrap().clone_dyn();
        let transform = OffsetColliderTransform {
            position: glam::Vec3::new(2., 3., 4.),
            rotation: glam::Quat::IDENTITY,
            shape: Arc::from(shape),
        };
        archive.collision_a.insert(
            10,
            (
                transform.clone(),
                RaycastMesh::new(data.clone(), RaycastGeoSource::Slot0),
            ),
        );
        archive.collision_b.insert(
            10,
            (
                transform,
                RaycastMesh::new(data.clone(), RaycastGeoSource::Slot1),
            ),
        );
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(9)))
            .insert(
                DecoratorId(9),
                country_core::resources::walls::decorator::DecoratorInfo {
                    decorator_id: 9,
                    ..Default::default()
                },
            );
        storage
            .decorator_to_storage
            .insert(DecoratorId(9), DecoratorStorageAddr::Wall(WallId(9)));
        let mut physics = DecoratorPhysicsColliders::default();
        let mut world = RaycastWorld::new();
        restore_storage_and_colliders(
            &mut storage,
            &archive,
            &InnerWalls::default(),
            &mut physics,
            &mut world,
        );
        assert!(storage.lookup_decorator_info(DecoratorId(9)).is_none());
        assert!(storage.lookup_decorator_info(DecoratorId(10)).is_some());
        for map in [&physics.first, &physics.second] {
            let instance = map[&DecoratorId(10)];
            let mesh = world.get_collider_mesh(instance.registry_index, instance.generation);
            assert!(Arc::ptr_eq(&mesh.data, &data));
            let t = world.into_transformed_shape(instance.handle());
            assert_eq!(t.position, glam::Vec3::new(2., 3., 4.));
        }
    }
}
