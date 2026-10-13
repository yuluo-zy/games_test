// ============================================================
// inc/frame_constants.hlsl —— 每帧常量 UBO
// 用途:定义 set1/b0 的 frame_constants UBO 与每相机视图矩阵集合。
//   成员声明为扁平(全局作用域 cbuffer 成员),消费方直接引用
//   `view_constants[2]`、`sun_color` 等(cbuffer 容器名不参与成员访问)。
//   `view_constants[5]` 每相机/视图一套;`current_view_index`
//   (current_view.hlsl 声明)在它们之间选择。
// 关键绑定:set1/b0;ViewConstants 步长 800、view_constants[5]
// ============================================================
// ==RECONSTRUCTED== shaders/inc/frame_constants.hlsl | evidence: _apply_screen_shadows.cs.hlsl_cs_main_0.bin (UBO struct 32 = type.ConstantBuffer.FrameConstants, 27 members, offsets 0..240; ViewConstants struct 33 offsets 0..792; view_constants[5] ArrayStride=800, len const=5) + _billboard_forest raster (same layout) + catalog.json set1/b0 frame_constants UBO | status: O (members/types/offsets verbatim); O/I (ViewConstants array size=5: O via ArrayStride 800 + len const 5)
//
// Per-frame UBO, bound at set1/b0, named `frame_constants`. Members are declared
// flat (global-scope cbuffer members), so consumers reference e.g. `view_constants[2]`
// and `sun_color` directly (the cbuffer container name is NOT part of member access).
// `ViewConstants` holds per-camera matrices; `view_constants[5]` holds one set per
// camera/view; `current_view_index` (declared in current_view.hlsl) selects among them.

#ifndef TINYGLADE_INC_FRAME_CONSTANTS_HLSL
#define TINYGLADE_INC_FRAME_CONSTANTS_HLSL

#include "uv.hlsl"

// RuntimeDescriptorArray 触发(裁定 inc-fix #1,RECOVERY_TRACKER #1/#2/#6):
// 游戏构建每个含 frame_constants 的着色器都带 RuntimeDescriptorArray 能力 +
// SPV_EXT_descriptor_indexing 扩展。已装的 dxc 只有在声明无界描述符数组时
// 才发出它们,所以这里声明一个未使用的数组(space2/t0,独立描述符空间,
// 不移动任何消费方的 set0 绑定)来复现能力集。
// 声明为 [unused] 以显式表达意图;dxc 在自带触发器的消费方里会 DCE 掉它。
Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);

// 每视图矩阵集合。所有矩阵为列主序(SPIR-V column_major)。
struct ViewConstants
{
    float4x4 view_to_clip;
    float4x4 clip_to_view;
    float4x4 view_to_sample;
    float4x4 sample_to_view;
    float4x4 world_to_view;
    float4x4 view_to_world;
    float4x4 world_to_sample;
    float4x4 clip_to_prev_clip;
    float4x4 prev_view_to_prev_clip;
    float4x4 prev_clip_to_prev_view;
    float4x4 prev_world_to_prev_view;
    float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

#endif // TINYGLADE_INC_FRAME_CONSTANTS_HLSL
