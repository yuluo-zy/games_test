// ======================================================================
// inc/lighting.hlsl —— 太阳直射光与天空环境光公共头文件
// ======================================================================
// 用途:被 _deferred_*_light_sun_sky / _billboard_* / _clearing_* 使用。
// 模型:太阳漫反射+高光 = BRDF(NdotL,NdotV,NdotH,HdotL) * sun_color * NdotL *
//       shadow;天空环境光由 sky_low/high 渐变 + indirect_light_tint 给出。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/lighting.hlsl | evidence: spirv-cross (_deferred_*_light_sun_sky/_billboard_*/_clearing_*) sun diffuse+specular = BRDF(NdotL,NdotV,NdotH,HdotL) * sun_color * NdotL * shadow, sky ambient from sky_low/high + indirect_light_tint; include-graph edge lighting.hlsl -> sun.inc.hlsl (brdf.hlsl is pulled in via material_eval; included here guarded for standalone use) | status: O (model); O/I (function names Inference)

#ifndef TINYGLADE_INC_LIGHTING_HLSL
#define TINYGLADE_INC_LIGHTING_HLSL

#include "sun.inc.hlsl"
#include "brdf.hlsl"

// 太阳直射光:Cook-Torrance 高光 + GGX 漫反射,再乘太阳颜色、NdotL 与阴影因子。
float3 evaluate_sun_light(float3 albedo, float roughness, float3 normal, float3 view_dir,
                          float3 sun_dir, float shadow)
{
    float NdotL = saturate(dot(normal, sun_dir));
    float NdotV = saturate(dot(normal, view_dir));
    float3 H = normalize(view_dir + sun_dir);
    float NdotH = saturate(dot(normal, H));
    float HdotL = saturate(dot(H, sun_dir));
    float3 diffuse = albedo * diffuse_ggx(NdotL, NdotV, NdotH, roughness);
    float3 specular = cook_torrance_specular(NdotL, NdotV, NdotH, HdotL, roughness);
    return get_sun_color() * (diffuse + specular) * NdotL * shadow;
}

// 简单天空环境光:按法线朝上分量在 sky_low 与 sky_high 之间渐变,
// 再叠加主题的间接光染色。
float3 evaluate_sky_ambient(float3 normal)
{
    float up = saturate(normal.y) * 0.5 + 0.5;
    float3 sky = lerp(get_sky_low_color(), get_sky_high_color(), up);
    return sky * color_library.indirect_light_tint.xyz;
}

// 总前向光照:太阳 + 天空环境光。
float3 evaluate_lighting(float3 albedo, float roughness, float3 normal, float3 view_dir,
                        float3 world_pos, float shadow, float ao)
{
    float3 sun = evaluate_sun_light(albedo, roughness, normal, view_dir, get_sun_direction(), shadow);
    float3 ambient = evaluate_sky_ambient(normal) * albedo * ao;
    return sun + ambient;
}

#endif // TINYGLADE_INC_LIGHTING_HLSL
