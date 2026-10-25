# water-ice-a decisions (A-side, dual reconstruction — no coordination with water-ice-b)

Scope: water/ice surface + software-BVH deferred-RT family (15 files + helper incs
ice_shared/waves/water_surface_frag/waterfall_vert/water_mask_sampler) under
`candidates/water-ice-a/shaders/`.

**Validation:** all modules compile; hard gate (bindings/entry/LocalSize/caps) PASS on
33/36 module checks. 3 caps residuals (deferred_rt, ice_refl_resolve, ice_refr_resolve):
explicit-format storage images emit ExtendedFormats but target also carries
WriteWithoutFormat (dxc-version; ice_refl_resolve adds ReadWithoutFormat). Opcode
histogram residuals recorded everywhere (dxc-version codegen + inc drift).

**Key decisions:** deferred_rt/ice_deferred_rt BVH traversal is morton-coded
(8-entry uint2 stack, firstbithigh+countbits+octant XOR), NOT inc/rt.hlsl's
trace_software_bvh — reconstructed inline (inc flag). Raster 16-hex variants are
feature switches (b903e=full, f0adff=no-output/RT-ray), gated on inference macros.
deferred_rt bounce is dead at runtime; branches kept so blue_noise Load survives
(t15 binding). Perlin used by snow_plane/x_paths/waterfall is explicit-arithmetic
bilinear, inlined (inc/perlin.hlsl lerp form emits extra OpExtInst). RT tracer
water-surface color static is unused by the output — 0 (U).

**inc flags:** rt.hlsl traversal mismatch; blue_noise.hlsl t14 vs family t15;
perlin.hlsl lerp body; terrain_heightmap.inc references unbindable texture;
current_view_index $Globals member 0.

**Top unknowns:** morton child-index decode semantics (I); wave octave table (I);
waterfall perlin3 lattice (I); waves/water_surface_frag inc split (I).
