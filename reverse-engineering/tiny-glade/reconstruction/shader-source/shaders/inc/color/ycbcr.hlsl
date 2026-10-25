// ==RECONSTRUCTED== shaders/inc/color/ycbcr.hlsl | evidence: _taa.cs.hlsl_cs_main_0 and _tgsr inlined Rec.709 RGB<->YCbCr matrix — RGB->Y row (0.2126,0.7152,0.0722); chroma rows (-0.1146,-0.3854,0.5) and (0.5,-0.4542,-0.0458), i.e. B-Y / R-Y scaled to +/-0.5 | status: O (matrix verbatim from inlined use-site); I (function names — inlined)
//
// Rec.709 YCbCr conversion with chroma in [-0.5, 0.5] (used by temporal
// accumulation shaders to keep luma/chroma separated).

#ifndef TINYGLADE_INC_COLOR_YCBCR_HLSL
#define TINYGLADE_INC_COLOR_YCBCR_HLSL

float3 rgb_to_ycbcr(float3 rgb)
{
    return mul(
        float3x3(
            float3( 0.2126f,  0.7152f,  0.0722f),
            float3(-0.1146f, -0.3854f,  0.5f),
            float3( 0.5f,   -0.4542f, -0.0458f)),
        rgb);
}

float3 ycbcr_to_rgb(float3 ycbcr)
{
    // Exact inverse of the matrix above.
    return mul(
        float3x3(
            float3(1.0f,        1.0f,       1.0f),
            float3(0.0f,       -0.18732f,   1.8556f),
            float3(1.5748f,    -0.46812f,   0.0f)),
        ycbcr);
}

#endif // TINYGLADE_INC_COLOR_YCBCR_HLSL
