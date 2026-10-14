// ============================================================
// inc/draw_indirect_readonly.hlsl —— 间接绘制命令只读访问
// 用途:从只读 ByteAddressBuffer 读取间接绘制命令与计数。
//   消费方:_garden_read_bush_positions_for_cpu、_rocky_terrain_stairs_stairs_counter。
// 入口:无(纯头文件;依赖 draw_indirect_types.hlsl)
// 关键算法:StairsCounter{int count} 步长 4,命令步长 20
// ============================================================
// ==RECONSTRUCTED== shaders/inc/draw_indirect_readonly.hlsl | evidence: raw-per-bin (_garden_read_bush_positions_for_cpu/_rocky_terrain_stairs_stairs_counter) DrawIndexedIndirectCommand + StairsCounter names; include-graph direct edge draw_indirect_readonly.hlsl -> draw_indirect_types.hlsl | status: O/I (StairsCounter layout Inference)

#ifndef TINYGLADE_INC_DRAW_INDIRECT_READONLY_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_READONLY_HLSL

#include "draw_indirect_types.hlsl"

// CPU 写入的计数器,报告某个 pass 记录了多次间接绘制。
// 裁定 inc-fix #14(RECOVERY_TRACKER #14-f-r):原版 bin 内嵌
// `StairsCounter{int count}`(步长 4)——见 _rocky_terrain_stairs_stairs_counter
// OpMemberName 证据——而非更早的 {uint,pad0,pad1,pad2} 猜测。
struct StairsCounter {
    int count;
};

// 读取第 cmd 条 DrawIndexedIndirectCommand 的 instanceCount 字段。
uint load_draw_indexed_indirect_instance_count(
    ByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index)
{
    return draw_commands.Load<uint>((command_base_offset + cmd_index) * 20u + 4u);
}

// 从只读缓冲区读取完整的一条 DrawIndexedIndirectCommand。
DrawIndexedIndirectCommand load_draw_indexed_indirect_command(
    ByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index)
{
    uint byte_offset = (command_base_offset + cmd_index) * 20u;
    DrawIndexedIndirectCommand cmd;
    cmd.count = draw_commands.Load<uint>(byte_offset + 0u);
    cmd.instanceCount = draw_commands.Load<uint>(byte_offset + 4u);
    cmd.firstIndex = draw_commands.Load<uint>(byte_offset + 8u);
    cmd.vertexOffset = draw_commands.Load<uint>(byte_offset + 12u);
    cmd.firstInstance = draw_commands.Load<uint>(byte_offset + 16u);
    return cmd;
}

#endif // TINYGLADE_INC_DRAW_INDIRECT_READONLY_HLSL
