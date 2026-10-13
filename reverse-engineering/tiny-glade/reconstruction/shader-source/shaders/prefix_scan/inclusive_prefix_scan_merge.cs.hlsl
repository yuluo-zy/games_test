// ==RECONSTRUCTED== shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl | F-D
// Evidence: _prefix_scan_inclusive_prefix_scan_merge.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_inout_buf u13 (RWByteAddressBuffer), subtotal_sum_buf t14
//   (ByteAddressBuffer), LocalSize [512,1,1], entry cs_main.
//   spirv-cross: for each lane, loads its two elements, adds the previous block's
//   subtotal (skipped for group 0), stores the merged values back.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
ByteAddressBuffer subtotal_sum_buf : register(t14, space0);

[numthreads(512, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    uint a = (4u * ((groupThreadID.x * 2u) + (groupID.x * 1024u))) >> 2u;
    uint b = a + 1u;

    uint v0 = __alias_inout_buf.Load<uint>(a * 4 + 0);
    uint v1 = __alias_inout_buf.Load<uint>(b * 4 + 0);
    uint subtotal = subtotal_sum_buf.Load<uint>(((4u * (groupID.x - 1u)) >> 2u) * 4 + 0);

    uint2 sum = uint2(v0, v1) + ((groupID.x == 0u) ? 0u : subtotal).xx;

    __alias_inout_buf.Store<uint>(a * 4 + 0, sum.x);
    __alias_inout_buf.Store<uint>(b * 4 + 0, sum.y);
}
