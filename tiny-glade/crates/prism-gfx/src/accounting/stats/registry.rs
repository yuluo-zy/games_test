//! Registration and release of allocations.
use crate::native_types::MemoryProperties;
use crate::Error;
use crate::Result;
use std::sync::atomic::Ordering;
use super::AllocationRecord;
use super::AllocatorStats;
use super::state::AllocationPhase;
use super::state::encode_action;
use super::state::heap_index;
use super::state::replace_spare;
use super::super::budget::BudgetCategory;

impl AllocatorStats {
    pub fn register(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: MemoryProperties,
        name: String,
    ) -> Result<u64> {
        self.register_with_spare(size, memory_type, heap, properties, name, None, false)
    }

    /// Register a dedicated VkDeviceMemory object whose native slot was taken
    /// by [`Self::acquire_native_object`]; releasing the record frees it.
    #[doc(hidden)]
    pub fn register_native(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: MemoryProperties,
    ) -> Result<u64> {
        self.register_with_spare(
            size,
            memory_type,
            heap,
            properties,
            String::new(),
            None,
            true,
        )
    }

    #[cfg(any(feature = "suballocation", test))]
    #[doc(hidden)]
    pub fn register_suballocation(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: MemoryProperties,
        spare: u64,
    ) -> Result<u64> {
        self.register_with_spare(
            size,
            memory_type,
            heap,
            properties,
            String::new(),
            Some(spare),
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn register_with_spare(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: MemoryProperties,
        name: String,
        spare: Option<u64>,
        native: bool,
    ) -> Result<u64> {
        if size == 0 {
            return Err(Error::invalid_usage(
                "cannot register a zero-sized allocation",
            ));
        }
        let heap_slot = heap_index(heap)?;
        let type_slot = usize::try_from(memory_type)
            .ok()
            .filter(|index| *index < 32)
            .ok_or_else(|| Error::invalid_usage("memory type index out of range"))?;
        let overflow = || Error::invalid_usage("memory statistics overflow");
        self.with_state(|state| {
            let total = state
                .totals
                .total_allocated_bytes
                .checked_add(size)
                .ok_or_else(overflow)?;
            let old_spare = spare.map_or(0, |_| state.suballocator_spare[type_slot]);
            let new_spare = spare.unwrap_or(0);
            let used = state.heap_used[heap_slot]
                .checked_sub(old_spare)
                .and_then(|used| used.checked_add(new_spare))
                .and_then(|used| used.checked_add(size))
                .ok_or_else(overflow)?;
            if state
                .heap_limit(heap_slot)
                .is_some_and(|limit| used > limit)
            {
                return Err(Error::OutOfDeviceMemory);
            }
            let id = state.next_id.checked_add(1).ok_or_else(overflow)?;
            let mut totals = state.totals;
            totals.total_allocated_bytes = total;
            if properties.contains(MemoryProperties::DEVICE_LOCAL) {
                totals.device_local_bytes = totals
                    .device_local_bytes
                    .checked_add(size)
                    .ok_or_else(overflow)?;
            } else {
                totals.system_bytes = totals.system_bytes.checked_add(size).ok_or_else(overflow)?;
            }
            let mut budget = state.budget;
            budget.release(BudgetCategory::Cache, old_spare);
            budget
                .reserve(BudgetCategory::Cache, new_spare)
                .map_err(|_| Error::OutOfDeviceMemory)?;
            budget
                .reserve(BudgetCategory::Live, size)
                .map_err(|_| Error::OutOfDeviceMemory)?;
            if state.peak_dirty && state.peak_released {
                if total > state.peak.total_allocated_bytes {
                    // The new peak supersedes the old one. Do not construct
                    // an obsolete snapshot while allocations are increasing.
                    state.prune_released_peak_records();
                } else {
                    state.materialize_peak();
                }
            }
            state.budget = budget;
            if spare.is_some() {
                state.suballocator_spare[type_slot] = new_spare;
            }
            state.next_id = id;
            state.phases.insert(
                id,
                AllocationPhase {
                    category: BudgetCategory::Live,
                    last_use: crate::sync::SyncSet::EMPTY,
                    native,
                },
            );
            state.records.insert(
                id,
                AllocationRecord {
                    id,
                    size,
                    memory_type,
                    heap,
                    properties: properties.as_raw(),
                    name,
                },
            );
            state.totals = totals;
            state.heap_used[heap_slot] = used;
            state.refresh_heap_pressure();
            state.peak.device_local_bytes =
                state.peak.device_local_bytes.max(totals.device_local_bytes);
            state.peak.system_bytes = state.peak.system_bytes.max(totals.system_bytes);
            if total > state.peak.total_allocated_bytes {
                state.peak.total_allocated_bytes = total;
                state.peak_dirty = true;
                state.peak_snapshot_totals = totals;
            }
            Ok(id)
        })
    }

    pub fn release(&self, id: u64) -> Result<()> {
        self.release_with_spare(id, None)
    }

    #[cfg(any(feature = "suballocation", test))]
    #[doc(hidden)]
    pub fn release_suballocation(&self, id: u64, spare: u64) -> Result<()> {
        self.release_with_spare(id, Some(spare))
    }

    fn release_with_spare(&self, id: u64, spare: Option<u64>) -> Result<()> {
        // Exclusive physical ownership has ended. Neither flushing staged
        // uses nor cloning the entire peak report belongs in this Drop path.
        let native = {
            let mut state = self.state.lock();
            let record = state
                .records
                .get(&id)
                .filter(|_| state.phases.contains_key(&id))
                .ok_or_else(|| Error::invalid_usage("unknown or already released memory id"))?;
            let (size, memory_type, heap, properties) = (
                record.size,
                record.memory_type,
                record.heap,
                record.properties,
            );
            let phase = state.phases.remove(&id).expect("live allocation phase");
            state.budget.release(phase.category, size);
            if state.peak_dirty {
                state.peak_released = true;
            } else {
                state.records.remove(&id);
            }
            if let Some(spare) = spare {
                replace_spare(&mut state, memory_type, heap, spare);
            }
            if let Some(used) = state.heap_used.get_mut(heap as usize) {
                *used = used.saturating_sub(size);
            }
            state.refresh_heap_pressure();
            state.totals.total_allocated_bytes -= size;
            if properties & MemoryProperties::DEVICE_LOCAL.as_raw() != 0 {
                state.totals.device_local_bytes -= size;
            } else {
                state.totals.system_bytes -= size;
            }
            self.action
                .store(encode_action(state.action()), Ordering::Release);
            phase.native
        };
        if native {
            self.release_native_object();
        }
        Ok(())
    }

    /// Only unused native blocks may be trimmed. No live range changes here.
    #[cfg(any(feature = "suballocation", test))]
    #[doc(hidden)]
    pub fn trim_suballocator_spare(&self, memory_type: u32, heap: u32) {
        self.with_state(|state| {
            replace_spare(state, memory_type, heap, 0);
            state.refresh_heap_pressure();
        });
    }

    #[doc(hidden)]
    pub fn rename(&self, id: u64, name: String) {
        let mut state = self.state.lock();
        if let Some(record) = state.records.get_mut(&id) {
            record.name = name.clone();
        }
        // Keep a materialized peak snapshot consistent with the new name.
        if let Some(report) = &mut state.peak_report
            && let Ok(index) = report
                .allocations
                .binary_search_by_key(&id, |record| record.id)
        {
            report.allocations[index].name = name;
        }
    }
}
