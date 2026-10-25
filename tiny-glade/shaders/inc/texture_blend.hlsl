// ======================================================================
// inc/texture_blend.hlsl —— 纹理混合辅助
// ======================================================================
// 用途:嵌入 8 个 bin(材质混合路径);基于遮罩的图层混合。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/texture_blend.hlsl | evidence: embedded in 8 bins (material blend paths); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard texture blend helpers)
//
// Texture blending helpers (mask-based layer blends).

#ifndef TINYGLADE_INC_TEXTURE_BLEND_HLSL
#define TINYGLADE_INC_TEXTURE_BLEND_HLSL

// 用标量遮罩混合两个图层采样。
float4 blend_layers(float4 a, float4 b, float mask)
{
    return lerp(a, b, saturate(mask));
}

// 高度图式平滑混合,把图层 alpha/遮罩当作高度。
float4 blend_layers_height(float4 a, float4 b, float mask_a, float mask_b, float blend_width)
{
    float h = saturate((mask_a - mask_b) / max(blend_width, 1.0e-5f) + 0.5f);
    return lerp(b, a, h);
}

#endif // TINYGLADE_INC_TEXTURE_BLEND_HLSL
