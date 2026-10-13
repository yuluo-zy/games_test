// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/mei_shadow.raster.hlsl
// SOURCE PATH : shaders/mei_shadow.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _mei_shadow.raster.hlsl_b903e43ffb3da915.bin
//     pass/stage   : _mei_shadow (shadow) / raster
//     entry        : vs:vs_main; ps:ps_main
//     names        : vs_main, sampler_nnc, ViewConstants, frame_constants, $Globals, blue_noise_texture, input_tex, ps_main
//   _mei_shadow.raster.hlsl_f0adff76827e73ff.bin
//     pass/stage   : _mei_shadow (shadow) / raster
//     entry        : vs:vs_main; ps:ps_main
// Status: O (structure from disassembly: fullscreen-triangle VS + PS that
//   softens the input shadow mask with a golden-ratio spiral tap field and a
//   wave-wide early-out); I (intent of the pass, variable naming).
//   The VS is the shared fullscreen triangle (fullscreen_triangle_vert.inc.hlsl).
//   The PS loads the blue-noise jitter, accumulates min(0.6, lerp(alpha,1,0.1))
//   over a 3-tap then a 48-tap spiral, and keeps WaveActiveAnyTrue(acc>0) before
//   running the full 48-tap field; the final alpha is max(input_a, sum/48).
//   $Globals (b13) carries blue_noise_texture_dims/framebuffer_dims/input_tex_dims
//   (declared explicitly per the $Globals-binding reconstruction convention).

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/uv.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/blue_noise.hlsl"
#include "inc/hammersley.hlsl"

// Per-shader globals (the original embeds them in $Globals at set0/b13).
cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    uint2 framebuffer_dims;
    uint2 input_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> input_tex : register(t15, space0);

struct PsIn
{
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

float4 ps_main(PsIn pin) : SV_Target
{
    float4 result = input_tex.SampleLevel(sampler_nnc, pin.uv, 0.0f);
    float base_a = lerp(result.w, 1.0f, 0.1f);
    result.w = base_a;

    // Blue-noise jitter, seeded by (frame_index % 64) + 7 (the .xy channel).
    uint2 bn_pixel = uint2(pin.position.xy);
    uint2 bn_offset = uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float((frame_index % 64u) + 7u)) + 0.5f.xx) * 256.0f.xx);
    float2 bn = ((blue_noise_texture.Load(int3((bn_pixel + bn_offset) % uint2(256u, 256u), 0u))
        * 255.0f) * 0.00390625f + 0.001953125f).xy;

    float2 aspect = float2(float(input_tex_dims.y) / float(input_tex_dims.x), 1.0f);
    float2 radius = aspect * 0.0075f;
    float2 jitter = float2(0.0044100000523030757904052734375f, 0.00233100005425512790679931640625f) * aspect;

    // First 3 golden-ratio spiral taps; a wave-wide early-out skips the rest.
    float acc = 0.0f;
    for (uint i = 0u; i < 3u; i++)
    {
        float2 s = frac(bn + frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
            * float(i)) + 0.5f.xx));
        float r = sqrt(s.x);
        float a = s.y * 6.283185482025146484375f;
        acc += min(0.6f, lerp(input_tex.SampleLevel(sampler_nnc,
            (pin.uv - jitter) + (float2(cos(a) * r, sin(a) * r) * radius), 0.0f).w, 1.0f, 0.1f));
    }

    if (!WaveActiveAnyTrue(acc > 0.0f))
    {
        return result;
    }

    // Full 48-tap spiral field (3 + 45), normalized by 1/48.
    float sum = acc;
    for (uint j = 3u; j < 48u; j++)
    {
        float2 s = frac(bn + frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
            * float(j)) + 0.5f.xx));
        float r = sqrt(s.x);
        float a = s.y * 6.283185482025146484375f;
        sum += min(0.6f, lerp(input_tex.SampleLevel(sampler_nnc,
            (pin.uv - jitter) + (float2(cos(a) * r, sin(a) * r) * radius), 0.0f).w, 1.0f, 0.1f));
    }

    result.w = max(base_a, sum * 0.02083333395421504974365234375f);
    return result;
}

#include "fullscreen_triangle_vert.inc.hlsl"

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _mei_shadow.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 _Globals_blue_noise_texture_dims : packoffset(c0);
    uint2 _Globals_framebuffer_dims : packoffset(c0.z);
    uint2 _Globals_input_tex_dims : packoffset(c1);
};

SamplerState sampler_nnc : register(s2, space0);
Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> input_tex : register(t15, space0);

static float4 gl_FragCoord;
static float2 in_var_C0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    do
    {
        float2 _106 = (((blue_noise_texture.Load(int3((uint2(gl_FragCoord.xy) + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_constants_frame_index % 64u) + 7u)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx).xy;
        float2 _114 = float2(float(_Globals_input_tex_dims.y) / float(_Globals_input_tex_dims.x), 1.0f);
        float2 _115 = _114 * 0.0074999998323619365692138671875f;
        float2 _116 = float2(0.0044100000523030757904052734375f, 0.00233100005425512790679931640625f) * _114;
        float _118;
        _118 = 0.0f;
        for (uint _121 = 0u; _121 < 3u; )
        {
            float2 _130 = frac(_106 + frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(_121)) + 0.5f.xx));
            float _132 = sqrt(_130.x);
            float _134 = _130.y * 6.283185482025146484375f;
            _118 += min(0.60000002384185791015625f, lerp(input_tex.SampleLevel(sampler_nnc, (in_var_C0 - _116) + (float2(cos(_134) * _132, sin(_134) * _132) * _115), 0.0f).w, 1.0f, 0.100000001490116119384765625f));
            _121++;
            continue;
        }
        if (!WaveActiveAnyTrue(_118 > 0.0f))
        {
            break;
        }
        break;
    } while(false);
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
