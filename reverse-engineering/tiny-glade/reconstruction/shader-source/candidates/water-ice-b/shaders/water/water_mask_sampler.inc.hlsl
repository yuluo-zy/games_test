// ==RECONSTRUCTED== shaders/water/water_mask_sampler.inc.hlsl | evidence: consumers
// (_water_x_paths, _water_ice_cracks, _generate_grass, _path_pebbles, _clearing_*, _garden_*,
// _water_read_water_mask) inline `water_mask.SampleLevel(sampler_llb, world_xz*0.0076923077+0.5, 0).x`
// (the same 1/130 world->uv mapping as the terrain heightmap / path mask); include-graph edge
// water_mask_sampler.inc.hlsl -> texture_utils.hlsl | status: O (uv mapping/channel from inlined
// use-sites); I (function name — inlined, not embedded)
//
// Water-mask sampling scale matches the terrain heightmap world mapping (1/130).

#ifndef TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
#define TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL

#include "inc/texture_utils.hlsl"

static const float WATER_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// Sample the water mask (used coverage) at a world xz position. The water mask is
// sampled with the linear-clamp-bilinear sampler; the red channel holds the water coverage.
float get_water_mask(Texture2D<float4> water_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * WATER_MASK_SAMPLE_SCALE + 0.5f;
    return water_mask.SampleLevel(s, uv, 0.0f).x;
}

// True when a world xz position is covered by water (mask coverage above a threshold).
bool is_water(Texture2D<float4> water_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_water_mask(water_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
