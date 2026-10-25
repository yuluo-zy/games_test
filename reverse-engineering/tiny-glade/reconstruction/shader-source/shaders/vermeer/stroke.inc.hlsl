// ==RECONSTRUCTED== shaders/vermeer/stroke.inc.hlsl | evidence: _vermeer_generate_strokes.cs.hlsl_cs_main_{2f7da8dc0793466b,f890205ffc078ce8}.bin + _vermeer_raster_strokes.raster.hlsl_{b903,f0ad}.bin — VermeerStrokePacked{pos,color,size_rotation,appear_t} (16B; pos/size_rotation store two half2x16 in one uint, color RGBA8 incl. alpha); f32tof16/f16tof32 pack/unpack inlined; include-graph edge vermeer/stroke.inc.hlsl -> inc/pack_unpack.hlsl | status: O (struct + pack layout from inlined SPIR-V); I (helper function names — inlined)
//
// Shared vermeer stroke record + packing helpers (same layout as the painterly
// chain but RGBA8 keeps alpha, read back by the raster VS as the stroke alpha).

#ifndef TINYGLADE_VERMEER_STROKE_INC_HLSL
#define TINYGLADE_VERMEER_STROKE_INC_HLSL

#include "inc/pack_unpack.hlsl"

struct VermeerStrokePacked
{
    uint pos;
    uint color;
    uint size_rotation;
    float appear_t;
};

// Pack two halves into one uint (x low, y high).
uint pack_half2(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

// Unpack a half stored in the low 16 bits.
float unpack_half_x(uint v)
{
    return f16tof32(v & 0xffffu);
}

// Unpack a half stored in the high 16 bits.
float unpack_half_y(uint v)
{
    return f16tof32(v >> 16u);
}

// Unpack the centered position stored in `pos`.
float2 unpack_stroke_pos(uint pos)
{
    return float2(unpack_half_x(pos & 0xffffu), unpack_half_x(pos >> 16u));
}

// Unpack (size, rotation angle) stored in `size_rotation`.
float2 unpack_stroke_size_rotation(uint size_rotation)
{
    return float2(unpack_half_x(size_rotation & 0xffffu), unpack_half_x(size_rotation >> 16u));
}

#endif // TINYGLADE_VERMEER_STROKE_INC_HLSL
