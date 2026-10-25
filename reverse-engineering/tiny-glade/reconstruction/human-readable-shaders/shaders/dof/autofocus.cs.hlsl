// PATCHED: modules us.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 自动对焦 (dof/autofocus):在对焦框内采样 64 个深度点,分别对两焦点平面做
//   位序归并排序,再把 focus0/focus1/bias 向锁定目标(或场景中位数)做指数平滑,
//   写回 DofFocus(并写 readback)。
// 入口: cs_main (compute, numthreads 8x8x1)。变体: TGV_0 的 cs_main 被 lifted。
// 绑定: depth_tex t14, autofocus_buf u15, focus_readback_buf u16, $Globals b13,
//   frame_constants set1/b0。
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
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
    // 组内 8x8 网格坐标 -> 归一化偏移(对焦框内的采样点)。
    float2 local_uv = (float2(dispatch_id.xy) * 0.14285714924335479736f) - 0.5f;
    GroupMemoryBarrierWithGroupSync();
    bool not_first_frame = (frame_index != 0u);

    // 读取上一帧的自动对焦状态。
    float focus0 = __alias_autofocus_buf[camera_idx].focus0;
    float focus1 = __alias_autofocus_buf[camera_idx].focus1;
    float lock_bias0 = __alias_autofocus_buf[camera_idx].lock_bias0;
    float lock_bias1 = __alias_autofocus_buf[camera_idx].lock_bias1;
    float3 lock_pos_ws = __alias_autofocus_buf[camera_idx].lock_pos_ws;
    float3 interp_lock_pos_ws = __alias_autofocus_buf[camera_idx].interp_lock_pos_ws;
    float interp_lock_bias0 = __alias_autofocus_buf[camera_idx].interp_lock_bias0;
    float interp_lock_bias1 = __alias_autofocus_buf[camera_idx].interp_lock_bias1;
    uint pad2 = __alias_autofocus_buf[camera_idx].pad2;

    // 限制在合理范围(避免除零/爆炸)。
    float clamped_focus0 = clamp(focus0, 0.0f, 100000.0f);
    float clamped_focus1 = clamp(focus1, 0.0f, 100000.0f);

    // 对焦框尺寸:按长宽比归一化,高度 = focus_box_height。
    float2 focus_box_extent = float2(1.0f / (view_constants[0].view_to_clip[1].y / view_constants[0].view_to_clip[0].x), 1.0f)
        * focus_box_height;
    bool dual_focus = use_dual_focus != 0u;

    // 焦点 0 的世界坐标采样点(双焦点时集中在 point_uv 附近的 0.2 倍小框)。
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

    // 焦点 1 的世界坐标采样点(双焦点时铺满整个对焦框;否则复用焦点 0)。
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

    // 倾斜偏移(移轴)模式下忽略高度差。
    bool tilt_shift = use_tilt_shift != 0u;
    float3 p0 = focus0_point_ws;
    p0.y *= (tilt_shift ? 0.0f : 1.0f);
    float4 cam_h = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w;
    float3 cam_dir = normalize(mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 p1 = focus1_point_ws;
    p1.y *= (tilt_shift ? 0.0f : 1.0f);

    // 沿视线方向的距离存入共享内存。
    group_focus0_dists[local_index] = asuint(dot(p0 - cam_pos, cam_dir));
    group_focus1_dists[local_index] = asuint(dot(p1 - cam_pos, cam_dir));
    GroupMemoryBarrierWithGroupSync();

    // 两组 64 元素的位序排序。
    BITONIC_SORT_UINT(group_focus0_dists, local_index, 64);
    BITONIC_SORT_UINT(group_focus1_dists, local_index, 64);

    if (local_index == 0u)
    {
        // 排序后取最小/最大/中位数作为焦点统计量。
        float focus0_min = asfloat(group_focus0_dists[0]);
        float focus0_max = asfloat(group_focus0_dists[63]);
        float focus0_median = asfloat(group_focus0_dists[32]);
        float focus1_median = asfloat(group_focus1_dists[32]);

        float near_focus;
        float far_focus;
        if (tilt_shift)
        {
            // 移轴模式:近/远焦点按是否双焦点取最大/最小或中位数。
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
            // 有锁定请求:把锁定点的视线距离作为新的焦点基准。
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
            // 无锁定:沿用上一帧的插值目标。
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
            // 已锁定:快速平滑逼近锁定点,焦点 = 锁定距离 + 平滑后的偏差。
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
            // 未锁定:在倒数空间做快/慢两层指数平滑。
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
                // 首帧直接取统计量。
                out_focus0 = far_focus;
                out_focus1 = near_focus;
            }
            out_lock_pos = interp_lock_pos_ws;
            out_bias0 = target_bias0;
            out_bias1 = target_focus0;
        }

        // 写回自动对焦状态。
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

        // 读回:锁定点 + 锁定请求。
        DofFocusReadback readback;
        readback.lock_pos_ws = target_lock_pos;
        readback.lock_reponse = focus_lock_request;
        __alias_focus_readback_buf[0] = readback;
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_autofocus.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// TGV_0 变体的 cs 段:与手写 cs 语义一致(64 点采样、位序排序、指数平滑写回),
//   排序循环被内联展开(变量名已语义化)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct DofFocus
{
    float focus0;
    float focus1;
    float lock_bias0;
    float lock_bias1;
    float3 lock_pos_ws;
    uint is_locked;
    float3 interp_lock_pos_ws;
    uint use_tilt_shift;
    float interp_lock_bias0;
    float interp_lock_bias1;
    float focus_scale;
    uint pad2;
};

struct DofFocusReadback
{
    float3 lock_pos_ws;
    uint lock_reponse;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_tex_dims : packoffset(c0);
    float2 point_uv : packoffset(c0.z);
    uint use_dual_focus : packoffset(c1);
    uint focus_lock_request : packoffset(c1.y);
    uint focus_is_locked : packoffset(c1.z);
    float focus_box_height : packoffset(c1.w);
    uint use_tilt_shift : packoffset(c2);
    float aperture_size : packoffset(c2.y);
};

RWByteAddressBuffer __alias_autofocus_buf : register(u15, space0);
RWByteAddressBuffer __alias_focus_readback_buf : register(u16, space0);
Texture2D<float4> depth_tex : register(t14, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared uint group_focus0_dists[64];
groupshared uint group_focus1_dists[64];

void comp_main()
{
    // 组内网格坐标 -> 归一化采样偏移。
    float2 local_uv = (float2(gl_GlobalInvocationID.xy) * 0.14285714924335479736328125f.xx) - 0.5f.xx;
    GroupMemoryBarrierWithGroupSync();
    bool not_first_frame = !(0u == frame_constants_frame_index);
    // 读取上一帧自动对焦状态。
    float focus0 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 0);
    float focus1 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 4);
    float lock_bias0 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 8);
    float lock_bias1 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 12);
    float3 lock_pos_ws = __alias_autofocus_buf.Load<float3>(frame_constants_camera_idx * 64 + 16);
    float3 interp_lock_pos_ws = __alias_autofocus_buf.Load<float3>(frame_constants_camera_idx * 64 + 32);
    float interp_lock_bias0 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 48);
    float interp_lock_bias1 = __alias_autofocus_buf.Load<float>(frame_constants_camera_idx * 64 + 52);
    uint pad2 = __alias_autofocus_buf.Load<uint>(frame_constants_camera_idx * 64 + 60);
    // 限制范围。
    float clamped_focus0 = clamp(focus0, 0.0f, 100000.0f);
    float clamped_focus1 = clamp(focus1, 0.0f, 100000.0f);
    // 对焦框尺寸与双焦点标志。
    float2 focus_box_extent = float2(1.0f / (frame_constants_view_constants[0u].view_to_clip[1].y / frame_constants_view_constants[0u].view_to_clip[0].x), 1.0f) * focus_box_height;
    bool dual_focus = use_dual_focus != 0u;
    // 焦点 0 的世界坐标采样点。
    float3 focus0_point_ws;
    if (dual_focus)
    {
        float2 focus0_uv = point_uv + ((local_uv * focus_box_extent) * 0.20000000298023223876953125f);
        float4 w0 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((focus0_uv - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        focus0_point_ws = w0.xyz / w0.w.xxx;
    }
    else
    {
        float2 focus0_uv = point_uv + (local_uv * focus_box_extent);
        float4 w0 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((focus0_uv - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(focus0_uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        focus0_point_ws = w0.xyz / w0.w.xxx;
    }
    // 焦点 1 的世界坐标采样点。
    float3 focus1_point_ws;
    if (dual_focus)
    {
        float2 focus1_uv = local_uv * focus_box_extent;
        float4 w1 = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4(focus1_uv * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2((0.5f.xx + focus1_uv) * float2(depth_tex_dims))), 0u)).x, 1.0f)));
        focus1_point_ws = w1.xyz / w1.w.xxx;
    }
    else
    {
        focus1_point_ws = focus0_point_ws;
    }
    // 移轴模式忽略高度;取相机位置与朝向。
    bool tilt_shift = use_tilt_shift != 0u;
    float tilt_y_factor = float(tilt_shift ? 0 : 1);
    float3 p0 = focus0_point_ws;
    p0.y = focus0_point_ws.y * tilt_y_factor;
    float4 cam_h = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_h.xyz / cam_h.w.xxx;
    float3 cam_dir = normalize(mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, -1.0f, 0.0f)).xyz);
    float3 p1 = focus1_point_ws;
    p1.y = focus1_point_ws.y * tilt_y_factor;
    // 沿视线距离写入共享内存。
    group_focus0_dists[gl_LocalInvocationIndex] = asuint(dot(p0 - cam_pos, cam_dir));
    group_focus1_dists[gl_LocalInvocationIndex] = asuint(dot(p1 - cam_pos, cam_dir));
    GroupMemoryBarrierWithGroupSync();
    // 位序排序 focus0(64 元素)。
    uint sort_size = 2u << (firstbithigh(63u) & 31u);
    for (uint sort_step = 2u; sort_step <= sort_size; sort_step *= 2u)
    {
        uint half_step = sort_step >> 1u;
        for (uint inner_step = half_step; inner_step > 0u; inner_step = inner_step >> 1u)
        {
            uint lane_block = gl_LocalInvocationIndex & (inner_step - 1u);
            uint lane_base = 2u * (gl_LocalInvocationIndex - lane_block);
            uint idx_low = lane_base + lane_block;
            uint idx_high;
            if (inner_step == half_step)
            {
                idx_high = (lane_base + ((2u * inner_step) - 1u)) - lane_block;
            }
            else
            {
                idx_high = (lane_base + inner_step) + lane_block;
            }
            bool in_range;
            if (idx_high < 64u)
            {
                in_range = idx_low < 64u;
            }
            else
            {
                in_range = false;
            }
            if (in_range)
            {
                if (group_focus0_dists[idx_low] > group_focus0_dists[idx_high])
                {
                    uint tmp0 = group_focus0_dists[idx_low];
                    group_focus0_dists[idx_low] = group_focus0_dists[idx_high];
                    group_focus0_dists[idx_high] = tmp0;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    // 位序排序 focus1(64 元素)。
    for (uint sort_step2 = 2u; sort_step2 <= sort_size; sort_step2 *= 2u)
    {
        uint half_step2 = sort_step2 >> 1u;
        for (uint inner_step2 = half_step2; inner_step2 > 0u; inner_step2 = inner_step2 >> 1u)
        {
            uint lane_block2 = gl_LocalInvocationIndex & (inner_step2 - 1u);
            uint lane_base2 = 2u * (gl_LocalInvocationIndex - lane_block2);
            uint idx_low2 = lane_base2 + lane_block2;
            uint idx_high2;
            if (inner_step2 == half_step2)
            {
                idx_high2 = (lane_base2 + ((2u * inner_step2) - 1u)) - lane_block2;
            }
            else
            {
                idx_high2 = (lane_base2 + inner_step2) + lane_block2;
            }
            bool in_range2;
            if (idx_high2 < 64u)
            {
                in_range2 = idx_low2 < 64u;
            }
            else
            {
                in_range2 = false;
            }
            if (in_range2)
            {
                if (group_focus1_dists[idx_low2] > group_focus1_dists[idx_high2])
                {
                    uint tmp1 = group_focus1_dists[idx_low2];
                    group_focus1_dists[idx_low2] = group_focus1_dists[idx_high2];
                    group_focus1_dists[idx_high2] = tmp1;
                }
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    // 仅 lane 0 做统计与写回。
    if (0u == gl_LocalInvocationIndex)
    {
        // 排序后的最小/最大/中位数。
        float focus0_min = asfloat(group_focus0_dists[0]);
        float focus0_max = asfloat(group_focus0_dists[63u]);
        float focus0_median = asfloat(group_focus0_dists[32u]);
        float focus1_median = asfloat(group_focus1_dists[32u]);
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
        // 目标状态:锁定请求或沿用上一帧。
        float target_focus0;
        float target_focus1;
        float target_bias0;
        float target_bias1;
        float3 target_lock_pos;
        if (focus_lock_request == frame_constants_camera_idx)
        {
            // 锁定点世界坐标 -> 视线距离基准。
            float4 w_lock = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((point_uv - 0.5f.xx) * float2(2.0f, -2.0f), depth_tex.Load(int3(uint2(int2(point_uv * float2(depth_tex_dims))), 0u)).x, 1.0f)));
            float3 lock_point = w_lock.xyz / w_lock.w.xxx;
            float3 lock_point_f = lock_point;
            lock_point_f.y = lock_point.y * tilt_y_factor;
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
        // 输出状态。
        float out_focus0;
        float out_focus1;
        float3 out_lock_pos;
        float out_bias0;
        float out_bias1;
        if (focus_is_locked != 0u)
        {
            // 已锁定:快速平滑。
            float3 lerped_lock_pos;
            if (not_first_frame)
            {
                lerped_lock_pos = lerp(interp_lock_pos_ws, target_lock_pos, (1.0f - exp(frame_constants_delta_time_seconds * (-8.0f))).xxx);
            }
            else
            {
                lerped_lock_pos = target_lock_pos;
            }
            float lerped_bias0;
            if (not_first_frame)
            {
                lerped_bias0 = lerp(target_bias0, target_bias1, 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f)));
            }
            else
            {
                lerped_bias0 = target_bias1;
            }
            float lerped_focus0;
            if (not_first_frame)
            {
                lerped_focus0 = lerp(target_focus0, target_focus1, 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f)));
            }
            else
            {
                lerped_focus0 = target_focus1;
            }
            float3 lock_point_f = target_lock_pos;
            lock_point_f.y = target_lock_pos.y * tilt_y_factor;
            float lock_dist = dot(lock_point_f - cam_pos, cam_dir);
            out_focus0 = lock_dist + lerped_bias0;
            out_focus1 = lock_dist + lerped_focus0;
            out_lock_pos = lerped_lock_pos;
            out_bias0 = lerped_bias0;
            out_bias1 = lerped_focus0;
        }
        else
        {
            // 未锁定:倒数空间快/慢两层平滑。
            float out_focus0_t;
            float out_focus1_t;
            if (not_first_frame)
            {
                float lerp_fast = 1.0f - exp(frame_constants_delta_time_seconds * (-8.0f));
                float lerp_slow = 1.0f - exp(frame_constants_delta_time_seconds * (-4.0f));
                out_focus0_t = 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, clamped_focus0), 1.0f / max(9.9999997473787516355514526367188e-06f, clamp(clamped_focus0, focus0_min, focus0_max)), lerp_fast))), 1.0f / max(9.9999997473787516355514526367188e-06f, far_focus), lerp_slow));
                out_focus1_t = 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, 1.0f / max(9.9999997473787516355514526367188e-06f, lerp(1.0f / max(9.9999997473787516355514526367188e-06f, clamped_focus1), 1.0f / max(9.9999997473787516355514526367188e-06f, clamp(clamped_focus1, asfloat(group_focus1_dists[0]), asfloat(group_focus1_dists[63u]))), lerp_fast))), 1.0f / max(9.9999997473787516355514526367188e-06f, near_focus), lerp_slow));
            }
            else
            {
                // 首帧直接取统计量。
                out_focus0_t = far_focus;
                out_focus1_t = near_focus;
            }
            out_focus0 = out_focus0_t;
            out_focus1 = out_focus1_t;
            out_lock_pos = interp_lock_pos_ws;
            out_bias0 = target_bias0;
            out_bias1 = target_focus0;
        }
        // 构造并逐字段写回 DofFocus。
        DofFocus result_focus = { out_focus0, out_focus1, target_bias1, target_focus1, target_lock_pos, focus_is_locked, out_lock_pos, use_tilt_shift, out_bias0, out_bias1, aperture_size, pad2 };
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 0, result_focus.focus0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 4, result_focus.focus1);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 8, result_focus.lock_bias0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 12, result_focus.lock_bias1);
        __alias_autofocus_buf.Store<float3>(frame_constants_camera_idx * 64 + 16, result_focus.lock_pos_ws);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 28, result_focus.is_locked);
        __alias_autofocus_buf.Store<float3>(frame_constants_camera_idx * 64 + 32, result_focus.interp_lock_pos_ws);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 44, result_focus.use_tilt_shift);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 48, result_focus.interp_lock_bias0);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 52, result_focus.interp_lock_bias1);
        __alias_autofocus_buf.Store<float>(frame_constants_camera_idx * 64 + 56, result_focus.focus_scale);
        __alias_autofocus_buf.Store<uint>(frame_constants_camera_idx * 64 + 60, result_focus.pad2);
        // 写 readback。
        DofFocusReadback readback = { target_lock_pos, focus_lock_request };
        __alias_focus_readback_buf.Store<float3>(0, readback.lock_pos_ws);
        __alias_focus_readback_buf.Store<uint>(12, readback.lock_reponse);
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
