# gi-deferred-b reconstruction decisions

Date 2026-10-13. Scope: `gi/`, `light_binning/`, `deferred/` (B build). Independent dual side; no coordination with gi-deferred-a.

## Key structure choices
- **Feature variants**: the two 16-hex raster variants are compile-time flags: `DEFERRED_FULL_LIGHTING=1` → b903 (full deferred sun+sky+lantern, SV_Target output); `=0` → f0ad (reservoir-gather only, no color output — reproduced with a `void ps_main`). `bin_lights_to_cells` uses `BIN_LIGHTS_FULL=1` → 14dc (writes light indices), `=0` → 8fae (count-only).
- **color_library**: deferred/trace bind at b23/b18 (inc placeholder is b14); consumers guard-skip `TINYGLADE_INC_COLOR_LIBRARY_HLSL` and declare a flat/struct cbuffer at the pass binding (generate_grass convention).
- **$Globals**: declared per shader with the exact OpMemberName set; only `lantern_count` for bin_lights, the full dims struct for deferred.
- **RT**: trace uses `inc/rt.hlsl` software-BVH traversal; `PackedBlBvhNode`/`PackedTriangle` shared.
- **Reservoir/lanterns**: reuse `inc/reservoir.hlsl`, `inc/lanterns.hlsl`, `inc/lighting_indirect.hlsl` grid helpers; the f0ad gather keeps WaveActiveAllTrue + QuadReadAcrossDiagonal so both variants carry GroupNonUniformVote/Quad.

## Names
- Flag names (`DEFERRED_FULL_LIGHTING`, `BIN_LIGHTS_FULL`) and fast-sqrt/`hi_px_subpixels`/`pack_half2x16` local helper names are Inference.

## Open Unknowns
- LanternPacked type survival, ByteAddressBuffer-load OpBitcast, f32tof16 scalarization, and OpSource/OpString +1 temp-path residual are dxc-1.10.2605.37-vs-original version residuals (same class as accepted F-D prefix_scan).
- `grid.inc.hlsl`, `gi_settings.inc.hlsl`, `restir.inc.hlsl` bodies fully inlined → content U (kept minimal, fold-identical).
- Full deferred PS (2049-line spirv-cross) reconstructed structurally; opcode histogram has large residuals (recorded, not converged).
