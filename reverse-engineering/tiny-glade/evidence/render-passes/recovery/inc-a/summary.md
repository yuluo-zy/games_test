# INC-A summary — shared inc layer reconstruction

Date: 2026-10-13. Owner: INC-A. All 32 owned files under
`reconstruction/shader-source/shaders/inc/` reconstructed from compiled SPIR-V
evidence and compile-validated.

## Files (32/32)
- High-confidence / observation-heavy: `frame_constants.hlsl`, `current_view.hlsl`,
  `samplers.hlsl`, `hash.hlsl`, `pack_unpack.hlsl`, `uv.hlsl`, `utils.hlsl`,
  `bicubic.hlsl`, `bilinear.hlsl`, `color/srgb.hlsl`, `color/ycbcr.hlsl`,
  `quad_mesh.hlsl`, `upsample/easu.hlsl`.
- Framework / inference-heavy (marked I/U, documented): `common.hlsl`,
  `math.hlsl`, `math_const.hlsl`, `consts.hlsl`, `projection.hlsl`, `halfres.hlsl`,
  `texture_utils.hlsl`, `atomics.hlsl`, `debug_utils.hlsl`, `tracked_buffer.hlsl`,
  `image.hlsl`, `texture_blend.hlsl`, `parallax_mapping.hlsl`,
  `stochastic_tiling.hlsl`, `color/{math,xyz,luv,helmholtz_kohlrausch}.hlsl`,
  `upsample/lanczos2.hlsl`.

## Key evidence results
- `frame_constants.hlsl` fully reconstructed: 27-member UBO, `view_constants[5]`
  (ArrayStride 800), flat cbuffer `frame_constants` set1/b0, bare-global member
  access. Compiled layout re-extracted and byte-offset identical to game
  `_apply_screen_shadows` struct 32/33 (offsets 0..240).
- `ViewConstants` 16 members verbatim. `current_view_index` global (178 bins).
- Samplers: 9 names/bindings verbatim (s0..s12, set0).
- Hash/Affine/RGB8-pack/sRGB/YCbCr/Bicubic/EASU algorithms matched from inlined
  consumer use-sites (spirv-cross + OpLine attribution).

## Compile self-test
PASS (dxc v1.10.2605.37, canonical flags `-spirv -WX -Ges -HV 2021
-fspv-target-env=vulkan1.1 -Zi -Qembed_debug`), split into two entries:
- Entry A (29 files, math/curve chain): PASS, exit 0, no warnings.
- Entry B (pack/bilinear/stochastic chain): PASS, exit 0, no warnings.
- Cross-agent: INC-C `cascades.hlsl` compiles against `frame_constants.hlsl`.
Log: `compile-self-test.log`.

## Top Unknowns
- All inlined helper function names (unrecoverable — marked Inference).
- `consts.hlsl`, `color/{xyz,luv,helmholtz_kohlrausch}.hlsl`, `lanczos2.hlsl`
  bodies (standard implementations, U).
- `tracked_buffer.hlsl` / `debug_utils.hlsl` APIs (near-no-op at codegen).

## Cross-agent dependency
CONFLICT: `history_valid.hlsl` (INC-C) vs `curve.hlsl` (INC-B) both define
`FrexpStructType` (different layouts); shaders including both the bilinear and
math chains fail until reconciled. INC-A keeps evidence-based includes;
self-test isolates the chains. All other deps (smooth_max, unjitter_taa) compile
cleanly.
