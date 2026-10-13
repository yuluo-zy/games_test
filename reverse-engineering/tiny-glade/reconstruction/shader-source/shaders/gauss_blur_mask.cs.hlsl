// ==RECONSTRUCTED== shaders/gauss_blur_mask.cs.hlsl | F-D
// Evidence: _gauss_blur_mask.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (water_mask_dims/out_water_mask_dims/is_horizontal/sigma/
//   output_channel), water_mask t14, out_water_mask u15, LocalSize [8,8,1], entry cs_main.
//   spirv-cross: separable gaussian, radius=ceil(sigma*2); in-bounds taps accumulate
//   (.x channel if horizontal else .y); writes mean into out_water_mask .y if
//   output_channel==1 else .z if output_channel==2.
// Status: O (structure from disassembly; member names verbatim).

#include "inc/common.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 water_mask_dims;
    uint2 out_water_mask_dims;
    uint is_horizontal;
    float sigma;
    int output_channel;
};

Texture2D<float4> water_mask : register(t14, space0);
RWTexture2D<float4> out_water_mask : register(u15, space0);

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

        if (all(upixel < water_mask_dims))
        {
            float t = float(offset);
            float weight = exp((-t * t) / ((2.0f * sigma) * sigma));
            weighted_sum += (horizontal ? water_mask.Load(int3(upixel, 0)).x : water_mask.Load(int3(upixel, 0)).y) * weight;
            weight_sum += weight;
        }
    }

    float mean = weighted_sum / weight_sum;
    if (output_channel == 1)
    {
        out_water_mask[dispatchThreadID.xy].y = mean;
    }
    else if (output_channel == 2)
    {
        out_water_mask[dispatchThreadID.xy].z = mean;
    }
}
