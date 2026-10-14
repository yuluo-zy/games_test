// ============================================================
// inc/color/luv.hlsl —— CIE L*u*v*(CIELUV)色彩空间转换
// 用途:与 helmholtz_kohlrausch.hlsl 搭配做感知色调分级。
//   消费方:final_post 像素着色器(唯一)。
// 入口:无(纯头文件;依赖 xyz.hlsl 的 D65 白点)
// 关键算法:标准 CIELUV(D65 白点),含正反转换
// ============================================================
// ==RECONSTRUCTED== shaders/inc/color/luv.hlsl | evidence: consumer final_post.raster.hlsl (only); used together with helmholtz_kohlrausch.hlsl for perceptual grading | status: U (formulas Inference — standard CIELUV on D65 white point; exact constants not observable in sampled decompile)
//
// CIE L*u*v* (CIELUV) conversion with D65 white point.

#ifndef TINYGLADE_INC_COLOR_LUV_HLSL
#define TINYGLADE_INC_COLOR_LUV_HLSL

#include "xyz.hlsl"

static const float LUV_EPSILON = 216.0f / 24389.0f;  // ~0.008856
static const float LUV_KAPPA = 24389.0f / 27.0f;     // ~903.3

// D65 白点的 u',v'。
static const float LUV_UN = (4.0f * XYZ_D65_WHITE.x) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);
static const float LUV_VN = (9.0f * XYZ_D65_WHITE.y) / (XYZ_D65_WHITE.x + 15.0f * XYZ_D65_WHITE.y + 3.0f * XYZ_D65_WHITE.z);

// XYZ -> Luv。
float3 xyz_to_luv(float3 xyz)
{
    float y_ratio = xyz.y / XYZ_D65_WHITE.y;
    float L = y_ratio > LUV_EPSILON
        ? 116.0f * pow(y_ratio, 1.0f / 3.0f) - 16.0f
        : LUV_KAPPA * y_ratio;

    float denom = xyz.x + 15.0f * xyz.y + 3.0f * xyz.z;
    float u_p = 4.0f * xyz.x / max(denom, 1.0e-5f);
    float v_p = 9.0f * xyz.y / max(denom, 1.0e-5f);

    return float3(L, 13.0f * L * (u_p - LUV_UN), 13.0f * L * (v_p - LUV_VN));
}

// Luv -> XYZ。
float3 luv_to_xyz(float3 luv)
{
    float L = luv.x;
    float u = luv.y;
    float v = luv.z;

    float u_p = (L > 8.0f ? u / (13.0f * L) : 0.0f) + LUV_UN;
    float v_p = (L > 8.0f ? v / (13.0f * L) : 0.0f) + LUV_VN;

    float Y = L > 8.0f
        ? XYZ_D65_WHITE.y * pow((L + 16.0f) / 116.0f, 3.0f)
        : XYZ_D65_WHITE.y * (L / LUV_KAPPA);

    float denom = 4.0f * v_p;
    float X = Y * (9.0f * u_p) / max(denom, 1.0e-5f);
    float Z = Y * (12.0f - 3.0f * u_p - 20.0f * v_p) / max(denom, 1.0e-5f);

    return float3(X, Y, Z);
}

#endif // TINYGLADE_INC_COLOR_LUV_HLSL
