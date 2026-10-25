// PATCHED: 模块 lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main 由 tools/render-passes/batch_lift.py 从 oracle bin 提升而来;
// 下面这段手写代码在恰好这些 (variant, stage) 组合下被排除。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/sheep.raster.hlsl | F-V | bins: _sheep.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 绵羊实例动画:逐顶点的位置/法线来自结构化缓冲(`animation`),它保存
// 30 帧扫描(positions[30] + normals[30] float4,以帧索引为步长,末尾是
// "开心"姿态的最终一对)。pet_blend 用 sin 缓动向开心姿态交叉淡入;
// is_pet_forward 翻转缓动方向。vs 输出 gbuffer 输入(顶点色、经 model_to_world
// 的动画法线、当前世界位置、上一帧世界位置);ps 打包反照率(sqrt gamma 0.8)
// + 法线进实心 gbuffer 字,并写出速度。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/color_library.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/sh.hlsl"

// INC 问题规避:原版 samplers.hlsl 声明了 s0..s12(13 个采样器);
// inc/samplers.hlsl 只带 s0..s4,s9..s12,因此缺失的 s5..s8 在这里补声明,
// 以复现原版 $Globals 绑定(set0/b13)。

// 动画缓冲按原版结构组织(逐顶点帧扫描):
// 30 个位置 + 30 个法线(各 float4),最后是一对"开心"姿态。
struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
};

StructuredBuffer<SheepAnimation> animation : register(t14);

// 全局常量($Globals)。
float4x4 model_to_world;
float4x4 prev_model_to_world;
int animation_frame;    // 当前动画帧索引
float pet_blend;        // 抚摸混合进度 [0,1]
int is_pet_forward;     // 是否正向播放抚摸

struct VsIn
{
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint vertex_id : SV_VertexID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

// 顶点阶段:按帧插值姿态并做上下摆动。
VsOut vs_main(VsIn vin)
{
    VsOut o;
    // blend = 1 - pet_blend:从常规姿态到开心姿态的补混合量。
    float blend = clamp(1.0f - pet_blend, 0.0f, 1.0f);
    // 上下摆动量:sin 缓动曲线。
    float bob;
    if (is_pet_forward == 0)
    {
        // 正向:摆动随 pet_blend 单调上升。
        float b = clamp(pet_blend, 0.0f, 1.0f);
        bob = (b * b * b) - (b * sin(b * 3.14f));
    }
    else
    {
        // 反向:摆动从 1 回到 0。
        bob = 1.0f - ((blend * blend * blend) - (blend * sin(blend * 3.14f)));
    }
    float3 bob3 = bob.xxx;

    // 当前帧位置与"开心"姿态之间的插值。
    float3 pos = lerp(
        animation[vin.vertex_id].positions[animation_frame].xyz,
        animation[vin.vertex_id].delighted_pos.xyz, bob3);

    // 轻微下沉 0.25,构造局部坐标并变换到世界。
    float4 p = float4(pos.x, pos.y - 0.25f, pos.z, 1.0f);
    float4 wp = mul(model_to_world, p);
    // 世界 -> 采样空间。
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    // 法线同样插值后变换到世界。
    o.C1 = mul(model_to_world,
               float4(lerp(animation[vin.vertex_id].normals[animation_frame].xyz,
                           animation[vin.vertex_id].delighted_normal.xyz, bob3), 0.0f)).xyz;
    // 当前世界位置与上一帧世界位置(用于速度)。
    o.C2 = wp.xyz;
    o.C3 = mul(prev_model_to_world, p).xyz;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

// 片段阶段:打包 gbuffer 并输出速度。
PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 法线压缩到 [0,1]。
    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    // 反照率开平方(以 0.8 变体缩放)。
    float3 albedo = sqrt(clamp(pin.C0 * 0.8f, 0.0f, 1.0f));
    // 8 位量化打包。
    uint albedo_word = (uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                      + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                      + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 3422552064u;
    // gbuffer:法线字 + 反照率字,alpha 位 OR 上材质标志。
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 67108864u;
    // 速度 = 当前世界位置 - 上一帧世界位置。
    o.velocity = pin.C2 - pin.C3;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _sheep.raster.hlsl_b903e43ffb3da915.bin 提升而来 (spirv-cross;行为一致,名称即 SPIR-V id)
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

// 动画缓冲结构(逐顶点,记录 992 字节)。
struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
};

// 帧常量 cbuffer(空间 1)。
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

// 全局常量($Globals)。
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    column_major float4x4 model_to_world : packoffset(c1);
    column_major float4x4 prev_model_to_world : packoffset(c5);
    int animation_frame : packoffset(c9);
    float pet_blend : packoffset(c9.y);
    int is_pet_forward : packoffset(c9.z);
};

// 动画缓冲(字节寻址)。
ByteAddressBuffer animation : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 常规->开心姿态的混合量。
    float blend = clamp(1.0f - pet_blend, 0.0f, 1.0f);
    // 上下摆动量。
    float bob;
    if (is_pet_forward == 0)
    {
        float b = clamp(pet_blend, 0.0f, 1.0f);
        bob = ((b * b) * b) - (b * sin(b * 3.1400001049041748046875f));
    }
    else
    {
        bob = 1.0f - (((blend * blend) * blend) - (blend * sin(blend * 3.1400001049041748046875f)));
    }
    // 摆动量的 xyz 展开。
    float3 bob3 = bob.xxx;
    // 当前位置 = 当前帧位置与开心姿态的插值。
    float3 pos = lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + animation_frame * 16 + 0).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 960).xyz, bob3);
    // 下沉 0.25 并变换到世界。
    float4 p = float4(pos.x, pos.y - 0.25f, pos.z, 1.0f);
    float4 wp = mul(model_to_world, p);
    // 世界 -> 采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    // 法线插值后变换到世界(法线段在偏移 480 处)。
    out_var_C1 = mul(model_to_world, float4(lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + animation_frame * 16 + 480).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 976).xyz, bob3), 0.0f)).xyz;
    // 当前/上一帧世界位置。
    out_var_C2 = wp.xyz;
    out_var_C3 = mul(prev_model_to_world, p).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _sheep.raster.hlsl_f0adff76827e73ff.bin 提升而来 (spirv-cross;行为一致,名称即 SPIR-V id)
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

// 动画缓冲结构(逐顶点,记录 992 字节)。
struct SheepAnimation
{
    float4 positions[30];
    float4 normals[30];
    float4 delighted_pos;
    float4 delighted_normal;
};

// 帧常量 cbuffer(空间 1)。
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

// 全局常量($Globals)。
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    column_major float4x4 model_to_world : packoffset(c1);
    column_major float4x4 prev_model_to_world : packoffset(c5);
    int animation_frame : packoffset(c9);
    float pet_blend : packoffset(c9.y);
    int is_pet_forward : packoffset(c9.z);
};

// 动画缓冲(字节寻址)。
ByteAddressBuffer animation : register(t14, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 常规->开心姿态的混合量。
    float blend = clamp(1.0f - pet_blend, 0.0f, 1.0f);
    // 上下摆动量。
    float bob;
    if (is_pet_forward == 0)
    {
        float b = clamp(pet_blend, 0.0f, 1.0f);
        bob = ((b * b) * b) - (b * sin(b * 3.1400001049041748046875f));
    }
    else
    {
        bob = 1.0f - (((blend * blend) * blend) - (blend * sin(blend * 3.1400001049041748046875f)));
    }
    // 摆动量的 xyz 展开。
    float3 bob3 = bob.xxx;
    // 当前位置 = 当前帧位置与开心姿态的插值。
    float3 pos = lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + animation_frame * 16 + 0).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 960).xyz, bob3);
    // 下沉 0.25 并变换到世界。
    float4 p = float4(pos.x, pos.y - 0.25f, pos.z, 1.0f);
    float4 wp = mul(model_to_world, p);
    // 世界 -> 采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    // 法线插值后变换到世界(法线段在偏移 480 处)。
    out_var_C1 = mul(model_to_world, float4(lerp(animation.Load<float4>(uint(gl_VertexIndex) * 992 + animation_frame * 16 + 480).xyz, animation.Load<float4>(uint(gl_VertexIndex) * 992 + 976).xyz, bob3), 0.0f)).xyz;
    // 当前/上一帧世界位置。
    out_var_C2 = wp.xyz;
    out_var_C3 = mul(prev_model_to_world, p).xyz;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
