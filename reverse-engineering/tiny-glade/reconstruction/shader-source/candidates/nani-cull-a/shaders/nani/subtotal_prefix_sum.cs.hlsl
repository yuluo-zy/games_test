// ==RECONSTRUCTED== shaders/nani/subtotal_prefix_sum.cs.hlsl | nani-cull-a
// Observation: _nani_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin (LocalSize
// 512,1,1; push_constants {range_start,range_end}; u13 RWByteAddressBuffer
// __alias_inout_buf; groupshared shared_data2x[1024]; 11 OpControlBarrier).
// Two-words-per-lane Hillis-Steele INCLUSIVE block scan over the
// [range_start, range_end) window of the histogram buffer; lanes whose second
// word lies outside the window load a zero and write back only in-range words.
#include "inc/common.hlsl"

struct PushConstants
{
    uint range_start;
    uint range_end;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID)
{
    uint idx = groupThreadID.x * 2u;
    uint base = idx + push_constants.range_start;
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

    if (valid2)
    {
        __alias_inout_buf.Store<uint>(byte_off, shared_data2x[idx]);
        __alias_inout_buf.Store<uint>(byte_off + 4u, shared_data2x[idx + 1u]);
    }
    else if (base < push_constants.range_end)
    {
        __alias_inout_buf.Store<uint>(byte_off, shared_data2x[idx]);
    }
}
