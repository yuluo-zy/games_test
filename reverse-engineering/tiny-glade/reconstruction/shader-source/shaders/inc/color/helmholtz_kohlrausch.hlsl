// ==RECONSTRUCTED== shaders/inc/color/helmholtz_kohlrausch.hlsl | evidence: consumer final_post.raster.hlsl (only); ICC-path correction `0.42/max(r,g,b)` (chroma-dependent brightness) is HK-like but exact Luv formula not directly observable in sampled decompile | status: U (formula Inference — standard Luv-based Helmholtz-Kohlrausch lightness boost; constants provisional)
//
// Helmholtz-Kohlrausch effect: saturated colors appear brighter than their
// luminance. Operates in CIELUV, adding a chroma-proportional lightness boost.

#ifndef TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL
#define TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL

#include "luv.hlsl"

// Boosted lightness L' from Luv color. chroma = sqrt(u^2+v^2); the boost grows
// with chroma and falls off at high lightness.
float helmholtz_kohlrausch_lightness(float3 luv)
{
    float L = luv.x;
    float chroma = length(luv.yz);
    return L + (0.3f - 0.15f * L) * chroma;
}

// Perceptual lightness (Luv lightness with HK boost), range [0,1]-ish.
float helmholtz_kohlrausch_perceived_lightness(float3 linear_rgb)
{
    float3 luv = xyz_to_luv(srgb_xyz_to_xyz(linear_rgb));
    return helmholtz_kohlrausch_lightness(luv);
}

#endif // TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL
