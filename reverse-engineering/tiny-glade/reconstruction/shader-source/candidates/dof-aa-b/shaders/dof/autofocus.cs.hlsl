// ==SKELETON==
// SOURCE PATH : shaders/dof/autofocus.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_autofocus.cs.hlsl_cs_main_0.bin
//     file_sha256  : dd61ca97962c9cb04e3347f7e354a9da08dbe381d34a1c478e572b752695772c
//     pass/stage   : _dof_autofocus (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W32 (Inference)
//     names        : ViewConstants, frame_constants, $Globals, depth_tex, DofFocus, __alias_autofocus_buf, DofFocusReadback, __alias_focus_readback_buf, group_focus0_dists, group_focus1_dists, cs_main
//
// ==RECONSTRUCTED== shaders/dof/autofocus.cs.hlsl
// Evidence: _dof_autofocus.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 547).
//   bindings: $Globals b13 {depth_tex_dims, point_uv, use_dual_focus,
//   focus_lock_request, focus_is_locked, focus_box_height, use_tilt_shift,
//   aperture_size}, depth_tex t14, __alias_autofocus_buf u15
//   (RWStructuredBuffer<DofFocus>), __alias_focus_readback_buf u16
//   (RWStructuredBuffer<DofFocusReadback>), frame_constants set1/b0.
//   Samples a focus box on the depth texture, bitonic-sorts per-group focus
//   distances, picks near/far focus, applies lock/request smoothing and writes
//   the updated DofFocus back (plus a CPU readback).  Status: O (structure from
//   disassembly; names from debug_names).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/bitonic_sort.hlsl"
#include "inc/coc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims;
    float2 point_uv;
    uint use_dual_focus;
    uint focus_lock_request;
    uint focus_is_locked;
    float focus_box_height;
    uint use_tilt_shift;
    float aperture_size;
};

Texture2D<float4> depth_tex : register(t14, space0);
RWStructuredBuffer<DofFocus> __alias_autofocus_buf : register(u15, space0);
RWStructuredBuffer<DofFocusReadback> __alias_focus_readback_buf : register(u16, space0);

groupshared uint group_focus0_dists[64];
groupshared uint group_focus1_dists[64];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    float2 box_uv_span = (float2(gl_GlobalInvocationID.xy) * 0.14285714924335479736328125f) - 0.5f;
    GroupMemoryBarrierWithGroupSync();

    bool has_history = (frame_index != 0u);
    DofFocus focus = __alias_autofocus_buf[camera_idx];
    float focus0 = clamp(focus.focus0, 0.0f, 100000.0f);
    float focus1 = clamp(focus.focus1, 0.0f, 100000.0f);

    // Focus box aspect (from the vertical FOV).
    float2 box_extent = float2(1.0f / (view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x), 1.0f) * focus_box_height;
    bool dual = (use_dual_focus != 0u);

    // Depth-to-world in the focus box; tilt-shift flattens Y.
    float3 sample_pos;
    if (dual)
    {
        float2 uv = point_uv + ((box_uv_span * box_extent) * 0.2f);
        float4 wp = mul(view_constants[0].view_to_world,
                        mul(view_constants[0].sample_to_view,
                            float4((uv - 0.5f) * float2(2.0f, -2.0f),
                                   depth_tex.Load(int3(uint2(int2(uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        sample_pos = wp.xyz / wp.w;
    }
    else
    {
        float2 uv = point_uv + (box_uv_span * box_extent);
        float4 wp = mul(view_constants[0].view_to_world,
                        mul(view_constants[0].sample_to_view,
                            float4((uv - 0.5f) * float2(2.0f, -2.0f),
                                   depth_tex.Load(int3(uint2(int2(uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        sample_pos = wp.xyz / wp.w;
    }

    float3 dual_pos;
    if (dual)
    {
        float2 offset = box_uv_span * box_extent;
        float4 wp = mul(view_constants[0].view_to_world,
                        mul(view_constants[0].sample_to_view,
                            float4(offset * float2(2.0f, -2.0f),
                                   depth_tex.Load(int3(uint2(int2((0.5f + offset) * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        dual_pos = wp.xyz / wp.w;
    }
    else
    {
        dual_pos = sample_pos;
    }

    bool tilt = (use_tilt_shift != 0u);
    float y_flip = float(tilt ? 0 : 1);
    float3 flat_pos = sample_pos;
    flat_pos.y = sample_pos.y * y_flip;

    float4 origin4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 origin = origin4.xyz / origin4.w;
    float3 forward = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);

    float3 flat_dual = dual_pos;
    flat_dual.y = dual_pos.y * y_flip;
    group_focus0_dists[gl_LocalInvocationIndex] = asuint(dot(flat_pos - origin, forward));
    group_focus1_dists[gl_LocalInvocationIndex] = asuint(dot(flat_dual - origin, forward));
    GroupMemoryBarrierWithGroupSync();

    // Bitonic sort of group_focus0_dists and group_focus1_dists (64 lanes each).
    uint pow2 = 2u << (firstbithigh(63u) & 31u);
    for (uint k = 2u; k <= pow2; k *= 2u)
    {
        uint half_k = k >> 1u;
        for (uint j = half_k; j > 0u; j >>= 1u)
        {
            uint lane = gl_LocalInvocationIndex & (j - 1u);
            uint lo = 2u * (gl_LocalInvocationIndex - lane);
            uint a = lo + lane;
            uint b;
            if (j == half_k)
            {
                b = (lo + ((2u * j) - 1u)) - lane;
            }
            else
            {
                b = (lo + j) + lane;
            }
            bool active = (b < 64u) && (a < 64u);
            if (active)
            {
                if (group_focus0_dists[a] > group_focus0_dists[b])
                {
                    uint tmp = group_focus0_dists[a];
                    group_focus0_dists[a] = group_focus0_dists[b];
                    group_focus0_dists[b] = tmp;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    for (uint k = 2u; k <= pow2; k *= 2u)
    {
        uint half_k = k >> 1u;
        for (uint j = half_k; j > 0u; j >>= 1u)
        {
            uint lane = gl_LocalInvocationIndex & (j - 1u);
            uint lo = 2u * (gl_LocalInvocationIndex - lane);
            uint a = lo + lane;
            uint b;
            if (j == half_k)
            {
                b = (lo + ((2u * j) - 1u)) - lane;
            }
            else
            {
                b = (lo + j) + lane;
            }
            bool active = (b < 64u) && (a < 64u);
            if (active)
            {
                if (group_focus1_dists[a] > group_focus1_dists[b])
                {
                    uint tmp = group_focus1_dists[a];
                    group_focus1_dists[a] = group_focus1_dists[b];
                    group_focus1_dists[b] = tmp;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }

    if (gl_LocalInvocationIndex == 0u)
    {
        float lo0 = asfloat(group_focus0_dists[0]);
        float hi0 = asfloat(group_focus0_dists[63u]);
        float mid0 = asfloat(group_focus0_dists[32u]);
        float mid1 = asfloat(group_focus1_dists[32u]);
        float near_focus;
        float far_focus;
        if (tilt)
        {
            bool not_dual = !dual;
            near_focus = not_dual ? lo0 : mid0;
            far_focus = not_dual ? hi0 : mid1;
        }
        else
        {
            near_focus = mid1;
            far_focus = mid0;
        }

        float new_bias0, new_bias1, new_interp_bias0, new_interp_bias1;
        float3 new_pos;
        if (focus_lock_request == camera_idx)
        {
            float4 wp = mul(view_constants[0].view_to_world,
                            mul(view_constants[0].sample_to_view,
                                float4((point_uv - 0.5f) * float2(2.0f, -2.0f),
                                       depth_tex.Load(int3(uint2(int2(point_uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
            float3 lock_pos = wp.xyz / wp.w;
            float3 flat_lock = lock_pos;
            flat_lock.y = lock_pos.y * y_flip;
            float lock_dist = dot(flat_lock - origin, forward);
            new_bias0 = max(focus0, focus1) - lock_dist;
            new_bias1 = max(near_focus, far_focus) - lock_dist;
            new_interp_bias0 = min(focus0, focus1) - lock_dist;
            new_interp_bias1 = min(near_focus, far_focus) - lock_dist;
            new_pos = lock_pos;
        }
        else
        {
            new_bias0 = focus.interp_lock_bias1;
            new_bias1 = focus.lock_bias0;
            new_interp_bias0 = focus.interp_lock_bias0;
            new_interp_bias1 = focus.lock_bias1;
            new_pos = focus.lock_pos_ws;
        }

        float out_focus0, out_focus1;
        float3 out_pos;
        float out_bias0, out_bias1;
        if (focus_is_locked != 0u)
        {
            float3 interp_pos;
            if (has_history)
            {
                interp_pos = lerp(focus.interp_lock_pos_ws, new_pos, (1.0f - exp(delta_time_seconds * (-8.0f))).xxx);
            }
            else
            {
                interp_pos = new_pos;
            }
            float interp_bias0;
            if (has_history)
            {
                interp_bias0 = lerp(new_interp_bias0, new_interp_bias1, 1.0f - exp(delta_time_seconds * (-8.0f)));
            }
            else
            {
                interp_bias0 = new_interp_bias1;
            }
            float interp_bias1;
            if (has_history)
            {
                interp_bias1 = lerp(new_bias0, new_bias1, 1.0f - exp(delta_time_seconds * (-8.0f)));
            }
            else
            {
                interp_bias1 = new_bias1;
            }
            float3 flat_new = new_pos;
            flat_new.y = new_pos.y * y_flip;
            float new_dist = dot(flat_new - origin, forward);
            out_focus0 = new_dist + interp_bias0;
            out_focus1 = new_dist + interp_bias1;
            out_pos = interp_pos;
            out_bias0 = interp_bias0;
            out_bias1 = interp_bias1;
        }
        else
        {
            if (has_history)
            {
                float blend1 = 1.0f - exp(delta_time_seconds * (-8.0f));
                float blend2 = 1.0f - exp(delta_time_seconds * (-4.0f));
                out_focus0 = 1.0f / max(1e-5f, lerp(1.0f / max(1e-5f, 1.0f / max(1e-5f, lerp(1.0f / max(1e-5f, focus0), 1.0f / max(1e-5f, clamp(focus0, lo0, hi0)), blend1))), 1.0f / max(1e-5f, near_focus), blend2));
                out_focus1 = 1.0f / max(1e-5f, lerp(1.0f / max(1e-5f, 1.0f / max(1e-5f, lerp(1.0f / max(1e-5f, focus1), 1.0f / max(1e-5f, clamp(focus1, asfloat(group_focus1_dists[0]), asfloat(group_focus1_dists[63u]))), blend1))), 1.0f / max(1e-5f, far_focus), blend2));
            }
            else
            {
                out_focus0 = near_focus;
                out_focus1 = far_focus;
            }
            out_pos = focus.interp_lock_pos_ws;
            out_bias0 = new_interp_bias0;
            out_bias1 = new_bias0;
        }

        DofFocus updated = { out_focus0, out_focus1, new_interp_bias1, new_bias1, new_pos, focus_is_locked, out_pos, use_tilt_shift, out_bias0, out_bias1, aperture_size, focus.pad2 };
        __alias_autofocus_buf[camera_idx] = updated;

        DofFocusReadback readback = { new_pos, focus_lock_request };
        __alias_focus_readback_buf[0] = readback;
    }
}
