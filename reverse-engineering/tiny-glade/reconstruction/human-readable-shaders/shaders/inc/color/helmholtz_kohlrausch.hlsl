// ============================================================
// inc/color/helmholtz_kohlrausch.hlsl —— 亥姆霍兹-柯劳斯效应
// 用途:饱和色看起来比其亮度更亮,在 CIELUV 里加一个与彩度成正比的
//   亮度提升。唯一消费方为 final_post 像素着色器。
// 入口:无(纯头文件;依赖 luv.hlsl 与 xyz.hlsl)
// 关键算法:chroma = length(u,v),boost = (0.3 - 0.15*L) * chroma
// ============================================================
// ==RECONSTRUCTED== shaders/inc/color/helmholtz_kohlrausch.hlsl | evidence: consumer final_post.raster.hlsl (only); ICC-path correction `0.42/max(r,g,b)` (chroma-dependent brightness) is HK-like but exact Luv formula not directly observable in sampled decompile | status: U (formula Inference — standard Luv-based Helmholtz-Kohlrausch lightness boost; constants provisional)
//
// Helmholtz-Kohlrausch effect: saturated colors appear brighter than their
// luminance. Operates in CIELUV, adding a chroma-proportional lightness boost.

#ifndef TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL
#define TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL

#include "luv.hlsl"

// 从 Luv 颜色计算提升后的亮度 L'。chroma = sqrt(u^2+v^2);
// 提升随彩度增长,并在高亮度处衰减。
float helmholtz_kohlrausch_lightness(float3 luv)
{
    float L = luv.x;
    float chroma = length(luv.yz);
    return L + (0.3f - 0.15f * L) * chroma;
}

// 感知亮度(Luv 亮度加 HK 提升),范围约 [0,1]。
float helmholtz_kohlrausch_perceived_lightness(float3 linear_rgb)
{
    float3 luv = xyz_to_luv(srgb_xyz_to_xyz(linear_rgb));
    return helmholtz_kohlrausch_lightness(luv);
}

#endif // TINYGLADE_INC_COLOR_HELMHOLTZ_KOHLRAUSCH_HLSL
