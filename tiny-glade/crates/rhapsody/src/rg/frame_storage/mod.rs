//! Owned per-frame metadata, separate from temporary recording arenas and GPU
//! memory. A frame returns its tables only when its last CPU owner is dropped.
use super::{graph, plan::CompiledPlan};
use crate::{Error, Result, collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, CpuPoolValue}};
use crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct FrameStorageConfig {
    pub max_frames: usize,
    pub max_total_bytes: usize,
    pub max_cached_bytes: usize,
}
impl Default for FrameStorageConfig {
    fn default() -> Self {
        Self {
            max_frames: 32,
            max_total_bytes: 4 << 20,
            max_cached_bytes: 1 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameStorageStats {
    pub checked_out: usize,
    pub cached_frames: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub capacity_growths: u64,
    pub capacity_stalls: u64,
}

#[derive(Default)]
pub(crate) struct FrameTables {
    pub registry: graph::Registry,
    pub physical_mapping: Vec<graph::PhysicalMapping>,
    pub pool_hits: Vec<u32>,
    pub history_validity: Vec<Option<bool>>,
}
fn bytes<T>(capacity: usize) -> usize {
    capacity.saturating_mul(std::mem::size_of::<T>())
}
fn desired<T>(values: &Vec<T>, len: usize) -> usize {
    bytes::<T>(values.capacity().max(len))
}
fn reserve<T>(values: &mut Vec<T>, len: usize) -> Result<()> {
    if values.capacity() < len {
        values
            .try_reserve_exact(len.saturating_sub(values.len()))
            .map_err(|_| Error::OutOfHostMemory)?;
    }
    Ok(())
}
impl FrameTables {
    fn capacity_bytes(&self) -> usize {
        self.desired_bytes(0, 0)
    }
    fn desired_bytes(&self, resources: usize, exports: usize) -> usize {
        desired(&self.registry.resources, resources)
            .saturating_add(desired(&self.registry.resource_initial_access, resources))
            .saturating_add(desired(&self.registry.backing_resources, resources))
            .saturating_add(desired(&self.registry.exported_logical_handles, exports))
            .saturating_add(desired(&self.physical_mapping, resources))
            .saturating_add(desired(&self.pool_hits, resources))
            .saturating_add(desired(&self.history_validity, resources))
    }
    pub(crate) fn reserve_plan(&mut self, plan: &CompiledPlan) -> Result<()> {
        let resources = plan.info.lifetimes.len();
        let exports = plan.definition.exports.len();
        reserve(&mut self.registry.resources, resources)?;
        reserve(&mut self.registry.resource_initial_access, resources)?;
        reserve(&mut self.registry.backing_resources, resources)?;
        reserve(&mut self.registry.exported_logical_handles, exports)?;
        reserve(&mut self.physical_mapping, resources)?;
        reserve(&mut self.pool_hits, resources)?;
        reserve(&mut self.history_validity, resources)?;
        Ok(())
    }
    fn clear(&mut self) {
        self.registry.resources.clear();
        self.registry.resource_initial_access.clear();
        self.registry.backing_resources.clear();
        self.registry.exported_logical_handles.clear();
        self.physical_mapping.clear();
        self.pool_hits.clear();
        self.history_validity.clear();
    }
}
struct CachedTables {
    // Free storage before crediting its exact capacity reservation.
    tables: FrameTables,
    reservation: CpuMemoryReservation,
    bytes: usize,
}
impl CpuPoolValue for CachedTables {
    type Statistics = ();
    fn capacity_bytes(&self) -> usize { self.bytes }
    fn reset(&mut self) { self.tables.clear(); }
    fn reclassify(&mut self, category: BudgetCategory) { self.reservation.reclassify(category); }
}
#[derive(Clone)]
pub struct FrameStoragePool {
    pool: CpuPool<CachedTables>,
    config: FrameStorageConfig,
    budget: Arc<AllocatorStats>,
}
impl FrameStoragePool {
    pub fn new(config: FrameStorageConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        let pool = CpuPool::new(CpuPoolLimits {
            items: config.max_frames, total_bytes: config.max_total_bytes,
            cached_bytes: config.max_cached_bytes, full: || Error::Timeout,
        }, budget.clone())?;
        Ok(Self { pool, config, budget })
    }
    pub fn config(&self) -> FrameStorageConfig { self.config }
    pub fn stats(&self) -> FrameStorageStats {
        let stats = self.pool.stats();
        FrameStorageStats {
            checked_out: stats.checked_out, cached_frames: stats.cached,
            live_bytes: stats.live_bytes, cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes, capacity_growths: stats.capacity_growths,
            capacity_stalls: stats.capacity_stalls,
        }
    }
    pub fn trim_idle(&self, target_bytes: usize) -> usize { self.pool.trim_idle(target_bytes) }
    pub(crate) fn checkout(&self, plan: &CompiledPlan) -> Result<FrameStorageLease> {
        let mut lease = FrameStorageLease { lease: self.pool.checkout(|| Ok(CachedTables {
            tables: FrameTables::default(),
            reservation: self.budget.reserve_cpu_bytes(0, BudgetCategory::Live)?, bytes: 0,
        }))? };
        if let Err(error) = lease.reserve_plan(plan) {
            lease.lease.discard_on_return();
            return Err(error);
        }
        Ok(lease)
    }
}
pub(crate) struct FrameStorageLease {
    lease: CpuPoolLease<CachedTables>,
}
impl FrameStorageLease {
    fn reserve_plan(&mut self, plan: &CompiledPlan) -> Result<()> {
        let cached = self.lease.value();
        let required = cached.tables.desired_bytes(plan.info.lifetimes.len(), plan.definition.exports.len());
        self.lease.update(required.saturating_sub(cached.bytes), |cached, available| {
            let limit = cached.bytes.saturating_add(available);
            cached.reservation.resize(required as u64)?;
            let result = cached.tables.reserve_plan(plan);
            let actual = cached.tables.capacity_bytes();
            if actual > limit {
                drop(std::mem::take(&mut cached.tables));
                cached.reservation.resize(0).expect("releasing CPU reservation");
                cached.bytes = 0;
                return Err(Error::Timeout);
            }
            if let Err(error) = cached.reservation.resize(actual as u64) {
                drop(std::mem::take(&mut cached.tables));
                cached.reservation.resize(0).expect("releasing CPU reservation");
                cached.bytes = 0;
                return Err(error);
            }
            cached.bytes = actual;
            result
        })
    }
    pub(crate) fn take_tables(&mut self) -> FrameTables {
        std::mem::take(&mut self.lease.value_mut().tables)
    }
    pub(crate) fn return_tables(&mut self, tables: FrameTables) {
        let actual = tables.capacity_bytes();
        let admitted = self.lease.value().bytes;
        if actual > admitted {
            // An external mutation cannot leave unadmitted capacity retained.
            drop(tables);
            self.lease.discard_on_return();
            self.lease.update(0, |cached, _| {
                cached.tables = FrameTables::default();
                cached.reservation.resize(0)?;
                cached.bytes = 0;
                Ok(())
            }).expect("releasing CPU reservation");
        } else {
            self.lease.update(0, |cached, _| {
                cached.reservation.resize(actual as u64)?;
                cached.bytes = actual;
                cached.tables = tables;
                Ok(())
            }).expect("shrinking CPU reservation");
        }
    }
}

#[cfg(test)]
mod tests;
