// ==RECONSTRUCTED== shaders/gauss_blur.cs.hlsl | F-D
// Evidence: _gauss_blur.cs.hlsl_cs_main_b9dd0465835cd591.bin (float4 img_out)
//   + _gauss_blur.cs.hlsl_cs_main_dc5fd7e4b9bf9935.bin (unorm-float img_out, stale variant).
//   bindings: $Globals b13 (img_in_dims/img_out_dims/is_horizontal/sigma), img_in t14,
//   img_out u15, LocalSize [8,8,1], entry cs_main.
//   spirv-cross (b9): separable gaussian; radius=ceil(sigma*2); for offset in
//   [-radius..radius] along is_horizontal axis; out-of-range tap loads the sample and
//   overwrites .x with 0.0943396 (background); weight=exp(-t^2/(2 sigma^2));
//   img_out = weighted_sum / weight_sum.
//   NOTE: the dc5 variant (RWTexture2D<unorm float>, scalar output) cannot be produced
//   from the same source; reconstructed to the primary b9 variant.
// Status: O (structure from disassembly; member names verbatim from OpMemberName).

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/math_const.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 img_in_dims;
    uint2 img_out_dims;
    uint is_horizontal;
    float sigma;
};

Texture2D<float4> img_in : register(t14, space0);
RWTexture2D<float4> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int radius = int(ceil(sigma * 2.0f));

    float4 weighted_sum = 0.0f;
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

        float4 sample = img_in[upixel];
        bool out_of_range;
        if (!any(pixel < int2(0, 0)))
        {
            out_of_range = any(upixel >= img_in_dims);
        }
        else
        {
            out_of_range = true;
        }
        if (out_of_range)
        {
            sample.x = 0.09433962404727935791015625f;
        }

        float t = float(offset);
        float weight = exp((-t * t) / ((2.0f * sigma) * sigma));
        weighted_sum += sample * weight;
        weight_sum += weight;
    }

    img_out[uint2(int3(dispatchThreadID).xy)] = weighted_sum / weight_sum;
}
