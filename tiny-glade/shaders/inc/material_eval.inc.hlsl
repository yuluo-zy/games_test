// ======================================================================
// inc/material_eval.inc.hlsl —— 材质求值结果与 gbuffer 法线解码
// ======================================================================
// 用途:被 _deferred_*/_billboard_*/_clearing_* 用于解码 gbuffer 法线/粗糙度
//       + BRDF 光照;共享由各材质求值函数产生的表面着色输入。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/material_eval.inc.hlsl | evidence: include-graph edge material_eval.inc.hlsl -> brdf.hlsl; consumers _deferred_*/_billboard_*/_clearing_* decode gbuffer normal/roughness + BRDF lighting; raw-per-bin shows no material-specific struct member tables (params are SSA-inlined) | status: O/I (struct/function names Inference, inlined in consumers)

#ifndef TINYGLADE_INC_MATERIAL_EVAL_HLSL
#define TINYGLADE_INC_MATERIAL_EVAL_HLSL

// 由各材质求值函数产生、并被延迟/前向光照代码消费的共享表面着色输入。
struct MaterialEvalData {
    float3 albedo;
    float3 normal;
    float roughness;
    float opacity;
    float3 emissive;
};

// 把 3 字节打包的法线解码为 float4 gbuffer 词:字节为 (n*0.5+0.5) 存于 xyz,
// 最高字节存放单独的值(如粗糙度)。
float3 unpack_gbuffer_normal(uint packed_normal)
{
    float3 n = float3(float(packed_normal & 255u),
                      float((packed_normal >> 8u) & 255u),
                      float((packed_normal >> 16u) & 255u)) * (1.0 / 255.0);
    n = n * 2.0 - 1.0;
    return normalize(n);
}

// 用恒等默认值初始化材质求值结果。
void init_material_eval_data(out MaterialEvalData d)
{
    d.albedo = float3(1.0, 1.0, 1.0);
    d.normal = float3(0.0, 1.0, 0.0);
    d.roughness = 1.0;
    d.opacity = 1.0;
    d.emissive = float3(0.0, 0.0, 0.0);
}

#endif // TINYGLADE_INC_MATERIAL_EVAL_HLSL
