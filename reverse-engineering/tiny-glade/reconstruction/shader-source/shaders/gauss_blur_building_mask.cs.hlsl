// ==RECONSTRUCTED== shaders/gauss_blur_building_mask.cs.hlsl | F-D
// Evidence: _gauss_blur_building_mask.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (building_mask_dims/out_building_mask_dims/is_horizontal),
//   building_mask t14, out_building_mask u15 (float2), LocalSize [8,8,1], entry cs_main.
//   spirv-cross: separable gaussian with fixed radius 20 (ceil(20.0f)), weight
//   exp(-t^2 * 0.005) (i.e. sigma=10), in-bounds taps only, output .x = mean
//   preserving existing .y.
// Status: O (structure from disassembly; member names verbatim). building_mask.inc.hlsl
//   is included (embedded in the bin) but contributes no codegen here (skeleton).

#include "inc/common.hlsl"
#include "buildings/building_mask.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 building_mask_dims;
    uint2 out_building_mask_dims;
    uint is_horizontal;
};

Texture2D<float4> building_mask : register(t14, space0);
RWTexture2D<float2> out_building_mask : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int radius = int(ceil(20.0f));

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

        if (all(upixel < building_mask_dims))
        {
            float t = float(offset);
            float weight = exp((-t * t) * 0.004999999888241291046142578125f);
            weighted_sum += building_mask.Load(int3(upixel, 0)).x * weight;
            weight_sum += weight;
        }
    }

    out_building_mask[uint2(int3(dispatchThreadID).xy)] = float2(weighted_sum / weight_sum, out_building_mask[uint2(int3(dispatchThreadID).xy)].y);
}
