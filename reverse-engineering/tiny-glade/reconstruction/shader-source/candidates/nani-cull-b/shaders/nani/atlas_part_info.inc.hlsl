// ==RECONSTRUCTED== shaders/nani/atlas_part_info.inc.hlsl | evidence: raw-per-bin OpMemberName
// MeshAtlasPartInfo{first_index,index_count,bounding_sphere_radius,pad0} (16B); generate_draw_lists/
// cull_and_bucket access atlas_part_info as StructuredBuffer<MeshAtlasPartInfo[64]> (ArrayStride
// 16 inner / 1024 outer -> world_subset*64+mesh byte layout, OpName type.StructuredBuffer.) |
// status: O (layout verbatim, buffer-of-array shape)

#ifndef TINYGLADE_NANI_ATLAS_PART_INFO_HLSL
#define TINYGLADE_NANI_ATLAS_PART_INFO_HLSL

// One atlas part (a mesh) per entry. 64 entries per world subset.
struct MeshAtlasPartInfo
{
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

#endif // TINYGLADE_NANI_ATLAS_PART_INFO_HLSL
