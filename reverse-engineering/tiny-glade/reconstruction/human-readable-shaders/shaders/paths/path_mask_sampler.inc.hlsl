// ==RECONSTRUCTED== shaders/paths/path_mask_sampler.inc.hlsl
// 依据:消费方 _clearing_*/_generate_grass/_path_pebbles/_ivy_* 内联了
// `path_mask.SampleLevel(sampler_llb, world_xz*0.0076923077+0.5, 0).x`
// (uv 由世界 xz 经同一套 1/130 高度图映射得到);include 图直接边
// path_mask_sampler.inc.hlsl -> texture_utils.hlsl。
// 状态:O (uv 映射/通道来自内联使用点);I (函数名——内联、未嵌入)

#ifndef TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL
#define TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL

#include "inc/texture_utils.hlsl"

// 路径遮罩采样缩放与地形高度图的世界坐标映射一致(1/130)。
static const float PATH_MASK_SAMPLE_SCALE = 0.0076923076994717121124267578125f;

// 在世界 xz 位置采样路径遮罩(道路覆盖度)。路径遮罩使用
// mipmap/线性夹取采样器,红色通道保存道路覆盖度。
float get_path_mask(Texture2D<float4> path_mask, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * PATH_MASK_SAMPLE_SCALE + 0.5f;
    return path_mask.SampleLevel(s, uv, 0.0f).x;
}

// 判断世界 xz 位置是否处于道路上(路径遮罩覆盖度超过阈值),
// 供植被/石子放置时吸附到道路上。
bool is_path(Texture2D<float4> path_mask, SamplerState s, float2 world_xz, float threshold)
{
    return get_path_mask(path_mask, s, world_xz) > threshold;
}

#endif // TINYGLADE_PATHS_PATH_MASK_SAMPLER_HLSL
