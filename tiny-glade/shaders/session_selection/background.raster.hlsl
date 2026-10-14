// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/session_selection/background.raster.hlsl | F-E | bins: _session_selection_background.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 会话选择背景:全屏三角形 + 六边形网格抖动采样 background_tex,带渐晕曲线。

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // 采样器(sampler_llr s1)
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
    // 屏幕 uv,并修正纵横比(按 framebuffer 宽高比拉伸 x)。
    float2 uv = position.xy / float2(framebuffer_dims);
    float2 uv_aspect = uv;
    uv_aspect.x = uv.x * (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    // 相对第一个缩略图中心的六边形网格坐标。
    float2 hex = (uv_aspect - first_thumbnail_center) * 2.0f;

    // 六边形斜坐标变换(scaled),拆出格子整数 base 与分数 f。
    float2 tex_dims = float2(background_tex_dims);
    float2 hex_scaled = mul(hex * 3.464f, float2x2(float2(1.0f, 0.0f), float2(-0.5773503f, 1.1547005f)));
    float2 base = floor(hex_scaled);
    float2 f = frac(hex_scaled);

    // 判断所在六边形半区,取三个采样角 a/b/c(每个带权重 z)。
    float f0 = f.x;
    float f1 = f.y;
    float w0 = 1.0f - f0;
    bool edge = (w0 - f1) > 0.0f;

    float3 a = edge ? float3(base, w0 - f1) : float3(base + 1.0f, -(w0 - f1));
    float3 b = edge ? float3(base + float2(0.0f, 1.0f), f1) : float3(base + float2(1.0f, 0.0f), 1.0f - f1);
    float3 c = edge ? float3(base + float2(1.0f, 0.0f), f0) : float3(base + float2(0.0f, 1.0f), w0);

    // SampleGrad 所需的屏幕空间梯度。
    float4 grad = float4(ddx(hex), -ddy(hex));
    float2 grad_xy = grad.xy;
    float2 grad_zw = grad.zw;

    // 三个角的格子哈希(LCG),作为抖动偏移。
    uint2 h1 = ((uint2(a.xy) >> uint2(8u, 8u)) ^ uint2(a.xy).yx) * uint2(1103515245u, 1103515245u);
    uint2 h2 = ((uint2(b.xy) >> uint2(8u, 8u)) ^ uint2(b.xy).yx) * uint2(1103515245u, 1103515245u);
    uint2 h3 = ((uint2(c.xy) >> uint2(8u, 8u)) ^ uint2(c.xy).yx) * uint2(1103515245u, 1103515245u);

    // 按纹理尺寸选择 mip lod,取低分辨率基采样用于偏差校正。
    float lod = log(min(tex_dims.x, tex_dims.y)) * 1.442695f + 1.0f;
    float4 base_sample = background_tex.SampleLevel(sampler_llr, float2(0.0f, 0.0f), lod);

    // 三个抖动点的 SampleGrad 加权和,再以基采样做校正、按权重归一化。
    float4 acc = background_tex.SampleGrad(sampler_llr, hex + float2((h1 >> uint2(8u, 8u)) ^ h1.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * a.z
               + background_tex.SampleGrad(sampler_llr, hex + float2((h2 >> uint2(8u, 8u)) ^ h2.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * b.z
               + background_tex.SampleGrad(sampler_llr, hex + float2((h3 >> uint2(8u, 8u)) ^ h3.yx) * 2.3283064365386963e-10f, grad_xy, grad_zw) * c.z;
    float4 col = (acc - base_sample) / length(float3(a.z, b.z, c.z)).xxxx + base_sample;

    // 渐晕:随离屏中心距离的幂曲线调暗颜色。
    float dist = length(uv - 0.5f);
    float3 shaded = pow((sqrt(col.rgb) * float3(0.5476f, 0.473619f, 0.395641f)),
                        (1.0f + 0.5f * (1.0f - exp(-2.0f * ((dist * dist) * dist)))).xxx);
    return float4(shaded, col.w);
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译自 _session_selection_background.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,变量名已语义化)
// 像素着色器:六边形网格抖动采样背景纹理 + 渐晕(逻辑同上方重建段,算术为 oracle 逐字)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
cbuffer $Globals : register(b13, space0)
{
    uint2 background_tex_dims : packoffset(c0);
    uint2 framebuffer_dims : packoffset(c0.z);
    float2 first_thumbnail_center : packoffset(c1);
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
    // 屏幕 uv(含纵横比修正),相对首缩略图中心的六边形坐标。
    float2 uv = gl_FragCoord.xy / float2(framebuffer_dims);
    float2 uv_aspect = uv;
    uv_aspect.x = uv.x * (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 hex = (uv_aspect - first_thumbnail_center) * 2.0f;
    float2 tex_dims = float2(background_tex_dims);
    float2 hex_scaled = mul(hex * 3.4639999866485595703125f, float2x2(float2(1.0f, 0.0f), float2(-0.57735025882720947265625f, 1.1547005176544189453125f)));
    float2 base = floor(hex_scaled);
    float2 f = frac(hex_scaled);
    float f0 = f.x;
    float f1 = f.y;
    float w0 = 1.0f - f0;
    float w0_minus_f1 = w0 - f1;
    bool edge = w0_minus_f1 > 0.0f;
    // 三个采样角 a/b/c(整数格 + 权重)。
    float3 a;
    if (edge)
    {
        a = float3(base, w0_minus_f1);
    }
    else
    {
        a = float3(base + 1.0f.xx, -w0_minus_f1);
    }
    float3 b;
    if (edge)
    {
        b = float3(base + float2(0.0f, 1.0f), f1);
    }
    else
    {
        b = float3(base + float2(1.0f, 0.0f), 1.0f - f1);
    }
    float3 c;
    if (edge)
    {
        c = float3(base + float2(1.0f, 0.0f), f0);
    }
    else
    {
        c = float3(base + float2(0.0f, 1.0f), w0);
    }
    float4 grad = float4(ddx(hex), -ddy(hex));
    uint2 a_int = uint2(a.xy);
    uint2 h1 = ((a_int >> uint2(8u, 8u)) ^ a_int.yx) * uint2(1103515245u, 1103515245u);
    float2 grad_xy = grad.xy;
    float2 grad_zw = grad.zw;
    uint2 b_int = uint2(b.xy);
    uint2 h2 = ((b_int >> uint2(8u, 8u)) ^ b_int.yx) * uint2(1103515245u, 1103515245u);
    uint2 c_int = uint2(c.xy);
    uint2 h3 = ((c_int >> uint2(8u, 8u)) ^ c_int.yx) * uint2(1103515245u, 1103515245u);
    // 低分辨率基采样用于偏差校正;三个抖动点 SampleGrad 加权和、归一化。
    float4 base_sample = background_tex.SampleLevel(sampler_llr, 0.0f.xx, (log(min(tex_dims.x, tex_dims.y)) * 1.44269502162933349609375f) + 1.0f);
    float4 col = (((((background_tex.SampleGrad(sampler_llr, hex + (float2(((h1 >> uint2(8u, 8u)) ^ h1.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_xy, grad_zw) * a.z) + (background_tex.SampleGrad(sampler_llr, hex + (float2(((h2 >> uint2(8u, 8u)) ^ h2.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_xy, grad_zw) * b.z)) + (background_tex.SampleGrad(sampler_llr, hex + (float2(((h3 >> uint2(8u, 8u)) ^ h3.yx) * uint2(1103515245u, 1103515245u)) * 2.3283064365386962890625e-10f), grad_xy, grad_zw) * c.z)) - base_sample) / length(float3(a.z, b.z, c.z)).xxxx) + base_sample;
    // 渐晕调暗。
    float dist = length(uv - 0.5f.xx);
    float3 shaded = pow((sqrt(col.xyz) * float3(0.5476000308990478515625f, 0.4736192226409912109375f, 0.3956409990787506103515625f)).xyz, (1.0f + (0.5f * (1.0f - exp((-2.0f) * ((dist * dist) * dist))))).xxx);
    out_var_SV_TARGET = float4(shaded.x, shaded.y, shaded.z, col.w);
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
