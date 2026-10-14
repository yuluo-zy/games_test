// ============================================================
// inc/color/xyz.hlsl —— sRGB <-> CIE XYZ(D65)转换
// 用途:在线性 sRGB 与 CIE XYZ(D65)之间互转。
//   消费方:final_post 像素着色器(唯一,供 Luv/HK 分级链使用)。
// 入口:无(纯头文件)
// 关键算法:标准 Rec.709/D65 矩阵,输入输出均按线性 sRGB 假定
// ============================================================
// ==RECONSTRUCTED== shaders/inc/color/xyz.hlsl | evidence: consumer final_post.raster.hlsl (only); no inlined matrix visible in the sampled ps path (included but LUT/ICC path used); standard Rec.709 / D65 sRGB<->XYZ matrix | status: U (formulas Inference — standard D65 matrices, not directly observable in sampled decompile)
//
// sRGB <-> CIE XYZ (D65) conversion. Input/output assumed linear sRGB.

#ifndef TINYGLADE_INC_COLOR_XYZ_HLSL
#define TINYGLADE_INC_COLOR_XYZ_HLSL

static const float3 XYZ_D65_WHITE = float3(0.95047f, 1.0f, 1.08883f);

// 线性 sRGB -> XYZ(D65)。
float3 srgb_xyz_to_xyz(float3 c)
{
    return mul(
        float3x3(
            float3(0.4124564f, 0.3575761f, 0.1804375f),
            float3(0.2126729f, 0.7151522f, 0.0721750f),
            float3(0.0193339f, 0.1191920f, 0.9503041f)),
        c);
}

// XYZ(D65) -> 线性 sRGB。
float3 xyz_to_srgb_xyz(float3 xyz)
{
    return mul(
        float3x3(
            float3( 3.2404542f, -1.5371385f, -0.4985314f),
            float3(-0.9692660f,  1.8760108f,  0.0415560f),
            float3( 0.0556434f, -0.2040259f,  1.0572252f)),
        xyz);
}

#endif // TINYGLADE_INC_COLOR_XYZ_HLSL
