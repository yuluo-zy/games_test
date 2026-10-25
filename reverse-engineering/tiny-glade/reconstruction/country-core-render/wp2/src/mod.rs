//! WP2 GPU buffer primitives — module root mirroring `crates/country-core/src/render/`.
//!
//! Original file layout (B skeleton):
//!   ssbo.rs, shadowed_buffer.rs, sparse_instance_buffer.rs, vao.rs,
//!   raster_single.rs, dirty_index_range.rs, uniform.rs, indirect.rs
//!
//! This crate is a contract-aligned reconstruction (not recovered author source). Each module
//! tags fields/methods O (observed), I (inferred), U (unknown typed gap). Reused deps
//! (rhapsody, bumpalo, parking_lot, puffin, core atomics) are called, never reimplemented.

pub mod dirty_index_range;
pub mod indirect;
pub mod raster_single;
pub mod shadowed_buffer;
pub mod sparse_instance_buffer;
pub mod ssbo;
pub mod uniform;
pub mod vao;
