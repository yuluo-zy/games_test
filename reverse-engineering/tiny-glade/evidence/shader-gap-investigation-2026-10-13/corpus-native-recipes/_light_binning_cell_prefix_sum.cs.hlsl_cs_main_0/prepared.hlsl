#line 1 "shaders/light_binning/cell_prefix_sum.cs.hlsl"















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
#line 16 "shaders/light_binning/cell_prefix_sum.cs.hlsl"

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
#line 17 "shaders/light_binning/cell_prefix_sum.cs.hlsl"

#line 1 "shaders/light_binning/grid.inc.hlsl"












#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL


static const float3 GRID_WORLD_SCALE = float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f);
static const float3 GRID_WORLD_OFFSET = float3(16.0f, 2.0f, 16.0f);
static const uint3 GRID_DIMS = uint3(32u, 16u, 32u);
static const float3 GRID_CELL_SIZE = float3(4.0625f, 8.125f, 4.0625f);


uint3 grid_world_pos_to_cell(float3 world_pos)
{
    return uint3(clamp(floor(world_pos * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                       float3(0.0f, 0.0f, 0.0f), float3(GRID_DIMS) - 1.0f));
}



uint grid_cell_index(uint3 cell)
{
    return 32u * (32u * cell.y + cell.z) + cell.x;
}


float3 grid_cell_min_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET) * GRID_CELL_SIZE;
}

float3 grid_cell_max_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET + float3(1.0f, 1.0f, 1.0f)) * GRID_CELL_SIZE;
}



float grid_point_to_cell_dist2(float3 p, float r2, uint3 cell)
{
    float3 mn = grid_cell_min_world(cell);
    float3 mx = grid_cell_max_world(cell);
    float d2 = r2;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        if (p[i] < mn[i]) {
            float e = p[i] - mn[i];
            d2 -= e * e;
        } else if (p[i] > mx[i]) {
            float e = p[i] - mx[i];
            d2 -= e * e;
        }
    }
    return d2;
}

#endif 
#line 18 "shaders/light_binning/cell_prefix_sum.cs.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 19 "shaders/light_binning/cell_prefix_sum.cs.hlsl"


RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID, uint3 gl_LocalInvocationID : SV_GroupThreadID)
{
    uint lane = gl_LocalInvocationID.x;
    uint lane2 = 2u * lane;
    
    
    
    uint byte_addr = 4u * (lane2 + (gl_WorkGroupID.x * 1024u));

    uint2 v = min(uint2(__alias_inout_buf.Load<uint>(byte_addr),
                        __alias_inout_buf.Load<uint>(byte_addr + 4u)), uint2(64u, 64u));
    shared_data2x[lane2] = v.x;
    shared_data2x[lane2 + 1u] = v.y;
    GroupMemoryBarrierWithGroupSync();

    
    
    shared_data2x[((lane >> 0u) << 1u) + 1u] += shared_data2x[(lane >> 0u) << 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 1u) << 2u) + 2u + (lane & 1u)] += shared_data2x[((lane >> 1u) << 2u) + 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 2u) << 3u) + 4u + (lane & 3u)] += shared_data2x[((lane >> 2u) << 3u) + 3u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 3u) << 4u) + 8u + (lane & 7u)] += shared_data2x[((lane >> 3u) << 4u) + 7u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 4u) << 5u) + 16u + (lane & 15u)] += shared_data2x[((lane >> 4u) << 5u) + 15u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 5u) << 6u) + 32u + (lane & 31u)] += shared_data2x[((lane >> 5u) << 6u) + 31u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 6u) << 7u) + 64u + (lane & 63u)] += shared_data2x[((lane >> 6u) << 7u) + 63u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 7u) << 8u) + 128u + (lane & 127u)] += shared_data2x[((lane >> 7u) << 8u) + 127u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 8u) << 9u) + 256u + (lane & 255u)] += shared_data2x[((lane >> 8u) << 9u) + 255u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((lane >> 9u) << 10u) + 512u + (lane & 511u)] += shared_data2x[((lane >> 9u) << 10u) + 511u];
    GroupMemoryBarrierWithGroupSync();

    
    uint ex0 = (lane > 0u) ? shared_data2x[lane2 - 1u] : 0u;
    uint ex1 = shared_data2x[lane2];
    __alias_inout_buf.Store<uint>(byte_addr, ex0);
    __alias_inout_buf.Store<uint>(byte_addr + 4u, ex1);

    if (lane == 0u) {
        __alias_subtotals_buf[gl_WorkGroupID.x] = shared_data2x[1023u];
    }
}
