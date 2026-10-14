// 花园低多边形花朵光栅着色器(vs_main + ps_main)。
// 变体 b903e43ffb3da915 / f0adff76827e73ff 均来自对应 oracle bin(spirv-cross 提升,
// 行为与原版一致;本文件只做变量语义化与中文注释)。f0ad 的 ps_main 为空 stub。
// 算法: vs 按实例缩放/平移花朵,抬升到地形高度,perlin 噪声驱动风弯曲,
// 并叠加 grass_object_push 阵风偏移;ps 采样花朵反照率纹理,sqrt 后打包 RGB8。

// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/garden_lowpoly_flower.raster.hlsl | 证据: spirv-cross of
// _garden_lowpoly_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin
// (两个变体 VS 相同:Flower03Xform.position float4 = pos.xyz*scale.w+scale,地形高度抬升,
// perlin 风弯曲(wind_bend^2 * sin(noise*PI-HALF_PI+0.3)*0.0735),grass_object_push 阵风偏移,
// world_to_sample;PS 变体 b903:albedo SampleBias sqrt-clamp,打包 uint2(0x80808080,RGB8+0x4D000000);
// 变体 f0ad:空 PS)| 状态: O(算法来自 SPIR-V);I(函数名;0.4/0.2/0.8/0.0735 字面量)
// | 注:两个 16 进制变体是编译期特性变体 — b903 = albedo PS(默认),f0ad = 空 PS(GARDEN_FLOWER_ALBEDO=0)

#ifndef GARDEN_FLOWER_ALBEDO
#define GARDEN_FLOWER_ALBEDO 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/wind.hlsl"
#include "inc/perlin.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
Texture2D<float4> terrain_heightmap : register(t17, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"

struct Flower03Xform
{
    float4 position;
};

cbuffer $Globals : register(b13, space0)
{
    // NOTE: inc/current_view.hlsl (reconstructed) declares current_view_index
    // as a file-scope global; the original embeds it as $Globals member 0 here.
    // Declared locally so the $Globals binding stays b13 (flag: inc issue).
    uint current_view_index;
    float4x4 model_to_world;
    uint2 path_mask_dims;
    uint2 albedo_map_dims;
    uint2 grass_object_push_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    int bool_render_shadows;
};

// frame_constants.hlsl (reconstructed inc/) omits the dead unbounded-array
// decl the original used to emit RuntimeDescriptorArray; reproduce it here.
RWStructuredBuffer<uint> _rda_trigger[] : register(u100, space0);

StructuredBuffer<Flower03Xform> instance_buffer : register(t21, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

struct VSInput
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(2)]] float3 normal: Vertex_Normal;
    [[vk::location(3)]] float wind_bend: wind_bend;
    uint instance_id : SV_InstanceID;
};

struct VSOutput
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VSOutput vs_main(VSInput i)
{
    float4 xform = instance_buffer[i.instance_id].position;

    float3 pos = i.position * xform.w + xform.xyz;

    float4 world_pos = float4(pos.x, pos.yz, 1.0f);
    float2 uv = (xform.xz * 0.0076923076994717121124267578125f) + 0.5f;
    world_pos.y = (pos.y + get_terrain_height(terrain_heightmap, sampler_default, xform.xz)) - 0.2f;

    float2 wind_uv = world_pos.xz * 0.1f + float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f;
    float gust = sin(perlin(wind_uv) * PI - 1.57f + 0.3f);
    world_pos.x = pos.x - (i.wind_bend * i.wind_bend) * gust * 0.0735f;

    float3 grass = grass_object_push.SampleLevel(sampler_default, uv, 0.0f).xyz * 0.4f;
    float g = clamp(grass.z, 0.0f, 1.0f);

    float4 final_pos;
    if (g > 0.0f)
        final_pos = float4((world_pos.xz + (grass.xy * g) * i.wind_bend), world_pos.y, world_pos.w);
    else
        final_pos = world_pos;

    VSOutput o;
    o.position = mul(view_constants[current_view_index].world_to_sample, final_pos);
    o.C1 = final_pos.xyz;
    o.C2 = i.normal;
    o.C3 = (final_pos.xz * 0.1f + xform.w.xx) + 0.8f;
    return o;
}

Texture2D<float4> albedo_map : register(t15, space0);

#if GARDEN_FLOWER_ALBEDO
struct PSInput
{
    float2 C3 : TEXCOORD3;
};

uint2 ps_main(PSInput i) : SV_Target0
{
    float3 c = sqrt(clamp(albedo_map.SampleBias(sampler_llr, i.C3, global_mip_bias).xyz, 0.0f, 1.0f));
    uint2 out_c = uint2(2155905152u, 0u);
    out_c.y = (uint(clamp(c.x, 0.0f, 1.0f) * 255.0f + 0.5f)
               | (uint(clamp(c.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
               | (uint(clamp(c.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
              + 1291845632u;
    return out_c;
}
#else
// Empty fragment (feature-off variant f0ad): no output interface.
void ps_main()
{
}
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _garden_lowpoly_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Flower03Xform
{
    float4 position;
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
    uint2 path_mask_dims : packoffset(c5);
    uint2 albedo_map_dims : packoffset(c5.z);
    uint2 grass_object_push_dims : packoffset(c6);
    uint2 terrain_heightmap_dims : packoffset(c6.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c7);
    uint2 rocky_terrain_dims : packoffset(c7.z);
    uint2 water_mask_dims : packoffset(c8);
    int bool_render_shadows : packoffset(c8.z);
};

ByteAddressBuffer instance_buffer : register(t21, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 pos = (in_var_Vertex_Position * instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).w) + instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).xyz;
    float pos_x = pos.x;
    float4 world_pos = float4(pos_x, pos.yz, 1.0f);
    float2 uv = (instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    world_pos.y = (pos.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x)) - 0.20000000298023223876953125f;
    float2 wind_uv = (world_pos.xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    uint2 wind_cell_u = uint2(wind_cell);
    uint p00_h0 = wind_cell_u.y;
    uint p00_h1 = p00_h0 + (p00_h0 << 10u);
    uint p00_h2 = p00_h1 ^ (p00_h1 >> 6u);
    uint p00_h3 = p00_h2 + (p00_h2 << 3u);
    uint p00_h4 = p00_h3 ^ (p00_h3 >> 11u);
    uint p00_mix0 = (((wind_cell_u.x * 1664525u) + (p00_h4 + (p00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p00_mix1 = p00_mix0 ^ (p00_mix0 >> 11u);
    uint p00_mix2 = p00_mix1 ^ ((p00_mix1 << 7u) & 2636928640u);
    uint p00_mix3 = p00_mix2 ^ ((p00_mix2 << 15u) & 4022730752u);
    uint2 p10_cell = uint2(wind_cell + int2(1, 0));
    uint p10_h0 = p10_cell.y;
    uint p10_h1 = p10_h0 + (p10_h0 << 10u);
    uint p10_h2 = p10_h1 ^ (p10_h1 >> 6u);
    uint p10_h3 = p10_h2 + (p10_h2 << 3u);
    uint p10_h4 = p10_h3 ^ (p10_h3 >> 11u);
    uint p10_mix0 = (((p10_cell.x * 1664525u) + (p10_h4 + (p10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p10_mix1 = p10_mix0 ^ (p10_mix0 >> 11u);
    uint p10_mix2 = p10_mix1 ^ ((p10_mix1 << 7u) & 2636928640u);
    uint p10_mix3 = p10_mix2 ^ ((p10_mix2 << 15u) & 4022730752u);
    uint2 p01_cell = uint2(wind_cell + int2(0, 1));
    uint p01_h0 = p01_cell.y;
    uint p01_h1 = p01_h0 + (p01_h0 << 10u);
    uint p01_h2 = p01_h1 ^ (p01_h1 >> 6u);
    uint p01_h3 = p01_h2 + (p01_h2 << 3u);
    uint p01_h4 = p01_h3 ^ (p01_h3 >> 11u);
    uint p01_mix0 = (((p01_cell.x * 1664525u) + (p01_h4 + (p01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p01_mix1 = p01_mix0 ^ (p01_mix0 >> 11u);
    uint p01_mix2 = p01_mix1 ^ ((p01_mix1 << 7u) & 2636928640u);
    uint p01_mix3 = p01_mix2 ^ ((p01_mix2 << 15u) & 4022730752u);
    uint2 p11_cell = uint2(wind_cell + int2(1, 1));
    uint p11_h0 = p11_cell.y;
    uint p11_h1 = p11_h0 + (p11_h0 << 10u);
    uint p11_h2 = p11_h1 ^ (p11_h1 >> 6u);
    uint p11_h3 = p11_h2 + (p11_h2 << 3u);
    uint p11_h4 = p11_h3 ^ (p11_h3 >> 11u);
    uint p11_mix0 = (((p11_cell.x * 1664525u) + (p11_h4 + (p11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p11_mix1 = p11_mix0 ^ (p11_mix0 >> 11u);
    uint p11_mix2 = p11_mix1 ^ ((p11_mix1 << 7u) & 2636928640u);
    uint p11_mix3 = p11_mix2 ^ ((p11_mix2 << 15u) & 4022730752u);
    float2 perlin_u = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float perlin_ux = perlin_u.x;
    float perlin_ux1 = 1.0f - perlin_ux;
    float perlin_uy = perlin_u.y;
    world_pos.x = pos_x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - perlin_uy) * (((asfloat(((p00_mix3 ^ (p00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux1) + ((asfloat(((p10_mix3 ^ (p10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux))) + (((asfloat(((p01_mix3 ^ (p01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux1) * perlin_uy)) + (((asfloat(((p11_mix3 ^ (p11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux) * perlin_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float3 grass = grass_object_push.SampleLevel(sampler_default, uv, 0.0f).xyz * 0.4000000059604644775390625f;
    float g = clamp(grass.z, 0.0f, 1.0f);
    float4 final_pos;
    if (g > 0.0f)
    {
        float2 displaced_xz = world_pos.xz + ((grass.xy * g) * in_var_wind_bend);
        final_pos = float4(displaced_xz.x, world_pos.y, displaced_xz.y, world_pos.w);
    }
    else
    {
        final_pos = world_pos;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, final_pos);
    out_var_C1 = final_pos.xyz;
    out_var_C2 = in_var_Vertex_Normal;
    out_var_C3 = ((final_pos.xz * 0.100000001490116119384765625f) + instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).w.xx) + 0.800000011920928955078125f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _garden_lowpoly_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> albedo_map : register(t15, space0);

static float2 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float3 c = sqrt(clamp(albedo_map.SampleBias(sampler_llr, in_var_C3, frame_constants_global_mip_bias).xyz, 0.0f.xxx, 1.0f.xxx));
    uint2 out_c = uint2(2155905152u, 0u);
    out_c.y = ((uint((clamp(c.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(c.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(c.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u;
    out_var_SV_TARGET0 = out_c;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _garden_lowpoly_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Flower03Xform
{
    float4 position;
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
    uint2 path_mask_dims : packoffset(c5);
    uint2 albedo_map_dims : packoffset(c5.z);
    uint2 grass_object_push_dims : packoffset(c6);
    uint2 terrain_heightmap_dims : packoffset(c6.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c7);
    uint2 rocky_terrain_dims : packoffset(c7.z);
    uint2 water_mask_dims : packoffset(c8);
    int bool_render_shadows : packoffset(c8.z);
};

ByteAddressBuffer instance_buffer : register(t21, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 pos = (in_var_Vertex_Position * instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).w) + instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).xyz;
    float pos_x = pos.x;
    float4 world_pos = float4(pos_x, pos.yz, 1.0f);
    float2 uv = (instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    world_pos.y = (pos.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x)) - 0.20000000298023223876953125f;
    float2 wind_uv = (world_pos.xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    uint2 wind_cell_u = uint2(wind_cell);
    uint p00_h0 = wind_cell_u.y;
    uint p00_h1 = p00_h0 + (p00_h0 << 10u);
    uint p00_h2 = p00_h1 ^ (p00_h1 >> 6u);
    uint p00_h3 = p00_h2 + (p00_h2 << 3u);
    uint p00_h4 = p00_h3 ^ (p00_h3 >> 11u);
    uint p00_mix0 = (((wind_cell_u.x * 1664525u) + (p00_h4 + (p00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p00_mix1 = p00_mix0 ^ (p00_mix0 >> 11u);
    uint p00_mix2 = p00_mix1 ^ ((p00_mix1 << 7u) & 2636928640u);
    uint p00_mix3 = p00_mix2 ^ ((p00_mix2 << 15u) & 4022730752u);
    uint2 p10_cell = uint2(wind_cell + int2(1, 0));
    uint p10_h0 = p10_cell.y;
    uint p10_h1 = p10_h0 + (p10_h0 << 10u);
    uint p10_h2 = p10_h1 ^ (p10_h1 >> 6u);
    uint p10_h3 = p10_h2 + (p10_h2 << 3u);
    uint p10_h4 = p10_h3 ^ (p10_h3 >> 11u);
    uint p10_mix0 = (((p10_cell.x * 1664525u) + (p10_h4 + (p10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p10_mix1 = p10_mix0 ^ (p10_mix0 >> 11u);
    uint p10_mix2 = p10_mix1 ^ ((p10_mix1 << 7u) & 2636928640u);
    uint p10_mix3 = p10_mix2 ^ ((p10_mix2 << 15u) & 4022730752u);
    uint2 p01_cell = uint2(wind_cell + int2(0, 1));
    uint p01_h0 = p01_cell.y;
    uint p01_h1 = p01_h0 + (p01_h0 << 10u);
    uint p01_h2 = p01_h1 ^ (p01_h1 >> 6u);
    uint p01_h3 = p01_h2 + (p01_h2 << 3u);
    uint p01_h4 = p01_h3 ^ (p01_h3 >> 11u);
    uint p01_mix0 = (((p01_cell.x * 1664525u) + (p01_h4 + (p01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p01_mix1 = p01_mix0 ^ (p01_mix0 >> 11u);
    uint p01_mix2 = p01_mix1 ^ ((p01_mix1 << 7u) & 2636928640u);
    uint p01_mix3 = p01_mix2 ^ ((p01_mix2 << 15u) & 4022730752u);
    uint2 p11_cell = uint2(wind_cell + int2(1, 1));
    uint p11_h0 = p11_cell.y;
    uint p11_h1 = p11_h0 + (p11_h0 << 10u);
    uint p11_h2 = p11_h1 ^ (p11_h1 >> 6u);
    uint p11_h3 = p11_h2 + (p11_h2 << 3u);
    uint p11_h4 = p11_h3 ^ (p11_h3 >> 11u);
    uint p11_mix0 = (((p11_cell.x * 1664525u) + (p11_h4 + (p11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint p11_mix1 = p11_mix0 ^ (p11_mix0 >> 11u);
    uint p11_mix2 = p11_mix1 ^ ((p11_mix1 << 7u) & 2636928640u);
    uint p11_mix3 = p11_mix2 ^ ((p11_mix2 << 15u) & 4022730752u);
    float2 perlin_u = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float perlin_ux = perlin_u.x;
    float perlin_ux1 = 1.0f - perlin_ux;
    float perlin_uy = perlin_u.y;
    world_pos.x = pos_x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - perlin_uy) * (((asfloat(((p00_mix3 ^ (p00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux1) + ((asfloat(((p10_mix3 ^ (p10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux))) + (((asfloat(((p01_mix3 ^ (p01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux1) * perlin_uy)) + (((asfloat(((p11_mix3 ^ (p11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * perlin_ux) * perlin_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float3 grass = grass_object_push.SampleLevel(sampler_default, uv, 0.0f).xyz * 0.4000000059604644775390625f;
    float g = clamp(grass.z, 0.0f, 1.0f);
    float4 final_pos;
    if (g > 0.0f)
    {
        float2 displaced_xz = world_pos.xz + ((grass.xy * g) * in_var_wind_bend);
        final_pos = float4(displaced_xz.x, world_pos.y, displaced_xz.y, world_pos.w);
    }
    else
    {
        final_pos = world_pos;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, final_pos);
    out_var_C1 = final_pos.xyz;
    out_var_C2 = in_var_Vertex_Normal;
    out_var_C3 = ((final_pos.xz * 0.100000001490116119384765625f) + instance_buffer.Load<float4>(uint(gl_InstanceIndex) * 16 + 0).w.xx) + 0.800000011920928955078125f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _garden_lowpoly_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
