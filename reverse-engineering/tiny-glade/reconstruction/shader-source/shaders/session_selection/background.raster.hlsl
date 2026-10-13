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
