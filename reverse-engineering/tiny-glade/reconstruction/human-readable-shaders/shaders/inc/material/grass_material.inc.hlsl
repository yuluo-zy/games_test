// ======================================================================
// inc/material/grass_material.inc.hlsl —— 草叶表面参数结构
// ======================================================================
// 用途:被 _grass/_grass_lod1/_deferred_grass/_terrain_lighting_texture 消费;
//       半透明通过对草叶后的深度纹理步进求值。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/grass_material.inc.hlsl | evidence: include-graph edges material/grass_material.inc.hlsl -> material_eval.inc.hlsl and -> raymarch.hlsl (raw embedded style "eval/../grass_material.inc.hlsl"); consumers _grass/_grass_lod1/_deferred_grass/_terrain_lighting_texture | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_GRASS_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_GRASS_MATERIAL_HLSL

#include "../material_eval.inc.hlsl"

// 逐草叶表面参数;半透明通过对草叶后深度纹理步进求值。
struct GrassMaterialParams {
    float3 base_color;
    float3 tip_color;
    float roughness;
    float translucency;
};

GrassMaterialParams init_grass_material(float3 base_color, float3 tip_color, float roughness)
{
    GrassMaterialParams p;
    p.base_color = base_color;
    p.tip_color = tip_color;
    p.roughness = saturate(roughness);
    p.translucency = 1.0;
    return p;
}

#endif // TINYGLADE_INC_MATERIAL_GRASS_MATERIAL_HLSL
