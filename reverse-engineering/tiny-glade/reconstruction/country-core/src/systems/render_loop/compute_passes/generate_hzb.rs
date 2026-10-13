//! `generate_hzb.rs` — W26 hierarchical depth buffer via FidelityFX SPD (0x1409286f0).
//!
//! URI `/generate_hzb.cs.hlsl` (SPD single-pass signature: `spdConstants` push
//! const + imgSrc/imgDst/imgDst6/spdGlobalAtomic, LocalSize 256 — evidence §7).
//! O deps: `bind_individual_mips` + `write_rg_storage_buffer` +
//! `get_or_create_temporal` + `all_mip_levels` (sequence.md W26).
//! Uses `ustr::ustr` for the image name, `next_power_of_two`, `div_ceil`,
//! `Handle::desc`, read_texture (main depth) + read_write_texture (mips) +
//! push_constants (spdConstants) + dispatch_groups (O, bundle).

use crate::typed_gap::Gap;

pub fn generate_hzb(_name: &Gap, _depth: &Gap, _mips: impl Iterator<Item = u32>) {
    // O:
    //  let name = ustr::ustr("generate_hzb");
    //  let mip_count = depth.desc().mip_levels; // next_power_of_two chain
    //  RenderGraphRecorder::create -> spdConstants push_constants -> dispatch_groups.
    unimplemented!()
}
