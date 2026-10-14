// ======================================================================
// inc/material/solid_material.inc.hlsl —— 实心表面参数结构
// ======================================================================
// 用途:被 _nani_*_bricks/_plaster/_roof_floor/_wall_*_brick 从实例数据 +
//       brdf LUT 读取实心 albedo/roughness。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material/solid_material.inc.hlsl | evidence: include-graph edge material/solid_material.inc.hlsl -> brdf_lut.hlsl (raw embedded style "eval/../solid_material.inc.hlsl"); consumers _nani_*_bricks/_plaster/_roof_floor/_wall_*_brick read solid albedo/roughness from instance data + brdf LUT | status: O/I (params struct fields Inference; members SSA-inlined)

#ifndef TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
#define TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL

// 延迟/前向光照使用的逐实例实心表面参数。
struct SolidMaterialParams {
    float3 albedo;
    float roughness;
    uint flags; // 低位:渲染标志(高亮、阴影、AO 等)
};

// 从 albedo/roughness 对构造实心参数(实例或纹理颜色)。
SolidMaterialParams init_solid_material(float3 albedo, float roughness, uint flags)
{
    SolidMaterialParams p;
    p.albedo = albedo;
    p.roughness = saturate(roughness);
    p.flags = flags;
    return p;
}

#endif // TINYGLADE_INC_MATERIAL_SOLID_MATERIAL_HLSL
