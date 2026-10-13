// ==RECONSTRUCTED== shaders/water/waves.inc.hlsl
// SOURCE PATH : shaders/water/waves.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN: _leaf_particles/_lilypad/_lilypad_flower/_reed/_water_plane/_waterfall
//   (11 bins total)
// Status: O (4-octave rotated water-noise displacement + ripple-source sum from the
//   inlined use-sites); I (function names)
// MERGE (water-ice review water/waves.inc.hlsl, inc-fix #5): A's wave tables
//   {2,4,8,12}/{2,2,0.5,0.2}/{17,18,20,21} + 2-lattice smoothstep blend + ripple damp
//   formula; B's scalar two-component hash01 (x from h, y from h+(h<<15)) for the
//   per-octave offset (A replicated .xx — wrong, the bin's x != y); added the bin's
//   3-phase central-difference ripple normal (sin(w1), (sin(w2)-sin(w1))*0.5,
//   (sin(w3)-sin(w1))*0.5) at +0.01 offsets — neither candidate captured it.
//
// Water surface wave displacement helpers shared by the water plane and waterfall
// fragment shaders.

#ifndef TINYGLADE_WATER_WAVES_HLSL
#define TINYGLADE_WATER_WAVES_HLSL

#include "inc/math.hlsl"
#include "inc/math_const.hlsl"

// Octave scales / weights / seeds observed in the consumers (_246 = {2,4,8,12} scale,
// _247 = {2,2,0.5,0.2} weight, _248 = {17,18,20,21} seed).
static const float WAVE_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WAVE_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WAVE_SEED[4] = { 17u, 18u, 20u, 21u };

// Scalar two-component hash01 of a lattice cell (bin: x from the avalanche hash h,
// y from the re-hash chain of h with +<<15, so x != y). The cell hash is the
// perlin 2D-lattice hash with p.x == p.y == cell.
float2 waves_hash01_2(uint cell)
{
    uint y = cell;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((cell * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    uint hx = h ^ (h >> 18u);
    uint hy = hx;
    hy += (hy << 10u);
    hy ^= (hy >> 6u);
    hy += (hy << 3u);
    hy ^= (hy >> 11u);
    return float2(asfloat((hx & 0x7FFFFFu) | 0x3F800000u) - 1.0f,
                  asfloat(((hy + (hy << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f);
}

// Per-octave rotated water-noise sample coordinate. `octave` selects the seed table;
// the offset is a 2-lattice smoothstep blend of the two-component hash01 vectors,
// scaled by time (mirrors the bin's (normalize(lerp(...)) - 0.5) * time * 3).
float2 water_wave_offset(uint octave, float time)
{
    float seed = float(WAVE_SEED[octave]);
    int cell = int(floor(seed));
    float f = frac(seed);
    uint c0 = uint(cell);
    uint c1 = uint(cell + 1);
    float u = f * f * (3.0f - 2.0f * f);
    float2 d0 = waves_hash01_2(c0);
    float2 d1 = waves_hash01_2(c1);
    return (normalize(lerp(d0, d1, u)) - 0.5f) * time * 3.0f;
}

// Sum the 4 rotated water-noise octaves into a displacement vector.
float3 water_wave_displacement(float3 world_xz, float time)
{
    float3 acc = 0.0f;
    for (uint i = 0u; i < 4u; ++i)
    {
        float2 r = water_wave_offset(i, time);
        float angle = float(i + 1u) * 10.16640758514404296875f;
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float3 sample_v = (water_noise_texture.Sample(sampler_llr,
            mul(rot, (world_xz.xz * WAVE_SCALE[i]) + r) * 0.01200000010430812835693359375f).xyz
            * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f);
        acc = float3(acc.x, acc.yz + mul(((sample_v * WAVE_WEIGHT[i])
            * float2(1.0f, WAVE_SCALE[i] * 0.01200000010430812835693359375f).xyy).yz, rot));
    }
    return acc;
}

// Ripple-ring falloff of one source at a world xz point:
// sin((ring*16.365) - time*6) / (1 + max(len,ring)^4), ring = len + dot(d, vel).
float waves_ripple_ring(float2 mid, float2 vel, float2 p, float time)
{
    float2 d = mid - p;
    float len_d = length(d);
    float ring = len_d + dot(d, vel);
    float amp = max(len_d, ring);
    return sin((ring * 16.3649997711181640625f) - (time * 6.0f)) / (1.0f + ((amp * amp) * (amp * amp)));
}

// Sum the ripple-source displacements near `world_xz` from the byte-address
// ripple buffer (each record: half2 start, half2 end, float amplitude). Returns
// the 3-phase central-difference normal (sin(w1), (sin(w2)-sin(w1))*0.5,
// (sin(w3)-sin(w1))*0.5) at +0.01 world-xz offsets.
float3 water_ripple_displacement(ByteAddressBuffer ripple_source_buffer, float3 world_xz, float time)
{
    float3 acc = 0.0f;
    uint count = min(32u, ripple_source_buffer.Load<uint>(512));
    for (uint i = 0u; i < count; ++i)
    {
        uint4 rec = ripple_source_buffer.Load<uint4>(i * 16u + 0u);
        float2 a = float2(f16tof32(rec.x & 0xFFFFu), f16tof32((rec.x >> 16u) & 0xFFFFu));
        float2 b = float2(f16tof32(rec.y & 0xFFFFu), f16tof32((rec.y >> 16u) & 0xFFFFu));
        float2 mid = lerp(a, b, 0.5f);
        float2 vel = (b - a) * 3.0f;
        float w1 = waves_ripple_ring(mid, vel, world_xz.xz, time);
        float w2 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.00999999977648258209228515625f, 0.0f), time);
        float w3 = waves_ripple_ring(mid, vel, world_xz.xz - float2(0.0f, 0.00999999977648258209228515625f), time);
        acc += float3(w1, (w2 - w1) * 0.5f, (w3 - w1) * 0.5f) * asfloat(rec.z);
    }
    return acc;
}

#endif // TINYGLADE_WATER_WAVES_HLSL
