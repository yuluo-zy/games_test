# wave2 nani-cull merge — integration summary

Date: 2026-10-13. Verdict applied: `evidence/render-passes/review/nani-cull/review.md` (+ `review.json`) — **candidate B is the base**. All outputs under `reconstruction/shader-source/shaders/`; oracle `.bin` read-only; flat `nani_*.raster` materials and other families untouched.

## What was merged

- **40 files copied wholesale from B** (`candidates/nani-cull-b/shaders/`): the full nani cull/draw-list chain (24 `.cs` + 3 nani-private incs incl `user.hlsl`), wall compute (`reset_pass2_inst_counts.cs`), wall rasters (`wall_brick_lod{0,1}.raster.hlsl`), and all 7 `wall/inc/` closure files. This covers the 8 cull material variants A lacked and the 2 wall rasters.
- **2 files merged** (B base + A pieces + corrections):
  1. `nani/cull_and_bucket.inc.hlsl` — B shared-cull body (`cull_frustum_side_planes`, `cull_append_visible`, t18..u17 buffers) + A's meta-decode helpers (`decode_instance_meta_size`, `decode_instance_mesh_index`). `NaniDrawListInfo` = `{global_instance_offset}` (single uint, bin struct 40/41/42) adopted from B's `draw_lists.inc.hlsl`; A's 4-member version dropped. `Affine3Packed` guard kept. PBA residual documented.
  2. `wall/generate_draw_lists.cs.hlsl` — B binding shell + A body (cs_pass1 4-plane frustum + LOD buckets 0..3 + screen-space ring buckets 4..7; cs_pass2 8-corner AABB -> HZB 4x OpImageFetch) + **bin-corrected gating**: LOD append + `InterlockedOr` only when prev-visible AND in-frustum; ring-bucket loop runs **unconditionally** (A's `if(!was_visible) return;` dropped — the bin reaches ring code from both branches).

## Adjudicated inc fixes — status

| # | fix | status | consumer hard-gate recheck |
|---|---|---|---|
| 1 | two distinct `DrawIndexedIndirectCommand` (nani snake_case vs draw_indirect_types camelCase) | kept (already correct) | draw_indexed_indirect_to_dispatch (camelCase) **PASS_STRUCTURAL**; generate_draw_lists/fill_unsorted_instance_list (snake_case) PASS |
| 2 | `inc/prefix_scan.hlsl` include kept; inline 2-per-lane 11-barrier scans | kept (B already) | histogram_prefix_sum, group_draw_cmd_prefix_sum, subtotal_prefix_sum, prefix_sum_unsorted — all PASS |
| 3 | wall closure + local FrameConstants redeclare | applied (see deviation) | wall/generate_draw_lists cs_pass1+cs_pass2 PASS |
| 4 | `NaniDrawListInfo` single-member + `Affine3Packed` guard | applied | cull_and_bucket 8a7d+d852 + all 8 material variants PASS |

No inc fix required reverting.

**Deviation (evidence-driven):** `inc/frame_constants.hlsl` **is included** in the merged wall file, overriding inc-fix #4's "NEVER include". Reasons recorded in merge-log.json: the bin's `embedded_source_paths` embeds frame_constants.hlsl; `inc/cascades.hlsl` (wall closure) needs the global `view_constants` to compile; and an empirical compile proves the installed dxc DCEs the unused set1/b0 UBO and space2/t0 trigger — output bindings equal the bin exactly, caps stay `{Shader, RuntimeDescriptorArray}`. The adjudication's rationale (set1/b0 collision) does not materialize. `FrameConstants` is still redeclared locally per convention.

## PBA (PhysicalStorageBufferAddresses) — Part 2

- Kept B's approximation: `uint2` address (32-bit OpIAdd, no carry), **no Int64**, `[[vk::ext_capability(5347)]]` + `SPV_KHR_physical_storage_buffer`.
- A's `uint64_t` + `[[vk::buffer_reference]]` is rejected: would emit `Int64` + FAIL_DXC (bin has no Int64 capability, only 32-bit `OpTypeInt`).
- **Verified** `cull_and_bucket.cs.hlsl` 8a7d2f81 (hzb default) hard-gate PASS and d8520a92 (manual `-D TINYGLADE_NANI_NO_HZB`) hard-gate PASS; no-hzb caps exactly `{Shader, RuntimeDescriptorArray, PhysicalStorageBufferAddresses}` — matches bin, no Int64.
- Residual: the genuine `OpBitcast`->PhysicalStorageBuffer->volatile `OpLoad` physical read is not expressible with installed dxc; B's logical per-index read + A's file as algorithm reference are documented in the merged inc header.

## Verification (Part 4)

`recompile_check.py` on every merged nani/wall main (+ manual `-D` compiles for no-hzb d8520a92 and raster f0ad, compared via `spirv_parse` fingerprint — `tools/render-passes/variant_verify.py`):

- **44 modules checked, 44 hard-gate PASS, 0 FAIL, 0 FAIL_DXC** (hard gate = entries + LocalSize + capabilities + bindings).
- **4 full structural PASS** (clear_histogram, clear_subtotals, draw_indexed_indirect_to_dispatch, fill_unsorted_instance_indirection) — matches the review exactly.
- Residual classes (documented, NOT failures): OpLine/OpSource line-mapping counts; ternary->OpSwitch codegen; OpBitcast/OpExtInstImport RWByteAddressBuffer address-math; PBA physical-load approximation; HZB screen-extent approximation; wall raster large body residual (binding-correct, body trimmed; f0ad trivial-PS under `TINYGLADE_RASTER_NO_COLOR`).

## Deliverables

- `reconstruction/shader-source/shaders/nani/*` + `nani/inc/*` (31 files), `wall/*` + `wall/inc/*` (11 files) — all `==RECONSTRUCTED==`, merged files carry MERGE provenance.
- `evidence/render-passes/integration/wave2-nani-cull-merge/{merge-log.json, verify.json, summary.md}` + per-module reports under `reports/`.
- Helper tool: `tools/render-passes/variant_verify.py` (manual `-D` variant fingerprint check; recompile_check has no `-D` passthrough).
