// ==RECONSTRUCTED== shaders/inc/lod.hlsl | evidence: raw-per-bin (_nani_cull_and_bucket/_grass_calc_tiles) struct{first_index,index_count,bounding_sphere_radius,pad0}; consumers select between grass/static LODs by distance; lod1_commands_offset in grass globals | status: O/I (struct + function names Inference)

#ifndef TINYGLADE_INC_LOD_HLSL
#define TINYGLADE_INC_LOD_HLSL

// Range of a single mesh LOD inside a combined vertex/index buffer.
struct MeshLod {
    uint first_index;
    uint index_count;
    float bounding_sphere_radius;
    uint pad0;
};

// Pick the lowest LOD level whose distance threshold is not exceeded.
// thresholds are the squared distances at which LOD 0 and LOD 1 kick in.
uint select_lod(float distance_sq, float lod0_distance_sq, float lod1_distance_sq)
{
    if (distance_sq < lod0_distance_sq) {
        return 0u;
    }
    if (distance_sq < lod1_distance_sq) {
        return 1u;
    }
    return 2u;
}

// Grass LOD selector: full-resolution blades vs. a single lod1 draw.
uint select_grass_lod(float distance_sq, float lod1_distance_sq)
{
    return (distance_sq < lod1_distance_sq) ? 0u : 1u;
}

#endif // TINYGLADE_INC_LOD_HLSL
