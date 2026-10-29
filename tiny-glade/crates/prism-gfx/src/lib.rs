//! Backend-neutral core shared by every graphics backend.
//!
//! Extracted from `rhapsody` step by step (see crates/rhapsody/CORE_SPLIT_DESIGN.md and
//! METAL_EXECUTION_PLAN.md). `rhapsody` re-exports everything moved here at its original path, so code
//! that imports `rhapsody::sync::...` does not change.
pub mod accounting;
pub mod error;
pub mod native_types;
pub mod sync;

pub use error::{Error, ErrorKind, Result, SharedError};

// Internal diagnostics. `doc(hidden)` and `pub` only because the facade crate re-exports them; the facade
// keeps the visibility they had inside `rhapsody` (crate-private unless its `test-support` feature is on).
#[doc(hidden)]
pub mod log_ring;
#[doc(hidden)]
pub mod trace;
