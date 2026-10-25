# wave2 gi-deferred merge — summary

Date 2026-10-13. Applies the independent gi-deferred A-vs-B review
(`evidence/render-passes/review/gi-deferred/review.json` + `review.md`) into the main tree and
adjudicates the inc-layer fixes, then re-verifies against the ground-truth `.bin`s.

## Merged files (reconstruction/shader-source/shaders/)

- **gi/** — `trace_gi_candidates.cs.hlsl` = **A body** (morton scramble, far early-out, 2x2 block
  writes, 288*(frame%64) dither) on B's near-exact include closure. `update_gi_radiance` /
  `eval_screen_radiance` / `reproject_screen_radiance` / `blur_screen_radiance` = **B content**.
  `gi_settings.inc.hlsl` (hi_px_subpixels) + `restir.inc.hlsl` (reservoir packing) = **A**.
- **light_binning/** — `bin_lights_to_cells.cs.hlsl` = **merge**: A's RNG
  `wang_hash((idx*1664525+hash(frame_index)+1013904223)*1664525)` + `StructuredBuffer<LanternPacked>`,
  B's `BIN_LIGHTS_FULL` flag for the 8fae variant, bin's carried-rng mantissa reservoir replace.
  `grid.inc.hlsl` = **A** (helpers). `cell_prefix_{sum,subtotal,merge}` = tie, A adopted.
- **deferred/** — all four `*_light_sun_sky.raster.hlsl` + `light_sun_sky{,_prelude}.inc.hlsl` =
  **B base** (`DEFERRED_FULL_LIGHTING` reproduces b903/f0ad).

Every merged main keeps the `// ==RECONSTRUCTED==` header with a MERGE provenance note.

## rt.hlsl reconciliation (Part 2)

`inc/rt.hlsl` already implemented the hybrid morton decode (compact-index entry, corner-byte
decode, extent `asfloat(byte<<23)`). Reconciled to the fuller gi-deferred decode by adding the
**per-lane `uint2[4]` local stack spilling to groupshared `rt_lds_stack[256]` at
`lane+(sp-4)*64`** (confirmed in the `_gi_trace_gi_candidates` bin) and hoisting the octant byte.
Two review claims were **rejected on bin evidence**: the buffers stay `ByteAddressBuffer`
(the bin decompiles to `.Load<uint4>`; both candidates hard-gate PASS) and the corner decode
stays 2-child x 4-byte-positions (the bin uses 0/8/16/24, not 3 nested levels). Verified: the gi
trace consumer and the water `deferred_rt.cs.hlsl` consumer both still hard-gate.

## Inc fixes (Part 3)

- `inc/lighting_indirect.hlsl`: **removed** the spurious `lanterns.hlsl` include edge (gi
  trace/update closures embed lighting_indirect without lanterns; deferred gets lanterns via
  `light_sun_sky.inc.hlsl`).
- `inc/terrain_heightmap.inc.hlsl`: already guarded (`TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL`,
  water-ice merge); verified with the gi update (no terrain binding) and trace (binds t14).
- `inc/blue_noise.hlsl`: no fixed register already; eval redeclares at t14 and uses the 3-arg
  texture-parameter helper (coordinated with the concurrent water-ice merge).
- `inc/color_library.hlsl`: placeholder b14 left as-is; consumers redeclare flat (trace b18 /
  deferred b23).

## Verification (Part 4)

`tools/render-passes/recompile_check.py` (+ a variant driver for the `-D` toggles) on every
merged main: **18 checks / 22 modules — hard-gate PASS 22, FAIL 0, compile FAIL 0**
(bindings / entries / LocalSize / caps all match; reduced variants f0ad and 8fae checked with
`-D DEFERRED_FULL_LIGHTING=0` / `-D BIN_LIGHTS_FULL=0`). Residual classes are histogram-only:
trace march/material simplification, deferred b903 RDI temporal reservoir loops, f0ad
trivial-PS, OpSource closure within 1-5, OpSourceContinued / path-form OpString, the water
`StorageImageWriteWithoutFormat` dxc-version capability, and the grid clamp-bound flag
(30/15/30 vs 31/15/31). Detail: `merge-log.json` + `verify.json`.

## Evidence override (important)

Two of the review's deferred-PS "merge deltas" were contradicted by direct bin disassembly and
were **not applied**: the normal decode is `RGB8 normalize` of the gbuffer.x low bytes (B's
version, not A's octahedral) and the SV_Target alpha written is `0.0` (B's value, not 2.0).
Both were verified in the solid and canopy b903 ps modules.
