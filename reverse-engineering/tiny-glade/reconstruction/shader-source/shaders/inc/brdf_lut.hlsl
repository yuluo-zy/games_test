// ==RECONSTRUCTED== shaders/inc/brdf_lut.hlsl | evidence: spirv-cross (_deferred_*_light_sun_sky PS) brdf_fg_lut.SampleLevel((float2(NdotV, roughness^2)*0.984375+0.0078125),0).xy -> (F,G); include-graph edge brdf_lut.hlsl -> highlight_color.hlsl | status: O (function name Inference: inlined)

#ifndef TINYGLADE_INC_BRDF_LUT_HLSL
#define TINYGLADE_INC_BRDF_LUT_HLSL

// Pre-integrated GGX BRDF LUT: x = Fresnel-weighted F, y = G. The 0.984375/
// 0.0078125 remap keeps samples inside the (256^2) LUT border.
float2 sample_brdf_fg_lut(Texture2D brdf_fg_lut_tex, SamplerState s, float NdotV, float roughness)
{
    float2 uv = float2(saturate(NdotV), roughness * roughness) * 0.984375 + 0.0078125;
    return brdf_fg_lut_tex.SampleLevel(s, uv, 0.0).xy;
}

#endif // TINYGLADE_INC_BRDF_LUT_HLSL
