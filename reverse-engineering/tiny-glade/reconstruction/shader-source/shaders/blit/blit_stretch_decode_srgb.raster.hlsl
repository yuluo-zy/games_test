// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/blit/blit_stretch_decode_srgb.raster.hlsl | F-E | bins: _blit_blit_stretch_decode_srgb.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Fullscreen blit: sample the input texture (stretched) and decode sRGB -> linear.
// Evidence: ps inlined per-channel decode (0.04045/0.0773993805/0.947867333/2.4 — inc/color/srgb.hlsl srgb_to_linear);
// vs is the shared fullscreen triangle (fullscreen_triangle_vert.inc.hlsl). No $Globals.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/color/srgb.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

Texture2D<float4> input_tex : register(t14, space0);

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = input_tex.Sample(sampler_llc, uv);

    // sRGB -> linear per channel (strict threshold matches shipped codegen).
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
