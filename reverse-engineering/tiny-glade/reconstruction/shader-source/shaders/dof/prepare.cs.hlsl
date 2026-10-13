// ==RECONSTRUCTED== shaders/dof/prepare.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (read-only StructuredBuffer<DofFocus>, member-wise reads, no OpMemberDecorate drift) + shared format fix: coc_out_tex RWTexture2D<half> (R16f), tile_output_tex RWTexture2D<half2> (Rg16f).
//   Computes the per-pixel signed circle-of-confusion from focus depth and writes the tile
//   min/max abs-CoC used by the analyze pass.

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

StructuredBuffer<DofFocus> autofocus_buf : register(t15, space0);
Texture2D<float4> depth_tex : register(t14, space0);
RWTexture2D<half> coc_out_tex : register(u16, space0);
RWTexture2D<half2> tile_output_tex : register(u17, space0);

groupshared uint min_abs_coc_asuint;
groupshared uint max_abs_coc_asuint;

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    if (local_index == 0u)
    {
        min_abs_coc_asuint = 1343554297u;
        max_abs_coc_asuint = 0u;
    }
    GroupMemoryBarrierWithGroupSync();

    float4 view_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
        float4((((float2(int2(dispatch_id.xy)) + 0.5f) / float2(coc_out_tex_dims)) - 0.5f) * float2(2.0f, -2.0f),
               depth_tex.Load(int3(dispatch_id.xy, 0)).x, 1.0f)));
    float3 view_pos = view_world.xyz / view_world.w;

    float focus_scale = autofocus_buf[camera_idx].focus_scale;
    float inv_focus0 = 1.0f / autofocus_buf[camera_idx].focus0;
    float inv_focus1 = 1.0f / autofocus_buf[camera_idx].focus1;
    float focus_max = max(0.001f, max(inv_focus0, inv_focus1)) + 9.9999997473787516e-06f;
    float focus_min = max(0.001f, min(inv_focus0, inv_focus1)) - 9.9999997473787516e-06f;
    bool use_tilt_shift = autofocus_buf[camera_idx].use_tilt_shift != 0u;
    view_pos.y *= (use_tilt_shift ? 0.0f : 1.0f);

    float4 cam_h = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w;
    float3 cam_dir = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float inv_dist = 1.0f / max(1.000000013351432e-10f, dot(view_pos - cam_pos, cam_dir));
    float focus_range = focus_max - focus_min;
    float epsilon = min(0.01f, 0.05f * focus_range);

    float signed_dist;
    if (inv_dist > focus_max)
    {
        signed_dist = (inv_dist - focus_max) + epsilon;
    }
    else if (inv_dist < focus_min)
    {
        signed_dist = (inv_dist - focus_min) - epsilon;
    }
    else
    {
        signed_dist = -epsilon + ((2.0f * epsilon) * (inv_dist - focus_min)) / focus_range;
    }

    float coc_dir_dist;
    if (use_tilt_shift)
    {
        coc_dir_dist = signed_dist / max(0.01f, length(cam_dir.xz));
    }
    else
    {
        coc_dir_dist = signed_dist;
    }

    float coc_signed = (float(depth_tex_dims.y) * 0.0013888889225199819f)
        * ((-coc_dir_dist) * focus_scale * view_constants[0].view_to_clip[1].y);
    uint coc_abs = asuint(min(float(depth_tex_dims.y) * 0.041666667908430099f, abs(coc_signed)));

    InterlockedMin(min_abs_coc_asuint, coc_abs);
    InterlockedMax(max_abs_coc_asuint, coc_abs);

    GroupMemoryBarrierWithGroupSync();
    if (local_index == 0u)
    {
        tile_output_tex[dispatch_id.xy / uint2(8u, 8u)] =
            float2(asfloat(min_abs_coc_asuint), asfloat(max_abs_coc_asuint));
    }

    coc_out_tex[dispatch_id.xy] = coc_signed;
}
