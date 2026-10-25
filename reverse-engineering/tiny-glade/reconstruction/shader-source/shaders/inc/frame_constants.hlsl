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

// RuntimeDescriptorArray trigger (adjudicated inc-fix #1, RECOVERY_TRACKER #1/#2/#6):
// the game builds every shader that includes frame_constants with the
// RuntimeDescriptorArray capability + SPV_EXT_descriptor_indexing extension. The
// installed dxc only emits them when an unbounded descriptor array is declared,
// so this unused array is declared here (space2/t0, separate descriptor space —
// does not shift any consumer's set0 bindings) to reproduce the capability set.
// Declared `[unused]` to make the intent explicit; dxc DCEs it in consumers that
// already carry their own trigger.
Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);

// Per-view matrix set. All matrices are column-major (SPIR-V column_major).
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
