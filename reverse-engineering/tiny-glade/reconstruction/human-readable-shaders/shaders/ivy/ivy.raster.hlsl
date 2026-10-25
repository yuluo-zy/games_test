// ==RECONSTRUCTED==
// 中文说明:
//   常春藤渲染:一段由四边形条带组成的常春藤(每个 IvySegment 两个顶点)。
//   每个顶点吸附到地形高度图,并把"透传厚度"(段厚度的小数部分)写入
//   TEXCOORD0,让片元把常春藤按厚度缩放的纯灰着色。
//   源路径 shaders/ivy/ivy.raster.hlsl,2 bin:
//   _ivy_ivy.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin
//   入口 vs:vs_main; ps:ps_main。

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"

// 复现原版共享 13 采样器 $Globals 布局(set0/b13)。

// 每着色器 $Globals 成员(隐式 $Globals UBO,set0/b13)。
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;

// 常春藤段:起点/厚度 + 终点/厚度。
struct IvySegment
{
    float3 start_pos;
    float start_thickness;
    float3 end_pos;
    float end_thickness;
};

StructuredBuffer<IvySegment> ivy_buffer : register(t14, space0);

struct VSOutput
{
    float4 position : SV_Position;
    float thickness : TEXCOORD0;
};

VSOutput vs_main(uint vid : SV_VertexID)
{
    VSOutput output;
    // 每段两个顶点:偶数为起点,奇数为终点。
    uint segment = vid / 2u;
    float3 position;
    if ((vid % 2u) == 0u)
    {
        position = ivy_buffer[segment].start_pos;
        output.thickness = frac(ivy_buffer[segment].start_thickness);
    }
    else
    {
        position = ivy_buffer[segment].end_pos;
        output.thickness = frac(ivy_buffer[segment].end_thickness);
    }
    // 由世界 xz 经 1/130 映射采样地形高度图,得到地面高度。
    float2 uv = position.xz * (0.0076923076994717121124267578125f).xx + 0.5f.xx;
    float height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x);
    // 顶点吸附到地面。
    ViewConstants view = GetCurrentViewConstants();
    output.position = mul(view.world_to_sample,
                          float4(position.x, position.y + height, position.z, 1.0f));
    return output;
}

struct PSInput
{
    float thickness : TEXCOORD0;
};

// 按厚度输出灰阶(厚度作为 RGB 灰值)。
float4 ps_main(PSInput input) : SV_Target0
{
    return float4(input.thickness, input.thickness, input.thickness, 1.0f);
}
