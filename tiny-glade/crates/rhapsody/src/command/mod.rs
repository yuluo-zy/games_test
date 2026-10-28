//! Command recording and CPU scratch-memory infrastructure.
//!
//! CPU arena 依赖 CPU lease；真实帧槽及其 Vulkan 命令池由图后端管理，
//! 只有相关队列全部完成后才能复用。

pub use barrier::SynchronizationScope;
pub use binding::DescriptorSetBinding;
pub use command_buffer::{PendingCommand, VulkanCommandBuffer, VulkanCommandPool};
pub use resource_access::{buffer_access, image_access};
pub use texture_copy::{TextureUpdateRegion, record_texture_regions, record_texture_zero_fill};

internal_mod!(
    barrier,
    binding,
    command_buffer,
    cpu_arena,
    crash,
    descriptor_set,
    recording,
    resource_access,
    texture_copy
);
mod device_ext;
pub(crate) mod pending_acquire;
pub use pending_acquire::{AcquireTarget, DrainedAcquires, PendingAcquire, PendingAcquires};
pub(crate) use pending_acquire::PendingAcquireReservation;

pub use cpu_arena::{CpuArenaConfig, CpuArenaLease, CpuArenaPool, CpuArenaStats};
