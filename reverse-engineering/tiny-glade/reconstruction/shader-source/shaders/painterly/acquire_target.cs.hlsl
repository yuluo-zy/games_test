// ==RECONSTRUCTED== shaders/painterly/acquire_target.cs.hlsl | evidence: _painterly_acquire_target.cs.hlsl_cs_main_0.bin — 7x7 depth-aware box gather (weights {20,15,6,1} per axis, exp2(-min(30, 50*lerp(0,|depth_ratio-1|,smoothstep(.3,.5,uv.x))))), coc_tex alpha pass-through, depth_output copy; LocalSize 8,8,1; caps Shader+RDA+StorageImageExtendedFormats (R16f depth_output via RWTexture2D<half>) | status: O (gather math + resources from inlined SPIR-V); I (loop bound -3..3, gaussian_weights name from embedded OpName)
//
// Painterly thumbnail chain: resolves the multisampled frame color into a
// non-MS target with a depth-aware edge-preserving box blur, writing the
// circle-of-confusion to alpha and the center depth to a separate R16f target.

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b16, space0)
{
    uint2 color_tex_dims;
    uint2 output_tex_dims;
    uint2 depth_output_tex_dims;
};

Texture2D<float4> coc_tex : register(t13, space0);
Texture2DMS<float4> color_tex : register(t14, space0);
Texture2D<float4> depth_tex : register(t15, space0);
RWTexture2D<unorm float4> output_tex : register(u17, space0);
RWTexture2D<half> depth_output_tex : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    static const float gaussian_weights[4] = { 20.0f, 15.0f, 6.0f, 1.0f };

    float2 uv = (float2(int2(dispatch_thread_id.xy)) + 0.5f.xx) / float2(output_tex_dims);
    float2 color_dims = float2(color_tex_dims);

    float center_depth = depth_tex.Load(int3(uint2(int2(uv * color_dims)), 0u)).x;

    float3 color_acc = 0.0f.xxx;
    float weight_acc = 0.0f;
    for (int j = -3; j <= 3; j++)
    {
        for (int i = -3; i <= 3; i++)
        {
            int2 coord = int2((uv + (float2(float(i), float(j)) / float2(output_tex_dims))) * color_dims);
            float depth = depth_tex.Load(int3(uint2(coord), 0u)).x;
            float edge = smoothstep(0.300000011920928955078125f, 0.5f, uv.x);
            float weight = (gaussian_weights[abs(i)] * gaussian_weights[abs(j)]) *
                           exp2(-min(30.0f, 50.0f * lerp(0.0f, abs((max(1.0e-20f, center_depth) / max(1.0e-20f, depth)) - 1.0f), edge)));
            weight_acc += weight;
            color_acc += color_tex.Load(coord, 0).xyz * weight;
        }
    }

    float coc = (clamp(coc_tex.SampleLevel(sampler_lnc, uv, 0.0f).x * 0.0333333350718021392822265625f, -1.0f, 1.0f) * 0.5f) + 0.5f;
    output_tex[dispatch_thread_id.xy] = float4(color_acc / weight_acc.xxx, coc);
    depth_output_tex[dispatch_thread_id.xy] = center_depth;
}
