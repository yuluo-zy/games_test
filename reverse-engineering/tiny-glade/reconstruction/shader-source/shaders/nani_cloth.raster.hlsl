// ==RECONSTRUCTED== shaders/nani_cloth.raster.hlsl | B nani raster | bins: _nani_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani cloth: VS flutters the cloth with two layered 2D wind lattices + a
// vertical sine wave, offsets the bottom hem per-instance; PS picks the cloth
// color by palette id, jitters hue/sat/val, blends a ferris pattern and samples
// a bindless normal map; packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/hsv.hlsl"
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

uint2 ferris_dims;

struct InstanceData
{
    Affine3Packed xform;
    float hem_offset_low;
    float hem_offset_high;
    uint color_id;
    uint albedo_texture;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> ferris : register(t16);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

// 2D value-noise over 4 lattice corners in [-1,1].
float noise2(float2 p)
{
    int2 cell = int2(floor(p));
    float2 fr = frac(p);
    float2 f = fr * fr * (3.0f.xx - (fr * 2.0f));
    float n = 0.0f;
    n += hash2_rand(hash2(uint2(cell))) * (1.0f - f.x) * (1.0f - f.y);
    n += hash2_rand(hash2(uint2(cell + int2(1, 0)))) * f.x * (1.0f - f.y);
    n += hash2_rand(hash2(uint2(cell + int2(0, 1)))) * (1.0f - f.x) * f.y;
    n += hash2_rand(hash2(uint2(cell + int2(1, 1)))) * f.x * f.y;
    return n;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Color : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    float3 tangent : TEXCOORD4;
    float3 bitangent : TEXCOORD5;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    nointerpolation uint C0 : TEXCOORD0;
    nointerpolation int C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float3x3 C6 : TEXCOORD6;
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
    float time = game_time_sec + game_time_frac;

    uint seed = uint(nani_instance_data[idx].color_id);
    seed = hash(seed);
    float flutter_seed = hash2_rand(seed);

    // Layered wind lattices (cloth + roof flutter).
    float2 wind_a = (origin.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
    float flutter_a = noise2(wind_a);
    float2 flutter_b = float2(time + origin.x, origin.y);
    float flutter_b_noise = noise2(flutter_b);

    float phase = sin(fmod(time * 1.8f, 6.2831855f) + game_time_frac * 1.8f + vin.Vertex_Position.y + flutter_seed * 0.5f);
    float z_flutter = phase + flutter_b_noise;
    z_flutter *= smoothstep(0.0f, -1.0f, vin.Vertex_Position.y);

    float gust = sin(((flutter_a * 3.14f) - 1.57f + 0.3f) * 0.7f) * lerp(0.7f, 1.0f, flutter_seed) * 0.2f;
    float displacement = (z_flutter * lerp(0.3f, 1.0f, gust)) * lerp(0.7f, 1.0f, flutter_seed) * 0.2f;
    float new_z = vin.Vertex_Position.z + displacement;

    float3 world = mul(xform, float4(vin.Vertex_Position.xy, new_z, 1.0f));
    world.y += lerp(nani_instance_data[idx].hem_offset_low, nani_instance_data[idx].hem_offset_high,
        (vin.Vertex_Position.x * 1.4285715f) + 0.5f);

    float3 normal_world = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = idx;
    o.C1 = int((origin.x + origin.y) + origin.z);
    o.C2 = vin.Vertex_Color;
    o.C3 = normal_world;
    o.C4 = world;
    o.C5 = float2(vin.Vertex_UV.x, 1.0f - vin.Vertex_UV.y);
    o.C6 = transpose(float3x3(normalize(mul(xform, float4(vin.tangent, 0.0f))),
                              normalize(mul(xform, float4(vin.bitangent, 0.0f))),
                              normalize(normal_world)));
    return o;
}

struct PsIn
{
    nointerpolation uint C0 : TEXCOORD0;
    nointerpolation int C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float3x3 C6 : TEXCOORD6;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    uint color_id = uint(nani_instance_data[pin.C0].color_id);
    uint palette_idx = color_id % 7u;
    float3 base_color;
    if (palette_idx == 0u)
    {
        base_color = 0.7f.xxx;
    }
    else if (palette_idx == 1u)
    {
        base_color = float3(0.1137f, 0.15f, 0.0675f);
    }
    else if (palette_idx == 2u)
    {
        base_color = float3(0.049765f, 0.132287f, 0.11814f);
    }
    else if (palette_idx == 3u)
    {
        base_color = float3(0.173464f, 0.166082f, 0.064536f);
    }
    else
    {
        base_color = palette_idx == 4u ? float3(0.450245f, 0.265458f, 0.085742f) : 0.5f.xxx;
    }

    // RGB -> HSV with per-instance hue/sat/val jitter (explicit channel sort).
    float4 a;
    if (base_color.y < base_color.z)
    {
        a = float4(base_color.zy, -1.0f, 0.666667f);
    }
    else
    {
        a = float4(base_color.yz, 0.0f, -0.333333f);
    }
    float4 b;
    if (base_color.x < a.x)
    {
        b = float4(a.xyw, base_color.x);
    }
    else
    {
        b = float4(base_color.x, a.yzx);
    }
    float delta = b.x - min(b.w, b.y);
    uint seed_a = hash(uint(pin.C1) ^ 7230u);
    float hue = clamp(abs(((b.w - b.y) / ((6.0f * delta) + 1e-10f)) + b.z) + lerp(-0.1f, 0.05f, hash2_rand(seed_a)), 0.1f, 0.9f) * 6.0f;
    uint seed_b = hash(uint(pin.C1) ^ 890348u);
    float3 retinted = ((clamp(float3(abs(hue - 3.0f) - 1.0f, 2.0f - abs(hue - 2.0f), 2.0f - abs(hue - 4.0f)), 0.0f.xxx, 1.0f.xxx) - 1.0f.xxx) * (delta / (b.x + 1e-10f)) + 1.0f.xxx) * (b.x + lerp(0.0f, 0.1f, hash2_rand(seed_b)));

    if ((color_id % 10u) == 1u)
    {
        float4 ferris_sample = ferris.Sample(sampler_llr, pin.C5 * 2.0f);
        retinted = lerp(retinted, pow(ferris_sample.xyz, 1.3f.xxx), (ferris_sample.w * 0.6f).xxx);
    }

    uint tex_idx = nani_instance_data[pin.C0].albedo_texture;
    float3 normal_map = bindless_textures_dyn[NonUniformResourceIndex(tex_idx)].Sample(sampler_llr, pin.C5).xyz * 2.0f - 1.0f.xxx;
    float3 normal = normalize(mul(pin.C6, normal_map));
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    if (dot(normalize(cam_pos - pin.C4), normal) <= 0.0f)
    {
        normal = -normal;
    }

    float3 packed_normal = (normalize(normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(retinted, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}
