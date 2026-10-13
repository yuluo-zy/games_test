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
