//! Device and host memory accounting: allocation totals and records, host (CPU)
//! credit tickets, and the allocator statistics registry with its budget actions.
//!
//! This layer sits above `sync` (queue ids and completion values) and below everything that
//! owns CPU storage or GPU memory, so pools, trackers and allocators can all charge it.
use crate::sync::MAX_QUEUES;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize};
mod reservation;
mod state;
mod budget;
mod lifetime;
mod reports;
mod registry;
pub(crate) use reservation::{CpuMemoryReservation, shared_cpu_bytes, reserve_shared_cpu_metadata, cpu_vec_bytes, DescriptorReservation};
use state::{HostState, StatisticsState};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationTotals {
    pub total_allocated_bytes: u64,
    pub device_local_bytes: u64,
    pub system_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationRecord {
    pub id: u64,
    pub size: u64,
    pub memory_type: u32,
    pub heap: u32,
    pub properties: u32,
    pub name: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocatorReport {
    pub totals: AllocationTotals,
    pub allocations: Vec<AllocationRecord>,
}
/// Device memory accounting.
///
/// VkDeviceMemory ranges and unused native suballocator capacity are charged
/// to the device budget, optionally further limited per heap (explicit limits
/// or limits derived from `VK_EXT_memory_budget`). Renderer-owned host
/// metadata is charged to a separate host budget, so exhausting host metadata
/// reports `OutOfHostMemory` and never consumes device memory headroom.
///
/// GPU uses are staged in per-queue buffers without touching the global state
/// lock; completion moves only the in-flight entries whose timeline value has
/// been reached (per-queue queues ordered by value).
#[derive(Default)]
pub struct AllocatorStats {
    update_after_bind_descriptors: AtomicU64,
    /// Live VkDeviceMemory objects (dedicated allocations and native blocks).
    native_objects: AtomicU64,
    /// `maxMemoryAllocationCount`; 0 means unlimited.
    max_native_objects: AtomicU64,
    /// `(value, allocation id)` uses recorded since the last state drain.
    pending_uses: [Mutex<Vec<(u64, u64)>>; MAX_QUEUES],
    pending_count: AtomicUsize,
    /// Completion values already applied to the in-flight queues.
    observed: [AtomicU64; MAX_QUEUES],
    /// Combined device/heap/host budget action, published after each change.
    action: AtomicU8,
    host_action: AtomicU8,
    host: Mutex<HostState>,
    state: Mutex<StatisticsState>,
}

#[cfg(test)]
mod budget_tests;

#[cfg(test)]
mod native_spare_tests;
