//! Bounded CPU-only lists. Each checkout owns its list and returns its capacity
//! after all values have been dropped or transferred to actual GPU owners.
use super::{
    allocator::AllocatorStats,
    temp_list::TempList,
};
use crate::{
    Error, Result,
    device::BudgetCategory,
    collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue},
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct TempListPoolConfig {
    pub max_lists: usize,
    pub max_items: usize,
    pub max_total_bytes: usize,
    pub max_cached_bytes: usize,
}
impl Default for TempListPoolConfig {
    fn default() -> Self {
        Self {
            max_lists: 64,
            max_items: 65536,
            max_total_bytes: 16 << 20,
            max_cached_bytes: 4 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TempListPoolStats {
    pub checked_out: usize,
    pub cached_lists: usize,
    pub bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub capacity_stalls: u64,
    pub capacity_growths: u64,
}
impl<T> CpuPoolValue for TempList<T> {
    type Statistics = ();
    fn capacity_bytes(&self) -> usize { self.capacity_bytes() }
    fn reset(&mut self) { self.clear(); }
    fn reclassify(&mut self, category: BudgetCategory) { self.reclassify(category); }
}
pub struct TempListPool<T> {
    pool: CpuPool<TempList<T>>,
    config: TempListPoolConfig,
    budget: Arc<AllocatorStats>,
}
impl<T> Clone for TempListPool<T> {
    fn clone(&self) -> Self {
        Self { pool: self.pool.clone(), config: self.config, budget: self.budget.clone() }
    }
}
impl<T> TempListPool<T> {
    pub fn new(config: TempListPoolConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        if config.max_items == 0 {
            return Err(Error::invalid_usage("CPU list limit must be nonzero"));
        }
        let pool = CpuPool::new(CpuPoolLimits {
            items: config.max_lists, total_bytes: config.max_total_bytes,
            cached_bytes: config.max_cached_bytes,
            full: || Error::exhausted("CPU list pool capacity exceeded"),
        }, budget.clone())?;
        Ok(Self { pool, config, budget })
    }
    pub fn statistics(&self) -> TempListPoolStats {
        let stats = self.pool.stats();
        TempListPoolStats {
            checked_out: stats.checked_out, cached_lists: stats.cached,
            bytes: stats.live_bytes + stats.cached_bytes, cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes, capacity_stalls: stats.capacity_stalls,
            capacity_growths: stats.capacity_growths,
        }
    }
    pub fn checkout(&self) -> Result<TempListLease<T>> {
        Ok(TempListLease {
            lease: self.pool.checkout(|| TempList::new(self.config.max_items, self.budget.clone()))?,
            max_items: self.config.max_items,
        })
    }
    pub fn trim_idle(&self, target_bytes: usize) -> usize { self.pool.trim_idle(target_bytes) }
}
pub struct TempListLease<T> {
    lease: CpuPoolLease<TempList<T>>,
    max_items: usize,
}
impl<T> TempListLease<T> {
    pub fn reserve(&mut self, needed: usize) -> Result<()> {
        let list = self.lease.value();
        if needed > self.max_items {
            return Err(Error::exhausted("CPU list pool capacity exceeded"));
        }
        if needed <= list.capacity() { return Ok(()); }
        let new_capacity = needed.max(list.capacity().saturating_mul(2)).min(self.max_items);
        let desired = std::alloc::Layout::array::<T>(new_capacity)
            .map_err(|_| Error::invalid_usage("CPU list pool layout overflows"))?.size();
        self.lease.update(desired, |list, available| list.reserve_capped(needed, available))
    }
    pub fn add(&mut self, item: T) -> Result<()> {
        let needed = self.as_slice().len().checked_add(1)
            .ok_or_else(|| Error::invalid_usage("CPU list length overflows"))?;
        self.reserve(needed)?;
        self.lease.value_mut().add_reserved(item);
        Ok(())
    }
    pub fn is_empty(&self) -> bool { self.as_slice().is_empty() }
    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, T> { self.lease.value_mut().drain() }
    pub fn as_slice(&self) -> &[T] { self.lease.value().as_slice() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> TempListPoolConfig {
        TempListPoolConfig {
            max_lists: 2,
            max_items: 64,
            max_total_bytes: 16 << 10,
            max_cached_bytes: 8 << 10,
        }
    }
    #[test]
    fn returning_elements_can_reenter_the_pool_and_panics_release_the_checkout() {
        struct Reenter(TempListPool<Reenter>);
        impl Drop for Reenter {
            fn drop(&mut self) { drop(self.0.checkout().unwrap()); }
        }
        let budget = Arc::new(AllocatorStats::default());
        let pool = TempListPool::new(config(), budget.clone()).unwrap();
        let mut lease = pool.checkout().unwrap();
        lease.add(Reenter(pool.clone())).unwrap();
        drop(lease);
        assert_eq!(pool.statistics().checked_out, 0);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);

        struct Panics;
        impl Drop for Panics { fn drop(&mut self) { panic!("element drop"); } }
        let pool = TempListPool::new(config(), budget.clone()).unwrap();
        let mut lease = pool.checkout().unwrap();
        lease.add(Panics).unwrap();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(lease))).is_err());
        assert_eq!(pool.statistics().checked_out, 0);
        assert_eq!(pool.statistics().bytes, 0);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn zero_sized_values_still_obey_the_item_limit() {
        let pool = TempListPool::<()>::new(config(), Arc::new(AllocatorStats::default())).unwrap();
        let mut lease = pool.checkout().unwrap();
        for _ in 0..64 { lease.add(()).unwrap(); }
        assert!(matches!(lease.add(()), Err(Error::Exhausted(_))));
        assert_eq!(lease.as_slice().len(), 64);
    }
    #[test]
    fn destructible_owner_leases_reuse_capacity_and_budget_until_last_cpu_drop() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = TempListPool::<Arc<()>>::new(config(), budget.clone()).unwrap();
        let owner = Arc::new(());
        let mut first = pool.checkout().unwrap();
        first.reserve(16).unwrap();
        first.add(owner.clone()).unwrap();
        let pointer = first.as_slice().as_ptr();
        assert_eq!(Arc::strong_count(&owner), 2);
        let second = pool.checkout().unwrap();
        assert!(matches!(pool.checkout(), Err(Error::Exhausted(_))));
        drop(first);
        assert_eq!(Arc::strong_count(&owner), 1);
        for _ in 0..1000 {
            let mut list = pool.checkout().unwrap();
            list.add(owner.clone()).unwrap();
            assert_eq!(list.as_slice().as_ptr(), pointer);
            let transferred = list.drain().next().unwrap();
            drop(list);
            assert_eq!(Arc::strong_count(&owner), 2);
            drop(transferred);
        }
        assert_eq!(pool.statistics().capacity_growths, 1);
        assert!(pool.trim_idle(0) > 0);
        let metadata = pool.statistics().metadata_bytes;
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), metadata as u64);
        drop(second);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn local_limits_and_growth_oom_keep_previous_values_and_credit() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = TempListPool::<u64>::new(config(), budget.clone()).unwrap();
        let mut list = pool.checkout().unwrap();
        list.add(7).unwrap();
        let before = budget.cpu_memory_usage().total();
        budget.set_total_budget(before).unwrap();
        assert!(matches!(list.reserve(32), Err(Error::OutOfHostMemory)));
        assert_eq!(list.as_slice(), [7]);
        assert_eq!(budget.cpu_memory_usage().total(), before);
        assert!(matches!(list.reserve(65), Err(Error::Exhausted(_))));
        budget.set_total_budget(0).unwrap();
        drop(list);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}
