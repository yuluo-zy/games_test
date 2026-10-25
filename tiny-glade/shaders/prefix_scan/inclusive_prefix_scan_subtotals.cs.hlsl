// ==RECONSTRUCTED== shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl
// 中文说明:
//   前缀扫描的块尾部/子块汇总。证据来自 _prefix_scan_inclusive_prefix_scan_subtotals
//   .cs.hlsl_cs_main_0.bin:绑定 input_buf t13(ByteAddressBuffer)、
//   __alias_output_buf u14,groupshared shared_data[1024],LocalSize [512,1,1]。
//   载入块尾元素 ((lane*2048+1023), ((2*lane+1)*1024+1023)),做包含式
//   Hillis-Steele 扫描(10 步),把每线程结果存到输出元素 (2*lane, 2*lane+1)。

#include "inc/common.hlsl"

ByteAddressBuffer input_buf : register(t13, space0);
RWByteAddressBuffer __alias_output_buf : register(u14, space0);

groupshared uint shared_data[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID)
{
    uint e0 = groupThreadID.x * 2u;
    uint e1 = e0 + 1u;

    // 载入两个块的块尾元素(每 2048 元素取一个尾值)。
    shared_data[e0] = input_buf.Load<uint>(((4u * ((groupThreadID.x * 2048u) + 1023u)) >> 2u) * 4 + 0);
    shared_data[e1] = input_buf.Load<uint>(((4u * ((e1 * 1024u) + 1023u)) >> 2u) * 4 + 0);

    // Hillis-Steele 10 步包含扫描,每步后同步(对应原 dxc 展开指令流)。
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

    // 存回每线程两个扫描结果。
    uint out_a = (4u * e0) >> 2u;
    __alias_output_buf.Store<uint>(out_a * 4 + 0, shared_data[e0]);
    __alias_output_buf.Store<uint>((out_a + 1u) * 4 + 0, shared_data[e1]);
}
