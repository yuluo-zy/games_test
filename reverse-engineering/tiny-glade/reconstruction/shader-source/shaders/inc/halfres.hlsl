// ==RECONSTRUCTED== shaders/inc/halfres.hlsl | evidence: embedded in 18 bins (morton.hlsl and half-res passes); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard half-resolution coordinate helpers)
//
// Full-resolution <-> half-resolution coordinate helpers.

#ifndef TINYGLADE_INC_HALFRES_HLSL
#define TINYGLADE_INC_HALFRES_HLSL

// Full-res pixel -> half-res pixel (integer rounding down).
int2 to_halfres_pixel(int2 full_pixel)
{
    return full_pixel >> 1;
}

// Half-res pixel -> full-res pixel (restore a 2x2 block origin).
int2 from_halfres_pixel(int2 half_pixel)
{
    return half_pixel << 1;
}

// Half-res texel count for a full-res texture.
uint2 halfres_dims(uint2 full_dims)
{
    return (full_dims + 1u) >> 1;
}

#endif // TINYGLADE_INC_HALFRES_HLSL
