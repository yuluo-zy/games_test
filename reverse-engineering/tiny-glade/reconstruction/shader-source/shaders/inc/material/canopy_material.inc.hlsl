// ==RECONSTRUCTED== shaders/inc/material/canopy_material.inc.hlsl | evidence: include-graph edge material/canopy_material.inc.hlsl -> material_eval.inc.hlsl; consumers _clearing_tree_canopy/_deferred_canopy sample canopy_color/canopy_flowers_color textures | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_CANOPY_MATERIAL_HLSL

#include "../material_eval.inc.hlsl"

// Canopy (tree crown) surface parameters; leaves use a dithered alpha +
// per-texel canopy/flowers colors.
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
