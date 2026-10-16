//! 设备与队列配置。
pub mod budget;
pub mod caps;
pub mod generation;
pub mod policy;
pub mod queue_config;
mod submission;

// 迁移中的 raw API 仍返回 vk::Result；只在此边界保留旧返回类型。
pub(crate) fn legacy_vk_error(error: crate::Error) -> ash::vk::Result {
    use ash::vk;
    match error {
        crate::Error::DeviceLost => vk::Result::ERROR_DEVICE_LOST,
        crate::Error::OutOfDeviceMemory => vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
        crate::Error::OutOfHostMemory => vk::Result::ERROR_OUT_OF_HOST_MEMORY,
        crate::Error::SurfaceOutOfDate => vk::Result::ERROR_OUT_OF_DATE_KHR,
        crate::Error::SurfaceLost => vk::Result::ERROR_SURFACE_LOST_KHR,
        crate::Error::Timeout => vk::Result::TIMEOUT,
        crate::Error::Unsupported(_) => vk::Result::ERROR_FEATURE_NOT_PRESENT,
        crate::Error::InvalidUsage { .. } => vk::Result::ERROR_INITIALIZATION_FAILED,
        crate::Error::Vulkan(error) => error,
    }
}

pub use budget::{
    BudgetAction, BudgetCategory, BudgetExceeded, BudgetUsage, MemoryBudgetController,
};
pub use caps::{DeviceCaps, Feature, FeatureSet};
pub use generation::{DeviceGeneration, Generationed};
pub use policy::{
    AllocationStrategy, DescriptorPreference, DescriptorStrategy, ExecutionPolicy,
    PolicyPreferences, RecordingPolicy, plan_policy,
};
pub use queue_config::{
    FamilyInfo, PhysicalQueue, QueueClass, QueueConfig, QueuePlan, QueuePreference, plan_queues,
};
