// ======================================================================
// inc/material/eval/canopy_material.inc.hlsl —— 树冠材质求值
// ======================================================================
// 用途:被 _deferred_canopy 求值树冠。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/eval/canopy_material.inc.hlsl | evidence: raw embedded path style "eval/../canopy_material.inc.hlsl" -> includes ../canopy_material.inc.hlsl; consumers _deferred_canopy evaluate tree crowns; eval function inlined | status: O/I (function name/signature Inference)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL

#include "../canopy_material.inc.hlsl"

// 求值树冠表面;树叶为双面,并保留 alpha 用于抖动透明。
MaterialEvalData eval_canopy_material(CanopyMaterialParams p, float3 world_normal)
{
    MaterialEvalData d;
    init_material_eval_data(d);
    d.albedo = p.leaf_color;
    d.normal = normalize(world_normal);
    d.roughness = p.roughness;
    d.opacity = p.alpha;
    d.emissive = p.flower_color * 0.0; // 花朵在树冠着色器中经 albedo 上色
    return d;
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_CANOPY_MATERIAL_HLSL
