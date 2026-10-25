// ==RECONSTRUCTED== shaders/normal_debug.raster.hlsl | F-D
// 证据: _normal_debug.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main)。
//   绑定: $Globals b13 (current_view_index), debug_normals_buffer b14,
//   frame_constants set1/b0。NormalDebug = { position, normal } (32 字节记录)。
//   用途: 法线可视化调试。vs 按实例绘制一条"法线调试"线段(顶点 1 在记录位置上
//   叠加记录法线),用当前视图的 world_to_sample 变换;ps 输出常量红色。
// 状态: O(结构来自反汇编)。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

// 当前要显示的视图索引。
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
};

// 法线调试记录:位置 + 法线(每个 16 字节,共 32 字节)。
struct NormalDebug
{
    float4 position;
    float4 normal;
};

// CPU 填充的法线调试记录缓冲。
StructuredBuffer<NormalDebug> debug_normals_buffer : register(t14, space0);

struct VSOut
{
    float4 position : SV_Position;
};

VSOut vs_main(uint vertexIndex : SV_VertexID, uint instanceIndex : SV_InstanceID)
{
    VSOut output;
    // 取实例的位置起点。
    float3 p = debug_normals_buffer[instanceIndex].position.xyz;
    // 第二个顶点(线段终点)把位置沿法线方向拉出,形成线段。
    if (vertexIndex == 1u)
    {
        p += debug_normals_buffer[instanceIndex].normal.xyz;
    }
    // 变换到当前视图的采样空间(裁剪空间)。
    output.position = mul(view_constants[current_view_index].world_to_sample, float4(p, 1.0f));
    return output;
}

// 法线线段统一渲染为纯红色。
float4 ps_main() : SV_Target0
{
    return float4(1.0f, 0.0f, 0.0f, 1.0f);
}
