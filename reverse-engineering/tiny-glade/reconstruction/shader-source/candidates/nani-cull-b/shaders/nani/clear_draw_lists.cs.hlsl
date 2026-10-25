// ==RECONSTRUCTED== shaders/nani/clear_draw_lists.cs.hlsl | evidence: raw disasm of
// _nani_clear_draw_lists.cs.hlsl_cs_main_0.bin: instance_list_offset b13 UBO + 3 RWStructuredBuffer<uint>
// (draw_list_length u14, instance_indirection_allocator u15, filtered_instance_allocator u16);
// per-thread: offset = instance_list_offset_at(thread/4,thread%4), store into both allocators,
// zero length | status: O (body, bindings, localsize)

#line 1 "shaders/nani/clear_draw_lists.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

RWStructuredBuffer<uint> __alias_draw_list_length_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_allocator_buffer : register(u15, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x < 108u)
    {
        uint packed_index = thread_id.x / 4u;
        uint lane = thread_id.x % 4u;
        uint offset = instance_list_offset_array[packed_index][lane];
        __alias_instance_indirection_allocator_buffer[thread_id.x] = offset;
        __alias_filtered_instance_allocator_buffer[thread_id.x] = offset;
        __alias_draw_list_length_buffer[thread_id.x] = 0u;
    }
}
