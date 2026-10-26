//! Opaque host-memory admission for storage owned by a renderer consumer.
//! Capacity credit follows the physical owner; it is never Clone.
use crate::{Result, device::AllocatorStats, raw::allocator::CpuMemoryReservation};
use std::sync::Arc;

pub struct CpuStorageReservation(CpuMemoryReservation);
impl core::fmt::Debug for CpuStorageReservation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CpuStorageReservation")
            .field("bytes", &self.bytes())
            .finish()
    }
}
impl CpuStorageReservation {
    pub fn belongs_to(&self, budget: &Arc<AllocatorStats>) -> bool {
        self.0.belongs_to(budget)
    }
    /// Split already-admitted credit without acquiring more budget. The two
    /// non-Clone owners together return exactly the original admitted bytes.
    pub fn split_off(&mut self, bytes: u64) -> Result<Self> {
        self.0.split_off(bytes).map(Self)
    }
    pub fn bytes(&self) -> u64 {
        self.0.bytes()
    }
    /// Reserve growth before allocating. On refusal the original owner and
    /// credit remain unchanged. Shrink only after freeing physical backing.
    pub fn resize(&mut self, bytes: u64) -> Result<()> {
        self.0.resize(bytes)
    }
}
impl AllocatorStats {
    /// External CPU storage is counted as Live for its entire owned lifetime,
    /// including a consumer's cache. No internal reclassification is exposed.
    pub fn reserve_cpu_storage(self: &Arc<Self>, bytes: u64) -> Result<CpuStorageReservation> {
        self.reserve_cpu_bytes(bytes, crate::device::BudgetCategory::Live)
            .map(CpuStorageReservation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejected_growth_keeps_original_credit_and_drop_returns_it() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_host_budget(128).unwrap();
        let mut owner = stats.reserve_cpu_storage(128).unwrap();
        assert!(matches!(
            owner.resize(129),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(owner.bytes(), 128);
        assert_eq!(stats.cpu_memory_usage().total(), 128);
        owner.resize(64).unwrap();
        assert_eq!(stats.cpu_memory_usage().total(), 64);
        drop(owner);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
    #[test]
    fn split_moves_credit_without_readmission_or_early_release() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_host_budget(128).unwrap();
        let mut stock = stats.reserve_cpu_storage(128).unwrap();
        let escaped = stock.split_off(80).unwrap();
        assert_eq!(stock.bytes(), 48);
        assert_eq!(escaped.bytes(), 80);
        assert_eq!(stats.cpu_memory_usage().total(), 128);
        assert!(stock.split_off(49).is_err());
        assert_eq!(stock.bytes(), 48);
        drop(stock);
        assert_eq!(stats.cpu_memory_usage().total(), 80);
        drop(escaped);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
