// ==RECONSTRUCTED== shaders/ssgi/upsample.cs.hlsl | F-D
// Evidence: _ssgi_upsample.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23; SHA-256
//   2a466f7f8b21c2cedf861db078351469a1dc11eebc14e29de06359d8ecc077d6).
//   bindings: ssgi_tex t14, depth_tex t15, half_depth_tex t16,
//   half_view_normal_tex t17, gbuffer_tex t18, output_tex u19,
//   frame_constants b0 space1; LocalSize [8,8,1], entry cs_main.
//   Bilateral upsample of the half-res SSGI AO to full res: 5 half-res taps
//   (center + 4-neighborhood) weighted by a view-normal cosine^8 term, an
//   exponential depth match, and a gaussian falloff of the tap offset; falls
//   back to the nearest half-res texel when no tap contributes.
// Status: O (structure/constants from disassembly; bindings verbatim);
//         I (function/var names — inlined by dxc).

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/uv.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"

Texture2D<float4> ssgi_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
Texture2D<float4> half_depth_tex : register(t16, space0);
Texture2D<float4> half_view_normal_tex : register(t17, space0);
Texture2D<uint4> gbuffer_tex : register(t18, space0);
RWTexture2D<half> output_tex : register(u19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    int2 samples[5] = { int2(0, 0), int2(-1, 0), int2(1, 0), int2(0, -1), int2(0, 1) };

    float center_depth = depth_tex.Load(uint3(dispatchThreadID.xy, 0u)).x;

    float4 weighted_sum = 0.0f.xxxx;
    float total_weight = 0.0f;

    if (center_depth != 0.0f)
    {
        float3 center_normal = mul(view_constants[0].world_to_view, float4(unpack_normal_rgb8(gbuffer_tex.Load(uint3(dispatchThreadID.xy, 0u)).x), 0.0f)).xyz;

        for (uint i = 0u; i < 5u; i++)
        {
            uint2 half_res_pos = uint2(int2((dispatchThreadID.xy / uint2(2u, 2u)) + uint2(samples[i])));
            float half_depth = half_depth_tex.Load(uint3(half_res_pos, 0u)).x;
            float4 sample_result = 0.0f.xxxx;
            do
            {
                if (half_depth != 0.0f)
                {
                    float ndv = max(0.0f, dot(half_view_normal_tex.Load(uint3(half_res_pos, 0u)).xyz, center_normal));
                    float n2 = ndv * ndv;
                    float n4 = n2 * n2;
                    float n8 = n4 * n4;
                    float2 offset2 = float2(samples[i]);
                    float weight = exp2((-200.0f) * abs(1.0f - (center_depth / half_depth))) * n8 * exp(-dot(offset2, offset2));
                    total_weight += weight;
                    sample_result = ssgi_tex.Load(uint3(half_res_pos, 0u)) * weight;
                    break;
                }
                else
                {
                    sample_result = 0.0f.xxxx;
                    break;
                }
            } while (false);
            weighted_sum += sample_result;
        }
    }

    if (total_weight > 1.0e-7f)
    {
        output_tex[dispatchThreadID.xy] = (weighted_sum.x / total_weight);
    }
    else
    {
        output_tex[dispatchThreadID.xy] = ssgi_tex.Load(uint3(dispatchThreadID.xy / uint2(2u, 2u), 0u)).x;
    }
}
