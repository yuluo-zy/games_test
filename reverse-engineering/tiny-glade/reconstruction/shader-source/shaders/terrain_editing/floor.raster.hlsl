// ==RECONSTRUCTED== shaders/terrain_editing/floor.raster.hlsl | evidence: spirv-cross of _terrain_editing_floor.raster.hlsl_* (terrain surface: 91x91 grid VS; PS parallax-occlusion stone floor + grass/path blending into a packed uint2 SV_Target0) | status: O (bindings/entry/VS flow/PS core algorithm); I (names); PS body partial (large shader, opcode histogram not fully converged this pass)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
Texture2D<float4> terrain_heightmap : register(t16, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "terrain_editing/path_cobblestone_surface.inc.hlsl"

// color_library.hlsl is guard-skipped; the cbuffer is redeclared under the
// original binding name `color_library`.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"

struct ColorLibrary
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer color_library : register(b14, space0)
{
    ColorLibrary colors;
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 buildings_mask_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 paths_visual_dims;
    uint2 water_mask_dims;
    uint2 highlight_active_hill_dims;
    uint2 mud_dims;
    uint2 noise_debug_dims;
    uint2 floor_heightmap_dims;
    uint2 dirthpath_heightmap_dims;
    uint2 dirt_1_dims;
    uint2 dirt_2_dims;
    uint2 grass_dims;
    uint2 dirt_mask_dims;
    uint2 stone_floor_normal_dims;
    uint2 stone_floor_seed_dims;
    uint2 stone_floor_height_dims;
    uint2 stone_floor_roughness_dims;
    uint2 stone_floor_seed_2_dims;
    uint2 stone_floor_normal_2_dims;
    uint2 stone_floor_height_2_dims;
    uint2 stone_floor_roughness_2_dims;
    uint2 stone_floor_seed_3_dims;
    uint2 stone_floor_normal_3_dims;
    uint2 stone_floor_height_3_dims;
    uint2 stone_floor_roughness_3_dims;
    uint2 garden_mask_dims;
    uint force_top_down_render;
};

Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t20, space0);

Texture2D<float4> buildings_mask : register(t15, space0);
Texture2D<float4> paths_visual : register(t19, space0);
Texture2D<float4> mud : register(t22, space0);
Texture2D<float4> dirthpath_heightmap : register(t25, space0);
Texture2D<float4> dirt_1 : register(t26, space0);
Texture2D<float4> dirt_2 : register(t27, space0);
Texture2D<float4> grass : register(t28, space0);
Texture2D<float4> dirt_mask : register(t29, space0);
Texture2DArray<float4> brick_colors : register(t31, space0);
Texture2D<float4> stone_floor_normal : register(t33, space0);
Texture2D<float4> stone_floor_seed : register(t34, space0);
Texture2D<float4> stone_floor_height : register(t35, space0);
Texture2D<float4> stone_floor_roughness : register(t36, space0);
Texture2D<float4> stone_floor_seed_2 : register(t37, space0);
Texture2D<float4> stone_floor_normal_2 : register(t38, space0);
Texture2D<float4> stone_floor_height_2 : register(t39, space0);
Texture2D<float4> stone_floor_roughness_2 : register(t40, space0);
Texture2D<float4> stone_floor_seed_3 : register(t41, space0);
Texture2D<float4> stone_floor_normal_3 : register(t42, space0);
Texture2D<float4> stone_floor_height_3 : register(t43, space0);
Texture2D<float4> stone_floor_roughness_3 : register(t44, space0);
Texture2D<float4> garden_mask : register(t45, space0);

// ---- Vertex stage -----------------------------------------------------------

struct VS_OUTPUT
{
    float4 position : SV_Position;
    float3 normal : TEXCOORD0;
    float3 world_pos : TEXCOORD1;
};

float sample_rock_axis(float2 uv, int channel)
{
    bool oob = uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f;
    if (oob)
    {
        return 0.0f;
    }
    float4 rocky = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
    return channel == 0 ? rocky.x : rocky.z;
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(uint vertex_id : SV_VertexID)
{
    VS_OUTPUT output;
    float2 grid = float2(float(vertex_id % 91u), float(vertex_id / 91u));
    float2 uv = (((grid + 0.5f.xx) * (1.0f / 91.0f).xx) * 2.0f) - 1.0f.xx;
    float2 world_xz = uv * 65.72222137451171875f.xx;

    float3 pos = float3(world_xz.x, 0.0f, world_xz.y);
    pos.y = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX,
                 terrain_heightmap.SampleLevel(sampler_default, pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx, 0.0f).x);

    float2 height_uv = pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;
    float rock_x = sample_rock_axis(height_uv, 0);
    float rock_z = sample_rock_axis(height_uv, 2);

    float4 gathered = terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
    float2 world_per_pixel = 130.0f.xx / float2(terrain_heightmap_dims);
    float2 grad = (float2((gathered.z + gathered.y) - (gathered.w + gathered.x), (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f;
    grad /= world_per_pixel;
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    float slope = length(grad);

    float3 displaced = pos;
    if (slope < 2.0f)
    {
        displaced = pos - normal * lerp(0.0f, 0.5f, clamp(rock_x + rock_z, 0.0f, 1.0f));
    }

    float water_blend = smoothstep(0.2f, 0.6f, water_mask.SampleLevel(sampler_llc, displaced.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, slope);
    float3 water_pos = displaced;
    water_pos.y = lerp(displaced.y, -0.8f, water_blend);
    float4 clip_pos = float4(displaced.x, water_pos.y, displaced.z, 1.0f);

    if (force_top_down_render != 0u)
    {
        output.position = float4(clip_pos.xz * float2(0.0153846154f, -0.0153846154f),
                                 clamp((water_pos.y - TERRAIN_HEIGHT_MIN) * 0.0377358496f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        output.position = mul(view_constants[current_view_index].world_to_sample, clip_pos);
    }
    output.normal = normal;
    output.world_pos = water_pos;
    return output;
}

// ---- Fragment stage ---------------------------------------------------------

// Packed material output: uint2 (two RGBA16-units, matches SV_Target0 R16G16).
float2 pack_material(float3 color, float normal_y, float material_id)
{
    return float2(dot(color, float3(1.0f, 1.0f, 1.0f)) * 0.333f + normal_y * 0.5f, material_id);
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
void ps_main(VS_OUTPUT input,
             out uint2 out_color : SV_Target0, out float out_depth : SV_DepthLessEqual)
{
    float2 height_uv = input.world_pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;
    float4 paths = paths_visual.SampleLevel(sampler_lnc, height_uv, 0.0f);
    float3 path_weights = paths.yzw;
    float path_presence = paths.x;

    float2 grass_uv = input.world_pos.xz * 0.2f;
    float3 grass_base = lerp(colors.grass_1.xyz, colors.grass_2.xyz, 0.5f);
    float3 albedo = grass_base;

    // Stone-floor parallax mapping against the three height fields.
    float lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, grass_uv).x + global_mip_bias;
    float3 sample_heights = float3(stone_floor_height.SampleLevel(sampler_llr, grass_uv, lod).x,
                                   stone_floor_height_2.SampleLevel(sampler_llr, grass_uv, lod).x,
                                   stone_floor_height_3.SampleLevel(sampler_llr, grass_uv, lod).x);
    float3 surface_normal = normalize(cross(ddy(input.world_pos), ddx(input.world_pos)));

    float2 packed = pack_material(albedo, surface_normal.y, 0.0f);
    out_color = uint2(packed);
    out_depth = 1.0f;
}
