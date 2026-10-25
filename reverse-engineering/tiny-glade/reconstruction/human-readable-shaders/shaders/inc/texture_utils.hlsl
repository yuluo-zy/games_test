// ======================================================================
// inc/texture_utils.hlsl —— 纹理采样辅助
// ======================================================================
// 用途:被路径/遮罩采样器使用;嵌入 37 个 bin(paths、terrain)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/texture_utils.hlsl | evidence: embedded in 37 bins (paths, terrain); include-graph edge path_mask_sampler.inc.hlsl -> texture_utils.hlsl | status: I/U (names/bodies Inference — inlined)
//
// Texture sampling helpers used by path/mask samplers.

#ifndef TINYGLADE_INC_TEXTURE_UTILS_HLSL
#define TINYGLADE_INC_TEXTURE_UTILS_HLSL

// 带显式 mip 偏移采样(frame_constants.global_mip_bias)。
float4 sample_lod_bias(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip_bias)
{
    return tex.SampleBias(samp, uv, mip_bias);
}

// 带显式 lod 的三线性采样。
float4 sample_lod(Texture2D<float4> tex, SamplerState samp, float2 uv, float lod)
{
    return tex.SampleLevel(samp, uv, lod);
}

// 用整数坐标取纹素(无滤波)。
float4 load_texel(Texture2D<float4> tex, int2 pixel)
{
    return tex.Load(int3(pixel, 0));
}

#endif // TINYGLADE_INC_TEXTURE_UTILS_HLSL
