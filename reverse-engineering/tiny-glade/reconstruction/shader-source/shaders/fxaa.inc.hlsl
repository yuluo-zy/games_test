// ==RECONSTRUCTED== shaders/fxaa.inc.hlsl | F-E | evidence: _fxaa.cs.hlsl_cs_main_0.bin inlined FXAA (gather luma, edge-gradient direction select, subpixel blend, 1.5/2/4/12 quality tap chain, final blend taper) | status: O (algorithm/constants from inlined SPIR-V); I (helper names/signature — inlined; split between inc and entry is Inference)
//
// FXAA 3.11-style antialiasing core operating on the caller-provided texture.

#ifndef TINYGLADE_FXAA_INC_HLSL
#define TINYGLADE_FXAA_INC_HLSL

static const float FXAA_EDGE_THRESHOLD = 0.166f;
static const float FXAA_SUBPIXEL = 0.75f;

// Run FXAA on tex at uv. rcp = 1/dims. color_center = sampled color at uv.
float4 fxaa(Texture2D<float4> tex, SamplerState samp, float2 uv, float2 rcp, float4 color_center)
{
    float4 lumaNW = tex.GatherGreen(samp, uv, int2(-1, -1));
    float4 lumaNE = tex.GatherGreen(samp, uv);

    float lumaN = lumaNE.x;
    float lumaC = color_center.y;
    float lumaS = lumaNE.z;
    float lumaW = lumaNW.z;
    float lumaE = lumaNW.x;

    float lumaMax = max(max(lumaN, lumaE), max(lumaS, max(lumaC, lumaW)));
    float lumaMin = min(min(lumaN, lumaE), min(lumaS, min(lumaC, lumaW)));
    float lumaRange = lumaMax - lumaMin;

    if (lumaRange < max(0.0f, lumaMax * FXAA_EDGE_THRESHOLD))
        return color_center;

    float4 sampleB = tex.SampleLevel(samp, uv, 0.0f, int2(1, -1));
    float4 sampleC = tex.SampleLevel(samp, uv, 0.0f, int2(-1, 1));
    float lumaSW2 = lumaNW.z;
    float lumaSE2 = lumaNW.x;

    // Edge gradient in both directions.
    float lumaGradVert = (-2.0f * lumaC) + lumaSW2 + sampleC.y;
    float lumaGradHoriz = (-2.0f * lumaC) + lumaSE2 + sampleB.y;

    float gradVert = abs(lumaGradHoriz) + (abs(lumaGradVert + lumaSE2) * 2.0f) + abs(lumaGradHoriz + lumaNE.w);
    float gradHoriz = abs(lumaGradVert) + (abs(lumaGradHoriz + lumaSW2) * 2.0f) + abs(lumaGradVert + lumaNE.w);
    bool horizontal = gradHoriz >= gradVert;

    // Direction of the dominant edge.
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

    // Sub-pixel blend estimate.
    float lumaAvg = (lumaN + lumaS + lumaE + lumaW + lumaC) * 0.083333336f;
    float subpixel = clamp(abs(lumaAvg - lumaC) / lumaRange, 0.0f, 1.0f);
    float subpixelTap = horizontal ? 0.0f : rcp.x;

    // Center the search on the edge and move out.
    float2 pix = uv;
    if (horizontal)
        pix.y = uv.y + dir.y * 0.5f;
    else
        pix.x = uv.x + dir.x * 0.5f;

    float2 neg = pix - dir * subpixelTap;
    float2 pos = pix + dir * subpixelTap;

    float lumaTap = max(abs(lumaEdgeN - lumaC), abs(lumaEdgeE - lumaC)) * 0.25f;
    float lumaCenter = ((horizontal ? lumaEdgeE : lumaEdgeN) + lumaC) * 0.5f;
    float subpixelSmooth = ((-2.0f * subpixel) + 3.0f) * (subpixel * subpixel);
    bool lumaBelow = (lumaC - lumaCenter) < 0.0f;

    float lumaL = tex.SampleLevel(samp, neg, 0.0f).y - lumaCenter;
    float lumaR = tex.SampleLevel(samp, pos, 0.0f).y - lumaCenter;

    // Quality chain (1.5, 2, 4, 12 pixels).
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

    // Blend amount: use the nearer side that crossed below the edge.
    float spanL = horizontal ? pix.x - negL.x : pix.y - negL.y;
    float spanR = horizontal ? posR.x - pix.x : posR.y - pix.y;
    float spanMax = max(spanL, spanR);

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

    float2 final_uv = uv;
    if (horizontal)
        final_uv.y = uv.y + blend * dir.y;
    else
        final_uv.x = uv.x + blend * dir.x;

    return float4(tex.SampleLevel(samp, final_uv, 0.0f).xyz, 0.0f);
}

#endif // TINYGLADE_FXAA_INC_HLSL
