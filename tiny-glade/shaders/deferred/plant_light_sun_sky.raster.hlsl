// ==RECONSTRUCTED== shaders/deferred/plant_light_sun_sky.raster.hlsl | status: O (structure from disassembly); I (flag name; $Globals member names verbatim)

// 中文说明:
//   植物植物 plant 材质的延迟"太阳+天空"光照合成(全屏 pass)。包含:
//   - 由 gbuffer+深度重建世界位置/法线(RGB8 法线解码,gbuffer.x 低 24 位);
//   - 太阳:能量守恒 GGX 漫反射 + Cook-Torrance 镜面 * sun_color * NdotL;
//   - 天空环境:间接辐射 3-slab 空间解析的 SH 带 1 求值;
//   - ReSTIR 灯笼光:reservoir 收集 + 流式采样;
//   - 网格灯:light_indirection_grid / grid_light_indices 迭代;
//   - 级联阴影 / 屏幕空间阴影 / ssgi 合成。
//   两个变体由 DEFERRED_FULL_LIGHTING 门控(b903 全 / f0ad 精简);ps_main
//   主体为 spirv-cross 转译(行为精确;内部临时变量已全量语义化命名)。
// MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both
// b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's octahedral-normal and SV_Target
// alpha=2.0 merge deltas are REJECTED on bin evidence (the PS decodes the gbuffer normal as
// RGB8 normalize of gbuffer.x low bytes and writes alpha 0.0 — both match B as-is).

#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
struct ColorLibrary {
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

cbuffer color_library : register(b23, space0) {
    ColorLibrary color_library;
};
#include "light_sun_sky_prelude.inc.hlsl"
#include "light_sun_sky.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/sh.hlsl"
#include "inc/shadow.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/utils.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/material/eval/plant_material.inc.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"
#include "light_binning/grid.inc.hlsl"

#ifndef DEFERRED_FULL_LIGHTING
#define DEFERRED_FULL_LIGHTING 1
#endif

cbuffer $Globals : register(b13, space0)
{
    uint2 gbuffer_tex_dims;
    uint2 depth_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint taa_history_valid;
    uint2 blue_noise_texture_dims;
    uint2 ssgi_tex_dims;
    uint2 screen_shadows_tex_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 highlight_active_hill_dims;
    float time;
    uint lantern_count;
    uint2 indirect_radiance_tex_dims;
    uint2 rdi_reservoir_tex_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
};

Texture2D<uint4> gbuffer_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> ssgi_tex : register(t19, space0);
Texture2D<float4> screen_shadows_tex : register(t20, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


#if DEFERRED_FULL_LIGHTING
// b903: pixel stage lifted from _deferred_plant_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.
static float s_scratch_f;
static uint s_scratch_u;
static const float4 zero4x3[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState res_stream_zero = { 0.0f, 0.0f };
static const Reservoir1spp reservoir_zero = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 pixel_xy = int2(input.position.xy);
    uint2 pixel_uv = uint2(pixel_xy);
    float4 blue_noise = ((blue_noise_texture.Load(int3((pixel_uv + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 gbuffer = gbuffer_tex.Load(int3(pixel_uv, 0u));
    float4 depth_sample = depth_tex.Load(int3(pixel_uv, 0u));
    float depth = depth_sample.x;
    uint gbuffer_x = gbuffer.x;
    float octa_a = float((gbuffer_x >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    uint gbuffer_y = gbuffer.y;
    float octa_b = float((gbuffer_y >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 n_fine = (float4(float(gbuffer_x & 255u) * 0.0039215688593685626983642578125f, float((gbuffer_x >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gbuffer_x >> 16u) & 255u) * 0.0039215688593685626983642578125f, octa_a).xyz * 2.0f) - 1.0f.xxx;
    float3 albedo_enc = float4(float(gbuffer_y & 255u) * 0.0039215688593685626983642578125f, float((gbuffer_y >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gbuffer_y >> 16u) & 255u) * 0.0039215688593685626983642578125f, octa_b).xyz;
    float3 albedo = albedo_enc * albedo_enc;
    float3 sun_dir = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 depth_dims_f = float2(depth_tex_dims);
    float2 uv01 = (float2(pixel_xy) + 0.5f.xx) / depth_dims_f;
    float2 ndc_xy = (uv01 - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc_xy.x;
    float4 view_pos_h = float4(ndc_x, ndc_xy.y, depth, 1.0f);
    float4 world_pos_h = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, view_pos_h));
    float3 view_dir = -normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(ndc_x, ndc_xy.y, 0.0f, 1.0f))).xyz);
    float3 world_pos = world_pos_h.xyz / world_pos_h.w.xxx;
    float ssgi_mix = clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, uv01, 0.0f).x, 0.0f, 1.0f);
    bool3 override_mask = ((render_overrides & 1u) != 0u).xxx;
    float3 albedo_masked = float3(override_mask.x ? 0.5f.xxx.x : albedo.x, override_mask.y ? 0.5f.xxx.y : albedo.y, override_mask.z ? 0.5f.xxx.z : albedo.z);
    float shadow_vis = 1.0f - screen_shadows_tex.Load(int3(pixel_uv, 0u)).x;
    float NdotL = dot(n_fine, sun_dir);
    float3 specular;
    do
    {
        bool spec_cull;
        if (!(NdotL <= 0.0f))
        {
            spec_cull = dot(n_fine, view_dir) <= 0.0f;
        }
        else
        {
            spec_cull = true;
        }
        if (spec_cull)
        {
            specular = 0.0f.xxx;
            break;
        }
        float3 half_vec = normalize(view_dir + sun_dir);
        float NdotH = dot(n_fine, half_vec);
        float HdotL = max(0.0f, 1.0f - dot(half_vec, sun_dir));
        float HdotL2 = HdotL * HdotL;
        float NdotV = dot(n_fine, view_dir);
        float ggx_a = ((NdotH * NdotH) * (-0.9375f)) + 1.0f;
        specular = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((HdotL2 * HdotL2) * HdotL).xxx) * (((2.0f * NdotL) * NdotV) / ((NdotL * sqrt((((NdotV * (-0.0625f)) + NdotV) * NdotV) + 0.0625f)) + (NdotV * sqrt((((NdotL * (-0.0625f)) + NdotL) * NdotL) + 0.0625f))))) * (0.0625f / ((3.1415927410125732421875f * ggx_a) * ggx_a))) / ((4.0f * NdotV) * NdotL).xxx;
        break;
    } while(false);
    float4 indirect_slabs[3] = zero4x3;
    float blue_noise_x = blue_noise.x;
    uint noise_quad = uint((blue_noise_x * 3.0f) + 0.5f) & 3u;
    float weight_acc;
    weight_acc = 0.0f;
    float next_weight_acc;
    for (uint sample_i = 0u; sample_i < 3u; weight_acc = next_weight_acc, sample_i++)
    {
        uint2 half_uv = (pixel_uv / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(noise_quad * 16u) + sample_i].xy);
        float depth_weight = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(half_uv, 0u)).x / depth))));
        for (uint band_i = 0u; band_i < 3u; )
        {
            indirect_slabs[band_i] += (indirect_radiance_tex.Load(int3(half_uv + uint2((band_i * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * depth_weight);
            band_i++;
            continue;
        }
        next_weight_acc = weight_acc + depth_weight;
    }
    float total_weight;
    if (weight_acc < 0.001000000047497451305389404296875f)
    {
        float fallback_weight;
        fallback_weight = weight_acc;
        float next_fallback_weight;
        for (uint fallback_i = 3u; fallback_i < 6u; fallback_weight = next_fallback_weight, fallback_i++)
        {
            uint2 fallback_half_uv = (pixel_uv / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(noise_quad * 16u) + fallback_i].xy);
            float fallback_depth_weight = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(fallback_half_uv, 0u)).x / depth))));
            for (uint fallback_band_i = 0u; fallback_band_i < 3u; )
            {
                indirect_slabs[fallback_band_i] += (indirect_radiance_tex.Load(int3(fallback_half_uv + uint2((fallback_band_i * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * fallback_depth_weight);
                fallback_band_i++;
                continue;
            }
            next_fallback_weight = fallback_weight + fallback_depth_weight;
        }
        total_weight = fallback_weight;
    }
    else
    {
        total_weight = weight_acc;
    }
    float direct_light;
    float inv_total_weight = 1.0f / max(9.9999997473787516355514526367188e-05f, total_weight);
    indirect_slabs[0] *= inv_total_weight;
    indirect_slabs[1] *= inv_total_weight;
    indirect_slabs[2] *= inv_total_weight;
    float3 slab_sum = (indirect_slabs[0].yzw + indirect_slabs[1].yzw) + indirect_slabs[2].yzw;
    float3 slab_dir = slab_sum / max(9.9999996826552253889678874634872e-21f, length(slab_sum)).xxx;
    float3 slab_irradiance = float3(indirect_slabs[0].x, indirect_slabs[1].x, indirect_slabs[2].x);
    float3 slab_dir_x = indirect_slabs[0].yzw / max(9.9999996826552253889678874634872e-21f, indirect_slabs[0].x).xxx;
    float3 slab_dir_y = indirect_slabs[1].yzw / max(9.9999996826552253889678874634872e-21f, indirect_slabs[1].x).xxx;
    float3 slab_dir_z = indirect_slabs[2].yzw / max(9.9999996826552253889678874634872e-21f, indirect_slabs[2].x).xxx;
    float3 indirect_fade = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 indirect_color = max(0.0f.xxx, lerp((indirect_fade / dot(indirect_fade, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(slab_irradiance, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), slab_irradiance, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    float3 indirect_neg_y = (-float3(slab_dir_x.y, slab_dir_y.y, slab_dir_z.y)) * indirect_color;
    float3 indirect_pos_z = float3(slab_dir_x.z, slab_dir_y.z, slab_dir_z.z) * indirect_color;
    float3 indirect_neg_x = (-float3(slab_dir_x.x, slab_dir_y.x, slab_dir_z.x)) * indirect_color;
    float3 indirect_dir = normalize(float3(-dot(indirect_neg_x, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(indirect_neg_y, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(indirect_pos_z, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 indirect_sh_mag = float3(abs(dot(float3(-indirect_neg_x.x, -indirect_neg_y.x, indirect_pos_z.x), indirect_dir)), abs(dot(float3(-indirect_neg_x.y, -indirect_neg_y.y, indirect_pos_z.y), indirect_dir)), abs(dot(float3(-indirect_neg_x.z, -indirect_neg_y.z, indirect_pos_z.z), indirect_dir))) / indirect_color;
    float sh_dir_dot = dot(indirect_dir, n_fine);
    float3 indirect_radiance = max(max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * indirect_color) + ((1.0f.xxx * (n_fine.y * (-1.02332770824432373046875f))) * indirect_neg_y)) + ((1.0f.xxx * (n_fine.z * 1.02332770824432373046875f)) * indirect_pos_z)) + ((1.0f.xxx * (n_fine.x * (-1.02332770824432373046875f))) * indirect_neg_x)) + (((indirect_color * ((indirect_sh_mag * 0.07999999821186065673828125f) + ((indirect_sh_mag * 0.60000002384185791015625f) * indirect_sh_mag))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * sh_dir_dot) * sh_dir_dot) - 1.0f)))), indirect_color * 2.0f);
    float ssgi_mix2 = lerp(0.5f, 1.0f, ssgi_mix);
    float3 sky_specular;
    do
    {
        direct_light = dot(n_fine, slab_dir);
        bool sky_spec_cull;
        if (!(direct_light <= 0.0f))
        {
            sky_spec_cull = dot(n_fine, view_dir) <= 0.0f;
        }
        else
        {
            sky_spec_cull = true;
        }
        if (sky_spec_cull)
        {
            sky_specular = 0.0f.xxx;
            break;
        }
        float3 sky_half_vec = normalize(view_dir + slab_dir);
        float sky_NdotH = dot(n_fine, sky_half_vec);
        float sky_HdotL = max(0.0f, 1.0f - dot(sky_half_vec, slab_dir));
        float sky_HdotL2 = sky_HdotL * sky_HdotL;
        float sky_NdotV = dot(n_fine, view_dir);
        float sky_ggx_a = ((sky_NdotH * sky_NdotH) * (-0.9084937572479248046875f)) + 1.0f;
        sky_specular = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((sky_HdotL2 * sky_HdotL2) * sky_HdotL).xxx) * (((2.0f * direct_light) * sky_NdotV) / ((direct_light * sqrt((((sky_NdotV * (-0.09150625765323638916015625f)) + sky_NdotV) * sky_NdotV) + 0.09150625765323638916015625f)) + (sky_NdotV * sqrt((((direct_light * (-0.09150625765323638916015625f)) + direct_light) * direct_light) + 0.09150625765323638916015625f))))) * (0.09150625765323638916015625f / ((3.1415927410125732421875f * sky_ggx_a) * sky_ggx_a))) / ((4.0f * sky_NdotV) * direct_light).xxx;
        break;
    } while(false);
    float3 diffuse = (0.3183098733425140380859375f.xxx * (((((albedo_masked * 1.0f) * lerp(lerp(0.0f, 0.5f, octa_b), 1.0f, smoothstep(lerp(0.0f, -0.699999988079071044921875f, octa_b), lerp(1.0f, 0.699999988079071044921875f, octa_b), NdotL))) * sun_color.xyz) * shadow_vis) + ((((((specular * sun_color.xyz) * octa_a) * (color_library.sky_base.xyz * color_library.sky_base.xyz)) * max(0.0f, NdotL)) * sun_color.xyz) * shadow_vis))) + ((((max(ssgi_mix2.xxx, ((((((albedo_masked * 2.040400028228759765625f) - 0.3323999941349029541015625f.xxx) * ssgi_mix2) + ((albedo_masked * (-4.79510021209716796875f)) + 0.6417000293731689453125f.xxx)) * ssgi_mix2) + ((albedo_masked * 2.755199909210205078125f) + 0.69029998779296875f.xxx)) * ssgi_mix2) * 1.10000002384185791015625f) * albedo_masked) * indirect_radiance) + ((((indirect_radiance * octa_a) * sky_specular) * ssgi_mix) * clamp(direct_light * 12.0f, 0.0f, 4.0f)));
    float3 out_color;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 grid_cell = uint3(clamp(floor((world_pos * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint grid_idx = (32u * ((32u * grid_cell.y) + grid_cell.z)) + grid_cell.x;
        bool grid_full = WaveActiveAllTrue((light_indirection_grid.Load<uint>(grid_idx * 4 + 0) >> 20u) <= 2u);
        uint res_mode = grid_full ? 2u : 6u;
        float3 lantern_light2;
        if (6u == res_mode)
        {
            uint res_ids[8];
            for (uint id_init_i2 = 0u; id_init_i2 < 8u; )
            {
                res_ids[id_init_i2] = 4294967295u;
                id_init_i2++;
                continue;
            }
            uint2 quad_xy2 = uint2(pixel_xy & int2(1, 1));
            uint quad_idx2 = (quad_xy2.x + (quad_xy2.y * 2u)) & 3u;
            uint slot_count2;
            slot_count2 = 0u;
            uint next_slot2;
            uint tap_i2 = 0u;
            for (;;)
            {
                bool have_slot2;
                if (tap_i2 < 8u)
                {
                    have_slot2 = slot_count2 < 8u;
                }
                else
                {
                    have_slot2 = false;
                }
                if (have_slot2)
                {
                    uint4 res_sample2 = rdi_reservoir_tex.Load(int3(uint2((pixel_xy / int2(2, 2)) + spatial_resolve_offsets_array[((quad_idx2 * 16u) + tap_i2) + 256u].xy), 0u));
                    uint2 res_sample2_xy = res_sample2.xy;
                    uint res_sample2_id = res_sample2.x;
                    bool res_valid2;
                    if (any(bool2(res_sample2_xy.x != uint2(0u, 0u).x, res_sample2_xy.y != uint2(0u, 0u).y)))
                    {
                        res_valid2 = res_sample2_id < lantern_count;
                    }
                    else
                    {
                        res_valid2 = false;
                    }
                    bool d2_1;
                    if (res_valid2)
                    {
                        d2_1 = res_ids[0u] != res_sample2_id;
                    }
                    else
                    {
                        d2_1 = false;
                    }
                    bool d2_2;
                    if (d2_1)
                    {
                        d2_2 = res_ids[1u] != res_sample2_id;
                    }
                    else
                    {
                        d2_2 = false;
                    }
                    bool d2_3;
                    if (d2_2)
                    {
                        d2_3 = res_ids[2u] != res_sample2_id;
                    }
                    else
                    {
                        d2_3 = false;
                    }
                    bool d2_4;
                    if (d2_3)
                    {
                        d2_4 = res_ids[3u] != res_sample2_id;
                    }
                    else
                    {
                        d2_4 = false;
                    }
                    bool d2_5;
                    if (d2_4)
                    {
                        d2_5 = res_ids[4u] != res_sample2_id;
                    }
                    else
                    {
                        d2_5 = false;
                    }
                    bool d2_6;
                    if (d2_5)
                    {
                        d2_6 = res_ids[5u] != res_sample2_id;
                    }
                    else
                    {
                        d2_6 = false;
                    }
                    bool d2_7;
                    if (d2_6)
                    {
                        d2_7 = res_ids[6u] != res_sample2_id;
                    }
                    else
                    {
                        d2_7 = false;
                    }
                    bool d2_8;
                    if (d2_7)
                    {
                        d2_8 = res_ids[7u] != res_sample2_id;
                    }
                    else
                    {
                        d2_8 = false;
                    }
                    if (d2_8)
                    {
                        res_ids[slot_count2] = res_sample2_id;
                        next_slot2 = slot_count2 + 1u;
                    }
                    else
                    {
                        next_slot2 = slot_count2;
                    }
                    slot_count2 = next_slot2;
                    tap_i2++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint merge_count2;
            merge_count2 = slot_count2;
            uint next_merge3;
            for (uint q_i2 = 0u; q_i2 < 8u; merge_count2 = next_merge3, q_i2++)
            {
                uint quad_id2 = QuadReadAcrossDiagonal(res_ids[q_i2]);
                bool quad_add2;
                if (quad_id2 < lantern_count)
                {
                    quad_add2 = merge_count2 < 8u;
                }
                else
                {
                    quad_add2 = false;
                }
                if (quad_add2)
                {
                    bool q2_1;
                    if (res_ids[0u] != quad_id2)
                    {
                        q2_1 = res_ids[1u] != quad_id2;
                    }
                    else
                    {
                        q2_1 = false;
                    }
                    bool q2_2;
                    if (q2_1)
                    {
                        q2_2 = res_ids[2u] != quad_id2;
                    }
                    else
                    {
                        q2_2 = false;
                    }
                    bool q2_3;
                    if (q2_2)
                    {
                        q2_3 = res_ids[3u] != quad_id2;
                    }
                    else
                    {
                        q2_3 = false;
                    }
                    bool q2_4;
                    if (q2_3)
                    {
                        q2_4 = res_ids[4u] != quad_id2;
                    }
                    else
                    {
                        q2_4 = false;
                    }
                    bool q2_5;
                    if (q2_4)
                    {
                        q2_5 = res_ids[5u] != quad_id2;
                    }
                    else
                    {
                        q2_5 = false;
                    }
                    bool q2_6;
                    if (q2_5)
                    {
                        q2_6 = res_ids[6u] != quad_id2;
                    }
                    else
                    {
                        q2_6 = false;
                    }
                    bool q2_7;
                    if (q2_6)
                    {
                        q2_7 = res_ids[7u] != quad_id2;
                    }
                    else
                    {
                        q2_7 = false;
                    }
                    uint next_merge4;
                    if (q2_7)
                    {
                        res_ids[merge_count2] = quad_id2;
                        next_merge4 = merge_count2 + 1u;
                    }
                    else
                    {
                        next_merge4 = merge_count2;
                    }
                    next_merge3 = next_merge4;
                }
                else
                {
                    next_merge3 = merge_count2;
                }
            }
            Reservoir1sppStreamState res_streams[2];
            Reservoir1spp reservoirs[2];
            for (uint res_init_i = 0u; res_init_i < 2u; )
            {
                res_streams[res_init_i] = res_stream_zero;
                reservoirs[res_init_i] = reservoir_zero;
                res_init_i++;
                continue;
            }
            for (uint lantern_i = 0u; lantern_i < merge_count2; lantern_i++)
            {
                float2 lantern_half2 = spvUnpackHalf2x16(lantern_buf.Load<uint>(res_ids[lantern_i] * 16 + 12) & 65535u);
                float lantern_intensity2 = lantern_half2.x;
                uint lantern_color_id2 = lantern_buf.Load<uint>(res_ids[lantern_i] * 16 + 12) >> 16u;
                float3 lantern_vec2 = lantern_buf.Load<float3>(res_ids[lantern_i] * 16 + 0) - world_pos;
                float lantern_dist_sq2 = dot(lantern_vec2, lantern_vec2);
                float3 lantern_vec2_h = lantern_vec2 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 lantern_color2;
                do
                {
                    if (lantern_color_id2 == 0u)
                    {
                        lantern_color2 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (lantern_color_id2 == 1u)
                        {
                            lantern_color2 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (lantern_color_id2 == 2u)
                            {
                                lantern_color2 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (lantern_color_id2 == 3u)
                                {
                                    lantern_color2 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    lantern_color2 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                                    break;
                                }
                                break; // unreachable workaround
                            }
                            break; // unreachable workaround
                        }
                        break; // unreachable workaround
                    }
                    break; // unreachable workaround
                } while(false);
                float lantern_weight = dot(((lantern_color2 * 4.0f) * lantern_intensity2) * ((max(0.0f, dot(n_fine, lantern_vec2 / length(lantern_vec2).xxx)) * ((1.0f / (0.3359999954700469970703125f + lantern_dist_sq2)) * smoothstep(36.0f * lantern_intensity2, 16.0f * lantern_intensity2, lantern_dist_sq2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_vec2_h / length(lantern_vec2_h).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float lantern_count_f = float(merge_count2);
                float stream_rand = frac(blue_noise_x + (float(lantern_i) / lantern_count_f));
                if (reservoirs[0].w_sum <= reservoirs[1].w_sum)
                {
                    do
                    {
                        res_streams[0].M_sum += 1.0f;
                        float w_scaled_a = lantern_weight * lantern_count_f;
                        bool hit_a;
                        do
                        {
                            reservoirs[0].w_sum += w_scaled_a;
                            reservoirs[0].M += 1.0f;
                            if ((w_scaled_a / reservoirs[0].w_sum) >= stream_rand)
                            {
                                reservoirs[0].payload = res_ids[lantern_i];
                                hit_a = true;
                                break;
                            }
                            else
                            {
                                hit_a = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (hit_a)
                        {
                            res_streams[0].p_q_sel = lantern_weight;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                }
                else
                {
                    do
                    {
                        res_streams[1].M_sum += 1.0f;
                        float w_scaled_b = lantern_weight * lantern_count_f;
                        bool hit_b;
                        do
                        {
                            reservoirs[1].w_sum += w_scaled_b;
                            reservoirs[1].M += 1.0f;
                            if ((w_scaled_b / reservoirs[1].w_sum) >= stream_rand)
                            {
                                reservoirs[1].payload = res_ids[lantern_i];
                                hit_b = true;
                                break;
                            }
                            else
                            {
                                hit_b = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (hit_b)
                        {
                            res_streams[1].p_q_sel = lantern_weight;
                            break;
                        }
                        else
                        {
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                }
            }
            for (uint merge_red_i = 1u; merge_red_i < 2u; )
            {
                res_streams[0].M_sum += res_streams[merge_red_i].M_sum;
                merge_red_i++;
                continue;
            }
            for (uint merge_broad_i = 1u; merge_broad_i < 2u; )
            {
                res_streams[merge_broad_i].M_sum = res_streams[0].M_sum;
                merge_broad_i++;
                continue;
            }
            float3 light_acc2;
            light_acc2 = diffuse;
            float next_near2;
            float next_far2;
            float next_eps2;
            float3 next_light_acc2;
            uint next_hash_keep2;
            uint next_hash_hit2;
            bool next_valid_keep2;
            bool next_valid_hit2;
            float march_near2 = 0.0f;
            float march_far2 = 0.0f;
            float march_eps2b = 0.0f;
            uint res_i = 0u;
            uint hash_keep2 = 0u;
            uint hash_hit2 = 0u;
            bool valid_keep2 = false;
            bool valid_hit2 = false;
            for (; res_i < 2u; march_near2 = next_near2, march_far2 = next_far2, march_eps2b = next_eps2, light_acc2 = next_light_acc2, res_i++, hash_keep2 = next_hash_keep2, hash_hit2 = next_hash_hit2, valid_keep2 = next_valid_keep2, valid_hit2 = next_valid_hit2)
            {
                reservoirs[res_i].M = res_streams[res_i].M_sum;
                reservoirs[res_i].W = reservoirs[res_i].w_sum / max(9.9999999392252902907785028219223e-09f, reservoirs[res_i].M * res_streams[res_i].p_q_sel);
                if (reservoirs[res_i].W > 0.0f)
                {
                    float2 lantern_half3 = spvUnpackHalf2x16(lantern_buf.Load<uint>(reservoirs[res_i].payload * 16 + 12) & 65535u);
                    float lantern_intensity3 = lantern_half3.x;
                    uint lantern_color_id3 = lantern_buf.Load<uint>(reservoirs[res_i].payload * 16 + 12) >> 16u;
                    float3 lantern_vec3 = lantern_buf.Load<float3>(reservoirs[res_i].payload * 16 + 0) - world_pos;
                    float lantern_dist3b = length(lantern_vec3);
                    float lantern_dist_sq3 = dot(lantern_vec3, lantern_vec3);
                    float3 lantern_vec3_h = lantern_vec3 + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 lantern_dist3x = lantern_dist3b.xxx;
                    float3 lantern_color3;
                    do
                    {
                        if (lantern_color_id3 == 0u)
                        {
                            lantern_color3 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (lantern_color_id3 == 1u)
                            {
                                lantern_color3 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (lantern_color_id3 == 2u)
                                {
                                    lantern_color3 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (lantern_color_id3 == 3u)
                                    {
                                        lantern_color3 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        lantern_color3 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                                        break;
                                    }
                                    break; // unreachable workaround
                                }
                                break; // unreachable workaround
                            }
                            break; // unreachable workaround
                        }
                        break; // unreachable workaround
                    } while(false);
                    float3 lantern_weight3 = ((lantern_color3 * 4.0f) * lantern_intensity3) * ((max(0.0f, dot(n_fine, lantern_vec3 / lantern_dist3x)) * ((1.0f / (0.3359999954700469970703125f + lantern_dist_sq3)) * smoothstep(36.0f * lantern_intensity3, 16.0f * lantern_intensity3, lantern_dist_sq3))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_vec3_h / length(lantern_vec3_h).xxx).y));
                    float out_near;
                    float out_far;
                    float out_eps;
                    uint out_keep_bits;
                    uint out_hit_bits;
                    bool out_valid_keep;
                    bool out_valid_hit;
                    float3 out_light;
                    if ((reservoirs[res_i].W * max(max(lantern_weight3.x, lantern_weight3.y), lantern_weight3.z)) > 0.0f)
                    {
                        float acc_near2;
                        float acc_far2;
                        float acc_eps2;
                        float3 acc_contrib2;
                        if (lantern_dist3b > 0.5f)
                        {
                            float3 sample_pos2 = (world_pos + ((lantern_vec3 * (lantern_dist3b - 0.5f)) / lantern_dist3x)) + (n_fine * (0.0199999995529651641845703125f * blue_noise.y));
                            float3 view_pos2 = view_pos_h.xyz;
                            float4 sample_clip2 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(sample_pos2, 1.0f)));
                            float3 sample_ndc2 = sample_clip2.xyz / sample_clip2.w.xxx;
                            float4 march_view_h2 = view_pos_h + (float4(sample_ndc2 - view_pos2, 0.0f) * float(int(sign(sample_ndc2.z))));
                            float march_w2 = march_view_h2.w;
                            float3 march_pos2b = (march_view_h2 / (float((march_w2 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(march_w2))).xxxx).xyz;
                            float3 march_delta2 = march_pos2b - view_pos2;
                            bool3 march_neg2 = bool3(march_delta2.x < 0.0f.xxx.x, march_delta2.y < 0.0f.xxx.y, march_delta2.z < 0.0f.xxx.z);
                            float3 march_t_xy2 = (float3(march_neg2.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_neg2.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_neg2.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos2) / march_delta2;
                            float3 march_enter2 = view_pos2 + (march_delta2 * max(0.0f, max(march_t_xy2.x, march_t_xy2.y)));
                            float3 march_dir2 = march_pos2b - march_enter2;
                            bool3 march_pos22 = bool3(march_dir2.x >= 0.0f.xxx.x, march_dir2.y >= 0.0f.xxx.y, march_dir2.z >= 0.0f.xxx.z);
                            float3 march_t_xyz2 = (float3(march_pos22.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_pos22.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_pos22.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - march_enter2) / march_dir2;
                            float3 march_exit2 = march_enter2 + (march_dir2 * min(1.0f, min(min(march_t_xyz2.x, march_t_xyz2.y), march_t_xyz2.z)));
                            float near_ret2;
                            float far_ret2;
                            float eps_ret2;
                            bool hit_final2;
                            float t_final2;
                            float shadow_t_scale2;
                            do
                            {
                                float shadow_eps2 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint march_steps2 = uint(int(max(2u, min(4u, uint(int(floor(length((((march_exit2.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((march_enter2.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float march_prev_t2;
                                float march_t_ret2;
                                bool hit_ret2;
                                do
                                {
                                    float3 march_step_vec2 = march_exit2 - march_enter2;
                                    float march_steps2_f;
                                    if (march_exit2.z > 9.5367431640625e-07f)
                                    {
                                        march_steps2_f = float(march_steps2);
                                    }
                                    else
                                    {
                                        march_steps2_f = max(1.0f, float(march_steps2) - 1.0f);
                                    }
                                    float inv_steps2 = 1.0f / march_steps2_f;
                                    float march_over2;
                                    float march_under2;
                                    float march_eps3;
                                    float march_t3;
                                    bool march_hit2;
                                    if (march_steps2 > 0u)
                                    {
                                        float march_t02 = lerp(0.0f, 1.0f, pow(blue_noise_x * inv_steps2, 1.0f));
                                        float3 march_pos02 = march_enter2 + (march_step_vec2 * march_t02);
                                        float2 march_uv02 = (march_pos02.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float inv_z02 = 1.0f / march_pos02.z;
                                        float inv_depth_llc2 = 1.0f / depth_tex.SampleLevel(sampler_llc, march_uv02, 0.0f).x;
                                        float inv_depth_nnc2 = 1.0f / depth_tex.SampleLevel(sampler_nnc, march_uv02, 0.0f).x;
                                        float depth_over2 = (max(inv_depth_llc2, inv_depth_nnc2) * 1.00000202655792236328125f) - inv_z02;
                                        float depth_under2 = inv_z02 - min(inv_depth_llc2, inv_depth_nnc2);
                                        bool shadow_hit02 = (depth_over2 < 0.0f) ? (depth_under2 < shadow_eps2) : false;
                                        float res_over2;
                                        float res_under2;
                                        float res_eps2;
                                        float res_t2;
                                        bool res_hit2;
                                        float res_prev_t2;
                                        if (shadow_hit02)
                                        {
                                            res_over2 = depth_over2;
                                            res_under2 = depth_under2;
                                            res_eps2 = shadow_eps2;
                                            res_t2 = march_t02;
                                            res_hit2 = shadow_hit02;
                                            res_prev_t2 = 0.0f;
                                        }
                                        else
                                        {
                                            float prev_t2;
                                            prev_t2 = march_t02;
                                            bool shadow_hit2;
                                            float t2;
                                            float loop_over2;
                                            float loop_under2;
                                            float loop_eps2;
                                            float loop_t2;
                                            bool loop_hit2;
                                            uint march_i2 = 1u;
                                            bool shadow_hit_prev2 = shadow_hit02;
                                            for (;;)
                                            {
                                                if (march_i2 < march_steps2)
                                                {
                                                    t2 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(march_i2) + blue_noise_x) * inv_steps2, 1.0f)));
                                                    float3 march_pos_i2 = march_enter2 + (march_step_vec2 * t2);
                                                    float2 march_uv_i2 = (march_pos_i2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float inv_z_i2 = 1.0f / march_pos_i2.z;
                                                    float inv_depth_llc_i2 = 1.0f / depth_tex.SampleLevel(sampler_llc, march_uv_i2, 0.0f).x;
                                                    float inv_depth_nnc_i2 = 1.0f / depth_tex.SampleLevel(sampler_nnc, march_uv_i2, 0.0f).x;
                                                    float depth_over_i2 = (max(inv_depth_llc_i2, inv_depth_nnc_i2) * 1.00000202655792236328125f) - inv_z_i2;
                                                    float depth_under_i2 = inv_z_i2 - min(inv_depth_llc_i2, inv_depth_nnc_i2);
                                                    shadow_hit2 = (depth_over_i2 < 0.0f) ? (depth_under_i2 < shadow_eps2) : false;
                                                    if (shadow_hit2)
                                                    {
                                                        loop_over2 = depth_over_i2;
                                                        loop_under2 = depth_under_i2;
                                                        loop_eps2 = shadow_eps2;
                                                        loop_t2 = t2;
                                                        loop_hit2 = shadow_hit2;
                                                        break;
                                                    }
                                                    march_i2++;
                                                    shadow_hit_prev2 = shadow_hit2;
                                                    prev_t2 = t2;
                                                    continue;
                                                }
                                                else
                                                {
                                                    loop_over2 = 0.0f;
                                                    loop_under2 = 0.0f;
                                                    loop_eps2 = 0.0f;
                                                    loop_t2 = 1.0f;
                                                    loop_hit2 = shadow_hit_prev2;
                                                    break;
                                                }
                                            }
                                            res_over2 = loop_over2;
                                            res_under2 = loop_under2;
                                            res_eps2 = loop_eps2;
                                            res_t2 = loop_t2;
                                            res_hit2 = loop_hit2;
                                            res_prev_t2 = prev_t2;
                                        }
                                        march_over2 = res_over2;
                                        march_under2 = res_under2;
                                        march_eps3 = res_eps2;
                                        march_t3 = res_t2;
                                        march_hit2 = res_hit2;
                                        march_prev_t2 = res_prev_t2;
                                    }
                                    else
                                    {
                                        march_over2 = 0.0f;
                                        march_under2 = 0.0f;
                                        march_eps3 = 0.0f;
                                        march_t3 = 1.0f;
                                        march_hit2 = false;
                                        march_prev_t2 = 0.0f;
                                    }
                                    if (march_hit2)
                                    {
                                        march_t_ret2 = march_t3;
                                        near_ret2 = march_over2;
                                        far_ret2 = march_eps3;
                                        eps_ret2 = march_under2;
                                        hit_ret2 = true;
                                        break;
                                    }
                                    else
                                    {
                                        march_t_ret2 = march_prev_t2;
                                        near_ret2 = march_near2;
                                        far_ret2 = march_far2;
                                        eps_ret2 = march_eps2b;
                                        hit_ret2 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool hit_less2;
                                if (hit_ret2)
                                {
                                    bool hit_cmp2;
                                    if (eps_ret2 < far_ret2)
                                    {
                                        hit_cmp2 = near_ret2 < far_ret2;
                                    }
                                    else
                                    {
                                        hit_cmp2 = false;
                                    }
                                    hit_less2 = hit_cmp2;
                                }
                                else
                                {
                                    hit_less2 = false;
                                }
                                if (hit_less2)
                                {
                                    hit_final2 = true;
                                    t_final2 = march_t_ret2;
                                    shadow_t_scale2 = eps_ret2 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                hit_final2 = false;
                                t_final2 = march_prev_t2;
                                shadow_t_scale2 = 0.0f;
                                break;
                            } while(false);
                            float3 lantern_shadowed2;
                            if (hit_final2)
                            {
                                float4 shadow_world_h2 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(march_enter2, march_exit2, t_final2.xxx), 1.0f)));
                                float3 shadow_vec2 = ((shadow_world_h2.xyz / shadow_world_h2.w.xxx) + (view_dir * shadow_t_scale2)) - lantern_buf.Load<float3>(reservoirs[res_i].payload * 16 + 0);
                                bool shadow_outside2;
                                if (!(abs(shadow_vec2.y) > 0.5f))
                                {
                                    float2 shadow_vec2_xz = shadow_vec2.xz;
                                    shadow_outside2 = dot(shadow_vec2_xz, shadow_vec2_xz) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    shadow_outside2 = true;
                                }
                                bool3 shadow_outside32 = shadow_outside2.xxx;
                                lantern_shadowed2 = float3(shadow_outside32.x ? 0.0f.xxx.x : lantern_weight3.x, shadow_outside32.y ? 0.0f.xxx.y : lantern_weight3.y, shadow_outside32.z ? 0.0f.xxx.z : lantern_weight3.z);
                            }
                            else
                            {
                                lantern_shadowed2 = lantern_weight3;
                            }
                            acc_near2 = near_ret2;
                            acc_far2 = far_ret2;
                            acc_eps2 = eps_ret2;
                            acc_contrib2 = lantern_shadowed2;
                        }
                        else
                        {
                            acc_near2 = march_near2;
                            acc_far2 = march_far2;
                            acc_eps2 = march_eps2b;
                            acc_contrib2 = lantern_weight3;
                        }
                        float2 hash_coord2 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(reservoirs[res_i].payload * 10u));
                        int2 hash_cell2 = int2(floor(hash_coord2));
                        float2 hash_frac2 = frac(hash_coord2);
                        uint2 hash_cell2_u = uint2(hash_cell2);
                        uint r_o_y = hash_cell2_u.y;
                        uint r_o1 = r_o_y + (r_o_y << 10u);
                        uint r_o2 = r_o1 ^ (r_o1 >> 6u);
                        uint r_o3 = r_o2 + (r_o2 << 3u);
                        uint r_o4 = r_o3 ^ (r_o3 >> 11u);
                        uint r_o5 = (((hash_cell2_u.x * 1664525u) + (r_o4 + (r_o4 << 15u))) + 1013904223u) * 1664525u;
                        uint r_o6 = r_o5 ^ (r_o5 >> 11u);
                        uint r_o7 = r_o6 ^ ((r_o6 << 7u) & 2636928640u);
                        uint r_o8 = r_o7 ^ ((r_o7 << 15u) & 4022730752u);
                        bool keep_r_o = valid_hit2 ? valid_hit2 : true;
                        uint keep_r_o_bits = valid_hit2 ? hash_hit2 : 8388607u;
                        bool hit_r_o = valid_keep2 ? valid_keep2 : true;
                        uint hit_r_o_bits = valid_keep2 ? hash_keep2 : 1065353216u;
                        uint2 cell2_px = uint2(hash_cell2 + int2(1, 0));
                        uint r_px_y = cell2_px.y;
                        uint r_px1 = r_px_y + (r_px_y << 10u);
                        uint r_px2 = r_px1 ^ (r_px1 >> 6u);
                        uint r_px3 = r_px2 + (r_px2 << 3u);
                        uint r_px4 = r_px3 ^ (r_px3 >> 11u);
                        uint r_px5 = (((cell2_px.x * 1664525u) + (r_px4 + (r_px4 << 15u))) + 1013904223u) * 1664525u;
                        uint r_px6 = r_px5 ^ (r_px5 >> 11u);
                        uint r_px7 = r_px6 ^ ((r_px6 << 7u) & 2636928640u);
                        uint r_px8 = r_px7 ^ ((r_px7 << 15u) & 4022730752u);
                        bool keep_r_px = keep_r_o ? keep_r_o : true;
                        uint keep_r_px_bits = keep_r_o ? keep_r_o_bits : 8388607u;
                        bool hit_r_px = hit_r_o ? hit_r_o : true;
                        uint hit_r_px_bits = hit_r_o ? hit_r_o_bits : 1065353216u;
                        uint2 cell2_py = uint2(hash_cell2 + int2(0, 1));
                        uint r_py_y = cell2_py.y;
                        uint r_py1 = r_py_y + (r_py_y << 10u);
                        uint r_py2 = r_py1 ^ (r_py1 >> 6u);
                        uint r_py3 = r_py2 + (r_py2 << 3u);
                        uint r_py4 = r_py3 ^ (r_py3 >> 11u);
                        uint r_py5 = (((cell2_py.x * 1664525u) + (r_py4 + (r_py4 << 15u))) + 1013904223u) * 1664525u;
                        uint r_py6 = r_py5 ^ (r_py5 >> 11u);
                        uint r_py7 = r_py6 ^ ((r_py6 << 7u) & 2636928640u);
                        uint r_py8 = r_py7 ^ ((r_py7 << 15u) & 4022730752u);
                        bool keep_r_py = keep_r_px ? keep_r_px : true;
                        uint keep_r_py_bits = keep_r_px ? keep_r_px_bits : 8388607u;
                        bool hit_r_py = hit_r_px ? hit_r_px : true;
                        uint hit_r_py_bits = hit_r_px ? hit_r_px_bits : 1065353216u;
                        uint2 cell2_pxy = uint2(hash_cell2 + int2(1, 1));
                        uint r_pxy_y = cell2_pxy.y;
                        uint r_pxy1 = r_pxy_y + (r_pxy_y << 10u);
                        uint r_pxy2 = r_pxy1 ^ (r_pxy1 >> 6u);
                        uint r_pxy3 = r_pxy2 + (r_pxy2 << 3u);
                        uint r_pxy4 = r_pxy3 ^ (r_pxy3 >> 11u);
                        uint r_pxy5 = (((cell2_pxy.x * 1664525u) + (r_pxy4 + (r_pxy4 << 15u))) + 1013904223u) * 1664525u;
                        uint r_pxy6 = r_pxy5 ^ (r_pxy5 >> 11u);
                        uint r_pxy7 = r_pxy6 ^ ((r_pxy6 << 7u) & 2636928640u);
                        uint r_pxy8 = r_pxy7 ^ ((r_pxy7 << 15u) & 4022730752u);
                        uint keep_r_pxy_bits = keep_r_py ? keep_r_py_bits : 8388607u;
                        uint hit_r_pxy_bits = hit_r_py ? hit_r_py_bits : 1065353216u;
                        float2 smooth2 = (hash_frac2 * hash_frac2) * (3.0f.xx - (hash_frac2 * 2.0f));
                        float smooth2_x = smooth2.x;
                        float om_smooth2_x = 1.0f - smooth2_x;
                        float smooth2_y = smooth2.y;
                        out_near = acc_near2;
                        out_far = acc_far2;
                        out_eps = acc_eps2;
                        out_keep_bits = hit_r_pxy_bits;
                        out_hit_bits = keep_r_pxy_bits;
                        out_valid_keep = hit_r_py ? hit_r_py : true;
                        out_valid_hit = keep_r_py ? keep_r_py : true;
                        out_light = (((acc_contrib2 * lerp(0.5f, 1.0f, (((1.0f - smooth2_y) * (((asfloat(((r_o8 ^ (r_o8 >> 18u)) & keep_r_o_bits) | hit_r_o_bits) - 1.0f) * om_smooth2_x) + ((asfloat(((r_px8 ^ (r_px8 >> 18u)) & keep_r_px_bits) | hit_r_px_bits) - 1.0f) * smooth2_x))) + (((asfloat(((r_py8 ^ (r_py8 >> 18u)) & keep_r_py_bits) | hit_r_py_bits) - 1.0f) * om_smooth2_x) * smooth2_y)) + (((asfloat(((r_pxy8 ^ (r_pxy8 >> 18u)) & keep_r_pxy_bits) | hit_r_pxy_bits) - 1.0f) * smooth2_x) * smooth2_y))) * reservoirs[res_i].W) * albedo_masked) * ev_mult;
                    }
                    else
                    {
                        out_near = march_near2;
                        out_far = march_far2;
                        out_eps = march_eps2b;
                        out_keep_bits = hash_keep2;
                        out_hit_bits = hash_hit2;
                        out_valid_keep = valid_keep2;
                        out_valid_hit = valid_hit2;
                        out_light = 0.0f.xxx;
                    }
                    next_near2 = out_near;
                    next_far2 = out_far;
                    next_eps2 = out_eps;
                    next_light_acc2 = light_acc2 + out_light;
                    next_hash_keep2 = out_keep_bits;
                    next_hash_hit2 = out_hit_bits;
                    next_valid_keep2 = out_valid_keep;
                    next_valid_hit2 = out_valid_hit;
                }
                else
                {
                    next_near2 = march_near2;
                    next_far2 = march_far2;
                    next_eps2 = march_eps2b;
                    next_light_acc2 = light_acc2;
                    next_hash_keep2 = hash_keep2;
                    next_hash_hit2 = hash_hit2;
                    next_valid_keep2 = valid_keep2;
                    next_valid_hit2 = valid_hit2;
                }
            }
            lantern_light2 = light_acc2;
        }
        else
        {
            float3 grid_light_result;
            if (2u == res_mode)
            {
                uint cell_base = light_indirection_grid.Load<uint>(grid_idx * 4 + 0) & 1048575u;
                uint cell_count = min(64u, (light_indirection_grid.Load<uint>(grid_idx * 4 + 0) >> 20u));
                float3 light_acc;
                light_acc = diffuse;
                float3 next_light_acc;
                float next_near;
                float next_far;
                float next_eps;
                uint next_hash_keep;
                uint next_hash_hit;
                bool next_valid_keep;
                bool next_valid_hit;
                float march_near = 0.0f;
                float march_far = 0.0f;
                float march_eps = 0.0f;
                uint cell_i = 0u;
                uint hash_keep = 0u;
                uint hash_hit = 0u;
                bool valid_keep = false;
                bool valid_hit = false;
                for (; cell_i < cell_count; light_acc = next_light_acc, march_near = next_near, march_far = next_far, march_eps = next_eps, cell_i++, hash_keep = next_hash_keep, hash_hit = next_hash_hit, valid_keep = next_valid_keep, valid_hit = next_valid_hit)
                {
                    uint light_idx = cell_base + cell_i;
                    float2 lantern_half = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(light_idx * 4 + 0) * 16 + 12) & 65535u);
                    float lantern_intensity = lantern_half.x;
                    uint lantern_color_id = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(light_idx * 4 + 0) * 16 + 12) >> 16u;
                    float3 lantern_vec = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(light_idx * 4 + 0) * 16 + 0) - world_pos;
                    float lantern_dist = length(lantern_vec);
                    float lantern_dist_sq = dot(lantern_vec, lantern_vec);
                    float3 lantern_vec_h = lantern_vec + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 lantern_dist3 = lantern_dist.xxx;
                    float3 lantern_color;
                    do
                    {
                        if (lantern_color_id == 0u)
                        {
                            lantern_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (lantern_color_id == 1u)
                            {
                                lantern_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (lantern_color_id == 2u)
                                {
                                    lantern_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (lantern_color_id == 3u)
                                    {
                                        lantern_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        lantern_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                                        break;
                                    }
                                    break; // unreachable workaround
                                }
                                break; // unreachable workaround
                            }
                            break; // unreachable workaround
                        }
                        break; // unreachable workaround
                    } while(false);
                    float3 lantern_contrib = ((lantern_color * 4.0f) * lantern_intensity) * ((max(0.0f, dot(n_fine, lantern_vec / lantern_dist3)) * ((1.0f / (0.3359999954700469970703125f + lantern_dist_sq)) * smoothstep(36.0f * lantern_intensity, 16.0f * lantern_intensity, lantern_dist_sq))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_vec_h / length(lantern_vec_h).xxx).y));
                    float3 lantern_light;
                    if (max(max(lantern_contrib.x, lantern_contrib.y), lantern_contrib.z) > 0.0f)
                    {
                        float acc_near;
                        float acc_far;
                        float acc_eps;
                        float3 acc_contrib;
                        if (lantern_dist > 0.5f)
                        {
                            float3 lantern_sample_pos = (world_pos + ((lantern_vec * (lantern_dist - 0.5f)) / lantern_dist3)) + (n_fine * (0.0199999995529651641845703125f * blue_noise.y));
                            float3 view_pos = view_pos_h.xyz;
                            float4 lantern_sample_clip = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(lantern_sample_pos, 1.0f)));
                            float3 lantern_sample_ndc = lantern_sample_clip.xyz / lantern_sample_clip.w.xxx;
                            float4 march_view_h = view_pos_h + (float4(lantern_sample_ndc - view_pos, 0.0f) * float(int(sign(lantern_sample_ndc.z))));
                            float march_w = march_view_h.w;
                            float3 march_pos = (march_view_h / (float((march_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(march_w))).xxxx).xyz;
                            float3 march_delta = march_pos - view_pos;
                            bool3 march_neg = bool3(march_delta.x < 0.0f.xxx.x, march_delta.y < 0.0f.xxx.y, march_delta.z < 0.0f.xxx.z);
                            float3 march_t_xy = (float3(march_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos) / march_delta;
                            float3 march_enter = view_pos + (march_delta * max(0.0f, max(march_t_xy.x, march_t_xy.y)));
                            float3 march_dir = march_pos - march_enter;
                            bool3 march_pos2 = bool3(march_dir.x >= 0.0f.xxx.x, march_dir.y >= 0.0f.xxx.y, march_dir.z >= 0.0f.xxx.z);
                            float3 march_t_xyz = (float3(march_pos2.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, march_pos2.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, march_pos2.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - march_enter) / march_dir;
                            float3 march_exit = march_enter + (march_dir * min(1.0f, min(min(march_t_xyz.x, march_t_xyz.y), march_t_xyz.z)));
                            float near_ret;
                            float far_ret;
                            float eps_ret;
                            bool hit_final;
                            float t_final;
                            float shadow_t_scale;
                            do
                            {
                                float shadow_eps = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint march_steps = uint(int(max(2u, min(4u, uint(int(floor(length((((march_exit.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((march_enter.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float march_prev_t;
                                float march_t_ret;
                                bool hit_ret;
                                do
                                {
                                    float3 march_step_vec = march_exit - march_enter;
                                    float march_steps_f;
                                    if (march_exit.z > 9.5367431640625e-07f)
                                    {
                                        march_steps_f = float(march_steps);
                                    }
                                    else
                                    {
                                        march_steps_f = max(1.0f, float(march_steps) - 1.0f);
                                    }
                                    float inv_steps = 1.0f / march_steps_f;
                                    float march_over;
                                    float march_under;
                                    float march_eps2;
                                    float march_t2;
                                    bool march_hit;
                                    if (march_steps > 0u)
                                    {
                                        float march_t0 = lerp(0.0f, 1.0f, pow(blue_noise_x * inv_steps, 1.0f));
                                        float3 march_pos0 = march_enter + (march_step_vec * march_t0);
                                        float2 march_uv0 = (march_pos0.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float inv_z0 = 1.0f / march_pos0.z;
                                        float inv_depth_llc = 1.0f / depth_tex.SampleLevel(sampler_llc, march_uv0, 0.0f).x;
                                        float inv_depth_nnc = 1.0f / depth_tex.SampleLevel(sampler_nnc, march_uv0, 0.0f).x;
                                        float depth_over = (max(inv_depth_llc, inv_depth_nnc) * 1.00000202655792236328125f) - inv_z0;
                                        float depth_under = inv_z0 - min(inv_depth_llc, inv_depth_nnc);
                                        bool shadow_hit0 = (depth_over < 0.0f) ? (depth_under < shadow_eps) : false;
                                        float res_over;
                                        float res_under;
                                        float res_eps;
                                        float res_t;
                                        bool res_hit;
                                        float res_prev_t;
                                        if (shadow_hit0)
                                        {
                                            res_over = depth_over;
                                            res_under = depth_under;
                                            res_eps = shadow_eps;
                                            res_t = march_t0;
                                            res_hit = shadow_hit0;
                                            res_prev_t = 0.0f;
                                        }
                                        else
                                        {
                                            float prev_t;
                                            prev_t = march_t0;
                                            bool shadow_hit;
                                            float t;
                                            float loop_over;
                                            float loop_under;
                                            float loop_eps;
                                            float loop_t;
                                            bool loop_hit;
                                            uint march_i = 1u;
                                            bool shadow_hit_prev = shadow_hit0;
                                            for (;;)
                                            {
                                                if (march_i < march_steps)
                                                {
                                                    t = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(march_i) + blue_noise_x) * inv_steps, 1.0f)));
                                                    float3 march_pos_i = march_enter + (march_step_vec * t);
                                                    float2 march_uv_i = (march_pos_i.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float inv_z_i = 1.0f / march_pos_i.z;
                                                    float inv_depth_llc_i = 1.0f / depth_tex.SampleLevel(sampler_llc, march_uv_i, 0.0f).x;
                                                    float inv_depth_nnc_i = 1.0f / depth_tex.SampleLevel(sampler_nnc, march_uv_i, 0.0f).x;
                                                    float depth_over_i = (max(inv_depth_llc_i, inv_depth_nnc_i) * 1.00000202655792236328125f) - inv_z_i;
                                                    float depth_under_i = inv_z_i - min(inv_depth_llc_i, inv_depth_nnc_i);
                                                    shadow_hit = (depth_over_i < 0.0f) ? (depth_under_i < shadow_eps) : false;
                                                    if (shadow_hit)
                                                    {
                                                        loop_over = depth_over_i;
                                                        loop_under = depth_under_i;
                                                        loop_eps = shadow_eps;
                                                        loop_t = t;
                                                        loop_hit = shadow_hit;
                                                        break;
                                                    }
                                                    march_i++;
                                                    shadow_hit_prev = shadow_hit;
                                                    prev_t = t;
                                                    continue;
                                                }
                                                else
                                                {
                                                    loop_over = 0.0f;
                                                    loop_under = 0.0f;
                                                    loop_eps = 0.0f;
                                                    loop_t = 1.0f;
                                                    loop_hit = shadow_hit_prev;
                                                    break;
                                                }
                                            }
                                            res_over = loop_over;
                                            res_under = loop_under;
                                            res_eps = loop_eps;
                                            res_t = loop_t;
                                            res_hit = loop_hit;
                                            res_prev_t = prev_t;
                                        }
                                        march_over = res_over;
                                        march_under = res_under;
                                        march_eps2 = res_eps;
                                        march_t2 = res_t;
                                        march_hit = res_hit;
                                        march_prev_t = res_prev_t;
                                    }
                                    else
                                    {
                                        march_over = 0.0f;
                                        march_under = 0.0f;
                                        march_eps2 = 0.0f;
                                        march_t2 = 1.0f;
                                        march_hit = false;
                                        march_prev_t = 0.0f;
                                    }
                                    if (march_hit)
                                    {
                                        march_t_ret = march_t2;
                                        near_ret = march_over;
                                        far_ret = march_eps2;
                                        eps_ret = march_under;
                                        hit_ret = true;
                                        break;
                                    }
                                    else
                                    {
                                        march_t_ret = march_prev_t;
                                        near_ret = march_near;
                                        far_ret = march_far;
                                        eps_ret = march_eps;
                                        hit_ret = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool hit_less;
                                if (hit_ret)
                                {
                                    bool hit_cmp;
                                    if (eps_ret < far_ret)
                                    {
                                        hit_cmp = near_ret < far_ret;
                                    }
                                    else
                                    {
                                        hit_cmp = false;
                                    }
                                    hit_less = hit_cmp;
                                }
                                else
                                {
                                    hit_less = false;
                                }
                                if (hit_less)
                                {
                                    hit_final = true;
                                    t_final = march_t_ret;
                                    shadow_t_scale = eps_ret / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                hit_final = false;
                                t_final = march_prev_t;
                                shadow_t_scale = 0.0f;
                                break;
                            } while(false);
                            float3 lantern_shadowed;
                            if (hit_final)
                            {
                                float4 shadow_world_h = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(march_enter, march_exit, t_final.xxx), 1.0f)));
                                float3 shadow_vec = ((shadow_world_h.xyz / shadow_world_h.w.xxx) + (view_dir * shadow_t_scale)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(light_idx * 4 + 0) * 16 + 0);
                                bool shadow_outside;
                                if (!(abs(shadow_vec.y) > 0.5f))
                                {
                                    float2 shadow_vec_xz = shadow_vec.xz;
                                    shadow_outside = dot(shadow_vec_xz, shadow_vec_xz) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    shadow_outside = true;
                                }
                                bool3 shadow_outside3 = shadow_outside.xxx;
                                lantern_shadowed = float3(shadow_outside3.x ? 0.0f.xxx.x : lantern_contrib.x, shadow_outside3.y ? 0.0f.xxx.y : lantern_contrib.y, shadow_outside3.z ? 0.0f.xxx.z : lantern_contrib.z);
                            }
                            else
                            {
                                lantern_shadowed = lantern_contrib;
                            }
                            acc_near = near_ret;
                            acc_far = far_ret;
                            acc_eps = eps_ret;
                            acc_contrib = lantern_shadowed;
                        }
                        else
                        {
                            acc_near = march_near;
                            acc_far = march_far;
                            acc_eps = march_eps;
                            acc_contrib = lantern_contrib;
                        }
                        float2 hash_coord = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(light_idx * 4 + 0) * 10u));
                        int2 hash_cell = int2(floor(hash_coord));
                        float2 hash_frac = frac(hash_coord);
                        uint2 hash_cell_u = uint2(hash_cell);
                        uint h_o_y = hash_cell_u.y;
                        uint h_o1 = h_o_y + (h_o_y << 10u);
                        uint h_o2 = h_o1 ^ (h_o1 >> 6u);
                        uint h_o3 = h_o2 + (h_o2 << 3u);
                        uint h_o4 = h_o3 ^ (h_o3 >> 11u);
                        uint h_o5 = (((hash_cell_u.x * 1664525u) + (h_o4 + (h_o4 << 15u))) + 1013904223u) * 1664525u;
                        uint h_o6 = h_o5 ^ (h_o5 >> 11u);
                        uint h_o7 = h_o6 ^ ((h_o6 << 7u) & 2636928640u);
                        uint h_o8 = h_o7 ^ ((h_o7 << 15u) & 4022730752u);
                        bool keep_o = valid_hit ? valid_hit : true;
                        uint keep_o_bits = valid_hit ? hash_hit : 8388607u;
                        bool hit_o = valid_keep ? valid_keep : true;
                        uint hit_o_bits = valid_keep ? hash_keep : 1065353216u;
                        uint2 cell_px = uint2(hash_cell + int2(1, 0));
                        uint h_px_y = cell_px.y;
                        uint h_px1 = h_px_y + (h_px_y << 10u);
                        uint h_px2 = h_px1 ^ (h_px1 >> 6u);
                        uint h_px3 = h_px2 + (h_px2 << 3u);
                        uint h_px4 = h_px3 ^ (h_px3 >> 11u);
                        uint h_px5 = (((cell_px.x * 1664525u) + (h_px4 + (h_px4 << 15u))) + 1013904223u) * 1664525u;
                        uint h_px6 = h_px5 ^ (h_px5 >> 11u);
                        uint h_px7 = h_px6 ^ ((h_px6 << 7u) & 2636928640u);
                        uint h_px8 = h_px7 ^ ((h_px7 << 15u) & 4022730752u);
                        bool keep_px = keep_o ? keep_o : true;
                        uint keep_px_bits = keep_o ? keep_o_bits : 8388607u;
                        bool hit_px = hit_o ? hit_o : true;
                        uint hit_px_bits = hit_o ? hit_o_bits : 1065353216u;
                        uint2 cell_py = uint2(hash_cell + int2(0, 1));
                        uint h_py_y = cell_py.y;
                        uint h_py1 = h_py_y + (h_py_y << 10u);
                        uint h_py2 = h_py1 ^ (h_py1 >> 6u);
                        uint h_py3 = h_py2 + (h_py2 << 3u);
                        uint h_py4 = h_py3 ^ (h_py3 >> 11u);
                        uint h_py5 = (((cell_py.x * 1664525u) + (h_py4 + (h_py4 << 15u))) + 1013904223u) * 1664525u;
                        uint h_py6 = h_py5 ^ (h_py5 >> 11u);
                        uint h_py7 = h_py6 ^ ((h_py6 << 7u) & 2636928640u);
                        uint h_py8 = h_py7 ^ ((h_py7 << 15u) & 4022730752u);
                        bool keep_py = keep_px ? keep_px : true;
                        uint keep_py_bits = keep_px ? keep_px_bits : 8388607u;
                        bool hit_py = hit_px ? hit_px : true;
                        uint hit_py_bits = hit_px ? hit_px_bits : 1065353216u;
                        uint2 cell_pxy = uint2(hash_cell + int2(1, 1));
                        uint h_pxy_y = cell_pxy.y;
                        uint h_pxy1 = h_pxy_y + (h_pxy_y << 10u);
                        uint h_pxy2 = h_pxy1 ^ (h_pxy1 >> 6u);
                        uint h_pxy3 = h_pxy2 + (h_pxy2 << 3u);
                        uint h_pxy4 = h_pxy3 ^ (h_pxy3 >> 11u);
                        uint h_pxy5 = (((cell_pxy.x * 1664525u) + (h_pxy4 + (h_pxy4 << 15u))) + 1013904223u) * 1664525u;
                        uint h_pxy6 = h_pxy5 ^ (h_pxy5 >> 11u);
                        uint h_pxy7 = h_pxy6 ^ ((h_pxy6 << 7u) & 2636928640u);
                        uint h_pxy8 = h_pxy7 ^ ((h_pxy7 << 15u) & 4022730752u);
                        uint keep_pxy_bits = keep_py ? keep_py_bits : 8388607u;
                        uint hit_pxy_bits = hit_py ? hit_py_bits : 1065353216u;
                        float2 smooth = (hash_frac * hash_frac) * (3.0f.xx - (hash_frac * 2.0f));
                        float smooth_x = smooth.x;
                        float om_smooth_x = 1.0f - smooth_x;
                        float smooth_y = smooth.y;
                        next_near = acc_near;
                        next_far = acc_far;
                        next_eps = acc_eps;
                        next_hash_keep = hit_pxy_bits;
                        next_hash_hit = keep_pxy_bits;
                        next_valid_keep = hit_py ? hit_py : true;
                        next_valid_hit = keep_py ? keep_py : true;
                        lantern_light = (((acc_contrib * lerp(0.5f, 1.0f, (((1.0f - smooth_y) * (((asfloat(((h_o8 ^ (h_o8 >> 18u)) & keep_o_bits) | hit_o_bits) - 1.0f) * om_smooth_x) + ((asfloat(((h_px8 ^ (h_px8 >> 18u)) & keep_px_bits) | hit_px_bits) - 1.0f) * smooth_x))) + (((asfloat(((h_py8 ^ (h_py8 >> 18u)) & keep_py_bits) | hit_py_bits) - 1.0f) * om_smooth_x) * smooth_y)) + (((asfloat(((h_pxy8 ^ (h_pxy8 >> 18u)) & keep_pxy_bits) | hit_pxy_bits) - 1.0f) * smooth_x) * smooth_y))) * 1.0f) * albedo_masked) * ev_mult;
                    }
                    else
                    {
                        next_near = march_near;
                        next_far = march_far;
                        next_eps = march_eps;
                        next_hash_keep = hash_keep;
                        next_hash_hit = hash_hit;
                        next_valid_keep = valid_keep;
                        next_valid_hit = valid_hit;
                        lantern_light = 0.0f.xxx;
                    }
                    next_light_acc = light_acc + lantern_light;
                }
                grid_light_result = light_acc;
            }
            else
            {
                grid_light_result = diffuse;
            }
            lantern_light2 = grid_light_result;
        }
        out_color = lantern_light2;
    }
    else
    {
        out_color = diffuse;
    }
    out_var_SV_TARGET = float4(out_color, 0.0f);
    return out_var_SV_TARGET;
}
#else
// f0ad: pixel stage lifted from _deferred_plant_light_sun_sky.raster.hlsl_f0adff76827e73ff.bin (spirv-cross, mechanically renamed;
// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.

void ps_main(VsToPs input)
{
    int2 pixel_xy2 = int2(input.position.xy);
    float4 view_pos_h2 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4((((float2(pixel_xy2) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(pixel_xy2), 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 grid_cell2 = uint3(clamp(floor(((view_pos_h2.xyz / view_pos_h2.w.xxx) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * grid_cell2.y) + grid_cell2.z)) + grid_cell2.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint res_ids8[8];
            for (uint id_init_i = 0u; id_init_i < 8u; )
            {
                res_ids8[id_init_i] = 4294967295u;
                id_init_i++;
                continue;
            }
            uint2 quad_xy = uint2(pixel_xy2 & int2(1, 1));
            uint quad_idx = (quad_xy.x + (quad_xy.y * 2u)) & 3u;
            uint slot_count;
            slot_count = 0u;
            uint next_slot_count;
            uint tap_i = 0u;
            for (;;)
            {
                bool have_slot;
                if (tap_i < 8u)
                {
                    have_slot = slot_count < 8u;
                }
                else
                {
                    have_slot = false;
                }
                if (have_slot)
                {
                    uint4 res_sample = rdi_reservoir_tex.Load(int3(uint2((pixel_xy2 / int2(2, 2)) + spatial_resolve_offsets_array[((quad_idx * 16u) + tap_i) + 256u].xy), 0u));
                    uint2 res_sample_xy = res_sample.xy;
                    uint res_sample_id = res_sample.x;
                    bool res_valid;
                    if (any(bool2(res_sample_xy.x != uint2(0u, 0u).x, res_sample_xy.y != uint2(0u, 0u).y)))
                    {
                        res_valid = res_sample_id < lantern_count;
                    }
                    else
                    {
                        res_valid = false;
                    }
                    bool dup1;
                    if (res_valid)
                    {
                        dup1 = res_ids8[0u] != res_sample_id;
                    }
                    else
                    {
                        dup1 = false;
                    }
                    bool dup2;
                    if (dup1)
                    {
                        dup2 = res_ids8[1u] != res_sample_id;
                    }
                    else
                    {
                        dup2 = false;
                    }
                    bool dup3;
                    if (dup2)
                    {
                        dup3 = res_ids8[2u] != res_sample_id;
                    }
                    else
                    {
                        dup3 = false;
                    }
                    bool dup4;
                    if (dup3)
                    {
                        dup4 = res_ids8[3u] != res_sample_id;
                    }
                    else
                    {
                        dup4 = false;
                    }
                    bool dup5;
                    if (dup4)
                    {
                        dup5 = res_ids8[4u] != res_sample_id;
                    }
                    else
                    {
                        dup5 = false;
                    }
                    bool dup6;
                    if (dup5)
                    {
                        dup6 = res_ids8[5u] != res_sample_id;
                    }
                    else
                    {
                        dup6 = false;
                    }
                    bool dup7;
                    if (dup6)
                    {
                        dup7 = res_ids8[6u] != res_sample_id;
                    }
                    else
                    {
                        dup7 = false;
                    }
                    bool dup8;
                    if (dup7)
                    {
                        dup8 = res_ids8[7u] != res_sample_id;
                    }
                    else
                    {
                        dup8 = false;
                    }
                    if (dup8)
                    {
                        res_ids8[slot_count] = res_sample_id;
                        next_slot_count = slot_count + 1u;
                    }
                    else
                    {
                        next_slot_count = slot_count;
                    }
                    slot_count = next_slot_count;
                    tap_i++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint next_merge_count;
            for (uint merge_count = slot_count, q_i = 0u; q_i < 8u; merge_count = next_merge_count, q_i++)
            {
                uint quad_id = QuadReadAcrossDiagonal(res_ids8[q_i]);
                bool quad_add;
                if (quad_id < lantern_count)
                {
                    quad_add = merge_count < 8u;
                }
                else
                {
                    quad_add = false;
                }
                if (quad_add)
                {
                    bool qdup1;
                    if (res_ids8[0u] != quad_id)
                    {
                        qdup1 = res_ids8[1u] != quad_id;
                    }
                    else
                    {
                        qdup1 = false;
                    }
                    bool qdup2;
                    if (qdup1)
                    {
                        qdup2 = res_ids8[2u] != quad_id;
                    }
                    else
                    {
                        qdup2 = false;
                    }
                    bool qdup3;
                    if (qdup2)
                    {
                        qdup3 = res_ids8[3u] != quad_id;
                    }
                    else
                    {
                        qdup3 = false;
                    }
                    bool qdup4;
                    if (qdup3)
                    {
                        qdup4 = res_ids8[4u] != quad_id;
                    }
                    else
                    {
                        qdup4 = false;
                    }
                    bool qdup5;
                    if (qdup4)
                    {
                        qdup5 = res_ids8[5u] != quad_id;
                    }
                    else
                    {
                        qdup5 = false;
                    }
                    bool qdup6;
                    if (qdup5)
                    {
                        qdup6 = res_ids8[6u] != quad_id;
                    }
                    else
                    {
                        qdup6 = false;
                    }
                    bool qdup7;
                    if (qdup6)
                    {
                        qdup7 = res_ids8[7u] != quad_id;
                    }
                    else
                    {
                        qdup7 = false;
                    }
                    uint next_merge2;
                    if (qdup7)
                    {
                        res_ids8[merge_count] = quad_id;
                        next_merge2 = merge_count + 1u;
                    }
                    else
                    {
                        next_merge2 = merge_count;
                    }
                    next_merge_count = next_merge2;
                }
                else
                {
                    next_merge_count = merge_count;
                }
            }
        }
        else
        {
        }
    }
}
#endif
