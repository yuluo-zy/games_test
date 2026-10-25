// ============================================================
// inc/image.hlsl —— 图像/纹理常用辅助
// 用途:掩码与边界操作的常用纹理采样与尺寸辅助。
//   消费方:11 个 bin(采样图像纹理做掩码/边框)。
// 入口:无(纯头文件;依赖 curve.hlsl)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/image.hlsl | evidence: consumer bins (11) sample image textures for masks/borders; include-graph edge image.hlsl -> curve.hlsl; OpLine attribution shows bilinear-style lookups | status: I (function names/bodies Inference — inlined)
//
// Image/texture helpers for common mask and border operations.

#ifndef TINYGLADE_INC_IMAGE_HLSL
#define TINYGLADE_INC_IMAGE_HLSL

#include "curve.hlsl"

// 1D / 2D 纹理尺寸辅助。
float2 texture_dims(Texture2D<float4> tex)
{
    uint w, h;
    tex.GetDimensions(w, h);
    return float2(w, h);
}

// 在 UV 处采样 1 通道掩码纹理,返回标量值。
float sample_mask(Texture2D<float4> mask_tex, SamplerState samp, float2 uv)
{
    return mask_tex.SampleLevel(samp, uv, 0.0f).r;
}

#endif // TINYGLADE_INC_IMAGE_HLSL
