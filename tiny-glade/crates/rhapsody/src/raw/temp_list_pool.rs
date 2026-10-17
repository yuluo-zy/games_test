//! Bounded CPU-only lists. Each checkout owns its list and returns its capacity
//! after all values have been dropped or transferred to actual GPU owners.
use super::{
    allocator::{AllocatorStats, CpuMemoryReservation},
    temp_list::TempList,
};
use crate::{
    Error, Result,
    device::{BudgetAction, BudgetCategory},
};
use parking_lot::Mutex;
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
struct State<T> {
    idle: Vec<TempList<T>>,
    stats: TempListPoolStats,
}
struct Shared<T> {
    state: Mutex<State<T>>,
    config: TempListPoolConfig,
    budget: Arc<AllocatorStats>,
    _metadata: CpuMemoryReservation,
}
pub struct TempListPool<T> {
    shared: Arc<Shared<T>>,
}
impl<T> Clone for TempListPool<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}
impl<T> TempListPool<T> {
    pub fn new(config: TempListPoolConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        if !(1..=256).contains(&config.max_lists)
            || config.max_items == 0
            || config.max_cached_bytes > config.max_total_bytes
        {
            return Err(Error::invalid_usage(
                "invalid bounded CPU list pool configuration",
            ));
        }
        let mut idle = Vec::new();
        idle.try_reserve_exact(config.max_lists)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes = idle
            .capacity()
            .saturating_mul(std::mem::size_of::<TempList<T>>())
            .saturating_add(std::mem::size_of::<Shared<T>>())
            .saturating_add(2 * std::mem::size_of::<usize>())
            .saturating_add(std::mem::align_of::<Shared<T>>());
        if metadata_bytes >= config.max_total_bytes {
            return Err(Error::invalid_usage(
                "CPU list pool cannot hold its metadata",
            ));
        }
        let memory = budget.reserve_cpu_bytes(metadata_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    idle,
                    stats: TempListPoolStats {
                        metadata_bytes,
                        ..Default::default()
                    },
                }),
                config,
                budget,
                _metadata: memory,
            }),
        })
    }
    pub fn statistics(&self) -> TempListPoolStats {
        self.shared.state.lock().stats
    }
    pub fn checkout(&self) -> Result<TempListLease<T>> {
        let mut state = self.shared.state.lock();
        let mut list = if let Some(list) = state.idle.pop() {
            state.stats.cached_lists -= 1;
            state.stats.cached_bytes -= list.capacity_bytes();
            list
        } else {
            if state.stats.checked_out >= self.shared.config.max_lists {
                state.stats.capacity_stalls += 1;
                return Err(Error::Timeout);
            }
            TempList::new(self.shared.config.max_items, self.shared.budget.clone())?
        };
        list.reserve(0)?;
        state.stats.checked_out += 1;
        Ok(TempListLease {
            list: Some(list),
            shared: self.shared.clone(),
        })
    }
    pub fn trim_idle(&self, target_bytes: usize) -> usize {
        let mut state = self.shared.state.lock();
        let mut freed = 0;
        while state.stats.cached_bytes > target_bytes {
            let Some(list) = state.idle.pop() else {
                break;
            };
            let bytes = list.capacity_bytes();
            drop(list);
            state.stats.bytes -= bytes;
            state.stats.cached_bytes -= bytes;
            state.stats.cached_lists -= 1;
            freed += bytes;
        }
        freed
    }
}
pub struct TempListLease<T> {
    list: Option<TempList<T>>,
    shared: Arc<Shared<T>>,
}
impl<T> TempListLease<T> {
    pub fn reserve(&mut self, needed: usize) -> Result<()> {
        let list = self.list.as_mut().expect("active CPU list lease");
        if needed <= list.capacity() {
            return list.reserve(needed);
        }
        if needed > self.shared.config.max_items {
            return Err(Error::Timeout);
        }
        let new_capacity = needed
            .max(list.capacity().saturating_mul(2))
            .min(self.shared.config.max_items);
        let desired = std::alloc::Layout::array::<T>(new_capacity)
            .map_err(|_| Error::invalid_usage("CPU list pool layout overflows"))?
            .size();
        let mut state = self.shared.state.lock();
        // Growth temporarily holds old and replacement blocks simultaneously.
        // Enforce this peak locally as well as in the shared CPU budget.
        if state
            .stats
            .bytes
            .saturating_add(state.stats.metadata_bytes)
            .saturating_add(desired)
            > self.shared.config.max_total_bytes
        {
            state.stats.capacity_stalls += 1;
            return Err(Error::Timeout);
        }
        let previous = list.capacity_bytes();
        let available =
            self.shared.config.max_total_bytes - state.stats.bytes - state.stats.metadata_bytes;
        let result = list.reserve_capped(needed, available);
        let actual = list.capacity_bytes();
        state.stats.bytes = state.stats.bytes - previous + actual;
        state.stats.capacity_growths += u64::from(actual > previous);
        result
    }
    pub fn add(&mut self, item: T) -> Result<()> {
        let needed = self
            .list
            .as_ref()
            .expect("active CPU list lease")
            .as_slice()
            .len()
            .checked_add(1)
            .ok_or_else(|| Error::invalid_usage("CPU list length overflows"))?;
        self.reserve(needed)?;
        self.list
            .as_mut()
            .expect("active CPU list lease")
            .add(item)?;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.list
            .as_ref()
            .expect("active CPU list lease")
            .as_slice()
            .is_empty()
    }
    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.list.as_mut().expect("active CPU list lease").drain()
    }
    pub fn as_slice(&self) -> &[T] {
        self.list
            .as_ref()
            .expect("active CPU list lease")
            .as_slice()
    }
}
impl<T> Drop for TempListLease<T> {
    fn drop(&mut self) {
        let mut list = self.list.take().expect("active CPU list lease");
        list.clear();
        let bytes = list.capacity_bytes();
        let mut state = self.shared.state.lock();
        state.stats.checked_out -= 1;
        if state.stats.cached_bytes.saturating_add(bytes) <= self.shared.config.max_cached_bytes
            && !matches!(
                self.shared.budget.budget_action(),
                BudgetAction::EvictCache | BudgetAction::Backpressure
            )
        {
            state.stats.cached_bytes += bytes;
            state.stats.cached_lists += 1;
            state.idle.push(list);
        } else {
            drop(list);
            state.stats.bytes -= bytes;
        }
    }
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
        assert!(matches!(pool.checkout(), Err(Error::Timeout)));
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
        assert!(matches!(list.reserve(65), Err(Error::Timeout)));
        budget.set_total_budget(0).unwrap();
        drop(list);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}
