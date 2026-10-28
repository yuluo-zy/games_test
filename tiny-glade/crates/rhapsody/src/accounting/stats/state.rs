//! Mutable statistics state, allocation phases and heap bookkeeping.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::sync::CompletedValues;
use crate::sync::MAX_QUEUES;
use crate::sync::QueueId;
use crate::sync::SyncPoint;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use super::AllocationRecord;
use super::AllocationTotals;
use super::AllocatorReport;
use super::super::budget::BudgetCategory;

#[derive(Default)]
pub(super) struct HostState {
    pub(super) budget: crate::accounting::MemoryBudgetController,
    pub(super) usage: crate::accounting::BudgetUsage,
    pub(super) peak: u64,
}

/// Vulkan's `VK_MAX_MEMORY_HEAPS`.
pub(super) const MAX_HEAPS: usize = vk::MAX_MEMORY_HEAPS;

#[derive(Default)]
pub(super) struct StatisticsState {
    pub(super) budget: crate::accounting::MemoryBudgetController,
    pub(super) phases: BTreeMap<u64, AllocationPhase>,
    pub(super) next_id: u64,
    pub(super) records: BTreeMap<u64, AllocationRecord>,
    pub(super) totals: AllocationTotals,
    pub(super) peak: AllocationTotals,
    pub(super) peak_report: Option<AllocatorReport>,
    /// The last total-byte peak is represented by records and saved totals.
    /// Release retains its metadata; registration either materializes that
    /// evidence or supersedes it with a higher peak.
    pub(super) peak_dirty: bool,
    // Released records remain as owned peak evidence until a report or the
    // next registration consumes it. Physical GPU memory is already freed.
    pub(super) peak_released: bool,
    pub(super) peak_snapshot_totals: AllocationTotals,
    /// Explicit per-heap limits set by the application.
    pub(super) heap_budgets: [Option<u64>; MAX_HEAPS],
    /// Limits derived from `VK_EXT_memory_budget` (`None` = not reported).
    pub(super) heap_limits: [Option<u64>; MAX_HEAPS],
    /// Native bytes charged per heap (ranges plus unused suballocator capacity).
    pub(super) heap_used: [u64; MAX_HEAPS],
    heap_pressure: bool,
    /// Native allocator capacity not occupied by a live resource range. It is
    /// known GPU cache memory, distinct from host cache reservations.
    pub(super) suballocator_spare: [u64; 32],
    /// Per-queue `(timeline value, id)` of in-flight uses, ordered by value.
    pub(super) in_flight: [VecDeque<(u64, u64)>; MAX_QUEUES],
    pub(super) observed: CompletedValues,
    /// Swap buffer for draining staged uses without reallocating.
    pub(super) scratch: Vec<(u64, u64)>,
}

pub(super) struct AllocationPhase {
    pub(super) category: BudgetCategory,
    pub(super) last_use: crate::sync::SyncSet,
    /// The record owns one VkDeviceMemory object (dedicated allocation).
    pub(super) native: bool,
}

impl StatisticsState {
    pub(super) fn report(&self) -> AllocatorReport {
        AllocatorReport {
            totals: self.totals,
            allocations: self
                .records
                .iter()
                .filter(|(id, _)| self.phases.contains_key(id))
                .map(|(_, record)| record.clone())
                .collect(),
        }
    }
    pub(super) fn materialize_peak(&mut self) {
        if self.peak_dirty {
            self.peak_report = Some(AllocatorReport {
                totals: self.peak_snapshot_totals,
                allocations: self.records.values().cloned().collect(),
            });
            self.peak_dirty = false;
            self.prune_released_peak_records();
        }
    }
    pub(super) fn prune_released_peak_records(&mut self) {
        if self.peak_released {
            let phases = &self.phases;
            self.records.retain(|id, _| phases.contains_key(id));
            self.peak_released = false;
        }
    }
    pub(super) fn heap_limit(&self, heap: usize) -> Option<u64> {
        match (self.heap_budgets[heap], self.heap_limits[heap]) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }
    pub(super) fn refresh_heap_pressure(&mut self) {
        // Same high-water fraction as the device controller (80%).
        self.heap_pressure = (0..MAX_HEAPS).any(|heap| {
            self.heap_limit(heap).is_some_and(|limit| {
                u128::from(self.heap_used[heap]) * 5 >= u128::from(limit) * 4 && limit != 0
            })
        });
    }
    pub(super) fn apply_use(&mut self, queue: QueueId, value: u64, id: u64) {
        let observed = self.observed.get(queue);
        let Some(phase) = self.phases.get_mut(&id) else {
            return;
        };
        let previous = phase.last_use.get(queue);
        phase.last_use.insert(SyncPoint { queue, value });
        if phase.category == BudgetCategory::Retiring || value <= observed {
            return;
        }
        if value > previous {
            let fifo = &mut self.in_flight[queue.index()];
            // Submissions on one queue publish almost in value order.
            let at = fifo.partition_point(|&(queued, _)| queued <= value);
            fifo.insert(at, (value, id));
        }
        transition_phase(self, id, BudgetCategory::InFlight);
    }
    pub(super) fn action(&self) -> crate::accounting::BudgetAction {
        use crate::accounting::BudgetAction;
        let device = self.budget.tick();
        if device == BudgetAction::EvictCache {
            return device;
        }
        if self.heap_pressure {
            return if self.budget.usage().cache > 0 {
                BudgetAction::EvictCache
            } else {
                BudgetAction::Backpressure
            };
        }
        device
    }
}

pub(super) fn encode_action(action: crate::accounting::BudgetAction) -> u8 {
    match action {
        crate::accounting::BudgetAction::None => 0,
        crate::accounting::BudgetAction::EvictCache => 1,
        crate::accounting::BudgetAction::Backpressure => 2,
    }
}

pub(super) fn heap_index(heap: u32) -> Result<usize> {
    usize::try_from(heap)
        .ok()
        .filter(|index| *index < MAX_HEAPS)
        .ok_or_else(|| Error::invalid_usage("memory heap index out of range"))
}

pub(super) fn replace_spare(state: &mut StatisticsState, memory_type: u32, heap: u32, spare: u64) {
    let Some(slot) = state.suballocator_spare.get_mut(memory_type as usize) else {
        return;
    };
    let previous = std::mem::replace(slot, spare);
    state.budget.release(BudgetCategory::Cache, previous);
    // Freeing or trimming native memory never increases the admitted total,
    // so this re-admission cannot exceed a limit that admitted `previous`.
    // A limit lowered in between is tolerated: the charge is a fact.
    if state.budget.reserve(BudgetCategory::Cache, spare).is_err() {
        state.budget.record_cache(spare);
    }
    if let Some(used) = state.heap_used.get_mut(heap as usize) {
        *used = used.saturating_sub(previous).saturating_add(spare);
    }
}

pub(super) fn transition_phase(state: &mut StatisticsState, id: u64, category: BudgetCategory) {
    let Some(phase) = state.phases.get_mut(&id) else {
        return;
    };
    if phase.category == category {
        return;
    }
    // Retiring is terminal: accounting can never make an unowned allocation
    // eligible for a cache just because another completion snapshot arrived.
    if phase.category == BudgetCategory::Retiring {
        return;
    }
    let Some(record) = state.records.get(&id) else {
        return;
    };
    state
        .budget
        .reclassify(phase.category, category, record.size);
    phase.category = category;
}
