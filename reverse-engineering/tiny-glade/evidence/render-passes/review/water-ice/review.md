# water-ice family cross-review — A vs B

Reviewer verdict for the two independent reconstructions of the water/ice surface +
software-BVH deferred-RT HLSL family. Ground truth: the shipped `.bin` modules
(`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`), extracted at
SPIR-V magic offset 23 and cross-checked with `spirv-cross --hlsl --shader-model 64`.
Per-file detail is in `review.json`; this is the recommendation.

## Bottom line

**Adopt candidate A wholesale, then apply the small set of B/bin merges listed below.**
A is the better base on every axis that matters: it matches the bins' *embedded include
closures* (the OpString/OpSource lists) for every file in the family — a structural
signal the original hard gate does not use; its PS bodies are faithful transcriptions of
the bin structure (water_plane PCSS/DoF/foam/composite, ice_plane hex-lattice roughness,
snow_plane thresholds/tint); and A reproduces **both** the b903 and f0ad PS binding sets
from a single source via its feature macros, whereas B records the f0ad PS bindings as a
permanent hard residual. B's files are consistently condensed approximations (minimal
include closures, simplified inner loops) that do not reproduce the bins' structure.
Both sides are equal on the hard gate for the b903 modules; the caps residuals
(StorageImageWriteWithoutFormat/ReadWithoutFormat on explicit-format images) are dxc
version artifacts and are not grounds for preference.

## Per-file winner summary

| file | winner | why |
|---|---|---|
| water/deferred_rt.cs.hlsl | **merge (A base)** | A: closure, count-byte mask, node-entry, control flow, march, resolve. B: octant corner selection, `+bounce*137` scroll, miss sky branch, fresnel `dot(ray_dir,tangent)`. |
| water/ice_deferred_rt.cs.hlsl | **merge (A base)** | same hybrid as deferred_rt. |
| water/ice_refl_march.cs.hlsl | **merge (A base)** | A: Hilbert index + retry-on-bad + closure. B: accept-gate polarity (`reflect(basis,v).z<0 -> accept`); A inverts it — fix. |
| water/ice_refl_resolve.cs.hlsl | **A** | bin carries `FrexpStructType`; A uses it, B invents `ice_quantize_radiance`. Drop A's curve.hlsl include (not in bin closure). |
| water/ice_refr_resolve.cs.hlsl | **A** | closure matches bin exactly; behavior equivalent. |
| water/calc_deferred_rt_args.cs.hlsl | **A** | bin is RWByteAddressBuffer; B uses RWStructuredBuffer. |
| water_plane.raster.hlsl | **merge (A base)** | A PS = bin structure; B PS condensed. Merge theme-4 fast path + TAA jitter table from bin/B. |
| water/water_surface_frag.inc.hlsl | **merge** | B's shared-inc layout matches the bin (PS lives in the inc); A's PS body is the faithful one. |
| water_ice_cracks.raster.hlsl | **A** | A's path `water/ice_cracks.raster.hlsl` matches the bin's embedded source path; add water_mask_sampler.inc. |
| ice_plane.raster.hlsl | **A** | A has hex-lattice SampleGrad roughness + PCSS + 5 RTs; B's roughness is a single uv(0,0)/mip1 sample. |
| waterfall.raster.hlsl | **merge (A base)** | A full PS+VS; adopt B's waterfall_vert hash3 + inc layout. |
| water_snow_plane.raster.hlsl | **A** | exact thresholds/tint `uint2(4286644096,114677205)`. |
| water_x_paths.raster.hlsl | **A** | closure (texture_blend/texture_utils/mask sampler incs). |
| water_plane_refraction_copy.raster.hlsl | **A** | closure; note this bin DOES embed terrain_heightmap.inc. |
| waterfall_refraction_copy.raster.hlsl | **A** | A has the procedural VS the bin embeds. |
| water/waves.inc.hlsl | **merge** | A tables {2,4,8,12}/{2,2,0.5,0.2}/{17,18,20,21} confirmed; B two-component hash01; neither has the 3-phase ripple normal. |
| water/waterfall_vert.inc.hlsl | **merge** | B hash3 matches bin; A mesh layout; vs_main belongs in the inc. |
| water/ice_shared.inc.hlsl | **merge** | B's FrexpStructType here (bin name+closure), A's half helpers. |
| water/water_mask_sampler.inc.hlsl | **A** | green channel `.y` matches shared tree + family bins; B uses `.x`. |

## BVH-traversal adjudication (the crux)

Both sides correctly recognized that `inc/rt.hlsl`'s `trace_software_bvh` (non-morton,
byte-field child decode) does not match what the bins inline, and both reconstructed a
morton-coded traversal: `firstbithigh` pop, octant permutation `(7-oct)*0x01010101`,
morton child-index decode `node.x + countbits(mask & ~(0xFFFFFFFF << ((slot-24)^octant)))`,
extent decode `asfloat((byte)<<23)` (= 2^(byte-127); inc/rt.hlsl's `exp2(float(byte))` is
wrong), slab test with t_min=1e-4, and the `(data[1].z ^ (octant & ((((data[1].z&(data[1].z<<1))&0x10101010)>>4)*255))) & 0x1F1F1F1F` lane-mask formula.

The bins' actual traversal (deferred_rt + ice_deferred_rt are identical here) is a
**hybrid**, and neither candidate matches it alone:

- **A-correct, B-wrong:** mask accumulation `slot |= ((count>>shift)&255) << ((mask>>shift)&31)`
  (B emits a single `0x1` bit); node entry `uint2(data[1].x, (accum&0xFF000000)|((data[0].w>>24)&255))`
  and leaf `uint2(data[1].y, accum&0xFFFFFF)` (B reuses the current node's remainder);
  the traversal break/pop semantics (`push_depth==0 -> break`, else pop stack).
- **B-correct, A-wrong:** the octant **sign-selected corner bytes** — `c0..c5` chosen as
  `rd.x<0 ? data[2].z : data[2].x` (and mirror for y/z) — A reads `data[2].x/.z` per lane
  without the sign swap.

**Recommended inc/rt.hlsl fix:** replace `trace_software_bvh` with the morton `uint2[8]`-stack
traversal transcribed from the bins — A's mask accumulation + node-entry/leaf assembly +
control flow, B's sign-selected corner decode, both sides' shared morton/octant/slab
scaffolding — and fix the extent decode to `asfloat(byte<<23)`. Keep `PackedBlBvhNode`/
`PackedTriangle`.

## Inc-layer fixes both sides surfaced

1. `terrain_heightmap.inc.hlsl` — the reconstructed inc references a consumer-declared
   texture the family never binds, but `water_plane_refraction_copy` bin DOES embed the
   include → make the inc compile without a binding instead of dropping it.
2. `blue_noise.hlsl` — pins t14; family binds t15 (consumers omit/include locally).
   Align with tracker inc-fix #17.
3. `perlin.hlsl` — lerp body emits extra OpExtInst; family consumers embed explicit
   `asfloat(hash & 0x7FFFFF | 0x3F800000) - 1` bilinear/trilinear → rewrite to explicit form.
4. `waves.inc.hlsl` — per-octave offset hash must be scalar `hash(uint(cell))` producing a
   two-component hash01 (x=h, y=h+(h<<15)); ripple is a 3-phase central-difference normal.
5. `waterfall_vert.inc.hlsl` — VS lattice hash is the 3-component uint3 chain
   (1664525/1013904223); `vs_main` belongs in the inc.
6. `ice_shared.inc.hlsl` — define `FrexpStructType` here (ice_refl_resolve bin name +
   closure); drop `inc/curve.hlsl` from ice_refl_resolve.
7. `water_mask_sampler.inc.hlsl` — green channel `.y` (matches shared tree + family).
8. `current_view_index` as `$Globals` member 0, explicit `cbuffer $Globals : register(b13,space0)`
   — both sides already conform (tracker inc-fix #2/#6).

## Recommend to the main agent

Adopt **A** as the base for the whole family. Apply the A/B/bin merges above (the BVH
hybrid, the theme-4 fast path + TAA jitter on water_plane, the accept-gate polarity in
ice_refl_march, the shared-inc file layout for water_surface_frag/waterfall_vert, the
waves/ice_shared hashes), then fix the inc layer. B should not be adopted wholesale: its
only structural wins (octant corner selection, accept-gate polarity, waterfall_vert hash3,
shared-inc layout, FrexpStructType placement) are individually portable into A.
