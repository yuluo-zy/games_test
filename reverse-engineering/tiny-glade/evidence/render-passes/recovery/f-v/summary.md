# F-V recovery summary — critters / roof / particles / ivy

**Scope (34 files):** 20 flat (15 mains + 5 include-only), 6 critters, 4 roof, 4 ivy. All reconstructed.

## Validation
`recompile_check.py` (dxc 1.10.2605.37) against each bin's own -E/-T. **Acceptance: bindings +
entry + local_size match exactly.** Result: all 42 modules of the canonical b903 variant match
bindings+entries+local_size (0 FAIL_DXC). Histogram converges close; residual diffs are
dxc-version artifacts (see below). The f0adff raster variant compiles a different PS (depth-only
style: near-empty/other resource set) and cannot be produced from the same source without a build
macro — reconstruction targets the b903 variant.
Unfixable toolchain artifacts (recorded, not chased): target `RuntimeDescriptorArray` capability +
`SPV_EXT_descriptor_indexing` extension (installed dxc never emits them); OpSource/OpString/OpLine/
OpNoLine counts (reconstructed inc adds brdf_lut/bilinear/smooth_max/bicubic/quad_mesh/unjitter_taa
the original closure lacked); whole-struct vs per-field SSBO loads, resource-load CSE, ternary-vs-branch lowering — dxc codegen.

## Key findings
- Original samplers.hlsl had 13 samplers (s0..s12); inc/samplers.hlsl has s0..s4,s9..s12 →
  every consumer $Globals came out one-to-five slots low. Workaround: each F-V raster declares the
  unused s5..s8 (leaf_particles adds t13) to reproduce set0/b13 or b14. **INC-A issue.**
- inc/highlight_color.hlsl references color_library without including it; consumers must include
  inc/color_library.hlsl first. **INC-C include-order issue.**
- inc/ray_cone.hlsl and inc/wall_distortion.hlsl are still empty skeletons (no codegen needed).
- roof/roof_tile.inc.hlsl (RoofShingle/RoofPushConstants/half2x16/quat) recovered verbatim from
  OpMemberName tables; drives roof_tiles_gravity.cs.
- PhysicalStorageBufferAddresses consumers can't be spirv-cross decompiled: stairs_floor/plaster/
  wooden_plank/smoke structs recovered from OpMemberName; bodies flagged Unknown for wave-2 nani.
- roof floor incs (roof_material_id/dormer_hole/wooden_floor/stone_floor) partial: dormer-hole
  occlusion + stone height ray-march recovered; per-variant material dispatch Unknown.

## Top Unknowns
roof floor material dispatch; nani physical-addressing helper bodies; leaf_particles noise-loop
codegen detail (bindings exact, histogram drifts on the value-noise loops).

## Inc issues flagged
1) samplers s5..s8 missing; 2) highlight_color include-order; 3) ray_cone/wall_distortion skeletons.
