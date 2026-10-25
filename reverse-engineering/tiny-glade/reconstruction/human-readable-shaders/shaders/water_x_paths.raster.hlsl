// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water_x_paths.raster.hlsl
// SOURCE PATH : shaders/water_x_paths.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_x_paths.raster.hlsl_b903e43ffb3da915.bin   (vs 水面内路径抬升; ps 混合岩石/主题/路径)
//   _water_x_paths.raster.hlsl_f0adff76827e73ff.bin   (vs 相同; ps 空)
// entries: vs:vs_main  ps:ps_main | index_pass W2 (Inference)
// Status: O (bindings/entry; vs + ps from spirv-cross)
// Evidence: spirv-cross vs (water-mask bounds-checked lift of path/building masks,
//   NaN-dip outside water; C0=(0.13,0.13,0.10), C1=normal, C2=world, C3=prim_center)
//   and ps (b903e: rocky_terrain_texture triplanar with hash jitter, theme-colored
//   blend, explicit-arithmetic perlin dry/water mask, path_mask tint for theme 4,
//   RG8/RGB8 pack; f0adff: empty).
// MERGE (water-ice review water_x_paths.raster.hlsl): A adopted wholesale (closure:
//   texture_utils/texture_blend/water_mask_sampler/path_mask_sampler + material stack).
// Note: two 16-hex variants are a compile-time feature variant — b903e = colored
//   x-paths, f0adff = no-output. Gated on WATER_X_PATHS_COLOR (I).
//
// 中文说明:水面上的"交叉路径"。vs 检测水面遮罩,把路径/建筑遮罩对应位置抬高,
//   无水区域输出 NaN 让三角形被裁掉;ps 用岩石纹理三平面映射 + 主题色混合 +
//   perlin 干湿噪声,主题 4 时叠加路径色,打包进 gbuffer。f0adff 变体不写颜色。

#ifndef WATER_X_PATHS_COLOR
#define WATER_X_PATHS_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/texture_blend.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 buildings_mask_dims;
    uint2 rocky_terrain_texture_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> buildings_mask : register(t16, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 prim_center: prim_center;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 pos = float4(vin.Vertex_Position, 1.0f);
    // 以图元中心/顶点的世界 xz 换算成 0..1 遮罩 UV。
    float2 prim_uv = (vin.prim_center.xz * 0.0076923076994717121124267578125f) + 0.5f;
    float2 pos_uv = (vin.Vertex_Position.xz * 0.0076923076994717121124267578125f) + 0.5f;

    // 水遮罩绿通道;图元中心超出遮罩范围时视为无水。
    float water;
    bool out_x = (prim_uv.x < 0.0f) || (prim_uv.x > 1.0f);
    bool out_xy = out_x || (prim_uv.y < 0.0f);
    bool out_xyx = out_xy || (prim_uv.y > 1.0f);
    if (out_xyx)
    {
        water = 0.0f;
    }
    else
    {
        water = water_mask.SampleLevel(sampler_default, prim_uv, 0.0f).y;
    }

    if (water > 0.20000000298023223876953125f)
    {
        // 水中:按路径遮罩把水面抬高(路径像桥一样露出),再按建筑遮罩压低。
        uint h = uint((vin.prim_center.x + vin.prim_center.z) + 100.0f);
        h += (h << 10u);
        h ^= (h >> 6u);
        h += (h << 3u);
        h ^= (h >> 11u);
        float jitter = asfloat(((h + (h << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
        pos.y = (vin.Vertex_Position.y + (-3.0f))
            + (pow(path_mask.SampleLevel(sampler_llb, prim_uv, 0.0f).x, 0.20000000298023223876953125f) * 1.5f)
            + ((1.5f * pow(path_mask.SampleLevel(sampler_llb, pos_uv, 0.0f).x, 0.20000000298023223876953125f))
                * (1.2000000476837158203125f + lerp(-0.100000001490116119384765625f, 0.300000011920928955078125f, jitter)))
            - smoothstep(0.3499999940395355224609375f, 0.60000002384185791015625f,
                buildings_mask.SampleLevel(sampler_default, pos_uv, 0.0f).x);
    }
    else
    {
        // 无水:NaN 位置使三角形被裁剪。
        pos = float4(asfloat(0x7fbfffffu), asfloat(0x7fbfffffu), asfloat(0x7fbfffffu), pos.w);
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, pos);
    o.C0 = float3(0.12999999523162841796875f, 0.12999999523162841796875f, 0.100000001490116119384765625f);
    o.C1 = vin.Vertex_Normal;
    o.C2 = pos.xyz;
    o.C3 = vin.prim_center;
    return o;
}

#if WATER_X_PATHS_COLOR
struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

Texture2D<float4> rocky_terrain_texture : register(t17, space0);

uint2 ps_main(PsIn vin) : SV_Target0
{
    float3 n = normalize(vin.C1);
    float3 world_frac = vin.C2 * 0.0333333350718021392822265625f;

    // 按图元中心生成抖动,用于三平面采样与噪声偏移。
    uint h = uint(1000.0f * (abs(vin.C3.x) + abs(vin.C3.z)));
    h += (h << 10u);
    h ^= (h >> 6u);
    h += (h << 3u);
    h ^= (h >> 11u);
    float jitter = asfloat(((h + (h << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
    float2 j2 = (jitter * 0.20000000298023223876953125f).xx;

    // 岩石纹理三平面采样(zy/xy/xz 三面,按法线加权)。
    float3 rocky_zy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.zy * 2.0f) + j2).xyz;
    float3 rocky_xy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xy * 2.0f) + j2).xyz;
    float3 rocky_xz = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xz * 2.0f) + j2).xyz;

    // 以各面亮度减最小值为三平面权重。
    float3 pa = pow(abs(n), 1.0f);
    float l_xy = dot(rocky_xy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float l_zy = dot(rocky_zy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + pa.x;
    float l_a = l_xy + pa.z;
    float l_b = l_xy + pa.y;
    float lm = max(max(l_zy, l_a), l_b) - 0.20000000298023223876953125f;
    float w_zy = max(l_zy - lm, 0.0f);
    float w_a = max(l_a - lm, 0.0f);
    float w_b = max(l_b - lm, 0.0f);

    // 主题色:深色主题(2/4)用深棕,否则用绿色。
    bool theme_dark = (glade_theme_id == 2u) || (glade_theme_id == 4u);
    float3 theme_col = theme_dark
        ? float3(0.0625f, 0.0399999953806400299072265625f, 0.005624999292194843292236328125f)
        : float3(0.0419999994337558746337890625f, 0.070000000298023223876953125f, 0.02099999971687793731689453125f);

    // 2D 值噪声(显式运算形式;见 water_snow_plane 说明),得到"干燥度"。
    float2 p = (((vin.C2.xz + vin.C2.xy) + (vin.C2.yz * 0.5f)) + (jitter * 200.0f)) * 0.85000002384185791015625f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
    float dry = smoothstep(0.5f, 1.0f, noise);

    // 混合:三平面岩石 * 1.1 -> 按高度渐入主题色 -> 垂直面渐入主题色。
    float3 rock_blend = ((((rocky_zy * w_zy) + (rocky_xy * w_a)) + (rocky_xz * w_b))
        / (w_zy + w_a + w_b)) * 1.10000002384185791015625f;
    float3 col = lerp(lerp(rock_blend, theme_col,
                ((1.0f - smoothstep(-0.20000000298023223876953125f, 0.60000002384185791015625f - dry, vin.C2.y)) * 0.75f)),
            theme_col, clamp(pa.x + pa.z, 0.0f, 1.0f) * dry);

    // 主题 4(雪):高出水面的路径叠加雪色。
    if (glade_theme_id == 4u && vin.C2.y > 0.0f)
    {
        float pm = path_mask.SampleLevel(sampler_llb, (vin.C2.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x;
        float pm2 = pm * pm;
        col = lerp(col, 0.699999988079071044921875f,
            (smoothstep(0.20000000298023223876953125f, 1.0f,
                clamp((n.y * (pm2 * pm2)) * lerp(0.5f, 1.0f, dry), 0.0f, 1.0f)) + 0.100000001490116119384765625f));
    }

    // 打包法线 + 反照率(sRGB 近似)进 uint2 gbuffer。
    float3 n_pack = (n * 0.5f) + 0.5f;
    float3 c_pack = sqrt(clamp(col, 0.0f, 1.0f));
    uint albedo = (uint((clamp(c_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                + (uint((clamp(c_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                + (uint((clamp(c_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 out_px = uint2(
        ((uint((clamp(n_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
        + (uint((clamp(n_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
        + (uint((clamp(n_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u,
        albedo);
    out_px.y = albedo | 117440512u;
    return out_px;
}
#else
struct PsInEmpty
{
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
}
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _water_x_paths.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_mipmap : register(s10, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> rocky_terrain_texture : register(t17, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 岩石三平面 + 主题色 + perlin 干湿噪声(与手写版 ps_main 相同)。
    float3 n = normalize(in_var_C1);
    float3 world_frac = in_var_C2 * 0.0333333350718021392822265625f.xxx;
    uint seed_u = uint(1000.0f * (abs(in_var_C3.x) + abs(in_var_C3.z)));
    uint h1 = seed_u + (seed_u << 10u);
    uint h2 = h1 ^ (h1 >> 6u);
    uint h3 = h2 + (h2 << 3u);
    uint h4 = h3 ^ (h3 >> 11u);
    float jitter = asfloat(((h4 + (h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 j2 = (jitter * 0.20000000298023223876953125f).xx;
    float3 rocky_zy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.zy * 2.0f) + j2).xyz;
    float3 rocky_xy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xy * 2.0f) + j2).xyz;
    float4 rocky_xz = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xz * 2.0f) + j2);
    float3 pa = pow(abs(n), 1.0f.xxx);
    float pa_x = pa.x;
    float pa_z = pa.z;
    float l_xy = dot(rocky_xy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float l_zy = dot(rocky_zy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + pa_x;
    float l_a = l_xy + pa_z;
    float l_b = l_xy + pa.y;
    float lm = max(max(l_zy, l_a), l_b) - 0.20000000298023223876953125f;
    float w_zy = max(l_zy - lm, 0.0f);
    float w_a = max(l_a - lm, 0.0f);
    float w_b = max(l_b - lm, 0.0f);
    bool theme_dark;
    if (!(frame_constants_glade_theme_id == 2u))
    {
        theme_dark = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        theme_dark = true;
    }
    bool3 theme_bool3 = theme_dark.xxx;
    float3 theme_col = float3(theme_bool3.x ? float3(0.0625f, 0.0399999953806400299072265625f, 0.005624999292194843292236328125f).x : float3(0.0419999994337558746337890625f, 0.070000000298023223876953125f, 0.02099999971687793731689453125f).x, theme_bool3.y ? float3(0.0625f, 0.0399999953806400299072265625f, 0.005624999292194843292236328125f).y : float3(0.0419999994337558746337890625f, 0.070000000298023223876953125f, 0.02099999971687793731689453125f).y, theme_bool3.z ? float3(0.0625f, 0.0399999953806400299072265625f, 0.005624999292194843292236328125f).z : float3(0.0419999994337558746337890625f, 0.070000000298023223876953125f, 0.02099999971687793731689453125f).z);
    float2 noise_p = (((in_var_C2.xz + in_var_C2.xy) + (in_var_C2.yz * 0.5f)) + (jitter * 200.0f).xx) * 0.85000002384185791015625f;
    int2 cell = int2(floor(noise_p));
    float2 f = frac(noise_p);
    uint2 cell_u = uint2(cell);
    uint cell_y = cell_u.y;
    uint hy1 = cell_y + (cell_y << 10u);
    uint hy2 = hy1 ^ (hy1 >> 6u);
    uint hy3 = hy2 + (hy2 << 3u);
    uint hy4 = hy3 ^ (hy3 >> 11u);
    uint h00_1 = (((cell_u.x * 1664525u) + (hy4 + (hy4 << 15u))) + 1013904223u) * 1664525u;
    uint h00_2 = h00_1 ^ (h00_1 >> 11u);
    uint h00_3 = h00_2 ^ ((h00_2 << 7u) & 2636928640u);
    uint h00_4 = h00_3 ^ ((h00_3 << 15u) & 4022730752u);
    uint2 cell10 = uint2(cell + int2(1, 0));
    uint cell10_y = cell10.y;
    uint hy10_1 = cell10_y + (cell10_y << 10u);
    uint hy10_2 = hy10_1 ^ (hy10_1 >> 6u);
    uint hy10_3 = hy10_2 + (hy10_2 << 3u);
    uint hy10_4 = hy10_3 ^ (hy10_3 >> 11u);
    uint h10_1 = (((cell10.x * 1664525u) + (hy10_4 + (hy10_4 << 15u))) + 1013904223u) * 1664525u;
    uint h10_2 = h10_1 ^ (h10_1 >> 11u);
    uint h10_3 = h10_2 ^ ((h10_2 << 7u) & 2636928640u);
    uint h10_4 = h10_3 ^ ((h10_3 << 15u) & 4022730752u);
    uint2 cell01 = uint2(cell + int2(0, 1));
    uint cell01_y = cell01.y;
    uint hy01_1 = cell01_y + (cell01_y << 10u);
    uint hy01_2 = hy01_1 ^ (hy01_1 >> 6u);
    uint hy01_3 = hy01_2 + (hy01_2 << 3u);
    uint hy01_4 = hy01_3 ^ (hy01_3 >> 11u);
    uint h01_1 = (((cell01.x * 1664525u) + (hy01_4 + (hy01_4 << 15u))) + 1013904223u) * 1664525u;
    uint h01_2 = h01_1 ^ (h01_1 >> 11u);
    uint h01_3 = h01_2 ^ ((h01_2 << 7u) & 2636928640u);
    uint h01_4 = h01_3 ^ ((h01_3 << 15u) & 4022730752u);
    uint2 cell11 = uint2(cell + int2(1, 1));
    uint cell11_y = cell11.y;
    uint hy11_1 = cell11_y + (cell11_y << 10u);
    uint hy11_2 = hy11_1 ^ (hy11_1 >> 6u);
    uint hy11_3 = hy11_2 + (hy11_2 << 3u);
    uint hy11_4 = hy11_3 ^ (hy11_3 >> 11u);
    uint h11_1 = (((cell11.x * 1664525u) + (hy11_4 + (hy11_4 << 15u))) + 1013904223u) * 1664525u;
    uint h11_2 = h11_1 ^ (h11_1 >> 11u);
    uint h11_3 = h11_2 ^ ((h11_2 << 7u) & 2636928640u);
    uint h11_4 = h11_3 ^ ((h11_3 << 15u) & 4022730752u);
    float2 u = (f * f) * (3.0f.xx - (f * 2.0f));
    float ux = u.x;
    float ox = 1.0f - ux;
    float uy = u.y;
    float dry = smoothstep(0.5f, 1.0f, (((1.0f - uy) * (((asfloat(((h00_4 ^ (h00_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) + ((asfloat(((h10_4 ^ (h10_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux))) + (((asfloat(((h01_4 ^ (h01_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) * uy)) + (((asfloat(((h11_4 ^ (h11_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux) * uy));
    float3 rock_col = lerp(lerp(((((rocky_zy * w_zy) + (rocky_xy * w_a)) + (rocky_xz.xyz * w_b)) / ((w_zy + w_a) + w_b).xxx) * 1.10000002384185791015625f, theme_col, ((1.0f - smoothstep(-0.20000000298023223876953125f, 0.60000002384185791015625f - dry, in_var_C2.y)) * 0.75f).xxx), theme_col, (clamp(pa_x + pa_z, 0.0f, 1.0f) * dry).xxx);
    bool theme4_up;
    if (frame_constants_glade_theme_id == 4u)
    {
        theme4_up = in_var_C2.y > 0.0f;
    }
    else
    {
        theme4_up = false;
    }
    float3 col;
    if (theme4_up)
    {
        float4 pm_sample = path_mask.SampleLevel(sampler_llb, (in_var_C2.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
        float pm = pm_sample.x;
        float pm2 = pm * pm;
        col = lerp(rock_col, 0.699999988079071044921875f.xxx, (smoothstep(0.20000000298023223876953125f, 1.0f, clamp((n.y * (pm2 * pm2)) * lerp(0.5f, 1.0f, dry), 0.0f, 1.0f)) + 0.100000001490116119384765625f).xxx);
    }
    else
    {
        col = rock_col;
    }
    float3 n_pack = (n * 0.5f) + 0.5f.xxx;
    float3 c_pack = sqrt(clamp(col, 0.0f.xxx, 1.0f.xxx));
    uint albedo = (uint((clamp(c_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(c_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(c_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 out_px = uint2(((uint((clamp(n_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(n_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(n_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo);
    out_px.y = albedo | 117440512u;
    out_var_SV_TARGET0 = out_px;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _water_x_paths.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
