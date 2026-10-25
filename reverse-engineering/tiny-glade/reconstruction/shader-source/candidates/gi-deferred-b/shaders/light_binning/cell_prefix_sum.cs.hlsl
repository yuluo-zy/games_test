// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_sum.cs.hlsl (candidate gi-deferred-b)
// Evidence: _light_binning_cell_prefix_sum.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: __alias_inout_buf u13 (RWByteAddressBuffer), __alias_subtotals_buf u14
//   (RWStructuredBuffer<uint>), groupshared shared_data2x[1024], LocalSize [512,1,1],
//   entry cs_main.
//   spirv-cross: two elements per lane from (2*lane + group*1024), each clamped
//   to <= 64 (lights per cell), inclusive Hillis-Steele scan (unrolled 10 steps),
//   exclusive-prefix store back, block total to __alias_subtotals_buf[group].
//   Matches inc/prefix_scan.hlsl PREFIX_SCAN_2_PER_LANE (block_words=512,
//   value_clamp=64) written explicitly with byte-offset indexing to match codegen.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/prefix_scan.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "grid.inc.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    uint e0 = groupThreadID.x * 2u;
    uint byte_idx = (e0 + groupID.x * 1024u) * 4u;

    uint2 v = min(uint2(__alias_inout_buf.Load<uint>(byte_idx + 0u), __alias_inout_buf.Load<uint>(byte_idx + 4u)), uint2(BIN_LIGHT_GRID_MAX_LIGHTS, BIN_LIGHT_GRID_MAX_LIGHTS));
    shared_data2x[e0] = v.x;
    shared_data2x[e0 + 1u] = v.y;

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
    shared_data2x[(((groupThreadID.x >> 9u) << 10u) + 512u) + (groupThreadID.x & 511u)] += shared_data2x[((groupThreadID.x >> 9u) << 10u) + 511u];
    GroupMemoryBarrierWithGroupSync();

    uint ex0 = (groupThreadID.x > 0u) ? shared_data2x[e0 - 1u] : 0u;
    uint ex1 = shared_data2x[e0];

    __alias_inout_buf.Store<uint>(byte_idx + 0u, ex0);
    __alias_inout_buf.Store<uint>(byte_idx + 4u, ex1);

    if (groupThreadID.x == 0u) {
        __alias_subtotals_buf[groupID.x] = shared_data2x[1023];
    }
}
