// ==RECONSTRUCTED== shaders/leaf_particle.raster.hlsl | F-V | bins: _leaf_particle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Falling leaf particles: each instance carries a position / rotation / scale /
// up axis; the quad is oriented to the up axis with a horizontal alignment dir.
// ps discards outside the alpha texture and packs the solid gbuffer.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

// Leaf particle placement record (32-byte stride).
struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
};

StructuredBuffer<LeafParticle> leaf_placement_buffer : register(t14);
Texture2D<float4> leaf_alpha : register(t15);
Texture2D<float4> leaf_colors : register(t16);

uint2 leaf_alpha_dims;
uint2 leaf_colors_dims;
int bool_render_shadows;

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float2 Vertex_UV : TEXCOORD1;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float3 p_pos = leaf_placement_buffer[vin.instance_id].position;
    float p_rot = leaf_placement_buffer[vin.instance_id].rotation;
    float p_scale = leaf_placement_buffer[vin.instance_id].scale;
    float3 p_up = leaf_placement_buffer[vin.instance_id].up_axis;
    float3 scale3 = p_scale.xxx * 0.7f;
    float3 dir = normalize(cross(p_up, normalize(float3(cos(p_rot), 0.0f, sin(p_rot)))));

    float4x4 basis = float4x4(float4(cross(dir, p_up), 0.0f),
                               float4(p_up, 0.0f),
                               float4(dir, 0.0f),
                               float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4 m = mul(mul(float4x4(float4(1.0f, 0.0f, 0.0f, p_pos.x),
                               float4(0.0f, 1.0f, 0.0f, p_pos.y),
                               float4(0.0f, 0.0f, 1.0f, p_pos.z),
                               float4(0.0f, 0.0f, 0.0f, 1.0f)),
                      mul(transpose(basis), float4x4(float4(scale3.x, 0.0f, 0.0f, 0.0f),
                                                     float4(0.0f, scale3.y, 0.0f, 0.0f),
                                                     float4(0.0f, 0.0f, scale3.z, 0.0f),
                                                     float4(0.0f, 0.0f, 0.0f, 1.0f)))),
                  float4(vin.Vertex_Position, 1.0f));

    o.position = mul(GetCurrentViewConstants().world_to_sample, m);
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = m.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = float2(p_up.x + scale3.x, p_up.z + scale3.x);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (leaf_alpha.SampleBias(sampler_llc, pin.C3, -2.0f).x < 1.0f)
    {
        discard;
    }
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 normal = pin.C1;
    if (dot(normalize(cam_pos - pin.C2), pin.C1) < 0.0f)
    {
        normal = -pin.C1;
    }
    float3 packed_normal = (normalize(normal) * 0.5f) + 0.5f;
    float3 albedo = sqrt(clamp(leaf_colors.SampleLevel(sampler_llr, pin.C4, 0.0f).xyz, 0.0f, 1.0f));
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 2566914048u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 1174405120u;
    return o;
}
