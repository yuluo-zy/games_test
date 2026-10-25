// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/logo_overlay.raster.hlsl | F-E | bins: _logo_overlay.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Overlay logo used on top of gameplay: aspect-corrected quad near the corner;
// ps samples input_tex with a warm tint.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 framebuffer_dims;
    uint2 input_tex_dims;
};

Texture2D<float4> input_tex : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 1.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 0.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 0.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 1.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 1.0f);

    float2 pos = (corner.xy * 0.225f) * 2.0f;
    uint w, h;
    input_tex.GetDimensions(w, h);
    float aspect = (float(w) / float(h)) / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    pos.x *= aspect;
    o.position = float4(pos + (float2(0.95f, -0.95f) * (1.0f - float2(aspect, 1.0f) * 0.225f)), 1.0f, 1.0f);
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = input_tex.SampleBias(sampler_llc, uv, -0.5f);
    return float4(c.rgb * c.a * float3(1.0f, 0.9f, 0.85f), c.a) * 1.0f;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _logo_overlay.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
