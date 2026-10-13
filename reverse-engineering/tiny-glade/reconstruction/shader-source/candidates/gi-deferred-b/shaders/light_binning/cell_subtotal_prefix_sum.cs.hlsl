// ==RECONSTRUCTED== shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl (candidate gi-deferred-b)
// Evidence: _light_binning_cell_subtotal_prefix_sum.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: push_constants (uint range_end) + __alias_inout_buf u13, groupshared
//   shared_data2x[512], LocalSize [256,1,1], entry cs_main.
//   spirv-cross: single-block inclusive Hillis-Steele scan over the first
//   range_end elements of __alias_inout_buf (two elements per lane, out-of-range
//   lanes read 0), exclusive-prefix store back in place. This is the block
//   subtotal prefix pass of the light-grid prefix chain.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/prefix_scan.hlsl"

struct PushConstants {
    uint range_end;
};

[[vk::push_constant]]
PushConstants push_constants;

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

groupshared uint shared_data2x[512];

[numthreads(256, 1, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID)
{
    uint e0 = groupThreadID.x * 2u;
    uint e1 = e0 + 1u;
    bool full = e1 < push_constants.range_end;

    uint2 v;
    if (full) {
        v = uint2(__alias_inout_buf.Load<uint>(e0 * 4u + 0u), __alias_inout_buf.Load<uint>(e0 * 4u + 4u));
    } else if (e0 < push_constants.range_end) {
        v = uint2(__alias_inout_buf.Load<uint>(e0 * 4u + 0u), 0u);
    } else {
        v = uint2(0u, 0u);
    }
    shared_data2x[e0] = v.x;
    shared_data2x[e1] = v.y;

    GroupMemoryBarrierWithGroupSync();
    shared_data2x[((groupThreadID.x >> 0u) << 1u) + 1u] += shared_data2x[(groupThreadID.x >> 0u) << 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 1u) << 2u) + 2u) + (groupThreadID.x & 1u)] += shared_data2x[((groupThreadID.x >> 1u) << 2u) + 1u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 2u) << 3u) + 4u) + (groupThreadID.x & 3u)] += shared_data2x[((groupThreadID.x >> 2u) << 3u) + 3u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 3u) << 4u) + 8u) + (groupThreadID.x & 7u)] += shared_data2x[((groupThreadID.x >> 3u) << 4u) + 7u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 4u) << 5u) + 16u) + (groupThreadID.x & 15u)] += shared_data2x[((groupThreadID.x >> 4u) << 5u) + 15u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 5u) << 6u) + 32u) + (groupThreadID.x & 31u)] += shared_data2x[((groupThreadID.x >> 5u) << 6u) + 31u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 6u) << 7u) + 64u) + (groupThreadID.x & 63u)] += shared_data2x[((groupThreadID.x >> 6u) << 7u) + 63u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 7u) << 8u) + 128u) + (groupThreadID.x & 127u)] += shared_data2x[((groupThreadID.x >> 7u) << 8u) + 127u];
    GroupMemoryBarrierWithGroupSync();
    shared_data2x[(((groupThreadID.x >> 8u) << 9u) + 256u) + (groupThreadID.x & 255u)] += shared_data2x[((groupThreadID.x >> 8u) << 9u) + 255u];
    GroupMemoryBarrierWithGroupSync();

    uint ex0 = (groupThreadID.x > 0u) ? shared_data2x[e0 - 1u] : 0u;
    uint ex1 = shared_data2x[e0];

    if (full) {
        __alias_inout_buf.Store<uint>(e0 * 4u + 0u, ex0);
        __alias_inout_buf.Store<uint>(e0 * 4u + 4u, ex1);
    } else if (e0 < push_constants.range_end) {
        __alias_inout_buf.Store<uint>(e0 * 4u + 0u, ex0);
    }
}
