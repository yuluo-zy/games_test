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
// 【文件用途】树干/树身光栅化。VS 用 anim_t 出现动画(growth=sqrt 混合 appear_pos),
//   双层正弦风摆(trunk sway)+ bend_vector 弯折;PS 做生长阈值剔除、第一人称风锥剔除
//   (两路 2D 值噪声抖动),bark_color/bark_color_naked 双投影混合树皮色,按主题
//   tint,岩石/树线细节,打包 (法线, 反照率) uint2 + bark 标志字节。
// 【入口】vs_main/ps_main | 绑定:rocky_terrain t15、nani_instance_data t16、
//   instance_indirection_buffer t17、bark_color t18、bark_color_naked t19、
//   bark_lines t20、sampler_llr s1、sampler_default s9、$Globals b13、
//   frame_constants b0/space1。
// 【变体】b903+f0ad 的四个 (variant,stage) 组合均用提起段;手写段仅作参考基线。
// 重写说明:手写重建段补充中文注释;提起段(TGV_*)逐字保留,仅加块级中文注释。

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

// VS:growth 出现动画 -> 双层风摆 -> T*R*S 实例变换 -> bend_vector 弯折 -> 输出
VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
// 经 indirection 取实例;growth = sqrt(clamp(age + anim_t 混合)),控制出现进度
    uint indir = instance_indirection_buffer.Load<uint>(instance_id * 4u + 0);
    float growth = sqrt(clamp((1.0f - i.age) + lerp(-1.0f, 1.0f, clamp(f16tof32(nani_instance_data.Load<uint>(indir * 48u + 32) & 0xFFFFu), 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 pos = lerp(i.appear_pos, i.vertex_position, growth);

// 双层正弦风摆:游戏时间两路频率调制 x/z,振幅随顶点高 y*0.02
    float wind_amp = pos.y * 0.02f;
    float t_sec = float(game_time_sec);
    float t_frac = game_time_frac;
    float x_amp = 0.2f * pos.z;
    float sway_x = pos.x + ((wind_amp * sin(fmod(t_sec * 0.823529f, 6.2831855f) + (t_frac * 0.823529f) + (0.2f * pos.x)))
        * sin(fmod(t_sec * 0.893617f, 6.2831855f) + (t_frac * 0.893617f) + x_amp));
    float sway_z = pos.z + ((wind_amp * sin(fmod(t_sec * 0.792453f, 6.2831855f) + (t_frac * 0.792453f) + (0.2f * sway_x)))
        * sin(fmod(t_sec * 0.688525f, 6.2831855f) + (t_frac * 0.688525f) + x_amp));

// 实例变换:平移 * 旋转基 * 缩放(48 字节步长:pos0/scale12/rot16/id20)
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

// bend_vector(偏移 36)随顶点高 clamp(y*0.1)*0.75 与 age lerp 增强,弯折 xz
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

// PS:生长阈值剔除 -> 第一人称风锥剔除 -> 树皮颜色/主题 tint/岩石/细节 -> 打包
PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    if (i.C4 < 0.3f)
    {
        discard;
    }
// 世界->视图,供风锥剔除的视角距离/角度
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
// C7==3:裸树干(无树皮);风锥剔除只在非裸 + 第一人称下生效
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
// 风锥剔除:两路 2D 值噪声(原点*3 格点 LCG 哈希)抖动视锥角度,超锥 discard
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

// 树皮色:两个投影 uv 分别采样,按法线主导轴亮度加权混合
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

// 主题着色:theme 2 深棕、theme 4 雪向白淡化、默认青绿 tint
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

// 岩石掩码(世界/130+0.5):在岩石上时树皮淡向 tint
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

// 打包:法线 RGB8 + 0xE6 标志;反照率 sqrt 反 gamma + (7|naked<<6) 标志字节
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
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价(出现动画/风摆/
//   变换/bend);PS 为风锥剔除 + 树皮混合 + G-Buffer 打包。VS 与 f0ad VS 逐字节一致。
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
    float v141 = sqrt(clamp((1.0f - in_var_age) + lerp(-1.0f, 1.0f, clamp(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) & 65535u).x, 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 v143 = lerp(in_var_appear_pos, in_var_Vertex_Position, v141.xxx);
    float v144 = v143.y;
    float v145 = v144 * 0.0199999995529651641845703125f;
    float v150 = float(frame_constants_game_time_sec);
    float v155 = v143.x;
    float v164 = v143.z;
    float v165 = 0.20000000298023223876953125f * v164;
    float v169 = v155 + ((v145 * sin((fmod(v150 * 0.8235290050506591796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.8235290050506591796875f)) + (0.20000000298023223876953125f * v155))) * sin((fmod(v150 * 0.893616974353790283203125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.893616974353790283203125f)) + v165));
    uint v187 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    float v194 = radians(nani_instance_data.Load<float>(v187 * 48 + 16));
    float3 v198 = normalize(float3(cos(v194), 0.0f, sin(v194)));
    float4x4 v221 = transpose(float4x4(float4(v198, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(normalize(cross(v198, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 v225 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).x), float4(0.0f, 1.0f, 0.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).y), float4(0.0f, 0.0f, 1.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v221, mul(float4x4(float4(nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f, 0.0f, 0.0f), float4(0.0f, nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f, 0.0f), float4(0.0f, 0.0f, nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(v169, v144, v164 + ((v145 * sin((fmod(v150 * 0.792452991008758544921875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.792452991008758544921875f)) + (0.20000000298023223876953125f * v169))) * sin((fmod(v150 * 0.688525021076202392578125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.688525021076202392578125f)) + v165)), 1.0f))));
    float2 v242 = v225.xz + (nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 36) * ((clamp(in_var_Vertex_Position.y * 0.100000001490116119384765625f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, in_var_age)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v242.x, v225.y, v242.y, 1.0f));
    out_var_C1 = in_var_Vertex_Color;
    out_var_C2 = mul(v221, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C3 = float4(v242.x, v225.y, v242.y, v225.w).xyz;
    out_var_C4 = v141;
    out_var_C5 = spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) >> 16u) & 65535u).x;
    out_var_C6 = int(uint(nani_instance_data.Load<int>(v187 * 48 + 20)));
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
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价(出现动画/风摆/
//   变换/bend);PS 为风锥剔除 + 树皮混合 + G-Buffer 打包。VS 与 f0ad VS 逐字节一致。
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
    float4 v192 = mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C3, 1.0f));
    float3 v193 = v192.xyz;
    float v203;
    if ((frame_constants_render_overrides & 2u) != 0u)
    {
        v203 = in_var_C5 * smoothstep(0.0f, 6.0f, in_var_C8);
    }
    else
    {
        v203 = 0.0f;
    }
    bool v205 = in_var_C7 == 3u;
    bool v213;
    if (int(v203 > 0.0f) == 1)
    {
        v213 = is_firstperson_mode == 0;
    }
    else
    {
        v213 = false;
    }
    bool v217;
    if (v213)
    {
        v217 = !v205;
    }
    else
    {
        v217 = false;
    }
    if (v217)
    {
        v193.x = v192.x * 0.769230782985687255859375f;
        v193.y = v192.y * 1.10000002384185791015625f;
        float2 v232 = float2(6.659999847412109375f, max_cone_cull_dist_range) * frame_constants_view_constants[0u].view_to_clip[1].y;
        float v234 = -v192.z;
        float v235 = v232.y;
        if (v234 < v235)
        {
            float2 v240 = in_var_C3.zy * 3.0f;
            int2 v242 = int2(floor(v240));
            float2 v243 = frac(v240);
            uint2 v244 = uint2(v242);
            uint v246 = v244.y;
            uint v248 = v246 + (v246 << 10u);
            uint v250 = v248 ^ (v248 >> 6u);
            uint v252 = v250 + (v250 << 3u);
            uint v254 = v252 ^ (v252 >> 11u);
            uint v260 = (((v244.x * 1664525u) + (v254 + (v254 << 15u))) + 1013904223u) * 1664525u;
            uint v262 = v260 ^ (v260 >> 11u);
            uint v265 = v262 ^ ((v262 << 7u) & 2636928640u);
            uint v268 = v265 ^ ((v265 << 15u) & 4022730752u);
            uint2 v276 = uint2(v242 + int2(1, 0));
            uint v278 = v276.y;
            uint v280 = v278 + (v278 << 10u);
            uint v282 = v280 ^ (v280 >> 6u);
            uint v284 = v282 + (v282 << 3u);
            uint v286 = v284 ^ (v284 >> 11u);
            uint v292 = (((v276.x * 1664525u) + (v286 + (v286 << 15u))) + 1013904223u) * 1664525u;
            uint v294 = v292 ^ (v292 >> 11u);
            uint v297 = v294 ^ ((v294 << 7u) & 2636928640u);
            uint v300 = v297 ^ ((v297 << 15u) & 4022730752u);
            uint2 v308 = uint2(v242 + int2(0, 1));
            uint v310 = v308.y;
            uint v312 = v310 + (v310 << 10u);
            uint v314 = v312 ^ (v312 >> 6u);
            uint v316 = v314 + (v314 << 3u);
            uint v318 = v316 ^ (v316 >> 11u);
            uint v324 = (((v308.x * 1664525u) + (v318 + (v318 << 15u))) + 1013904223u) * 1664525u;
            uint v326 = v324 ^ (v324 >> 11u);
            uint v329 = v326 ^ ((v326 << 7u) & 2636928640u);
            uint v332 = v329 ^ ((v329 << 15u) & 4022730752u);
            uint2 v340 = uint2(v242 + int2(1, 1));
            uint v342 = v340.y;
            uint v344 = v342 + (v342 << 10u);
            uint v346 = v344 ^ (v344 >> 6u);
            uint v348 = v346 + (v346 << 3u);
            uint v350 = v348 ^ (v348 >> 11u);
            uint v356 = (((v340.x * 1664525u) + (v350 + (v350 << 15u))) + 1013904223u) * 1664525u;
            uint v358 = v356 ^ (v356 >> 11u);
            uint v361 = v358 ^ ((v358 << 7u) & 2636928640u);
            uint v364 = v361 ^ ((v361 << 15u) & 4022730752u);
            float2 v374 = (v243 * v243) * (3.0f.xx - (v243 * 2.0f));
            float v375 = v374.x;
            float v376 = 1.0f - v375;
            float v378 = v374.y;
            float2 v392 = in_var_C3.xy * 3.0f;
            int2 v394 = int2(floor(v392));
            float2 v395 = frac(v392);
            uint2 v396 = uint2(v394);
            uint v398 = v396.y;
            uint v400 = v398 + (v398 << 10u);
            uint v402 = v400 ^ (v400 >> 6u);
            uint v404 = v402 + (v402 << 3u);
            uint v406 = v404 ^ (v404 >> 11u);
            uint v412 = (((v396.x * 1664525u) + (v406 + (v406 << 15u))) + 1013904223u) * 1664525u;
            uint v414 = v412 ^ (v412 >> 11u);
            uint v417 = v414 ^ ((v414 << 7u) & 2636928640u);
            uint v420 = v417 ^ ((v417 << 15u) & 4022730752u);
            uint2 v428 = uint2(v394 + int2(1, 0));
            uint v430 = v428.y;
            uint v432 = v430 + (v430 << 10u);
            uint v434 = v432 ^ (v432 >> 6u);
            uint v436 = v434 + (v434 << 3u);
            uint v438 = v436 ^ (v436 >> 11u);
            uint v444 = (((v428.x * 1664525u) + (v438 + (v438 << 15u))) + 1013904223u) * 1664525u;
            uint v446 = v444 ^ (v444 >> 11u);
            uint v449 = v446 ^ ((v446 << 7u) & 2636928640u);
            uint v452 = v449 ^ ((v449 << 15u) & 4022730752u);
            uint2 v460 = uint2(v394 + int2(0, 1));
            uint v462 = v460.y;
            uint v464 = v462 + (v462 << 10u);
            uint v466 = v464 ^ (v464 >> 6u);
            uint v468 = v466 + (v466 << 3u);
            uint v470 = v468 ^ (v468 >> 11u);
            uint v476 = (((v460.x * 1664525u) + (v470 + (v470 << 15u))) + 1013904223u) * 1664525u;
            uint v478 = v476 ^ (v476 >> 11u);
            uint v481 = v478 ^ ((v478 << 7u) & 2636928640u);
            uint v484 = v481 ^ ((v481 << 15u) & 4022730752u);
            uint2 v492 = uint2(v394 + int2(1, 1));
            uint v494 = v492.y;
            uint v496 = v494 + (v494 << 10u);
            uint v498 = v496 ^ (v496 >> 6u);
            uint v500 = v498 + (v498 << 3u);
            uint v502 = v500 ^ (v500 >> 11u);
            uint v508 = (((v492.x * 1664525u) + (v502 + (v502 << 15u))) + 1013904223u) * 1664525u;
            uint v510 = v508 ^ (v508 >> 11u);
            uint v513 = v510 ^ ((v510 << 7u) & 2636928640u);
            uint v516 = v513 ^ ((v513 << 15u) & 4022730752u);
            float2 v526 = (v395 * v395) * (3.0f.xx - (v395 * 2.0f));
            float v527 = v526.x;
            float v528 = 1.0f - v527;
            float v530 = v526.y;
            float3 v544 = float3((((((1.0f - v378) * (((asfloat(((v268 ^ (v268 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v376) + ((asfloat(((v300 ^ (v300 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v375))) + (((asfloat(((v332 ^ (v332 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v376) * v378)) + (((asfloat(((v364 ^ (v364 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v375) * v378)) * 2.0f) - 1.0f, 0.0f, (((((1.0f - v530) * (((asfloat(((v420 ^ (v420 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v528) + ((asfloat(((v452 ^ (v452 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v527))) + (((asfloat(((v484 ^ (v484 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v528) * v530)) + (((asfloat(((v516 ^ (v516 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v527) * v530)) * 2.0f) - 1.0f) * 2.0f;
            float2 v549 = v544.xz + ((frac(float2(4.27438831329345703125f, 3.349201679229736328125f)) - 0.5f.xx) * 2.0f);
            float3 v551 = v193 + float3(v549.x, v544.y, v549.y);
            v551.z = min(v551.z, -10.0f);
            float v559 = abs(frame_constants_view_constants[0u].clip_to_view[1].y);
            bool v560 = v559 < 1.0f;
            float v565;
            if (v560)
            {
                v565 = v559;
            }
            else
            {
                v565 = 1.0f / v559;
            }
            float v566 = v565 * v565;
            float v571 = (1.0f + (((-0.3018949925899505615234375f) + (0.087292902171611785888671875f * v566)) * v566)) * v565;
            float v576;
            if (v560)
            {
                v576 = v571;
            }
            else
            {
                v576 = 1.57079601287841796875f - v571;
            }
            float v582;
            if (frame_constants_view_constants[0u].clip_to_view[1].y < 0.0f)
            {
                v582 = -v576;
            }
            else
            {
                v582 = v576;
            }
            if (dot(float3(0.0f, 0.0f, -1.0f), normalize(v551)) > cos(((max(0.070000000298023223876953125f, v582) * 1.2000000476837158203125f) * smoothstep(v235, v232.x, v234)) * v203))
            {
                discard;
            }
        }
    }
    float3 v594 = normalize(in_var_C2);
    float3 v595 = in_var_C3 * 0.0333333350718021392822265625f.xxx;
    float3 v621;
    float3 v622;
    if (v205)
    {
        v621 = bark_color_naked.SampleBias(sampler_llr, v595.xy, frame_constants_global_mip_bias).xyz;
        v622 = bark_color_naked.SampleBias(sampler_llr, v595.zy, frame_constants_global_mip_bias).xyz;
    }
    else
    {
        v621 = bark_color.SampleBias(sampler_llr, v595.xy, frame_constants_global_mip_bias).xyz;
        v622 = bark_color.SampleBias(sampler_llr, v595.zy, frame_constants_global_mip_bias).xyz;
    }
    float3 v624 = pow(abs(v594), 1.0f.xxx);
    float v629 = dot(v622, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + v624.x;
    float v630 = dot(v621, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + v624.z;
    float v632 = max(v629, v630) - 0.20000000298023223876953125f;
    float v634 = max(v629 - v632, 0.0f);
    float v636 = max(v630 - v632, 0.0f);
    float3 v642 = ((v622 * v634) + (v621 * v636)) / (v634 + v636).xxx;
    float3 v661;
    if (frame_constants_glade_theme_id == 2u)
    {
        v661 = float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.039999999105930328369140625f);
    }
    else
    {
        float3 v660;
        if (frame_constants_glade_theme_id == 4u)
        {
            v660 = lerp(v642, 1.0f.xxx, (((1.0f - smoothstep(-0.5f, 4.0f, in_var_C8)) * 0.300000011920928955078125f) * smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y)).xxx);
        }
        else
        {
            v660 = float3(0.0599999986588954925537109375f, 0.100000001490116119384765625f, 0.02999999932944774627685546875f);
        }
        v661 = v660;
    }
    float v694;
    do
    {
        float2 v666 = (in_var_C3.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float v667 = v666.x;
        bool v673;
        if (!(v667 < 0.0f))
        {
            v673 = v667 > 1.0f;
        }
        else
        {
            v673 = true;
        }
        bool v679;
        if (!v673)
        {
            v679 = v666.y < 0.0f;
        }
        else
        {
            v679 = true;
        }
        bool v685;
        if (!v679)
        {
            v685 = v666.y > 1.0f;
        }
        else
        {
            v685 = true;
        }
        if (v685)
        {
            v694 = 0.0f;
            break;
        }
        else
        {
            v694 = rocky_terrain.SampleLevel(sampler_default, v666, 0.0f).x;
            break;
        }
        break; // unreachable workaround
    } while(false);
    float3 v702 = lerp(v642, v661, ((1.0f - smoothstep(-0.100000001490116119384765625f, 0.4000000059604644775390625f, in_var_C1.y)) * clamp(1.0f - v694, 0.699999988079071044921875f, 1.0f)).xxx);
    float4 v710 = bark_lines.SampleBias(sampler_llr, in_var_C1.xy * 1.2000000476837158203125f, frame_constants_global_mip_bias);
    float3 v713 = lerp(v702, lerp(v702, v661, 0.5f.xxx) * 0.60000002384185791015625f, 1.0f.xxx - v710.xyz);
    bool v718;
    if (v205)
    {
        v718 = in_var_C1.x <= (-99.0f);
    }
    else
    {
        v718 = false;
    }
    bool3 v720 = v718.xxx;
    float3 v721 = float3(v720.x ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).x : v713.x, v720.y ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).y : v713.y, v720.z ? float3(0.25f, 0.1500000059604644775390625f, 0.037500001490116119384765625f).z : v713.z);
    float3 v732;
    if (frame_constants_glade_theme_id == 4u)
    {
        v732 = lerp(v721, 0.699999988079071044921875f.xxx, (smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C3.y) * smoothstep(0.20000000298023223876953125f, 1.0f, clamp(v594.y, 0.0f, 1.0f))).xxx);
    }
    else
    {
        v732 = v721;
    }
    float3 v734 = (v594 * 0.5f) + 0.5f.xxx;
    float3 v756 = sqrt(clamp(v732, 0.0f.xxx, 1.0f.xxx));
    uint v775 = (uint((clamp(v756.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v756.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v756.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 v776 = uint2(((uint((clamp(v734.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v734.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v734.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3858759680u, v775);
    v776.y = v775 | ((7u | (uint(v718 ? false : true) << 6u)) << 24u);
    out_var_SV_TARGET0 = v776;
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
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 VS 逐字节一致;
//   PS 仅做生长阈值 discard,无输出。
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
    float v141 = sqrt(clamp((1.0f - in_var_age) + lerp(-1.0f, 1.0f, clamp(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) & 65535u).x, 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 v143 = lerp(in_var_appear_pos, in_var_Vertex_Position, v141.xxx);
    float v144 = v143.y;
    float v145 = v144 * 0.0199999995529651641845703125f;
    float v150 = float(frame_constants_game_time_sec);
    float v155 = v143.x;
    float v164 = v143.z;
    float v165 = 0.20000000298023223876953125f * v164;
    float v169 = v155 + ((v145 * sin((fmod(v150 * 0.8235290050506591796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.8235290050506591796875f)) + (0.20000000298023223876953125f * v155))) * sin((fmod(v150 * 0.893616974353790283203125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.893616974353790283203125f)) + v165));
    uint v187 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    float v194 = radians(nani_instance_data.Load<float>(v187 * 48 + 16));
    float3 v198 = normalize(float3(cos(v194), 0.0f, sin(v194)));
    float4x4 v221 = transpose(float4x4(float4(v198, 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(normalize(cross(v198, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 v225 = mul(float4x4(float4(1.0f, 0.0f, 0.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).x), float4(0.0f, 1.0f, 0.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).y), float4(0.0f, 0.0f, 1.0f, nani_instance_data.Load<float3>(v187 * 48 + 0).z), float4(0.0f, 0.0f, 0.0f, 1.0f)), mul(v221, mul(float4x4(float4(nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f, 0.0f, 0.0f), float4(0.0f, nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f, 0.0f), float4(0.0f, 0.0f, nani_instance_data.Load<float>(v187 * 48 + 12), 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(v169, v144, v164 + ((v145 * sin((fmod(v150 * 0.792452991008758544921875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.792452991008758544921875f)) + (0.20000000298023223876953125f * v169))) * sin((fmod(v150 * 0.688525021076202392578125f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 0.688525021076202392578125f)) + v165)), 1.0f))));
    float2 v242 = v225.xz + (nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 36) * ((clamp(in_var_Vertex_Position.y * 0.100000001490116119384765625f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, in_var_age)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v242.x, v225.y, v242.y, 1.0f));
    out_var_C1 = in_var_Vertex_Color;
    out_var_C2 = mul(v221, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C3 = float4(v242.x, v225.y, v242.y, v225.w).xyz;
    out_var_C4 = v141;
    out_var_C5 = spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 32) >> 16u) & 65535u).x;
    out_var_C6 = int(uint(nani_instance_data.Load<int>(v187 * 48 + 20)));
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
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 VS 逐字节一致;
//   PS 仅做生长阈值 discard,无输出。
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
