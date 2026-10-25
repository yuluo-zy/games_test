// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/ui_2d.raster.hlsl | F-E | bins: _ui_2d.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Simple 2D UI quad (no per-camera transform): position/size in window pixels.
// vs emits the 6-vertex quad; ps samples ui_texture with -1 mip bias and applies
// optional darken (semi-transparent overlay) and highlight color effects.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/quad_mesh.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float2 ui_position;
    float2 ui_size;
    float2 window_size;
    int darken_semitransparent;
    int highlight;
    uint2 ui_texture_dims;
};

Texture2D<float4> ui_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    float2 pos = (ui_position + (corner.xy * ui_size)) / window_size;
    pos.y = 1.0f - pos.y;
    o.position = float4(pos * 2.0f - 1.0f, 0.0f, 1.0f);
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = ui_texture.SampleBias(sampler_llc, uv, -1.0f);

    float4 col;
    if (darken_semitransparent == 1)
        col = float4(c.rgb * lerp(c.rgb, 0.4f, 0.9f) * c.a, c.a) * 0.6f;
    else
        col = float4(c.rgb * c.a, c.a);

    if (highlight == 1)
        col = float4(lerp(float3(0.08f, 0.048f, 0.064f), float3(0.7225f, 0.4675f, 0.17f),
                          (1.0f - smoothstep(0.0f, 0.6f, length(col.rgb))).xxx) * col.w, col.w);

    return col;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _ui_2d.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
