// ==RECONSTRUCTED== shaders/critters/nani_wren.raster.hlsl | target: _critters_nani_wren.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main | evidence: spirv-cross + SPIR-V access/type inspection (instance indirection StructuredBuffer<uint> -> InstanceData with two Affine3Packed xforms, flap/wing-fold blend toward rest pose, bird-texture gbuffer) | bindings: $Globals b13, instance_indirection_buffer t14, nani_instance_data t15, frame_constants set1/b0; ps: sampler_mipmap s10, bird_texture t16 | status: R (reconstruction)

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/utils.hlsl"
#include "inc/math_const.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

// Nani-instance record for a wren (112-byte stride): two packed transforms + wing blend fields.
struct InstanceData
{
    Affine3Packed xform;      // +0
    Affine3Packed prev_xform; // +48
    float flap_t;             // +96
    float wing_fold_t;        // +100
    int id;                   // +104
    uint pad0;                // +108
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> bird_texture : register(t16);

uint2 bird_texture_dims;
int bool_render_shadows;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    float3 rest_pos : TEXCOORD4;
    float3 rest_n : TEXCOORD5;
    float wing_t : TEXCOORD7;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint inst = instance_indirection_buffer[vin.instance_id];
    Affine3Packed xform = nani_instance_data[inst].xform;
    Affine3Packed prev_xform = nani_instance_data[inst].prev_xform;
    float flap_t = nani_instance_data[inst].flap_t;
    float wing_fold_t = nani_instance_data[inst].wing_fold_t;
    int id = nani_instance_data[inst].id;

    float3 _155 = vin.Vertex_Position * 0.4000000059604644775390625f;
    _155.y = _155.y + ((vin.wing_t * 0.4000000059604644775390625f) * sin((flap_t * 6.283185482025146484375f) - (vin.wing_t * 0.31415927410125732421875f)));
    float3 _166 = wing_fold_t.xxx;

    float3x4 _174 = transpose(float4x3(
        float3(xform.data[0], xform.data[1], xform.data[2]),
        float3(xform.data[3], xform.data[4], xform.data[5]),
        float3(xform.data[6], xform.data[7], xform.data[8]),
        float3(xform.data[9], xform.data[10], xform.data[11])));
    float4 _178 = float4(lerp(_155, vin.rest_pos * 0.4000000059604644775390625f, _166), 1.0f);
    float3 _179 = mul(_174, _178);

    float2 _180 = vin.Vertex_UV * 0.5f;
    float _181 = _180.y;
    float2 _183 = _180;
    _183.y = _181 + 0.5f;
    int _184 = id - 8 * (id / 8);
    float2 _189;
    if (_184 == 3)
    {
        float2 _188 = _183;
        _188.y = _181;
        _189 = _188;
    }
    else
    {
        _189 = _183;
    }
    float2 _196;
    if (_184 == 4)
    {
        float2 _195 = _189;
        _195.x = _189.x - 0.5f;
        _196 = _195;
    }
    else
    {
        _196 = _189;
    }
    float2 _201;
    if (_184 == 5)
    {
        _201 = _196 - 0.5f.xx;
    }
    else
    {
        _201 = _196;
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(_179, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(_174, float4(lerp(vin.Vertex_Normal, vin.rest_n, _166), 0.0f));
    o.C2 = _179;
    o.C3 = _201;
    o.C4 = mul(transpose(float4x3(
        float3(prev_xform.data[0], prev_xform.data[1], prev_xform.data[2]),
        float3(prev_xform.data[3], prev_xform.data[4], prev_xform.data[5]),
        float3(prev_xform.data[6], prev_xform.data[7], prev_xform.data[8]),
        float3(prev_xform.data[9], prev_xform.data[10], prev_xform.data[11]))), _178);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 _111 = sqrt(clamp(bird_texture.SampleBias(sampler_mipmap, pin.C3, global_mip_bias).xyz, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(_111.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(_111.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(_111.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C4;
    return o;
}
