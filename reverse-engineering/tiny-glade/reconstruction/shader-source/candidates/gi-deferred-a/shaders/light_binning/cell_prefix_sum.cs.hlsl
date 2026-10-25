// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_sum.cs.hlsl | status: O (Hillis-Steele scan, structure matches disassembly)
// Evidence: _light_binning_cell_prefix_sum.cs.hlsl_cs_main_0.bin (LocalSize 512, entry cs_main).
//   bindings: __alias_inout_buf u13 (RWByteAddressBuffer), __alias_subtotals_buf u14
//   (RWStructuredBuffer<uint>); caps {Shader} only. groupshared shared_data2x[1024].
//   Each lane loads 2 words of a 1024-word per-workgroup block, clamps them at 64,
//   runs an inclusive Hillis-Steele scan (10 steps, barrier after each; the original
//   dxc unrolled the scan, so the steps are written explicitly to reproduce the
//   unrolled opcode stream), then writes back the exclusive prefix of the two words
//   and stores the block total (shared[1023]) to the subtotal buffer at group index.
//   Includes: common, prefix_scan (kept for OpSource parity), grid.inc, consts.
//   Residual: target OpSource/OpString = 9 vs candidate 8 (one extra source string
//   in the original closure; not in the recovered sources list).

#include "inc/common.hlsl"
#include "inc/prefix_scan.hlsl"
#include "grid.inc.hlsl"
#include "inc/consts.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);

groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID, uint3 gl_LocalInvocationID : SV_GroupThreadID)
{
    uint lane = gl_LocalInvocationID.x;
    uint lane2 = 2u * lane;
    // Byte offset of the lane's first element within the whole buffer. The two
    // loads/stores share this byte-address expression (dxc recovers the word
    // index via `>> 2` and reuses the multiply), matching the original disasm.
    uint byte_addr = 4u * (lane2 + (gl_WorkGroupID.x * 1024u));

    uint2 v = min(uint2(__alias_inout_buf.Load<uint>(byte_addr),
                        __alias_inout_buf.Load<uint>(byte_addr + 4u)), uint2(64u, 64u));
    shared_data2x[lane2] = v.x;
    shared_data2x[lane2 + 1u] = v.y;
    GroupMemoryBarrierWithGroupSync();

    // Inclusive Hillis-Steele scan over 1024 elements, 2 per lane, explicitly
    // unrolled to mirror the original dxc output (barrier after every step).
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

    // Exclusive prefix of the two words (shift inclusive by one element).
    uint ex0 = (lane > 0u) ? shared_data2x[lane2 - 1u] : 0u;
    uint ex1 = shared_data2x[lane2];
    __alias_inout_buf.Store<uint>(byte_addr, ex0);
    __alias_inout_buf.Store<uint>(byte_addr + 4u, ex1);

    if (lane == 0u) {
        __alias_subtotals_buf[gl_WorkGroupID.x] = shared_data2x[1023u];
    }
}
