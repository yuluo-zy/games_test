// ==RECONSTRUCTED== shaders/shadow_mask_mean.cs.hlsl | F-D
// Evidence: _shadow_mask_mean.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: __alias_shadow_mask_histogram u13 (RWByteAddressBuffer),
//   __alias_shadow_mask_mean u14, frame_constants set1/b0, LocalSize [256,1,1],
//   entry cs_main.
//   spirv-cross: each lane converts a shadow-mask histogram bucket into a weighted
//   (value = lane/255, weight = count) pair and clears the histogram; a tree
//   reduction computes the mean; the running mean is smoothed with
//   exp(-5*dt) and stored back.
// Status: O (structure from disassembly).
//
// 中文说明:计算"阴影遮罩平均覆盖率"的计算着色器。
//   1) 每个线程把直方图一个桶(计数)转成(灰度值=桶号/255, 权重=计数)的加权样本,
//      并清零该桶;
//   2) 组内树形归约求和,得到加权均值 mean;
//   3) 用指数平滑 lerp(old_mean, mean, 1-exp(-5*dt)) 更新持久化均值。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"

// 阴影遮罩直方图(每桶一个 uint 计数)。
RWByteAddressBuffer __alias_shadow_mask_histogram : register(u13, space0);
// 平滑后的平均覆盖率(持久化标量)。
RWByteAddressBuffer __alias_shadow_mask_mean : register(u14, space0);

// 组共享的(灰度值, 权重)样本,256 线程各占一项。
groupshared float2 thread_vals[256];

[numthreads(256, 1, 1)]
void cs_main(uint groupIndex : SV_GroupIndex)
{
    // 桶号 -> 灰度值(桶号 * 1/255),权重 = 该桶计数;随后清零直方图。
    thread_vals[groupIndex] = float2(float(groupIndex) * 0.0039215688593685626983642578125f, 1.0f) * float(__alias_shadow_mask_histogram.Load<uint>(groupIndex * 4 + 0));
    __alias_shadow_mask_histogram.Store<uint>(groupIndex * 4 + 0, 0u);

    float mean;
    float old_mean;
    if (!(0u == frame_index))
    {
        // 非首帧:组内树形归约(步长 128..1)求总灰度与总权重,得到加权均值。
        GroupMemoryBarrierWithGroupSync();
        for (uint stride = 128u; stride > 0u; stride = stride >> 1u)
        {
            if (groupIndex < stride)
            {
                thread_vals[groupIndex] += thread_vals[groupIndex + stride];
            }
            GroupMemoryBarrierWithGroupSync();
        }
        mean = thread_vals[0].x / thread_vals[0].y;
        old_mean = clamp(__alias_shadow_mask_mean.Load<float>(0), 0.0f, 1.0f);
    }
    else
    {
        // 首帧:以 0.5 作为均值初值。
        mean = 0.5f;
        old_mean = 0.5f;
    }

    // 线程 0 把新均值按时间步长做指数平滑后写回(衰减系数 5/s)。
    if (groupIndex == 0u)
    {
        __alias_shadow_mask_mean.Store<float>(0, lerp(old_mean, mean, clamp(1.0f - exp(delta_time_seconds * (-5.0f)), 0.0f, 1.0f)));
    }
}
