// ==RECONSTRUCTED== shaders/gi/blur_screen_radiance.cs.hlsl (candidate gi-deferred-b)
// Evidence: _gi_blur_screen_radiance.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: $Globals b13 {half_depth_tex_dims, half_view_normal_tex_dims, ssgi_tex_dims,
//     input_tex_dims, output_tex_dims, temporal_stats_out_tex_dims}, half_depth_tex t14,
//     half_view_normal_tex t15, ssgi_tex t16, input_tex t17, output_tex u18,
//     temporal_stats_out_tex u19, frame_constants set1/b0. LocalSize [8,8,1], entry cs_main.
//   spirv-cross: 3x3 half-res bilateral blur of the 3-slab screen radiance (input_tex),
//     weighted by normal similarity, depth delta and ssgi difference; step is 1 or 3
//     texels depending on the temporal stats byte (>33). The three slabs are iterated
//     with runtime-indexed loops (dims.x is a $Globals value, so dxc keeps them as loops).
// Status: O (structure from disassembly); I (hi_px_subpixels local name).

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/hash.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/color/srgb.hlsl"
#include "restir.inc.hlsl"
#include "gi_settings.inc.hlsl"

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

static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;

    float depth = half_depth_tex.Load(int3(xy, 0u)).x;
    float normal_z = max(0.300000011920928955078125f, half_view_normal_tex.Load(int3(xy, 0u)).z);
    uint sub = frame_index & 3u;
    float ssgi = ssgi_tex.Load(int3((xy * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x;
    float3 n = half_view_normal_tex.Load(int3(xy, 0u)).xyz;
    float step = ((temporal_stats_out_tex[xy] * float4(255.0f, 1.0f, 1.0f, 1.0f)).x > 33.0f) ? 1.0f : 3.0f;

    float4 acc[3];
    acc[0] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[1] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    acc[2] = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float total = 0.0f;

    for (int dy = -1; dy <= 1; ++dy) {
        for (int dx = -1; dx <= 1; ++dx) {
            int2 d = int2(dx, dy);
            uint2 off = xy + uint2(int2(float2(d) * step));
            float ndot = dot(n, half_view_normal_tex.Load(int3(off, 0u)).xyz);
            float w = exp2(-min(30.0f, 80.0f * (normal_z * abs(1.0f - (half_depth_tex.Load(int3(off, 0u)).x / depth)))))
                    * exp2((-20.0f) * abs(ssgi_tex.Load(int3((off * uint2(2u, 2u)) + hi_px_subpixels[sub], 0u)).x - ssgi))
                    * (0.010000000707805156707763671875f / ((1.0f - 0.89999997615814208984375f * ndot * ndot) * (1.0f - 0.89999997615814208984375f * ndot * ndot)));
            for (uint c = 0u; c < 3u; ++c) {
                acc[c] += input_tex.Load(int3(off + uint2(c * half_depth_tex_dims.x, 0u), 0u)) * w;
            }
            total += w;
        }
    }

    for (uint c = 0u; c < 3u; ++c) {
        acc[c] /= max(1.0000000133514319600180897396058e-10f, total).xxxx;
    }

    for (uint c = 0u; c < 3u; ++c) {
        output_tex[xy + uint2(c * half_depth_tex_dims.x, 0u)] = acc[c];
    }
}
