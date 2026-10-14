// ============================================================
// lilypad.raster.hlsl
// 功能:睡莲叶片渲染。vs_main(SV_InstanceID)从 instance_buffer 读 Reed 仿射变换,
//       把顶点变换到世界后做水面位移(waves.inc water_displacement)+ 草对象放置高度,
//       再叠加局部 x 轴的值噪声摆动;ps_main 用 alpha 纹理做裁剪测试并打包实体 gbuffer
//       (法线+反照率,uint2)。
// 入口:vs_main(VsIn)、ps_main(SV_Target0)。
// 变体:两个 TGV 变体(b903e43ffb3da915 / f0adff76827e73ff)顶点着色器相同,像素着色器
//       b903 为完整版(alpha 裁剪+打包),f0ad 仅 alpha 裁剪;每个变体各含普通版与
//       spv_fmsub 精确版两段提升代码。
// 绑定:t17 instance_buffer(ByteAddressBuffer,Reed xform)、t15 water_noise_texture、
//       t16 grass_object_push、t21 lilypad_alpha、s1 sampler_llr、s9 sampler_default、
//       b13 $Globals、b0 space1 frame_constants。
// ============================================================
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 手写段(非 TGV 变体时编译):语义化的 vs_main/ps_main,与下方提升段逻辑对应。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 说明:手写段为语义化参考,与提升段逻辑一一对应;
// 睡莲叶片:每实例 Reed 仿射变换 + 水面位移 + 局部 x 轴值噪声摆动 + 放置高度;
// ps 按 alpha 纹理裁剪并打包实体 gbuffer。

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
// 供 inc/terrain_heightmap.inc.hlsl 的 terrain_height() 使用的无寄存器全局;
// 此处未使用,dxc 会 DCE 掉它(不产生绑定)。
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

// INC 兼容:原始 common.hlsl 声明了 13 个采样器(s0..s12);
// inc/samplers.hlsl 只有 s0..s4,s9..s12。在此声明未用的 s5..s8,
// 以复刻原始 $Globals 绑定(set0/b13)。

// Vegetation shared globals (dims for the textures this shader binds).
uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 lilypad_normal_dims;
uint2 lilypad_alpha_dims;
int bool_render_shadows;

// Reed 实例变换(打包仿射矩阵,48 字节),每实例存储。
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t17);
// (water_noise_texture 移到 waves.inc include 之前——waves.inc 辅助函数体引用它)
Texture2D<float4> grass_object_push : register(t16);
Texture2D<float4> lilypad_alpha : register(t21);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

// 水面位移(waves.inc):每倍频抖动 + 采样波场;
// glade_theme=4 时直接返回(无水波运动)。
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

// 顶点着色器:读仿射变换,做水面位移 + 值噪声摆动 + 放置高度。
VsOut vs_main(VsIn vin)
{
    VsOut o;

    Affine3Packed xform = instance_buffer[vin.instance_id].xform;
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]),
                                    float3(xform.data[3], xform.data[4], xform.data[5]),
                                    float3(xform.data[6], xform.data[7], xform.data[8]),
                                    float3(xform.data[9], xform.data[10], xform.data[11])));

    float3 pos = mul(m, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));

    uint seed = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    float randv = rand01(hash(seed));

    pos.y = pos.y + lerp(-0.025f, 0.0f, randv);
    float2 origin_xz = origin.xz;
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    float2 disp2 = pos.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f) + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos.y, disp2.y);

    float wobble_k = vin.Vertex_Position.y + 0.5f;
    float2 wobble_pos = (origin_xz * 0.1f) + (float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    float2 wobble_u = wobble_frac * wobble_frac * (3.0f - 2.0f * wobble_frac);
    float n00 = incb_perlin_hash01(hash(uint2(wobble_cell)));
    float n10 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 1))));
    float wobble_noise = (1.0f - wobble_u.y) * ((n00 * (1.0f - wobble_u.x)) + (n10 * wobble_u.x))
                       + (n01 * (1.0f - wobble_u.x)) * wobble_u.y
                       + (n11 * wobble_u.x) * wobble_u.y;
    float wobble_rand = lerp(0.7f, 1.0f, rand01(hash(uint(int(randv)))));
    wpos.x = wpos.x - ((wobble_k * wobble_k) * sin((wobble_noise * PI) - HALF_PI + 0.3f) * 0.7f * (glade_theme_id == 4u ? 0.025f : 0.05f)) * wobble_rand;

    float3 color_pow = lerp(1.2f, 2.0f, randv).xxx;
    float3 vertex_color = pow(vin.Vertex_Color, color_pow);
    float3 color;
    if (glade_theme_id == 4u)
    {
        if (vin.Vertex_Color.y > 0.05f)
        {
            color = pow(float3(0.15f, 0.09f, 0.03f), color_pow);
        }
        else
        {
            color = vertex_color;
        }
    }
    else
    {
        color = vertex_color;
    }

    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos.x, pos.y, disp2.y, 1.0f));
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = wpos;
    o.C3 = color;
    o.C4 = (vin.Vertex_Position.xz * 1.5f) + 0.5f;
    o.C5 = randv;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// 像素着色器:alpha 裁剪测试 + 打包法线/反照率 gbuffer。
PsOut ps_main(PsIn pin)
{
    PsOut o;
    if (lilypad_alpha.SampleBias(sampler_llr, pin.C4, global_mip_bias).x < lerp(0.0f, 0.9f, pin.C5))
    {
        discard;
    }
    bool snow_branch = false;
    if (glade_theme_id == 4u)
    {
        snow_branch = pin.C2.y > 0.0f;
    }
    float3 albedo = pin.C3;
    if (snow_branch)
    {
        albedo = lerp(pin.C3, 0.7f.xxx, smoothstep(-0.2f, 1.0f, clamp(pin.C1.y, 0.0f, 1.0f)).xxx);
    }
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f;
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
// 提升段:TGV_b903e43ffb3da915 变体,顶点着色器(行为等价,语义重命名)。
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

static uint undef_u;

static const float v157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float v158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint v159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 从实例缓冲区加载 Reed 仿射变换。
    bool is_theme4;
    Affine3Packed xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        xform.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点局部坐标 → 世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点(世界坐标),用于水面位移采样。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h10 = seed + (seed << 10u);
    uint seed_h6 = seed_h10 ^ (seed_h10 >> 6u);
    uint seed_h3 = seed_h6 + (seed_h6 << 3u);
    uint seed_hash = seed_h3 ^ (seed_h3 >> 11u);
    float randv = asfloat(((seed_hash + (seed_hash << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float pos_y = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos2 = pos;
    pos2.y = pos_y;
    float2 origin_xz = origin.xz;
    // 草对象放置高度贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水面位移(waves.inc water_displacement;glade_theme=4 时直接返回 0)。
    uint v437;
    uint v438;
    bool v439;
    bool v440;
    float3 water_disp;
    do
    {
        is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            v437 = undef_u;
            v438 = undef_u;
            v439 = false;
            v440 = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint v272;
        uint v274;
        bool v276;
        bool v278;
        v272 = undef_u;
        v274 = undef_u;
        v276 = false;
        v278 = false;
        float2 v181[4];
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(v159[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint v290 = s + (s << 10u);
            uint v292 = v290 ^ (v290 >> 6u);
            uint v294 = v292 + (v292 << 3u);
            uint v296 = v294 ^ (v294 >> 11u);
            bool v299 = v278 ? v278 : true;
            uint v300 = v278 ? v274 : 1664525u;
            bool v301 = v276 ? v276 : true;
            uint v302 = v276 ? v272 : 1013904223u;
            uint v306 = (((s * v300) + (v296 + (v296 << 15u))) + v302) * v300;
            uint v308 = v306 ^ (v306 >> 11u);
            uint v311 = v308 ^ ((v308 << 7u) & 2636928640u);
            uint v314 = v311 ^ ((v311 << 15u) & 4022730752u);
            uint v316 = v314 ^ (v314 >> 18u);
            uint v318 = v316 + (v316 << 10u);
            uint v320 = v318 ^ (v318 >> 6u);
            uint v322 = v320 + (v320 << 3u);
            uint v324 = v322 ^ (v322 >> 11u);
            uint s1 = uint(cell + 1);
            uint v339 = s1 + (s1 << 10u);
            uint v341 = v339 ^ (v339 >> 6u);
            uint v343 = v341 + (v341 << 3u);
            uint v345 = v343 ^ (v343 >> 11u);
            uint v275 = v299 ? v300 : 1664525u;
            uint v273 = v301 ? v302 : 1013904223u;
            uint v351 = (((s1 * v275) + (v345 + (v345 << 15u))) + v273) * v275;
            uint v353 = v351 ^ (v351 >> 11u);
            uint v356 = v353 ^ ((v353 << 7u) & 2636928640u);
            uint v359 = v356 ^ ((v356 << 15u) & 4022730752u);
            uint v361 = v359 ^ (v359 >> 18u);
            uint v363 = v361 + (v361 << 10u);
            uint v365 = v363 ^ (v363 >> 6u);
            uint v367 = v365 + (v365 << 3u);
            uint v369 = v367 ^ (v367 >> 11u);
            float u = (f * f) * (3.0f - (2.0f * f));
            v181[oct] = (normalize(((float2(asfloat((v316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v324 + (v324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((v361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v369 + (v369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            v272 = v273;
            v274 = v275;
            v276 = v301 ? v301 : true;
            v278 = v299 ? v299 : true;
            continue;
        }
        float3 wacc;
        wacc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = wacc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * v157[wave]) + v181[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * v158[wave]) * float2(1.0f, v157[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            wacc = float3(wacc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        v437 = v272;
        v438 = v274;
        v439 = v276;
        v440 = v278;
        water_disp = wacc;
        break;
    } while(false);
    // 放置高度 + 水面位移合成最终 xz 偏移。
    float2 disp2 = pos2.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos2.y, disp2.y);
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 值噪声摆动:世界 uv + 时间。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint v467 = wobble_cell_u.y;
    uint v469 = v467 + (v467 << 10u);
    uint v471 = v469 ^ (v469 >> 6u);
    uint v473 = v471 + (v471 << 3u);
    uint v475 = v473 ^ (v473 >> 11u);
    bool v478 = v440 ? v440 : true;
    uint v479 = v440 ? v438 : 1664525u;
    bool v480 = v439 ? v439 : true;
    uint v481 = v439 ? v437 : 1013904223u;
    uint v485 = (((wobble_cell_u.x * v479) + (v475 + (v475 << 15u))) + v481) * v479;
    uint v487 = v485 ^ (v485 >> 11u);
    uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
    uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
    uint2 v501 = uint2(wobble_cell + int2(1, 0));
    uint v503 = v501.y;
    uint v505 = v503 + (v503 << 10u);
    uint v507 = v505 ^ (v505 >> 6u);
    uint v509 = v507 + (v507 << 3u);
    uint v511 = v509 ^ (v509 >> 11u);
    bool v514 = v478 ? v478 : true;
    uint v515 = v478 ? v479 : 1664525u;
    bool v516 = v480 ? v480 : true;
    uint v517 = v480 ? v481 : 1013904223u;
    uint v521 = (((v501.x * v515) + (v511 + (v511 << 15u))) + v517) * v515;
    uint v523 = v521 ^ (v521 >> 11u);
    uint v526 = v523 ^ ((v523 << 7u) & 2636928640u);
    uint v529 = v526 ^ ((v526 << 15u) & 4022730752u);
    uint2 v537 = uint2(wobble_cell + int2(0, 1));
    uint v539 = v537.y;
    uint v541 = v539 + (v539 << 10u);
    uint v543 = v541 ^ (v541 >> 6u);
    uint v545 = v543 + (v543 << 3u);
    uint v547 = v545 ^ (v545 >> 11u);
    uint v551 = v514 ? v515 : 1664525u;
    uint v553 = v516 ? v517 : 1013904223u;
    uint v557 = (((v537.x * v551) + (v547 + (v547 << 15u))) + v553) * v551;
    uint v559 = v557 ^ (v557 >> 11u);
    uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
    uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
    uint2 v573 = uint2(wobble_cell + int2(1, 1));
    uint v575 = v573.y;
    uint v577 = v575 + (v575 << 10u);
    uint v579 = v577 ^ (v577 >> 6u);
    uint v581 = v579 + (v579 << 3u);
    uint v583 = v581 ^ (v581 >> 11u);
    uint v586 = (v514 ? v514 : true) ? v551 : 1664525u;
    uint v591 = (((v573.x * v586) + (v583 + (v583 << 15u))) + ((v516 ? v516 : true) ? v553 : 1013904223u)) * v586;
    uint v593 = v591 ^ (v591 >> 11u);
    uint v596 = v593 ^ ((v593 << 7u) & 2636928640u);
    uint v599 = v596 ^ ((v596 << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_ux = wobble_u.x;
    float wobble_omux = 1.0f - wobble_ux;
    float wobble_uy = wobble_u.y;
    uint rand_seed = uint(int(randv));
    uint v633 = rand_seed + (rand_seed << 10u);
    uint v635 = v633 ^ (v633 >> 6u);
    uint v637 = v635 + (v635 << 3u);
    uint v639 = v637 ^ (v637 >> 11u);
    // 摆动:沿局部 x 轴的正弦抖动(幅度随高度平方)。
    float wpos_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_uy) * (((asfloat(((v493 ^ (v493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) + ((asfloat(((v529 ^ (v529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux))) + (((asfloat(((v565 ^ (v565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) * wobble_uy)) + (((asfloat(((v599 ^ (v599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux) * wobble_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_theme4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v639 + (v639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wpos_x;
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_theme4)
    {
        float3 snow_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            snow_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            snow_color = vertex_color;
        }
        color = snow_color;
    }
    else
    {
        color = vertex_color;
    }
    // 输出:顶点位置 + 法线(C1)/世界坐标(C2)/颜色(C3)/uv(C4)/随机(C5)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos_x, pos_y, disp2.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = wpos;
    out_var_C3 = color;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = randv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_b903e43ffb3da915 变体,像素着色器(alpha 裁剪 + 打包)。
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // alpha 遮罩测试,低于阈值丢弃。
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool snow_branch;
    if (frame_constants_glade_theme_id == 4u)
    {
        snow_branch = in_var_C2.y > 0.0f;
    }
    else
    {
        snow_branch = false;
    }
    // 雪景分支:沿法线高度混白。
    float3 albedo;
    if (snow_branch)
    {
        albedo = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        albedo = in_var_C3;
    }
    // 打包:法线 → [0,1],反照率开方后各 8 位写入 uint2 gbuffer。
    float3 packed_normal = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提升段:TGV_f0adff76827e73ff 变体,顶点着色器(行为等价,语义重命名)。
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

static uint undef_u;

static const float v157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float v158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint v159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 从实例缓冲区加载 Reed 仿射变换。
    bool is_theme4;
    Affine3Packed xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        xform.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点局部坐标 → 世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点(世界坐标),用于水面位移采样。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h10 = seed + (seed << 10u);
    uint seed_h6 = seed_h10 ^ (seed_h10 >> 6u);
    uint seed_h3 = seed_h6 + (seed_h6 << 3u);
    uint seed_hash = seed_h3 ^ (seed_h3 >> 11u);
    float randv = asfloat(((seed_hash + (seed_hash << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float pos_y = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos2 = pos;
    pos2.y = pos_y;
    float2 origin_xz = origin.xz;
    // 草对象放置高度贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水面位移(waves.inc water_displacement;glade_theme=4 时直接返回 0)。
    uint v437;
    uint v438;
    bool v439;
    bool v440;
    float3 water_disp;
    do
    {
        is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            v437 = undef_u;
            v438 = undef_u;
            v439 = false;
            v440 = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint v272;
        uint v274;
        bool v276;
        bool v278;
        v272 = undef_u;
        v274 = undef_u;
        v276 = false;
        v278 = false;
        float2 v181[4];
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(v159[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint v290 = s + (s << 10u);
            uint v292 = v290 ^ (v290 >> 6u);
            uint v294 = v292 + (v292 << 3u);
            uint v296 = v294 ^ (v294 >> 11u);
            bool v299 = v278 ? v278 : true;
            uint v300 = v278 ? v274 : 1664525u;
            bool v301 = v276 ? v276 : true;
            uint v302 = v276 ? v272 : 1013904223u;
            uint v306 = (((s * v300) + (v296 + (v296 << 15u))) + v302) * v300;
            uint v308 = v306 ^ (v306 >> 11u);
            uint v311 = v308 ^ ((v308 << 7u) & 2636928640u);
            uint v314 = v311 ^ ((v311 << 15u) & 4022730752u);
            uint v316 = v314 ^ (v314 >> 18u);
            uint v318 = v316 + (v316 << 10u);
            uint v320 = v318 ^ (v318 >> 6u);
            uint v322 = v320 + (v320 << 3u);
            uint v324 = v322 ^ (v322 >> 11u);
            uint s1 = uint(cell + 1);
            uint v339 = s1 + (s1 << 10u);
            uint v341 = v339 ^ (v339 >> 6u);
            uint v343 = v341 + (v341 << 3u);
            uint v345 = v343 ^ (v343 >> 11u);
            uint v275 = v299 ? v300 : 1664525u;
            uint v273 = v301 ? v302 : 1013904223u;
            uint v351 = (((s1 * v275) + (v345 + (v345 << 15u))) + v273) * v275;
            uint v353 = v351 ^ (v351 >> 11u);
            uint v356 = v353 ^ ((v353 << 7u) & 2636928640u);
            uint v359 = v356 ^ ((v356 << 15u) & 4022730752u);
            uint v361 = v359 ^ (v359 >> 18u);
            uint v363 = v361 + (v361 << 10u);
            uint v365 = v363 ^ (v363 >> 6u);
            uint v367 = v365 + (v365 << 3u);
            uint v369 = v367 ^ (v367 >> 11u);
            float u = (f * f) * (3.0f - (2.0f * f));
            v181[oct] = (normalize(((float2(asfloat((v316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v324 + (v324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((v361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v369 + (v369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            v272 = v273;
            v274 = v275;
            v276 = v301 ? v301 : true;
            v278 = v299 ? v299 : true;
            continue;
        }
        float3 wacc;
        wacc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = wacc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * v157[wave]) + v181[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * v158[wave]) * float2(1.0f, v157[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            wacc = float3(wacc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        v437 = v272;
        v438 = v274;
        v439 = v276;
        v440 = v278;
        water_disp = wacc;
        break;
    } while(false);
    // 放置高度 + 水面位移合成最终 xz 偏移。
    float2 disp2 = pos2.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos2.y, disp2.y);
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 值噪声摆动:世界 uv + 时间。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint v467 = wobble_cell_u.y;
    uint v469 = v467 + (v467 << 10u);
    uint v471 = v469 ^ (v469 >> 6u);
    uint v473 = v471 + (v471 << 3u);
    uint v475 = v473 ^ (v473 >> 11u);
    bool v478 = v440 ? v440 : true;
    uint v479 = v440 ? v438 : 1664525u;
    bool v480 = v439 ? v439 : true;
    uint v481 = v439 ? v437 : 1013904223u;
    uint v485 = (((wobble_cell_u.x * v479) + (v475 + (v475 << 15u))) + v481) * v479;
    uint v487 = v485 ^ (v485 >> 11u);
    uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
    uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
    uint2 v501 = uint2(wobble_cell + int2(1, 0));
    uint v503 = v501.y;
    uint v505 = v503 + (v503 << 10u);
    uint v507 = v505 ^ (v505 >> 6u);
    uint v509 = v507 + (v507 << 3u);
    uint v511 = v509 ^ (v509 >> 11u);
    bool v514 = v478 ? v478 : true;
    uint v515 = v478 ? v479 : 1664525u;
    bool v516 = v480 ? v480 : true;
    uint v517 = v480 ? v481 : 1013904223u;
    uint v521 = (((v501.x * v515) + (v511 + (v511 << 15u))) + v517) * v515;
    uint v523 = v521 ^ (v521 >> 11u);
    uint v526 = v523 ^ ((v523 << 7u) & 2636928640u);
    uint v529 = v526 ^ ((v526 << 15u) & 4022730752u);
    uint2 v537 = uint2(wobble_cell + int2(0, 1));
    uint v539 = v537.y;
    uint v541 = v539 + (v539 << 10u);
    uint v543 = v541 ^ (v541 >> 6u);
    uint v545 = v543 + (v543 << 3u);
    uint v547 = v545 ^ (v545 >> 11u);
    uint v551 = v514 ? v515 : 1664525u;
    uint v553 = v516 ? v517 : 1013904223u;
    uint v557 = (((v537.x * v551) + (v547 + (v547 << 15u))) + v553) * v551;
    uint v559 = v557 ^ (v557 >> 11u);
    uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
    uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
    uint2 v573 = uint2(wobble_cell + int2(1, 1));
    uint v575 = v573.y;
    uint v577 = v575 + (v575 << 10u);
    uint v579 = v577 ^ (v577 >> 6u);
    uint v581 = v579 + (v579 << 3u);
    uint v583 = v581 ^ (v581 >> 11u);
    uint v586 = (v514 ? v514 : true) ? v551 : 1664525u;
    uint v591 = (((v573.x * v586) + (v583 + (v583 << 15u))) + ((v516 ? v516 : true) ? v553 : 1013904223u)) * v586;
    uint v593 = v591 ^ (v591 >> 11u);
    uint v596 = v593 ^ ((v593 << 7u) & 2636928640u);
    uint v599 = v596 ^ ((v596 << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_ux = wobble_u.x;
    float wobble_omux = 1.0f - wobble_ux;
    float wobble_uy = wobble_u.y;
    uint rand_seed = uint(int(randv));
    uint v633 = rand_seed + (rand_seed << 10u);
    uint v635 = v633 ^ (v633 >> 6u);
    uint v637 = v635 + (v635 << 3u);
    uint v639 = v637 ^ (v637 >> 11u);
    // 摆动:沿局部 x 轴的正弦抖动(幅度随高度平方)。
    float wpos_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_uy) * (((asfloat(((v493 ^ (v493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) + ((asfloat(((v529 ^ (v529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux))) + (((asfloat(((v565 ^ (v565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) * wobble_uy)) + (((asfloat(((v599 ^ (v599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux) * wobble_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_theme4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v639 + (v639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wpos_x;
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_theme4)
    {
        float3 snow_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            snow_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            snow_color = vertex_color;
        }
        color = snow_color;
    }
    else
    {
        color = vertex_color;
    }
    // 输出:顶点位置 + 法线(C1)/世界坐标(C2)/颜色(C3)/uv(C4)/随机(C5)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos_x, pos_y, disp2.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = wpos;
    out_var_C3 = color;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = randv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_f0adff76827e73ff 变体,像素着色器(仅 alpha 裁剪)。
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    // alpha 遮罩测试,低于阈值丢弃。
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提升段:TGV_b903e43ffb3da915 变体,顶点着色器(行为等价,语义重命名)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 会把 (a*c)-(b*c) 因式分解为 (a-b)*c,舍入结果与原版不同;precise 临时量保持两个乘积。
template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

static uint undef_u;

static const float v157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float v158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint v159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 从实例缓冲区加载 Reed 仿射变换。
    bool is_theme4;
    Affine3Packed xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        xform.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点局部坐标 → 世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点(世界坐标),用于水面位移采样。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint seed = uint(((spv_fmsub(origin.x, origin.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h10 = seed + (seed << 10u);
    uint seed_h6 = seed_h10 ^ (seed_h10 >> 6u);
    uint seed_h3 = seed_h6 + (seed_h6 << 3u);
    uint seed_hash = seed_h3 ^ (seed_h3 >> 11u);
    float randv = asfloat(((seed_hash + (seed_hash << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float pos_y = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos2 = pos;
    pos2.y = pos_y;
    float2 origin_xz = origin.xz;
    // 草对象放置高度贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水面位移(waves.inc water_displacement;glade_theme=4 时直接返回 0)。
    uint v437;
    uint v438;
    bool v439;
    bool v440;
    float3 water_disp;
    do
    {
        is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            v437 = undef_u;
            v438 = undef_u;
            v439 = false;
            v440 = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint v272;
        uint v274;
        bool v276;
        bool v278;
        v272 = undef_u;
        v274 = undef_u;
        v276 = false;
        v278 = false;
        float2 v181[4];
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(v159[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint v290 = s + (s << 10u);
            uint v292 = v290 ^ (v290 >> 6u);
            uint v294 = v292 + (v292 << 3u);
            uint v296 = v294 ^ (v294 >> 11u);
            bool v299 = v278 ? v278 : true;
            uint v300 = v278 ? v274 : 1664525u;
            bool v301 = v276 ? v276 : true;
            uint v302 = v276 ? v272 : 1013904223u;
            uint v306 = (((s * v300) + (v296 + (v296 << 15u))) + v302) * v300;
            uint v308 = v306 ^ (v306 >> 11u);
            uint v311 = v308 ^ ((v308 << 7u) & 2636928640u);
            uint v314 = v311 ^ ((v311 << 15u) & 4022730752u);
            uint v316 = v314 ^ (v314 >> 18u);
            uint v318 = v316 + (v316 << 10u);
            uint v320 = v318 ^ (v318 >> 6u);
            uint v322 = v320 + (v320 << 3u);
            uint v324 = v322 ^ (v322 >> 11u);
            uint s1 = uint(cell + 1);
            uint v339 = s1 + (s1 << 10u);
            uint v341 = v339 ^ (v339 >> 6u);
            uint v343 = v341 + (v341 << 3u);
            uint v345 = v343 ^ (v343 >> 11u);
            uint v275 = v299 ? v300 : 1664525u;
            uint v273 = v301 ? v302 : 1013904223u;
            uint v351 = (((s1 * v275) + (v345 + (v345 << 15u))) + v273) * v275;
            uint v353 = v351 ^ (v351 >> 11u);
            uint v356 = v353 ^ ((v353 << 7u) & 2636928640u);
            uint v359 = v356 ^ ((v356 << 15u) & 4022730752u);
            uint v361 = v359 ^ (v359 >> 18u);
            uint v363 = v361 + (v361 << 10u);
            uint v365 = v363 ^ (v363 >> 6u);
            uint v367 = v365 + (v365 << 3u);
            uint v369 = v367 ^ (v367 >> 11u);
            float u = (f * f) * (3.0f - (2.0f * f));
            v181[oct] = (normalize(((float2(asfloat((v316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v324 + (v324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((v361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v369 + (v369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            v272 = v273;
            v274 = v275;
            v276 = v301 ? v301 : true;
            v278 = v299 ? v299 : true;
            continue;
        }
        float3 wacc;
        wacc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = wacc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * v157[wave]) + v181[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * v158[wave]) * float2(1.0f, v157[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            wacc = float3(wacc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        v437 = v272;
        v438 = v274;
        v439 = v276;
        v440 = v278;
        water_disp = wacc;
        break;
    } while(false);
    // 放置高度 + 水面位移合成最终 xz 偏移。
    float2 disp2 = pos2.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos2.y, disp2.y);
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 值噪声摆动:世界 uv + 时间。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint v467 = wobble_cell_u.y;
    uint v469 = v467 + (v467 << 10u);
    uint v471 = v469 ^ (v469 >> 6u);
    uint v473 = v471 + (v471 << 3u);
    uint v475 = v473 ^ (v473 >> 11u);
    bool v478 = v440 ? v440 : true;
    uint v479 = v440 ? v438 : 1664525u;
    bool v480 = v439 ? v439 : true;
    uint v481 = v439 ? v437 : 1013904223u;
    uint v485 = (((wobble_cell_u.x * v479) + (v475 + (v475 << 15u))) + v481) * v479;
    uint v487 = v485 ^ (v485 >> 11u);
    uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
    uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
    uint2 v501 = uint2(wobble_cell + int2(1, 0));
    uint v503 = v501.y;
    uint v505 = v503 + (v503 << 10u);
    uint v507 = v505 ^ (v505 >> 6u);
    uint v509 = v507 + (v507 << 3u);
    uint v511 = v509 ^ (v509 >> 11u);
    bool v514 = v478 ? v478 : true;
    uint v515 = v478 ? v479 : 1664525u;
    bool v516 = v480 ? v480 : true;
    uint v517 = v480 ? v481 : 1013904223u;
    uint v521 = (((v501.x * v515) + (v511 + (v511 << 15u))) + v517) * v515;
    uint v523 = v521 ^ (v521 >> 11u);
    uint v526 = v523 ^ ((v523 << 7u) & 2636928640u);
    uint v529 = v526 ^ ((v526 << 15u) & 4022730752u);
    uint2 v537 = uint2(wobble_cell + int2(0, 1));
    uint v539 = v537.y;
    uint v541 = v539 + (v539 << 10u);
    uint v543 = v541 ^ (v541 >> 6u);
    uint v545 = v543 + (v543 << 3u);
    uint v547 = v545 ^ (v545 >> 11u);
    uint v551 = v514 ? v515 : 1664525u;
    uint v553 = v516 ? v517 : 1013904223u;
    uint v557 = (((v537.x * v551) + (v547 + (v547 << 15u))) + v553) * v551;
    uint v559 = v557 ^ (v557 >> 11u);
    uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
    uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
    uint2 v573 = uint2(wobble_cell + int2(1, 1));
    uint v575 = v573.y;
    uint v577 = v575 + (v575 << 10u);
    uint v579 = v577 ^ (v577 >> 6u);
    uint v581 = v579 + (v579 << 3u);
    uint v583 = v581 ^ (v581 >> 11u);
    uint v586 = (v514 ? v514 : true) ? v551 : 1664525u;
    uint v591 = (((v573.x * v586) + (v583 + (v583 << 15u))) + ((v516 ? v516 : true) ? v553 : 1013904223u)) * v586;
    uint v593 = v591 ^ (v591 >> 11u);
    uint v596 = v593 ^ ((v593 << 7u) & 2636928640u);
    uint v599 = v596 ^ ((v596 << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_ux = wobble_u.x;
    float wobble_omux = 1.0f - wobble_ux;
    float wobble_uy = wobble_u.y;
    uint rand_seed = uint(int(randv));
    uint v633 = rand_seed + (rand_seed << 10u);
    uint v635 = v633 ^ (v633 >> 6u);
    uint v637 = v635 + (v635 << 3u);
    uint v639 = v637 ^ (v637 >> 11u);
    // 摆动:沿局部 x 轴的正弦抖动(幅度随高度平方)。
    float wpos_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_uy) * (((asfloat(((v493 ^ (v493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) + ((asfloat(((v529 ^ (v529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux))) + (((asfloat(((v565 ^ (v565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) * wobble_uy)) + (((asfloat(((v599 ^ (v599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux) * wobble_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_theme4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v639 + (v639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wpos_x;
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_theme4)
    {
        float3 snow_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            snow_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            snow_color = vertex_color;
        }
        color = snow_color;
    }
    else
    {
        color = vertex_color;
    }
    // 输出:顶点位置 + 法线(C1)/世界坐标(C2)/颜色(C3)/uv(C4)/随机(C5)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos_x, pos_y, disp2.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = wpos;
    out_var_C3 = color;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = randv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_b903e43ffb3da915 变体,像素着色器(alpha 裁剪 + 打包)。
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static float2 in_var_C4;
static float in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // alpha 遮罩测试,低于阈值丢弃。
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
    bool snow_branch;
    if (frame_constants_glade_theme_id == 4u)
    {
        snow_branch = in_var_C2.y > 0.0f;
    }
    else
    {
        snow_branch = false;
    }
    // 雪景分支:沿法线高度混白。
    float3 albedo;
    if (snow_branch)
    {
        albedo = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        albedo = in_var_C3;
    }
    // 打包:法线 → [0,1],反照率开方后各 8 位写入 uint2 gbuffer。
    float3 packed_normal = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提升段:TGV_f0adff76827e73ff 变体,顶点着色器(行为等价,语义重命名)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc 会把 (a*c)-(b*c) 因式分解为 (a-b)*c,舍入结果与原版不同;precise 临时量保持两个乘积。
template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
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

static uint undef_u;

static const float v157[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float v158[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint v159[4] = { 17u, 18u, 20u, 21u };

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 lilypad_normal_dims : packoffset(c2.z);
    uint2 lilypad_alpha_dims : packoffset(c3);
    int bool_render_shadows : packoffset(c3.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float2 out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 从实例缓冲区加载 Reed 仿射变换。
    bool is_theme4;
    Affine3Packed xform;
    [unroll]
    for (int _3ident = 0; _3ident < 12; _3ident++)
    {
        xform.data[_3ident] = instance_buffer.Load<float>(_3ident * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点局部坐标 → 世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点(世界坐标),用于水面位移采样。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint seed = uint(((spv_fmsub(origin.x, origin.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h10 = seed + (seed << 10u);
    uint seed_h6 = seed_h10 ^ (seed_h10 >> 6u);
    uint seed_h3 = seed_h6 + (seed_h6 << 3u);
    uint seed_hash = seed_h3 ^ (seed_h3 >> 11u);
    float randv = asfloat(((seed_hash + (seed_hash << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float pos_y = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos2 = pos;
    pos2.y = pos_y;
    float2 origin_xz = origin.xz;
    // 草对象放置高度贴图采样。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    // 水面位移(waves.inc water_displacement;glade_theme=4 时直接返回 0)。
    uint v437;
    uint v438;
    bool v439;
    bool v440;
    float3 water_disp;
    do
    {
        is_theme4 = frame_constants_glade_theme_id == 4u;
        if (is_theme4)
        {
            v437 = undef_u;
            v438 = undef_u;
            v439 = false;
            v440 = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint v272;
        uint v274;
        bool v276;
        bool v278;
        v272 = undef_u;
        v274 = undef_u;
        v276 = false;
        v278 = false;
        float2 v181[4];
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(v159[oct]);
            int cell = int(floor(seed_f));
            float f = frac(seed_f);
            uint s = uint(cell);
            uint v290 = s + (s << 10u);
            uint v292 = v290 ^ (v290 >> 6u);
            uint v294 = v292 + (v292 << 3u);
            uint v296 = v294 ^ (v294 >> 11u);
            bool v299 = v278 ? v278 : true;
            uint v300 = v278 ? v274 : 1664525u;
            bool v301 = v276 ? v276 : true;
            uint v302 = v276 ? v272 : 1013904223u;
            uint v306 = (((s * v300) + (v296 + (v296 << 15u))) + v302) * v300;
            uint v308 = v306 ^ (v306 >> 11u);
            uint v311 = v308 ^ ((v308 << 7u) & 2636928640u);
            uint v314 = v311 ^ ((v311 << 15u) & 4022730752u);
            uint v316 = v314 ^ (v314 >> 18u);
            uint v318 = v316 + (v316 << 10u);
            uint v320 = v318 ^ (v318 >> 6u);
            uint v322 = v320 + (v320 << 3u);
            uint v324 = v322 ^ (v322 >> 11u);
            uint s1 = uint(cell + 1);
            uint v339 = s1 + (s1 << 10u);
            uint v341 = v339 ^ (v339 >> 6u);
            uint v343 = v341 + (v341 << 3u);
            uint v345 = v343 ^ (v343 >> 11u);
            uint v275 = v299 ? v300 : 1664525u;
            uint v273 = v301 ? v302 : 1013904223u;
            uint v351 = (((s1 * v275) + (v345 + (v345 << 15u))) + v273) * v275;
            uint v353 = v351 ^ (v351 >> 11u);
            uint v356 = v353 ^ ((v353 << 7u) & 2636928640u);
            uint v359 = v356 ^ ((v356 << 15u) & 4022730752u);
            uint v361 = v359 ^ (v359 >> 18u);
            uint v363 = v361 + (v361 << 10u);
            uint v365 = v363 ^ (v363 >> 6u);
            uint v367 = v365 + (v365 << 3u);
            uint v369 = v367 ^ (v367 >> 11u);
            float u = (f * f) * (3.0f - (2.0f * f));
            v181[oct] = (normalize(((float2(asfloat((v316 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v324 + (v324 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - u)) + (float2(asfloat((v361 & 8388607u) | 1065353216u) - 1.0f, asfloat(((v369 + (v369 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            v272 = v273;
            v274 = v275;
            v276 = v301 ? v301 : true;
            v278 = v299 ? v299 : true;
            continue;
        }
        float3 wacc;
        wacc = 0.0f.xxx;
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = wacc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * v157[wave]) + v181[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * v158[wave]) * float2(1.0f, v157[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            wacc = float3(wacc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        v437 = v272;
        v438 = v274;
        v439 = v276;
        v440 = v278;
        water_disp = wacc;
        break;
    } while(false);
    // 放置高度 + 水面位移合成最终 xz 偏移。
    float2 disp2 = pos2.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos2.y, disp2.y);
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 值噪声摆动:世界 uv + 时间。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint v467 = wobble_cell_u.y;
    uint v469 = v467 + (v467 << 10u);
    uint v471 = v469 ^ (v469 >> 6u);
    uint v473 = v471 + (v471 << 3u);
    uint v475 = v473 ^ (v473 >> 11u);
    bool v478 = v440 ? v440 : true;
    uint v479 = v440 ? v438 : 1664525u;
    bool v480 = v439 ? v439 : true;
    uint v481 = v439 ? v437 : 1013904223u;
    uint v485 = (((wobble_cell_u.x * v479) + (v475 + (v475 << 15u))) + v481) * v479;
    uint v487 = v485 ^ (v485 >> 11u);
    uint v490 = v487 ^ ((v487 << 7u) & 2636928640u);
    uint v493 = v490 ^ ((v490 << 15u) & 4022730752u);
    uint2 v501 = uint2(wobble_cell + int2(1, 0));
    uint v503 = v501.y;
    uint v505 = v503 + (v503 << 10u);
    uint v507 = v505 ^ (v505 >> 6u);
    uint v509 = v507 + (v507 << 3u);
    uint v511 = v509 ^ (v509 >> 11u);
    bool v514 = v478 ? v478 : true;
    uint v515 = v478 ? v479 : 1664525u;
    bool v516 = v480 ? v480 : true;
    uint v517 = v480 ? v481 : 1013904223u;
    uint v521 = (((v501.x * v515) + (v511 + (v511 << 15u))) + v517) * v515;
    uint v523 = v521 ^ (v521 >> 11u);
    uint v526 = v523 ^ ((v523 << 7u) & 2636928640u);
    uint v529 = v526 ^ ((v526 << 15u) & 4022730752u);
    uint2 v537 = uint2(wobble_cell + int2(0, 1));
    uint v539 = v537.y;
    uint v541 = v539 + (v539 << 10u);
    uint v543 = v541 ^ (v541 >> 6u);
    uint v545 = v543 + (v543 << 3u);
    uint v547 = v545 ^ (v545 >> 11u);
    uint v551 = v514 ? v515 : 1664525u;
    uint v553 = v516 ? v517 : 1013904223u;
    uint v557 = (((v537.x * v551) + (v547 + (v547 << 15u))) + v553) * v551;
    uint v559 = v557 ^ (v557 >> 11u);
    uint v562 = v559 ^ ((v559 << 7u) & 2636928640u);
    uint v565 = v562 ^ ((v562 << 15u) & 4022730752u);
    uint2 v573 = uint2(wobble_cell + int2(1, 1));
    uint v575 = v573.y;
    uint v577 = v575 + (v575 << 10u);
    uint v579 = v577 ^ (v577 >> 6u);
    uint v581 = v579 + (v579 << 3u);
    uint v583 = v581 ^ (v581 >> 11u);
    uint v586 = (v514 ? v514 : true) ? v551 : 1664525u;
    uint v591 = (((v573.x * v586) + (v583 + (v583 << 15u))) + ((v516 ? v516 : true) ? v553 : 1013904223u)) * v586;
    uint v593 = v591 ^ (v591 >> 11u);
    uint v596 = v593 ^ ((v593 << 7u) & 2636928640u);
    uint v599 = v596 ^ ((v596 << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_ux = wobble_u.x;
    float wobble_omux = 1.0f - wobble_ux;
    float wobble_uy = wobble_u.y;
    uint rand_seed = uint(int(randv));
    uint v633 = rand_seed + (rand_seed << 10u);
    uint v635 = v633 ^ (v633 >> 6u);
    uint v637 = v635 + (v635 << 3u);
    uint v639 = v637 ^ (v637 >> 11u);
    // 摆动:沿局部 x 轴的正弦抖动(幅度随高度平方)。
    float wpos_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_uy) * (((asfloat(((v493 ^ (v493 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) + ((asfloat(((v529 ^ (v529 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux))) + (((asfloat(((v565 ^ (v565 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_omux) * wobble_uy)) + (((asfloat(((v599 ^ (v599 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_ux) * wobble_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_theme4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v639 + (v639 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wpos_x;
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_theme4)
    {
        float3 snow_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            snow_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            snow_color = vertex_color;
        }
        color = snow_color;
    }
    else
    {
        color = vertex_color;
    }
    // 输出:顶点位置 + 法线(C1)/世界坐标(C2)/颜色(C3)/uv(C4)/随机(C5)。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wpos_x, pos_y, disp2.y, 1.0f));
    out_var_C1 = float3(0.0f, 1.0f, 0.0f);
    out_var_C2 = wpos;
    out_var_C3 = color;
    out_var_C4 = (in_var_Vertex_Position.xz * 1.5f) + 0.5f.xx;
    out_var_C5 = randv;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 提升段:TGV_f0adff76827e73ff 变体,像素着色器(仅 alpha 裁剪)。
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

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> lilypad_alpha : register(t21, space0);

static float2 in_var_C4;
static float in_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
};

void frag_main()
{
    // alpha 遮罩测试,低于阈值丢弃。
    if (lilypad_alpha.SampleBias(sampler_llr, in_var_C4, frame_constants_global_mip_bias).x < lerp(0.0f, 0.89999997615814208984375f, in_var_C5))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
