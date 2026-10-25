// ==RECONSTRUCTED== shaders/water/ice_shared.inc.hlsl
// SOURCE PATH : shaders/water/ice_shared.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 6 bins):
//   _ice_plane.raster.hlsl_{b903e,f0adff}.bin, _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin,
//   _water_ice_refl_march.cs.hlsl_cs_main_0.bin, _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin,
//   _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
// Status: O (half-precision pack/unpack helpers from inlined use-sites in the ice
//   consumers: pack_half2x16 = f32tof16 | <<16, unpack_half2x16 = f16tof32(low/high));
//   O (FrexpStructType name from raw-per-bin debug_names); I (frexp helper body —
//   inlined in SPIR-V, mirrors the adjudicated curve.hlsl incb_frexp form)
// MERGE (water-ice review water/ice_shared.inc.hlsl, inc-fix #6): B's FrexpStructType
//   placement (the ice_refl_resolve bin carries the name and its closure has this file
//   but NOT inc/curve.hlsl) + the frexp workaround (dxc's native frexp SPIR-V lowering
//   is broken) + A's half pack/unpack helpers.
//
// Shared helpers for the ice reflection/refraction family.

#ifndef TINYGLADE_WATER_ICE_SHARED_HLSL
#define TINYGLADE_WATER_ICE_SHARED_HLSL

#include "inc/pack_unpack.hlsl"

// Pack two halfs (each in [0,1]) into one uint (ray/tangent encoding).
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}

// Unpack a half2 from the low and high halfs of a uint.
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

// frexp pair used by ice_refl_resolve to widen accumulated radiance into 8-bit
// exponent range. dxc v1.10.2605.37's native `frexp` SPIR-V lowering is broken,
// so the ice family defines its own (mirrors the adjudicated inc/curve.hlsl form);
// the macro routes `frexp(x, out e)` tokens to it for the rest of the TU.
#define frexp ice_frexp

struct FrexpStructType
{
    float3 mantissa;
    int3 exponent;
};

float3 ice_frexp(float3 x, out int3 exponent)
{
    uint3 bits = asuint(x);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu),
                       int((bits.y >> 23u) & 0xFFu),
                       int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127);
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);

    // Normalize mantissa to [0.5, 1) (C-style frexp).
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);

    // Zero / subnormal / inf / nan: mantissa = x, exponent = 0.
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, x, m);
}

#endif // TINYGLADE_WATER_ICE_SHARED_HLSL
