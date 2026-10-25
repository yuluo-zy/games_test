// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 蝴蝶着色器:实例化蝴蝶(32B 步长),翅膀扇动,级联阴影螺旋,按 skin id%8 的
// 图集 UV,天空环境光 + 太阳 * (1-阴影)/pi。
// 绑定:sampler_default s9, sampler_shadow_nearest s11, sampler_shadow_linear s12,
// $Globals b13, shadow_map t14, terrain_heightmap t17, butterfly_buffer t19,
// frame_constants set1/b0;ps: sampler_llr s1, butterfly_skins t16。入口 vs_main/ps_main。
// 行为取自 _critters_butterfly bins(spirv-cross 转译,语义化重命名,指令不变)。

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
Texture2D<float4> terrain_heightmap : register(t17);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"

// INC 问题规避:原版 samplers.hlsl 声明 s0..s12(13 个采样器);
// inc/samplers.hlsl 只带 s0..s4,s9..s12,因此缺失的 s5..s8 在此声明,
// 以还原原版 $Globals 绑定(set0/b13)。

struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

StructuredBuffer<Butterfly> butterfly_buffer : register(t19);
Texture2DArray<float4> shadow_map : register(t14);
Texture2D<float4> butterfly_skins : register(t16);

int bool_render_shadows;
uint2 butterfly_skins_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;

// 蝴蝶世界位置的级联阴影系数。先用 5 次采样螺旋取基础级联阴影,再用
// 10 次采样软环取半影因子;返回 float2(因子, 未用的第二通道);点在
// 阴影图外 / 完全受光时返回 0。
float2 butterfly_shadow(float3 world_pos, float4 shadow_dir)
{
    float4 proj = mul(view_constants[2].world_to_sample, float4(world_pos, 1.0f));
    if (proj.z > 1.0f)
    {
        return 0.0f.xx;
    }
    float2 proj_xy = proj.xy;
    float2 proj_abs = abs(proj_xy);
    bool edge_probe = any(bool2(proj_abs.x > 0.9833333492279052734375f, proj_abs.y > 0.9833333492279052734375f));
    float2 cascade_xy;
    float sample_count;
    float cascade_scale;
    if (edge_probe)
    {
        cascade_xy = proj_xy * 0.5f;
        sample_count = 5.0f;
        cascade_scale = 0.5f;
    }
    else
    {
        cascade_xy = proj_xy;
        sample_count = 10.0f;
        cascade_scale = 1.0f;
    }
    float2 cascade_abs = abs(cascade_xy);
    float2 cascade_limit = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * sample_count));
    bool outer_cascade = any(bool2(cascade_abs.x > cascade_limit.x, cascade_abs.y > cascade_limit.y));
    float soft_taps;
    float2 soft_xy;
    float soft_scale;
    if (outer_cascade)
    {
        soft_taps = sample_count * 0.2857142984867095947265625f;
        soft_xy = cascade_xy * 0.2857142984867095947265625f;
        soft_scale = cascade_scale * 0.2857142984867095947265625f;
    }
    else
    {
        soft_taps = sample_count;
        soft_xy = cascade_xy;
        soft_scale = cascade_scale;
    }
    float cascade_index = float(outer_cascade ? 2u : uint(edge_probe));
    uint cascade_view = 2u + uint(cascade_index);
    float shadow_dir_y = shadow_dir.y;
    float4 shadow_pos = float4(soft_xy.x, soft_xy.y, proj.z, proj.w)
        + mul(view_constants[cascade_view].world_to_sample,
              float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - shadow_dir_y, 0.0f, 1.0f))) / soft_scale.xxx, 0.0f));
    float2 shadow_uv = (shadow_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
    float3 shadow_coord = float3(shadow_uv.x, shadow_uv.y, shadow_pos.z);
    float shadow_bias = max((0.300000011920928955078125f / soft_scale) * (1.0f - abs(shadow_dir_y)), 0.02500000037252902984619140625f);
    float spiral_angle = float((frame_index * 5u) % 64u) * 1.61803400516510009765625f;
    float spiral_seed = frac(52.98291778564453125f * frac(float(frame_index % 64u) * 0.40764915943145751953125f));
    float4 light_dir = mul(view_constants[cascade_view].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
    float depth_acc = 0.0f;
    int depth_count = 0;
    for (int i = 0; i < 5; i++)
    {
        float i_f = float(i);
        float angle = (i_f + spiral_angle) * 2.3999631404876708984375f;
        float3 sample_pos = shadow_coord + float3(((float2(cos(angle), sin(angle)) * (i_f + spiral_seed)) * 0.20000000298023223876953125f.xx) * soft_taps * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
        float4 shadow_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sample_pos.xy, cascade_index), 0.0f);
        float shadow_depth = shadow_sample.x;
        if (sample_pos.z > shadow_depth)
        {
            depth_acc = depth_acc + shadow_depth;
            depth_count = depth_count + 1;
        }
    }
    float avg_depth = depth_acc / float(depth_count);
    if (depth_count == 0)
    {
        return 0.0f.xx;
    }
    float contact = max(((shadow_pos.z - avg_depth) * ((17.0666675567626953125f * soft_taps) * pow(10.0f, smoothstep(0.0f, 0.25f, fog_tint_thickness.w)))) / avg_depth, 0.0f);
    float penumbra = (contact / pow(pow(contact / (soft_taps - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
    float light_z = light_dir.z;
    float2 light_uv = (((light_dir.xy * float(int(sign(light_z)))) / max(0.1500000059604644775390625f, abs(light_z)).xx) * 0.0009765625f.xx) * 2.0f;
    float2 penumbra_acc = 0.0f.xx;
    for (int j = 0; j < 10; j++)
    {
        float j_f = float(j);
        float ring_angle = (j_f + spiral_angle) * 2.3999631404876708984375f;
        float2 ring_offset = (float2(cos(ring_angle), sin(ring_angle)) * (j_f + spiral_seed)) * 0.100000001490116119384765625f.xx * penumbra;
        float3 ring_pos = shadow_coord + float3(ring_offset * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
        penumbra_acc = penumbra_acc + float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(ring_pos.xy, cascade_index), ring_pos.z + min(0.0f, dot(light_uv, ring_offset))), 0.0f);
    }
    return penumbra_acc * 0.100000001490116119384765625f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float is_wing: is_wing;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float2 uv_in = vin.Vertex_UV;
    float3 scaled_pos = vin.Vertex_Position * 0.4000000059604644775390625f;
    float3 wing_pos;
    if (vin.is_wing > 0.0f)
    {
        float flap_progress = abs(0.5f - butterfly_buffer[vin.instance_id].anim_t);
        float wing_angle = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f,
            (cos((butterfly_buffer[vin.instance_id].anim_t + ((scaled_pos.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, flap_progress)) + 0.5f) * 6.283185482025146484375f) * 0.5f + 0.5f)
            * float(int(sign(scaled_pos.x)));
        float cos_a = cos(wing_angle);
        float sin_a = sin(wing_angle);
        float2 wing_xy = scaled_pos.xy;
        float2 rotated_xy = lerp(wing_xy, mul(transpose(float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a))), wing_xy), lerp(vin.is_wing, 1.0f, 1.0f - (flap_progress * 2.0f)).xx);
        wing_pos = float3(rotated_xy.x, rotated_xy.y, scaled_pos.z);
    }
    else
    {
        wing_pos = scaled_pos;
    }
    float3 world_pos = (mul(float3x3(cross(butterfly_buffer[vin.instance_id].forward, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer[vin.instance_id].forward), wing_pos) + butterfly_buffer[vin.instance_id].position) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, butterfly_buffer[vin.instance_id].position.xz * 0.0076923076994717121124267578125f.xx + 0.5f.xx, 0.0f).x), 0.0f);

    float shadow_attn;
    if (bool_render_shadows == 1)
    {
        float4 shadow_dir = mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 shadow_factor = butterfly_shadow(world_pos, shadow_dir);
        shadow_attn = shadow_factor.x;
    }
    else
    {
        shadow_attn = 0.0f;
    }

    uv_in.y = 1.0f - uv_in.y;
    float2 atlas_uv = uv_in * 0.5f;
    float atlas_v = atlas_uv.y;
    float2 atlas_tile = atlas_uv;
    atlas_tile.y = atlas_v + 0.5f;
    uint skin_id = vin.instance_id % 8u;
    float2 uv_id3;
    if (skin_id == 3u)
    {
        float2 uv_tmp3 = atlas_tile;
        uv_tmp3.y = atlas_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = atlas_tile;
    }
    float2 uv_id4;
    if (skin_id == 4u)
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
    if (skin_id == 5u)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    o.C0 = ((sun_color.xyz * (1.0f - shadow_attn)) * 0.3183098733425140380859375f.xxx) + sky_high_color.xyz;
    o.C1 = uv_final;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float2 C1 : TEXCOORD1;
};

struct PsOut
{
    float4 color : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 col = (butterfly_skins.Sample(sampler_llr, pin.C1).xyz * pin.C0) * 0.800000011920928955078125f;
    col = ((render_overrides & 1u) != 0u) ? 0.5f.xxx : col;
    o.color = float4(col, 1.0f);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _critters_butterfly.raster.hlsl_b903e43ffb3da915.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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

struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

static float zero_lane;

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
    int bool_render_shadows : packoffset(c0.y);
    uint2 butterfly_skins_dims : packoffset(c0.z);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
};

ByteAddressBuffer butterfly_buffer : register(t19, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float in_var_is_wing;
static float3 out_var_C0;
static float2 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float in_var_is_wing: is_wing;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 uv_in = in_var_Vertex_UV;
    float3 scaled_pos = in_var_Vertex_Position * 0.4000000059604644775390625f;
    float3 wing_pos;
    if (in_var_is_wing > 0.0f)
    {
        float flap_progress = abs(0.5f - butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12));
        float wing_angle = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f, (cos(((butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12) + (((scaled_pos.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, flap_progress))) + 0.5f) * 6.283185482025146484375f) * 0.5f) + 0.5f) * float(int(sign(scaled_pos.x)));
        float cos_a = cos(wing_angle);
        float sin_a = sin(wing_angle);
        float2 wing_xy = scaled_pos.xy;
        float2 rotated_xy = lerp(wing_xy, mul(transpose(float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a))), wing_xy), lerp(in_var_is_wing, 1.0f, 1.0f - (flap_progress * 2.0f)).xx);
        wing_pos = float3(rotated_xy.x, rotated_xy.y, scaled_pos.z);
    }
    else
    {
        wing_pos = scaled_pos;
    }
    float3 world_pos = (mul(float3x3(cross(butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16), float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16)), wing_pos) + butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0)) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), 0.0f);
    float shadow_attn;
    if (bool_render_shadows == 1)
    {
        float4 shadow_dir = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 shadow_factor;
        do
        {
            float4 proj = mul(frame_constants_view_constants[2u].world_to_sample, float4(world_pos, 1.0f));
            if (proj.z > 1.0f)
            {
                shadow_factor = 0.0f.xx;
                break;
            }
            float2 proj_xy = proj.xy;
            float2 proj_abs = abs(proj_xy);
            bool edge_probe = any(bool2(proj_abs.x > 0.9833333492279052734375f.xx.x, proj_abs.y > 0.9833333492279052734375f.xx.y));
            float2 cascade_xy;
            if (edge_probe)
            {
                cascade_xy = proj_xy * 0.5f;
            }
            else
            {
                cascade_xy = proj_xy;
            }
            float sample_count = edge_probe ? 5.0f : 10.0f;
            float cascade_scale = edge_probe ? 0.5f : 1.0f;
            float2 cascade_abs = abs(cascade_xy);
            float2 cascade_limit = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * sample_count));
            bool outer_cascade = any(bool2(cascade_abs.x > cascade_limit.x, cascade_abs.y > cascade_limit.y));
            float soft_taps;
            float2 soft_xy;
            float soft_scale;
            if (outer_cascade)
            {
                soft_taps = sample_count * 0.2857142984867095947265625f;
                soft_xy = cascade_xy * 0.2857142984867095947265625f;
                soft_scale = cascade_scale * 0.2857142984867095947265625f;
            }
            else
            {
                soft_taps = sample_count;
                soft_xy = cascade_xy;
                soft_scale = cascade_scale;
            }
            float cascade_index = float(outer_cascade ? 2u : uint(edge_probe));
            uint cascade_view = 2u + uint(cascade_index);
            float shadow_dir_y = shadow_dir.y;
            float4 shadow_pos = float4(soft_xy.x, soft_xy.y, proj.z, proj.w) + mul(frame_constants_view_constants[cascade_view].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - shadow_dir_y, 0.0f, 1.0f))) / soft_scale.xxx, 0.0f));
            float2 shadow_uv = (shadow_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 shadow_coord = float3(shadow_uv.x, shadow_uv.y, shadow_pos.z);
            float shadow_bias = max((0.300000011920928955078125f / soft_scale) * (1.0f - abs(shadow_dir_y)), 0.02500000037252902984619140625f);
            float spiral_angle = float((frame_constants_frame_index * 5u) % 64u) * 1.61803400516510009765625f;
            float spiral_seed = frac(52.98291778564453125f * frac(float(frame_constants_frame_index % 64u) * 0.40764915943145751953125f));
            float4 light_dir = mul(frame_constants_view_constants[cascade_view].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
            float depth_acc;
            int depth_count;
            depth_acc = 0.0f;
            depth_count = 0;
            float next_depth_acc;
            int next_depth_count;
            for (int i = 0; i < 5; depth_acc = next_depth_acc, depth_count = next_depth_count, i++)
            {
                float i_f = float(i);
                float angle = (i_f + spiral_angle) * 2.3999631404876708984375f;
                float3 sample_pos = shadow_coord + float3((((float2(cos(angle), sin(angle)) * (i_f + spiral_seed)) * 0.20000000298023223876953125f.xx) * soft_taps) * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
                float4 shadow_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sample_pos.xy, cascade_index), 0.0f);
                float shadow_depth = shadow_sample.x;
                if (sample_pos.z > shadow_depth)
                {
                    next_depth_acc = depth_acc + shadow_depth;
                    next_depth_count = depth_count + 1;
                }
                else
                {
                    next_depth_acc = depth_acc;
                    next_depth_count = depth_count;
                }
            }
            float avg_depth = depth_acc / float(depth_count);
            if (depth_count == 0)
            {
                shadow_factor = 0.0f.xx;
                break;
            }
            float contact = max(((shadow_pos.z - avg_depth) * ((17.0666675567626953125f * soft_taps) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / avg_depth, 0.0f);
            float penumbra = (contact / pow(pow(contact / (soft_taps - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float light_z = light_dir.z;
            float2 light_uv = (((light_dir.xy * float(int(sign(light_z)))) / max(0.1500000059604644775390625f, abs(light_z)).xx) * 0.0009765625f.xx) * 2.0f;
            float2 penumbra_acc;
            penumbra_acc = 0.0f.xx;
            for (int j = 0; j < 10; )
            {
                float j_f = float(j);
                float ring_angle = (j_f + spiral_angle) * 2.3999631404876708984375f;
                float2 ring_offset = ((float2(cos(ring_angle), sin(ring_angle)) * (j_f + spiral_seed)) * 0.100000001490116119384765625f.xx) * penumbra;
                float3 ring_pos = shadow_coord + float3(ring_offset * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
                penumbra_acc += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(ring_pos.xy, cascade_index), ring_pos.z + min(0.0f, dot(light_uv, ring_offset))), zero_lane);
                j++;
                continue;
            }
            shadow_factor = penumbra_acc * 0.100000001490116119384765625f;
            break;
        } while(false);
        shadow_attn = shadow_factor.x;
    }
    else
    {
        shadow_attn = 0.0f;
    }
    uv_in.y = 1.0f - uv_in.y;
    float2 atlas_uv = uv_in * 0.5f;
    float atlas_v = atlas_uv.y;
    float2 atlas_tile = atlas_uv;
    atlas_tile.y = atlas_v + 0.5f;
    uint skin_id = uint(gl_InstanceIndex) % 8u;
    float2 uv_id3;
    if (skin_id == 3u)
    {
        float2 uv_tmp3 = atlas_tile;
        uv_tmp3.y = atlas_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = atlas_tile;
    }
    float2 uv_id4;
    if (skin_id == 4u)
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
    if (skin_id == 5u)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    out_var_C0 = ((frame_constants_sun_color.xyz * (1.0f - shadow_attn)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz;
    out_var_C1 = uv_final;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_wing = stage_input.in_var_is_wing;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 从 _critters_butterfly.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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

struct Butterfly
{
    float3 position;
    float anim_t;
    float3 forward;
    uint pad;
};

static float zero_lane;

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
    int bool_render_shadows : packoffset(c0.y);
    uint2 butterfly_skins_dims : packoffset(c0.z);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
};

ByteAddressBuffer butterfly_buffer : register(t19, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);
Texture2DArray<float4> shadow_map : register(t14, space0);
Texture2D<float4> terrain_heightmap : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float in_var_is_wing;
static float3 out_var_C0;
static float2 out_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float in_var_is_wing: is_wing;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float2 uv_in = in_var_Vertex_UV;
    float3 scaled_pos = in_var_Vertex_Position * 0.4000000059604644775390625f;
    float3 wing_pos;
    if (in_var_is_wing > 0.0f)
    {
        float flap_progress = abs(0.5f - butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12));
        float wing_angle = lerp(-0.60000002384185791015625f, 1.39999997615814208984375f, (cos(((butterfly_buffer.Load<float>(uint(gl_InstanceIndex) * 32 + 12) + (((scaled_pos.z + 0.300000011920928955078125f) * (-0.25f)) * smoothstep(0.0f, 0.5f, flap_progress))) + 0.5f) * 6.283185482025146484375f) * 0.5f) + 0.5f) * float(int(sign(scaled_pos.x)));
        float cos_a = cos(wing_angle);
        float sin_a = sin(wing_angle);
        float2 wing_xy = scaled_pos.xy;
        float2 rotated_xy = lerp(wing_xy, mul(transpose(float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a))), wing_xy), lerp(in_var_is_wing, 1.0f, 1.0f - (flap_progress * 2.0f)).xx);
        wing_pos = float3(rotated_xy.x, rotated_xy.y, scaled_pos.z);
    }
    else
    {
        wing_pos = scaled_pos;
    }
    float3 world_pos = (mul(float3x3(cross(butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16), float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 16)), wing_pos) + butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0)) + float3(0.0f, lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (butterfly_buffer.Load<float3>(uint(gl_InstanceIndex) * 32 + 0).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x), 0.0f);
    float shadow_attn;
    if (bool_render_shadows == 1)
    {
        float4 shadow_dir = mul(frame_constants_view_constants[2u].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f));
        float2 shadow_factor;
        do
        {
            float4 proj = mul(frame_constants_view_constants[2u].world_to_sample, float4(world_pos, 1.0f));
            if (proj.z > 1.0f)
            {
                shadow_factor = 0.0f.xx;
                break;
            }
            float2 proj_xy = proj.xy;
            float2 proj_abs = abs(proj_xy);
            bool edge_probe = any(bool2(proj_abs.x > 0.9833333492279052734375f.xx.x, proj_abs.y > 0.9833333492279052734375f.xx.y));
            float2 cascade_xy;
            if (edge_probe)
            {
                cascade_xy = proj_xy * 0.5f;
            }
            else
            {
                cascade_xy = proj_xy;
            }
            float sample_count = edge_probe ? 5.0f : 10.0f;
            float cascade_scale = edge_probe ? 0.5f : 1.0f;
            float2 cascade_abs = abs(cascade_xy);
            float2 cascade_limit = 1.0f.xx - (0.000833333353511989116668701171875f.xx * (2.0f * sample_count));
            bool outer_cascade = any(bool2(cascade_abs.x > cascade_limit.x, cascade_abs.y > cascade_limit.y));
            float soft_taps;
            float2 soft_xy;
            float soft_scale;
            if (outer_cascade)
            {
                soft_taps = sample_count * 0.2857142984867095947265625f;
                soft_xy = cascade_xy * 0.2857142984867095947265625f;
                soft_scale = cascade_scale * 0.2857142984867095947265625f;
            }
            else
            {
                soft_taps = sample_count;
                soft_xy = cascade_xy;
                soft_scale = cascade_scale;
            }
            float cascade_index = float(outer_cascade ? 2u : uint(edge_probe));
            uint cascade_view = 2u + uint(cascade_index);
            float shadow_dir_y = shadow_dir.y;
            float4 shadow_pos = float4(soft_xy.x, soft_xy.y, proj.z, proj.w) + mul(frame_constants_view_constants[cascade_view].world_to_sample, float4((float3(0.0f, 1.0f, 0.0f) * (0.066666670143604278564453125f * clamp(1.0f - shadow_dir_y, 0.0f, 1.0f))) / soft_scale.xxx, 0.0f));
            float2 shadow_uv = (shadow_pos.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
            float3 shadow_coord = float3(shadow_uv.x, shadow_uv.y, shadow_pos.z);
            float shadow_bias = max((0.300000011920928955078125f / soft_scale) * (1.0f - abs(shadow_dir_y)), 0.02500000037252902984619140625f);
            float spiral_angle = float((frame_constants_frame_index * 5u) % 64u) * 1.61803400516510009765625f;
            float spiral_seed = frac(52.98291778564453125f * frac(float(frame_constants_frame_index % 64u) * 0.40764915943145751953125f));
            float4 light_dir = mul(frame_constants_view_constants[cascade_view].world_to_sample, float4(0.0f, 1.0f, 0.0f, 0.0f));
            float depth_acc;
            int depth_count;
            depth_acc = 0.0f;
            depth_count = 0;
            float next_depth_acc;
            int next_depth_count;
            for (int i = 0; i < 5; depth_acc = next_depth_acc, depth_count = next_depth_count, i++)
            {
                float i_f = float(i);
                float angle = (i_f + spiral_angle) * 2.3999631404876708984375f;
                float3 sample_pos = shadow_coord + float3((((float2(cos(angle), sin(angle)) * (i_f + spiral_seed)) * 0.20000000298023223876953125f.xx) * soft_taps) * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
                float4 shadow_sample = shadow_map.SampleLevel(sampler_shadow_nearest, float3(sample_pos.xy, cascade_index), 0.0f);
                float shadow_depth = shadow_sample.x;
                if (sample_pos.z > shadow_depth)
                {
                    next_depth_acc = depth_acc + shadow_depth;
                    next_depth_count = depth_count + 1;
                }
                else
                {
                    next_depth_acc = depth_acc;
                    next_depth_count = depth_count;
                }
            }
            float avg_depth = depth_acc / float(depth_count);
            if (depth_count == 0)
            {
                shadow_factor = 0.0f.xx;
                break;
            }
            float contact = max(((shadow_pos.z - avg_depth) * ((17.0666675567626953125f * soft_taps) * pow(10.0f, smoothstep(0.0f, 0.25f, frame_constants_fog_tint_thickness.w)))) / avg_depth, 0.0f);
            float penumbra = (contact / pow(pow(contact / (soft_taps - 1.0f), 5.0f) + 1.0f, 0.20000000298023223876953125f)) + 1.0f;
            float light_z = light_dir.z;
            float2 light_uv = (((light_dir.xy * float(int(sign(light_z)))) / max(0.1500000059604644775390625f, abs(light_z)).xx) * 0.0009765625f.xx) * 2.0f;
            float2 penumbra_acc;
            penumbra_acc = 0.0f.xx;
            for (int j = 0; j < 10; )
            {
                float j_f = float(j);
                float ring_angle = (j_f + spiral_angle) * 2.3999631404876708984375f;
                float2 ring_offset = ((float2(cos(ring_angle), sin(ring_angle)) * (j_f + spiral_seed)) * 0.100000001490116119384765625f.xx) * penumbra;
                float3 ring_pos = shadow_coord + float3(ring_offset * 0.000833333353511989116668701171875f.xx, shadow_bias * (-0.0009765625f));
                penumbra_acc += float2(1.0f - shadow_map.SampleCmpLevelZero(sampler_shadow_linear, float3(ring_pos.xy, cascade_index), ring_pos.z + min(0.0f, dot(light_uv, ring_offset))), zero_lane);
                j++;
                continue;
            }
            shadow_factor = penumbra_acc * 0.100000001490116119384765625f;
            break;
        } while(false);
        shadow_attn = shadow_factor.x;
    }
    else
    {
        shadow_attn = 0.0f;
    }
    uv_in.y = 1.0f - uv_in.y;
    float2 atlas_uv = uv_in * 0.5f;
    float atlas_v = atlas_uv.y;
    float2 atlas_tile = atlas_uv;
    atlas_tile.y = atlas_v + 0.5f;
    uint skin_id = uint(gl_InstanceIndex) % 8u;
    float2 uv_id3;
    if (skin_id == 3u)
    {
        float2 uv_tmp3 = atlas_tile;
        uv_tmp3.y = atlas_v;
        uv_id3 = uv_tmp3;
    }
    else
    {
        uv_id3 = atlas_tile;
    }
    float2 uv_id4;
    if (skin_id == 4u)
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
    if (skin_id == 5u)
    {
        uv_final = uv_id4 - 0.5f.xx;
    }
    else
    {
        uv_final = uv_id4;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(world_pos, 1.0f));
    out_var_C0 = ((frame_constants_sun_color.xyz * (1.0f - shadow_attn)) * 0.3183098733425140380859375f.xxx) + frame_constants_sky_high_color.xyz;
    out_var_C1 = uv_final;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_wing = stage_input.in_var_is_wing;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _critters_butterfly.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
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
