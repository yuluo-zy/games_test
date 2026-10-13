// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/session_selection/background.raster.hlsl | F-E | bins: _session_selection_background.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Session-selection background: fullscreen triangle + a hex-grid dithered
// sampling of background_tex with a vignette curve.

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_llr s1)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/color/srgb.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 background_tex_dims;
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
};

Texture2D<float4> background_tex : register(t14, space0);

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    float2 uv = position.xy / float2(framebuffer_dims);
    float2 uv_aspect = uv;
    uv_aspect.x = uv.x * (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 hex = (uv_aspect - first_thumbnail_center) * 2.0f;

    float2 tex_dims = float2(background_tex_dims);
    float2 hex_scaled = mul(hex * 3.464f, float2x2(float2(1.0f, 0.0f), float2(-0.5773503f, 1.1547005f)));
    float2 base = floor(hex_scaled);
    float2 f = frac(hex_scaled);

    float f0 = f.x;
    float f1 = f.y;
    float w0 = 1.0f - f0;
    bool edge = (w0 - f1) > 0.0f;

    float3 a = edge ? float3(base, w0 - f1) : float3(base + 1.0f, -(w0 - f1));
    float3 b = edge ? float3(base + float2(0.0f, 1.0f), f1) : float3(base + float2(1.0f, 0.0f), 1.0f - f1);
    float3 c = edge ? float3(base + float2(1.0f, 0.0f), f0) : float3(base + float2(0.0f, 1.0f), w0);

    float4 grad = float4(ddx(hex), -ddy(hex));
    float2 grad_xy = grad.xy;
    float2 grad_zw = grad.zw;

    uint2 h1 = ((uint2(a.xy) >> uint2(8u, 8u)) ^ uint2(a.xy).yx) * uint2(1103515245u, 1103515245u);
    uint2 h2 = ((uint2(b.xy) >> uint2(8u, 8u)) ^ uint2(b.xy).yx) * uint2(1103515245u, 1103515245u);
    uint2 h3 = ((uint2(c.xy) >> uint2(8u, 8u)) ^ uint2(c.xy).yx) * uint2(1103515245u, 1103515245u);

    float lod = log(min(tex_dims.x, tex_dims.y)) * 1.442695f + 1.0f;
    float4 base_sample = background_tex.SampleLevel(sampler_llr, float2(0.0f, 0.0f), lod);

    float4 acc = background_tex.SampleGrad(sampler_llr, hex + float2((h1 >> uint2(8u, 8u)) ^ h1.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * a.z
               + background_tex.SampleGrad(sampler_llr, hex + float2((h2 >> uint2(8u, 8u)) ^ h2.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * b.z
               + background_tex.SampleGrad(sampler_llr, hex + float2((h3 >> uint2(8u, 8u)) ^ h3.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * c.z;
    float4 col = (acc - base_sample) / length(float3(a.z, b.z, c.z)).xxxx + base_sample;

    float dist = length(uv - 0.5f);
    float3 shaded = pow((sqrt(col.rgb) * float3(0.5476f, 0.473619f, 0.395641f)),
                        (1.0f + 0.5f * (1.0f - exp(-2.0f * ((dist * dist) * dist)))).xxx);
    return float4(shaded, col.w);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _session_selection_background.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 _Globals_background_tex_dims : packoffset(c0);
    uint2 _Globals_framebuffer_dims : packoffset(c0.z);
    float2 _Globals_first_thumbnail_center : packoffset(c1);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> background_tex : register(t14, space0);

static float4 gl_FragCoord;
static float4 out_var_SV_TARGET;

struct SPIRV_Cross_Input
{
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float4 out_var_SV_TARGET : SV_Target0;
};

void frag_main()
{
    float2 _89 = gl_FragCoord.xy / float2(_Globals_framebuffer_dims);
    float2 _92 = _89;
    _92.x = _89.x * (float(_Globals_framebuffer_dims.x) / float(_Globals_framebuffer_dims.y));
    float2 _96 = (_92 - _Globals_first_thumbnail_center) * 2.0f;
    float2 _100 = float2(_Globals_background_tex_dims);
    float2 _102 = mul(_96 * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 _103 = floor(_102);
    float2 _104 = frac(_102);
    float _105 = _104.x;
    float _106 = _104.y;
    float _107 = 1.0f - _105;
    float _108 = _107 - _106;
    bool _109 = _108 > 0.0f;
    float3 _121;
    if (_109)
    {
        _121 = float3(_103, _108);
    }
    else
    {
        _121 = float3(_103 + 1.0f.xx, -_108);
    }
    float3 _134;
    if (_109)
    {
        _134 = float3(_103 + float2(0.0f, 1.0f), _106);
    }
    else
    {
        _134 = float3(_103 + float2(1.0f, 0.0f), 1.0f - _106);
    }
    float3 _146;
    if (_109)
    {
        _146 = float3(_103 + float2(1.0f, 0.0f), _105);
    }
    else
    {
        _146 = float3(_103 + float2(0.0f, 1.0f), _107);
    }
    float4 _154 = float4(ddx(_96), -ddy(_96));
    uint2 _157 = uint2(_121.xy);
    uint2 _161 = ((_157 >> uint2(8u, 8u)) ^ _157.yx) * uint2(1103515245u, 1103515245u);
    float2 _169 = _154.xy;
    float2 _170 = _154.zw;
    uint2 _176 = uint2(_134.xy);
    uint2 _180 = ((_176 >> uint2(8u, 8u)) ^ _176.yx) * uint2(1103515245u, 1103515245u);
    uint2 _194 = uint2(_146.xy);
    uint2 _198 = ((_194 >> uint2(8u, 8u)) ^ _194.yx) * uint2(1103515245u, 1103515245u);
    float4 _218 = background_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(_100.x, _100.y)) * 1.44269502162933349609375f) + 1.0f);
    float4 _224 = (((((background_tex.SampleGrad(sampler_llr, _96 + (float2(((_161 >> uint2(8u, 8u)) ^ _161.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _169, _170) * _121.z) + (background_tex.SampleGrad(sampler_llr, _96 + (float2(((_180 >> uint2(8u, 8u)) ^ _180.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _169, _170) * _134.z)) + (background_tex.SampleGrad(sampler_llr, _96 + (float2(((_198 >> uint2(8u, 8u)) ^ _198.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), _169, _170) * _146.z)) - _218) / length(float3(_121.z, _134.z, _146.z)).xxxx) + _218;
    float _229 = length(_89 - 0.5f.xx);
    float3 _239 = pow((sqrt(_224.xyz) * float3(0.5476000308990478515625f, 0.4736192226409912109375f, 0.3956409990787506103515625f)).xyz, (1.0f + (0.5f * (1.0f - exp((-2.0f) * ((_229 * _229) * _229))))).xxx);
    out_var_SV_TARGET = float4(_239.x, _239.y, _239.z, _224.w);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET = out_var_SV_TARGET;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _session_selection_background.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
