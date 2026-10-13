// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/clearing/meadow_flower.raster.hlsl
// Evidence: _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): sampler_llb s4, sampler_default s9, $Globals b13, path_mask t14,
//   grass_object_push t19, terrain_heightmap t20, rocky_terrain t22, water_mask t23,
//   frame_constants. (ps): sampler_llr s1, $Globals b13, albedo_map t15, normal_map t16,
//   alpha_map t17, leafy_greens t18, frame_constants.
//   vs_main: rocky/path/water rejection, terrain snap, wind value-noise, TBN. ps_main:
//   alpha discard + frame-stagger discard, albedo/normal from maps, packed uint2.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> terrain_heightmap : register(t20, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "inc/pack_unpack.hlsl"

// color_library.hlsl guard-skipped (not bound by this shader).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).

// $Globals members (implicit cbuffer at set0/b13).
float4x4 model_to_world;
float time;
uint2 path_mask_dims;
uint2 albedo_map_dims;
uint2 normal_map_dims;
int has_normal_map;
uint2 alpha_map_dims;
uint2 leafy_greens_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 rocky_terrain_dims;
uint2 water_mask_dims;
int bool_render_shadows;

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float3 vertex_normal: Vertex_Normal;
    [[vk::location(2)]] float2 vertex_uv: Vertex_UV;
    [[vk::location(3)]] float3 flower_pivot: flower_pivot;
    [[vk::location(4)]] int stem: stem;
    [[vk::location(5)]] float wind_bend: wind_bend;
};

struct VS_OUT
{
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 wp;
    float2 wind_uv;
    float3 world_pos;
    float3 normal;
    int stem_out;
    float2 uv_out;
    float3x3 tbn;
    do
    {
        float2 mask_uv = (i.flower_pivot.xz * (1.0f / 130.0f)) + 0.5f;
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
        float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
        if ((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f))
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        bool water_reject;
        if (!((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f)))
        {
            float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
            water_reject = water > 0.25f;
        }
        else
        {
            water_reject = true;
        }
        if (water_reject)
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
        float4 v = float4(i.vertex_position, 1.0f);
        v.y = i.vertex_position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, mask_uv, 0.0f).x);
        float2 wind_xy = (i.flower_pivot.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
        int2 wind_cell = int2(floor(wind_xy));
        float2 wind_frac = frac(wind_xy);
        float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
        float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
        float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
        float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
        float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
        float wind_noise = ((1.0f - wind_smooth.y) * ((wind_n00 * (1.0f - wind_smooth.x)) + (wind_n10 * wind_smooth.x)))
            + ((wind_n01 * (1.0f - wind_smooth.x)) * wind_smooth.y)
            + ((wind_n11 * wind_smooth.x) * wind_smooth.y);
        v.x = i.vertex_position.x - (i.wind_bend * sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.175f;
        float4 push = grass_object_push.SampleLevel(sampler_default, mask_uv, 0.0f);
        float4 v2;
        if (clamp(push.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 pushed = v.xz + ((push.xy * pow(i.wind_bend, 0.5f)) * 0.5f);
            v2 = float4(pushed.x, v.y, pushed.y, v.w);
        }
        else
        {
            v2 = v;
        }
        wp = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v2.xyz, 1.0f)));
        wind_uv = wind_xy;
        world_pos = v2.xyz;
        normal = i.vertex_normal;
        stem_out = i.stem;
        uv_out = i.vertex_uv;
        tbn = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while (false);
    o.position = wp;
    o.C1 = wind_uv;
    o.C2 = world_pos;
    o.C3 = normal;
    o.C5 = stem_out;
    o.C6 = uv_out;
    o.C7 = tbn;
    return o;
}

// Pack a [0,1] normal rgb into one uint channel of the uint2 target.
uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

struct PS_IN
{
    float2 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
    if (alpha_map.Sample(sampler_llr, i.C6).x < 0.5f)
    {
        discard;
    }
    if (bool_render_shadows != 1)
    {
        uint2 pix = uint2(frag_coord.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (pix.x + (pix.y * 2u)))
        {
            discard;
        }
    }
    float4 albedo_tex = albedo_map.Sample(sampler_llr, i.C1);
    float3 albedo;
    if (i.C5 == 1)
    {
        albedo = leafy_greens.Sample(sampler_llr, i.C1).xyz;
    }
    else
    {
        albedo = albedo_tex.xyz;
    }
    float3 n;
    if (has_normal_map == 1)
    {
        n = normalize(mul(i.C7, (normal_map.Sample(sampler_llr, i.C6).xyz * 2.0f) - 1.0f));
    }
    else
    {
        n = i.C3;
    }
    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    o.target = uint2((pack_normal8(packed_n) + 0x80000000u), (pack_normal8(color) + 0x4D000000u));
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _448;
    float2 _449;
    float3 _450;
    float3 _451;
    int _452;
    float2 _453;
    float3x3 _454;
    do
    {
        float _166;
        bool _168;
        float2 _159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _193;
        do
        {
            _166 = _159.x;
            _168 = !(_166 < 0.0f);
            bool _172;
            if (_168)
            {
                _172 = _166 > 1.0f;
            }
            else
            {
                _172 = true;
            }
            bool _178;
            if (!_172)
            {
                _178 = _159.y < 0.0f;
            }
            else
            {
                _178 = true;
            }
            bool _184;
            if (!_178)
            {
                _184 = _159.y > 1.0f;
            }
            else
            {
                _184 = true;
            }
            if (_184)
            {
                _193 = 0.0f;
                break;
            }
            else
            {
                _193 = rocky_terrain.SampleLevel(sampler_default, _159, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, _159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            _199 = _193 > 0.100000001490116119384765625f;
        }
        else
        {
            _199 = true;
        }
        bool _231;
        if (!_199)
        {
            float _229;
            do
            {
                bool _208;
                if (_168)
                {
                    _208 = _166 > 1.0f;
                }
                else
                {
                    _208 = true;
                }
                bool _214;
                if (!_208)
                {
                    _214 = _159.y < 0.0f;
                }
                else
                {
                    _214 = true;
                }
                bool _220;
                if (!_214)
                {
                    _220 = _159.y > 1.0f;
                }
                else
                {
                    _220 = true;
                }
                if (_220)
                {
                    _229 = 0.0f;
                    break;
                }
                else
                {
                    _229 = water_mask.SampleLevel(sampler_default, _159, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            _231 = _229 > 0.25f;
        }
        else
        {
            _231 = true;
        }
        if (_231)
        {
            _448 = asfloat(0x7fbfffffu /* nan */).xxxx;
            _449 = 0.0f.xx;
            _450 = 0.0f.xxx;
            _451 = 0.0f.xxx;
            _452 = 0;
            _453 = 0.0f.xx;
            _454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 _237 = float4(in_var_Vertex_Position, 1.0f);
        _237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _159, 0.0f).x);
        float2 _246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 _251 = _246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 _253 = int2(floor(_251));
        float2 _254 = frac(_251);
        uint2 _255 = uint2(_253);
        uint _257 = _255.y;
        uint _259 = _257 + (_257 << 10u);
        uint _261 = _259 ^ (_259 >> 6u);
        uint _263 = _261 + (_261 << 3u);
        uint _265 = _263 ^ (_263 >> 11u);
        uint _271 = (((_255.x * 1664525u) + (_265 + (_265 << 15u))) + 1013904223u) * 1664525u;
        uint _273 = _271 ^ (_271 >> 11u);
        uint _276 = _273 ^ ((_273 << 7u) & 2636928640u);
        uint _279 = _276 ^ ((_276 << 15u) & 4022730752u);
        uint2 _287 = uint2(_253 + int2(1, 0));
        uint _289 = _287.y;
        uint _291 = _289 + (_289 << 10u);
        uint _293 = _291 ^ (_291 >> 6u);
        uint _295 = _293 + (_293 << 3u);
        uint _297 = _295 ^ (_295 >> 11u);
        uint _303 = (((_287.x * 1664525u) + (_297 + (_297 << 15u))) + 1013904223u) * 1664525u;
        uint _305 = _303 ^ (_303 >> 11u);
        uint _308 = _305 ^ ((_305 << 7u) & 2636928640u);
        uint _311 = _308 ^ ((_308 << 15u) & 4022730752u);
        uint2 _319 = uint2(_253 + int2(0, 1));
        uint _321 = _319.y;
        uint _323 = _321 + (_321 << 10u);
        uint _325 = _323 ^ (_323 >> 6u);
        uint _327 = _325 + (_325 << 3u);
        uint _329 = _327 ^ (_327 >> 11u);
        uint _335 = (((_319.x * 1664525u) + (_329 + (_329 << 15u))) + 1013904223u) * 1664525u;
        uint _337 = _335 ^ (_335 >> 11u);
        uint _340 = _337 ^ ((_337 << 7u) & 2636928640u);
        uint _343 = _340 ^ ((_340 << 15u) & 4022730752u);
        uint2 _351 = uint2(_253 + int2(1, 1));
        uint _353 = _351.y;
        uint _355 = _353 + (_353 << 10u);
        uint _357 = _355 ^ (_355 >> 6u);
        uint _359 = _357 + (_357 << 3u);
        uint _361 = _359 ^ (_359 >> 11u);
        uint _367 = (((_351.x * 1664525u) + (_361 + (_361 << 15u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        float2 _385 = (_254 * _254) * (3.0f.xx - (_254 * 2.0f));
        float _386 = _385.x;
        float _387 = 1.0f - _386;
        float _389 = _385.y;
        _237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - _389) * (((asfloat(((_279 ^ (_279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) + ((asfloat(((_311 ^ (_311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386))) + (((asfloat(((_343 ^ (_343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) * _389)) + (((asfloat(((_375 ^ (_375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386) * _389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _410 = grass_object_push.SampleLevel(sampler_default, _159, 0.0f);
        float4 _423;
        if (clamp(_410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 _421 = _237.xz + ((_410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            _423 = float4(_421.x, _237.y, _421.y, _237.w);
        }
        else
        {
            _423 = _237;
        }
        _448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(_423.xyz, 1.0f)));
        _449 = _246;
        _450 = _423.xyz;
        _451 = in_var_Vertex_Normal;
        _452 = in_var_stem;
        _453 = in_var_Vertex_UV;
        _454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = _448;
    out_var_C1 = _449;
    out_var_C2 = _450;
    out_var_C3 = _451;
    out_var_C5 = _452;
    out_var_C6 = _453;
    out_var_C7 = _454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

static float4 gl_FragCoord;
static float2 in_var_C1;
static float3 in_var_C3;
static int in_var_C5;
static float2 in_var_C6;
static float3x3 in_var_C7;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float2 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 in_var_C7 : TEXCOORD7;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _128 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_128.x + (_128.y * 2u)))
        {
            discard;
        }
    }
    float4 _141 = albedo_map.Sample(sampler_llr, in_var_C1);
    float3 _150;
    if (in_var_C5 == 1)
    {
        _150 = leafy_greens.Sample(sampler_llr, in_var_C1).xyz;
    }
    else
    {
        _150 = _141.xyz;
    }
    float3 _165;
    if (has_normal_map == 1)
    {
        _165 = normalize(mul(in_var_C7, (normal_map.Sample(sampler_llr, in_var_C6).xyz * 2.0f) - 1.0f.xxx));
    }
    else
    {
        _165 = in_var_C3;
    }
    float3 _167 = (_165 * 0.5f) + 0.5f.xxx;
    float3 _189 = sqrt(clamp(_150, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_167.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_167.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_167.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 2147483648u, ((uint((clamp(_189.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_189.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_189.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_meadow_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 _448;
    float2 _449;
    float3 _450;
    float3 _451;
    int _452;
    float2 _453;
    float3x3 _454;
    do
    {
        float _166;
        bool _168;
        float2 _159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _193;
        do
        {
            _166 = _159.x;
            _168 = !(_166 < 0.0f);
            bool _172;
            if (_168)
            {
                _172 = _166 > 1.0f;
            }
            else
            {
                _172 = true;
            }
            bool _178;
            if (!_172)
            {
                _178 = _159.y < 0.0f;
            }
            else
            {
                _178 = true;
            }
            bool _184;
            if (!_178)
            {
                _184 = _159.y > 1.0f;
            }
            else
            {
                _184 = true;
            }
            if (_184)
            {
                _193 = 0.0f;
                break;
            }
            else
            {
                _193 = rocky_terrain.SampleLevel(sampler_default, _159, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool _199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, _159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            _199 = _193 > 0.100000001490116119384765625f;
        }
        else
        {
            _199 = true;
        }
        bool _231;
        if (!_199)
        {
            float _229;
            do
            {
                bool _208;
                if (_168)
                {
                    _208 = _166 > 1.0f;
                }
                else
                {
                    _208 = true;
                }
                bool _214;
                if (!_208)
                {
                    _214 = _159.y < 0.0f;
                }
                else
                {
                    _214 = true;
                }
                bool _220;
                if (!_214)
                {
                    _220 = _159.y > 1.0f;
                }
                else
                {
                    _220 = true;
                }
                if (_220)
                {
                    _229 = 0.0f;
                    break;
                }
                else
                {
                    _229 = water_mask.SampleLevel(sampler_default, _159, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            _231 = _229 > 0.25f;
        }
        else
        {
            _231 = true;
        }
        if (_231)
        {
            _448 = asfloat(0x7fbfffffu /* nan */).xxxx;
            _449 = 0.0f.xx;
            _450 = 0.0f.xxx;
            _451 = 0.0f.xxx;
            _452 = 0;
            _453 = 0.0f.xx;
            _454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 _237 = float4(in_var_Vertex_Position, 1.0f);
        _237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _159, 0.0f).x);
        float2 _246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 _251 = _246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 _253 = int2(floor(_251));
        float2 _254 = frac(_251);
        uint2 _255 = uint2(_253);
        uint _257 = _255.y;
        uint _259 = _257 + (_257 << 10u);
        uint _261 = _259 ^ (_259 >> 6u);
        uint _263 = _261 + (_261 << 3u);
        uint _265 = _263 ^ (_263 >> 11u);
        uint _271 = (((_255.x * 1664525u) + (_265 + (_265 << 15u))) + 1013904223u) * 1664525u;
        uint _273 = _271 ^ (_271 >> 11u);
        uint _276 = _273 ^ ((_273 << 7u) & 2636928640u);
        uint _279 = _276 ^ ((_276 << 15u) & 4022730752u);
        uint2 _287 = uint2(_253 + int2(1, 0));
        uint _289 = _287.y;
        uint _291 = _289 + (_289 << 10u);
        uint _293 = _291 ^ (_291 >> 6u);
        uint _295 = _293 + (_293 << 3u);
        uint _297 = _295 ^ (_295 >> 11u);
        uint _303 = (((_287.x * 1664525u) + (_297 + (_297 << 15u))) + 1013904223u) * 1664525u;
        uint _305 = _303 ^ (_303 >> 11u);
        uint _308 = _305 ^ ((_305 << 7u) & 2636928640u);
        uint _311 = _308 ^ ((_308 << 15u) & 4022730752u);
        uint2 _319 = uint2(_253 + int2(0, 1));
        uint _321 = _319.y;
        uint _323 = _321 + (_321 << 10u);
        uint _325 = _323 ^ (_323 >> 6u);
        uint _327 = _325 + (_325 << 3u);
        uint _329 = _327 ^ (_327 >> 11u);
        uint _335 = (((_319.x * 1664525u) + (_329 + (_329 << 15u))) + 1013904223u) * 1664525u;
        uint _337 = _335 ^ (_335 >> 11u);
        uint _340 = _337 ^ ((_337 << 7u) & 2636928640u);
        uint _343 = _340 ^ ((_340 << 15u) & 4022730752u);
        uint2 _351 = uint2(_253 + int2(1, 1));
        uint _353 = _351.y;
        uint _355 = _353 + (_353 << 10u);
        uint _357 = _355 ^ (_355 >> 6u);
        uint _359 = _357 + (_357 << 3u);
        uint _361 = _359 ^ (_359 >> 11u);
        uint _367 = (((_351.x * 1664525u) + (_361 + (_361 << 15u))) + 1013904223u) * 1664525u;
        uint _369 = _367 ^ (_367 >> 11u);
        uint _372 = _369 ^ ((_369 << 7u) & 2636928640u);
        uint _375 = _372 ^ ((_372 << 15u) & 4022730752u);
        float2 _385 = (_254 * _254) * (3.0f.xx - (_254 * 2.0f));
        float _386 = _385.x;
        float _387 = 1.0f - _386;
        float _389 = _385.y;
        _237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - _389) * (((asfloat(((_279 ^ (_279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) + ((asfloat(((_311 ^ (_311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386))) + (((asfloat(((_343 ^ (_343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _387) * _389)) + (((asfloat(((_375 ^ (_375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _386) * _389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 _410 = grass_object_push.SampleLevel(sampler_default, _159, 0.0f);
        float4 _423;
        if (clamp(_410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 _421 = _237.xz + ((_410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            _423 = float4(_421.x, _237.y, _421.y, _237.w);
        }
        else
        {
            _423 = _237;
        }
        _448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(_423.xyz, 1.0f)));
        _449 = _246;
        _450 = _423.xyz;
        _451 = in_var_Vertex_Normal;
        _452 = in_var_stem;
        _453 = in_var_Vertex_UV;
        _454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = _448;
    out_var_C1 = _449;
    out_var_C2 = _450;
    out_var_C3 = _451;
    out_var_C5 = _452;
    out_var_C6 = _453;
    out_var_C7 = _454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_meadow_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> alpha_map : register(t17, space0);

static float4 gl_FragCoord;
static float2 in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 _99 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (_99.x + (_99.y * 2u)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
