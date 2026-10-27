//! Memory accounting: budgets, eviction registry, host-storage credit and the allocator
//! statistics registry.

internal_mod!(budget, stats);
mod cpu_storage;

pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, Evictable, EvictionPriority,
    EvictionRegistry, MemoryBudgetController,
};
pub use cpu_storage::CpuStorageReservation;
pub use stats::{AllocationRecord, AllocationTotals, AllocatorReport, AllocatorStats};
pub(crate) use stats::{
    CpuMemoryReservation, DescriptorReservation, cpu_vec_bytes, reserve_shared_cpu_metadata,
    shared_cpu_bytes,
};
