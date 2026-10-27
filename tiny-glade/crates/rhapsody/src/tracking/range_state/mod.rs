//! Native per-range state, prepared under the existing submission gate and
//! published only with the existing device timeline's successful SyncPoint.
//! This module records no independent completion counter and issues no Vulkan.
mod native;
mod owner;
mod frontier;
mod patch;
mod storage;
mod ops;
mod table;
pub use native::{NativeRange, NativeState};
pub use owner::{RangeOwner, RangeTransfer};
pub(crate) use owner::OwnershipDenial;
pub use frontier::Frontier;
pub use patch::{RangeCell, PatchAction, StatePatch, RangeStateConfig};
pub(crate) use storage::Storage;
pub use ops::{validate_transfer, validate_range, contains, intersection, difference};
pub(crate) use ops::released_range_error;
pub use table::RangeStateTable;

#[cfg(test)]
mod tests;
