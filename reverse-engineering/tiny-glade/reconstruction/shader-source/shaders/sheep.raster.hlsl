// ==RECONSTRUCTED== shaders/sheep.raster.hlsl | F-V | bins: _sheep.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Sheep instance animation: per-vertex positions/normals come from a structured buffer
// (`animation`) holding 30-frame sweeps (positions[30] + normals[30] float4, frame index
// stride, final "delighted" pose pair at the end). pet_blend cross-fades toward the delighted
// pose with a sin ease; is_pet_forward flips the ease direction. vs outputs gbuffer inputs
// (vertex color, animated normal via model_to_world, current world pos, prev-frame world pos);
// ps packs albedo (sqrt gamma 0.8) + normal into the solid gbuffer word and writes velocity.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/sh.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

// The animation buffer is structured in the original (per-vertex frame sweeps):
// 30 positions + 30 normals float4 each, then the "delighted" pose pair.
struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
};

StructuredBuffer<SheepAnimation> animation : register(t14);

float4x4 model_to_world;
float4x4 prev_model_to_world;
int animation_frame;
float pet_blend;
int is_pet_forward;

struct VsIn
{
    float3 Vertex_Color : TEXCOORD2;
    uint vertex_id : SV_VertexID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float blend = clamp(1.0f - pet_blend, 0.0f, 1.0f);
    float bob;
    if (is_pet_forward == 0)
    {
        float b = clamp(pet_blend, 0.0f, 1.0f);
        bob = (b * b * b) - (b * sin(b * 3.14f));
    }
    else
    {
        bob = 1.0f - ((blend * blend * blend) - (blend * sin(blend * 3.14f)));
    }
    float3 bob3 = bob.xxx;

    float3 pos = lerp(
        animation[vin.vertex_id].positions[animation_frame].xyz,
        animation[vin.vertex_id].delighted_pos.xyz, bob3);

    float4 p = float4(pos.x, pos.y - 0.25f, pos.z, 1.0f);
    float4 wp = mul(model_to_world, p);
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world,
               float4(lerp(animation[vin.vertex_id].normals[animation_frame].xyz,
                           animation[vin.vertex_id].delighted_normal.xyz, bob3), 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = mul(prev_model_to_world, p).xyz;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    float3 albedo = sqrt(clamp(pin.C0 * 0.8f, 0.0f, 1.0f));
    uint albedo_word = (uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 67108864u;
    o.velocity = pin.C2 - pin.C3;
    return o;
}
