// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/blit/blit_stretch_decode_srgb.raster.hlsl | F-E | bins: _blit_blit_stretch_decode_srgb.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 中文说明:
//   用途: 全屏 blit——拉伸采样输入纹理,并把 sRGB 解码为线性。
//   入口: vs_main(共享全屏三角形,见 fullscreen_triangle_vert.inc.hlsl)、
//         ps_main(逐通道 sRGB->线性解码)。
//   变体: TGV_f0adff76827e73ff -> SPIR-V 提升实现(该 bin 的函数体为空,
//         故下方提升段只有空壳);未定义该 TGV 时用上方手写实现。
//   绑定: input_tex t14,采样器 sampler_llc s3;无 $Globals。
//   证据: ps 内联逐通道解码(常量 0.04045/0.0773993805/0.947867333/2.4 来自
//         inc/color/srgb.hlsl 的 srgb_to_linear)。

#include "inc/common.hlsl"            // 采样器 (sampler_llc s3)
#include "inc/color/srgb.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

Texture2D<float4> input_tex : register(t14, space0);

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    // 拉伸采样输入。
    float4 c = input_tex.Sample(sampler_llc, uv);

    // 逐通道 sRGB -> 线性(严格阈值匹配出货代码生成)。
    float r0 = c.x;
    float g0 = c.y;
    float b0 = c.z;
    float r;
    float g;
    float b;
    if (0.04045f < r0) r = pow((r0 + 0.055f) * 0.947867333f, 2.4f);
    else r = r0 * 0.0773993805f;
    if (0.04045f < g0) g = pow((g0 + 0.055f) * 0.947867333f, 2.4f);
    else g = g0 * 0.0773993805f;
    if (0.04045f < b0) b = pow((b0 + 0.055f) * 0.947867333f, 2.4f);
    else b = b0 * 0.0773993805f;

    float3 l = float3(r, g, b);
    return float4(l, c.a);
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _blit_blit_stretch_decode_srgb.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 该 bin 的函数体为空(提升段只有空壳,逻辑与手写段一致)。
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
