// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water_snow_plane.raster.hlsl
// SOURCE PATH : shaders/water_snow_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin  (ps writes uint2(0xFF80..) snow tint)
//   _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin  (ps: same samples/discards, no output interface)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs transform + ps snow-mask/perlin from spirv-cross)
// Evidence: spirv-cross vs (water plane at y=-0.01, xz*130 heightmap-space, world_to_sample;
//   C1 = world.xyz) and ps (water_mask.SampleLevel(sampler_llc) green channel vs 0.2;
//   perlin(C1.xz*1.5) variance discard; b903e writes const uint2 snow tint, f0adff no target).
// MERGE (water-ice review water_snow_plane.raster.hlsl): A adopted wholesale — exact
//   transcription (thresholds/tint uint2(4286644096,114677205)).
// Note: the two 16-hex variants are a compile-time feature variant — b903e = colored
//   snow plane, f0adff = no-output (depth/mask-only). Gated on WATER_SNOW_PLANE_COLOR (I).

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
    float4 wm = water_mask.SampleLevel(sampler_llc, (vin.C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }

    // 2D value-noise (perlin.hlsl) inlined with explicit bilinear arithmetic.
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

    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f,
                wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
    return uint2(4286644096u, 114677205u);
}
#else
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
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    uint2 _Globals_water_mask_dims : packoffset(c5);
    int _Globals_bool_render_shadows : packoffset(c5.z);
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
    float4 _72 = mul(_Globals_model_to_world, float4(in_var_Vertex_Position, 1.0f));
    float2 _74 = _72.xz * 130.0f.xx;
    float4 _75 = float4(_74.x, _72.y, _74.y, _72.w);
    _75.y = -0.00999999977648258209228515625f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _75);
    out_var_C1 = _75.xyz;
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
    float4 _95 = water_mask.SampleLevel(sampler_llc, (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _96 = _95.y;
    if (_96 < 0.20000000298023223876953125f)
    {
        discard;
    }
    float2 _100 = in_var_C1.xz * 1.5f;
    int2 _102 = int2(floor(_100));
    float2 _103 = frac(_100);
    uint2 _104 = uint2(_102);
    uint _106 = _104.y;
    uint _108 = _106 + (_106 << 10u);
    uint _110 = _108 ^ (_108 >> 6u);
    uint _112 = _110 + (_110 << 3u);
    uint _114 = _112 ^ (_112 >> 11u);
    uint _120 = (((_104.x * 1664525u) + (_114 + (_114 << 15u))) + 1013904223u) * 1664525u;
    uint _122 = _120 ^ (_120 >> 11u);
    uint _125 = _122 ^ ((_122 << 7u) & 2636928640u);
    uint _128 = _125 ^ ((_125 << 15u) & 4022730752u);
    uint2 _136 = uint2(_102 + int2(1, 0));
    uint _138 = _136.y;
    uint _140 = _138 + (_138 << 10u);
    uint _142 = _140 ^ (_140 >> 6u);
    uint _144 = _142 + (_142 << 3u);
    uint _146 = _144 ^ (_144 >> 11u);
    uint _152 = (((_136.x * 1664525u) + (_146 + (_146 << 15u))) + 1013904223u) * 1664525u;
    uint _154 = _152 ^ (_152 >> 11u);
    uint _157 = _154 ^ ((_154 << 7u) & 2636928640u);
    uint _160 = _157 ^ ((_157 << 15u) & 4022730752u);
    uint2 _168 = uint2(_102 + int2(0, 1));
    uint _170 = _168.y;
    uint _172 = _170 + (_170 << 10u);
    uint _174 = _172 ^ (_172 >> 6u);
    uint _176 = _174 + (_174 << 3u);
    uint _178 = _176 ^ (_176 >> 11u);
    uint _184 = (((_168.x * 1664525u) + (_178 + (_178 << 15u))) + 1013904223u) * 1664525u;
    uint _186 = _184 ^ (_184 >> 11u);
    uint _189 = _186 ^ ((_186 << 7u) & 2636928640u);
    uint _192 = _189 ^ ((_189 << 15u) & 4022730752u);
    uint2 _200 = uint2(_102 + int2(1, 1));
    uint _202 = _200.y;
    uint _204 = _202 + (_202 << 10u);
    uint _206 = _204 ^ (_204 >> 6u);
    uint _208 = _206 + (_206 << 3u);
    uint _210 = _208 ^ (_208 >> 11u);
    uint _216 = (((_200.x * 1664525u) + (_210 + (_210 << 15u))) + 1013904223u) * 1664525u;
    uint _218 = _216 ^ (_216 >> 11u);
    uint _221 = _218 ^ ((_218 << 7u) & 2636928640u);
    uint _224 = _221 ^ ((_221 << 15u) & 4022730752u);
    float2 _234 = (_103 * _103) * (3.0f.xx - (_103 * 2.0f));
    float _235 = _234.x;
    float _236 = 1.0f - _235;
    float _238 = _234.y;
    float _248 = (((1.0f - _238) * (((asfloat(((_128 ^ (_128 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _236) + ((asfloat(((_160 ^ (_160 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _235))) + (((asfloat(((_192 ^ (_192 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _236) * _238)) + (((asfloat(((_224 ^ (_224 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _235) * _238);
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f, _96 + ((_248 * _248) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
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
    uint _Globals_current_view_index : packoffset(c0);
    column_major float4x4 _Globals_model_to_world : packoffset(c1);
    uint2 _Globals_water_mask_dims : packoffset(c5);
    int _Globals_bool_render_shadows : packoffset(c5.z);
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
    float4 _72 = mul(_Globals_model_to_world, float4(in_var_Vertex_Position, 1.0f));
    float2 _74 = _72.xz * 130.0f.xx;
    float4 _75 = float4(_74.x, _72.y, _74.y, _72.w);
    _75.y = -0.00999999977648258209228515625f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _75);
    out_var_C1 = _75.xyz;
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
    float4 _90 = water_mask.SampleLevel(sampler_llc, (in_var_C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _91 = _90.y;
    if (_91 < 0.20000000298023223876953125f)
    {
        discard;
    }
    float2 _95 = in_var_C1.xz * 1.5f;
    int2 _97 = int2(floor(_95));
    float2 _98 = frac(_95);
    uint2 _99 = uint2(_97);
    uint _101 = _99.y;
    uint _103 = _101 + (_101 << 10u);
    uint _105 = _103 ^ (_103 >> 6u);
    uint _107 = _105 + (_105 << 3u);
    uint _109 = _107 ^ (_107 >> 11u);
    uint _115 = (((_99.x * 1664525u) + (_109 + (_109 << 15u))) + 1013904223u) * 1664525u;
    uint _117 = _115 ^ (_115 >> 11u);
    uint _120 = _117 ^ ((_117 << 7u) & 2636928640u);
    uint _123 = _120 ^ ((_120 << 15u) & 4022730752u);
    uint2 _131 = uint2(_97 + int2(1, 0));
    uint _133 = _131.y;
    uint _135 = _133 + (_133 << 10u);
    uint _137 = _135 ^ (_135 >> 6u);
    uint _139 = _137 + (_137 << 3u);
    uint _141 = _139 ^ (_139 >> 11u);
    uint _147 = (((_131.x * 1664525u) + (_141 + (_141 << 15u))) + 1013904223u) * 1664525u;
    uint _149 = _147 ^ (_147 >> 11u);
    uint _152 = _149 ^ ((_149 << 7u) & 2636928640u);
    uint _155 = _152 ^ ((_152 << 15u) & 4022730752u);
    uint2 _163 = uint2(_97 + int2(0, 1));
    uint _165 = _163.y;
    uint _167 = _165 + (_165 << 10u);
    uint _169 = _167 ^ (_167 >> 6u);
    uint _171 = _169 + (_169 << 3u);
    uint _173 = _171 ^ (_171 >> 11u);
    uint _179 = (((_163.x * 1664525u) + (_173 + (_173 << 15u))) + 1013904223u) * 1664525u;
    uint _181 = _179 ^ (_179 >> 11u);
    uint _184 = _181 ^ ((_181 << 7u) & 2636928640u);
    uint _187 = _184 ^ ((_184 << 15u) & 4022730752u);
    uint2 _195 = uint2(_97 + int2(1, 1));
    uint _197 = _195.y;
    uint _199 = _197 + (_197 << 10u);
    uint _201 = _199 ^ (_199 >> 6u);
    uint _203 = _201 + (_201 << 3u);
    uint _205 = _203 ^ (_203 >> 11u);
    uint _211 = (((_195.x * 1664525u) + (_205 + (_205 << 15u))) + 1013904223u) * 1664525u;
    uint _213 = _211 ^ (_211 >> 11u);
    uint _216 = _213 ^ ((_213 << 7u) & 2636928640u);
    uint _219 = _216 ^ ((_216 << 15u) & 4022730752u);
    float2 _229 = (_98 * _98) * (3.0f.xx - (_98 * 2.0f));
    float _230 = _229.x;
    float _231 = 1.0f - _230;
    float _233 = _229.y;
    float _243 = (((1.0f - _233) * (((asfloat(((_123 ^ (_123 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _231) + ((asfloat(((_155 ^ (_155 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _230))) + (((asfloat(((_187 ^ (_187 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _231) * _233)) + (((asfloat(((_219 ^ (_219 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _230) * _233);
    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f, _91 + ((_243 * _243) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
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
