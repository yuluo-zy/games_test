// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_sum.cs.hlsl
// 中文说明:
//   灯光装箱:每工作组的 1024 字块做包含扫描,写回两字的排他前缀并把块总和
//   写入 subtotals。证据来自 _light_binning_cell_prefix_sum.cs.hlsl_cs_main_0.bin
//   (LocalSize 512)。绑定:__alias_inout_buf u13(RWByteAddressBuffer)、
//   __alias_subtotals_buf u14(RWStructuredBuffer<uint>);caps 仅 {Shader}。
//   groupshared shared_data2x[1024]。每线程载入块内 2 字,clamp 到 64,做 10 步
//   Hillis-Steele 包含扫描(原 dxc 展开扫描,故显式写出以复现展开的指令流),
//   写回两字的排他前缀并把块总和 shared[1023] 存到 subtotal 缓冲的组下标。

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
    // 本线程首元素在整个缓冲区的字节偏移。两个载入/存储共用这个字节地址
    // 表达式(dxc 经 `>> 2` 恢复字下标并复用乘法),与原反汇编一致。
    uint byte_addr = 4u * (lane2 + (gl_WorkGroupID.x * 1024u));

    // 载入 2 字并 clamp 到 64(灯光计数上限)。
    uint2 v = min(uint2(__alias_inout_buf.Load<uint>(byte_addr),
                        __alias_inout_buf.Load<uint>(byte_addr + 4u)), uint2(64u, 64u));
    shared_data2x[lane2] = v.x;
    shared_data2x[lane2 + 1u] = v.y;
    GroupMemoryBarrierWithGroupSync();

    // 1024 元素、每线程 2 字的包含式 Hillis-Steele 扫描,显式展开以镜像原
    // dxc 输出(每步后同步)。
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

    // 两字的排他前缀(把包含扫描整体左移一个元素)。
    uint ex0 = (lane > 0u) ? shared_data2x[lane2 - 1u] : 0u;
    uint ex1 = shared_data2x[lane2];
    __alias_inout_buf.Store<uint>(byte_addr, ex0);
    __alias_inout_buf.Store<uint>(byte_addr + 4u, ex1);

    // 线程 0 把块总和(shared[1023])写入 subtotal 缓冲的组下标。
    if (lane == 0u) {
        __alias_subtotals_buf[gl_WorkGroupID.x] = shared_data2x[1023u];
    }
}
