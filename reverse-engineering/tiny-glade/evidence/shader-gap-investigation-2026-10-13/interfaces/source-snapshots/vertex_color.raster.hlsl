// ==RECONSTRUCTED== shaders/vertex_color.raster.hlsl | B simple-solid | bins: _vertex_color.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Trivial per-vertex-color raster: vertex position is transformed by the
// per-draw model_to_world then the active camera's world_to_sample; the
// fragment passes the vertex color through unchanged.
//
// $Globals (set0/b13): current_view_index + model_to_world.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

float4x4 model_to_world;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, mul(model_to_world, float4(vin.Vertex_Position, 1.0f)));
    o.C0 = vin.Vertex_Color;
    return o;
}

float4 ps_main(VsOut pin) : SV_Target0
{
    return float4(pin.C0, 1.0f);
}
