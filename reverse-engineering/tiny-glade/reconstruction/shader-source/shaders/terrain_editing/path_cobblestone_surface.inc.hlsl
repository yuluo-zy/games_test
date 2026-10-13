// ==RECONSTRUCTED== shaders/terrain_editing/path_cobblestone_surface.inc.hlsl | evidence: consumed only by floor.raster.hlsl (path cobblestone shading over dirthpath_heightmap / dirt_1 / dirt_2 / brick_colors); no include edges | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL
#define TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL

// Per-pixel cobblestone surface state used by the floor shader for the
// dirt-path cobblestone material.
struct CobblestoneSurface
{
    float3 albedo;
    float3 normal;
    float roughness;
    float height;
};

// Evaluate the cobblestone surface at a path-space coordinate.
// `path_uv` maps the path bounds to [0,1]; the three stone-floor texture sets
// (seed/normal/height/roughness) are used as the mortar pattern.
CobblestoneSurface evaluate_cobblestone_surface(
    float2 path_uv,
    Texture2D dirthpath_heightmap, SamplerState s,
    Texture2D brick_colors,
    Texture2D stone_floor_normal, Texture2D stone_floor_seed,
    Texture2D stone_floor_height, Texture2D stone_floor_roughness,
    float4 brick_color)
{
    CobblestoneSurface surf;
    float2 stone_uv = path_uv;
    surf.height = dirthpath_heightmap.SampleLevel(s, path_uv, 0.0f).x;
    surf.albedo = brick_color.xyz;
    surf.roughness = stone_floor_roughness.SampleLevel(s, stone_uv, 0.0f).x;
    float3 normal_ts = stone_floor_normal.SampleLevel(s, stone_uv, 0.0f).xyz * 2.0f - 1.0f;
    surf.normal = normalize(normal_ts);
    return surf;
}

#endif // TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL
