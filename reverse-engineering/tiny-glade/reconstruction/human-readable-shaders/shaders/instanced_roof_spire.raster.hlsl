// ============================================================
// instanced_roof_spire.raster.hlsl
// 功能:实例化屋顶尖塔的顶点着色器(两个 TGV 变体为同一代码)。
// 顶点逻辑:
//   1. 依 roof_profile_mult 决定是否做锥形收窄(xz 按高度缩放);
//   2. 叠加屋顶轮廓(2^x 曲线)与实例位移得到世界坐标;
//   3. 用两层噪声(xz 2x2 晶格 + 3D 2x2x2 晶格)做轻微风力摆动,
//      幅度 0.06,输出到 SV_Position 与 C0..C3。
// ============================================================
// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/instanced_roof_spire.raster.hlsl
// SOURCE PATH : shaders/instanced_roof_spire.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin
//   _instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin
//   pass/stage   : _instanced_roof_spire (roof) / raster
//   entry        : vs:vs_main; ps:ps_main
//   names        : ViewConstants, frame_constants, $Globals, RoofSpire, instanced_data, vs_main, ps_main
// Status: O (structure from disassembly: per-instance 32-byte RoofSpire record,
//   roof-profile-gated xz scaling, back-ease-out spire rise, a 2D value-noise
//   (inc/perlin) bend of world.y, an inline 3D float2 value noise for xz jitter,
//   constant dark-brown albedo; PS blends the vertex normal toward the derived
//   face normal and packs the solid gbuffer). I (variable naming, helper names —
//   the 3D float2 noise is inline in the original, no inc file provides it).
//   $Globals (b13) carries only current_view_index; the full inc/samplers.hlsl
//   set reserves set0 bindings 0..12, so the plain global auto-assigns to b13.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color_library.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Per-instance roof spire record (32 bytes).
struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
};

ByteAddressBuffer instanced_data : register(t15, space0);

// Two-component hash of a 3D lattice cell (the 3D-adapted form of the waves
// 2-component hash: z-avalanche -> y LCG mix -> x LCG mix, then the x/y hash01
// pair). Returns a float2 in [0,1).
float2 spire_cell_hash01_2(uint3 cell)
{
    uint z = cell.z;
    z += (z << 10u);
    z ^= (z >> 6u);
    z += (z << 3u);
    z ^= (z >> 11u);

    uint h1 = ((cell.y * 1664525u) + (z + (z << 15u))) + 1013904223u;
    h1 *= 1664525u;
    h1 ^= (h1 >> 11u);
    h1 ^= (h1 << 7u) & 2636928640u;
    h1 ^= (h1 << 15u) & 4022730752u;

    uint h2 = ((cell.x * 1664525u) + (h1 ^ (h1 >> 18u))) + 1013904223u;
    h2 *= 1664525u;
    h2 ^= (h2 >> 11u);
    h2 ^= (h2 << 7u) & 2636928640u;
    h2 ^= (h2 << 15u) & 4022730752u;

    uint xv = h2 ^ (h2 >> 18u);
    uint yv = xv;
    yv += (yv << 10u);
    yv ^= (yv >> 6u);
    yv += (yv << 3u);
    yv ^= (yv >> 11u);

    return float2(asfloat((xv & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((yv + (yv << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// Inline 3D value noise (8-lattice trilinear blend of the 2-component cell hash)
// with a clamped [-0.5, 0.5] output.
float2 spire_noise3(float3 p)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);

    float2 n = spire_cell_hash01_2(uint3(cell)) * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 0))) * u.x * (1.0f - u.y) * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 0))) * (1.0f - u.x) * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 0))) * u.x * u.y * (1.0f - u.z);
    n += spire_cell_hash01_2(uint3(cell + int3(0, 0, 1))) * (1.0f - u.x) * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 0, 1))) * u.x * (1.0f - u.y) * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(0, 1, 1))) * (1.0f - u.x) * u.y * u.z;
    n += spire_cell_hash01_2(uint3(cell + int3(1, 1, 1))) * u.x * u.y * u.z;

    return clamp((n - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float roof_profile_mult: roof_profile_mult;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint base = vin.instance_id * 32u;
    float roof_t = instanced_data.Load<float>(base + 16u);
    float roof_profile = instanced_data.Load<float>(base + 20u);
    float radius = instanced_data.Load<float>(base + 24u);

    // Spire profile gate: only vertices tagged with a profile multiplier bulge
    // horizontally.
    bool is_profile = vin.roof_profile_mult > -0.001f;
    float3 scaled_pos = vin.Vertex_Position;
    if (is_profile)
    {
        float s = max(0.6f, (radius * 2.0f) * lerp(0.6f, 1.0f, vin.roof_profile_mult)
            * lerp(0.4f, 1.5f, 1.0f - roof_t));
        scaled_pos = float3(vin.Vertex_Position.x * s, vin.Vertex_Position.y, vin.Vertex_Position.z * s);
    }

    // Back-ease-out rise: (2^-10t * sin((10t - 0.75) * 2PI/3) + 1), halved.
    float3 world = ((scaled_pos * ((pow(2.0f, (-10.0f) * roof_t)
        * sin(((roof_t * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f)
                 + instanced_data.Load<float4>(base + 0u).xyz;

    // 2D value-noise bend of world y, then the 3D float2 jitter.
    float3 wpos = world;
    wpos.y = world.y + perlin(world.xz * 0.5f) * 0.3f;
    float2 n3 = spire_noise3(wpos * 0.4f);
    float3 disp = wpos + (float3(n3.x, 0.0f, n3.y) * 0.06f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(disp, 1.0f));
    o.C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    o.C1 = vin.Vertex_Normal;
    o.C2 = disp;
    o.C3 = float(is_profile);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 face_normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    float3 normal = normalize(lerp(pin.C1, float3(pin.C1.x, face_normal.y, pin.C1.z), pin.C3));
    float3 albedo = sqrt(clamp(pin.C0 * 0.5f, 0.0f, 1.0f));

    uint normal_word = uint(clamp((normal.x * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp((normal.y * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp((normal.z * 0.5f) + 0.5f, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3003121664u;
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 117440512u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _instanced_roof_spire.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
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
};

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_roof_profile_mult;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float in_var_roof_profile_mult: roof_profile_mult;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 vertex_pos = in_var_Vertex_Position;
    bool has_profile = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 scaled_pos;
    if (has_profile)
    {
        float taper_scale = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 tapered_pos = vertex_pos;
        tapered_pos.x = vertex_pos.x * taper_scale;
        tapered_pos.z = vertex_pos.z * taper_scale;
        scaled_pos = tapered_pos;
    }
    else
    {
        scaled_pos = vertex_pos;
    }
    float3 world_pos = ((scaled_pos * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 v174 = world_pos.xz * 0.5f;
    int2 v176 = int2(floor(v174));
    float2 v177 = frac(v174);
    uint2 v178 = uint2(v176);
    uint v180 = v178.y;
    uint v182 = v180 + (v180 << 10u);
    uint v184 = v182 ^ (v182 >> 6u);
    uint v186 = v184 + (v184 << 3u);
    uint v188 = v186 ^ (v186 >> 11u);
    uint v194 = (((v178.x * 1664525u) + (v188 + (v188 << 15u))) + 1013904223u) * 1664525u;
    uint v196 = v194 ^ (v194 >> 11u);
    uint v199 = v196 ^ ((v196 << 7u) & 2636928640u);
    uint v202 = v199 ^ ((v199 << 15u) & 4022730752u);
    uint2 v210 = uint2(v176 + int2(1, 0));
    uint v212 = v210.y;
    uint v214 = v212 + (v212 << 10u);
    uint v216 = v214 ^ (v214 >> 6u);
    uint v218 = v216 + (v216 << 3u);
    uint v220 = v218 ^ (v218 >> 11u);
    uint v226 = (((v210.x * 1664525u) + (v220 + (v220 << 15u))) + 1013904223u) * 1664525u;
    uint v228 = v226 ^ (v226 >> 11u);
    uint v231 = v228 ^ ((v228 << 7u) & 2636928640u);
    uint v234 = v231 ^ ((v231 << 15u) & 4022730752u);
    uint2 v242 = uint2(v176 + int2(0, 1));
    uint v244 = v242.y;
    uint v246 = v244 + (v244 << 10u);
    uint v248 = v246 ^ (v246 >> 6u);
    uint v250 = v248 + (v248 << 3u);
    uint v252 = v250 ^ (v250 >> 11u);
    uint v258 = (((v242.x * 1664525u) + (v252 + (v252 << 15u))) + 1013904223u) * 1664525u;
    uint v260 = v258 ^ (v258 >> 11u);
    uint v263 = v260 ^ ((v260 << 7u) & 2636928640u);
    uint v266 = v263 ^ ((v263 << 15u) & 4022730752u);
    uint2 v274 = uint2(v176 + int2(1, 1));
    uint v276 = v274.y;
    uint v278 = v276 + (v276 << 10u);
    uint v280 = v278 ^ (v278 >> 6u);
    uint v282 = v280 + (v280 << 3u);
    uint v284 = v282 ^ (v282 >> 11u);
    uint v290 = (((v274.x * 1664525u) + (v284 + (v284 << 15u))) + 1013904223u) * 1664525u;
    uint v292 = v290 ^ (v290 >> 11u);
    uint v295 = v292 ^ ((v292 << 7u) & 2636928640u);
    uint v298 = v295 ^ ((v295 << 15u) & 4022730752u);
    float2 v308 = (v177 * v177) * (3.0f.xx - (v177 * 2.0f));
    float v309 = v308.x;
    float v310 = 1.0f - v309;
    float v312 = v308.y;
    world_pos.y = world_pos.y + (((((1.0f - v312) * (((asfloat(((v202 ^ (v202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v310) + ((asfloat(((v234 ^ (v234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v309))) + (((asfloat(((v266 ^ (v266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v310) * v312)) + (((asfloat(((v298 ^ (v298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v309) * v312)) * 0.300000011920928955078125f);
    float3 noise_coord = world_pos * 0.4000000059604644775390625f;
    int3 v329 = int3(floor(noise_coord));
    float3 v330 = frac(noise_coord);
    uint3 v331 = uint3(v329);
    uint v334 = v331.z;
    uint v336 = v334 + (v334 << 10u);
    uint v338 = v336 ^ (v336 >> 6u);
    uint v340 = v338 + (v338 << 3u);
    uint v342 = v340 ^ (v340 >> 11u);
    uint v348 = (((v331.y * 1664525u) + (v342 + (v342 << 15u))) + 1013904223u) * 1664525u;
    uint v350 = v348 ^ (v348 >> 11u);
    uint v353 = v350 ^ ((v350 << 7u) & 2636928640u);
    uint v356 = v353 ^ ((v353 << 15u) & 4022730752u);
    uint v362 = (((v331.x * 1664525u) + (v356 ^ (v356 >> 18u))) + 1013904223u) * 1664525u;
    uint v364 = v362 ^ (v362 >> 11u);
    uint v367 = v364 ^ ((v364 << 7u) & 2636928640u);
    uint v370 = v367 ^ ((v367 << 15u) & 4022730752u);
    uint v372 = v370 ^ (v370 >> 18u);
    uint v374 = v372 + (v372 << 10u);
    uint v376 = v374 ^ (v374 >> 6u);
    uint v378 = v376 + (v376 << 3u);
    uint v380 = v378 ^ (v378 >> 11u);
    uint3 v393 = uint3(v329 + int3(1, 0, 0));
    uint v396 = v393.z;
    uint v398 = v396 + (v396 << 10u);
    uint v400 = v398 ^ (v398 >> 6u);
    uint v402 = v400 + (v400 << 3u);
    uint v404 = v402 ^ (v402 >> 11u);
    uint v410 = (((v393.y * 1664525u) + (v404 + (v404 << 15u))) + 1013904223u) * 1664525u;
    uint v412 = v410 ^ (v410 >> 11u);
    uint v415 = v412 ^ ((v412 << 7u) & 2636928640u);
    uint v418 = v415 ^ ((v415 << 15u) & 4022730752u);
    uint v424 = (((v393.x * 1664525u) + (v418 ^ (v418 >> 18u))) + 1013904223u) * 1664525u;
    uint v426 = v424 ^ (v424 >> 11u);
    uint v429 = v426 ^ ((v426 << 7u) & 2636928640u);
    uint v432 = v429 ^ ((v429 << 15u) & 4022730752u);
    uint v434 = v432 ^ (v432 >> 18u);
    uint v436 = v434 + (v434 << 10u);
    uint v438 = v436 ^ (v436 >> 6u);
    uint v440 = v438 + (v438 << 3u);
    uint v442 = v440 ^ (v440 >> 11u);
    uint3 v455 = uint3(v329 + int3(0, 1, 0));
    uint v458 = v455.z;
    uint v460 = v458 + (v458 << 10u);
    uint v462 = v460 ^ (v460 >> 6u);
    uint v464 = v462 + (v462 << 3u);
    uint v466 = v464 ^ (v464 >> 11u);
    uint v472 = (((v455.y * 1664525u) + (v466 + (v466 << 15u))) + 1013904223u) * 1664525u;
    uint v474 = v472 ^ (v472 >> 11u);
    uint v477 = v474 ^ ((v474 << 7u) & 2636928640u);
    uint v480 = v477 ^ ((v477 << 15u) & 4022730752u);
    uint v486 = (((v455.x * 1664525u) + (v480 ^ (v480 >> 18u))) + 1013904223u) * 1664525u;
    uint v488 = v486 ^ (v486 >> 11u);
    uint v491 = v488 ^ ((v488 << 7u) & 2636928640u);
    uint v494 = v491 ^ ((v491 << 15u) & 4022730752u);
    uint v496 = v494 ^ (v494 >> 18u);
    uint v498 = v496 + (v496 << 10u);
    uint v500 = v498 ^ (v498 >> 6u);
    uint v502 = v500 + (v500 << 3u);
    uint v504 = v502 ^ (v502 >> 11u);
    uint3 v517 = uint3(v329 + int3(1, 1, 0));
    uint v520 = v517.z;
    uint v522 = v520 + (v520 << 10u);
    uint v524 = v522 ^ (v522 >> 6u);
    uint v526 = v524 + (v524 << 3u);
    uint v528 = v526 ^ (v526 >> 11u);
    uint v534 = (((v517.y * 1664525u) + (v528 + (v528 << 15u))) + 1013904223u) * 1664525u;
    uint v536 = v534 ^ (v534 >> 11u);
    uint v539 = v536 ^ ((v536 << 7u) & 2636928640u);
    uint v542 = v539 ^ ((v539 << 15u) & 4022730752u);
    uint v548 = (((v517.x * 1664525u) + (v542 ^ (v542 >> 18u))) + 1013904223u) * 1664525u;
    uint v550 = v548 ^ (v548 >> 11u);
    uint v553 = v550 ^ ((v550 << 7u) & 2636928640u);
    uint v556 = v553 ^ ((v553 << 15u) & 4022730752u);
    uint v558 = v556 ^ (v556 >> 18u);
    uint v560 = v558 + (v558 << 10u);
    uint v562 = v560 ^ (v560 >> 6u);
    uint v564 = v562 + (v562 << 3u);
    uint v566 = v564 ^ (v564 >> 11u);
    uint3 v579 = uint3(v329 + int3(0, 0, 1));
    uint v582 = v579.z;
    uint v584 = v582 + (v582 << 10u);
    uint v586 = v584 ^ (v584 >> 6u);
    uint v588 = v586 + (v586 << 3u);
    uint v590 = v588 ^ (v588 >> 11u);
    uint v596 = (((v579.y * 1664525u) + (v590 + (v590 << 15u))) + 1013904223u) * 1664525u;
    uint v598 = v596 ^ (v596 >> 11u);
    uint v601 = v598 ^ ((v598 << 7u) & 2636928640u);
    uint v604 = v601 ^ ((v601 << 15u) & 4022730752u);
    uint v610 = (((v579.x * 1664525u) + (v604 ^ (v604 >> 18u))) + 1013904223u) * 1664525u;
    uint v612 = v610 ^ (v610 >> 11u);
    uint v615 = v612 ^ ((v612 << 7u) & 2636928640u);
    uint v618 = v615 ^ ((v615 << 15u) & 4022730752u);
    uint v620 = v618 ^ (v618 >> 18u);
    uint v622 = v620 + (v620 << 10u);
    uint v624 = v622 ^ (v622 >> 6u);
    uint v626 = v624 + (v624 << 3u);
    uint v628 = v626 ^ (v626 >> 11u);
    uint3 v641 = uint3(v329 + int3(1, 0, 1));
    uint v644 = v641.z;
    uint v646 = v644 + (v644 << 10u);
    uint v648 = v646 ^ (v646 >> 6u);
    uint v650 = v648 + (v648 << 3u);
    uint v652 = v650 ^ (v650 >> 11u);
    uint v658 = (((v641.y * 1664525u) + (v652 + (v652 << 15u))) + 1013904223u) * 1664525u;
    uint v660 = v658 ^ (v658 >> 11u);
    uint v663 = v660 ^ ((v660 << 7u) & 2636928640u);
    uint v666 = v663 ^ ((v663 << 15u) & 4022730752u);
    uint v672 = (((v641.x * 1664525u) + (v666 ^ (v666 >> 18u))) + 1013904223u) * 1664525u;
    uint v674 = v672 ^ (v672 >> 11u);
    uint v677 = v674 ^ ((v674 << 7u) & 2636928640u);
    uint v680 = v677 ^ ((v677 << 15u) & 4022730752u);
    uint v682 = v680 ^ (v680 >> 18u);
    uint v684 = v682 + (v682 << 10u);
    uint v686 = v684 ^ (v684 >> 6u);
    uint v688 = v686 + (v686 << 3u);
    uint v690 = v688 ^ (v688 >> 11u);
    uint3 v703 = uint3(v329 + int3(0, 1, 1));
    uint v706 = v703.z;
    uint v708 = v706 + (v706 << 10u);
    uint v710 = v708 ^ (v708 >> 6u);
    uint v712 = v710 + (v710 << 3u);
    uint v714 = v712 ^ (v712 >> 11u);
    uint v720 = (((v703.y * 1664525u) + (v714 + (v714 << 15u))) + 1013904223u) * 1664525u;
    uint v722 = v720 ^ (v720 >> 11u);
    uint v725 = v722 ^ ((v722 << 7u) & 2636928640u);
    uint v728 = v725 ^ ((v725 << 15u) & 4022730752u);
    uint v734 = (((v703.x * 1664525u) + (v728 ^ (v728 >> 18u))) + 1013904223u) * 1664525u;
    uint v736 = v734 ^ (v734 >> 11u);
    uint v739 = v736 ^ ((v736 << 7u) & 2636928640u);
    uint v742 = v739 ^ ((v739 << 15u) & 4022730752u);
    uint v744 = v742 ^ (v742 >> 18u);
    uint v746 = v744 + (v744 << 10u);
    uint v748 = v746 ^ (v746 >> 6u);
    uint v750 = v748 + (v748 << 3u);
    uint v752 = v750 ^ (v750 >> 11u);
    uint3 v765 = uint3(v329 + int3(1, 1, 1));
    uint v768 = v765.z;
    uint v770 = v768 + (v768 << 10u);
    uint v772 = v770 ^ (v770 >> 6u);
    uint v774 = v772 + (v772 << 3u);
    uint v776 = v774 ^ (v774 >> 11u);
    uint v782 = (((v765.y * 1664525u) + (v776 + (v776 << 15u))) + 1013904223u) * 1664525u;
    uint v784 = v782 ^ (v782 >> 11u);
    uint v787 = v784 ^ ((v784 << 7u) & 2636928640u);
    uint v790 = v787 ^ ((v787 << 15u) & 4022730752u);
    uint v796 = (((v765.x * 1664525u) + (v790 ^ (v790 >> 18u))) + 1013904223u) * 1664525u;
    uint v798 = v796 ^ (v796 >> 11u);
    uint v801 = v798 ^ ((v798 << 7u) & 2636928640u);
    uint v804 = v801 ^ ((v801 << 15u) & 4022730752u);
    uint v806 = v804 ^ (v804 >> 18u);
    uint v808 = v806 + (v806 << 10u);
    uint v810 = v808 ^ (v808 >> 6u);
    uint v812 = v810 + (v810 << 3u);
    uint v814 = v812 ^ (v812 >> 11u);
    float3 noise_w = (v330 * v330) * (3.0f.xxx - (v330 * 2.0f));
    float wx = noise_w.x;
    float wx_inv = 1.0f - wx;
    float wy = noise_w.y;
    float wy_inv = 1.0f - wy;
    float wz = noise_w.z;
    float wz_inv = 1.0f - wz;
    float2 noise_grad = ((((((float2(asfloat((v372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v380 + (v380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy_inv) * wz_inv) + (((float2(asfloat((v434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v442 + (v442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy_inv) * wz_inv)) + (((float2(asfloat((v496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v504 + (v504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy) * wz_inv)) + (((float2(asfloat((v558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v566 + (v566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz_inv)) + (((float2(asfloat((v620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v628 + (v628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy_inv) * wz);
    float2 wind_offset = clamp(((((noise_grad + (((float2(asfloat((v682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v690 + (v690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy_inv) * wz)) + (((float2(asfloat((v744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v752 + (v752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy) * wz)) + (((float2(asfloat((v806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v814 + (v814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 final_pos = world_pos + (float3(wind_offset.x, 0.0f, wind_offset.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(final_pos, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = final_pos;
    out_var_C3 = float(has_profile);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_roof_profile_mult = stage_input.in_var_roof_profile_mult;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _instanced_roof_spire.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct RoofSpire
{
    float4 position;
    float roof_animation_t;
    float roof_profile;
    float radius;
    int pad1;
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
};

ByteAddressBuffer instanced_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_roof_profile_mult;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float in_var_roof_profile_mult: roof_profile_mult;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 vertex_pos = in_var_Vertex_Position;
    bool has_profile = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 scaled_pos;
    if (has_profile)
    {
        float taper_scale = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 tapered_pos = vertex_pos;
        tapered_pos.x = vertex_pos.x * taper_scale;
        tapered_pos.z = vertex_pos.z * taper_scale;
        scaled_pos = tapered_pos;
    }
    else
    {
        scaled_pos = vertex_pos;
    }
    float3 world_pos = ((scaled_pos * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 v174 = world_pos.xz * 0.5f;
    int2 v176 = int2(floor(v174));
    float2 v177 = frac(v174);
    uint2 v178 = uint2(v176);
    uint v180 = v178.y;
    uint v182 = v180 + (v180 << 10u);
    uint v184 = v182 ^ (v182 >> 6u);
    uint v186 = v184 + (v184 << 3u);
    uint v188 = v186 ^ (v186 >> 11u);
    uint v194 = (((v178.x * 1664525u) + (v188 + (v188 << 15u))) + 1013904223u) * 1664525u;
    uint v196 = v194 ^ (v194 >> 11u);
    uint v199 = v196 ^ ((v196 << 7u) & 2636928640u);
    uint v202 = v199 ^ ((v199 << 15u) & 4022730752u);
    uint2 v210 = uint2(v176 + int2(1, 0));
    uint v212 = v210.y;
    uint v214 = v212 + (v212 << 10u);
    uint v216 = v214 ^ (v214 >> 6u);
    uint v218 = v216 + (v216 << 3u);
    uint v220 = v218 ^ (v218 >> 11u);
    uint v226 = (((v210.x * 1664525u) + (v220 + (v220 << 15u))) + 1013904223u) * 1664525u;
    uint v228 = v226 ^ (v226 >> 11u);
    uint v231 = v228 ^ ((v228 << 7u) & 2636928640u);
    uint v234 = v231 ^ ((v231 << 15u) & 4022730752u);
    uint2 v242 = uint2(v176 + int2(0, 1));
    uint v244 = v242.y;
    uint v246 = v244 + (v244 << 10u);
    uint v248 = v246 ^ (v246 >> 6u);
    uint v250 = v248 + (v248 << 3u);
    uint v252 = v250 ^ (v250 >> 11u);
    uint v258 = (((v242.x * 1664525u) + (v252 + (v252 << 15u))) + 1013904223u) * 1664525u;
    uint v260 = v258 ^ (v258 >> 11u);
    uint v263 = v260 ^ ((v260 << 7u) & 2636928640u);
    uint v266 = v263 ^ ((v263 << 15u) & 4022730752u);
    uint2 v274 = uint2(v176 + int2(1, 1));
    uint v276 = v274.y;
    uint v278 = v276 + (v276 << 10u);
    uint v280 = v278 ^ (v278 >> 6u);
    uint v282 = v280 + (v280 << 3u);
    uint v284 = v282 ^ (v282 >> 11u);
    uint v290 = (((v274.x * 1664525u) + (v284 + (v284 << 15u))) + 1013904223u) * 1664525u;
    uint v292 = v290 ^ (v290 >> 11u);
    uint v295 = v292 ^ ((v292 << 7u) & 2636928640u);
    uint v298 = v295 ^ ((v295 << 15u) & 4022730752u);
    float2 v308 = (v177 * v177) * (3.0f.xx - (v177 * 2.0f));
    float v309 = v308.x;
    float v310 = 1.0f - v309;
    float v312 = v308.y;
    world_pos.y = world_pos.y + (((((1.0f - v312) * (((asfloat(((v202 ^ (v202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v310) + ((asfloat(((v234 ^ (v234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v309))) + (((asfloat(((v266 ^ (v266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v310) * v312)) + (((asfloat(((v298 ^ (v298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v309) * v312)) * 0.300000011920928955078125f);
    float3 noise_coord = world_pos * 0.4000000059604644775390625f;
    int3 v329 = int3(floor(noise_coord));
    float3 v330 = frac(noise_coord);
    uint3 v331 = uint3(v329);
    uint v334 = v331.z;
    uint v336 = v334 + (v334 << 10u);
    uint v338 = v336 ^ (v336 >> 6u);
    uint v340 = v338 + (v338 << 3u);
    uint v342 = v340 ^ (v340 >> 11u);
    uint v348 = (((v331.y * 1664525u) + (v342 + (v342 << 15u))) + 1013904223u) * 1664525u;
    uint v350 = v348 ^ (v348 >> 11u);
    uint v353 = v350 ^ ((v350 << 7u) & 2636928640u);
    uint v356 = v353 ^ ((v353 << 15u) & 4022730752u);
    uint v362 = (((v331.x * 1664525u) + (v356 ^ (v356 >> 18u))) + 1013904223u) * 1664525u;
    uint v364 = v362 ^ (v362 >> 11u);
    uint v367 = v364 ^ ((v364 << 7u) & 2636928640u);
    uint v370 = v367 ^ ((v367 << 15u) & 4022730752u);
    uint v372 = v370 ^ (v370 >> 18u);
    uint v374 = v372 + (v372 << 10u);
    uint v376 = v374 ^ (v374 >> 6u);
    uint v378 = v376 + (v376 << 3u);
    uint v380 = v378 ^ (v378 >> 11u);
    uint3 v393 = uint3(v329 + int3(1, 0, 0));
    uint v396 = v393.z;
    uint v398 = v396 + (v396 << 10u);
    uint v400 = v398 ^ (v398 >> 6u);
    uint v402 = v400 + (v400 << 3u);
    uint v404 = v402 ^ (v402 >> 11u);
    uint v410 = (((v393.y * 1664525u) + (v404 + (v404 << 15u))) + 1013904223u) * 1664525u;
    uint v412 = v410 ^ (v410 >> 11u);
    uint v415 = v412 ^ ((v412 << 7u) & 2636928640u);
    uint v418 = v415 ^ ((v415 << 15u) & 4022730752u);
    uint v424 = (((v393.x * 1664525u) + (v418 ^ (v418 >> 18u))) + 1013904223u) * 1664525u;
    uint v426 = v424 ^ (v424 >> 11u);
    uint v429 = v426 ^ ((v426 << 7u) & 2636928640u);
    uint v432 = v429 ^ ((v429 << 15u) & 4022730752u);
    uint v434 = v432 ^ (v432 >> 18u);
    uint v436 = v434 + (v434 << 10u);
    uint v438 = v436 ^ (v436 >> 6u);
    uint v440 = v438 + (v438 << 3u);
    uint v442 = v440 ^ (v440 >> 11u);
    uint3 v455 = uint3(v329 + int3(0, 1, 0));
    uint v458 = v455.z;
    uint v460 = v458 + (v458 << 10u);
    uint v462 = v460 ^ (v460 >> 6u);
    uint v464 = v462 + (v462 << 3u);
    uint v466 = v464 ^ (v464 >> 11u);
    uint v472 = (((v455.y * 1664525u) + (v466 + (v466 << 15u))) + 1013904223u) * 1664525u;
    uint v474 = v472 ^ (v472 >> 11u);
    uint v477 = v474 ^ ((v474 << 7u) & 2636928640u);
    uint v480 = v477 ^ ((v477 << 15u) & 4022730752u);
    uint v486 = (((v455.x * 1664525u) + (v480 ^ (v480 >> 18u))) + 1013904223u) * 1664525u;
    uint v488 = v486 ^ (v486 >> 11u);
    uint v491 = v488 ^ ((v488 << 7u) & 2636928640u);
    uint v494 = v491 ^ ((v491 << 15u) & 4022730752u);
    uint v496 = v494 ^ (v494 >> 18u);
    uint v498 = v496 + (v496 << 10u);
    uint v500 = v498 ^ (v498 >> 6u);
    uint v502 = v500 + (v500 << 3u);
    uint v504 = v502 ^ (v502 >> 11u);
    uint3 v517 = uint3(v329 + int3(1, 1, 0));
    uint v520 = v517.z;
    uint v522 = v520 + (v520 << 10u);
    uint v524 = v522 ^ (v522 >> 6u);
    uint v526 = v524 + (v524 << 3u);
    uint v528 = v526 ^ (v526 >> 11u);
    uint v534 = (((v517.y * 1664525u) + (v528 + (v528 << 15u))) + 1013904223u) * 1664525u;
    uint v536 = v534 ^ (v534 >> 11u);
    uint v539 = v536 ^ ((v536 << 7u) & 2636928640u);
    uint v542 = v539 ^ ((v539 << 15u) & 4022730752u);
    uint v548 = (((v517.x * 1664525u) + (v542 ^ (v542 >> 18u))) + 1013904223u) * 1664525u;
    uint v550 = v548 ^ (v548 >> 11u);
    uint v553 = v550 ^ ((v550 << 7u) & 2636928640u);
    uint v556 = v553 ^ ((v553 << 15u) & 4022730752u);
    uint v558 = v556 ^ (v556 >> 18u);
    uint v560 = v558 + (v558 << 10u);
    uint v562 = v560 ^ (v560 >> 6u);
    uint v564 = v562 + (v562 << 3u);
    uint v566 = v564 ^ (v564 >> 11u);
    uint3 v579 = uint3(v329 + int3(0, 0, 1));
    uint v582 = v579.z;
    uint v584 = v582 + (v582 << 10u);
    uint v586 = v584 ^ (v584 >> 6u);
    uint v588 = v586 + (v586 << 3u);
    uint v590 = v588 ^ (v588 >> 11u);
    uint v596 = (((v579.y * 1664525u) + (v590 + (v590 << 15u))) + 1013904223u) * 1664525u;
    uint v598 = v596 ^ (v596 >> 11u);
    uint v601 = v598 ^ ((v598 << 7u) & 2636928640u);
    uint v604 = v601 ^ ((v601 << 15u) & 4022730752u);
    uint v610 = (((v579.x * 1664525u) + (v604 ^ (v604 >> 18u))) + 1013904223u) * 1664525u;
    uint v612 = v610 ^ (v610 >> 11u);
    uint v615 = v612 ^ ((v612 << 7u) & 2636928640u);
    uint v618 = v615 ^ ((v615 << 15u) & 4022730752u);
    uint v620 = v618 ^ (v618 >> 18u);
    uint v622 = v620 + (v620 << 10u);
    uint v624 = v622 ^ (v622 >> 6u);
    uint v626 = v624 + (v624 << 3u);
    uint v628 = v626 ^ (v626 >> 11u);
    uint3 v641 = uint3(v329 + int3(1, 0, 1));
    uint v644 = v641.z;
    uint v646 = v644 + (v644 << 10u);
    uint v648 = v646 ^ (v646 >> 6u);
    uint v650 = v648 + (v648 << 3u);
    uint v652 = v650 ^ (v650 >> 11u);
    uint v658 = (((v641.y * 1664525u) + (v652 + (v652 << 15u))) + 1013904223u) * 1664525u;
    uint v660 = v658 ^ (v658 >> 11u);
    uint v663 = v660 ^ ((v660 << 7u) & 2636928640u);
    uint v666 = v663 ^ ((v663 << 15u) & 4022730752u);
    uint v672 = (((v641.x * 1664525u) + (v666 ^ (v666 >> 18u))) + 1013904223u) * 1664525u;
    uint v674 = v672 ^ (v672 >> 11u);
    uint v677 = v674 ^ ((v674 << 7u) & 2636928640u);
    uint v680 = v677 ^ ((v677 << 15u) & 4022730752u);
    uint v682 = v680 ^ (v680 >> 18u);
    uint v684 = v682 + (v682 << 10u);
    uint v686 = v684 ^ (v684 >> 6u);
    uint v688 = v686 + (v686 << 3u);
    uint v690 = v688 ^ (v688 >> 11u);
    uint3 v703 = uint3(v329 + int3(0, 1, 1));
    uint v706 = v703.z;
    uint v708 = v706 + (v706 << 10u);
    uint v710 = v708 ^ (v708 >> 6u);
    uint v712 = v710 + (v710 << 3u);
    uint v714 = v712 ^ (v712 >> 11u);
    uint v720 = (((v703.y * 1664525u) + (v714 + (v714 << 15u))) + 1013904223u) * 1664525u;
    uint v722 = v720 ^ (v720 >> 11u);
    uint v725 = v722 ^ ((v722 << 7u) & 2636928640u);
    uint v728 = v725 ^ ((v725 << 15u) & 4022730752u);
    uint v734 = (((v703.x * 1664525u) + (v728 ^ (v728 >> 18u))) + 1013904223u) * 1664525u;
    uint v736 = v734 ^ (v734 >> 11u);
    uint v739 = v736 ^ ((v736 << 7u) & 2636928640u);
    uint v742 = v739 ^ ((v739 << 15u) & 4022730752u);
    uint v744 = v742 ^ (v742 >> 18u);
    uint v746 = v744 + (v744 << 10u);
    uint v748 = v746 ^ (v746 >> 6u);
    uint v750 = v748 + (v748 << 3u);
    uint v752 = v750 ^ (v750 >> 11u);
    uint3 v765 = uint3(v329 + int3(1, 1, 1));
    uint v768 = v765.z;
    uint v770 = v768 + (v768 << 10u);
    uint v772 = v770 ^ (v770 >> 6u);
    uint v774 = v772 + (v772 << 3u);
    uint v776 = v774 ^ (v774 >> 11u);
    uint v782 = (((v765.y * 1664525u) + (v776 + (v776 << 15u))) + 1013904223u) * 1664525u;
    uint v784 = v782 ^ (v782 >> 11u);
    uint v787 = v784 ^ ((v784 << 7u) & 2636928640u);
    uint v790 = v787 ^ ((v787 << 15u) & 4022730752u);
    uint v796 = (((v765.x * 1664525u) + (v790 ^ (v790 >> 18u))) + 1013904223u) * 1664525u;
    uint v798 = v796 ^ (v796 >> 11u);
    uint v801 = v798 ^ ((v798 << 7u) & 2636928640u);
    uint v804 = v801 ^ ((v801 << 15u) & 4022730752u);
    uint v806 = v804 ^ (v804 >> 18u);
    uint v808 = v806 + (v806 << 10u);
    uint v810 = v808 ^ (v808 >> 6u);
    uint v812 = v810 + (v810 << 3u);
    uint v814 = v812 ^ (v812 >> 11u);
    float3 noise_w = (v330 * v330) * (3.0f.xxx - (v330 * 2.0f));
    float wx = noise_w.x;
    float wx_inv = 1.0f - wx;
    float wy = noise_w.y;
    float wy_inv = 1.0f - wy;
    float wz = noise_w.z;
    float wz_inv = 1.0f - wz;
    float2 noise_grad = ((((((float2(asfloat((v372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v380 + (v380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy_inv) * wz_inv) + (((float2(asfloat((v434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v442 + (v442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy_inv) * wz_inv)) + (((float2(asfloat((v496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v504 + (v504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy) * wz_inv)) + (((float2(asfloat((v558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v566 + (v566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz_inv)) + (((float2(asfloat((v620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v628 + (v628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy_inv) * wz);
    float2 wind_offset = clamp(((((noise_grad + (((float2(asfloat((v682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v690 + (v690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy_inv) * wz)) + (((float2(asfloat((v744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v752 + (v752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx_inv) * wy) * wz)) + (((float2(asfloat((v806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v814 + (v814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * wx) * wy) * wz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 final_pos = world_pos + (float3(wind_offset.x, 0.0f, wind_offset.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(final_pos, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = final_pos;
    out_var_C3 = float(has_profile);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_roof_profile_mult = stage_input.in_var_roof_profile_mult;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
