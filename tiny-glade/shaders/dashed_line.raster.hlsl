// ==RECONSTRUCTED== shaders/dashed_line.raster.hlsl | F-E | bins: _dashed_line.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Dashed line: per-vertex model_to_world transform; fragment discards every
// other dash cell (frac of the line-length parameter).
//
// === 中文说明 ===
// 虚线绘制:顶点着色器做 model_to_world 变换;片元着色器按线长参数的 frac
//   丢弃奇数段,实现虚线段效果。
// 入口: vs_main (顶点) / ps_main (片元)。该文件在 manifest 中带两个变体名
//   (b903e43ffb3da915 / f0adff76827e73ff)但代码本体无 TGV_ 守卫,两变体共用。
// 绑定: $Globals b13 (model_to_world / vertex_count / line_length), frame_constants。

#include "inc/common.hlsl"            // samplers (unused here)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    int vertex_count;
    float line_length;
};

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
};

struct VsOut
{
    float3 color : TEXCOORD0;
    float line_pos : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin, uint vertex_id : SV_VertexID)
{
    VsOut o;
    // 模型空间 -> 世界空间。
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    // 世界空间 -> 当前视图裁剪空间。
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.color = vin.Vertex_Color;
    // 线长参数:按顶点序号归一化到 [0, line_length],供片元做虚线切分。
    o.line_pos = (float(vertex_id) / float(vertex_count - 1)) * line_length;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    // 虚线:线长参数的 frac 大于 0.5 的段丢弃。
    if (frac(vin.line_pos) > 0.5f)
        discard;
    return float4(vin.color, 1.0f);
}
