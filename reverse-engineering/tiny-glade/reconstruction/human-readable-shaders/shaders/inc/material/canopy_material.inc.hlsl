// ======================================================================
// inc/material/canopy_material.inc.hlsl —— 树冠表面参数结构
// ======================================================================
// 用途:被 _clearing_tree_canopy / _deferred_canopy 采样 canopy_color/
//       canopy_flowers_color 纹理;树叶使用抖动 alpha + 逐纹素树冠/花朵颜色。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/canopy_material.inc.hlsl | evidence: include-graph edge material/canopy_material.inc.hlsl -> material_eval.inc.hlsl; consumers _clearing_tree_canopy/_deferred_canopy sample canopy_color/canopy_flowers_color textures | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL

#include "../material_eval.inc.hlsl"

// 树冠(树冠层)表面参数;树叶使用抖动 alpha + 逐纹素树冠/花朵颜色。
struct CanopyMaterialParams {
    float3 leaf_color;
    float3 flower_color;
    float roughness;
    float alpha;
};

CanopyMaterialParams init_canopy_material(float3 leaf_color, float3 flower_color, float roughness, float alpha)
{
    CanopyMaterialParams p;
    p.leaf_color = leaf_color;
    p.flower_color = flower_color;
    p.roughness = saturate(roughness);
    p.alpha = saturate(alpha);
    return p;
}

#endif // TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL
