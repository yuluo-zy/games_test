//! `utils/load_json.rs` — JSON mesh/attribute loaders.
//!
//! Source: B skeleton `crates/country-core/src/utils/load_json.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `read_attribute_from_json` @0x14083e210 (7 callers) — reads a numeric
//!   attribute from `serde_json::Value`; array shapes `Vec<f32>`, `Vec<i32>`,
//!   `[f32;2]`, `[f32;3]`, `[f32;4]` (closures collect each element).
//! - `load_json_as_mesh<&str>` @0x140b0eeb0 (68 callers) — parses a `Mesh`
//!   from a JSON string; instantiations for `PathBuf`, `String`, `&PathBuf`
//!   (O). Called by every startup mesh load.

#![allow(dead_code, unused_imports)]

use crate::utils::Mesh;

/// `read_attribute_from_json` @0x14083e210 (O).
pub fn read_attribute_from_json(_value: &serde_json::Value, _name: &str) -> Vec<f32> {
    todo!("recover read_attribute_from_json")
}

/// `load_json_as_mesh<&str>` @0x140b0eeb0 (O; 68 callers).
pub fn load_json_as_mesh(_json: &str) -> anyhow::Result<Mesh> {
    todo!("recover load_json_as_mesh")
}
