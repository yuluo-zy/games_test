//! Semantic resource identity and GPU-use tracking (see the「资源身份与使用追踪」section of `ARCHITECTURE.md`).
//!
//! - `handle`: typed generational [`Handle`] and dense [`ResourceStore`];
//! - `tracked`: the record of a resource's last GPU use, [`SyncCell`];
//! - `range_state`: per-range ownership and access state behind a [`SyncCell`];
//! - `sharing`: exclusive or concurrent queue-family sharing.
//!
//! This layer binds no concrete resource type (`Buffer`, `Texture`, ...) and knows nothing
//! about devices: it sits below `device` so the device can own tracked objects.

internal_mod!(handle, range_state, sharing, tracked);
pub(crate) mod identity;
pub use handle::{Handle, ResourceStore};
pub use sharing::{QueueFamilies, ResourceSharing};
pub use tracked::{GpuUse, OwnershipRange, ResourceOwner, SyncCell, UseVisitor};
