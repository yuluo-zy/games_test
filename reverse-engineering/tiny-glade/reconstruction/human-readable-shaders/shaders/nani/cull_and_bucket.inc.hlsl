// ==RECONSTRUCTED== shaders/nani/cull_and_bucket.inc.hlsl | evidence: raw disasm of
// _nani_cull_and_bucket*.cs.hlsl bins: shared cull body (frustum side-plane test, next-visibility
// bitmask OR + filtered-instance append with allocator + histogram atomic); OpLine attribution
// pins the side-plane loop / append in this file | status: O (algorithm); I (helper names)
//
// MERGE (wave2 nani-cull-merge, per evidence/render-passes/review/nani-cull/review.md):
//   base = candidate B (shared cull buffers + cull_frustum_side_planes + cull_append_visible),
//   plus candidate A's meta-decode helpers (decode_instance_meta_size / decode_instance_mesh_index).
//   NaniDrawListInfo = { global_instance_offset } (single uint, bin struct 40/41/42) is defined in
//   draw_lists.inc.hlsl (adopt B). A's 4-member NaniDrawListInfo
//   {first_instance,instance_count,mesh_histogram_offset,pad0} is WRONG vs the bin and was dropped.
//   PBA residual (review): the bin's genuine `OpBitcast <ptr PhysicalStorageBuffer> <- uint2` +
//   volatile OpLoad physical instance read is not expressible with the installed dxc; the cull
//   family keeps B's uint2-address + `[[vk::ext_capability(5347)]]` approximation (no Int64), and
//   candidate A's cull_and_bucket.cs.hlsl is retained as the algorithm reference for that residual.
//
// nani 裁剪与分桶共享体:视锥侧平面测试、下一帧可见性位图、按分配器追加可见
// 实例并做直方图原子计数。

#ifndef TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL
#define TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL

#include "inc/frustum.hlsl"
#include "inc/projection.hlsl"
#include "inc/math.hlsl"
#include "inc/cascades.hlsl"
#include "inc/lod.hlsl"

// 仿射 3x4 打包变换(12 个 float,列主序)。裁剪闭包不引入 inc/utils.hlsl,
// 所以这里自带 Affine3Packed,除非消费者已通过 smoke.inc / wooden_plank.inc 引入 utils。
#ifndef TINYGLADE_INC_UTILS_HLSL
struct Affine3Packed
{
    float data[12];
};
#endif

// 裁剪家族共用的缓冲(各 cull 变体一致)。
StructuredBuffer<uint> prev_visibility_bitmask : register(t18, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u19, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWStructuredBuffer<GpuFilteredInstanceMeta> __alias_filtered_instance_meta_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_output_histogram : register(u17, space0);

// 网格元数据解码(合并自 candidate A):实例字低 16 位存 size_x(半精度),
// 高 16 位存图集部件索引,其低位再解码出 size_y。bin 从实例字解码两个半精度 + 网格。
float2 decode_instance_meta_size(uint meta)
{
    return float2(f16tof32(meta & 0xFFFFu),
                  f16tof32((meta >> 16u) & 0xFFFFu));
}

uint decode_instance_mesh_index(uint meta)
{
    return meta >> 16u;
}

// 用视空间点/半径测试视锥四个侧平面,全部通过才可见。
bool cull_frustum_side_planes(Frustum f, float3 center, float radius)
{
    for (uint i = 0u; i < 4u; ++i)
    {
        if (frustum_plane_distance(f.planes[i], center) < -radius)
        {
            return false;
        }
    }
    return true;
}

// 把实例标为可见(写下一帧可见性位图);若上一帧不可见,按直方图桶追加进过滤列表。
void cull_append_visible(uint draw_list_index, uint global_instance_index, uint mesh_id, uint bucket)
{
    uint bit_index = global_instance_index / 32u;
    uint bit_mask = 1u << (global_instance_index % 32u);
    InterlockedOr(__alias_next_visibility_bitmask[bit_index], bit_mask);

    if ((prev_visibility_bitmask[bit_index] & bit_mask) == 0u)
    {
        uint slot;
        InterlockedAdd(__alias_filtered_instance_allocator_buffer[draw_list_index], 1u, slot);
        __alias_filtered_instance_buffer[slot] = global_instance_index;
        GpuFilteredInstanceMeta meta = GpuFilteredInstanceMeta(mesh_id);
        __alias_filtered_instance_meta_buffer[slot] = meta;
        InterlockedAdd(__alias_output_histogram[draw_list_index * 1024u + bucket], 1u);
    }
}

#endif // TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL
