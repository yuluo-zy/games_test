// ==RECONSTRUCTED== shaders/roof/stone_floor.inc.hlsl
// 依据:F-V | 内嵌于 nani_roof_floor.raster(2) + nani_stairs_floor.raster(2) + terrain_editing_floor.raster(2)
// 石质地板程序化辅助函数。纹理绑定(stone_floor_normal/seed/height/roughness,x3 变体)
// 与高度图光线步进 + 法线/种子/粗糙度采样观察自 _nani_roof_floor ps_main 的内联使用点。
// 逐变体分发与颜色/图案逻辑在本轮未知(由 wave-2 nani 重建细化)。

#ifndef TINYGLADE_ROOF_STONE_FLOOR_HLSL
#define TINYGLADE_ROOF_STONE_FLOOR_HLSL

// 用二分法对高度图做光线步进(根据内联使用点重建:3 步粗步进 + 4 次二分迭代),
// 返回沿 `dir` 从 `origin` 出发的参数量命中距离
// (origin.xy = 纹理空间位置,origin.z = 高度偏差)。
float stone_floor_march_height(float3 origin, float2 dir, Texture2D height_map, SamplerState s,
                               float mip_bias, float height_scale)
{
    float hit = 1.0f;
    float t_lo = 0.0f;
    float t_hi = 1.0f;
    float prev = origin.z;
    for (uint i = 1u; i < 3u; i++)
    {
        float2 p = origin.xy + (dir * t_hi);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) break;
        prev = d;
        t_lo = t_hi;
        t_hi = saturate(min(0.9999847f, pow((float(i) + frac(0.0f)) * 0.5f, 2.0f)));
    }
    // 二分细化
    for (uint i = 0u; i < 4u; i++)
    {
        float t = (t_lo + t_hi) * 0.5f;
        float2 p = origin.xy + (dir * t);
        float h = height_map.SampleLevel(s, p, mip_bias).x - 1.0f;
        float d = origin.z - (h * height_scale);
        if (d < 0.0f) { t_hi = t; prev = d; }
        else { t_lo = t; prev = d; }
    }
    return lerp(t_lo, t_hi, prev / (prev + (prev - prev)));
}

// 在纹理空间位置采样一组石质地板纹素(法线/种子/高度/粗糙度)。
void stone_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                        Texture2D roughness_map, SamplerState s, float mip_bias,
                        out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    normal.z *= 0.13333332538604736328125f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif // TINYGLADE_ROOF_STONE_FLOOR_HLSL
