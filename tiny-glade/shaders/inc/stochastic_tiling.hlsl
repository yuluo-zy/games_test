// ======================================================================
// inc/stochastic_tiling.hlsl —— 随机纹理平铺辅助
// ======================================================================
// 用途:哈希驱动的瓦片选择,使重复纹理不呈现可见周期;嵌入 2 个 bin。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/stochastic_tiling.hlsl | evidence: embedded in 2 bins; include-graph edge stochastic_tiling.hlsl -> pack_unpack.hlsl | status: I/U (names/bodies Inference — standard stochastic texture tiling)
//
// Stochastic texture tiling: hash-driven tile selection so a repeating
// texture does not show visible periodicity.

#ifndef TINYGLADE_INC_STOCHASTIC_TILING_HLSL
#define TINYGLADE_INC_STOCHASTIC_TILING_HLSL

#include "pack_unpack.hlsl"
#include "hash.hlsl"

// 对几个随机瓦片旋转做偏移采样并返回混合后的颜色。
// tiles = 混合的变体数量(通常 2-3)。
float3 stochastic_tile_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 tile_scale, uint seed, uint tiles)
{
    float2 tile_id = floor(uv * tile_scale);
    float2 f = frac(uv * tile_scale);
    uint rng = hash(uint(tile_id.x) * 73856093u ^ uint(tile_id.y) * 19349663u ^ seed);

    float3 result = float3(0.0f, 0.0f, 0.0f);
    float total = 0.0f;
    for (uint t = 0u; t < tiles; t++)
    {
        uint h = hash(rng + t);
        float2 offset = float2(rand01(h), rand01(h >> 8u));
        float2 sample_uv = (tile_id + f) / tile_scale + (offset - 0.5f) * (1.0f / tile_scale);
        result += tex.SampleLevel(samp, sample_uv, 0.0f).rgb;
        total += 1.0f;
    }
    return result / total;
}

#endif // TINYGLADE_INC_STOCHASTIC_TILING_HLSL
