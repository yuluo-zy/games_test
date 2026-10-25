#line 1 "shaders/capture_panic_screenshot.cs.hlsl"





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
#line 6 "shaders/capture_panic_screenshot.cs.hlsl"
            
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
#line 7 "shaders/capture_panic_screenshot.cs.hlsl"

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
#line 8 "shaders/capture_panic_screenshot.cs.hlsl"

#line 1 "shaders/inc/color/ycbcr.hlsl"





#ifndef TINYGLADE_INC_COLOR_YCBCR_HLSL
#define TINYGLADE_INC_COLOR_YCBCR_HLSL

float3 rgb_to_ycbcr(float3 rgb)
{
    return mul(
        float3x3(
            float3( 0.2126f,  0.7152f,  0.0722f),
            float3(-0.1146f, -0.3854f,  0.5f),
            float3( 0.5f,   -0.4542f, -0.0458f)),
        rgb);
}

float3 ycbcr_to_rgb(float3 ycbcr)
{
    
    return mul(
        float3x3(
            float3(1.0f,        1.0f,       1.0f),
            float3(0.0f,       -0.18732f,   1.8556f),
            float3(1.5748f,    -0.46812f,   0.0f)),
        ycbcr);
}

#endif 
#line 9 "shaders/capture_panic_screenshot.cs.hlsl"


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
