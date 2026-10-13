// ==RECONSTRUCTED== shaders/timer_circle.raster.hlsl | F-E | bins: _timer_circle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Circular countdown timer drawn as a billboard quad in world space. The ps
// renders a ring whose filled arc grows with timer_progress.

#include "inc/common.hlsl"            // samplers (unused here)
#include "inc/quad_mesh.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float timer_progress;
    float4x4 model_to_world;
};


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

    float4 local = float4(corner.xy * 1.5f, -4.0f, 1.0f);
    o.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, local));
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    float2 p = uv * 2.0f - 1.0f;
    float dist = length(p);

    float ring = min(smoothstep(0.5f, 0.55f, dist), 1.0f - smoothstep(0.95f, 1.0f, dist));
    float progress = clamp(timer_progress, 0.0f, 1.0f);
    float arc = lerp(-3.1415927f, 3.1415927f, 1.0f - smoothstep(0.0f, 0.9f, progress));
    if (atan2(p.y, p.x) < arc)
        ring = min(ring, 0.3f);

    return ring.xxxx;
}
