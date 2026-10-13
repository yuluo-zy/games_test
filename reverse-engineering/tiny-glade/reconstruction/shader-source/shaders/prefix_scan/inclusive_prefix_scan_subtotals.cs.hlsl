// ==RECONSTRUCTED== shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl | F-D
// Evidence: _prefix_scan_inclusive_prefix_scan_subtotals.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: input_buf t13 (ByteAddressBuffer), __alias_output_buf u14, groupshared
//   shared_data[1024], LocalSize [512,1,1], entry cs_main.
//   spirv-cross: loads block-tail elements ((lane*2048+1023), ((2*lane+1)*1024+1023)),
//   inclusive Hillis-Steele scan (unrolled 10 steps), stores per-lane results at output
//   elements (2*lane, 2*lane+1).
// Status: O (structure from disassembly; scan matches inc/prefix_scan.hlsl but unrolled).

#include "inc/common.hlsl"

ByteAddressBuffer input_buf : register(t13, space0);
RWByteAddressBuffer __alias_output_buf : register(u14, space0);

groupshared uint shared_data[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID)
{
    uint e0 = groupThreadID.x * 2u;
    uint e1 = e0 + 1u;

    shared_data[e0] = input_buf.Load<uint>(((4u * ((groupThreadID.x * 2048u) + 1023u)) >> 2u) * 4 + 0);
    shared_data[e1] = input_buf.Load<uint>(((4u * ((e1 * 1024u) + 1023u)) >> 2u) * 4 + 0);

    GroupMemoryBarrierWithGroupSync();
    shared_data[((groupThreadID.x >> 0u) << 1u) + 1u] += shared_data[(groupThreadID.x >> 0u) << 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 1u) << 2u) + 2u) + (groupThreadID.x & 1u)] += shared_data[((groupThreadID.x >> 1u) << 2u) + 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 2u) << 3u) + 4u) + (groupThreadID.x & 3u)] += shared_data[((groupThreadID.x >> 2u) << 3u) + 3u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 3u) << 4u) + 8u) + (groupThreadID.x & 7u)] += shared_data[((groupThreadID.x >> 3u) << 4u) + 7u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 4u) << 5u) + 16u) + (groupThreadID.x & 15u)] += shared_data[((groupThreadID.x >> 4u) << 5u) + 15u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 5u) << 6u) + 32u) + (groupThreadID.x & 31u)] += shared_data[((groupThreadID.x >> 5u) << 6u) + 31u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 6u) << 7u) + 64u) + (groupThreadID.x & 63u)] += shared_data[((groupThreadID.x >> 6u) << 7u) + 63u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 7u) << 8u) + 128u) + (groupThreadID.x & 127u)] += shared_data[((groupThreadID.x >> 7u) << 8u) + 127u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 8u) << 9u) + 256u) + (groupThreadID.x & 255u)] += shared_data[((groupThreadID.x >> 8u) << 9u) + 255u];
    GroupMemoryBarrierWithGroupSync();
    shared_data[(((groupThreadID.x >> 9u) << 10u) + 512u) + (groupThreadID.x & 511u)] += shared_data[((groupThreadID.x >> 9u) << 10u) + 511u];
    GroupMemoryBarrierWithGroupSync();

    uint out_a = (4u * e0) >> 2u;
    __alias_output_buf.Store<uint>(out_a * 4 + 0, shared_data[e0]);
    __alias_output_buf.Store<uint>((out_a + 1u) * 4 + 0, shared_data[e1]);
}
