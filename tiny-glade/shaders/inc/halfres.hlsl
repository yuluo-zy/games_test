// ============================================================
// inc/halfres.hlsl —— 全分辨率 <-> 半分辨率 坐标辅助
// 用途:全分辨率/半分辨率像素与尺寸互转。
//   消费方:morton.hlsl 与半分辨率 pass(18 个 bin)。
// 入口:无(纯头文件)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/halfres.hlsl | evidence: embedded in 18 bins (morton.hlsl and half-res passes); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard half-resolution coordinate helpers)
//
// Full-resolution <-> half-resolution coordinate helpers.

#ifndef TINYGLADE_INC_HALFRES_HLSL
#define TINYGLADE_INC_HALFRES_HLSL

// 全分辨率像素 -> 半分辨率像素(向下取整)。
int2 to_halfres_pixel(int2 full_pixel)
{
    return full_pixel >> 1;
}

// 半分辨率像素 -> 全分辨率像素(还原 2x2 块原点)。
int2 from_halfres_pixel(int2 half_pixel)
{
    return half_pixel << 1;
}

// 全分辨率纹理对应的半分辨率纹素数量。
uint2 halfres_dims(uint2 full_dims)
{
    return (full_dims + 1u) >> 1;
}

#endif // TINYGLADE_INC_HALFRES_HLSL
