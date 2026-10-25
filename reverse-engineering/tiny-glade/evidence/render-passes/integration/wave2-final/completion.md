# Wave-2 Final — Completion

Date 2026-10-13. Shader tree `reconstruction/shader-source/shaders/` ~100% reconstructed.

## Orphans (4/4 done)
- `mei_shadow.raster.hlsl` — O. Shadow-mask softener: fullscreen-triangle VS + spiral-tap PS (`sampler_nnc`, `blue_noise_texture`, `input_tex`, `$Globals` b13). Hard gate PASS on both b903 + f0ad.
- `instanced_roof_beams.raster.hlsl` — O. 80-byte RoofSupportBeam affine + circle-plane lean, perlin bend, inverse-transpose normal, fence-color beam. Hard gate PASS both variants.
- `instanced_roof_spire.raster.hlsl` — O. 32-byte RoofSpire, profile-gated scaling, back-ease rise, 2D perlin + inline 3D float2 noise. Hard gate PASS both variants.
- `water/water_object_push.inc.hlsl` — I. Shared water-object displacement helper (theme-4 gate + wave jitter); include-only, 7 consumer bins re-verified unchanged.

## Full-tree compile
260 mains with bins: **258 PASS**, 2 genuine FAIL — `nani_solid_vertex_color`, `nani_velocity_only` (`Affine3Packed` redefinition: `inc/utils.hlsl` vs `wall/inc/draw_lists.hlsl`, pre-existing wave-merge conflict). 14 subdir include-only files have no main bin.

## Hard-gate sweep (b903 primary)
**235/260 mains PASS** (bindings/entry/local-size/capabilities). 25 fail: 17 capabilities (dxc-version class), 5 bindings (nani_gothic_window_bricks, nani_solid_vertex_color_bricks, terrain_editing/floor, vermeer/generate_strokes, light_binning/bin_lights_to_cells), 1 local_size (tgsr), 2 FAIL_DXC (above). Opcode-histogram drift present in ~369 modules (dxc-version codegen, structural gate unaffected).

## Coverage vs 404 bins
**404/404 bins** have a matching reconstructed main (100%). f0ad raster ps stubs (86) = documented variant artifact; no-hzb `-D` variants 9/13 modules prior-validated PASS.

## Cosmetic
Removed leftover line-1 `==SKELETON==` blocks from 21 dof/aa/tgsr files carrying `==RECONSTRUCTED==` headers (candidates/ untouched).

## Blockers
The 2 Affine3Packed-conflict nani mains (adjudicate which definition wins); 17 capability + 5 binding + 1 local_size residuals to reconcile in a future wave.
