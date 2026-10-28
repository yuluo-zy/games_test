use super::*;
use crate::tracking::range_state::Storage;
use crate::accounting::{AllocatorStats, CpuMemoryReservation};
use std::sync::Arc;
mod frontier;
mod shadow;
mod templates;
mod full;
mod access;
use shadow::{ShadowCell, ResourceShadow};
use templates::{InitKey, Template};

#[derive(Clone, Copy, Debug)]
pub struct SynchronizationConfig {
    pub max_cells: usize,
    pub max_rows: usize,
    pub max_batches: usize,
    pub max_cpu_bytes: usize,
}
impl Default for SynchronizationConfig {
    fn default() -> Self {
        Self {
            max_cells: 65_536,
            max_rows: 262_144,
            max_batches: 4096,
            max_cpu_bytes: 64 << 20,
        }
    }
}
pub struct SynchronizationPlanner {
    bound: BoundSynchronization,
    shadow: Storage<ShadowCell>,
    by_resource: Storage<ResourceShadow>,
    /// `initial` row of each pushed patch (parallel to `bound.patches`).
    patch_slots: Storage<u32>,
    /// Reused every frame so computing the template key allocates nothing.
    key_scratch: Storage<InitKey>,
    index_spill_bytes: usize,
    borrowed_key_bytes: usize,
    _index_spill_memory: CpuMemoryReservation,
    templates: Vec<Template>,
    template_clock: u64,
    template_hits: u64,
    budget: Arc<AllocatorStats>,
    config: SynchronizationConfig,
    next_pair: u64,
    ready: bool,
    owner_bytes: usize,
    _owner_memory: Option<CpuMemoryReservation>,
}
impl SynchronizationPlanner {
    pub fn new(budget: Arc<AllocatorStats>, config: SynchronizationConfig) -> Result<Self> {
        if config.max_cells == 0
            || config.max_cells > 65_536
            || config.max_rows == 0
            || config.max_batches == 0
            || config.max_batches > 65_536
            || config.max_cpu_bytes == 0
        {
            return Err(Error::invalid_usage(
                "invalid synchronization storage bounds",
            ));
        }
        Ok(Self {
            bound: BoundSynchronization {
                initial: Storage::new(0, &budget)?,
                aliases: Storage::new(0, &budget)?,
                batches: Storage::new(0, &budget)?,
                order: Storage::new(0, &budget)?,
                transitions: Storage::new(0, &budget)?,
                patches: Storage::new(0, &budget)?,
                waits: Storage::new(0, &budget)?,
                index: Default::default(),
                shared: None,
            },
            shadow: Storage::new(0, &budget)?,
            by_resource: Storage::new(0, &budget)?,
            patch_slots: Storage::new(0, &budget)?,
            key_scratch: Storage::new(0, &budget)?,
            index_spill_bytes: 0,
            borrowed_key_bytes: 0,
            _index_spill_memory: budget.reserve_cpu_bytes(0, crate::accounting::BudgetCategory::Live)?,
            templates: Vec::new(),
            template_clock: 0,
            template_hits: 0,
            budget,
            config,
            next_pair: 1,
            ready: true,
            owner_bytes: 0,
            _owner_memory: None,
        })
    }
    /// Production shared owner. Its actual Arc envelope is charged separately
    /// from the reusable arrays and follows the last CPU recording owner.
    pub fn new_shared(
        budget: Arc<AllocatorStats>,
        config: SynchronizationConfig,
    ) -> Result<Arc<Self>> {
        let bytes = crate::accounting::shared_cpu_bytes::<Self>()?;
        if bytes > config.max_cpu_bytes as u64 {
            return Err(Error::Timeout);
        }
        let memory = budget.reserve_cpu_bytes(bytes, crate::accounting::BudgetCategory::Live)?;
        let mut planner = Self::new(budget, config)?;
        planner.owner_bytes = bytes as usize;
        planner._owner_memory = Some(memory);
        Ok(Arc::new(planner))
    }
    pub fn bound(&self) -> &BoundSynchronization {
        &self.bound
    }
    pub fn known_cpu_bytes(&self) -> usize {
        self.owner_bytes + self.bound.bytes() + self.shadow.bytes()
            + self.by_resource.bytes() + self.patch_slots.bytes() + self.key_scratch.bytes() + self.borrowed_key_bytes + self.index_spill_bytes
    }
    pub fn begin(&mut self) {
        self.ready = true;
        self.bound.initial.values.clear();
        self.bound.aliases.values.clear();
        self.bound.batches.values.clear();
        self.bound.order.values.clear();
        self.bound.transitions.values.clear();
        self.bound.patches.values.clear();
        self.bound.waits.values.clear();
        self.bound.invalidate_index();
        self.patch_slots.values.clear();
        self.shadow.values.clear();
        for entry in &mut self.by_resource.values {
            entry.cells.clear();
            entry.alias_flags = 0;
            entry.alias_previous.clear();
        }
    }
}
fn queue_id(domain: QueueDomain) -> Result<QueueId> {
    QueueId::new(domain.queue_index as usize)
        .ok_or_else(|| Error::invalid_usage("native queue index exceeds timeline capacity"))
}
impl BoundSynchronization {
    fn bytes(&self) -> usize {
        self.initial.bytes()
            + self.aliases.bytes()
            + self.batches.bytes()
            + self.order.bytes()
            + self.transitions.bytes()
            + self.patches.bytes()
            + self.waits.bytes()
    }
}

#[cfg(test)]
mod tests;
