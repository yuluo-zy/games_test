// ============================================================
// nani/generate_dispatch_args.cs.hlsl
// 功能:为每张绘制列表生成分发参数。把筛选后的实例计数减去列表起始偏移,
//       得到实际实例数 count,再按 4 种线程束宽度(64/32/32768/1)分别
//       计算 dispatch 参数写入 dispatch_args 缓冲。
// 入口:cs_main,64 线程/工作组,每个线程处理一条绘制列表。
// 绑定:filtered_instance_allocator_buffer (t14)、
//       dispatch_args_buffer (u15)、thread_count_buffer (u16)。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/generate_dispatch_args.cs.hlsl | evidence: raw disasm of
// _nani_generate_dispatch_args.cs.hlsl_cs_main_0.bin: per draw list, clamp allocator count to the
// list offset and emit 4 dispatch args (64/32/32768/1 lanes per thread variants) | status: O

#line 1 "shaders/nani/generate_dispatch_args.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "draw_list_offsets.inc.hlsl"

// 每张绘制列表筛选后的候选实例计数。
StructuredBuffer<uint> filtered_instance_allocator_buffer : register(t14, space0);
// 分发参数输出(每列表 4 组 uint4)。
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u15, space0);
// 线程数输出。
RWStructuredBuffer<uint> __alias_thread_count_buffer : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x < 108u)
    {
        // packed_index 定位到列表, lane 选择 4 个偏移槽之一。
        uint packed_index = thread_id.x / 4u;
        uint lane = thread_id.x % 4u;
        uint offset = instance_list_offset_array[packed_index][lane];

        // 候选实例数减去列表偏移,得到该列表的实际实例数(小于偏移则置 0)。
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

        // 线程计数 = 实际实例数。
        __alias_thread_count_buffer[thread_id.x] = count;
        // 4 组分发参数:按不同线程束宽度向上取整得到工作组数。
        __alias_dispatch_args_buffer[thread_id.x] = uint4((count + 63u) / 64u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 108u] = uint4((count + 31u) / 32u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 216u] = uint4((count + 32767u) / 32768u, 1u, 1u, 0u);
        __alias_dispatch_args_buffer[thread_id.x + 324u] = uint4(uint(int(count >= 65536u)) / 1u, 1u, 1u, 0u);
    }
}
