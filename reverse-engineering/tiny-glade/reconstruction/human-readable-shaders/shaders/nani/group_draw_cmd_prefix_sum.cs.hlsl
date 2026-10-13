// ============================================================
// nani/group_draw_cmd_prefix_sum.cs.hlsl
// 功能:对字节寻址的绘制命令直方图区间做 Hillis-Steele 前缀和,
//       结果写回原缓冲;块总和累加到绘制列表长度缓冲并写入小计。
// 入口:cs_main,512 线程/工作组,每线程处理 2 个元素。
// 绑定:inout_buf (u13)、subtotals_buf (u14)、draw_list_length_buffer (u15)。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl | evidence: raw disasm of
// _nani_group_draw_cmd_prefix_sum.cs.hlsl_cs_main_0.bin: Hillis-Steele scan over a byte-address
// histogram range with end masking; block total added to draw-list length and subtotals | status: O

#line 1 "shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/prefix_scan.hlsl"

// 待扫描的字节寻址缓冲(直方图区间),扫描后原位写回。
RWByteAddressBuffer __alias_inout_buf : register(u13, space0);
// 前缀和小计缓冲。
RWStructuredBuffer<uint> __alias_subtotals_buf : register(u14, space0);
// 绘制列表长度缓冲。
RWStructuredBuffer<uint> __alias_draw_list_length_buffer : register(u15, space0);

struct PushConstants
{
    uint range_start;   // 直方图区间起点(元素索引)
    uint range_end;     // 直方图区间终点(元素索引)
    uint subtotals_start; // 小计写入起点
    uint subtotals_end;   // 小计写入终点
    uint draw_list_index; // 绘制列表索引
};

[[vk::push_constant]]
PushConstants push_constants;

// 共享扫描缓冲:每线程 2 个元素,共 512*2=1024。
groupshared uint shared_data2x[1024];

[numthreads(512, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 local_id : SV_GroupThreadID)
{
    // 每线程负责 2 个连续元素:idx0 = 2*local, base 为全局元素起始。
    uint idx0 = local_id.x * 2u;
    uint base = idx0 + push_constants.range_start + (group_id.x * 1024u);
    bool in_range = (base + 1u) < push_constants.range_end;

    // 加载区间内元素;越界部分按 0 处理(尾部掩码)。
    uint2 value;
    if (in_range)
    {
        value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), __alias_inout_buf.Load<uint>((base + 1u) * 4u + 0u));
    }
    else
    {
        if (base < push_constants.range_end)
        {
            value = uint2(__alias_inout_buf.Load<uint>(base * 4u + 0u), 0u);
        }
        else
        {
            value = uint2(0u, 0u);
        }
    }

    // 载入共享缓冲并同步。
    shared_data2x[idx0] = value.x;
    shared_data2x[idx0 + 1u] = value.y;
    GroupMemoryBarrierWithGroupSync();

    // Hillis-Steele 扫描的 10 步(1024 元素,每步翻倍跨度)。
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

    // 独占前缀:前一个元素的值(线程首元素为 0)。
    uint prefix = (local_id.x > 0u) ? shared_data2x[idx0 - 1u] : 0u;
    // 写回前缀;区间内完整对则同时写回第二个元素的前缀。
    if (in_range)
    {
        __alias_inout_buf.Store<uint>(base * 4u + 0u, prefix);
        __alias_inout_buf.Store<uint>((base + 1u) * 4u + 0u, shared_data2x[idx0]);
    }
    else
    {
        if (base < push_constants.range_end)
        {
            __alias_inout_buf.Store<uint>(base * 4u + 0u, prefix);
        }
    }

    // 块总和累加到绘制列表长度,并写入小计(用于跨组前缀)。
    uint total = shared_data2x[1023];
    if (local_id.x == 0u)
    {
        uint previous;
        InterlockedAdd(__alias_draw_list_length_buffer[push_constants.draw_list_index], total, previous);
        uint subtotal_index = 1u + group_id.x + push_constants.subtotals_start;
        if (subtotal_index < push_constants.subtotals_end)
        {
            __alias_subtotals_buf[subtotal_index] = total;
        }
    }
}
