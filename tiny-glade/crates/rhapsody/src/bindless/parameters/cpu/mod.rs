//! Callback-local, bounded CPU credit for parameter payloads. Published
//! parameters pin the same block; their destruction never takes the budget lock.
use crate::{Error, Result, collections::cpu_pool::{ArcPoolValue, CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue}};
use crate::accounting::{AllocatorStats, BudgetAction, BudgetCategory, CpuMemoryReservation, reserve_shared_cpu_metadata, shared_cpu_bytes};
use std::sync::Arc;

pub(crate) struct ParameterCpuBlock {
    credit_bytes: usize,
    accounted_bytes: usize,
    reservation: CpuMemoryReservation,
}
pub(crate) type ParameterCpuCredit = Arc<ParameterCpuBlock>;
impl ArcPoolValue for ParameterCpuBlock {
    fn capacity_bytes(this: &Self) -> usize { this.accounted_bytes }
    fn reset(_: &mut Arc<Self>) {}
    fn reclassify(this: &mut Arc<Self>, category: BudgetCategory) {
        Arc::get_mut(this).expect("exclusive CPU credit").reservation.reclassify(category);
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
mod tests;
