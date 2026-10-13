#line 1 "shaders/blur_downsample.cs.hlsl"











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
#line 12 "shaders/blur_downsample.cs.hlsl"


RWTexture2D<unorm float4> output_tex : register(u15, space0);
Texture2D<float4> input_tex : register(t0, space0);
Texture2DMS<float4> input_ms_tex : register(t0, space0);

groupshared float4 vblur_out[138];

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID,
             uint3 groupThreadID : SV_GroupThreadID,
             uint3 groupID : SV_GroupID)
{
    int2 gid_i = int2(dispatchThreadID.xy);

    for (uint i = groupThreadID.x; i < 138u; i += 64u)
    {
        int2 tile = int2((groupID.xy * uint2(128u, 2u)) + uint2(int2(int(i - 5u), -5)));

        float weight_sum = 0.0f;
        float4 sum = 0.0f;
        for (uint j = 0u; j <= 10u; ++j)
        {
            float d = ((float(gid_i.y) + 0.5f) * 2.0f) - (float(uint(tile.y) + j) + 0.5f);
            float w = exp(-d * d * 0.1599999964237213134765625f);
            weight_sum += w;
            sum += input_tex.Load(int3(uint2(tile + int2(0, int(j))), 0)) * w;
        }
        vblur_out[i] = sum / weight_sum;
    }

    GroupMemoryBarrierWithGroupSync();

    float weight_sum = 0.0f;
    float4 sum = 0.0f;
    for (uint k = 0u; k <= 10u; ++k)
    {
        float d = ((float(dispatchThreadID.x) + 0.5f) * 2.0f) - (float((dispatchThreadID.x * 2u + k) - 5u) + 0.5f);
        float w = exp(-d * d * 0.1599999964237213134765625f);
        sum += vblur_out[groupThreadID.x * 2u + k] * w;
        weight_sum += w;
    }

    output_tex[dispatchThreadID.xy] = sum / weight_sum;
}

[numthreads(64, 1, 1)]
void cs_main_multisampled(uint3 dispatchThreadID : SV_DispatchThreadID,
                          uint3 groupThreadID : SV_GroupThreadID,
                          uint3 groupID : SV_GroupID)
{
    int2 gid_i = int2(dispatchThreadID.xy);

    for (uint i = groupThreadID.x; i < 138u; i += 64u)
    {
        int2 tile = int2((groupID.xy * uint2(128u, 2u)) + uint2(int2(int(i - 5u), -5)));

        float weight_sum = 0.0f;
        float4 sum = 0.0f;
        for (uint j = 0u; j <= 10u; ++j)
        {
            float d = ((float(gid_i.y) + 0.5f) * 2.0f) - (float(uint(tile.y) + j) + 0.5f);
            float w = exp(-d * d * 0.1599999964237213134765625f);
            weight_sum += w;
            sum += input_ms_tex.Load(uint2(tile + int2(0, int(j))), 0u) * w;
        }
        vblur_out[i] = sum / weight_sum;
    }

    GroupMemoryBarrierWithGroupSync();

    float weight_sum = 0.0f;
    float4 sum = 0.0f;
    for (uint k = 0u; k <= 10u; ++k)
    {
        float d = ((float(dispatchThreadID.x) + 0.5f) * 2.0f) - (float((dispatchThreadID.x * 2u + k) - 5u) + 0.5f);
        float w = exp(-d * d * 0.1599999964237213134765625f);
        sum += vblur_out[groupThreadID.x * 2u + k] * w;
        weight_sum += w;
    }

    output_tex[dispatchThreadID.xy] = sum / weight_sum;
}
