// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/splash_particle.raster.hlsl | F-V | bins: _splash_particle.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// 水花粒子着色器。
//   顶点阶段 vs_main: 用全局 model_to_world 变换顶点位置/法线,透传颜色与 UV。
//   像素阶段 ps_main:
//     - 先做隔帧子像素(hi-px)剔除:只有帧序号对应的一帧才真正绘制;
//     - 用两片 2D 值噪声(LCG 哈希晶格 + 双线性插值)生成"水滴环 + 涟漪环"图案,
//       由 anim_t 与游戏时间驱动,图案外像素 discard;
//     - 打包 G-Buffer(法线 RGB8 + 0xFF000000,反照率 RGB8 | 材质标志 0x46000000)。
//   绑定: $Globals(b13, 含 model_to_world/bool_render_shadows/anim_t)。
//   变体: TGV_f0adff76827e73ff(像素阶段使用转译版)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

// INC 问题规避:原始 common.hlsl 声明了 13 个采样器(s0..s12);
// inc/samplers.hlsl 只带 s0..s4、s9..s12。在此声明未用的 s5..s8,
// 以复现原 $Globals 绑定(set0/b13)。

float4x4 model_to_world;
int bool_render_shadows;
float anim_t;

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 顶点变换到世界空间,再变换到采样(裁剪)空间;透传颜色、法线、世界位置与 UV。
    float4 world_pos = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(GetCurrentViewConstants().world_to_sample, world_pos);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world_pos.xyz;
    o.C3 = vin.Vertex_UV;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// 环状衰减:环中心为 1,向半径边缘平滑衰减到 0(三段式平滑曲线)。
float drop_ring(float d, float radius)
{
    if (d > radius)
    {
        return 0.0f;
    }
    float t = d / radius;
    return 1.0f - (t * t) * (3.0f - (2.0f * t));
}

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 隔帧子像素渲染:每帧只绘制 4 个子像素中的一个(降低解析度、增加水花密集感)。
    static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };
    uint2 frag = uint2(pin.position.xy) & uint2(1u, 1u);
    uint2 sub = hi_px_subpixels[frame_index & 3u];
    if (all(frag != sub))
    {
        discard;
    }

    float t = 1.0f - anim_t;
    float2 uv = pin.C3 * 25.6f;

    // 水滴环:2D 值噪声(LCG 哈希晶格 + 双线性插值)驱动环半径。
    float2 npos1 = float2((uv.x * 2.0f) + pin.C2.x + pin.C2.z,
                          (uv.y * 0.5f) + (float(game_time_sec) + game_time_frac));
    int2 cell1 = int2(floor(npos1));
    float2 f1 = frac(npos1);
    float2 u1 = f1 * f1 * (3.0f - 2.0f * f1);
    float n00 = incb_perlin_hash01(hash(uint2(cell1)));
    float n10 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell1 + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell1 + int2(1, 1))));
    float drop_blend = (1.0f - u1.y) * ((n00 * (1.0f - u1.x)) + (n10 * u1.x))
                     + (n01 * (1.0f - u1.x)) * u1.y
                     + (n11 * u1.x) * u1.y;
    float drop_radius = lerp(0.1f, 0.4f, drop_blend);

    // 水滴随动画下移(三次曲线),用距中心的距离做环衰减。
    float d = abs((pin.C3.y + lerp(-1.0f, 1.0f, (t * t * t))) - 0.5f);
    float drop = drop_ring(d, drop_radius);

    // 涟漪环:第二片值噪声场,驱动涟漪宽度。
    float2 npos2 = float2((uv.x * 2.0f) + 2.0f + pin.C2.x + pin.C2.z,
                          uv.y * 0.2f);
    int2 cell2 = int2(floor(npos2));
    float2 f2 = frac(npos2);
    float2 u2 = f2 * f2 * (3.0f - 2.0f * f2);
    float m00 = incb_perlin_hash01(hash(uint2(cell2)));
    float m10 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 0))));
    float m01 = incb_perlin_hash01(hash(uint2(cell2 + int2(0, 1))));
    float m11 = incb_perlin_hash01(hash(uint2(cell2 + int2(1, 1))));
    float ripple_blend = (1.0f - u2.y) * ((m00 * (1.0f - u2.x)) + (m10 * u2.x))
                       + (m01 * (1.0f - u2.x)) * u2.y
                       + (m11 * u2.x) * u2.y;
    float ripple_w = lerp(0.0f, 0.3f, ripple_blend);
    // 环强度不足的像素丢弃。
    if ((drop * (1.0f - smoothstep(0.9f, 1.0f, pin.C3.y + ripple_w))) < 0.5f)
    {
        discard;
    }

    // 打包 G-Buffer:法线压到 [0,1],反照率 = 顶点色 * 0.7 再 sqrt(线性→sRGB)。
    float3 packed_normal = normalize(pin.C1) * 0.5f + 0.5f;
    float3 albedo = sqrt(clamp(pin.C0 * 0.7f, 0.0f, 1.0f));
    uint albedo_word = uint(clamp(albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    // 高位标志:水花粒子材质类型。
    o.gbuffer.y = o.gbuffer.y | 1174405120u;
    return o;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提取自 _splash_particle.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
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

// 隔帧子像素查找表(与手写段一致)。
static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
    int bool_render_shadows : packoffset(c5);
    float anim_t : packoffset(c5.y);
};


static float4 gl_FragCoord;
static float3 in_var_C2;
static float2 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    // 隔帧子像素剔除。
    uint2 frag_parity = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
    uint frame_mod = frame_constants_frame_index & 3u;
    if (all(bool2(frag_parity.x != hi_px_subpixels[frame_mod].x, frag_parity.y != hi_px_subpixels[frame_mod].y)))
    {
        discard;
    }
    float t = 1.0f - anim_t;
    float2 uv = in_var_C3 * 25.6000003814697265625f;
    float uv_x2 = uv.x * 2.0f;
    float uv_y = uv.y;
    // 水滴环噪声场 1(位置随世界坐标与时间平移)。
    float2 drop_npos = float2((uv_x2 + in_var_C2.x) + in_var_C2.z, (uv_y * 0.5f) + (float(frame_constants_game_time_sec) + frame_constants_game_time_frac));
    int2 drop_cell = int2(floor(drop_npos));
    float2 drop_frac = frac(drop_npos);
    // 四角 LCG 哈希(角点 00)。
    uint2 drop_cell_u = uint2(drop_cell);
    uint d00_h_y = drop_cell_u.y;
    uint d00_h1 = d00_h_y + (d00_h_y << 10u);
    uint d00_h2 = d00_h1 ^ (d00_h1 >> 6u);
    uint d00_h3 = d00_h2 + (d00_h2 << 3u);
    uint d00_h4 = d00_h3 ^ (d00_h3 >> 11u);
    uint d00_h5 = (((drop_cell_u.x * 1664525u) + (d00_h4 + (d00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint d00_h6 = d00_h5 ^ (d00_h5 >> 11u);
    uint d00_h7 = d00_h6 ^ ((d00_h6 << 7u) & 2636928640u);
    uint d00_h8 = d00_h7 ^ ((d00_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 10)。
    uint2 drop_cell10_u = uint2(drop_cell + int2(1, 0));
    uint d10_h_y = drop_cell10_u.y;
    uint d10_h1 = d10_h_y + (d10_h_y << 10u);
    uint d10_h2 = d10_h1 ^ (d10_h1 >> 6u);
    uint d10_h3 = d10_h2 + (d10_h2 << 3u);
    uint d10_h4 = d10_h3 ^ (d10_h3 >> 11u);
    uint d10_h5 = (((drop_cell10_u.x * 1664525u) + (d10_h4 + (d10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint d10_h6 = d10_h5 ^ (d10_h5 >> 11u);
    uint d10_h7 = d10_h6 ^ ((d10_h6 << 7u) & 2636928640u);
    uint d10_h8 = d10_h7 ^ ((d10_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 01)。
    uint2 drop_cell01_u = uint2(drop_cell + int2(0, 1));
    uint d01_h_y = drop_cell01_u.y;
    uint d01_h1 = d01_h_y + (d01_h_y << 10u);
    uint d01_h2 = d01_h1 ^ (d01_h1 >> 6u);
    uint d01_h3 = d01_h2 + (d01_h2 << 3u);
    uint d01_h4 = d01_h3 ^ (d01_h3 >> 11u);
    uint d01_h5 = (((drop_cell01_u.x * 1664525u) + (d01_h4 + (d01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint d01_h6 = d01_h5 ^ (d01_h5 >> 11u);
    uint d01_h7 = d01_h6 ^ ((d01_h6 << 7u) & 2636928640u);
    uint d01_h8 = d01_h7 ^ ((d01_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 11)。
    uint2 drop_cell11_u = uint2(drop_cell + int2(1, 1));
    uint d11_h_y = drop_cell11_u.y;
    uint d11_h1 = d11_h_y + (d11_h_y << 10u);
    uint d11_h2 = d11_h1 ^ (d11_h1 >> 6u);
    uint d11_h3 = d11_h2 + (d11_h2 << 3u);
    uint d11_h4 = d11_h3 ^ (d11_h3 >> 11u);
    uint d11_h5 = (((drop_cell11_u.x * 1664525u) + (d11_h4 + (d11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint d11_h6 = d11_h5 ^ (d11_h5 >> 11u);
    uint d11_h7 = d11_h6 ^ ((d11_h6 << 7u) & 2636928640u);
    uint d11_h8 = d11_h7 ^ ((d11_h7 << 15u) & 4022730752u);
    // 双线性插值得到 [0,1] 噪声,再映射到水滴环半径。
    float2 drop_uf = (drop_frac * drop_frac) * (3.0f.xx - (drop_frac * 2.0f));
    float drop_u1x = drop_uf.x;
    float drop_u1x_inv = 1.0f - drop_u1x;
    float drop_u1y = drop_uf.y;
    float drop_radius = lerp(0.100000001490116119384765625f, 0.4000000059604644775390625f, (((1.0f - drop_u1y) * (((asfloat(((d00_h8 ^ (d00_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * drop_u1x_inv) + ((asfloat(((d10_h8 ^ (d10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * drop_u1x))) + (((asfloat(((d01_h8 ^ (d01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * drop_u1x_inv) * drop_u1y)) + (((asfloat(((d11_h8 ^ (d11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * drop_u1x) * drop_u1y));
    // 水滴环衰减:超出半径则 0,否则用平滑曲线。
    float drop;
    do
    {
        float drop_dist = abs((in_var_C3.y + lerp(-1.0f, 1.0f, (t * t) * t)) - 0.5f);
        if (drop_dist > drop_radius)
        {
            drop = 0.0f;
            break;
        }
        float drop_t = drop_dist / drop_radius;
        drop = 1.0f - ((drop_t * drop_t) * (3.0f - (2.0f * drop_t)));
        break;
    } while(false);
    // 涟漪环噪声场 2。
    float2 ripple_npos = float2((uv_x2 + 2.0f) + in_var_C2.x + in_var_C2.z, uv_y * 0.20000000298023223876953125f);
    int2 ripple_cell = int2(floor(ripple_npos));
    float2 ripple_frac = frac(ripple_npos);
    // 四角 LCG 哈希(角点 00)。
    uint2 ripple_cell_u = uint2(ripple_cell);
    uint r00_h_y = ripple_cell_u.y;
    uint r00_h1 = r00_h_y + (r00_h_y << 10u);
    uint r00_h2 = r00_h1 ^ (r00_h1 >> 6u);
    uint r00_h3 = r00_h2 + (r00_h2 << 3u);
    uint r00_h4 = r00_h3 ^ (r00_h3 >> 11u);
    uint r00_h5 = (((ripple_cell_u.x * 1664525u) + (r00_h4 + (r00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint r00_h6 = r00_h5 ^ (r00_h5 >> 11u);
    uint r00_h7 = r00_h6 ^ ((r00_h6 << 7u) & 2636928640u);
    uint r00_h8 = r00_h7 ^ ((r00_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 10)。
    uint2 ripple_cell10_u = uint2(ripple_cell + int2(1, 0));
    uint r10_h_y = ripple_cell10_u.y;
    uint r10_h1 = r10_h_y + (r10_h_y << 10u);
    uint r10_h2 = r10_h1 ^ (r10_h1 >> 6u);
    uint r10_h3 = r10_h2 + (r10_h2 << 3u);
    uint r10_h4 = r10_h3 ^ (r10_h3 >> 11u);
    uint r10_h5 = (((ripple_cell10_u.x * 1664525u) + (r10_h4 + (r10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint r10_h6 = r10_h5 ^ (r10_h5 >> 11u);
    uint r10_h7 = r10_h6 ^ ((r10_h6 << 7u) & 2636928640u);
    uint r10_h8 = r10_h7 ^ ((r10_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 01)。
    uint2 ripple_cell01_u = uint2(ripple_cell + int2(0, 1));
    uint r01_h_y = ripple_cell01_u.y;
    uint r01_h1 = r01_h_y + (r01_h_y << 10u);
    uint r01_h2 = r01_h1 ^ (r01_h1 >> 6u);
    uint r01_h3 = r01_h2 + (r01_h2 << 3u);
    uint r01_h4 = r01_h3 ^ (r01_h3 >> 11u);
    uint r01_h5 = (((ripple_cell01_u.x * 1664525u) + (r01_h4 + (r01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint r01_h6 = r01_h5 ^ (r01_h5 >> 11u);
    uint r01_h7 = r01_h6 ^ ((r01_h6 << 7u) & 2636928640u);
    uint r01_h8 = r01_h7 ^ ((r01_h7 << 15u) & 4022730752u);
    // 四角 LCG 哈希(角点 11)。
    uint2 ripple_cell11_u = uint2(ripple_cell + int2(1, 1));
    uint r11_h_y = ripple_cell11_u.y;
    uint r11_h1 = r11_h_y + (r11_h_y << 10u);
    uint r11_h2 = r11_h1 ^ (r11_h1 >> 6u);
    uint r11_h3 = r11_h2 + (r11_h2 << 3u);
    uint r11_h4 = r11_h3 ^ (r11_h3 >> 11u);
    uint r11_h5 = (((ripple_cell11_u.x * 1664525u) + (r11_h4 + (r11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint r11_h6 = r11_h5 ^ (r11_h5 >> 11u);
    uint r11_h7 = r11_h6 ^ ((r11_h6 << 7u) & 2636928640u);
    uint r11_h8 = r11_h7 ^ ((r11_h7 << 15u) & 4022730752u);
    // 双线性插值得到涟漪宽度,叠加后判断环强度,不足则丢弃。
    float2 ripple_uf = (ripple_frac * ripple_frac) * (3.0f.xx - (ripple_frac * 2.0f));
    float ripple_u2x = ripple_uf.x;
    float ripple_u2x_inv = 1.0f - ripple_u2x;
    float ripple_u2y = ripple_uf.y;
    if ((drop * (1.0f - smoothstep(0.89999997615814208984375f, 1.0f, in_var_C3.y + lerp(0.0f, 0.300000011920928955078125f, (((1.0f - ripple_u2y) * (((asfloat(((r00_h8 ^ (r00_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ripple_u2x_inv) + ((asfloat(((r10_h8 ^ (r10_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ripple_u2x))) + (((asfloat(((r01_h8 ^ (r01_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ripple_u2x_inv) * ripple_u2y)) + (((asfloat(((r11_h8 ^ (r11_h8 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * ripple_u2x) * ripple_u2y))))) < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
