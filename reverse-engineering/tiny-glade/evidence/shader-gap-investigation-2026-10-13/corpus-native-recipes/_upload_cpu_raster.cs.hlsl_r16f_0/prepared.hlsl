#line 1 "shaders/upload_cpu_raster.cs.hlsl"


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
#line 3 "shaders/upload_cpu_raster.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 4 "shaders/upload_cpu_raster.cs.hlsl"

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
#line 5 "shaders/upload_cpu_raster.cs.hlsl"




ByteAddressBuffer cpu_raster : register(t14, space0);

cbuffer $Globals : register(b15, space0)
{
    uint output_tex_width;
};


RWTexture2D<half> output_tex_r16f : register(u0, space0);


[[vk::ext_capability(49)]]
RWTexture2D<half4> output_tex_rgba16f : register(u0, space0);

float4 decode_rgba8(uint word)
{
    return float4(
        float(word & 0xffu) * (1.0f / 255.0f),
        float((word >> 8u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 16u) & 0xffu) * (1.0f / 255.0f),
        float((word >> 24u) & 0xffu) * (1.0f / 255.0f));
}




uint load_pixel_word(uint2 pixel)
{
    uint tile_width = output_tex_width / 64u;
    uint2 tile = pixel / uint2(tile_width, tile_width);
    uint2 in_tile = pixel % uint2(tile_width, tile_width);
    uint slot = (((64u * tile.y) + tile.x) * (tile_width * tile_width))
                + (in_tile.y * tile_width) + in_tile.x;
    return cpu_raster.Load<uint>(slot);
}

[numthreads(8, 8, 1)]
void r16f(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 start_pixel = dispatch_thread_id.xy * uint2(4u, 1u);
    float4 pixel = decode_rgba8(load_pixel_word(start_pixel));
    for (uint i = 0u; i < 4u; i++)
    {
        output_tex_r16f[start_pixel + uint2(i, 0u)] = pixel[i];
    }
}

[numthreads(8, 8, 1)]
void rgba16f(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 start_pixel = dispatch_thread_id.xy * uint2(4u, 1u);
    float4 pixel = decode_rgba8(load_pixel_word(start_pixel));
    for (uint i = 0u; i < 4u; i++)
    {
        output_tex_rgba16f[start_pixel + uint2(i, 0u)] = float4(pixel[i], 0.0f, 0.0f, 0.0f);
    }
}
