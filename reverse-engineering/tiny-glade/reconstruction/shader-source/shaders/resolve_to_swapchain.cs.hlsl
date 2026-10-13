// ==RECONSTRUCTED== shaders/resolve_to_swapchain.cs.hlsl | F-E | bins: _resolve_to_swapchain.cs.hlsl_cs_main_0.bin | entry cs:cs_main | local_size [8,8,1]
// Compute swapchain resolve (MSAA path): flip pixel (transform_image vs
// output_tex_dims), average MSAA samples, encode linear -> sRGB, write u15.

#include "inc/common.hlsl"            // samplers (unused here)
#include "resolve_to_swapchain.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint transform_image;
    uint2 output_tex_dims;
};

Texture2DMS<float4> input_tex : register(t14, space0);
RWTexture2D<unorm float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    int2 pixel = int3(dispatch_id).xy;
    int2 coord = resolve_to_swapchain_coord(pixel, output_tex_dims, transform_image);
    uint2 coord_u = uint2(coord);

    uint width, height, sample_count;
    input_tex.GetDimensions(width, height, sample_count);

    float3 acc = 0.0f;
    if (0u < sample_count) acc = input_tex.Load(int2(coord_u), 0).xyz;
    if (1u < sample_count) acc += input_tex.Load(int2(coord_u), 1).xyz;
    if (2u < sample_count) acc += input_tex.Load(int2(coord_u), 2).xyz;
    if (3u < sample_count) acc += input_tex.Load(int2(coord_u), 3).xyz;

    float3 c = acc / float(sample_count);

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

    output_tex[uint2(pixel)] = float4(r, g, b, 1.0f);
}
