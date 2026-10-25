// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water/waves.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN: _water_plane.raster.hlsl, _waterfall.raster.hlsl (2 bins each)
// Status: O (4-octave rotated water-noise displacement + ripple-source sum from the
//   inlined use-sites); I (function names; exact octave table)
//
// Water surface wave displacement helpers shared by the water plane and waterfall
// fragment shaders.

#ifndef TINYGLADE_WATER_WAVES_HLSL
#define TINYGLADE_WATER_WAVES_HLSL

#include "inc/math.hlsl"
#include "inc/math_const.hlsl"

// Octave scales / weights observed in the consumers (_246 = {2,4,8,12} scale,
// _247 = {2,2,0.5,0.2} weight, _248 = {17,18,20,21} seed).
static const float WAVE_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WAVE_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WAVE_SEED[4] = { 17u, 18u, 20u, 21u };

// Per-octave rotated water-noise sample coordinate. `seed` is the octave seed;
// `dir_out` returns the octave rotation direction. Mirrors the inlined 4-octave
// hash + smoothstep blend of the consumers.
float2 water_wave_offset(uint octave, float3 world_xz, float time)
{
    float seed = float(WAVE_SEED[octave]);
    int cell = int(floor(seed));
    float f = frac(seed);
    uint c0 = uint(cell);
    uint c1 = uint(cell + 1);
    float2 d0 = (incb_perlin_hash01(hash(uint2(c0, c0))) * 2.0f - 1.0f).xx;
    float2 d1 = (incb_perlin_hash01(hash(uint2(c1, c1))) * 2.0f - 1.0f).xx;
    float u = f * f * (3.0f - 2.0f * f);
    return (normalize(lerp(d0, d1, u)) - 0.5f) * time * 3.0f;
}

// Sum the 4 rotated water-noise octaves into a displacement vector.
float3 water_wave_displacement(float3 world_xz, float time)
{
    float3 acc = 0.0f;
    for (uint i = 0u; i < 4u; ++i)
    {
        float2 r = water_wave_offset(i, world_xz, time);
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

// Sum the ripple-source displacements near `world_xz` from the byte-address
// ripple buffer (each record: half2 start, half2 end, float amplitude).
float3 water_ripple_displacement(ByteAddressBuffer ripple_source_buffer, float3 world_xz, float time)
{
    float3 acc = 0.0f;
    uint count = min(32u, ripple_source_buffer.Load<uint>(512));
    for (uint i = 0u; i < count; ++i)
    {
        uint4 rec = ripple_source_buffer.Load<uint4>(i * 16u + 0u);
        float2 a = float2(unpack_half2x16(rec.x & 65535u).x, unpack_half2x16((rec.x >> 16u) & 65535u).x);
        float2 b = float2(unpack_half2x16(rec.y & 65535u).x, unpack_half2x16((rec.y >> 16u) & 65535u).x);
        float2 mid = lerp(a, b, 0.5f);
        float2 d = mid - world_xz.xz;
        float len_d = length(d);
        float2 vel = (b - a) * 3.0f;
        float wave = len_d + dot(d, vel);
        float amp = (len_d > wave) ? len_d : wave;
        float damp = sin((wave * 16.3649997711181640625f) - (time * 6.0f)) / (1.0f + ((amp * amp) * (amp * amp)));
        acc += float3(damp, damp * 0.5f, damp * 0.5f) * asfloat(rec.z);
    }
    return acc;
}

#endif // TINYGLADE_WATER_WAVES_HLSL
