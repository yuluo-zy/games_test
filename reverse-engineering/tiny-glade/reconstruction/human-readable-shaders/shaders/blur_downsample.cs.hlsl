// ==RECONSTRUCTED== shaders/blur_downsample.cs.hlsl | F-D
// Evidence: _blur_downsample.cs.hlsl_cs_main_0.bin (Texture2D input, entry cs_main)
//   + _blur_downsample.cs.hlsl_cs_main_multisampled_0.bin (Texture2DMS input,
//   entry cs_main_multisampled), LocalSize [64,1,1].
//   bindings: input_tex/input_ms_tex t0, output_tex u15 (unorm float4),
//   groupshared vblur_out[138].
//   spirv-cross: separable blur over a 128x2 tile per group: vertical pass (11 taps,
//   gaussian coeff 0.16) writes vblur_out, group barrier, horizontal pass (11 taps)
//   writes output.
// Status: O (structure from disassembly).
//
// === 中文说明 ===
// 可分离高斯模糊 + 垂直下采样:每个工作组覆盖 128x2 的 tile。
//   入口 cs_main(普通纹理)/ cs_main_multisampled(MSAA 纹理),工作组 64x1x1,无 TGV_ 变体。
//   算法: 垂直方向 11 抽头高斯(系数 0.16)写入 groupshared vblur_out,
//   组同步后水平方向 11 抽头并 2x 下采样写出。
// 绑定: input_tex / input_ms_tex t0, output_tex u15 (unorm float4), groupshared vblur_out[138]。

#include "inc/common.hlsl"

RWTexture2D<unorm float4> output_tex : register(u15, space0);
Texture2D<float4> input_tex : register(t0, space0);
Texture2DMS<float4> input_ms_tex : register(t0, space0);

groupshared float4 vblur_out[138];

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID,
             uint3 groupThreadID : SV_GroupThreadID,
             uint3 groupID : SV_GroupID)
{
    int2 gid_i = int2(dispatchThreadID.xy);

    // 垂直方向高斯模糊:每组 64 线程循环处理 138 个位置(含 5 像素横向余量)。
    for (uint i = groupThreadID.x; i < 138u; i += 64u)
    {
        int2 tile = int2((groupID.xy * uint2(128u, 2u)) + uint2(int2(int(i - 5u), -5)));

        // 11 抽头高斯(方差系数 0.16),采样输入纹理并加权平均。
        float weight_sum = 0.0f;
        float4 sum = 0.0f;
        for (uint j = 0u; j <= 10u; ++j)
        {
            float d = ((float(gid_i.y) + 0.5f) * 2.0f) - (float(uint(tile.y) + j) + 0.5f);
            float w = exp(-d * d * 0.1599999964237213134765625f);
            weight_sum += w;
            sum += input_tex.Load(int3(uint2(tile + int2(0, int(j))), 0)) * w;
        }
        vblur_out[i] = sum / weight_sum;
    }

    // 组同步:确保所有垂直结果写入 vblur_out 后再做水平方向。
    GroupMemoryBarrierWithGroupSync();

    // 水平方向 11 抽头高斯 + 2x 下采样,写出最终结果。
    float weight_sum = 0.0f;
    float4 sum = 0.0f;
    for (uint k = 0u; k <= 10u; ++k)
    {
        float d = ((float(dispatchThreadID.x) + 0.5f) * 2.0f) - (float((dispatchThreadID.x * 2u + k) - 5u) + 0.5f);
        float w = exp(-d * d * 0.1599999964237213134765625f);
        sum += vblur_out[groupThreadID.x * 2u + k] * w;
        weight_sum += w;
    }

    output_tex[dispatchThreadID.xy] = sum / weight_sum;
}

[numthreads(64, 1, 1)]
void cs_main_multisampled(uint3 dispatchThreadID : SV_DispatchThreadID,
                          uint3 groupThreadID : SV_GroupThreadID,
                          uint3 groupID : SV_GroupID)
{
    int2 gid_i = int2(dispatchThreadID.xy);

    // 与 cs_main 相同,唯一区别是输入为 MSAA 纹理 input_ms_tex(加载第 0 个子采样)。
    for (uint i = groupThreadID.x; i < 138u; i += 64u)
    {
        int2 tile = int2((groupID.xy * uint2(128u, 2u)) + uint2(int2(int(i - 5u), -5)));

        float weight_sum = 0.0f;
        float4 sum = 0.0f;
        for (uint j = 0u; j <= 10u; ++j)
        {
            float d = ((float(gid_i.y) + 0.5f) * 2.0f) - (float(uint(tile.y) + j) + 0.5f);
            float w = exp(-d * d * 0.1599999964237213134765625f);
            weight_sum += w;
            sum += input_ms_tex.Load(uint2(tile + int2(0, int(j))), 0u) * w;
        }
        vblur_out[i] = sum / weight_sum;
    }

    GroupMemoryBarrierWithGroupSync();

    float weight_sum = 0.0f;
    float4 sum = 0.0f;
    for (uint k = 0u; k <= 10u; ++k)
    {
        float d = ((float(dispatchThreadID.x) + 0.5f) * 2.0f) - (float((dispatchThreadID.x * 2u + k) - 5u) + 0.5f);
        float w = exp(-d * d * 0.1599999964237213134765625f);
        sum += vblur_out[groupThreadID.x * 2u + k] * w;
        weight_sum += w;
    }

    output_tex[dispatchThreadID.xy] = sum / weight_sum;
}
