// ==RECONSTRUCTED== shaders/nani_smoke.raster.hlsl | B nani raster | bins: _nani_smoke.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani smoke: VS builds a procedural smoke puff from the vertex id (ring of
// billboard quads) displaced by a 3D value-noise volume + per-instance
// thickness; PS alpha-discards the noise volume shell and packs a soft
// (normal, constant-albedo) gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

struct InstanceData
{
    Affine3Packed xform;
    uint smoke_type;
    uint pad0;
    uint pad1;
    float thickness;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);

uint hash3(uint3 p)
{
    p.y *= 1664525u;
    uint yz = p.z;
    yz += (yz << 10u);
    yz ^= (yz >> 6u);
    yz += (yz << 3u);
    yz ^= (yz >> 11u);
    p.y += yz + (yz << 15u);
    p.y += 1013904223u;
    p.y *= 1664525u;
    uint y = p.y ^ (p.y >> 11u);
    y = y ^ ((y << 7u) & 2636928640u);
    y = y ^ ((y << 15u) & 4022730752u);
    p.x *= 1664525u;
    p.x += y ^ (y >> 18u);
    p.x += 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    x = x ^ (x >> 18u);
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

float hash3_rand(uint h)
{
    return asfloat((h & 8388607u) | 1065353216u) - 1.0f;
}

// 3D value-noise over 8 lattice corners in [-1,1].
float noise3(float3 p)
{
    int3 cell = int3(floor(p));
    float3 fr = frac(p);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float n = 0.0f;
    n += hash3_rand(hash3(uint3(cell))) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += hash3_rand(hash3(uint3(cell + int3(1, 0, 0)))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += hash3_rand(hash3(uint3(cell + int3(0, 1, 0)))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += hash3_rand(hash3(uint3(cell + int3(1, 1, 0)))) * f.x * f.y * (1.0f - f.z);
    n += hash3_rand(hash3(uint3(cell + int3(0, 0, 1)))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += hash3_rand(hash3(uint3(cell + int3(1, 0, 1)))) * f.x * (1.0f - f.y) * f.z;
    n += hash3_rand(hash3(uint3(cell + int3(0, 1, 1)))) * (1.0f - f.x) * f.y * f.z;
    n += hash3_rand(hash3(uint3(cell + int3(1, 1, 1)))) * f.x * f.y * f.z;
    return n;
}

struct VsIn
{
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));

    if (nani_instance_data[idx].thickness < 0.1f)
    {
        o.position = asfloat(0x7fbfffffu).xxxx;
        o.C0 = 0.0f.xxx;
        o.C1 = 0.0f.xxx;
        o.C2 = 0.0f;
        return o;
    }

    float2 ring = float2(float(uint(vin.vertex_id) % 13u), float(uint(vin.vertex_id) / 13u))
        * float2(0.0833333f, 0.02f);
    float angle = ring.x * -6.2831855f;
    float time = game_time_sec + game_time_frac;
    float3 puff_local = float3(cos(angle), ring.y, sin(angle)) * float3(0.06f, 15.0f, 0.06f);
    puff_local.y += (frac(time * 4.3333f) - 1.0f) * 0.3f;
    float3 volume = (origin * 2.0f) + (puff_local * 2.0f) + (float3(0.0f, -2.0f, 0.0f) * time);
    float noise = noise3(volume);
    float vertical = puff_local.y * 0.0666667f;
    float thickness = nani_instance_data[idx].thickness;
    float2 spread = puff_local.xz * (lerp(0.5f, 5.0f, noise * clamp(1.0f - vertical, 0.0f, 1.0f))
        * (1.0f - smoothstep(0.0f, 1.0f, puff_local.y + lerp(0.0f, -10.0f, thickness))))
        * (thickness * thickness);
    float3 displaced;
    if (glade_theme_id == 4u)
    {
        displaced = float3(spread.x * 2.0f, puff_local.y, spread.y * 2.0f);
    }
    else
    {
        displaced = float3(spread.x, puff_local.y, spread.y);
    }
    if (nani_instance_data[idx].smoke_type == 1u)
    {
        displaced.z += smoothstep(0.2f, 0.9f, displaced.y) * 0.8f;
    }
    float3 world = mul(xform, float4(displaced, 1.0f));
    world.y -= 0.15f;

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = mul(xform, float4(normalize(float3(cos(angle), 0.0f, sin(angle))), 0.0f));
    o.C1 = world;
    o.C2 = vertical;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float C2 : TEXCOORD2;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (pin.C2 < 0.0f || pin.C2 > 1.0f)
    {
        discard;
    }
    float3 volume = (pin.C1 * 2.0f) + (float3(0.0f, -4.0f, 0.0f) * (game_time_sec + game_time_frac));
    if (noise3(volume) > smoothstep(0.8f, 0.1f, pin.C2))
    {
        discard;
    }
    float3 packed_normal = (normalize(pin.C0) * 0.5f) + 0.5f.xxx;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, 9211020u);
    o.gbuffer.y = 1183616140u;
    o.velocity = float3(0.0f, (1.3f * delta_time_seconds) * smoothstep(0.0f, 0.02f, pin.C2), 0.0f);
    return o;
}
