// ==RECONSTRUCTED== shaders/nani/inc/trimesh.hlsl | nani-cull-a
// Observation: embedded as include in 4 bins (stairs_floor cull + raster).
// Recovered verbatim from the stairs_floor consumer OpMemberName tables:
//   StairsFloorInstanceData { mesh (uint), color_id (uint), pad0..pad2 } (stride 20)
//   NaniTrimeshChunk        { gpu_address (uint64), vertex_index_count (uint),
//                              vertex_byte_offset (uint), bounding_sphere (float4) }
#ifndef NANI_INC_TRIMESH_HLSL
#define NANI_INC_TRIMESH_HLSL

struct StairsFloorInstanceData
{
    uint mesh;
    uint color_id;
    uint pad0;
    uint pad1;
    uint pad2;
};

struct NaniTrimeshChunk
{
    uint64_t gpu_address;
    uint vertex_index_count;
    uint vertex_byte_offset;
    float4 bounding_sphere;
};

#endif // NANI_INC_TRIMESH_HLSL
