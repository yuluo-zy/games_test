// ============================================================
// generate_grass.cs.hlsl
// 功能:GPU 程序化生成草叶(Compute Shader)。
// 流程:
//   1. 按 bundle 索引取草束,定位本线程对应的草砖(tile)与格点;
//   2. 在格点附近做 3x3 邻域查找最近的草籽点(距离场),得到草籽 id;
//   3. 主题 4(秋天)用 2D simplex 噪声做密度裁剪;
//   4. 由草籽 hash 推导草叶高度/朝向/颜色等参数;
//   5. 读取地形高度、排除路径/岩石/水/花园遮罩与建筑遮罩;
//   6. 打包草叶数据写入草叶缓冲(供后续渲染)。
// 入口:cs_main,64 线程/工作组。
// ============================================================
// PATCHED: modules ss.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/generate_grass.cs.hlsl
// Evidence: _generate_grass.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: sampler_llb s4, sampler_default s9, color_library b13, __alias_draw_commands u14
//   (RWStructuredBuffer<DrawIndexedIndirectCommand>), $Globals b15, bg_terrain_heightmap t16,
//   terrain_heightmap t17, path_mask t19, grass_object_push t20, rocky_terrain t21, water_mask t22,
//   foliage_exclusion_mask t23, buildings_mask t24, garden_mask t25, tile_buffer t26
//   (StructuredBuffer<GrassTilePacked>), bundle_buffer t27 (StructuredBuffer<GrassBundlePacked>),
//   grasslet_bundle_ranges t28 (StructuredBuffer<uint>), __alias_grass_blade_buffer u29
//   (RWStructuredBuffer<GrassBladeDataPacked>), __alias_lod1_commands u30 (RWStructuredBuffer<DrawIndirectCommand>),
//   __alias_atomics u31 (RWStructuredBuffer<uint>), frame_constants set1/b0.
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disasm: procedural grasslet generation (hammersley + golden-ratio jitter,
//   nearest scatter-cell search, gaussian blade scale, slerped blade direction, terrain/rocky/
//   path/water/garden/buildings rejection, value-noise color), atomic append into
//   grass_blade_buffer or LOD1 draw commands.
//   Status: O (structure verbatim); helper names use inc/ (hash/rand01/gaussian_rand/
//   scatter_cell_point/snoise/radicalInverse_VdC/incb_perlin_hash01).

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/simplex_noise.hlsl"

// scatter.hlsl's scatter_point() reads the $Globals member `scatter_grid_dim`
// (declared only by the path-pebble consumer); generate_grass only uses
// scatter_cell_point, so a static stand-in keeps the unused function body valid
// without adding a $Globals member.
static int scatter_grid_dim;
#include "inc/scatter.hlsl"

// color_library.hlsl is guard-skipped; generate_grass binds the original
// flat-member cbuffer `color_library` at set0/b13 (the inc declares a nested
// member at a placeholder register). The file OpString is still recorded.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b13, space0)
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
#include "inc/color_library.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/grass_blade_data.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/atomics.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/math.hlsl"
#include "inc/frustum.hlsl"
#include "inc/lod.hlsl"
#include "grass/grass_common.inc.hlsl"
#include "inc/draw_indirect.hlsl"

Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> bg_terrain_heightmap : register(t16, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> grass_object_push : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
Texture2D<float4> buildings_mask : register(t24, space0);
Texture2D<float4> garden_mask : register(t25, space0);
StructuredBuffer<GrassTilePacked> tile_buffer : register(t26, space0);
StructuredBuffer<GrassBundlePacked> bundle_buffer : register(t27, space0);
StructuredBuffer<uint> grasslet_bundle_ranges : register(t28, space0);
RWStructuredBuffer<GrassBladeDataPacked> __alias_grass_blade_buffer : register(u29, space0);
RWStructuredBuffer<DrawIndirectCommand> __alias_lod1_commands : register(u30, space0);
RWStructuredBuffer<uint> __alias_atomics : register(u31, space0);
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u14, space0);

cbuffer $Globals : register(b15, space0)
{
    int draw_indirect_buffer_offset;
    uint2 bg_terrain_heightmap_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 grass_object_push_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
    uint2 buildings_mask_dims;
    uint2 garden_mask_dims;
    uint lod1_commands_offset;
    uint grass_buffer_size;
    int brush_index;
};

// Golden-ratio jitter vector (2D), as embedded in scatter_cell_point.
static const float2 GRASS_GOLDEN_JITTER = float2(0.75487768650054931640625f, 0.56984031200408935546875f);

// Pack two halves into a uint (f32tof16). Inlined at each use.
uint pack_half2(float2 v)
{
    uint2 h = f32tof16(v);
    return h.x | (h.y << 16u);
}

groupshared GrassBundlePacked group_bundles[64];

[numthreads(64, 1, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint local_index : SV_GroupIndex)
{
    do
    {
        uint range = grasslet_bundle_ranges[group_id.x];
        uint bundle_count = range % 128u;
        uint bundle_start = range / 128u;
        if (local_index < bundle_count)
        {
            group_bundles[local_index] = bundle_buffer[bundle_start + local_index];
        }
        GroupMemoryBarrierWithGroupSync();

        // Find which bundle this thread's grasslet lands in.
        uint2 fallback_xy = uint2(0u, 0u);
        uint grasslet = local_index;
        uint bundle_index = 0u;
        uint2 chosen_xy;
        uint chosen_grasslet;
        for (;;)
        {
            if (bundle_index < bundle_count)
            {
                uint bxy = group_bundles[bundle_index].raw;
                uint2 b_xy = uint2((bxy >> 24u) & 255u, (bxy >> 16u) & 255u);
                uint bundle_blades = bxy & 0xFFFFu;
                uint blade_count = bundle_blades % 128u;
                if (grasslet < blade_count)
                {
                    chosen_grasslet = (bundle_blades / 128u) + grasslet;
                    chosen_xy = b_xy;
                    break;
                }
                grasslet -= blade_count;
                bundle_index++;
                fallback_xy = b_xy;
                continue;
            }
            else
            {
                chosen_grasslet = 0u;
                chosen_xy = fallback_xy;
                break;
            }
        }
        uint tile_index = chosen_xy.x + (chosen_xy.y * 128u);

        GrassTilePacked tile = tile_buffer[tile_index];
        float tile_blades_f = float((tile.raw & 0xFFFFu) % 512u);

        // Hammersley scatter seed_point within the tile plus golden-ratio jitter.
        uint hs = ((chosen_grasslet * 17u) % 256u) + 1u;
        float3 scatter_pos = frac(float2(float(hs) * (1.0f / 256.0f), radicalInverse_VdC(hs))
            + frac(GRASS_GOLDEN_JITTER * float((tile_index * 137u) & 65535u) + 0.5f)).xyy;
        float2 tile_center_ws = (scatter_pos.xz + (float2(chosen_xy) - 64.0f)) * (1.0f / 128.0f) * 260.0f;
        float3 blade_pos = float3(tile_center_ws.x, scatter_pos.y, tile_center_ws.y);
        blade_pos.y = 0.0f;

        // Nearest scatter-cell seed_point (3x3) for direction / placement reference.
        int2 base_cell = int2(blade_pos.xz * 0.4f);
        float3 nearest = 0.0f.xxx;
        int nearest_seed = 0;
        float nearest_dist = 100000.0f;
        for (int cx = base_cell.x - 1; cx <= (base_cell.x + 1); cx++)
        {
            for (int cy = base_cell.y - 1; cy <= (base_cell.y + 1); cy++)
            {
                int seed = cx + (cy * 577);
                float2 cell_point = scatter_cell_point(int2(cx, cy), seed);
                float3 cp = float3(cell_point.x, 0.0f, cell_point.y);
                float d = distance(cp, blade_pos);
                if (d < nearest_dist)
                {
                    nearest_dist = d;
                    nearest = cp;
                    nearest_seed = seed;
                }
            }
        }

        uint blade_seed_full = hash((tile_index * 256u) + chosen_grasslet);
        uint blade_seed = blade_seed_full % 65536u;
        bool theme4 = (glade_theme_id == 4u);
        if (theme4)
        {
            // Sparse-grass rejection: reject blades under dense simplex noise.
            if (snoise(blade_pos.xz * 0.2f) > 0.5f)
            {
                break;
            }
        }

        // Blade scale: gaussian around the nearest scatter seed_point, seeded by the
        // nearest-cell seed and the blade seed.
        float gaussian = gaussian_rand(uint(nearest_seed * 13), blade_seed);
        bool spread_bool = rand01(hash(uint(nearest_seed * 2))) < 0.7f;
        float2 spread_dir;
        if (spread_bool)
        {
            float angle = (1.0f - rand01(hash(uint(nearest_seed)))) * 6.283185482025146484375f;
            spread_dir = normalize(float2(cos(angle), sin(angle)));
        }
        else
        {
            spread_dir = normalize(blade_pos - nearest).xz;
        }
        float2 dir2 = normalize(float2(cos((1.0f - rand01(hash(blade_seed + 69u))) * 6.283185482025146484375f),
                                      sin((1.0f - rand01(hash(blade_seed + 69u))) * 6.283185482025146484375f)));
        float slerp_t = (rand01(hash(blade_seed + 34u)) * 0.2f) + 0.5f;
        float2 spread;
        float ddot = dot(normalize(spread_dir), normalize(dir2));
        if ((ddot > 0.99989998340606689453125f) || (ddot < (-0.99989998340606689453125f)))
        {
            spread = (slerp_t <= 0.5f) ? spread_dir : dir2;
        }
        else
        {
            float a = acos(ddot);
            spread = ((spread_dir * sin((0.5f - slerp_t) * a)) + (dir2 * sin(slerp_t * a))) / sin(a);
        }

        // Final blade world position, lerped between the scatter seed_point and the
        // nearest cell seed_point, then snapped to terrain.
        float3 ground_pos = lerp(blade_pos, nearest, (rand01(hash(uint(nearest_seed + 4235))) - (rand01(hash(blade_seed + 15u)) * 0.9f)));
        float ground_height;
        if ((abs(ground_pos.x) < 65.0f) && (abs(ground_pos.z) < 65.0f))
        {
            ground_height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (ground_pos.xz * (1.0f / 130.0f)) + 0.5f, 0.0f).x);
        }
        else
        {
            float2 bg_uv = (ground_pos.xz * 0.001f) + 0.5f;
            ground_height = lerp(-30.1061992645263671875f, 27.7313995361328125f, bg_terrain_heightmap.SampleLevel(sampler_default, float2(bg_uv.y, 1.0f - bg_uv.x), 0.0f).x);
        }
        ground_pos.y = ground_height;

        float2 mask_uv = (ground_pos.xz * (1.0f / 130.0f)) + 0.5f;
        if (foliage_exclusion_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x > 0.0f)
        {
            break;
        }
        float path_factor = smoothstep(0.1f, 0.2f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x);
        if (path_factor > 0.9f)
        {
            break;
        }
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky_r = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
        float rocky_g = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).z : 0.0f;
        float rocky_b = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((rocky_b + smoothstep(0.0f, 0.5f, rocky_r)) + (rocky_g * 0.4f) > 0.2f)
        {
            break;
        }
        float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
        if (water > 0.25f)
        {
            break;
        }
        if (smoothstep(0.35f, 0.4f, garden_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).y) > 0.0f)
        {
            break;
        }

        float width = (((rand01(hash(blade_seed + 45u)) * 1.5f) + 0.5f) * (spread_bool ? 1.0f : 0.5f)) * (1.0f - max(path_factor, 0.0f));
        float bend = rand01(hash(uint(nearest_seed * 2 - 334))) * 0.3f;
        float scale;
        if (in_bounds)
        {
            float building = buildings_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;
            scale = gaussian * (lerp(1.0f, 0.0f, smoothstep(0.3f, 0.4f, building)) + (smoothstep(0.5f, 0.7f, building) * 0.4f));
        }
        else
        {
            scale = gaussian;
        }
        float width_scale = theme4 ? (width * 2.0f) : width;
        if (scale < 0.01f)
        {
            break;
        }

        // Theme color ramp.
        float3 color_low;
        float3 color_high;
        if (glade_theme_id == 2u)
        {
            color_low = float3(0.314999997615814208984375f, 0.2700000107288360595703125f, 0.1500000059604644775390625f);
            color_high = grass_1.xyz;
        }
        else if (glade_theme_id == 3u)
        {
            color_low = grass_2.xyz * 0.75f;
            color_high = grass_1.xyz * 0.75f;
        }
        else
        {
            color_low = grass_2.xyz;
            color_high = grass_1.xyz;
        }

        // Two-octave value-noise field for color variation.
        float noise1 = (perlin(ground_pos.xz * 0.4f) * 2.0f) - 1.0f;
        float noise2 = (perlin(ground_pos.xz * 0.2f) * 2.0f) - 1.0f;
        float4 grass_push = grass_object_push.SampleLevel(sampler_llb, mask_uv, 0.0f);
        float push_strength = clamp(grass_push.z, 0.0f, 1.0f);
        float2 final_spread;
        float bend_amount;
        if (push_strength > 0.0f)
        {
            float2 push_dir = normalize(grass_push.xy);
            float d2 = dot(normalize(spread), normalize(push_dir));
            float2 blended;
            if ((d2 > 0.99989998340606689453125f) || (d2 < (-0.99989998340606689453125f)))
            {
                blended = (push_strength <= 0.5f) ? spread : push_dir;
            }
            else
            {
                float a2 = acos(d2);
                blended = ((spread * sin((1.0f - push_strength) * a2)) + (push_dir * sin(push_strength * a2))) / sin(a2);
            }
            bend_amount = lerp(bend, 0.9f, push_strength);
            final_spread = blended;
        }
        else
        {
            bend_amount = bend;
            final_spread = spread;
        }
        float rand_jitter = rand01(hash(blade_seed + 34u)) * 10.0f;
        float lod = smoothstep(50.0f - rand_jitter, 40.0f - rand_jitter,
            ((-1.0f) / mul(view_constants[0].world_to_view, float4(ground_pos.x, ground_height, ground_pos.z, 1.0f)).z)
            * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u))));

        // Append the blade (LOD0 draw or LOD1 command).
        uint blade_slot;
        if (0u == uint(floor(lod)))
        {
            uint unused;
            InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, unused);
            uint atomics_prev;
            InterlockedAdd(__alias_atomics[0], 1u, atomics_prev);
            blade_slot = atomics_prev;
        }
        else
        {
            uint unused;
            InterlockedAdd(__alias_lod1_commands[lod1_commands_offset].vertexCount, 3u, unused);
            uint atomics_prev;
            InterlockedAdd(__alias_atomics[1], 0xFFFFFFFFu, atomics_prev);
            blade_slot = atomics_prev - 1u;
        }

        float2 dir_norm = normalize(final_spread);
        float color_t = clamp((noise1 * 0.25f) + (noise2 * 0.35f) + (noise1 * 0.125f)
            + ((rand01(blade_seed_full) - 0.5f) * 0.5f) + 0.2f, 0.0f, 1.0f);
        float3 blade_color = sqrt(clamp(clamp(lerp(color_high, color_low, color_t) * 1.2f, 0.0f, 1.0f), 0.0f, 1.0f));

        GrassBladeDataPacked blade;
        blade.position = ground_pos;
        blade.color = pack_rgb8(blade_color);
        blade.packed0 = (uint(clamp(lod, 0.0f, 1.0f) * 255.0f + 0.5f)
            + (uint(clamp(smoothstep(tile_blades_f, tile_blades_f * 0.9f, float(chosen_grasslet)), 0.0f, 1.0f) * 255.0f + 0.5f) << 8u))
            + (uint(clamp(bend_amount, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
            + (uint(clamp(clamp((dir_norm.x * 0.5f) + 0.5f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
        blade.packed1 = pack_half2(float2(scale * (1.0f - (pow(path_factor, 1.0f) * 0.7f)), 0.0f))
            | (pack_half2(float2(width_scale, 0.0f)) << 16u);
        blade.packed2 = pack_half2(float2(2.0f, 0.0f)) | (pack_half2(float2(dir_norm.y, 0.0f)) << 16u);
        blade.pad = 0u;
        __alias_grass_blade_buffer[blade_slot] = blade;
        break;
    } while (false);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _generate_grass.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct DrawIndexedIndirectCommand
{
    uint count;
    uint instanceCount;
    uint firstIndex;
    uint vertexOffset;
    uint firstInstance;
};

struct GrassTilePacked
{
    uint raw;
};

struct GrassBundlePacked
{
    uint raw;
};

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
};

struct DrawIndirectCommand
{
    uint vertexCount;
    uint instanceCount;
    uint firstVertex;
    uint firstInstance;
};

static float3 v272;

cbuffer color_library : register(b13, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

RWByteAddressBuffer __alias_draw_commands : register(u14, space0);
cbuffer $Globals : register(b15, space0)
{
    int draw_indirect_buffer_offset : packoffset(c0);
    uint2 bg_terrain_heightmap_dims : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 path_mask_dims : packoffset(c2);
    uint2 grass_object_push_dims : packoffset(c2.z);
    uint2 rocky_terrain_dims : packoffset(c3);
    uint2 water_mask_dims : packoffset(c3.z);
    uint2 foliage_exclusion_mask_dims : packoffset(c4);
    uint2 buildings_mask_dims : packoffset(c4.z);
    uint2 garden_mask_dims : packoffset(c5);
    uint lod1_commands_offset : packoffset(c5.z);
    uint grass_buffer_size : packoffset(c5.w);
    int brush_index : packoffset(c6);
};

ByteAddressBuffer tile_buffer : register(t26, space0);
ByteAddressBuffer bundle_buffer : register(t27, space0);
ByteAddressBuffer grasslet_bundle_ranges : register(t28, space0);
RWByteAddressBuffer __alias_grass_blade_buffer : register(u29, space0);
RWByteAddressBuffer __alias_lod1_commands : register(u30, space0);
RWByteAddressBuffer __alias_atomics : register(u31, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> bg_terrain_heightmap : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> grass_object_push : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
Texture2D<float4> buildings_mask : register(t24, space0);
Texture2D<float4> garden_mask : register(t25, space0);

static uint3 gl_WorkGroupID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_WorkGroupID : SV_GroupID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared GrassBundlePacked group_bundles[64];

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        uint bundle_range_raw = grasslet_bundle_ranges.Load<uint>(gl_WorkGroupID.x * 4 + 0);
        uint bundle_count = bundle_range_raw % 128u;
        if (gl_LocalInvocationIndex < bundle_count)
        {
            GrassBundlePacked bundle_packed;
            bundle_packed.raw = bundle_buffer.Load<uint>(((bundle_range_raw / 128u) + gl_LocalInvocationIndex) * 4 + 0);
            GrassBundlePacked bundle = { bundle_packed.raw };
            group_bundles[gl_LocalInvocationIndex] = bundle;
        }
        GroupMemoryBarrierWithGroupSync();
        uint2 last_cell;
        last_cell = uint2(0u, 0u);
        uint v296;
        uint2 cell_xy;
        uint tile_in_bundle;
        uint2 cell_uv;
        uint lane = gl_LocalInvocationIndex;
        uint bundle_idx = 0u;
        for (;;)
        {
            if (bundle_idx < bundle_count)
            {
                cell_xy = uint2((group_bundles[bundle_idx].raw >> 24u) & 255u, (group_bundles[bundle_idx].raw >> 16u) & 255u);
                uint bundle_grass_count = group_bundles[bundle_idx].raw & 65535u;
                uint bundle_lanes = bundle_grass_count % 128u;
                if (lane < bundle_lanes)
                {
                    tile_in_bundle = (bundle_grass_count / 128u) + lane;
                    cell_uv = cell_xy;
                    break;
                }
                v296 = lane - bundle_lanes;
                lane = v296;
                bundle_idx++;
                last_cell = cell_xy;
                continue;
            }
            else
            {
                tile_in_bundle = 0u;
                cell_uv = last_cell;
                break;
            }
        }
        uint tile_flat = cell_uv.x + (cell_uv.y * 128u);
        GrassTilePacked tile;
        tile.raw = tile_buffer.Load<uint>(tile_flat * 4 + 0);
        float tile_hash_rand = float((tile.raw & 65535u) % 512u);
        uint hash = ((tile_in_bundle * 17u) % 256u) + 1u;
        uint v341 = (hash << 16u) | (hash >> 16u);
        uint v346 = ((v341 & 1431655765u) << 1u) | ((v341 & 2863311530u) >> 1u);
        uint v351 = ((v346 & 858993459u) << 2u) | ((v346 & 3435973836u) >> 2u);
        uint v356 = ((v351 & 252645135u) << 4u) | ((v351 & 4042322160u) >> 4u);
        float3 rand3 = frac(float2(float(hash) * 0.00390625f, float(((v356 & 16711935u) << 8u) | ((v356 & 4278255360u) >> 8u)) * 2.3283064365386962890625e-10f).xyy + frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((tile_flat * 137u) & 65535u)) + 0.5f.xx).xyy);
        float2 blade_xz = ((rand3.xz + (float2(cell_uv) - 64.0f.xx)).xy * 0.0078125f.xx).xy * 260.0f;
        float3 blade_pos = float3(blade_xz.x, rand3.y, blade_xz.y);
        uint tile_seed = (tile_flat * 256u) + tile_in_bundle;
        blade_pos.y = 0.0f;
        int2 cell_int = int2(blade_pos.xz * 0.4000000059604644775390625f.xx);
        int cell_x = cell_int.x;
        float3 nearest_pos;
        int nearest_id;
        int cx;
        nearest_pos = 0.0f.xxx;
        nearest_id = 0;
        cx = cell_x - 1;
        float3 v394;
        int v397;
        float v401;
        for (float nearest_dist = 100000.0f; cx <= (cell_x + 1); nearest_pos = v394, nearest_id = v397, cx++, nearest_dist = v401)
        {
            int v406 = cell_int.y;
            int v407 = v406 - 1;
            v401 = nearest_dist;
            v394 = nearest_pos;
            v397 = nearest_id;
            for (int cy = v407; cy <= (v406 + 1); )
            {
                int hash_idx = cy * 577;
                int cell_hash = cx + hash_idx;
                float cy_f = float(cy);
                float2 seed_point = (float2(float(cx), cy_f) * 2.5f).xy + ((frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(cell_hash) & 65535u)) + 0.5f.xx) - 0.5f.xx) * 2.5f);
                float3 seed_point3 = float3(seed_point.x, 0.0f.xxx.y, seed_point.y);
                float dist = distance(seed_point3, blade_pos);
                bool is_nearer = dist < v401;
                bool3 nearer3 = is_nearer.xxx;
                v401 = is_nearer ? dist : v401;
                cy++;
                v394 = float3(nearer3.x ? seed_point3.x : v394.x, nearer3.y ? seed_point3.y : v394.y, nearer3.z ? seed_point3.z : v394.z);
                v397 = is_nearer ? cell_hash : v397;
                continue;
            }
        }
        uint seed1 = tile_seed + (tile_seed << 10u);
        uint v441 = seed1 ^ (seed1 >> 6u);
        uint v443 = v441 + (v441 << 3u);
        uint v445 = v443 ^ (v443 >> 11u);
        uint seed2 = v445 + (v445 << 15u);
        uint grass_seed = seed2 % 65536u;
        bool is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            float2 v454 = blade_pos.xz * 0.20000000298023223876953125f;
            float2 v458 = floor(v454 + dot(v454, 0.3660254180431365966796875f.xx).xx);
            float2 v462 = (v454 - v458) + dot(v458, 0.211324870586395263671875f.xx).xx;
            float v463 = v462.x;
            float v464 = v462.y;
            bool2 v466 = (v463 > v464).xx;
            float2 v467 = float2(v466.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, v466.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
            float4 v469 = v462.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
            float2 v471 = v469.xy - v467;
            float4 v472 = float4(v471.x, v471.y, v469.z, v469.w);
            float2 v473 = fmod(v458, 289.0f.xx);
            float3 v478 = v473.y.xxx + float3(0.0f, v467.y, 1.0f);
            float3 v488 = (fmod(((v478 * 34.0f) + 1.0f.xxx) * v478, 289.0f.xxx) + v473.x.xxx) + float3(0.0f, v467.x, 1.0f);
            float2 v494 = v471.xy;
            float2 v496 = v469.zw;
            float3 v500 = max(0.5f.xxx - float3(dot(v462, v462), dot(v494, v494), dot(v496, v496)), 0.0f.xxx);
            float3 v501 = v500 * v500;
            float3 v505 = frac(fmod(((v488 * 34.0f) + 1.0f.xxx) * v488, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
            float3 v506 = v505 - 1.0f.xxx;
            float3 v508 = abs(v506) - 0.5f.xxx;
            float3 v511 = v506 - floor(v505 + (-0.5f).xxx);
            float3 v523;
            v523.x = (v511.x * v463) + (v508.x * v464);
            float2 v530 = (v511.yz * v472.xz) + (v508.yz * v472.yw);
            if ((130.0f * dot((v501 * v501) * (1.792842864990234375f.xxx - (((v511 * v511) + (v508 * v508)) * 0.8537347316741943359375f)), float3(v523.x, v530.x, v530.y))) > 0.5f)
            {
                break;
            }
        }
        uint v538 = uint(nearest_id * 13);
        uint v540 = v538 + (v538 << 10u);
        uint v542 = v540 ^ (v540 >> 6u);
        uint v544 = v542 + (v542 << 3u);
        uint v546 = v544 ^ (v544 >> 11u);
        uint v556 = grass_seed + (grass_seed << 10u);
        uint v558 = v556 ^ (v556 >> 6u);
        uint v560 = v558 + (v558 << 3u);
        uint v562 = v560 ^ (v560 >> 11u);
        float rand_a = ((asfloat(((v562 + (v562 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 2.0f) - 1.0f;
        float log_term = log(1.0f - (rand_a * rand_a));
        float sq_term = 4.54728412628173828125f + (0.5f * log_term);
        float blade_height = (((asfloat(((v546 + (v546 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 1.5f) + 0.5f) + ((sqrt(sqrt((sq_term * sq_term) - (log_term * 7.142857074737548828125f)) - sq_term) * float(int(sign(rand_a)))) * 0.300000011920928955078125f);
        int v588 = nearest_id * 2;
        uint v589 = uint(v588);
        uint v591 = v589 + (v589 << 10u);
        uint v593 = v591 ^ (v591 >> 6u);
        uint v595 = v593 + (v593 << 3u);
        uint v597 = v595 ^ (v595 >> 11u);
        bool use_random_dir = (asfloat(((v597 + (v597 << 15u)) & 8388607u) | 1065353216u) - 1.0f) < 0.699999988079071044921875f;
        float2 dir_to_nearest;
        if (use_random_dir)
        {
            uint v611 = uint(nearest_id);
            uint v613 = v611 + (v611 << 10u);
            uint v615 = v613 ^ (v613 >> 6u);
            uint v617 = v615 + (v615 << 3u);
            uint v619 = v617 ^ (v617 >> 11u);
            float angle = (1.0f - asfloat(((v619 + (v619 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
            dir_to_nearest = normalize(float2(cos(angle), sin(angle)));
        }
        else
        {
            dir_to_nearest = normalize(blade_pos - nearest_pos).xz;
        }
        uint v635 = uint(int(grass_seed + 69u));
        uint v637 = v635 + (v635 << 10u);
        uint v639 = v637 ^ (v637 >> 6u);
        uint v641 = v639 + (v639 << 3u);
        uint v643 = v641 ^ (v641 >> 11u);
        float angle2 = (1.0f - asfloat(((v643 + (v643 << 15u)) & 8388607u) | 1065353216u)) * 6.280000209808349609375f;
        float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));
        uint v655 = grass_seed + 34u;
        uint v657 = v655 + (v655 << 10u);
        uint v659 = v657 ^ (v657 >> 6u);
        uint v661 = v659 + (v659 << 3u);
        uint v663 = v661 ^ (v661 >> 11u);
        float blend_t = asfloat(((v663 + (v663 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float blend = blend_t * 0.199999988079071044921875f;
        float blend_half = blend + 0.5f;
        float2 blade_dir;
        do
        {
            float v676 = dot(normalize(dir_to_nearest), normalize(dir2));
            bool v682;
            if (!(v676 > 0.99989998340606689453125f))
            {
                v682 = v676 < (-0.99989998340606689453125f);
            }
            else
            {
                v682 = true;
            }
            if (v682)
            {
                if (blend_half <= 0.5f)
                {
                    blade_dir = dir_to_nearest;
                    break;
                }
                blade_dir = dir2;
                break;
            }
            float v688 = acos(v676);
            blade_dir = ((dir_to_nearest * sin((0.5f - blend) * v688)) + (dir2 * sin(blend_half * v688))) / sin(v688).xx;
            break;
        } while(false);
        float gx;
        float gz;
        uint v702 = uint(nearest_id + 4235);
        uint v704 = v702 + (v702 << 10u);
        uint v706 = v704 ^ (v704 >> 6u);
        uint v708 = v706 + (v706 << 3u);
        uint v710 = v708 ^ (v708 >> 11u);
        uint v717 = grass_seed + 15u;
        uint v719 = v717 + (v717 << 10u);
        uint v721 = v719 ^ (v719 >> 6u);
        uint v723 = v721 + (v721 << 3u);
        uint v725 = v723 ^ (v723 >> 11u);
        float3 grow_pos = lerp(blade_pos, nearest_pos, ((asfloat(((v710 + (v710 << 15u)) & 8388607u) | 1065353216u) - 1.0f) - ((asfloat(((v725 + (v725 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.89999997615814208984375f)).xxx);
        float2 grow_xz = grow_pos.xz;
        float grow_height;
        do
        {
            gx = grow_pos.x;
            gz = grow_pos.z;
            if ((int(abs(gx) < 65.0f) & int(abs(gz) < 65.0f)) != 0)
            {
                grow_height = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (grow_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                break;
            }
            else
            {
                float2 v754 = (grow_xz * 0.001000000047497451305389404296875f.xx) + 0.5f.xx;
                grow_height = lerp(-30.1061992645263671875f, 27.7313995361328125f, bg_terrain_heightmap.SampleLevel(sampler_default, float2(v754.y, 1.0f - v754.x), 0.0f).x);
                break;
            }
            break; // unreachable workaround
        } while(false);
        grow_pos.y = grow_height;
        float2 ground_uv = (grow_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        if (foliage_exclusion_mask.SampleLevel(sampler_llb, ground_uv, 0.0f).x > 0.0f)
        {
            break;
        }
        float path_smooth = smoothstep(0.100000001490116119384765625f, 0.20000000298023223876953125f, path_mask.SampleLevel(sampler_llb, ground_uv, 0.0f).x);
        if (path_smooth > 0.89999997615814208984375f)
        {
            break;
        }
        float v800;
        bool v802;
        float rock_h;
        do
        {
            v800 = ground_uv.x;
            v802 = !(v800 < 0.0f);
            bool v806;
            if (v802)
            {
                v806 = v800 > 1.0f;
            }
            else
            {
                v806 = true;
            }
            bool v812;
            if (!v806)
            {
                v812 = ground_uv.y < 0.0f;
            }
            else
            {
                v812 = true;
            }
            bool v818;
            if (!v812)
            {
                v818 = ground_uv.y > 1.0f;
            }
            else
            {
                v818 = true;
            }
            if (v818)
            {
                rock_h = 0.0f;
                break;
            }
            else
            {
                rock_h = rocky_terrain.SampleLevel(sampler_default, ground_uv, 0.0f).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float rock_z;
        do
        {
            bool v833;
            if (v802)
            {
                v833 = v800 > 1.0f;
            }
            else
            {
                v833 = true;
            }
            bool v839;
            if (!v833)
            {
                v839 = ground_uv.y < 0.0f;
            }
            else
            {
                v839 = true;
            }
            bool v845;
            if (!v839)
            {
                v845 = ground_uv.y > 1.0f;
            }
            else
            {
                v845 = true;
            }
            if (v845)
            {
                rock_z = 0.0f;
                break;
            }
            else
            {
                rock_z = rocky_terrain.SampleLevel(sampler_default, ground_uv, 0.0f).z;
                break;
            }
            break; // unreachable workaround
        } while(false);
        float rock_x;
        do
        {
            bool v861;
            if (v802)
            {
                v861 = v800 > 1.0f;
            }
            else
            {
                v861 = true;
            }
            bool v867;
            if (!v861)
            {
                v867 = ground_uv.y < 0.0f;
            }
            else
            {
                v867 = true;
            }
            bool v873;
            if (!v867)
            {
                v873 = ground_uv.y > 1.0f;
            }
            else
            {
                v873 = true;
            }
            if (v873)
            {
                rock_x = 0.0f;
                break;
            }
            else
            {
                rock_x = rocky_terrain.SampleLevel(sampler_default, ground_uv, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        if (((rock_x + smoothstep(0.0f, 0.5f, rock_h)) + (rock_z * 0.4000000059604644775390625f)) > 0.20000000298023223876953125f)
        {
            break;
        }
        float water_y;
        do
        {
            bool v894;
            if (v802)
            {
                v894 = v800 > 1.0f;
            }
            else
            {
                v894 = true;
            }
            bool v900;
            if (!v894)
            {
                v900 = ground_uv.y < 0.0f;
            }
            else
            {
                v900 = true;
            }
            bool v906;
            if (!v900)
            {
                v906 = ground_uv.y > 1.0f;
            }
            else
            {
                v906 = true;
            }
            if (v906)
            {
                water_y = 0.0f;
                break;
            }
            else
            {
                water_y = water_mask.SampleLevel(sampler_default, ground_uv, 0.0f).y;
                break;
            }
            break; // unreachable workaround
        } while(false);
        if (water_y > 0.25f)
        {
            break;
        }
        if (smoothstep(0.3499999940395355224609375f, 0.4000000059604644775390625f, garden_mask.SampleLevel(sampler_llb, ground_uv, 0.0f).y) > 0.0f)
        {
            break;
        }
        uint v927 = grass_seed + 45u;
        uint v929 = v927 + (v927 << 10u);
        uint v931 = v929 ^ (v929 >> 6u);
        uint v933 = v931 + (v931 << 3u);
        uint v935 = v933 ^ (v933 >> 11u);
        float density = ((((asfloat(((v935 + (v935 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 1.5f) + 0.5f) * (use_random_dir ? 1.0f : 0.5f)) * (1.0f - max(path_smooth, 0.0f));
        uint v947 = uint(v588 - 334);
        uint v949 = v947 + (v947 << 10u);
        uint v951 = v949 ^ (v949 >> 6u);
        uint v953 = v951 + (v951 << 3u);
        uint v955 = v953 ^ (v953 >> 11u);
        float color_rand = (asfloat(((v955 + (v955 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f;
        bool v966;
        if (v802)
        {
            v966 = v800 > 1.0f;
        }
        else
        {
            v966 = true;
        }
        bool v972;
        if (!v966)
        {
            v972 = ground_uv.y < 0.0f;
        }
        else
        {
            v972 = true;
        }
        bool v978;
        if (!v972)
        {
            v978 = ground_uv.y > 1.0f;
        }
        else
        {
            v978 = true;
        }
        float final_height;
        if (!v978)
        {
            float4 v985 = buildings_mask.SampleLevel(sampler_default, ground_uv, 0.0f);
            float v986 = v985.y;
            final_height = blade_height * (lerp(1.0f, 0.0f, smoothstep(0.300000011920928955078125f, 0.4000000059604644775390625f, v986)) + (smoothstep(0.5f, 0.699999988079071044921875f, v986) * 0.4000000059604644775390625f));
        }
        else
        {
            final_height = blade_height;
        }
        float v997;
        if (is_theme4)
        {
            v997 = density * 2.0f;
        }
        else
        {
            v997 = density;
        }
        if (final_height < 0.00999999977648258209228515625f)
        {
            break;
        }
        float3 v1018;
        float3 v1019;
        if (frame_constants_glade_theme_id == 2u)
        {
            v1018 = float3(0.314999997615814208984375f, 0.2700000107288360595703125f, 0.1500000059604644775390625f);
            v1019 = color_library_grass_1.xyz;
        }
        else
        {
            float3 v1016;
            float3 v1017;
            if (frame_constants_glade_theme_id == 3u)
            {
                v1016 = color_library_grass_2.xyz * 0.75f;
                v1017 = color_library_grass_1.xyz * 0.75f;
            }
            else
            {
                v1016 = color_library_grass_2.xyz;
                v1017 = color_library_grass_1.xyz;
            }
            v1018 = v1016;
            v1019 = v1017;
        }
        float2 v1020 = grow_pos.xz * 0.4000000059604644775390625f.xx;
        float2 v1021 = v1020 * 1.0f;
        int2 v1023 = int2(floor(v1021));
        float2 v1024 = frac(v1021);
        uint2 v1025 = uint2(v1023);
        uint v1027 = v1025.y;
        uint v1029 = v1027 + (v1027 << 10u);
        uint v1031 = v1029 ^ (v1029 >> 6u);
        uint v1033 = v1031 + (v1031 << 3u);
        uint v1035 = v1033 ^ (v1033 >> 11u);
        uint v1041 = (((v1025.x * 1664525u) + (v1035 + (v1035 << 15u))) + 1013904223u) * 1664525u;
        uint v1043 = v1041 ^ (v1041 >> 11u);
        uint v1046 = v1043 ^ ((v1043 << 7u) & 2636928640u);
        uint v1049 = v1046 ^ ((v1046 << 15u) & 4022730752u);
        uint2 v1057 = uint2(v1023 + int2(1, 0));
        uint v1059 = v1057.y;
        uint v1061 = v1059 + (v1059 << 10u);
        uint v1063 = v1061 ^ (v1061 >> 6u);
        uint v1065 = v1063 + (v1063 << 3u);
        uint v1067 = v1065 ^ (v1065 >> 11u);
        uint v1073 = (((v1057.x * 1664525u) + (v1067 + (v1067 << 15u))) + 1013904223u) * 1664525u;
        uint v1075 = v1073 ^ (v1073 >> 11u);
        uint v1078 = v1075 ^ ((v1075 << 7u) & 2636928640u);
        uint v1081 = v1078 ^ ((v1078 << 15u) & 4022730752u);
        uint2 v1089 = uint2(v1023 + int2(0, 1));
        uint v1091 = v1089.y;
        uint v1093 = v1091 + (v1091 << 10u);
        uint v1095 = v1093 ^ (v1093 >> 6u);
        uint v1097 = v1095 + (v1095 << 3u);
        uint v1099 = v1097 ^ (v1097 >> 11u);
        uint v1105 = (((v1089.x * 1664525u) + (v1099 + (v1099 << 15u))) + 1013904223u) * 1664525u;
        uint v1107 = v1105 ^ (v1105 >> 11u);
        uint v1110 = v1107 ^ ((v1107 << 7u) & 2636928640u);
        uint v1113 = v1110 ^ ((v1110 << 15u) & 4022730752u);
        uint2 v1121 = uint2(v1023 + int2(1, 1));
        uint v1123 = v1121.y;
        uint v1125 = v1123 + (v1123 << 10u);
        uint v1127 = v1125 ^ (v1125 >> 6u);
        uint v1129 = v1127 + (v1127 << 3u);
        uint v1131 = v1129 ^ (v1129 >> 11u);
        uint v1137 = (((v1121.x * 1664525u) + (v1131 + (v1131 << 15u))) + 1013904223u) * 1664525u;
        uint v1139 = v1137 ^ (v1137 >> 11u);
        uint v1142 = v1139 ^ ((v1139 << 7u) & 2636928640u);
        uint v1145 = v1142 ^ ((v1142 << 15u) & 4022730752u);
        float2 v1155 = (v1024 * v1024) * (3.0f.xx - (v1024 * 2.0f));
        float v1156 = v1155.x;
        float v1157 = 1.0f - v1156;
        float v1159 = v1155.y;
        float v1171 = (-1.0f) + (2.0f * ((((1.0f - v1159) * (((asfloat(((v1049 ^ (v1049 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1157) + ((asfloat(((v1081 ^ (v1081 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1156))) + (((asfloat(((v1113 ^ (v1113 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1157) * v1159)) + (((asfloat(((v1145 ^ (v1145 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1156) * v1159)));
        float2 v1173 = v1020 * 0.5f;
        int2 v1175 = int2(floor(v1173));
        float2 v1176 = frac(v1173);
        uint2 v1177 = uint2(v1175);
        uint v1179 = v1177.y;
        uint v1181 = v1179 + (v1179 << 10u);
        uint v1183 = v1181 ^ (v1181 >> 6u);
        uint v1185 = v1183 + (v1183 << 3u);
        uint v1187 = v1185 ^ (v1185 >> 11u);
        uint v1193 = (((v1177.x * 1664525u) + (v1187 + (v1187 << 15u))) + 1013904223u) * 1664525u;
        uint v1195 = v1193 ^ (v1193 >> 11u);
        uint v1198 = v1195 ^ ((v1195 << 7u) & 2636928640u);
        uint v1201 = v1198 ^ ((v1198 << 15u) & 4022730752u);
        uint2 v1209 = uint2(v1175 + int2(1, 0));
        uint v1211 = v1209.y;
        uint v1213 = v1211 + (v1211 << 10u);
        uint v1215 = v1213 ^ (v1213 >> 6u);
        uint v1217 = v1215 + (v1215 << 3u);
        uint v1219 = v1217 ^ (v1217 >> 11u);
        uint v1225 = (((v1209.x * 1664525u) + (v1219 + (v1219 << 15u))) + 1013904223u) * 1664525u;
        uint v1227 = v1225 ^ (v1225 >> 11u);
        uint v1230 = v1227 ^ ((v1227 << 7u) & 2636928640u);
        uint v1233 = v1230 ^ ((v1230 << 15u) & 4022730752u);
        uint2 v1241 = uint2(v1175 + int2(0, 1));
        uint v1243 = v1241.y;
        uint v1245 = v1243 + (v1243 << 10u);
        uint v1247 = v1245 ^ (v1245 >> 6u);
        uint v1249 = v1247 + (v1247 << 3u);
        uint v1251 = v1249 ^ (v1249 >> 11u);
        uint v1257 = (((v1241.x * 1664525u) + (v1251 + (v1251 << 15u))) + 1013904223u) * 1664525u;
        uint v1259 = v1257 ^ (v1257 >> 11u);
        uint v1262 = v1259 ^ ((v1259 << 7u) & 2636928640u);
        uint v1265 = v1262 ^ ((v1262 << 15u) & 4022730752u);
        uint2 v1273 = uint2(v1175 + int2(1, 1));
        uint v1275 = v1273.y;
        uint v1277 = v1275 + (v1275 << 10u);
        uint v1279 = v1277 ^ (v1277 >> 6u);
        uint v1281 = v1279 + (v1279 << 3u);
        uint v1283 = v1281 ^ (v1281 >> 11u);
        uint v1289 = (((v1273.x * 1664525u) + (v1283 + (v1283 << 15u))) + 1013904223u) * 1664525u;
        uint v1291 = v1289 ^ (v1289 >> 11u);
        uint v1294 = v1291 ^ ((v1291 << 7u) & 2636928640u);
        uint v1297 = v1294 ^ ((v1294 << 15u) & 4022730752u);
        float2 v1307 = (v1176 * v1176) * (3.0f.xx - (v1176 * 2.0f));
        float v1308 = v1307.x;
        float v1309 = 1.0f - v1308;
        float v1311 = v1307.y;
        float4 v1340 = grass_object_push.SampleLevel(sampler_llb, ground_uv, 0.0f);
        float v1342 = clamp(v1340.z, 0.0f, 1.0f);
        float v1378;
        float2 v1379;
        if (v1342 > 0.0f)
        {
            float2 v1347 = normalize(v1340.xy);
            float2 v1376;
            do
            {
                float v1352 = dot(normalize(blade_dir), normalize(v1347));
                bool v1358;
                if (!(v1352 > 0.99989998340606689453125f))
                {
                    v1358 = v1352 < (-0.99989998340606689453125f);
                }
                else
                {
                    v1358 = true;
                }
                if (v1358)
                {
                    if (v1342 <= 0.5f)
                    {
                        v1376 = blade_dir;
                        break;
                    }
                    v1376 = v1347;
                    break;
                }
                float v1364 = acos(v1352);
                v1376 = ((blade_dir * sin((1.0f - v1342) * v1364)) + (v1347 * sin(v1342 * v1364))) / sin(v1364).xx;
                break;
            } while(false);
            v1378 = lerp(color_rand, 0.89999997615814208984375f, v1342);
            v1379 = v1376;
        }
        else
        {
            v1378 = color_rand;
            v1379 = blade_dir;
        }
        float v1385 = blend_t * 10.0f;
        float v1399 = smoothstep(50.0f - v1385, 40.0f - v1385, ((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(gx, grow_height, gz, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u))));
        uint v1420;
        if (0u == uint(floor(v1399)))
        {
            uint v1417;
            __alias_draw_commands.InterlockedAdd(uint(draw_indirect_buffer_offset) * 20 + 4, 1u, v1417);
            uint v1419;
            __alias_atomics.InterlockedAdd(0, 1u, v1419);
            v1420 = v1419;
        }
        else
        {
            uint v1409;
            __alias_lod1_commands.InterlockedAdd(lod1_commands_offset * 16 + 0, 3u, v1409);
            uint v1411;
            __alias_atomics.InterlockedAdd(4, 4294967295u, v1411);
            v1420 = v1411 - 1u;
        }
        float2 v1422 = normalize(v1379);
        float3 v1426 = sqrt(clamp(clamp(lerp(v1019, v1018, clamp(((((v1171 * 0.25f) + (((-1.0f) + (2.0f * ((((1.0f - v1311) * (((asfloat(((v1201 ^ (v1201 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1309) + ((asfloat(((v1233 ^ (v1233 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1308))) + (((asfloat(((v1265 ^ (v1265 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1309) * v1311)) + (((asfloat(((v1297 ^ (v1297 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v1308) * v1311)))) * 0.3499999940395355224609375f)) + (v1171 * 0.125f)) + ((asfloat((seed2 & 8388607u) | 1065353216u) - 1.5f) * 0.5f)) + 0.20000000298023223876953125f, 0.0f, 1.0f).xxx) * 1.2000000476837158203125f, 0.0f.xxx, 1.0f.xxx), 0.0f.xxx, 1.0f.xxx));
        GrassBladeDataPacked v1485 = { grow_pos, (uint((clamp(v1426.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v1426.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v1426.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u), ((uint((clamp(v1399, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(smoothstep(tile_hash_rand, tile_hash_rand * 0.89999997615814208984375f, float(tile_in_bundle)), 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v1378, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(clamp((v1422.x * 0.5f) + 0.5f, 0.0f, 1.0f), 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), spvPackHalf2x16(float2(final_height * (1.0f - (pow(path_smooth, 1.0f) * 0.699999988079071044921875f)), 0.0f)) | (spvPackHalf2x16(float2(v997, 0.0f)) << 16u), spvPackHalf2x16(float2(2.0f, 0.0f)) | (spvPackHalf2x16(float2(v1422.y, 0.0f)) << 16u), 0u };
        __alias_grass_blade_buffer.Store<float3>(v1420 * 32 + 0, v1485.position);
        __alias_grass_blade_buffer.Store<uint>(v1420 * 32 + 12, v1485.color);
        __alias_grass_blade_buffer.Store<uint>(v1420 * 32 + 16, v1485.packed0);
        __alias_grass_blade_buffer.Store<uint>(v1420 * 32 + 20, v1485.packed1);
        __alias_grass_blade_buffer.Store<uint>(v1420 * 32 + 24, v1485.packed2);
        __alias_grass_blade_buffer.Store<uint>(v1420 * 32 + 28, v1485.pad);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_WorkGroupID = stage_input.gl_WorkGroupID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
