# gi-deferred-a decisions

Scope: gi/, light_binning/, deferred/ (18 files), A-side. Hard gate (bindings/entry/
LocalSize/caps) PASSES on all 13 bins / 17 modules; histogram fails everywhere
(residuals: dxc-version OpBitcast/loop codegen, tmp-path OpSource +1, inc closure
drift, simplified bodies).

## Key structures / names
- grid.inc: 32x16x32 light grid (scale 0.24615…, offset 16,2,16), index 32*(32y+z)+x.
- light_binning = Hillis-Steele 2-per-lane scan (barrier per step, byte-addr
  `4u*(2*lane+gid*1024)`), push_constants {range_end} in cell_subtotal.
- bin_lights_to_cells: sphere→grid AABB, InterlockedAdd(1<<20), reservoir keeps 64.
- gi_settings.inc: hi_px_subpixels[4] (pinned); restir.inc: reservoir uint2 packing.
- gi chain: update seeds reservoir; eval 8-sample golden-spiral SH; reproject octahedral
  temporal + spatial fallback; blur adaptive 3x3 bilateral; trace cosine-hemisphere ray
  + screen march + software BVH (rt.hlsl). All 8x8.
- deferred: vs = fullscreen triangle; ps = GBuffer octahedral normal + albedo, sun/sky/
  lantern-grid/indirect/RDI; wave ops for caps; color_library redeclared at b23.
  light_sun_sky{_prelude}.inc carry shared helpers.

## Open unknowns
- bin_lights 8faee3cc reduced variant (no frame/light_indices); reservoir RNG carry-chain.
- trace march terminator + wall/terrain material eval Inference.
- deferred PS full BRDF/RDI body simplified (largest histogram gap).
- f0adff RDI-only deferred variant not compiled; include-closure residuals everywhere.
