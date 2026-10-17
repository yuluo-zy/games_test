//! 设备与队列配置。
pub mod budget;
pub mod caps;
pub mod generation;
pub mod init;
pub mod memory;
pub mod policy;
pub mod queue_config;
pub mod reclamation;
mod submission;
pub use submission::{Signal, SubmitBatch, Wait};

pub use crate::raw::device::Device;
pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, MemoryBudgetController,
};
pub use caps::{DeviceCaps, Feature, FeatureSet};
pub use generation::{DeviceGeneration, Generationed};
pub use init::{DeviceConfig, DevicePreference, SubmissionApi};
pub use policy::{
    AllocationStrategy, DescriptorStrategy, ExecutionPolicy, PolicyPreferences, RecordingPolicy,
    plan_policy,
};
pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
pub use reclamation::{ReclamationBudget, ReclamationConfig};
