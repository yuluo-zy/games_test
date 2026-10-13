// ==RECONSTRUCTED== shaders/wall/reset_pass2_inst_counts.cs.hlsl | evidence: raw disasm of
// _wall_reset_pass2_inst_counts.cs.hlsl_cs_main_0.bin: zero instanceCount of the first 4 draw
// commands at draw_indirect_buffer_offset | status: O

#line 1 "shaders/wall/reset_pass2_inst_counts.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/draw_lists.hlsl"

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
};

[numthreads(32, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x < 4u)
    {
        uint cmd_index = uint(draw_indirect_buffer_offset) + thread_id.x;
        DrawIndexedIndirectCommand cmd = __alias_draw_commands[cmd_index];
        cmd.instanceCount = 0u;
        __alias_draw_commands[cmd_index] = cmd;
    }
}
