// ==RECONSTRUCTED== shaders/inc/draw_indirect.hlsl | evidence: raw-per-bin (_generate_grass/_wall_generate_draw_lists) writes DrawIndexedIndirectCommand.instanceCount via InterlockedAdd((offset+cmd)*20+4); include-graph direct edge draw_indirect.hlsl -> draw_indirect_types.hlsl | status: O/I (function names Inference: inlined, not embedded)

#ifndef TINYGLADE_INC_DRAW_INDIRECT_HLSL
#define TINYGLADE_INC_DRAW_INDIRECT_HLSL

#include "draw_indirect_types.hlsl"

// Stride (bytes) of a DrawIndexedIndirectCommand in a ByteAddressBuffer.
static const uint DRAW_INDEXED_INDIRECT_STRIDE_BYTES = 20;

// Increment the instanceCount field of the cmd-th DrawIndexedIndirectCommand
// in the given buffer (relative to command_base_offset). Returns the previous
// value. Mirrors the observed `InterlockedAdd(base*20 + 4, n, old)` pattern.
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

// Atomically reserve n instances from the cmd-th command and return the base
// instance that the caller should use for this batch.
uint reserve_draw_indexed_indirect_instances(
    RWByteAddressBuffer draw_commands,
    uint command_base_offset,
    uint cmd_index,
    uint n)
{
    return increment_draw_indexed_indirect_instance_count(draw_commands, command_base_offset, cmd_index, n);
}

#endif // TINYGLADE_INC_DRAW_INDIRECT_HLSL
