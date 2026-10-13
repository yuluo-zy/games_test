// ==RECONSTRUCTED== shaders/nani/atlas_part_info.inc.hlsl | nani-cull-a
// Observation: embedded as include in 20 nani cull bins. Per-atlas-mesh draw
// record; members recovered verbatim from OpMemberName ({first_index,
// index_count, bounding_sphere_radius, pad0}; stride 16). Consumers index it
// as atlas_part_info[world_subset * 1024 + mesh_index] (byte offsets used by
// prepare/generate draw lists confirm the 16-byte stride).
#ifndef NANI_ATLAS_PART_INFO_INC_HLSL
#define NANI_ATLAS_PART_INFO_INC_HLSL

struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

#endif // NANI_ATLAS_PART_INFO_INC_HLSL
