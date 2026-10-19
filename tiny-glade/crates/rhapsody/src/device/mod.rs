//! 设备与队列配置。
pub mod budget;
mod caches;
pub mod caps;
pub mod generation;
pub mod init;
mod lifetime;
mod logical;
pub mod memory;
mod native;
pub(crate) mod pending_acquire;
pub mod policy;
pub mod queue_config;
mod queues;
pub mod reclamation;
mod submission;
pub use pending_acquire::{AcquireTarget, DrainedAcquires, PendingAcquire, PendingAcquires};
pub use submission::{Signal, SubmitBatch, Wait};

pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, Evictable, EvictionPriority,
    EvictionRegistry, MemoryBudgetController,
};
pub use caps::{DeviceCaps, Feature, FeatureSet};
pub use generation::{DeviceGeneration, Generationed};
pub use init::{DeviceConfig, DevicePreference, SubmissionApi};
pub use logical::Device;
pub use native::DeviceCapabilities;
pub use policy::{
    AllocationStrategy, DescriptorStrategy, ExecutionPolicy, PolicyPreferences, RecordingPolicy,
    plan_policy,
};
pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
pub use queues::DeviceQueue;
pub use reclamation::{ReclamationBudget, ReclamationConfig};
