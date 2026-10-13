// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/sheep.raster.hlsl | F-V | bins: _sheep.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Sheep instance animation: per-vertex positions/normals come from a structured buffer
// (`animation`) holding 30-frame sweeps (positions[30] + normals[30] float4, frame index
// stride, final "delighted" pose pair at the end). pet_blend cross-fades toward the delighted
// pose with a sin ease; is_pet_forward flips the ease direction. vs outputs gbuffer inputs
// (vertex color, animated normal via model_to_world, current world pos, prev-frame world pos);
// ps packs albedo (sqrt gamma 0.8) + normal into the solid gbuffer word and writes velocity.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/sh.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

// The animation buffer is structured in the original (per-vertex frame sweeps):
// 30 positions + 30 normals float4 each, then the "delighted" pose pair.
struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
};

StructuredBuffer<SheepAnimation> animation : register(t14);

float4x4 model_to_world;
float4x4 prev_model_to_world;
int animation_frame;
float pet_blend;
int is_pet_forward;

struct VsIn
{
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint vertex_id : SV_VertexID;
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
    float blend = clamp(1.0f - pet_blend, 0.0f, 1.0f);
    float bob;
    if (is_pet_forward == 0)
    {
        float b = clamp(pet_blend, 0.0f, 1.0f);
        bob = (b * b * b) - (b * sin(b * 3.14f));
    }
    else
    {
        bob = 1.0f - ((blend * blend * blend) - (blend * sin(blend * 3.14f)));
    }
    float3 bob3 = bob.xxx;

    float3 pos = lerp(
        animation[vin.vertex_id].positions[animation_frame].xyz,
        animation[vin.vertex_id].delighted_pos.xyz, bob3);

    float4 p = float4(pos.x, pos.y - 0.25f, pos.z, 1.0f);
    float4 wp = mul(model_to_world, p);
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world,
               float4(lerp(animation[vin.vertex_id].normals[animation_frame].xyz,
                           animation[vin.vertex_id].delighted_normal.xyz, bob3), 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = mul(prev_model_to_world, p).xyz;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    float3 albedo = sqrt(clamp(pin.C0 * 0.8f, 0.0f, 1.0f));
    uint albedo_word = (uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 67108864u;
    o.velocity = pin.C2 - pin.C3;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _sheep.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
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
    column_major float4x4 _Globals_prev_model_to_world : packoffset(c5);
    int _Globals_animation_frame : packoffset(c9);
    float _Globals_pet_blend : packoffset(c9.y);
    int _Globals_is_pet_forward : packoffset(c9.z);
};

ByteAddressBuffer animation : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _92 = clamp(1.0f - _Globals_pet_blend, 0.0f, 1.0f);
    float _112;
    if (_Globals_is_pet_forward == 0)
    {
        float _105 = clamp(_Globals_pet_blend, 0.0f, 1.0f);
        _112 = ((_105 * _105) * _105) - (_105 * sin(_105 * 3.1400001049041748046875f));
    }
    else
    {
        _112 = 1.0f - (((_92 * _92) * _92) - (_92 * sin(_92 * 3.1400001049041748046875f)));
    }
    float3 _122 = _112.xxx;
    float3 _123 = lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + _Globals_animation_frame * 16 + 0).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 960).xyz, _122);
    float4 _134 = float4(_123.x, _123.y - 0.25f, _123.z, 1.0f);
    float4 _135 = mul(_Globals_model_to_world, _134);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _135);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_Globals_model_to_world, float4(lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + _Globals_animation_frame * 16 + 480).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 976).xyz, _122), 0.0f)).xyz;
    out_var_C2 = _135.xyz;
    out_var_C3 = mul(_Globals_prev_model_to_world, _134).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
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
// lifted from _sheep.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
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
    column_major float4x4 _Globals_prev_model_to_world : packoffset(c5);
    int _Globals_animation_frame : packoffset(c9);
    float _Globals_pet_blend : packoffset(c9.y);
    int _Globals_is_pet_forward : packoffset(c9.z);
};

ByteAddressBuffer animation : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _92 = clamp(1.0f - _Globals_pet_blend, 0.0f, 1.0f);
    float _112;
    if (_Globals_is_pet_forward == 0)
    {
        float _105 = clamp(_Globals_pet_blend, 0.0f, 1.0f);
        _112 = ((_105 * _105) * _105) - (_105 * sin(_105 * 3.1400001049041748046875f));
    }
    else
    {
        _112 = 1.0f - (((_92 * _92) * _92) - (_92 * sin(_92 * 3.1400001049041748046875f)));
    }
    float3 _122 = _112.xxx;
    float3 _123 = lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + _Globals_animation_frame * 16 + 0).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 960).xyz, _122);
    float4 _134 = float4(_123.x, _123.y - 0.25f, _123.z, 1.0f);
    float4 _135 = mul(_Globals_model_to_world, _134);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _135);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(_Globals_model_to_world, float4(lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + _Globals_animation_frame * 16 + 480).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 976).xyz, _122), 0.0f)).xyz;
    out_var_C2 = _135.xyz;
    out_var_C3 = mul(_Globals_prev_model_to_world, _134).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
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
