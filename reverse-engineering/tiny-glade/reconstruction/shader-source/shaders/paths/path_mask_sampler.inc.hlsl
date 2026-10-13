// ==RECONSTRUCTED== shaders/paths/path_mask_sampler.inc.hlsl | evidence: consumers _clearing_*/_generate_grass/_path_pebbles/_ivy_* inline `path_mask.SampleLevel(sampler_llb, world_xz*0.0076923077+0.5, 0).x` (uv from world xz via the same 1/130 mapping as the heightmap); include-graph direct edge path_mask_sampler.inc.hlsl -> texture_utils.hlsl | status: O (uv mapping/channel from inlined use-sites); I (function name — inlined, not embedded)

#ifndef TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL
#define TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL

#include "inc/texture_utils.hlsl"

// Path-mask sampling scale matches the terrain heightmap world mapping (1/130).
static const float PATH_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// Sample the path mask (used coverage) at a world xz position. The path mask is
// sampled with the mipmap/linear-clamp sampler and the red channel holds the
// path coverage.
float get_path_mask(Texture2D<float4> path_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * PATH_MASK_SAMPLE_SCALE + 0.5f;
    return path_mask.SampleLevel(s, uv, 0.0f).x;
}

// True when a world xz position is on a path (path-mask coverage above a
// threshold), used by vegetation/pebble placement to snap onto the path.
bool is_path(Texture2D<float4> path_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_path_mask(path_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL
