// ============================================================
// inc/color/math.hlsl —— 颜色数学小工具
// 用途:提供 Rec.709 亮度等颜色转换头共享的小助手。
//   消费方:final_post 像素着色器(唯一)。
// 入口:无(纯头文件)
// 关键算法:Rec.709 亮度权重 (0.2126, 0.7152, 0.0722)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/color/math.hlsl | evidence: consumer final_post.raster.hlsl (only); inlined luminance weights (0.2126,0.7152,0.0722) and clamp-to-[0,1] patterns throughout the grading chain | status: I (names/bodies Inference — helpers inlined, not embedded)
//
// Small color-math helpers shared by the color conversion headers.

#ifndef TINYGLADE_INC_COLOR_MATH_HLSL
#define TINYGLADE_INC_COLOR_MATH_HLSL

// Rec.709 亮度。
float luma(float3 rgb)
{
    return dot(rgb, float3(0.2126f, 0.7152f, 0.0722f));
}

#endif // TINYGLADE_INC_COLOR_MATH_HLSL
