// ==RECONSTRUCTED== shaders/loading_screen.raster.hlsl | F-E | bins: _loading_screen.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Loading-screen animation: a centered quad samples a Texture2DArray frame strip
// (8 layers driven by progress_time); black content (1 - sample.x) fades in/out.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 animation_dims;
    float progress_time;
    float fade;
    uint2 framebuffer_dims;
};

Texture2DArray<float4> animation : register(t13, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    float x = float(vertex_id & 1u);
    float y = float((vertex_id >> 1u) & 1u);
    float2 corner = float2(x, y) - 0.5f;
    corner.x = (corner.x * (float(animation_dims.x) / float(animation_dims.y)))
             / (float(framebuffer_dims.x) / float(framebuffer_dims.y));
    float2 pos = corner * 0.3f;
    o.position = float4(pos.x, pos.y - 0.6f, 1.0f, 1.0f);
    o.uv = float2(x, 1.0f - y);
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    int layer = int(progress_time * 8.0f) - 4 * (int(progress_time * 8.0f) / 4);
    float a = animation.Sample(sampler_lnc, float3(uv, float(layer))).x;
    return (1.0f - a) * (1.0f - fade) * smoothstep(3.0f, 3.5f, progress_time);
}
