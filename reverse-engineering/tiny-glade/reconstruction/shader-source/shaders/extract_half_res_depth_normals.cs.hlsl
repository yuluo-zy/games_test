// ==RECONSTRUCTED== shaders/extract_half_res_depth_normals.cs.hlsl | F-D
// Evidence: _extract_half_res_depth_normals.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: stencil_tex t14 (uint4), depth_input_tex t15, gbuffer_input_tex t16
//   (uint4), depth_output_tex u17, normals_output_tex u18 (snorm float4),
//   frame_constants set1/b0, LocalSize [8,8,1], entry cs_main.
//   spirv-cross: picks the frame-jitter subpixel (frame_index & 3) of the 2x2 half-res
//   block, reads depth (sets/clears LSB per stencil water bits 1|2), writes depth;
//   unpacks the RGB8 gbuffer normal to view space (normalized) and writes it with a
//   quadrant sign flag.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/halfres.hlsl"

Texture2D<uint4> stencil_tex : register(t14, space0);
Texture2D<float4> depth_input_tex : register(t15, space0);
Texture2D<uint4> gbuffer_input_tex : register(t16, space0);
RWTexture2D<float> depth_output_tex : register(u17, space0);
RWTexture2D<snorm float4> normals_output_tex : register(u18, space0);

static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    uint2 sub = hi_px_subpixels[frame_index & 3u];
    uint2 px = uint2(int2(uint2(int3(dispatchThreadID).xy * int2(2, 2)) + sub));

    float depth = depth_input_tex.Load(int3(px, 0)).x;
    uint stencil = stencil_tex.Load(int3(px, 0)).x & 7u;
    bool is_water;
    if (!(1u == stencil))
    {
        is_water = 2u == stencil;
    }
    else
    {
        is_water = true;
    }
    float out_depth = is_water ? asfloat(asuint(depth) | 1u) : asfloat(asuint(depth) & 4294967294u);
    depth_output_tex[dispatchThreadID.xy] = out_depth;

    uint packed = gbuffer_input_tex.Load(int3(px, 0)).x;
    float pad;
    float3 view_normal = normalize(mul(view_constants[0].world_to_view, float4((float4(float(packed & 255u) * 0.0039215688593685626983642578125f, float((packed >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((packed >> 16u) & 255u) * 0.0039215688593685626983642578125f, pad).xyz * 2.0f) - 1.0f, 0.0f)).xyz);
    float sign_flag = (hi_px_subpixels[frame_index & 3u].x == 1u ? 1.0f : 0.5f) * (hi_px_subpixels[frame_index & 3u].y == 1u ? 1.0f : (-1.0f));
    normals_output_tex[dispatchThreadID.xy] = float4(view_normal, sign_flag);
}
