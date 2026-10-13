// ==RECONSTRUCTED== shaders/water/water_object_push.inc.hlsl
// SOURCE PATH : shaders/water/water_object_push.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 7 bins):
//   _leaf_particles.cs.hlsl_cs_main_0.bin, _lilypad.raster.hlsl_b903e43ffb3da915.bin,
//   _lilypad.raster.hlsl_f0adff76827e73ff.bin, _lilypad_flower.raster.hlsl_b903e43ffb3da915.bin,
//   _lilypad_flower.raster.hlsl_f0adff76827e73ff.bin, _reed.raster.hlsl_b903e43ffb3da915.bin,
//   _reed.raster.hlsl_f0adff76827e73ff.bin
// Status: I (Inference). The consumers include this file BEFORE declaring
//   water_noise_texture and BEFORE waves.inc.hlsl (which references it), and
//   every consumer duplicates the same {2,4,8,12}/{2,2,0.5,0.2}/{17,18,20,21}
//   wave tables inline — evidence that the tables + a water-object displacement
//   helper originally lived here. OpLine attribution in the lilypad/reed bins
//   shows the include contributed a small inlined helper (a theme-4 early-out
//   at line ~21 plus a wave-displacement combine), but the exact helper body is
//   not recoverable (dxc inlines it; the two function-arg loads at line 10 match
//   a (origin_xz, time) signature). This is the minimal self-contained helper
//   consistent with the water-surface consumers (theme-4 gate + per-octave
//   rotated hash jitter); consumers keep their own inline displacement bodies.

#ifndef TINYGLADE_WATER_OBJECT_PUSH_HLSL
#define TINYGLADE_WATER_OBJECT_PUSH_HLSL

#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/frame_constants.hlsl"

// Wave tables shared by the water-surface consumers (lilypad / reed /
// lilypad_flower / leaf_particles) and waves.inc.hlsl.
static const float WATER_PUSH_SCALE[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float WATER_PUSH_WEIGHT[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint WATER_PUSH_SEED[4] = { 17u, 18u, 20u, 21u };

// Water object-push displacement for a surface object origin. Glade theme 4
// (autumn/snow) disables water motion; otherwise the object is pushed by the
// per-octave rotated hash jitter scaled by time (the same 2-component lattice
// the consumers inline). The consumer applies the returned xz offset to the
// object position.
float3 water_object_push(float2 origin_xz, float time)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter = 0.0f;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(WATER_PUSH_SEED[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint c0 = uint(cell);
        uint c1 = uint(cell + 1);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(hash(uint2(c0, c0))), rand01(hash(c0 ^ (c0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(hash(uint2(c1, c1))), rand01(hash(c1 ^ (c1 >> 18u))));
        jitter += (normalize(lerp(a, b, u) - 0.5f) * time * 3.0f) * WATER_PUSH_WEIGHT[oct];
    }
    return float3(jitter, 0.0f);
}

#endif // TINYGLADE_WATER_OBJECT_PUSH_HLSL
