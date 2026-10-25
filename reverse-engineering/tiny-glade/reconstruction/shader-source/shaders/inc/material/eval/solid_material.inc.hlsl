// ==RECONSTRUCTED== shaders/inc/material/eval/solid_material.inc.hlsl | evidence: raw embedded path style "eval/../solid_material.inc.hlsl" -> includes ../solid_material.inc.hlsl; consumers _deferred_*/_gi_*/_primary_rt_debug evaluate solid surfaces; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_SOLID_MATERIAL_HLSL

#include "../solid_material.inc.hlsl"
#include "../../material_eval.inc.hlsl"

// Evaluate a solid surface into shared shading inputs for the deferred pass.
MaterialEvalData eval_solid_material(SolidMaterialParams p, float3 world_normal)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    d.albedo = p.albedo;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
    return d;
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_SOLID_MATERIAL_HLSL
