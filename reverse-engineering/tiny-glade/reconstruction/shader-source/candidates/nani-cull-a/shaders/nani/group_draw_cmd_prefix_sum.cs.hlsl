// ==RECONSTRUCTED== shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl | nani-cull-a
// Observation: _nani_group_draw_cmd_prefix_sum.cs.hlsl_cs_main_0.bin (LocalSize
// 512,1,1; push_constants {range_start,range_end,subtotals_start,subtotals_end,
// draw_list_index}; u13 RWByteAddressBuffer __alias_inout_buf, u14
// RWStructuredBuffer<uint> __alias_subtotals_buf, u15 RWStructuredBuffer<uint>
// __alias_draw_list_length_buffer; groupshared shared_data2x[1024]).
// Two-words-per-lane EXCLUSIVE block scan over the per-group [range_start+1024*gid,
// range_end) window of the draw-command histogram; lane 0 accumulates the block
// total into the draw-list length counter and writes the group subtotal slot.
#include "inc/common.hlsl"

struct PushConstants
{
    uint range_start;
    uint range_end;
    uint subtotals_start;
    uint subtotals_end;
    uint draw_list_index;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);
RWStructuredBuffer<uint> __alias_draw_list_length_buffer : register(u15, space0);
groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    uint idx = groupThreadID.x * 2u;
    uint base = idx + push_constants.range_start + groupID.x * 1024u;
    uint byte_off = base * 4u;
    bool valid2 = (base + 1u) < push_constants.range_end;

    uint2 v;
    if (valid2)
    {
        v = uint2(__alias_inout_buf.Load<uint>(byte_off), __alias_inout_buf.Load<uint>(byte_off + 4u));
    }
    else if (base < push_constants.range_end)
    {
        v = uint2(__alias_inout_buf.Load<uint>(byte_off), 0u);
    }
    else
    {
        v = uint2(0u, 0u);
    }
    shared_data2x[idx] = v.x;
    shared_data2x[idx + 1u] = v.y;
    GroupMemoryBarrierWithGroupSync();

    [unroll]
    for (uint s = 0u; s < 10u; ++s)
    {
        uint d = 1u << s;
        uint block = (groupThreadID.x >> s) << (s + 1u);
        shared_data2x[block + d + (groupThreadID.x & (d - 1u))] += shared_data2x[block + d - 1u];
        GroupMemoryBarrierWithGroupSync();
    }

    uint ex0 = groupThreadID.x > 0u ? shared_data2x[idx - 1u] : 0u;
    if (valid2)
    {
        __alias_inout_buf.Store<uint>(byte_off, ex0);
        __alias_inout_buf.Store<uint>(byte_off + 4u, shared_data2x[idx]);
    }
    else if (base < push_constants.range_end)
    {
        __alias_inout_buf.Store<uint>(byte_off, ex0);
    }

    uint total = shared_data2x[1023];
    if (groupThreadID.x == 0u)
    {
        uint unused;
        InterlockedAdd(__alias_draw_list_length_buffer[push_constants.draw_list_index], total, unused);
        uint subtotal_slot = (1u + groupID.x) + push_constants.subtotals_start;
        if (subtotal_slot < push_constants.subtotals_end)
        {
            __alias_subtotals_buf[subtotal_slot] = total;
        }
    }
}
