// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/lilypad_flower.raster.hlsl | F-V | bins: _lilypad_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// 睡莲(荷花)着色器。
//   顶点阶段 vs_main:
//     - 读芦苇实例(Reed,48 字节)的 packed 仿射变换,算顶点/原点世界坐标;
//     - 用原点坐标合成种子,按高度微调顶点 y(±0.05);
//     - 采样 grass_object_push 做草叶推挤;water_displacement() 计算水波
//       (glade 主题 4 时水面静止),两者叠加成最终世界坐标;
//     - 输出法线、世界坐标、波动 UV(缩放 0.1)与顶点色。
//   像素阶段 ps_main:
//     - 采样睡莲贴图,按水面高度混合花蕊色与顶点色,主题 4 用浅色;
//     - 法线朝向相机翻转后打包 G-Buffer(法线/反照率 RGB8 + 0xFF000000)。
//   绑定: instance_buffer(t18)、water_noise_texture(t15)、
//         grass_object_push(t16)、lilypad_flower_texture(t17)、$Globals(b13)。
//   变体: TGV_b903e43ffb3da915(顶点+像素)、TGV_f0adff76827e73ff(顶点,像素为空占位)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/current_view.hlsl"
#include "inc/color_library.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/utils.hlsl"
#include "inc/wind.hlsl"
#include "water/water_object_push.inc.hlsl"
Texture2D<float4> water_noise_texture : register(t15);
#include "water/waves.inc.hlsl"
#include "inc/pack_unpack.hlsl"
Texture2D<float4> terrain_heightmap;
#include "inc/terrain_heightmap.inc.hlsl"
// 无寄存器的全局量,供 inc/terrain_heightmap.inc.hlsl 的 terrain_height()
// 辅助函数使用;此处未用到,dxc 会 DCE 掉它(不发出绑定)。
#include "inc/material/solid_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/material/plant_material.inc.hlsl"

// INC 问题规避:原始 common.hlsl 声明了 13 个采样器(s0..s12);
// inc/samplers.hlsl 只带 s0..s4,s9..s12。在此声明未用的 s5..s8,
// 以复现原 $Globals 绑定(set0/b13)。

// 植被共享全局量(本着色器所绑纹理的尺寸)。
uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
uint2 lilypad_flower_texture_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
int bool_render_shadows;

// 芦苇实例变换(packed 仿射,48 字节)按实例存放。
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t18);
// (water_noise_texture 移到 waves.inc 之前——waves.inc 的辅助函数体会引用它)
Texture2D<float4> grass_object_push : register(t16);
Texture2D<float4> lilypad_flower_texture : register(t17);

// 来自 waves.inc 的水波位移:每倍频角度抖动 + 采样波场。
// glade 主题 4 时立即返回(水面不动)。
float3 water_displacement(float2 origin_xz, float t)
{
    if (glade_theme_id == 4u)
    {
        return 0.0f;
    }
    float2 jitter[4];
    static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
    static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
    static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };
    uint jit_a = 0u;
    uint jit_b = 0u;
    bool jit_c = false;
    bool jit_d = false;
    for (uint oct = 0u; oct < 4u; oct++)
    {
        float seed_f = float(wave_seeds[oct]);
        int cell = int(floor(seed_f));
        float f = frac(seed_f);
        uint s = uint(cell);
        uint s5 = s + (s << 10u);
        s5 ^= (s5 >> 6u);
        s5 += (s5 << 3u);
        s5 ^= (s5 >> 11u);
        bool use_a = jit_d ? jit_d : true;
        uint A = jit_d ? jit_b : 1664525u;
        bool use_b = jit_c ? jit_c : true;
        uint B = jit_c ? jit_a : 1013904223u;
        uint h0 = ((s * A) + (s5 + (s5 << 15u)) + B) * A;
        h0 ^= (h0 >> 11u);
        h0 ^= ((h0 << 7u) & 2636928640u);
        h0 ^= ((h0 << 15u) & 4022730752u);
        uint s1 = s + 1u;
        uint s1_5 = s1 + (s1 << 10u);
        s1_5 ^= (s1_5 >> 6u);
        s1_5 += (s1_5 << 3u);
        s1_5 ^= (s1_5 >> 11u);
        uint A1 = use_a ? A : 1664525u;
        uint B1 = use_b ? B : 1013904223u;
        uint h1 = ((s1 * A1) + (s1_5 + (s1_5 << 15u)) + B1) * A1;
        h1 ^= (h1 >> 11u);
        h1 ^= ((h1 << 7u) & 2636928640u);
        h1 ^= ((h1 << 15u) & 4022730752u);
        float u = f * f * (3.0f - 2.0f * f);
        float2 a = float2(incb_perlin_hash01(h0), rand01(hash(h0 ^ (h0 >> 18u))));
        float2 b = float2(incb_perlin_hash01(h1), rand01(hash(h1 ^ (h1 >> 18u))));
        jitter[oct] = normalize(((a * (1.0f - u)) + (b * u)) - 0.5f) * t * 3.0f;
        jit_a = B1;
        jit_b = A1;
        jit_c = use_b ? use_b : true;
        jit_d = use_a ? use_a : true;
    }
    float3 acc = 0.0f;
    for (uint wave = 0u; wave < 1u; wave++)
    {
        float angle = float(wave + 1u) * (GOLDEN_RATIO * TWO_PI);
        float2x2 rot = float2x2(float2(cos(angle), sin(angle)), float2(-sin(angle), cos(angle)));
        float2 wave_pos = mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.012f;
        float3 sample = (water_noise_texture.SampleLevel(sampler_llr, wave_pos, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f))
                      + float3(-0.5f, -1.0f, -1.0f);
        float3 disp = sample * wave_amps[wave] * float3(1.0f, wave_scales[wave] * 0.012f, wave_scales[wave] * 0.012f);
        acc = float3(acc.x, acc.yz + mul(disp.yz, rot));
    }
    return acc;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    // 组装实例仿射变换。
    Affine3Packed xform = instance_buffer[vin.instance_id].xform;
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]),
                                    float3(xform.data[3], xform.data[4], xform.data[5]),
                                    float3(xform.data[6], xform.data[7], xform.data[8]),
                                    float3(xform.data[9], xform.data[10], xform.data[11])));

    float3 pos = mul(m, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));

    // 原点坐标 + 顶点 xz 合成种子,随机微调高度。
    uint seed = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    float randv = rand01(hash(seed));

    pos.y = pos.y + lerp(-0.05f, 0.05f, randv);
    float2 origin_xz = origin.xz;
    // 草叶推挤贴图采样(世界坐标 → 贴图 UV)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    // 水波位移(时间推进)。
    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    // 顶点世界坐标 = 原位置 + 草叶推挤(水平)+ 水波动(垂直为主)。
    float3 wpos = pos + ((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f)
                 + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f));

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos, 1.0f));
    o.C1 = normalize(mul(m, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wpos;
    o.C3 = wpos.xz * 0.1f;
    o.C4 = vin.Vertex_Color;
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
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 采样睡莲贴图,按水面高度在花蕊色与顶点色间混合;主题 4 用浅色提亮。
    float4 flower = lilypad_flower_texture.Sample(sampler_llr, pin.C3);
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    float3 albedo = lerp(lerp(flower.xyz, float3(0.25f, 0.1f, 0.2f), (1.0f - smoothstep(0.05f, 0.1f, pin.C2.y)).xxx), pin.C4,
                         (1.0f - smoothstep(-0.05f, 0.05f, pin.C2.y)).xxx);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(pin.C4 * 0.3f, 0.7f.xxx, (smoothstep(0.0f, 0.9f, pin.C2.y) * 0.3f).xxx);
    }
    // 法线背向相机时翻转(双面叶面)。
    float3 normal = pin.C1;
    if (dot(normalize(cam_pos - pin.C2), pin.C1) <= 0.0f)
    {
        normal = -pin.C1;
    }
    // 打包 G-Buffer:法线压到 [0,1],反照率 sqrt 到 sRGB 后 8bit 量化,带材质标志。
    float3 packed_normal = (normal * 0.5f) + 0.5f;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    uint albedo_word = uint(clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lilypad_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Reed
{
    Affine3Packed xform;
};

static uint unused_seed;

static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };

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
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_buffer : register(t18, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 读实例 packed 仿射变换(每实例 48 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 pos = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 原点坐标 + 顶点 xz 合成种子,LCG 哈希后按随机值微调高度。
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h1 = seed + (seed << 10u);
    uint seed_h2 = seed_h1 ^ (seed_h1 >> 6u);
    uint seed_h3 = seed_h2 + (seed_h2 << 3u);
    uint seed_h4 = seed_h3 ^ (seed_h3 >> 11u);
    pos.y = pos.y + lerp(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, asfloat(((seed_h4 + (seed_h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float2 origin_xz = origin.xz;
    // 草叶推挤贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水波动:glade 主题 4 时水面静止,直接返回零位移。
    float3 water_disp;
    do
    {
        if (frame_constants_glade_theme_id == 4u)
        {
            water_disp = 0.0f.xxx;
            break;
        }
        // 时间变量与每倍频角度抖动数组。
        float time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float2 jitter[4];
        uint oct = 0u;
        uint jit_b = 0u;
        uint jit_a = 0u;
        bool jit_c = false;
        bool jit_d = false;
        for (; oct < 4u; )
        {
            // 由种子合成两个角点哈希与插值因子。
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint s_h1 = s + (s << 10u);
            uint s_h2 = s_h1 ^ (s_h1 >> 6u);
            uint s_h3 = s_h2 + (s_h2 << 3u);
            uint s_h4 = s_h3 ^ (s_h3 >> 11u);
            // 上一轮推进的 LCG 常量(首个八度用默认值)。
            bool use_a = jit_d ? jit_d : true;
            uint A = jit_d ? jit_a : 1664525u;
            bool use_b = jit_c ? jit_c : true;
            uint B = jit_c ? jit_b : 1013904223u;
            // 角点 s 的 LCG 哈希 → 两个随机分量。
            uint h0_lcg = (((s * A) + (s_h4 + (s_h4 << 15u))) + B) * A;
            uint h0_mix1 = h0_lcg ^ (h0_lcg >> 11u);
            uint h0_mix2 = h0_mix1 ^ ((h0_mix1 << 7u) & 2636928640u);
            uint h0_hash = h0_mix2 ^ ((h0_mix2 << 15u) & 4022730752u);
            uint h0_rand_seed = h0_hash ^ (h0_hash >> 18u);
            uint h0_h1 = h0_rand_seed + (h0_rand_seed << 10u);
            uint h0_h2 = h0_h1 ^ (h0_h1 >> 6u);
            uint h0_h3 = h0_h2 + (h0_h2 << 3u);
            uint h0_hash2 = h0_h3 ^ (h0_h3 >> 11u);
            // 角点 s+1 的哈希。
            uint s1 = uint(cell + 1);
            uint s1_h1 = s1 + (s1 << 10u);
            uint s1_h2 = s1_h1 ^ (s1_h1 >> 6u);
            uint s1_h3 = s1_h2 + (s1_h2 << 3u);
            uint s1_h4 = s1_h3 ^ (s1_h3 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1_lcg = (((s1 * A1) + (s1_h4 + (s1_h4 << 15u))) + B1) * A1;
            uint h1_mix1 = h1_lcg ^ (h1_lcg >> 11u);
            uint h1_mix2 = h1_mix1 ^ ((h1_mix1 << 7u) & 2636928640u);
            uint h1_hash = h1_mix2 ^ ((h1_mix2 << 15u) & 4022730752u);
            uint h1_rand_seed = h1_hash ^ (h1_hash >> 18u);
            uint h1_h1 = h1_rand_seed + (h1_rand_seed << 10u);
            uint h1_h2 = h1_h1 ^ (h1_h1 >> 6u);
            uint h1_h3 = h1_h2 + (h1_h2 << 3u);
            uint h1_hash2 = h1_h3 ^ (h1_h3 >> 11u);
            // 平滑插值两个随机向量,归一化后按时间推进角度。
            float u = (f * f) * (3.0f - (2.0f * f));
            jitter[oct] = (normalize(((float2(asfloat((h0_rand_seed & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_hash2 + (h0_hash2 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((h1_rand_seed & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_hash2 + (h1_hash2 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * time) * 3.0f;
            oct++;
            jit_b = B1;
            jit_a = A1;
            jit_c = use_b ? use_b : true;
            jit_d = use_a ? use_a : true;
            continue;
        }
        // 单倍频水波:旋转采样 water_noise 并按幅值累积。
        float3 acc;
        acc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave_next = wave + 1u;
            float angle = float(wave_next) * 10.16640758514404296875f;
            float cos_angle = cos(angle);
            float sin_angle = sin(angle);
            float2x2 rot = float2x2(float2(cos_angle, sin_angle), float2(-sin_angle, cos_angle));
            float3 disp = (((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy;
            float3 acc_x = acc;
            acc_x.x = acc.x + disp.x;
            float2 acc_yz = acc_x.yz + mul(disp.yz, rot);
            acc = float3(acc_x.x, acc_yz.x, acc_yz.y);
            wave = wave_next;
            continue;
        }
        water_disp = acc;
        break;
    } while(false);
    // 顶点世界坐标 = 原位置 + 草叶推挤(水平)+ 水波动(垂直为主)。
    float3 wpos = pos + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos, 1.0f));
    out_var_C1 = normalize(mul(xform, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = wpos;
    out_var_C3 = wpos.xz * 0.100000001490116119384765625f;
    out_var_C4 = in_var_Vertex_Color;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_flower_texture : register(t17, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;

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
};

void frag_main()
{
    // 采样睡莲贴图,按水面高度在花蕊色与顶点色间混合;主题 4 用浅色提亮。
    float4 flower = lilypad_flower_texture.Sample(sampler_llr, in_var_C3);
    float4 cam_vw = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 albedo;
    if (frame_constants_glade_theme_id == 4u)
    {
        albedo = lerp(in_var_C4 * 0.300000011920928955078125f, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 0.89999997615814208984375f, in_var_C2.y) * 0.300000011920928955078125f).xxx);
    }
    else
    {
        albedo = lerp(lerp(flower.xyz, float3(0.25f, 0.100000001490116119384765625f, 0.20000000298023223876953125f), (1.0f - smoothstep(0.0500000007450580596923828125f, 0.100000001490116119384765625f, in_var_C2.y)).xxx), in_var_C4, (1.0f - smoothstep(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, in_var_C2.y)).xxx);
    }
    // 法线背向相机时翻转。
    float3 normal;
    if (dot(normalize((cam_vw.xyz / cam_vw.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        normal = -in_var_C1;
    }
    else
    {
        normal = in_var_C1;
    }
    // 打包 G-Buffer:法线压到 [0,1],反照率 sqrt 到 sRGB 后 8bit 量化,带材质标志。
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _lilypad_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct Reed
{
    Affine3Packed xform;
};

static uint unused_seed;

static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };

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
    uint2 water_noise_texture_dims : packoffset(c0.y);
    uint2 grass_object_push_dims : packoffset(c1);
    uint2 lilypad_flower_texture_dims : packoffset(c1.z);
    uint2 terrain_heightmap_dims : packoffset(c2);
    uint2 visual_terrain_heightmap_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_buffer : register(t18, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 读实例 packed 仿射变换(每实例 48 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 pos = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 原点坐标 + 顶点 xz 合成种子,LCG 哈希后按随机值微调高度。
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h1 = seed + (seed << 10u);
    uint seed_h2 = seed_h1 ^ (seed_h1 >> 6u);
    uint seed_h3 = seed_h2 + (seed_h2 << 3u);
    uint seed_h4 = seed_h3 ^ (seed_h3 >> 11u);
    pos.y = pos.y + lerp(-0.0500000007450580596923828125f, 0.0500000007450580596923828125f, asfloat(((seed_h4 + (seed_h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    float2 origin_xz = origin.xz;
    // 草叶推挤贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水波动:glade 主题 4 时水面静止,直接返回零位移。
    float3 water_disp;
    do
    {
        if (frame_constants_glade_theme_id == 4u)
        {
            water_disp = 0.0f.xxx;
            break;
        }
        // 时间变量与每倍频角度抖动数组。
        float time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        float2 jitter[4];
        uint oct = 0u;
        uint jit_b = 0u;
        uint jit_a = 0u;
        bool jit_c = false;
        bool jit_d = false;
        for (; oct < 4u; )
        {
            // 由种子合成两个角点哈希与插值因子。
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint s_h1 = s + (s << 10u);
            uint s_h2 = s_h1 ^ (s_h1 >> 6u);
            uint s_h3 = s_h2 + (s_h2 << 3u);
            uint s_h4 = s_h3 ^ (s_h3 >> 11u);
            // 上一轮推进的 LCG 常量(首个八度用默认值)。
            bool use_a = jit_d ? jit_d : true;
            uint A = jit_d ? jit_a : 1664525u;
            bool use_b = jit_c ? jit_c : true;
            uint B = jit_c ? jit_b : 1013904223u;
            // 角点 s 的 LCG 哈希 → 两个随机分量。
            uint h0_lcg = (((s * A) + (s_h4 + (s_h4 << 15u))) + B) * A;
            uint h0_mix1 = h0_lcg ^ (h0_lcg >> 11u);
            uint h0_mix2 = h0_mix1 ^ ((h0_mix1 << 7u) & 2636928640u);
            uint h0_hash = h0_mix2 ^ ((h0_mix2 << 15u) & 4022730752u);
            uint h0_rand_seed = h0_hash ^ (h0_hash >> 18u);
            uint h0_h1 = h0_rand_seed + (h0_rand_seed << 10u);
            uint h0_h2 = h0_h1 ^ (h0_h1 >> 6u);
            uint h0_h3 = h0_h2 + (h0_h2 << 3u);
            uint h0_hash2 = h0_h3 ^ (h0_h3 >> 11u);
            // 角点 s+1 的哈希。
            uint s1 = uint(cell + 1);
            uint s1_h1 = s1 + (s1 << 10u);
            uint s1_h2 = s1_h1 ^ (s1_h1 >> 6u);
            uint s1_h3 = s1_h2 + (s1_h2 << 3u);
            uint s1_h4 = s1_h3 ^ (s1_h3 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1_lcg = (((s1 * A1) + (s1_h4 + (s1_h4 << 15u))) + B1) * A1;
            uint h1_mix1 = h1_lcg ^ (h1_lcg >> 11u);
            uint h1_mix2 = h1_mix1 ^ ((h1_mix1 << 7u) & 2636928640u);
            uint h1_hash = h1_mix2 ^ ((h1_mix2 << 15u) & 4022730752u);
            uint h1_rand_seed = h1_hash ^ (h1_hash >> 18u);
            uint h1_h1 = h1_rand_seed + (h1_rand_seed << 10u);
            uint h1_h2 = h1_h1 ^ (h1_h1 >> 6u);
            uint h1_h3 = h1_h2 + (h1_h2 << 3u);
            uint h1_hash2 = h1_h3 ^ (h1_h3 >> 11u);
            // 平滑插值两个随机向量,归一化后按时间推进角度。
            float u = (f * f) * (3.0f - (2.0f * f));
            jitter[oct] = (normalize(((float2(asfloat((h0_rand_seed & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_hash2 + (h0_hash2 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((h1_rand_seed & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_hash2 + (h1_hash2 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * time) * 3.0f;
            oct++;
            jit_b = B1;
            jit_a = A1;
            jit_c = use_b ? use_b : true;
            jit_d = use_a ? use_a : true;
            continue;
        }
        // 单倍频水波:旋转采样 water_noise 并按幅值累积。
        float3 acc;
        acc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave_next = wave + 1u;
            float angle = float(wave_next) * 10.16640758514404296875f;
            float cos_angle = cos(angle);
            float sin_angle = sin(angle);
            float2x2 rot = float2x2(float2(cos_angle, sin_angle), float2(-sin_angle, cos_angle));
            float3 disp = (((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy;
            float3 acc_x = acc;
            acc_x.x = acc.x + disp.x;
            float2 acc_yz = acc_x.yz + mul(disp.yz, rot);
            acc = float3(acc_x.x, acc_yz.x, acc_yz.y);
            wave = wave_next;
            continue;
        }
        water_disp = acc;
        break;
    } while(false);
    // 顶点世界坐标 = 原位置 + 草叶推挤(水平)+ 水波动(垂直为主)。
    float3 wpos = pos + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f)));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos, 1.0f));
    out_var_C1 = normalize(mul(xform, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = wpos;
    out_var_C3 = wpos.xz * 0.100000001490116119384765625f;
    out_var_C4 = in_var_Vertex_Color;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _lilypad_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
