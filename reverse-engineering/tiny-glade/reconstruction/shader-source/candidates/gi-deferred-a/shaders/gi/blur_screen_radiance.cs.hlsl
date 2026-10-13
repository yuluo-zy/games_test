// ==RECONSTRUCTED== shaders/gi/blur_screen_radiance.cs.hlsl | status: O (structure from disassembly)
// Evidence: _gi_blur_screen_radiance.cs.hlsl_cs_main_0.bin (LocalSize 8x8, entry cs_main).
//   bindings: $Globals b13 (6 dims), half_depth_tex t14, half_view_normal_tex t15,
//   ssgi_tex t16, input_tex t17, output_tex u18, temporal_stats_out_tex u19,
//   frame_constants set1/b0; caps {Shader, RuntimeDescriptorArray}.
//   Adaptive 3x3 cross-bilateral blur of the 3 SH radiance texels: samples weighted by
//   depth error, normal similarity, SSGI agreement and the temporal-stats confidence
//   (stride 1 when the texel has stabilized, stride 3 otherwise).
//   Include residual: original closure ~18 files vs candidate ~14 (see decisions.md).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/reservoir.hlsl"
#include "gi_settings.inc.hlsl"
#include "restir.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 ssgi_tex_dims;
    uint2 input_tex_dims;
    uint2 output_tex_dims;
    uint2 temporal_stats_out_tex_dims;
};

Texture2D<float4> half_depth_tex : register(t14, space0);
Texture2D<float4> half_view_normal_tex : register(t15, space0);
Texture2D<float4> ssgi_tex : register(t16, space0);
Texture2D<float4> input_tex : register(t17, space0);
RWTexture2D<float4> output_tex : register(u18, space0);
RWTexture2D<unorm float4> temporal_stats_out_tex : register(u19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;

    float depth = half_depth_tex.Load(int3(px, 0)).x;
    float normal_z = max(0.3f, half_view_normal_tex.Load(int3(px, 0)).z);
    float3 view_normal = half_view_normal_tex.Load(int3(px, 0)).xyz;
    uint subp = frame_index & 3u;
    float ssgi = ssgi_tex.Load(int3((px * 2u) + hi_px_subpixels[subp], 0)).x;

    // Temporal-stats confidence: once the texel has accumulated enough frames
    // (count > 33) use a tight stride, otherwise a wide one.
    float stride = ((temporal_stats_out_tex[px] * float4(255.0f, 1.0f, 1.0f, 1.0f)).x > 33.0f) ? 1.0f : 3.0f;

    float4 acc[3] = { float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f), float4(0.0f, 0.0f, 0.0f, 0.0f) };
    float weight_sum = 0.0f;

    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            uint2 tap = px + uint2(int2(dx, dy) * int(stride));

            float ndot = dot(view_normal, half_view_normal_tex.Load(int3(tap, 0)).xyz);
            float nw = (ndot * ndot) * (-0.9f) + 1.0f;
            float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(tap, 0)).x / depth))))) *
                      exp2((-20.0f) * abs(ssgi_tex.Load(int3((tap * 2u) + hi_px_subpixels[subp], 0)).x - ssgi)) *
                      (0.01f / (nw * nw));

            [unroll]
            for (uint c = 0u; c < 3u; ++c) {
                acc[c] += input_tex.Load(int3(tap + uint2(c * half_depth_tex_dims.x, 0u), 0)) * w;
            }
            weight_sum += w;
        }
    }

    [unroll]
    for (uint c = 0u; c < 3u; ++c) {
        acc[c] /= max(1e-10f, weight_sum);
    }
    [unroll]
    for (uint c = 0u; c < 3u; ++c) {
        output_tex[px + uint2(c * half_depth_tex_dims.x, 0u)] = acc[c];
    }
}
