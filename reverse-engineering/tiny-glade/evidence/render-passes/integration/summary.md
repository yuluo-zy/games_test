# P1c Integration Summary — inc-layer fixes, F-R verification, full-tree compile

Date 2026-10-13. Evidence-driven; original .bin untouched; wave-2 families untouched.

## Part A — adjudicated inc fixes (13 applied, 13 verified, 0 reverted, 0 blocked)
1. **RDA trigger** (O): `inc/frame_constants.hlsl` declares `bindless_textures_dyn[] : register(t0, space2)`; 19 consumer-local triggers removed. apply_screen_shadows caps gain RuntimeDescriptorArray; bindings unchanged; all 307 target frame_constants bins carry RDA.
2. **current_view_index** (O): inc already non-static; in-consumer `cbuffer $Globals : register(b13)` pattern verified (demo_terrain_overlay, $Globals b13).
3. **terrain_height** (I): defined in `terrain_heightmap.inc.hlsl`; 27 consumers re-ordered texture decls before the include; stairs.cs local copy removed. stairs.cs compiles, hard gate PASS.
4. **include-drift prune** (O): removed 13 provably-spurious inc edges (tracked_buffer->smooth_max, math->bicubic/quad_mesh/unjitter_taa, consts->math_const, pack_unpack->bilinear, bilinear->history_valid, material_eval->brdf, solid_material->brdf_lut, brdf_lut->highlight_color, quad_mesh->image, grass/plant->raymarch, ycbcr->color/math, bitonic_sort->coc, building_mask->foliage_exclusion). Wave-1 EXTRA-file mains ~121->0; closure-match mains 3->81; apply_screen_shadows OpSource 26->22 (target 23).
5. **mask sampler incs** (I): water_mask_sampler filled (get_water_mask/is_water); building_mask gained get_building_mask; path_mask_sampler kept (F-R).
6. **srgb** (O): if/return -> if/else assignment.
7. **blue_noise register** (U, documented deviation): register(t14) added; text.raster consumes it. Others bind t15/t16/t17 -> per-consumer registers noted for main agent.
8. **color_library cbuffer** (O): renamed to `color_library`.
9. **samplers s5..s8** (O): 13-sampler set restored in `inc/samplers.hlsl` (dxc DCEs unused ones from SPIR-V; declarations position $Globals at b13); 23 consumer-local duplicates removed. flag_small/lilypad/sheep $Globals b13 restored.
10. **highlight_color** (O): includes color_library + ordering note.
11. **ray_cone / wall_distortion** (I): skeletons filled, minimal.
12. **StairsCounter** (O): `{int count}` per bin; stairs_counter.cs uses inc.
13. **history_valid -> is_history_valid** (O): rdi/ssgi guard hacks removed; apply_screen_shadows/calculate_reprojection_map re-includes restored.
14. **rev_blur s3** (O): no change — self-contained, verified.

## Part B — F-R verification
aggregate_results.py folded sub-agent reports. Fresh recompile_check on all 15 F-R bins: 12 hard-gate PASS; 3 f0ad-variant raster bins have trivial-target ps modules (variant artifact; b903 variants PASS). All paths/* rocky_terrain/* files now have reports.

## Part C — full-tree compile
260 reconstructed mains compiled with canonical dxc commands: **125 PASS, 135 FAIL** — all 135 are wave-2 skeleton placeholders (missing entry point). Zero dxc errors in the reconstructed wave-1 tree.

## Hard-gate sweep
151 wave-1 mains: 108 hard-gate PASS; residuals are documented dxc-version capability artifacts (StorageImageExtendedFormats), f0ad trivial-ps variants, and pre-existing terrain_editing/floor.ps incomplete body.

## Blockers
- blue_noise_texture per-consumer registers (t14-t17) — fixed register can't serve all.
- Pre-existing consumer MISSING include gaps in F-V/F-D files (critters/flag_* missing lighting/brdf/etc.) — next-wave consumer completeness.
