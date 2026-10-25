// Vermeer 缩略图链:对获取的目标做高斯卷积,带与 painterly 链相同的
// 照片风格水平衰减。7x7 高斯采样(每轴权重 {20,15,6,1}),
// 水平扩散 lerp(6,0,smoothstep(0,0.5,uv.x))。
// 绑定:$Globals b14{output_tex_dims},input_tex t13,output_tex u15。LocalSize 8,8,1。

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
    // 归一化 UV;水平方向越靠近左边缘扩散越大(照片风格边缘聚焦)。
    float2 uv = (float2(int2(dispatch_thread_id.xy)) + 0.5f.xx) / float2(output_tex_dims);
    float spread = lerp(6.0f, 0.0f, smoothstep(0.0f, 0.5f, uv.x));

    float4 color_acc = 0.0f.xxxx;
    float weight_acc = 0.0f;
    // 7x7 高斯环,权重为横向权重 × 纵向权重。
    for (int j = -3; j <= 3; j++)
    {
        for (int i = -3; i <= 3; i++)
        {
            float w = gaussian_weights[abs(i)] * gaussian_weights[abs(j)];
            weight_acc += w;
            color_acc += input_tex.SampleLevel(sampler_lnc, uv + ((float2(float(i), float(j)) / float2(output_tex_dims)) * spread), 0.0f) * w;
        }
    }

    output_tex[dispatch_thread_id.xy] = color_acc / weight_acc.xxxx;
}
