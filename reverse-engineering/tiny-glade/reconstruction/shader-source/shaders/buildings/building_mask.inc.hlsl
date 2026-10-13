// ==RECONSTRUCTED== shaders/buildings/building_mask.inc.hlsl | evidence: include-graph direct edge -> inc/foliage_exclusion.hlsl; consumed by rasterize_buildings / gauss_blur_building_mask / garden spawners; rasterize_buildings inlines the shape tests (circle shape_id 0, oriented box shape_id 1) against the buildings_buffer | status: O (include edge, building-shape roles); O/I (function names/body Inference — inlined)

#ifndef TINYGLADE_BUILDING_MASK_HLSL
#define TINYGLADE_BUILDING_MASK_HLSL

// A building footprint record, laid out exactly as rasterize_buildings reads it
// (32-byte stride). Fields are packed by the CPU-side building list.
struct BuildingShape
{
    float2 center;
    float2 dims;
    float2 orientation;
    int shape_id;
    int pad0;
};

// True when the world position is inside a circular building footprint
// (shape_id 0): radius = dims.x + 0.75.
bool is_building_circle(float2 world_xz, float2 center, float radius)
{
    return distance(world_xz, center) < radius;
}

// True when the world position is inside an oriented box building footprint
// (shape_id 1). orientation is a unit direction; the box half-extents are
// dims scaled by 0.5.
bool is_building_box(float2 world_xz, float2 center, float2 dims, float2 orientation)
{
    float2 local = mul(world_xz - center,
                       float2x2(float2(orientation.x, -orientation.y),
                                float2(orientation.y, orientation.x)));
    float half_w = (dims.x + 1.0f) * 0.5f;
    float half_h = (dims.y + 1.0f) * 0.5f;
    return abs(local.x) <= half_w && abs(local.y) <= half_h;
}

// True when a building footprint covers the world position.
bool is_building(BuildingShape building, float2 world_xz)
{
    if (building.shape_id == 0)
    {
        return is_building_circle(world_xz, building.center, building.dims.x + 0.75f);
    }
    else if (building.shape_id == 1)
    {
        return is_building_box(world_xz, building.center, building.dims, building.orientation);
    }
    return false;
}

// Building-mask scalar for a single footprint: 1.0 inside the footprint, 0.0 outside.
// Inference (adjudicated inc-fix #5 "get_*_mask()" naming; consumers inline the test).
float get_building_mask(BuildingShape building, float2 world_xz)
{
    return is_building(building, world_xz) ? 1.0f : 0.0f;
}

#endif // TINYGLADE_BUILDING_MASK_HLSL
