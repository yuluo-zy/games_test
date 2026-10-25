// ======================================================================
// inc/material/eval/grass_material.inc.hlsl —— 草叶材质求值
// ======================================================================
// 用途:被 _deferred_grass / _terrain_lighting_texture 用于带半透明地求值草叶。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/eval/grass_material.inc.hlsl | evidence: raw embedded path style "eval/../grass_material.inc.hlsl" -> includes ../grass_material.inc.hlsl; consumers _deferred_grass/_terrain_lighting_texture evaluate grass blades with translucency; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_GRASS_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_GRASS_MATERIAL_HLSL

#include "../grass_material.inc.hlsl"
#include "../../raymarch.hlsl"

// 求值草叶表面;当草叶后的深度场很近(自遮挡)时降低不透明度,从而获得
// 柔和半透明效果。
MaterialEvalData eval_grass_material(GrassMaterialParams p, float3 world_normal,
                                     Texture2D depth_tex, SamplerState s, float2 texel_size,
                                     float3 ray_start_view, float3 ray_end_view)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    float3 albedo = lerp(p.base_color, p.tip_color, saturate(world_normal.y * 0.5 + 0.5));
    d.albedo = albedo;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = 1.0 - raymarch_opacity(depth_tex, s, texel_size, ray_start_view, ray_end_view, p.translucency);
    d.emissive = float3(0.0, 0.0, 0.0);
    return d;
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_GRASS_MATERIAL_HLSL
