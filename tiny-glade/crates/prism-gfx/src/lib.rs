//! Backend-neutral core shared by every graphics backend.
//!
//! Extracted from `rhapsody` step by step (see crates/rhapsody/CORE_SPLIT_DESIGN.md and
//! METAL_EXECUTION_PLAN.md). `rhapsody` re-exports everything moved here at its original path, so code
//! that imports `rhapsody::sync::...` does not change.
pub mod native_types;
