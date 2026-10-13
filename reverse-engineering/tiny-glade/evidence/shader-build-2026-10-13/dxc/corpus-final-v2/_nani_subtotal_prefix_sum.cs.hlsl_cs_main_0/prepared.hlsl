#line 1 "shaders/nani/subtotal_prefix_sum.cs.hlsl"




#line 1 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

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
#line 7 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

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
#line 8 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

#line 1 "shaders/nani/draw_lists.inc.hlsl"






#ifndef TINYGLADE_NANI_DRAW_LISTS_HLSL
#define TINYGLADE_NANI_DRAW_LISTS_HLSL


struct GpuFilteredInstance
{
    uint raw;
};


struct GpuFilteredInstanceMeta
{
    uint raw;
};


struct NaniDrawListInfo
{
    uint global_instance_offset;
};


struct DrawIndexedIndirectCommand
{
    uint index_count;
    uint instance_count;
    uint first_index;
    uint vertex_offset;
    uint first_instance;
};

#endif 
#line 9 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

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
#line 10 "shaders/nani/subtotal_prefix_sum.cs.hlsl"

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
#line 11 "shaders/nani/subtotal_prefix_sum.cs.hlsl"


RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

struct PushConstants
{
    uint range_start;
    uint range_end;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 local_id : SV_GroupThreadID)
{
    uint idx0 = local_id.x * 2u;
    uint base = idx0 + push_constants.range_start;
    bool in_range = (base + 1u) < push_constants.range_end;

    uint2 value;
    if (in_range)
    {
        value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), __alias_inout_buf.Load<uint>((base + 1u) * 4u + 0u));
    }
    else
    {
        if (base < push_constants.range_end)
        {
            value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), 0u);
        }
        else
        {
            value = uint2(0u, 0u);
        }
    }

    shared_data2x[idx0] = value.x;
    shared_data2x[idx0 + 1u] = value.y;
    GroupMemoryBarrierWithGroupSync();

    uint d = (local_id.x >> 0u) << 1u;
    shared_data2x[d + 1u] += shared_data2x[d];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 1u) << 2u;
    shared_data2x[d + 2u + (local_id.x & 1u)] += shared_data2x[d + 1u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 2u) << 3u;
    shared_data2x[d + 4u + (local_id.x & 3u)] += shared_data2x[d + 3u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 3u) << 4u;
    shared_data2x[d + 8u + (local_id.x & 7u)] += shared_data2x[d + 7u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 4u) << 5u;
    shared_data2x[d + 16u + (local_id.x & 15u)] += shared_data2x[d + 15u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 5u) << 6u;
    shared_data2x[d + 32u + (local_id.x & 31u)] += shared_data2x[d + 31u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 6u) << 7u;
    shared_data2x[d + 64u + (local_id.x & 63u)] += shared_data2x[d + 63u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 7u) << 8u;
    shared_data2x[d + 128u + (local_id.x & 127u)] += shared_data2x[d + 127u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 8u) << 9u;
    shared_data2x[d + 256u + (local_id.x & 255u)] += shared_data2x[d + 255u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 9u) << 10u;
    shared_data2x[d + 512u + (local_id.x & 511u)] += shared_data2x[d + 511u];
    GroupMemoryBarrierWithGroupSync();

    if (in_range)
    {
        __alias_inout_buf.Store<uint>(base * 4u + 0u, shared_data2x[idx0]);
        __alias_inout_buf.Store<uint>((base + 1u) * 4u + 0u, shared_data2x[idx0 + 1u]);
    }
    else
    {
        if (base < push_constants.range_end)
        {
            __alias_inout_buf.Store<uint>(base * 4u + 0u, shared_data2x[idx0]);
        }
    }
}
