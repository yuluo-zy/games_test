# F-T recovery summary — terrain / building / read family

Date 2026-10-13. Owner F-T. Reconstructed all 15 owned mains + 6 include-only files under
`reconstruction/shader-source/shaders/` (flat terrain/building files, `terrain_editing/`,
`buildings/`). Details: `per-shader-results.json` (recompile_check structural fingerprint per module).

## Files done (21)
Mains: `upload_cpu_raster` (r16f/rgba16f), `timed_terrain_gizmo`, `demo_terrain_overlay`,
`read_terrain_stroke`, `mouse_terrain_raycast`, `terrain_derivative`, `rasterize_buildings`,
`rasterize_buildings_animate`, `terrain_lighting_texture`, and `terrain_editing/`:
`cache_stroke`, `calc_highlight_hill_mask`, `floor`, `rasterize_terrain_stroke`,
`read_terrain_height`, `read_terrain_rock_mask`. Include-only: `buildings/building_mask.inc`,
`buildings/curved_halftimber.inc`, `terrain_editing/inc_terrain_stroke.inc`, `sdf_utils.inc`,
`inc_highlight_active_terrain_chunk.inc`, `path_cobblestone_surface.inc`.

## Validation
18/18 compilable modules: **binding+entry+LocalSize+capabilities match on 17** (floor ps is the
exception — bindings differ). Structural PASS_STRUCTURAL on 2 (timed_terrain_gizmo ps,
demo_terrain_overlay ps). Remaining diffs are opcode-histogram only: code-opcode residuals are
dxc-version codegen (e.g. `||`/`!=` IEqual/LogicalNot folding, ByteAddressBuffer offset `*4+0`
folding, struct-copy access-chain splits); debug-info ops differ systemically (see below).
`read_terrain_stroke` cannot run through recompile_check (push-constant binding set=None crashes
`structural_fingerprint`); bindings/entry/LocalSize verified manually against the catalog.

## Top Unknowns
- `rasterize_terrain_stroke` body: PhysicalStorageBufferAddresses device-address stroke list not
  representable in HLSL; index list read from `stroke_indices` image instead (I).
- `floor` ps: parallax-occlusion + packed uint2 output; full 845-line body partially reconstructed.
- Include-only file bodies are Inference (no standalone bins).

## inc-layer issues (flagged, not modified)
1. `inc/consts.hlsl` includes `math_const.hlsl`; `inc/pack_unpack.hlsl` includes `bilinear.hlsl`
   -> `history_valid.hlsl`; `inc/tracked_buffer.hlsl` includes `smooth_max.hlsl`. The original
   closures for these consumers exclude them -> +3..4 OpString/OpSource per module (histogram FAIL
   source). `cache_stroke` genuinely used smooth_max.
2. `inc/color_library.hlsl` names its cbuffer `ColorLibraryData`; the original variable is
   `color_library` -> consumers must guard-skip it and redeclare (binding-name mismatch otherwise).
3. `recompile_check.py` crashes on modules with push-constant bindings (None in sort key).
