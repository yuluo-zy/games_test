// ==SKELETON==
// SOURCE PATH : shaders/dof/prepare.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_prepare.cs.hlsl_cs_main_0.bin
//     file_sha256  : ca491a564b30bfe3c844c8ae34e183e545c781cd9238c5df980f2b1b5d5a95a9
//     pass/stage   : _dof_prepare (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W49 (Inference)
//     names        : ViewConstants, frame_constants, $Globals, depth_tex, DofFocus, autofocus_buf, coc_out_tex, tile_output_tex, min_abs_coc_asuint, max_abs_coc_asuint, cs_main
//
// ==RECONSTRUCTED== shaders/dof/prepare.cs.hlsl
// Evidence: _dof_prepare.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 218).
//   bindings: depth_tex t14, autofocus_buf t15 (RWStructuredBuffer<DofFocus>),
//   coc_out_tex u16 (RWTexture2D<float> R16f), tile_output_tex u17
//   (RWTexture2D<float2>), $Globals b13 {depth_tex_dims, coc_out_tex_dims,
//   tile_output_tex_dims}, frame_constants set1/b0; LocalSize [8,8,1], entry cs_main.
//   Per-pixel signed CoC: reconstruct world pos from depth, apply tilt-shift
//   flatten + focus-distance piecewise smoothing, scale by focus_scale and the
//   vertical FoC, write coc_out_tex; per-group min/max |CoC| atoms land in
//   tile_output_tex. Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    uint2 coc_out_tex_dims;
    uint2 tile_output_tex_dims;
};

Texture2D<float4> depth_tex : register(t14, space0);
RWStructuredBuffer<DofFocus> autofocus_buf : register(u15, space0);
RWTexture2D<float> coc_out_tex : register(u16, space0);
RWTexture2D<float2> tile_output_tex : register(u17, space0);

groupshared uint min_abs_coc_asuint;
groupshared uint max_abs_coc_asuint;

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    bool is_first = (gl_LocalInvocationIndex == 0u);
    if (is_first)
    {
        min_abs_coc_asuint = 1343554297u; // asuint(50.0)
        max_abs_coc_asuint = 0u;
    }
    GroupMemoryBarrierWithGroupSync();

    // Reconstruct world position from the depth texture.
    float4 world_pos = mul(view_constants[0].view_to_world,
                           mul(view_constants[0].sample_to_view,
                               float4((((float2(int2(gl_GlobalInvocationID.xy)) + 0.5f)
                                   / float2(coc_out_tex_dims)) - 0.5f) * float2(2.0f, -2.0f),
                                      depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x, 1.0f)));
    float3 view_pos = world_pos.xyz / world_pos.w;

    DofFocus focus = autofocus_buf[camera_idx];
    float inv_focus0 = 1.0f / focus.focus0;
    float inv_focus1 = 1.0f / focus.focus1;
    float focal_low = max(0.001f, min(inv_focus0, inv_focus1)) - 1e-5f;
    float focal_high = max(0.001f, max(inv_focus0, inv_focus1)) + 1e-5f;

    bool use_tilt_shift = (focus.use_tilt_shift != 0u);
    view_pos.y = view_pos.y * float(use_tilt_shift ? 0 : 1);

    // View origin and forward axis.
    float4 view_origin4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 view_origin = view_origin4.xyz / view_origin4.w;
    float3 view_forward = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float inv_depth = 1.0f / max(1e-10f, dot(view_pos - view_origin, view_forward));

    // Piecewise-smooth focus-distance remap.
    float focal_range = focal_high - focal_low;
    float soft = min(0.01f, 0.05f * focal_range);
    float focus_err;
    if (inv_depth > focal_high)
    {
        focus_err = (inv_depth - focal_high) + soft;
    }
    else
    {
        if (inv_depth < focal_low)
        {
            focus_err = (inv_depth - focal_low) - soft;
        }
        else
        {
            focus_err = (-soft) + (((2.0f * soft) * (inv_depth - focal_low)) / focal_range);
        }
    }

    float coc_norm;
    if (use_tilt_shift)
    {
        coc_norm = focus_err / max(0.01f, length(view_forward.xz));
    }
    else
    {
        coc_norm = focus_err;
    }

    float coc = ((float(depth_tex_dims.y) * 0.001388888922519981861114501953125f)
        * ((-coc_norm) * focus.focus_scale * view_constants[0].view_to_clip[1].y));

    uint abs_coc_bits = asuint(min(float(depth_tex_dims.y) * 0.0416666679084300994873046875f, abs(coc)));
    uint prev_min;
    InterlockedMin(min_abs_coc_asuint, abs_coc_bits, prev_min);
    uint prev_max;
    InterlockedMax(max_abs_coc_asuint, abs_coc_bits, prev_max);

    GroupMemoryBarrierWithGroupSync();
    if (is_first)
    {
        tile_output_tex[gl_GlobalInvocationID.xy / uint2(8u, 8u)] = float2(asfloat(min_abs_coc_asuint), asfloat(max_abs_coc_asuint));
    }
    coc_out_tex[gl_GlobalInvocationID.xy] = coc;
}
