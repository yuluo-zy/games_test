// ==RECONSTRUCTED== shaders/wall/inc/draw_lists.hlsl | evidence: raw-per-bin OpMemberName of
// _wall_generate_draw_lists / _wall_wall_brick_lod{0,1} bins: InstancedWallData{transform
// (Affine3Packed), sin_deform_y (f4), global_arch_height_vals (f3), seed, source_id,
// wallspace_x_range (f2), flags} — 96B record | status: O (layout verbatim); I (helpers)

#ifndef TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
#define TINYGLADE_WALL_INC_DRAW_LISTS_HLSL

#ifndef TINYGLADE_AFFINE3PACKED
#define TINYGLADE_AFFINE3PACKED
struct Affine3Packed
{
    float data[12];
};
#endif

// Per-instance wall record consumed by the wall draw-list builder and the brick rasters.
struct InstancedWallData
{
    Affine3Packed transform;
    float4 sin_deform_y;
    float3 global_arch_height_vals;
    uint seed;
    uint source_id;
    float2 wallspace_x_range;
    uint flags;
};

#endif // TINYGLADE_WALL_INC_DRAW_LISTS_HLSL
