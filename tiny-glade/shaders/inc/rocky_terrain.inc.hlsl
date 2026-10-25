// ======================================================================
// inc/rocky_terrain.inc.hlsl —— 岩石地形位移采样
// ======================================================================
// 用途:被 _clearing_*/_generate_grass/_path_pebbles/_rocky_terrain_displace
//       与 terrain_heightmap 一起采样 rocky_terrain 纹理(岩石位移)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/rocky_terrain.inc.hlsl | evidence: consumers _clearing_*/_generate_grass/_path_pebbles/_rocky_terrain_displace sample rocky_terrain texture (rock displacement) alongside terrain_heightmap; raw-per-bin shows Triangle structs belong to rocky_terrain mesh passes, not this file | status: O (texture role); O/I (function names/body Inference)

#ifndef TINYGLADE_INC_ROCKY_TERRAIN_HLSL
#define TINYGLADE_INC_ROCKY_TERRAIN_HLSL

// 与基础高度图相同的 130 单位世界映射。
static const float ROCKY_TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125;

// 采样岩石地形位移(与基础高度图相同的 lerp(-2.5,24) 高度范围,两者可
// 组合用于植被摆放)。
float get_rocky_terrain_height(Texture2D rocky_terrain, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    float t = rocky_terrain.SampleLevel(s, uv, 0.0).x;
    return lerp(-2.5, 24.0, t);
}

// 当点位于岩石(位移)区域而非平地地形时为真,植被摆放用它吸附到岩石表面。
bool is_rocky_terrain(Texture2D rocky_terrain, SamplerState s, float2 world_xz, float threshold)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    return rocky_terrain.SampleLevel(s, uv, 0.0).x > threshold;
}

#endif // TINYGLADE_INC_ROCKY_TERRAIN_HLSL
