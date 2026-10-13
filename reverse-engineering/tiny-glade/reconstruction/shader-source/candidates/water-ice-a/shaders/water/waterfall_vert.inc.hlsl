// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water/waterfall_vert.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN: _waterfall.raster.hlsl, _waterfall_refraction_copy.raster.hlsl
//   (2 bins each)
// Status: O (WaterfallMesh layout + 3D value-noise vertex displacement from the
//   inlined use-sites); I (function names)
//
// Waterfall vertex helpers: per-instance waterfall mesh (position/tangent/normal0/
// normal1 float4) and the time-animated 3D value-noise displacement.

#ifndef TINYGLADE_WATER_WATERFALL_VERT_HLSL
#define TINYGLADE_WATER_WATERFALL_VERT_HLSL

#include "inc/perlin.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"

// Per-instance waterfall mesh (stride 64 bytes in the instance buffer).
struct WaterfallMesh
{
    float4 position;
    float4 tangent;
    float4 normal0;
    float4 normal1;
};

// Load a waterfall mesh instance from the byte-address instance buffer.
WaterfallMesh load_waterfall_mesh(ByteAddressBuffer instance_buffer, uint instance_id)
{
    WaterfallMesh m;
    m.position = instance_buffer.Load<float4>(instance_id * 64u + 0u);
    m.tangent = instance_buffer.Load<float4>(instance_id * 64u + 16u);
    m.normal0 = instance_buffer.Load<float4>(instance_id * 64u + 32u);
    m.normal1 = instance_buffer.Load<float4>(instance_id * 64u + 48u);
    return m;
}

// 3D value-noise (8-corner hash lattice) at `p`, explicit-arithmetic form (the
// reconstructed inc/perlin.hlsl lerp form emits extra OpExtInst; consumers embed
// the explicit form — flag: inc issue).
float3 perlin3_explicit(float3 p)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);
    float3 c000 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x), uint(cell.y) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y), uint(cell.x) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z), uint(cell.x) + (uint(cell.y) << 10u)))));
    float3 c100 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x + 1), uint(cell.y) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y), uint(cell.x + 1) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z), uint(cell.x + 1) + (uint(cell.y) << 10u)))));
    float3 c010 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x), uint(cell.y + 1) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y + 1), uint(cell.x) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z), uint(cell.x) + (uint(cell.y + 1) << 10u)))));
    float3 c110 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x + 1), uint(cell.y + 1) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y + 1), uint(cell.x + 1) + (uint(cell.z) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z), uint(cell.x + 1) + (uint(cell.y + 1) << 10u)))));
    float3 c001 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x), uint(cell.y) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y), uint(cell.x) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z + 1), uint(cell.x) + (uint(cell.y) << 10u)))));
    float3 c101 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x + 1), uint(cell.y) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y), uint(cell.x + 1) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z + 1), uint(cell.x + 1) + (uint(cell.y) << 10u)))));
    float3 c011 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x), uint(cell.y + 1) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y + 1), uint(cell.x) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z + 1), uint(cell.x) + (uint(cell.y + 1) << 10u)))));
    float3 c111 = float3(incb_perlin_hash01(hash(uint2(uint(cell.x + 1), uint(cell.y + 1) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.y + 1), uint(cell.x + 1) + (uint(cell.z + 1) << 10u)))),
                         incb_perlin_hash01(hash(uint2(uint(cell.z + 1), uint(cell.x + 1) + (uint(cell.y + 1) << 10u)))));
    float3 r =
        (c000 * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z))
      + (c100 * u.x * (1.0f - u.y) * (1.0f - u.z))
      + (c010 * (1.0f - u.x) * u.y * (1.0f - u.z))
      + (c110 * u.x * u.y * (1.0f - u.z))
      + (c001 * (1.0f - u.x) * (1.0f - u.y) * u.z)
      + (c101 * u.x * (1.0f - u.y) * u.z)
      + (c011 * (1.0f - u.x) * u.y * u.z)
      + (c111 * u.x * u.y * u.z);
    return r;
}

// Compute the animated waterfall vertex (position + normal + world pos) for the
// given mesh and 2D grid coordinate (grid.x = across width, grid.y = down height).
void waterfall_vert(WaterfallMesh m, float2 grid, float time,
                    out float3 position, out float3 normal, out float3 world_pos)
{
    float3 base_normal = lerp(m.normal0.xyz, m.normal1.xyz, grid.x);
    float3 base_pos = (m.position.xyz + (float3(m.tangent.xy, 0.0f).xzy * grid.x)) + (base_normal * (grid.y * 0.5f));
    base_pos.y = lerp(m.position.w, m.position.y, 1.0f - pow(grid.y, 3.0f));

    float3 noise_p = (base_pos * float3(0.5f, 1.0f, 0.5f)) + (float3(0.0f, 3.0f, 0.0f) * time);
    float3 noise = perlin3_explicit(noise_p);
    position = base_pos + (((noise - 0.5f) * float3(1.0f, 0.0f, 1.0f)) * grid.y) * 0.20000000298023223876953125f;
    normal = normalize((base_normal * (-((m.position.y - m.position.w) * ((-3.0f) * pow(grid.y, 2.0f))))) + float3(0.0f, 0.5f, 0.0f));
    world_pos = position;
}

#endif // TINYGLADE_WATER_WATERFALL_VERT_HLSL
