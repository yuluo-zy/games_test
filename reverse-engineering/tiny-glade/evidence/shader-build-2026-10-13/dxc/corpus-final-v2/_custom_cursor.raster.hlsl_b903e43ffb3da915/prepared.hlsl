#define SHADER_VARIANT_DEPTH_ONLY 0
#line 1 "shaders/custom_cursor.raster.hlsl"




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
#line 5 "shaders/custom_cursor.raster.hlsl"
            
#line 1 "shaders/inc/quad_mesh.hlsl"




#ifndef TINYGLADE_INC_QUAD_MESH_HLSL
#define TINYGLADE_INC_QUAD_MESH_HLSL


float2 quad_corner(uint vertex_id)
{
    return float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u));
}



float4 fullscreen_triangle_clip(uint vertex_id)
{
    float2 uv = float2(float(vertex_id & 1u), float(vertex_id >> 1u));
    return float4(uv * 2.0f - 1.0f, 0.0f, 1.0f);
}


float4 unit_quad_clip(uint vertex_id)
{
    float2 corner = quad_corner(vertex_id);
    return float4(corner * 2.0f - 1.0f, 0.0f, 1.0f);
}

#endif 
#line 6 "shaders/custom_cursor.raster.hlsl"

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
#line 7 "shaders/custom_cursor.raster.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 window_size_pixels;
    uint2 cursor_tex_dims;
};

struct CustomCursor
{
    float2 pos;
    uint enabled;
    float size_px;
};

ByteAddressBuffer cursor_buf : register(t14, space0);
Texture2D<float4> cursor_tex : register(t15, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 1.0f);

    CustomCursor c;
    c.pos = cursor_buf.Load<float2>(0);
    c.enabled = cursor_buf.Load<uint>(8);
    c.size_px = cursor_buf.Load<float>(12);

    float2 screen_pos = (((corner.xy * float2(1.0f, -1.0f) + 0.5f) * c.size_px + (c.pos - 1.0f)) / float2(window_size_pixels)) - 0.5f;
    screen_pos *= float2(2.0f, -2.0f);
    float4 pos = float4(screen_pos, 1.0f, 1.0f);
    if (c.enabled == 0u)
        pos = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = pos;
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = cursor_tex.SampleBias(sampler_llc, uv, -0.5f);
    return float4(c.rgb * c.a, c.a);
}
