// ======================================================================
// inc/ray_cone.hlsl —— 射线锥体 LOD 辅助
// ======================================================================
// 用途:水面 / GI 反射锥体与重投影消费者(307 个 bin 嵌入)使用;最小自洽
//       射线锥体 LOD 工具(精确函数名不可恢复,SPIR-V 中已内联)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/ray_cone.hlsl | status: Inference (minimal
// behavior-consistent body, filled in P1c integration pass, adjudicated inc-fix #13).
// Used by water / GI reflection-cone and reprojection consumers (307 bins embed it);
// the exact helper names are not recoverable (inlined away in SPIR-V), so this is a
// self-consistent minimal ray-cone LOD utility. Wave-1 consumers only embed the file
// for OpSource parity; they do not call these helpers.

#ifndef TINYGLADE_INC_RAY_CONE_HLSL
#define TINYGLADE_INC_RAY_CONE_HLSL

// 锥角随距离增长的射线,用于按锥体在距离 t 处的覆盖范围选择反射/辐照度 mip。
struct RayCone
{
    float3 origin;
    float3 direction;
    float spread; // tan(半锥角)
};

// 锥体在距离 t 处的覆盖半宽。
float ray_cone_half_width(RayCone cone, float t)
{
    return t * cone.spread;
}

// 覆盖 `footprint_texels` 个纹素的锥体对应的 mip 级别:最长轴取 log2。
// `footprint_world` 与 `world_per_texel` 为 float2 范围。
float ray_cone_mip(float2 footprint_world, float2 world_per_texel)
{
    float2 footprint_texels = max(footprint_world / max(world_per_texel, 1e-6f.xx), 1.0f.xx);
    return log2(max(footprint_texels.x, footprint_texels.y));
}

#endif // TINYGLADE_INC_RAY_CONE_HLSL
