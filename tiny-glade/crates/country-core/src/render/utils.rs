//! `render/utils.rs` — `calculate_hash` (referenced by wp1 `shader_uri` /
//! `draw_stream` as `crate::render::utils::calculate_hash`).
//!
//! 唯一实现位于共享 `utils` crate；这里 re-export 以保留 `crate::render::utils` 路径。
pub use ::utils::calculate_hash;
