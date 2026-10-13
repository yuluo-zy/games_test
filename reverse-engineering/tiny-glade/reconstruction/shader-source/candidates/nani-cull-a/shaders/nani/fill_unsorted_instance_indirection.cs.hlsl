// ==RECONSTRUCTED== shaders/nani/fill_unsorted_instance_indirection.cs.hlsl | nani-cull-a
// Observation: _nani_fill_unsorted_instance_indirection.cs.hlsl_cs_main_0.bin
// (LocalSize 32,1,1; b13 instance_list_offset, t14 sorted_instances_buffer,
// t15 thread_count_buffer, u16 __alias_unsorted_instance_atlas_entry_prefix_buffer,
// u17 __alias_instance_indirection_buffer; push_constants {draw_list_index}).
// For each sorted instance of the draw list, atomically count it into the 64
// atlas-entry histogram slot (value % 64) and record value/64 into the
// indirection list at the acquired position.
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math.hlsl"
#include "draw_lists.inc.hlsl"
#include "draw_list_offsets.inc.hlsl"

struct PushConstants
{
    uint draw_list_index;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

StructuredBuffer<uint> sorted_instances_buffer : register(t14, space0);
StructuredBuffer<uint> thread_count_buffer : register(t15, space0);
RWStructuredBuffer<uint> __alias_unsorted_instance_atlas_entry_prefix_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_instance_indirection_buffer : register(u17, space0);

[numthreads(32, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint list_base = instance_list_offset_array[push_constants.draw_list_index / 4u][push_constants.draw_list_index % 4u];
    uint sorted_val = sorted_instances_buffer[dispatchThreadID.x + list_base];
    uint count = thread_count_buffer[push_constants.draw_list_index];
    if (dispatchThreadID.x < count)
    {
        uint prev;
        InterlockedAdd(__alias_unsorted_instance_atlas_entry_prefix_buffer[push_constants.draw_list_index * 64u + sorted_val % 64u], 1u, prev);
        __alias_instance_indirection_buffer[prev] = sorted_val / 64u;
    }}
