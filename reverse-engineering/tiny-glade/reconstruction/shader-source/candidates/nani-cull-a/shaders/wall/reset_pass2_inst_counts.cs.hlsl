// ==RECONSTRUCTED== shaders/wall/reset_pass2_inst_counts.cs.hlsl | nani-cull-a
// Observation: _wall_reset_pass2_inst_counts.cs.hlsl_cs_main_0.bin (LocalSize
// 32,1,1; u13 RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands;
// b14 $Globals { int draw_indirect_buffer_offset }). Before the round-2
// (HZB) wall cull, zeroes instanceCount for the 4 brick-draw commands (the
// round-1 survivors stay untouched).
#include "inc/common.hlsl"
#include "inc/draw_indirect.hlsl"
#include "wall/inc/draw_lists.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);

[numthreads(32, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x < 4u)
    {
        uint cmd_idx = uint(draw_indirect_buffer_offset) + dispatchThreadID.x;
        DrawIndexedIndirectCommand cmd;
        cmd.count = __alias_draw_commands[cmd_idx].count;
        cmd.instanceCount = 0u;
        cmd.firstIndex = __alias_draw_commands[cmd_idx].firstIndex;
        cmd.vertexOffset = __alias_draw_commands[cmd_idx].vertexOffset;
        cmd.firstInstance = __alias_draw_commands[cmd_idx].firstInstance;
        __alias_draw_commands[cmd_idx] = cmd;
    }
}
