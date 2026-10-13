// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/star.raster.hlsl | F-E | bins: _star.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Stars: instanced procedural billboards scattered on a large shell around the
// camera; the ps samples the star texture, gated by view direction and the
// day/night time window.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 framebuffer_dims;
    uint2 star_texture_dims;
};

Texture2D<float4> star_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    nointerpolation uint instance : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float3 right : TEXCOORD3;
    float3 facing : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VsOut o;

    float2 quad = (float2(float(vertex_id & 1u), float((vertex_id >> 1u) & 1u)) - 0.5f) * float2(1.0f, 2.0f);

    uint seed = (instance_id * 12u) + 25u;
    uint h1 = hash(seed);
    uint h2 = hash(h1 + (h1 << 10u));
    uint h3 = hash(h2 + (h2 << 10u));

    float fi = float(instance_id);
    float angle = (frac(0.3f + fi * GOLDEN_RATIO) * TWO_PI)
                + ((float(frame_index) * 1e-5f) * lerp(1.0f, 0.07f, rand01(h2)));

    float3 facing = -float3(cos(angle), 0.0f, sin(angle));
    float t = 1.0f - fi * 0.0001666667f;
    float3 center = (-facing) * (10.0f * lerp(800.0f, 2200.0f, t * t));
    center.y = (center.y + 10000.0f * (rand01(h1) * rand01(h1))) - 1000.0f;

    float2 size = (float2(2.0f, 1.0f) * (lerp(1.0f, 0.2f, smoothstep(0.7f, 1.0f, rand01(h1))) * 10.0f)) * 3.0f
                * rand01(hash(h3));
    float3 right = float3(facing.z, 0.0f, -facing.x);

    float3 world = center + (right * (size.x * quad.x)) + (float3(0.0f, 1.0f, 0.0f) * (size.y * quad.y));

    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(world, 1.0f));
    if (instance_id > 6000u)
        clip = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = clip;
    o.uv = float2(float(vertex_id & 1u), float(1u - ((vertex_id >> 1u) & 1u)));
    o.instance = instance_id;
    o.world_pos = world;
    o.right = right;
    o.facing = facing;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - vin.world_pos);

    // Night / dusk intensity windows around specific times of day.
    float d_night = abs(time_of_day - 0.9f);
    float night = (d_night > 0.1f) ? 0.0f : 1.0f - (d_night * 10.0f) * (d_night * 10.0f) * (3.0f - d_night * 20.0f);
    float d_dusk = abs(time_of_day - 0.45f);
    float dusk = (d_dusk > 0.15f) ? 0.0f : 1.0f - (d_dusk * 6.6667f) * (d_dusk * 6.6667f) * (3.0f - d_dusk * 13.333f);

    float occlusion = smoothstep(-1.0f, 1.0f, dot(view_dir, -mul(view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz));
    float visible = smoothstep(0.4f, 0.6f, time_of_day) - smoothstep(0.8f, 0.92f, time_of_day);

    float star = star_texture.Sample(sampler_lnc, vin.uv).x * (1.0f - occlusion * (night + dusk)) * visible;
    return float4(star.xxx * (10.0f * ev_mult), star);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _star.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _103 = float(uint(gl_VertexIndex) & 1u);
    uint _105 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _109 = (float2(_103, float(_105)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _111 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _113 = _111 + (_111 << 10u);
    uint _115 = _113 ^ (_113 >> 6u);
    uint _117 = _115 + (_115 << 3u);
    uint _119 = _117 ^ (_117 >> 11u);
    uint _121 = _119 + (_119 << 15u);
    float _122 = float(uint(gl_InstanceIndex));
    uint _127 = _121 + (_121 << 10u);
    uint _129 = _127 ^ (_127 >> 6u);
    uint _131 = _129 + (_129 << 3u);
    uint _133 = _131 ^ (_131 >> 11u);
    uint _135 = _133 + (_133 << 15u);
    float _139 = asfloat((_121 & 8388607u) | 1065353216u) - 1.0f;
    uint _141 = _135 + (_135 << 10u);
    uint _143 = _141 ^ (_141 >> 6u);
    uint _145 = _143 + (_143 << 3u);
    uint _147 = _145 ^ (_145 >> 11u);
    uint _149 = _147 + (_147 << 15u);
    uint _155 = _149 + (_149 << 10u);
    uint _157 = _155 ^ (_155 >> 6u);
    uint _159 = _157 + (_157 << 3u);
    uint _161 = _159 ^ (_159 >> 11u);
    float _171 = (frac(0.300000011920928955078125f + (_122 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_135 & 8388607u) | 1065353216u) - 1.0f));
    float2 _175 = -float2(cos(_171), sin(_171));
    float _176 = _175.y;
    float _177 = _175.x;
    float3 _179 = float3(_177, 0.0f, _176);
    float _182 = 1.0f - (_122 * 0.00016666666488163173198699951171875f);
    float3 _186 = (-_179) * (10.0f * lerp(800.0f, 2200.0f, _182 * _182));
    _186.y = (_186.y + (10000.0f * (_139 * _139))) - 1000.0f;
    float2 _202 = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, _139)) * 10.0f)) * 3.0f) * (asfloat(((_161 + (_161 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 _203 = float3(_176, 0.0f, -_177);
    float3 _213 = _186 + ((_203 * (_202.x * _109.x)) + (float3(0.0f, 1.0f, 0.0f) * (_202.y * _109.y)));
    float4 _226 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_213, 1.0f));
    bool4 _228 = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(_228.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _226.x, _228.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _226.y, _228.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _226.z, _228.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _226.w);
    out_var_C0 = float2(_103, float(1u - _105));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _213;
    out_var_C3 = _203;
    out_var_C4 = _179;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _star.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> star_texture : register(t14, space0);

static float2 in_var_C0;
static float3 in_var_C2;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    float4 _94 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _121;
    do
    {
        float _111 = abs(frame_constants_time_of_day - 0.89999997615814208984375f);
        if (_111 > 0.100000001490116119384765625f)
        {
            _121 = 0.0f;
            break;
        }
        float _115 = _111 * 10.0f;
        _121 = 1.0f - ((_115 * _115) * (3.0f - (_111 * 20.0f)));
        break;
    } while(false);
    float _135;
    do
    {
        float _125 = abs(frame_constants_time_of_day - 0.449999988079071044921875f);
        if (_125 > 0.1500000059604644775390625f)
        {
            _135 = 0.0f;
            break;
        }
        float _129 = _125 * 6.666666507720947265625f;
        _135 = 1.0f - ((_129 * _129) * (3.0f - (_125 * 13.33333301544189453125f)));
        break;
    } while(false);
    float4 _152 = 1.0f.xxxx * ((star_texture.Sample(sampler_lnc, in_var_C0).x * (1.0f - (smoothstep(-1.0f, 1.0f, dot(normalize((_94.xyz / _94.w.xxx) - in_var_C2), -mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz)) * (_121 + _135)))) * (smoothstep(0.4000000059604644775390625f, 0.60000002384185791015625f, frame_constants_time_of_day) - smoothstep(0.800000011920928955078125f, 0.920000016689300537109375f, frame_constants_time_of_day)));
    float3 _157 = _152.xyz * (10.0f * frame_constants_ev_mult);
    out_var_SV_TARGET = float4(_157.x, _157.y, _157.z, _152.w);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _star.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 _Globals_framebuffer_dims : packoffset(c0.y);
    uint2 _Globals_star_texture_dims : packoffset(c1);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float _103 = float(uint(gl_VertexIndex) & 1u);
    uint _105 = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 _109 = (float2(_103, float(_105)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint _111 = (uint(gl_InstanceIndex) * 12u) + 25u;
    uint _113 = _111 + (_111 << 10u);
    uint _115 = _113 ^ (_113 >> 6u);
    uint _117 = _115 + (_115 << 3u);
    uint _119 = _117 ^ (_117 >> 11u);
    uint _121 = _119 + (_119 << 15u);
    float _122 = float(uint(gl_InstanceIndex));
    uint _127 = _121 + (_121 << 10u);
    uint _129 = _127 ^ (_127 >> 6u);
    uint _131 = _129 + (_129 << 3u);
    uint _133 = _131 ^ (_131 >> 11u);
    uint _135 = _133 + (_133 << 15u);
    float _139 = asfloat((_121 & 8388607u) | 1065353216u) - 1.0f;
    uint _141 = _135 + (_135 << 10u);
    uint _143 = _141 ^ (_141 >> 6u);
    uint _145 = _143 + (_143 << 3u);
    uint _147 = _145 ^ (_145 >> 11u);
    uint _149 = _147 + (_147 << 15u);
    uint _155 = _149 + (_149 << 10u);
    uint _157 = _155 ^ (_155 >> 6u);
    uint _159 = _157 + (_157 << 3u);
    uint _161 = _159 ^ (_159 >> 11u);
    float _171 = (frac(0.300000011920928955078125f + (_122 * 1.61803400516510009765625f)) * 6.283185482025146484375f) + ((float(frame_constants_frame_index) * 9.9999997473787516355514526367188e-06f) * lerp(1.0f, 0.070000000298023223876953125f, asfloat((_135 & 8388607u) | 1065353216u) - 1.0f));
    float2 _175 = -float2(cos(_171), sin(_171));
    float _176 = _175.y;
    float _177 = _175.x;
    float3 _179 = float3(_177, 0.0f, _176);
    float _182 = 1.0f - (_122 * 0.00016666666488163173198699951171875f);
    float3 _186 = (-_179) * (10.0f * lerp(800.0f, 2200.0f, _182 * _182));
    _186.y = (_186.y + (10000.0f * (_139 * _139))) - 1000.0f;
    float2 _202 = ((float2(2.0f, 1.0f) * (lerp(1.0f, 0.20000000298023223876953125f, smoothstep(0.699999988079071044921875f, 1.0f, _139)) * 10.0f)) * 3.0f) * (asfloat(((_161 + (_161 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float3 _203 = float3(_176, 0.0f, -_177);
    float3 _213 = _186 + ((_203 * (_202.x * _109.x)) + (float3(0.0f, 1.0f, 0.0f) * (_202.y * _109.y)));
    float4 _226 = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_213, 1.0f));
    bool4 _228 = (uint(gl_InstanceIndex) > 6000u).xxxx;
    gl_Position = float4(_228.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : _226.x, _228.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : _226.y, _228.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : _226.z, _228.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : _226.w);
    out_var_C0 = float2(_103, float(1u - _105));
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = _213;
    out_var_C3 = _203;
    out_var_C4 = _179;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _star.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
