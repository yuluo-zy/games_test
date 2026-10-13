// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/skydome.raster.hlsl | F-E | bins: _skydome.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Skydome: a dome mesh is transformed by model_to_world; the ps blends a
// sun-disc + glow, a moon sprite (projected onto the dome), and a day/night
// sky gradient, then applies a gradient-noise dither.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "gi/gi_settings.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 moon_tex_dims;
    float4x4 model_to_world;
    uint2 framebuffer_dims;
};

Texture2D<float4> moon_tex : register(t14, space0);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float bbx_y: bbx_y;
};

struct VsOut
{
    float bbx_y : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float3 world_pos : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float3 scaled = vin.Vertex_Position * 5000.0f;
    float4 world = mul(model_to_world, float4(scaled.x, scaled.y * 0.5f, scaled.z, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.bbx_y = vin.bbx_y;
    o.normal = vin.Vertex_Normal;
    o.world_pos = world.xyz;
    return o;
}

float4 ps_main(float3 world_pos : TEXCOORD2, float4 position : SV_Position) : SV_Target0
{
    float3 camera_world = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xyz;
    float3 view_dir = normalize(camera_world - world_pos);

    float sun_angle = acos(-dot(dir_to_sun, view_dir));
    float sun_dist = max(0.0f, sun_angle - 0.02617994f);
    float glow_inner = 0.5f + 25.0f * sun_dist;
    float glow_outer = 1.0f + 5.0f * sun_dist;

    // Moon direction: contracted horizontally and faded toward day.
    float3 moon_dir = normalize(dir_to_moon * float2(lerp(2.5f, 1.0f, smoothstep(0.53f, 0.84f, time_of_day)), 1.0f).xyx);

    // Project the moon sprite onto the dome around moon_dir.
    float3 moon_light = 0.0f;
    float3 off = view_dir - moon_dir;
    float3 axis = normalize(cross(off, float3(0.0f, 1.0f, 0.0f)));
    float2 moon_uv = (float2(dot(moon_dir, axis), dot(moon_dir, normalize(cross(axis, off)))) * 35.0f) * 0.5f + 0.5f;
    float2 clamped = clamp(moon_uv, 0.0f, 1.0f);
    if (all(moon_uv == clamped))
    {
        float4 m = moon_tex.SampleLevel(sampler_lnc, moon_uv, 0.0f);
        moon_light = m.rgb * m.a * 2.0f;
    }

    float day_night = smoothstep(0.35f, 0.65f, time_of_day) - smoothstep(0.8f, 0.95f, time_of_day);

    float3 col = (
        (float3(1.0f, 0.7f, 0.2f) * (smoothstep(0.02617994f, 0.02443461f, sun_angle) * 30.0f))
        + (float3(1.0f, 0.65f, 0.2f) * (5.0f / (glow_inner * glow_inner)))
        + (float3(1.0f, 0.75f, 0.6f) * (0.8f / (glow_outer * glow_outer)))
        + (moon_light * (day_night * day_night * day_night))
    ) * ev_mult
    + (lerp(sky_low_color.xyz * 1.5f, sky_high_color.xyz,
            pow(clamp(1.0f - pow(1.0f - clamp(-view_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f)))
      * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f));

    // Slight saturation boost via luma-weighted lerp.
    float3 boosted = lerp(dot(col, float3(0.2126f, 0.7152f, 0.0722f)).xxx, col, 1.25f);

    // Gradient-noise dither (interleaved gradient noise).
    float noise = (frac(52.9829177f * frac(0.06711056f * position.x + 0.0058371499f * position.y)) * 2.0f) - 1.0f;
    float dither = (max(-1.0f, noise * rsqrt(abs(noise))) - float(int(sign(noise)))) * 0.0039215688f;

    return float4(boosted + clamp(boosted * 512.0f, 0.0f, 1.0f) * dither, 1.0f);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _skydome.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 moon_tex_dims : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 framebuffer_dims : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_bbx_y;
static float out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_bbx_y: bbx_y;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _67 = in_var_Vertex_Position * 5000.0f;
    float4 _80 = mul(model_to_world, float4(_67.x, _67.y * 0.5f, _67.z, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _80);
    out_var_C0 = in_var_bbx_y;
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _80.xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_bbx_y = stage_input.in_var_bbx_y;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _skydome.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 moon_tex_dims : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 framebuffer_dims : packoffset(c5);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> moon_tex : register(t14, space0);

static float4 gl_FragCoord;
static float3 in_var_C2;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    float4 _129 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 _135 = normalize((_129.xyz / _129.w.xxx) - in_var_C2);
    float _140 = acos(-dot(frame_constants_dir_to_sun, _135));
    float _145 = max(0.0f, _140 - 0.0261799395084381103515625f);
    float _147 = 0.5f + (25.0f * _145);
    float _153 = 1.0f + (5.0f * _145);
    float3 _167 = normalize(frame_constants_dir_to_moon * float2(lerp(2.5f, 1.0f, smoothstep(0.5299999713897705078125f, 0.839999973773956298828125f, frame_constants_time_of_day)), 1.0f).xyx);
    float3 _195;
    do
    {
        float3 _170 = _135 - _167;
        float3 _172 = normalize(cross(_170, float3(0.0f, 1.0f, 0.0f)));
        float2 _180 = ((float2(dot(_167, _172), dot(_167, normalize(cross(_172, _170)))) * 35.0f) * 0.5f) + 0.5f.xx;
        float2 _181 = clamp(_180, 0.0f.xx, 1.0f.xx);
        if (all(bool2(_180.x == _181.x, _180.y == _181.y)))
        {
            float4 _190 = moon_tex.SampleLevel(sampler_lnc, _180, 0.0f);
            _195 = (_190.xyz * _190.w) * 2.0f;
            break;
        }
        else
        {
            _195 = 0.0f.xxx;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float _198 = smoothstep(0.3499999940395355224609375f, 0.64999997615814208984375f, frame_constants_time_of_day) - smoothstep(0.800000011920928955078125f, 0.949999988079071044921875f, frame_constants_time_of_day);
    float3 _235 = (((((float3(1.0f, 0.699999988079071044921875f, 0.20000000298023223876953125f) * (smoothstep(0.0261799395084381103515625f, 0.02443460933864116668701171875f, _140) * 30.0f)) + (float3(1.0f, 0.64999997615814208984375f, 0.20000000298023223876953125f) * (5.0f / (_147 * _147)))) + (float3(1.0f, 0.75f, 0.60000002384185791015625f) * (0.800000011920928955078125f / (_153 * _153)))) + (_195 * ((_198 * _198) * _198))) * frame_constants_ev_mult) + (((lerp(frame_constants_sky_low_color.xyz * 1.5f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp((-_135).y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * (1.0f - ((smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, frame_constants_time_of_day) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, frame_constants_time_of_day)) * 0.75f)));
    float3 _238 = lerp(dot(_235, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, _235, 1.25f.xxx);
    uint2 _240 = uint2(gl_FragCoord.xy);
    float _252 = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(_240.x)) + (0.005837149918079376220703125f * float(_240.y)))) * 2.0f) - 1.0f;
    out_var_SV_TARGET = float4(_238 + (clamp(_238 * 512.0f, 0.0f.xxx, 1.0f.xxx) * ((max(-1.0f, _252 * rsqrt(abs(_252))) - float(int(sign(_252)))) * 0.0039215688593685626983642578125f)), 1.0f);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _skydome.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 moon_tex_dims : packoffset(c0.y);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 framebuffer_dims : packoffset(c5);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float in_var_bbx_y;
static float out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_bbx_y: bbx_y;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 _67 = in_var_Vertex_Position * 5000.0f;
    float4 _80 = mul(model_to_world, float4(_67.x, _67.y * 0.5f, _67.z, 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, _80);
    out_var_C0 = in_var_bbx_y;
    out_var_C1 = in_var_Vertex_Normal;
    out_var_C2 = _80.xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_bbx_y = stage_input.in_var_bbx_y;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _skydome.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
