// ==RECONSTRUCTED== shaders/nani/histogram_prefix_sum.cs.hlsl | nani-cull-a
// Observation: _nani_histogram_prefix_sum.cs.hlsl_cs_main_0.bin (LocalSize
// 512,1,1; u13 RWByteAddressBuffer __alias_inout_buf; groupshared
// shared_data2x[1024]; 11 OpControlBarrier). Two-words-per-lane Hillis-Steele
// block scan over 1024 histogram words; each lane writes back the EXCLUSIVE
// prefix of its two words (block totals are left for group_draw_cmd_prefix_sum
// which consumes the same buffer through a subtotal window).
#include "inc/common.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID, uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    uint idx = groupThreadID.x * 2u;
    uint base = idx + groupID.x * 1024u;
    uint byte_off = base * 4u;

    shared_data2x[idx] = __alias_inout_buf.Load<uint>(byte_off);
    shared_data2x[idx + 1u] = __alias_inout_buf.Load<uint>(byte_off + 4u);
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
    uint ex1 = shared_data2x[idx];
    __alias_inout_buf.Store<uint>(byte_off, ex0);
    __alias_inout_buf.Store<uint>(byte_off + 4u, ex1);
}
