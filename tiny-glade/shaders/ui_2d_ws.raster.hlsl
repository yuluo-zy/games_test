// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/ui_2d_ws.raster.hlsl | F-E | bins: _ui_2d_ws.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 世界空间 UI 四边形:位置由 ui_position(世界)+ ui_size 决定,沿相机 right/up 轴朝向;
// ps 以 -1 mip 偏移采样,alpha 预乘输出。
//
// 中文说明:世界空间中的 UI 元素(如头顶名称标签)。vs 把四边形排布在相机
// 右/上轴组成的广告牌平面内;ps 预乘 alpha。

#include "inc/common.hlsl"            // 采样器 (sampler_llc s3)
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float3 ui_position;              // 世界空间中心位置
    float2 ui_size;                  // 世界空间尺寸
    float2 window_size;
    uint2 ui_texture_dims;
};

Texture2D<float4> ui_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    // 3x3 三角形扇展开四边形角点:位置偏移 + UV。
    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    // 取活动相机的 right/up 轴,构造面向相机的广告牌平面。
    ViewConstants v = view_constants[current_view_index];
    float3 right = normalize(mul(v.view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz);
    float3 up = -normalize(mul(v.view_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz);

    // 中心位置 + 角点偏移,变换到采样空间。
    float3 world = ui_position + right * (corner.x * ui_size.x) + up * (corner.y * ui_size.y);
    o.position = mul(v.world_to_sample, float4(world, 1.0f));
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    // 采样贴图并预乘 alpha。
    float4 c = ui_texture.SampleBias(sampler_llc, uv, -1.0f);
    return float4(c.rgb * c.a, c.a);
}

#endif

// f0adff76827e73ff 变体的像素阶段:原版 bin 为空壳(无指令),按行为逐字提升。
#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _ui_2d_ws.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
