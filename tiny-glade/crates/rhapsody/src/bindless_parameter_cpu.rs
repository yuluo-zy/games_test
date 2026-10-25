//! Callback-local, bounded CPU credit for parameter payloads. Published
//! parameters pin the same block; their destruction never takes the budget lock.
use crate::{Error, Result, collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue},
    device::{BudgetAction, BudgetCategory},
    raw::allocator::{AllocatorStats, CpuMemoryReservation, reserve_shared_cpu_metadata, shared_cpu_bytes}};
use std::sync::Arc;

pub(crate) struct ParameterCpuBlock {
    credit_bytes: usize,
    accounted_bytes: usize,
    reservation: CpuMemoryReservation,
}
pub(crate) type ParameterCpuCredit = Arc<ParameterCpuBlock>;
impl CpuPoolValue for ParameterCpuCredit {
    type Statistics = ();
    fn capacity_bytes(&self) -> usize { self.accounted_bytes }
    fn available(&self) -> bool { Arc::strong_count(self) == 1 && Arc::weak_count(self) == 0 }
    fn reset(&mut self) {}
    fn reclassify(&mut self, category: BudgetCategory) {
        Arc::get_mut(self).expect("exclusive CPU credit").reservation.reclassify(category);
    }
}
pub(crate) struct ParameterCpuPool {
    pool: CpuPool<ParameterCpuCredit>,
    budget: Arc<AllocatorStats>,
    block_bytes: usize,
    cached_bytes: usize,
}
#[derive(Default)]
pub(crate) struct ParameterCpuCursor {
    current: Option<CpuPoolLease<ParameterCpuCredit>>,
    remaining: usize,
}
impl ParameterCpuPool {
    pub(crate) fn new(budget: Arc<AllocatorStats>) -> Result<Self> {
        Self::with_limits(budget, CpuPoolLimits { items: 128, total_bytes: 16 << 20,
            cached_bytes: 2 << 20, full: || Error::exhausted("parameter CPU credit pool is full") }, 64 << 10)
    }
    fn with_limits(budget: Arc<AllocatorStats>, limits: CpuPoolLimits, block_bytes: usize) -> Result<Self> {
        Ok(Self {pool: CpuPool::new(limits, budget.clone())?, budget, block_bytes,
            cached_bytes: limits.cached_bytes})
    }
    pub(crate) fn trim(&self) {
        self.pool.trim_idle(if matches!(self.budget.budget_action(), BudgetAction::EvictCache | BudgetAction::Backpressure)
            { 0 } else { self.cached_bytes });
    }
}
impl ParameterCpuCursor {
    pub(crate) fn claim(&mut self, pool: &ParameterCpuPool, bytes: usize) -> Result<ParameterCpuCredit> {
        if self.remaining < bytes {
            if self.current.as_ref().is_some_and(|lease| !lease.value().available()) {
                self.current.take();
                self.remaining = 0;
            }
            if self.current.is_none() {
                pool.trim();
                let capacity = bytes.max(pool.block_bytes);
                self.current = Some(pool.pool.checkout(|| {
                    let reservation = reserve_shared_cpu_metadata::<ParameterCpuBlock>(&pool.budget, capacity as u64)?;
                    let accounted_bytes = usize::try_from(shared_cpu_bytes::<ParameterCpuBlock>()?).map_err(|_| Error::OutOfHostMemory)?
                        .checked_add(capacity).ok_or(Error::OutOfHostMemory)?;
                    Ok(Arc::new(ParameterCpuBlock {credit_bytes: capacity, accounted_bytes, reservation}))
                })?);
            }
            let lease = self.current.as_mut().expect("admitted CPU credit");
            if lease.value().credit_bytes < bytes {
                let capacity = bytes.max(pool.block_bytes);
                let accounted = usize::try_from(shared_cpu_bytes::<ParameterCpuBlock>()?).map_err(|_| Error::OutOfHostMemory)?
                    .checked_add(capacity).ok_or(Error::OutOfHostMemory)?;
                lease.update(accounted - lease.value().accounted_bytes, |arc, _| {
                    let block = Arc::get_mut(arc).expect("unpublished CPU credit");
                    block.reservation.resize(accounted as u64)?;
                    block.credit_bytes = capacity;
                    block.accounted_bytes = accounted;
                    Ok(())
                })?;
            }
            self.remaining = lease.value().credit_bytes;
        }
        self.remaining -= bytes;
        Ok(self.current.as_ref().expect("active CPU credit").value().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pool(budget: Arc<AllocatorStats>) -> ParameterCpuPool {
        ParameterCpuPool::with_limits(budget, CpuPoolLimits { items: 2, total_bytes: 16384,
            cached_bytes: 8192, full: || Error::exhausted("test credit pool full") }, 1024).unwrap()
    }
    #[test]
    fn completed_payloads_reuse_callback_credit_and_published_payloads_pin_it() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = pool(budget.clone());
        let mut cursor = ParameterCpuCursor::default();
        drop(cursor.claim(&pool, 128).unwrap());
        let usage = budget.cpu_memory_usage().total();
        for _ in 0..1000 { drop(cursor.claim(&pool, 128).unwrap()); }
        assert_eq!(budget.cpu_memory_usage().total(), usage);
        assert_eq!(pool.pool.stats().creations, 1);
        let a = cursor.claim(&pool, 1024).unwrap();
        let b = cursor.claim(&pool, 1024).unwrap();
        assert!(!Arc::ptr_eq(&a, &b));
        assert!(matches!(cursor.claim(&pool, 1024), Err(Error::Exhausted(_))));
        assert_eq!(pool.pool.trim_idle(0), 0);
        drop(a);
        assert!(pool.pool.trim_idle(0) > 0);
        drop(cursor.claim(&pool, 1024).unwrap());
        drop(cursor);
        drop(pool);
        assert!(budget.cpu_memory_usage().total() > 0, "published block must outlive its pool");
        drop(b);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn failed_growth_keeps_existing_credit_and_budget_unchanged() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = pool(budget.clone());
        let mut cursor = ParameterCpuCursor::default();
        let old = cursor.claim(&pool, 1024).unwrap();
        let address = Arc::as_ptr(&old);
        drop(old);
        let usage = budget.cpu_memory_usage().total();
        budget.set_host_budget(usage).unwrap();
        assert!(matches!(cursor.claim(&pool, 2048), Err(Error::OutOfHostMemory)));
        assert_eq!(budget.cpu_memory_usage().total(), usage);
        assert_eq!(Arc::as_ptr(&cursor.claim(&pool, 1024).unwrap()), address);
        budget.set_host_budget(0).unwrap();
        drop(cursor);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn pinned_credit_survives_failed_new_block_admission_and_retries() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = pool(budget.clone());
        let mut cursor = ParameterCpuCursor::default();
        let published = cursor.claim(&pool, 1024).unwrap();
        let usage = budget.cpu_memory_usage().total();
        budget.set_host_budget(usage).unwrap();
        for bytes in [1024, 1] {
            assert!(matches!(cursor.claim(&pool, bytes), Err(Error::OutOfHostMemory)));
            assert_eq!(cursor.remaining, 0);
            assert!(cursor.current.is_none());
            assert_eq!(budget.cpu_memory_usage().total(), usage);
            assert_eq!(pool.pool.stats().pinned, 1);
        }
        let address = Arc::as_ptr(&published);
        drop(published);
        budget.set_host_budget(0).unwrap();
        assert_eq!(Arc::as_ptr(&cursor.claim(&pool, 1024).unwrap()), address);
        drop(cursor);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn cached_block_growth_respects_pool_capacity_without_corrupting_statistics() {
        let budget = Arc::new(AllocatorStats::default());
        let pool = ParameterCpuPool::with_limits(budget.clone(), CpuPoolLimits {
            items: 2, total_bytes: 4096, cached_bytes: 3072,
            full: || Error::exhausted("bounded credit test"),
        }, 1024).unwrap();
        let mut cursor = ParameterCpuCursor::default();
        drop(cursor.claim(&pool, 1024).unwrap());
        drop(cursor);
        let before = pool.pool.stats();
        assert_eq!(before.cached, 1);
        let usage = budget.cpu_memory_usage().total();
        let mut cursor = ParameterCpuCursor::default();
        assert!(matches!(cursor.claim(&pool, 4096), Err(Error::Exhausted(_))));
        let rejected = pool.pool.stats();
        assert_eq!(rejected.checked_out, 1);
        assert_eq!(rejected.cached, 0);
        assert_eq!(rejected.live_bytes, before.cached_bytes);
        assert_eq!(budget.cpu_memory_usage().total(), usage);
        assert_eq!(cursor.claim(&pool, 1024).unwrap().credit_bytes, 1024);
        drop(cursor);
        drop(pool);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }
}
