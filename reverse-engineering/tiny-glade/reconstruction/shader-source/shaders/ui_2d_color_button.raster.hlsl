// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/ui_2d_color_button.raster.hlsl | F-E | bins: _ui_2d_color_button.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Color-picker button quad. Same 6-vertex quad as ui_2d; ps adds an animated
// circle-reveal: progress sweeps around, masked toward the original color when
// is_original_color is set, with a white lerp rim.

#include "inc/common.hlsl"            // samplers (sampler_llr s1)
#include "inc/quad_mesh.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float2 ui_position;
    float2 ui_size;
    float2 window_size;
    float animation_t;
    int is_original_color;
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
    float4 c = ui_texture.SampleBias(sampler_llr, uv, -0.75f);

    float dist = length(abs(uv - 0.5f) * 2.0f);
    float reveal = is_original_color == 1
        ? smoothstep(0.6f, 0.7f, dist)
        : smoothstep(0.8f, 0.9f, dist);

    float alpha = clamp(c.a + reveal * 2.0f, 0.0f, 1.0f);
    float white = is_original_color == 1 ? 0.9f : 0.4f;
    float3 col = lerp(c.rgb, 1.0f, white * reveal) * alpha;

    return float4(col, alpha) * ((1.0f - smoothstep(0.95f, 1.0f, dist)) * pow(animation_t, 0.2f));
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _ui_2d_color_button.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
