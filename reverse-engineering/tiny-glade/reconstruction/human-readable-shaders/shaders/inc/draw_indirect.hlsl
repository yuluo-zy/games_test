// ============================================================
// inc/draw_indirect.hlsl —— 间接绘制命令写入辅助
// 用途:给 ByteAddressBuffer 里的 DrawIndexedIndirectCommand 原子地
//   递增 instanceCount / 预留实例批次。
//   消费方:_generate_grass、_wall_generate_draw_lists。
// 入口:无(纯头文件;依赖 draw_indirect_types.hlsl)
// 关键算法:instanceCount 字段偏移为 base*20 + 4(InterlockedAdd)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/draw_indirect.hlsl | evidence: raw-per-bin (_generate_grass/_wall_generate_draw_lists) writes DrawIndexedIndirectCommand.instanceCount via InterlockedAdd((offset+cmd)*20+4); include-graph direct edge draw_indirect.hlsl -> draw_indirect_types.hlsl | status: O/I (function names Inference: inlined, not embedded)

#ifndef TINYGLADE_INC_DRAW_INDIRECT_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_HLSL

#include "draw_indirect_types.hlsl"

// DrawIndexedIndirectCommand 在 ByteAddressBuffer 中的字节步长。
static const uint DRAW_INDEXED_INDIRECT_STRIDE_BYTES = 20;

// 递增给定缓冲区里第 cmd 条 DrawIndexedIndirectCommand 的 instanceCount
// 字段(相对 command_base_offset),返回旧值。镜像观察到的
// `InterlockedAdd(base*20 + 4, n, old)` 模式。
uint increment_draw_indexed_indirect_instance_count(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    uint previous;
    draw_commands.InterlockedAdd((command_base_offset + cmd_index) * 20u + 4u, n, previous);
    return previous;
}

// 从第 cmd 条命令原子预留 n 个实例,返回本批次应使用的基准实例号。
uint reserve_draw_indexed_indirect_instances(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    return increment_draw_indexed_indirect_instance_count(draw_commands, command_base_offset, cmd_index, n);
}

#endif // TINYGLADE_INC_DRAW_INDIRECT_HLSL
