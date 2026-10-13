//! `rasterize_terrain_stroke.rs` — W3 terrain stroke rasterize + gauss blur.
//!
//! URI `/terrain_editing/rasterize_terrain_stroke.cs.hlsl` (PhysicalStorageBufferAddresses).
//! `Events::is_empty` guard; `DynamicConstantsChunk::with_data` +
//! `DynamicConstants::push_slice` + uniform + write_texture + dispatch_2d;
//! `bind_terrain_heightmap` (O). `GaussBlur` implements `AsComputeShaderSource`
//! → `/gauss_blur.cs.hlsl:cs_main` (O, pass-declaration §2.1). `RecalculateHeightMapCmd`
//! (U). `as_shader_def_value` + `hash` (O, shader-def value path).

use crate::consume::pass_builder::{AsComputeShaderSource, ComputeDispatchApi, ComputePass, ShaderUri};
use crate::typed_gap::Gap;

/// `/gauss_blur.cs.hlsl:cs_main` source (O).
pub struct GaussBlur;
impl AsComputeShaderSource for GaussBlur {
    fn source_hash(&self) -> u64 { unimplemented!() }
    fn as_compute_pipeline(&self) -> ShaderUri { ShaderUri(Default::default()) }
}

/// Terrain-stroke recalc command (O name; layout U).
pub struct RecalculateHeightMapCmd(pub Gap);

pub fn rasterize_terrain_stroke(world: &mut Gap, _stroke_events: &Gap, _terrain: &Gap) {
    let _ = world;
    unimplemented!()
}

/// Gauss-blur the terrain mask (dispatch_2d, track_gpu_lifetime).
fn gauss_blur(mut pass: ComputePass, _src: &Gap, _dst: &Gap) {
    pass.dispatch_2d(0, 0);
}
