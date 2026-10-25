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

#ifndef TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL
#define TINYGLADE_NANI_CULL_AND_BUCKET_INC_HLSL

#include "inc/frustum.hlsl"
#include "inc/projection.hlsl"
#include "inc/math.hlsl"
#include "inc/cascades.hlsl"
#include "inc/lod.hlsl"

// Affine 3x4 packed transform (12 floats, column-major). The cull-family closures omit
// inc/utils.hlsl, so Affine3Packed is provided here unless a consumer pulls utils.hlsl
// (e.g. via smoke.inc / wooden_plank.inc which carry an extra utils include — flagged drift).
#ifndef TINYGLADE_INC_UTILS_HLSL
struct Affine3Packed
{
    float data[12];
};
#endif

// Shared cull buffers (identical across the cull family).
StructuredBuffer<uint> prev_visibility_bitmask : register(t18, space0);
RWStructuredBuffer<uint> __alias_next_visibility_bitmask : register(u19, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_buffer : register(u14, space0);
RWStructuredBuffer<uint> __alias_filtered_instance_allocator_buffer : register(u15, space0);
RWStructuredBuffer<GpuFilteredInstanceMeta> __alias_filtered_instance_meta_buffer : register(u16, space0);
RWStructuredBuffer<uint> __alias_output_histogram : register(u17, space0);

// Mesh meta decode (folded from candidate A): the instance word packs size_x in the low 16 bits
// (half float) and the mesh atlas-part index in the high 16 bits, whose low bits decode to size_y.
// Bin decodes two halfs + mesh from the instance word (see PBA residual note above).
float2 decode_instance_meta_size(uint meta)
{
    return float2(f16tof32(meta & 0xFFFFu),
                  f16tof32((meta >> 16u) & 0xFFFFu));
}

uint decode_instance_mesh_index(uint meta)
{
    return meta >> 16u;
}

// Test a view-space point/radius against the four side frustum planes.
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

// Mark the instance visible (next-visibility bitmask) and, if it was not visible in the
// previous frame, append it to the filtered list under a histogram bucket.
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
