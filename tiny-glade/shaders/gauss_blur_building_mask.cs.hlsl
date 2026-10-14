// ==RECONSTRUCTED== shaders/gauss_blur_building_mask.cs.hlsl | F-D
// 证据: _gauss_blur_building_mask.cs.hlsl_cs_main_0.bin (SPIR-V, 偏移 23)。
//   绑定: $Globals b13 (building_mask_dims/out_building_mask_dims/is_horizontal),
//   building_mask t14, out_building_mask u15 (float2), 工作组尺寸 [8,8,1],
//   入口 cs_main。
//   用途: 对建筑掩码做一维可分离高斯模糊,固定半径 20 (ceil(20.0f)),权重
//   exp(-t^2 * 0.005)(即 sigma=10),只累加界内纹素;输出 .x 为均值,保留 .y。
// 状态: O(结构来自反汇编;成员名原样保留)。building_mask.inc.hlsl 被包含
//   (已嵌入二进制)但在此不产生代码(骨架)。

#include "inc/common.hlsl"
#include "buildings/building_mask.inc.hlsl"

// 掩码尺寸与模糊方向。
cbuffer $Globals : register(b13, space0)
{
    uint2 building_mask_dims;
    uint2 out_building_mask_dims;
    uint is_horizontal;
};

// 建筑掩码输入(x 分量参与均值)。
Texture2D<float4> building_mask : register(t14, space0);
// 模糊结果输出(.x = 均值,.y 原样保留)。
RWTexture2D<float2> out_building_mask : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // 固定模糊半径(向上取整到 20)。
    int radius = int(ceil(20.0f));

    // 加权和与权重累加和。
    float weighted_sum = 0.0f;
    float weight_sum = 0.0f;

    // 水平/垂直方向选择:水平时沿 x 偏移,垂直时沿 y 偏移。
    bool horizontal = is_horizontal != 0u;
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

        // 只累加落在掩码范围内的纹素。
        if (all(upixel < building_mask_dims))
        {
            float t = float(offset);
            // 高斯权重 exp(-t^2 * 0.005)。
            float weight = exp((-t * t) * 0.004999999888241291046142578125f);
            weighted_sum += building_mask.Load(int3(upixel, 0)).x * weight;
            weight_sum += weight;
        }
    }

    // 归一化均值写到 .x,保留原有 .y。
    out_building_mask[uint2(int3(dispatchThreadID).xy)] = float2(weighted_sum / weight_sum, out_building_mask[uint2(int3(dispatchThreadID).xy)].y);
}
