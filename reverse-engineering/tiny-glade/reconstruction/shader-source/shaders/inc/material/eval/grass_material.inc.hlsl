// ==RECONSTRUCTED== shaders/inc/material/eval/grass_material.inc.hlsl | evidence: raw embedded path style "eval/../grass_material.inc.hlsl" -> includes ../grass_material.inc.hlsl; consumers _deferred_grass/_terrain_lighting_texture evaluate grass blades with translucency; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_GRASS_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_GRASS_MATERIAL_HLSL

#include "../grass_material.inc.hlsl"
#include "../../raymarch.hlsl"

// Evaluate a grass blade surface; opacity is reduced when the depth field
// behind the blade is close (self-occlusion), enabling soft translucency.
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
