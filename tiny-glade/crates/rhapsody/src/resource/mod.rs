//! 语义化资源身份与 GPU 使用登记（见 `ARCHITECTURE.md` 的「资源身份与使用追踪」一节）。
//!
//! 资源身份与使用状态可在 CPU 上独立测试：
//! - `handle`：类型化代数句柄 `Handle` 与稠密存储 `ResourceStore`；
//! - `tracked`：资源“最后一次 GPU 使用”的登记 `SyncCell`。
//!
//! 本层只是资源身份的**基础**：它不绑定具体资源类型（`Buffer`/`Texture` 等），也不
//! 负责线程同步或退役——这些由后续的 `resource::tracked` 与 `Device::retire` 组合完成。

mod cpu_bytes;
pub use cpu_bytes::CpuBytes;
pub mod name;
pub use name::ResourceName;

internal_mod!(buffer, bytes, device_api, dynamic_constants, managed, sampler, texture);
pub use buffer::{Buffer, BufferDesc, MemoryLocation, plan_buffer_update};
pub use device_api::{BufferInitialData, TextureInitialBytes};
pub use dynamic_constants::{
    DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, DynamicConstants, DynamicConstantsChunk,
    DynamicConstantsConfig, DynamicConstantsStats,
};
pub use managed::{
    buffer::ManagedBuffer,
    handle::Managed,
    resources::ManagedResources,
    texture::{ManagedTexture, ManagedTextureView},
};
pub use sampler::{BUILTIN_SAMPLER_NAMES, Sampler};
pub use texture::{Texture, TextureDesc, TextureKind, TextureView, TextureViewDesc};


