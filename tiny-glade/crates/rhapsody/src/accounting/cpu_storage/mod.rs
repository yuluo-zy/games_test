//! Opaque host-memory admission for storage owned by a renderer consumer.
//! Capacity credit follows the physical owner; it is never Clone.
use crate::Result;
use crate::accounting::{AllocatorStats, CpuMemoryReservation};
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
        self.reserve_cpu_bytes(bytes, crate::accounting::BudgetCategory::Live)
            .map(CpuStorageReservation)
    }
}

#[cfg(test)]
mod tests;
