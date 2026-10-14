// ==RECONSTRUCTED== shaders/deferred/solid_light_sun_sky.raster.hlsl | status: O (structure from disassembly); I (flag name; $Globals member names verbatim)

// 中文说明:
//   固体固体 solid 材质的延迟"太阳+天空"光照合成(全屏 pass)。包含:
//   - 由 gbuffer+深度重建世界位置/法线(RGB8 法线解码,gbuffer.x 低 24 位);
//   - 太阳:能量守恒 GGX 漫反射 + Cook-Torrance 镜面 * sun_color * NdotL;
//   - 天空环境:间接辐射 3-slab 空间解析的 SH 带 1 求值;
//   - ReSTIR 灯笼光:reservoir 收集 + 流式采样;
//   - 网格灯:light_indirection_grid / grid_light_indices 迭代;
//   - 级联阴影 / 屏幕空间阴影 / ssgi 合成。
//   两个变体由 DEFERRED_FULL_LIGHTING 门控(b903 全 / f0ad 精简);ps_main
//   主体为 spirv-cross 转译(行为精确;已全量语义化变量名并加中文注释)。
// MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (B is the
// only side covering both b903/f0ad variants via DEFERRED_FULL_LIGHTING). Two review "merge
// deltas" were checked against the bins and REJECTED: (1) the solid b903 ps module decodes the
// gbuffer normal as RGB8 normalize of the low 24 bits of gbuffer.x (not octahedral) and (2) the
// SV_Target alpha written is 0.0 (not 2.0). Both match B as-is, so the PS body is unchanged.
// Evidence: _deferred_solid_light_sun_sky.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin.
//   b903 (full) bindings: sampler_lnc s0, sampler_nnc s2, sampler_llc s3, sampler_default s9,
//     $Globals b13, gbuffer_tex t14, depth_tex t15, half_depth_tex t16, blue_noise_texture t18,
//     ssgi_tex t19, screen_shadows_tex t20, highlight_active_hill t22, color_library b23,
//     spatial_resolve_offsets b24, lantern_buf t26, indirect_radiance_tex t27, rdi_reservoir_tex t28,
//     brdf_fg_lut t29, light_indirection_grid t36, grid_light_indices t37, frame_constants set1/b0;
//   f0ad (reduced) bindings: $Globals b13, gbuffer_tex t14, depth_tex t15, spatial_resolve_offsets b24,
//     rdi_reservoir_tex t28, light_indirection_grid t36, frame_constants set1/b0.
//   vs_main = fullscreen triangle; ps_main = deferred sun+sky+lantern composition. The two hex
//   variants are compile-time feature variants: DEFERRED_FULL_LIGHTING=1 -> b903, =0 -> f0ad
//   (reservoir-gather only, no color output). The gather keeps quad/wave intrinsics so both
//   variants carry GroupNonUniformVote/Quad capabilities.
// Status: O (structure from disassembly); I (flag name; $Globals member names verbatim).

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
#include "inc/material/eval/solid_material.inc.hlsl"
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
    uint2 brdf_fg_lut_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
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
Texture2D<float4> highlight_active_hill : register(t22, space0);
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> brdf_fg_lut : register(t29, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


#if DEFERRED_FULL_LIGHTING
// b903: pixel stage lifted from _deferred_solid_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (spirv-cross, semanticized
// 命名 + 中文注释;行为与原版逐指令一致,readable_check 校验通过)。
static float hit_miss_default;
static uint unused_u;
static float unused_f;
static const float4 sh_zero_init[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState res_stream_init = { 0.0f, 0.0f };
static const Reservoir1spp res_init = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    float4 out_var_SV_TARGET;
    int2 pixel_i2 = int2(input.position.xy);
    uint2 pixel_u = uint2(pixel_i2);
    float4 blue_noise_smp = ((blue_noise_texture.Load(int3((pixel_u + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float rand_frac = frac(frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(pixel_u.x)) + (0.005837149918079376220703125f * float(pixel_u.y)))) + (float(frame_index % 4u) * 0.25f));
    uint4 gb0 = gbuffer_tex.Load(int3(pixel_u, 0u));
    float4 depth_smp = depth_tex.Load(int3(pixel_u, 0u));
    float depth = depth_smp.x;
    uint gb_norm_pack = gb0.x;
    float alpha8 = float((gb_norm_pack >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    uint gb_rough_pack = gb0.y;
    float3 normal_rgb = normalize((float4(float(gb_norm_pack & 255u) * 0.0039215688593685626983642578125f, float((gb_norm_pack >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gb_norm_pack >> 16u) & 255u) * 0.0039215688593685626983642578125f, alpha8).xyz * 2.0f) - 1.0f.xxx);
    float3 albedo_srgb = float4(float(gb_rough_pack & 255u) * 0.0039215688593685626983642578125f, float((gb_rough_pack >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gb_rough_pack >> 16u) & 255u) * 0.0039215688593685626983642578125f, unused_f).xyz;
    float3 albedo = albedo_srgb * albedo_srgb;
    uint gb_flags = gb_rough_pack >> 24u;
    bool flag_bits0 = ((gb_flags >> 0u) & 1u) != 0u;
    bool flag_bits1 = ((gb_flags >> 1u) & 1u) != 0u;
    bool flag_bits6 = ((gb_flags >> 6u) & 1u) != 0u;
    float3 view_dir = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 depth_dims_f = float2(depth_tex_dims);
    float2 ndc = (float2(pixel_i2) + 0.5f.xx) / depth_dims_f;
    float2 ndc_scaled = (ndc - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc_scaled.x;
    float4 view_pos4 = float4(ndc_x, ndc_scaled.y, depth, 1.0f);
    float4 sample_pos = mul(view_constants[0u].sample_to_view, view_pos4);
    float4 world_pos = mul(view_constants[0u].view_to_world, sample_pos);
    float3 view_ray = normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(ndc_x, ndc_scaled.y, 0.0f, 1.0f))).xyz);
    float3 view_ray_n = -view_ray;
    float3 probe_pos = world_pos.xyz / world_pos.w.xxx;
    float ssgi_occ = clamp(0.100000001490116119384765625f + ssgi_tex.SampleLevel(sampler_nnc, ndc, 0.0f).x, 0.0f, 1.0f);
    float shadow_falloff;
    if (flag_bits0)
    {
        float3 view_pos_xyz = view_pos4.xyz;
        float4 w2s_pos = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(probe_pos + (view_dir * 0.300000011920928955078125f), 1.0f)));
        float3 ray_dir_near = w2s_pos.xyz / w2s_pos.w.xxx;
        float4 s_enter = view_pos4 + (float4(ray_dir_near - view_pos_xyz, 0.0f) * float(int(sign(ray_dir_near.z))));
        float s_enter_w = s_enter.w;
        float3 s_exit = (s_enter / (float((s_enter_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(s_enter_w))).xxxx).xyz;
        float3 dir_abs = s_exit - view_pos_xyz;
        bool3 side_flags = bool3(dir_abs.x < 0.0f.xxx.x, dir_abs.y < 0.0f.xxx.y, dir_abs.z < 0.0f.xxx.z);
        float3 max_step = (float3(side_flags.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos_xyz) / dir_abs;
        float3 probe_march = view_pos_xyz + (dir_abs * max(0.0f, max(max_step.x, max_step.y)));
        float3 probe_dir = s_exit - probe_march;
        bool3 side_flags2 = bool3(probe_dir.x >= 0.0f.xxx.x, probe_dir.y >= 0.0f.xxx.y, probe_dir.z >= 0.0f.xxx.z);
        float3 max_step2 = (float3(side_flags2.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags2.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags2.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - probe_march) / probe_dir;
        float3 probe_march2 = probe_march + (probe_dir * min(1.0f, min(min(max_step2.x, max_step2.y), max_step2.z)));
        bool hit_t_ratio;
        float hit_smooth;
        do
        {
            float shadow_bias = 0.5f * view_constants[0u].clip_to_view[3].z;
            uint shadow_steps = uint(int(max(2u, min(3u, uint(int(floor(length((((probe_march2.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((probe_march.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
            float hit_t2;
            float hit_t_c2;
            float hit_flag2;
            bool hit_flag;
            do
            {
                float3 shadow_dir = probe_march2 - probe_march;
                float inv_steps;
                if (probe_march2.z > 9.5367431640625e-07f)
                {
                    inv_steps = float(shadow_steps);
                }
                else
                {
                    inv_steps = max(1.0f, float(shadow_steps) - 1.0f);
                }
                float inv_steps2 = 1.0f / inv_steps;
                float prev_t;
                float prev_t_o;
                float t_o;
                bool t_c;
                if (shadow_steps > 0u)
                {
                    float3 shadow_sample = probe_march + (shadow_dir * lerp(0.0f, 1.0f, pow(rand_frac * inv_steps2, 1.0f)));
                    float2 probe_uv = (shadow_sample.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    float probe_invz = 1.0f / shadow_sample.z;
                    float probe_depth_llc = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv, 0.0f).x;
                    float probe_depth_nnc = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv, 0.0f).x;
                    float depth_diff_max = (max(probe_depth_llc, probe_depth_nnc) * 1.00000202655792236328125f) - probe_invz;
                    float depth_diff_min = probe_invz - min(probe_depth_llc, probe_depth_nnc);
                    bool hit_front = (depth_diff_max < 0.0f) ? (depth_diff_min < shadow_bias) : false;
                    float dmax_c;
                    float dmin_c;
                    float bias_c;
                    bool flag_c;
                    if (hit_front)
                    {
                        dmax_c = depth_diff_max;
                        dmin_c = depth_diff_min;
                        bias_c = shadow_bias;
                        flag_c = hit_front;
                    }
                    else
                    {
                        bool depth_hit2;
                        float dmax_o;
                        float dmin_o;
                        float bias_o;
                        bool flag_o;
                        uint step_i = 1u;
                        bool hit_any = hit_front;
                        for (;;)
                        {
                            if (step_i < shadow_steps)
                            {
                                float3 shadow_sample2 = probe_march + (shadow_dir * lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + rand_frac) * inv_steps2, 1.0f))));
                                float2 probe_uv2 = (shadow_sample2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float probe_invz2 = 1.0f / shadow_sample2.z;
                                float probe_depth_llc2 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv2, 0.0f).x;
                                float probe_depth_nnc2 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv2, 0.0f).x;
                                float depth_diff_max2 = (max(probe_depth_llc2, probe_depth_nnc2) * 1.00000202655792236328125f) - probe_invz2;
                                float depth_diff_min2 = probe_invz2 - min(probe_depth_llc2, probe_depth_nnc2);
                                depth_hit2 = (depth_diff_max2 < 0.0f) ? (depth_diff_min2 < shadow_bias) : false;
                                if (depth_hit2)
                                {
                                    dmax_o = depth_diff_max2;
                                    dmin_o = depth_diff_min2;
                                    bias_o = shadow_bias;
                                    flag_o = depth_hit2;
                                    break;
                                }
                                step_i++;
                                hit_any = depth_hit2;
                                continue;
                            }
                            else
                            {
                                dmax_o = 0.0f;
                                dmin_o = 0.0f;
                                bias_o = 0.0f;
                                flag_o = hit_any;
                                break;
                            }
                        }
                        dmax_c = dmax_o;
                        dmin_c = dmin_o;
                        bias_c = bias_o;
                        flag_c = flag_o;
                    }
                    prev_t = dmax_c;
                    prev_t_o = dmin_c;
                    t_o = bias_c;
                    t_c = flag_c;
                }
                else
                {
                    prev_t = 0.0f;
                    prev_t_o = 0.0f;
                    t_o = 0.0f;
                    t_c = false;
                }
                if (t_c)
                {
                    hit_t2 = prev_t;
                    hit_t_c2 = t_o;
                    hit_flag2 = prev_t_o;
                    hit_flag = true;
                    break;
                }
                else
                {
                    hit_t2 = hit_miss_default;
                    hit_t_c2 = hit_miss_default;
                    hit_flag2 = hit_miss_default;
                    hit_flag = false;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            bool hit_better;
            if (hit_flag)
            {
                bool hit_better_gray;
                if (hit_flag2 < hit_t_c2)
                {
                    hit_better_gray = hit_t2 < hit_t_c2;
                }
                else
                {
                    hit_better_gray = false;
                }
                hit_better = hit_better_gray;
            }
            else
            {
                hit_better = false;
            }
            if (hit_better)
            {
                hit_t_ratio = true;
                hit_smooth = hit_flag2 / hit_t_c2;
                break;
            }
            hit_t_ratio = false;
            hit_smooth = 0.0f;
            break;
        } while(false);
        float smooth_hit;
        if (hit_t_ratio)
        {
            smooth_hit = smoothstep(1.0f, 0.5f, hit_smooth);
        }
        else
        {
            smooth_hit = 0.0f;
        }
        shadow_falloff = smooth_hit;
    }
    else
    {
        shadow_falloff = 0.0f;
    }
    bool3 norm_ovr_flag = ((render_overrides & 1u) != 0u).xxx;
    float3 albedo_use = float3(norm_ovr_flag.x ? 0.5f.xxx.x : albedo.x, norm_ovr_flag.y ? 0.5f.xxx.y : albedo.y, norm_ovr_flag.z ? 0.5f.xxx.z : albedo.z);
    float3 normal_n = normalize(normal_rgb);
    float ndotv = clamp(dot(normal_n, view_dir), 0.0f, 1.0f);
    float fresnel_o = clamp(dot(view_dir, normalize(view_dir + view_ray_n)), 0.0f, 1.0f);
    float roughness2 = alpha8 * alpha8;
    float ao_mix;
    if (flag_bits6)
    {
        ao_mix = 0.20000000298023223876953125f + (ndotv * 0.60000002384185791015625f);
    }
    else
    {
        ao_mix = ndotv;
    }
    float ggx_lobe = (((2.0f * roughness2) * fresnel_o) * fresnel_o) + (-0.5f);
    float fresnel2 = 1.0f - ao_mix;
    float fresnel4 = fresnel2 * fresnel2;
    float ndotv2 = 1.0f - max(dot(normal_n, view_ray_n), 9.9999997473787516355514526367188e-05f);
    float ndotv4 = ndotv2 * ndotv2;
    float3 zero_light;
    do
    {
        float ndotl = dot(normal_rgb, view_dir);
        bool backface;
        if (!(ndotl <= 0.0f))
        {
            backface = dot(normal_rgb, view_ray_n) <= 0.0f;
        }
        else
        {
            backface = true;
        }
        if (backface)
        {
            zero_light = 0.0f.xxx;
            break;
        }
        float roughness4 = roughness2 * roughness2;
        float3 half_dir = normalize(view_ray_n + view_dir);
        float ndoth = dot(normal_rgb, half_dir);
        float max_nv = max(0.0f, 1.0f - dot(half_dir, view_dir));
        float h_nd_half = max_nv * max_nv;
        float rim_ndotv2 = dot(normal_rgb, view_ray_n);
        float gterm_half = ((ndoth * ndoth) * (roughness4 - 1.0f)) + 1.0f;
        zero_light = ((lerp(0.039999999105930328369140625f.xxx, 1.0f.xxx, ((h_nd_half * h_nd_half) * max_nv).xxx) * (((2.0f * ndotl) * rim_ndotv2) / ((ndotl * sqrt(((((-rim_ndotv2) * roughness4) + rim_ndotv2) * rim_ndotv2) + roughness4)) + (rim_ndotv2 * sqrt(((((-ndotl) * roughness4) + ndotl) * ndotl) + roughness4))))) * (roughness4 / ((3.1415927410125732421875f * gterm_half) * gterm_half))) / ((4.0f * rim_ndotv2) * ndotl).xxx;
        break;
    } while(false);
    float3 lit_base = ((sun_color.xyz * ((albedo_use * (((1.0f + (ggx_lobe * ((fresnel4 * fresnel4) * fresnel2))) * (1.0f + (ggx_lobe * ((ndotv4 * ndotv4) * ndotv2)))) * 0.3183098733425140380859375f)) + zero_light)) * ao_mix) * 1.0f;
    float3 lit_shadow;
    if (((gb_flags >> 2u) & 1u) != 0u)
    {
        lit_shadow = lit_base * ((1.0f - shadow_falloff) * (1.0f - screen_shadows_tex.Load(int3(pixel_u, 0u)).x));
    }
    else
    {
        lit_shadow = lit_base;
    }
    float3 lit_ssgi;
    if (((gb_flags >> 5u) & 1u) != 0u)
    {
        lit_ssgi = lit_shadow * lerp(0.60000002384185791015625f, 1.0f, ssgi_occ);
    }
    else
    {
        lit_ssgi = lit_shadow;
    }
    float3 lit_plus;
    [branch]
    if (((gb_flags >> 4u) & 1u) != 0u)
    {
        float3 zero_light2;
        do
        {
            bool far_plane;
            if (!(abs(probe_pos.x) > 65.0f))
            {
                far_plane = abs(probe_pos.z) > 65.0f;
            }
            else
            {
                far_plane = true;
            }
            if (far_plane)
            {
                zero_light2 = 0.0f.xxx;
                break;
            }
            zero_light2 = color_library.terrain_highlight.xyz * highlight_active_hill.SampleLevel(sampler_default, (probe_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x;
            break;
        } while(false);
        lit_plus = lit_ssgi + zero_light2;
    }
    else
    {
        lit_plus = lit_ssgi;
    }
    float3 lit_editing;
    if (((gb_flags >> 3u) & 1u) != 0u)
    {
        lit_editing = lit_plus + ((editing_highlight_color * dot(sky_high_color.xyz, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))) * 2.0f);
    }
    else
    {
        lit_editing = lit_plus;
    }
    float4 count_ok_q[3] = sh_zero_init;
    float bn_r = blue_noise_smp.x;
    uint sel = uint((bn_r * 3.0f) + 0.5f) & 3u;
    float acc_w;
    acc_w = 0.0f;
    float acc_w_next;
    for (uint band_i = 0u; band_i < 3u; acc_w = acc_w_next, band_i++)
    {
        uint2 ss_off = (pixel_u / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(sel * 16u) + band_i].xy);
        float depth_w = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(ss_off, 0u)).x / depth))));
        for (uint ch_i = 0u; ch_i < 3u; )
        {
            count_ok_q[ch_i] += (indirect_radiance_tex.Load(int3(ss_off + uint2((ch_i * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * depth_w);
            ch_i++;
            continue;
        }
        acc_w_next = acc_w + depth_w;
    }
    float weight_sum;
    if (acc_w < 0.001000000047497451305389404296875f)
    {
        float acc_w_ext;
        acc_w_ext = acc_w;
        float acc_w_ext_next;
        for (uint band_i_ext = 3u; band_i_ext < 6u; acc_w_ext = acc_w_ext_next, band_i_ext++)
        {
            uint2 ss_off_ext = (pixel_u / uint2(2u, 2u)) + uint2(spatial_resolve_offsets_array[(sel * 16u) + band_i_ext].xy);
            float depth_w_ext = exp2(-min(30.0f, 80.0f * abs(1.0f - (half_depth_tex.Load(int3(ss_off_ext, 0u)).x / depth))));
            for (uint ch_i_ext = 0u; ch_i_ext < 3u; )
            {
                count_ok_q[ch_i_ext] += (indirect_radiance_tex.Load(int3(ss_off_ext + uint2((ch_i_ext * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * depth_w_ext);
                ch_i_ext++;
                continue;
            }
            acc_w_ext_next = acc_w_ext + depth_w_ext;
        }
        weight_sum = acc_w_ext;
    }
    else
    {
        weight_sum = acc_w;
    }
    float inv_wsum = 1.0f / max(9.9999997473787516355514526367188e-05f, weight_sum);
    count_ok_q[0] *= inv_wsum;
    count_ok_q[1] *= inv_wsum;
    count_ok_q[2] *= inv_wsum;
    float3 sh_irr = float3(count_ok_q[0].x, count_ok_q[1].x, count_ok_q[2].x);
    float3 sh0 = count_ok_q[0].yzw / max(9.9999996826552253889678874634872e-21f, count_ok_q[0].x).xxx;
    float3 sh1 = count_ok_q[1].yzw / max(9.9999996826552253889678874634872e-21f, count_ok_q[1].x).xxx;
    float3 sh2 = count_ok_q[2].yzw / max(9.9999996826552253889678874634872e-21f, count_ok_q[2].x).xxx;
    float3 ind_tint_inv = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 ind_light = max(0.0f.xxx, lerp((ind_tint_inv / dot(ind_tint_inv, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(sh_irr, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), sh_irr, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    float ss_ray_bias = 0.004999999888241291046142578125f / (((-0.5f) / (sample_pos.xyz / sample_pos.w.xxx).z) * view_constants[0u].view_to_clip[1].y);
    float2 bn_shuffle[4] = { blue_noise_smp.xw, blue_noise_smp.yz, blue_noise_smp.zy, blue_noise_smp.wx };
    float2 res_rand_offsets[4] = bn_shuffle;
    float acc_d0;
    acc_d0 = 0.0f;
    float acc_d0_next;
    float hit_dmin_r_next;
    float hit_bias_r_next;
    float hit_t_r_next;
    float hit_dmin_r = 0.0f;
    float hit_bias_r = 0.0f;
    float hit_t_r = 0.0f;
    for (uint acc_i2 = 0u; acc_i2 < 2u; hit_dmin_r = hit_dmin_r_next, hit_bias_r = hit_bias_r_next, hit_t_r = hit_t_r_next, acc_d0 = acc_d0_next, acc_i2++)
    {
        float rand_ss = 1.0f - (2.0f * res_rand_offsets[acc_i2].x);
        float rand_ss_angle = sqrt(max(0.0f, 1.0f - (rand_ss * rand_ss)));
        float rand_angle = 6.283185482025146484375f * res_rand_offsets[acc_i2].y;
        float3 view_pos_xyz2 = view_pos4.xyz;
        float4 w2s_pos2 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(probe_pos + (normalize(float3(cos(rand_angle) * rand_ss_angle, sin(rand_angle) * rand_ss_angle, rand_ss) + normal_rgb) * ss_ray_bias), 1.0f)));
        float3 ray_dir_near2 = w2s_pos2.xyz / w2s_pos2.w.xxx;
        float4 s_enter2 = view_pos4 + (float4(ray_dir_near2 - view_pos_xyz2, 0.0f) * float(int(sign(ray_dir_near2.z))));
        float s_enter_w2 = s_enter2.w;
        float3 s_exit2 = (s_enter2 / (float((s_enter_w2 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(s_enter_w2))).xxxx).xyz;
        float3 dir_abs2 = s_exit2 - view_pos_xyz2;
        bool3 side_flags3 = bool3(dir_abs2.x < 0.0f.xxx.x, dir_abs2.y < 0.0f.xxx.y, dir_abs2.z < 0.0f.xxx.z);
        float3 max_step3 = (float3(side_flags3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos_xyz2) / dir_abs2;
        float3 probe_march3 = view_pos_xyz2 + (dir_abs2 * max(0.0f, max(max_step3.x, max_step3.y)));
        float3 probe_dir2 = s_exit2 - probe_march3;
        bool3 side_flags4 = bool3(probe_dir2.x >= 0.0f.xxx.x, probe_dir2.y >= 0.0f.xxx.y, probe_dir2.z >= 0.0f.xxx.z);
        float3 max_step4 = (float3(side_flags4.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags4.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags4.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - probe_march3) / probe_dir2;
        float3 probe_march4 = probe_march3 + (probe_dir2 * min(1.0f, min(min(max_step4.x, max_step4.y), max_step4.z)));
        bool hit_t3;
        float hit_t_ratio2;
        do
        {
            float shadow_bias2 = 0.300000011920928955078125f * view_constants[0u].clip_to_view[3].z;
            uint shadow_steps2 = uint(int(max(2u, min(2u, uint(int(floor(length((((probe_march4.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((probe_march3.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
            bool hit_flag3;
            do
            {
                float3 shadow_dir2 = probe_march4 - probe_march3;
                float inv_steps3;
                if (probe_march4.z > 9.5367431640625e-07f)
                {
                    inv_steps3 = float(shadow_steps2);
                }
                else
                {
                    inv_steps3 = max(1.0f, float(shadow_steps2) - 1.0f);
                }
                float inv_steps4 = 1.0f / inv_steps3;
                float prev_t2;
                float prev_t_o2;
                float t_o2;
                bool t_c2;
                if (shadow_steps2 > 0u)
                {
                    float3 shadow_sample3 = probe_march3 + (shadow_dir2 * lerp(0.0f, 1.0f, pow(inv_steps4, 1.0f)));
                    float2 probe_uv2r = (shadow_sample3.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                    float probe_invz2r = 1.0f / shadow_sample3.z;
                    float probe_depth_llc2r = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv2r, 0.0f).x;
                    float probe_depth_nnc2r = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv2r, 0.0f).x;
                    float depth_diff_max2r = (max(probe_depth_llc2r, probe_depth_nnc2r) * 1.00000202655792236328125f) - probe_invz2r;
                    bool hit_front2r = (depth_diff_max2r < 0.0f) ? true : false;
                    float dmax_c2;
                    float dmin_c2;
                    float bias_c2;
                    bool flag_c2;
                    if (hit_front2r)
                    {
                        dmax_c2 = depth_diff_max2r;
                        dmin_c2 = probe_invz2r - min(probe_depth_llc2r, probe_depth_nnc2r);
                        bias_c2 = shadow_bias2;
                        flag_c2 = hit_front2r;
                    }
                    else
                    {
                        bool depth_hit3;
                        float dmax_o2;
                        float dmin_o2;
                        float bias_o2;
                        bool flag_o2;
                        uint step_i2r = 1u;
                        bool hit_any2r = hit_front2r;
                        for (;;)
                        {
                            if (step_i2r < shadow_steps2)
                            {
                                float3 shadow_sample4 = probe_march3 + (shadow_dir2 * lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i2r) + 1.0f) * inv_steps4, 1.0f))));
                                float2 probe_uv3 = (shadow_sample4.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float probe_invz3 = 1.0f / shadow_sample4.z;
                                float probe_depth_llc3 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv3, 0.0f).x;
                                float probe_depth_nnc3 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv3, 0.0f).x;
                                float depth_diff_max3 = (max(probe_depth_llc3, probe_depth_nnc3) * 1.00000202655792236328125f) - probe_invz3;
                                depth_hit3 = (depth_diff_max3 < 0.0f) ? true : false;
                                if (depth_hit3)
                                {
                                    dmax_o2 = depth_diff_max3;
                                    dmin_o2 = probe_invz3 - min(probe_depth_llc3, probe_depth_nnc3);
                                    bias_o2 = shadow_bias2;
                                    flag_o2 = depth_hit3;
                                    break;
                                }
                                step_i2r++;
                                hit_any2r = depth_hit3;
                                continue;
                            }
                            else
                            {
                                dmax_o2 = 0.0f;
                                dmin_o2 = 0.0f;
                                bias_o2 = 0.0f;
                                flag_o2 = hit_any2r;
                                break;
                            }
                        }
                        dmax_c2 = dmax_o2;
                        dmin_c2 = dmin_o2;
                        bias_c2 = bias_o2;
                        flag_c2 = flag_o2;
                    }
                    prev_t2 = dmax_c2;
                    prev_t_o2 = dmin_c2;
                    t_o2 = bias_c2;
                    t_c2 = flag_c2;
                }
                else
                {
                    prev_t2 = 0.0f;
                    prev_t_o2 = 0.0f;
                    t_o2 = 0.0f;
                    t_c2 = false;
                }
                if (t_c2)
                {
                    hit_dmin_r_next = prev_t2;
                    hit_bias_r_next = t_o2;
                    hit_t_r_next = prev_t_o2;
                    hit_flag3 = true;
                    break;
                }
                else
                {
                    hit_dmin_r_next = hit_dmin_r;
                    hit_bias_r_next = hit_bias_r;
                    hit_t_r_next = hit_t_r;
                    hit_flag3 = false;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            bool hit_better2;
            if (hit_flag3)
            {
                bool hit_better_gray2;
                if (hit_t_r_next < hit_bias_r_next)
                {
                    hit_better_gray2 = hit_dmin_r_next < hit_bias_r_next;
                }
                else
                {
                    hit_better_gray2 = false;
                }
                hit_better2 = hit_better_gray2;
            }
            else
            {
                hit_better2 = false;
            }
            if (hit_better2)
            {
                hit_t3 = true;
                hit_t_ratio2 = hit_t_r_next / hit_bias_r_next;
                break;
            }
            hit_t3 = false;
            hit_t_ratio2 = 0.0f;
            break;
        } while(false);
        acc_d0_next = acc_d0 + (hit_t3 ? hit_t_ratio2 : 1.0f);
    }
    float ev_mix = lerp(acc_d0 * 0.5f, 1.0f, 0.4000000059604644775390625f);
    bool not_flag6 = !flag_bits6;
    float3 sh_amb;
    if (not_flag6)
    {
        float3 sh_neg = (-float3(sh0.y, sh1.y, sh2.y)) * ind_light;
        float3 sh_pos = float3(sh0.z, sh1.z, sh2.z) * ind_light;
        float3 sh_neg2 = (-float3(sh0.x, sh1.x, sh2.x)) * ind_light;
        float3 sh_norm = normalize(float3(-dot(sh_neg2, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(sh_neg, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(sh_pos, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
        float3 sh_abs = float3(abs(dot(float3(-sh_neg2.x, -sh_neg.x, sh_pos.x), sh_norm)), abs(dot(float3(-sh_neg2.y, -sh_neg.y, sh_pos.y), sh_norm)), abs(dot(float3(-sh_neg2.z, -sh_neg.z, sh_pos.z), sh_norm))) / ind_light;
        float ndotsh = dot(sh_norm, normal_rgb);
        sh_amb = max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * ind_light) + ((1.0f.xxx * (normal_rgb.y * (-1.02332770824432373046875f))) * sh_neg)) + ((1.0f.xxx * (normal_rgb.z * 1.02332770824432373046875f)) * sh_pos)) + ((1.0f.xxx * (normal_rgb.x * (-1.02332770824432373046875f))) * sh_neg2)) + (((ind_light * ((sh_abs * 0.07999999821186065673828125f) + ((sh_abs * 0.60000002384185791015625f) * sh_abs))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * ndotsh) * ndotsh) - 1.0f))));
    }
    else
    {
        sh_amb = ind_light * 1.7999999523162841796875f;
    }
    float ao_ssgi_f;
    if (flag_bits1)
    {
        ao_ssgi_f = lerp(ssgi_occ, 1.0f, 0.300000011920928955078125f);
    }
    else
    {
        ao_ssgi_f = 1.0f;
    }
    float4 brdf_lut_fg = brdf_fg_lut.SampleLevel(sampler_lnc, (float2(dot(normal_rgb, view_ray_n), roughness2) * 0.984375f.xx) + 0.0078125f.xx, 0.0f);
    float brdf_lut_f = brdf_lut_fg.x;
    float brdf_lut_g = brdf_lut_fg.y;
    float3 f_avg = (0.039999999105930328369140625f.xxx * brdf_lut_f) + brdf_lut_g.xxx;
    float fr_fresnel = brdf_lut_f + brdf_lut_g;
    float3 k_s = lerp(f_avg / fr_fresnel.xxx, 1.0f.xxx, 0.4000000059604644775390625f.xxx) * (1.0f - fr_fresnel);
    float3 k_sp = f_avg * (1.0f.xxx + (k_s / (1.0f.xxx - k_s)));
    float3 sh_refl_amb;
    if (not_flag6)
    {
        float r2_rough = lerp(1.0f, alpha8, 0.5f);
        float r2_1 = r2_rough * r2_rough;
        float r2_sqrt = 1.0f - r2_1;
        float3 refl_dir = normalize(lerp(normal_rgb, reflect(view_ray, normal_rgb), (r2_sqrt * (sqrt(r2_sqrt) + r2_1)).xxx));
        float3 sh_refl_neg = (-float3(sh0.y, sh1.y, sh2.y)) * ind_light;
        float3 sh_refl_pos = float3(sh0.z, sh1.z, sh2.z) * ind_light;
        float3 sh_refl_neg2 = (-float3(sh0.x, sh1.x, sh2.x)) * ind_light;
        float3 sh_refl_norm = normalize(float3(-dot(sh_refl_neg2, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(sh_refl_neg, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(sh_refl_pos, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
        float3 sh_refl_abs = float3(abs(dot(float3(-sh_refl_neg2.x, -sh_refl_neg.x, sh_refl_pos.x), sh_refl_norm)), abs(dot(float3(-sh_refl_neg2.y, -sh_refl_neg.y, sh_refl_pos.y), sh_refl_norm)), abs(dot(float3(-sh_refl_neg2.z, -sh_refl_neg.z, sh_refl_pos.z), sh_refl_norm))) / ind_light;
        float ndot_refl = dot(sh_refl_norm, refl_dir);
        sh_refl_amb = max(0.0f.xxx, ((((0.886227548122406005859375f.xxx * ind_light) + ((1.0f.xxx * (refl_dir.y * (-1.02332770824432373046875f))) * sh_refl_neg)) + ((1.0f.xxx * (refl_dir.z * 1.02332770824432373046875f)) * sh_refl_pos)) + ((1.0f.xxx * (refl_dir.x * (-1.02332770824432373046875f))) * sh_refl_neg2)) + (((ind_light * ((sh_refl_abs * 0.07999999821186065673828125f) + ((sh_refl_abs * 0.60000002384185791015625f) * sh_refl_abs))) * 0.25f) * (0.3153915703296661376953125f * (((3.0f * ndot_refl) * ndot_refl) - 1.0f))));
    }
    else
    {
        sh_refl_amb = sh_amb;
    }
    float r2_smooth2;
    if (flag_bits0)
    {
        float r2_smooth = ev_mix * ev_mix;
        r2_smooth2 = (r2_smooth * r2_smooth) * ev_mix;
    }
    else
    {
        r2_smooth2 = 1.0f;
    }
    float mix_spec_ev;
    if (flag_bits1)
    {
        mix_spec_ev = lerp(ssgi_occ, 1.0f, 0.300000011920928955078125f);
    }
    else
    {
        mix_spec_ev = 1.0f;
    }
    float3 lit = lit_editing + (((((1.0f.xxx - k_sp) * min(flag_bits0 ? ev_mix : 1.0f, ao_ssgi_f)) * albedo_use) * sh_amb) + (((sh_refl_amb * 1.0f) * k_sp) * min(r2_smooth2, mix_spec_ev)));
    float3 lit_res_result2;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 cell_u3b = uint3(clamp(floor((probe_pos * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint cell_idx = (32u * ((32u * cell_u3b.y) + cell_u3b.z)) + cell_u3b.x;
        bool wave_ok = WaveActiveAllTrue((light_indirection_grid.Load<uint>(cell_idx * 4 + 0) >> 20u) <= 2u);
        uint lod_sel = wave_ok ? 2u : 6u;
        float3 lit_res_result;
        if (6u == lod_sel)
        {
            uint cand_lights2[8];
            for (uint cand_i0 = 0u; cand_i0 < 8u; )
            {
                cand_lights2[cand_i0] = 4294967295u;
                cand_i0++;
                continue;
            }
            uint2 px_parity2 = uint2(pixel_i2 & int2(1, 1));
            uint quad_sel2 = (px_parity2.x + (px_parity2.y * 2u)) & 3u;
            uint count2;
            count2 = 0u;
            uint count_next2;
            uint off_i2 = 0u;
            for (;;)
            {
                bool count_ok2;
                if (off_i2 < 8u)
                {
                    count_ok2 = count2 < 8u;
                }
                else
                {
                    count_ok2 = false;
                }
                if (count_ok2)
                {
                    uint4 res_smp2 = rdi_reservoir_tex.Load(int3(uint2((pixel_i2 / int2(2, 2)) + spatial_resolve_offsets_array[((quad_sel2 * 16u) + off_i2) + 256u].xy), 0u));
                    uint2 res_xy2 = res_smp2.xy;
                    uint res_lantern2 = res_smp2.x;
                    bool lantern_valid2;
                    if (any(bool2(res_xy2.x != uint2(0u, 0u).x, res_xy2.y != uint2(0u, 0u).y)))
                    {
                        lantern_valid2 = res_lantern2 < lantern_count;
                    }
                    else
                    {
                        lantern_valid2 = false;
                    }
                    bool no_dup0;
                    if (lantern_valid2)
                    {
                        no_dup0 = cand_lights2[0u] != res_lantern2;
                    }
                    else
                    {
                        no_dup0 = false;
                    }
                    bool no_dup1;
                    if (no_dup0)
                    {
                        no_dup1 = cand_lights2[1u] != res_lantern2;
                    }
                    else
                    {
                        no_dup1 = false;
                    }
                    bool no_dup2;
                    if (no_dup1)
                    {
                        no_dup2 = cand_lights2[2u] != res_lantern2;
                    }
                    else
                    {
                        no_dup2 = false;
                    }
                    bool no_dup3;
                    if (no_dup2)
                    {
                        no_dup3 = cand_lights2[3u] != res_lantern2;
                    }
                    else
                    {
                        no_dup3 = false;
                    }
                    bool no_dup4;
                    if (no_dup3)
                    {
                        no_dup4 = cand_lights2[4u] != res_lantern2;
                    }
                    else
                    {
                        no_dup4 = false;
                    }
                    bool no_dup5;
                    if (no_dup4)
                    {
                        no_dup5 = cand_lights2[5u] != res_lantern2;
                    }
                    else
                    {
                        no_dup5 = false;
                    }
                    bool no_dup6;
                    if (no_dup5)
                    {
                        no_dup6 = cand_lights2[6u] != res_lantern2;
                    }
                    else
                    {
                        no_dup6 = false;
                    }
                    bool no_dup7;
                    if (no_dup6)
                    {
                        no_dup7 = cand_lights2[7u] != res_lantern2;
                    }
                    else
                    {
                        no_dup7 = false;
                    }
                    if (no_dup7)
                    {
                        cand_lights2[count2] = res_lantern2;
                        count_next2 = count2 + 1u;
                    }
                    else
                    {
                        count_next2 = count2;
                    }
                    count2 = count_next2;
                    off_i2++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint cand_count2;
            cand_count2 = count2;
            uint cand_count_next2;
            for (uint q_i2 = 0u; q_i2 < 8u; cand_count2 = cand_count_next2, q_i2++)
            {
                uint quad_lantern2 = QuadReadAcrossDiagonal(cand_lights2[q_i2]);
                bool count_ok_q2;
                if (quad_lantern2 < lantern_count)
                {
                    count_ok_q2 = cand_count2 < 8u;
                }
                else
                {
                    count_ok_q2 = false;
                }
                if (count_ok_q2)
                {
                    bool q_no_dup1;
                    if (cand_lights2[0u] != quad_lantern2)
                    {
                        q_no_dup1 = cand_lights2[1u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup1 = false;
                    }
                    bool q_no_dup2;
                    if (q_no_dup1)
                    {
                        q_no_dup2 = cand_lights2[2u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup2 = false;
                    }
                    bool q_no_dup3;
                    if (q_no_dup2)
                    {
                        q_no_dup3 = cand_lights2[3u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup3 = false;
                    }
                    bool q_no_dup4;
                    if (q_no_dup3)
                    {
                        q_no_dup4 = cand_lights2[4u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup4 = false;
                    }
                    bool q_no_dup5;
                    if (q_no_dup4)
                    {
                        q_no_dup5 = cand_lights2[5u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup5 = false;
                    }
                    bool q_no_dup6;
                    if (q_no_dup5)
                    {
                        q_no_dup6 = cand_lights2[6u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup6 = false;
                    }
                    bool q_no_dup7;
                    if (q_no_dup6)
                    {
                        q_no_dup7 = cand_lights2[7u] != quad_lantern2;
                    }
                    else
                    {
                        q_no_dup7 = false;
                    }
                    uint cand_count_next3;
                    if (q_no_dup7)
                    {
                        cand_lights2[cand_count2] = quad_lantern2;
                        cand_count_next3 = cand_count2 + 1u;
                    }
                    else
                    {
                        cand_count_next3 = cand_count2;
                    }
                    cand_count_next2 = cand_count_next3;
                }
                else
                {
                    cand_count_next2 = cand_count2;
                }
            }
            Reservoir1sppStreamState rstream[2];
            Reservoir1spp reservoir[2];
            for (uint cand_i2 = 0u; cand_i2 < 2u; )
            {
                rstream[cand_i2] = res_stream_init;
                reservoir[cand_i2] = res_init;
                cand_i2++;
                continue;
            }
            for (uint cand_i = 0u; cand_i < cand_count2; cand_i++)
            {
                float2 int_pack = spvUnpackHalf2x16(lantern_buf.Load<uint>(cand_lights2[cand_i] * 16 + 12) & 65535u);
                float intensity = int_pack.x;
                uint lamp_type = lantern_buf.Load<uint>(cand_lights2[cand_i] * 16 + 12) >> 16u;
                float3 lantern_d = lantern_buf.Load<float3>(cand_lights2[cand_i] * 16 + 0) - probe_pos;
                float lantern_d2 = dot(lantern_d, lantern_d);
                float3 lantern_up = lantern_d + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 lamp_color2;
                do
                {
                    if (lamp_type == 0u)
                    {
                        lamp_color2 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (lamp_type == 1u)
                        {
                            lamp_color2 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (lamp_type == 2u)
                            {
                                lamp_color2 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (lamp_type == 3u)
                                {
                                    lamp_color2 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                    break;
                                }
                                else
                                {
                                    lamp_color2 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                float lamp_att2 = dot(((lamp_color2 * 4.0f) * intensity) * ((max(0.0f, dot(normal_rgb, lantern_d / length(lantern_d).xxx)) * ((1.0f / (0.3359999954700469970703125f + lantern_d2)) * smoothstep(36.0f * intensity, 16.0f * intensity, lantern_d2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_up / length(lantern_up).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float cand_count_f = float(cand_count2);
                float target_w = frac(bn_r + (float(cand_i) / cand_count_f));
                if (reservoir[0].w_sum <= reservoir[1].w_sum)
                {
                    do
                    {
                        rstream[0].M_sum += 1.0f;
                        float w_add2 = lamp_att2 * cand_count_f;
                        bool replaced2;
                        do
                        {
                            reservoir[0].w_sum += w_add2;
                            reservoir[0].M += 1.0f;
                            if ((w_add2 / reservoir[0].w_sum) >= target_w)
                            {
                                reservoir[0].payload = cand_lights2[cand_i];
                                replaced2 = true;
                                break;
                            }
                            else
                            {
                                replaced2 = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (replaced2)
                        {
                            rstream[0].p_q_sel = lamp_att2;
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
                        rstream[1].M_sum += 1.0f;
                        float w_add = lamp_att2 * cand_count_f;
                        bool replaced;
                        do
                        {
                            reservoir[1].w_sum += w_add;
                            reservoir[1].M += 1.0f;
                            if ((w_add / reservoir[1].w_sum) >= target_w)
                            {
                                reservoir[1].payload = cand_lights2[cand_i];
                                replaced = true;
                                break;
                            }
                            else
                            {
                                replaced = false;
                                break;
                            }
                            break; // unreachable workaround
                        } while(false);
                        if (replaced)
                        {
                            rstream[1].p_q_sel = lamp_att2;
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
            for (uint msum_i = 1u; msum_i < 2u; )
            {
                rstream[0].M_sum += rstream[msum_i].M_sum;
                msum_i++;
                continue;
            }
            for (uint msum_j = 1u; msum_j < 2u; )
            {
                rstream[msum_j].M_sum = rstream[0].M_sum;
                msum_j++;
                continue;
            }
            float3 lit_res_acc;
            lit_res_acc = lit;
            float hit_dmin_r_next2;
            float hit_bias_r_next2;
            float hit_t_r_next2;
            float3 lit_res_acc_next;
            uint hash_or_r_next;
            uint hash_mask_r_next;
            bool hash_en_r_next;
            bool hash_en2_r_next;
            float hit_dmin_r2 = 0.0f;
            float hit_bias_r2 = 0.0f;
            float hit_t_r2 = 0.0f;
            uint acc_i = 0u;
            uint hash_or_r = 0u;
            uint hash_mask_r = 0u;
            bool hash_en_r = false;
            bool hash_en2_r = false;
            for (; acc_i < 2u; hit_dmin_r2 = hit_dmin_r_next2, hit_bias_r2 = hit_bias_r_next2, hit_t_r2 = hit_t_r_next2, lit_res_acc = lit_res_acc_next, acc_i++, hash_or_r = hash_or_r_next, hash_mask_r = hash_mask_r_next, hash_en_r = hash_en_r_next, hash_en2_r = hash_en2_r_next)
            {
                reservoir[acc_i].M = rstream[acc_i].M_sum;
                reservoir[acc_i].W = reservoir[acc_i].w_sum / max(9.9999999392252902907785028219223e-09f, reservoir[acc_i].M * rstream[acc_i].p_q_sel);
                if (reservoir[acc_i].W > 0.0f)
                {
                    float2 int_pack2 = spvUnpackHalf2x16(lantern_buf.Load<uint>(reservoir[acc_i].payload * 16 + 12) & 65535u);
                    float intensity2 = int_pack2.x;
                    uint lamp_type2 = lantern_buf.Load<uint>(reservoir[acc_i].payload * 16 + 12) >> 16u;
                    float3 lantern_d2v = lantern_buf.Load<float3>(reservoir[acc_i].payload * 16 + 0) - probe_pos;
                    float lantern_dist = length(lantern_d2v);
                    float lantern_dist2 = dot(lantern_d2v, lantern_d2v);
                    float3 lantern_up2 = lantern_d2v + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 lantern_dist_xxx = lantern_dist.xxx;
                    float3 lamp_color3;
                    do
                    {
                        if (lamp_type2 == 0u)
                        {
                            lamp_color3 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (lamp_type2 == 1u)
                            {
                                lamp_color3 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (lamp_type2 == 2u)
                                {
                                    lamp_color3 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (lamp_type2 == 3u)
                                    {
                                        lamp_color3 = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        lamp_color3 = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 lamp_att3 = ((lamp_color3 * 4.0f) * intensity2) * ((max(0.0f, dot(normal_rgb, lantern_d2v / lantern_dist_xxx)) * ((1.0f / (0.3359999954700469970703125f + lantern_dist2)) * smoothstep(36.0f * intensity2, 16.0f * intensity2, lantern_dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_up2 / length(lantern_up2).xxx).y));
                    float hit_dmin_r_next3;
                    float hit_bias_r_next3;
                    float hit_t_r_next3;
                    uint hash_or_r_next2;
                    uint hash_mask_r_next2;
                    bool hash_en_r_next2;
                    bool hash_en2_r_next2;
                    float3 lamp_contrib2;
                    if ((reservoir[acc_i].W * max(max(lamp_att3.x, lamp_att3.y), lamp_att3.z)) > 0.0f)
                    {
                        float hit_dmin_r_final;
                        float hit_bias_r_final;
                        float hit_t_r_final;
                        float3 lamp_att_c2;
                        if (lantern_dist > 0.5f)
                        {
                            float3 ray_origin2 = (probe_pos + ((lantern_d2v * (lantern_dist - 0.5f)) / lantern_dist_xxx)) + (normal_rgb * (0.0199999995529651641845703125f * blue_noise_smp.y));
                            float3 view_pos_xyz4 = view_pos4.xyz;
                            float4 w2s_pos4 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(ray_origin2, 1.0f)));
                            float3 ray_dir_near4 = w2s_pos4.xyz / w2s_pos4.w.xxx;
                            float4 s_enter4 = view_pos4 + (float4(ray_dir_near4 - view_pos_xyz4, 0.0f) * float(int(sign(ray_dir_near4.z))));
                            float s_enter_w4 = s_enter4.w;
                            float3 s_exit4 = (s_enter4 / (float((s_enter_w4 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(s_enter_w4))).xxxx).xyz;
                            float3 dir_abs4 = s_exit4 - view_pos_xyz4;
                            bool3 side_flags7 = bool3(dir_abs4.x < 0.0f.xxx.x, dir_abs4.y < 0.0f.xxx.y, dir_abs4.z < 0.0f.xxx.z);
                            float3 max_step7 = (float3(side_flags7.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags7.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags7.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos_xyz4) / dir_abs4;
                            float3 probe_march7 = view_pos_xyz4 + (dir_abs4 * max(0.0f, max(max_step7.x, max_step7.y)));
                            float3 probe_dir4 = s_exit4 - probe_march7;
                            bool3 side_flags8 = bool3(probe_dir4.x >= 0.0f.xxx.x, probe_dir4.y >= 0.0f.xxx.y, probe_dir4.z >= 0.0f.xxx.z);
                            float3 max_step8 = (float3(side_flags8.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags8.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags8.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - probe_march7) / probe_dir4;
                            float3 probe_march8 = probe_march7 + (probe_dir4 * min(1.0f, min(min(max_step8.x, max_step8.y), max_step8.z)));
                            float hit_t_c5;
                            float hit_better_gray5;
                            float hit_better5;
                            bool hit_t6;
                            float shadow_t2;
                            float shadow_push2;
                            do
                            {
                                float shadow_bias4 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint shadow_steps4 = uint(int(max(2u, min(4u, uint(int(floor(length((((probe_march8.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((probe_march7.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float hit_flag7r;
                                float hit_t5r;
                                bool hit_flag8;
                                do
                                {
                                    float3 shadow_dir4 = probe_march8 - probe_march7;
                                    float inv_steps7;
                                    if (probe_march8.z > 9.5367431640625e-07f)
                                    {
                                        inv_steps7 = float(shadow_steps4);
                                    }
                                    else
                                    {
                                        inv_steps7 = max(1.0f, float(shadow_steps4) - 1.0f);
                                    }
                                    float inv_steps8 = 1.0f / inv_steps7;
                                    float t_c4;
                                    float hit_t4r;
                                    float hit_t_c4r;
                                    float hit_flag5r;
                                    bool hit_flag6r;
                                    if (shadow_steps4 > 0u)
                                    {
                                        float rand_frac4 = lerp(0.0f, 1.0f, pow(bn_r * inv_steps8, 1.0f));
                                        float3 shadow_sample7 = probe_march7 + (shadow_dir4 * rand_frac4);
                                        float2 probe_uv5 = (shadow_sample7.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float probe_invz5 = 1.0f / shadow_sample7.z;
                                        float probe_depth_llc5 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv5, 0.0f).x;
                                        float probe_depth_nnc5 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv5, 0.0f).x;
                                        float depth_diff_max5 = (max(probe_depth_llc5, probe_depth_nnc5) * 1.00000202655792236328125f) - probe_invz5;
                                        float depth_diff_min5 = probe_invz5 - min(probe_depth_llc5, probe_depth_nnc5);
                                        bool hit_front4 = (depth_diff_max5 < 0.0f) ? (depth_diff_min5 < shadow_bias4) : false;
                                        float dmin_c4;
                                        float bias_c4;
                                        float flag_c4;
                                        float prev_t4;
                                        bool prev_t_o4;
                                        float t_o4;
                                        if (hit_front4)
                                        {
                                            dmin_c4 = depth_diff_max5;
                                            bias_c4 = depth_diff_min5;
                                            flag_c4 = shadow_bias4;
                                            prev_t4 = rand_frac4;
                                            prev_t_o4 = hit_front4;
                                            t_o4 = 0.0f;
                                        }
                                        else
                                        {
                                            float rand_frac5;
                                            rand_frac5 = rand_frac4;
                                            bool depth_hit5;
                                            float shadow_lerp2;
                                            float dmax_o4;
                                            float dmin_o4;
                                            float bias_o4;
                                            float flag_o4;
                                            bool dmax_c4;
                                            uint step_i4 = 1u;
                                            bool hit_any4 = hit_front4;
                                            for (;;)
                                            {
                                                if (step_i4 < shadow_steps4)
                                                {
                                                    shadow_lerp2 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i4) + bn_r) * inv_steps8, 1.0f)));
                                                    float3 shadow_sample8 = probe_march7 + (shadow_dir4 * shadow_lerp2);
                                                    float2 probe_uv6 = (shadow_sample8.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float probe_invz6 = 1.0f / shadow_sample8.z;
                                                    float probe_depth_llc6 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv6, 0.0f).x;
                                                    float probe_depth_nnc6 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv6, 0.0f).x;
                                                    float depth_diff_max6 = (max(probe_depth_llc6, probe_depth_nnc6) * 1.00000202655792236328125f) - probe_invz6;
                                                    float depth_diff_min6 = probe_invz6 - min(probe_depth_llc6, probe_depth_nnc6);
                                                    depth_hit5 = (depth_diff_max6 < 0.0f) ? (depth_diff_min6 < shadow_bias4) : false;
                                                    if (depth_hit5)
                                                    {
                                                        dmax_o4 = depth_diff_max6;
                                                        dmin_o4 = depth_diff_min6;
                                                        bias_o4 = shadow_bias4;
                                                        flag_o4 = shadow_lerp2;
                                                        dmax_c4 = depth_hit5;
                                                        break;
                                                    }
                                                    step_i4++;
                                                    hit_any4 = depth_hit5;
                                                    rand_frac5 = shadow_lerp2;
                                                    continue;
                                                }
                                                else
                                                {
                                                    dmax_o4 = 0.0f;
                                                    dmin_o4 = 0.0f;
                                                    bias_o4 = 0.0f;
                                                    flag_o4 = 1.0f;
                                                    dmax_c4 = hit_any4;
                                                    break;
                                                }
                                            }
                                            dmin_c4 = dmax_o4;
                                            bias_c4 = dmin_o4;
                                            flag_c4 = bias_o4;
                                            prev_t4 = flag_o4;
                                            prev_t_o4 = dmax_c4;
                                            t_o4 = rand_frac5;
                                        }
                                        t_c4 = dmin_c4;
                                        hit_t4r = bias_c4;
                                        hit_t_c4r = flag_c4;
                                        hit_flag5r = prev_t4;
                                        hit_flag6r = prev_t_o4;
                                        hit_flag7r = t_o4;
                                    }
                                    else
                                    {
                                        t_c4 = 0.0f;
                                        hit_t4r = 0.0f;
                                        hit_t_c4r = 0.0f;
                                        hit_flag5r = 1.0f;
                                        hit_flag6r = false;
                                        hit_flag7r = 0.0f;
                                    }
                                    if (hit_flag6r)
                                    {
                                        hit_t5r = hit_flag5r;
                                        hit_t_c5 = t_c4;
                                        hit_better_gray5 = hit_t_c4r;
                                        hit_better5 = hit_t4r;
                                        hit_flag8 = true;
                                        break;
                                    }
                                    else
                                    {
                                        hit_t5r = hit_flag7r;
                                        hit_t_c5 = hit_dmin_r2;
                                        hit_better_gray5 = hit_bias_r2;
                                        hit_better5 = hit_t_r2;
                                        hit_flag8 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool hit_better6;
                                if (hit_flag8)
                                {
                                    bool hit_better_gray6;
                                    if (hit_better5 < hit_better_gray5)
                                    {
                                        hit_better_gray6 = hit_t_c5 < hit_better_gray5;
                                    }
                                    else
                                    {
                                        hit_better_gray6 = false;
                                    }
                                    hit_better6 = hit_better_gray6;
                                }
                                else
                                {
                                    hit_better6 = false;
                                }
                                if (hit_better6)
                                {
                                    hit_t6 = true;
                                    shadow_t2 = hit_t5r;
                                    shadow_push2 = hit_better5 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                hit_t6 = false;
                                shadow_t2 = hit_flag7r;
                                shadow_push2 = 0.0f;
                                break;
                            } while(false);
                            float3 lamp_att_shadow2;
                            if (hit_t6)
                            {
                                float4 shadow_world2 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(probe_march7, probe_march8, shadow_t2.xxx), 1.0f)));
                                float3 lantern_shadow_d2 = ((shadow_world2.xyz / shadow_world2.w.xxx) + (view_ray_n * shadow_push2)) - lantern_buf.Load<float3>(reservoir[acc_i].payload * 16 + 0);
                                bool lantern_occluded2;
                                if (!(abs(lantern_shadow_d2.y) > 0.5f))
                                {
                                    float2 lantern_shadow_xz2 = lantern_shadow_d2.xz;
                                    lantern_occluded2 = dot(lantern_shadow_xz2, lantern_shadow_xz2) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    lantern_occluded2 = true;
                                }
                                bool3 occluded_mask2 = lantern_occluded2.xxx;
                                lamp_att_shadow2 = float3(occluded_mask2.x ? 0.0f.xxx.x : lamp_att3.x, occluded_mask2.y ? 0.0f.xxx.y : lamp_att3.y, occluded_mask2.z ? 0.0f.xxx.z : lamp_att3.z);
                            }
                            else
                            {
                                lamp_att_shadow2 = lamp_att3;
                            }
                            hit_dmin_r_final = hit_t_c5;
                            hit_bias_r_final = hit_better_gray5;
                            hit_t_r_final = hit_better5;
                            lamp_att_c2 = lamp_att_shadow2;
                        }
                        else
                        {
                            hit_dmin_r_final = hit_dmin_r2;
                            hit_bias_r_final = hit_bias_r2;
                            hit_t_r_final = hit_t_r2;
                            lamp_att_c2 = lamp_att3;
                        }
                        float2 uv_seed2 = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(reservoir[acc_i].payload * 10u));
                        int2 uv_seed_i2 = int2(floor(uv_seed2));
                        float2 uv_seed_f2 = frac(uv_seed2);
                        uint2 uv_seed_u2 = uint2(uv_seed_i2);
                        uint r00_seed_y = uv_seed_u2.y;
                        uint r00_h1 = r00_seed_y + (r00_seed_y << 10u);
                        uint r00_h2 = r00_h1 ^ (r00_h1 >> 6u);
                        uint r00_h3 = r00_h2 + (r00_h2 << 3u);
                        uint r00_h4 = r00_h3 ^ (r00_h3 >> 11u);
                        uint r00_h5 = (((uv_seed_u2.x * 1664525u) + (r00_h4 + (r00_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint r00_h6 = r00_h5 ^ (r00_h5 >> 11u);
                        uint r00_h7 = r00_h6 ^ ((r00_h6 << 7u) & 2636928640u);
                        uint r00_h8 = r00_h7 ^ ((r00_h7 << 15u) & 4022730752u);
                        bool r00_en = hash_en2_r ? hash_en2_r : true;
                        uint r00_mask = hash_en2_r ? hash_mask_r : 8388607u;
                        bool r00_en2 = hash_en_r ? hash_en_r : true;
                        uint r00_or = hash_en_r ? hash_or_r : 1065353216u;
                        uint2 rx_seed_i = uint2(uv_seed_i2 + int2(1, 0));
                        uint rx_seed_y = rx_seed_i.y;
                        uint rx_h1 = rx_seed_y + (rx_seed_y << 10u);
                        uint rx_h2 = rx_h1 ^ (rx_h1 >> 6u);
                        uint rx_h3 = rx_h2 + (rx_h2 << 3u);
                        uint rx_h4 = rx_h3 ^ (rx_h3 >> 11u);
                        uint rx_h5 = (((rx_seed_i.x * 1664525u) + (rx_h4 + (rx_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint rx_h6 = rx_h5 ^ (rx_h5 >> 11u);
                        uint rx_h7 = rx_h6 ^ ((rx_h6 << 7u) & 2636928640u);
                        uint rx_h8 = rx_h7 ^ ((rx_h7 << 15u) & 4022730752u);
                        bool rx_en = r00_en ? r00_en : true;
                        uint rx_mask = r00_en ? r00_mask : 8388607u;
                        bool rx_en2 = r00_en2 ? r00_en2 : true;
                        uint rx_or = r00_en2 ? r00_or : 1065353216u;
                        uint2 ry_seed_i = uint2(uv_seed_i2 + int2(0, 1));
                        uint ry_seed_y = ry_seed_i.y;
                        uint ry_h1 = ry_seed_y + (ry_seed_y << 10u);
                        uint ry_h2 = ry_h1 ^ (ry_h1 >> 6u);
                        uint ry_h3 = ry_h2 + (ry_h2 << 3u);
                        uint ry_h4 = ry_h3 ^ (ry_h3 >> 11u);
                        uint ry_h5 = (((ry_seed_i.x * 1664525u) + (ry_h4 + (ry_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint ry_h6 = ry_h5 ^ (ry_h5 >> 11u);
                        uint ry_h7 = ry_h6 ^ ((ry_h6 << 7u) & 2636928640u);
                        uint ry_h8 = ry_h7 ^ ((ry_h7 << 15u) & 4022730752u);
                        bool ry_en = rx_en ? rx_en : true;
                        uint ry_mask = rx_en ? rx_mask : 8388607u;
                        bool ry_en2 = rx_en2 ? rx_en2 : true;
                        uint ry_or = rx_en2 ? rx_or : 1065353216u;
                        uint2 rxy_seed_i = uint2(uv_seed_i2 + int2(1, 1));
                        uint rxy_seed_y = rxy_seed_i.y;
                        uint rxy_h1 = rxy_seed_y + (rxy_seed_y << 10u);
                        uint rxy_h2 = rxy_h1 ^ (rxy_h1 >> 6u);
                        uint rxy_h3 = rxy_h2 + (rxy_h2 << 3u);
                        uint rxy_h4 = rxy_h3 ^ (rxy_h3 >> 11u);
                        uint rxy_h5 = (((rxy_seed_i.x * 1664525u) + (rxy_h4 + (rxy_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint rxy_h6 = rxy_h5 ^ (rxy_h5 >> 11u);
                        uint rxy_h7 = rxy_h6 ^ ((rxy_h6 << 7u) & 2636928640u);
                        uint rxy_h8 = rxy_h7 ^ ((rxy_h7 << 15u) & 4022730752u);
                        uint rxy_mask = ry_en ? ry_mask : 8388607u;
                        uint rxy_or = ry_en2 ? ry_or : 1065353216u;
                        float2 bilinear_t2 = (uv_seed_f2 * uv_seed_f2) * (3.0f.xx - (uv_seed_f2 * 2.0f));
                        float tx2 = bilinear_t2.x;
                        float one_minus_tx2 = 1.0f - tx2;
                        float ty2 = bilinear_t2.y;
                        hit_dmin_r_next3 = hit_dmin_r_final;
                        hit_bias_r_next3 = hit_bias_r_final;
                        hit_t_r_next3 = hit_t_r_final;
                        hash_or_r_next2 = rxy_or;
                        hash_mask_r_next2 = rxy_mask;
                        hash_en_r_next2 = ry_en2 ? ry_en2 : true;
                        hash_en2_r_next2 = ry_en ? ry_en : true;
                        lamp_contrib2 = (((lamp_att_c2 * lerp(0.5f, 1.0f, (((1.0f - ty2) * (((asfloat(((r00_h8 ^ (r00_h8 >> 18u)) & r00_mask) | r00_or) - 1.0f) * one_minus_tx2) + ((asfloat(((rx_h8 ^ (rx_h8 >> 18u)) & rx_mask) | rx_or) - 1.0f) * tx2))) + (((asfloat(((ry_h8 ^ (ry_h8 >> 18u)) & ry_mask) | ry_or) - 1.0f) * one_minus_tx2) * ty2)) + (((asfloat(((rxy_h8 ^ (rxy_h8 >> 18u)) & rxy_mask) | rxy_or) - 1.0f) * tx2) * ty2))) * reservoir[acc_i].W) * albedo_use) * ev_mult;
                    }
                    else
                    {
                        hit_dmin_r_next3 = hit_dmin_r2;
                        hit_bias_r_next3 = hit_bias_r2;
                        hit_t_r_next3 = hit_t_r2;
                        hash_or_r_next2 = hash_or_r;
                        hash_mask_r_next2 = hash_mask_r;
                        hash_en_r_next2 = hash_en_r;
                        hash_en2_r_next2 = hash_en2_r;
                        lamp_contrib2 = 0.0f.xxx;
                    }
                    hit_dmin_r_next2 = hit_dmin_r_next3;
                    hit_bias_r_next2 = hit_bias_r_next3;
                    hit_t_r_next2 = hit_t_r_next3;
                    lit_res_acc_next = lit_res_acc + lamp_contrib2;
                    hash_or_r_next = hash_or_r_next2;
                    hash_mask_r_next = hash_mask_r_next2;
                    hash_en_r_next = hash_en_r_next2;
                    hash_en2_r_next = hash_en2_r_next2;
                }
                else
                {
                    hit_dmin_r_next2 = hit_dmin_r2;
                    hit_bias_r_next2 = hit_bias_r2;
                    hit_t_r_next2 = hit_t_r2;
                    lit_res_acc_next = lit_res_acc;
                    hash_or_r_next = hash_or_r;
                    hash_mask_r_next = hash_mask_r;
                    hash_en_r_next = hash_en_r;
                    hash_en2_r_next = hash_en2_r;
                }
            }
            lit_res_result = lit_res_acc;
        }
        else
        {
            float3 lit_grid_result;
            if (2u == lod_sel)
            {
                uint grid_start = light_indirection_grid.Load<uint>(cell_idx * 4 + 0) & 1048575u;
                uint grid_count = min(64u, (light_indirection_grid.Load<uint>(cell_idx * 4 + 0) >> 20u));
                float3 lit_grid_acc;
                lit_grid_acc = lit;
                float3 lit_grid_acc_next;
                float g_hit_dmin_next;
                float g_hit_bias_next;
                float g_hit_t_next;
                uint g_hash_or_next;
                uint g_hash_mask_next;
                bool g_hash_en_next;
                bool g_hash_en2_next;
                float g_hit_dmin = 0.0f;
                float g_hit_bias = 0.0f;
                float g_hit_t = 0.0f;
                uint grid_i = 0u;
                uint g_hash_or = 0u;
                uint g_hash_mask = 0u;
                bool g_hash_en = false;
                bool g_hash_en2 = false;
                for (; grid_i < grid_count; lit_grid_acc = lit_grid_acc_next, g_hit_dmin = g_hit_dmin_next, g_hit_bias = g_hit_bias_next, g_hit_t = g_hit_t_next, grid_i++, g_hash_or = g_hash_or_next, g_hash_mask = g_hash_mask_next, g_hash_en = g_hash_en_next, g_hash_en2 = g_hash_en2_next)
                {
                    uint grid_idx = grid_start + grid_i;
                    float2 grid_lantern = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(grid_idx * 4 + 0) * 16 + 12) & 65535u);
                    float gintensity = grid_lantern.x;
                    uint glamp_type = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(grid_idx * 4 + 0) * 16 + 12) >> 16u;
                    float3 glantern_d = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(grid_idx * 4 + 0) * 16 + 0) - probe_pos;
                    float glantern_dist = length(glantern_d);
                    float glantern_d2 = dot(glantern_d, glantern_d);
                    float3 glantern_up = glantern_d + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 glantern_dist_xxx = glantern_dist.xxx;
                    float3 lamp_color;
                    do
                    {
                        if (glamp_type == 0u)
                        {
                            lamp_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (glamp_type == 1u)
                            {
                                lamp_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (glamp_type == 2u)
                                {
                                    lamp_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (glamp_type == 3u)
                                    {
                                        lamp_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        lamp_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 lamp_att = ((lamp_color * 4.0f) * gintensity) * ((max(0.0f, dot(normal_rgb, glantern_d / glantern_dist_xxx)) * ((1.0f / (0.3359999954700469970703125f + glantern_d2)) * smoothstep(36.0f * gintensity, 16.0f * gintensity, glantern_d2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (glantern_up / length(glantern_up).xxx).y));
                    float3 lamp_contrib;
                    if (max(max(lamp_att.x, lamp_att.y), lamp_att.z) > 0.0f)
                    {
                        float g_hit_dmin_final;
                        float g_hit_bias_final;
                        float g_hit_t_final;
                        float3 lamp_att_c;
                        if (glantern_dist > 0.5f)
                        {
                            float3 ray_origin = (probe_pos + ((glantern_d * (glantern_dist - 0.5f)) / glantern_dist_xxx)) + (normal_rgb * (0.0199999995529651641845703125f * blue_noise_smp.y));
                            float3 view_pos_xyz3 = view_pos4.xyz;
                            float4 w2s_pos3 = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(ray_origin, 1.0f)));
                            float3 ray_dir_near3 = w2s_pos3.xyz / w2s_pos3.w.xxx;
                            float4 s_enter3 = view_pos4 + (float4(ray_dir_near3 - view_pos_xyz3, 0.0f) * float(int(sign(ray_dir_near3.z))));
                            float s_enter_w3 = s_enter3.w;
                            float3 s_exit3 = (s_enter3 / (float((s_enter_w3 >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(s_enter_w3))).xxxx).xyz;
                            float3 dir_abs3 = s_exit3 - view_pos_xyz3;
                            bool3 side_flags5 = bool3(dir_abs3.x < 0.0f.xxx.x, dir_abs3.y < 0.0f.xxx.y, dir_abs3.z < 0.0f.xxx.z);
                            float3 max_step5 = (float3(side_flags5.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags5.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags5.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos_xyz3) / dir_abs3;
                            float3 probe_march5 = view_pos_xyz3 + (dir_abs3 * max(0.0f, max(max_step5.x, max_step5.y)));
                            float3 probe_dir3 = s_exit3 - probe_march5;
                            bool3 side_flags6 = bool3(probe_dir3.x >= 0.0f.xxx.x, probe_dir3.y >= 0.0f.xxx.y, probe_dir3.z >= 0.0f.xxx.z);
                            float3 max_step6 = (float3(side_flags6.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, side_flags6.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, side_flags6.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - probe_march5) / probe_dir3;
                            float3 probe_march6 = probe_march5 + (probe_dir3 * min(1.0f, min(min(max_step6.x, max_step6.y), max_step6.z)));
                            float hit_t_c4;
                            float hit_better_gray3;
                            float hit_better3;
                            bool hit_t5;
                            float shadow_t;
                            float shadow_push;
                            do
                            {
                                float shadow_bias3 = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint shadow_steps3 = uint(int(max(2u, min(4u, uint(int(floor(length((((probe_march6.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((probe_march5.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float flag_o3;
                                float hit_t4;
                                bool hit_flag6;
                                do
                                {
                                    float3 shadow_dir3 = probe_march6 - probe_march5;
                                    float inv_steps5;
                                    if (probe_march6.z > 9.5367431640625e-07f)
                                    {
                                        inv_steps5 = float(shadow_steps3);
                                    }
                                    else
                                    {
                                        inv_steps5 = max(1.0f, float(shadow_steps3) - 1.0f);
                                    }
                                    float inv_steps6 = 1.0f / inv_steps5;
                                    float t_c3;
                                    float hit_t3r;
                                    float hit_t_c3;
                                    float hit_flag4;
                                    bool hit_flag5;
                                    if (shadow_steps3 > 0u)
                                    {
                                        float rand_frac2 = lerp(0.0f, 1.0f, pow(bn_r * inv_steps6, 1.0f));
                                        float3 shadow_sample5 = probe_march5 + (shadow_dir3 * rand_frac2);
                                        float2 probe_uv3g = (shadow_sample5.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float probe_invz3g = 1.0f / shadow_sample5.z;
                                        float probe_depth_llc3g = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv3g, 0.0f).x;
                                        float probe_depth_nnc3g = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv3g, 0.0f).x;
                                        float depth_diff_max3g = (max(probe_depth_llc3g, probe_depth_nnc3g) * 1.00000202655792236328125f) - probe_invz3g;
                                        float depth_diff_min3g = probe_invz3g - min(probe_depth_llc3g, probe_depth_nnc3g);
                                        bool hit_front3g = (depth_diff_max3g < 0.0f) ? (depth_diff_min3g < shadow_bias3) : false;
                                        float dmin_c3;
                                        float bias_c3;
                                        float flag_c3;
                                        float prev_t3;
                                        bool prev_t_o3;
                                        float t_o3;
                                        if (hit_front3g)
                                        {
                                            dmin_c3 = depth_diff_max3g;
                                            bias_c3 = depth_diff_min3g;
                                            flag_c3 = shadow_bias3;
                                            prev_t3 = rand_frac2;
                                            prev_t_o3 = hit_front3g;
                                            t_o3 = 0.0f;
                                        }
                                        else
                                        {
                                            float rand_frac3;
                                            rand_frac3 = rand_frac2;
                                            bool depth_hit4;
                                            float shadow_lerp;
                                            float dmax_o3;
                                            float dmin_o3;
                                            float bias_o3;
                                            float flag_o3;
                                            bool dmax_c3;
                                            uint step_i3g = 1u;
                                            bool hit_any3g = hit_front3g;
                                            for (;;)
                                            {
                                                if (step_i3g < shadow_steps3)
                                                {
                                                    shadow_lerp = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i3g) + bn_r) * inv_steps6, 1.0f)));
                                                    float3 shadow_sample6 = probe_march5 + (shadow_dir3 * shadow_lerp);
                                                    float2 probe_uv4 = (shadow_sample6.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float probe_invz4 = 1.0f / shadow_sample6.z;
                                                    float probe_depth_llc4 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv4, 0.0f).x;
                                                    float probe_depth_nnc4 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv4, 0.0f).x;
                                                    float depth_diff_max4 = (max(probe_depth_llc4, probe_depth_nnc4) * 1.00000202655792236328125f) - probe_invz4;
                                                    float depth_diff_min4 = probe_invz4 - min(probe_depth_llc4, probe_depth_nnc4);
                                                    depth_hit4 = (depth_diff_max4 < 0.0f) ? (depth_diff_min4 < shadow_bias3) : false;
                                                    if (depth_hit4)
                                                    {
                                                        dmax_o3 = depth_diff_max4;
                                                        dmin_o3 = depth_diff_min4;
                                                        bias_o3 = shadow_bias3;
                                                        flag_o3 = shadow_lerp;
                                                        dmax_c3 = depth_hit4;
                                                        break;
                                                    }
                                                    step_i3g++;
                                                    hit_any3g = depth_hit4;
                                                    rand_frac3 = shadow_lerp;
                                                    continue;
                                                }
                                                else
                                                {
                                                    dmax_o3 = 0.0f;
                                                    dmin_o3 = 0.0f;
                                                    bias_o3 = 0.0f;
                                                    flag_o3 = 1.0f;
                                                    dmax_c3 = hit_any3g;
                                                    break;
                                                }
                                            }
                                            dmin_c3 = dmax_o3;
                                            bias_c3 = dmin_o3;
                                            flag_c3 = bias_o3;
                                            prev_t3 = flag_o3;
                                            prev_t_o3 = dmax_c3;
                                            t_o3 = rand_frac3;
                                        }
                                        t_c3 = dmin_c3;
                                        hit_t3r = bias_c3;
                                        hit_t_c3 = flag_c3;
                                        hit_flag4 = prev_t3;
                                        hit_flag5 = prev_t_o3;
                                        flag_o3 = t_o3;
                                    }
                                    else
                                    {
                                        t_c3 = 0.0f;
                                        hit_t3r = 0.0f;
                                        hit_t_c3 = 0.0f;
                                        hit_flag4 = 1.0f;
                                        hit_flag5 = false;
                                        flag_o3 = 0.0f;
                                    }
                                    if (hit_flag5)
                                    {
                                        hit_t4 = hit_flag4;
                                        hit_t_c4 = t_c3;
                                        hit_better_gray3 = hit_t_c3;
                                        hit_better3 = hit_t3r;
                                        hit_flag6 = true;
                                        break;
                                    }
                                    else
                                    {
                                        hit_t4 = flag_o3;
                                        hit_t_c4 = g_hit_dmin;
                                        hit_better_gray3 = g_hit_bias;
                                        hit_better3 = g_hit_t;
                                        hit_flag6 = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool hit_better4;
                                if (hit_flag6)
                                {
                                    bool hit_better_gray4;
                                    if (hit_better3 < hit_better_gray3)
                                    {
                                        hit_better_gray4 = hit_t_c4 < hit_better_gray3;
                                    }
                                    else
                                    {
                                        hit_better_gray4 = false;
                                    }
                                    hit_better4 = hit_better_gray4;
                                }
                                else
                                {
                                    hit_better4 = false;
                                }
                                if (hit_better4)
                                {
                                    hit_t5 = true;
                                    shadow_t = hit_t4;
                                    shadow_push = hit_better3 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                hit_t5 = false;
                                shadow_t = flag_o3;
                                shadow_push = 0.0f;
                                break;
                            } while(false);
                            float3 lamp_att_shadow;
                            if (hit_t5)
                            {
                                float4 shadow_world = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(probe_march5, probe_march6, shadow_t.xxx), 1.0f)));
                                float3 lantern_shadow_d = ((shadow_world.xyz / shadow_world.w.xxx) + (view_ray_n * shadow_push)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(grid_idx * 4 + 0) * 16 + 0);
                                bool lantern_occluded;
                                if (!(abs(lantern_shadow_d.y) > 0.5f))
                                {
                                    float2 lantern_shadow_xz = lantern_shadow_d.xz;
                                    lantern_occluded = dot(lantern_shadow_xz, lantern_shadow_xz) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    lantern_occluded = true;
                                }
                                bool3 occluded_mask = lantern_occluded.xxx;
                                lamp_att_shadow = float3(occluded_mask.x ? 0.0f.xxx.x : lamp_att.x, occluded_mask.y ? 0.0f.xxx.y : lamp_att.y, occluded_mask.z ? 0.0f.xxx.z : lamp_att.z);
                            }
                            else
                            {
                                lamp_att_shadow = lamp_att;
                            }
                            g_hit_dmin_final = hit_t_c4;
                            g_hit_bias_final = hit_better_gray3;
                            g_hit_t_final = hit_better3;
                            lamp_att_c = lamp_att_shadow;
                        }
                        else
                        {
                            g_hit_dmin_final = g_hit_dmin;
                            g_hit_bias_final = g_hit_bias;
                            g_hit_t_final = g_hit_t;
                            lamp_att_c = lamp_att;
                        }
                        float2 uv_seed = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(grid_idx * 4 + 0) * 10u));
                        int2 uv_seed_i = int2(floor(uv_seed));
                        float2 uv_seed_f = frac(uv_seed);
                        uint2 uv_seed_u = uint2(uv_seed_i);
                        uint c00_seed_y = uv_seed_u.y;
                        uint c00_h1 = c00_seed_y + (c00_seed_y << 10u);
                        uint c00_h2 = c00_h1 ^ (c00_h1 >> 6u);
                        uint c00_h3 = c00_h2 + (c00_h2 << 3u);
                        uint c00_h4 = c00_h3 ^ (c00_h3 >> 11u);
                        uint c00_h5 = (((uv_seed_u.x * 1664525u) + (c00_h4 + (c00_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint c00_h6 = c00_h5 ^ (c00_h5 >> 11u);
                        uint c00_h7 = c00_h6 ^ ((c00_h6 << 7u) & 2636928640u);
                        uint c00_h8 = c00_h7 ^ ((c00_h7 << 15u) & 4022730752u);
                        bool c00_en = g_hash_en2 ? g_hash_en2 : true;
                        uint c00_mask = g_hash_en2 ? g_hash_mask : 8388607u;
                        bool c00_en2 = g_hash_en ? g_hash_en : true;
                        uint c00_or = g_hash_en ? g_hash_or : 1065353216u;
                        uint2 cx_seed_i = uint2(uv_seed_i + int2(1, 0));
                        uint cx_seed_y = cx_seed_i.y;
                        uint cx_h1 = cx_seed_y + (cx_seed_y << 10u);
                        uint cx_h2 = cx_h1 ^ (cx_h1 >> 6u);
                        uint cx_h3 = cx_h2 + (cx_h2 << 3u);
                        uint cx_h4 = cx_h3 ^ (cx_h3 >> 11u);
                        uint cx_h5 = (((cx_seed_i.x * 1664525u) + (cx_h4 + (cx_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint cx_h6 = cx_h5 ^ (cx_h5 >> 11u);
                        uint cx_h7 = cx_h6 ^ ((cx_h6 << 7u) & 2636928640u);
                        uint cx_h8 = cx_h7 ^ ((cx_h7 << 15u) & 4022730752u);
                        bool cx_en = c00_en ? c00_en : true;
                        uint cx_mask = c00_en ? c00_mask : 8388607u;
                        bool cx_en2 = c00_en2 ? c00_en2 : true;
                        uint cx_or = c00_en2 ? c00_or : 1065353216u;
                        uint2 cy_seed_i = uint2(uv_seed_i + int2(0, 1));
                        uint cy_seed_y = cy_seed_i.y;
                        uint cy_h1 = cy_seed_y + (cy_seed_y << 10u);
                        uint cy_h2 = cy_h1 ^ (cy_h1 >> 6u);
                        uint cy_h3 = cy_h2 + (cy_h2 << 3u);
                        uint cy_h4 = cy_h3 ^ (cy_h3 >> 11u);
                        uint cy_h5 = (((cy_seed_i.x * 1664525u) + (cy_h4 + (cy_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint cy_h6 = cy_h5 ^ (cy_h5 >> 11u);
                        uint cy_h7 = cy_h6 ^ ((cy_h6 << 7u) & 2636928640u);
                        uint cy_h8 = cy_h7 ^ ((cy_h7 << 15u) & 4022730752u);
                        bool cy_en = cx_en ? cx_en : true;
                        uint cy_mask = cx_en ? cx_mask : 8388607u;
                        bool cy_en2 = cx_en2 ? cx_en2 : true;
                        uint cy_or = cx_en2 ? cx_or : 1065353216u;
                        uint2 cxy_seed_i = uint2(uv_seed_i + int2(1, 1));
                        uint cxy_seed_y = cxy_seed_i.y;
                        uint cxy_h1 = cxy_seed_y + (cxy_seed_y << 10u);
                        uint cxy_h2 = cxy_h1 ^ (cxy_h1 >> 6u);
                        uint cxy_h3 = cxy_h2 + (cxy_h2 << 3u);
                        uint cxy_h4 = cxy_h3 ^ (cxy_h3 >> 11u);
                        uint cxy_h5 = (((cxy_seed_i.x * 1664525u) + (cxy_h4 + (cxy_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint cxy_h6 = cxy_h5 ^ (cxy_h5 >> 11u);
                        uint cxy_h7 = cxy_h6 ^ ((cxy_h6 << 7u) & 2636928640u);
                        uint cxy_h8 = cxy_h7 ^ ((cxy_h7 << 15u) & 4022730752u);
                        uint cxy_mask = cy_en ? cy_mask : 8388607u;
                        uint cxy_or = cy_en2 ? cy_or : 1065353216u;
                        float2 bilinear_t = (uv_seed_f * uv_seed_f) * (3.0f.xx - (uv_seed_f * 2.0f));
                        float tx = bilinear_t.x;
                        float one_minus_tx = 1.0f - tx;
                        float ty = bilinear_t.y;
                        g_hit_dmin_next = g_hit_dmin_final;
                        g_hit_bias_next = g_hit_bias_final;
                        g_hit_t_next = g_hit_t_final;
                        g_hash_or_next = cxy_or;
                        g_hash_mask_next = cxy_mask;
                        g_hash_en_next = cy_en2 ? cy_en2 : true;
                        g_hash_en2_next = cy_en ? cy_en : true;
                        lamp_contrib = (((lamp_att_c * lerp(0.5f, 1.0f, (((1.0f - ty) * (((asfloat(((c00_h8 ^ (c00_h8 >> 18u)) & c00_mask) | c00_or) - 1.0f) * one_minus_tx) + ((asfloat(((cx_h8 ^ (cx_h8 >> 18u)) & cx_mask) | cx_or) - 1.0f) * tx))) + (((asfloat(((cy_h8 ^ (cy_h8 >> 18u)) & cy_mask) | cy_or) - 1.0f) * one_minus_tx) * ty)) + (((asfloat(((cxy_h8 ^ (cxy_h8 >> 18u)) & cxy_mask) | cxy_or) - 1.0f) * tx) * ty))) * 1.0f) * albedo_use) * ev_mult;
                    }
                    else
                    {
                        g_hit_dmin_next = g_hit_dmin;
                        g_hit_bias_next = g_hit_bias;
                        g_hit_t_next = g_hit_t;
                        g_hash_or_next = g_hash_or;
                        g_hash_mask_next = g_hash_mask;
                        g_hash_en_next = g_hash_en;
                        g_hash_en2_next = g_hash_en2;
                        lamp_contrib = 0.0f.xxx;
                    }
                    lit_grid_acc_next = lit_grid_acc + lamp_contrib;
                }
                lit_grid_result = lit_grid_acc;
            }
            else
            {
                lit_grid_result = lit;
            }
            lit_res_result = lit_grid_result;
        }
        lit_res_result2 = lit_res_result;
    }
    else
    {
        lit_res_result2 = lit;
    }
    out_var_SV_TARGET = float4(lit_res_result2, 0.0f);
    return out_var_SV_TARGET;
}
#else
// f0ad: pixel stage lifted from _deferred_solid_light_sun_sky.raster.hlsl_f0adff76827e73ff.bin (spirv-cross, semanticized
// 命名 + 中文注释;行为与原版逐指令一致,readable_check 校验通过)。

void ps_main(VsToPs input)
{
    int2 pixel_i = int2(input.position.xy);
    float4 world_pos4 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4((((float2(pixel_i) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(pixel_i), 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 cell_u3 = uint3(clamp(floor(((world_pos4.xyz / world_pos4.w.xxx) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * cell_u3.y) + cell_u3.z)) + cell_u3.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint cand_lights[8];
            for (uint i = 0u; i < 8u; )
            {
                cand_lights[i] = 4294967295u;
                i++;
                continue;
            }
            uint2 px_parity = uint2(pixel_i & int2(1, 1));
            uint quad_sel = (px_parity.x + (px_parity.y * 2u)) & 3u;
            uint count;
            count = 0u;
            uint count_next;
            uint off_i = 0u;
            for (;;)
            {
                bool count_ok;
                if (off_i < 8u)
                {
                    count_ok = count < 8u;
                }
                else
                {
                    count_ok = false;
                }
                if (count_ok)
                {
                    uint4 res_smp = rdi_reservoir_tex.Load(int3(uint2((pixel_i / int2(2, 2)) + spatial_resolve_offsets_array[((quad_sel * 16u) + off_i) + 256u].xy), 0u));
                    uint2 res_xy = res_smp.xy;
                    uint res_lantern = res_smp.x;
                    bool lantern_valid;
                    if (any(bool2(res_xy.x != uint2(0u, 0u).x, res_xy.y != uint2(0u, 0u).y)))
                    {
                        lantern_valid = res_lantern < lantern_count;
                    }
                    else
                    {
                        lantern_valid = false;
                    }
                    bool no_dup0;
                    if (lantern_valid)
                    {
                        no_dup0 = cand_lights[0u] != res_lantern;
                    }
                    else
                    {
                        no_dup0 = false;
                    }
                    bool no_dup1;
                    if (no_dup0)
                    {
                        no_dup1 = cand_lights[1u] != res_lantern;
                    }
                    else
                    {
                        no_dup1 = false;
                    }
                    bool no_dup2;
                    if (no_dup1)
                    {
                        no_dup2 = cand_lights[2u] != res_lantern;
                    }
                    else
                    {
                        no_dup2 = false;
                    }
                    bool no_dup3;
                    if (no_dup2)
                    {
                        no_dup3 = cand_lights[3u] != res_lantern;
                    }
                    else
                    {
                        no_dup3 = false;
                    }
                    bool no_dup4;
                    if (no_dup3)
                    {
                        no_dup4 = cand_lights[4u] != res_lantern;
                    }
                    else
                    {
                        no_dup4 = false;
                    }
                    bool no_dup5;
                    if (no_dup4)
                    {
                        no_dup5 = cand_lights[5u] != res_lantern;
                    }
                    else
                    {
                        no_dup5 = false;
                    }
                    bool no_dup6;
                    if (no_dup5)
                    {
                        no_dup6 = cand_lights[6u] != res_lantern;
                    }
                    else
                    {
                        no_dup6 = false;
                    }
                    bool no_dup7;
                    if (no_dup6)
                    {
                        no_dup7 = cand_lights[7u] != res_lantern;
                    }
                    else
                    {
                        no_dup7 = false;
                    }
                    if (no_dup7)
                    {
                        cand_lights[count] = res_lantern;
                        count_next = count + 1u;
                    }
                    else
                    {
                        count_next = count;
                    }
                    count = count_next;
                    off_i++;
                    continue;
                }
                else
                {
                    break;
                }
            }
            uint cand_count_next;
            for (uint cand_count = count, q_i = 0u; q_i < 8u; cand_count = cand_count_next, q_i++)
            {
                uint quad_lantern = QuadReadAcrossDiagonal(cand_lights[q_i]);
                bool count_ok_q;
                if (quad_lantern < lantern_count)
                {
                    count_ok_q = cand_count < 8u;
                }
                else
                {
                    count_ok_q = false;
                }
                if (count_ok_q)
                {
                    bool q_no_dup1;
                    if (cand_lights[0u] != quad_lantern)
                    {
                        q_no_dup1 = cand_lights[1u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup1 = false;
                    }
                    bool q_no_dup2;
                    if (q_no_dup1)
                    {
                        q_no_dup2 = cand_lights[2u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup2 = false;
                    }
                    bool blue_noise_smp;
                    if (q_no_dup2)
                    {
                        blue_noise_smp = cand_lights[3u] != quad_lantern;
                    }
                    else
                    {
                        blue_noise_smp = false;
                    }
                    bool q_no_dup4;
                    if (blue_noise_smp)
                    {
                        q_no_dup4 = cand_lights[4u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup4 = false;
                    }
                    bool q_no_dup5;
                    if (q_no_dup4)
                    {
                        q_no_dup5 = cand_lights[5u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup5 = false;
                    }
                    bool q_no_dup6;
                    if (q_no_dup5)
                    {
                        q_no_dup6 = cand_lights[6u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup6 = false;
                    }
                    bool q_no_dup7;
                    if (q_no_dup6)
                    {
                        q_no_dup7 = cand_lights[7u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup7 = false;
                    }
                    uint cand_count_next2;
                    if (q_no_dup7)
                    {
                        cand_lights[cand_count] = quad_lantern;
                        cand_count_next2 = cand_count + 1u;
                    }
                    else
                    {
                        cand_count_next2 = cand_count;
                    }
                    cand_count_next = cand_count_next2;
                }
                else
                {
                    cand_count_next = cand_count;
                }
            }
        }
        else
        {
        }
    }
}
#endif
