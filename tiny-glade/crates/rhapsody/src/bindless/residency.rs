//! Residency of leased resources and trimming of resident keys.
use crate::Result;
use crate::profile::RecordCount;
use crate::profile::record_count;
use std::collections::VecDeque;
use std::sync::Arc;
use super::BindlessHeap;
use super::lease::BindlessLease;
use super::parameters;
use super::slots::Slots;

#[derive(Default)]
pub(super) struct ResidentLeases {
    pub(super) leases: std::collections::HashMap<parameters::ResourceKey, std::sync::Weak<BindlessLease>>,
    inserts_since_trim: usize,
    memory: Option<crate::accounting::CpuMemoryReservation>,
}

impl ResidentLeases {
    fn trim(&mut self) {
        self.inserts_since_trim = 0;
        self.leases.retain(|_, lease| lease.strong_count() != 0);
    }
}

/// Expired weak keys are swept periodically; a fixed limit also bounds live
/// keys when unusually many groups are retained by a caller.
pub(super) const RESIDENT_TRIM_INTERVAL: usize = 512;

pub(super) const MAX_RESIDENT_KEYS: usize = 4096;

impl BindlessHeap {
    /// The resident lease for `key`, created by `make` on first use.
    pub(crate) fn resident_lease(
        &self,
        key: parameters::ResourceKey,
        make: impl FnOnce() -> Result<Arc<BindlessLease>>,
    ) -> Result<Arc<BindlessLease>> {
        let mut resident = self.resident.lock();
        let Some(cache) = resident.as_mut() else {
            drop(resident);
            return make();
        };
        if let Some(lease) = cache.leases.get(&key).and_then(std::sync::Weak::upgrade) {
            record_count(RecordCount::LocalCacheHits, 1);
            return Ok(lease);
        }
        if cache.inserts_since_trim >= RESIDENT_TRIM_INTERVAL
            || cache.leases.len() >= MAX_RESIDENT_KEYS
        {
            cache.trim();
        }
        drop(resident);
        let lease = make()?;
        if let Some(cache) = self.resident.lock().as_mut() {
            if let Some(existing) = cache.leases.get(&key).and_then(std::sync::Weak::upgrade) {
                return Ok(existing);
            }
            if cache.leases.len() >= MAX_RESIDENT_KEYS {
                return Ok(lease);
            }
            if cache.leases.len() == cache.leases.capacity() {
                // Account a conservative bucket/control-byte allocation bound
                // before growing the optional index. A rejected cache admission
                // still returns the valid uncached descriptor.
                let capacity = cache.leases.capacity().saturating_mul(2).max(3);
                let bytes = (capacity + 1).next_power_of_two()
                    * (std::mem::size_of::<(
                        parameters::ResourceKey,
                        std::sync::Weak<BindlessLease>,
                    )>() + 1)
                    + 16;
                let Ok(memory) = self
                    .device
                    .memory
                    .stats
                    .reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Cache)
                else {
                    return Ok(lease);
                };
                if cache.leases.try_reserve(1).is_err() {
                    return Ok(lease);
                }
                cache.memory = Some(memory);
            }
            cache.leases.insert(key, Arc::downgrade(&lease));
            cache.inserts_since_trim += 1;
        }
        Ok(lease)
    }

    pub(super) fn set_residency(&self, enabled: bool) {
        let old = std::mem::replace(
            &mut *self.resident.lock(),
            enabled.then(ResidentLeases::default),
        );
        drop(old);
    }

    /// Remove expired index entries; GPU owners govern the actual lease lifetime.
    pub(super) fn trim_resident(&self) {
        if let Some(cache) = self.resident.lock().as_mut() {
            cache.trim();
        }
    }

    /// Collect completed command owners. Their last lease drops release slots;
    /// descriptors still referenced by pending commands remain live.
    pub fn collect(&self) -> Result<usize> {
        let free = |slots: &Slots| slots.free.iter().map(VecDeque::len).sum::<usize>();
        let before = free(&self.slots.lock());
        self.device.try_collect_garbage()?;
        Ok(free(&self.slots.lock()).saturating_sub(before))
    }
}
