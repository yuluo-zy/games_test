# GC-A: grass chain + clearing/billboard rasters (B build)

Date 2026-10-13. Reconstructed 24 files into the main tree (replacing ==SKELETON==
placeholders). 19 main bins all pass the HARD GATE (bindings + entry + LocalSize +
capabilities) on every b903 vs/ps/cs module. f0ad variants compile with the same source
(f0ad ps_main is the trivial-PS build-macro artifact — its ps has no bindings; targeted b903).

## Files
- grass compute chain: generate_grass.cs, grass/{calc_tiles, tile_bundle_count,
  tile_bundle_gen, bundles_prefix, tiles_prefix, calc_dispatch_args,
  calc_grasslet_bundle_ranges}.cs, grass/grass_common.inc
- grass rasters: grass.raster (blade quad vs + pack ps), grass_lod1.raster (3-vert LOD1
  vs + near-identical ps, includes grass.raster under the GRASS_LOD1 guard),
  grass_object_push.raster (soft push sprite)
- clearing: distant_billboard, instanced_background_tree, meadow_flower, meadow_plant,
  tree_canopy, tree_trunk (rasters), canopy_vert.inc,
  clearing/inc/{instanced_placement, canopy_common, forest_floor}
- billboard: billboard_forest (procedural cloud-billboard quads + 5-tap cluster normal),
  billboard_plant (plant-atlas blend + grass-patch sub-tile)

## Residuals (opcode_histogram only)
Documented in per-shader-results.json. Dominant classes are dxc-version artifacts
(original build dxc 16ceb560 vs installed 1.10.2605.37): OpSwitch vs BranchConditional
control flow, whole-matrix loads vs per-element access chains, Rodrigues matrix folds,
RWByteAddressBuffer byte-offset bitcast dance, and the systematic -1 OpSource/OpString
from the original's temp-include wrapper. OpLine/OpNoLine debug markers differ.

## Inc issues flagged (INC owners)
1. inc/lighting.hlsl references nested `color_library.indirect_light_tint`; flat-member
   color_library consumers cannot include it (omitted where unused; -1 OpSource).
2. inc/scatter.hlsl scatter_point() needs $Globals `scatter_grid_dim`; generate_grass
   carries a static stand-in.
3. inc/blue_noise.hlsl pins blue_noise_texture at t14; tree_canopy binds t21 (omitted).
4. inc/color_library.hlsl nested-member + placeholder b14; flat consumers redeclare.
