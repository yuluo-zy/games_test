// ==RECONSTRUCTED== shaders/dof/autofocus.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (tilt-shift focus0=min/median0, focus1=max/median1; locked focus1 = lock_dist + lerped(focus0); B swapped). readback member renamed to lock_reponse (inc fix #2).
//   Autofocus: samples 64 depth points in the focus box, bitonic-sorts the view-space focus
//   distances per focus plane, and blends focus0/focus1/bias toward the locked target (or the
//   scene median) with exponential smoothing, writing DofFocus back.

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

RWStructuredBuffer<DofFocus> __alias_autofocus_buf : register(u15, space0);
RWStructuredBuffer<DofFocusReadback> __alias_focus_readback_buf : register(u16, space0);
Texture2D<float4> depth_tex : register(t14, space0);

groupshared uint group_focus0_dists[64];
groupshared uint group_focus1_dists[64];

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    float2 local_uv = (float2(dispatch_id.xy) * 0.14285714924335479736f) - 0.5f;
    GroupMemoryBarrierWithGroupSync();
    bool not_first_frame = (frame_index != 0u);

    float focus0 = __alias_autofocus_buf[camera_idx].focus0;
    float focus1 = __alias_autofocus_buf[camera_idx].focus1;
    float lock_bias0 = __alias_autofocus_buf[camera_idx].lock_bias0;
    float lock_bias1 = __alias_autofocus_buf[camera_idx].lock_bias1;
    float3 lock_pos_ws = __alias_autofocus_buf[camera_idx].lock_pos_ws;
    float3 interp_lock_pos_ws = __alias_autofocus_buf[camera_idx].interp_lock_pos_ws;
    float interp_lock_bias0 = __alias_autofocus_buf[camera_idx].interp_lock_bias0;
    float interp_lock_bias1 = __alias_autofocus_buf[camera_idx].interp_lock_bias1;
    uint pad2 = __alias_autofocus_buf[camera_idx].pad2;

    float clamped_focus0 = clamp(focus0, 0.0f, 100000.0f);
    float clamped_focus1 = clamp(focus1, 0.0f, 100000.0f);

    float2 focus_box_extent = float2(1.0f / (view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x), 1.0f)
        * focus_box_height;
    bool dual_focus = use_dual_focus != 0u;

    float3 focus0_point_ws;
    if (dual_focus)
    {
        float2 focus0_uv = point_uv + ((local_uv * focus_box_extent) * 0.2f);
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4((focus0_uv - 0.5f) * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus0_point_ws = w.xyz / w.w;
    }
    else
    {
        float2 focus0_uv = point_uv + (local_uv * focus_box_extent);
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4((focus0_uv - 0.5f) * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus0_point_ws = w.xyz / w.w;
    }

    float3 focus1_point_ws;
    if (dual_focus)
    {
        float2 focus1_uv = local_uv * focus_box_extent;
        float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
            float4(focus1_uv * float2(2.0f, -2.0f),
                   depth_tex.Load(int3(uint2(int2((0.5f + focus1_uv) * float2(depth_tex_dims))), 0)).x, 1.0f)));
        focus1_point_ws = w.xyz / w.w;
    }
    else
    {
        focus1_point_ws = focus0_point_ws;
    }

    bool tilt_shift = use_tilt_shift != 0u;
    float3 p0 = focus0_point_ws;
    p0.y *= (tilt_shift ? 0.0f : 1.0f);
    float4 cam_h = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w;
    float3 cam_dir = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 p1 = focus1_point_ws;
    p1.y *= (tilt_shift ? 0.0f : 1.0f);

    group_focus0_dists[local_index] = asuint(dot(p0 - cam_pos, cam_dir));
    group_focus1_dists[local_index] = asuint(dot(p1 - cam_pos, cam_dir));
    GroupMemoryBarrierWithGroupSync();

    BITONIC_SORT_UINT(group_focus0_dists, local_index, 64);
    BITONIC_SORT_UINT(group_focus1_dists, local_index, 64);

    if (local_index == 0u)
    {
        float focus0_min = asfloat(group_focus0_dists[0]);
        float focus0_max = asfloat(group_focus0_dists[63]);
        float focus0_median = asfloat(group_focus0_dists[32]);
        float focus1_median = asfloat(group_focus1_dists[32]);

        float near_focus;
        float far_focus;
        if (tilt_shift)
        {
            bool not_dual = !dual_focus;
            near_focus = not_dual ? focus0_max : focus1_median;
            far_focus = not_dual ? focus0_min : focus0_median;
        }
        else
        {
            near_focus = focus1_median;
            far_focus = focus0_median;
        }

        float target_focus0;
        float target_focus1;
        float target_bias0;
        float target_bias1;
        float3 target_lock_pos;
        if (focus_lock_request == camera_idx)
        {
            float4 w = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view,
                float4((point_uv - 0.5f) * float2(2.0f, -2.0f),
                       depth_tex.Load(int3(uint2(int2(point_uv * float2(depth_tex_dims))), 0)).x, 1.0f)));
            float3 lock_point = w.xyz / w.w;
            float3 lock_point_f = lock_point;
            lock_point_f.y *= (tilt_shift ? 0.0f : 1.0f);
            float lock_dist = dot(lock_point_f - cam_pos, cam_dir);
            target_focus0 = max(clamped_focus0, clamped_focus1) - lock_dist;
            target_focus1 = max(far_focus, near_focus) - lock_dist;
            target_bias0 = min(clamped_focus0, clamped_focus1) - lock_dist;
            target_bias1 = min(far_focus, near_focus) - lock_dist;
            target_lock_pos = lock_point;
        }
        else
        {
            target_focus0 = interp_lock_bias1;
            target_focus1 = lock_bias1;
            target_bias0 = interp_lock_bias0;
            target_bias1 = lock_bias0;
            target_lock_pos = lock_pos_ws;
        }

        float out_focus0;
        float out_focus1;
        float3 out_lock_pos;
        float out_bias0;
        float out_bias1;
        if (focus_is_locked != 0u)
        {
            float lerp_fast = 1.0f - exp(delta_time_seconds * -8.0f);
            float3 lerped_lock_pos = not_first_frame
                ? lerp(interp_lock_pos_ws, target_lock_pos, lerp_fast) : target_lock_pos;
            float lerped_bias0 = not_first_frame
                ? lerp(target_bias0, target_bias1, lerp_fast) : target_bias1;
            float lerped_focus0 = not_first_frame
                ? lerp(target_focus0, target_focus1, lerp_fast) : target_focus1;

            float3 lock_point_f = target_lock_pos;
            lock_point_f.y *= (tilt_shift ? 0.0f : 1.0f);
            float lock_dist = dot(lock_point_f - cam_pos, cam_dir);
            out_focus0 = lock_dist + lerped_bias0;
            out_focus1 = lock_dist + lerped_focus0;
            out_lock_pos = lerped_lock_pos;
            out_bias0 = lerped_bias0;
            out_bias1 = lerped_focus0;
        }
        else
        {
            if (not_first_frame)
            {
                float lerp_fast = 1.0f - exp(delta_time_seconds * -8.0f);
                float lerp_slow = 1.0f - exp(delta_time_seconds * -4.0f);
                float inv = 1.0f / max(9.9999997473787516e-06f, clamped_focus0);
                float inv_clamped = 1.0f / max(9.9999997473787516e-06f,
                    clamp(clamped_focus0, focus0_min, focus0_max));
                float inv_lerped = 1.0f / max(9.9999997473787516e-06f, lerp(inv, inv_clamped, lerp_fast));
                float inv_target = 1.0f / max(9.9999997473787516e-06f, far_focus);
                out_focus0 = 1.0f / max(9.9999997473787516e-06f, lerp(inv_lerped, inv_target, lerp_slow));

                float inv1 = 1.0f / max(9.9999997473787516e-06f, clamped_focus1);
                float inv1_clamped = 1.0f / max(9.9999997473787516e-06f,
                    clamp(clamped_focus1, asfloat(group_focus1_dists[0]), asfloat(group_focus1_dists[63])));
                float inv1_lerped = 1.0f / max(9.9999997473787516e-06f, lerp(inv1, inv1_clamped, lerp_fast));
                float inv1_target = 1.0f / max(9.9999997473787516e-06f, near_focus);
                out_focus1 = 1.0f / max(9.9999997473787516e-06f, lerp(inv1_lerped, inv1_target, lerp_slow));
            }
            else
            {
                out_focus0 = far_focus;
                out_focus1 = near_focus;
            }
            out_lock_pos = interp_lock_pos_ws;
            out_bias0 = target_bias0;
            out_bias1 = target_focus0;
        }

        DofFocus result_focus;
        result_focus.focus0 = out_focus0;
        result_focus.focus1 = out_focus1;
        result_focus.lock_bias0 = target_bias1;
        result_focus.lock_bias1 = target_focus1;
        result_focus.lock_pos_ws = target_lock_pos;
        result_focus.is_locked = focus_is_locked;
        result_focus.interp_lock_pos_ws = out_lock_pos;
        result_focus.use_tilt_shift = use_tilt_shift;
        result_focus.interp_lock_bias0 = out_bias0;
        result_focus.interp_lock_bias1 = out_bias1;
        result_focus.focus_scale = aperture_size;
        result_focus.pad2 = pad2;
        __alias_autofocus_buf[camera_idx] = result_focus;

        DofFocusReadback readback;
        readback.lock_pos_ws = target_lock_pos;
        readback.lock_reponse = focus_lock_request;
        __alias_focus_readback_buf[0] = readback;
    }
}
