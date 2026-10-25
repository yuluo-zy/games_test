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
    uint _Globals_current_view_index : packoffset(c0);
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
    float3 _131 = in_var_Vertex_Position;
    bool _141 = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 _159;
    if (_141)
    {
        float _152 = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 _155 = _131;
        _155.x = _131.x * _152;
        _155.z = _131.z * _152;
        _159 = _155;
    }
    else
    {
        _159 = _131;
    }
    float3 _172 = ((_159 * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 _174 = _172.xz * 0.5f;
    int2 _176 = int2(floor(_174));
    float2 _177 = frac(_174);
    uint2 _178 = uint2(_176);
    uint _180 = _178.y;
    uint _182 = _180 + (_180 << 10u);
    uint _184 = _182 ^ (_182 >> 6u);
    uint _186 = _184 + (_184 << 3u);
    uint _188 = _186 ^ (_186 >> 11u);
    uint _194 = (((_178.x * 1664525u) + (_188 + (_188 << 15u))) + 1013904223u) * 1664525u;
    uint _196 = _194 ^ (_194 >> 11u);
    uint _199 = _196 ^ ((_196 << 7u) & 2636928640u);
    uint _202 = _199 ^ ((_199 << 15u) & 4022730752u);
    uint2 _210 = uint2(_176 + int2(1, 0));
    uint _212 = _210.y;
    uint _214 = _212 + (_212 << 10u);
    uint _216 = _214 ^ (_214 >> 6u);
    uint _218 = _216 + (_216 << 3u);
    uint _220 = _218 ^ (_218 >> 11u);
    uint _226 = (((_210.x * 1664525u) + (_220 + (_220 << 15u))) + 1013904223u) * 1664525u;
    uint _228 = _226 ^ (_226 >> 11u);
    uint _231 = _228 ^ ((_228 << 7u) & 2636928640u);
    uint _234 = _231 ^ ((_231 << 15u) & 4022730752u);
    uint2 _242 = uint2(_176 + int2(0, 1));
    uint _244 = _242.y;
    uint _246 = _244 + (_244 << 10u);
    uint _248 = _246 ^ (_246 >> 6u);
    uint _250 = _248 + (_248 << 3u);
    uint _252 = _250 ^ (_250 >> 11u);
    uint _258 = (((_242.x * 1664525u) + (_252 + (_252 << 15u))) + 1013904223u) * 1664525u;
    uint _260 = _258 ^ (_258 >> 11u);
    uint _263 = _260 ^ ((_260 << 7u) & 2636928640u);
    uint _266 = _263 ^ ((_263 << 15u) & 4022730752u);
    uint2 _274 = uint2(_176 + int2(1, 1));
    uint _276 = _274.y;
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint _290 = (((_274.x * 1664525u) + (_284 + (_284 << 15u))) + 1013904223u) * 1664525u;
    uint _292 = _290 ^ (_290 >> 11u);
    uint _295 = _292 ^ ((_292 << 7u) & 2636928640u);
    uint _298 = _295 ^ ((_295 << 15u) & 4022730752u);
    float2 _308 = (_177 * _177) * (3.0f.xx - (_177 * 2.0f));
    float _309 = _308.x;
    float _310 = 1.0f - _309;
    float _312 = _308.y;
    _172.y = _172.y + (((((1.0f - _312) * (((asfloat(((_202 ^ (_202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) + ((asfloat(((_234 ^ (_234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309))) + (((asfloat(((_266 ^ (_266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) * _312)) + (((asfloat(((_298 ^ (_298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309) * _312)) * 0.300000011920928955078125f);
    float3 _327 = _172 * 0.4000000059604644775390625f;
    int3 _329 = int3(floor(_327));
    float3 _330 = frac(_327);
    uint3 _331 = uint3(_329);
    uint _334 = _331.z;
    uint _336 = _334 + (_334 << 10u);
    uint _338 = _336 ^ (_336 >> 6u);
    uint _340 = _338 + (_338 << 3u);
    uint _342 = _340 ^ (_340 >> 11u);
    uint _348 = (((_331.y * 1664525u) + (_342 + (_342 << 15u))) + 1013904223u) * 1664525u;
    uint _350 = _348 ^ (_348 >> 11u);
    uint _353 = _350 ^ ((_350 << 7u) & 2636928640u);
    uint _356 = _353 ^ ((_353 << 15u) & 4022730752u);
    uint _362 = (((_331.x * 1664525u) + (_356 ^ (_356 >> 18u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _372 = _370 ^ (_370 >> 18u);
    uint _374 = _372 + (_372 << 10u);
    uint _376 = _374 ^ (_374 >> 6u);
    uint _378 = _376 + (_376 << 3u);
    uint _380 = _378 ^ (_378 >> 11u);
    uint3 _393 = uint3(_329 + int3(1, 0, 0));
    uint _396 = _393.z;
    uint _398 = _396 + (_396 << 10u);
    uint _400 = _398 ^ (_398 >> 6u);
    uint _402 = _400 + (_400 << 3u);
    uint _404 = _402 ^ (_402 >> 11u);
    uint _410 = (((_393.y * 1664525u) + (_404 + (_404 << 15u))) + 1013904223u) * 1664525u;
    uint _412 = _410 ^ (_410 >> 11u);
    uint _415 = _412 ^ ((_412 << 7u) & 2636928640u);
    uint _418 = _415 ^ ((_415 << 15u) & 4022730752u);
    uint _424 = (((_393.x * 1664525u) + (_418 ^ (_418 >> 18u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _434 = _432 ^ (_432 >> 18u);
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint3 _455 = uint3(_329 + int3(0, 1, 0));
    uint _458 = _455.z;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_455.y * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint _486 = (((_455.x * 1664525u) + (_480 ^ (_480 >> 18u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _496 = _494 ^ (_494 >> 18u);
    uint _498 = _496 + (_496 << 10u);
    uint _500 = _498 ^ (_498 >> 6u);
    uint _502 = _500 + (_500 << 3u);
    uint _504 = _502 ^ (_502 >> 11u);
    uint3 _517 = uint3(_329 + int3(1, 1, 0));
    uint _520 = _517.z;
    uint _522 = _520 + (_520 << 10u);
    uint _524 = _522 ^ (_522 >> 6u);
    uint _526 = _524 + (_524 << 3u);
    uint _528 = _526 ^ (_526 >> 11u);
    uint _534 = (((_517.y * 1664525u) + (_528 + (_528 << 15u))) + 1013904223u) * 1664525u;
    uint _536 = _534 ^ (_534 >> 11u);
    uint _539 = _536 ^ ((_536 << 7u) & 2636928640u);
    uint _542 = _539 ^ ((_539 << 15u) & 4022730752u);
    uint _548 = (((_517.x * 1664525u) + (_542 ^ (_542 >> 18u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _558 = _556 ^ (_556 >> 18u);
    uint _560 = _558 + (_558 << 10u);
    uint _562 = _560 ^ (_560 >> 6u);
    uint _564 = _562 + (_562 << 3u);
    uint _566 = _564 ^ (_564 >> 11u);
    uint3 _579 = uint3(_329 + int3(0, 0, 1));
    uint _582 = _579.z;
    uint _584 = _582 + (_582 << 10u);
    uint _586 = _584 ^ (_584 >> 6u);
    uint _588 = _586 + (_586 << 3u);
    uint _590 = _588 ^ (_588 >> 11u);
    uint _596 = (((_579.y * 1664525u) + (_590 + (_590 << 15u))) + 1013904223u) * 1664525u;
    uint _598 = _596 ^ (_596 >> 11u);
    uint _601 = _598 ^ ((_598 << 7u) & 2636928640u);
    uint _604 = _601 ^ ((_601 << 15u) & 4022730752u);
    uint _610 = (((_579.x * 1664525u) + (_604 ^ (_604 >> 18u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _620 = _618 ^ (_618 >> 18u);
    uint _622 = _620 + (_620 << 10u);
    uint _624 = _622 ^ (_622 >> 6u);
    uint _626 = _624 + (_624 << 3u);
    uint _628 = _626 ^ (_626 >> 11u);
    uint3 _641 = uint3(_329 + int3(1, 0, 1));
    uint _644 = _641.z;
    uint _646 = _644 + (_644 << 10u);
    uint _648 = _646 ^ (_646 >> 6u);
    uint _650 = _648 + (_648 << 3u);
    uint _652 = _650 ^ (_650 >> 11u);
    uint _658 = (((_641.y * 1664525u) + (_652 + (_652 << 15u))) + 1013904223u) * 1664525u;
    uint _660 = _658 ^ (_658 >> 11u);
    uint _663 = _660 ^ ((_660 << 7u) & 2636928640u);
    uint _666 = _663 ^ ((_663 << 15u) & 4022730752u);
    uint _672 = (((_641.x * 1664525u) + (_666 ^ (_666 >> 18u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _682 = _680 ^ (_680 >> 18u);
    uint _684 = _682 + (_682 << 10u);
    uint _686 = _684 ^ (_684 >> 6u);
    uint _688 = _686 + (_686 << 3u);
    uint _690 = _688 ^ (_688 >> 11u);
    uint3 _703 = uint3(_329 + int3(0, 1, 1));
    uint _706 = _703.z;
    uint _708 = _706 + (_706 << 10u);
    uint _710 = _708 ^ (_708 >> 6u);
    uint _712 = _710 + (_710 << 3u);
    uint _714 = _712 ^ (_712 >> 11u);
    uint _720 = (((_703.y * 1664525u) + (_714 + (_714 << 15u))) + 1013904223u) * 1664525u;
    uint _722 = _720 ^ (_720 >> 11u);
    uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
    uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
    uint _734 = (((_703.x * 1664525u) + (_728 ^ (_728 >> 18u))) + 1013904223u) * 1664525u;
    uint _736 = _734 ^ (_734 >> 11u);
    uint _739 = _736 ^ ((_736 << 7u) & 2636928640u);
    uint _742 = _739 ^ ((_739 << 15u) & 4022730752u);
    uint _744 = _742 ^ (_742 >> 18u);
    uint _746 = _744 + (_744 << 10u);
    uint _748 = _746 ^ (_746 >> 6u);
    uint _750 = _748 + (_748 << 3u);
    uint _752 = _750 ^ (_750 >> 11u);
    uint3 _765 = uint3(_329 + int3(1, 1, 1));
    uint _768 = _765.z;
    uint _770 = _768 + (_768 << 10u);
    uint _772 = _770 ^ (_770 >> 6u);
    uint _774 = _772 + (_772 << 3u);
    uint _776 = _774 ^ (_774 >> 11u);
    uint _782 = (((_765.y * 1664525u) + (_776 + (_776 << 15u))) + 1013904223u) * 1664525u;
    uint _784 = _782 ^ (_782 >> 11u);
    uint _787 = _784 ^ ((_784 << 7u) & 2636928640u);
    uint _790 = _787 ^ ((_787 << 15u) & 4022730752u);
    uint _796 = (((_765.x * 1664525u) + (_790 ^ (_790 >> 18u))) + 1013904223u) * 1664525u;
    uint _798 = _796 ^ (_796 >> 11u);
    uint _801 = _798 ^ ((_798 << 7u) & 2636928640u);
    uint _804 = _801 ^ ((_801 << 15u) & 4022730752u);
    uint _806 = _804 ^ (_804 >> 18u);
    uint _808 = _806 + (_806 << 10u);
    uint _810 = _808 ^ (_808 >> 6u);
    uint _812 = _810 + (_810 << 3u);
    uint _814 = _812 ^ (_812 >> 11u);
    float3 _829 = (_330 * _330) * (3.0f.xxx - (_330 * 2.0f));
    float _830 = _829.x;
    float _831 = 1.0f - _830;
    float _833 = _829.y;
    float _834 = 1.0f - _833;
    float _836 = _829.z;
    float _837 = 1.0f - _836;
    float2 _854 = ((((((float2(asfloat((_372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_380 + (_380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _837) + (((float2(asfloat((_434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_442 + (_442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _837)) + (((float2(asfloat((_496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_504 + (_504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _837)) + (((float2(asfloat((_558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_566 + (_566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _837)) + (((float2(asfloat((_620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_628 + (_628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _836);
    float2 _869 = clamp(((((_854 + (((float2(asfloat((_682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_690 + (_690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _836)) + (((float2(asfloat((_744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_752 + (_752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _836)) + (((float2(asfloat((_806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_814 + (_814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _836)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _874 = _172 + (float3(_869.x, 0.0f, _869.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_874, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _874;
    out_var_C3 = float(_141);
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
    uint _Globals_current_view_index : packoffset(c0);
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
    float3 _131 = in_var_Vertex_Position;
    bool _141 = in_var_roof_profile_mult > (-0.001000000047497451305389404296875f);
    float3 _159;
    if (_141)
    {
        float _152 = max(0.60000002384185791015625f, ((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 24) * 2.0f) * lerp(0.60000002384185791015625f, 1.0f, in_var_roof_profile_mult)) * lerp(0.4000000059604644775390625f, 1.5f, 1.0f - instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 20)));
        float3 _155 = _131;
        _155.x = _131.x * _152;
        _155.z = _131.z * _152;
        _159 = _155;
    }
    else
    {
        _159 = _131;
    }
    float3 _172 = ((_159 * ((pow(2.0f, (-10.0f) * instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16)) * sin(((instanced_data.Load<float>(uint(gl_InstanceIndex) * 32 + 16) * 10.0f) - 0.75f) * 2.094395160675048828125f)) + 1.0f)) * 0.5f) + instanced_data.Load<float4>(uint(gl_InstanceIndex) * 32 + 0).xyz;
    float2 _174 = _172.xz * 0.5f;
    int2 _176 = int2(floor(_174));
    float2 _177 = frac(_174);
    uint2 _178 = uint2(_176);
    uint _180 = _178.y;
    uint _182 = _180 + (_180 << 10u);
    uint _184 = _182 ^ (_182 >> 6u);
    uint _186 = _184 + (_184 << 3u);
    uint _188 = _186 ^ (_186 >> 11u);
    uint _194 = (((_178.x * 1664525u) + (_188 + (_188 << 15u))) + 1013904223u) * 1664525u;
    uint _196 = _194 ^ (_194 >> 11u);
    uint _199 = _196 ^ ((_196 << 7u) & 2636928640u);
    uint _202 = _199 ^ ((_199 << 15u) & 4022730752u);
    uint2 _210 = uint2(_176 + int2(1, 0));
    uint _212 = _210.y;
    uint _214 = _212 + (_212 << 10u);
    uint _216 = _214 ^ (_214 >> 6u);
    uint _218 = _216 + (_216 << 3u);
    uint _220 = _218 ^ (_218 >> 11u);
    uint _226 = (((_210.x * 1664525u) + (_220 + (_220 << 15u))) + 1013904223u) * 1664525u;
    uint _228 = _226 ^ (_226 >> 11u);
    uint _231 = _228 ^ ((_228 << 7u) & 2636928640u);
    uint _234 = _231 ^ ((_231 << 15u) & 4022730752u);
    uint2 _242 = uint2(_176 + int2(0, 1));
    uint _244 = _242.y;
    uint _246 = _244 + (_244 << 10u);
    uint _248 = _246 ^ (_246 >> 6u);
    uint _250 = _248 + (_248 << 3u);
    uint _252 = _250 ^ (_250 >> 11u);
    uint _258 = (((_242.x * 1664525u) + (_252 + (_252 << 15u))) + 1013904223u) * 1664525u;
    uint _260 = _258 ^ (_258 >> 11u);
    uint _263 = _260 ^ ((_260 << 7u) & 2636928640u);
    uint _266 = _263 ^ ((_263 << 15u) & 4022730752u);
    uint2 _274 = uint2(_176 + int2(1, 1));
    uint _276 = _274.y;
    uint _278 = _276 + (_276 << 10u);
    uint _280 = _278 ^ (_278 >> 6u);
    uint _282 = _280 + (_280 << 3u);
    uint _284 = _282 ^ (_282 >> 11u);
    uint _290 = (((_274.x * 1664525u) + (_284 + (_284 << 15u))) + 1013904223u) * 1664525u;
    uint _292 = _290 ^ (_290 >> 11u);
    uint _295 = _292 ^ ((_292 << 7u) & 2636928640u);
    uint _298 = _295 ^ ((_295 << 15u) & 4022730752u);
    float2 _308 = (_177 * _177) * (3.0f.xx - (_177 * 2.0f));
    float _309 = _308.x;
    float _310 = 1.0f - _309;
    float _312 = _308.y;
    _172.y = _172.y + (((((1.0f - _312) * (((asfloat(((_202 ^ (_202 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) + ((asfloat(((_234 ^ (_234 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309))) + (((asfloat(((_266 ^ (_266 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _310) * _312)) + (((asfloat(((_298 ^ (_298 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _309) * _312)) * 0.300000011920928955078125f);
    float3 _327 = _172 * 0.4000000059604644775390625f;
    int3 _329 = int3(floor(_327));
    float3 _330 = frac(_327);
    uint3 _331 = uint3(_329);
    uint _334 = _331.z;
    uint _336 = _334 + (_334 << 10u);
    uint _338 = _336 ^ (_336 >> 6u);
    uint _340 = _338 + (_338 << 3u);
    uint _342 = _340 ^ (_340 >> 11u);
    uint _348 = (((_331.y * 1664525u) + (_342 + (_342 << 15u))) + 1013904223u) * 1664525u;
    uint _350 = _348 ^ (_348 >> 11u);
    uint _353 = _350 ^ ((_350 << 7u) & 2636928640u);
    uint _356 = _353 ^ ((_353 << 15u) & 4022730752u);
    uint _362 = (((_331.x * 1664525u) + (_356 ^ (_356 >> 18u))) + 1013904223u) * 1664525u;
    uint _364 = _362 ^ (_362 >> 11u);
    uint _367 = _364 ^ ((_364 << 7u) & 2636928640u);
    uint _370 = _367 ^ ((_367 << 15u) & 4022730752u);
    uint _372 = _370 ^ (_370 >> 18u);
    uint _374 = _372 + (_372 << 10u);
    uint _376 = _374 ^ (_374 >> 6u);
    uint _378 = _376 + (_376 << 3u);
    uint _380 = _378 ^ (_378 >> 11u);
    uint3 _393 = uint3(_329 + int3(1, 0, 0));
    uint _396 = _393.z;
    uint _398 = _396 + (_396 << 10u);
    uint _400 = _398 ^ (_398 >> 6u);
    uint _402 = _400 + (_400 << 3u);
    uint _404 = _402 ^ (_402 >> 11u);
    uint _410 = (((_393.y * 1664525u) + (_404 + (_404 << 15u))) + 1013904223u) * 1664525u;
    uint _412 = _410 ^ (_410 >> 11u);
    uint _415 = _412 ^ ((_412 << 7u) & 2636928640u);
    uint _418 = _415 ^ ((_415 << 15u) & 4022730752u);
    uint _424 = (((_393.x * 1664525u) + (_418 ^ (_418 >> 18u))) + 1013904223u) * 1664525u;
    uint _426 = _424 ^ (_424 >> 11u);
    uint _429 = _426 ^ ((_426 << 7u) & 2636928640u);
    uint _432 = _429 ^ ((_429 << 15u) & 4022730752u);
    uint _434 = _432 ^ (_432 >> 18u);
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint3 _455 = uint3(_329 + int3(0, 1, 0));
    uint _458 = _455.z;
    uint _460 = _458 + (_458 << 10u);
    uint _462 = _460 ^ (_460 >> 6u);
    uint _464 = _462 + (_462 << 3u);
    uint _466 = _464 ^ (_464 >> 11u);
    uint _472 = (((_455.y * 1664525u) + (_466 + (_466 << 15u))) + 1013904223u) * 1664525u;
    uint _474 = _472 ^ (_472 >> 11u);
    uint _477 = _474 ^ ((_474 << 7u) & 2636928640u);
    uint _480 = _477 ^ ((_477 << 15u) & 4022730752u);
    uint _486 = (((_455.x * 1664525u) + (_480 ^ (_480 >> 18u))) + 1013904223u) * 1664525u;
    uint _488 = _486 ^ (_486 >> 11u);
    uint _491 = _488 ^ ((_488 << 7u) & 2636928640u);
    uint _494 = _491 ^ ((_491 << 15u) & 4022730752u);
    uint _496 = _494 ^ (_494 >> 18u);
    uint _498 = _496 + (_496 << 10u);
    uint _500 = _498 ^ (_498 >> 6u);
    uint _502 = _500 + (_500 << 3u);
    uint _504 = _502 ^ (_502 >> 11u);
    uint3 _517 = uint3(_329 + int3(1, 1, 0));
    uint _520 = _517.z;
    uint _522 = _520 + (_520 << 10u);
    uint _524 = _522 ^ (_522 >> 6u);
    uint _526 = _524 + (_524 << 3u);
    uint _528 = _526 ^ (_526 >> 11u);
    uint _534 = (((_517.y * 1664525u) + (_528 + (_528 << 15u))) + 1013904223u) * 1664525u;
    uint _536 = _534 ^ (_534 >> 11u);
    uint _539 = _536 ^ ((_536 << 7u) & 2636928640u);
    uint _542 = _539 ^ ((_539 << 15u) & 4022730752u);
    uint _548 = (((_517.x * 1664525u) + (_542 ^ (_542 >> 18u))) + 1013904223u) * 1664525u;
    uint _550 = _548 ^ (_548 >> 11u);
    uint _553 = _550 ^ ((_550 << 7u) & 2636928640u);
    uint _556 = _553 ^ ((_553 << 15u) & 4022730752u);
    uint _558 = _556 ^ (_556 >> 18u);
    uint _560 = _558 + (_558 << 10u);
    uint _562 = _560 ^ (_560 >> 6u);
    uint _564 = _562 + (_562 << 3u);
    uint _566 = _564 ^ (_564 >> 11u);
    uint3 _579 = uint3(_329 + int3(0, 0, 1));
    uint _582 = _579.z;
    uint _584 = _582 + (_582 << 10u);
    uint _586 = _584 ^ (_584 >> 6u);
    uint _588 = _586 + (_586 << 3u);
    uint _590 = _588 ^ (_588 >> 11u);
    uint _596 = (((_579.y * 1664525u) + (_590 + (_590 << 15u))) + 1013904223u) * 1664525u;
    uint _598 = _596 ^ (_596 >> 11u);
    uint _601 = _598 ^ ((_598 << 7u) & 2636928640u);
    uint _604 = _601 ^ ((_601 << 15u) & 4022730752u);
    uint _610 = (((_579.x * 1664525u) + (_604 ^ (_604 >> 18u))) + 1013904223u) * 1664525u;
    uint _612 = _610 ^ (_610 >> 11u);
    uint _615 = _612 ^ ((_612 << 7u) & 2636928640u);
    uint _618 = _615 ^ ((_615 << 15u) & 4022730752u);
    uint _620 = _618 ^ (_618 >> 18u);
    uint _622 = _620 + (_620 << 10u);
    uint _624 = _622 ^ (_622 >> 6u);
    uint _626 = _624 + (_624 << 3u);
    uint _628 = _626 ^ (_626 >> 11u);
    uint3 _641 = uint3(_329 + int3(1, 0, 1));
    uint _644 = _641.z;
    uint _646 = _644 + (_644 << 10u);
    uint _648 = _646 ^ (_646 >> 6u);
    uint _650 = _648 + (_648 << 3u);
    uint _652 = _650 ^ (_650 >> 11u);
    uint _658 = (((_641.y * 1664525u) + (_652 + (_652 << 15u))) + 1013904223u) * 1664525u;
    uint _660 = _658 ^ (_658 >> 11u);
    uint _663 = _660 ^ ((_660 << 7u) & 2636928640u);
    uint _666 = _663 ^ ((_663 << 15u) & 4022730752u);
    uint _672 = (((_641.x * 1664525u) + (_666 ^ (_666 >> 18u))) + 1013904223u) * 1664525u;
    uint _674 = _672 ^ (_672 >> 11u);
    uint _677 = _674 ^ ((_674 << 7u) & 2636928640u);
    uint _680 = _677 ^ ((_677 << 15u) & 4022730752u);
    uint _682 = _680 ^ (_680 >> 18u);
    uint _684 = _682 + (_682 << 10u);
    uint _686 = _684 ^ (_684 >> 6u);
    uint _688 = _686 + (_686 << 3u);
    uint _690 = _688 ^ (_688 >> 11u);
    uint3 _703 = uint3(_329 + int3(0, 1, 1));
    uint _706 = _703.z;
    uint _708 = _706 + (_706 << 10u);
    uint _710 = _708 ^ (_708 >> 6u);
    uint _712 = _710 + (_710 << 3u);
    uint _714 = _712 ^ (_712 >> 11u);
    uint _720 = (((_703.y * 1664525u) + (_714 + (_714 << 15u))) + 1013904223u) * 1664525u;
    uint _722 = _720 ^ (_720 >> 11u);
    uint _725 = _722 ^ ((_722 << 7u) & 2636928640u);
    uint _728 = _725 ^ ((_725 << 15u) & 4022730752u);
    uint _734 = (((_703.x * 1664525u) + (_728 ^ (_728 >> 18u))) + 1013904223u) * 1664525u;
    uint _736 = _734 ^ (_734 >> 11u);
    uint _739 = _736 ^ ((_736 << 7u) & 2636928640u);
    uint _742 = _739 ^ ((_739 << 15u) & 4022730752u);
    uint _744 = _742 ^ (_742 >> 18u);
    uint _746 = _744 + (_744 << 10u);
    uint _748 = _746 ^ (_746 >> 6u);
    uint _750 = _748 + (_748 << 3u);
    uint _752 = _750 ^ (_750 >> 11u);
    uint3 _765 = uint3(_329 + int3(1, 1, 1));
    uint _768 = _765.z;
    uint _770 = _768 + (_768 << 10u);
    uint _772 = _770 ^ (_770 >> 6u);
    uint _774 = _772 + (_772 << 3u);
    uint _776 = _774 ^ (_774 >> 11u);
    uint _782 = (((_765.y * 1664525u) + (_776 + (_776 << 15u))) + 1013904223u) * 1664525u;
    uint _784 = _782 ^ (_782 >> 11u);
    uint _787 = _784 ^ ((_784 << 7u) & 2636928640u);
    uint _790 = _787 ^ ((_787 << 15u) & 4022730752u);
    uint _796 = (((_765.x * 1664525u) + (_790 ^ (_790 >> 18u))) + 1013904223u) * 1664525u;
    uint _798 = _796 ^ (_796 >> 11u);
    uint _801 = _798 ^ ((_798 << 7u) & 2636928640u);
    uint _804 = _801 ^ ((_801 << 15u) & 4022730752u);
    uint _806 = _804 ^ (_804 >> 18u);
    uint _808 = _806 + (_806 << 10u);
    uint _810 = _808 ^ (_808 >> 6u);
    uint _812 = _810 + (_810 << 3u);
    uint _814 = _812 ^ (_812 >> 11u);
    float3 _829 = (_330 * _330) * (3.0f.xxx - (_330 * 2.0f));
    float _830 = _829.x;
    float _831 = 1.0f - _830;
    float _833 = _829.y;
    float _834 = 1.0f - _833;
    float _836 = _829.z;
    float _837 = 1.0f - _836;
    float2 _854 = ((((((float2(asfloat((_372 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_380 + (_380 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _837) + (((float2(asfloat((_434 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_442 + (_442 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _837)) + (((float2(asfloat((_496 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_504 + (_504 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _837)) + (((float2(asfloat((_558 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_566 + (_566 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _837)) + (((float2(asfloat((_620 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_628 + (_628 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _834) * _836);
    float2 _869 = clamp(((((_854 + (((float2(asfloat((_682 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_690 + (_690 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _834) * _836)) + (((float2(asfloat((_744 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_752 + (_752 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _831) * _833) * _836)) + (((float2(asfloat((_806 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_814 + (_814 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _830) * _833) * _836)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _874 = _172 + (float3(_869.x, 0.0f, _869.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_874, 1.0f));
    out_var_C0 = float3(0.2067500054836273193359375f, 0.1769330203533172607421875f, 0.10665000975131988525390625f);
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _874;
    out_var_C3 = float(_141);
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
