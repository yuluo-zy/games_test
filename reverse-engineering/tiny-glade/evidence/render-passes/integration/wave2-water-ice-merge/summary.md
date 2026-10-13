# Wave-2 water/ice merge — summary

Date 2026-10-13. Applies the water-ice cross-review verdict
(`evidence/render-passes/review/water-ice/review.md` + `review.json`) into the main tree.
Evidence-driven; original `.bin` untouched; other wave-2 families untouched.

## Part A — water/ice merged into the main tree (14 files, 22 modules)

Candidate **A adopted wholesale** as the family base; the review's B/bin fragments folded in.
All merged files carry the `// ==RECONSTRUCTED==` header with MERGE provenance
(`evidence/render-passes/integration/wave2-water-ice-merge/merge-log.json`).

Per-file winners applied:
- `water/deferred_rt.cs.hlsl`, `water/ice_deferred_rt.cs.hlsl` — **merge (A base)**: A count-byte
  mask accumulation + node-entry/leaf assembly + break/pop control flow, **plus** B's octant
  sign-selected corner bytes (child A `data[2].x/.z`, child B `data[2].y/.w`), combined
  accumulation feeding the node entry + leaf, blue-noise scroll `+bounce*137`, post-bounce
  miss-sky branch, and `1-dot(view_ray,tangent)` fresnel.
- `water/ice_refl_march.cs.hlsl` — **merge (A base)**: Hilbert index + retry-on-bad kept (A);
  accept-gate polarity fixed to the bin (`reflect(basis,v).z<0 -> accept`).
- `water/ice_refl_resolve.cs.hlsl` — **A**: `inc/curve.hlsl` include dropped; `FrexpStructType`
  + frexp now sourced from `water/ice_shared.inc.hlsl`.
- `water/ice_refr_resolve.cs.hlsl`, `water/calc_deferred_rt_args.cs.hlsl`,
  `water_plane_refraction_copy.raster.hlsl`, `waterfall_refraction_copy.raster.hlsl`,
  `water_snow_plane.raster.hlsl`, `water_x_paths.raster.hlsl`, `ice_plane.raster.hlsl`,
  `water/ice_cracks.raster.hlsl` — **A** (ice_cracks also gains the `water_mask_sampler.inc`
  include the bin closure carries).
- `water_plane.raster.hlsl` / `waterfall.raster.hlsl` — **merge (A base)**: PS moved into the
  shared `water/water_surface_frag.inc.hlsl` (B's layout — the bin PS lives in that inc), with
  the bin **theme==4 fast path** (`waves=0,ripples=0,normal=(0,1,0)`) and **TAA jitter table**
  `_244[frame_index&3]` folded in.
- `water/water_surface_frag.inc.hlsl`, `water/waterfall_vert.inc.hlsl`,
  `water/waves.inc.hlsl`, `water/ice_shared.inc.hlsl` — **merge** per the review's shared-inc
  and hash guidance.

## Part B — adjudicated inc fixes (8 applied, 0 reverted, 0 blocked)

1. **`inc/rt.hlsl`** — `trace_software_bvh` rewritten as the morton `uint2[8]`-stack hybrid from
   the bins (review `bvh_traversal_finding`): A's count-byte accumulation, node-entry/leaf
   assembly and break/pop control flow; B's sign-selected corner decode; both sides' shared
   morton/octant/slab code. Extent decode fixed to `asfloat(byte<<23)` (the old
   `exp2(float(byte))` was wrong). `decode_bvh_corner`/`load_child_aabb` removed.
2. **`inc/terrain_heightmap.inc.hlsl`** — `terrain_height()` guarded behind
   `TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL` so it compiles without a consumer-declared global
   texture; `water_plane_refraction_copy` (which embeds the include but binds no terrain
   texture) uses the guard. Verified: `water_plane_refraction_copy` and `stairs.cs` (declares
   the texture) both hard-gate PASS.
3. **`inc/blue_noise.hlsl`** — global `blue_noise_texture` + fixed `register(t14)` removed
   (family binds t15, apply_volumetrics t17, tree_canopy t21); the helper now takes the texture
   as a parameter; `text.raster` declares it locally at t14. Verified `text.raster` PASS.
4. **`inc/perlin.hlsl`** — `perlin()` rewritten to the explicit-arithmetic bilinear blend
   (family consumers embed the `asfloat(hash & 0x7FFFFF | 0x3F800000) - 1` form). Verified via
   `water_plane` full PS foam.
5. **`water/waves.inc.hlsl`** — scalar two-component hash01 (x=h, y=h+(h<<15), x!=y) for the
   per-octave offset (A's `.xx` replicate was wrong) + the bin's 3-phase central-difference
   ripple normal. Consumer consequence: `leaf_particles`/`lilypad`/`lilypad_flower`/`reed`
   moved their `water_noise_texture` register decl before the include (eager dxc sema).
6. **`water/waterfall_vert.inc.hlsl`** — `waterfall_hash3` (3-component uint3 chain
   1664525/1013904223, matches the bin VS 8-corner chain); `vs_main` lives in the inc.
7. **`water/ice_shared.inc.hlsl`** — `FrexpStructType` + frexp workaround defined here
   (ice_refl_resolve bin name + closure); `inc/curve.hlsl` dropped from ice_refl_resolve.
8. **`water/water_mask_sampler.inc.hlsl`** — already correct (green channel `.y`); no edit.

## Part C — verification

- **b903**: 14/14 bins, 22/22 modules hard-gate **PASS** (bindings/entry/LocalSize/capabilities
  match). Zero `FAIL_DXC`.
- **f0ad PS variants**: 8/8 reproduce the exact f0ad PS binding+capability sets from the same
  source via feature macros (`WATER_SURFACE_FULL` / `WATER_SNOW_PLANE_COLOR` /
  `WATER_ICE_CRACKS_COLOR` / `WATER_X_PATHS_COLOR` / `WATER_REFRACTION_COPY` /
  `WATERFALL_REFRACTION_COPY`; ice_plane identical source).
- Residual classes (documented, not failures): opcode histogram on every module (dxc-version
  codegen + include-closure drift); `StorageImageWriteWithoutFormat`/`ReadWithoutFormat` caps
  on the r11g11b10f storage images of deferred_rt / ice_refl_resolve / ice_refr_resolve
  (dxc-version; review water-ice hard_gate summary).

## Blockers

None. `merge-log.json` + `verify.json` in this directory.
