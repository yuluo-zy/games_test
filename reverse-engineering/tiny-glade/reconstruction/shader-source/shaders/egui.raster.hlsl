// ==RECONSTRUCTED== shaders/egui.raster.hlsl | F-E | bins: _egui.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// egui (immediate-mode UI) textured quad: a packed vertex buffer (20-byte stride:
// xy = pixel position, zw = uv, byte16 = RGBA8 color) is decoded in the VS; ps
// multiplies the vertex color by the sampled font/texture.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/uv.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/color/srgb.hlsl"

struct PushConstants
{
    float2 screen_size;
};

[[vk::push_constant]]
PushConstants push_constants;

ByteAddressBuffer vertices : register(t13, space0);
Texture2D<float4> color_tex : register(t14, space0);

struct VsOut
{
    float4 color : TEXCOORD0;
    float2 uv : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    uint byte_base = 20u * vertex_id;
    uint pos_word = byte_base >> 2u;
    float4 pos = asfloat(uint4(
        vertices.Load<uint>(pos_word * 4u + 0u),
        vertices.Load<uint>((pos_word + 1u) * 4u + 0u),
        vertices.Load<uint>((pos_word + 2u) * 4u + 0u),
        vertices.Load<uint>((pos_word + 3u) * 4u + 0u)));

    uint col_word = (byte_base + 16u) >> 2u;
    uint packed = vertices.Load<uint>(col_word * 4u + 0u);
    float r_raw = float(packed & 255u);
    float g_raw = float((packed >> 8u) & 255u);
    float b_raw = float((packed >> 16u) & 255u);
    float a = float((packed >> 24u) & 255u) * (1.0f / 255.0f);

    // sRGB -> linear per channel (raw-byte arithmetic matches shipped codegen).
    float r = r_raw * 0.003921568859f;
    float g = g_raw * 0.003921568859f;
    float b = b_raw * 0.003921568859f;
    float rl;
    float gl;
    float bl;
    if (0.04045f < r) rl = pow((r + 0.055f) * 0.947867333f, 2.4f);
    else rl = r_raw * 0.000303526991f;
    if (0.04045f < g) gl = pow((g + 0.055f) * 0.947867333f, 2.4f);
    else gl = g_raw * 0.000303526991f;
    if (0.04045f < b) bl = pow((b + 0.055f) * 0.947867333f, 2.4f);
    else bl = b_raw * 0.000303526991f;

    o.position = float4((pos.xy / push_constants.screen_size - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.color = float4(rl, gl, bl, a);
    o.uv = pos.zw;
    return o;
}

// 原 SHADER_VARIANT_DEPTH_ONLY=1 的片元入口无资源、输入、输出或 discard。
#if SHADER_VARIANT_DEPTH_ONLY
void ps_main()
{
}
#else
float4 ps_main(VsOut vin) : SV_Target0
{
    return vin.color * color_tex.Sample(sampler_llc, vin.uv);
}
#endif
