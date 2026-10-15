//! B DecoratorArchivist::extract_and_restore_to_storage and tracked_clone_from.
//! Hash maps, sets, allocation and collision-handle drops use Rust/std owners.
use crate::recovered_building::building_wall_state::WallId;
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecoratorStorageAddress {
    Wall(WallId),
    ObservedSpecial1,
    ObservedSpecial2,
}
/// Active DecoratorInfo fields are carried unchanged; their render semantics
/// are not inferred from this storage consumer. Last byte2 is the original
/// Option niche encountered by the old-item iterator, outside valid live info.
#[derive(Clone, Debug)]
pub struct ObservedDecoratorInfo(pub [u8; 72]);
#[derive(Clone, Debug, Default)]
pub struct TrackedDecorators {
    pub removed: HashSet<u32>,
    pub items: HashMap<u32, ObservedDecoratorInfo>,
    pub added: HashSet<u32>,
}
#[derive(Clone, Debug, Default)]
pub struct DecoratorStorage {
    pub walls: HashMap<WallId, TrackedDecorators>,
    pub special1: Option<TrackedDecorators>,
    pub special2: Option<TrackedDecorators>,
    pub id_to_address: HashMap<u32, DecoratorStorageAddress>,
    /// get_or_insert_default takes the saved removed IDs for a newly reintroduced
    /// address. This set is not read when its Decorators object already exists.
    pub removed_by_address: HashMap<DecoratorStorageAddress, HashSet<u32>>,
}
#[derive(bevy_ecs::resource::Resource)]
pub struct DecoratorArchivist<C: Send + Sync + 'static> {
    pub walls: HashMap<WallId, TrackedDecorators>,
    pub special1: Option<TrackedDecorators>,
    pub special2: Option<TrackedDecorators>,
    pub collision_a: HashMap<u32, C>,
    pub collision_b: HashMap<u32, C>,
    pub culling: crate::resources::walls::decorator_archivist::DecoratorCullPolicy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecoratorRestoreError {
    MissingLiveIndex(u32),
    DuplicateLiveIndex(u32),
    InvalidLiveInfo(u32),
}
fn restore_one(
    storage: &mut DecoratorStorage,
    address: DecoratorStorageAddress,
    source: &TrackedDecorators,
) -> Result<(), DecoratorRestoreError> {
    let target = match address {
        DecoratorStorageAddress::Wall(wall) => {
            storage
                .walls
                .entry(wall)
                .or_insert_with(|| TrackedDecorators {
                    removed: storage
                        .removed_by_address
                        .remove(&address)
                        .unwrap_or_default(),
                    ..TrackedDecorators::default()
                })
        }
        DecoratorStorageAddress::ObservedSpecial1 => {
            storage.special1.get_or_insert_with(|| TrackedDecorators {
                removed: storage
                    .removed_by_address
                    .remove(&address)
                    .unwrap_or_default(),
                ..TrackedDecorators::default()
            })
        }
        DecoratorStorageAddress::ObservedSpecial2 => {
            storage.special2.get_or_insert_with(|| TrackedDecorators {
                removed: storage
                    .removed_by_address
                    .remove(&address)
                    .unwrap_or_default(),
                ..TrackedDecorators::default()
            })
        }
    };
    if target.removed.is_empty() && target.items.is_empty() && target.added.is_empty() {
        *target = source.clone();
    } else {
        target.added.clear();
        target.removed.extend(target.items.keys().copied());
        for (id, info) in &target.items {
            if info.0[71] == 2 {
                return Err(DecoratorRestoreError::InvalidLiveInfo(*id));
            }
            storage
                .id_to_address
                .remove(id)
                .ok_or(DecoratorRestoreError::MissingLiveIndex(*id))?;
        }
        target.items.clear();
        target.removed.extend(source.removed.iter().copied());
        for (id, info) in &source.items {
            target.items.insert(*id, info.clone());
            target.removed.remove(id);
            target.added.insert(*id);
        }
    }
    for id in source.items.keys() {
        if storage.id_to_address.insert(*id, address).is_some() {
            return Err(DecoratorRestoreError::DuplicateLiveIndex(*id));
        }
    }
    Ok(())
}
/// Actual order is wall-map drain, then special1, then special2. No ownership
/// notifications substitute for restoring the data and global live-ID index.
/// Invalid original panic domains return typed errors before committing state.
pub fn restore_archived_decorators<C: Send + Sync + 'static>(
    archive: &mut DecoratorArchivist<C>,
    storage: &mut DecoratorStorage,
) -> Result<(), DecoratorRestoreError> {
    let mut restored = storage.clone();
    for (wall, decorators) in &archive.walls {
        restore_one(
            &mut restored,
            DecoratorStorageAddress::Wall(*wall),
            decorators,
        )?;
    }
    if let Some(source) = &archive.special1 {
        restore_one(
            &mut restored,
            DecoratorStorageAddress::ObservedSpecial1,
            source,
        )?;
    }
    if let Some(source) = &archive.special2 {
        restore_one(
            &mut restored,
            DecoratorStorageAddress::ObservedSpecial2,
            source,
        )?;
    }
    archive.collision_a.clear();
    archive.collision_b.clear();
    archive.walls.clear();
    archive.special1 = None;
    archive.special2 = None;
    *storage = restored;
    Ok(())
}
