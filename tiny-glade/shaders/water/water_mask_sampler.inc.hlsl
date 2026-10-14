// ==RECONSTRUCTED== shaders/water/water_mask_sampler.inc.hlsl
// 中文说明:
//   水面掩码采样器。消费者 _path_pebbles/_rocky_terrain_stairs_stairs/
//   _water_spawn_reeds/_water_spawn_lilypads/_paths_path_mask_visual/
//   _garden_lowpoly_flower 内联了
//     `water_mask.SampleLevel(sampler_default|sampler_llc, world_xz*0.0076923077+0.5, 0).y`
//   (uv 由世界 xz 经与高度图相同的 1/130 映射换算;绿色通道是水面覆盖率)。
//   include 图直接边:water_mask_sampler.inc.hlsl -> inc/texture_utils.hlsl
//   | 状态:O(uv 映射/通道来自内联使用点);I(函数名——被内联,未嵌入)

#ifndef TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
#define TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL

#include "inc/texture_utils.hlsl"

// 水面掩码采样缩放与地形高度图的世界映射一致(1/130)。
static const float WATER_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// 在世界 xz 位置采样水面掩码(绿色通道保存水面覆盖率)。
float get_water_mask(Texture2D<float4> water_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * WATER_MASK_SAMPLE_SCALE + 0.5f;
    return water_mask.SampleLevel(s, uv, 0.0f).y;
}

// 世界 xz 位置是否在水面上(水面掩码覆盖率超过阈值)。
bool is_water(Texture2D<float4> water_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_water_mask(water_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_WATER_WATER_MASK_SAMPLER_HLSL
