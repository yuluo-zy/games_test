//! `taa.rs` — Temporal anti-aliasing / TGSR (W52, mutually exclusive).
//!
//! URIs (O): `/taa.cs.hlsl` ↔ `/tgsr.cs.hlsl:cs_main` (+ `/tgsr_reproject.cs.hlsl`,
//! `/tgsr_blur_lum.cs.hlsl`). `TgsrMain` implements `AsComputeShaderSource`
//! (O, pass-declaration §2.1).
//! Branch: `param_6[0xab]==0 → calculate_taa`, `!=0 → calculate_tgsr`
//! (frame-wiring §3). TGSR uses `min_subgroup_size` + `get_or_create_temporal_pair`
//! (O, sequence.md W52).

use crate::consume::pass_builder::{AsComputeShaderSource, ComputeShaderSource, ShaderDefValue};
use crate::render::pass_builder::shader_uri::{ComputeShaderSourceHash, ShaderUri};
use crate::typed_gap::Gap;

/// `/tgsr.cs.hlsl:cs_main` source (O).
pub struct TgsrMain;

impl AsComputeShaderSource for TgsrMain {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute("/tgsr.cs.hlsl:cs_main")
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse("/tgsr.cs.hlsl:cs_main"),
            defs: ShaderDefValue::default(),
        }
    }
}

/// W52 TAA — `/taa.cs.hlsl`; uniform + read_texture + write_texture + dispatch_2d
/// (O, address 0x140a3b2f0).
pub fn calculate_taa(world: &mut Gap, _frame: &crate::consume::core::FrameConstants) {
    let _ = world;
    unimplemented!()
}

/// W52 TGSR — `/tgsr.cs.hlsl:cs_main`; `Temporal::split_pair`/`into_inner`,
/// `Option::is_some_and`, dispatch_2d (O, address 0x140a3ba60).
pub fn calculate_tgsr(world: &mut Gap, _history: &Gap) {
    let _ = world;
    unimplemented!()
}
