// PATCHED: modules aa.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/taa.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (target Catmull-Rom w1 = (1 + f²(-2.5+1.5f)) + w2, center_uv = (base_c + w2/w1); B uses standard w1).
// ============================================================
// 时序抗锯齿(TAA)compute。
//   入口 cs_main(numthreads 8x8x1):
//     1. 用重投影图把当前帧中心重投影到历史帧,做 5 采样 bicubic 十字滤波(Catmull-Rom 权重);
//     2. 在 YCoCg 空间做邻域钳制(9-tap 统计均值/标准差,wide/narrow 双高斯加权);
//     3. 当前帧与钳制后的历史按 blend 混合(无历史时直接用当前帧);
//     4. 结果做 curve 量化后写回 history_output_tex(u17);
//     5. 可选的 tile 亮度锐化(8x8 groupshared 相邻亮度加权),写入 output_tex(u18)。
//   绑定: color_tex(t14)、history_tex(t15)、reprojection_tex(t16)、
//         history_output_tex(u17)、output_tex(u18)、$Globals(b13)。
//   变体: TGV_0(compute)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/ycbcr.hlsl"
#include "inc/image.hlsl"
#include "inc/curve.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/unjitter_taa.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/history_valid.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims;
    uint2 history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 history_output_tex_dims;
    uint2 output_tex_dims;
    uint2 temp_debug_tex_dims;
    uint bypass_neighborhood_clamping;
    float sharpen_amount;
    uint history_valid;
};

Texture2D<float4> color_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> history_output_tex : register(u17, space0);
[[vk::ext_capability(49)]] RWTexture2D<float3> output_tex : register(u18, space0);

groupshared float tile_luminance[8][8];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint3 local_id : SV_GroupThreadID)
{
    // 像素中心 + 重投影偏移 → 历史帧像素坐标(亚像素)。
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(dispatch_id.xy)) + 0.5f;
    float2 reproj_px = ((center / out_dims)
        + reprojection_tex.Load(int3(uint2((float2(dispatch_id.xy) + 0.5f)
            * float2(color_tex_dims / output_tex_dims)), 0)).xy) * out_dims;

    // 5-tap bicubic-cross 历史滤波(Catmull-Rom 风格权重)。
    float2 base = floor(reproj_px - 0.5f);
    float2 base_c = base + 0.5f;
    float2 frac = reproj_px - base_c;
    float2 h2 = frac * 0.5f;
    float2 w0 = frac * ((-0.5f) + (frac * (1.0f - h2)));
    float2 frac2 = frac * frac;
    float2 frac15 = frac * 1.5f;
    float2 w2 = frac * (0.5f + (frac * (2.0f - frac15)));
    float2 w3 = frac2 * ((-0.5f) + h2);
    float2 w1 = (1.0f + (frac2 * ((-2.5f) + frac15))) + w2;
    float2 left_uv = (base + (-0.5f)) / out_dims;
    float2 right_uv = (base + 2.5f) / out_dims;
    float2 center_uv = (base_c + (w2 / w1)) / out_dims;

    // 5 个十字方向的历史样本按权重求和(中心权重修正),归一化得历史颜色。
    float3 hist = (
        (history_tex.SampleLevel(sampler_default, float2(center_uv.x, left_uv.y), 0.0f).xyz * w1.x * w0.y)
        + (history_tex.SampleLevel(sampler_default, float2(left_uv.x, center_uv.y), 0.0f).xyz * w0.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, center_uv, 0.0f).xyz * w1.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(right_uv.x, center_uv.y), 0.0f).xyz * w3.x * w1.y)
        + (history_tex.SampleLevel(sampler_default, float2(center_uv.x, right_uv.y), 0.0f).xyz * w1.x * w3.y)
    ) / ((((w1.x * w0.y) + (w0.x * w1.y)) + (w1.x * w1.y)) + ((w3.x * w1.y) + (w1.x * w3.y)));

    float3 hist_ycbcr = rgb_to_ycbcr(hist);

    // 当前帧邻域统计(3x3):用 wide/narrow 两个高斯核加权,求均值与方差用于钳制。
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = (float2(current_px) + 0.5f
        + (view_constants[0].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float sigma_wide = ((-10.0f) * scale.x) * min(4.0f * scale.x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale.x * scale.x);

    float3 sum_sq = float3(0.0f, 0.0f, 0.0f);
    float sum_w1 = 0.0f;
    float3 sum_c = float3(0.0f, 0.0f, 0.0f);
    float sum_w2 = 0.0f;
    float4 sum_c2 = float4(0.0f, 0.0f, 0.0f, 0.0f);
    [loop]
    for (int dy = -1; dy <= 1; ++dy)
    {
        [loop]
        for (int dx = -1; dx <= 1; ++dx)
        {
            float4 c = float4(rgb_to_ycbcr(
                color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0)).xyz), 1.0f);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            sum_w1 += w_narrow;
            sum_sq += (c.xyz * c.xyz) * w_narrow;
            sum_c += c.xyz * w_narrow;
            sum_w2 += w_wide;
            sum_c2 += c * w_wide;
        }
    }

    // 邻域均值/方差 → 钳制范围(标准差按相对亮度自适应缩放)。
    float3 mean = sum_c / max(9.9999997473787516e-06f, sum_w1);
    float3 clamp_center = sum_c2.xyz / max(9.9999997473787516e-06f, sum_w2);
    float3 std = sqrt(max(float3(0.0f, 0.0f, 0.0f), (sum_sq / sum_w1) - (mean * mean)));
    float3 clamp_range = (std * lerp(0.5f, 1.0f,
        smoothstep(-0.1f, 0.3f, std.x / (mean.x + 9.9999997473787516e-06f)))) * 1.0f;
    float3 clamp_min = mean - clamp_range;
    float3 clamp_max = mean + clamp_range;

    // 历史颜色钳制:根据历史与钳制中心的接近程度决定混合比例 blend。
    float blend;
    float3 clamped_ycbcr;
    if (bypass_neighborhood_clamping != 0u)
    {
        blend = 0.125f;
        clamped_ycbcr = hist_ycbcr;
    }
    else
    {
        blend = 0.125f * lerp(0.2f, 1.0f, smoothstep(0.0f, 2.0f,
            min(abs(hist_ycbcr.x - clamp_min.x), abs(hist_ycbcr.x - clamp_max.x))
            / max(max(hist_ycbcr.x, mean.x), 9.9999997473787516e-06f)));
        clamped_ycbcr = clamp(hist_ycbcr, clamp_min, clamp_max);
    }

    // 无历史时直接用当前帧(钳制中心),否则用混合结果。
    bool history_available = (history_valid != 0u) && (frame_index != 0u);
    float3 final_color;
    if (!history_available)
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(clamp_center));
    }
    else
    {
        final_color = max(float3(0.0f, 0.0f, 0.0f), ycbcr_to_rgb(lerp(clamped_ycbcr, clamp_center, blend)));
    }

    // 历史写入:curve 量化(经 frexp 指数偏置)存下动态范围。
    history_output_tex[dispatch_id.xy] = curve(final_color);

    // 可选锐化:8x8 tile 亮度,与上下左右邻居做加权锐化。
    float3 out_color;
    if (sharpen_amount > 0.0f)
    {
        float lum = dot(final_color, float3(0.2126f, 0.7152f, 0.0722f));
        tile_luminance[local_id.x][local_id.y] = lum;
        GroupMemoryBarrierWithGroupSync();

        uint2 right = local_id.xy + uint2(1u, 0u);
        uint2 left = local_id.xy - uint2(1u, 0u);
        float x_accum = 0.0f;
        float x_weight = 0.0f;
        if (all(bool4(right.x < 8u, right.y < 8u, left.x < 8u, left.y < 8u)))
        {
            // 邻居亮度与中心越接近权重越高;再受 sharpen_amount 缩放。
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[right.x][right.y]) + abs(lum - tile_luminance[left.x][left.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            x_accum = w2 * (tile_luminance[right.x][right.y] + tile_luminance[left.x][left.y]);
            x_weight = w2 * 2.0f;
        }

        uint2 up = local_id.xy + uint2(0u, 1u);
        uint2 down = local_id.xy - uint2(0u, 1u);
        float y_accum = 0.0f;
        float y_weight = 0.0f;
        if (all(bool4(up.x < 8u, up.y < 8u, down.x < 8u, down.y < 8u)))
        {
            float w = max(0.0f, 1.0f - (6.0f
                * (abs(lum - tile_luminance[up.x][up.y]) + abs(lum - tile_luminance[down.x][down.y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            y_accum = (x_accum + (tile_luminance[up.x][up.y] * w2)) + (tile_luminance[down.x][down.y] * w2);
            y_weight = x_weight + (w2 * 2.0f);
        }
        else
        {
            y_accum = x_accum;
            y_weight = x_weight;
        }

        // 用邻域加权和恢复中心亮度,得到锐化后的颜色。
        out_color = final_color * max(0.0f, max(0.0f,
            (lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516e-06f, lum));
    }
    else
    {
        out_color = final_color;
    }

    output_tex[dispatch_id.xy] = out_color;
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _taa.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
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

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
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

cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims : packoffset(c0);
    uint2 history_tex_dims : packoffset(c0.z);
    uint2 reprojection_tex_dims : packoffset(c1);
    uint2 history_output_tex_dims : packoffset(c1.z);
    uint2 output_tex_dims : packoffset(c2);
    uint2 temp_debug_tex_dims : packoffset(c2.z);
    uint bypass_neighborhood_clamping : packoffset(c3);
    float sharpen_amount : packoffset(c3.y);
    uint history_valid : packoffset(c3.z);
};

SamplerState sampler_default : register(s9, space0);
Texture2D<float4> color_tex : register(t14, space0);
Texture2D<float4> history_tex : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
RWTexture2D<float3> history_output_tex : register(u17, space0);
RWTexture2D<float3> output_tex : register(u18, space0);

static uint3 gl_LocalInvocationID;
static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

groupshared float tile_luminance[8][8];

void comp_main()
{
    // 像素中心 + 重投影偏移 → 历史帧亚像素坐标。
    float2 out_dims = float2(output_tex_dims);
    float2 center = float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx;
    float2 reproj_px = ((center / out_dims) + reprojection_tex.Load(int3(uint2((float2(gl_GlobalInvocationID.xy) + 0.5f.xx) * float2(color_tex_dims / output_tex_dims)), 0u)).xy) * out_dims;
    // Catmull-Rom 风格 bicubic 十字权重。
    float2 base = floor(reproj_px - 0.5f.xx);
    float2 base_c = base + 0.5f.xx;
    float2 frac = reproj_px - base_c;
    float2 h2 = frac * 0.5f;
    float2 w0 = frac * ((-0.5f).xx + (frac * (1.0f.xx - h2)));
    float2 frac2 = frac * frac;
    float2 frac15 = frac * 1.5f;
    float2 w2 = frac * (0.5f.xx + (frac * (2.0f.xx - frac15)));
    float2 w3 = frac2 * ((-0.5f).xx + h2);
    float2 w1 = (1.0f.xx + (frac2 * ((-2.5f).xx + frac15))) + w2;
    float2 left_uv = (base + (-0.5f).xx) / out_dims;
    float2 right_uv = (base + 2.5f.xx) / out_dims;
    float2 center_uv = (base_c + (w2 / w1)) / out_dims;
    float center_uv_x = center_uv.x;
    float w1_x = w1.x;
    float w0_y = w0.y;
    float center_uv_y = center_uv.y;
    float w0_x = w0.x;
    float w1_y = w1.y;
    float w3_x = w3.x;
    float w3_y = w3.y;
    // 5 采样十字方向历史滤波。
    float3 hist = (((((((float4(history_tex.SampleLevel(sampler_default, float2(center_uv_x, left_uv.y), 0.0f).xyz, 1.0f) * w1_x) * w0_y) + ((float4(history_tex.SampleLevel(sampler_default, float2(left_uv.x, center_uv_y), 0.0f).xyz, 1.0f) * w0_x) * w1_y)) + ((float4(history_tex.SampleLevel(sampler_default, center_uv, 0.0f).xyz, 1.0f) * w1_x) * w1_y)) + ((float4(history_tex.SampleLevel(sampler_default, float2(right_uv.x, center_uv_y), 0.0f).xyz, 1.0f) * w3_x) * w1_y)) + ((float4(history_tex.SampleLevel(sampler_default, float2(center_uv_x, right_uv.y), 0.0f).xyz, 1.0f) * w1_x) * w3_y)) / (((((w1_x * w0_y) + (w0_x * w1_y)) + (w1_x * w1_y)) + (w3_x * w1_y)) + (w1_x * w3_y)).xxxx).xyz;
    // 历史颜色转 YCoCg。
    float3 hist_ycbcr = mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), hist);
    // 当前帧 3x3 邻域统计(wide/narrow 双高斯加权)。
    float2 scale = float2(int2(color_tex_dims)) / out_dims;
    int2 current_px = int2(center * scale);
    float2 current_pos = ((float2(current_px) + 0.5f.xx) + (frame_constants_view_constants[0u].sample_offset_pixels * float2(1.0f, -1.0f))) / scale;
    float scale_x = scale.x;
    float sigma_wide = ((-10.0f) * scale_x) * min(4.0f * scale_x, 1.0f);
    float sigma_narrow = (-1.0f) * (scale_x * scale_x);
    float3 sum_sq;
    float sum_w1;
    float3 sum_c;
    float sum_w2;
    float4 sum_c2;
    int dy;
    sum_sq = 0.0f.xxx;
    sum_w1 = 0.0f;
    sum_c = 0.0f.xxx;
    sum_w2 = 0.0f;
    sum_c2 = 0.0f.xxxx;
    dy = -1;
    float3 sum_sq_next;
    float sum_w1_next;
    float3 sum_c_next;
    float sum_w2_next;
    float4 sum_c2_next;
    for (; dy <= 1; sum_sq = sum_sq_next, sum_w1 = sum_w1_next, sum_c = sum_c_next, sum_w2 = sum_w2_next, sum_c2 = sum_c2_next, dy++)
    {
        sum_w1_next = sum_w1;
        sum_sq_next = sum_sq;
        sum_c_next = sum_c;
        sum_w2_next = sum_w2;
        sum_c2_next = sum_c2;
        for (int dx = -1; dx <= 1; )
        {
            float4 c = float4(mul(float3x3(float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f), float3(-0.114600002765655517578125f, -0.385399997234344482421875f, 0.5f), float3(0.5f, -0.4541999995708465576171875f, -0.0458000004291534423828125f)), color_tex.Load(int3(uint2(current_px + int2(dx, dy)), 0u)).xyz), 1.0f);
            float2 d = (current_pos + (float2(float(dx), float(dy)) / scale)) - center;
            float dist = dot(d, d);
            float w_narrow = exp2(dist * sigma_narrow);
            float w_wide = exp2(dist * sigma_wide);
            float3 c_xyz = c.xyz;
            sum_w1_next += w_narrow;
            sum_sq_next += ((c_xyz * c_xyz) * w_narrow);
            sum_c_next += (c_xyz * w_narrow);
            sum_w2_next += w_wide;
            sum_c2_next += (c * w_wide);
            dx++;
            continue;
        }
    }
    // 邻域均值/方差 → 钳制中心与范围。
    float3 w1_xxx = sum_w1.xxx;
    float3 mean = sum_c / w1_xxx;
    float3 clamp_center = sum_c2.xyz / max(9.9999997473787516355514526367188e-06f, sum_w2).xxx;
    float3 std = sqrt(max(0.0f.xxx, (sum_sq / w1_xxx) - (mean * mean)));
    float mean_x = mean.x;
    float3 clamp_range = (std * lerp(0.5f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.300000011920928955078125f, std.x / (mean_x + 9.9999997473787516355514526367188e-06f)))) * 1.0f;
    float3 clamp_min = mean - clamp_range;
    float3 clamp_max = mean + clamp_range;
    // 历史钳制与混合比例。
    float blend;
    float3 clamped_ycbcr;
    if (!(bypass_neighborhood_clamping != 0u))
    {
        float hist_luma = hist_ycbcr.x;
        blend = 0.125f * lerp(0.20000000298023223876953125f, 1.0f, smoothstep(0.0f, 2.0f, min(abs(hist_luma - clamp_min.x), abs(hist_luma - clamp_max.x)) / max(max(hist_luma, mean_x), 9.9999997473787516355514526367188e-06f)));
        clamped_ycbcr = clamp(hist_ycbcr, clamp_min, clamp_max);
    }
    else
    {
        blend = 0.125f;
        clamped_ycbcr = hist_ycbcr;
    }
    // 历史可用性判断。
    bool history_available;
    if (history_valid != 0u)
    {
        history_available = !(0u == frame_constants_frame_index);
    }
    else
    {
        history_available = false;
    }
    // 最终颜色:无历史用当前帧钳制中心,否则混合。
    float3 final_color;
    if (!history_available)
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), clamp_center));
    }
    else
    {
        final_color = max(0.0f.xxx, mul(float3x3(float3(1.0f, 0.0f, 1.574800014495849609375f), float3(1.0f, -0.1872999966144561767578125f, -0.468100011348724365234375f), float3(1.0f, 1.85559999942779541015625f, 0.0f)), lerp(clamped_ycbcr, clamp_center, blend.xxx)));
    }
    // curve 量化:用 frexp 取指数并做指数偏置,写回历史纹理。
    FrexpStructType curve_data;
    { int3 fe; curve_data.mantissa = frexp(final_color, fe); curve_data.exponent = fe; }
    history_output_tex[gl_GlobalInvocationID.xy] = final_color + exp2(float3(curve_data.exponent) - float3(8.0f, 8.0f, 7.0f));
    // 可选锐化。
    float3 out_color;
    if (sharpen_amount > 0.0f)
    {
        float lum = dot(final_color, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
        tile_luminance[gl_LocalInvocationID.x][gl_LocalInvocationID.y] = lum;
        GroupMemoryBarrierWithGroupSync();
        // 水平邻居亮度加权。
        uint2 right = gl_LocalInvocationID.xy + uint2(1u, 0u);
        uint2 left = gl_LocalInvocationID.xy - uint2(1u, 0u);
        uint right_x = right.x;
        uint right_y = right.y;
        uint left_x = left.x;
        uint left_y = left.y;
        uint4 right_left = uint4(right_x, right_y, left_x, left_y);
        float x_accum;
        float x_weight;
        if (all(bool4(right_left.x < uint4(8u, 8u, 8u, 8u).x, right_left.y < uint4(8u, 8u, 8u, 8u).y, right_left.z < uint4(8u, 8u, 8u, 8u).z, right_left.w < uint4(8u, 8u, 8u, 8u).w)))
        {
            float w = max(0.0f, 1.0f - (6.0f * (abs(lum - tile_luminance[right_x][right_y]) + abs(lum - tile_luminance[left_x][left_y]))));
            float w2 = min(w, (sharpen_amount * w) * 1.25f);
            x_accum = w2 * (tile_luminance[right_x][right_y] + tile_luminance[left_x][left_y]);
            x_weight = w2 * 2.0f;
        }
        else
        {
            x_accum = 0.0f;
            x_weight = 0.0f;
        }
        // 垂直邻居亮度加权(累加水平结果)。
        uint2 up = gl_LocalInvocationID.xy + uint2(0u, 1u);
        uint2 down = gl_LocalInvocationID.xy - uint2(0u, 1u);
        uint up_x = up.x;
        uint up_y = up.y;
        uint down_x = down.x;
        uint down_y = down.y;
        uint4 up_down = uint4(up_x, up_y, down_x, down_y);
        float y_accum;
        float y_weight;
        if (all(bool4(up_down.x < uint4(8u, 8u, 8u, 8u).x, up_down.y < uint4(8u, 8u, 8u, 8u).y, up_down.z < uint4(8u, 8u, 8u, 8u).z, up_down.w < uint4(8u, 8u, 8u, 8u).w)))
        {
            float y_w = max(0.0f, 1.0f - (6.0f * (abs(lum - tile_luminance[up_x][up_y]) + abs(lum - tile_luminance[down_x][down_y]))));
            float y_w2 = min(y_w, (sharpen_amount * y_w) * 1.25f);
            y_accum = (x_accum + (tile_luminance[up_x][up_y] * y_w2)) + (tile_luminance[down_x][down_y] * y_w2);
            y_weight = x_weight + (y_w2 * 2.0f);
        }
        else
        {
            y_accum = x_accum;
            y_weight = x_weight;
        }
        // 由邻域加权和恢复中心亮度,得到锐化颜色。
        out_color = final_color * max(0.0f, max(0.0f, (lum * (y_weight + 1.0f)) - y_accum) / max(9.9999997473787516355514526367188e-06f, lum));
    }
    else
    {
        out_color = final_color;
    }
    output_tex[gl_GlobalInvocationID.xy] = out_color;
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
