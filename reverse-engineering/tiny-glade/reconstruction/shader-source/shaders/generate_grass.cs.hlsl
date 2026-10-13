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

        // Hammersley scatter point within the tile plus golden-ratio jitter.
        uint hs = ((chosen_grasslet * 17u) % 256u) + 1u;
        float3 scatter_pos = frac(float2(float(hs) * (1.0f / 256.0f), radicalInverse_VdC(hs))
            + frac(GRASS_GOLDEN_JITTER * float((tile_index * 137u) & 65535u) + 0.5f)).xyy;
        float2 tile_center_ws = (scatter_pos.xz + (float2(chosen_xy) - 64.0f)) * (1.0f / 128.0f) * 260.0f;
        float3 blade_pos = float3(tile_center_ws.x, scatter_pos.y, tile_center_ws.y);
        blade_pos.y = 0.0f;

        // Nearest scatter-cell point (3x3) for direction / placement reference.
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

        // Blade scale: gaussian around the nearest scatter point, seeded by the
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

        // Final blade world position, lerped between the scatter point and the
        // nearest cell point, then snapped to terrain.
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
