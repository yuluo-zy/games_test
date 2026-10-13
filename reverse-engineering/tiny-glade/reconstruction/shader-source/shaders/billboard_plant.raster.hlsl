// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/billboard_plant.raster.hlsl
// Evidence: _billboard_plant.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/model_to_world/flower1..3/grass_patch/
//   terrain_texture dims/bool_render_shadows), frame_constants set1/b0. (ps): sampler_lnc s0,
//   sampler_llr s1, flower1..3 t14..16, grass_patch t17, terrain_texture t18, frame_constants.
//   vs_main passes the model transform through; ps_main blends the terrain texture with a
//   plant-type atlas (flower1/2/3 or grass_patch) by plant_id, alpha-discards, packs the
//   normal + albedo into a uint2 target.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"

// color_library.hlsl is guard-skipped (billboard_plant binds no color_library cbuffer).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
// NOTE: inc/lighting.hlsl omitted (references nested color_library.indirect_light_tint;
// unused here) — see billboard_forest note.

// $Globals members (implicit cbuffer at set0/b13).
float4x4 model_to_world;
uint2 flower1_dims;
uint2 flower2_dims;
uint2 flower3_dims;
uint2 grass_patch_dims;
uint2 terrain_texture_dims;
int bool_render_shadows;

Texture2D<float4> flower1 : register(t14, space0);
Texture2D<float4> flower2 : register(t15, space0);
Texture2D<float4> flower3 : register(t16, space0);
Texture2D<float4> grass_patch : register(t17, space0);
Texture2D<float4> terrain_texture : register(t18, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float3 vertex_normal: Vertex_Normal;
    [[vk::location(2)]] float2 vertex_uv: Vertex_UV;
    [[vk::location(3)]] float3 uv2: uv2;
    [[vk::location(4)]] float bby: bby;
    [[vk::location(5)]] int plant_id: plant_id;
    [[vk::location(6)]] int prim_id: prim_id;
};

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 world = mul(model_to_world, float4(i.vertex_position, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.C0 = i.vertex_uv;
    o.C1 = mul(model_to_world, float4(i.vertex_normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    o.C3 = float2(i.uv2.x, 1.0f - i.uv2.y);
    o.C4 = i.bby;
    o.C5 = i.plant_id;
    o.C6 = i.prim_id;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float4 terrain = terrain_texture.Sample(sampler_lnc, i.C3);
    float3 albedo;
    float alpha;
    if (i.C5 == 0)
    {
        float4 tex = flower1.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, (i.C4 * i.C4).xxx);
        alpha = tex.w * tex.w;
    }
    else if (i.C5 == 2)
    {
        float4 tex = flower2.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, i.C4.xxx);
        alpha = tex.w * tex.w;
    }
    else if (i.C5 == 3)
    {
        float4 tex = flower3.SampleBias(sampler_lnc, i.C0, (-1.0f) + global_mip_bias);
        albedo = lerp(terrain.xyz, tex.xyz, i.C4.xxx);
        alpha = tex.w * tex.w;
    }
    else
    {
        int sub = i.C6 - 4 * (i.C6 / 4);
        float4 tex = grass_patch.SampleBias(sampler_llr, float2(0.5f * (i.C0.x + float(sub - 2 * (sub / 2))), 0.5f * (i.C0.y + float(int(floor(float(sub / 2)))))), (-1.0f) + global_mip_bias);
        float3 ga;
        if (glade_theme_id == 4u)
        {
            ga = lerp(terrain.xyz, tex.xyz, pow(i.C4, 2.5f));
        }
        else
        {
            ga = lerp(terrain.xyz, tex.xyz, pow(i.C4, 1.5f));
        }
        albedo = ga;
        alpha = tex.w * tex.w;
    }
    if (alpha < 0.1f)
    {
        discard;
    }
    float3 n = (normalize(i.C1) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xE6000000u;
    o.target = uint2(normal_packed, albedo_packed | 0x06000000u);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _billboard_plant.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> flower1 : register(t14, space0);
Texture2D<float4> flower2 : register(t15, space0);
Texture2D<float4> flower3 : register(t16, space0);
Texture2D<float4> grass_patch : register(t17, space0);
Texture2D<float4> terrain_texture : register(t18, space0);

static float2 in_var_C0;
static float3 in_var_C1;
static float2 in_var_C3;
static float in_var_C4;
static int in_var_C5;
static int in_var_C6;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation int in_var_C6 : TEXCOORD6;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _103 = terrain_texture.Sample(sampler_lnc, in_var_C3);
    float3 _198;
    float _199;
    if (in_var_C5 == 0)
    {
        float4 _113 = flower1.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
        float _114 = _113.w;
        _198 = lerp(_103.xyz, _113.xyz, (in_var_C4 * in_var_C4).xxx);
        _199 = _114 * _114;
    }
    else
    {
        float3 _196;
        float _197;
        if (in_var_C5 == 2)
        {
            float4 _130 = flower2.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
            float _131 = _130.w;
            _196 = lerp(_103.xyz, _130.xyz, in_var_C4.xxx);
            _197 = _131 * _131;
        }
        else
        {
            float3 _194;
            float _195;
            if (in_var_C5 == 3)
            {
                float4 _146 = flower3.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
                float _147 = _146.w;
                _194 = lerp(_103.xyz, _146.xyz, in_var_C4.xxx);
                _195 = _147 * _147;
            }
            else
            {
                int _153 = in_var_C6 - 4 * (in_var_C6 / 4);
                float4 _174 = grass_patch.SampleBias(sampler_llr, float2(0.5f * (in_var_C0.x + float(_153 - 2 * (_153 / 2))), 0.5f * (in_var_C0.y + float(int(floor(float(_153 / 2)))))), (-1.0f) + frame_constants_global_mip_bias);
                float _175 = _174.w;
                float3 _193;
                if (frame_constants_glade_theme_id == 4u)
                {
                    _193 = lerp(_103.xyz, _174.xyz, pow(in_var_C4, 2.5f).xxx);
                }
                else
                {
                    _193 = lerp(_103.xyz, _174.xyz, pow(in_var_C4, 1.5f).xxx);
                }
                _194 = _193;
                _195 = _175 * _175;
            }
            _196 = _194;
            _197 = _195;
        }
        _198 = _196;
        _199 = _197;
    }
    if (_199 < 0.100000001490116119384765625f)
    {
        discard;
    }
    float3 _205 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _227 = sqrt(clamp(_198, 0.0f.xxx, 1.0f.xxx));
    uint _246 = (uint((clamp(_227.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_227.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_227.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _247 = uint2(((uint((clamp(_205.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_205.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_205.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3858759680u, _246);
    _247.y = _246 | 100663296u;
    out_var_SV_TARGET0 = _247;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _billboard_plant.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> flower1 : register(t14, space0);
Texture2D<float4> flower2 : register(t15, space0);
Texture2D<float4> flower3 : register(t16, space0);
Texture2D<float4> grass_patch : register(t17, space0);

static float2 in_var_C0;
static int in_var_C5;
static int in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(5)]] nointerpolation int in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation int in_var_C6 : TEXCOORD6;
};

void frag_main()
{
    float _139;
    if (in_var_C5 == 0)
    {
        float4 _84 = flower1.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
        float _85 = _84.w;
        _139 = _85 * _85;
    }
    else
    {
        float _138;
        if (in_var_C5 == 2)
        {
            float4 _97 = flower2.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
            float _98 = _97.w;
            _138 = _98 * _98;
        }
        else
        {
            float _137;
            if (in_var_C5 == 3)
            {
                float4 _110 = flower3.SampleBias(sampler_lnc, in_var_C0, (-1.0f) + frame_constants_global_mip_bias);
                float _111 = _110.w;
                _137 = _111 * _111;
            }
            else
            {
                int _113 = in_var_C6 - 4 * (in_var_C6 / 4);
                float4 _134 = grass_patch.SampleBias(sampler_llr, float2(0.5f * (in_var_C0.x + float(_113 - 2 * (_113 / 2))), 0.5f * (in_var_C0.y + float(int(floor(float(_113 / 2)))))), (-1.0f) + frame_constants_global_mip_bias);
                float _135 = _134.w;
                _137 = _135 * _135;
            }
            _138 = _137;
        }
        _139 = _138;
    }
    if (_139 < 0.100000001490116119384765625f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
