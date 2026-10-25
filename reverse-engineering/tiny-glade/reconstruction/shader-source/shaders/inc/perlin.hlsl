#pragma once
// ==RECONSTRUCTED== shaders/inc/perlin.hlsl | evidence: _leaf_particles/_paths_path_mask_visual/_water_plane/_rocky_terrain_displace inlined 2D value-noise (floor/frac/hash2D/smoothstep bilinear blend, lattice value = asfloat((h ^ (h>>18)) & 0x7FFFFF | 0x3F800000) - 1); include order shows perlin.hlsl includes hash.hlsl (water_plane: perlin(10), hash(11)); 133 consumer bins | status: I
// MERGE (water-ice review inc-fix #4): perlin() rewritten to the explicit-arithmetic
// bilinear blend (multiply-add of the four lattice values, not lerp()) — the lerp body
// emits extra OpExtInst in dxc SPIR-V, while the water/ice family consumers
// (water_plane foam, water_snow_plane, water_x_paths, waterfall) embed the explicit
// asfloat(hash & 0x7FFFFF | 0x3F800000) - 1 bilinear/trilinear form.
//
// Observation: this "perlin" is 2D *value* noise (hash lattice, not gradient), interpolated with
// the smoothstep curve f*f*(3-2f), returning [0,1).  The 2D hash (LCG mix with 1664525/1013904223)
// is defined here (not in INC-A hash.hlsl).  Function name not embedded -> "perlin" Inference (alt: perlin_noise).

#include "hash.hlsl"

// 2D integer hash (LCG mix with 1664525/1013904223 + final avalanche) observed inlined in
// _leaf_particles/_paths_path_mask_visual/_water_plane/_rocky_terrain_displace perlin lattice and
// direct consumer jitter.  Not present in INC-A hash.hlsl (which has hash(uint), rand01, ...), so it
// lives here; flag for reconciliation if INC-A later adds hash(uint2).
uint hash(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((p.x * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

float incb_perlin_hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    // Explicit bilinear blend (the family consumers embed this exact form).
    return (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
         + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
}
