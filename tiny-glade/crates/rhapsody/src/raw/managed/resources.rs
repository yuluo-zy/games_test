//! Device-local weak resource indexes using the public typed generational store.
use super::handle::{Managed, ManagedInner};
use crate::{
    Error, Result,
    device::BudgetCategory,
    raw::allocator::{AllocatorStats, CpuMemoryReservation, shared_cpu_bytes},
    resource::{Handle, ResourceStore},
};
use parking_lot::Mutex;
use std::{
    cmp::Ordering as Comparison,
    fmt,
    hash::{Hash, Hasher},
    sync::{
        Arc, Weak,
        atomic::{AtomicU64, Ordering},
    },
};
static NEXT_STORE: AtomicU64 = AtomicU64::new(1);

/// A typed slotmap key qualified by its store. Slotmap keys alone can have the
/// same bits in two stores; their store identity must also match.
pub struct ManagedKey<T> {
    store: u64,
    handle: Handle<T>,
}
impl<T> ManagedKey<T> {
    pub fn handle(self) -> Handle<T> {
        self.handle
    }
    pub fn index(self) -> u32 {
        self.handle.index()
    }
    pub fn store(self) -> u64 {
        self.store
    }
}
impl<T> Copy for ManagedKey<T> {}
impl<T> Clone for ManagedKey<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> PartialEq for ManagedKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.store == other.store && self.handle == other.handle
    }
}
impl<T> Eq for ManagedKey<T> {}
impl<T> PartialOrd for ManagedKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Comparison> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for ManagedKey<T> {
    fn cmp(&self, other: &Self) -> Comparison {
        (self.store, self.handle).cmp(&(other.store, other.handle))
    }
}
impl<T> Hash for ManagedKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.store.hash(state);
        self.handle.hash(state);
    }
}
impl<T> fmt::Debug for ManagedKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ManagedKey")
            .field("store", &self.store)
            .field("handle", &self.handle)
            .finish()
    }
}

/// The limit applies to live entries; freed generations reuse the same backing.
const DEFAULT_CAPACITY_LIMIT: usize = 65_535;

struct ManagedSlots<T: Send + Sync + 'static> {
    // Drop the actual backing before returning its cache credit.
    slots: ResourceStore<Weak<ManagedInner<T>>>,
    memory: CpuMemoryReservation,
}
impl<T: Send + Sync + 'static> ManagedSlots<T> {
    fn backing_bytes(capacity: usize) -> Result<u64> {
        // slotmap 1.1.1 stores a value/free-list union and u32 generation per
        // slot, including its reserved sentinel. Weak is wider than u32.
        let (layout, _) = std::alloc::Layout::new::<Weak<ManagedInner<T>>>()
            .extend(std::alloc::Layout::new::<u32>())
            .map_err(|_| Error::OutOfHostMemory)?;
        capacity
            .checked_add(1)
            .and_then(|count| count.checked_mul(layout.pad_to_align().size()))
            .and_then(|bytes| u64::try_from(bytes).ok())
            .ok_or(Error::OutOfHostMemory)
    }
    fn reserve_one(&mut self, limit: usize) -> Result<()> {
        if self.slots.len() >= limit {
            return Err(Error::Timeout);
        }
        if self.slots.len() < self.slots.capacity() {
            return Ok(());
        }
        let old_bytes = Self::backing_bytes(self.slots.capacity())?;
        // Request the geometric Vec growth explicitly. The sentinel is part of
        // the allocation; minimum backing capacity for these slots is four.
        let maximum = limit
            .checked_add(1)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(Error::OutOfHostMemory)?
            .max(4);
        let backing_capacity = self
            .slots
            .capacity()
            .checked_add(1)
            .and_then(|n| n.checked_mul(2))
            .ok_or(Error::OutOfHostMemory)?
            .clamp(4, maximum);
        self.memory
            .resize(Self::backing_bytes(backing_capacity - 1)?)?;
        if self
            .slots
            .try_reserve(backing_capacity - 1 - self.slots.len())
            .is_err()
        {
            self.memory.resize(old_bytes)?;
            return Err(Error::OutOfHostMemory);
        }
        self.memory
            .resize(Self::backing_bytes(self.slots.capacity())?)?;
        Ok(())
    }
}
pub(crate) struct ResourceState<T: Send + Sync + 'static> {
    identity: u64,
    capacity_limit: usize,
    slots: Mutex<ManagedSlots<T>>,
    stats: Arc<AllocatorStats>,
    _memory: CpuMemoryReservation,
}
impl<T: Send + Sync + 'static> ResourceState<T> {
    pub(crate) fn remove(&self, key: ManagedKey<T>) -> Option<Weak<ManagedInner<T>>> {
        if key.store != self.identity {
            return None;
        }
        self.slots
            .lock()
            .slots
            .remove(Handle::from_ffi(key.handle.as_ffi()))
    }
}

/// The Device owns this index, while each slot holds only a Weak reference.
/// Raw resources own their payload reservations and may own Arc<Device> without
/// forming a resource-store cycle. This index charges only its own allocations.
pub struct ResourceManager<T: Send + Sync + 'static> {
    state: Arc<ResourceState<T>>,
}
impl<T: Send + Sync + 'static> Clone for ResourceManager<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}
impl<T: Send + Sync + 'static> ResourceManager<T> {
    pub fn new(stats: Arc<AllocatorStats>) -> Result<Self> {
        Self::with_capacity_limit(stats, DEFAULT_CAPACITY_LIMIT)
    }
    pub fn with_capacity_limit(stats: Arc<AllocatorStats>, capacity_limit: usize) -> Result<Self> {
        if capacity_limit == 0 || capacity_limit >= u32::MAX as usize - 1 {
            return Err(Error::invalid_usage(
                "invalid managed resource capacity limit",
            ));
        }
        let identity = NEXT_STORE
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .map_err(|_| Error::unsupported("resource store identity exhausted"))?;
        let memory = stats.reserve_cpu_bytes(
            shared_cpu_bytes::<ResourceState<T>>()?,
            BudgetCategory::Live,
        )?;
        let table_memory =
            stats.reserve_cpu_bytes(ManagedSlots::<T>::backing_bytes(0)?, BudgetCategory::Cache)?;
        Ok(Self {
            state: Arc::new(ResourceState {
                identity,
                capacity_limit,
                slots: Mutex::new(ManagedSlots {
                    slots: ResourceStore::new(),
                    memory: table_memory,
                }),
                stats,
                _memory: memory,
            }),
        })
    }
    pub fn identity(&self) -> u64 {
        self.state.identity
    }
    pub fn len(&self) -> usize {
        self.state.slots.lock().slots.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Index an owned value. Resource-specific payload accounting belongs to T;
    /// in production Buffer/Texture carry it through every escaping Arc<T>.
    pub fn allocate(&self, resource: T) -> Result<Managed<T>> {
        self.allocate_shared(Arc::new(resource))
    }
    pub fn allocate_shared(&self, resource: Arc<T>) -> Result<Managed<T>> {
        let mut slots = self.state.slots.lock();
        if slots.slots.len() >= self.state.capacity_limit {
            return Err(Error::Timeout);
        }
        let memory = self
            .state
            .stats
            .reserve_cpu_bytes(shared_cpu_bytes::<ManagedInner<T>>()?, BudgetCategory::Live)?;
        slots.reserve_one(self.state.capacity_limit)?;
        let slot = slots.slots.insert(Weak::new());
        let key = ManagedKey {
            store: self.state.identity,
            handle: Handle::from_ffi(slot.as_ffi()),
        };
        let inner = Arc::new(ManagedInner {
            key,
            resource,
            state: self.state.clone(),
            views: Default::default(),
            _memory: memory,
        });
        *slots.slots.get_mut(slot).expect("new typed slot exists") = Arc::downgrade(&inner);
        Ok(Managed(inner))
    }
    pub fn get(&self, key: ManagedKey<T>) -> Option<Managed<T>> {
        if key.store != self.state.identity {
            return None;
        }
        let inner = self
            .state
            .slots
            .lock()
            .slots
            .get(Handle::from_ffi(key.handle.as_ffi()))?
            .upgrade()?;
        (inner.key == key).then_some(Managed(inner))
    }
}

#[derive(Clone)]
pub struct ManagedResources {
    pub buffers: ResourceManager<crate::raw::buffer::Buffer>,
    pub textures: ResourceManager<crate::raw::texture::Texture>,
}
impl ManagedResources {
    pub fn new(stats: Arc<AllocatorStats>) -> Result<Self> {
        Ok(Self {
            buffers: ResourceManager::new(stats.clone())?,
            textures: ResourceManager::new(stats)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manager_creation_rejects_cpu_budget_exhaustion_without_a_charge() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_total_budget(1).unwrap();
        assert!(matches!(
            ResourceManager::<u32>::new(stats.clone()),
            Err(Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn metadata_charges_follow_live_inners_and_cached_slot_capacity() {
        let stats = Arc::new(AllocatorStats::default());
        let store = ResourceManager::with_capacity_limit(stats.clone(), 2).unwrap();
        let initial = stats.cpu_memory_usage();
        assert!(initial.live > 0 && initial.cache > 0);
        let first = store.allocate(1u32).unwrap();
        let warm = stats.cpu_memory_usage();
        assert_eq!(
            warm.live - initial.live,
            shared_cpu_bytes::<ManagedInner<u32>>().unwrap()
        );
        assert!(warm.cache > initial.cache);
        let clone = first.clone();
        drop(first);
        assert_eq!(stats.cpu_memory_usage(), warm);
        drop(clone);
        assert_eq!(stats.cpu_memory_usage().live, initial.live);
        assert_eq!(stats.cpu_memory_usage().cache, warm.cache);
        for _ in 0..1000 {
            drop(store.allocate(7u32).unwrap());
        }
        assert_eq!(stats.cpu_memory_usage().live, initial.live);
        assert_eq!(stats.cpu_memory_usage().cache, warm.cache);
        drop(store);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn capacity_backpressure_and_cpu_oom_do_not_publish_a_slot() {
        let stats = Arc::new(AllocatorStats::default());
        let store = ResourceManager::with_capacity_limit(stats.clone(), 1).unwrap();
        let first = store.allocate(1u32).unwrap();
        assert!(matches!(store.allocate(2), Err(Error::Timeout)));
        let key = first.key();
        drop(first);
        let used = stats.cpu_memory_usage().total();
        stats.set_total_budget(used).unwrap();
        assert!(matches!(store.allocate(3), Err(Error::OutOfHostMemory)));
        assert!(store.is_empty());
        assert!(store.get(key).is_none());
        assert_eq!(stats.cpu_memory_usage().total(), used);
        stats.set_total_budget(used + 4096).unwrap();
        let replacement = store.allocate(4).unwrap();
        assert_eq!(replacement.key().index(), key.index());
        assert_ne!(replacement.key(), key);
    }
    #[test]
    fn rejected_slot_growth_preserves_the_previous_capacity_and_credit() {
        let stats = Arc::new(AllocatorStats::default());
        let store = ResourceManager::<u32>::new(stats.clone()).unwrap();
        let initial = stats.cpu_memory_usage();
        let growth = ManagedSlots::<u32>::backing_bytes(3).unwrap() - initial.cache;
        let inner = shared_cpu_bytes::<ManagedInner<u32>>().unwrap();
        stats
            .set_total_budget(initial.total() + inner + growth - 1)
            .unwrap();
        assert!(matches!(store.allocate(1), Err(Error::OutOfHostMemory)));
        assert!(store.is_empty());
        assert_eq!(store.state.slots.lock().slots.capacity(), 0);
        assert_eq!(stats.cpu_memory_usage(), initial);
    }
    #[test]
    fn rejected_allocation_drops_payload_after_releasing_the_store_lock() {
        struct Reenter {
            store: ResourceManager<Self>,
            dropped: Arc<AtomicU64>,
        }
        impl Drop for Reenter {
            fn drop(&mut self) {
                let _ = self.store.len();
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
        let store =
            ResourceManager::with_capacity_limit(Arc::new(AllocatorStats::default()), 1).unwrap();
        let dropped = Arc::new(AtomicU64::new(0));
        let create = || Reenter {
            store: store.clone(),
            dropped: dropped.clone(),
        };
        let first = store.allocate(create()).unwrap();
        assert!(matches!(store.allocate(create()), Err(Error::Timeout)));
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
        drop(first);
        assert_eq!(dropped.load(Ordering::Relaxed), 2);
    }
    #[test]
    fn typed_keys_reject_another_store_even_when_slot_bits_match() {
        let a = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let b = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let first = a.allocate(7u32).unwrap();
        let second = b.allocate(8u32).unwrap();
        assert_eq!(first.key().handle(), second.key().handle());
        assert_ne!(first.key(), second.key());
        assert!(a.get(second.key()).is_none());
        assert!(b.get(first.key()).is_none());
        assert_eq!(*a.clone().get(first.key()).unwrap(), 7);
    }
    #[test]
    fn only_last_arc_release_invalidates_the_generation() {
        let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let first = store.allocate(3u32).unwrap();
        let key = first.key();
        let second = first.clone();
        drop(first);
        assert_eq!(*store.get(key).unwrap(), 3);
        drop(second);
        assert!(store.is_empty());
        assert!(store.get(key).is_none());
        let next = store.allocate(4u32).unwrap();
        assert_eq!(next.key().index(), key.index());
        assert_ne!(next.key().handle(), key.handle());
        assert!(store.get(key).is_none());
    }
    #[test]
    fn typed_key_retrieval_keeps_the_raw_resource_alive_until_all_owners_leave() {
        let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let value = Arc::new(11u32);
        let weak = Arc::downgrade(&value);
        let first = store.allocate_shared(value).unwrap();
        let second = store.get(first.key()).unwrap();
        let raw = second.shared();
        let key = first.key();
        drop(first);
        drop(second);
        assert!(store.get(key).is_none());
        assert_eq!(*raw, 11);
        drop(raw);
        assert!(weak.upgrade().is_none());
    }
    #[test]
    fn a_thousand_resource_batches_reuse_bounded_slot_storage() {
        let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let warm: Vec<_> = (0..8u32)
            .map(|value| store.allocate(value).unwrap())
            .collect();
        drop(warm);
        let capacity = store.state.slots.lock().slots.capacity();
        for _ in 0..1000 {
            let batch: Vec<_> = (0..8u32)
                .map(|value| store.allocate(value).unwrap())
                .collect();
            let keys: Vec<_> = batch.iter().map(Managed::key).collect();
            drop(batch);
            assert!(store.is_empty());
            for key in keys {
                assert!(store.get(key).is_none());
            }
            assert_eq!(store.state.slots.lock().slots.capacity(), capacity);
        }
    }
    #[test]
    fn store_removal_does_not_hold_the_lock_during_resource_drop() {
        struct Reenter {
            store: ResourceManager<Self>,
            dropped: Arc<AtomicU64>,
        }
        impl Drop for Reenter {
            fn drop(&mut self) {
                assert!(self.store.is_empty());
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
        let store = ResourceManager::new(Arc::new(AllocatorStats::default())).unwrap();
        let dropped = Arc::new(AtomicU64::new(0));
        let value = store
            .allocate(Reenter {
                store: store.clone(),
                dropped: dropped.clone(),
            })
            .unwrap();
        drop(value);
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
    }
}
