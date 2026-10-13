// ==RECONSTRUCTED== shaders/water/waves.inc.hlsl | evidence: _water_plane.raster.hlsl and
// _waterfall.raster.hlsl PS modules (both embed this file) + spirv-cross: rotating 4-octave
// water_noise_texture sampling (frequencies 2/4/8/12, amplitudes 2/2/0.5/0.2, per-octave hash
// offsets, rotation 10.1664 * n) plus a ripple-source accumulation (RippleSourceSsbo, sin ring
// falloff 16.365 * t, half-float positions). status: O (algorithm + constants); O/I (helper names
// and the per-octave hash offsets — inlined in SPIR-V).

#ifndef TINYGLADE_WATER_WAVES_INC_HLSL
#define TINYGLADE_WATER_WAVES_INC_HLSL

#include "inc/samplers.hlsl"

static const float WATER_SAMPLE_SCALE = 0.01200000010430812835693359375f; // 1.5 / 125
static const float WATER_RIPPLE_K = 16.3649997711181640625f;
static const float WATER_RIPPLE_SPEED = 6.0f;

// Per-octave rotating water-noise sample. `seed` advances a small hash chain so each octave
// decorrelates its grid offset (observed as an unrolled 4-step chain in the disassembly).
float2 water_octave_offset(uint seed)
{
    uint h = seed;
    h += h << 10u;
    h ^= h >> 6u;
    h += h << 3u;
    h ^= h >> 11u;
    h = ((h * 1664525u) + 1013904223u) * 1664525u;
    h ^= h >> 11u;
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return float2(asfloat((h & 8388607u) | 1065353216u) - 1.0f,
                  asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f) - 0.5f;
}

// Accumulate the rotating 4-octave water-noise slope (yz-plane vector) and return the perturbed
// normal (x = slope, y = up, z = slope).
float3 water_wave_normal(float2 world_xz, float time, inout uint2 pixel_seed)
{
    float3 acc = 0.0f;
    for (uint i = 0u; i < 4u; ++i)
    {
        float ang = float(i + 1u) * 10.16640758514404296875f;
        float cs = cos(ang);
        float sn = sin(ang);
        float2x2 rot = float2x2(cs, sn, -sn, cs);
        float freq = (i == 0u) ? 2.0f : (i == 1u) ? 4.0f : (i == 2u) ? 8.0f : 12.0f;
        float amp = (i == 0u) ? 2.0f : (i == 1u) ? 2.0f : (i == 2u) ? 0.5f : 0.2f;
        float2 off = water_octave_offset(uint(frame_index) + (i * 137u));
        float2 uv = mul(rot, (world_xz * freq) + off) * WATER_SAMPLE_SCALE;
        float3 sample3 = ((water_noise_texture.Sample(sampler_llr, uv).xyz * float3(1.0f, 2.0f, 2.0f))
            + float3(-0.5f, -1.0f, -1.0f)) * amp;
        float2 v = acc.yz + mul(rot, float2(sample3.x, sample3.y));
        acc = float3(acc.x, v.x, v.y);
    }
    return acc;
}

// Accumulate the ripple sources (sin ring falloff around each source).
float3 water_ripple_accum(float2 world_xz, float time)
{
    uint count = min(32u, ripple_source_buffer.Load<uint>(512));
    float3 acc = 0.0f;
    for (uint i = 0u; i < count; ++i)
    {
        uint4 src = ripple_source_buffer.Load<uint4>(i * 16u + 0u);
        float2 a = float2(f16tof32(src.x & 0xFFFFu), f16tof32((src.x >> 16u) & 0xFFFFu));
        float2 b = float2(f16tof32(src.y & 0xFFFFu), f16tof32((src.y >> 16u) & 0xFFFFu));
        float2 mid = lerp(a, b, 0.5f);
        float2 d = mid - world_xz;
        float len = length(d);
        float2 dir = (b - a) * 3.0f;
        float ring = len + dot(d, dir);
        float amp = asfloat(src.z);
        float ripple = sin((ring * WATER_RIPPLE_K) - (time * WATER_RIPPLE_SPEED)) / (1.0f + (max(len, ring) * max(len, ring) * max(len, ring) * max(len, ring)));
        acc += (ripple.xxx) * amp;
    }
    return acc;
}

#endif // TINYGLADE_WATER_WAVES_INC_HLSL
