//! Command recording and CPU scratch-memory infrastructure.
//!
//! CPU arena 依赖 CPU lease；真实帧槽及其 Vulkan 命令池由图后端管理，
//! 只有相关队列全部完成后才能复用。

pub use crate::raw::access_table::is_write;
pub use crate::raw::barrier::SynchronizationScope;
pub use crate::raw::binding::DescriptorSetBinding;
pub use crate::raw::command_buffer::{VulkanCommandBuffer, VulkanCommandPool};
pub use crate::raw::resource_access::{buffer_access, image_access};
pub use crate::raw::texture_upload::{record_texture_regions, record_texture_zero_fill};

pub mod cpu_arena;
pub mod recording;

pub use cpu_arena::{CpuArenaConfig, CpuArenaLease, CpuArenaPool, CpuArenaStats};
