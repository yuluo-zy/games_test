// ==RECONSTRUCTED== shaders/nani/clear_draw_lists.cs.hlsl | nani-cull-a
// Observation: _nani_clear_draw_lists.cs.hlsl_cs_main_0.bin (LocalSize 64,1,1;
// b13 instance_list_offset, u14..u16 RWStructuredBuffer<uint> aliases). Body:
// for the first 108 slots re-seed the two allocator buffers with the per-draw-
// list base offset and zero the draw-list length counters.
#include "inc/common.hlsl"
#include "draw_list_offsets.inc.hlsl"

RWStructuredBuffer<uint> __alias_draw_list_length_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u15, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x < 108u)
    {
        uint base = instance_list_offset_array[dispatchThreadID.x / 4u][dispatchThreadID.x % 4u];
        __alias_instance_indirection_allocator_buffer[dispatchThreadID.x] = base;
        __alias_filtered_instance_allocator_buffer[dispatchThreadID.x] = base;
        __alias_draw_list_length_buffer[dispatchThreadID.x] = 0;
    }
}
