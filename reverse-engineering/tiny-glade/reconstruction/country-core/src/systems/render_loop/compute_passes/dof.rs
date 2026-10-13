//! `dof.rs` — Depth-of-field compute chain (W49 dof_prepare, W51 calc_dof).
//!
//! URIs (O): `/dof/{prepare, analyze, init_buffers, clear_halfres, halfres, fused,
//! fullres, fullres_upsample, fullres_all_lod0, fullres_all_lod1}.cs.hlsl`.
//! AsComputeShaderSource impls (O, pass-declaration §2.1):
//! `FullResDof`→`/dof/fullres.cs.hlsl:cs_main`, `FullResAllLod0Dof`,
//! `FullResAllLod1Dof`, `HalfResDof`→`:cs_halfres`, `HalfResAllDof`→`:cs_halfres_all`.
//! Branch: W50/W51 gated by `param_6[0xa9+(res_type==0x13?1:0)] != 0` (frame-wiring §3).

use crate::consume::pass_builder::{AsComputeShaderSource, ComputePass, ComputeShaderSource, ShaderDefValue};
use crate::render::pass_builder::shader_uri::{ComputeShaderSourceHash, ShaderUri};
use crate::typed_gap::Gap;

/// `/dof/fullres.cs.hlsl:cs_main` source (O).
pub struct FullResDof;
/// `/dof/fullres_all_lod0.cs.hlsl:cs_main` source (O).
pub struct FullResAllLod0Dof;
/// `/dof/fullres_all_lod1.cs.hlsl:cs_main` source (O).
pub struct FullResAllLod1Dof;
/// `/dof/halfres.cs.hlsl:cs_halfres` source (O).
pub struct HalfResDof;
/// `/dof/halfres.cs.hlsl:cs_halfres_all` source (O).
pub struct HalfResAllDof;

impl AsComputeShaderSource for FullResDof {
    fn source_hash(&self) -> u64 {
        ComputeShaderSourceHash::compute("/dof/fullres.cs.hlsl:cs_main")
    }
    fn as_compute_pipeline(&self) -> ComputeShaderSource<'_> {
        ComputeShaderSource {
            uri: ShaderUri::parse("/dof/fullres.cs.hlsl:cs_main"),
            defs: ShaderDefValue::default(),
        }
    }
}
// … the other four follow the identical pattern (O: pass-declaration §2.1;
// `/dof/fullres_all_lod0.cs.hlsl`, `/dof/fullres_all_lod1.cs.hlsl`,
// `/dof/halfres.cs.hlsl:cs_halfres`, `/dof/halfres.cs.hlsl:cs_halfres_all`).

/// Indirect args selector for the DoF multires chain (O name; fields U).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndirectArgsType {
    Slot(Gap),
}

/// W49 — `/dof/prepare.cs.hlsl` (+ `/dof/init_buffers.cs.hlsl`,
/// `/dof/clear_halfres.cs.hlsl`). Reads `DofAutofocusSsbo`; read_texture +
/// write_texture + dispatch (O, address 0x140a5a020).
pub fn dof_prepare(world: &mut Gap, _autofocus: &crate::consume::core::DofAutofocusSsbo) {
    let _ = world;
    unimplemented!()
}

/// W51 — `calc_dof` (0x140a5a570). Selects `calc_dof_multires` (line 162) or
/// `calc_dof_fullres` (line 79). Gated by `param_6[0xa9+(type==0x13)] != 0`.
pub fn calc_dof(_multires: bool, _half_res_input: &Gap, _config: &Gap) {
    unimplemented!()
}

/// Multires DoF chain — iterates mip levels, pushes DynamicConstants slices, and
/// dispatches `/dof/{halfres, fused, fullres_upsample, fullres_all_lod0,
/// fullres_all_lod1}` (O, line 162).
fn calc_dof_multires(_mips: impl Iterator<Item = u32>, _cfg: &Gap) {
    // O: Handle::desc + div_ceil + Iterator::count + MultiUnzip + Vec::push +
    //     DynamicConstants::push_slice + push_constants + dispatch_1d/2d.
    unimplemented!()
}

/// Full-res DoF — `/dof/fullres.cs.hlsl` direct dispatch (O, line 79).
fn calc_dof_fullres(_pass: ComputePass, _cfg: &Gap) {
    unimplemented!()
}
