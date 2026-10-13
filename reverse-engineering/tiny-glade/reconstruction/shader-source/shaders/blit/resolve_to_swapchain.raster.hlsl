// ==RECONSTRUCTED== shaders/blit/resolve_to_swapchain.raster.hlsl | F-E | bins: _blit_resolve_to_swapchain.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Non-MSAA swapchain resolve: flip pixel (transform_image), Load the texel,
// encode linear -> sRGB. vs is the shared fullscreen triangle.

#include "inc/common.hlsl"            // samplers (unused here)
#include "resolve_to_swapchain.inc.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint transform_image;
    uint2 framebuffer_dims;
};

Texture2D<float4> input_tex : register(t14, space0);

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    int2 coord = resolve_to_swapchain_coord(int2(position.xy), framebuffer_dims, transform_image);
    float3 c = input_tex.Load(int3(uint2(coord), 0)).rgb;

    // linear -> sRGB per channel (strict threshold matches shipped codegen).
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
