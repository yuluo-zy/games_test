// ==RECONSTRUCTED== shaders/vertex_color.raster.hlsl | B simple-solid | bins: _vertex_color.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 简单位移-顶点色光栅化器:顶点先经过每实例的 model_to_world,再乘活动相机的
// world_to_sample 变换;片元把顶点色原样透传。
//
// $Globals (set0/b13): current_view_index + model_to_world。
//
// 中文说明:最基础的"实心网格 + 顶点色"着色器,无光照、无纹理采样。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

// 每实例模型到世界变换(由 $Globals 提供)。
float4x4 model_to_world;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 取活动相机的世界到采样空间矩阵,逐实例变换顶点位置。
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, mul(model_to_world, float4(vin.Vertex_Position, 1.0f)));
    // 顶点色直接透传给片元。
    o.C0 = vin.Vertex_Color;
    return o;
}

float4 ps_main(VsOut pin) : SV_Target0
{
    // 输出顶点色(alpha 恒为 1)。
    return float4(pin.C0, 1.0f);
}
