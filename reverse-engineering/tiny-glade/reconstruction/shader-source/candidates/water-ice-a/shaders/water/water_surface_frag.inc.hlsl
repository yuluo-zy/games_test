// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water/water_surface_frag.inc.hlsl
// KIND        : include_only
// COMPILED AS MAIN IN: none (include-only file)
// ALSO EMBEDDED IN: _water_plane.raster.hlsl, _waterfall.raster.hlsl (2 bins each)
// Status: O (half-pack helpers + shared water-surface constants from inlined
//   use-sites); I (exact split of helpers between this inc and waves.inc)
//
// Shared water-surface fragment helpers.

#ifndef TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL
#define TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL

#include "inc/pack_unpack.hlsl"

// Half2 pack/unpack (used by the ripple buffer and reflection-ray encoding).
uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16u);
}

float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

#endif // TINYGLADE_WATER_WATER_SURFACE_FRAG_HLSL
