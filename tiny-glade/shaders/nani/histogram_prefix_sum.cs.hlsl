// ============================================================
// nani/histogram_prefix_sum.cs.hlsl
// 功能:对共享缓冲 shared_data2x[1024] 做展开的 Hillis-Steele 包含前缀和
//       (每线程 2 元素,10 步),再把独占前缀写回字节寻址缓冲。
// 入口:cs_main,512 线程/工作组。
// 绑定:inout_buf (u13, space0)。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/histogram_prefix_sum.cs.hlsl | evidence: raw disasm of
// _nani_histogram_prefix_sum.cs.hlsl_cs_main_0.bin: unrolled Hillis-Steele inclusive scan over
// shared_data2x[1024] (2 elems/lane, 10 steps), exclusive prefix written back | status: O

#line 1 "shaders/nani/histogram_prefix_sum.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/prefix_scan.hlsl"

// 待扫描的字节寻址直方图缓冲,扫描后原位写回。
RWByteAddressBuffer __alias_inout_buf : register(u13, space0);

// 共享扫描缓冲:每线程 2 元素,共 512*2=1024。
groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 local_id : SV_GroupThreadID)
{
    // 每线程负责 2 个连续元素,加载到共享缓冲。
    uint idx0 = local_id.x * 2u;
    uint base = idx0 + group_id.x * 1024u;
    shared_data2x[idx0] = __alias_inout_buf.Load<uint>(base * 4u + 0u);
    shared_data2x[idx0 + 1u] = __alias_inout_buf.Load<uint>((base + 1u) * 4u + 0u);
    GroupMemoryBarrierWithGroupSync();

    // Unrolled Hillis-Steele scan (10 steps for 1024 elements, 2 per lane).
    // 展开的 Hillis-Steele 扫描:1024 元素共 10 步,每步跨度翻倍。
    uint d = (local_id.x >> 0u) << 1u;
    shared_data2x[d + 1u] += shared_data2x[d];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 1u) << 2u;
    shared_data2x[d + 2u + (local_id.x & 1u)] += shared_data2x[d + 1u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 2u) << 3u;
    shared_data2x[d + 4u + (local_id.x & 3u)] += shared_data2x[d + 3u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 3u) << 4u;
    shared_data2x[d + 8u + (local_id.x & 7u)] += shared_data2x[d + 7u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 4u) << 5u;
    shared_data2x[d + 16u + (local_id.x & 15u)] += shared_data2x[d + 15u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 5u) << 6u;
    shared_data2x[d + 32u + (local_id.x & 31u)] += shared_data2x[d + 31u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 6u) << 7u;
    shared_data2x[d + 64u + (local_id.x & 63u)] += shared_data2x[d + 63u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 7u) << 8u;
    shared_data2x[d + 128u + (local_id.x & 127u)] += shared_data2x[d + 127u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 8u) << 9u;
    shared_data2x[d + 256u + (local_id.x & 255u)] += shared_data2x[d + 255u];
    GroupMemoryBarrierWithGroupSync();
    d = (local_id.x >> 9u) << 10u;
    shared_data2x[d + 512u + (local_id.x & 511u)] += shared_data2x[d + 511u];
    GroupMemoryBarrierWithGroupSync();

    // 独占前缀:线程首元素为 0,其余为前一个元素的值。
    uint prefix = (local_id.x > 0u) ? shared_data2x[idx0 - 1u] : 0u;
    __alias_inout_buf.Store<uint>(base * 4u + 0u, prefix);
    __alias_inout_buf.Store<uint>((base + 1u) * 4u + 0u, shared_data2x[idx0]);
}
