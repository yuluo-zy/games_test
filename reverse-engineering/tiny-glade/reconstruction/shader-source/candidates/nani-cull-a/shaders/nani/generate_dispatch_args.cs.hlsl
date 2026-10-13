// ==RECONSTRUCTED== shaders/nani/generate_dispatch_args.cs.hlsl | nani-cull-a
// Observation: _nani_generate_dispatch_args.cs.hlsl_cs_main_0.bin (LocalSize
// 64,1,1; b13 instance_list_offset, t14 StructuredBuffer<uint>
// filtered_instance_allocator_buffer, u15 RWStructuredBuffer<uint4>
// __alias_dispatch_args_buffer, u16 RWStructuredBuffer<uint>
// __alias_thread_count_buffer). Body: for each of the 108 draw-list slots clamp
// the allocator's running count to the slot base, then write thread count +
// four dispatch-args records (64/32/32768/65536-thread group variants).
#include "inc/common.hlsl"
#include "draw_list_offsets.inc.hlsl"

StructuredBuffer<uint> filtered_instance_allocator_buffer : register(t14, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u15, space0);
RWStructuredBuffer<uint> __alias_thread_count_buffer : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x < 108u)
    {
        uint base = instance_list_offset_array[dispatchThreadID.x / 4u][dispatchThreadID.x % 4u];
        uint count;
        if (filtered_instance_allocator_buffer[dispatchThreadID.x] >= base)
        {
            count = filtered_instance_allocator_buffer[dispatchThreadID.x] - base;
        }
        else
        {
            count = 0u;
        }
        __alias_thread_count_buffer[dispatchThreadID.x] = count;
        __alias_dispatch_args_buffer[dispatchThreadID.x] = uint4((count + 63u) / 64u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[dispatchThreadID.x + 108u] = uint4((count + 31u) / 32u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[dispatchThreadID.x + 216u] = uint4((count + 32767u) / 32768u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[dispatchThreadID.x + 324u] = uint4(uint(int(count >= 65536u)) / 1u, 1u, 1u, 0u);
    }
}
