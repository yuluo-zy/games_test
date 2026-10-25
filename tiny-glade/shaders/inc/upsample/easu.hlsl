// FidelityFX FSR 1.0 EASU(边缘自适应空间上采样),用于最终后处理上采样。
// 标准 FSR1.0 EASU 路径(12 次采样、亮度梯度钳制)。

#ifndef TINYGLADE_INC_UPSAMPLE_EASU_HLSL
#define TINYGLADE_INC_UPSAMPLE_EASU_HLSL

// 在输出像素坐标 `pos` 处对 `tex` 做 EASU 上采样(纹素单位)。
// `input_dims` = 源纹素数,`output_dims` = 目标纹素数。返回该像素的上采样颜色。
float3 easu_sample(Texture2D<float4> tex, SamplerState samp,
                   float2 pos, float2 input_dims, float2 output_dims)
{
    // 源纹素坐标(居中)与亚纹素小数部分。
    float2 scale = input_dims / output_dims;
    float2 src = (pos + 0.5f) * scale - 0.5f;
    float2 base = floor(src);
    float2 t = src - base;               // [0,1) 内的亚纹素小数
    float2 t_inv = 1.0f - t;

    float2 pixel = float2(1.0f, 1.0f) / input_dims;

    // 4x4 源邻域(实际使用 12 次采样)。
    float3 p[4][4];
    for (int j = 0; j < 4; j++)
    {
        for (int i = 0; i < 4; i++)
        {
            float2 c = (base + float2(float(i) - 1.0f, float(j) - 1.0f)) * pixel;
            p[j][i] = tex.SampleLevel(samp, c, 0.0f).xyz;
        }
    }

    // 中心 2x2 周围 12 次采样的亮度(y + 0.5*(x+z))。
    float l[4][4];
    for (int j = 0; j < 4; j++)
        for (int i = 0; i < 4; i++)
            l[j][i] = p[j][i].y + 0.5f * (p[j][i].x + p[j][i].z);

    // 亮度梯度(EASU 中心 g 排列顺序)。
    float g1 = l[1][1] - l[1][0];  // 上
    float g2 = l[1][2] - l[1][1];
    float g3 = l[0][1] - l[0][0];  // 下
    float g4 = l[0][2] - l[0][1];
    float g5 = l[2][1] - l[2][0];
    float g6 = l[2][2] - l[2][1];
    float g7 = l[1][1] - l[0][1];  // 左
    float g8 = l[1][2] - l[0][2];

    // 边缘自适应钳制(0 = 强边缘,1 = 平坦)。
    float b = clamp(abs(g2 - g1) / max(max(abs(g1), abs(g2)), 1.0e-6f), 0.0f, 1.0f);
    float c = clamp(abs(g4 - g3) / max(max(abs(g3), abs(g4)), 1.0e-6f), 0.0f, 1.0f);
    float d = clamp(abs(g6 - g5) / max(max(abs(g5), abs(g6)), 1.0e-6f), 0.0f, 1.0f);
    float e = clamp(abs(g8 - g7) / max(max(abs(g7), abs(g8)), 1.0e-6f), 0.0f, 1.0f);

    // 双线性权重。
    float2 xw = float2(t_inv.x, t.x);
    float2 yw = float2(t_inv.y, t.y);

    // 两阶段组合:中心 2x2 双线性,再叠加边缘衰减的外圈采样。
    float3 result = xw.x * yw.x * p[1][1] + xw.y * yw.x * p[1][2]
                  + xw.x * yw.y * p[2][1] + xw.y * yw.y * p[2][2];

    result += xw.x * yw.x * (p[1][0] * (1.0f - b) + p[0][1] * (1.0f - c));
    result += xw.y * yw.x * (p[1][3] * (1.0f - b) + p[0][2] * (1.0f - d));
    result += xw.x * yw.y * (p[2][0] * (1.0f - e) + p[3][1] * (1.0f - c));
    result += xw.y * yw.y * (p[2][3] * (1.0f - e) + p[3][2] * (1.0f - d));

    return result;
}

#endif // TINYGLADE_INC_UPSAMPLE_EASU_HLSL
