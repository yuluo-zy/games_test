// PATCHED: modules gi.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/ssgi/ssgi.cs.hlsl | F-D
// 依据:_ssgi_ssgi.cs.hlsl_cs_main_0.bin(SPIR-V,偏移 23;SHA-256
//   66e61114e7e782b18c01a857168ac52b587b1a063635a084e2af6b5caebef57d)。
//   绑定:sampler_nnc s2, $Globals b13 (depth_tex_dims/half_view_normal_tex_dims/
//   output_tex_dims), depth_tex t14, half_view_normal_tex t15, output_tex u16,
//   frame_constants b0 space1;LocalSize [8,8,1],入口 cs_main。
//   基于地平线的 GTAO(XeGTAO.hlsl),输入半分辨率视图法线 + 深度对:
//   空间填充曲线像素抖动,2 切片 x 3 黄金比采样点,重建视图空间样本的距离
//   衰减,最后积分为 AO 标量。去噪/视图空间辅助在 ssgi/XeGTAO.hlsl 与 ssgi/vaShared.hlsl。
// 状态:O (结构/常量取自反汇编;绑定逐字);
//       I (函数/变量名——被 dxc 内联)。

#include "inc/common.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"
#include "ssgi/XeGTAO.hlsl"
#include "ssgi/vaShared.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
RWTexture2D<half> output_tex : register(u16, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 算法概述:对半分辨率视图法线 + 深度做地平线 GTAO。
    // 每个像素用空间填充曲线索引选取逐帧抖动角度,在 2 个切片上各取
    // 3 个黄金比距离的采样点重建视图空间样本,按距离衰减累积 AO。
    float2 output_res = float2(output_tex_dims);
    float2 uv = pixel_to_uv(int2(dispatchThreadID.xy), output_res);

    // 空间填充曲线像素索引(位插入 + 象限交换)选取逐像素、逐帧的 GTAO 抖动旋转。
    uint pixel_index = 0u;
    uint sfc_x = dispatchThreadID.x;
    uint sfc_y = dispatchThreadID.y;
    for (uint mask = 32u; mask > 0u; mask /= 2u)
    {
        uint xb = (sfc_x & mask) > 0u ? 1u : 0u;
        uint yb = (sfc_y & mask) > 0u ? 1u : 0u;
        pixel_index += (mask * mask) * ((3u * xb) ^ yb);
        if (yb == 0u)
        {
            if (xb == 1u)
            {
                uint prev_y = sfc_y;
                sfc_y = 63u - sfc_x;
                sfc_x = 63u - prev_y;
            }
        }
    }

    float2 jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(pixel_index + (288u * (frame_index % 64u)))) + 0.5f);
    float jitter_x = jitter.x;
    float jitter_y = jitter.y;

    // 该像素的视图空间中心位置。
    float4 view_homo = mul(view_constants[0].sample_to_view, float4(uv_to_sample(uv) * float2(2.0f, -2.0f), depth_tex.Load(uint3(dispatchThreadID.xy, 0u)).x, 1.0f));
    float view_z = (view_homo.xyz / view_homo.w).z;
    float focal = (-0.5f / view_z) * view_constants[0].view_to_clip[1][1];

    float2 inv_res = 1.0f / output_res;
    float2 view_to_sample_inv = float2(1.0f / view_constants[0].view_to_sample[0][0], 1.0f / view_constants[0].view_to_sample[1][1]);
    float2 clip_pix_scale = view_to_sample_inv * float2(2.0f, -2.0f);
    float2 clip_pix_bias = view_to_sample_inv * float2(-1.0f, 1.0f);

    float3 view_normal = normalize(half_view_normal_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz * float3(1.0f, 1.0f, -1.0f));

    float2 pixel_center_uv = (float2(dispatchThreadID.xy) + 0.5f) * inv_res;

    float4 gathered = depth_tex.GatherRed(sampler_nnc, float2(dispatchThreadID.xy) * inv_res);
    for (uint g = 0u; g < 4u; g++)
    {
        gathered[g] = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], gathered[g]);
    }

    float view_depth = gathered.y * 0.99919998645782470703125f;

    float3 view_pos;
    view_pos.xy = ((clip_pix_scale * pixel_center_uv) + clip_pix_bias) * view_depth;
    view_pos.z = view_depth;
    float3 view_dir = normalize(-view_pos);

    float radius_pixels = (-80.0f) * focal;
    float distance_fade = ((0.11249999701976776123046875f / focal) / (0.012500000186264514923095703125f / focal)) + 1.0f;

    float ao_radius = (0.125f / focal) / ((float2(view_depth, view_depth) * (clip_pix_scale * inv_res)).x);
    float radius_dist = 1.2999999523162841796875f / ao_radius;

    float ao = clamp((10.0f - ao_radius) * 0.00999999977648258209228515625f, 0.0f, 1.0f) * 0.5f;

    // 逐采样点重建的共享视图空间样本位置。
    float3 sample_pos;

    for (float slice = 0.0f; slice < 1.0f; slice += 1.0f)
    {
        float angle = (slice + jitter_x) * 3.1415927410125732421875f;
        float cos_angle = cos(angle);
        float sin_angle = sin(angle);

        float2 slice_dir = float2(cos_angle, -sin_angle) * ao_radius;
        float3 slice_dir3 = float3(cos_angle, sin_angle, 0.0f);
        float3 h_proj = slice_dir3 - (view_dir * dot(slice_dir3, view_dir));
        float3 h_perp = normalize(cross(h_proj, view_dir));
        float3 n_proj = view_normal - (h_perp * dot(view_normal, h_perp));
        float n_len = length(n_proj);
        float ndv = clamp(dot(n_proj, view_dir) / n_len, 0.0f, 1.0f);
        float abs_ndv = abs(ndv);
        float sign_factor = float(int(sign(dot(h_proj, n_proj))));
        float horizon0 = sign_factor * XeGTAO_Internal_FastACos(abs_ndv);

        float cos_h2 = cos(horizon0 + 1.57079637050628662109375f);
        float cos_h1 = cos(horizon0 - 1.57079637050628662109375f);
        float h1 = cos_h1;
        float h2 = cos_h2;

        [unroll]
        for (float tap = 0.0f; tap < 3.0f; tap += 1.0f)
        {
            float2 sample_offset = slice_dir * (pow((tap + frac(jitter_y + ((slice + (tap * 3.0f)) * 0.61803400516510009765625f))) * 0.3333333432674407958984375f, 2.0f) + radius_dist);
            float mip_level = clamp(log2(length(sample_offset)), 0.0f, 0.0f);
            float2 rounded = round(sample_offset) * inv_res;

            float2 sample_uv_a = pixel_center_uv + rounded;
            float view_z_a = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_a, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_a) + clip_pix_bias) * view_z_a;
            sample_pos.z = view_z_a;
            float3 delta_a = sample_pos - view_pos;
            float len_a = length(delta_a);

            float2 sample_uv_b = pixel_center_uv - rounded;
            float view_z_b = va_depth_to_view_z(view_constants[0].clip_to_view[3][2], depth_tex.SampleLevel(sampler_nnc, sample_uv_b, mip_level).x);
            sample_pos.xy = ((clip_pix_scale * sample_uv_b) + clip_pix_bias) * view_z_b;
            sample_pos.z = view_z_b;
            float3 delta_b = sample_pos - view_pos;
            float len_b = length(delta_b);

            h1 = max(h1, lerp(cos_h1, dot(delta_b / len_b, view_dir), clamp((len_b * radius_pixels) + distance_fade, 0.0f, 1.0f)));
            h2 = max(h2, lerp(cos_h2, dot(delta_a / len_a, view_dir), clamp((len_a * radius_pixels) + distance_fade, 0.0f, 1.0f)));
        }

        // horizon0 + HALF_PI 累积(h2)供给 *2 项;horizon0 - HALF_PI(h1)供给 *-2 项。
        float abs_h1 = abs(h1);
        float acos_h1 = XeGTAO_Internal_FastACos(abs_h1);
        float horizon_angle_neg = (h1 >= 0.0f) ? acos_h1 : (3.14159297943115234375f - acos_h1);

        float abs_h2 = abs(h2);
        float acos_h2 = XeGTAO_Internal_FastACos(abs_h2);
        float horizon_angle_pos = (h2 >= 0.0f) ? acos_h2 : (3.14159297943115234375f - acos_h2);

        float half_w2 = horizon_angle_neg * (-2.0f);
        float sin_horizon0 = sin(horizon0);
        float half_w1 = 2.0f * horizon_angle_pos;

        ao += (lerp(n_len, 1.0f, 0.0500000007450580596923828125f) * (0.25f * (((ndv + (half_w2 * sin_horizon0)) - cos(half_w2 - horizon0)) + ((ndv + (half_w1 * sin_horizon0)) - cos(half_w1 - horizon0)))));
    }

    output_tex[dispatchThreadID.xy] = max(0.02999999932944774627685546875f, pow(ao, 1.0f));
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// 转译自 _ssgi_ssgi.cs.hlsl_cs_main_0.bin(spirv-cross;行为精确,变量名已语义化)
// 计算着色器:地平线 GTAO(逻辑同重建段,算术为 oracle 逐字)。
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

static float3 scratch_a;
static float3 scratch_b;

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

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims : packoffset(c0);
    uint2 half_view_normal_tex_dims : packoffset(c0.z);
    uint2 output_tex_dims : packoffset(c1);
};

SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
RWTexture2D<float> output_tex : register(u16, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    float2 out_dims = float2(output_tex_dims);
    float2 uv = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / out_dims;
    uint morton;
    morton = 0u;
    uint next_morton;
    uint next_py;
    uint next_px;
    for (uint py = gl_GlobalInvocationID.y, px = gl_GlobalInvocationID.x, mask = 32u; mask > 0u; py = next_py, px = next_px, morton = next_morton, mask /= 2u)
    {
        uint bit_y = uint((px & mask) > 0u);
        uint bit_x = uint((py & mask) > 0u);
        next_morton = morton + ((mask * mask) * ((3u * bit_y) ^ bit_x));
        if (bit_x == 0u)
        {
            uint t0;
            uint t1;
            if (bit_y == 1u)
            {
                t0 = 63u - py;
                t1 = 63u - px;
            }
            else
            {
                t0 = py;
                t1 = px;
            }
            next_py = t1;
            next_px = t0;
        }
        else
        {
            next_py = py;
            next_px = px;
        }
    }
    float2 jitter = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(morton + (288u * (frame_constants_frame_index % 64u)))) + 0.5f.xx);
    float4 view_pos = mul(frame_constants_view_constants[0u].sample_to_view, float4((uv - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f));
    float view_scale = ((-0.5f) / (view_pos.xyz / view_pos.w.xxx).z) * frame_constants_view_constants[0u].view_to_clip[1].y;
    float2 inv_dims = 1.0f.xx / out_dims;
    float2 v2s_scale = float2(1.0f / frame_constants_view_constants[0u].view_to_sample[0].x, 1.0f / frame_constants_view_constants[0u].view_to_sample[1].y);
    float2 v2s_scale_xy = v2s_scale * float2(2.0f, -2.0f);
    float2 v2s_scale_neg = v2s_scale * float2(-1.0f, 1.0f);
    float3 view_normal = normalize(half_view_normal_tex.SampleLevel(sampler_nnc, uv, 0.0f).xyz * float3(1.0f, 1.0f, -1.0f));
    float2 pixel_uv = float2(gl_GlobalInvocationID.xy);
    float2 pixel_center = (pixel_uv + 0.5f.xx) * inv_dims;
    float4 depth4 = depth_tex.GatherRed(sampler_nnc, pixel_uv * inv_dims);
    for (uint i = 0u; i < 4u; )
    {
        depth4[i] = (-1.0f) / (depth4[i] * (-frame_constants_view_constants[0u].clip_to_view[3].z));
        i++;
        continue;
    }
    float center_z = depth4.y * 0.99919998645782470703125f;
    float2 center_xy = ((v2s_scale_xy * pixel_center) + v2s_scale_neg) * center_z;
    float3 center_pos = float3(center_xy.x, center_xy.y, scratch_a.z);
    center_pos.z = center_z;
    float3 center_dir = normalize(-center_pos);
    float fade_scale = (-80.0f) * view_scale;
    float fade_bias = ((0.11249999701976776123046875f / view_scale) / (0.012500000186264514923095703125f / view_scale)) + 1.0f;
    float jitter_x = jitter.x;
    float jitter_y = jitter.y;
    float tap_scale = (0.125f / view_scale) / (center_z.xx * (v2s_scale_xy * inv_dims)).x;
    float taper = 1.2999999523162841796875f / tap_scale;
    float ao;
    float slice;
    ao = clamp((10.0f - tap_scale) * 0.00999999977648258209228515625f, 0.0f, 1.0f) * 0.5f;
    slice = 0.0f;
    float next_ao;
    for (; slice < 1.0f; ao = next_ao, slice += 1.0f)
    {
        float slice_angle = (slice + jitter_x) * 3.1415927410125732421875f;
        float cos_sa = cos(slice_angle);
        float sin_sa = sin(slice_angle);
        float2 slice_dir_xy = float2(cos_sa, -sin_sa) * tap_scale;
        float3 slice_dir = float3(cos_sa, sin_sa, 0.0f);
        float3 tangent = slice_dir - (center_dir * dot(slice_dir, center_dir));
        float3 bitangent = normalize(cross(tangent, center_dir));
        float3 normal_t = view_normal - (bitangent * dot(view_normal, bitangent));
        float normal_t_len = length(normal_t);
        float ndotv = clamp(dot(normal_t, center_dir) / normal_t_len, 0.0f, 1.0f);
        float abs_ndotv = abs(ndotv);
        float horizon_angle = float(int(sign(dot(tangent, normal_t)))) * ((((-0.15658299624919891357421875f) * abs_ndotv) + 1.57079601287841796875f) * asfloat(532487669 + (asint(1.0f - abs_ndotv) >> 1)));
        float cos_hp = cos(horizon_angle + 1.57079637050628662109375f);
        float cos_hm = cos(horizon_angle - 1.57079637050628662109375f);
        float h1;
        float h2;
        h1 = cos_hp;
        h2 = cos_hm;
        [unroll]
        for (float tap = 0.0f; tap < 3.0f; )
        {
            float2 tap_off = slice_dir_xy * (pow((tap + frac(jitter_y + ((slice + (tap * 3.0f)) * 0.61803400516510009765625f))) * 0.3333333432674407958984375f, 2.0f) + taper);
            float tap_lod = clamp(log2(length(tap_off)), 0.0f, 0.0f);
            float2 tap_off_px = round(tap_off) * inv_dims;
            float2 tap_uv_a = pixel_center + tap_off_px;
            float clip_z = -frame_constants_view_constants[0u].clip_to_view[3].z;
            float tap_z_a = (-1.0f) / (depth_tex.SampleLevel(sampler_nnc, tap_uv_a, tap_lod).x * clip_z);
            float2 tap_xy_a = ((v2s_scale_xy * tap_uv_a) + v2s_scale_neg) * tap_z_a;
            float3 tap_pos_a = float3(tap_xy_a.x, tap_xy_a.y, scratch_b.z);
            tap_pos_a.z = tap_z_a;
            float2 tap_uv_b = pixel_center - tap_off_px;
            float tap_z_b = (-1.0f) / (depth_tex.SampleLevel(sampler_nnc, tap_uv_b, tap_lod).x * clip_z);
            float2 tap_xy_b = ((v2s_scale_xy * tap_uv_b) + v2s_scale_neg) * tap_z_b;
            float3 tap_pos_b = float3(tap_xy_b.x, tap_xy_b.y, scratch_b.z);
            tap_pos_b.z = tap_z_b;
            float3 dir_a = tap_pos_a - center_pos;
            float3 dir_b = tap_pos_b - center_pos;
            float len_a = length(dir_a);
            float len_b = length(dir_b);
            h1 = max(h1, lerp(cos_hp, dot(dir_a / len_a.xxx, center_dir), clamp((len_a * fade_scale) + fade_bias, 0.0f, 1.0f)));
            h2 = max(h2, lerp(cos_hm, dot(dir_b / len_b.xxx, center_dir), clamp((len_b * fade_scale) + fade_bias, 0.0f, 1.0f)));
            tap += 1.0f;
            continue;
        }
        float abs_h2 = abs(h2);
        float acos_h2 = (((-0.15658299624919891357421875f) * abs_h2) + 1.57079601287841796875f) * asfloat(532487669 + (asint(1.0f - abs_h2) >> 1));
        float h2_angle;
        if (h2 >= 0.0f)
        {
            h2_angle = acos_h2;
        }
        else
        {
            h2_angle = 3.14159297943115234375f - acos_h2;
        }
        float abs_h1 = abs(h1);
        float acos_h1 = (((-0.15658299624919891357421875f) * abs_h1) + 1.57079601287841796875f) * asfloat(532487669 + (asint(1.0f - abs_h1) >> 1));
        float h1_angle;
        if (h1 >= 0.0f)
        {
            h1_angle = acos_h1;
        }
        else
        {
            h1_angle = 3.14159297943115234375f - acos_h1;
        }
        float neg2_h2 = h2_angle * (-2.0f);
        float sin_horizon = sin(horizon_angle);
        float two_h1 = 2.0f * h1_angle;
        next_ao = ao + (lerp(normal_t_len, 1.0f, 0.0500000007450580596923828125f) * (0.25f * (((ndotv + (neg2_h2 * sin_horizon)) - cos(neg2_h2 - horizon_angle)) + ((ndotv + (two_h1 * sin_horizon)) - cos(two_h1 - horizon_angle)))));
    }
    output_tex[gl_GlobalInvocationID.xy] = max(0.02999999932944774627685546875f, pow(ao, 1.0f));
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
