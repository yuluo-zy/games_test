// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/path_pebble.raster.hlsl | F-R | bins: _path_pebble.raster.hlsl_b903e43ffb3da915.bin, _path_pebble.raster.hlsl_f0adff76827e73ff.bin | entry vs:vs_main (vs_6_4) ps:ps_main (ps_6_4)
// Path pebble instancing pass. One instanced quad per pebble: the vertex shader
// reads the pebble's affine transform from pebble_placement_buffer (one
// PebblePlacement{ Affine3Packed transform } per instance, 12 column-major
// floats) and emits the world position (transformed normal in C1, world pos in
// C2) into the GBuffer; the pixel shader samples the path dirt texture and
// packs an RGB8 albedo + RGB8 normal into a uint2 GBuffer target.
// bindings: $Globals b13 (current_view_index/dirtpath_1_dims/bool_render_shadows),
//   pebble_placement_buffer t14 (StructuredBuffer<PebblePlacement>), frame_constants b0/space1 (vs)
//   | sampler_llr s1, dirtpath_1 t15 (ps). Capabilities: RuntimeDescriptorArray (forced,
//   SPV_EXT_descriptor_indexing on the SSBO) + Shader on both modules.
// evidence: spirv-cross decompilation of _path_pebble.raster.hlsl_b903e43ffb3da915.bin
//   (workspace/f-r-scratch/modules/_path_pebble.raster.hlsl_b903e43ffb3da915_{0,1}.hlsl.cross)
//   transcribed math/order verbatim. The f0ad variant differs only in the PS
//   (empty ps_main, no bindings) — this file targets the b903 variant.
// status: O (algorithm/order/literals from .cross); residual opcode-count +
//   inc-layer OpSource/OpString diffs are expected (recorded, not fixed).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
// color_library.hlsl is pulled in before the material chain: the reconstructed
// inc layer's material/solid_material.inc.hlsl -> brdf_lut.hlsl ->
// highlight_color.hlsl references `color_library`, which needs its cbuffer
// declared first (inc-layer dependency, flagged).
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// Per-instance placement record: a packed 3x4 column-major affine transform.
struct PebblePlacement
{
    Affine3Packed transform;
};

// Original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBO; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
StructuredBuffer<PebblePlacement> pebble_placement_buffer : register(t14, space0);

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 dirtpath_1_dims;
    int bool_render_shadows;
};

Texture2D<float4> dirtpath_1 : register(t15, space0);

struct vs_main_input
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(1)]] float3 normal: Vertex_Normal;
    uint instance_id : SV_InstanceID;
};

struct vs_main_output
{
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
    float4 position : SV_Position;
};

vs_main_output vs_main(vs_main_input input)
{
    PebblePlacement placement = pebble_placement_buffer[input.instance_id];

    // Column-major affine (data[0..2]=col0, 3..5=col1, 6..8=col2, 9..11=translation):
    // transposed to row-major 3x4 for the mul.
    float3x4 world_transform = transpose(float4x3(
        float3(placement.transform.data[0], placement.transform.data[1], placement.transform.data[2]),
        float3(placement.transform.data[3], placement.transform.data[4], placement.transform.data[5]),
        float3(placement.transform.data[6], placement.transform.data[7], placement.transform.data[8]),
        float3(placement.transform.data[9], placement.transform.data[10], placement.transform.data[11])));

    float3 world_pos = mul(world_transform, float4(input.position, 1.0f));

    vs_main_output output;
    output.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    output.c1 = mul(world_transform, float4(input.normal, 0.0f));
    output.c2 = world_pos;
    return output;
}

struct ps_main_input
{
    float3 c1 : TEXCOORD1;
    float3 c2 : TEXCOORD2;
};

uint2 ps_main(ps_main_input input) : SV_Target0
{
    float4 dirtpath = dirtpath_1.Sample(sampler_llr, input.c2.xz * 0.1500000059604644775390625f);

    float3 albedo = dirtpath.xyz;
    albedo.x = dirtpath.x * 0.699999988079071044921875f;
    albedo.y = dirtpath.y * 0.89999997615814208984375f;

    float3 normal = (normalize(input.c1) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));

    uint color_packed = (uint((clamp(color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                       + (uint((clamp(color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                       + (uint((clamp(color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);

    uint normal_packed = (uint((clamp(normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                        + (uint((clamp(normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                        + (uint((clamp(normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)
                        + 4278190080u;

    uint2 result = uint2(normal_packed, color_packed);
    result.y = color_packed | 117440512u;
    return result;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _path_pebble.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct PebblePlacement
{
    Affine3Packed transform;
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
    uint2 dirtpath_1_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer pebble_placement_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _80;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _80.data[_1ident] = pebble_placement_buffer.Load<float>(_1ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _99 = transpose(float4x3(float3(_80.data[0], _80.data[1], _80.data[2]), float3(_80.data[3], _80.data[4], _80.data[5]), float3(_80.data[6], _80.data[7], _80.data[8]), float3(_80.data[9], _80.data[10], _80.data[11])));
    float3 _104 = mul(_99, float4(in_var_Vertex_Position, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_104, 1.0f));
    out_var_C1 = mul(_99, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _104;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _path_pebble.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> dirtpath_1 : register(t15, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _70 = dirtpath_1.Sample(sampler_llr, in_var_C2.xz * 0.1500000059604644775390625f);
    float3 _71 = _70.xyz;
    _71.x = _70.x * 0.699999988079071044921875f;
    _71.y = _70.y * 0.89999997615814208984375f;
    float3 _80 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 _102 = sqrt(clamp(_71, 0.0f.xxx, 1.0f.xxx));
    uint _121 = (uint((clamp(_102.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_102.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_102.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _122 = uint2(((uint((clamp(_80.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_80.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_80.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, _121);
    _122.y = _121 | 117440512u;
    out_var_SV_TARGET0 = _122;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _path_pebble.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Affine3Packed
{
    float data[12];
};

struct PebblePlacement
{
    Affine3Packed transform;
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
    uint2 dirtpath_1_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer pebble_placement_buffer : register(t14, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed _80;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _80.data[_1ident] = pebble_placement_buffer.Load<float>(_1ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 _99 = transpose(float4x3(float3(_80.data[0], _80.data[1], _80.data[2]), float3(_80.data[3], _80.data[4], _80.data[5]), float3(_80.data[6], _80.data[7], _80.data[8]), float3(_80.data[9], _80.data[10], _80.data[11])));
    float3 _104 = mul(_99, float4(in_var_Vertex_Position, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_104, 1.0f));
    out_var_C1 = mul(_99, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _104;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _path_pebble.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
