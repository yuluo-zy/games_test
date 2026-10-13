// ==RECONSTRUCTED== shaders/inc/texture_blend.hlsl | evidence: embedded in 8 bins (material blend paths); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard texture blend helpers)
//
// Texture blending helpers (mask-based layer blends).

#ifndef TINYGLADE_INC_TEXTURE_BLEND_HLSL
#define TINYGLADE_INC_TEXTURE_BLEND_HLSL

// Blend two layer samples by a scalar mask.
float4 blend_layers(float4 a, float4 b, float mask)
{
    return lerp(a, b, saturate(mask));
}

// Heightmap-style smooth blend using the layer alpha / mask as a height.
float4 blend_layers_height(float4 a, float4 b, float mask_a, float mask_b, float blend_width)
{
    float h = saturate((mask_a - mask_b) / max(blend_width, 1.0e-5f) + 0.5f);
    return lerp(b, a, h);
}

#endif // TINYGLADE_INC_TEXTURE_BLEND_HLSL
