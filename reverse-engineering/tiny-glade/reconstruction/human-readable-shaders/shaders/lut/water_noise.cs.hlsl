// ==RECONSTRUCTED== shaders/lut/water_noise.cs.hlsl | F-D
// 中文说明:
//   生成水面噪声 LUT(蓝噪声纹理的双三次重采样 + 导数)。证据来自
//   _lut_water_noise.cs.hlsl_cs_main_0.bin:绑定 sampler_llr s1、$Globals b13
//   (blue_noise_texture_dims/output_tex_dims)、blue_noise_texture t14、
//   output_tex u15(unorm float4),LocalSize [8,8,1]。对每个输出像素,在像素
//   及其 +/-1 纹素邻域对蓝噪声纹理做 3x3 加权重采样;输出
//   (value, -1/3*d/dx + 0.5, -1/3*d/dy + 0.5, 1)。
//   注意:3 抽头内核(w0=t*(-0.5+t*(1-0.5t)), w1=t*(0.5+t*(2-1.5t)),
//   w0b=t^2*(-0.5+0.5t), w2=1+t^2*(-2.5+1.5t)+w1)与 inc/bicubic.hlsl
//   (匹配 final_post)不同;此处按反汇编原样内联。

#include "inc/common.hlsl"
#include "inc/uv.hlsl"
#include "inc/bicubic.hlsl"
#include "inc/image.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 blue_noise_texture_dims;
    uint2 output_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
RWTexture2D<unorm float4> output_tex : register(u15, space0);

// 对蓝噪声纹理做双三次重采样(3x3 加权和)。
float4 sample_noise_bicubic(float2 p, float2 texel)
{
    float2 f = floor(p - 0.5f);
    float2 t = p - (f + 0.5f);
    float2 t2 = t * t;
    float2 t_half = t * 0.5f;
    float2 t15 = t * 1.5f;

    // Catmull-Rom 风格的 3 抽头权重。
    float2 w0 = t * (-0.5f + t * (1.0f - t_half));
    float2 w1 = t * (0.5f + t * (2.0f - t15));
    float2 w0b = t2 * (-0.5f + t_half);
    float2 w2 = (1.0f + t2 * (-2.5f + t15)) + w1;

    // 三个采样列位置。
    float2 p0 = (f + (-0.5f).xx) * texel;
    float2 p2 = (f + 2.5f.xx) * texel;
    float2 p1 = (f + 0.5f.xx + (w1 / w2)) * texel;

    float4 s00 = blue_noise_texture.SampleLevel(sampler_llr, p0, 0.0f);
    float4 s10 = blue_noise_texture.SampleLevel(sampler_llr, float2(p1.x, p0.y), 0.0f);
    float4 s20 = blue_noise_texture.SampleLevel(sampler_llr, float2(p2.x, p0.y), 0.0f);
    float4 s01 = blue_noise_texture.SampleLevel(sampler_llr, float2(p0.x, p1.y), 0.0f);
    float4 s11 = blue_noise_texture.SampleLevel(sampler_llr, p1, 0.0f);
    float4 s21 = blue_noise_texture.SampleLevel(sampler_llr, float2(p2.x, p1.y), 0.0f);
    float4 s02 = blue_noise_texture.SampleLevel(sampler_llr, float2(p0.x, p2.y), 0.0f);
    float4 s12 = blue_noise_texture.SampleLevel(sampler_llr, float2(p1.x, p2.y), 0.0f);
    float4 s22 = blue_noise_texture.SampleLevel(sampler_llr, p2, 0.0f);

    // 9 项加权和。
    return (s00 * w0.x * w0.y)
        + (s10 * w2.x * w0.y)
        + (s20 * w0b.x * w0.y)
        + (s01 * w0.x * w2.y)
        + (s11 * w2.x * w2.y)
        + (s21 * w0b.x * w2.y)
        + (s02 * w0.x * w0b.y)
        + (s12 * w2.x * w0b.y)
        + (s22 * w0b.x * w0b.y);
}

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    float2 out_dims = float2(output_tex_dims);
    // 读取蓝噪声纹理实际尺寸。
    uint bn_w;
    uint bn_h;
    uint bn_elements;
    blue_noise_texture.GetDimensions(0u, bn_w, bn_h, bn_elements);
    float2 bn_dims = float2(bn_w, bn_h);
    float2 texel = 1.0f / bn_dims;

    // 输出像素中心 + 右/左/上/下邻域映射到蓝噪声纹理坐标。
    float2 p = ((float2(int3(dispatchThreadID).xy) + 0.5f) / out_dims) * bn_dims;
    float2 pr = ((float2(int3(dispatchThreadID).xy + int2(1, 0)) + 0.5f) / out_dims) * bn_dims;
    float2 pl = ((float2(int3(dispatchThreadID).xy - int2(1, 0)) + 0.5f) / out_dims) * bn_dims;
    float2 pu = ((float2(int3(dispatchThreadID).xy + int2(0, 1)) + 0.5f) / out_dims) * bn_dims;
    float2 pd = ((float2(int3(dispatchThreadID).xy - int2(0, 1)) + 0.5f) / out_dims) * bn_dims;

    float4 center = sample_noise_bicubic(p, texel);
    float4 right = sample_noise_bicubic(pr, texel);
    float4 left = sample_noise_bicubic(pl, texel);
    float4 up = sample_noise_bicubic(pu, texel);
    float4 down = sample_noise_bicubic(pd, texel);

    // 输出值 + 中心差分导数(-1/3 缩放并偏移到 0.5)。
    output_tex[uint2(int3(dispatchThreadID).xy)] = float4(center.x, (right.x - left.x) * (-0.3333333432674407958984375f) + 0.5f, (up.x - down.x) * (-0.3333333432674407958984375f) + 0.5f, 1.0f);
}
