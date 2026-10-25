//! `gpu_generate_draw_lists.rs` — Nani GPU-driven draw-list generation (W22/W28)
//! plus the wall draw-list passes (W21/W27).
//!
//! Skeleton bundle: `compute_passes/gpu_generate_draw_lists.rs.{pdb,structures,
//! enriched,dependencies}` (source_sha256 3699a3cd…, module 521).
//! Decompile: `evidence/render-architecture/0x140987dd0.c` (already decoded, see
//! contracts/compute-passes.md §1).
//!
//! Imports owned elsewhere (O, dependencies.json): `country_core` render/nani
//! (GpuWorld, buffers, compute_pass_builder, pass_resource_access, shadowed_buffer),
//! `rhapsody` (RenderGraphRecorder/pass_api/util), `enum-iterator` 1.2,
//! `lock_api`, `alloc`/`core`.
//!
//! Frame order: W21 `generate_wall_draw_lists` → W22 `gpu_generate_draw_lists`
//! (round 0) → … → W27 `generate_wall_draw_lists_pass2` + W28 `gpu_generate_draw_lists`
//! (round non-0) when `\*0x14362c0d8 == 0`.

use crate::consume::nani::{GpuWorld, GpuWorldSubsetKind};
use crate::consume::pass_builder::{AsComputeShaderSource, ComputeDispatchApi, ComputePass};
use crate::typed_gap::Gap;

/// Indirect args type selector (O name, line 678; `buffer_offset()` semantics I).
// NB: the original derives `enum_iterator::Sequence` (enum-iterator 1.2) so the
// frame pass iterates all `GpuWorldSubsetKind`s; the crate is dependency-light
// here, so `all()` is provided manually.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndirectArgsType {
    // Variants not recovered (typed gap); each maps to an indirect-buffer slot.
    Slot(Gap),
}

impl IndirectArgsType {
    /// Byte offset of this args type inside the indirect args buffer (O method name).
    pub fn buffer_offset(&self) -> u64 {
        unimplemented!()
    }
}

/// Per-subset push constants for `cull_and_bucket` (O, line 260; fields U).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PushConstants(pub u32, pub u32);

mod cull_and_bucket {
    use super::*;

    /// cull_and_bucket dispatch_1d closure: iterate all `GpuWorldSubsetKind`s,
    /// filter by subset presence, collect `(Vec<PushConstants>, Vec<u32>)` via
    /// itertools::multiunzip, then dispatch each subset (O:
    /// `enum_iterator::All<GpuWorldSubsetKind>` filter_map + multiunzip).
    pub fn build_push_constants(_world: &Gap) -> Vec<(PushConstants, u32)> {
        let mut out = Vec::new();
        for kind in GpuWorldSubsetKind::all() {
            // O: GpuWorld::try_get_subset_mut + OwnedGpuWorldSubset::instance_count +
            //     ShadowedBuffer::buffer; filter empty subsets.
            if !kind.use_fine_grained_sorting() {
                out.push((PushConstants(0, 0), 0));
            }
        }
        out
    }
}

/// RenderSublistsIter — iterator over fine-grained-sorted render sublists
/// (O, line 662; `next` drives group_draw_cmd_prefix_sum dispatch_indirect_with).
pub struct RenderSublistsIter<'a> {
    _world: &'a Gap,
    _kind_iter: std::vec::IntoIter<GpuWorldSubsetKind>,
}

impl<'a> Iterator for RenderSublistsIter<'a> {
    type Item = (PushConstants, u32);
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

/// W22/W28 entry — round selects the HZB-less (0) or HZB cull_and_bucket variant
/// (0x140987dd0, third param).
pub fn gpu_generate_draw_lists(world: &mut Gap, round: u32) {
    let _ = world;
    let _ = round;
    // O (0x140987dd0.c, decoded):
    //  1. create GPU-only scratch buffers: `BufferDesc::new_gpu_only(0x6c000, 0)`
    //     + `RenderGraph::create_raw_resource` (clear) and `(instance_count<<2)`.
    //  2. clear_draw_lists   (dispatch_1d, /nani/clear_draw_lists.cs.hlsl)
    //  3. clear_histogram    (dispatch_1d, /nani/clear_histogram.cs.hlsl)
    //  4. render_loop::clear_buffer / clear_rg_buffer
    //  5. prepare_empty_unsorted_draw_lists
    //  6. cull_and_bucket   (dispatch_with closure_env$1)
    //  7. histogram_prefix_sum / generate_draw_list_dispatch_args (dispatch_1d)
    //  8. clear_draw_cmd_histogram / clear_draw_cmd_subtotals
    //  9. allocate_draw_lists / fill_unsorted_instance_indirection
    unimplemented!()
}

/// Clear the draw-list buffer (O, line 216; /nani/clear_draw_lists.cs.hlsl).
pub fn clear_draw_lists(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Clear the draw-list histogram (O, line 240; /nani/clear_histogram.cs.hlsl).
pub fn clear_histogram(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Prepare empty unsorted draw lists (O, line 1033; /nani/prepare_unsorted_draw_lists.cs.hlsl).
/// Reads `GpuWorld::mesh_atlas_info_buf` (O).
pub fn prepare_empty_unsorted_draw_lists(mut pass: ComputePass, gpu_world: &GpuWorld) {
    let _ = gpu_world.mesh_atlas_info_buf();
    pass.dispatch_1d(0);
}

/// Cull-and-bucket entry (O, line 260; /nani/cull_and_bucket.cs.hlsl:cs_main).
/// Iterates subsets, pushes per-subset PushConstants, dispatches 1D.
pub fn cull_and_bucket(mut pass: ComputePass, gpu_world: &GpuWorld) {
    let _ = gpu_world.draw_list_info_buf();
    for (pc, count) in cull_and_bucket::build_push_constants(&gpu_world.0) {
        let _ = (pc, count);
        pass.push_constants(0);
        pass.dispatch_1d(count);
    }
}

/// draw_indexed_indirect_to_dispatch (O, line 506; /nani/draw_indexed_indirect_to_dispatch.cs.hlsl).
pub fn draw_indexed_indirect_to_dispatch(mut pass: ComputePass, _indirect: &Gap) {
    // O: RenderGraphRecorder::create (indirect arg buffer) + push_constants + dispatch_1d.
    pass.push_constants(0);
    pass.dispatch_1d(0);
}

/// Histogram prefix sum (O, line 645; /nani/histogram_prefix_sum.cs.hlsl).
pub fn histogram_prefix_sum(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Generate draw-list dispatch args (O, line 687; /nani/generate_dispatch_args.cs.hlsl).
pub fn generate_draw_list_dispatch_args(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Clear the draw-command histogram (O, line 871).
pub fn clear_draw_cmd_histogram(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Clear the draw-command subtotals (O, line 891; /nani/clear_subtotals.cs.hlsl).
pub fn clear_draw_cmd_subtotals(mut pass: ComputePass) {
    pass.push_constants(0);
    pass.dispatch_1d(0);
}

/// Allocate draw lists (O, line 911; /nani/allocate_draw_commands.cs.hlsl).
pub fn allocate_draw_lists(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Fill the unsorted instance indirection (O, line 1165;
/// /nani/fill_unsorted_instance_indirection.cs.hlsl).
pub fn fill_unsorted_instance_indirection(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}

/// Create the cull-and-bucket pass for a shader source (O, line 584,
/// address 0x140a28950 = `create_cull_and_bucket_pass`).
/// Round 2 declares `hzb_tex` read_texture (I from the hzb variant pair).
pub fn create_cull_and_bucket_pass<T: AsComputeShaderSource>(
    _shader: T,
    _world: &mut Gap,
    _with_hzb: bool,
) -> ComputePass {
    ComputePass(Default::default())
}

/// External-instance cull for a per-kind instance SSBO (O: `cull_and_bucket_external<
/// SsboT, CullT>`). WoodenPlank→NaniCullAndBucketWoodenPlank, RoofTile→
/// NaniCullAndBucketRoofTile.
pub fn cull_and_bucket_external<SsboT, CullT>(_world: &mut Gap) -> ComputePass
where
    SsboT: Sized,
    CullT: AsComputeShaderSource,
{
    // O: utils::get_res + AssetSsboLibrary::get<SsboT> + create_cull_and_bucket_pass.
    ComputePass(Default::default())
}

/// W21 — wall draw-list generation (0x14098aaf0, /wall/generate_draw_lists.cs.hlsl:cs_pass1).
/// Reads wall instances via `UnsafeWorldCell::get_entity` closures; writes the wall
/// draw-indexed-indirect buffer (`ComputePass::write_draw_indirect_cmd_buffer`).
pub fn generate_wall_draw_lists(world: &mut Gap) {
    let _ = world;
    unimplemented!()
}

/// W27 — wall draw-list pass 2 (0x14098b2a0; /wall/generate_draw_lists.cs.hlsl:cs_pass2
/// + /wall/reset_pass2_inst_counts.cs.hlsl). Reads `hzb_tex` (read_texture@0.19, O).
/// Gated by `\*0x14362c0d8 == 0` at the frame level.
pub fn generate_wall_draw_lists_pass2(world: &mut Gap) {
    let _ = world;
    unimplemented!()
}

/// Reset wall draw indirect args (O, line 1287; `DrawBuilder::uniform` + dispatch_1d).
pub fn reset_wall_draw_indirect_args(mut pass: ComputePass) {
    pass.dispatch_1d(0);
}
