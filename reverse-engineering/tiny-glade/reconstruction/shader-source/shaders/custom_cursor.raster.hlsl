// ==RECONSTRUCTED== shaders/custom_cursor.raster.hlsl | F-E | bins: _custom_cursor.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Custom OS cursor quad driven by a CPU-written cursor_buf (position px,
// enabled flag, size px). Disabled cursor is moved off-screen via NaN position.

#include "inc/common.hlsl"            // samplers (sampler_llc s3)
#include "inc/quad_mesh.hlsl"
#include "inc/uv.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 window_size_pixels;
    uint2 cursor_tex_dims;
};

struct CustomCursor
{
    float2 pos;
    uint enabled;
    float size_px;
};

ByteAddressBuffer cursor_buf : register(t14, space0);
Texture2D<float4> cursor_tex : register(t15, space0);

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

    CustomCursor c;
    c.pos = cursor_buf.Load<float2>(0);
    c.enabled = cursor_buf.Load<uint>(8);
    c.size_px = cursor_buf.Load<float>(12);

    float2 screen_pos = (((corner.xy * float2(1.0f, -1.0f) + 0.5f) * c.size_px + (c.pos - 1.0f)) / float2(window_size_pixels)) - 0.5f;
    screen_pos *= float2(2.0f, -2.0f);
    float4 pos = float4(screen_pos, 1.0f, 1.0f);
    if (c.enabled == 0u)
        pos = float4(asfloat(0x7fbfffffu).xxxx);

    o.position = pos;
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = cursor_tex.SampleBias(sampler_llc, uv, -0.5f);
    return float4(c.rgb * c.a, c.a);
}
