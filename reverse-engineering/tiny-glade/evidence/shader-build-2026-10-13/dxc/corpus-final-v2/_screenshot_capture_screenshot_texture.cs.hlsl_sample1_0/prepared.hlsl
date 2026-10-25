#line 1 "shaders/screenshot/capture_screenshot_texture.cs.hlsl"




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
#line 5 "shaders/screenshot/capture_screenshot_texture.cs.hlsl"
            
#line 1 "shaders/inc/color/srgb.hlsl"




#ifndef TINYGLADE_INC_COLOR_SRGB_HLSL
#define TINYGLADE_INC_COLOR_SRGB_HLSL

static const float SRGB_ALPHA = 0.04045f;
static const float SRGB_OFFSET = 0.055f;
static const float SRGB_GAMMA = 2.4f;


float srgb_to_linear(float c)
{
    float r;
    if (c <= SRGB_ALPHA)
        r = c * 0.077399380505084991455078125f; 
    else
        r = pow((c + SRGB_OFFSET) * 0.947867333889007568359375f, SRGB_GAMMA); 
    return r;
}

float3 srgb_to_linear(float3 c)
{
    return float3(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b));
}

float4 srgb_to_linear(float4 c)
{
    return float4(srgb_to_linear(c.rgb), c.a);
}


float linear_to_srgb(float c)
{
    float r;
    if (c <= 0.0031308f)
        r = c * 12.92f;
    else
        r = 1.055f * pow(c, 1.0f / SRGB_GAMMA) - SRGB_OFFSET;
    return r;
}

float3 linear_to_srgb(float3 c)
{
    return float3(linear_to_srgb(c.r), linear_to_srgb(c.g), linear_to_srgb(c.b));
}

float4 linear_to_srgb(float4 c)
{
    return float4(linear_to_srgb(c.rgb), c.a);
}

#endif 
#line 6 "shaders/screenshot/capture_screenshot_texture.cs.hlsl"

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
#line 7 "shaders/screenshot/capture_screenshot_texture.cs.hlsl"

#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 8 "shaders/screenshot/capture_screenshot_texture.cs.hlsl"


cbuffer $Globals : register(b14, space0)
{
    uint2 dst_tex_dims;
    uint2 dst_dims;
    uint2 src_tex_dims;
    uint2 src_ms_tex_dims;
};

RWTexture2D<unorm float4> dst_tex : register(u15, space0);

Texture2D<float4> src_tex : register(t0, space0);
Texture2DMS<float4> src_ms_tex : register(t0, space0);

float encode_srgb(float c)
{
    if (0.0031308f >= c) return c * 12.92f;
    return 1.055f * pow(c, 0.416666657f) - 0.055f;
}

float4 encode4(float3 c)
{
    return float4(encode_srgb(c.x), encode_srgb(c.y), encode_srgb(c.z), 1.0f);
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

    dst_tex[dispatch_id.xy] = encode4(c);
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

    dst_tex[dispatch_id.xy] = encode4(c);
}
