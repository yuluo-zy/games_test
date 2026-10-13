// ==RECONSTRUCTED== shaders/capture_panic_screenshot.cs.hlsl | F-E | bins: _capture_panic_screenshot.cs.hlsl_cs_main_0.bin | entry cs:cs_main | local_size [64,1,1]
// Panic-screen capture: writes dst_dims to the buffer header, then maps every
// thread to two dst pixels, encodes each to linear, converts to Rec.709 YCbCr
// and packs (Y0,Cb0,Y1,Cr1) into one uint per pixel pair.

#include "inc/common.hlsl"            // samplers (unused here)
#include "inc/pack_unpack.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/color/ycbcr.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 src_tex_dims;
};

struct PushConstants
{
    uint2 dst_dims;
};

[[vk::push_constant]]
PushConstants push_constants;

Texture2DMS<float4> src_tex : register(t13, space0);
RWByteAddressBuffer __alias_dst_buf : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    if (0u == dispatch_id.x)
    {
        __alias_dst_buf.Store<uint>(0, push_constants.dst_dims.x);
        __alias_dst_buf.Store<uint>(4, push_constants.dst_dims.y);
    }

    float2 dstf = float2(push_constants.dst_dims);
    float2 srcf = float2(src_tex_dims);

    // Two destination pixels per thread.
    uint base = dispatch_id.x * 2u;

    uint2 dst0 = uint2(base % push_constants.dst_dims.x, base / push_constants.dst_dims.x);
    uint2 dst1 = uint2((base + 1u) % push_constants.dst_dims.x, (base + 1u) / push_constants.dst_dims.x);

    float3 lin0 = clamp(src_tex.Load(int2(((float2(dst0) + 0.5f) / dstf) * srcf), 0).xyz, 0.0f, 1.0f);
    float3 lin1 = clamp(src_tex.Load(int2(((float2(dst1) + 0.5f) / dstf) * srcf), 0).xyz, 0.0f, 1.0f);

    float3 e0;
    e0.x = (0.0031308f >= lin0.x) ? lin0.x * 12.92f : 1.055f * pow(lin0.x, 0.416666657f) - 0.055f;
    e0.y = (0.0031308f >= lin0.y) ? lin0.y * 12.92f : 1.055f * pow(lin0.y, 0.416666657f) - 0.055f;
    e0.z = (0.0031308f >= lin0.z) ? lin0.z * 12.92f : 1.055f * pow(lin0.z, 0.416666657f) - 0.055f;
    float3 e1;
    e1.x = (0.0031308f >= lin1.x) ? lin1.x * 12.92f : 1.055f * pow(lin1.x, 0.416666657f) - 0.055f;
    e1.y = (0.0031308f >= lin1.y) ? lin1.y * 12.92f : 1.055f * pow(lin1.y, 0.416666657f) - 0.055f;
    e1.z = (0.0031308f >= lin1.z) ? lin1.z * 12.92f : 1.055f * pow(lin1.z, 0.416666657f) - 0.055f;

    float4 ycc = clamp(float4(
        (rgb_to_ycbcr(e0) * float3(1.0f, 0.5f, 0.5f) + float3(0.0f, 0.5f, 0.5f)).xy,
        (rgb_to_ycbcr(e1) * float3(1.0f, 0.5f, 0.5f) + float3(0.0f, 0.5f, 0.5f)).xz), 0.0f, 1.0f);

    uint packed = (uint(clamp(ycc.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                 | (uint(clamp(ycc.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                 | (uint(clamp(ycc.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                 | (uint(clamp(ycc.w, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u));

    __alias_dst_buf.Store<uint>(dispatch_id.x * 4u + 8u, packed);
}
