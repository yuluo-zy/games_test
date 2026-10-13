// ==RECONSTRUCTED== shaders/water/waterfall_vert.inc.hlsl
// SOURCE PATH : shaders/water/waterfall_vert.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN: _waterfall.raster.hlsl, _waterfall_refraction_copy.raster.hlsl
//   (2 bins each)
// Status: O (WaterfallMesh layout + 3D value-noise vertex displacement from the
//   inlined use-sites); I (function names)
// MERGE (water-ice review water/waterfall_vert.inc.hlsl, inc-fix #5): A's mesh layout
//   (position/tangent/normal0/normal1 float4, stride 64) + B's hash3 (3-component
//   uint3 LCG chain 1664525/1013904223, matching the bin's VS 8-corner chain) +
//   B's layout with vs_main living in this inc (the bin's VS entry lives in the inc).
//
// Waterfall vertex helpers: per-instance waterfall mesh and the vs_main entry.

#ifndef TINYGLADE_WATER_WATERFALL_VERT_HLSL
#define TINYGLADE_WATER_WATERFALL_VERT_HLSL

#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

// Per-instance waterfall mesh (stride 64 bytes in the instance buffer).
struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
};

ByteAddressBuffer instance_buffer : register(t14, space0);

// Per-corner 3-value hash (waterfall displacement noise), from the disassembly:
// LCG 1664525/1013904223 avalanche over the cell, three mantissa-random words per corner.
float3 waterfall_hash3(uint3 p)
{
    uint z = p.z;
    z += z << 10u; z ^= z >> 6u; z += z << 3u; z ^= z >> 11u;
    uint y = ((p.y * 1664525u) + (z + (z << 15u)) + 1013904223u) * 1664525u;
    y ^= y >> 11u; y ^= (y << 7u) & 2636928640u; y ^= (y << 15u) & 4022730752u;
    uint x = ((p.x * 1664525u) + (y ^ (y >> 18u)) + 1013904223u) * 1664525u;
    x ^= x >> 11u; x ^= (x << 7u) & 2636928640u; x ^= (x << 15u) & 4022730752u;
    uint r0 = x ^ (x >> 18u);
    uint t = r0 + (r0 << 10u); t ^= t >> 6u; t += t << 3u; t ^= t >> 11u;
    uint r1 = t + (t << 15u);
    t = r1 + (r1 << 10u); t ^= t >> 6u; t += t << 3u; t ^= t >> 11u;
    uint r2 = t + (t << 15u);
    float3 unit;
    unit.x = asfloat((r0 & 8388607u) | 1065353216u) - 1.0f;
    unit.y = asfloat((r1 & 8388607u) | 1065353216u) - 1.0f;
    unit.z = asfloat((r2 & 8388607u) | 1065353216u) - 1.0f;
    return unit;
}

struct WaterfallVSOut
{
    float3 c0 : TEXCOORD0;
    float3 c1 : TEXCOORD1;
    float4 pos : SV_Position;
};

WaterfallVSOut vs_main(uint VertexID : SV_VertexID, uint InstanceID : SV_InstanceID)
{
    WaterfallVSOut o;
    WaterfallMesh mesh;
    mesh.position = instance_buffer.Load<float4>(InstanceID * 64u + 0u);
    mesh.tangent = instance_buffer.Load<float4>(InstanceID * 64u + 16u);
    mesh.normal0 = instance_buffer.Load<float4>(InstanceID * 64u + 32u);
    mesh.normal1 = instance_buffer.Load<float4>(InstanceID * 64u + 48u);

    float2 g = float2(float(VertexID % 2u), float(VertexID / 2u)) * float2(1.0f, 0.1f);
    float3 n = lerp(mesh.normal0.xyz, mesh.normal1.xyz, g.x);

    float3 pos = (mesh.position.xyz + float3(mesh.tangent.xy, 0.0f).xzy * g.x) + n * (g.y * 0.5f);
    pos.y = lerp(mesh.position.w, mesh.position.y, 1.0f - pow(g.y, 3.0f));

    // Value-noise displacement over a moving world-space grid.
    float3 disp = (pos * float3(0.5f, 1.0f, 0.5f)) + float3(0.0f, 3.0f, 0.0f) * (float(game_time_sec) + game_time_frac);
    int3 cell = int3(floor(disp));
    float3 f = frac(disp);
    float3 sm = f * f * (3.0f - 2.0f * f);

    float3 noise = 0.0f;
    [unroll]
    for (uint zi = 0u; zi < 2u; ++zi)
    [unroll]
    for (uint yi = 0u; yi < 2u; ++yi)
    [unroll]
    for (uint xi = 0u; xi < 2u; ++xi)
    {
        float3 w = float3(xi == 0u ? (1.0f - sm.x) : sm.x,
                          yi == 0u ? (1.0f - sm.y) : sm.y,
                          zi == 0u ? (1.0f - sm.z) : sm.z);
        noise += waterfall_hash3(uint3(cell) + uint3(xi, yi, zi)) * (w.x * w.y * w.z);
    }

    float3 displaced = pos + ((noise - 0.5f) * float3(1.0f, 0.0f, 1.0f)) * g.y * 0.2f;
    o.pos = mul(view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    o.c0 = normalize((n * (-((mesh.position.y - mesh.position.w) * ((-3.0f) * pow(g.y, 2.0f)))))
        + float3(0.0f, 0.5f, 0.0f));
    o.c1 = displaced;
    return o;
}

#endif // TINYGLADE_WATER_WATERFALL_VERT_HLSL
