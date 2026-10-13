// ==RECONSTRUCTED== shaders/session_selection/thumbnail.raster.hlsl | F-E | bins: _session_selection_thumbnail.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Session thumbnail tile: rounded-rect masked preview, hover highlight, and an
// optional black-and-white mode (e.g. for unavailable saves).

#include "inc/uv.hlsl"
#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 framebuffer_dims;
    float2 first_thumbnail_center;
    float2 thumbnail_size;
    float hover;
    uint thumbnail_index;
    uint is_black_and_white;
};

Texture2D<float4> thumbnail_tex : register(t13, space0);

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

    corner.w = 1.0f - corner.w;
    float2 pos = (corner.xy * 1.05f) * thumbnail_size + first_thumbnail_center;
    pos.x = pos.x / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 uv = lerp(corner.zw, float2(0.5f, 0.5f), -0.05f);
    uv.y = 1.0f - uv.y;
    o.position = float4((pos - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f);
    o.uv = uv;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float4 c = thumbnail_tex.SampleLevel(sampler_lnc, uv, 0.0f);
    float3 rgb = c.rgb * c.a;

    float2 d = abs(uv - 0.5f) - 0.225f;
    float mask = smoothstep(0.0f, -0.22f, (length(max(d, 0.0f)) + min(max(d.x, d.y), 0.0f)) - 0.3f);
    float4 col = float4(rgb, c.a) * mask;

    float3 shaded = col.rgb * lerp(1.0f, 1.2f, hover);
    float4 out_col;
    if (is_black_and_white == 1u)
        out_col = float4((dot(shaded, float3(0.2126f, 0.7152f, 0.0722f)) * 0.8f).xxx, col.a);
    else
        out_col = float4(shaded, col.w);
    return out_col;
}
