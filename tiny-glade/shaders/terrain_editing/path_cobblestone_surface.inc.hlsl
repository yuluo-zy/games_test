// ============================================================
// terrain_editing/path_cobblestone_surface.inc.hlsl
// 功能:路径鹅卵石表面(泥径高度图 + 石地纹理集)的逐像素求值。
// 消费方:floor.raster.hlsl(在泥径高度图/dirt_1/dirt_2/brick_colors 上做
//         路径鹅卵石着色)。无 include 边。
// ============================================================

// ==RECONSTRUCTED== shaders/terrain_editing/path_cobblestone_surface.inc.hlsl | evidence: consumed only by floor.raster.hlsl (path cobblestone shading over dirthpath_heightmap / dirt_1 / dirt_2 / brick_colors); no include edges | status: I (names/body Inference — inlined, no standalone bin)

#ifndef TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL
#define TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL

// 逐像素鹅卵石表面状态,供 floor 着色器用于泥径鹅卵石材质。
struct CobblestoneSurface
{
    float3 albedo;    // 表面反照率
    float3 normal;    // 表面法线(切线空间)
    float roughness;  // 粗糙度
    float height;     // 高度(来自泥径高度图)
};

// 在路径空间坐标处求值鹅卵石表面。
// path_uv 把路径范围映射到 [0,1];三组石材纹理
// (seed/normal/height/roughness)用作灰浆图案。
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
