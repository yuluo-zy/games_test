// ==RECONSTRUCTED== shaders/fxaa.inc.hlsl | F-E | 证据: _fxaa.cs.hlsl_cs_main_0.bin
//   内联的 FXAA(gather 亮度、边缘梯度方向选择、亚像素混合、1.5/2/4/12 质量采样链、
//   最终混合收窄)| 状态: O(算法/常量来自内联 SPIR-V); I(辅助名/签名 — 内联;
//   inc 与入口的拆分属推断)
//
// 用途: FXAA 3.11 风格抗锯齿核心,作用于调用方提供的纹理。

#ifndef TINYGLADE_FXAA_INC_HLSL
#define TINYGLADE_FXAA_INC_HLSL

// 边缘阈值与亚像素混合强度常量。
static const float FXAA_EDGE_THRESHOLD = 0.166f;
static const float FXAA_SUBPIXEL = 0.75f;

// 在 tex 的 uv 处运行 FXAA。rcp = 1/尺寸。color_center = 采样到的 uv 颜色。
float4 fxaa(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 rcp, float4 color_center)
{
    // 用 GatherGreen 一次性取得 4 邻域亮度(左上/右上/左下/右下)。
    float4 lumaNW = tex.GatherGreen(samp, uv, int2(-1, -1));
    float4 lumaNE = tex.GatherGreen(samp, uv);

    // 从两次 gather 中拆出 N/E/S/W/C 五点亮度。
    float lumaN = lumaNE.x;
    float lumaC = color_center.y;
    float lumaS = lumaNE.z;
    float lumaW = lumaNW.z;
    float lumaE = lumaNW.x;

    // 亮度范围(最大值-最小值)。
    float lumaMax = max(max(lumaN, lumaE), max(lumaS, max(lumaC, lumaW)));
    float lumaMin = min(min(lumaN, lumaE), min(lumaS, min(lumaC, lumaW)));
    float lumaRange = lumaMax - lumaMin;

    // 亮度变化太小:不是边缘,直接返回中心色。
    if (lumaRange < max(0.0f, lumaMax * FXAA_EDGE_THRESHOLD))
        return color_center;

    // 额外采样 B(右)与 C(下),用于计算边缘梯度。
    float4 sampleB = tex.SampleLevel(samp, uv, 0.0f, int2(1, -1));
    float4 sampleC = tex.SampleLevel(samp, uv, 0.0f, int2(-1, 1));
    float lumaSW2 = lumaNW.z;
    float lumaSE2 = lumaNW.x;

    // 两个方向的边缘梯度。
    float lumaGradVert = (-2.0f * lumaC) + lumaSW2 + sampleC.y;
    float lumaGradHoriz = (-2.0f * lumaC) + lumaSE2 + sampleB.y;

    // 用带二阶差分项的长度比较,判断主导边缘是横向还是纵向。
    float gradVert = abs(lumaGradHoriz) + (abs(lumaGradVert + lumaSE2) * 2.0f) + abs(lumaGradHoriz + lumaNE.w);
    float gradHoriz = abs(lumaGradVert) + (abs(lumaGradHoriz + lumaSW2) * 2.0f) + abs(lumaGradVert + lumaNE.w);
    bool horizontal = gradHoriz >= gradVert;

    // 主导边缘的方向(水平或垂直)。
    float2 dir;
    float lumaEdgeN;
    float lumaEdgeE;
    if (horizontal)
    {
        dir = float2(rcp.x, 0.0f);
        lumaEdgeN = lumaSE2;
        lumaEdgeE = lumaNE.x;
    }
    else
    {
        dir = float2(0.0f, rcp.y);
        lumaEdgeN = lumaSW2;
        lumaEdgeE = lumaNE.z;
    }

    // 亚像素混合量估计(中心亮度与邻域均值的偏差)。
    float lumaAvg = (lumaN + lumaS + lumaE + lumaW + lumaC) * 0.083333336f;
    float subpixel = clamp(abs(lumaAvg - lumaC) / lumaRange, 0.0f, 1.0f);
    float subpixelTap = horizontal ? 0.0f : rcp.x;

    // 把搜索起点移到边缘上,再向两侧外推。
    float2 pix = uv;
    if (horizontal)
        pix.y = uv.y + dir.y * 0.5f;
    else
        pix.x = uv.x + dir.x * 0.5f;

    float2 neg = pix - dir * subpixelTap;
    float2 pos = pix + dir * subpixelTap;

    // 边缘判定阈值与中心亮度基准;亚像素平滑曲线 (-2s+3)*s^2。
    float lumaTap = max(abs(lumaEdgeN - lumaC), abs(lumaEdgeE - lumaC)) * 0.25f;
    float lumaCenter = ((horizontal ? lumaEdgeE : lumaEdgeN) + lumaC) * 0.5f;
    float subpixelSmooth = ((-2.0f * subpixel) + 3.0f) * (subpixel * subpixel);
    bool lumaBelow = (lumaC - lumaCenter) < 0.0f;

    // 向两侧各采样一次亮度,与边缘中心亮度做差。
    float lumaL = tex.SampleLevel(samp, neg, 0.0f).y - lumaCenter;
    float lumaR = tex.SampleLevel(samp, pos, 0.0f).y - lumaCenter;

    // 质量链(1.5, 2, 4, 12 像素),逐步外扩直到亮度越过阈值或到尽头。
    bool doneL = !(abs(lumaL) >= lumaTap);
    bool doneR = !(abs(lumaR) >= lumaTap);
    float2 negL = doneL ? neg - dir * (subpixelTap * 1.5f) : neg;
    float2 posR = doneR ? pos + dir * (subpixelTap * 1.5f) : pos;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 2.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 2.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 4.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 4.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;
    doneL = !(abs(lumaL) >= lumaTap);
    doneR = !(abs(lumaR) >= lumaTap);
    negL = doneL ? negL - dir * (subpixelTap * 12.0f) : negL;
    posR = doneR ? posR + dir * (subpixelTap * 12.0f) : posR;

    lumaL = doneL ? tex.SampleLevel(samp, negL, 0.0f).y - lumaCenter : lumaL;
    lumaR = doneR ? tex.SampleLevel(samp, posR, 0.0f).y - lumaCenter : lumaR;

    // 混合量:取越过边缘的较近一侧跨度。
    float spanL = horizontal ? pix.x - negL.x : pix.y - negL.y;
    float spanR = horizontal ? posR.x - pix.x : posR.y - pix.y;
    float spanMax = max(spanL, spanR);

    // 只有越过边界的相反侧才计入混合,再与亚像素平滑量取较大值。
    float blend = 0.0f;
    if (spanL < spanR)
    {
        if ((lumaL < 0.0f) != lumaBelow)
            blend = (min(spanL, spanR) * (-1.0f / (spanR + spanL))) + 0.5f;
    }
    else
    {
        if ((lumaR < 0.0f) != lumaBelow)
            blend = (min(spanL, spanR) * (-1.0f / (spanR + spanL))) + 0.5f;
    }
    blend = max(blend, subpixelSmooth * subpixelSmooth * FXAA_SUBPIXEL);

    // 按混合量沿边缘法线平移最终采样位置。
    float2 final_uv = uv;
    if (horizontal)
        final_uv.y = uv.y + blend * dir.y;
    else
        final_uv.x = uv.x + blend * dir.x;

    return float4(tex.SampleLevel(samp, final_uv, 0.0f).xyz, 0.0f);
}

#endif // TINYGLADE_FXAA_INC_HLSL
