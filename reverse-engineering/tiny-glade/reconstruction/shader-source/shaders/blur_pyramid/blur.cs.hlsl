// ==RECONSTRUCTED== shaders/blur_pyramid/blur.cs.hlsl | F-D
// Evidence: _blur_pyramid_blur.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b15 (input_mip_dims), input_tex t13, output_tex u14 (float3),
//   groupshared vblur_out[136], LocalSize [64,1,1], entry cs_main.
//   spirv-cross: separable blur over a 128x2 tile per group; vertical pass (9 taps)
//   with gaussian coeff 0.16 and in-dims mask, group barrier, horizontal pass (9 taps),
//   output = (sum/weight_sum).xyz. half-res mip dims = max(1, dims/2).
// Status: O (structure from disassembly; member name verbatim).

#include "inc/common.hlsl"

cbuffer $Globals : register(b15, space0)
{
    uint2 input_mip_dims;
};

Texture2D<float4> input_tex : register(t13, space0);
RWTexture2D<float3> output_tex : register(u14, space0);

groupshared float4 vblur_out[136];

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID,
             uint3 groupThreadID : SV_GroupThreadID,
             uint3 groupID : SV_GroupID)
{
    int2 gid_i = int2(dispatchThreadID.xy);

    for (uint i = groupThreadID.x; i < 136u; i += 64u)
    {
        int2 tile = int2(groupID.xy * uint2(128u, 2u) + uint2(int(i - 4u), -4));

        float weight_sum = 0.0f;
        float4 sum = 0.0f;
        for (uint j = 0u; j <= 8u; ++j)
        {
            uint tile_y = uint(tile.y) + j;
            float half_y = float(max(1u, input_mip_dims.y / 2u));
            float d = ((float(gid_i.y) + 0.5f) * float(input_mip_dims.y) - (float(tile_y) + 0.5f) * half_y) / half_y;
            float w = exp(-d * d * 0.1599999964237213134765625f) * float(tile_y < input_mip_dims.y);
            weight_sum += w;
            sum += input_tex.Load(int3(tile + int2(0, int(j)), 0)) * w;
        }
        vblur_out[i] = sum / weight_sum;
    }

    GroupMemoryBarrierWithGroupSync();

    float weight_sum = 0.0f;
    float4 sum = 0.0f;
    for (uint k = 0u; k <= 8u; ++k)
    {
        uint tile_x = dispatchThreadID.x * 2u + k - 4u;
        float half_x = float(max(1u, input_mip_dims.x / 2u));
        float d = ((float(dispatchThreadID.x) + 0.5f) * float(input_mip_dims.x) - (float(tile_x) + 0.5f) * half_x) / half_x;
        float w = exp(-d * d * 0.1599999964237213134765625f) * float(tile_x < input_mip_dims.x);
        sum += vblur_out[groupThreadID.x * 2u + k] * w;
        weight_sum += w;
    }

    output_tex[dispatchThreadID.xy] = (sum / weight_sum).xyz;
}
