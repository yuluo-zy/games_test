// Vermeer 共享笔触记录与打包工具(与 painterly 链布局相同,但 RGBA8 保留 alpha,
// 由 raster VS 作为笔触 alpha 读回)。

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

// 把两个半精度打包进一个 uint(x 低,y 高)。
uint pack_half2(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

// 解包存于低 16 位的半精度。
float unpack_half_x(uint v)
{
    return f16tof32(v & 0xffffu);
}

// 解包存于高 16 位的半精度。
float unpack_half_y(uint v)
{
    return f16tof32(v >> 16u);
}

// 解包存于 `pos` 的居中位置。
float2 unpack_stroke_pos(uint pos)
{
    return float2(unpack_half_x(pos & 0xffffu), unpack_half_x(pos >> 16u));
}

// 解包存于 `size_rotation` 的(尺寸,旋转角)。
float2 unpack_stroke_size_rotation(uint size_rotation)
{
    return float2(unpack_half_x(size_rotation & 0xffffu), unpack_half_x(size_rotation >> 16u));
}

#endif // TINYGLADE_VERMEER_STROKE_INC_HLSL
