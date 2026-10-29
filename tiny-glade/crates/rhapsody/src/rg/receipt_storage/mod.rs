//! Bounded immutable CPU receipts. GPU completion is independent: an external
//! receipt clone keeps its own snapshot capacity and budget alive until Drop.
use super::graph;
use crate::{Error, Result, collections::cpu_pool::{CpuPool, CpuPoolLease, CpuPoolLimits, ArcPoolValue}};
use crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct ReceiptStorageConfig {
    pub max_receipts: usize,
    pub max_total_bytes: usize,
    pub max_cached_bytes: usize,
}
impl Default for ReceiptStorageConfig {
    fn default() -> Self {
        Self {
            max_receipts: 32,
            max_total_bytes: 4 << 20,
            max_cached_bytes: 1 << 20,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ReceiptStorageStats {
    pub slots: usize,
    pub borrowed: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub capacity_growths: u64,
    pub capacity_stalls: u64,
}
/// Read-only receipt contents. SessionFrameReceipt exposes these through Deref;
/// changing pooled vector capacities through a published receipt is forbidden.
#[derive(Debug)]
pub struct ReceiptData {
    pub receipts: Vec<crate::sync::SyncPoint>,
    pub snapshot: graph::Snapshot,
}
pub(crate) struct ReceiptSlot {
    pub data: ReceiptData,
    bytes: usize,
    // Drop data before crediting its capacity, including when receipts outlive
    // the Session and the pool's baseline Arc owner.
    reservation: CpuMemoryReservation,
}
impl std::fmt::Debug for ReceiptSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
pub(crate) struct ReceiptStorage {
    pool: CpuPool<Arc<ReceiptSlot>>,
    config: ReceiptStorageConfig,
    budget: Arc<AllocatorStats>,
}
pub(crate) struct ReceiptLease {
    lease: CpuPoolLease<Arc<ReceiptSlot>>,
}
impl ArcPoolValue for ReceiptSlot {
    fn capacity_bytes(this: &Self) -> usize { this.bytes }
    fn reset(this: &mut Arc<Self>) {
        let slot = Arc::get_mut(this).expect("exclusive receipt slot");
        slot.data.receipts.clear();
        slot.data.snapshot.physical_mapping.clear();
        slot.data.snapshot.pool_hits.clear();
    }
    fn reclassify(this: &mut Arc<Self>, category: BudgetCategory) {
        Arc::get_mut(this).expect("exclusive receipt slot").reservation.reclassify(category);
    }
}
fn bytes<T>(capacity: usize) -> usize {
    capacity.saturating_mul(std::mem::size_of::<T>())
}
fn reserve<T>(values: &mut Vec<T>, count: usize) -> Result<()> {
    if values.capacity() < count {
        values
            .try_reserve_exact(count.saturating_sub(values.len()))
            .map_err(|_| Error::OutOfHostMemory)?;
    }
    Ok(())
}
fn capacity(snapshot: &graph::Snapshot, receipts: usize) -> usize {
    std::mem::size_of::<ReceiptSlot>()
        .saturating_add(2 * std::mem::size_of::<usize>())
        .saturating_add(std::mem::align_of::<ReceiptSlot>())
        .saturating_add(bytes::<crate::sync::SyncPoint>(receipts))
        .saturating_add(bytes::<graph::PhysicalMapping>(
            snapshot.physical_mapping.capacity(),
        ))
        .saturating_add(bytes::<u32>(snapshot.pool_hits.capacity()))
}
fn desired(current: &graph::Snapshot, source: &graph::Snapshot, receipts: usize) -> usize {
    capacity(current, receipts)
        .saturating_add(bytes::<graph::PhysicalMapping>(
            source
                .physical_mapping
                .len()
                .saturating_sub(current.physical_mapping.capacity()),
        ))
        .saturating_add(bytes::<u32>(
            source
                .pool_hits
                .len()
                .saturating_sub(current.pool_hits.capacity()),
        ))
}
fn copy_snapshot(target: &mut graph::Snapshot, source: &graph::Snapshot) -> Result<()> {
    reserve(&mut target.physical_mapping, source.physical_mapping.len())?;
    reserve(&mut target.pool_hits, source.pool_hits.len())?;
    target.shared.clone_from(&source.shared);
    target.physical_mapping.clone_from(&source.physical_mapping);
    target.pool_hits.clone_from(&source.pool_hits);
    Ok(())
}
fn empty_snapshot(source: &graph::Snapshot) -> graph::Snapshot {
    graph::Snapshot {
        shared: source.shared.clone(),
        physical_mapping: Vec::new(),
        pool_hits: Vec::new(),
    }
}
impl ReceiptStorage {
    pub(crate) fn new(config: ReceiptStorageConfig, budget: Arc<AllocatorStats>) -> Result<Self> {
        let pool = CpuPool::new(CpuPoolLimits {
            items: config.max_receipts, total_bytes: config.max_total_bytes,
            cached_bytes: config.max_cached_bytes, full: || Error::Timeout,
        }, budget.clone())?;
        Ok(Self { pool, config, budget })
    }
    pub(crate) fn statistics(&self) -> ReceiptStorageStats {
        let stats = self.pool.stats();
        ReceiptStorageStats {
            slots: stats.checked_out + stats.cached + stats.pinned,
            borrowed: stats.checked_out + stats.pinned,
            live_bytes: stats.live_bytes, cached_bytes: stats.cached_bytes,
            metadata_bytes: stats.metadata_bytes, capacity_growths: stats.capacity_growths,
            capacity_stalls: stats.capacity_stalls,
        }
    }
    pub(crate) fn trim_idle(&self, target: usize) -> usize { self.pool.trim_idle(target) }
    /// An owned reservation follows the compiled frame through execute/cancel;
    /// a canceled preparation immediately returns its storage to the same pool.
    pub(crate) fn prepare(&self, source: &graph::Snapshot, receipts: usize) -> Result<ReceiptLease> {
        self.pool.trim_idle(self.config.max_cached_bytes);
        let mut lease = ReceiptLease { lease: self.pool.checkout(|| {
            let snapshot = empty_snapshot(source);
            let bytes = capacity(&snapshot, 0);
            let reservation = self.budget.reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)?;
            Ok(Arc::new(ReceiptSlot { data: ReceiptData { receipts: Vec::new(), snapshot }, bytes, reservation }))
        })? };
        let slot = lease.lease.value();
        let required = desired(&slot.data.snapshot, source, slot.data.receipts.capacity().max(receipts));
        if let Err(error) = lease.resize(required, |data| {
            reserve(&mut data.receipts, receipts)?;
            copy_snapshot(&mut data.snapshot, source)
        }) {
            lease.lease.discard_on_return();
            return Err(error);
        }
        Ok(lease)
    }
}
impl ReceiptLease {
    fn resize(&mut self, required: usize, change: impl FnOnce(&mut ReceiptData) -> Result<()>) -> Result<()> {
        let previous = self.lease.value().bytes;
        self.lease.update(required.saturating_sub(previous), |arc, available| {
            let slot = Arc::get_mut(arc).expect("unpublished receipt is exclusive");
            let limit = slot.bytes.saturating_add(available);
            slot.reservation.resize(required as u64)?;
            let result = change(&mut slot.data);
            let actual = capacity(&slot.data.snapshot, slot.data.receipts.capacity());
            let accounted = if actual > limit { Err(Error::Timeout) } else { slot.reservation.resize(actual as u64) };
            if let Err(error) = accounted {
                // Release enlarged vectors before crediting their reservation.
                drop(std::mem::take(&mut slot.data.receipts));
                drop(std::mem::take(&mut slot.data.snapshot.physical_mapping));
                drop(std::mem::take(&mut slot.data.snapshot.pool_hits));
                slot.bytes = capacity(&slot.data.snapshot, 0);
                slot.reservation.resize(slot.bytes as u64).expect("shrinking receipt reservation");
                return Err(error);
            }
            slot.bytes = actual;
            result
        })
    }
    /// The exact bound count is known after ownership preludes, before submit.
    pub(crate) fn reserve_points(&mut self, count: usize) -> Result<()> {
        let slot = self.lease.value();
        if count <= slot.data.receipts.capacity() { return Ok(()); }
        let required = capacity(&slot.data.snapshot, count);
        self.resize(required, |data| reserve(&mut data.receipts, count))
    }
    pub(crate) fn publish(mut self, receipts: &[crate::sync::SyncPoint]) -> Arc<ReceiptSlot> {
        let arc = self.lease.value_mut();
        let slot = Arc::get_mut(arc).expect("unpublished receipt is exclusive");
        assert!(receipts.len() <= slot.data.receipts.capacity(), "receipt capacity admitted before submit");
        slot.data.receipts.extend_from_slice(receipts);
        arc.clone()
    }
}

#[cfg(test)]
mod tests;
