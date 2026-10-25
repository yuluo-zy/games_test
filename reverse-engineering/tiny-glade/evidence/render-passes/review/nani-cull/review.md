# nani cull / draw-list + wall family cross-review — A vs B

Reviewer verdict for the two independent reconstructions of the nani GPU cull/draw-list
chain and the wall family. Ground truth: the shipped `.bin` modules
(`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`), extracted at SPIR-V
magic offset 23/24 and disassembled word-level (`tools/render-passes/dump_spv_ops.py`).
Every candidate HLSL was recompiled with the same dxc 1.10.2605.37 and checked via
`tools/render-passes/recompile_check.py`; the no-hzb variant bins were verified with a manual
`-D TINYGLADE_NANI_NO_HZB` compile plus `spirv_parse.py` fingerprint (recompile_check has no
`-D` passthrough). Per-file detail is in `review.json`; this is the recommendation.

## Bottom line

**Adopt candidate B as the base for the whole family, then apply the wall/generate_draw_lists
merge below.** B is the better base on every axis that matters: full file coverage (A is
missing 8 cull-family variants, the 2 wall rasters, and `nani/inc/user.hlsl`); it passes the
hard gate on **all 40** bins including the 10 `PhysicalStorageBufferAddresses` bins and both
hzB/no-hzB variant pairs (4 full structural PASS — clear_histogram, clear_subtotals,
draw_indexed_indirect_to_dispatch, fill_unsorted_instance_indirection — vs 0 for A); its
include closures match the bins' embedded OpString/OpSource lists (B's residuals are only
OpLine counts on most files, A's include OpSource/OpString closure drift); its struct/push-
constant member names match the embedded OpMemberName tables verbatim (NaniDrawListInfo =
single `global_instance_offset`, generate_draw_lists/group_draw_cmd push constants, nani
snake_case vs draw_indirect_types camelCase DrawIndexedIndirectCommand split); and on every
shared nani file B's opcode-histogram residual is smaller than A's. A's one structural win —
the wall/generate_draw_lists body — is portable into B (see below).

## Per-file winner summary

| file | winner | why |
|---|---|---|
| nani/clear_draw_lists.cs.hlsl | **B** | B residual = OpLine only (OpSource/OpString match bin closure); A has closure drift. |
| nani/clear_histogram.cs.hlsl | **B** | B full structural PASS. |
| nani/clear_subtotals.cs.hlsl | **B** | B full structural PASS. |
| nani/cull_and_bucket.cs.hlsl | **B** | A FAIL_DXC; B hard-gate PASS on 8a7d2f81 (default) and d8520a92 (with `-D TINYGLADE_NANI_NO_HZB`, manually verified — exact 10-binding set, no hzb_tex/`$Globals`, caps incl 5347). B's body = bin structure (4-plane OpDot frustum loop, HZB 2x2, visibility-OR + allocator/histogram atomics). |
| nani/cull_and_bucket.inc.hlsl | **merge (B base)** | B: shared cull buffers + `cull_frustum_side_planes` + `cull_append_visible` (bin structure). A's `NaniDrawListInfo` 4-member is **wrong** (bin = `{global_instance_offset}`). Fold A's meta-decode helpers in. |
| nani/cull_and_bucket_{bush,flowers,roof_tile,wooden_plank}.cs.hlsl | **B** | A missing. B hard-gate PASS, per-material instance structs match bin (BushXform/instance_buffer t23, dispatch_args_buffer t24 dead-binding guard). |
| nani/cull_and_bucket_{curved_halftimber,plaster,smoke,stairs_floor}.cs.hlsl | **B** | A missing. B hard-gate PASS (PBA caps via ext_capability 5347). |
| nani/draw_indexed_indirect_to_dispatch.cs.hlsl | **B** | B full structural PASS; both correctly use `inc/draw_indirect_types.hlsl` camelCase struct (bin struct 10). |
| nani/draw_list_offsets.inc.hlsl | **B** | both match cbuffer; B adds the `instance_list_offset_at` div/mod helper the bin inlines. |
| nani/draw_lists.inc.hlsl | **B** | B's `NaniDrawListInfo{global_instance_offset}` = bin verbatim; A's 4-member wrong. Both snake_case DrawIndexedIndirectCommand correct. |
| nani/atlas_part_info.inc.hlsl | **B** | both correct; B's `StructuredBuffer<MeshAtlasPartInfo[64]>` shape + ArrayStride 1024 evidence matches bin. |
| nani/fill_sorted_instance_list.cs.hlsl | **B** | B residual 8 vs A 12. |
| nani/fill_unsorted_instance_indirection.cs.hlsl | **B** | B full structural PASS. |
| nani/fill_unsorted_instance_list.cs.hlsl | **B** | B residual 8 vs A 12. |
| nani/generate_dispatch_args.cs.hlsl | **B** | B residual 16 vs A 18; no push constants (bin) — B matches. |
| nani/generate_draw_lists.cs.hlsl | **B** | B residual 3 vs A 18; push constants = bin struct 24 verbatim. |
| nani/group_draw_cmd_prefix_sum.cs.hlsl | **B** | B residual 10 vs A 12; push constants = bin struct 14 verbatim; 11-barrier scan. |
| nani/histogram_prefix_sum.cs.hlsl | **B** | B residual 6 vs A 8; B includes `inc/prefix_scan.hlsl` matching bin closure (A does not); unrolled 2-per-lane 11-barrier scan. |
| nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl | **B** | B residual 6 vs A 19. |
| nani/prepare_unsorted_draw_lists.cs.hlsl | **B** | B residual 3 vs A 19. |
| nani/subtotal_prefix_sum.cs.hlsl | **B** | B residual 12 vs A 14. |
| nani/allocate_draw_commands.cs.hlsl | **B** | B residual 2 vs A 6. |
| nani/inc/{user,trimesh,cull_and_bucket_owned_custom}.hlsl | **B** | A missing (user.hlsl) or minimal stubs; B documents the family closures (embedded in 52 nani bins). |
| wall/generate_draw_lists.cs.hlsl | **merge (A body, B shell + corrected gating)** | Bin cs_pass1: 4-plane frustum loop, then `if (prev_visibility & bit) { InterlockedOr(next); LOD-bucket append }`, then the screen-space LOD **ring-bucket loop runs unconditionally** (reached from both branches, no early-return). A has frustum+LOD+ring (82-key residual, closer structure) but gates everything behind `if (!was_visible) return;` — wrong (bin ring-appends non-prev-visible bricks too). B omits frustum + ring (89-key residual). Merge B's binding shell with A's body and the bin's exact gating. cs_pass2 bin = 4 OpImageFetch (2x2) + 13 matrix ops (8-corner AABB) — A's body is the closer approximation. |
| wall/reset_pass2_inst_counts.cs.hlsl | **B** | B residual 2 vs A 6. |
| wall/wall_brick_lod0.raster.hlsl | **B** | A missing; B hard-gate PASS (binding names match bin verbatim). |
| wall/wall_brick_lod1.raster.hlsl | **B** | A missing; B hard-gate PASS. |
| wall/inc/{brick_instance,brick_instance_flags,draw_lists,brick_frag,brick_vert,brick_vs_out,highlight_mode}.hlsl | **B** | A has only the first three (as stubs) and lacks the rest; B integrated + closure-correct. |

## PhysicalStorageBufferAddresses adjudication

The bin's actual address-read opcode sequence (`_nani_cull_and_bucket.cs.hlsl_cs_main_8a7d2f81e72c8eeb.bin`):

- Capability list is exactly `{Shader, RuntimeDescriptorArray, PhysicalStorageBufferAddresses}` — **no `Int64`**, and the module has no 64-bit integer type (only `OpTypeInt 32`).
- Addressing model `PhysicalStorageBuffer64` (5348).
- Push-constant member 0 is loaded as a **`uint2`**; the per-thread offset is added with a **32-bit `OpIAdd` on the low word only** (no carry into the high word).
- The uint2 is converted to a pointer with **`OpBitcast`** `<ptr PhysicalStorageBuffer>` ← `uint2`, then the instance record is read with **`OpLoad uint4 <ptr>, alignment 2, volatile`** directly from the physical pointer (no `OpAccessChain`).

**Verdict: B is correct on both counts.** B's claim — "the original kept the address as uint2 and had no Int64" — is confirmed verbatim, and B's strategy (`uint2` address + `[[vk::ext_capability(5347)]]`, no Int64) reproduces the bin's capability set and the uint2 representation while compiling. A's strategy (`uint64_t` + `[[vk::buffer_reference]]`) would compile to `Int64` + 64-bit `OpIAdd`, which **does not match the bin**, and it does not compile with the installed dxc anyway. On "closer to the bin's opcode structure", B wins; on "better reconstruction strategy", B also wins (compiling, hard-gate-passing artifact). Residual for both: neither reproduces the genuine `OpBitcast`→PhysicalStorageBuffer→volatile `OpLoad` physical read (A documents it as a reference; B substitutes a logical per-index read). That physical load is not expressible with the installed dxc, so B's approximation plus a documented residual note is the practical ceiling; A's file is the reference for the residual.

## hzB-variant handling

Adopt B's compile-time switch: `#if !defined(TINYGLADE_NANI_NO_HZB)` with **default = the hzb superset (8a7d2f81)**, reduced variant = d8520a92. Verified: B's default compile matches 8a7d (t13..t22 incl hzb_tex t21 + `$Globals` b22); B with `-D TINYGLADE_NANI_NO_HZB` matches d852 exactly (t13..t20, no hzb_tex/`$Globals`, caps Shader+RuntimeDescriptorArray+PhysicalStorageBufferAddresses). This maps the W22 (first round, no hzb) vs W28 (second round, post-`generate_hzb`) dispatch pair. The bin's hzb test is a 2x2 mip fetch (4× OpImageFetch + min) — B's approximated 2x2 sample is the right shape. Keep the define documented; recompile_check has no `-D` passthrough so variant checks must be done manually (as B did).

## Inc-layer fixes

1. **`NaniDrawListInfo` must be `{global_instance_offset}`** (single uint) — bin struct 40/41/42. A's 4-member version is wrong; adopt B's. The cull body reads `draw_list_info[draw_list_index_base].global_instance_offset`.
2. **Keep two distinct `DrawIndexedIndirectCommand` structs**: the nani snake_case `{index_count,instance_count,first_index,vertex_offset,first_instance}` in `nani/draw_lists.inc.hlsl` (bins generate/fill/prepare) **and** the camelCase `{count,instanceCount,firstIndex,vertexOffset,firstInstance}` in `inc/draw_indirect_types.hlsl` for `draw_indexed_indirect_to_dispatch`/wall/grass. Do not merge them — the bins use both.
3. **`inc/prefix_scan.hlsl`**: the histogram/subtotal/group/prefix_sum bins DO embed the include, but the scans are inline 2-per-lane Hillis-Steele with **barrier-after-add** (11 `OpControlBarrier` over 1024 elements). Keep the include for closure; scan inline; document that the `PREFIX_SCAN_UINT` macro's barrier-before-add order differs.
4. **wall compute consumers** use `StructuredBuffer<FrameConstants>` at set1/b1 and must **not** include `inc/frame_constants.hlsl` (would add the set1/b0 UBO). Both candidates redeclare FrameConstants locally — keep that convention.
5. **wall include closure** = `inc/draw_indirect.hlsl` + `inc/draw_indirect_types.hlsl` + `wall/inc/{brick_instance,brick_instance_flags,draw_lists}.hlsl` (embedded_source_paths) — do not pull shared `draw_indirect.hlsl` static-const globals into an explicit `$Globals` shader.
6. **mesh meta decode** (f16tof32(meta & 0xFFFF), mesh = meta >> 16, second half from mesh_index low bits) is in the bin's instance word — carry A's decode helpers into B's `cull_and_bucket.inc` as documented helpers for when the physical read is restored.
7. **nani-private incs** `nani/inc/{user,trimesh,cull_and_bucket_owned_custom}.hlsl` are embedded in the family closures and belong under `shaders/nani/inc/` (not the shared `inc/` tree) — B's placement is correct.
8. **wall generate_draw_lists cs_pass2** needs a final pass against the bin's 8-corner AABB HZB (4 OpImageFetch + 13 matrix ops) once the A-body merge lands.

## Recommend to the main agent

Adopt **B** wholesale for the nani cull/draw-list chain and the wall compute/rasters, then:
1. Merge A's `wall/generate_draw_lists.cs.hlsl` body into B's shell, **correcting the gating to match the bin** (LOD-bucket append + `InterlockedOr` only when prev-visible AND in-frustum; screen-space ring-bucket append unconditional; cs_pass2 2x2/8-corner HZB per A's closer body).
2. Carry A's meta-decode + physical-read documentation into `cull_and_bucket.inc` as the PBA residual reference.
3. Apply the inc fixes above (NaniDrawListInfo single-member is already B; keep the two DrawIndexedIndirectCommand structs; keep prefix_scan include + inline scans; keep the wall local FrameConstants + closure).
A should not be adopted wholesale: its only structural wins (wall body, meta-decode documentation) are individually portable into B, and its `NaniDrawListInfo`, `cull_and_bucket.cs` (non-compiling + Int64-mismatched PBA), and missing file coverage make it the weaker base.
