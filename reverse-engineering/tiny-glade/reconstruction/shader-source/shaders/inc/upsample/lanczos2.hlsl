// ==RECONSTRUCTED== shaders/inc/upsample/lanczos2.hlsl | evidence: consumer _final_post.raster.hlsl (only; upsampling_mode branch); inlined Catmull-Rom bicubic weights cover the bicubic mode; Lanczos2 not directly observable in the sampled path | status: U (implementation Inference — standard 2-lobe Lanczos kernel; used as an upsampling mode alongside nearest/linear/bicubic/EASU)
//
// Lanczos2 (2-lobe) interpolation for final-post upscaling.

#ifndef TINYGLADE_INC_UPSAMPLE_LANCZOS2_HLSL
#define TINYGLADE_INC_UPSAMPLE_LANCZOS2_HLSL

static const float LANCZOS2_TWO = 2.0f;

// Lanczos2 kernel, |x| < 2.
float lanczos2_kernel(float x)
{
    float ax = abs(x);
    if (ax < 1.0e-6f)
        return 1.0f;
    if (ax >= LANCZOS2_TWO)
        return 0.0f;
    float px = 3.14159265358979323846f * x;
    float px_half = px * 0.5f;
    return (sin(px) / px) * (sin(px_half) / px_half);
}

// Lanczos2 4x4 sample at a given UV.
float3 lanczos2_sample(Texture2D<float4> tex, SamplerState samp,
                       float2 uv, float2 texel_size)
{
    float2 p = uv / texel_size - 0.5f;
    float2 base = floor(p);
    float2 f = p - base;

    float3 result = float3(0.0f, 0.0f, 0.0f);
    float wsum = 0.0f;
    for (int j = -1; j <= 2; j++)
    {
        float wy = lanczos2_kernel(f.y - float(j));
        for (int i = -1; i <= 2; i++)
        {
            float wx = lanczos2_kernel(f.x - float(i));
            float w = wx * wy;
            float2 c = (base + float2(float(i), float(j))) * texel_size;
            result += w * tex.SampleLevel(samp, c, 0.0f).xyz;
            wsum += w;
        }
    }
    return result / max(wsum, 1.0e-6f);
}

#endif // TINYGLADE_INC_UPSAMPLE_LANCZOS2_HLSL
