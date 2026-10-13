#line 1 "shaders/grass/tiles_prefix.cs.hlsl"








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
#line 9 "shaders/grass/tiles_prefix.cs.hlsl"

#line 1 "shaders/grass/grass_common.inc.hlsl"










#ifndef TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL
#define TINYGLADE_GRASS_GRASS_COMMON_INC_HLSL

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 14 "shaders/grass/grass_common.inc.hlsl"





struct GrassTilePacked
{
    uint raw;
};




struct GrassBundlePacked
{
    uint raw;
};

#endif 
#line 10 "shaders/grass/tiles_prefix.cs.hlsl"

#line 1 "shaders/inc/prefix_scan.hlsl"


#ifndef TINYGLADE_INC_PREFIX_SCAN_HLSL
#define TINYGLADE_INC_PREFIX_SCAN_HLSL






#define PREFIX_SCAN_UINT(shared, lane, element_count) \
    { \
        for (uint _ps_s = 0u; (1u << _ps_s) < (element_count); ++_ps_s) { \
            GroupMemoryBarrierWithGroupSync(); \
            uint _ps_d = 1u << _ps_s; \
            uint _ps_base = ((lane) >> _ps_s) << (_ps_s + 1u); \
            uint _ps_i = _ps_base + _ps_d + ((lane) & (_ps_d - 1u)); \
            if (_ps_i < (element_count)) { \
                shared[_ps_i] += shared[_ps_i - _ps_d]; \
            } \
        } \
        GroupMemoryBarrierWithGroupSync(); \
    }





#define PREFIX_SCAN_2_PER_LANE(inout_buf, subtotals_buf, shared, lane, group_id, block_words, value_clamp) \
    { \
        uint _ps2_idx0 = 4u * ((2u * (lane)) + ((group_id) * (block_words))); \
        uint _ps2_idx1 = _ps2_idx0 + 1u; \
        uint _ps2_v0 = min((inout_buf).Load<uint>(_ps2_idx0 * 4u + 0u), (value_clamp)); \
        uint _ps2_v1 = min((inout_buf).Load<uint>(_ps2_idx1 * 4u + 0u), (value_clamp)); \
        (shared)[(lane) * 2u] = _ps2_v0; \
        (shared)[(lane) * 2u + 1u] = _ps2_v1; \
        GroupMemoryBarrierWithGroupSync(); \
        PREFIX_SCAN_UINT((shared), (lane), (block_words) * 2u); \
        uint _ps2_ex0 = ((lane) == 0u) ? 0u : (shared)[(lane) * 2u - 1u]; \
        uint _ps2_ex1 = (shared)[(lane) * 2u]; \
        (inout_buf).Store<uint>(_ps2_idx0 * 4u + 0u, _ps2_ex0); \
        (inout_buf).Store<uint>(_ps2_idx1 * 4u + 0u, _ps2_ex1); \
        if ((lane) == 0u) { \
            (subtotals_buf).Store<uint>((group_id) * 4u + 0u, (shared)[(block_words) * 2u - 1u]); \
        } \
    }

#endif 
#line 11 "shaders/grass/tiles_prefix.cs.hlsl"


RWByteAddressBuffer __alias_blade_prefix_subtotal_buffer : register(u13, space0);

groupshared uint shared_data2x[256];

[numthreads(128, 1, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint word = dispatch_id.x * 2u;
    shared_data2x[word] = __alias_blade_prefix_subtotal_buffer.Load<uint>(word * 4 + 0);
    shared_data2x[word + 1u] = __alias_blade_prefix_subtotal_buffer.Load<uint>(word * 4 + 4);
    GroupMemoryBarrierWithGroupSync();
    uint i = (dispatch_id.x >> 0u) << 1u;
    shared_data2x[i + 1u] += shared_data2x[i];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 1u) << 2u;
    shared_data2x[i + 2u + (dispatch_id.x & 1u)] += shared_data2x[i + 1u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 2u) << 3u;
    shared_data2x[i + 4u + (dispatch_id.x & 3u)] += shared_data2x[i + 3u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 3u) << 4u;
    shared_data2x[i + 8u + (dispatch_id.x & 7u)] += shared_data2x[i + 7u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 4u) << 5u;
    shared_data2x[i + 16u + (dispatch_id.x & 15u)] += shared_data2x[i + 15u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 5u) << 6u;
    shared_data2x[i + 32u + (dispatch_id.x & 31u)] += shared_data2x[i + 31u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 6u) << 7u;
    shared_data2x[i + 64u + (dispatch_id.x & 63u)] += shared_data2x[i + 63u];
    GroupMemoryBarrierWithGroupSync();
    i = (dispatch_id.x >> 7u) << 8u;
    shared_data2x[i + 128u + (dispatch_id.x & 127u)] += shared_data2x[i + 127u];
    GroupMemoryBarrierWithGroupSync();
    uint excl;
    if (dispatch_id.x > 0u)
    {
        excl = shared_data2x[word - 1u];
    }
    else
    {
        excl = 0u;
    }
    __alias_blade_prefix_subtotal_buffer.Store<uint>(word * 4 + 0, excl);
    __alias_blade_prefix_subtotal_buffer.Store<uint>(word * 4 + 4, shared_data2x[word]);
    if (local_index == 0u)
    {
        __alias_blade_prefix_subtotal_buffer.Store<uint>(1024, shared_data2x[255]);
    }
}
