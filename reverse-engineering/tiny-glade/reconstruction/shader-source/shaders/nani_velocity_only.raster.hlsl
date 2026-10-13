// ==RECONSTRUCTED== shaders/nani_velocity_only.raster.hlsl | B nani raster | bins: _nani_velocity_only.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani velocity-only raster: VS computes per-vertex motion from the current and
// previous instance transform (96-byte record = two Affine3Packed); PS writes
// the motion vector straight to the velocity target.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "nani/inc/user.hlsl"

StructuredBuffer<uint> instance_indirection_buffer : register(t14);

struct Affine3Packed
{
    float data[12];
};

struct InstanceVelocityData
{
    Affine3Packed xform;
    Affine3Packed prev_xform;
};

StructuredBuffer<InstanceVelocityData> nani_instance_data : register(t15);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed current_packed = nani_instance_data[idx].xform;
    Affine3Packed previous_packed = nani_instance_data[idx].prev_xform;
    float3x4 current_xform = transpose(float4x3(float3(current_packed.data[0], current_packed.data[1], current_packed.data[2]),
                                                float3(current_packed.data[3], current_packed.data[4], current_packed.data[5]),
                                                float3(current_packed.data[6], current_packed.data[7], current_packed.data[8]),
                                                float3(current_packed.data[9], current_packed.data[10], current_packed.data[11])));
    float3x4 previous_xform = transpose(float4x3(float3(previous_packed.data[0], previous_packed.data[1], previous_packed.data[2]),
                                                 float3(previous_packed.data[3], previous_packed.data[4], previous_packed.data[5]),
                                                 float3(previous_packed.data[6], previous_packed.data[7], previous_packed.data[8]),
                                                 float3(previous_packed.data[9], previous_packed.data[10], previous_packed.data[11])));
    float4 pos = float4(vin.Vertex_Position, 1.0f);
    float3 world = mul(current_xform, pos);
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = world - mul(previous_xform, pos);
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
};

struct PsOut
{
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    o.velocity = pin.C0;
    return o;
}
