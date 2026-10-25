// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water_snow_plane.raster.hlsl
// SOURCE PATH : shaders/water_snow_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin  (ps 写入雪色调 uint2(0xFF80..))
//   _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin  (ps: 同样的采样/丢弃,无输出接口)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs transform + ps snow-mask/perlin from spirv-cross)
// Evidence: spirv-cross vs (water plane at y=-0.01, xz*130 heightmap-space, world_to_sample;
//   C1 = world.xyz) and ps (water_mask.SampleLevel(sampler_llc) green channel vs 0.2;
//   perlin(C1.xz*1.5) variance discard; b903e writes const uint2 snow tint, f0adff no target).
// MERGE (water-ice review water_snow_plane.raster.hlsl): A adopted wholesale — exact
//   transcription (thresholds/tint uint2(4286644096,114677205)).
// Note: the two 16-hex variants are a compile-time feature variant — b903e = colored
//   snow plane, f0adff = no-output (depth/mask-only). Gated on WATER_SNOW_PLANE_COLOR (I).
//
// 中文说明:水面上的积雪平面。
//   vs:把水面网格放到 y=-0.01、xz 放大 130 倍,变换到采样空间;
//   ps:按水遮罩的绿色通道过滤水面,用 perlin 值噪声做方差抖动丢弃;
//       b903e 变体写雪色调, f0adff 变体只做深度/遮罩不写颜色。

#ifndef WATER_SNOW_PLANE_COLOR
#define WATER_SNOW_PLANE_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 water_mask_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 水面网格:世界 xz 放大 130 倍,钉在 y=-0.01。
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = -0.00999999977648258209228515625f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C1 = p.xyz;
    return o;
}

#if WATER_SNOW_PLANE_COLOR
struct PsIn
{
    float3 C1 : TEXCOORD1;
};

uint2 ps_main(PsIn vin) : SV_Target0
{
    // 采样水遮罩的绿色通道,低于 0.2 的区域(无水)直接丢弃。
    float4 wm = water_mask.SampleLevel(sampler_llc, (vin.C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }

    // 2D 值噪声(perlin.hlsl)内联,显式双线性运算。
    float2 p = vin.C1.xz * 1.5f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;

    // 噪声方差抖动:遮罩值叠加噪声后,覆盖度不足 0.8 的区域丢弃。
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f,
                wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
    return uint2(4286644096u, 114677205u);
}
#else
// 无输出变体:同样的采样/丢弃,但不写颜色(仅深度/遮罩)。
struct PsInEmpty
{
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
    float4 wm = water_mask.SampleLevel(sampler_llc, (vin.C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }

    // 2D 值噪声内联(与有色分支相同)。
    float2 p = vin.C1.xz * 1.5f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;

    // 噪声方差抖动丢弃。
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f,
                wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
}
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 water_mask_dims : packoffset(c5);
    int bool_render_shadows : packoffset(c5.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 水面网格变换(与手写版 vs_main 相同)。
    float4 world = mul(model_to_world, float4(in_var_Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = -0.00999999977648258209228515625f;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, p);
    out_var_C1 = p.xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> water_mask : register(t14, space0);

static float3 in_var_C1;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 水遮罩过滤 + perlin 噪声方差抖动(与手写版 ps_main 相同)。
    float4 wm = water_mask.SampleLevel(sampler_llc, (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }
    float2 noise_p = in_var_C1.xz * 1.5f;
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
    float noise = (((1.0f - uy) * (((asfloat(((h00_4 ^ (h00_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) + ((asfloat(((h10_4 ^ (h10_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux))) + (((asfloat(((h01_4 ^ (h01_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) * uy)) + (((asfloat(((h11_4 ^ (h11_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux) * uy);
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f, wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
    out_var_SV_TARGET0 = uint2(4286644096u, 114677205u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 water_mask_dims : packoffset(c5);
    int bool_render_shadows : packoffset(c5.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 水面网格变换(与手写版 vs_main 相同)。
    float4 world = mul(model_to_world, float4(in_var_Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = -0.00999999977648258209228515625f;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, p);
    out_var_C1 = p.xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> water_mask : register(t14, space0);

static float3 in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    // 水遮罩过滤 + perlin 噪声方差抖动(与手写版 ps_main 相同)。
    float4 wm = water_mask.SampleLevel(sampler_llc, (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }
    float2 noise_p = in_var_C1.xz * 1.5f;
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
    float noise = (((1.0f - uy) * (((asfloat(((h00_4 ^ (h00_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) + ((asfloat(((h10_4 ^ (h10_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux))) + (((asfloat(((h01_4 ^ (h01_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ox) * uy)) + (((asfloat(((h11_4 ^ (h11_4 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ux) * uy);
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f, wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
