#line 1 "shaders/dof/init_buffers.cs.hlsl"



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
#line 4 "shaders/dof/init_buffers.cs.hlsl"

#line 1 "shaders/dof/tiles.inc.hlsl"









#ifndef TINYGLADE_DOF_TILES_INC_HLSL
#define TINYGLADE_DOF_TILES_INC_HLSL

struct DofTileOffsets
{
    uint offsets[8];
};

struct DofTileOffsetsPacked
{
    uint4 offsets[2];
};

#endif 
#line 5 "shaders/dof/init_buffers.cs.hlsl"


struct PushConstants
{
    uint args_type_count;
};
[[vk::push_constant]]
PushConstants push_constants;

ConstantBuffer<DofTileOffsetsPacked> tile_offsets_initial_values : register(b13, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u14, space0);
RWStructuredBuffer<DofTileOffsetsPacked> tile_offsets_buffer : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x < push_constants.args_type_count)
    {
        __alias_dispatch_args_buffer[gl_GlobalInvocationID.x] = uint4(0u, 1u, 1u, 0u);
    }
    if (gl_GlobalInvocationID.x == 0u)
    {
        tile_offsets_buffer[0] = tile_offsets_initial_values;
    }
}
