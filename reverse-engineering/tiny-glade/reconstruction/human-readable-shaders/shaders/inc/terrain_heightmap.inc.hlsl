// ======================================================================
// inc/terrain_heightmap.inc.hlsl —— 地形高度图采样公共头文件
// ======================================================================
// 用途:被 _terrain_lighting_texture / _generate_grass / _critters_* 用于地形
//       高度与法线采样。
// 映射:terrain_heightmap(R16F)按 world_xz*0.0076923+0.5 采样,高度范围
//       lerp(-2.5, 24);法线用 GatherRed 中心差分 * 26.5 / (130/dims);世界范围为 130。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/terrain_heightmap.inc.hlsl | evidence: spirv-cross (_terrain_lighting_texture/_generate_grass/_critters_*) terrain_heightmap R16F sampled at (world_xz*0.0076923+0.5), lerp(-2.5,24,sampled), normal from GatherRed central difference *26.5/(130/dims); world extent 130 | status: O (mapping/range); O/I (function names Inference)
// MERGE (water-ice review inc-fix): terrain_height() references a consumer-declared
// `terrain_heightmap` global texture. _water_plane_refraction_copy embeds this include but
// binds no terrain texture, so consumers that cannot declare the global define
// TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL before including this file to compile without the
// binding (the pure-parameter get_terrain_height/get_terrain_normal remain available).

#ifndef TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
#define TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL

// 地形世界范围与采样的高度范围。
static const float TERRAIN_WORLD_HALF_EXTENT = 65.0;
static const float TERRAIN_HEIGHT_MIN = -2.5;
static const float TERRAIN_HEIGHT_MAX = 24.0;
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125; // 1 / 130

// 世界 xz -> 高度图 uv。
float2 terrain_heightmap_uv(float2 world_xz)
{
    return world_xz * TERRAIN_SAMPLE_SCALE + 0.5;
}

// 在世界 xz 位置采样地形高度(返回世界 y)。
float get_terrain_height(Texture2D terrain_heightmap, SamplerState s, float2 world_xz)
{
    float t = terrain_heightmap.SampleLevel(s, terrain_heightmap_uv(world_xz), 0.0).x;
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, t);
}

// 世界 xz 位置处的地形高度,处于 lerp(-2.5, 24) 高度范围。裁决于 inc-fix #3
// (RECOVERY_TRACKER #3):inc/find_root.hlsl 调用 `terrain_height(float2)`,这就是
// stairs 闭包此前本地提供的定义。全局纹理 `terrain_heightmap` 由每个 include
// 本文件的消费者(无 register)声明;不用的消费者会被 dxc DCE 掉。无法声明该
// 全局的消费者(如只嵌入 include 但不绑定地形纹理的 water_plane_refraction_copy)
// 需在 include 前定义 TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL。
#ifndef TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
float terrain_height(float2 world_xz)
{
    return get_terrain_height(terrain_heightmap, sampler_default, world_xz);
}
#endif

// 由高度图中心差分得到的地形世界空间法线。
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
