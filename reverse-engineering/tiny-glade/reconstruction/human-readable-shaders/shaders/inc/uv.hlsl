// 坐标空间辅助工具,供 frame_constants 的消费方使用。
// 全部为基于矩阵的函数(不依赖 cbuffer),因此可在 frame_constants.hlsl
// 声明视矩阵 cbuffer 之前被包含。

#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL

// 整数像素坐标 -> 归一化 UV([0,1],纹素中心在 .5)。
float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}

// 归一化 UV -> 以像素中心为原点的"sample"坐标(y 向上)。
// sample 空间正是 view_to_sample / sample_to_view 矩阵作用的坐标系。
float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}

// sample 空间的 x,y(已居中)-> 裁剪空间 x,y(-1..1,y 向上)。
float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}

// 由 sample 坐标与线性深度重建视空间位置。
// sample_uv 为居中坐标;depth 是视空间 -z 距离。
float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}

// 视空间位置 -> 居中 sample 坐标。
float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}

// 视空间位置 -> 裁剪空间(透视除法)。
float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}

// 世界空间位置 -> 视空间。
float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}

// 世界空间位置 -> 居中 sample 坐标。
float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif // TINYGLADE_INC_UV_HLSL
