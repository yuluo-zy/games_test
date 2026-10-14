// ==RECONSTRUCTED== shaders/buildings/curved_halftimber.inc.hlsl
// 依据:include 图直接边 -> inc/material/solid_material.inc.hlsl;消费方为
// nani_curved_halftimber 栅格化 + nani cull/bucket(wave-2 家族);弯曲半木墙材质。
// 状态:O (include 边,材质角色);I (名字/函数体为推断——内联,无独立 bin)

#ifndef TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL
#define TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL

#include "inc/material/solid_material.inc.hlsl"

// 弯曲半木参数:木框架颜色与填充(灰泥)颜色。
struct CurvedHalftimberParams
{
    float3 timber_color;      // 木框架颜色
    float3 fill_color;        // 填充(灰泥)颜色
    float timber_thickness;   // 木条厚度
    float2 uv_scale;          // uv 缩放
    uint flags;               // 材质标志
};

// 在局部 uv 上求值弯曲半木固体材质。木框架作为框架色与填充色之间的
// 固体反照率混合处理。
SolidMaterialParams evaluate_curved_halftimber(CurvedHalftimberParams p, float2 uv, float3 normal)
{
    float timber = 0.0f;
    // 沿局部 uv 的框架线条(曲率由实例网格施加)。
    float2 line_uv = uv * p.uv_scale;
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.x) - 0.5f)));
    timber = max(timber, 1.0f - smoothstep(0.0f, p.timber_thickness, abs(frac(line_uv.y) - 0.5f)));
    float3 albedo = lerp(p.fill_color, p.timber_color, saturate(timber));
    return init_solid_material(albedo, 0.5f, p.flags);
}

#endif // TINYGLADE_BUILDINGS_CURVED_HALFTIMBER_HLSL
