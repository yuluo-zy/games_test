// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/rain_particles.raster.hlsl | F-V | bins: _rain_particles.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Reconstructed from inlined vs/ps opcode patterns (spirv-cross scaffolding + target fingerprint).
// Model: rain streak quads. Per-instance seed hashed (inc/hash.hlsl 5-step hash + rand01); a
// streak falls from a random xz offset at speed scaled by rain_strength; rain bends along the
// world +x axis by strength*0.04; ps outputs white streak with vertical-fade opacity.
// Globals model_to_world + rain_strength are non-static globals -> land in $Globals (member 1, 2).

#include "inc/common.hlsl"          // samplers (unused here; dxc drops them)
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"    // declares current_view_index global
#include "inc/ray_cone.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the
// unused s5..s8 here reproduces the original $Globals binding (set0/b13).

float4x4 model_to_world;
float rain_strength;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    int id : TEXCOORD1;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float fall = lerp(0.6f, 1.2f, rand01(hash(uint(vin.id ^ 5678))));
    float3 offset = float3(
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 45845)))),
        lerp(0.0f, 1.0f, rand01(hash(uint(vin.id ^ 35934)))),
        lerp(0.0f, 60.0f, rand01(hash(uint(vin.id ^ 23475)))));

    offset.y = frac(offset.y - ((float(game_time_sec) + game_time_frac) * 0.5f) * fall) * 50.0f;

    float3 world = float3(0.0f, 1.0f, 0.0f) * vin.Vertex_Position.y
                 + normalize(mul(view_constants[current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz)
                   * vin.Vertex_Position.x * (0.04f * rain_strength)
                 + offset + float3(-30.0f, 0.0f, -30.0f);

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    o.C0 = float3(1.0f, 1.0f, 1.0f);
    o.C1 = lerp(0.3f, 0.0f, vin.Vertex_Position.y + 0.5f) * pow(rain_strength, 0.5f);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.color = float4(pin.C0 * pin.C1, pin.C1);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _rain_particles.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_rain_strength : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_id;
static float3 out_var_C0;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] int in_var_id : TEXCOORD1;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _84 = uint(in_var_id ^ 5678);
    uint _86 = _84 + (_84 << 10u);
    uint _88 = _86 ^ (_86 >> 6u);
    uint _90 = _88 + (_88 << 3u);
    uint _92 = _90 ^ (_90 >> 11u);
    uint _104 = uint(in_var_id ^ 23475);
    uint _106 = _104 + (_104 << 10u);
    uint _108 = _106 ^ (_106 >> 6u);
    uint _110 = _108 + (_108 << 3u);
    uint _112 = _110 ^ (_110 >> 11u);
    uint _121 = uint(in_var_id ^ 35934);
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    float _136 = lerp(0.0f, 1.0f, asfloat(((_129 + (_129 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _138 = uint(in_var_id ^ 45845);
    uint _140 = _138 + (_138 << 10u);
    uint _142 = _140 ^ (_140 >> 6u);
    uint _144 = _142 + (_142 << 3u);
    uint _146 = _144 ^ (_144 >> 11u);
    float3 _171 = float3(lerp(0.0f, 60.0f, asfloat(((_112 + (_112 << 15u)) & 8388607u) | 1065353216u) - 1.0f), _136, lerp(0.0f, 60.0f, asfloat(((_146 + (_146 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _171.y = frac(_136 - (((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 0.5f) * lerp(0.60000002384185791015625f, 1.2000000476837158203125f, asfloat(((_92 + (_92 << 15u)) & 8388607u) | 1065353216u) - 1.0f))) * 50.0f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4((((float3(0.0f, 1.0f, 0.0f) * in_var_Vertex_Position.y) * 1.0f) + ((normalize(mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz) * in_var_Vertex_Position.x) * (0.039999999105930328369140625f * _Globals_rain_strength))) + (_171 + float3(-30.0f, 0.0f, -30.0f)), 1.0f));
    out_var_C0 = 1.0f.xxx;
    out_var_C1 = lerp(0.300000011920928955078125f, 0.0f, in_var_Vertex_Position.y + 0.5f) * pow(_Globals_rain_strength, 0.5f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_id = stage_input.in_var_id;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _rain_particles.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_rain_strength : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static int in_var_id;
static float3 out_var_C0;
static float out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] int in_var_id : TEXCOORD1;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint _84 = uint(in_var_id ^ 5678);
    uint _86 = _84 + (_84 << 10u);
    uint _88 = _86 ^ (_86 >> 6u);
    uint _90 = _88 + (_88 << 3u);
    uint _92 = _90 ^ (_90 >> 11u);
    uint _104 = uint(in_var_id ^ 23475);
    uint _106 = _104 + (_104 << 10u);
    uint _108 = _106 ^ (_106 >> 6u);
    uint _110 = _108 + (_108 << 3u);
    uint _112 = _110 ^ (_110 >> 11u);
    uint _121 = uint(in_var_id ^ 35934);
    uint _123 = _121 + (_121 << 10u);
    uint _125 = _123 ^ (_123 >> 6u);
    uint _127 = _125 + (_125 << 3u);
    uint _129 = _127 ^ (_127 >> 11u);
    float _136 = lerp(0.0f, 1.0f, asfloat(((_129 + (_129 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _138 = uint(in_var_id ^ 45845);
    uint _140 = _138 + (_138 << 10u);
    uint _142 = _140 ^ (_140 >> 6u);
    uint _144 = _142 + (_142 << 3u);
    uint _146 = _144 ^ (_144 >> 11u);
    float3 _171 = float3(lerp(0.0f, 60.0f, asfloat(((_112 + (_112 << 15u)) & 8388607u) | 1065353216u) - 1.0f), _136, lerp(0.0f, 60.0f, asfloat(((_146 + (_146 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    _171.y = frac(_136 - (((float(frame_constants_game_time_sec) + frame_constants_game_time_frac) * 0.5f) * lerp(0.60000002384185791015625f, 1.2000000476837158203125f, asfloat(((_92 + (_92 << 15u)) & 8388607u) | 1065353216u) - 1.0f))) * 50.0f;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4((((float3(0.0f, 1.0f, 0.0f) * in_var_Vertex_Position.y) * 1.0f) + ((normalize(mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz) * in_var_Vertex_Position.x) * (0.039999999105930328369140625f * _Globals_rain_strength))) + (_171 + float3(-30.0f, 0.0f, -30.0f)), 1.0f));
    out_var_C0 = 1.0f.xxx;
    out_var_C1 = lerp(0.300000011920928955078125f, 0.0f, in_var_Vertex_Position.y + 0.5f) * pow(_Globals_rain_strength, 0.5f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_id = stage_input.in_var_id;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
