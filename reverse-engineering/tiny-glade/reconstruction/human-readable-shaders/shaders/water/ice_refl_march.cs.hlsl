// PATCHED: modules ch.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.

// 冰面反射步进:每反射像素做光线设置、Hilbert 曲线有序蓝噪声反射采样(8 次尝试),
// 半分辨率深度步进 + 括住细化;命中写历史,未命中把射线发射进软件 RT 射线缓冲
// (morton 计数记账),并写 refl_hit/辐射度。
// 转译段(TGV_0,变量为 SPIR-V 编号)已语义化命名;逻辑未改。
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/ice_refl_march.cs.hlsl
// SOURCE PATH : shaders/water/ice_refl_march.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_refl_march.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [64,1,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — ice reflection march: per-reflected-pixel ray setup,
//   Hilbert-curve-ordered blue-noise reflection sampling (8 tries), half-res
//   depth march with bracketed refinement, history write or ray emission into
//   the software-RT ray buffer (morton-count bookkeeping), refl_hit/radiance
//   writes.
// MERGE (water-ice review water/ice_refl_march.cs.hlsl): A base adopted wholesale;
//   folded in the B/bin accept-gate polarity fix — the bin accepts the sample when
//   `reflect(basis, v).z < 0` (B matches), whereas A inverted it (retried on
//   reflect.z<0). The Hilbert index + retry-on-bad structure are A's (match the bin).

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
// inc/blue_noise.hlsl 在 t14 声明 blue_noise_texture;水面/冰面 RT 家族
// 在 t15 绑定(消费者不嵌入 blue_noise.hlsl 包含)。本地声明。
#include "inc/hammersley.hlsl"
#include "inc/math_const.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/rt.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
// terrain_heightmap.inc 已省略(重建 inc 引用消费者声明的纹理,本着色器从不绑定;
// 原版内嵌它但不带绑定)。
#include "inc/material_eval.inc.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/history_valid.hlsl"
#include "ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 ice_roughness_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
};

ByteAddressBuffer ice_refl_count_buffer : register(t25, space0);
ByteAddressBuffer ice_refl_px_buffer : register(t26, space0);
RWByteAddressBuffer __alias_ice_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_ice_rt_ray_buffer : register(u34, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
Texture2D<float4> ice_roughness_tex : register(t30, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> refl_hit_tex : register(u31, space0);
[[vk::image_format("r32ui")]] RWTexture2D<uint> refl_hit_radiance_tex : register(u32, space0);

// 切空间坐标交换 (x, y, z) -> (x, -z, y);见 ice_refr_resolve。
float3 swap_yz(float3 v)
{
    return mul(v, float3x3(float3(1.0f, -0.0f, 0.0f),
                          float3(-0.0f, 0.0f, 1.0f),
                          float3(-0.0f, -1.0f, 0.0f)));
}

// 半分辨率像素 (x, y) 在 64x64 块内的 Hilbert 曲线索引。
uint hilbert_index(uint x, uint y)
{
    uint acc = 0u;
    uint xi = x;
    uint yi = y;
    for (uint s = 32u; s > 0u; s /= 2u)
    {
        uint bx = uint((xi & s) > 0u);
        uint by = uint((yi & s) > 0u);
        acc += (s * s) * ((3u * bx) ^ by);
        if (by == 0u)
        {
            if (bx == 1u)
            {
                xi = 63u - xi;
                yi = 63u - yi;
            }
        }
    }
    return acc;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (gtid.x >= ice_refl_count_buffer.Load<uint>(12))
    {
        return;
    }

    uint px = ice_refl_px_buffer.Load<uint>(gtid.x * 4u + 0u);
    uint px_x = px & 65535u;
    uint px_y = (px >> 16u) & 65535u;
    uint2 pixel = uint2(px_x, px_y);
    float2 half_dims = float2(half_depth_tex_dims);
    float2 ndc = (((float2(int2(pixel)) + 0.5f) / half_dims) - 0.5f) * float2(2.0f, -2.0f);

    float3 view_ray = normalize(mul(view_constants[0].view_to_world,
        mul(view_constants[0].sample_to_view, float4(ndc, 0.0f, 1.0f))).xyz);
    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam4.xyz / cam4.w;
    float3 ground = cam_pos + (view_ray * ((-cam_pos.y) / view_ray.y));
    float4 g4 = mul(view_constants[0].view_to_sample, mul(view_constants[0].world_to_view, float4(ground, 1.0f)));
    float4 pos_vs = float4(ndc, (g4.xyz / g4.w).z, 1.0f);

    uint frame_mod = frame_index % 64u;
    float frame_rand = 5.588237762451171875f * float(frame_mod);
    float h = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_x) + frame_rand))
        + (0.005837149918079376220703125f * (float(px_y) + frame_rand))));

    float4 rough4 = ice_roughness_tex.Load(int3(pixel * uint2(2u, 2u), 0u));
    float rough = rough4.x;
    float rough2 = max(0.000600000028498470783233642578125f, rough * rough);

    float3 view_dir = swap_yz(-view_ray);

    // 反射方向重要性采样(最多 8 次尝试保持在水平线上方)。
    float3 refl_dir = 0.0f;
    uint k = 0u;
    for (;;)
    {
        if (k < 8u)
        {
            uint curve = hilbert_index(pixel.x, pixel.y) + (288u * frame_mod);
            float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(curve)) + 0.5f);

            float3 v = 0.0f;
            bool rejected = true;
            while (true)
            {
                float3 z = normalize(float3(rough2 * view_dir.x, rough2 * view_dir.y, view_dir.z));
                float3 binorm = (z.z < 0.99989998340606689453125f) ? normalize(cross(float3(0.0f, 0.0f, 1.0f), z)) : float3(1.0f, 0.0f, 0.0f);
                float r = sqrt(bn.x);
                float theta = bn.y * 3.1415927410125732421875f;
                float c1 = r * cos(theta);
                float s1 = r * sin(theta);
                float hh = 0.5f * (1.0f + z.z);
                float c2 = (1.0f - hh) * sqrt(1.0f - (c1 * c1)) + (hh * s1);
                float3 dir = (binorm * c1) + (cross(z, binorm) * c2) + (z * sqrt(max(0.0f, (1.0f - (c1 * c1)) - (c2 * c2))));
                float3 dir2 = normalize(float3(rough2 * dir.x, rough2 * dir.y, max(0.0f, dir.z)));

                bool bad = (dir2.z <= 9.9999997473787516355514526367188e-06f)
                    || (reflect(-view_dir, dir2).z <= 9.9999997473787516355514526367188e-06f);
                if (bad || view_dir.z <= 9.9999997473787516355514526367188e-06f)
                {
                    v = 0.0f;
                    rejected = true;
                    break;
                }
                v = dir2;
                rejected = false;
                break;
            }
            // Bin accept-gate polarity: accept when reflect(basis, v).z < 0
            // (a bad sample has v=0 -> reflect.z=0 -> retry below).
            if (!rejected && reflect(view_dir, v).z < 0.0f)
            {
                refl_dir = v;
                break;
            }
            k++;
            continue;
        }
        else
        {
            refl_dir = float3(0.0f, 1.0f, 0.0f);
            break;
        }
    }

    float3 refl = -reflect(-view_ray, swap_yz(refl_dir));
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }

    // 让反射射线对半分辨率深度缓冲进行步进。
    float3 march_start = pos_vs.xyz;
    float4 ref_end = pos_vs + mul(view_constants[0].view_to_clip, mul(view_constants[0].world_to_view, float4(refl, 0.0f)));
    float3 ref_dir = (ref_end / (float((ref_end.w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(ref_end.w)))).xyz - march_start;
    bool3 neg = ref_dir < 0.0f;
    float3 t0 = (float3(neg.x ? 1.0f : -1.0f, neg.y ? 1.0f : -1.0f, neg.z ? 1.0f : -1.0f) - march_start) / ref_dir;
    float3 start = march_start + (ref_dir * max(0.0f, max(t0.x, t0.y)));
    float3 r2 = ref_dir - (start - march_start);
    bool3 pos3 = r2 >= 0.0f;
    float3 t1 = (float3(pos3.x ? 1.0f : -1.0f, pos3.y ? 1.0f : -1.0f, pos3.z ? 1.0f : -1.0f) - start) / r2;
    float3 end = start + (r2 * min(min(t1.x, t1.y), t1.z));

    float2 start_uv = (start.xy * float2(0.5f, -0.5f)) + 0.5f;
    float2 end_uv = (end.xy * float2(0.5f, -0.5f)) + 0.5f;

    bool hit = false;
    float hit_t = 0.0f;
    float2 hit_uv = 0.0f;

    uint steps = uint(int(max(2u, min(16u, uint(int(floor(length((end_uv - start_uv) * half_dims))))))));
    float step_count = (end.z > 9.5367431640625e-07f) ? float(steps) : max(1.0f, float(steps) - 1.0f);
    float inv_steps = 1.0f / step_count;

    float prev_e = 0.0f;
    float prev_hit = 0.0f;
    float prev_diff = 0.0f;
    float prev_depth = 0.0f;
    float prev_ray = 0.0f;
    bool prev_under = false;
    float prev_t = 0.0f;

    if (steps > 0u)
    {
        float t_first = lerp(0.0f, 1.0f, pow(h * inv_steps, 2.0f));
        float3 sp = start + (r2 * t_first);
        float inv_z = 1.0f / sp.z;
        float d = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diff = (1.00000202655792236328125f / d) - inv_z;
        bool under = (diff < 0.0f);
        prev_e = under ? 0.0f : diff;
        prev_hit = under ? diff : 0.0f;
        prev_diff = under ? inv_z - (1.0f / d) : 0.0f;
        prev_depth = under ? view_constants[0].clip_to_view[3].z : 0.0f;
        prev_ray = under ? t_first : 0.0f;
        prev_under = under;
        prev_t = 0.0f;

        for (uint i = 1u; i < steps; ++i)
        {
            float tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + h) * inv_steps, 2.0f)));
            float3 sp2 = start + (r2 * tt);
            float inv_z2 = 1.0f / sp2.z;
            float d2 = half_depth_tex.SampleLevel(sampler_nnc, (sp2.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diff2 = (1.00000202655792236328125f / d2) - inv_z2;
            bool under2 = (diff2 < 0.0f);
            if (under2)
            {
                prev_hit = diff2;
                prev_diff = inv_z2 - (1.0f / d2);
                prev_depth = view_constants[0].clip_to_view[3].z;
                prev_ray = tt;
                prev_under = true;
                break;
            }
            prev_e = diff2;
            prev_under = under2;
            prev_t = tt;
        }
    }

    if (prev_under)
    {
        // 用几次二分步细化。
        float lo = prev_e;
        float hi = prev_ray;
        float d_lo = prev_hit;
        float d_hi = prev_diff;
        float ray_hi = prev_depth;
        float ray_lo = prev_under ? prev_t : 0.0f;
        float t_hi = prev_ray;
        for (uint j = 0u; j < 3u; ++j)
        {
            float mid = lo + t_hi;
            mid *= 0.5f;
            float3 sm = start + (r2 * mid);
            float inv_zm = 1.0f / sm.z;
            float dm = half_depth_tex.SampleLevel(sampler_nnc, (sm.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
            float diffm = (1.00000202655792236328125f / dm) - inv_zm;
            bool under_m = (diffm < 0.0f);
            lo = under_m ? lo : mid;
            d_lo = under_m ? d_lo : diffm;
            d_hi = under_m ? diffm : d_hi;
            ray_hi = under_m ? ray_hi : (inv_zm - (1.0f / dm));
            t_hi = under_m ? mid : t_hi;
        }
        float interp = lerp(lo, t_hi, d_lo / (d_lo + (-d_hi)));
        float3 fp = start + (r2 * interp);
        float inv_zp = 1.0f / fp.z;
        float dp = half_depth_tex.SampleLevel(sampler_nnc, (fp.xy * float2(0.5f, -0.5f)) + 0.5f, 0.0f).x;
        float diffp = (1.00000202655792236328125f / dp) - inv_zp;
        bool converged = (abs(diffp) < (d_lo * 0.89999997615814208984375f));
        hit = converged;
        hit_t = converged ? interp : t_hi;
        hit_uv = converged ? lerp(start_uv, end_uv, interp) : lerp(start_uv, end_uv, prev_e);
    }
    else
    {
        hit = false;
        hit_t = prev_e;
        hit_uv = lerp(start_uv, end_uv, prev_e);
    }

    float dist;
    float3 history_color;
    if (hit)
    {
        float3 history;
        if (history_valid != 0u && frame_index != 0u)
        {
            history = taa_history_tex.SampleLevel(sampler_lnc,
                reprojection_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).xy + hit_uv, 0.0f).xyz;
        }
        else
        {
            history = 0.0f;
        }
        float4 wp = mul(view_constants[0].view_to_world, mul(view_constants[0].clip_to_view,
            float4(lerp(start, end, hit_t), 1.0f)));
        dist = distance(wp.xyz / wp.w, ground);
        history_color = history;
    }
    else
    {
        float4 wp = mul(view_constants[0].view_to_world,
            mul(view_constants[0].sample_to_view, float4(lerp(start, end, hit_t), 1.0f)));
        float3 wp_xyz = wp.xyz / wp.w;
        uint slot;
        __alias_ice_rt_count_buffer.InterlockedAdd(12, 1u, slot);
        float3 origin = wp_xyz - cam_pos;
        __alias_ice_rt_ray_buffer.Store<uint4>(slot * 16u + 0u,
            uint4(px_x + (px_y * 65536u),
                pack_half2x16(float2(origin.x, 0.0f)) | (pack_half2x16(float2(refl.x, 0.0f)) << 16u),
                pack_half2x16(float2(origin.y, 0.0f)) | (pack_half2x16(float2(refl.y, 0.0f)) << 16u),
                pack_half2x16(float2(origin.z, 0.0f)) | (pack_half2x16(float2(refl.z, 0.0f)) << 16u)));
        if ((slot & 63u) == 0u)
        {
            uint group;
            __alias_ice_rt_count_buffer.InterlockedAdd(0, 1u, group);
        }
        dist = distance(wp_xyz, ground);
        history_color = 0.0f;
    }

    refl_hit_tex[pixel] = float2(dist, rough2);
    float3 clamped = clamp(history_color, 0.0f, 65408.0f);
    float scale = asfloat((asuint(max(max(1.52587890625e-05f, clamped.x), max(clamped.y, clamped.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(clamped + scale);
    refl_hit_radiance_tex[pixel] = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
}

#endif


#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译段:与手写段相同算法的 spirv-cross 输出(语义化命名)。 && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_ice_refl_march.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static float ZERO_F;

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 brdf_fg_lut_dims : packoffset(c2);
    uint rt_bvh_node_count : packoffset(c2.z);
    uint rt_triangle_count : packoffset(c2.w);
    uint history_valid : packoffset(c3);
    uint2 half_depth_tex_dims : packoffset(c3.y);
    uint2 taa_history_tex_dims : packoffset(c4);
    uint2 reprojection_tex_dims : packoffset(c4.z);
    uint2 ice_roughness_tex_dims : packoffset(c5);
    uint2 refl_hit_tex_dims : packoffset(c5.z);
    uint2 refl_hit_radiance_tex_dims : packoffset(c6);
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

ByteAddressBuffer ice_refl_count_buffer : register(t25, space0);
ByteAddressBuffer ice_refl_px_buffer : register(t26, space0);
RWByteAddressBuffer __alias_ice_rt_count_buffer : register(u33, space0);
RWByteAddressBuffer __alias_ice_rt_ray_buffer : register(u34, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
Texture2D<float4> ice_roughness_tex : register(t30, space0);
RWTexture2D<float2> refl_hit_tex : register(u31, space0);
RWTexture2D<uint> refl_hit_radiance_tex : register(u32, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

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
        if (gl_GlobalInvocationID.x >= ice_refl_count_buffer.Load<uint>(12))
        {
            break;
        }
        uint px_packed = ice_refl_px_buffer.Load<uint>(gl_GlobalInvocationID.x * 4 + 0);
        uint px_x = px_packed & 65535u;
        uint px_y = (px_packed >> 16u) & 65535u;
        uint2 pixel = uint2(px_x, px_y);
        float2 half_dims = float2(half_depth_tex_dims);
        float2 ndc = (((float2(int2(pixel)) + 0.5f.xx) / half_dims) - 0.5f.xx) * float2(2.0f, -2.0f);
        float ndc_x = ndc.x;
        float3 view_ray = normalize(mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(ndc_x, ndc.y, 0.0f, 1.0f))).xyz);
        float3 neg_view_ray = -view_ray;
        float4 cam4 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 cam_pos = cam4.xyz / cam4.w.xxx;
        float3 ground = cam_pos + (view_ray * ((-cam_pos).y / view_ray.y));
        float4 g4 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(ground, 1.0f)));
        float4 pos_vs = float4(ndc_x, ndc.y, (g4.xyz / g4.w.xxx).z, 1.0f);
        uint frame_mod = frame_constants_frame_index % 64u;
        float frame_rand = 5.588237762451171875f * float(frame_mod);
        float h = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(px_x) + frame_rand)) + (0.005837149918079376220703125f * (float(px_y) + frame_rand))));
        float4 rough4 = ice_roughness_tex.Load(int3(pixel * uint2(2u, 2u), 0u));
        float rough = rough4.x;
        float rough2 = max(0.000600000028498470783233642578125f, rough * rough);
        float3 view_dir = mul(neg_view_ray, float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)));
        float3 refl_dir;
        uint k = 0u;
        for (;;)
        {
            if (k < 8u)
            {
                uint hilbert_acc;
                hilbert_acc = 0u;
                uint hilbert_next;
                uint next_yi;
                uint next_xi;
                for (uint yi = px_y, xi = px_x, s = 32u; s > 0u; yi = next_yi, xi = next_xi, hilbert_acc = hilbert_next, s /= 2u)
                {
                    uint bx = uint((xi & s) > 0u);
                    uint by = uint((yi & s) > 0u);
                    hilbert_next = hilbert_acc + ((s * s) * ((3u * bx) ^ by));
                    if (by == 0u)
                    {
                        uint new_yi;
                        uint new_xi;
                        if (bx == 1u)
                        {
                            new_yi = 63u - yi;
                            new_xi = 63u - xi;
                        }
                        else
                        {
                            new_yi = yi;
                            new_xi = xi;
                        }
                        next_yi = new_xi;
                        next_xi = new_yi;
                    }
                    else
                    {
                        next_yi = yi;
                        next_xi = xi;
                    }
                }
                float2 bn = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(hilbert_acc + (288u * frame_mod))) + 0.5f.xx);
                float3 v;
                do
                {
                    float view_dir_z = view_dir.z;
                    float3 z = normalize(float3(rough2 * view_dir.x, rough2 * view_dir.y, view_dir_z));
                    float z_z = z.z;
                    float3 binorm;
                    if (z_z < 0.99989998340606689453125f)
                    {
                        binorm = normalize(cross(float3(0.0f, 0.0f, 1.0f), z));
                    }
                    else
                    {
                        binorm = float3(1.0f, 0.0f, 0.0f);
                    }
                    float r = sqrt(bn.x);
                    float theta = bn.y * 3.1415927410125732421875f;
                    float c1 = r * cos(theta);
                    float hh = 0.5f * (1.0f + z_z);
                    float one_minus_c1sq = 1.0f - (c1 * c1);
                    float c2 = ((1.0f - hh) * sqrt(one_minus_c1sq)) + (hh * (r * sin(theta)));
                    float3 dir = ((binorm * c1) + (cross(z, binorm) * c2)) + (z * sqrt(max(0.0f, one_minus_c1sq - (c2 * c2))));
                    float3 dir2 = normalize(float3(rough2 * dir.x, rough2 * dir.y, max(0.0f, dir.z)));
                    bool bad;
                    if (!(dir2.z <= 9.9999997473787516355514526367188e-06f))
                    {
                        bad = reflect(-view_dir, dir2).z <= 9.9999997473787516355514526367188e-06f;
                    }
                    else
                    {
                        bad = true;
                    }
                    bool rejected;
                    if (!bad)
                    {
                        rejected = view_dir_z <= 9.9999997473787516355514526367188e-06f;
                    }
                    else
                    {
                        rejected = true;
                    }
                    if (rejected)
                    {
                        v = 0.0f.xxx;
                        break;
                    }
                    v = dir2;
                    break;
                } while(false);
                if (reflect(view_dir, v).z < 0.0f)
                {
                    refl_dir = v;
                    break;
                }
                k++;
                continue;
            }
            else
            {
                refl_dir = float3(0.0f, 1.0f, 0.0f);
                break;
            }
        }
        float3 refl = -reflect(neg_view_ray, mul(float3x3(float3(1.0f, -0.0f, 0.0f), float3(-0.0f, 0.0f, 1.0f), float3(-0.0f, -1.0f, 0.0f)), refl_dir));
        float refl_y = refl.y;
        float3 refl_fixed;
        if (refl_y < 0.0f)
        {
            float3 refl_flip = refl;
            refl_flip.y = -refl_y;
            refl_fixed = refl_flip;
        }
        else
        {
            refl_fixed = refl;
        }
        float4x4 clip_to_view;
        float3 march_start = pos_vs.xyz;
        float4 ref_end = pos_vs + mul(frame_constants_view_constants[0u].view_to_clip, mul(frame_constants_view_constants[0u].world_to_view, float4(refl_fixed, 0.0f)));
        float ref_end_w = ref_end.w;
        float3 ref_end_norm = (ref_end / (float((ref_end_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(ref_end_w))).xxxx).xyz;
        float3 ref_dir = ref_end_norm - march_start;
        bool3 neg = bool3(ref_dir.x < 0.0f.xxx.x, ref_dir.y < 0.0f.xxx.y, ref_dir.z < 0.0f.xxx.z);
        float3 t0 = (float3(neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - march_start) / ref_dir;
        float3 start = march_start + (ref_dir * max(0.0f, max(t0.x, t0.y)));
        float3 r2 = ref_end_norm - start;
        bool3 pos3 = bool3(r2.x >= 0.0f.xxx.x, r2.y >= 0.0f.xxx.y, r2.z >= 0.0f.xxx.z);
        float3 t1 = (float3(pos3.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, pos3.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, pos3.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - start) / r2;
        float3 end = start + (r2 * min(min(t1.x, t1.y), t1.z));
        bool hit;
        float hit_t;
        float2 hit_uv;
        do
        {
            float2 start_uv = (start.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float2 end_uv = (end.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            clip_to_view = frame_constants_view_constants[0u].clip_to_view;
            uint steps = uint(int(max(2u, min(16u, uint(int(floor(length((end_uv - start_uv) * half_dims))))))));
            float prev_t2;
            float hit_t2;
            float diff_final;
            float depth_final;
            float diff_under;
            bool converged_final;
            do
            {
                float3 march_dir = end - start;
                float step_count;
                if (end.z > 9.5367431640625e-07f)
                {
                    step_count = float(steps);
                }
                else
                {
                    step_count = max(1.0f, float(steps) - 1.0f);
                }
                float inv_steps = 1.0f / step_count;
                float prev_e2;
                float prev_hit2;
                float prev_diff2;
                float prev_depth2;
                float prev_ray2;
                bool prev_under2;
                if (steps > 0u)
                {
                    float t_first = lerp(0.0f, 1.0f, pow(h * inv_steps, 2.0f));
                    float3 sp = start + (march_dir * t_first);
                    float inv_z = 1.0f / sp.z;
                    float4 d_tex = half_depth_tex.SampleLevel(sampler_nnc, (sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float d = d_tex.x;
                    float diff = (1.00000202655792236328125f / d) - inv_z;
                    bool under = (diff < 0.0f) ? true : false;
                    float prev_e;
                    float prev_hit;
                    float prev_diff;
                    float prev_depth;
                    float prev_ray;
                    bool prev_under;
                    float prev_t;
                    if (under)
                    {
                        prev_e = 0.0f;
                        prev_hit = diff;
                        prev_diff = inv_z - (1.0f / d);
                        prev_depth = clip_to_view[3].z;
                        prev_ray = t_first;
                        prev_under = under;
                        prev_t = 0.0f;
                    }
                    else
                    {
                        float e_cur;
                        float t_cur;
                        e_cur = diff;
                        t_cur = t_first;
                        float diff2;
                        bool under2;
                        float tt;
                        float next_hit;
                        float next_diff;
                        float next_depth;
                        float next_ray;
                        bool next_under;
                        uint i = 1u;
                        bool under_carry = under;
                        for (;;)
                        {
                            if (i < steps)
                            {
                                tt = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(i) + h) * inv_steps, 2.0f)));
                                float3 sp2 = start + (march_dir * tt);
                                float inv_z2 = 1.0f / sp2.z;
                                float4 d2_tex = half_depth_tex.SampleLevel(sampler_nnc, (sp2.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                                float d2 = d2_tex.x;
                                diff2 = (1.00000202655792236328125f / d2) - inv_z2;
                                under2 = (diff2 < 0.0f) ? true : false;
                                if (under2)
                                {
                                    next_hit = diff2;
                                    next_diff = inv_z2 - (1.0f / d2);
                                    next_depth = clip_to_view[3].z;
                                    next_ray = tt;
                                    next_under = under2;
                                    break;
                                }
                                i++;
                                e_cur = diff2;
                                under_carry = under2;
                                t_cur = tt;
                                continue;
                            }
                            else
                            {
                                next_hit = 0.0f;
                                next_diff = 0.0f;
                                next_depth = 0.0f;
                                next_ray = 1.0f;
                                next_under = under_carry;
                                break;
                            }
                        }
                        prev_e = e_cur;
                        prev_hit = next_hit;
                        prev_diff = next_diff;
                        prev_depth = next_depth;
                        prev_ray = next_ray;
                        prev_under = next_under;
                        prev_t = t_cur;
                    }
                    prev_e2 = prev_e;
                    prev_hit2 = prev_hit;
                    prev_diff2 = prev_diff;
                    prev_depth2 = prev_depth;
                    prev_ray2 = prev_ray;
                    prev_under2 = prev_under;
                    prev_t2 = prev_t;
                }
                else
                {
                    prev_e2 = 0.0f;
                    prev_hit2 = 0.0f;
                    prev_diff2 = 0.0f;
                    prev_depth2 = 0.0f;
                    prev_ray2 = 1.0f;
                    prev_under2 = false;
                    prev_t2 = 0.0f;
                }
                if (prev_under2)
                {
                    float lo;
                    float d_lo;
                    float d_hi;
                    float ray_diff;
                    float ray_hi;
                    float t_hi;
                    lo = prev_t2;
                    d_lo = prev_e2;
                    d_hi = prev_hit2;
                    ray_diff = prev_diff2;
                    ray_hi = prev_depth2;
                    t_hi = prev_ray2;
                    for (uint j = 0u; j < 3u; )
                    {
                        float mid_plus = lo + t_hi;
                        float mid = mid_plus * 0.5f;
                        float3 sm = start + (march_dir * mid);
                        float inv_zm = 1.0f / sm.z;
                        float4 dm_tex = half_depth_tex.SampleLevel(sampler_nnc, (sm.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                        float dm = dm_tex.x;
                        float diffm = (1.00000202655792236328125f / dm) - inv_zm;
                        bool under_m = (diffm < 0.0f) ? true : false;
                        lo = under_m ? lo : mid;
                        d_lo = under_m ? d_lo : diffm;
                        d_hi = under_m ? diffm : d_hi;
                        ray_diff = under_m ? (inv_zm - (1.0f / dm)) : ray_diff;
                        ray_hi = under_m ? clip_to_view[3].z : ray_hi;
                        t_hi = under_m ? mid : t_hi;
                        j++;
                        continue;
                    }
                    float interp = lerp(lo, t_hi, d_lo / (d_lo + (-d_hi)));
                    float3 fp = start + (march_dir * interp);
                    float inv_zp = 1.0f / fp.z;
                    float4 dp_tex = half_depth_tex.SampleLevel(sampler_nnc, (fp.xy * float2(0.5f, -0.5f)) + 0.5f.xx, 0.0f);
                    float dp = dp_tex.x;
                    float diffp = (1.00000202655792236328125f / dp) - inv_zp;
                    bool converged = (abs(diffp) < (d_lo * 0.89999997615814208984375f)) ? true : false;
                    hit_t2 = converged ? interp : t_hi;
                    diff_final = converged ? diffp : d_hi;
                    depth_final = converged ? clip_to_view[3].z : ray_hi;
                    diff_under = converged ? (inv_zp - (1.0f / dp)) : ray_diff;
                    converged_final = true;
                    break;
                }
                else
                {
                    hit_t2 = prev_t2;
                    diff_final = ZERO_F;
                    depth_final = ZERO_F;
                    diff_under = ZERO_F;
                    converged_final = false;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            bool hit_ok;
            if (converged_final)
            {
                bool hit_check;
                if (diff_under < depth_final)
                {
                    hit_check = diff_final < depth_final;
                }
                else
                {
                    hit_check = false;
                }
                hit_ok = hit_check;
            }
            else
            {
                hit_ok = false;
            }
            if (hit_ok)
            {
                hit = true;
                hit_t = hit_t2;
                hit_uv = lerp(start_uv, end_uv, hit_t2.xx);
                break;
            }
            hit = false;
            hit_t = prev_t2;
            hit_uv = lerp(start_uv, end_uv, prev_t2.xx);
            break;
        } while(false);
        float4 lerp_pos = float4(lerp(start, end, hit_t.xxx), 1.0f);
        float dist;
        float3 history_color;
        if (hit)
        {
            bool use_history;
            if (history_valid != 0u)
            {
                use_history = !(0u == frame_constants_frame_index);
            }
            else
            {
                use_history = false;
            }
            float3 history;
            if (use_history)
            {
                history = taa_history_tex.SampleLevel(sampler_lnc, reprojection_tex.SampleLevel(sampler_nnc, hit_uv, 0.0f).xy + hit_uv, 0.0f).xyz;
            }
            else
            {
                history = 0.0f.xxx;
            }
            float4 wp = mul(frame_constants_view_constants[0u].view_to_world, mul(clip_to_view, lerp_pos));
            dist = distance(wp.xyz / wp.w.xxx, ground);
            history_color = history;
        }
        else
        {
            float4 wp_miss = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, lerp_pos));
            float3 wp_xyz = wp_miss.xyz / wp_miss.w.xxx;
            uint slot;
            __alias_ice_rt_count_buffer.InterlockedAdd(12, 1u, slot);
            float3 origin = wp_xyz - cam_pos;
            __alias_ice_rt_ray_buffer.Store<uint4>(slot * 16 + 0, uint4(px_x + (px_y * 65536u), spvPackHalf2x16(float2(origin.x, 0.0f)) | (spvPackHalf2x16(float2(refl_fixed.x, 0.0f)) << 16u), spvPackHalf2x16(float2(origin.y, 0.0f)) | (spvPackHalf2x16(float2(refl_fixed.y, 0.0f)) << 16u), spvPackHalf2x16(float2(origin.z, 0.0f)) | (spvPackHalf2x16(float2(refl_fixed.z, 0.0f)) << 16u)));
            if ((slot & 63u) == 0u)
            {
                uint group;
                __alias_ice_rt_count_buffer.InterlockedAdd(0, 1u, group);
            }
            dist = distance(wp_xyz, ground);
            history_color = 0.0f.xxx;
        }
        refl_hit_tex[pixel] = float2(dist, rough2);
        float3 clamped = clamp(history_color, 0.0f.xxx, 65408.0f.xxx);
        float scale = asfloat((asuint(max(max(1.52587890625e-05f, clamped.x), max(clamped.y, clamped.z))) + 125845504u) & 2139095040u);
        uint3 mantissa = asuint(clamped + scale.xxx);
        refl_hit_radiance_tex[pixel] = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
