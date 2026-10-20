//! Genuine per-frame tracking reset must preserve all authoritative buckets.
use std::collections::HashSet;

use country_core::resources::walls::{
    WallId,
    decorator::{DecoratorId, DecoratorInfo},
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
};

fn populated_storage() -> DecoratorStorage {
    let mut storage = DecoratorStorage::default();
    let addresses = [
        DecoratorStorageAddr::Wall(WallId(0x1_0000_0007)),
        DecoratorStorageAddr::Wall(WallId(0x2_0000_0007)),
        DecoratorStorageAddr::Terrain,
        DecoratorStorageAddr::Stair,
    ];
    for (index, address) in addresses.into_iter().enumerate() {
        let id = DecoratorId(index as u32 + 10);
        let info = DecoratorInfo {
            decorator_id: id.0,
            dst_wall: index as u64 + 70,
            ..DecoratorInfo::default()
        };
        let bucket = storage.get_or_insert_default(address);
        bucket.insert(id, info);
        bucket.removed_ids.insert(100 + id.0);
        storage.decorator_to_storage.insert(id, address);
    }
    storage.removed.insert(
        DecoratorStorageAddr::Wall(WallId(0x3_0000_0007)),
        HashSet::from([111, 112]),
    );
    storage
}

#[test]
fn two_frame_reset_preserves_wall_terrain_stairs_payloads_and_all_d2s_entries() {
    let mut storage = populated_storage();
    let before = storage.archive_current_decorators();
    let wall_capacity = storage.per_wall.capacity();
    let addresses: Vec<_> = (10..14)
        .map(|id| {
            (
                DecoratorId(id),
                storage.lookup_storage(DecoratorId(id)).unwrap(),
            )
        })
        .collect();
    for frame in 0..2 {
        for &(id, address) in &addresses {
            let bucket = storage.get_mut(address).unwrap();
            bucket.modified_ids.insert(id.0);
            bucket.removed_ids.insert(id.0 + 1000 + frame);
            assert_eq!(bucket.get(id).unwrap().decorator_id, id.0);
        }
        storage
            .removed
            .insert(DecoratorStorageAddr::Wall(WallId(99)), HashSet::from([77]));
        storage.clear_modification_tracking();
        assert!(storage.removed.is_empty());
        assert_eq!(
            storage.per_wall.len(),
            2,
            "same low 32 bits are distinct walls"
        );
        assert_eq!(
            storage.per_wall.capacity(),
            wall_capacity,
            "frame clear retains buckets"
        );
        assert!(storage.terrain.is_some() && storage.stairs.is_some());
        for &(id, address) in &addresses {
            assert_eq!(storage.lookup_storage(id), Some(address));
            let bucket = storage.get(address).unwrap();
            assert!(bucket.modified_ids.is_empty() && bucket.removed_ids.is_empty());
            assert_eq!(bucket.decorators.len(), 1);
            let original = before.iter().find(|(addr, _)| *addr == address).unwrap();
            assert_eq!(bucket.get(id), Some(&original.1[0].1));
        }
    }
}

#[test]
fn immutable_guard_fails_before_clearing_any_bucket_or_removed_registry() {
    let mut storage = populated_storage();
    storage.mark_immutable();
    let before = storage.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        storage.clear_modification_tracking();
    }));
    assert!(result.is_err());
    assert!(storage.immutable);
    assert_eq!(storage.removed, before.removed);
    for (address, items) in before.archive_current_decorators() {
        let bucket = storage.get(address).unwrap();
        let old_bucket = before.get(address).unwrap();
        assert_eq!(bucket.modified_ids, old_bucket.modified_ids);
        assert_eq!(bucket.removed_ids, old_bucket.removed_ids);
        for (id, info) in items {
            assert_eq!(bucket.get(DecoratorId(id)), Some(&info));
            assert_eq!(storage.lookup_storage(DecoratorId(id)), Some(address));
        }
    }
}
