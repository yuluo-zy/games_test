//! 语义化资源身份与 GPU 使用登记（P4，见 `ARCHITECTURE.md` §3.5）。
//!
//! 资源身份与使用状态可在 CPU 上独立测试：
//! - `handle`：类型化代数句柄 [`Handle`] 与稠密存储 [`ResourceStore`]；
//! - `tracked`：资源“最后一次 GPU 使用”的登记 [`SyncCell`]。
//!
//! 本层只是资源身份的**基础**：它不绑定具体资源类型（`Buffer`/`Texture` 等），也不
//! 负责线程同步或退役——这些由后续的 `resource::tracked` 与 `Device::retire` 组合完成。

pub mod name;
pub use name::ResourceName;

// Curated public resource types; implementations live in `crate::raw`.
pub use crate::raw::buffer::{Buffer, BufferDesc, MemoryLocation, plan_buffer_update};
pub use crate::raw::device_api::{BufferInitialData, TextureInitialBytes};
pub use crate::raw::dynamic_constants::{
    DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, DynamicConstants, DynamicConstantsChunk,
    DynamicConstantsConfig, DynamicConstantsStats,
};
pub use crate::raw::format::{
    FormatInfo, PlaneInfo, observed::try_vk_format_info, vk_format_block_bytes, vk_format_info,
    vk_format_valid_aspect_flags,
};
pub use crate::raw::managed::{
    buffer::ManagedBuffer,
    handle::Managed,
    resources::ManagedResources,
    texture::{ManagedTexture, ManagedTextureView},
};
pub use crate::raw::sampler::{BUILTIN_SAMPLER_NAMES, Sampler};
pub use crate::raw::setup::{
    SETUP_WAIT_TIMEOUT_NS, SetupAccess, SetupContext, SetupStats, SetupSubmission,
    TextureInitialData, init_texture_data, update_texture, update_texture_regions,
    with_setup_command, with_setup_resources,
};
pub use crate::raw::temp_list::TempList;
pub use crate::raw::texture::{Texture, TextureDesc, TextureKind, TextureView, TextureViewDesc};
pub use crate::raw::texture_upload::TextureUpdateRegion;

internal_mod!(handle, range_state, sharing, tracked);
pub use range_state::AccessScope;
pub use sharing::{QueueFamilies, ResourceSharing};

pub use handle::{Handle, ResourceStore};
pub use tracked::{GpuUse, OwnershipRange, SyncCell, UseVisitor};
