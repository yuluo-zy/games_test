// ==RECONSTRUCTED== shaders/nani/generate_dispatch_args.cs.hlsl | evidence: raw disasm of
// _nani_generate_dispatch_args.cs.hlsl_cs_main_0.bin: per draw list, clamp allocator count to the
// list offset and emit 4 dispatch args (64/32/32768/1 lanes per thread variants) | status: O

#line 1 "shaders/nani/generate_dispatch_args.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

StructuredBuffer<uint> filtered_instance_allocator_buffer : register(t14, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u15, space0);
RWStructuredBuffer<uint> __alias_thread_count_buffer : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x < 108u)
    {
        uint packed_index = thread_id.x / 4u;
        uint lane = thread_id.x % 4u;
        uint offset = instance_list_offset_array[packed_index][lane];

        uint count;
        do
        {
            if (filtered_instance_allocator_buffer[thread_id.x] >= offset)
            {
                count = filtered_instance_allocator_buffer[thread_id.x] - offset;
                break;
            }
            else
            {
                count = 0u;
                break;
            }
        } while(false);

        __alias_thread_count_buffer[thread_id.x] = count;
        __alias_dispatch_args_buffer[thread_id.x] = uint4((count + 63u) / 64u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 108u] = uint4((count + 31u) / 32u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 216u] = uint4((count + 32767u) / 32768u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 324u] = uint4(uint(int(count >= 65536u)) / 1u, 1u, 1u, 0u);
    }
}
