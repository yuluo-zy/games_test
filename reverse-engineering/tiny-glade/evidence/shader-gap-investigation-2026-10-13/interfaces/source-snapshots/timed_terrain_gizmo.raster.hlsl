// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/timed_terrain_gizmo.raster.hlsl | evidence: spirv-cross of _timed_terrain_gizmo.raster.hlsl_* (vs: arrow_index selects y/x axis stretch, world_to_sample output in sample space; ps: passthrough C1->SV_Target0); raw-per-bin $Globals = current_view_index/model_to_world/y_axis_stretch/x_axis_stretch b13 | status: O (bindings/entry/logic); I (names)

#include "inc/common.hlsl"
// current_view.hlsl is pulled in for its include closure; its global
// current_view_index is redeclared in our $Globals cbuffer below, so the
// include body is guard-skipped (the file's OpString is still recorded).
#define TINYGLADE_INC_CURRENT_VIEW_HLSL
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

// Active-view index + gizmo transform constants (single $Globals UBO at b13).
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    float y_axis_stretch;
    float x_axis_stretch;
};

struct VS_INPUT
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(1)]] int arrow_index: arrow_index;
};

struct VS_OUTPUT
{
    float4 position : SV_Position;
    float c1 : TEXCOORD1;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;
    float3 local_pos;
    if ((input.arrow_index == 1) || (input.arrow_index == 3))
    {
        local_pos = input.position * (1.0f + pow(smoothstep(0.0f, 0.8f, y_axis_stretch), 0.5f));
    }
    else
    {
        local_pos = input.position * (1.0f + pow(smoothstep(0.0f, 0.8f, x_axis_stretch), 0.5f));
    }
    ViewConstants view = view_constants[current_view_index];
    output.position = mul(view.world_to_sample, mul(model_to_world, float4(local_pos, 1.0f)));
    output.c1 = 1.0f;
    return output;
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
float4 ps_main(float c1 : TEXCOORD1) : SV_Target0
{
    return c1.xxxx;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _timed_terrain_gizmo.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_y_axis_stretch : packoffset(c5);
    float _Globals_x_axis_stretch : packoffset(c5.y);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_arrow_index;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] int in_var_arrow_index: arrow_index;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _65;
    if (!(in_var_arrow_index == 1))
    {
        _65 = in_var_arrow_index == 3;
    }
    else
    {
        _65 = true;
    }
    float3 _81;
    if (_65)
    {
        _81 = in_var_Vertex_Position * (1.0f + pow(smoothstep(0.0f, 0.800000011920928955078125f, _Globals_y_axis_stretch), 0.5f));
    }
    else
    {
        _81 = in_var_Vertex_Position * (1.0f + pow(smoothstep(0.0f, 0.800000011920928955078125f, _Globals_x_axis_stretch), 0.5f));
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, mul(_Globals_model_to_world, float4(_81, 1.0f)));
    out_var_C1 = 1.0f;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_arrow_index = stage_input.in_var_arrow_index;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _timed_terrain_gizmo.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float in_var_C1;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float in_var_C1: arrow_index;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    out_var_SV_TARGET = in_var_C1.xxxx;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _timed_terrain_gizmo.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_y_axis_stretch : packoffset(c5);
    float _Globals_x_axis_stretch : packoffset(c5.y);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_arrow_index;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] int in_var_arrow_index: arrow_index;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool _65;
    if (!(in_var_arrow_index == 1))
    {
        _65 = in_var_arrow_index == 3;
    }
    else
    {
        _65 = true;
    }
    float3 _81;
    if (_65)
    {
        _81 = in_var_Vertex_Position * (1.0f + pow(smoothstep(0.0f, 0.800000011920928955078125f, _Globals_y_axis_stretch), 0.5f));
    }
    else
    {
        _81 = in_var_Vertex_Position * (1.0f + pow(smoothstep(0.0f, 0.800000011920928955078125f, _Globals_x_axis_stretch), 0.5f));
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, mul(_Globals_model_to_world, float4(_81, 1.0f)));
    out_var_C1 = 1.0f;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_arrow_index = stage_input.in_var_arrow_index;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
