#line 1 "shaders/blur.cs.hlsl"








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
#line 9 "shaders/blur.cs.hlsl"


cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims;
    uint2 img_out_dims;
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float4> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int2 dims_i = int2(img_in_dims);
    float2 dims = float2(dims_i);
    float dx = float(dims_i.x);
    float dy = float(dims_i.y);

    float2 uv = float2(float(int3(dispatchThreadID).x) / dx, float(int3(dispatchThreadID).y) / dy);
    uint2 pixel = uint2(int3(dispatchThreadID).xy);

    float4 acc = img_in[pixel];
    for (float angle = 0.0f; angle < 6.283185482025146484375f; angle += 0.2094395160675048828125f)
    {
        float4 sum = acc;
        for (float t = 0.100000001490116119384765625f; t <= 1.0f; t += 0.100000001490116119384765625f)
        {
            float2 sample_uv = uv + float2(cos(angle), sin(angle)) * (7.0f / dims) * t;
            sum += img_in[uint2(int2(int(sample_uv.x * dx), int(sample_uv.y * dy)))];
        }
        acc = sum;
    }

    img_out[pixel] = float4((acc * 0.00350877200253307819366455078125f.xxxx).xxx, 1.0f);
}
