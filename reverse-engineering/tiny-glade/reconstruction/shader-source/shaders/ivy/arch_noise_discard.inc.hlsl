// ==RECONSTRUCTED== Shared helpers for the ivy instanced shaders: the arch-shaped discard test
// SOURCE PATH : shaders/ivy/arch_noise_discard.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 4 bins):
//   _ivy_instanced_ivy_branch.raster.hlsl_b903e43ffb3da915.bin, _ivy_instanced_ivy_branch.raster.hlsl_f0adff76827e73ff.bin, _ivy_instanced_ivy_leaf.raster.hlsl_b903e43ffb3da915.bin, _ivy_instanced_ivy_leaf.raster.hlsl_f0adff76827e73ff.bin
// ==RECONSTRUCTED==
// Shared helpers for the ivy instanced shaders: the arch-shaped discard test
// (path-mask fade times a 2D value-noise threshold) and the 3D value-noise
// displacement used to give ivy branches/leaves organic arching. All helper
// bodies are inlined into the consumer vertex shaders; names are Inference.
//
// The 2D and 3D value noises here deliberately use the bilinear *product*
// form (not lerp) because that is the exact instruction pattern observed in
// the compiled SPIR-V (OpFAdd/OpFMul, no OpExtInst FMix from the noise blend).
// The 3D lattice hash mixes (y,z) with the 2D lattice hash then folds x in;
// each cell yields a float2 of lattice values (v0 from the hash, v1 from a
// re-hash of the top bits). The per-corner LCG seed constants are selected by
// a runtime bool (path-mask hit) against an uninitialized seed, which the
// compiler turns into OpSelect chains.

#ifndef TINYGLADE_IVY_ARCH_NOISE_DISCARD_HLSL
#define TINYGLADE_IVY_ARCH_NOISE_DISCARD_HLSL

#include "inc/hash.hlsl"

// 2D lattice hash (LCG mix with 1664525/1013904223 + final avalanche).
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

// Lattice value in [-1,1] from a hashed cell (mantissa bitcast trick).
float hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

// 2D value noise (bilinear product blend, no lerp).
float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = hash01(hash(uint2(cell + int2(1, 1))));
    return (1.0f - u.y) * (n00 * (1.0f - u.x) + n10 * u.x)
         + u.y * (n01 * (1.0f - u.x) + n11 * u.x);
}

// 3D lattice hash. First mixes the (y,z) pair with the 2D lattice hash, then
// folds x in with a second mix (the high 18 bits of the yz hash are re-mixed).
uint hash3(uint3 p)
{
    uint h = hash(uint2(p.y, p.z));
    h = ((p.x * 1664525u) + (h ^ (h >> 18u)) + 1013904223u) * 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

// Two lattice values (in [-1,1]) for a 3D cell, with the LCG seeds selected by
// `masked` against an (uninitialized) alternate seed. v0 comes from the lattice
// hash, v1 from re-hashing the hash's top 14 bits.
float2 hash3_01(uint3 cell, bool masked, uint seed)
{
    uint s1 = masked ? 1664525u : seed;
    uint s2 = masked ? 1013904223u : seed;
    uint mask = masked ? 8388607u : seed;
    uint off = masked ? 1065353216u : seed;

    uint hz = cell.z;
    hz += (hz << 10u);
    hz ^= (hz >> 6u);
    hz += (hz << 3u);
    hz ^= (hz >> 11u);
    uint h = ((cell.y * s1) + (hz + (hz << 15u)) + s2) * s1;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = ((cell.x * s1) + (h ^ (h >> 18u)) + s2) * s1;
    hx ^= (hx >> 11u);
    hx ^= (hx << 7u) & 2636928640u;
    hx ^= (hx << 15u) & 4022730752u;

    float2 v;
    v.x = asfloat(((hx ^ (hx >> 18u)) & mask) | off) - 1.0f;
    uint h2 = hx ^ (hx >> 18u);
    h2 += (h2 << 10u);
    h2 ^= (h2 >> 6u);
    h2 += (h2 << 3u);
    h2 ^= (h2 >> 11u);
    h2 += (h2 << 15u);
    v.y = asfloat((h2 & mask) | off) - 1.0f;
    return v;
}

// 3D value noise (two components) at frequency `p`, trilinearly blended.
float2 perlin3(float3 p, bool masked, uint seed)
{
    int3 cell = int3(floor(p));
    float3 f = frac(p);
    float3 u = f * f * (3.0f - 2.0f * f);

    float2 n = hash3_01(uint3(cell + int3(0, 0, 0)), masked, seed)
        * (1.0f - u.x) * (1.0f - u.y) * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(1, 0, 0)), masked, seed)
        * u.x * (1.0f - u.y) * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(0, 1, 0)), masked, seed)
        * (1.0f - u.x) * u.y * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(1, 1, 0)), masked, seed)
        * u.x * u.y * (1.0f - u.z);
    n += hash3_01(uint3(cell + int3(0, 0, 1)), masked, seed)
        * (1.0f - u.x) * (1.0f - u.y) * u.z;
    n += hash3_01(uint3(cell + int3(1, 0, 1)), masked, seed)
        * u.x * (1.0f - u.y) * u.z;
    n += hash3_01(uint3(cell + int3(0, 1, 1)), masked, seed)
        * (1.0f - u.x) * u.y * u.z;
    n += hash3_01(uint3(cell + int3(1, 1, 1)), masked, seed)
        * u.x * u.y * u.z;
    return n;
}

// Arch-shape discard: at a world position whose terrain height is `height`,
// with `arch_height` from the segment and `path` from the path mask, returns
// whether the ivy should be culled (its vertex pushed to NaN). The strength
// fades in toward the arch center and is shaped by 2D value noise.
bool arch_noise_discard(float3 pos, float height, float arch_height, float path)
{
    float base = arch_height - min(height, 0.0f);
    float fade = (1.0f - exp(-5.0f * path)) * base;
    float n = perlin(pos.xz * 0.6f + pos.y * 0.6f);
    float t = clamp(base - pos.y, 0.0f, 1.0f);
    return pos.y < fade * (t * t * clamp(smoothstep(0.0f, 0.3f, n), 0.8f, 1.0f));
}

#endif // TINYGLADE_IVY_ARCH_NOISE_DISCARD_HLSL
