// ==RECONSTRUCTED== shaders/screenshot/capture_screenshot_buffer.cs.hlsl | F-E | bins: _screenshot_capture_screenshot_buffer.cs.hlsl_{sample1,sample4}_0.bin | entries cs:sample1 cs:sample4 | local_size [8,8,1]
// Captures a screenshot into a byte-address buffer as RGBA8 rows. sample1 reads
// a non-MSAA source; sample4 averages/resamples a Texture2DMS source. When the
// sizes match, a direct copy (+ MSAA average for sample4) is used; otherwise a
// gaussian downscale (inc/capture_screenshot.inc.hlsl).

#include "inc/common.hlsl"            // samplers (unused here)
#include "inc/pack_unpack.hlsl"
#include "capture_screenshot.inc.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 dst_dims;
    uint2 src_tex_dims;
    uint2 src_ms_tex_dims;
};

RWByteAddressBuffer __alias_dst_buf : register(u14, space0);

Texture2D<float4> src_tex : register(t0, space0);
Texture2DMS<float4> src_ms_tex : register(t0, space0);

// linear -> sRGB per channel (strict threshold matches shipped codegen).
float encode_srgb(float c)
{
    if (0.0031308f >= c) return c * 12.92f;
    return 1.055f * pow(c, 0.416666657f) - 0.055f;
}

uint pack_rgba8_opaque(float3 c)
{
    float r = encode_srgb(c.x);
    float g = encode_srgb(c.y);
    float b = encode_srgb(c.z);
    return (uint(clamp(r, 0.0f, 1.0f) * 255.0f + 0.5f)
          | (uint(clamp(g, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
          | (uint(clamp(b, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
         | 0xFF000000u;
}

[numthreads(8, 8, 1)]
void sample1(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (any(dispatch_id.xy >= dst_dims))
        return;

    float3 c;
    if (all(src_tex_dims == dst_dims))
        c = capture_direct(src_tex, dispatch_id.xy);
    else
        c = capture_gaussian(src_tex, src_tex_dims, dst_dims, dispatch_id.xy);

    uint index = dispatch_id.x + dispatch_id.y * dst_dims.x;
    __alias_dst_buf.Store<uint>(index * 4u, pack_rgba8_opaque(c));
}

[numthreads(8, 8, 1)]
void sample4(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (any(dispatch_id.xy >= dst_dims))
        return;

    float3 c;
    if (all(src_ms_tex_dims == dst_dims))
    {
        float3 acc = 0.0f;
        for (int i = 0; i < 4; ++i)
            acc += src_ms_tex.Load(dispatch_id.xy, i).xyz;
        c = clamp(acc * 0.25f, 0.0f, 1.0f);
    }
    else
        c = capture_gaussian_msaa(src_ms_tex, src_ms_tex_dims, dst_dims, dispatch_id.xy);

    uint index = dispatch_id.x + dispatch_id.y * dst_dims.x;
    __alias_dst_buf.Store<uint>(index * 4u, pack_rgba8_opaque(c));
}
