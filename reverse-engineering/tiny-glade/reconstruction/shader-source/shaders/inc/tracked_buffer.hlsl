// ==RECONSTRUCTED== shaders/inc/tracked_buffer.hlsl | evidence: OpLine attribution (consumers _rocky_terrain_stairs_stairs, _garden_marching_squares, _dof_autofocus, _dof_analyze, _nani_group_draw_cmd_prefix_sum): AccessChain + CompositeInsert + Store + Load + ShiftRightLogical mix — structured-record writes with a tracked atomic counter (e.g. stairs: InterlockedAdd(draw_commands,1) then Store<float> per record element); include-graph edge tracked_buffer.hlsl -> smooth_max.hlsl | status: I/U (helper names/body Inference — inlined; exact record layout is per-consumer, only the reserve+store pattern is observable)
//
// Helpers for appending packed records into a byte-address buffer while
// tracking the written count with a GPU atomic. Consumers (stairs, mesh
// generators) reserve a slot, then store record elements.

#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL

// Atomically reserve the next slot in a tracked buffer. `counter` holds the
// current count at `counter_offset` bytes; returns the previous value (the
// slot index to write to) and leaves the incremented count for draw dispatch.
uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}

// Store one float element of a record at `base_byte_offset + element_index*4`.
void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif // TINYGLADE_INC_TRACKED_BUFFER_HLSL
