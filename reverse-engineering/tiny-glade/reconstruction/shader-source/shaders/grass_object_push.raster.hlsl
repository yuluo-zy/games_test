// ==RECONSTRUCTED== shaders/grass_object_push.raster.hlsl
// Evidence: _grass_object_push.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V).
//   bindings: $Globals b13 (obj_position_radius float4). vs_main builds a 6-vertex quad
//   billboard around obj_position_radius (pos.xy + quad.xy*radius) scaled by 1/65 screen
//   units; ps_main outputs a soft radial "push" sprite (normalized offset * smooth falloff).
//   Status: O (structure verbatim).

#include "inc/uv.hlsl"
#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/quad_mesh.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float4 obj_position_radius;
};

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID)
{
    VS_OUT o;
    float4 quad;
    if (vertex_id == 0u)
    {
        quad = float4(0.5f, -0.5f, 1.0f, 0.0f);
    }
    else if (vertex_id == 1u)
    {
        quad = float4(0.5f, 0.5f, 1.0f, 1.0f);
    }
    else if (vertex_id == 2u)
    {
        quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    }
    else if (vertex_id == 3u)
    {
        quad = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    }
    else if (vertex_id == 4u)
    {
        quad = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    }
    else
    {
        quad = float4(0.5f, -0.5f, 1.0f, 0.0f);
    }
    o.position = float4((obj_position_radius.xz + (quad.xy * obj_position_radius.w))
        * float2(0.015384615398943424224853515625f, -0.015384615398943424224853515625f), 0.0f, 1.0f);
    o.C0 = quad.zw;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
};

struct PS_OUT
{
    float4 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float2 uv = i.C0 - 0.5f;
    float r = smoothstep(0.5f, 0.0f, length(uv));
    o.target = float4(normalize(uv) * r, r, 1.0f);
    return o;
}
