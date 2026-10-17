//! 语义化资源身份与 GPU 使用登记（P4，见 `ARCHITECTURE.md` §3.5）。
//!
//! 资源身份与使用状态可在 CPU 上独立测试：
//! - [`handle`]：类型化代数句柄 [`Handle`] 与稠密存储 [`ResourceStore`]；
//! - [`tracked`]：资源“最后一次 GPU 使用”的登记 [`SyncCell`]。
//!
//! 本层只是资源身份的**基础**：它不绑定具体资源类型（`Buffer`/`Texture` 等），也不
//! 负责线程同步或退役——这些由后续的 `resource::tracked` 与 `Device::retire` 组合完成。

pub mod handle;
pub mod range_state;
pub mod sharing;
pub mod tracked;
pub use sharing::{QueueFamilies, ResourceSharing};

pub use handle::{Handle, ResourceStore};
pub use tracked::{GpuUse, OwnershipRange, SyncCell, UseVisitor};
