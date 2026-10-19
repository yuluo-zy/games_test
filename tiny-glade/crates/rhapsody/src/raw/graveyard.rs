//! Objects awaiting destruction; the retirement queue itself is
//! [`crate::device::Device::retire`] (see `device::lifetime`).
use ash::vk;
pub enum DeferredMemory {
    Shared(std::sync::Arc<super::transient_backing::SharedBacking>),
    /// A whole dedicated `VkDeviceMemory` object.
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// A range returned to gpu-allocator.
    #[cfg(feature = "suballocation")]
    Sub {
        allocation: super::suballoc::SubAllocation,
    },
}

/// A Vulkan object awaiting destruction.
pub enum Doomed {
    Buffer(vk::Buffer, DeferredMemory),
    Image(vk::Image, DeferredMemory),
    ImageView(vk::ImageView),
    Sampler(vk::Sampler),
    Semaphore(vk::Semaphore),
    QueryPool(vk::QueryPool),
    PipelineLayout(vk::PipelineLayout),
    Pipeline(vk::Pipeline),
    DescriptorSetLayout(vk::DescriptorSetLayout),
    DescriptorPool(std::sync::Arc<super::descriptor_set::DescriptorPoolAllocation>),
    DescriptorSet(
        vk::DescriptorSet,
        std::sync::Arc<super::descriptor_set::DescriptorPoolAllocation>,
    ),
    CommandBuffer(
        vk::CommandBuffer,
        std::sync::Arc<super::command_buffer::CommandPoolAllocation>,
    ),
}

/// Retirement queue state. Per-queue timeline values are reported as-is; sums
/// across queues have no meaning.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct RetirementStats {
    pub pending: usize,
    /// Last successfully submitted value per queue (0 = none).
    pub submitted: [u64; crate::sync::MAX_QUEUES],
    /// Last observed completed value per queue (cached, no device query).
    pub completed: [u64; crate::sync::MAX_QUEUES],
    pub idle_waits: u64,
}
/// Former name of [`RetirementStats`].
pub type GraveyardStats = RetirementStats;
