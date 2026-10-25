// Current presentation shader. A completed main image can retain its old extent
// while acquisition observes a resize. Map target pixel centers to source pixels
// before resolving/encoding; equal-size presentation retains exact texel loads.
// This resize behavior intentionally extends the original equal-extent contract.
#include "inc/common.hlsl"            // 采样器 (此处未使用)
#include "resolve_to_swapchain.inc.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint transform_image;              // 是否翻转/变换图像
    uint2 framebuffer_dims;            // 帧缓冲尺寸
};

Texture2D<float4> input_tex : register(t14, space0);

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    uint width, height;
    input_tex.GetDimensions(width, height);
    int2 pixel = resolve_to_swapchain_coord(int2(position.xy), framebuffer_dims, transform_image);
    int2 coord = int2((float2(pixel) + 0.5f) * float2(width, height) / float2(framebuffer_dims));
    float3 c = input_tex.Load(int3(uint2(coord), 0)).rgb;

    // 逐通道线性 -> sRGB(严格阈值匹配出货代码生成)。
    float r;
    float g;
    float b;
    if (0.0031308f >= c.r) r = c.r * 12.92f;
    else r = 1.055f * pow(c.r, 0.416666657f) - 0.055f;
    if (0.0031308f >= c.g) g = c.g * 12.92f;
    else g = 1.055f * pow(c.g, 0.416666657f) - 0.055f;
    if (0.0031308f >= c.b) b = c.b * 12.92f;
    else b = 1.055f * pow(c.b, 0.416666657f) - 0.055f;

    return float4(r, g, b, 1.0f);
}

