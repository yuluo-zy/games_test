// ==RECONSTRUCTED== shaders/inc/hash.hlsl | evidence: _billboard_forest raster vs_main inlined use (uint hash: x+(x<<10), x^(x>>6), x+(x<<3), x^(x>>11), x+(x<<15) applied over instance seed; rand01: asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0; angle: frac(0.9 + float(x) * 1.618034f) * 6.283185482025146484375f) — matches include-graph 250 consumer bins | status: O (algorithm/value patterns from inlined SPIR-V); I (function names — helpers are inlined, names not embedded)
//
// Integer + float hash helpers used for deterministic per-instance / per-texel
// randomness. `hash` is the classic 5-step integer hash; `rand01` bitcasts a
// hash to a uniform float in [0,1); `hash_angle` maps an index to a pseudo-random
// angle via golden-ratio sequence.

#ifndef TINYGLADE_INC_HASH_HLSL
#define TINYGLADE_INC_HASH_HLSL

#include "math_const.hlsl"

// 5-step integer hash (avalanche). Used directly on instance/pixel indices.
uint hash(uint x)
{
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    x += (x << 15u);
    return x;
}

// Uniform float in [0, 1) from a hashed uint (mantissa bitcast trick).
float rand01(uint x)
{
    return asfloat((x & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

// Pseudo-random angle in [0, 2*PI) from an index (golden-ratio hashing).
float hash_angle(uint x)
{
    return frac(0.9f + (float(x) * GOLDEN_RATIO)) * TWO_PI;
}

// Pseudo-random float in [0, 1) from a float seed.
float hash01(float x)
{
    return frac(sin(x) * 43758.5453f);
}

#endif // TINYGLADE_INC_HASH_HLSL
