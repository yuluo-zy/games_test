// ==RECONSTRUCTED== shaders/painterly/convolve_target.cs.hlsl | evidence: _painterly_convolve_target.cs.hlsl_cs_main_0.bin — 7x7 gaussian gather (weights {20,15,6,1} per axis) with a horizontal spread lerp(6,0,smoothstep(0,0.5,uv.x)); LocalSize 8,8,1; caps Shader only | status: O (gather math + resources from inlined SPIR-V); I (gaussian_weights name from embedded OpName)
//
// 画笔缩略图链:对已采集目标做近似可分的高斯卷积。模糊半径向画面右侧收缩
// (照片式衰减),让左边缘更柔和。

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 output_tex_dims;
};

static const float gaussian_weights[4] = { 20.0f, 15.0f, 6.0f, 1.0f };

Texture2D<float4> input_tex : register(t13, space0);
RWTexture2D<unorm float4> output_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    float2 uv = (float2(int2(dispatch_thread_id.xy)) + 0.5f.xx) / float2(output_tex_dims);
    // 采样展宽:uv.x 越靠右,展宽越小(6 -> 0)。
    float spread = lerp(6.0f, 0.0f, smoothstep(0.0f, 0.5f, uv.x));

    float4 color_acc = 0.0f.xxxx;
    float weight_acc = 0.0f;
    // 7x7 高斯采样:权 = 两轴权值乘积,偏移按 spread 缩放。
    for (int j = -3; j <= 3; j++)
    {
        for (int i = -3; i <= 3; i++)
        {
            float w = gaussian_weights[abs(i)] * gaussian_weights[abs(j)];
            weight_acc += w;
            color_acc += input_tex.SampleLevel(sampler_lnc, uv + ((float2(float(i), float(j)) / float2(output_tex_dims)) * spread), 0.0f) * w;
        }
    }

    // 归一化输出。
    output_tex[dispatch_thread_id.xy] = color_acc / weight_acc.xxxx;
}
