// ==RECONSTRUCTED== shaders/inc/rocky_terrain.inc.hlsl | evidence: consumers _clearing_*/_generate_grass/_path_pebbles/_rocky_terrain_displace sample rocky_terrain texture (rock displacement) alongside terrain_heightmap; raw-per-bin shows Triangle structs belong to rocky_terrain mesh passes, not this file | status: O (texture role); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_ROCKY_TERRAIN_HLSL
#define TINYGLADE_INC_ROCKY_TERRAIN_HLSL

// Same 130-unit world mapping as the base heightmap.
static const float ROCKY_TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125;

// Sample the rocky-terrain displacement (same lerp(-2.5,24) height range used
// for the base heightmap, so the two compose for vegetation placement).
float get_rocky_terrain_height(Texture2D rocky_terrain, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    float t = rocky_terrain.SampleLevel(s, uv, 0.0).x;
    return lerp(-2.5, 24.0, t);
}

// True when the point is inside a rocky (displaced) region rather than the
// flat terrain, used by vegetation placement to snap to the rock surface.
bool is_rocky_terrain(Texture2D rocky_terrain, SamplerState s, float2 world_xz, float threshold)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    return rocky_terrain.SampleLevel(s, uv, 0.0).x > threshold;
}

#endif // TINYGLADE_INC_ROCKY_TERRAIN_HLSL
