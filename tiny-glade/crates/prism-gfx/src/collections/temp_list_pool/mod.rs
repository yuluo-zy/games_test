//! Bounded CPU-only lists. Each checkout owns its list and returns its capacity
//! after all values have been dropped or transferred to actual GPU owners.
use crate::collections::temp_list::TempList;
use crate::accounting::AllocatorStats;
use crate::{Error, Result, collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue}};
use crate::accounting::BudgetCategory;
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
    #[doc(hidden)]
    pub fn drain(&mut self) -> std::vec::Drain<'_, T> { self.lease.value_mut().drain() }
    pub fn as_slice(&self) -> &[T] { self.lease.value().as_slice() }
}

#[cfg(test)]
mod tests;
