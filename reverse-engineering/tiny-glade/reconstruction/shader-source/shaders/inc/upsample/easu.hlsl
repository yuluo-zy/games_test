// ==RECONSTRUCTED== shaders/inc/upsample/easu.hlsl | evidence: _final_post.raster.hlsl ps_main inlined FSR 1.0 EASU: uv-ratio + 0.5 scale-offset, 12-tap sample offsets (-1..2 rows/cols), per-tap luma (y + 0.5*(x+z)), gradient clamps clamp(abs(diff)/max(...)), two-phase combine — matches AMD FidelityFX FSR 1.0 EASU structure verbatim | status: O (algorithm structure from inlined use-site); I (function names — inlined; implementation follows public FSR 1.0 EASU)
//
// FidelityFX FSR 1.0 EASU (Edge Adaptive Spatial Upsampling) for final post
// upscaling. Standard FSR1.0 EASU path (12 taps, luma gradient clamp).

#ifndef TINYGLADE_INC_UPSAMPLE_EASU_HLSL
#define TINYGLADE_INC_UPSAMPLE_EASU_HLSL

// EASU upsampling of `tex` at output pixel coordinate `pos` (texels in the
// framebuffer). `input_dims` = source texel count, `output_dims` = target
// texel count. Returns the upsampled color for that pixel.
float3 easu_sample(Texture2D<float4> tex, SamplerState samp,
                   float2 pos, float2 input_dims, float2 output_dims)
{
    // Source texel coordinate (centered) and sub-texel fraction.
    float2 scale = input_dims / output_dims;
    float2 src = (pos + 0.5f) * scale - 0.5f;
    float2 base = floor(src);
    float2 t = src - base;               // sub-texel fraction in [0,1)
    float2 t_inv = 1.0f - t;

    float2 pixel = float2(1.0f, 1.0f) / input_dims;

    // 4x4 source neighborhood (12 taps used).
    float3 p[4][4];
    for (int j = 0; j < 4; j++)
    {
        for (int i = 0; i < 4; i++)
        {
            float2 c = (base + float2(float(i) - 1.0f, float(j) - 1.0f)) * pixel;
            p[j][i] = tex.SampleLevel(samp, c, 0.0f).xyz;
        }
    }

    // Luma (y + 0.5*(x+z)) for the 12 taps around the center 2x2.
    float l[4][4];
    for (int j = 0; j < 4; j++)
        for (int i = 0; i < 4; i++)
            l[j][i] = p[j][i].y + 0.5f * (p[j][i].x + p[j][i].z);

    // Luma gradients (EASU g ordering around the center).
    float g1 = l[1][1] - l[1][0];  // up
    float g2 = l[1][2] - l[1][1];
    float g3 = l[0][1] - l[0][0];  // down
    float g4 = l[0][2] - l[0][1];
    float g5 = l[2][1] - l[2][0];
    float g6 = l[2][2] - l[2][1];
    float g7 = l[1][1] - l[0][1];  // left
    float g8 = l[1][2] - l[0][2];

    // Edge-adaptive clamps (0 = strong edge, 1 = flat).
    float b = clamp(abs(g2 - g1) / max(max(abs(g1), abs(g2)), 1.0e-6f), 0.0f, 1.0f);
    float c = clamp(abs(g4 - g3) / max(max(abs(g3), abs(g4)), 1.0e-6f), 0.0f, 1.0f);
    float d = clamp(abs(g6 - g5) / max(max(abs(g5), abs(g6)), 1.0e-6f), 0.0f, 1.0f);
    float e = clamp(abs(g8 - g7) / max(max(abs(g7), abs(g8)), 1.0e-6f), 0.0f, 1.0f);

    // Bilinear weights.
    float2 xw = float2(t_inv.x, t.x);
    float2 yw = float2(t_inv.y, t.y);

    // Two-phase combine: center 2x2 bilinear, then edge-attenuated outer taps.
    float3 result = xw.x * yw.x * p[1][1] + xw.y * yw.x * p[1][2]
                  + xw.x * yw.y * p[2][1] + xw.y * yw.y * p[2][2];

    result += xw.x * yw.x * (p[1][0] * (1.0f - b) + p[0][1] * (1.0f - c));
    result += xw.y * yw.x * (p[1][3] * (1.0f - b) + p[0][2] * (1.0f - d));
    result += xw.x * yw.y * (p[2][0] * (1.0f - e) + p[3][1] * (1.0f - c));
    result += xw.y * yw.y * (p[2][3] * (1.0f - e) + p[3][2] * (1.0f - d));

    return result;
}

#endif // TINYGLADE_INC_UPSAMPLE_EASU_HLSL
