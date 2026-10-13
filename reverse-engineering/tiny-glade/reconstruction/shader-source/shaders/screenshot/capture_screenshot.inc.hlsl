// ==RECONSTRUCTED== shaders/screenshot/capture_screenshot.inc.hlsl | F-E | evidence: _screenshot_capture_screenshot_{buffer,texture}.cs.hlsl_{sample1,sample4} bins — shared gaussian downscale (aspect-corrected source center, radius clamp(1..3), exp(-3*d^2/sigma^2) box weights) + direct-copy path | status: O (downscale algorithm/constants from inlined SPIR-V); I (helper names/signatures — inlined)
//
// Shared screenshot capture helpers: a gaussian-weighted box downscale used when
// the source and destination sizes differ, plus the same-size direct path.

#ifndef TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL
#define TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL

// Direct copy path: read one source texel.
float3 capture_direct(Texture2D<float4> tex, uint2 coord)
{
    return clamp(tex.Load(int3(coord, 0)).xyz, 0.0f, 1.0f);
}

// Gaussian-weighted box downscale of tex into dst_dims at dst_pixel.
float3 capture_gaussian(Texture2D<float4> tex, uint2 src_dims, uint2 dst_dims, uint2 dst_pixel)
{
    float2 srcf = float2(src_dims);
    float2 dstf = float2(dst_dims);

    float2 src_aspect = srcf / float(min(src_dims.x, src_dims.y));
    float2 dst_aspect = dstf / float(min(dst_dims.x, dst_dims.y));
    float2 ratio = src_aspect / dst_aspect;
    float2 scale = min(ratio.x, ratio.y).xx;

    float2 sigma_scale = (((3.0f / dstf) * dst_aspect) * scale / src_aspect) * srcf;
    float radius = clamp(max(sigma_scale.x, sigma_scale.y), 1.0f, 3.0f);

    float2 center = (((float2(int2(dst_pixel)) + 0.5f) / dstf - 0.5f) * dst_aspect) * scale / src_aspect + 0.5f;
    center *= srcf;

    int2 lo = int2((center - 0.5f) - radius);
    int2 hi = int2((center + 0.5f) + radius);

    float4 acc = 0.0f;
    for (int y = lo.y; y < hi.y; ++y)
    {
        for (int x = lo.x; x < hi.x; ++x)
        {
            int2 p = int2(x, y);
            float2 d = (float2(p) + 0.5f) - center;
            float w = exp((-3.0f * dot(d, d)) / (radius * radius));
            acc += float4(tex.Load(int3(p, 0)).xyz, 1.0f) * w * float(all(uint2(p) < src_dims));
        }
    }

    return clamp(acc.xyz / max(9.9999997e-6f, acc.w), 0.0f, 1.0f);
}

// MSAA variant: same downscale but reading sample 0 of a Texture2DMS.
float3 capture_gaussian_msaa(Texture2DMS<float4> tex, uint2 src_dims, uint2 dst_dims, uint2 dst_pixel)
{
    float2 srcf = float2(src_dims);
    float2 dstf = float2(dst_dims);

    float2 src_aspect = srcf / float(min(src_dims.x, src_dims.y));
    float2 dst_aspect = dstf / float(min(dst_dims.x, dst_dims.y));
    float2 ratio = src_aspect / dst_aspect;
    float2 scale = min(ratio.x, ratio.y).xx;

    float2 sigma_scale = (((3.0f / dstf) * dst_aspect) * scale / src_aspect) * srcf;
    float radius = clamp(max(sigma_scale.x, sigma_scale.y), 1.0f, 3.0f);

    float2 center = (((float2(int2(dst_pixel)) + 0.5f) / dstf - 0.5f) * dst_aspect) * scale / src_aspect + 0.5f;
    center *= srcf;

    int2 lo = int2((center - 0.5f) - radius);
    int2 hi = int2((center + 0.5f) + radius);

    float4 acc = 0.0f;
    for (int y = lo.y; y < hi.y; ++y)
    {
        for (int x = lo.x; x < hi.x; ++x)
        {
            int2 p = int2(x, y);
            float2 d = (float2(p) + 0.5f) - center;
            float w = exp((-3.0f * dot(d, d)) / (radius * radius));
            acc += float4(tex.Load(uint2(p), 0u).xyz, 1.0f) * w * float(all(uint2(p) < src_dims));
        }
    }

    return clamp(acc.xyz / max(9.9999997e-6f, acc.w), 0.0f, 1.0f);
}

#endif // TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL
