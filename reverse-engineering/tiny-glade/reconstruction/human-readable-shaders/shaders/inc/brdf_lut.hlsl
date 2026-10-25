// ============================================================
// inc/brdf_lut.hlsl —— 预积分 BRDF 查找表采样
// 用途:从 brdf_fg 2D LUT 采样菲涅尔加权 F 与可见性 G。
//   消费方:_deferred_*_light_sun_sky 像素着色器。
// 入口:无(纯头文件)
// 关键绑定:brdf_fg_lut 由消费方声明
// 关键算法:UV 用 0.984375/0.0078125 重映射,保持在 LUT 边界内
// ============================================================
// ==RECONSTRUCTED== shaders/inc/brdf_lut.hlsl | evidence: spirv-cross (_deferred_*_light_sun_sky PS) brdf_fg_lut.SampleLevel((float2(NdotV, roughness^2)*0.984375+0.0078125),0).xy -> (F,G); include-graph edge brdf_lut.hlsl -> highlight_color.hlsl | status: O (function name Inference: inlined)

#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL

// 采样预积分 GGX BRDF LUT:x = 菲涅尔加权 F,y = G。
// 0.984375/0.0078125 重映射让采样点落在 (256^2) LUT 边界之内。
float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif // TINYGLADE_INC_BRDF_LUT_HLSL
