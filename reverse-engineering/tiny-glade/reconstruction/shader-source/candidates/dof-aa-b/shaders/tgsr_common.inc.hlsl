// ==SKELETON==
// SOURCE PATH : shaders/tgsr_common.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 6 bins):
//   _tgsr.cs.hlsl_cs_main_{3ee6a953408e4c4c,7a0af18450cdbcd1,b303f36f0dbdd5f6,f7efabb81dfe256b}.bin,
//   _tgsr_blur_lum.cs.hlsl_cs_main_0.bin, _tgsr_reproject.cs.hlsl_cs_main_0.bin
//
// ==RECONSTRUCTED== shaders/tgsr_common.inc.hlsl
// Evidence: tgsr family bins inline a Rec.709 -> YCoCg transform
// (matrix {0.2126,0.7152,0.0722 / -0.1146,-0.3854,0.5 / 0.5,-0.4542,-0.0458})
// plus the inverse; tgsr_reproject and tgsr_blur_lum use the forward matrix on
// history/color before writing lum+chrom.  Function/matrix names Inference
// (inlined); the matrices below reproduce the observed constants.  Status: I.

#ifndef TINYGLADE_TGSR_COMMON_INC_HLSL
#define TINYGLADE_TGSR_COMMON_INC_HLSL

// Rec.709 <-> YCoCg matrices (inlined at use sites, matching the compiled
// constant set).
#define YCOCG_FWD float3x3(float3(0.2126f, 0.7152f, 0.0722f), float3(-0.1146f, -0.3854f, 0.5f), float3(0.5f, -0.4542f, -0.0458f))
#define YCOCG_INV float3x3(float3(1.0f, 0.0f, 1.5748f), float3(1.0f, -0.1873f, -0.4681f), float3(1.0f, 1.8556f, 0.0f))

// Rec.709 -> YCoCg. Forward: ycocg = mul(FWD, rgb); inverse: rgb = mul(INV, ycocg).
float3 tgsr_rgb_to_ycocg(float3 rgb)
{
    return mul(YCOCG_FWD, rgb);
}

float3 tgsr_ycocg_to_rgb(float3 ycocg)
{
    return mul(YCOCG_INV, ycocg);
}

#endif // TINYGLADE_TGSR_COMMON_INC_HLSL
