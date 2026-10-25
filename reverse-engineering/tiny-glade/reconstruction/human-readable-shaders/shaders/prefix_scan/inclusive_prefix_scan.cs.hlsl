// ==RECONSTRUCTED== shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl
// 中文说明:
//   块内包含前缀扫描(两元素/线程,显式展开)。证据来自
//   _prefix_scan_inclusive_prefix_scan.cs.hlsl_cs_main_0.bin:
//   绑定 __alias_inout_buf u13(RWByteAddressBuffer uint),groupshared
//   shared_data[1024],LocalSize [512,1,1]。每线程从 (2*lane + group*1024) 载入
//   两个元素,做包含式 Hillis-Steele 扫描(10 步),原地存回。扫描与
//   inc/prefix_scan.hlsl PREFIX_SCAN_UINT 一致,但被编译器完全展开 —— 为匹配
//   codegen 这里显式写出。

#include "inc/common.hlsl"

RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

groupshared uint shared_data[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 groupID : SV_GroupID, uint3 groupThreadID : SV_GroupThreadID)
{
    uint e0 = groupThreadID.x * 2u;
    uint a = (4u * (e0 + groupID.x * 1024u)) >> 2u;
    uint b = a + 1u;

    // 载入本线程的两个元素。
    shared_data[e0] = __alias_inout_buf.Load<uint>(a * 4 + 0);
    shared_data[e0 + 1u] = __alias_inout_buf.Load<uint>(b * 4 + 0);

    // Hillis-Steele 10 步包含扫描,每步后同步(对应原 dxc 展开的指令流)。
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

    // 存回本线程的两个扫描结果。
    __alias_inout_buf.Store<uint>(a * 4 + 0, shared_data[e0]);
    __alias_inout_buf.Store<uint>(b * 4 + 0, shared_data[e0 + 1u]);
}
