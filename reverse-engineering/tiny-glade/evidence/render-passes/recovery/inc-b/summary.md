# INC-B summary

**Scope.** Reconstructed 13 shared noise/random inc files (blue_noise, hammersley, gaussian_rand,
perlin, simplex_noise, fbm_noise, morton, hsv, sh, smooth_max, curve, find_root, scatter) from
compiled SPIR-V. Helper names are not embedded (dxc inlines them; only globals/types/entries get
OpName), so all function names are Inference and every body is rebuilt from inlined opcode patterns
at consumer use-sites (spirv-cross decompiles of ~25 game bins).

**Result.** All 13 files written under `reconstruction/shader-source/shaders/inc/`. Compile
self-test: combined TU + 13 per-file 1x1-compute entries all PASS under canonical flags
(`dxc -E main -T cs_6_4 -spirv -WX -Ges ... -HV 2021 -Zi`), using the real tree (INC-A's
reconstructed hash.hlsl/math_const.hlsl). Decompile check confirms 22 key algorithm constants
survive in emitted SPIR-V.

**Status by file.** O: (none - no symbol names embedded anywhere). I (solid opcode evidence):
hammersley, morton, hsv, perlin, simplex_noise, fbm_noise, gaussian_rand, blue_noise. U (partial/
inferred signature): sh, smooth_max, curve, find_root, scatter.

**Top unknowns.**
- sh.hlsl: only band-0+1 (4-coeff) observed; band-2 eval (if used by lighting.hlsl) unknown.
- curve.hlsl `curve()` body inferred; frexp verified.
- find_root signature and its `terrain_height` dependency name (assumed from INC-C).
- scatter full generate_grass algorithm (neighbor spacing + slerp) only partially recovered.

**Cross-agent dependencies.**
- INC-A hash.hlsl: used `hash(uint)`/`rand01(uint)` (reconciled); perlin.hlsl defines `hash(uint2)`
  (flag if INC-A later adds it - redefinition).
- INC-C terrain_heightmap.inc.hlsl: `terrain_height()` assumed for find_root.
- **Conflict:** INC-C history_valid.hlsl defines `FrexpStructType` scalar (float/float); curve.hlsl
  defines float3/int3 per decompiled taa/tgsr. Both are included by _taa/_tgsr - must reconcile.
- Toolchain: dxc 1.10.2605.37 native `frexp` broken for SPIR-V; curve.hlsl uses
  `#define frexp incb_frexp` so consumer calls keep resolving to the custom implementation.
