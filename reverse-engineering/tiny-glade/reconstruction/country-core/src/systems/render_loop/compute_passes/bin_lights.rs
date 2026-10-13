//! `bin_lights.rs` — W40 light binning (0x1409e7770).
//!
//! URI `/light_binning/bin_lights_to_cells.cs.hlsl:cs_main` (`BinLights` is an
//! `AsComputeShaderSource`, O pass-declaration §2.1) + `/light_binning/{cell_prefix_sum,
//! cell_subtotal_prefix_sum, cell_prefix_merge}.cs.hlsl`. `AssetSsboLibrary::get` +
//! `InstanceBuffer::instance_count` + uniform + push_constants + dispatch_1d (O).

use crate::consume::pass_builder::{AsComputeShaderSource, ComputeShaderSource, ShaderDefValue};
use crate::render::pass_builder::shader_uri::{ComputeShaderSourceHash, ShaderUri};

/// `/light_binning/bin_lights_to_cells.cs.hlsl:cs_main` source (O).
pub struct BinLights;

impl AsComputeShaderSource for BinLights {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute("/light_binning/bin_lights_to_cells.cs.hlsl:cs_main")
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse("/light_binning/bin_lights_to_cells.cs.hlsl:cs_main"),
            defs: ShaderDefValue::default(),
        }
    }
}

pub fn bin_lights(world: &mut crate::typed_gap::Gap, _light_count: u32) {
    let _ = world;
    unimplemented!()
}
