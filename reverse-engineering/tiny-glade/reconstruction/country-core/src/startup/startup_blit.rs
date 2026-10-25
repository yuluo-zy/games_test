//! `startup/startup_blit.rs` — boot blit (splash/loading blit) shader.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_blit.rs`.
//! O evidence: `startup_blit` @0x140901430 (2 callers) — system taking
//! `GraphicsDeviceResource + AssetShaderLibrary + ShaderWatch`, loads the boot
//! blit shader.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_blit(_gpu: &Gap, _assets: &Gap, _shader_watch: &Gap) {
    todo!("recover startup_blit (boot blit shader)")
}
