// ==RECONSTRUCTED== shaders/resolve_to_swapchain.inc.hlsl | F-E | evidence: blit/resolve_to_swapchain.raster + blit/resolve_msaa_to_swapchain.raster + resolve_to_swapchain.cs 模块 —— 共享的翻转逻辑:`transform_image & 1` 根据 dims.y 选择对采样坐标做 y 翻转;三个消费方随后各自 Load/求平均,并调用 linear_to_srgb(color/srgb.hlsl);include 集合(frame_constants/uv/ray_cone/math_const)在三份 bin 里都存在,虽然 codegen 时未被使用 | status: O (transform_image 是 $Globals 全局成员 0,翻转算式来自内联使用点); I (辅助函数名 —— 已内联;dims 全局名在各消费方不同: framebuffer_dims 与 output_tex_dims,因此保留在主文件里)
//
// 把 HDR 帧缓冲解析到交换链的共享逻辑:坐标变换标志 + 逐像素采样坐标
// (在 MSAA 解析 / 编码之前)。

#ifndef TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL
#define TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL

#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color/srgb.hlsl"

// 把交换链方向变换应用到像素坐标上。`transform` 是引擎设置的
// transform_image 标志(各着色器自己的 $Globals 常量缓冲的第 0 个成员,
// 在消费主文件里声明)。
int2 resolve_to_swapchain_coord(int2 pixel, uint2 dims, uint transform)
{
    // 最低位为 1 表示需要上下翻转:以 dims.y 为轴镜像像素的 y 坐标。
    if ((transform & 1u) != 0u)
    {
        pixel.y = int(dims.y - 1u - uint(pixel.y));
    }
    return pixel;
}

#endif // TINYGLADE_RESOLVE_TO_SWAPCHAIN_INC_HLSL
