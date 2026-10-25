// ==RECONSTRUCTED== shaders/inc/draw_indirect_readonly.hlsl | evidence: raw-per-bin (_garden_read_bush_positions_for_cpu/_rocky_terrain_stairs_stairs_counter) DrawIndexedIndirectCommand + StairsCounter names; include-graph direct edge draw_indirect_readonly.hlsl -> draw_indirect_types.hlsl | status: O/I (StairsCounter layout Inference)

#ifndef TINYGLADE_INC_DRAW_INDIRECT_READONLY_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_READONLY_HLSL

#include "draw_indirect_types.hlsl"

// CPU-written counter that reports how many indirect draws a pass recorded.
// Adjudicated inc-fix #14 (RECOVERY_TRACKER #14-f-r): the original bin embeds
// `StairsCounter{int count}` (stride 4) — see _rocky_terrain_stairs_stairs_counter
// OpMemberName evidence — not the earlier {uint,pad0,pad1,pad2} guess.
struct StairsCounter {
    int count;
};

// Read the instanceCount field of the cmd-th DrawIndexedIndirectCommand.
uint load_draw_indexed_indirect_instance_count(
    ByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index)
{
    return draw_commands.Load<uint>((command_base_offset + cmd_index) * 20u + 4u);
}

// Read a full DrawIndexedIndirectCommand from a read-only buffer.
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
