// ==RECONSTRUCTED== shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl
// 中文说明:
//   前缀扫描的跨块合并阶段。证据来自 _prefix_scan_inclusive_prefix_scan_merge
//   .cs.hlsl_cs_main_0.bin:绑定 __alias_inout_buf u13(RWByteAddressBuffer)、
//   subtotal_sum_buf t14(ByteAddressBuffer),LocalSize [512,1,1]。
//   spirv-cross:每线程载入它的两个元素,加上前一块的块前缀和(组 0 跳过),
//   再存回合并后的值。

#include "inc/common.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
ByteAddressBuffer subtotal_sum_buf : register(t14, space0);

[numthreads(512, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    // 本线程的两个元素的字节偏移(每块 1024 字,每线程 2 字)。
    uint a = (4u * ((groupThreadID.x * 2u) + (groupID.x * 1024u))) >> 2u;
    uint b = a + 1u;

    uint v0 = __alias_inout_buf.Load<uint>(a * 4 + 0);
    uint v1 = __alias_inout_buf.Load<uint>(b * 4 + 0);
    // 前一块的块前缀和(第 0 块没有前缀,加 0)。
    uint subtotal = subtotal_sum_buf.Load<uint>(((4u * (groupID.x - 1u)) >> 2u) * 4 + 0);

    uint2 sum = uint2(v0, v1) + ((groupID.x == 0u) ? 0u : subtotal).xx;

    __alias_inout_buf.Store<uint>(a * 4 + 0, sum.x);
    __alias_inout_buf.Store<uint>(b * 4 + 0, sum.y);
}
