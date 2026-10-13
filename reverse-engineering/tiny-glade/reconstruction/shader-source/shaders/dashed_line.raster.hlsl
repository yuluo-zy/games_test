// ==RECONSTRUCTED== shaders/dashed_line.raster.hlsl | F-E | bins: _dashed_line.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Dashed line: per-vertex model_to_world transform; fragment discards every
// other dash cell (frac of the line-length parameter).

#include "inc/common.hlsl"            // samplers (unused here)
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    int vertex_count;
    float line_length;
};

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
};

struct VsOut
{
    float3 color : TEXCOORD0;
    float line_pos : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin, uint vertex_id : SV_VertexID)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.color = vin.Vertex_Color;
    o.line_pos = (float(vertex_id) / float(vertex_count - 1)) * line_length;
    return o;
}

float4 ps_main(VsOut vin) : SV_Target0
{
    if (frac(vin.line_pos) > 0.5f)
        discard;
    return float4(vin.color, 1.0f);
}
