// ==RECONSTRUCTED== shaders/painterly/stroke.inc.hlsl | evidence: _painterly_generate_strokes.cs.hlsl_cs_main_0.bin + _painterly_raster_strokes.raster.hlsl_{b903,f0ad}.bin — PainterlyStrokePacked{pos,color,size_rotation,appear_t} (16B; pos/size_rotation store two half2x16 in one uint, color RGBA8); f32tof16/f16tof32 pack/unpack inlined; include-graph edge painterly/stroke.inc.hlsl -> inc/pack_unpack.hlsl | status: O (struct + pack layout from inlined SPIR-V); I (helper function names — inlined)
//
// 功能:画家风笔触共享记录结构 + 打包/解包辅助函数。一条笔触把"居中位置"
// 以两个 half 存在 pos、RGBA8 颜色存在 color、尺寸+旋转角以两个 half 存在
// size_rotation;appear_t 是该笔触被创建的时刻戳(光栅阶段用于浮现动画)。
// 逻辑与原文完全一致,仅补充中文注释。

#ifndef TINYGLADE_PAINTERLY_STROKE_INC_HLSL
#define TINYGLADE_PAINTERLY_STROKE_INC_HLSL

#include "inc/pack_unpack.hlsl"

// 一条笔触的打包记录(共 16 字节)。
struct PainterlyStrokePacked
{
    uint pos;            // 居中位置,两个 half(x 低 16 位,y 高 16 位)
    uint color;          // RGBA8 颜色
    uint size_rotation;  // 尺寸 + 旋转角,两个 half(x 低 16 位,y 高 16 位)
    float appear_t;      // 出现时间戳
};

// 把两个 half 打包进一个 uint(x 低 16 位,y 高 16 位)。
uint pack_half2(float2 v)
{
    uint2 p = f32tof16(v);
    return p.x | (p.y << 16u);
}

// 解包低 16 位存储的 half。
float unpack_half_x(uint v)
{
    return f16tof32(v & 0xffffu);
}

// 解包高 16 位存储的 half。
float unpack_half_y(uint v)
{
    return f16tof32(v >> 16u);
}

// 解包存于 pos 的居中位置。
float2 unpack_stroke_pos(uint pos)
{
    return float2(unpack_half_x(pos & 0xffffu), unpack_half_x(pos >> 16u));
}

// 解包存于 size_rotation 的(尺寸, 旋转角)。
float2 unpack_stroke_size_rotation(uint size_rotation)
{
    return float2(unpack_half_x(size_rotation & 0xffffu), unpack_half_x(size_rotation >> 16u));
}

#endif // TINYGLADE_PAINTERLY_STROKE_INC_HLSL
