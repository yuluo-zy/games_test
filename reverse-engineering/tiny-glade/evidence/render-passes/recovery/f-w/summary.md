# F-W summary — water masks/spawn + garden

**Scope.** Reconstructed 16 shaders under `reconstruction/shader-source/shaders/`: `water/` (draw_water_mask,
read_water_mask, spawn_boats, spawn_lilypads, spawn_reeds, spawn_waterwheels), `garden/` (exclude_other_masks,
marching_squares, read_bush_positions_for_cpu, read_garden_mask, spawn_bushes_buildings, spawn_bushes_garden,
spawn_flowers_buildings, spawn_flowers_garden, spawn_small_flowers_garden, plus the shared defs.inc.hlsl and
placement_utils.inc.hlsl), and `garden_lowpoly_flower.raster.hlsl` (two hex variants).

**Result.** All 15 compute modules + 4 raster modules compile under the canonical dxc flags; **bindings + entry +
LocalSize + capabilities match the shipped bins in every case** (`per-shader-results.json`). `spawn_waterwheels`
is a full structural PASS (it is a stub — empty cs_main; RDA cap reproduced from the original frame_constants
unbounded-array). The remaining 15 are behavior-equivalent with a recorded opcode-histogram residual only.

**Residuals (all non-behavioral, recorded per file):**
- Include-graph drift in the reconstructed `inc/` layer pulls extra files into every consumer (`consts->math_const`,
  `tracked_buffer->smooth_max`, `pack_unpack->bilinear`), adding OpSource/OpString counts the original lacked.
- Image-descriptor load hoisting: installed dxc emits per-use OpLoad where the original dxc hoisted one.
- Comparison folds (`!(x>c)` -> FUnordLessThanEqual) and dropped redundant int2 bitcasts.
- Complex spawners (marching_squares, lilypads, boats) keep the original's unrolled do/switch/ternary structure;
  helpers/loops used here compile to fewer control-flow ops — algorithm equivalence verified against decompiles.

**Top unknowns (I/U).** Marching-squares case table constants and per-stone scale/color seeding; boat/reeds/lilypad
size and amplitude literals; garden "mask bands" thresholds; the RDA trigger inside frame_constants.hlsl (flagged —
an `inc/` gap, reproduced locally as a dead unbounded-array decl).

**Flagged inc issues (I did NOT modify `inc/*`).**
1. frame_constants.hlsl omits the unbounded-array decl that emits RuntimeDescriptorArray+SPV_EXT_descriptor_indexing
   (307/307 bins correlate frame_constants include with RDA).
2. current_view.hlsl declares `current_view_index` as a file-scope global; the original puts it in `$Globals` member 0
   (breaks the explicit `cbuffer $Globals` b13 binding — worked around locally).
3. find_root.hlsl references `terrain_height()` which the current terrain_heightmap.inc.hlsl does not define.
4. water_mask_sampler / path_mask_sampler / building_mask family sampler incs are still skeletons (other owners).
