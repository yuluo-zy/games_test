// ==RECONSTRUCTED== shaders/inc/terrain_heightmap.inc.hlsl | evidence: spirv-cross (_terrain_lighting_texture/_generate_grass/_critters_*) terrain_heightmap R16F sampled at (world_xz*0.0076923+0.5), lerp(-2.5,24,sampled), normal from GatherRed central difference *26.5/(130/dims); world extent 130 | status: O (mapping/range); O/I (function names Inference)
// MERGE (water-ice review inc-fix): terrain_height() references a consumer-declared
// `terrain_heightmap` global texture. _water_plane_refraction_copy embeds this include but
// binds no terrain texture, so consumers that cannot declare the global define
// TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL before including this file to compile without the
// binding (the pure-parameter get_terrain_height/get_terrain_normal remain available).

#ifndef TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
#define TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL

// Terrain world extent and the sampled height range.
static const float TERRAIN_WORLD_HALF_EXTENT = 65.0;
static const float TERRAIN_HEIGHT_MIN = -2.5;
static const float TERRAIN_HEIGHT_MAX = 24.0;
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125; // 1 / 130

// World xz -> heightmap uv.
float2 terrain_heightmap_uv(float2 world_xz)
{
    return world_xz * TERRAIN_SAMPLE_SCALE + 0.5;
}

// Sample terrain height at a world xz position (returns world y).
float get_terrain_height(Texture2D terrain_heightmap, SamplerState s, float2 world_xz)
{
    float t = terrain_heightmap.SampleLevel(s, terrain_heightmap_uv(world_xz), 0.0).x;
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, t);
}

// World-space terrain height at a world xz position, in the same lerp(-2.5,24)
// height range. Adjudicated inc-fix #3 (RECOVERY_TRACKER #3): inc/find_root.hlsl
// calls `terrain_height(float2)`; this is the definition the stairs closure
// previously provided locally. The global `terrain_heightmap` texture is
// declared (registerless) by each consumer that includes this file; consumers
// that do not use the terrain height leave it unused (dxc DCEs it). Consumers
// that cannot declare the global (e.g. water_plane_refraction_copy, which embeds
// the include but binds no terrain texture) define
// TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL before the include.
#ifndef TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
float terrain_height(float2 world_xz)
{
    return get_terrain_height(terrain_heightmap, sampler_default, world_xz);
}
#endif

// Terrain world-space normal from a heightmap central difference.
float3 get_terrain_normal(Texture2D terrain_heightmap, SamplerState s, float2 world_xz, float2 heightmap_dims)
{
    float4 g = terrain_heightmap.GatherRed(s, terrain_heightmap_uv(world_xz), int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5) * 26.5;
    grad /= (130.0 / max(heightmap_dims, 1e-6));
    float3 right = normalize(float3(1.0, grad.x, 0.0));
    float3 fwd = normalize(float3(0.0, grad.y, 1.0));
    return normalize(cross(fwd, right));
}

#endif // TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
