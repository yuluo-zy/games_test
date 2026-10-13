// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/star.raster.hlsl | F-E | bins: _star.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// 星空着色器:实例化程序化广告牌散布在相机周围的大球壳上;
// 像素阶段采样星点纹理,并由视线方向与昼夜时间窗口做门控。
//   顶点阶段 vs_main:
//     - 用实例 id 派生种子,经多级哈希生成每颗星的朝向角、距离、大小;
//     - 生成竖直广告牌四边形,>6000 的实例钉到 NaN 以剔除;
//     - 输出 UV、实例 id、世界位置、right 轴与朝向。
//   像素阶段 ps_main:
//     - 夜间/黄昏两个时间窗口计算可见强度;
//     - 视线与"前向"夹角做遮挡,叠加星点纹理后输出。
//   绑定: $Globals(b13)、star_texture(t14, sampler_lnc s0)。
//   变体: TGV_b903e43ffb3da915(顶点+像素)、TGV_f0adff76827e73ff(顶点,像素为空占位)。
// ============================================================

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 star_texture_dims;
};

Texture2D<float4> star_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    // 四边形顶点偏移:2x1 网格,中心对齐,y 方向拉长到 2 倍。
    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    // 用实例 id 派生三级种子哈希,生成朝向、距离与大小的随机性。
    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));

    // 朝向角:黄金角散布 + 随时间极缓慢漂移(时间影响程度随 h2 随机)。
    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 1e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    // 相机前方扇形方向的朝向,中心沿 -facing 放在远近变化的球壳上。
    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.0001666667f;
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + 10000.0f * (rand01(h1) * rand01(h1))) - 1000.0f;

    // 大小:大部分星较小,少数大星,再乘 h3 系随机因子。
    float2 size = (float2(2.0f, 1.0f) * (lerp(1.0f, 0.2f, smoothstep(0.7f, 1.0f, rand01(h1))) * 10.0f)) * 3.0f
                * rand01(hash(h3));
    // 竖直方向上的水平 right 轴(朝向的 90 度旋转)。
    float3 right = float3(facing.z, 0.0f, -facing.x);

    float3 world = center + (right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y));

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 超过 6000 的实例输出 NaN,整点被硬件裁剪掉。
    if (instance_id > 6000u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    // 相机世界位置与指向该星的视线方向。
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - vin.world_pos);

    // 夜间/黄昏强度窗口:围绕特定时刻的平滑钟形脉冲。
    float d_night = abs(time_of_day - 0.9f);
    float night = (d_night > 0.1f) ? 0.0f : 1.0f - (d_night * 10.0f) * (d_night * 10.0f) * (3.0f - d_night * 20.0f);
    float d_dusk = abs(time_of_day - 0.45f);
    float dusk = (d_dusk > 0.15f) ? 0.0f : 1.0f - (d_dusk * 6.6667f) * (d_dusk * 6.6667f) * (3.0f - d_dusk * 13.333f);

    // 视线偏离相机前向越远,星被地平线/遮挡越多(occlusion)。
    float occlusion = smoothstep(-1.0f, 1.0f, dot(view_dir, -mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz));
    // 可见时间段:日落后到黎明前。
    float visible = smoothstep(0.4f, 0.6f, time_of_day) - smoothstep(0.8f, 0.92f, time_of_day);

    float star = star_texture.Sample(sampler_lnc, vin.uv).x * (1.0f - occlusion * (night + dusk)) * visible;
    return float4(star.xxx * (10.0f * ev_mult), star);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _star.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 四边形顶点偏移(2x1 网格,中心对齐)。
    float quad_x = float(uint(gl_VertexIndex) & 1u);
    uint quad_y = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 quad = (float2(quad_x, float(quad_y)) - 0.5f.xx) * float2(1.0f, 2.0f);
    // 实例种子与三级哈希链(h1/h2/h3)。
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint hash1_a = seed + (seed << 10u);
    uint hash1_b = hash1_a ^ (hash1_a >> 6u);
    uint hash1_c = hash1_b + (hash1_b << 3u);
    uint hash1_d = hash1_c ^ (hash1_c >> 11u);
    uint h1 = hash1_d + (hash1_d << 15u);
    float fi = float(uint(gl_InstanceIndex));
    uint hash2_a = h1 + (h1 << 10u);
    uint hash2_b = hash2_a ^ (hash2_a >> 6u);
    uint hash2_c = hash2_b + (hash2_b << 3u);
    uint hash2_d = hash2_c ^ (hash2_c >> 11u);
    uint h2 = hash2_d + (hash2_d << 15u);
    // h1 低 23 位作为 [0,1) 随机数。
    float rand01_h1 = asfloat((h1 & 8388607u) | 1065353216u) - 1.0f;
    uint hash3_a = h2 + (h2 << 10u);
    uint hash3_b = hash3_a ^ (hash3_a >> 6u);
    uint hash3_c = hash3_b + (hash3_b << 3u);
    uint hash3_d = hash3_c ^ (hash3_c >> 11u);
    uint h3 = hash3_d + (hash3_d << 15u);
    uint hash4_a = h3 + (h3 << 10u);
    uint hash4_b = hash4_a ^ (hash4_a >> 6u);
    uint hash4_c = hash4_b + (hash4_b << 3u);
    uint h4 = hash4_c ^ (hash4_c >> 11u);
    // 朝向角:黄金角散布 + 时间极缓慢漂移(漂移率随 h2 随机)。
    float angle = (frac(0.300000011920928955078125f + (fi * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((h2 & 8388607u) | 1065353216u) - 1.0f));
    float2 facing2d = -float2(cos(angle), sin(angle));
    float facing_z = facing2d.y;
    float facing_x = facing2d.x;
    float3 facing = float3(facing_x, 0.0f, facing_z);
    // 距离随实例序号递增。
    float t = 1.0f - (fi * 0.00016666666488163173198699951171875f);
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + (10000.0f * (rand01_h1 * rand01_h1))) - 1000.0f;
    // 星点大小:大部分小、少数大。
    float2 size = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, rand01_h1)) * 10.0f)) * 3.0f) * (asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 right = float3(facing_z, 0.0f, -facing_x);
    // 广告牌顶点世界位置。
    float3 world = center + ((right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y)));
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 超过 6000 的实例输出 NaN(硬件裁剪掉)。
    bool4 is_far = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(is_far.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, is_far.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, is_far.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, is_far.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = float2(quad_x, float(1u - quad_y));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = world;
    out_var_C3 = right;
    out_var_C4 = facing;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _star.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 star_texture_dims : packoffset(c1);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> star_texture : register(t14, space0);

static float2 in_var_C0;
static float3 in_var_C2;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    // 相机世界位置(w 分量还原)。
    float4 camera_world4 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 夜间强度窗口:围绕 0.9 时刻的平滑钟形。
    float night;
    do
    {
        float d_night = abs(frame_constants_time_of_day - 0.89999997615814208984375f);
        if (d_night > 0.100000001490116119384765625f)
        {
            night = 0.0f;
            break;
        }
        float night_scale = d_night * 10.0f;
        night = 1.0f - ((night_scale * night_scale) * (3.0f - (d_night * 20.0f)));
        break;
    } while(false);
    // 黄昏强度窗口:围绕 0.45 时刻的平滑钟形。
    float dusk;
    do
    {
        float d_dusk = abs(frame_constants_time_of_day - 0.449999988079071044921875f);
        if (d_dusk > 0.1500000059604644775390625f)
        {
            dusk = 0.0f;
            break;
        }
        float dusk_scale = d_dusk * 6.666666507720947265625f;
        dusk = 1.0f - ((dusk_scale * dusk_scale) * (3.0f - (d_dusk * 13.33333301544189453125f)));
        break;
    } while(false);
    // 星点纹理 × 视线遮挡 × 时间可见窗口。
    float4 star4 = 1.0f.xxxx * ((star_texture.Sample(sampler_lnc, in_var_C0).x * (1.0f - (smoothstep(-1.0f, 1.0f, dot(normalize((camera_world4.xyz / camera_world4.w.xxx) - in_var_C2), -mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz)) * (night + dusk)))) * (smoothstep(0.4000000059604644775390625f, 0.60000002384185791015625f, frame_constants_time_of_day) - smoothstep(0.800000011920928955078125f, 0.920000016689300537109375f, frame_constants_time_of_day)));
    // RGB 乘 EV 倍率,alpha 原样输出。
    float3 star_rgb = star4.xyz * (10.0f * frame_constants_ev_mult);
    out_var_SV_TARGET = float4(star_rgb.x, star_rgb.y, star_rgb.z, star4.w);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _star.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 四边形顶点偏移(2x1 网格,中心对齐)。
    float quad_x = float(uint(gl_VertexIndex) & 1u);
    uint quad_y = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 quad = (float2(quad_x, float(quad_y)) - 0.5f.xx) * float2(1.0f, 2.0f);
    // 实例种子与三级哈希链(h1/h2/h3)。
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint hash1_a = seed + (seed << 10u);
    uint hash1_b = hash1_a ^ (hash1_a >> 6u);
    uint hash1_c = hash1_b + (hash1_b << 3u);
    uint hash1_d = hash1_c ^ (hash1_c >> 11u);
    uint h1 = hash1_d + (hash1_d << 15u);
    float fi = float(uint(gl_InstanceIndex));
    uint hash2_a = h1 + (h1 << 10u);
    uint hash2_b = hash2_a ^ (hash2_a >> 6u);
    uint hash2_c = hash2_b + (hash2_b << 3u);
    uint hash2_d = hash2_c ^ (hash2_c >> 11u);
    uint h2 = hash2_d + (hash2_d << 15u);
    // h1 低 23 位作为 [0,1) 随机数。
    float rand01_h1 = asfloat((h1 & 8388607u) | 1065353216u) - 1.0f;
    uint hash3_a = h2 + (h2 << 10u);
    uint hash3_b = hash3_a ^ (hash3_a >> 6u);
    uint hash3_c = hash3_b + (hash3_b << 3u);
    uint hash3_d = hash3_c ^ (hash3_c >> 11u);
    uint h3 = hash3_d + (hash3_d << 15u);
    uint hash4_a = h3 + (h3 << 10u);
    uint hash4_b = hash4_a ^ (hash4_a >> 6u);
    uint hash4_c = hash4_b + (hash4_b << 3u);
    uint h4 = hash4_c ^ (hash4_c >> 11u);
    // 朝向角:黄金角散布 + 时间极缓慢漂移(漂移率随 h2 随机)。
    float angle = (frac(0.300000011920928955078125f + (fi * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((h2 & 8388607u) | 1065353216u) - 1.0f));
    float2 facing2d = -float2(cos(angle), sin(angle));
    float facing_z = facing2d.y;
    float facing_x = facing2d.x;
    float3 facing = float3(facing_x, 0.0f, facing_z);
    // 距离随实例序号递增。
    float t = 1.0f - (fi * 0.00016666666488163173198699951171875f);
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + (10000.0f * (rand01_h1 * rand01_h1))) - 1000.0f;
    // 星点大小:大部分小、少数大。
    float2 size = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, rand01_h1)) * 10.0f)) * 3.0f) * (asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 right = float3(facing_z, 0.0f, -facing_x);
    // 广告牌顶点世界位置。
    float3 world = center + ((right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y)));
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    // 超过 6000 的实例输出 NaN(硬件裁剪掉)。
    bool4 is_far = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(is_far.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, is_far.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, is_far.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, is_far.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = float2(quad_x, float(1u - quad_y));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = world;
    out_var_C3 = right;
    out_var_C4 = facing;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _star.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
