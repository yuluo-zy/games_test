// ==RECONSTRUCTED== shaders/inc/color/srgb.hlsl | evidence: _blit_stretch_decode_srgb ps inlined decode (c>0.04045 ? pow((c+0.055)*0.947867333f, 2.400000095f) : c*0.0773993805f, per channel; 0.947867333=1/1.055, 0.0773993805=1/12.92); 232 consumer bins (blit/resolve/lighting all decode) | status: O (decode polynomial + constants from inlined use-site); I (function names — inlined; encode = standard inverse)
//
// sRGB <-> linear conversion (per-channel and vector forms).

#ifndef TINYGLADE_INC_COLOR_SRGB_HLSL
#define TINYGLADE_INC_COLOR_SRGB_HLSL

static const float SRGB_ALPHA = 0.04045f;
static const float SRGB_OFFSET = 0.055f;
static const float SRGB_GAMMA = 2.4f;

// sRGB channel -> linear.
float srgb_to_linear(float c)
{
    float r;
    if (c <= SRGB_ALPHA)
        r = c * 0.077399380505084991455078125f; // / 12.92
    else
        r = pow((c + SRGB_OFFSET) * 0.947867333889007568359375f, SRGB_GAMMA); // * 1/1.055
    return r;
}

float3 srgb_to_linear(float3 c)
{
    return float3(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b));
}

float4 srgb_to_linear(float4 c)
{
    return float4(srgb_to_linear(c.rgb), c.a);
}

// Linear channel -> sRGB (standard inverse).
float linear_to_srgb(float c)
{
    float r;
    if (c <= 0.0031308f)
        r = c * 12.92f;
    else
        r = 1.055f * pow(c, 1.0f / SRGB_GAMMA) - SRGB_OFFSET;
    return r;
}

float3 linear_to_srgb(float3 c)
{
    return float3(linear_to_srgb(c.r), linear_to_srgb(c.g), linear_to_srgb(c.b));
}

float4 linear_to_srgb(float4 c)
{
    return float4(linear_to_srgb(c.rgb), c.a);
}

#endif // TINYGLADE_INC_COLOR_SRGB_HLSL
