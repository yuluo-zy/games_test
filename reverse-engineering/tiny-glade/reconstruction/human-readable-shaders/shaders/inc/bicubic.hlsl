// ============================================================
// inc/bicubic.hlsl —— 双三次(Catmull-Rom)插值采样
// 用途:纹理采样时做 4x4 双三次插值,比双线性更平滑。
//   被 final_post 像素着色器与 lut_water_noise 内联使用。
// 入口:无(纯头文件)
// 关键算法:Catmull-Rom 核权重 w0..w3(见 bicubic_weights)
// ============================================================
// ==RECONSTRUCTED== shaders/inc/bicubic.hlsl | evidence: _final_post.raster.hlsl ps_main + _lut_water_noise inlined Catmull-Rom weights: w0=t*(-0.5+t*(1-0.5t)); w1=t*(0.5+t*(2-0.5t)); w3=(1-t)*(-0.5+0.5t); w2=1+(1-t)*(-2.5+t*(2-0.5t))+w3 (final_post lines ~124-134); include-graph edge math.hlsl -> bicubic.hlsl | status: O (weight polynomial from inlined use-site); I (function names/sample function Inference)
//
// Bicubic (Catmull-Rom) interpolation for texture sampling.

#ifndef TINYGLADE_INC_BICUBIC_HLSL
#define TINYGLADE_INC_BICUBIC_HLSL

// 计算小数偏移 f(∈[0,1))对应的四个 Catmull-Rom 核权重。
// 顺序与 final_post 内联计算一致:w0 对应 (f-1) 采样点,
//   w1 对应 f, w2 对应 (f+1), w3 对应 (f+2)。
float4 bicubic_weights(float f)
{
    float t = f;
    float t_half = t * 0.5f;
    float w0 = t * (-0.5f + t * (1.0f - t_half));
    float w1 = t * (0.5f + t * (2.0f - t_half));
    float one_minus_t = 1.0f - t;
    float w3 = one_minus_t * (-0.5f + t_half);
    float w2 = (1.0f + one_minus_t * (-2.5f + t * (2.0f - t_half))) + w3;
    return float4(w0, w1, w2, w3);
}

// 在给定 UV 处做 4x4 双三次(Catmull-Rom)纹理采样。
float4 bicubic_sample(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 texel_size)
{
    float2 p = uv * (float2(1.0f, 1.0f) / texel_size) - 0.5f;
    float2 f = frac(p);
    float2 base = (p - f) * texel_size;
    float4 wx = bicubic_weights(f.x);
    float4 wy = bicubic_weights(f.y);

    // 采样 4x4 邻域:wy 权重作用于 y 方向 4 行,每行 4 个点。
    float4 c0 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f, -1.0f) * texel_size, 0.0f);
    float4 c1 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f, -1.0f) * texel_size, 0.0f);
    float4 c2 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f, -1.0f) * texel_size, 0.0f);
    float4 c3 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f, -1.0f) * texel_size, 0.0f);
    float4 c01 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  0.0f) * texel_size, 0.0f);
    float4 c11 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  0.0f) * texel_size, 0.0f);
    float4 c21 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  0.0f) * texel_size, 0.0f);
    float4 c31 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  0.0f) * texel_size, 0.0f);
    float4 c02 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  1.0f) * texel_size, 0.0f);
    float4 c12 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  1.0f) * texel_size, 0.0f);
    float4 c22 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  1.0f) * texel_size, 0.0f);
    float4 c32 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  1.0f) * texel_size, 0.0f);
    float4 c03 = wy.x * tex.SampleLevel(samp, base + float2(-1.0f,  2.0f) * texel_size, 0.0f);
    float4 c13 = wy.y * tex.SampleLevel(samp, base + float2( 0.0f,  2.0f) * texel_size, 0.0f);
    float4 c23 = wy.z * tex.SampleLevel(samp, base + float2( 1.0f,  2.0f) * texel_size, 0.0f);
    float4 c33 = wy.w * tex.SampleLevel(samp, base + float2( 2.0f,  2.0f) * texel_size, 0.0f);

    // 先按 y 行累加、再按 x 权重合成最终颜色。
    float4 col0 = (c0 + c1 + c2 + c3) * wx.x
                + (c01 + c11 + c21 + c31) * wx.y
                + (c02 + c12 + c22 + c32) * wx.z
                + (c03 + c13 + c23 + c33) * wx.w;
    return col0;
}

#endif // TINYGLADE_INC_BICUBIC_HLSL
