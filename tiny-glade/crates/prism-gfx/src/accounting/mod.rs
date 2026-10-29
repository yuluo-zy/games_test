//! Memory accounting: budgets, eviction registry, host-storage credit and the allocator
//! statistics registry.

#[doc(hidden)]
pub mod budget;
#[doc(hidden)]
pub mod stats;
mod cpu_storage;

pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, Evictable, EvictionPriority,
    EvictionRegistry, MemoryBudgetController,
};
pub use cpu_storage::CpuStorageReservation;
pub use stats::{AllocationRecord, AllocationTotals, AllocatorReport, AllocatorStats};
// Crate-private inside `rhapsody`; `pub` + hidden here because `rhapsody` (a separate crate now) uses them.
#[doc(hidden)]
pub use stats::{
    CpuMemoryReservation, DescriptorReservation, cpu_vec_bytes, reserve_shared_cpu_metadata,
    shared_cpu_bytes,
};
