// ==RECONSTRUCTED== shaders/water/water_mask_sampler.inc.hlsl | evidence: consumers
// _path_pebbles/_rocky_terrain_stairs_stairs/_water_spawn_reeds/_water_spawn_lilypads/
// _paths_path_mask_visual/_garden_lowpoly_flower inline
// `water_mask.SampleLevel(sampler_default|sampler_llc, world_xz*0.0076923077+0.5, 0).y`
// (uv from world xz via the same 1/130 mapping as the heightmap; green channel is the
// water coverage). include-graph direct edge water_mask_sampler.inc.hlsl -> inc/texture_utils.hlsl
// | status: O (uv mapping/channel from inlined use-sites); I (function name — inlined, not embedded)
//
// Filled in the P1c integration pass (adjudicated inc-fix #5).

#ifndef TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
#define TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL

#include "inc/texture_utils.hlsl"

// Water-mask sampling scale matches the terrain heightmap world mapping (1/130).
static const float WATER_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// Sample the water mask (green channel holds water coverage) at a world xz position.
float get_water_mask(Texture2D<float4> water_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * WATER_MASK_SAMPLE_SCALE + 0.5f;
    return water_mask.SampleLevel(s, uv, 0.0f).y;
}

// True when a world xz position is on water (water-mask coverage above a threshold).
bool is_water(Texture2D<float4> water_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_water_mask(water_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
