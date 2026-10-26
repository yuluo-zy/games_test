//! 设备与队列配置。

pub use crate::raw::allocator::AllocatorStats;
pub use crate::raw::graveyard::RetirementStats;
pub use crate::raw::stats::{RenderStats, StatsSnapshot};
pub use memory::GpuMemoryUsage;
mod cpu_storage;
pub use cpu_storage::CpuStorageReservation;
internal_mod!(
    budget,
    caps,
    generation,
    init,
    memory,
    policy,
    queue_config,
    reclamation
);
mod caches;
mod lifetime;
pub(crate) use lifetime::RetireTicket;
mod logical;
mod native;
pub(crate) mod pending_acquire;
mod queues;
mod submission;
pub use pending_acquire::{AcquireTarget, DrainedAcquires, PendingAcquire, PendingAcquires};
pub use submission::{Signal, SubmitBatch, Wait};

pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, Evictable, EvictionPriority,
    EvictionRegistry, MemoryBudgetController,
};
pub use caps::{DeviceCaps, Feature, FeatureSet};
pub use generation::{DeviceGeneration, Generationed};
pub use init::{DeviceConfig, DevicePreference};
pub use logical::Device;
pub use native::DeviceCapabilities;
pub use policy::{
    AllocationStrategy, DescriptorStrategy, ExecutionPolicy, PolicyPreferences, plan_policy,
};
pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
pub use queues::DeviceQueue;
pub use reclamation::{ReclamationBudget, ReclamationConfig};
