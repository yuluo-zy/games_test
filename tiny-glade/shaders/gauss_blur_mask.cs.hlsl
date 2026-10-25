// ============================================================
// gauss_blur_mask.cs.hlsl
// 功能:可分离高斯模糊,用于水面蒙版(water_mask)。
//       半径 = ceil(sigma*2);在半径范围内采样加权平均,
//       越界采样跳过;结果按 output_channel 写入目标通道(.y/.z)。
// 入口:cs_main,8x8 线程/工作组。
// 绑定:$Globals b13(water_mask_dims / out_water_mask_dims /
//       is_horizontal / sigma / output_channel)、water_mask (t14)、
//       out_water_mask (u15)。
// ============================================================

// ==RECONSTRUCTED== shaders/gauss_blur_mask.cs.hlsl | F-D
// Evidence: _gauss_blur_mask.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (water_mask_dims/out_water_mask_dims/is_horizontal/sigma/
//   output_channel), water_mask t14, out_water_mask u15, LocalSize [8,8,1], entry cs_main.
//   spirv-cross: separable gaussian, radius=ceil(sigma*2); in-bounds taps accumulate
//   (.x channel if horizontal else .y); writes mean into out_water_mask .y if
//   output_channel==1 else .z if output_channel==2.
// Status: O (structure from disassembly; member names verbatim).

#include "inc/common.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 water_mask_dims;    // 输入水面蒙版尺寸
    uint2 out_water_mask_dims; // 输出蒙版尺寸
    uint is_horizontal;       // 1=水平模糊,0=垂直模糊
    float sigma;              // 高斯标准差
    int output_channel;       // 输出到蒙版的哪个通道(1->.y, 2->.z)
};

Texture2D<float4> water_mask : register(t14, space0);
RWTexture2D<float4> out_water_mask : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 模糊半径:2*sigma 向上取整。
    int radius = int(ceil(sigma * 2.0f));

    // 加权和与权重总和。
    float weighted_sum = 0.0f;
    float weight_sum = 0.0f;

    bool horizontal = is_horizontal != 0u;
    // 沿水平或垂直方向遍历半径范围。
    for (int offset = -radius; offset <= radius; ++offset)
    {
        int2 pixel;
        if (!horizontal)
        {
            pixel = int3(dispatchThreadID).xy + int2(0, offset);
        }
        else
        {
            pixel = int3(dispatchThreadID).xy + int2(offset, 0);
        }
        uint2 upixel = uint2(pixel);

        // 只累加在输入范围内的采样(避免越界)。
        if (all(upixel < water_mask_dims))
        {
            // 高斯权重:exp(-t^2 / (2*sigma^2))。
            float t = float(offset);
            float weight = exp((-t * t) / ((2.0f * sigma) * sigma));
            // 水平模糊取 .x 通道,垂直模糊取 .y 通道。
            weighted_sum += (horizontal ? water_mask.Load(int3(upixel, 0)).x : water_mask.Load(int3(upixel, 0)).y) * weight;
            weight_sum += weight;
        }
    }

    // 归一化均值,按 output_channel 写入目标通道。
    float mean = weighted_sum / weight_sum;
    if (output_channel == 1)
    {
        out_water_mask[dispatchThreadID.xy].y = mean;
    }
    else if (output_channel == 2)
    {
        out_water_mask[dispatchThreadID.xy].z = mean;
    }
}
