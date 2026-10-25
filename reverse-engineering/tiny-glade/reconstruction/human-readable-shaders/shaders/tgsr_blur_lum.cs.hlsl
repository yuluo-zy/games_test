// ==RECONSTRUCTED== shaders/tgsr_blur_lum.cs.hlsl | A (dof-aa review, 2026-10-13)
// TGSR 亮度模糊 pass:对亮度做 3x3 高斯模糊(权重 exp(-(i^2+j^2)*0.75)),
// 采样点按当前裁剪偏移的一半去抖动,结果写 sqrt(加权亮度 / 权重和)。
// 说明:RWTexture2D<unorm float> R8 与 B 版一样缺少 StorageImageWriteWithoutFormat 能力。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "tgsr_common.inc.hlsl"
#include "inc/color/ycbcr.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 color_tex_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> color_tex : register(t14, space0);
[[vk::ext_capability(49)]] RWTexture2D<unorm float> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID)
{
    float2 color_dims = float2(color_tex_dims);
    float2 uv = (float2(int2(dispatch_id.xy)) + 0.5f) / color_dims;

    float weight_sum = 0.0f;
    float lum_sum = 0.0f;
    // 3x3 高斯邻域:中心权重最大,越远越轻。
    [loop]
    for (int dy = -1; dy <= 1; ++dy)
    {
        [loop]
        for (int dx = -1; dx <= 1; ++dx)
        {
            float w = exp(float((dx * dx) + (dy * dy)) * (-0.75f));
            // 采样坐标 = 中心 uv + 偏移,再减去当前帧裁剪偏移的一半(去抖动)。
            float2 sample_uv = (uv + ((float2(float(dx), float(dy)) * 2.0f) / color_dims))
                - ((view_constants[0].sample_offset_clip * 0.5f) * float2(1.0f, -1.0f));
            // 用 Rec.709 亮度系数加权累加。
            lum_sum += dot(color_tex.SampleLevel(sampler_lnc, sample_uv, 0.0f).xyz,
                float3(0.2126f, 0.7152f, 0.0722f)) * w;
            weight_sum += w;
        }
    }

    // 归一化后开根号:把"平均亮度"映射回线性域。
    output_tex[dispatch_id.xy] = sqrt(lum_sum / weight_sum).x;
}
