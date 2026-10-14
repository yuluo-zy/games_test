// ======================================================================
// inc/material/eval/plant_material.inc.hlsl —— 植物/叶片材质求值
// ======================================================================
// 用途:被 _deferred_plant 用于带半透明地求值叶子/花朵。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/eval/plant_material.inc.hlsl | evidence: raw embedded path style "eval/../plant_material.inc.hlsl" -> includes ../plant_material.inc.hlsl; consumers _deferred_plant evaluate foliage/flowers with translucency; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_PLANT_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_PLANT_MATERIAL_HLSL

#include "../plant_material.inc.hlsl"
#include "../../raymarch.hlsl"

// 求值植物/叶片表面。花朵把 albedo 向花朵颜色靠拢;叶子保持类似草的半透明着色。
MaterialEvalData eval_plant_material(PlantMaterialParams p, float3 world_normal,
                                     Texture2D depth_tex, SamplerState s, float2 texel_size,
                                     float3 ray_start_view, float3 ray_end_view)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    float3 albedo = p.base_color;
    if (p.is_flower != 0u) {
        albedo = lerp(albedo, p.flower_color, 0.5);
    }
    d.albedo = albedo;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = 1.0 - raymarch_opacity(depth_tex, s, texel_size, ray_start_view, ray_end_view, p.translucency);
    d.emissive = float3(0.0, 0.0, 0.0);
    return d;
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_PLANT_MATERIAL_HLSL
