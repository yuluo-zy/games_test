//! Narrow consumer adapter over the existing bounded CPU pool. A device owns
//! each pool; neither storage nor reservations are transferred between devices.
use crate::{
    Error, Result,
    collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue},
    device::{AllocatorStats, BudgetAction, BudgetCategory},
    raw::allocator::CpuMemoryReservation,
};
use std::sync::Arc;

pub trait CpuScratchValue: Default {
    /// The default must own no heap allocations. Report real capacities.
    fn capacity_bytes(&self) -> usize;
    /// Drop elements/graph identities while retaining admitted backing.
    fn reset(&mut self);
}
struct Accounted<T> {
    value: T,
    memory: CpuMemoryReservation,
}
impl<T: CpuScratchValue> CpuPoolValue for Accounted<T> {
    type Statistics = ();
    fn capacity_bytes(&self) -> usize {
        self.memory.bytes() as usize
    }
    fn reset(&mut self) {
        self.value.reset();
    }
    fn reclassify(&mut self, category: BudgetCategory) {
        self.memory.reclassify(category);
    }
}

pub struct CpuScratchPool<T: CpuScratchValue> {
    pool: CpuPool<Accounted<T>>,
    budget: Arc<AllocatorStats>,
    billing: std::sync::atomic::AtomicBool,
    checkouts: std::sync::atomic::AtomicU64,
    _wrapper_memory: CpuMemoryReservation,
}
#[derive(Clone, Copy, Default, Debug, serde::Serialize)]
pub struct CpuScratchStats {
    pub checked_out: usize,
    pub cached: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub high_water_bytes: usize,
    pub checkouts_high_water: usize,
    pub creations: u64,
    pub capacity_growths: u64,
    /// Pool slot/local-capacity refusals; global host-budget rejections are
    /// returned as errors and are not included in this counter.
    pub pool_capacity_stalls: u64,
    pub checkouts: u64,
}
impl<T: CpuScratchValue> CpuScratchPool<T> {
    pub fn new(
        budget: Arc<AllocatorStats>,
        max_leases: usize,
        total_bytes: usize,
        cached_bytes: usize,
    ) -> Result<Self> {
        let wrapper_bytes = std::mem::size_of::<Self>();
        let inner_total = total_bytes
            .checked_sub(wrapper_bytes)
            .ok_or(Error::OutOfHostMemory)?;
        let wrapper_memory =
            budget.reserve_cpu_bytes(wrapper_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            pool: CpuPool::new(
                CpuPoolLimits {
                    items: max_leases,
                    total_bytes: inner_total,
                    cached_bytes,
                    full: || Error::exhausted("consumer CPU scratch pool is full"),
                },
                budget.clone(),
            )?,
            budget,
            billing: std::sync::atomic::AtomicBool::new(false),
            checkouts: std::sync::atomic::AtomicU64::new(0),
            _wrapper_memory: wrapper_memory,
        })
    }
    pub fn checkout(&self) -> Result<CpuScratchLease<T>> {
        if self.billing.load(std::sync::atomic::Ordering::Relaxed) {
            self.checkouts
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        if matches!(
            self.budget.budget_action(),
            BudgetAction::EvictCache | BudgetAction::Backpressure
        ) {
            self.pool.trim_idle(0);
        }
        let lease = self.pool.checkout(|| {
            let value = T::default();
            if value.capacity_bytes() != 0 {
                return Err(Error::invalid_usage("CPU scratch default must be empty"));
            }
            Ok(Accounted {
                value,
                memory: self.budget.reserve_cpu_bytes(0, BudgetCategory::Live)?,
            })
        })?;
        Ok(CpuScratchLease { lease })
    }
    pub fn trim(&self) -> usize {
        self.pool.trim_idle(0)
    }
    pub fn set_billing(&self, enabled: bool) {
        self.billing
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn stats(&self) -> CpuScratchStats {
        let stats = self.pool.stats();
        CpuScratchStats {
            checked_out: stats.checked_out,
            cached: stats.cached,
            live_bytes: stats.live_bytes,
            cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes + self._wrapper_memory.bytes() as usize,
            high_water_bytes: stats.peak_total_bytes + self._wrapper_memory.bytes() as usize,
            checkouts_high_water: stats.peak_checked_out,
            creations: stats.creations,
            capacity_growths: stats.capacity_growths,
            pool_capacity_stalls: stats.capacity_stalls,
            checkouts: self.checkouts.load(std::sync::atomic::Ordering::Relaxed),
        }
    }
}
pub struct CpuScratchLease<T: CpuScratchValue> {
    lease: CpuPoolLease<Accounted<T>>,
}
impl<T: CpuScratchValue> CpuScratchLease<T> {
    pub fn value(&self) -> &T {
        &self.lease.value().value
    }
    /// Only capacity preparation belongs here; no host/native callback runs
    /// while the recycler admission lock is held. `peak_extra` includes the
    /// entire new blocks while old backing can still coexist during growth.
    pub fn prepare<R>(
        &mut self,
        peak_extra: usize,
        allocate: impl FnOnce(&mut T) -> Result<R>,
    ) -> Result<R> {
        let mut panic = None;
        let result = self.lease.update(peak_extra, |owner, _| {
            let previous = owner.memory.bytes();
            let admitted = previous
                .checked_add(peak_extra as u64)
                .ok_or(Error::OutOfHostMemory)?;
            owner.memory.resize(admitted)?;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                allocate(&mut owner.value)
            }));
            let result = match result {
                Ok(result) => result,
                Err(payload) => {
                    panic = Some(payload);
                    Err(Error::invalid_usage("CPU scratch preparation panicked"))
                }
            };
            let actual = owner.value.capacity_bytes() as u64;
            if actual > admitted {
                // A mis-sized growth must not retain uncredited storage. Drop
                // it before restoring credit; later maintenance can retry.
                let old = std::mem::take(&mut owner.value);
                drop(old);
                owner.memory.resize(0)?;
                return Err(Error::exhausted("CPU scratch exceeded admitted capacity"));
            }
            owner.memory.resize(actual)?;
            result
        });
        // Complete CpuPool's capacity reconciliation before propagating an
        // unwind; otherwise its live-byte counters could retain the old size.
        if let Some(payload) = panic {
            std::panic::resume_unwind(payload);
        }
        result
    }
    /// Work uses the previously prepared capacities. Callers must not grow
    /// heap backing here; declaration/host callbacks run outside pool locks.
    pub fn with_prepared<R>(&mut self, run: impl FnOnce(&mut T) -> R) -> R {
        run(&mut self.lease.value_mut().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Scratch(Vec<u32>);
    impl CpuScratchValue for Scratch {
        fn capacity_bytes(&self) -> usize {
            self.0.capacity() * std::mem::size_of::<u32>()
        }
        fn reset(&mut self) {
            self.0.clear();
        }
    }
    #[test]
    fn growth_is_admitted_before_allocation_and_values_clear_on_cancel() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = CpuScratchPool::<Scratch>::new(stats.clone(), 2, 1 << 20, 1 << 19).unwrap();
        let metadata = stats.cpu_memory_usage().total();
        let mut first = pool.checkout().unwrap();
        stats.set_host_budget(metadata).unwrap();
        let mut called = false;
        assert!(matches!(
            first.prepare(256, |value| {
                called = true;
                value
                    .0
                    .try_reserve_exact(64)
                    .map_err(|_| Error::OutOfHostMemory)
            }),
            Err(Error::OutOfHostMemory)
        ));
        assert!(!called);
        assert_eq!(stats.cpu_memory_usage().total(), metadata);
        stats.set_host_budget(0).unwrap();
        first
            .prepare(256, |value| {
                value
                    .0
                    .try_reserve_exact(64)
                    .map_err(|_| Error::OutOfHostMemory)
            })
            .unwrap();
        first.with_prepared(|value| value.0.push(7));
        drop(first);
        let second = pool.checkout().unwrap();
        assert!(second.value().0.is_empty());
        assert!(second.value().0.capacity() >= 64);
        drop(second);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn pool_capacity_refusal_and_device_budgets_stay_independent() {
        let first_stats = Arc::new(AllocatorStats::default());
        let second_stats = Arc::new(AllocatorStats::default());
        let first =
            CpuScratchPool::<Scratch>::new(first_stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
        let second =
            CpuScratchPool::<Scratch>::new(second_stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
        let mut held = first.checkout().unwrap();
        held.prepare(256, |value| {
            value
                .0
                .try_reserve_exact(64)
                .map_err(|_| Error::OutOfHostMemory)
        })
        .unwrap();
        assert!(matches!(first.checkout(), Err(Error::Exhausted(_))));
        assert_eq!(second.stats().live_bytes, 0);
        assert_eq!(second.stats().cached_bytes, 0);
        assert_eq!(first.stats().live_bytes, 256);
        drop(held);
        first.trim();
        assert_eq!(first.stats().live_bytes + first.stats().cached_bytes, 0);
        drop(first);
        drop(second);
        assert_eq!(first_stats.cpu_memory_usage().total(), 0);
        assert_eq!(second_stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn preparation_panic_reconciles_credit_before_lease_unwind() {
        let stats = Arc::new(AllocatorStats::default());
        let pool = CpuScratchPool::<Scratch>::new(stats.clone(), 1, 1 << 20, 1 << 19).unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut lease = pool.checkout().unwrap();
            let _: Result<()> = lease.prepare(256, |value| {
                value
                    .0
                    .try_reserve_exact(64)
                    .map_err(|_| Error::OutOfHostMemory)?;
                value.0.push(17);
                panic!("injected capacity preparation failure");
            });
        }));
        assert!(result.is_err());
        assert_eq!(pool.stats().checked_out, 0);
        let lease = pool.checkout().unwrap();
        assert!(lease.value().0.is_empty());
        drop(lease);
        drop(pool);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
