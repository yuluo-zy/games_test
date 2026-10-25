// ==RECONSTRUCTED== shaders/read_mouse_depth_value.cs.hlsl | F-D(手写段 + 提升段)
// 用途: 读取鼠标位置与屏幕中心的深度,换算成世界坐标写入 out_buffer(偏移 0/16);
//   再围绕鼠标与屏幕中心采样 256 个深度(groupshared group_depths),做升序双调排序,
//   用排序后 179/230 位置的深度把结果钳制后写回偏移 32/48,供 CPU 回读鼠标悬停处位置。
// 变体: TGV_0(compute,提升段);手写段在 #if !(...) 中被排除。
// 绑定: $Globals b13 (depth_map_dims/mouse_position_screenspace/window_size/
//   filter_box_y_frac), depth_map t14, out_buffer u15, frame_constants set1/b0,
//   工作组尺寸 [16,16,1],入口 cs_main。
// PATCHED: modules ue.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/read_mouse_depth_value.cs.hlsl | F-D
// Evidence: _read_mouse_depth_value.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: $Globals b13 (depth_map_dims/mouse_position_screenspace/window_size/
//   filter_box_y_frac), out_buffer u15, depth_map t14, frame_constants set1/b0,
//   LocalSize [16,16,1], entry cs_main.
//   spirv-cross: reads depth at mouse position and screen center, stores world-space
//   positions (offset 0/16); samples a box around the mouse into groupshared
//   group_depths[256] (float2), ascending bitonic-sorts (min/max per component, inc
//   bitonic_sort_pair index math), then stores depth-clamped world positions
//   (offset 32/48).
// Status: O (structure from disassembly; bitonic index math matches inc/bitonic_sort.hlsl).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/bitonic_sort.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 depth_map_dims;
    float2 mouse_position_screenspace;
    float2 window_size;
    float filter_box_y_frac;
};

Texture2D<float4> depth_map : register(t14, space0);
RWByteAddressBuffer out_buffer : register(u15, space0);

groupshared float2 group_depths[256];

[numthreads(16, 16, 1)]
void cs_main(uint3 groupThreadID : SV_GroupThreadID, uint groupIndex : SV_GroupIndex)
{
    // 鼠标屏幕坐标归一化到 [0,1] UV。
    float2 mouse_uv = mouse_position_screenspace / window_size;
    float2 dims = float2(depth_map_dims);
    uint2 max_px = depth_map_dims - uint2(1u, 1u);

    // 鼠标位置换算到像素坐标并钳制到合法范围。
    uint2 mouse_px = uint2(int2(clamp(uint2(int2(mouse_uv * dims)), uint2(0u, 0u), max_px)));
    // 读取鼠标处深度。
    float mouse_depth = depth_map.Load(int3(mouse_px, 0)).x;
    // 由深度反投影重建鼠标处的世界坐标。
    float4 mouse_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(mouse_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), mouse_depth, 1.0f)));

    // 屏幕中心像素坐标。
    uint2 center_px = uint2(int2(clamp(uint2(int2(0.5f.xx * dims)), uint2(0u, 0u), max_px)));
    // 读取屏幕中心深度。
    float center_depth = depth_map.Load(int3(center_px, 0)).x;
    // 重建屏幕中心的世界坐标。
    float4 center_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(int2(clamp(center_px, uint2(0u, 0u), max_px))) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), center_depth, 1.0f)));

    if (groupIndex == 0u)
    {
        // 线程 0 写回鼠标/中心的原始深度世界坐标。
        out_buffer.Store<float4>(0, float4(mouse_world.xyz / mouse_world.w, mouse_depth));
        out_buffer.Store<float4>(16, float4(center_world.xyz / center_world.w, center_depth));
    }

    // 采样框偏移:按 filter_box_y_frac 缩放并把整组 256 线程铺到一框。
    float2 sample_offset = ((float(depth_map_dims.y) * filter_box_y_frac).xx / dims) * (((float2(groupThreadID.xy) + 0.5f.xx) * 0.0625f.xx) - 0.5f.xx);
    // 把鼠标框与屏幕中心框的深度存入 groupshared。
    group_depths[groupIndex] = float2(depth_map.Load(int3(uint2(int2(clamp(uint2(int2((mouse_uv + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x, depth_map.Load(int3(uint2(int2(clamp(uint2(int2((0.5f + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0)).x);

    GroupMemoryBarrierWithGroupSync();

    // 双调排序总长度 = 256。
    uint sort_size = 2u << (firstbithigh(255u) & 31u);
    // 双调排序外层:合并跨度逐轮翻倍。
    for (uint stride = 2u; stride <= sort_size; stride *= 2u)
    {
        uint half = stride >> 1u;
        // 内层:比较步长逐步减半。
        for (uint step = half; step > 0u; step = step >> 1u)
        {
            uint lane = groupIndex & (step - 1u);
            uint a = 2u * (groupIndex - lane);
            uint b = a + lane;
            uint c;
            if (step == half)
            {
                c = (a + ((2u * step) - 1u)) - lane;
            }
            else
            {
                c = (a + step) + lane;
            }
            // 只对界内的一对元素做升序归并。
            if (c < 256u && b < 256u)
            {
                float2 va = group_depths[b];
                float2 vb = group_depths[c];
                bool2 cmp = bool2(va.x <= vb.x, va.y <= vb.y);
                group_depths[b] = float2(cmp.x ? va.x : vb.x, cmp.y ? va.y : vb.y);
                group_depths[c] = float2(cmp.x ? vb.x : va.x, cmp.y ? vb.y : va.y);
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }

    if (groupIndex == 0u)
    {
        // 用排序后 179/230 位置的深度钳制鼠标深度(剔除前景/背景离群)。
        float clamped_mouse = clamp(mouse_depth, group_depths[179u].x, group_depths[230u].x);
        float4 mouse_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(mouse_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_mouse, 1.0f)));
        out_buffer.Store<float4>(32, float4(mouse_world_clamped.xyz / mouse_world_clamped.w, clamped_mouse));

        float clamped_center = clamp(center_depth, group_depths[179u].y, group_depths[230u].y);
        float4 center_world_clamped = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4((((float2(center_px) + 0.5f) / dims) - 0.5f) * float2(2.0f, -2.0f), clamped_center, 1.0f)));
        out_buffer.Store<float4>(48, float4(center_world_clamped.xyz / center_world_clamped.w, clamped_center));
    }
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _read_mouse_depth_value.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 depth_map_dims : packoffset(c0);
    float2 mouse_position_screenspace : packoffset(c0.z);
    float2 window_size : packoffset(c1);
    float filter_box_y_frac : packoffset(c1.z);
};

RWByteAddressBuffer out_buffer : register(u15, space0);
Texture2D<float4> depth_map : register(t14, space0);

static uint3 gl_LocalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_LocalInvocationID : SV_GroupThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

groupshared float2 group_depths[256];

void comp_main()
{
    // 鼠标屏幕坐标归一化到 [0,1] UV。
    float2 mouse_uv = mouse_position_screenspace / window_size;
    float2 dims = float2(depth_map_dims);
    uint2 max_px = depth_map_dims - uint2(1u, 1u);
    int2 mouse_px_i = int2(clamp(uint2(int2(mouse_uv * dims)), uint2(0u, 0u), max_px));
    // 鼠标像素坐标(钳制后)。
    uint2 mouse_px = uint2(mouse_px_i);
    float4 mouse_depth4 = depth_map.Load(int3(mouse_px, 0u));
    float mouse_depth = mouse_depth4.x;
    // 由深度反投影重建鼠标处的世界坐标。
    float4 mouse_world = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(int2(clamp(mouse_px, uint2(0u, 0u), max_px))) + 0.5f.xx) / dims) - 0.5f.xx) * float2(2.0f, -2.0f), mouse_depth, 1.0f)));
    // 屏幕中心像素坐标。
    int2 center_px_i = int2(clamp(uint2(int2(0.5f.xx * dims)), uint2(0u, 0u), max_px));
    uint2 center_px = uint2(center_px_i);
    float4 center_depth4 = depth_map.Load(int3(center_px, 0u));
    float center_depth = center_depth4.x;
    // 重建屏幕中心的世界坐标。
    float4 center_world = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(int2(clamp(center_px, uint2(0u, 0u), max_px))) + 0.5f.xx) / dims) - 0.5f.xx) * float2(2.0f, -2.0f), center_depth, 1.0f)));
    // 线程 0 负责写回输出。
    bool is_first_lane = 0u == gl_LocalInvocationIndex;
    if (is_first_lane)
    {
        // 写回鼠标/中心的原始深度世界坐标(偏移 0/16)。
        out_buffer.Store<float4>(0, float4(mouse_world.xyz / mouse_world.w.xxx, mouse_depth));
        out_buffer.Store<float4>(16, float4(center_world.xyz / center_world.w.xxx, center_depth));
    }
    // 采样框偏移:按 filter_box_y_frac 缩放并把整组 256 线程铺到一框。
    float2 sample_offset = ((float(depth_map_dims.y) * filter_box_y_frac).xx / dims) * (((float2(gl_LocalInvocationID.xy) + 0.5f.xx) * 0.0625f.xx) - 0.5f.xx);
    // 把鼠标框与屏幕中心框的深度存入 groupshared。
    group_depths[gl_LocalInvocationIndex] = float2(depth_map.Load(int3(uint2(int2(clamp(uint2(int2((mouse_uv + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0u)).x, depth_map.Load(int3(uint2(int2(clamp(uint2(int2((0.5f.xx + sample_offset) * dims)), uint2(0u, 0u), max_px))), 0u)).x);
    GroupMemoryBarrierWithGroupSync();
    // 双调排序总长度 = 256。
    uint sort_size = 2u << (firstbithigh(255u) & 31u);
    // 双调排序外层:合并跨度逐轮翻倍。
    for (uint stride = 2u; stride <= sort_size; stride *= 2u)
    {
        uint half = stride >> 1u;
        // 内层:比较步长逐步减半。
        for (uint step = half; step > 0u; step = step >> 1u)
        {
            uint lane = gl_LocalInvocationIndex & (step - 1u);
            uint a_idx = 2u * (gl_LocalInvocationIndex - lane);
            uint b_idx = a_idx + lane;
            // 配对索引(step==half 时按反转方向配对)。
            uint c_idx;
            if (step == half)
            {
                c_idx = (a_idx + ((2u * step) - 1u)) - lane;
            }
            else
            {
                c_idx = (a_idx + step) + lane;
            }
            bool in_range;
            if (c_idx < 256u)
            {
                in_range = b_idx < 256u;
            }
            else
            {
                in_range = false;
            }
            // 只对界内的一对元素做升序归并。
            if (in_range)
            {
                // 取一对元素,按分量取 min/max 并写回(升序)。
                float2 va = group_depths[b_idx];
                float2 vb = group_depths[c_idx];
                bool2 cmp = bool2(va.x <= vb.x, va.y <= vb.y);
                group_depths[b_idx] = float2(cmp.x ? va.x : vb.x, cmp.y ? va.y : vb.y);
                group_depths[c_idx] = float2(cmp.x ? vb.x : va.x, cmp.y ? vb.y : va.y);
            }
            GroupMemoryBarrierWithGroupSync();
        }
    }
    if (is_first_lane)
    {
        // 用排序后 179/230 位置的深度钳制鼠标深度(剔除前景/背景离群)。
        float clamped_mouse = clamp(mouse_depth, group_depths[179u].x, group_depths[230u].x);
        float4 mouse_world_clamped = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(mouse_px_i) + 0.5f.xx) / dims) - 0.5f.xx) * float2(2.0f, -2.0f), clamped_mouse, 1.0f)));
        out_buffer.Store<float4>(32, float4(mouse_world_clamped.xyz / mouse_world_clamped.w.xxx, clamped_mouse));
        float clamped_center = clamp(center_depth, group_depths[179u].y, group_depths[230u].y);
        float4 center_world_clamped = mul(frame_constants_view_constants[0u].view_to_world, mul(frame_constants_view_constants[0u].sample_to_view, float4((((float2(center_px_i) + 0.5f.xx) / dims) - 0.5f.xx) * float2(2.0f, -2.0f), clamped_center, 1.0f)));
        out_buffer.Store<float4>(48, float4(center_world_clamped.xyz / center_world_clamped.w.xxx, clamped_center));
    }
}

[numthreads(16, 16, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_LocalInvocationID = stage_input.gl_LocalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
