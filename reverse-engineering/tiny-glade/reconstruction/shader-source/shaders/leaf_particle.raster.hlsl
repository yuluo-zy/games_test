// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/leaf_particle.raster.hlsl | F-V | bins: _leaf_particle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Falling leaf particles: each instance carries a position / rotation / scale /
// up axis; the quad is oriented to the up axis with a horizontal alignment dir.
// ps discards outside the alpha texture and packs the solid gbuffer.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
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

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

// Leaf particle placement record (32-byte stride).
struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
};

StructuredBuffer<LeafParticle> leaf_placement_buffer : register(t14);
Texture2D<float4> leaf_alpha : register(t15);
Texture2D<float4> leaf_colors : register(t16);

uint2 leaf_alpha_dims;
uint2 leaf_colors_dims;
int bool_render_shadows;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float2 Vertex_UV : TEXCOORD1;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float3 p_pos = leaf_placement_buffer[vin.instance_id].position;
    float p_rot = leaf_placement_buffer[vin.instance_id].rotation;
    float p_scale = leaf_placement_buffer[vin.instance_id].scale;
    float3 p_up = leaf_placement_buffer[vin.instance_id].up_axis;
    float3 scale3 = p_scale.xxx * 0.7f;
    float3 dir = normalize(cross(p_up, normalize(float3(cos(p_rot), 0.0f, sin(p_rot)))));

    float4x4 basis = float4x4(float4(cross(dir, p_up), 0.0f),
                               float4(p_up, 0.0f),
                               float4(dir, 0.0f),
                               float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 m = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, p_pos.x),
                               float4(0.0f, 1.0f, 0.0f, p_pos.y),
                               float4(0.0f, 0.0f, 1.0f, p_pos.z),
                               float4(0.0f, 0.0f, 0.0f, 1.0f)),
                      mul(transpose(basis), float4x4(float4(scale3.x, 0.0f, 0.0f, 0.0f),
                                                     float4(0.0f, scale3.y, 0.0f, 0.0f),
                                                     float4(0.0f, 0.0f, scale3.z, 0.0f),
                                                     float4(0.0f, 0.0f, 0.0f, 1.0f)))),
                  float4(vin.Vertex_Position, 1.0f));

    o.position = mul(GetCurrentViewConstants().world_to_sample, m);
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = m.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = float2(p_up.x + scale3.x, p_up.z + scale3.x);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (leaf_alpha.SampleBias(sampler_llc, pin.C3, -2.0f).x < 1.0f)
    {
        discard;
    }
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 normal = pin.C1;
    if (dot(normalize(cam_pos - pin.C2), pin.C1) < 0.0f)
    {
        normal = -pin.C1;
    }
    float3 packed_normal = (normalize(normal) * 0.5f) + 0.5f;
    float3 albedo = sqrt(clamp(leaf_colors.SampleLevel(sampler_llr, pin.C4, 0.0f).xyz, 0.0f, 1.0f));
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 2566914048u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 1174405120u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _leaf_particle.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
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
    uint2 _Globals_leaf_alpha_dims : packoffset(c0.y);
    uint2 _Globals_leaf_colors_dims : packoffset(c1);
    int _Globals_bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer leaf_placement_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float2 in_var_Vertex_UV : TEXCOORD1;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _93 = leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 16).xxx * 0.699999988079071044921875f;
    float3 _101 = normalize(cross(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20), normalize(float3(cos(leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12)), 0.0f, sin(leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12))))));
    float _103 = _93.x;
    float4 _137 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).x), float4(0.0f, 1.0f, 0.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).y), float4(0.0f, 0.0f, 1.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(cross(_101, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20)), 0.0f), float4(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20), 0.0f), float4(_101, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_103, 0.0f, 0.0f, 0.0f), float4(0.0f, _93.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _93.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), float4(in_var_Vertex_Position, 1.0f));
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _137);
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _137.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20).x + _103, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20).z + _103);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _leaf_particle.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 _Globals_leaf_alpha_dims : packoffset(c0.y);
    uint2 _Globals_leaf_colors_dims : packoffset(c1);
    int _Globals_bool_render_shadows : packoffset(c1.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> leaf_alpha : register(t15, space0);
Texture2D<float4> leaf_colors : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (leaf_alpha.SampleBias(sampler_llc, in_var_C3, -2.0f).x < 1.0f)
    {
        discard;
    }
    float4 _104 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _116;
    if (dot(normalize((_104.xyz / _104.w.xxx) - in_var_C2), in_var_C1) < 0.0f)
    {
        _116 = -in_var_C1;
    }
    else
    {
        _116 = in_var_C1;
    }
    float3 _119 = (normalize(_116) * 0.5f) + 0.5f.xxx;
    float3 _141 = sqrt(clamp(leaf_colors.SampleLevel(sampler_llr, in_var_C4, 0.0f).xyz, 0.0f.xxx, 1.0f.xxx));
    uint _160 = (uint((clamp(_141.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_141.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_141.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _161 = uint2(((uint((clamp(_119.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_119.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_119.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 2566914048u, _160);
    _161.y = _160 | 1174405120u;
    out_var_SV_TARGET0 = _161;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _leaf_particle.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
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
    uint2 _Globals_leaf_alpha_dims : packoffset(c0.y);
    uint2 _Globals_leaf_colors_dims : packoffset(c1);
    int _Globals_bool_render_shadows : packoffset(c1.z);
};

ByteAddressBuffer leaf_placement_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float2 in_var_Vertex_UV : TEXCOORD1;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _93 = leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 16).xxx * 0.699999988079071044921875f;
    float3 _101 = normalize(cross(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20), normalize(float3(cos(leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12)), 0.0f, sin(leaf_placement_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12))))));
    float _103 = _93.x;
    float4 _137 = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).x), float4(0.0f, 1.0f, 0.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).y), float4(0.0f, 0.0f, 1.0f, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(transpose(float4x4(float4(cross(_101, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20)), 0.0f), float4(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20), 0.0f), float4(_101, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f))), float4x4(float4(_103, 0.0f, 0.0f, 0.0f), float4(0.0f, _93.y, 0.0f, 0.0f), float4(0.0f, 0.0f, _93.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)))), float4(in_var_Vertex_Position, 1.0f));
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _137);
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = _137.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20).x + _103, leaf_placement_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 20).z + _103);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _leaf_particle.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> leaf_alpha : register(t15, space0);

static float2 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
};

void frag_main()
{
    if (leaf_alpha.SampleBias(sampler_llc, in_var_C3, -2.0f).x < 1.0f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
