#line 1 "shaders/screenshot/capture_screenshot_buffer.cs.hlsl"






#line 1 "shaders/inc/common.hlsl"





#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#line 1 "shaders/inc/samplers.hlsl"







#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 11 "shaders/inc/samplers.hlsl"


SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif 
#line 9 "shaders/inc/common.hlsl"


#endif 
#line 7 "shaders/screenshot/capture_screenshot_buffer.cs.hlsl"
            
#line 1 "shaders/inc/pack_unpack.hlsl"




#ifndef TINYGLADE_INC_PACK_UNPACK_HLSL
#define TINYGLADE_INC_PACK_UNPACK_HLSL


uint pack_rgba8(float4 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint a = uint(clamp(c.w, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}


uint pack_rgb8(float3 c)
{
    uint r = uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint g = uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f);
    uint b = uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f);
    return (b << 16u) | (g << 8u) | r;
}


float3 unpack_rgb8(uint p)
{
    float3 c;
    c.r = float(p & 0xFFu) * (1.0f / 255.0f);
    c.g = float((p >> 8u) & 0xFFu) * (1.0f / 255.0f);
    c.b = float((p >> 16u) & 0xFFu) * (1.0f / 255.0f);
    return c;
}


float4 unpack_rgba8(uint p)
{
    return float4(unpack_rgb8(p), float((p >> 24u) & 0xFFu) * (1.0f / 255.0f));
}


float3 unpack_normal_rgb8(uint p)
{
    return unpack_rgb8(p) * 2.0f - 1.0f;
}

#endif 
#line 8 "shaders/screenshot/capture_screenshot_buffer.cs.hlsl"

#line 1 "shaders/screenshot/capture_screenshot.inc.hlsl"





#ifndef TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL
#define TINYGLADE_CAPTURE_SCREENSHOT_INC_HLSL


float3 capture_direct(Texture2D<float4> tex, uint2 coord)
{
    return clamp(tex.Load(int3(coord, 0)).xyz, 0.0f, 1.0f);
}


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

#endif 
#line 9 "shaders/screenshot/capture_screenshot_buffer.cs.hlsl"


cbuffer $Globals : register(b15, space0)
{
    uint2 dst_dims;
    uint2 src_tex_dims;
    uint2 src_ms_tex_dims;
};

RWByteAddressBuffer __alias_dst_buf : register(u14, space0);

Texture2D<float4> src_tex : register(t0, space0);
Texture2DMS<float4> src_ms_tex : register(t0, space0);


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
