// ==RECONSTRUCTED== shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl | evidence: raw disasm of
// _nani_group_draw_cmd_prefix_sum.cs.hlsl_cs_main_0.bin: Hillis-Steele scan over a byte-address
// histogram range with end masking; block total added to draw-list length and subtotals | status: O

#line 1 "shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/prefix_scan.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);
RWStructuredBuffer<uint> __alias_draw_list_length_buffer : register(u15, space0);

struct PushConstants
{
    uint range_start;
    uint range_end;
    uint subtotals_start;
    uint subtotals_end;
    uint draw_list_index;
};

[[vk::push_constant]]
PushConstants push_constants;

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 local_id : SV_GroupThreadID)
{
    uint idx0 = local_id.x * 2u;
    uint base = idx0 + push_constants.range_start + (group_id.x * 1024u);
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

    uint prefix = (local_id.x > 0u) ? shared_data2x[idx0 - 1u] : 0u;
    if (in_range)
    {
        __alias_inout_buf.Store<uint>(base * 4u + 0u, prefix);
        __alias_inout_buf.Store<uint>((base + 1u) * 4u + 0u, shared_data2x[idx0]);
    }
    else
    {
        if (base < push_constants.range_end)
        {
            __alias_inout_buf.Store<uint>(base * 4u + 0u, prefix);
        }
    }

    uint total = shared_data2x[1023];
    if (local_id.x == 0u)
    {
        uint previous;
        InterlockedAdd(__alias_draw_list_length_buffer[push_constants.draw_list_index], total, previous);
        uint subtotal_index = 1u + group_id.x + push_constants.subtotals_start;
        if (subtotal_index < push_constants.subtotals_end)
        {
            __alias_subtotals_buf[subtotal_index] = total;
        }
    }
}
