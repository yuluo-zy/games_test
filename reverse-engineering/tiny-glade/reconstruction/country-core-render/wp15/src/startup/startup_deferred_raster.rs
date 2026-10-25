//! `startup/startup_deferred_raster.rs` — deferred-raster resources startup.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_deferred_raster.rs`.
//! O evidence: `startup_deferred_raster` @0x140a97a30 (1 caller) — system with
//! `GraphicsDeviceResource + AssetShaderLibrary + ShaderWatch +
//! ResMut<DeferredRasterResources>`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_deferred_raster(
    _gpu: &Gap,
    _assets: &Gap,
    _shader_watch: &Gap,
    _deferred: &mut Gap,
) {
    todo!("recover startup_deferred_raster")
}
