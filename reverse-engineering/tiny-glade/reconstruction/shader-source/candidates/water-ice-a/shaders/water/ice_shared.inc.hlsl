// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water/ice_shared.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN (as include, 6 bins):
//   _ice_plane.raster.hlsl_{b903e,f0adff}.bin, _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin,
//   _water_ice_refl_march.cs.hlsl_cs_main_0.bin, _water_ice_refl_resolve.cs.hlsl_cs_main_0.bin,
//   _water_ice_refr_resolve.cs.hlsl_cs_main_0.bin
// Status: O (half-precision pack/unpack helpers from inlined use-sites in the ice
//   consumers: pack_half2x16 = f32tof16 | <<16, unpack_half2x16 = f16tof32(low/high));
//   I (function names; whether the original ice_shared.inc carried additional ice
//   helpers is unknown — only these two are observable across consumers)
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

#endif // TINYGLADE_WATER_ICE_SHARED_HLSL
