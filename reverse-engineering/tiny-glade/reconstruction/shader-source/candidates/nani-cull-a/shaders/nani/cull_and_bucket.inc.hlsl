// ==RECONSTRUCTED== shaders/nani/cull_and_bucket.inc.hlsl | nani-cull-a
// Observation: embedded as include in 18 nani cull bins (8a7d2f81 hzb variant +
// d8520a92 no-hzb variant per material). Shared pieces of the GPU cull/bucket
// chain: the packed GpuFilteredInstanceMeta layout (two half-floats + mesh
// index packed into the instance word) and the draw-list info record.
//   meta bits [0:16)          -> size_x (half float)
//   meta bits [16:32)         -> mesh atlas part index; its low 16 bits decode
//                                to size_y (half float)
//   effective radius          = size_x * atlas_part.bounding_sphere_radius
#ifndef NANI_CULL_AND_BUCKET_INC_HLSL
#define NANI_CULL_AND_BUCKET_INC_HLSL

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "atlas_part_info.inc.hlsl"
#include "draw_lists.inc.hlsl"

struct NaniDrawListInfo
{
    uint first_instance;
    uint instance_count;
    uint mesh_histogram_offset;
    uint pad0;
};

float2 decode_instance_meta_size(uint meta)
{
    return float2(f16tof32(meta & 0xFFFFu),
                  f16tof32((meta >> 16u) & 0xFFFFu));
}

uint decode_instance_mesh_index(uint meta)
{
    return meta >> 16u;
}

#endif // NANI_CULL_AND_BUCKET_INC_HLSL
