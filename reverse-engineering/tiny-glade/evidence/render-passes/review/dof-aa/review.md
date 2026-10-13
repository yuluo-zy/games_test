# DoF + TAA/TGSR/FSR family — A vs B cross-review

Date 2026-10-13. Reviewer: independent. Both candidates recompiled against the 26 in-scope
ground-truth `.bin` (`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`) with
`tools/render-passes/recompile_check.py`; target modules additionally disassembled
(spirv-cross + word-level) to adjudicate ring-table unpack, loop structure, storage-image
formats, near-branch ramps and accumulator semantics. Machine-readable detail: `review.json`.

Method note: candidate B's sources use bare includes (`"tiles.inc.hlsl"`), which do not resolve
under the canonical single `-I shaders` layout the validator expects; B was therefore validated
through a merged include root (inc/ + dof/*.inc + flat files). A validates with its own
`-I candidates/dof-aa-a/shaders`. All hard-gate numbers below are from fresh recompiles.

## Verdict

**Neither candidate is a clean base. Adopt per file: B's structural scaffolding for the bokeh
gather family (halfres / fullres / lod0), A for the temporal + tile family (taa / tgsr /
tgsr_reproject / analyze / autofocus / prepare / fused), and B for fsr_copy_history.** B has
the closer opcode structure on the ring-gather files (loop layout, f16x2 masking unpack,
OpExtInst/BitwiseAnd/ShiftLeft/SelectionMerge parity) but carries five+ semantic errors and
wrong storage-image formats; A is semantically verified-correct and format-correct but
structurally further (aggressive `[unroll]`, full-vector unpack) on those same files.

Both candidates pass the same 22/26 hard-gate bins (the 4 residuals are the documented
dxc-version/format ones: analyze QuadRead cap, tgsr_blur_lum StorageImageWriteWithoutFormat,
tgsr 8x4 LocalSize). B additionally fails fused capabilities (missing the forced
StorageImageWriteWithoutFormat). The hard gate does not compare the OpTypeImage format
operand: B's gather outputs are declared float4/float3 (Rgba32f/Unknown) where the target is
Rgba16f / R11fG11fB10f, so B's gate PASS understates its divergence.

## Per-file winners

| File | Winner |
|---|---|
| dof/analyze.cs.hlsl | **A** (B's accumulator inits are inverted -> behaviorally broken) |
| dof/autofocus.cs.hlsl | **A** (B's near/far and focus0/focus1 targets are swapped) |
| dof/clear_halfres.cs.hlsl | A (tie; A uses the resolvable `dof/`-qualified include) |
| dof/fullres.cs.hlsl | **merge** (B structure + A decreasing near-branch ramp + A half4 format) |
| dof/fullres_all_lod0.cs.hlsl | **B base** + A half4 format |
| dof/fullres_all_lod1.cs.hlsl | A (format-correct, marginal histogram edge) |
| dof/fullres_upsample.cs.hlsl | tie (B slight) |
| dof/fused.cs.hlsl | **A** (OpExtInst/ImageFetch/Phi parity, near-branch, format + cap gate) |
| dof/halfres.cs.hlsl | **B base** + A half4 format |
| dof/init_buffers.cs.hlsl | tie (B slight) |
| dof/prepare.cs.hlsl | **A** (closer histogram, no member-pointer drift; both need R16f/Rg16f format fix) |
| dof/multires.inc.hlsl | **B** (real f16x2 pack/unpack helpers) |
| dof/tiles.inc.hlsl | tie |
| dof/upsample_halfres.inc.hlsl | B (helper content) |
| taa.cs.hlsl | **A** (target Catmull-Rom `w1 = (1 + f²(-2.5+1.5f)) + w2`; B uses standard w1) |
| tgsr.cs.hlsl | **A** (blend divides by the WIDE weight; B divides by the narrow one) |
| tgsr_reproject.cs.hlsl | **A** (bicubic form + leaner include closure) |
| tgsr_blur_lum.cs.hlsl | tie (identical residual) |
| tgsr_common.inc.hlsl | **A** (keep empty; B's YCoCg duplicates inc/color/ycbcr.hlsl) |
| fsr_copy_history.cs.hlsl | **B** (texel-fetch indexing matches OpImageFetch+OpVectorShuffle) |
| fsr_init_reactive.cs.hlsl | tie (identical residual) |

## Key algorithm findings

1. **Bokeh ring gather / CoC diffusion.** Both sides implement the same 2-ring x 17-tap
   (fullres/halfres), 21-tap x 6-bin (fused) and 9-tap x 4-bin (lod0/lod1) algorithm with a
   packed `groupshared ring_samples[64][9|11|5]` table. The target's unpack is the masking form
   `float2(spvUnpackHalf2x16(v & 0xFFFF).x, spvUnpackHalf2x16((v >> 16) & 0xFFFF).x)` — B's
   consumers reproduce this (OpBitwiseAnd 18=18, OpExtInst 154~153); A's `UnpackRingPair(v)`
   full-vector form overshoots OpExtInst (170). B's loop structure is also the target's
   (OpLoopMerge 6/7 exact vs A 4/5; A `[unroll]`s the ring taps, mask-zeroing and inner mask
   loops). The reported OpLoad residual (~+45 from the packed ring-table unpack) is present on
   BOTH sides (A +54, B +45); B is the smaller of the two but neither converges it. Root cause:
   the gather reads `dof_ring_samples[lane][slot]` more times than the original (per-half vs
   cached pair loads).
2. **Near-branch ramp is PER-PASS, not uniform.** The target halfres uses the INCREASING ramp
   (`abs < 8 continue; weight * smoothstep(8,11,ac)`), while the target fullres uses the
   DECREASING ramp (`abs >= 11 continue; weight * smoothstep(11,8,ac)`), and fused/lod0/lod1 use
   NO guard at all (`add_weight = weight_acc; add_color_weight = sample_weight`). A got all four
   right; B copied the halfres-style increasing ramp into fullres and ADDED the 8/11 guard to
   fused — semantically wrong in both places.
3. **TGSR/taa shared math.** The Catmull-Rom 5-tap cross in taa and tgsr_reproject uses
   `w1 = (1 + f²(-2.5+1.5f)) + w2` and `center_uv = (base_c + w2/w1)`; A matches, B does not.
   tgsr's blend lerp divides the current-frame wide sum by the WIDE weight (wc_sum) and the
   history-sample dummy middle component is the constant 0.5; A matches, B divides by the narrow
   weight and uses 0.0.
4. **Tile bookkeeping.** analyze's CoC-dilation accumulators: target = max over sample.y (init 0,
   written to coc_tiles_dilated_tex, near <= 7) and min over sample.x (init 1e10, far > 11).
   A matches; B's inits are inverted, so B's outputs are constant (0 / 1e10) and behaviorally
   broken. autofocus near/far under tilt-shift: focus0 target = min/median0, focus1 target =
   max/median1, locked focus1 = lock_dist + lerped(focus0); A matches, B is swapped.
5. **6f1d vs 72a2 noise scheme.** Both candidates reconstructed the 6f1d variant
   (hash-jitter ring phase + `5.588*float(frame%64)` spoke seed + pi/2 spoke angle). The 72a2
   bins use blue-noise.z phase + pi/4 angle and drop the `frame%64` (OpUMod 1/0, OpConvertUToF
   11/6) — a compile-time noise-scheme variant neither side implements. Record as a residual.

## Inc-layer fixes both sides surfaced

1. **`inc/coc.hlsl` must not export `ring_samples`.** The bin embeds `ring_samples` once per
   gather consumer as the consumer's own `groupshared`; the original coc.hlsl evidently did not
   define it (both candidates had to rename their consumer groupshared to `dof_ring_samples` to
   compile against the shared inc). Remove `ring_samples` (and the unused `gaussian_weights`)
   from `inc/coc.hlsl`; consumers revert to the bin-embedded name `ring_samples`.
2. **`inc/coc.hlsl` DofFocusReadback member is `lock_reponse`.** The autofocus module embeds the
   readback struct member as the typo `lock_reponse`; the $Globals member `focus_lock_request`
   is a separate, correct name. Rename the readback member in coc.hlsl to `lock_reponse`
   (`readback.lock_reponse = focus_lock_request`).
3. **`dof/prepare.cs.hlsl` storage formats.** Both candidates declare coc_out_tex as
   `RWTexture2D<float>` and tile_output_tex as `RWTexture2D<float2>`; the target is R16f / Rg16f
   (`RWTexture2D<half>` / `RWTexture2D<half2>`). Shared fix.
4. **Include form.** Normalize adopted dof files to `#include "dof/...inc.hlsl"` (A's form); B's
   bare `"tiles.inc.hlsl"` / `"multires.inc.hlsl"` / `"upsample_halfres.inc.hlsl"` break the
   canonical single-`-I` layout and the validator.
5. **Storage-image format convention.** The gather outputs must be declared `RWTexture2D<half4>`
   (Rgba16f) for halfres/fullres/lod0/lod1 and `RWTexture2D<float3>` with
   `[[vk::ext_capability(49)]]` for fused (R11fG11fB10f), matching A. B's float4/float3
   declarations diverge from the target image formats even though they pass the bindings gate.
