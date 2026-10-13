// ==RECONSTRUCTED== shaders/roof/wooden_floor.inc.hlsl
// 依据:F-V | 内嵌于 nani_roof_floor.raster(2) + nani_stairs_floor.raster(2)
// 木质地板程序化辅助函数。纹理绑定(wooden_floor_normal/height/seed/roughness,
// x3 变体)观察自 _nani_roof_floor ps_main 的内联使用点。
// 逐木板图案/颜色逻辑在本轮未知(由 wave-2 nani 重建细化)。

#ifndef TINYGLADE_ROOF_WOODEN_FLOOR_HLSL
#define TINYGLADE_ROOF_WOODEN_FLOOR_HLSL

// 在纹理空间位置采样一组木质地板纹素(法线/种子/高度/粗糙度)。
void wooden_floor_sample(float2 uv, Texture2D normal_map, Texture2D seed_map, Texture2D height_map,
                         Texture2D roughness_map, SamplerState s, float mip_bias,
                         out float3 normal, out float4 seed, out float4 height, out float4 roughness)
{
    normal = (normal_map.SampleLevel(s, uv, mip_bias).xyz * 2.0f) - 1.0f;
    seed = seed_map.SampleBias(s, uv, mip_bias);
    height = height_map.SampleBias(s, uv, mip_bias);
    roughness = roughness_map.SampleBias(s, uv, mip_bias);
}

#endif // TINYGLADE_ROOF_WOODEN_FLOOR_HLSL
