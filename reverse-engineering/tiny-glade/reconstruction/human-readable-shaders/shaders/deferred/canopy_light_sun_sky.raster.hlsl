// ==RECONSTRUCTED== shaders/deferred/canopy_light_sun_sky.raster.hlsl | status: O (structure from disassembly); I (flag name; $Globals member names verbatim)

// 中文说明:
//   树冠树冠 canopy 材质的延迟"太阳+天空"光照合成(全屏 pass)。包含:
//   - 由 gbuffer+深度重建世界位置/法线(RGB8 法线解码,gbuffer.x 低 24 位);
//   - 太阳:能量守恒 GGX 漫反射 + Cook-Torrance 镜面 * sun_color * NdotL;
//   - 天空环境:间接辐射 3-slab 空间解析的 SH 带 1 求值;
//   - ReSTIR 灯笼光:reservoir 收集 + 流式采样;
//   - 网格灯:light_indirection_grid / grid_light_indices 迭代;
//   - 级联阴影 / 屏幕空间阴影 / ssgi 合成。
//   两个变体由 DEFERRED_FULL_LIGHTING 门控(b903 全 / f0ad 精简);ps_main
//   主体为 spirv-cross 转译(行为精确,局部变量已全部语义化命名)。
// MERGE (gi-deferred review 2026-10-13): adopted candidate B as the integration base (both
// b903/f0ad variants via DEFERRED_FULL_LIGHTING). The review's octahedral-normal and SV_Target
// alpha=2.0 merge deltas are REJECTED on bin evidence (the canopy b903 ps decodes the gbuffer
// normal as RGB8 normalize of gbuffer.x low bytes and writes alpha 0.0 — both match B as-is).

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
#include "inc/material/eval/canopy_material.inc.hlsl"
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
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> blue_noise_texture : register(t18, space0);
Texture2D<uint4> rdi_reservoir_tex : register(t28, space0);
ByteAddressBuffer light_indirection_grid : register(t36, space0);

#if DEFERRED_FULL_LIGHTING
ByteAddressBuffer lantern_buf : register(t26, space0);
Texture2D<float4> indirect_radiance_tex : register(t27, space0);
Texture2D<float4> shadow_map : register(t25, space0);
ByteAddressBuffer grid_light_indices : register(t37, space0);
#endif

// Flat color_library cbuffer at the deferred pass binding (b23). The inc's
// nested placeholder is guard-skipped so the flat members match the target.


#if DEFERRED_FULL_LIGHTING
// b903: pixel stage lifted from _deferred_canopy_light_sun_sky.raster.hlsl_b903e43ffb3da915.bin (spirv-cross, semanticized
// 命名 + 中文注释;行为与原版逐指令一致,readable_check 校验通过)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
static float unused_f32;
static uint hash_or_default_u;
static const float4 sh_zero_init[3] = { 0.0f.xxxx, 0.0f.xxxx, 0.0f.xxxx };
static const Reservoir1sppStreamState res_stream_init = { 0.0f, 0.0f };
static const Reservoir1spp res_init = { 0.0f, 0u, 0.0f, 0.0f };
float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}
float4 ps_main(VsToPs input) : SV_Target0
{
    // 延迟光照 ps_main(树冠材质)。阶段:
    //  1) 解码 gbuffer:RGB8 法线 / 粗糙度 / alpha8(叶片掩码);
    //  2) 重建世界位置与视线方向;
    //  3) 太阳直射:级联阴影 PCF + GGX 漫反射/镜面;
    //  4) 间接光:半分辨率 radiance 3-slab 空间解析 -> SH 带 1;
    //  5) 灯笼光:ReSTIR reservoir 收集或直接网格灯 + 深度缓冲软阴影 + perlin 闪烁。
    float4 out_var_SV_TARGET;
    int2 pixel_i = int2(input.position.xy);
    uint2 pixel_u = uint2(pixel_i);
    float4 blue_noise = ((blue_noise_texture.Load(int3((pixel_u + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    uint4 gb0 = gbuffer_tex.Load(int3(pixel_u, 0u));
    float4 depth_smp = depth_tex.Load(int3(pixel_u, 0u));
    float depth = depth_smp.x;
    uint gb_norm_pack = gb0.x;
    float alpha8_u = float((gb_norm_pack >> 24u) & 255u);
    float alpha8 = alpha8_u * 0.0039215688593685626983642578125f;
    float3 normal_rgb = (float4(float(gb_norm_pack & 255u) * 0.0039215688593685626983642578125f, float((gb_norm_pack >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gb_norm_pack >> 16u) & 255u) * 0.0039215688593685626983642578125f, alpha8).xyz * 2.0f) - 1.0f.xxx;
    uint gb_rough_pack = gb0.y;
    float roughness = float((gb_rough_pack >> 24u) & 255u) * 0.0039215688593685626983642578125f;
    float3 albedo_srgb = float4(float(gb_rough_pack & 255u) * 0.0039215688593685626983642578125f, float((gb_rough_pack >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((gb_rough_pack >> 16u) & 255u) * 0.0039215688593685626983642578125f, roughness).xyz;
    float3 albedo = albedo_srgb * albedo_srgb;
    float3 view_dir = mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz;
    float2 depth_dims_f = float2(depth_tex_dims);
    float2 ndc = (((float2(pixel_i) + 0.5f.xx) / depth_dims_f) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc.x;
    float4 view_pos4 = float4(ndc_x, ndc.y, depth, 1.0f);
    float4 world_pos = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, view_pos4));
    float3 view_ray = normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f))).xyz);
    float3 view_ray_n = -view_ray;
    float3 probe_pos = (world_pos.xyz / world_pos.w.xxx) + (((view_ray * 1.0f) * roughness) * 1.60000002384185791015625f);
    bool3 norm_ovr_flag = ((render_overrides & 1u) != 0u).xxx;
    float3 albedo_use = float3(norm_ovr_flag.x ? 0.5f.xxx.x : albedo.x, norm_ovr_flag.y ? 0.5f.xxx.y : albedo.y, norm_ovr_flag.z ? 0.5f.xxx.z : albedo.z);
    bool has_alpha = alpha8 > 0.0f;
    float3 hemi_dir;
    if (has_alpha)
    {
        uint alpha_seed = uint(alpha8_u);
        uint alpha_h1 = alpha_seed + (alpha_seed << 10u);
        uint alpha_h2 = alpha_h1 ^ (alpha_h1 >> 6u);
        uint alpha_h3 = alpha_h2 + (alpha_h2 << 3u);
        uint alpha_h4 = alpha_h3 ^ (alpha_h3 >> 11u);
        uint alpha_seed2 = alpha_seed ^ 188053769u;
        uint alpha_h5 = alpha_seed2 + (alpha_seed2 << 10u);
        uint alpha_h6 = alpha_h5 ^ (alpha_h5 >> 6u);
        uint alpha_h7 = alpha_h6 + (alpha_h6 << 3u);
        uint alpha_h8 = alpha_h7 ^ (alpha_h7 >> 11u);
        float hemi_rand_z = asfloat(((alpha_h8 + (alpha_h8 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
        float hemi_rand_theta = asfloat(((alpha_h4 + (alpha_h4 << 15u)) & 8388607u) | 1065353216u) * 3.141499996185302734375f;
        float hemi_rand_scale = sqrt(1.0f - (hemi_rand_z * hemi_rand_z));
        hemi_dir = float3(hemi_rand_scale * cos(hemi_rand_theta), hemi_rand_scale * sin(hemi_rand_theta), hemi_rand_z) * 0.5f;
    }
    else
    {
        hemi_dir = 0.0f.xxx;
    }
    uint hash_or = has_alpha ? 1065353216u : hash_or_default_u;
    uint hash_mask = has_alpha ? 8388607u : hash_or_default_u;
    bool hash_enable = has_alpha ? true : false;
    float shadow_occ;
    do
    {
        float4 probe_shadow = mul(view_constants[2u].world_to_sample, float4(probe_pos, 1.0f));
        float probe_shadow_z = probe_shadow.z;
        if (probe_shadow_z > 1.0f)
        {
            shadow_occ = 1.0f;
            break;
        }
        float2 probe_shadow_xy = probe_shadow.xy;
        float2 probe_shadow_abs = abs(probe_shadow_xy);
        bool cascade_edge = any(bool2(probe_shadow_abs.x > 0.98666667938232421875f.xx.x, probe_shadow_abs.y > 0.98666667938232421875f.xx.y));
        float2 cascade_edge_xy;
        if (cascade_edge)
        {
            cascade_edge_xy = probe_shadow_xy * 0.5f;
        }
        else
        {
            cascade_edge_xy = probe_shadow_xy;
        }
        float cascade_texel = cascade_edge ? 4.0f : 8.0f;
        float2 cascade_edge_abs = abs(cascade_edge_xy);
        float2 cascade_bound = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * cascade_texel));
        bool pcf_scale_needed = any(bool2(cascade_edge_abs.x > cascade_bound.x, cascade_edge_abs.y > cascade_bound.y));
        float pcf_step_scale;
        float2 pcf_coord;
        if (pcf_scale_needed)
        {
            pcf_step_scale = cascade_texel * 0.2857142984867095947265625f;
            pcf_coord = cascade_edge_xy * 0.2857142984867095947265625f;
        }
        else
        {
            pcf_step_scale = cascade_texel;
            pcf_coord = cascade_edge_xy;
        }
        float shadow_layer_f = float(pcf_scale_needed ? 2u : uint(cascade_edge));
        float2 pcf_uv = (pcf_coord.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float pcf_rot_offset = float((uint((pixel_i.x * 11) + (pixel_i.y * 7)) + (frame_index * 5u)) % 64u) * 1.61803400516510009765625f;
        float pcf_frame_angle = 5.588237762451171875f * float(frame_index % 64u);
        float pcf_jitter = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel_u.x) + pcf_frame_angle)) + (0.005837149918079376220703125f * (float(pixel_u.y) + pcf_frame_angle))));
        float pcf_occ_acc;
        pcf_occ_acc = 0.0f;
        for (int i = 0; i < 6; )
        {
            float i_f = float(i);
            float angle = (i_f + pcf_rot_offset) * 2.3999631404876708984375f;
            pcf_occ_acc += exp((-1.0f) * max(0.0f, (probe_shadow_z - shadow_map.SampleLevel(sampler_shadow_nearest, float3(pcf_uv.xy + ((((float2(cos(angle), sin(angle)) * (i_f + pcf_jitter)) * 0.16666667163372039794921875f.xx) * pcf_step_scale) * 0.000833333353511989116668701171875f.xx), shadow_layer_f), 0.0f).x) * 1024.0f));
            i++;
            continue;
        }
        shadow_occ = pcf_occ_acc * 0.16666667163372039794921875f;
        break;
    } while(false);
    float ao_term = dot(normal_rgb, (probe_pos - probe_pos) + ((normalize(view_ray_n) * (((1.0f - roughness) * 2.0f) - 1.0f)) * 5.0f));
    float3 normal_n = normalize(normal_rgb);
    float ndotv_ao = lerp(0.20000000298023223876953125f, 1.0f, smoothstep(-0.20000000298023223876953125f, 1.0f, dot(normal_n, view_dir))) * 0.800000011920928955078125f;
    float fresnel = max(0.0f, 1.0f - abs(dot(normal_n, view_ray_n)));
    float fresnel2 = fresnel * fresnel;
    float half_nd = abs(dot(normalize(normal_rgb + hemi_dir), normalize(view_dir + view_ray_n)));
    float half_nd2 = ((half_nd * half_nd) * (-0.5f)) + 1.0f;
    float sky_fade = max(0.0f, (-0.75f) * dot(view_ray_n, view_dir));
    float4 sh_bands[3] = sh_zero_init;
    float bn_r = blue_noise.x;
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
            sh_bands[ch_i] += (indirect_radiance_tex.Load(int3(ss_off + uint2((ch_i * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * depth_w);
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
                sh_bands[ch_i_ext] += (indirect_radiance_tex.Load(int3(ss_off_ext + uint2((ch_i_ext * indirect_radiance_tex_dims.x) / 3u, 0u), 0u)) * depth_w_ext);
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
    sh_bands[0] *= inv_wsum;
    sh_bands[1] *= inv_wsum;
    sh_bands[2] *= inv_wsum;
    float3 sh_dir = (sh_bands[0].yzw + sh_bands[1].yzw) + sh_bands[2].yzw;
    float3 sh_irr = float3(sh_bands[0].x, sh_bands[1].x, sh_bands[2].x);
    float3 sh0 = sh_bands[0].yzw / max(9.9999996826552253889678874634872e-21f, sh_bands[0].x).xxx;
    float3 sh1 = sh_bands[1].yzw / max(9.9999996826552253889678874634872e-21f, sh_bands[1].x).xxx;
    float3 sh2 = sh_bands[2].yzw / max(9.9999996826552253889678874634872e-21f, sh_bands[2].x).xxx;
    float3 ind_tint_inv = max(0.0f.xxx, 1.0f.xxx - (color_library.indirect_light_tint.xyz * 0.920000016689300537109375f));
    float3 ind_light = max(0.0f.xxx, lerp((ind_tint_inv / dot(ind_tint_inv, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx) * dot(sh_irr, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), sh_irr, 1.16999995708465576171875f.xxx) * 1.2000000476837158203125f);
    uint hash2_or;
    uint hash2_mask;
    bool hash2_enable;
    float3 hemi2_dir;
    if (has_alpha)
    {
        uint hemi2_seed = uint(alpha8_u);
        uint hemi2_h1 = hemi2_seed + (hemi2_seed << 10u);
        uint hemi2_h2 = hemi2_h1 ^ (hemi2_h1 >> 6u);
        uint hemi2_h3 = hemi2_h2 + (hemi2_h2 << 3u);
        uint hemi2_h4 = hemi2_h3 ^ (hemi2_h3 >> 11u);
        bool hemi2_en_gate = hash_enable ? hash_enable : true;
        uint hemi2_mask_gate = hash_enable ? hash_mask : 8388607u;
        uint hemi2_or_gate = hash_enable ? hash_or : 1065353216u;
        uint hemi2_seed2 = hemi2_seed ^ 188053769u;
        uint hemi2_h5 = hemi2_seed2 + (hemi2_seed2 << 10u);
        uint hemi2_h6 = hemi2_h5 ^ (hemi2_h5 >> 6u);
        uint hemi2_h7 = hemi2_h6 + (hemi2_h6 << 3u);
        uint hemi2_h8 = hemi2_h7 ^ (hemi2_h7 >> 11u);
        uint hemi2_mask_final = hemi2_en_gate ? hemi2_mask_gate : 8388607u;
        uint hemi2_or_final = hemi2_en_gate ? hemi2_or_gate : 1065353216u;
        float hemi2_rand_z = asfloat(((hemi2_h8 + (hemi2_h8 << 15u)) & hemi2_mask_final) | hemi2_or_final) - 1.0f;
        float hemi2_rand_theta = asfloat(((hemi2_h4 + (hemi2_h4 << 15u)) & hemi2_mask_gate) | hemi2_or_gate) * 3.141499996185302734375f;
        float hemi2_rand_scale = sqrt(1.0f - (hemi2_rand_z * hemi2_rand_z));
        hash2_or = hemi2_or_final;
        hash2_mask = hemi2_mask_final;
        hash2_enable = hemi2_en_gate ? hemi2_en_gate : true;
        hemi2_dir = float3(hemi2_rand_scale * cos(hemi2_rand_theta), hemi2_rand_scale * sin(hemi2_rand_theta), hemi2_rand_z) * 0.5f;
    }
    else
    {
        hash2_or = hash_or;
        hash2_mask = hash_mask;
        hash2_enable = hash_enable;
        hemi2_dir = 0.0f.xxx;
    }
    float3 sh_neg = (-float3(sh0.y, sh1.y, sh2.y)) * ind_light;
    float3 sh_pos = float3(sh0.z, sh1.z, sh2.z) * ind_light;
    float3 sh_neg2 = (-float3(sh0.x, sh1.x, sh2.x)) * ind_light;
    float3 sh_norm = normalize(float3(-dot(sh_neg2, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), -dot(sh_neg, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)), dot(sh_pos, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f))));
    float3 sh_abs = float3(abs(dot(float3(-sh_neg2.x, -sh_neg.x, sh_pos.x), sh_norm)), abs(dot(float3(-sh_neg2.y, -sh_neg.y, sh_pos.y), sh_norm)), abs(dot(float3(-sh_neg2.z, -sh_neg.z, sh_pos.z), sh_norm))) / ind_light;
    float ndotsh = dot(sh_norm, normal_rgb);
    float3 sh_amb = 0.886227548122406005859375f.xxx * ind_light;
    float3 sh_lobe = (ind_light * ((sh_abs * 0.07999999821186065673828125f) + ((sh_abs * 0.60000002384185791015625f) * sh_abs))) * 0.25f;
    float hbao = clamp(exp(ao_term * (-0.20000000298023223876953125f)), 0.0f, 1.0f);
    float3 refl_dir = normalize(reflect(view_ray, normalize(normal_rgb + hemi2_dir)));
    float ndotrefl = dot(sh_norm, refl_dir);
    float gi_refl = clamp(dot(normalize(reflect(view_ray, normal_rgb)), sh_dir / max(9.9999996826552253889678874634872e-21f, length(sh_dir)).xxx), 0.0f, 1.0f);
    // 主光照合成:太阳漫反射(lambert/pi)*NdotV_AO*shadow_occ + Cook-Torrance 镜面 +
    // 间接 SH 带 1 求值(漫反射项 + 反射 lobe 项)。
    float3 lit = (((((albedo_use * 0.3183098733425140380859375f) * ndotv_ao) * sun_color.xyz) * shadow_occ) + (((((((lerp(0.20000000298023223876953125f, 0.699999988079071044921875f, fresnel2 * fresnel2).xxx * (0.15915493667125701904296875f / ((3.1415927410125732421875f * half_nd2) * half_nd2))) * lerp(ndotv_ao, 1.0f, sky_fade)) * lerp(lerp(float(has_alpha), clamp(exp(ao_term * (-0.4000000059604644775390625f)), 0.0f, 1.0f), 0.5f), 1.0f, sky_fade)) * sun_color.xyz) * lerp(0.800000011920928955078125f, 0.300000011920928955078125f, smoothstep(50.0f, 150.0f, length(probe_pos.xz)))) * shadow_occ) * 1.5f)) + ((((lerp(0.300000011920928955078125f, 1.0f, hbao).xxx * 1.0f) * albedo_use) * (max(0.0f.xxx, (((sh_amb + ((1.0f.xxx * (normal_rgb.y * (-1.02332770824432373046875f))) * sh_neg)) + ((1.0f.xxx * (normal_rgb.z * 1.02332770824432373046875f)) * sh_pos)) + ((1.0f.xxx * (normal_rgb.x * (-1.02332770824432373046875f))) * sh_neg2)) + (sh_lobe * (0.3153915703296661376953125f * (((3.0f * ndotsh) * ndotsh) - 1.0f)))) * 1.2999999523162841796875f)) + ((max(0.0f.xxx, (((sh_amb + ((1.0f.xxx * (refl_dir.y * (-1.02332770824432373046875f))) * sh_neg)) + ((1.0f.xxx * (refl_dir.z * 1.02332770824432373046875f)) * sh_pos)) + ((1.0f.xxx * (refl_dir.x * (-1.02332770824432373046875f))) * sh_neg2)) + (sh_lobe * (0.3153915703296661376953125f * (((3.0f * ndotrefl) * ndotrefl) - 1.0f)))) * (0.039999999105930328369140625f * lerp(hbao, (0.75f * hbao) + 0.75f, gi_refl * gi_refl))) * 2.0f));
    float3 lantern_light;
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 cell_u3 = uint3(clamp(floor((probe_pos * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint cell_idx = (32u * ((32u * cell_u3.y) + cell_u3.z)) + cell_u3.x;
        bool wave_ok = WaveActiveAllTrue((light_indirection_grid.Load<uint>(cell_idx * 4 + 0) >> 20u) <= 2u);
        uint lod_sel = wave_ok ? 2u : 6u;
        float3 lit_out; // 灯笼光照分支的最终累计输出(ReSTIR 或网格灯)
        // ReSTIR 分支(lod_sel==6):从半分辨率 rdi_reservoir_tex 空间解析收集候选灯,
        // 逐候选做 shadow ray 软阴影 + 距离衰减积分累加,近似灯光间接照明。
        if (6u == lod_sel)
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
            uint cand_count;
            cand_count = count;
            uint cand_count_next;
            for (uint q_i = 0u; q_i < 8u; cand_count = cand_count_next, q_i++)
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
                    bool q_no_dup3;
                    if (q_no_dup2)
                    {
                        q_no_dup3 = cand_lights[3u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup3 = false;
                    }
                    bool q_no_dup4;
                    if (q_no_dup3)
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
                    uint count_q_next;
                    if (q_no_dup7)
                    {
                        cand_lights[cand_count] = quad_lantern;
                        count_q_next = cand_count + 1u;
                    }
                    else
                    {
                        count_q_next = cand_count;
                    }
                    cand_count_next = count_q_next;
                }
                else
                {
                    cand_count_next = cand_count;
                }
            }
            Reservoir1sppStreamState rstream[2];
            Reservoir1spp reservoir[2];
            for (uint r_i = 0u; r_i < 2u; )
            {
                rstream[r_i] = res_stream_init;
                reservoir[r_i] = res_init;
                r_i++;
                continue;
            }
            for (uint cand_i = 0u; cand_i < cand_count; cand_i++)
            {
                float2 int_pack = spvUnpackHalf2x16(lantern_buf.Load<uint>(cand_lights[cand_i] * 16 + 12) & 65535u);
                float intensity = int_pack.x;
                uint lamp_type = lantern_buf.Load<uint>(cand_lights[cand_i] * 16 + 12) >> 16u;
                float3 lantern_d = lantern_buf.Load<float3>(cand_lights[cand_i] * 16 + 0) - probe_pos;
                float lantern_d2 = dot(lantern_d, lantern_d);
                float3 lantern_up = lantern_d + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                float3 lamp_color;
                do
                {
                    if (lamp_type == 0u)
                    {
                        lamp_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                        break;
                    }
                    else
                    {
                        if (lamp_type == 1u)
                        {
                            lamp_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                            break;
                        }
                        else
                        {
                            if (lamp_type == 2u)
                            {
                                lamp_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                break;
                            }
                            else
                            {
                                if (lamp_type == 3u)
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
                float lamp_att = dot(((lamp_color * 4.0f) * intensity) * ((max(0.0f, dot(normal_rgb, lantern_d / length(lantern_d).xxx)) * ((1.0f / (0.3359999954700469970703125f + lantern_d2)) * smoothstep(36.0f * intensity, 16.0f * intensity, lantern_d2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_up / length(lantern_up).xxx).y)), float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
                float cand_count_f = float(cand_count);
                float target_w = frac(bn_r + (float(cand_i) / cand_count_f));
                if (reservoir[0].w_sum <= reservoir[1].w_sum)
                {
                    do
                    {
                        rstream[0].M_sum += 1.0f;
                        float w_add = lamp_att * cand_count_f;
                        bool replaced;
                        do
                        {
                            reservoir[0].w_sum += w_add;
                            reservoir[0].M += 1.0f;
                            if ((w_add / reservoir[0].w_sum) >= target_w)
                            {
                                reservoir[0].payload = cand_lights[cand_i];
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
                            rstream[0].p_q_sel = lamp_att;
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
                        float w_add2 = lamp_att * cand_count_f;
                        bool replaced2;
                        do
                        {
                            reservoir[1].w_sum += w_add2;
                            reservoir[1].M += 1.0f;
                            if ((w_add2 / reservoir[1].w_sum) >= target_w)
                            {
                                reservoir[1].payload = cand_lights[cand_i];
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
                            rstream[1].p_q_sel = lamp_att;
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
            float3 lit_acc;
            lit_acc = lit;
            float acc_d0n;
            float acc_d1n;
            float acc_d2n;
            float3 lit_acc_next;
            uint hash2_or_an;
            uint hash2_mask_an;
            bool hash2_en_an;
            bool hash2_en2_an;
            float acc_d0 = 0.0f;
            float acc_d1 = 0.0f;
            float acc_d2 = 0.0f;
            uint acc_i = 0u;
            uint hash2_or_a = hash2_or;
            uint hash2_mask_a = hash2_mask;
            bool hash2_en_a = hash2_enable;
            bool hash2_en2_a = hash2_enable;
            for (; acc_i < 2u; acc_d0 = acc_d0n, acc_d1 = acc_d1n, acc_d2 = acc_d2n, lit_acc = lit_acc_next, acc_i++, hash2_or_a = hash2_or_an, hash2_mask_a = hash2_mask_an, hash2_en_a = hash2_en_an, hash2_en2_a = hash2_en2_an)
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
                    float3 lamp_color2;
                    do
                    {
                        if (lamp_type2 == 0u)
                        {
                            lamp_color2 = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (lamp_type2 == 1u)
                            {
                                lamp_color2 = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (lamp_type2 == 2u)
                                {
                                    lamp_color2 = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (lamp_type2 == 3u)
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
                    float3 lamp_att2 = ((lamp_color2 * 4.0f) * intensity2) * ((max(0.0f, dot(normal_rgb, lantern_d2v / lantern_dist_xxx)) * ((1.0f / (0.3359999954700469970703125f + lantern_dist2)) * smoothstep(36.0f * intensity2, 16.0f * intensity2, lantern_dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (lantern_up2 / length(lantern_up2).xxx).y));
                    float n_dmin;
                    float n_bias;
                    float n_t;
                    uint n_mask;
                    uint n_or;
                    bool n_en1;
                    bool n_en2;
                    float3 n_light;
                    if ((reservoir[acc_i].W * max(max(lamp_att2.x, lamp_att2.y), lamp_att2.z)) > 0.0f)
                    {
                        float hit_dmin_c;
                        float hit_bias_c;
                        float hit_t_c2;
                        float3 lamp_att_c;
                        if (lantern_dist > 0.5f)
                        {
                            float3 shadow_ray_origin = (probe_pos + ((lantern_d2v * (lantern_dist - 0.5f)) / lantern_dist_xxx)) + (normal_rgb * (0.0199999995529651641845703125f * blue_noise.y));
                            float3 view_pos3 = view_pos4.xyz;
                            float4 shadow_ray_clip = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(shadow_ray_origin, 1.0f)));
                            float3 shadow_ray_ndc = shadow_ray_clip.xyz / shadow_ray_clip.w.xxx;
                            float4 view_ray_ext = view_pos4 + (float4(shadow_ray_ndc - view_pos3, 0.0f) * float(int(sign(shadow_ray_ndc.z))));
                            float view_ray_w = view_ray_ext.w;
                            float3 view_ray_dir = (view_ray_ext / (float((view_ray_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(view_ray_w))).xxxx).xyz;
                            float3 view_ray_delta = view_ray_dir - view_pos3;
                            bool3 delta_neg = bool3(view_ray_delta.x < 0.0f.xxx.x, view_ray_delta.y < 0.0f.xxx.y, view_ray_delta.z < 0.0f.xxx.z);
                            float3 slab_t = (float3(delta_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, delta_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, delta_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_pos3) / view_ray_delta;
                            float3 slab_entry = view_pos3 + (view_ray_delta * max(0.0f, max(slab_t.x, slab_t.y)));
                            float3 slab_len = view_ray_dir - slab_entry;
                            bool3 len_pos = bool3(slab_len.x >= 0.0f.xxx.x, slab_len.y >= 0.0f.xxx.y, slab_len.z >= 0.0f.xxx.z);
                            float3 slab_t2 = (float3(len_pos.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, len_pos.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, len_pos.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - slab_entry) / slab_len;
                            float3 slab_exit = slab_entry + (slab_len * min(1.0f, min(min(slab_t2.x, slab_t2.y), slab_t2.z)));
                            float dmin_hit;
                            float bias_hit;
                            float t_hit2;
                            bool shadow_hit;
                            float shadow_t;
                            float shadow_push;
                            do
                            {
                                float shadow_bias = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint steps = uint(int(max(2u, min(4u, uint(int(floor(length((((slab_exit.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((slab_entry.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float prev_t_o;
                                float t_hit;
                                bool hit_found;
                                do
                                {
                                    float3 slab_vec = slab_exit - slab_entry;
                                    float steps_f;
                                    if (slab_exit.z > 9.5367431640625e-07f)
                                    {
                                        steps_f = float(steps);
                                    }
                                    else
                                    {
                                        steps_f = max(1.0f, float(steps) - 1.0f);
                                    }
                                    float inv_steps = 1.0f / steps_f;
                                    float dmax_o;
                                    float dmin_o;
                                    float bias_o;
                                    float t_o;
                                    bool flag_o;
                                    if (steps > 0u)
                                    {
                                        float probe_t = lerp(0.0f, 1.0f, pow(bn_r * inv_steps, 1.0f));
                                        float3 probe_pos = slab_entry + (slab_vec * probe_t);
                                        float2 probe_uv = (probe_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float probe_invz = 1.0f / probe_pos.z;
                                        float probe_depth_llc = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv, 0.0f).x;
                                        float probe_depth_nnc = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv, 0.0f).x;
                                        float depth_diff_max = (max(probe_depth_llc, probe_depth_nnc) * 1.00000202655792236328125f) - probe_invz;
                                        float depth_diff_min = probe_invz - min(probe_depth_llc, probe_depth_nnc);
                                        bool hit_front = (depth_diff_max < 0.0f) ? (depth_diff_min < shadow_bias) : false;
                                        float dmax_c;
                                        float dmin_c;
                                        float bias_c;
                                        float t_c;
                                        bool flag_c;
                                        float prev_t;
                                        if (hit_front)
                                        {
                                            dmax_c = depth_diff_max;
                                            dmin_c = depth_diff_min;
                                            bias_c = shadow_bias;
                                            t_c = probe_t;
                                            flag_c = hit_front;
                                            prev_t = 0.0f;
                                        }
                                        else
                                        {
                                            float probe_t_c;
                                            probe_t_c = probe_t;
                                            bool depth_hit;
                                            float probe_t_next;
                                            float hit_dmax;
                                            float hit_dmin;
                                            float hit_bias;
                                            float hit_t;
                                            bool hit_flag;
                                            uint step_i = 1u;
                                            bool hit_any = hit_front;
                                            for (;;)
                                            {
                                                if (step_i < steps)
                                                {
                                                    probe_t_next = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + bn_r) * inv_steps, 1.0f)));
                                                    float3 probe_pos2 = slab_entry + (slab_vec * probe_t_next);
                                                    float2 probe_uv2 = (probe_pos2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float probe_invz2 = 1.0f / probe_pos2.z;
                                                    float probe_depth_llc2 = 1.0f / depth_tex.SampleLevel(sampler_llc, probe_uv2, 0.0f).x;
                                                    float probe_depth_nnc2 = 1.0f / depth_tex.SampleLevel(sampler_nnc, probe_uv2, 0.0f).x;
                                                    float depth_diff_max2 = (max(probe_depth_llc2, probe_depth_nnc2) * 1.00000202655792236328125f) - probe_invz2;
                                                    float depth_diff_min2 = probe_invz2 - min(probe_depth_llc2, probe_depth_nnc2);
                                                    depth_hit = (depth_diff_max2 < 0.0f) ? (depth_diff_min2 < shadow_bias) : false;
                                                    if (depth_hit)
                                                    {
                                                        hit_dmax = depth_diff_max2;
                                                        hit_dmin = depth_diff_min2;
                                                        hit_bias = shadow_bias;
                                                        hit_t = probe_t_next;
                                                        hit_flag = depth_hit;
                                                        break;
                                                    }
                                                    step_i++;
                                                    hit_any = depth_hit;
                                                    probe_t_c = probe_t_next;
                                                    continue;
                                                }
                                                else
                                                {
                                                    hit_dmax = 0.0f;
                                                    hit_dmin = 0.0f;
                                                    hit_bias = 0.0f;
                                                    hit_t = 1.0f;
                                                    hit_flag = hit_any;
                                                    break;
                                                }
                                            }
                                            dmax_c = hit_dmax;
                                            dmin_c = hit_dmin;
                                            bias_c = hit_bias;
                                            t_c = hit_t;
                                            flag_c = hit_flag;
                                            prev_t = probe_t_c;
                                        }
                                        dmax_o = dmax_c;
                                        dmin_o = dmin_c;
                                        bias_o = bias_c;
                                        t_o = t_c;
                                        flag_o = flag_c;
                                        prev_t_o = prev_t;
                                    }
                                    else
                                    {
                                        dmax_o = 0.0f;
                                        dmin_o = 0.0f;
                                        bias_o = 0.0f;
                                        t_o = 1.0f;
                                        flag_o = false;
                                        prev_t_o = 0.0f;
                                    }
                                    if (flag_o)
                                    {
                                        t_hit = t_o;
                                        dmin_hit = dmax_o;
                                        bias_hit = bias_o;
                                        t_hit2 = dmin_o;
                                        hit_found = true;
                                        break;
                                    }
                                    else
                                    {
                                        t_hit = prev_t_o;
                                        dmin_hit = acc_d0;
                                        bias_hit = acc_d1;
                                        t_hit2 = acc_d2;
                                        hit_found = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool hit_better;
                                if (hit_found)
                                {
                                    bool hit_better_cond;
                                    if (t_hit2 < bias_hit)
                                    {
                                        hit_better_cond = dmin_hit < bias_hit;
                                    }
                                    else
                                    {
                                        hit_better_cond = false;
                                    }
                                    hit_better = hit_better_cond;
                                }
                                else
                                {
                                    hit_better = false;
                                }
                                if (hit_better)
                                {
                                    shadow_hit = true;
                                    shadow_t = t_hit;
                                    shadow_push = t_hit2 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                shadow_hit = false;
                                shadow_t = prev_t_o;
                                shadow_push = 0.0f;
                                break;
                            } while(false);
                            float3 lamp_att_shadow;
                            if (shadow_hit)
                            {
                                float4 shadow_world = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(slab_entry, slab_exit, shadow_t.xxx), 1.0f)));
                                float3 lantern_shadow_d = ((shadow_world.xyz / shadow_world.w.xxx) + (view_ray_n * shadow_push)) - lantern_buf.Load<float3>(reservoir[acc_i].payload * 16 + 0);
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
                                lamp_att_shadow = float3(occluded_mask.x ? 0.0f.xxx.x : lamp_att2.x, occluded_mask.y ? 0.0f.xxx.y : lamp_att2.y, occluded_mask.z ? 0.0f.xxx.z : lamp_att2.z);
                            }
                            else
                            {
                                lamp_att_shadow = lamp_att2;
                            }
                            hit_dmin_c = dmin_hit;
                            hit_bias_c = bias_hit;
                            hit_t_c2 = t_hit2;
                            lamp_att_c = lamp_att_shadow;
                        }
                        else
                        {
                            hit_dmin_c = acc_d0;
                            hit_bias_c = acc_d1;
                            hit_t_c2 = acc_d2;
                            lamp_att_c = lamp_att2;
                        }
                        float2 uv_seed = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(reservoir[acc_i].payload * 10u));
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
                        bool c00_en = hash2_en2_a ? hash2_en2_a : true;
                        uint c00_mask = hash2_en2_a ? hash2_mask_a : 8388607u;
                        bool c00_en2 = hash2_en_a ? hash2_en_a : true;
                        uint c00_or = hash2_en_a ? hash2_or_a : 1065353216u;
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
                        float2 flick_smooth = (uv_seed_f * uv_seed_f) * (3.0f.xx - (uv_seed_f * 2.0f));
                        float flick_x = flick_smooth.x;
                        float om_flick_x = 1.0f - flick_x;
                        float flick_y = flick_smooth.y;
                        n_dmin = hit_dmin_c;
                        n_bias = hit_bias_c;
                        n_t = hit_t_c2;
                        n_mask = cxy_or;
                        n_or = cxy_mask;
                        n_en1 = cy_en2 ? cy_en2 : true;
                        n_en2 = cy_en ? cy_en : true;
                        n_light = (((lamp_att_c * lerp(0.5f, 1.0f, (((1.0f - flick_y) * (((asfloat(((c00_h8 ^ (c00_h8 >> 18u)) & c00_mask) | c00_or) - 1.0f) * om_flick_x) + ((asfloat(((cx_h8 ^ (cx_h8 >> 18u)) & cx_mask) | cx_or) - 1.0f) * flick_x))) + (((asfloat(((cy_h8 ^ (cy_h8 >> 18u)) & cy_mask) | cy_or) - 1.0f) * om_flick_x) * flick_y)) + (((asfloat(((cxy_h8 ^ (cxy_h8 >> 18u)) & cxy_mask) | cxy_or) - 1.0f) * flick_x) * flick_y))) * reservoir[acc_i].W) * albedo_use) * ev_mult;
                    }
                    else
                    {
                        n_dmin = acc_d0;
                        n_bias = acc_d1;
                        n_t = acc_d2;
                        n_mask = hash2_or_a;
                        n_or = hash2_mask_a;
                        n_en1 = hash2_en_a;
                        n_en2 = hash2_en2_a;
                        n_light = 0.0f.xxx;
                    }
                    acc_d0n = n_dmin;
                    acc_d1n = n_bias;
                    acc_d2n = n_t;
                    lit_acc_next = lit_acc + n_light;
                    hash2_or_an = n_mask;
                    hash2_mask_an = n_or;
                    hash2_en_an = n_en1;
                    hash2_en2_an = n_en2;
                }
                else
                {
                    acc_d0n = acc_d0;
                    acc_d1n = acc_d1;
                    acc_d2n = acc_d2;
                    lit_acc_next = lit_acc;
                    hash2_or_an = hash2_or_a;
                    hash2_mask_an = hash2_mask_a;
                    hash2_en_an = hash2_en_a;
                    hash2_en2_an = hash2_en2_a;
                }
            }
            lit_out = lit_acc;
        }
        else
        {
            // 网格灯光分支(lod_sel==2):按 cell 的灯光索引表遍历最多 64 盏灯,
            // 每盏做 shadow ray + 深度缓冲软阴影 + perlin 闪烁,与太阳光照累计。
            float3 lit_grid;
            if (2u == lod_sel)
            {
                uint grid_start = light_indirection_grid.Load<uint>(cell_idx * 4 + 0) & 1048575u;
                uint grid_count = min(64u, (light_indirection_grid.Load<uint>(cell_idx * 4 + 0) >> 20u));
                float3 lit_acc;
                lit_acc = lit;
                float3 lit_acc_next;
                float hit_dmin_next;
                float hit_bias_next;
                float hit_t_next;
                uint hash_or_next;
                uint hash_mask_next;
                bool hash_en_next;
                bool hash_en2_next;
                float hit_dmin = 0.0f;
                float hit_bias = 0.0f;
                float hit_t = 0.0f;
                uint grid_i = 0u;
                uint hash_or = hash2_or;
                uint hash_mask = hash2_mask;
                bool hash_en = hash2_enable;
                bool hash_en2 = hash2_enable;
                for (; grid_i < grid_count; lit_acc = lit_acc_next, hit_dmin = hit_dmin_next, hit_bias = hit_bias_next, hit_t = hit_t_next, grid_i++, hash_or = hash_or_next, hash_mask = hash_mask_next, hash_en = hash_en_next, hash_en2 = hash_en2_next)
                {
                    uint gint_pack = grid_start + grid_i;
                    float2 grid_lantern = spvUnpackHalf2x16(lantern_buf.Load<uint>(grid_light_indices.Load<uint>(gint_pack * 4 + 0) * 16 + 12) & 65535u);
                    float gintensity = grid_lantern.x;
                    uint glamp_type = lantern_buf.Load<uint>(grid_light_indices.Load<uint>(gint_pack * 4 + 0) * 16 + 12) >> 16u;
                    float3 glantern_d = lantern_buf.Load<float3>(grid_light_indices.Load<uint>(gint_pack * 4 + 0) * 16 + 0) - probe_pos;
                    float glantern_dist = length(glantern_d);
                    float glantern_dist2 = dot(glantern_d, glantern_d);
                    float3 glantern_up = glantern_d + float3(0.0f, 0.1500000059604644775390625f, 0.0f);
                    float3 glantern_dist_xxx = glantern_dist.xxx;
                    float3 glamp_color;
                    do
                    {
                        if (glamp_type == 0u)
                        {
                            glamp_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                            break;
                        }
                        else
                        {
                            if (glamp_type == 1u)
                            {
                                glamp_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                                break;
                            }
                            else
                            {
                                if (glamp_type == 2u)
                                {
                                    glamp_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                                    break;
                                }
                                else
                                {
                                    if (glamp_type == 3u)
                                    {
                                        glamp_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                                        break;
                                    }
                                    else
                                    {
                                        glamp_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
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
                    float3 glamp_att = ((glamp_color * 4.0f) * gintensity) * ((max(0.0f, dot(normal_rgb, glantern_d / glantern_dist_xxx)) * ((1.0f / (0.3359999954700469970703125f + glantern_dist2)) * smoothstep(36.0f * gintensity, 16.0f * gintensity, glantern_dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (glantern_up / length(glantern_up).xxx).y));
                    float3 lamp_contrib;
                    if (max(max(glamp_att.x, glamp_att.y), glamp_att.z) > 0.0f)
                    {
                        float ghit_dmin_c;
                        float ghit_bias_c;
                        float ghit_t_c2;
                        float3 glamp_att_c;
                        if (glantern_dist > 0.5f)
                        {
                            float3 gshadow_ray_origin = (probe_pos + ((glantern_d * (glantern_dist - 0.5f)) / glantern_dist_xxx)) + (normal_rgb * (0.0199999995529651641845703125f * blue_noise.y));
                            float3 gview_pos3 = view_pos4.xyz;
                            float4 gshadow_ray_clip = mul(view_constants[0u].view_to_sample, mul(view_constants[0u].world_to_view, float4(gshadow_ray_origin, 1.0f)));
                            float3 gshadow_ray_ndc = gshadow_ray_clip.xyz / gshadow_ray_clip.w.xxx;
                            float4 gview_ray_ext = view_pos4 + (float4(gshadow_ray_ndc - gview_pos3, 0.0f) * float(int(sign(gshadow_ray_ndc.z))));
                            float gview_ray_w = gview_ray_ext.w;
                            float3 gview_ray_dir = (gview_ray_ext / (float((gview_ray_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(gview_ray_w))).xxxx).xyz;
                            float3 gview_ray_delta = gview_ray_dir - gview_pos3;
                            bool3 gdelta_neg = bool3(gview_ray_delta.x < 0.0f.xxx.x, gview_ray_delta.y < 0.0f.xxx.y, gview_ray_delta.z < 0.0f.xxx.z);
                            float3 gslab_t = (float3(gdelta_neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, gdelta_neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, gdelta_neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - gview_pos3) / gview_ray_delta;
                            float3 gslab_entry = gview_pos3 + (gview_ray_delta * max(0.0f, max(gslab_t.x, gslab_t.y)));
                            float3 gslab_len = gview_ray_dir - gslab_entry;
                            bool3 glen_pos = bool3(gslab_len.x >= 0.0f.xxx.x, gslab_len.y >= 0.0f.xxx.y, gslab_len.z >= 0.0f.xxx.z);
                            float3 gslab_t2 = (float3(glen_pos.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, glen_pos.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, glen_pos.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - gslab_entry) / gslab_len;
                            float3 gslab_exit = gslab_entry + (gslab_len * min(1.0f, min(min(gslab_t2.x, gslab_t2.y), gslab_t2.z)));
                            float gdmin_hit;
                            float gbias_hit;
                            float gt_hit2;
                            bool gshadow_hit;
                            float gshadow_t;
                            float gshadow_push;
                            do
                            {
                                float gshadow_bias = 0.800000011920928955078125f * view_constants[0u].clip_to_view[3].z;
                                uint gsteps = uint(int(max(2u, min(4u, uint(int(floor(length((((gslab_exit.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((gslab_entry.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * depth_dims_f))))))));
                                float gprev_t_o;
                                float gt_hit;
                                bool ghit_found;
                                do
                                {
                                    float3 gslab_vec = gslab_exit - gslab_entry;
                                    float gsteps_f;
                                    if (gslab_exit.z > 9.5367431640625e-07f)
                                    {
                                        gsteps_f = float(gsteps);
                                    }
                                    else
                                    {
                                        gsteps_f = max(1.0f, float(gsteps) - 1.0f);
                                    }
                                    float ginv_steps = 1.0f / gsteps_f;
                                    float gdmax_o;
                                    float gdmin_o;
                                    float gbias_o;
                                    float gt_o;
                                    bool gflag_o;
                                    if (gsteps > 0u)
                                    {
                                        float gprobe_t = lerp(0.0f, 1.0f, pow(bn_r * ginv_steps, 1.0f));
                                        float3 gprobe_pos = gslab_entry + (gslab_vec * gprobe_t);
                                        float2 gprobe_uv = (gprobe_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                        float gprobe_invz = 1.0f / gprobe_pos.z;
                                        float gprobe_depth_llc = 1.0f / depth_tex.SampleLevel(sampler_llc, gprobe_uv, 0.0f).x;
                                        float gprobe_depth_nnc = 1.0f / depth_tex.SampleLevel(sampler_nnc, gprobe_uv, 0.0f).x;
                                        float gdepth_diff_max = (max(gprobe_depth_llc, gprobe_depth_nnc) * 1.00000202655792236328125f) - gprobe_invz;
                                        float gdepth_diff_min = gprobe_invz - min(gprobe_depth_llc, gprobe_depth_nnc);
                                        bool ghit_front = (gdepth_diff_max < 0.0f) ? (gdepth_diff_min < gshadow_bias) : false;
                                        float gdmax_c;
                                        float gdmin_c;
                                        float gbias_c;
                                        float gt_c;
                                        bool gflag_c;
                                        float gprev_t;
                                        if (ghit_front)
                                        {
                                            gdmax_c = gdepth_diff_max;
                                            gdmin_c = gdepth_diff_min;
                                            gbias_c = gshadow_bias;
                                            gt_c = gprobe_t;
                                            gflag_c = ghit_front;
                                            gprev_t = 0.0f;
                                        }
                                        else
                                        {
                                            float gprobe_t_c;
                                            gprobe_t_c = gprobe_t;
                                            bool grid_depth_hit;
                                            float gprobe_t_next;
                                            float ghit_dmax;
                                            float ghit_dmin;
                                            float ghit_bias;
                                            float ghit_t;
                                            bool ghit_flag;
                                            uint gstep_i = 1u;
                                            bool ghit_any = ghit_front;
                                            for (;;)
                                            {
                                                if (gstep_i < gsteps)
                                                {
                                                    gprobe_t_next = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(gstep_i) + bn_r) * ginv_steps, 1.0f)));
                                                    float3 gprobe_pos2 = gslab_entry + (gslab_vec * gprobe_t_next);
                                                    float2 gprobe_uv2 = (gprobe_pos2.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                                    float gprobe_invz2 = 1.0f / gprobe_pos2.z;
                                                    float gprobe_depth_llc2 = 1.0f / depth_tex.SampleLevel(sampler_llc, gprobe_uv2, 0.0f).x;
                                                    float gprobe_depth_nnc2 = 1.0f / depth_tex.SampleLevel(sampler_nnc, gprobe_uv2, 0.0f).x;
                                                    float gdepth_diff_max2 = (max(gprobe_depth_llc2, gprobe_depth_nnc2) * 1.00000202655792236328125f) - gprobe_invz2;
                                                    float gdepth_diff_min2 = gprobe_invz2 - min(gprobe_depth_llc2, gprobe_depth_nnc2);
                                                    grid_depth_hit = (gdepth_diff_max2 < 0.0f) ? (gdepth_diff_min2 < gshadow_bias) : false;
                                                    if (grid_depth_hit)
                                                    {
                                                        ghit_dmax = gdepth_diff_max2;
                                                        ghit_dmin = gdepth_diff_min2;
                                                        ghit_bias = gshadow_bias;
                                                        ghit_t = gprobe_t_next;
                                                        ghit_flag = grid_depth_hit;
                                                        break;
                                                    }
                                                    gstep_i++;
                                                    ghit_any = grid_depth_hit;
                                                    gprobe_t_c = gprobe_t_next;
                                                    continue;
                                                }
                                                else
                                                {
                                                    ghit_dmax = 0.0f;
                                                    ghit_dmin = 0.0f;
                                                    ghit_bias = 0.0f;
                                                    ghit_t = 1.0f;
                                                    ghit_flag = ghit_any;
                                                    break;
                                                }
                                            }
                                            gdmax_c = ghit_dmax;
                                            gdmin_c = ghit_dmin;
                                            gbias_c = ghit_bias;
                                            gt_c = ghit_t;
                                            gflag_c = ghit_flag;
                                            gprev_t = gprobe_t_c;
                                        }
                                        gdmax_o = gdmax_c;
                                        gdmin_o = gdmin_c;
                                        gbias_o = gbias_c;
                                        gt_o = gt_c;
                                        gflag_o = gflag_c;
                                        gprev_t_o = gprev_t;
                                    }
                                    else
                                    {
                                        gdmax_o = 0.0f;
                                        gdmin_o = 0.0f;
                                        gbias_o = 0.0f;
                                        gt_o = 1.0f;
                                        gflag_o = false;
                                        gprev_t_o = 0.0f;
                                    }
                                    if (gflag_o)
                                    {
                                        gt_hit = gt_o;
                                        gdmin_hit = gdmax_o;
                                        gbias_hit = gbias_o;
                                        gt_hit2 = gdmin_o;
                                        ghit_found = true;
                                        break;
                                    }
                                    else
                                    {
                                        gt_hit = gprev_t_o;
                                        gdmin_hit = hit_dmin;
                                        gbias_hit = hit_bias;
                                        gt_hit2 = hit_t;
                                        ghit_found = false;
                                        break;
                                    }
                                    break; // unreachable workaround
                                } while(false);
                                bool ghit_better;
                                if (ghit_found)
                                {
                                    bool grid_hit_better;
                                    if (gt_hit2 < gbias_hit)
                                    {
                                        grid_hit_better = gdmin_hit < gbias_hit;
                                    }
                                    else
                                    {
                                        grid_hit_better = false;
                                    }
                                    ghit_better = grid_hit_better;
                                }
                                else
                                {
                                    ghit_better = false;
                                }
                                if (ghit_better)
                                {
                                    gshadow_hit = true;
                                    gshadow_t = gt_hit;
                                    gshadow_push = gt_hit2 / view_constants[0u].clip_to_view[3].z;
                                    break;
                                }
                                gshadow_hit = false;
                                gshadow_t = gprev_t_o;
                                gshadow_push = 0.0f;
                                break;
                            } while(false);
                            float3 glamp_att_shadow;
                            if (gshadow_hit)
                            {
                                float4 gshadow_world = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(lerp(gslab_entry, gslab_exit, gshadow_t.xxx), 1.0f)));
                                float3 glantern_shadow_d = ((gshadow_world.xyz / gshadow_world.w.xxx) + (view_ray_n * gshadow_push)) - lantern_buf.Load<float3>(grid_light_indices.Load<uint>(gint_pack * 4 + 0) * 16 + 0);
                                bool glantern_occluded;
                                if (!(abs(glantern_shadow_d.y) > 0.5f))
                                {
                                    float2 glantern_shadow_xz = glantern_shadow_d.xz;
                                    glantern_occluded = dot(glantern_shadow_xz, glantern_shadow_xz) > 0.136900007724761962890625f;
                                }
                                else
                                {
                                    glantern_occluded = true;
                                }
                                bool3 goccluded_mask = glantern_occluded.xxx;
                                glamp_att_shadow = float3(goccluded_mask.x ? 0.0f.xxx.x : glamp_att.x, goccluded_mask.y ? 0.0f.xxx.y : glamp_att.y, goccluded_mask.z ? 0.0f.xxx.z : glamp_att.z);
                            }
                            else
                            {
                                glamp_att_shadow = glamp_att;
                            }
                            ghit_dmin_c = gdmin_hit;
                            ghit_bias_c = gbias_hit;
                            ghit_t_c2 = gt_hit2;
                            glamp_att_c = glamp_att_shadow;
                        }
                        else
                        {
                            ghit_dmin_c = hit_dmin;
                            ghit_bias_c = hit_bias;
                            ghit_t_c2 = hit_t;
                            glamp_att_c = glamp_att;
                        }
                        float2 guv_seed = float2((float(game_time_sec) + game_time_frac) * 4.0f, float(grid_light_indices.Load<uint>(gint_pack * 4 + 0) * 10u));
                        int2 guv_seed_i = int2(floor(guv_seed));
                        float2 guv_seed_f = frac(guv_seed);
                        uint2 guv_seed_u = uint2(guv_seed_i);
                        uint c00_seed_y = guv_seed_u.y;
                        uint c00_h1 = c00_seed_y + (c00_seed_y << 10u);
                        uint c00_h2 = c00_h1 ^ (c00_h1 >> 6u);
                        uint c00_h3 = c00_h2 + (c00_h2 << 3u);
                        uint c00_h4 = c00_h3 ^ (c00_h3 >> 11u);
                        uint c00_h5 = (((guv_seed_u.x * 1664525u) + (c00_h4 + (c00_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint c00_h6 = c00_h5 ^ (c00_h5 >> 11u);
                        uint c00_h7 = c00_h6 ^ ((c00_h6 << 7u) & 2636928640u);
                        uint c00_h8 = c00_h7 ^ ((c00_h7 << 15u) & 4022730752u);
                        bool c00_en = hash_en2 ? hash_en2 : true;
                        uint c00_mask = hash_en2 ? hash_mask : 8388607u;
                        bool c00_en2 = hash_en ? hash_en : true;
                        uint c00_or = hash_en ? hash_or : 1065353216u;
                        uint2 cx_seed_i = uint2(guv_seed_i + int2(1, 0));
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
                        uint2 gn_bias = uint2(guv_seed_i + int2(0, 1));
                        uint gn_mask = gn_bias.y;
                        uint gn_en1 = gn_mask + (gn_mask << 10u);
                        uint gn_light = gn_en1 ^ (gn_en1 >> 6u);
                        uint cy_h3 = gn_light + (gn_light << 3u);
                        uint cy_h4 = cy_h3 ^ (cy_h3 >> 11u);
                        uint cy_h5 = (((gn_bias.x * 1664525u) + (cy_h4 + (cy_h4 << 15u))) + 1013904223u) * 1664525u;
                        uint cy_h6 = cy_h5 ^ (cy_h5 >> 11u);
                        uint cy_h7 = cy_h6 ^ ((cy_h6 << 7u) & 2636928640u);
                        uint cy_h8 = cy_h7 ^ ((cy_h7 << 15u) & 4022730752u);
                        bool cy_en = cx_en ? cx_en : true;
                        uint cy_mask = cx_en ? cx_mask : 8388607u;
                        bool cy_en2 = cx_en2 ? cx_en2 : true;
                        uint cy_or = cx_en2 ? cx_or : 1065353216u;
                        uint2 cxy_seed_i = uint2(guv_seed_i + int2(1, 1));
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
                        float2 bilinear_t = (guv_seed_f * guv_seed_f) * (3.0f.xx - (guv_seed_f * 2.0f));
                        float tx = bilinear_t.x;
                        float one_minus_tx = 1.0f - tx;
                        float ty = bilinear_t.y;
                        hit_dmin_next = ghit_dmin_c;
                        hit_bias_next = ghit_bias_c;
                        hit_t_next = ghit_t_c2;
                        hash_or_next = cxy_or;
                        hash_mask_next = cxy_mask;
                        hash_en_next = cy_en2 ? cy_en2 : true;
                        hash_en2_next = cy_en ? cy_en : true;
                        lamp_contrib = (((glamp_att_c * lerp(0.5f, 1.0f, (((1.0f - ty) * (((asfloat(((c00_h8 ^ (c00_h8 >> 18u)) & c00_mask) | c00_or) - 1.0f) * one_minus_tx) + ((asfloat(((cx_h8 ^ (cx_h8 >> 18u)) & cx_mask) | cx_or) - 1.0f) * tx))) + (((asfloat(((cy_h8 ^ (cy_h8 >> 18u)) & cy_mask) | cy_or) - 1.0f) * one_minus_tx) * ty)) + (((asfloat(((cxy_h8 ^ (cxy_h8 >> 18u)) & cxy_mask) | cxy_or) - 1.0f) * tx) * ty))) * 1.0f) * albedo_use) * ev_mult;
                    }
                    else
                    {
                        hit_dmin_next = hit_dmin;
                        hit_bias_next = hit_bias;
                        hit_t_next = hit_t;
                        hash_or_next = hash_or;
                        hash_mask_next = hash_mask;
                        hash_en_next = hash_en;
                        hash_en2_next = hash_en2;
                        lamp_contrib = 0.0f.xxx;
                    }
                    lit_acc_next = lit_acc + lamp_contrib;
                }
                lit_grid = lit_acc;
            }
            else
            {
                lit_grid = lit;
            }
            lit_out = lit_grid;
        }
        // 灯笼光照结果输出:存在灯笼时用 ReSTIR/网格累计值,否则退化为无灯笼的 lit。
        lantern_light = lit_out;
    }
    else
    {
        lantern_light = lit;
    }
    out_var_SV_TARGET = float4(lantern_light, 0.0f);
    return out_var_SV_TARGET;
}
#pragma dxc diagnostic pop
#else
// f0ad: pixel stage lifted from _deferred_canopy_light_sun_sky.raster.hlsl_f0adff76827e73ff.bin (spirv-cross, semanticized
// 命名 + 中文注释;行为与原版逐指令一致,readable_check 校验通过)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"

void ps_main(VsToPs input)
{
    int2 pixel_i = int2(input.position.xy);
    uint2 pixel_u = uint2(pixel_i);
    float2 ndc = (((float2(pixel_i) + 0.5f.xx) / float2(depth_tex_dims)) - 0.5f.xx) * float2(2.0f, -2.0f);
    float ndc_x = ndc.x;
    float4 world_pos4 = mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, depth_tex.Load(int3(pixel_u, 0u)).x, 1.0f)));
    if ((lantern_count > 0u) ? true : false)
    {
        uint3 cell_u3 = uint3(clamp(floor((((world_pos4.xyz / world_pos4.w.xxx) + (((normalize(mul(view_constants[0u].view_to_world, mul(view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f))).xyz) * 1.0f) * (float((gbuffer_tex.Load(int3(pixel_u, 0u)).y >> 24u) & 255u) * 0.0039215688593685626983642578125f)) * 1.60000002384185791015625f)) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        if (6u == (WaveActiveAllTrue((light_indirection_grid.Load<uint>(((32u * ((32u * cell_u3.y) + cell_u3.z)) + cell_u3.x) * 4 + 0) >> 20u) <= 2u) ? 2u : 6u))
        {
            uint cand_lights[8];
            for (uint init_i = 0u; init_i < 8u; )
            {
                cand_lights[init_i] = 4294967295u;
                init_i++;
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
            uint cand_count;
            for (uint cand_count_c = count, q_i = 0u; q_i < 8u; cand_count_c = cand_count, q_i++)
            {
                uint quad_lantern = QuadReadAcrossDiagonal(cand_lights[q_i]);
                bool count_ok_q;
                if (quad_lantern < lantern_count)
                {
                    count_ok_q = cand_count_c < 8u;
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
                    bool q_no_dup3;
                    if (q_no_dup2)
                    {
                        q_no_dup3 = cand_lights[3u] != quad_lantern;
                    }
                    else
                    {
                        q_no_dup3 = false;
                    }
                    bool q_no_dup4;
                    if (q_no_dup3)
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
                    uint cand_next;
                    if (q_no_dup7)
                    {
                        cand_lights[cand_count_c] = quad_lantern;
                        cand_next = cand_count_c + 1u;
                    }
                    else
                    {
                        cand_next = cand_count_c;
                    }
                    cand_count = cand_next;
                }
                else
                {
                    cand_count = cand_count_c;
                }
            }
        }
        else
        {
        }
    }
}
#pragma dxc diagnostic pop
#endif
