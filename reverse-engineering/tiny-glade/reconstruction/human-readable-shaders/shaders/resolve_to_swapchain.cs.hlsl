// ==RECONSTRUCTED== shaders/resolve_to_swapchain.cs.hlsl | F-E | bins:
//   _resolve_to_swapchain.cs.hlsl_cs_main_0.bin | 入口 cs:cs_main | 工作组尺寸 [8,8,1]
//   用途: 交换链解析(MSAA 路径):像素坐标翻转到目标尺寸,对多个 MSAA 样本求均值,
//   线性 -> sRGB 编码后写入 u15。
//   绑定: $Globals b13 (transform_image/output_tex_dims), input_tex t14
//   (Texture2DMS), output_tex u15, 工作组尺寸 [8,8,1], 入口 cs_main。

#include "inc/common.hlsl"            // 采样器(此处未使用)
#include "resolve_to_swapchain.inc.hlsl"

// 是否翻转图像、输出尺寸。
cbuffer $Globals : register(b13, space0)
{
    uint transform_image;
    uint2 output_tex_dims;
};

// MSAA 输入(最多 4 样本)。
Texture2DMS<float4> input_tex : register(t14, space0);
// sRGB 输出。
RWTexture2D<unorm float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    int2 pixel = int3(dispatch_id).xy;
    // 把像素坐标翻转到有效输出区域(依 transform_image 决定翻转方式)。
    int2 coord = resolve_to_swapchain_coord(pixel, output_tex_dims, transform_image);
    uint2 coord_u = uint2(coord);

    // 查询 MSAA 样本数。
    uint width, height, sample_count;
    input_tex.GetDimensions(width, height, sample_count);

    // 逐个样本累加(样本数小于 4 时其余分支不执行)。
    float3 acc = 0.0f;
    if (0u < sample_count) acc = input_tex.Load(int2(coord_u), 0).xyz;
    if (1u < sample_count) acc += input_tex.Load(int2(coord_u), 1).xyz;
    if (2u < sample_count) acc += input_tex.Load(int2(coord_u), 2).xyz;
    if (3u < sample_count) acc += input_tex.Load(int2(coord_u), 3).xyz;

    // 平均。
    float3 c = acc / float(sample_count);

    // 线性 -> sRGB 逐通道(阈值 0.0031308 与发布代码一致)。
    float r;
    float g;
    float b;
    if (0.0031308f >= c.r) r = c.r * 12.92f;
    else r = 1.055f * pow(c.r, 0.416666657f) - 0.055f;
    if (0.0031308f >= c.g) g = c.g * 12.92f;
    else g = 1.055f * pow(c.g, 0.416666657f) - 0.055f;
    if (0.0031308f >= c.b) b = c.b * 12.92f;
    else b = 1.055f * pow(c.b, 0.416666657f) - 0.055f;

    // 写回交换链(alpha 恒为 1)。
    output_tex[uint2(pixel)] = float4(r, g, b, 1.0f);
}
