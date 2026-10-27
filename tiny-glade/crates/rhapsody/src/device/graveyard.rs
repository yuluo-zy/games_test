//! Objects awaiting destruction; the retirement queue itself is
//! [`crate::device::Device::retire`] (see `device::lifetime`).
use ash::vk;
pub enum DeferredMemory {
    Shared(std::sync::Arc<crate::device::transient_backing::SharedBacking>),
    /// A whole dedicated `VkDeviceMemory` object.
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// A range returned to gpu-allocator.
    #[cfg(feature = "suballocation")]
    Sub {
        allocation: crate::device::suballoc::SubAllocation,
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
    DescriptorPool(std::sync::Arc<crate::device::pools::DescriptorPoolAllocation>),
    DescriptorSet(
        vk::DescriptorSet,
        std::sync::Arc<crate::device::pools::DescriptorPoolAllocation>,
    ),
    CommandBuffer(
        vk::CommandBuffer,
        std::sync::Arc<crate::device::pools::CommandPoolAllocation>,
        bool, // already counted as waiting for native host recording
    ),
    /// Objects a submitted command retained. Holding them until the command's
    /// completion is what makes per-object completion tracking unnecessary for
    /// purely CPU-side owners (bindless leases).
    Owners(crate::collections::temp_list::TempList<crate::tracking::ResourceOwner>),
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
