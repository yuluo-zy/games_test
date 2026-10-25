// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// 【文件用途】草甸花光栅化。VS 做岩石/路径/水剔除(NaN)、贴地、2D 风值噪声摆动
//   + grass_push 位移,输出 TBN 与风 uv;PS 做 alpha 剔除 + 帧交错棋盘剔除,
//   从贴图取反照率/法线(albedo_map、normal_map、alpha_map、leafy_greens),
//   打包 uint2 G-Buffer。
// 【入口】vs_main/ps_main | 绑定:path_mask t14、albedo_map t15、normal_map t16、
//   alpha_map t17、leafy_greens t18、grass_object_push t19、terrain_heightmap t20、
//   rocky_terrain t22、water_mask t23、sampler_llb s4、sampler_default s9、
//   sampler_llr s1、$Globals b13、frame_constants b0/space1。
// 【变体】b903+f0ad 的 VS 与 PS 均用提起段;手写段仅作参考基线。
// 重写说明:手写重建段补充中文注释;提起段(TGV_*)逐字保留,仅加块级中文注释。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/clearing/meadow_flower.raster.hlsl
// Evidence: _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): sampler_llb s4, sampler_default s9, $Globals b13, path_mask t14,
//   grass_object_push t19, terrain_heightmap t20, rocky_terrain t22, water_mask t23,
//   frame_constants. (ps): sampler_llr s1, $Globals b13, albedo_map t15, normal_map t16,
//   alpha_map t17, leafy_greens t18, frame_constants.
//   vs_main: rocky/path/water rejection, terrain snap, wind value-noise, TBN. ps_main:
//   alpha discard + frame-stagger discard, albedo/normal from maps, packed uint2.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> terrain_heightmap : register(t20, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "inc/pack_unpack.hlsl"

// color_library.hlsl guard-skipped (not bound by this shader).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).

// $Globals members (implicit cbuffer at set0/b13).
float4x4 model_to_world;
float time;
uint2 path_mask_dims;
uint2 albedo_map_dims;
uint2 normal_map_dims;
int has_normal_map;
uint2 alpha_map_dims;
uint2 leafy_greens_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 rocky_terrain_dims;
uint2 water_mask_dims;
int bool_render_shadows;

Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

struct VS_IN
{
    [[vk::location(0)]] float3 vertex_position: Vertex_Position;
    [[vk::location(1)]] float3 vertex_normal: Vertex_Normal;
    [[vk::location(2)]] float2 vertex_uv: Vertex_UV;
    [[vk::location(3)]] float3 flower_pivot: flower_pivot;
    [[vk::location(4)]] int stem: stem;
    [[vk::location(5)]] float wind_bend: wind_bend;
};

struct VS_OUT
{
    float2 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
    float4 position : SV_Position;
};

// VS:岩石/路径/水剔除(NaN) -> 贴地 -> 风摆动/推送 -> TBN/uv 输出
VS_OUT vs_main(VS_IN i)
{
    VS_OUT o;
    float4 wp;
    float2 wind_uv;
    float3 world_pos;
    float3 normal;
    int stem_out;
    float2 uv_out;
    float3x3 tbn;
    do
    {
// flower_pivot /130 + 0.5 得到掩码采样 uv
        float2 mask_uv = (i.flower_pivot.xz * (1.0f / 130.0f)) + 0.5f;
        bool in_bounds = !((mask_uv.x < 0.0f) || (mask_uv.x > 1.0f) || (mask_uv.y < 0.0f) || (mask_uv.y > 1.0f));
// 岩石采样(带边界检查)
        float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x : 0.0f;
// 路径或岩石上:整个顶点剔除(输出 NaN)
        if ((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f))
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
// 水掩码 > 0.25 剔除
        bool water_reject;
        if (!((smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x) > 0.1f) || (rocky > 0.1f)))
        {
            float water = in_bounds ? water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y : 0.0f;
            water_reject = water > 0.25f;
        }
        else
        {
            water_reject = true;
        }
        if (water_reject)
        {
            wp = asfloat(0x7fbfffffu);
            wind_uv = 0.0f;
            world_pos = 0.0f;
            normal = 0.0f;
            stem_out = 0;
            uv_out = 0.0f;
            tbn = float3x3(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f);
            break;
        }
// 贴地:y = 顶点 y + 高度图映射 lerp(-2.5, 24, h)
        float4 v = float4(i.vertex_position, 1.0f);
        v.y = i.vertex_position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, mask_uv, 0.0f).x);
        float2 wind_xy = (i.flower_pivot.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
        int2 wind_cell = int2(floor(wind_xy));
        float2 wind_frac = frac(wind_xy);
        float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
        float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
        float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
        float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
        float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
        float wind_noise = ((1.0f - wind_smooth.y) * ((wind_n00 * (1.0f - wind_smooth.x)) + (wind_n10 * wind_smooth.x)))
            + ((wind_n01 * (1.0f - wind_smooth.x)) * wind_smooth.y)
            + ((wind_n11 * wind_smooth.x) * wind_smooth.y);
// 风摆:x 减去 wind_bend * sin(wind_noise*π-π/2+0.3) * 0.175
        v.x = i.vertex_position.x - (i.wind_bend * sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.175f;
// 草推送掩码:z>0 时按 wind_bend^0.5 * 0.5 位移 xz
        float4 push = grass_object_push.SampleLevel(sampler_default, mask_uv, 0.0f);
        float4 v2;
        if (clamp(push.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 pushed = v.xz + ((push.xy * pow(i.wind_bend, 0.5f)) * 0.5f);
            v2 = float4(pushed.x, v.y, pushed.y, v.w);
        }
        else
        {
            v2 = v;
        }
// 经 model_to_world + world_to_sample 投影到裁剪空间
        wp = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v2.xyz, 1.0f)));
        wind_uv = wind_xy;
        world_pos = v2.xyz;
        normal = i.vertex_normal;
        stem_out = i.stem;
        uv_out = i.vertex_uv;
// 由 model_to_world 各轴构 TBN 切空间基(法线贴图用)
        tbn = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz),
                                 normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while (false);
    o.position = wp;
    o.C1 = wind_uv;
    o.C2 = world_pos;
    o.C3 = normal;
    o.C5 = stem_out;
    o.C6 = uv_out;
    o.C7 = tbn;
    return o;
}

// Pack a [0,1] normal rgb into one uint channel of the uint2 target.
// 把 [0,1] 法线 RGB 打包到单个 uint 通道
uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

struct PS_IN
{
    float2 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    nointerpolation int C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float3x3 C7 : TEXCOORD7;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

// PS:alpha 剔除 + 帧交错棋盘剔除 -> 反照率/法线贴图采样 -> uint2 打包
PS_OUT ps_main(PS_IN i, float4 frag_coord : SV_Position)
{
    PS_OUT o;
// alpha 贴图 < 0.5 剔除
    if (alpha_map.Sample(sampler_llr, i.C6).x < 0.5f)
    {
        discard;
    }
// 非阴影通道:按帧索引交错的 2x2 棋盘剔除一半像素
    if (bool_render_shadows != 1)
    {
        uint2 pix = uint2(frag_coord.xy) & uint2(1u, 1u);
        if ((frame_index & 3u) == (pix.x + (pix.y * 2u)))
        {
            discard;
        }
    }
// 采样反照率贴图(风 uv)
    float4 albedo_tex = albedo_map.Sample(sampler_llr, i.C1);
    float3 albedo;
// C5==1(叶片):用 leafy_greens 纹理
    if (i.C5 == 1)
    {
        albedo = leafy_greens.Sample(sampler_llr, i.C1).xyz;
    }
    else
    {
        albedo = albedo_tex.xyz;
    }
// 法线:有 normal_map 时经 TBN 从切线空间转到世界;否则用顶点法线
    float3 n;
    if (has_normal_map == 1)
    {
        n = normalize(mul(i.C7, (normal_map.Sample(sampler_llr, i.C6).xyz * 2.0f) - 1.0f));
    }
    else
    {
        n = i.C3;
    }
// 打包:法线(RGB8 + 0x80000000 标志)、反照率(RGB8 + 0x4D000000 标志)
    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    o.target = uint2((pack_normal8(packed_n) + 0x80000000u), (pack_normal8(color) + 0x4D000000u));
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价(剔除/
//   贴地/风摆动/推送/TBN 全内联);PS 为 alpha/棋盘剔除 + 贴图采样打包。
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 v448;
    float2 v449;
    float3 v450;
    float3 v451;
    int v452;
    float2 v453;
    float3x3 v454;
    do
    {
        float v166;
        bool v168;
        float2 v159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float v193;
        do
        {
            v166 = v159.x;
            v168 = !(v166 < 0.0f);
            bool v172;
            if (v168)
            {
                v172 = v166 > 1.0f;
            }
            else
            {
                v172 = true;
            }
            bool v178;
            if (!v172)
            {
                v178 = v159.y < 0.0f;
            }
            else
            {
                v178 = true;
            }
            bool v184;
            if (!v178)
            {
                v184 = v159.y > 1.0f;
            }
            else
            {
                v184 = true;
            }
            if (v184)
            {
                v193 = 0.0f;
                break;
            }
            else
            {
                v193 = rocky_terrain.SampleLevel(sampler_default, v159, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool v199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, v159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            v199 = v193 > 0.100000001490116119384765625f;
        }
        else
        {
            v199 = true;
        }
        bool v231;
        if (!v199)
        {
            float v229;
            do
            {
                bool v208;
                if (v168)
                {
                    v208 = v166 > 1.0f;
                }
                else
                {
                    v208 = true;
                }
                bool v214;
                if (!v208)
                {
                    v214 = v159.y < 0.0f;
                }
                else
                {
                    v214 = true;
                }
                bool v220;
                if (!v214)
                {
                    v220 = v159.y > 1.0f;
                }
                else
                {
                    v220 = true;
                }
                if (v220)
                {
                    v229 = 0.0f;
                    break;
                }
                else
                {
                    v229 = water_mask.SampleLevel(sampler_default, v159, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            v231 = v229 > 0.25f;
        }
        else
        {
            v231 = true;
        }
        if (v231)
        {
// 剔除命中:NaN 位置使该顶点永不光栅化
            v448 = asfloat(0x7fbfffffu /* nan */).xxxx;
            v449 = 0.0f.xx;
            v450 = 0.0f.xxx;
            v451 = 0.0f.xxx;
            v452 = 0;
            v453 = 0.0f.xx;
            v454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 v237 = float4(in_var_Vertex_Position, 1.0f);
        v237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v159, 0.0f).x);
        float2 v246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 v251 = v246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 v253 = int2(floor(v251));
        float2 v254 = frac(v251);
        uint2 v255 = uint2(v253);
        uint v257 = v255.y;
        uint v259 = v257 + (v257 << 10u);
        uint v261 = v259 ^ (v259 >> 6u);
        uint v263 = v261 + (v261 << 3u);
        uint v265 = v263 ^ (v263 >> 11u);
        uint v271 = (((v255.x * 1664525u) + (v265 + (v265 << 15u))) + 1013904223u) * 1664525u;
        uint v273 = v271 ^ (v271 >> 11u);
        uint v276 = v273 ^ ((v273 << 7u) & 2636928640u);
        uint v279 = v276 ^ ((v276 << 15u) & 4022730752u);
        uint2 v287 = uint2(v253 + int2(1, 0));
        uint v289 = v287.y;
        uint v291 = v289 + (v289 << 10u);
        uint v293 = v291 ^ (v291 >> 6u);
        uint v295 = v293 + (v293 << 3u);
        uint v297 = v295 ^ (v295 >> 11u);
        uint v303 = (((v287.x * 1664525u) + (v297 + (v297 << 15u))) + 1013904223u) * 1664525u;
        uint v305 = v303 ^ (v303 >> 11u);
        uint v308 = v305 ^ ((v305 << 7u) & 2636928640u);
        uint v311 = v308 ^ ((v308 << 15u) & 4022730752u);
        uint2 v319 = uint2(v253 + int2(0, 1));
        uint v321 = v319.y;
        uint v323 = v321 + (v321 << 10u);
        uint v325 = v323 ^ (v323 >> 6u);
        uint v327 = v325 + (v325 << 3u);
        uint v329 = v327 ^ (v327 >> 11u);
        uint v335 = (((v319.x * 1664525u) + (v329 + (v329 << 15u))) + 1013904223u) * 1664525u;
        uint v337 = v335 ^ (v335 >> 11u);
        uint v340 = v337 ^ ((v337 << 7u) & 2636928640u);
        uint v343 = v340 ^ ((v340 << 15u) & 4022730752u);
        uint2 v351 = uint2(v253 + int2(1, 1));
        uint v353 = v351.y;
        uint v355 = v353 + (v353 << 10u);
        uint v357 = v355 ^ (v355 >> 6u);
        uint v359 = v357 + (v357 << 3u);
        uint v361 = v359 ^ (v359 >> 11u);
        uint v367 = (((v351.x * 1664525u) + (v361 + (v361 << 15u))) + 1013904223u) * 1664525u;
        uint v369 = v367 ^ (v367 >> 11u);
        uint v372 = v369 ^ ((v369 << 7u) & 2636928640u);
        uint v375 = v372 ^ ((v372 << 15u) & 4022730752u);
        float2 v385 = (v254 * v254) * (3.0f.xx - (v254 * 2.0f));
        float v386 = v385.x;
        float v387 = 1.0f - v386;
        float v389 = v385.y;
// 内联后的风摆表达式:与手写一致,4 角点 2D 值噪声插值出 wind_noise
        v237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - v389) * (((asfloat(((v279 ^ (v279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v387) + ((asfloat(((v311 ^ (v311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v386))) + (((asfloat(((v343 ^ (v343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v387) * v389)) + (((asfloat(((v375 ^ (v375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v386) * v389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 v410 = grass_object_push.SampleLevel(sampler_default, v159, 0.0f);
        float4 v423;
        if (clamp(v410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 v421 = v237.xz + ((v410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            v423 = float4(v421.x, v237.y, v421.y, v237.w);
        }
        else
        {
            v423 = v237;
        }
        v448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v423.xyz, 1.0f)));
        v449 = v246;
        v450 = v423.xyz;
        v451 = in_var_Vertex_Normal;
        v452 = in_var_stem;
        v453 = in_var_Vertex_UV;
        v454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = v448;
    out_var_C1 = v449;
    out_var_C2 = v450;
    out_var_C3 = v451;
    out_var_C5 = v452;
    out_var_C6 = v453;
    out_var_C7 = v454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_meadow_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价(剔除/
//   贴地/风摆动/推送/TBN 全内联);PS 为 alpha/棋盘剔除 + 贴图采样打包。
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> albedo_map : register(t15, space0);
Texture2D<float4> normal_map : register(t16, space0);
Texture2D<float4> alpha_map : register(t17, space0);
Texture2D<float4> leafy_greens : register(t18, space0);

static float4 gl_FragCoord;
static float2 in_var_C1;
static float3 in_var_C3;
static int in_var_C5;
static float2 in_var_C6;
static float3x3 in_var_C7;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float2 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 in_var_C7 : TEXCOORD7;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 v128 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (v128.x + (v128.y * 2u)))
        {
            discard;
        }
    }
    float4 v141 = albedo_map.Sample(sampler_llr, in_var_C1);
    float3 v150;
    if (in_var_C5 == 1)
    {
        v150 = leafy_greens.Sample(sampler_llr, in_var_C1).xyz;
    }
    else
    {
        v150 = v141.xyz;
    }
    float3 v165;
    if (has_normal_map == 1)
    {
        v165 = normalize(mul(in_var_C7, (normal_map.Sample(sampler_llr, in_var_C6).xyz * 2.0f) - 1.0f.xxx));
    }
    else
    {
        v165 = in_var_C3;
    }
    float3 v167 = (v165 * 0.5f) + 0.5f.xxx;
    float3 v189 = sqrt(clamp(v150, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(v167.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v167.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v167.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 2147483648u, ((uint((clamp(v189.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v189.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v189.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _clearing_meadow_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 逐字节一致;PS 仅做
//   alpha/棋盘剔除,不写 GBuffer。
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> path_mask : register(t14, space0);
Texture2D<float4> grass_object_push : register(t19, space0);
Texture2D<float4> terrain_heightmap : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);

static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_flower_pivot;
static int in_var_stem;
static float in_var_wind_bend;
static float2 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static int out_var_C5;
static float2 out_var_C6;
static float3x3 out_var_C7;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(3)]] float3 in_var_flower_pivot: flower_pivot;
    [[vk::location(4)]] int in_var_stem: stem;
    [[vk::location(5)]] float in_var_wind_bend: wind_bend;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float2 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float3x3 out_var_C7 : TEXCOORD7;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float4 v448;
    float2 v449;
    float3 v450;
    float3 v451;
    int v452;
    float2 v453;
    float3x3 v454;
    do
    {
        float v166;
        bool v168;
        float2 v159 = (in_var_flower_pivot.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
        float v193;
        do
        {
            v166 = v159.x;
            v168 = !(v166 < 0.0f);
            bool v172;
            if (v168)
            {
                v172 = v166 > 1.0f;
            }
            else
            {
                v172 = true;
            }
            bool v178;
            if (!v172)
            {
                v178 = v159.y < 0.0f;
            }
            else
            {
                v178 = true;
            }
            bool v184;
            if (!v178)
            {
                v184 = v159.y > 1.0f;
            }
            else
            {
                v184 = true;
            }
            if (v184)
            {
                v193 = 0.0f;
                break;
            }
            else
            {
                v193 = rocky_terrain.SampleLevel(sampler_default, v159, 0.0f).x;
                break;
            }
            break; // unreachable workaround
        } while(false);
        bool v199;
        if (!(smoothstep(0.07500000298023223876953125f, 0.125f, path_mask.SampleLevel(sampler_llb, v159, 0.0f).x) > 0.100000001490116119384765625f))
        {
            v199 = v193 > 0.100000001490116119384765625f;
        }
        else
        {
            v199 = true;
        }
        bool v231;
        if (!v199)
        {
            float v229;
            do
            {
                bool v208;
                if (v168)
                {
                    v208 = v166 > 1.0f;
                }
                else
                {
                    v208 = true;
                }
                bool v214;
                if (!v208)
                {
                    v214 = v159.y < 0.0f;
                }
                else
                {
                    v214 = true;
                }
                bool v220;
                if (!v214)
                {
                    v220 = v159.y > 1.0f;
                }
                else
                {
                    v220 = true;
                }
                if (v220)
                {
                    v229 = 0.0f;
                    break;
                }
                else
                {
                    v229 = water_mask.SampleLevel(sampler_default, v159, 0.0f).y;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            v231 = v229 > 0.25f;
        }
        else
        {
            v231 = true;
        }
        if (v231)
        {
// 剔除命中:NaN 位置使该顶点永不光栅化
            v448 = asfloat(0x7fbfffffu /* nan */).xxxx;
            v449 = 0.0f.xx;
            v450 = 0.0f.xxx;
            v451 = 0.0f.xxx;
            v452 = 0;
            v453 = 0.0f.xx;
            v454 = float3x3(0.0f.xxx, 0.0f.xxx, 0.0f.xxx);
            break;
        }
        float4 v237 = float4(in_var_Vertex_Position, 1.0f);
        v237.y = in_var_Vertex_Position.y + lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, v159, 0.0f).x);
        float2 v246 = in_var_flower_pivot.xz * 0.100000001490116119384765625f;
        float2 v251 = v246 + (float2(time, 0.0f) * 1.2000000476837158203125f);
        int2 v253 = int2(floor(v251));
        float2 v254 = frac(v251);
        uint2 v255 = uint2(v253);
        uint v257 = v255.y;
        uint v259 = v257 + (v257 << 10u);
        uint v261 = v259 ^ (v259 >> 6u);
        uint v263 = v261 + (v261 << 3u);
        uint v265 = v263 ^ (v263 >> 11u);
        uint v271 = (((v255.x * 1664525u) + (v265 + (v265 << 15u))) + 1013904223u) * 1664525u;
        uint v273 = v271 ^ (v271 >> 11u);
        uint v276 = v273 ^ ((v273 << 7u) & 2636928640u);
        uint v279 = v276 ^ ((v276 << 15u) & 4022730752u);
        uint2 v287 = uint2(v253 + int2(1, 0));
        uint v289 = v287.y;
        uint v291 = v289 + (v289 << 10u);
        uint v293 = v291 ^ (v291 >> 6u);
        uint v295 = v293 + (v293 << 3u);
        uint v297 = v295 ^ (v295 >> 11u);
        uint v303 = (((v287.x * 1664525u) + (v297 + (v297 << 15u))) + 1013904223u) * 1664525u;
        uint v305 = v303 ^ (v303 >> 11u);
        uint v308 = v305 ^ ((v305 << 7u) & 2636928640u);
        uint v311 = v308 ^ ((v308 << 15u) & 4022730752u);
        uint2 v319 = uint2(v253 + int2(0, 1));
        uint v321 = v319.y;
        uint v323 = v321 + (v321 << 10u);
        uint v325 = v323 ^ (v323 >> 6u);
        uint v327 = v325 + (v325 << 3u);
        uint v329 = v327 ^ (v327 >> 11u);
        uint v335 = (((v319.x * 1664525u) + (v329 + (v329 << 15u))) + 1013904223u) * 1664525u;
        uint v337 = v335 ^ (v335 >> 11u);
        uint v340 = v337 ^ ((v337 << 7u) & 2636928640u);
        uint v343 = v340 ^ ((v340 << 15u) & 4022730752u);
        uint2 v351 = uint2(v253 + int2(1, 1));
        uint v353 = v351.y;
        uint v355 = v353 + (v353 << 10u);
        uint v357 = v355 ^ (v355 >> 6u);
        uint v359 = v357 + (v357 << 3u);
        uint v361 = v359 ^ (v359 >> 11u);
        uint v367 = (((v351.x * 1664525u) + (v361 + (v361 << 15u))) + 1013904223u) * 1664525u;
        uint v369 = v367 ^ (v367 >> 11u);
        uint v372 = v369 ^ ((v369 << 7u) & 2636928640u);
        uint v375 = v372 ^ ((v372 << 15u) & 4022730752u);
        float2 v385 = (v254 * v254) * (3.0f.xx - (v254 * 2.0f));
        float v386 = v385.x;
        float v387 = 1.0f - v386;
        float v389 = v385.y;
// 内联后的风摆表达式:与手写一致,4 角点 2D 值噪声插值出 wind_noise
        v237.x = in_var_Vertex_Position.x - ((in_var_wind_bend * (sin((((((1.0f - v389) * (((asfloat(((v279 ^ (v279 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v387) + ((asfloat(((v311 ^ (v311 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v386))) + (((asfloat(((v343 ^ (v343 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v387) * v389)) + (((asfloat(((v375 ^ (v375 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v386) * v389)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.17499999701976776123046875f);
        float4 v410 = grass_object_push.SampleLevel(sampler_default, v159, 0.0f);
        float4 v423;
        if (clamp(v410.z, 0.0f, 1.0f) > 0.0f)
        {
            float2 v421 = v237.xz + ((v410.xy * pow(in_var_wind_bend, 0.5f)) * 0.5f);
            v423 = float4(v421.x, v237.y, v421.y, v237.w);
        }
        else
        {
            v423 = v237;
        }
        v448 = mul(frame_constants_view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(v423.xyz, 1.0f)));
        v449 = v246;
        v450 = v423.xyz;
        v451 = in_var_Vertex_Normal;
        v452 = in_var_stem;
        v453 = in_var_Vertex_UV;
        v454 = transpose(float3x3(normalize(mul(model_to_world, float4(1.0f, 0.0f, 0.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz), normalize(mul(model_to_world, float4(0.0f, 1.0f, 0.0f, 0.0f)).xyz)));
        break;
    } while(false);
    gl_Position = v448;
    out_var_C1 = v449;
    out_var_C2 = v450;
    out_var_C3 = v451;
    out_var_C5 = v452;
    out_var_C6 = v453;
    out_var_C7 = v454;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_flower_pivot = stage_input.in_var_flower_pivot;
    in_var_stem = stage_input.in_var_stem;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _clearing_meadow_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 逐字节一致;PS 仅做
//   alpha/棋盘剔除,不写 GBuffer。
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
    column_major float4x4 model_to_world : packoffset(c1);
    float time : packoffset(c5);
    uint2 path_mask_dims : packoffset(c5.y);
    uint2 albedo_map_dims : packoffset(c6);
    uint2 normal_map_dims : packoffset(c6.z);
    int has_normal_map : packoffset(c7);
    uint2 alpha_map_dims : packoffset(c7.y);
    uint2 leafy_greens_dims : packoffset(c8);
    uint2 grass_object_push_dims : packoffset(c8.z);
    uint2 terrain_heightmap_dims : packoffset(c9);
    uint2 visual_terrain_heightmap_dims : packoffset(c9.z);
    uint2 rocky_terrain_dims : packoffset(c10);
    uint2 water_mask_dims : packoffset(c10.z);
    int bool_render_shadows : packoffset(c11);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> alpha_map : register(t17, space0);

static float4 gl_FragCoord;
static float2 in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    if (alpha_map.Sample(sampler_llr, in_var_C6).x < 0.5f)
    {
        discard;
    }
    if (!((bool_render_shadows == 1) ? true : false))
    {
        uint2 v99 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        if ((frame_constants_frame_index & 3u) == (v99.x + (v99.y * 2u)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
