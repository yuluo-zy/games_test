// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl | target: _rocky_terrain_stairs_stairs_counter.cs.hlsl_cs_main_0.bin | evidence: SPIR-V disasm (StructuredBuffer<DrawIndexedIndirectCommand> draw_commands, AccessChain [0][idx][1] -> instanceCount; RWStructuredBuffer<StairsCounter> stairs_counter[0].count = int(instanceCount); $Globals{int draw_indirect_buffer_offset} UBO; local_size [1,1,1], no entry params) | bindings: draw_commands t13, $Globals b14, stairs_counter u15 | status: O (snapshot instanceCount of the stair draw into the counter); note: inc/draw_indirect_readonly.hlsl StairsCounter is {uint,pad×3} but this bin embeds {int count} — inc discrepancy, flagged; skipped that include to avoid the name clash
// 【文件用途】楼梯生成计数器快照。单线程 CS:把楼梯 indirect draw 命令的
//   instanceCount(即已排布的楼梯/碎石数量)快照到 StairsCounter 中,供 CPU 读取。
// 【入口】cs_main,LocalSize [1,1,1],无入口参数。
// 【绑定】draw_commands t13、$Globals b14、stairs_counter u15。
// 重写说明:手写重建段,仅补充中文注释,代码逐字保留。

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/draw_indirect_readonly.hlsl"
#include "inc/draw_indirect_types.hlsl"

StructuredBuffer<DrawIndexedIndirectCommand> draw_commands : register(t13, space0);
RWStructuredBuffer<StairsCounter> stairs_counter : register(u15, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;    // 楼梯 indirect draw 命令在 draw_commands 中的索引
};

[numthreads(1, 1, 1)]
void cs_main()
{
    // 把楼梯 draw 命令当前的 instanceCount 快照到计数器第 0 项
    stairs_counter[0].count = int(draw_commands[uint(draw_indirect_buffer_offset)].instanceCount);
}
