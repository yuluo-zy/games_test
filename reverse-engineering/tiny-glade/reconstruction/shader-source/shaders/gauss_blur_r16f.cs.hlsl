// ==RECONSTRUCTED== shaders/gauss_blur_r16f.cs.hlsl | F-D
// Evidence: _gauss_blur_r16f.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (img_in_dims/img_out_dims/is_horizontal/sigma), img_in t14,
//   img_out u15 (R16F scalar), LocalSize [8,8,1], entry cs_main.
//   spirv-cross: separable gaussian (scalar), radius=ceil(sigma*2), weight
//   exp(-t^2/(2 sigma^2)); NO out-of-range masking (Load always in-bounds).
//   img_out = weighted_sum / weight_sum.
// Status: O (structure from disassembly; member names verbatim).

#include "inc/common.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims;
    uint2 img_out_dims;
    uint is_horizontal;
    float sigma;
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int radius = int(ceil(sigma * 2.0f));

    float weighted_sum = 0.0f;
    float weight_sum = 0.0f;

    bool horizontal = is_horizontal != 0u;
    for (int offset = -radius; offset <= radius; ++offset)
    {
        int2 pixel;
        if (!horizontal)
        {
            pixel = int3(dispatchThreadID).xy + int2(0, offset);
        }
        else
        {
            pixel = int3(dispatchThreadID).xy + int2(offset, 0);
        }
        uint2 upixel = uint2(pixel);

        float t = float(offset);
        float weight = exp((-t * t) / ((2.0f * sigma) * sigma));
        weighted_sum += img_in.Load(int3(upixel, 0)).x * weight;
        weight_sum += weight;
    }

    img_out[uint2(int3(dispatchThreadID).xy)] = weighted_sum / weight_sum;
}
