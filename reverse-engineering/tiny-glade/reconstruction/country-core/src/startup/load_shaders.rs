//! `startup/load_shaders.rs` — game shader load systems.
//!
//! Source: B skeleton `crates/country-core/src/startup/load_shaders.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `load_shaders` @0x140aab0c0 (2 callers) — loads the game shader set via
//!   `load_shader_into_library` (GraphicsDeviceResource + AssetShaderLibrary +
//!   ShaderWatch).
//! - `load_ui_shaders` @0x140aaaf80 (2 callers) — the UI pass shaders
//!   (ui_2d / text / custom-cursor family, see `evidence/render-passes/INDEX.md`).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn load_shaders(_gpu: &Gap, _assets: &Gap, _shader_watch: &Gap) {
    todo!("recover load_shaders")
}

pub fn load_ui_shaders(_gpu: &Gap, _assets: &Gap, _shader_watch: &Gap) {
    todo!("recover load_ui_shaders")
}
