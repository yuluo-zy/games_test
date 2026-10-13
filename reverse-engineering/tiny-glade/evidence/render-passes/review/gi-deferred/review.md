# GI / lighting / deferred family — A vs B cross-review

Date 2026-10-13. Reviewer: independent. Both candidates recompiled against the ground-truth
`.bin` (`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`) with
`tools/render-passes/recompile_check.py`; target `trace_gi_candidates` and `deferred_solid`
modules additionally disassembled at opcode level. Machine-readable detail: `review.json`.

## Verdict

**Adopt B (`gi-deferred-b`) as the integration base, merging A's trace/grid/bin_lights
bodies and the inc-layer content into it.** B is the only side that covers all 18 in-scope
bins: its `DEFERRED_FULL_LIGHTING` / `BIN_LIGHTS_FULL` feature flags reproduce both shipped
variants (f0ad re-validated here: bindings/caps/entries exact, histogram diff 171–189;
8fae: histdiff 44), and its include closures are near-exact (OpSource within 1–5 of target
on every gi/deferred bin, where A sits at half). A's hard gate covers only 14 bins (all
b903 + compute); it never compiled the f0ad deferred or 8fae bin_lights variants.

## Per-file winners

| File | Winner |
|---|---|
| gi/trace_gi_candidates.cs.hlsl | **A** (cs_main body) |
| gi/update_gi_radiance.cs.hlsl | B |
| gi/eval_screen_radiance.cs.hlsl | B (content tie) |
| gi/reproject_screen_radiance.cs.hlsl | B (content tie) |
| gi/blur_screen_radiance.cs.hlsl | B (content tie) |
| gi/gi_settings.inc.hlsl | **A** (hi_px_subpixels pinned) |
| gi/restir.inc.hlsl | **A** (packing helpers) |
| light_binning/bin_lights_to_cells.cs.hlsl | **merge**: A body + B `BIN_LIGHTS_FULL` |
| light_binning/cell_prefix_{sum,subtotal,merge}.cs.hlsl | tie (near-converged) |
| light_binning/grid.inc.hlsl | **A** (helpers) |
| deferred/*_light_sun_sky.raster.hlsl | **B base** + merge fixes |
| deferred/light_sun_sky{,_prelude}.inc.hlsl | B |

## Trace: why A

The bin's `cs_main` matches A opcode-for-opcode: the morton-scramble loop (offsets 8130–8413,
`m = 32..1`, `acc += m*m*((3*bx)^by)`, `a/b = 63 - a/b` when `by==0 && bx==1`), the far-plane
early-out that writes zero 2x2 candidates (7647–8052), the 2x2 full-res block writes
(`ImageWrite candidate_radiance_tex[full_px + (i,j)]`), world normal via `view_to_world`, and
the blue-noise dither with `288 * (frame_index % 64)`. B's trace instead uses a blue-noise
hash ray, 1:1 writes, no early-out and ray length 30 — structurally different from the bin.
Both candidates delegate the traversal to the shared `inc/rt.hlsl`, which is where the real
gap is.

## BVH adjudication (concrete)

The shared `inc/rt.hlsl` `trace_software_bvh` is divergent for both sides. The bin decodes:

- **Node**: `PackedBlBvhNode { uint4 v[5]; }` (80 B). AABB center = `asfloat(v[0].xyz)`;
  per-axis extent scale `2^(byte-127)` from the three bytes of `v[0].w` (bits 0/8/16);
  near/far corner byte-triplets at `v[2].x/y/z`, `v[3].x/z`, `v[4].x` (three byte positions
  ⇒ up to three nested box levels per child).
- **Traversal**: stack entry `uint2 { base, packed_mask }`. `packed_mask & 0xFF000000 != 0` →
  triangle batch: load `rt_triangles[packed_mask]`, lane bit `1 << (tri_data & 31)`, clear it,
  push remainder while the high byte stays nonzero. Internal child index = `base +
  bitcount(remaining_mask)` (compact contiguous children). Ray-slot XOR:
  per-axis sign byte `(7 - (rd.x<0|rd.y<0|rd.z<0)) * 0x01010101`; child slot =
  `(FindUMsb(packed_mask) - 24) XOR (7 - signbyte)` & 31.
- **Stack**: per-lane 4-entry groupshared `uint2[4]` local; overflow spills to groupshared
  `rt_lds_stack[256]` at `lane + (sp-4)*64`.
- **Buffer types**: `rt_bl_bvh`/`rt_triangles` are `StructuredBuffer<PackedBlBvhNode>` /
  `StructuredBuffer<PackedTriangle>` (24 B `{ float v[3]; uint e[3] }`); `wall_color_ids` /
  `rt_triangle_meta` are `StructuredBuffer<uint>`. The bin uses structured
  `OpAccessChain`+`OpLoad`, not `ByteAddressBuffer.Load`.

**Recommended inc/rt.hlsl fix**: rewrite `trace_software_bvh` to this bitmask+compact-index
scheme with StructuredBuffer types (keep `uint4 data[5]` layout).

## Deferred PS guidance

- **GBuffer**: octahedral normal from `gbuffer.x` **low** 24 bits (A reads the high bytes —
  wrong source); albedo `gbuffer.y` low-24 RGB8, squared (both correct); flags
  `gbuffer.y >> 24` bits 1..6 (B's model correct).
- **Sun**: energy-preserving GGX diffuse + Cook-Torrance specular, F0 0.04 from `brdf_fg_lut`,
  1/π — B's model matches the bin constants (0.04 / 0.3183098733 present).
- **Sky**: SH band-1 irradiance from the 3-slab `indirect_radiance_tex` (stride `dims.x/3`)
  with `0.8862275481` — B's model matches.
- **RDI**: the real body is much larger (`Reservoir1spp { float, uint, float, float }` +
  `Reservoir1sppStreamState { float, float }`, 18 loops / 17 switches). Both candidates are
  structural-only; keep B's gather + grid iteration, expect the histogram to stay far until
  the temporal reservoir loops are reconstructed.
- **Output**: the bin writes SV_Target alpha **2.0** (neither; A=1.0, B=0.0).

## Inc-layer fixes both sides surfaced

- `inc/rt.hlsl`: rewrite `trace_software_bvh` (bitmask+compact-index, StructuredBuffer types).
- `gi/gi_settings.inc.hlsl`: keep A's `hi_px_subpixels` table (bin debug names pin it in
  eval/blur/update; reproject/trace don't embed it → unused static dropped, so defining it in
  the shared inc matches the original).
- `gi/restir.inc.hlsl`: keep A's uint2 payload / two-half16-weight packing helpers.
- color_library guard-skip + per-pass cbuffer (deferred b23 / trace b18) is the correct
  pattern (matches generate_grass convention) — both sides already use it.
