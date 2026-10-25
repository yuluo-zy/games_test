// ======================================================================
// inc/parallax_mapping.hlsl —— 陡峭视差映射辅助
// ======================================================================
// 用途:被 10 个 bin 嵌入(roof dormer_hole 包含它);标准陡峭视差映射。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/parallax_mapping.hlsl | evidence: embedded in 10 bins (roof dormer_hole includes it); no named functions recoverable (inlined) | status: I/U (names/bodies Inference — standard steep parallax mapping)
//
// Steep parallax mapping helper (depth-map-based UV offset).

#ifndef TINYGLADE_INC_PARALLAX_MAPPING_HLSL
#define TINYGLADE_INC_PARALLAX_MAPPING_HLSL

// 陡峭视差遮挡步进:沿视图射线以 `layers` 步穿过高度图 `height_tex`,
// 返回偏移 UV。height_scale 缩放视差高度;view_dir 为切线空间视线方向。
float2 parallax_mapping(Texture2D<float4> height_tex, SamplerState samp,
                        float2 uv, float3 view_dir, float height_scale, uint layers)
{
    float layer_depth = 1.0f / float(layers);
    float current_depth = 1.0f;
    float2 p = view_dir.xy * (height_scale / float(layers));
    float2 delta = p;

    float2 current_uv = uv;
    float height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
    while (current_depth > 0.0f && height < current_depth)
    {
        current_uv -= delta;
        height = 1.0f - height_tex.SampleLevel(samp, current_uv, 0.0f).r;
        current_depth -= layer_depth;
    }

    // 最后两步之间线性插值以获得平滑结果。
    float2 prev_uv = current_uv + delta;
    float prev_depth = current_depth + layer_depth;
    float prev_height = 1.0f - height_tex.SampleLevel(samp, prev_uv, 0.0f).r;
    float w = (current_depth - height) / max((current_depth - height) - (prev_depth - prev_height), 1.0e-5f);
    return lerp(current_uv, prev_uv, saturate(w));
}

#endif // TINYGLADE_INC_PARALLAX_MAPPING_HLSL
