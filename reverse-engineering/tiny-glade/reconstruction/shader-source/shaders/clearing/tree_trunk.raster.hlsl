// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/clearing/tree_trunk.raster.hlsl
// Evidence: _clearing_tree_trunk.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/rocky_terrain_dims/wind_strength/
//   is_firstperson_mode/max_cone_cull_dist_range/bark_*_dims), nani_instance_data t16,
//   instance_indirection_buffer t17, frame_constants. (ps): sampler_llr s1, sampler_default s9,
//   rocky_terrain t15, bark_color t18, bark_color_naked t19, bark_lines t20, frame_constants.
//   vs_main: placement transform with trunk sway (two stacked sine bends + vertical wind),
//   growth-age blend from anim_t_packed. ps_main: first-person cone-cull, bark color/naked
//   variant blend, bark-lines detail, packed normal+albedo with a bark flags byte.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// color_library.hlsl guard-skipped (not bound by this shader).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"

Texture2D<float4> rocky_terrain : register(t15, space0);
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/math.hlsl"
#include "inc/perlin.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "clearing/inc/canopy_common.inc.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/texture_blend.hlsl"

// $Globals members (implicit cbuffer at set0/b13).
uint2 rocky_terrain_dims;
float wind_strength;
int is_firstperson_mode;
float max_cone_cull_dist_range;
uint2 bark_color_dims;
uint2 bark_color_naked_dims;
uint2 bark_lines_dims;

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer instance_indirection_buffer : register(t17, space0);
Texture2D<float4> bark_color : register(t18, space0);
Texture2D<float4> bark_color_naked : register(t19, space0);
Texture2D<float4> bark_lines : register(t20, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float3 vertex_color: Vertex_Color;
    [[vk::location(2)]] float3 vertex_normal: Vertex_Normal;
    [[vk::location(3)]] float3 appear_pos: appear_pos;
    [[vk::location(4)]] float age: age;
};

struct VS_OUT
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
    nointerpolation uint C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    uint indir = instance_indirection_buffer.Load<uint>(instance_id * 4u + 0);
    float growth = sqrt(clamp((1.0f - i.age) + lerp(-1.0f, 1.0f, clamp(f16tof32(nani_instance_data.Load<uint>(indir * 48u + 32) & 0xFFFFu), 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 pos = lerp(i.appear_pos, i.vertex_position, growth);

    float wind_amp = pos.y * 0.02f;
    float t_sec = float(game_time_sec);
    float t_frac = game_time_frac;
    float x_amp = 0.2f * pos.z;
    float sway_x = pos.x + ((wind_amp * sin(fmod(t_sec * 0.823529f, 6.2831855f) + (t_frac * 0.823529f) + (0.2f * pos.x)))
        * sin(fmod(t_sec * 0.893617f, 6.2831855f) + (t_frac * 0.893617f) + x_amp));
    float sway_z = pos.z + ((wind_amp * sin(fmod(t_sec * 0.792453f, 6.2831855f) + (t_frac * 0.792453f) + (0.2f * sway_x)))
        * sin(fmod(t_sec * 0.688525f, 6.2831855f) + (t_frac * 0.688525f) + x_amp));

    uint idx = uint(int(indir));
    float rot = radians(nani_instance_data.Load<float>(idx * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float px = nani_instance_data.Load<float3>(idx * 48u + 0).x;
    float py = nani_instance_data.Load<float3>(idx * 48u + 0).y;
    float pz = nani_instance_data.Load<float3>(idx * 48u + 0).z;
    float scale = nani_instance_data.Load<float>(idx * 48u + 12);
    float4 placed = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                 float4(0.0f, 1.0f, 0.0f, py),
                                 float4(0.0f, 0.0f, 1.0f, pz),
                                 float4(0.0f, 0.0f, 0.0f, 1.0f)),
                        mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                               float4(0.0f, scale, 0.0f, 0.0f),
                                               float4(0.0f, 0.0f, scale, 0.0f),
                                               float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(sway_x, pos.y, sway_z, 1.0f))));

    float2 bend = nani_instance_data.Load<float2>(indir * 48u + 36);
    float2 bent = placed.xz + (bend * ((clamp(i.vertex_position.y * 0.1f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, i.age)));
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(bent.x, placed.y, bent.y, 1.0f));
    o.C1 = i.vertex_color;
    o.C2 = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
    o.C3 = float4(bent.x, placed.y, bent.y, placed.w).xyz;
    o.C4 = growth;
    o.C5 = f16tof32((nani_instance_data.Load<uint>(indir * 48u + 32) >> 16u) & 0xFFFFu);
    o.C6 = int(uint(nani_instance_data.Load<int>(idx * 48u + 20)));
    o.C7 = uint(nani_instance_data.Load<int>(indir * 48u + 44));
    o.C8 = i.vertex_position.y;
    return o;
}

struct PS_IN
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    nointerpolation uint C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    if (i.C4 < 0.3f)
    {
        discard;
    }
    float4 view4 = mul(view_constants[current_view_index].world_to_view, float4(i.C3, 1.0f));
    float3 view_pos = view4.xyz;

    float wind_cone;
    if ((render_overrides & 2u) != 0u)
    {
        wind_cone = i.C5 * smoothstep(0.0f, 6.0f, i.C8);
    }
    else
    {
        wind_cone = 0.0f;
    }
    bool naked = (i.C7 == 3u);
    bool cone_active = (int(wind_cone > 0.0f) == 1) && (is_firstperson_mode == 0);
    if (cone_active && !naked)
    {
        view_pos.x = view4.x * 0.769230782985687255859375f;
        view_pos.y = view4.y * 1.1f;
        float2 cone_range = float2(6.66f, max_cone_cull_dist_range) * view_constants[0].view_to_clip[1].y;
        float dist = -view4.z;
        if (dist < cone_range.y)
        {
            float2 cull_cell1 = i.C3.zy * 3.0f;
            int2 cell1 = int2(floor(cull_cell1));
            float2 frac1 = frac(cull_cell1);
            float2 smooth1 = frac1 * frac1 * (3.0f - (2.0f * frac1));
            float n1 = ((1.0f - smooth1.y) * ((incb_perlin_hash01(hash(uint2(cell1 + int2(0, 0)))) * (1.0f - smooth1.x)) + (incb_perlin_hash01(hash(uint2(cell1 + int2(1, 0)))) * smooth1.x)))
                + ((incb_perlin_hash01(hash(uint2(cell1 + int2(0, 1)))) * (1.0f - smooth1.x)) * smooth1.y)
                + ((incb_perlin_hash01(hash(uint2(cell1 + int2(1, 1)))) * smooth1.x) * smooth1.y);
            float2 cull_cell2 = i.C3.xy * 3.0f;
            int2 cell2 = int2(floor(cull_cell2));
            float2 frac2 = frac(cull_cell2);
            float2 smooth2 = frac2 * frac2 * (3.0f - (2.0f * frac2));
            float n2 = ((1.0f - smooth2.y) * ((incb_perlin_hash01(hash(uint2(cell2 + int2(0, 0)))) * (1.0f - smooth2.x)) + (incb_perlin_hash01(hash(uint2(cell2 + int2(1, 0)))) * smooth2.x)))
                + ((incb_perlin_hash01(hash(uint2(cell2 + int2(0, 1)))) * (1.0f - smooth2.x)) * smooth2.y)
                + ((incb_perlin_hash01(hash(uint2(cell2 + int2(1, 1)))) * smooth2.x) * smooth2.y);
            float3 offset = float3(((n1 * 2.0f) - 1.0f), 0.0f, ((n2 * 2.0f) - 1.0f) * 2.0f);
            float2 jitter = (frac(float2(4.27438831329345703125f, 3.349201679229736328125f)) - 0.5f) * 2.0f;
            float3 jittered = view_pos + float3(offset.x + jitter.x, offset.y, offset.z + jitter.y);
            jittered.z = min(jittered.z, -10.0f);
            float f = abs(view_constants[0].clip_to_view[1].y);
            bool small_f = f < 1.0f;
            float inv_f = small_f ? f : (1.0f / f);
            float inv_f2 = inv_f * inv_f;
            float atan_f = (1.0f + (((-0.301895f) + (0.087293f * inv_f2)) * inv_f2)) * inv_f;
            float half_angle = small_f ? atan_f : (1.570796f - atan_f);
            float angle = (view_constants[0].clip_to_view[1].y < 0.0f) ? (-half_angle) : half_angle;
            if (dot(float3(0.0f, 0.0f, -1.0f), normalize(jittered)) > cos(((max(0.07f, angle) * 1.2f) * smoothstep(cone_range.y, cone_range.x, dist)) * wind_cone))
            {
                discard;
            }
        }
    }

    float3 n = normalize(i.C2);
    float3 trunk_uv = i.C3 * (1.0f / 30.0f);
    float3 bark_a;
    float3 bark_b;
    if (naked)
    {
        bark_a = bark_color_naked.SampleBias(sampler_llr, trunk_uv.xy, global_mip_bias).xyz;
        bark_b = bark_color_naked.SampleBias(sampler_llr, trunk_uv.zy, global_mip_bias).xyz;
    }
    else
    {
        bark_a = bark_color.SampleBias(sampler_llr, trunk_uv.xy, global_mip_bias).xyz;
        bark_b = bark_color.SampleBias(sampler_llr, trunk_uv.zy, global_mip_bias).xyz;
    }
    float3 n_abs = pow(abs(n), 1.0f);
    float lum_a = dot(bark_a, float3(0.2126f, 0.7152f, 0.0722f)) + n_abs.z;
    float lum_b = dot(bark_b, float3(0.2126f, 0.7152f, 0.0722f)) + n_abs.x;
    float min_lum = max(lum_a, lum_b) - 0.2f;
    float wa = max(lum_a - min_lum, 0.0f);
    float wb = max(lum_b - min_lum, 0.0f);
    float3 bark = ((bark_a * wa) + (bark_b * wb)) / (wa + wb);

    float3 tint;
    if (glade_theme_id == 2u)
    {
        tint = float3(0.15f, 0.09f, 0.04f);
    }
    else if (glade_theme_id == 4u)
    {
        tint = lerp(bark, 1.0f, ((1.0f - smoothstep(-0.5f, 4.0f, i.C8)) * 0.3f) * smoothstep(-0.1f, 0.0f, i.C3.y));
    }
    else
    {
        tint = float3(0.06f, 0.1f, 0.03f);
    }

    float2 trunk_uv2 = (i.C3.xz * (1.0f / 130.0f)) + 0.5f;
    bool in_bounds = !((trunk_uv2.x < 0.0f) || (trunk_uv2.x > 1.0f) || (trunk_uv2.y < 0.0f) || (trunk_uv2.y > 1.0f));
    float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, trunk_uv2, 0.0f).x : 0.0f;
    float3 bark_blend = lerp(bark, tint, ((1.0f - smoothstep(-0.1f, 0.4f, i.C1.y)) * clamp(1.0f - rocky, 0.7f, 1.0f)));
    float4 lines = bark_lines.SampleBias(sampler_llr, i.C1.xy * 1.2f, global_mip_bias);
    float3 bark_detail = lerp(bark_blend, lerp(bark_blend, tint, 0.5f) * 0.6f, 1.0f - lines.xyz);
    bool naked_color = naked && (i.C1.x <= (-99.0f));
    float3 bark_final = naked_color ? float3(0.25f, 0.15f, 0.0375f) : bark_detail;
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(bark_final, 0.7f, (smoothstep(-0.1f, 0.0f, i.C3.y) * smoothstep(0.2f, 1.0f, clamp(n.y, 0.0f, 1.0f))));
    }
    else
    {
        albedo = bark_final;
    }

    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint normal_packed = (uint(clamp(packed_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xE6000000u;
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    albedo_packed |= ((7u | (uint(naked_color ? false : true) << 6u)) << 24u);
    o.target = uint2(normal_packed, albedo_packed);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_tree_trunk.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct PlacementData
{
    float3 position;
    float scale;
    float rotation_angle;
    int id;
    uint custom0;
    uint custom1;
    uint anim_t_packed;
    float2 bend_vector;
    int pad0;
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
    uint2 rocky_terrain_dims : packoffset(c0.y);
    float wind_strength : packoffset(c0.w);
    int is_firstperson_mode : packoffset(c1);
    float max_cone_cull_dist_range : packoffset(c1.y);
    uint2 bark_color_dims : packoffset(c1.z);
    uint2 bark_color_naked_dims : packoffset(c2);
    uint2 bark_lines_dims : packoffset(c2.z);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer instance_indirection_buffer : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float3 in_var_appear_pos;
static float in_var_age;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float out_var_C4;
static float out_var_C5;
static int out_var_C6;
static uint out_var_C7;
static float out_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float3 in_var_appear_pos: appear_pos;
    [[vk::location(4)]] float in_var_age: age;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation int out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    float _141 = sqrt(clamp((1.0f - in_var_age) + lerp(-1.0f, 1.0f, clamp(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) & 65535u).x, 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 _143 = lerp(in_var_appear_pos, in_var_Vertex_Position, _141.xxx);
    float _144 = _143.y;
    float _145 = _144 * 0.0199999995529651641845703125f;
    float _150 = float(frame_constants_game_time_sec);
    float _155 = _143.x;
    float _164 = _143.z;
    float _165 = 0.20000000298023223876953125f * _164;
    float _169 = _155 + ((_145 * sin((fmod(_150 * 0.8235290050506591796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.8235290050506591796875f)) + (0.20000000298023223876953125f * _155))) * sin((fmod(_150 * 0.893616974353790283203125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.893616974353790283203125f)) + _165));
    uint _187 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    float _194 = radians(nani_instance_data.Load<float>(_187 * 48 + 16));
    float3 _198 = normalize(float3(cos(_194), 0.0f, sin(_194)));
    float4x4 _221 = transpose(float4x4(float4(_198, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(normalize(cross(_198, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _225 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).x), float4(0.0f, 1.0f, 0.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).y), float4(0.0f, 0.0f, 1.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_221, mul(float4x4(float4(nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f, 0.0f, 0.0f), float4(0.0f, nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f, 0.0f), float4(0.0f, 0.0f, nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_169, _144, _164 + ((_145 * sin((fmod(_150 * 0.792452991008758544921875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.792452991008758544921875f)) + (0.20000000298023223876953125f * _169))) * sin((fmod(_150 * 0.688525021076202392578125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.688525021076202392578125f)) + _165)), 1.0f))));
    float2 _242 = _225.xz + (nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 36) * ((clamp(in_var_Vertex_Position.y * 0.100000001490116119384765625f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, in_var_age)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_242.x, _225.y, _242.y, 1.0f));
    out_var_C1 = in_var_Vertex_Color;
    out_var_C2 = mul(_221, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C3 = float4(_242.x, _225.y, _242.y, _225.w).xyz;
    out_var_C4 = _141;
    out_var_C5 = spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) >> 16u) & 65535u).x;
    out_var_C6 = int(uint(nani_instance_data.Load<int>(_187 * 48 + 20)));
    out_var_C7 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 44));
    out_var_C8 = in_var_Vertex_Position.y;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_appear_pos = stage_input.in_var_appear_pos;
    in_var_age = stage_input.in_var_age;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_tree_trunk.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint current_view_index : packoffset(c0);
    uint2 rocky_terrain_dims : packoffset(c0.y);
    float wind_strength : packoffset(c0.w);
    int is_firstperson_mode : packoffset(c1);
    float max_cone_cull_dist_range : packoffset(c1.y);
    uint2 bark_color_dims : packoffset(c1.z);
    uint2 bark_color_naked_dims : packoffset(c2);
    uint2 bark_lines_dims : packoffset(c2.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> rocky_terrain : register(t15, space0);
Texture2D<float4> bark_color : register(t18, space0);
Texture2D<float4> bark_color_naked : register(t19, space0);
Texture2D<float4> bark_lines : register(t20, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float in_var_C4;
static float in_var_C5;
static uint in_var_C7;
static float in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] nointerpolation uint in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (in_var_C4 < 0.300000011920928955078125f)
    {
        discard;
    }
    float4 _192 = mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C3, 1.0f));
    float3 _193 = _192.xyz;
    float _203;
    if ((frame_constants_render_overrides & 2u) != 0u)
    {
        _203 = in_var_C5 * smoothstep(0.0f, 6.0f, in_var_C8);
    }
    else
    {
        _203 = 0.0f;
    }
    bool _205 = in_var_C7 == 3u;
    bool _213;
    if (int(_203 > 0.0f) == 1)
    {
        _213 = is_firstperson_mode == 0;
    }
    else
    {
        _213 = false;
    }
    bool _217;
    if (_213)
    {
        _217 = !_205;
    }
    else
    {
        _217 = false;
    }
    if (_217)
    {
        _193.x = _192.x * 0.769230782985687255859375f;
        _193.y = _192.y * 1.10000002384185791015625f;
        float2 _232 = float2(6.659999847412109375f, max_cone_cull_dist_range) * frame_constants_view_constants[0u].view_to_clip[1].y;
        float _234 = -_192.z;
        float _235 = _232.y;
        if (_234 < _235)
        {
            float2 _240 = in_var_C3.zy * 3.0f;
            int2 _242 = int2(floor(_240));
            float2 _243 = frac(_240);
            uint2 _244 = uint2(_242);
            uint _246 = _244.y;
            uint _248 = _246 + (_246 << 10u);
            uint _250 = _248 ^ (_248 >> 6u);
            uint _252 = _250 + (_250 << 3u);
            uint _254 = _252 ^ (_252 >> 11u);
            uint _260 = (((_244.x * 1664525u) + (_254 + (_254 << 15u))) + 1013904223u) * 1664525u;
            uint _262 = _260 ^ (_260 >> 11u);
            uint _265 = _262 ^ ((_262 << 7u) & 2636928640u);
            uint _268 = _265 ^ ((_265 << 15u) & 4022730752u);
            uint2 _276 = uint2(_242 + int2(1, 0));
            uint _278 = _276.y;
            uint _280 = _278 + (_278 << 10u);
            uint _282 = _280 ^ (_280 >> 6u);
            uint _284 = _282 + (_282 << 3u);
            uint _286 = _284 ^ (_284 >> 11u);
            uint _292 = (((_276.x * 1664525u) + (_286 + (_286 << 15u))) + 1013904223u) * 1664525u;
            uint _294 = _292 ^ (_292 >> 11u);
            uint _297 = _294 ^ ((_294 << 7u) & 2636928640u);
            uint _300 = _297 ^ ((_297 << 15u) & 4022730752u);
            uint2 _308 = uint2(_242 + int2(0, 1));
            uint _310 = _308.y;
            uint _312 = _310 + (_310 << 10u);
            uint _314 = _312 ^ (_312 >> 6u);
            uint _316 = _314 + (_314 << 3u);
            uint _318 = _316 ^ (_316 >> 11u);
            uint _324 = (((_308.x * 1664525u) + (_318 + (_318 << 15u))) + 1013904223u) * 1664525u;
            uint _326 = _324 ^ (_324 >> 11u);
            uint _329 = _326 ^ ((_326 << 7u) & 2636928640u);
            uint _332 = _329 ^ ((_329 << 15u) & 4022730752u);
            uint2 _340 = uint2(_242 + int2(1, 1));
            uint _342 = _340.y;
            uint _344 = _342 + (_342 << 10u);
            uint _346 = _344 ^ (_344 >> 6u);
            uint _348 = _346 + (_346 << 3u);
            uint _350 = _348 ^ (_348 >> 11u);
            uint _356 = (((_340.x * 1664525u) + (_350 + (_350 << 15u))) + 1013904223u) * 1664525u;
            uint _358 = _356 ^ (_356 >> 11u);
            uint _361 = _358 ^ ((_358 << 7u) & 2636928640u);
            uint _364 = _361 ^ ((_361 << 15u) & 4022730752u);
            float2 _374 = (_243 * _243) * (3.0f.xx - (_243 * 2.0f));
            float _375 = _374.x;
            float _376 = 1.0f - _375;
            float _378 = _374.y;
            float2 _392 = in_var_C3.xy * 3.0f;
            int2 _394 = int2(floor(_392));
            float2 _395 = frac(_392);
            uint2 _396 = uint2(_394);
            uint _398 = _396.y;
            uint _400 = _398 + (_398 << 10u);
            uint _402 = _400 ^ (_400 >> 6u);
            uint _404 = _402 + (_402 << 3u);
            uint _406 = _404 ^ (_404 >> 11u);
            uint _412 = (((_396.x * 1664525u) + (_406 + (_406 << 15u))) + 1013904223u) * 1664525u;
            uint _414 = _412 ^ (_412 >> 11u);
            uint _417 = _414 ^ ((_414 << 7u) & 2636928640u);
            uint _420 = _417 ^ ((_417 << 15u) & 4022730752u);
            uint2 _428 = uint2(_394 + int2(1, 0));
            uint _430 = _428.y;
            uint _432 = _430 + (_430 << 10u);
            uint _434 = _432 ^ (_432 >> 6u);
            uint _436 = _434 + (_434 << 3u);
            uint _438 = _436 ^ (_436 >> 11u);
            uint _444 = (((_428.x * 1664525u) + (_438 + (_438 << 15u))) + 1013904223u) * 1664525u;
            uint _446 = _444 ^ (_444 >> 11u);
            uint _449 = _446 ^ ((_446 << 7u) & 2636928640u);
            uint _452 = _449 ^ ((_449 << 15u) & 4022730752u);
            uint2 _460 = uint2(_394 + int2(0, 1));
            uint _462 = _460.y;
            uint _464 = _462 + (_462 << 10u);
            uint _466 = _464 ^ (_464 >> 6u);
            uint _468 = _466 + (_466 << 3u);
            uint _470 = _468 ^ (_468 >> 11u);
            uint _476 = (((_460.x * 1664525u) + (_470 + (_470 << 15u))) + 1013904223u) * 1664525u;
            uint _478 = _476 ^ (_476 >> 11u);
            uint _481 = _478 ^ ((_478 << 7u) & 2636928640u);
            uint _484 = _481 ^ ((_481 << 15u) & 4022730752u);
            uint2 _492 = uint2(_394 + int2(1, 1));
            uint _494 = _492.y;
            uint _496 = _494 + (_494 << 10u);
            uint _498 = _496 ^ (_496 >> 6u);
            uint _500 = _498 + (_498 << 3u);
            uint _502 = _500 ^ (_500 >> 11u);
            uint _508 = (((_492.x * 1664525u) + (_502 + (_502 << 15u))) + 1013904223u) * 1664525u;
            uint _510 = _508 ^ (_508 >> 11u);
            uint _513 = _510 ^ ((_510 << 7u) & 2636928640u);
            uint _516 = _513 ^ ((_513 << 15u) & 4022730752u);
            float2 _526 = (_395 * _395) * (3.0f.xx - (_395 * 2.0f));
            float _527 = _526.x;
            float _528 = 1.0f - _527;
            float _530 = _526.y;
            float3 _544 = float3((((((1.0f - _378) * (((asfloat(((_268 ^ (_268 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _376) + ((asfloat(((_300 ^ (_300 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _375))) + (((asfloat(((_332 ^ (_332 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _376) * _378)) + (((asfloat(((_364 ^ (_364 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _375) * _378)) * 2.0f) - 1.0f, 0.0f, (((((1.0f - _530) * (((asfloat(((_420 ^ (_420 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _528) + ((asfloat(((_452 ^ (_452 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _527))) + (((asfloat(((_484 ^ (_484 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _528) * _530)) + (((asfloat(((_516 ^ (_516 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _527) * _530)) * 2.0f) - 1.0f) * 2.0f;
            float2 _549 = _544.xz + ((frac(float2(4.27438831329345703125f, 3.349201679229736328125f)) - 0.5f.xx) * 2.0f);
            float3 _551 = _193 + float3(_549.x, _544.y, _549.y);
            _551.z = min(_551.z, -10.0f);
            float _559 = abs(frame_constants_view_constants[0u].clip_to_view[1].y);
            bool _560 = _559 < 1.0f;
            float _565;
            if (_560)
            {
                _565 = _559;
            }
            else
            {
                _565 = 1.0f / _559;
            }
            float _566 = _565 * _565;
            float _571 = (1.0f + (((-0.3018949925899505615234375f) + (0.087292902171611785888671875f * _566)) * _566)) * _565;
            float _576;
            if (_560)
            {
                _576 = _571;
            }
            else
            {
                _576 = 1.57079601287841796875f - _571;
            }
            float _582;
            if (frame_constants_view_constants[0u].clip_to_view[1].y < 0.0f)
            {
                _582 = -_576;
            }
            else
            {
                _582 = _576;
            }
            if (dot(float3(0.0f, 0.0f, -1.0f), normalize(_551)) > cos(((max(0.070000000298023223876953125f, _582) * 1.2000000476837158203125f) * smoothstep(_235, _232.x, _234)) * _203))
            {
                discard;
            }
        }
    }
    float3 _594 = normalize(in_var_C2);
    float3 _595 = in_var_C3 * 0.0333333350718021392822265625f.xxx;
    float3 _621;
    float3 _622;
    if (_205)
    {
        _621 = bark_color_naked.SampleBias(sampler_llr, _595.xy, frame_constants_global_mip_bias).xyz;
        _622 = bark_color_naked.SampleBias(sampler_llr, _595.zy, frame_constants_global_mip_bias).xyz;
    }
    else
    {
        _621 = bark_color.SampleBias(sampler_llr, _595.xy, frame_constants_global_mip_bias).xyz;
        _622 = bark_color.SampleBias(sampler_llr, _595.zy, frame_constants_global_mip_bias).xyz;
    }
    float3 _624 = pow(abs(_594), 1.0f.xxx);
    float _629 = dot(_622, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + _624.x;
    float _630 = dot(_621, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + _624.z;
    float _632 = max(_629, _630) - 0.20000000298023223876953125f;
    float _634 = max(_629 - _632, 0.0f);
    float _636 = max(_630 - _632, 0.0f);
    float3 _642 = ((_622 * _634) + (_621 * _636)) / (_634 + _636).xxx;
    float3 _661;
    if (frame_constants_glade_theme_id == 2u)
    {
        _661 = float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.039999999105930328369140625f);
    }
    else
    {
        float3 _660;
        if (frame_constants_glade_theme_id == 4u)
        {
            _660 = lerp(_642, 1.0f.xxx, (((1.0f - smoothstep(-0.5f, 4.0f, in_var_C8)) * 0.300000011920928955078125f) * smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y)).xxx);
        }
        else
        {
            _660 = float3(0.0599999986588954925537109375f, 0.100000001490116119384765625f, 0.02999999932944774627685546875f);
        }
        _661 = _660;
    }
    float _694;
    do
    {
        float2 _666 = (in_var_C3.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float _667 = _666.x;
        bool _673;
        if (!(_667 < 0.0f))
        {
            _673 = _667 > 1.0f;
        }
        else
        {
            _673 = true;
        }
        bool _679;
        if (!_673)
        {
            _679 = _666.y < 0.0f;
        }
        else
        {
            _679 = true;
        }
        bool _685;
        if (!_679)
        {
            _685 = _666.y > 1.0f;
        }
        else
        {
            _685 = true;
        }
        if (_685)
        {
            _694 = 0.0f;
            break;
        }
        else
        {
            _694 = rocky_terrain.SampleLevel(sampler_default, _666, 0.0f).x;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float3 _702 = lerp(_642, _661, ((1.0f - smoothstep(-0.100000001490116119384765625f, 0.4000000059604644775390625f, in_var_C1.y)) * clamp(1.0f - _694, 0.699999988079071044921875f, 1.0f)).xxx);
    float4 _710 = bark_lines.SampleBias(sampler_llr, in_var_C1.xy * 1.2000000476837158203125f, frame_constants_global_mip_bias);
    float3 _713 = lerp(_702, lerp(_702, _661, 0.5f.xxx) * 0.60000002384185791015625f, 1.0f.xxx - _710.xyz);
    bool _718;
    if (_205)
    {
        _718 = in_var_C1.x <= (-99.0f);
    }
    else
    {
        _718 = false;
    }
    bool3 _720 = _718.xxx;
    float3 _721 = float3(_720.x ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).x : _713.x, _720.y ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).y : _713.y, _720.z ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).z : _713.z);
    float3 _732;
    if (frame_constants_glade_theme_id == 4u)
    {
        _732 = lerp(_721, 0.699999988079071044921875f.xxx, (smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y) * smoothstep(0.20000000298023223876953125f, 1.0f, clamp(_594.y, 0.0f, 1.0f))).xxx);
    }
    else
    {
        _732 = _721;
    }
    float3 _734 = (_594 * 0.5f) + 0.5f.xxx;
    float3 _756 = sqrt(clamp(_732, 0.0f.xxx, 1.0f.xxx));
    uint _775 = (uint((clamp(_756.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_756.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_756.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _776 = uint2(((uint((clamp(_734.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_734.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_734.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3858759680u, _775);
    _776.y = _775 | ((7u | (uint(_718 ? false : true) << 6u)) << 24u);
    out_var_SV_TARGET0 = _776;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_tree_trunk.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct PlacementData
{
    float3 position;
    float scale;
    float rotation_angle;
    int id;
    uint custom0;
    uint custom1;
    uint anim_t_packed;
    float2 bend_vector;
    int pad0;
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
    uint2 rocky_terrain_dims : packoffset(c0.y);
    float wind_strength : packoffset(c0.w);
    int is_firstperson_mode : packoffset(c1);
    float max_cone_cull_dist_range : packoffset(c1.y);
    uint2 bark_color_dims : packoffset(c1.z);
    uint2 bark_color_naked_dims : packoffset(c2);
    uint2 bark_lines_dims : packoffset(c2.z);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer instance_indirection_buffer : register(t17, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float3 in_var_appear_pos;
static float in_var_age;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float out_var_C4;
static float out_var_C5;
static int out_var_C6;
static uint out_var_C7;
static float out_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float3 in_var_appear_pos: appear_pos;
    [[vk::location(4)]] float in_var_age: age;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation int out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    float _141 = sqrt(clamp((1.0f - in_var_age) + lerp(-1.0f, 1.0f, clamp(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) & 65535u).x, 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 _143 = lerp(in_var_appear_pos, in_var_Vertex_Position, _141.xxx);
    float _144 = _143.y;
    float _145 = _144 * 0.0199999995529651641845703125f;
    float _150 = float(frame_constants_game_time_sec);
    float _155 = _143.x;
    float _164 = _143.z;
    float _165 = 0.20000000298023223876953125f * _164;
    float _169 = _155 + ((_145 * sin((fmod(_150 * 0.8235290050506591796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.8235290050506591796875f)) + (0.20000000298023223876953125f * _155))) * sin((fmod(_150 * 0.893616974353790283203125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.893616974353790283203125f)) + _165));
    uint _187 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    float _194 = radians(nani_instance_data.Load<float>(_187 * 48 + 16));
    float3 _198 = normalize(float3(cos(_194), 0.0f, sin(_194)));
    float4x4 _221 = transpose(float4x4(float4(_198, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(normalize(cross(_198, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _225 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).x), float4(0.0f, 1.0f, 0.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).y), float4(0.0f, 0.0f, 1.0f, nani_instance_data.Load<float3>(_187 * 48 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(_221, mul(float4x4(float4(nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f, 0.0f, 0.0f), float4(0.0f, nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f, 0.0f), float4(0.0f, 0.0f, nani_instance_data.Load<float>(_187 * 48 + 12), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_169, _144, _164 + ((_145 * sin((fmod(_150 * 0.792452991008758544921875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.792452991008758544921875f)) + (0.20000000298023223876953125f * _169))) * sin((fmod(_150 * 0.688525021076202392578125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.688525021076202392578125f)) + _165)), 1.0f))));
    float2 _242 = _225.xz + (nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 36) * ((clamp(in_var_Vertex_Position.y * 0.100000001490116119384765625f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, in_var_age)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(_242.x, _225.y, _242.y, 1.0f));
    out_var_C1 = in_var_Vertex_Color;
    out_var_C2 = mul(_221, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C3 = float4(_242.x, _225.y, _242.y, _225.w).xyz;
    out_var_C4 = _141;
    out_var_C5 = spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) >> 16u) & 65535u).x;
    out_var_C6 = int(uint(nani_instance_data.Load<int>(_187 * 48 + 20)));
    out_var_C7 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 44));
    out_var_C8 = in_var_Vertex_Position.y;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_appear_pos = stage_input.in_var_appear_pos;
    in_var_age = stage_input.in_var_age;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_tree_trunk.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float in_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float in_var_C4 : TEXCOORD4;
};

void frag_main()
{
    if (in_var_C4 < 0.300000011920928955078125f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
