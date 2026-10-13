// ==RECONSTRUCTED== shaders/inc/material/eval/canopy_material.inc.hlsl | evidence: raw embedded path style "eval/../canopy_material.inc.hlsl" -> includes ../canopy_material.inc.hlsl; consumers _deferred_canopy evaluate tree crowns; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL

#include "../canopy_material.inc.hlsl"

// Evaluate a canopy surface; leaves are two-sided and keep their alpha for
// dithered transparency.
MaterialEvalData eval_canopy_material(CanopyMaterialParams p, float3 world_normal)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    d.albedo = p.leaf_color;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = p.alpha;
    d.emissive = p.flower_color * 0.0; // flowers tinted via albedo in canopy shaders
    return d;
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL
