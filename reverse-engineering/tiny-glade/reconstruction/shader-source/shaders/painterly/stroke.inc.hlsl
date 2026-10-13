// ==RECONSTRUCTED== shaders/painterly/stroke.inc.hlsl | evidence: _painterly_generate_strokes.cs.hlsl_cs_main_0.bin + _painterly_raster_strokes.raster.hlsl_{b903,f0ad}.bin — PainterlyStrokePacked{pos,color,size_rotation,appear_t} (16B; pos/size_rotation store two half2x16 in one uint, color RGBA8); f32tof16/f16tof32 pack/unpack inlined; include-graph edge painterly/stroke.inc.hlsl -> inc/pack_unpack.hlsl | status: O (struct + pack layout from inlined SPIR-V); I (helper function names — inlined)
//
// Shared painterly stroke record + packing helpers. A stroke packs its centered
// position as two halves in `pos`, its RGBA8 color in `color`, and its size +
// rotation angle as two halves in `size_rotation`. `appear_t` is the wall-clock
// timestamp the stroke was created at (used by the raster pass for the reveal).

#ifndef TINYGLADE_PAINTERLY_STROKE_INC_HLSL
#define TINYGLADE_PAINTERLY_STROKE_INC_HLSL

#include "inc/pack_unpack.hlsl"

struct PainterlyStrokePacked
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

#endif // TINYGLADE_PAINTERLY_STROKE_INC_HLSL
