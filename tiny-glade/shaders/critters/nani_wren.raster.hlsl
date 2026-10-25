// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 反嘴鹬(wren)着色器:实例间接寻址(StructuredBuffer<uint> -> InstanceData,含两个
// Affine3Packed 变换),翅膀扇动/收拢向静止姿态混合,鸟纹理 G-Buffer 打包。
// 绑定:$Globals b13,instance_indirection_buffer t14,nani_instance_data t15,
// frame_constants set1/b0;ps:sampler_mipmap s10,bird_texture t16。入口 vs_main/ps_main。
// 行为取自 _critters_nani_wren bins(spirv-cross 转译,语义化重命名,指令不变)。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/utils.hlsl"
#include "inc/math_const.hlsl"

// INC-issue workaround: original samplers.hlsl declared s0..s12 (13 samplers);
// inc/samplers.hlsl carries only s0..s4,s9..s12, so the missing s5..s8 are
// declared here to reproduce the original $Globals binding (set0/b13).

// Nani-instance record for a wren (112-byte stride): two packed transforms + wing blend fields.
struct InstanceData
{
    Affine3Packed xform;      // +0
    Affine3Packed prev_xform; // +48
    float flap_t;             // +96
    float wing_fold_t;        // +100
    int id;                   // +104
    uint pad0;                // +108
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> bird_texture : register(t16);

uint2 bird_texture_dims;
int bool_render_shadows;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 rest_pos: rest_pos;
    [[vk::location(5)]] float3 rest_n: rest_n;
    [[vk::location(7)]] float wing_t: wing_t;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    uint inst = instance_indirection_buffer[vin.instance_id];
    Affine3Packed xform = nani_instance_data[inst].xform;
    Affine3Packed prev_xform = nani_instance_data[inst].prev_xform;
    float flap_t = nani_instance_data[inst].flap_t;
    float wing_fold_t = nani_instance_data[inst].wing_fold_t;
    int id = nani_instance_data[inst].id;

    float3 scaled_pos = vin.Vertex_Position * 0.4000000059604644775390625f;
    scaled_pos.y = scaled_pos.y + ((vin.wing_t * 0.4000000059604644775390625f) * sin((flap_t * 6.283185482025146484375f) - (vin.wing_t * 0.31415927410125732421875f)));
    float3 fold_v = wing_fold_t.xxx;

    float3x4 xform_mat = transpose(float4x3(
        float3(xform.data[0], xform.data[1], xform.data[2]),
        float3(xform.data[3], xform.data[4], xform.data[5]),
        float3(xform.data[6], xform.data[7], xform.data[8]),
        float3(xform.data[9], xform.data[10], xform.data[11])));
    float4 model_pos = float4(lerp(scaled_pos, vin.rest_pos * 0.4000000059604644775390625f, fold_v), 1.0f);
    float3 world_pos = mul(xform_mat, model_pos);

    float2 uv_half = vin.Vertex_UV * 0.5f;
    float uv_v = uv_half.y;
    float2 uv_tile = uv_half;
    uv_tile.y = uv_v + 0.5f;
    int skin_id = id - 8 * (id / 8);
    float2 uv_id3;
    if (skin_id == 3)
    {
        float2 uv_tmp3 = uv_tile;
        uv_tmp3.y = uv_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = uv_tile;
    }
    float2 uv_id4;
    if (skin_id == 4)
    {
        float2 uv_tmp4 = uv_id3;
        uv_tmp4.x = uv_id3.x - 0.5f;
        uv_id4 = uv_tmp4;
    }
    else
    {
        uv_id4 = uv_id3;
    }
    float2 uv_final;
    if (skin_id == 5)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform_mat, float4(lerp(vin.Vertex_Normal, vin.rest_n, fold_v), 0.0f));
    o.C2 = world_pos;
    o.C3 = uv_final;
    o.C4 = mul(transpose(float4x3(
        float3(prev_xform.data[0], prev_xform.data[1], prev_xform.data[2]),
        float3(prev_xform.data[3], prev_xform.data[4], prev_xform.data[5]),
        float3(prev_xform.data[6], prev_xform.data[7], prev_xform.data[8]),
        float3(prev_xform.data[9], prev_xform.data[10], prev_xform.data[11]))), model_pos);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
    float3 velocity : SV_Target1;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(bird_texture.SampleBias(sampler_mipmap, pin.C3, global_mip_bias).xyz, 0.0f, 1.0f));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)
                      + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u));
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                      + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                      + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    o.gbuffer = gbuf;
    o.velocity = pin.C2 - pin.C4;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _critters_nani_wren.raster.hlsl_b903e43ffb3da915.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    Affine3Packed prev_xform;
    float flap_t;
    float wing_fold_t;
    int id;
    uint pad0;
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
    uint current_view_index : packoffset(c0);
    uint2 bird_texture_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_n;
static float in_var_wing_t;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_n: rest_n;
    [[vk::location(7)]] float in_var_wing_t: wing_t;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed xform;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        xform.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    Affine3Packed prev_xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        prev_xform.data[_3ident] = nani_instance_data.Load<float>(_3ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    }
    float3 scaled_pos = in_var_Vertex_Position * 0.4000000059604644775390625f;
    scaled_pos.y = scaled_pos.y + ((in_var_wing_t * 0.4000000059604644775390625f) * sin((nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96) * 6.283185482025146484375f) - (in_var_wing_t * 0.31415927410125732421875f)));
    float3 fold_v = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 100).xxx;
    float3x4 xform_mat = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    float4 model_pos = float4(lerp(scaled_pos, in_var_rest_pos * 0.4000000059604644775390625f, fold_v), 1.0f);
    float3 world_pos = mul(xform_mat, model_pos);
    float2 uv_half = in_var_Vertex_UV * 0.5f;
    float uv_v = uv_half.y;
    float2 uv_tile = uv_half;
    uv_tile.y = uv_v + 0.5f;
    int skin_id = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) - 8 * (nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) / 8);
    float2 uv_id3;
    if (skin_id == 3)
    {
        float2 uv_tmp3 = uv_tile;
        uv_tmp3.y = uv_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = uv_tile;
    }
    float2 uv_id4;
    if (skin_id == 4)
    {
        float2 uv_tmp4 = uv_id3;
        uv_tmp4.x = uv_id3.x - 0.5f;
        uv_id4 = uv_tmp4;
    }
    else
    {
        uv_id4 = uv_id3;
    }
    float2 uv_final;
    if (skin_id == 5)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(xform_mat, float4(lerp(in_var_Vertex_Normal, in_var_rest_n, fold_v), 0.0f));
    out_var_C2 = world_pos;
    out_var_C3 = uv_final;
    out_var_C4 = mul(transpose(float4x3(float3(prev_xform.data[0], prev_xform.data[1], prev_xform.data[2]), float3(prev_xform.data[3], prev_xform.data[4], prev_xform.data[5]), float3(prev_xform.data[6], prev_xform.data[7], prev_xform.data[8]), float3(prev_xform.data[9], prev_xform.data[10], prev_xform.data[11]))), model_pos);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_n = stage_input.in_var_rest_n;
    in_var_wing_t = stage_input.in_var_wing_t;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _critters_nani_wren.raster.hlsl_b903e43ffb3da915.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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

SamplerState sampler_mipmap : register(s10, space0);
Texture2D<float4> bird_texture : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;
static float3 out_var_SV_TARGET1;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float3 out_var_SV_TARGET1 : SV_Target1;
};

void frag_main()
{
    float3 packed_normal = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 albedo_sqrt = sqrt(clamp(bird_texture.SampleBias(sampler_mipmap, in_var_C3, frame_constants_global_mip_bias).xyz, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuf = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuf.y = albedo_word | 117440512u;
    out_var_SV_TARGET0 = gbuf;
    out_var_SV_TARGET1 = in_var_C2 - in_var_C4;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    stage_output.out_var_SV_TARGET1 = out_var_SV_TARGET1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _critters_nani_wren.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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

struct Affine3Packed
{
    float data[12];
};

struct InstanceData
{
    Affine3Packed xform;
    Affine3Packed prev_xform;
    float flap_t;
    float wing_fold_t;
    int id;
    uint pad0;
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
    uint current_view_index : packoffset(c0);
    uint2 bird_texture_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_n;
static float in_var_wing_t;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_n: rest_n;
    [[vk::location(7)]] float in_var_wing_t: wing_t;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    Affine3Packed xform;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        xform.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    Affine3Packed prev_xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        prev_xform.data[_3ident] = nani_instance_data.Load<float>(_3ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    }
    float3 scaled_pos = in_var_Vertex_Position * 0.4000000059604644775390625f;
    scaled_pos.y = scaled_pos.y + ((in_var_wing_t * 0.4000000059604644775390625f) * sin((nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96) * 6.283185482025146484375f) - (in_var_wing_t * 0.31415927410125732421875f)));
    float3 fold_v = nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 100).xxx;
    float3x4 xform_mat = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    float4 model_pos = float4(lerp(scaled_pos, in_var_rest_pos * 0.4000000059604644775390625f, fold_v), 1.0f);
    float3 world_pos = mul(xform_mat, model_pos);
    float2 uv_half = in_var_Vertex_UV * 0.5f;
    float uv_v = uv_half.y;
    float2 uv_tile = uv_half;
    uv_tile.y = uv_v + 0.5f;
    int skin_id = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) - 8 * (nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 104) / 8);
    float2 uv_id3;
    if (skin_id == 3)
    {
        float2 uv_tmp3 = uv_tile;
        uv_tmp3.y = uv_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = uv_tile;
    }
    float2 uv_id4;
    if (skin_id == 4)
    {
        float2 uv_tmp4 = uv_id3;
        uv_tmp4.x = uv_id3.x - 0.5f;
        uv_id4 = uv_tmp4;
    }
    else
    {
        uv_id4 = uv_id3;
    }
    float2 uv_final;
    if (skin_id == 5)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(xform_mat, float4(lerp(in_var_Vertex_Normal, in_var_rest_n, fold_v), 0.0f));
    out_var_C2 = world_pos;
    out_var_C3 = uv_final;
    out_var_C4 = mul(transpose(float4x3(float3(prev_xform.data[0], prev_xform.data[1], prev_xform.data[2]), float3(prev_xform.data[3], prev_xform.data[4], prev_xform.data[5]), float3(prev_xform.data[6], prev_xform.data[7], prev_xform.data[8]), float3(prev_xform.data[9], prev_xform.data[10], prev_xform.data[11]))), model_pos);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_n = stage_input.in_var_rest_n;
    in_var_wing_t = stage_input.in_var_wing_t;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _critters_nani_wren.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
