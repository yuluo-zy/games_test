// ==RECONSTRUCTED== shaders/timed_terrain_gizmo.raster.hlsl | evidence: spirv-cross of _timed_terrain_gizmo.raster.hlsl_* (vs: arrow_index selects y/x axis stretch, world_to_sample output in sample space; ps: passthrough C1->SV_Target0); raw-per-bin $Globals = current_view_index/model_to_world/y_axis_stretch/x_axis_stretch b13 | status: O (bindings/entry/logic); I (names)

#include "inc/common.hlsl"
// current_view.hlsl is pulled in for its include closure; its global
// current_view_index is redeclared in our $Globals cbuffer below, so the
// include body is guard-skipped (the file's OpString is still recorded).
#define TINYGLADE_INC_CURRENT_VIEW_HLSL
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

// Active-view index + gizmo transform constants (single $Globals UBO at b13).
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    float y_axis_stretch;
    float x_axis_stretch;
};

struct VS_INPUT
{
    float3 position : TEXCOORD0;
    int arrow_index : TEXCOORD1;
};

struct VS_OUTPUT
{
    float4 position : SV_Position;
    float c1 : TEXCOORD1;
};

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(VS_INPUT input)
{
    VS_OUTPUT output;
    float3 local_pos;
    if ((input.arrow_index == 1) || (input.arrow_index == 3))
    {
        local_pos = input.position * (1.0f + pow(smoothstep(0.0f, 0.8f, y_axis_stretch), 0.5f));
    }
    else
    {
        local_pos = input.position * (1.0f + pow(smoothstep(0.0f, 0.8f, x_axis_stretch), 0.5f));
    }
    ViewConstants view = view_constants[current_view_index];
    output.position = mul(view.world_to_sample, mul(model_to_world, float4(local_pos, 1.0f)));
    output.c1 = 1.0f;
    return output;
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
float4 ps_main(float c1 : TEXCOORD1) : SV_Target0
{
    return c1.xxxx;
}
