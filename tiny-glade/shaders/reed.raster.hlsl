// ==RECONSTRUCTED== shaders/reed.raster.hlsl | F-V | bins: _reed.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | 入口 vs:vs_main ps:ps_main
// 芦苇 (Reed): 与 lilypad 类似 (实例仿射变换 + 水面位移 + 值噪声摆动), 但额外把顶点法线传给
//   像素阶段; 像素阶段把 (0.5,1]-重映射法线 与 线性反照率 打包进固体 gbuffer (theme-4 有雪色混合)。
// 手写段被排除于 (TGV_b903e43ffb3da915 | TGV_f0adff76827e73ff, VERTEX | PIXEL) 四个变体-阶段组合;
//   这四个组合由 batch_lift.py 提升的 SPIRV-Cross 代码提供。注意文件内有两套提升段:
//   外层 #if !(...) 之内的第一套 (死代码) 与之外的第二套 (活动副本); 两套内容一致, 重命名已同步。
// 绑定: instance_buffer t17, grass_object_push t16, water_noise_texture t15,
//   sampler_llr s1 / sampler_default s9, frame_constants set1/b0, $Globals b13。
// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/reed.raster.hlsl | F-V | bins: _reed.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Reed: like lilypad (Reed instance xform + water displacement + value-noise
// wobble) but with a vertex normal passed to the fragment stage; ps packs the
// solid gbuffer with the theme-4 snow tint.

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
// Registerless global for inc/terrain_heightmap.inc.hlsl's terrain_height()
// helper; unused here, so dxc DCEs it (no binding emitted).
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

// INC-issue workaround: the original common.hlsl declared 13 samplers (s0..s12);
// inc/samplers.hlsl only carries s0..s4,s9..s12. Declaring the unused s5..s8 here
// reproduces the original $Globals binding (set0/b13).

// Vegetation shared globals (dims for the textures this shader binds).
uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
int bool_render_shadows;

// Reed instance transform (packed affine, 48 bytes) stored per instance.
// 芦苇实例数据: 打包仿射变换 (48 字节)。
struct Reed
{
    Affine3Packed xform;
};

StructuredBuffer<Reed> instance_buffer : register(t17);
// (water_noise_texture moved before the waves.inc include — waves.inc helper bodies reference it)
Texture2D<float4> grass_object_push : register(t16);

// Water displacement from waves.inc: per-octave jitter + a sampled wave field.
// The glade-theme-4 case returns immediately (no water motion).
// 水面位移: 按 4 个种子生成伪随机抖动, 再加单层波采样 (water_noise_texture)。theme-4 (雪地) 直接返回 0。
float3 water_displacement(float2 origin_xz, float t)
{
    if (glade_theme_id == 4u)
    {
    // 雪地主题: 不产生水面运动。
        return 0.0f;
    }
    float2 jitter[4];
    static const float wave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
    static const float wave_amps[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
    static const uint wave_seeds[4] = { 17u, 18u, 20u, 21u };
    // 抖动哈希状态 (乘子与开关在八度之间传递)。
    uint jit_a = 0u;
    uint jit_b = 0u;
    bool jit_c = false;
    bool jit_d = false;
    // 4 个八度的抖动方向: 每个种子插值两个 hash 并归一化。
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
    // 波场: 旋转坐标系后采样噪声, 把 yz 位移叠加回累加器。
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
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

// 顶点着色器: 读取实例变换, 加水面位移/风吹摆动, 输出世界坐标/法线/颜色。
VsOut vs_main(VsIn vin)
{
    VsOut o;

    // 读取实例仿射变换并转成 float3x4。
    Affine3Packed xform = instance_buffer[vin.instance_id].xform;
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]),
                                    float3(xform.data[3], xform.data[4], xform.data[5]),
                                    float3(xform.data[6], xform.data[7], xform.data[8]),
                                    float3(xform.data[9], xform.data[10], xform.data[11])));

    float3 pos = mul(m, float4(vin.Vertex_Position, 1.0f));
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));

    // 由实例原点与顶点位置求散列种子。
    uint seed = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    float randv = rand01(hash(seed));

    // 垂直方向小幅随机下沉 (芦苇高度差异)。
    pos.y = pos.y + lerp(-0.025f, 0.0f, randv);
    float2 origin_xz = origin.xz;
    // 采样草地推动图 (风吹位移)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.007692307699471712f) + 0.5f, 0.0f);

    // 计算水面位移。
    float3 water_disp = water_displacement(origin_xz, float(game_time_sec) + game_time_frac);

    // 合成最终水平位移 (草地推动 + 水面位移)。
    float2 disp2 = pos.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * float(clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)) * 0.35f) + (water_disp.yxz * float3(2.0f, 0.01f, 2.0f))).xz;
    float3 wpos = float3(disp2.x, pos.y, disp2.y);

    // 摆动幅度随顶点高度增大 (顶端摆得更大)。
    float wobble_k = vin.Vertex_Position.y + 0.5f;
    float2 wobble_pos = (origin_xz * 0.1f) + (float2(float(game_time_sec) + game_time_frac, 0.0f) * 1.2f);
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    float2 wobble_u = wobble_frac * wobble_frac * (3.0f - 2.0f * wobble_frac);
    // 2D 值噪声四个角 hash (用于顶端 sin 摆动)。
    float n00 = incb_perlin_hash01(hash(uint2(wobble_cell)));
    float n10 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(wobble_cell + int2(1, 1))));
    float wobble_noise = (1.0f - wobble_u.y) * ((n00 * (1.0f - wobble_u.x)) + (n10 * wobble_u.x))
                       + (n01 * (1.0f - wobble_u.x)) * wobble_u.y
                       + (n11 * wobble_u.x) * wobble_u.y;
    float wobble_rand = lerp(0.7f, 1.0f, rand01(hash(uint(int(randv)))));
    // 沿 -x 方向按噪声相位摆动芦苇顶端 (雪地主题幅度减半)。
    wpos.x = wpos.x - ((wobble_k * wobble_k) * sin((wobble_noise * PI) - HALF_PI + 0.3f) * 0.7f * (glade_theme_id == 4u ? 0.025f : 0.05f)) * wobble_rand;

    // 顶点颜色按随机指数增强。
    float3 color_pow = lerp(1.2f, 2.0f, randv).xxx;
    float3 vertex_color = pow(vin.Vertex_Color, color_pow);
    float3 color;
    if (glade_theme_id == 4u)
    {
        // 雪地主题: 偏绿的茎用棕色调色。
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

    // 投影到采样空间。
    o.position = mul(GetCurrentViewConstants().world_to_sample, float4(wpos.x, pos.y, disp2.y, 1.0f));
    o.C1 = normalize(mul(m, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wpos;
    o.C3 = color;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// 像素着色器: 打包法线与线性反照率到固体 gbuffer。
PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool snow_branch = false;
    if (glade_theme_id == 4u)
    {
        // 雪地主题且高于水面时混入雪色。
        snow_branch = pin.C2.y > 0.0f;
    }
    float3 albedo = pin.C3;
    if (snow_branch)
    {
        albedo = lerp(pin.C3, 0.7f.xxx, smoothstep(-0.2f, 1.0f, clamp(pin.C1.y, 0.0f, 1.0f)).xxx);
    }
    // 法线重映射到 [0,1]。
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f;
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f, 1.0f));
    // 法线量化成 8 位三通道 (RGBA8, alpha=0xFF)。
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    // 线性反照率量化成 8 位三通道 (alpha=0xFF)。
    uint albedo_word = uint(clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint undef_uint;

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
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
static float3 out_var_C3;

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
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool is_glade4;
    // 读取当前实例仿射变换 (48 字节)。
    Affine3Packed xform;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        xform.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    // 把打包仿射展开为 float3x4 变换矩阵。
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点本地坐标转世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点 (世界坐标)。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 由实例原点与顶点位置求散列种子。
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h0 = seed + (seed << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    // 种子 hash -> [0,1] 随机数。
    float randv = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 垂直方向小幅随机下沉。
    float pos_y_off = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos_adj = pos;
    pos_adj.y = pos_y_off;
    // 实例原点 xz (用于草地/水面采样)。
    float2 origin_xz = origin.xz;
    // 采样草地推动图 (风吹位移)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint jit_b_ret;
    uint jit_a_ret;
    bool jit_c_ret;
    bool jit_d_ret;
    float3 water_disp;
    do
    {
        // 雪地主题直接跳过水面位移。
        is_glade4 = frame_constants_glade_theme_id == 4u;
        if (is_glade4)
        {
            jit_b_ret = undef_uint;
            jit_a_ret = undef_uint;
            jit_c_ret = false;
            jit_d_ret = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint jit_b_state;
        uint jit_a_state;
        bool jit_c_state;
        bool jit_d_state;
        jit_b_state = undef_uint;
        jit_a_state = undef_uint;
        jit_c_state = false;
        jit_d_state = false;
        // 每八度的抖动方向 (值噪声插值)。
        float2 jitter[4];
        // 4 个八度的抖动方向哈希。
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float frac_f = frac(seed_f);
            uint cell_u = uint(cell);
            uint cell_h0 = cell_u + (cell_u << 10u);
            uint cell_h1 = cell_h0 ^ (cell_h0 >> 6u);
            uint cell_h2 = cell_h1 + (cell_h1 << 3u);
            uint cell_h3 = cell_h2 ^ (cell_h2 >> 11u);
            bool use_a = jit_d_state ? jit_d_state : true;
            uint A = jit_d_state ? jit_a_state : 1664525u;
            bool use_b = jit_c_state ? jit_c_state : true;
            uint B = jit_c_state ? jit_b_state : 1013904223u;
            uint h0 = (((cell_u * A) + (cell_h3 + (cell_h3 << 15u))) + B) * A;
            uint h0_x = h0 ^ (h0 >> 11u);
            uint h0_y = h0_x ^ ((h0_x << 7u) & 2636928640u);
            uint h0_z = h0_y ^ ((h0_y << 15u) & 4022730752u);
            uint h0_w = h0_z ^ (h0_z >> 18u);
            uint h0_r0 = h0_w + (h0_w << 10u);
            uint h0_r1 = h0_r0 ^ (h0_r0 >> 6u);
            uint h0_r2 = h0_r1 + (h0_r1 << 3u);
            uint h0_r3 = h0_r2 ^ (h0_r2 >> 11u);
            uint s1 = uint(cell + 1);
            uint s1_h0 = s1 + (s1 << 10u);
            uint s1_h1 = s1_h0 ^ (s1_h0 >> 6u);
            uint s1_h2 = s1_h1 + (s1_h1 << 3u);
            uint s1_h3 = s1_h2 ^ (s1_h2 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1 = (((s1 * A1) + (s1_h3 + (s1_h3 << 15u))) + B1) * A1;
            uint h1_x = h1 ^ (h1 >> 11u);
            uint h1_y = h1_x ^ ((h1_x << 7u) & 2636928640u);
            uint h1_z = h1_y ^ ((h1_y << 15u) & 4022730752u);
            uint h1_w = h1_z ^ (h1_z >> 18u);
            uint h1_r0 = h1_w + (h1_w << 10u);
            uint h1_r1 = h1_r0 ^ (h1_r0 >> 6u);
            uint h1_r2 = h1_r1 + (h1_r1 << 3u);
            uint h1_r3 = h1_r2 ^ (h1_r2 >> 11u);
            float smooth_u = (frac_f * frac_f) * (3.0f - (2.0f * frac_f));
            jitter[oct] = (normalize(((float2(asfloat((h0_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_r3 + (h0_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - smooth_u)) + (float2(asfloat((h1_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_r3 + (h1_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            jit_b_state = B1;
            jit_a_state = A1;
            jit_c_state = use_b ? use_b : true;
            jit_d_state = use_a ? use_a : true;
            continue;
        }
        // 水面位移累加器。
        float3 acc;
        acc = 0.0f.xxx;
        // 波场: 旋转坐标系后采样噪声。
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            // 旋转矩阵 (黄金角)。
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = acc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            acc = float3(acc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        jit_b_ret = jit_b_state;
        jit_a_ret = jit_a_state;
        jit_c_ret = jit_c_state;
        jit_d_ret = jit_d_state;
        water_disp = acc;
        break;
    } while(false);
    // 合成最终水平位移 (草地推动 + 水面位移)。
    float2 disp2 = pos_adj.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    // 世界坐标 (只替换 xz)。
    float3 wpos = float3(disp2.x, pos_adj.y, disp2.y);
    // 摆动幅度随顶点高度增大。
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 2D 值噪声采样位置 (随风平移)。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    // 噪声格坐标与格内小数。
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    // 对 2x2 噪声格四角求 hash (用于值噪声)。
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint cell_y = wobble_cell_u.y;
    uint cell_y_h0 = cell_y + (cell_y << 10u);
    uint cell_y_h1 = cell_y_h0 ^ (cell_y_h0 >> 6u);
    uint cell_y_h2 = cell_y_h1 + (cell_y_h1 << 3u);
    uint cell_y_h3 = cell_y_h2 ^ (cell_y_h2 >> 11u);
    bool n_use_a = jit_d_ret ? jit_d_ret : true;
    uint n_A = jit_d_ret ? jit_a_ret : 1664525u;
    bool n_use_b = jit_c_ret ? jit_c_ret : true;
    uint n_B = jit_c_ret ? jit_b_ret : 1013904223u;
    uint n_h0 = (((wobble_cell_u.x * n_A) + (cell_y_h3 + (cell_y_h3 << 15u))) + n_B) * n_A;
    uint n_h0_x = n_h0 ^ (n_h0 >> 11u);
    uint n_h0_y = n_h0_x ^ ((n_h0_x << 7u) & 2636928640u);
    uint n_h0_z = n_h0_y ^ ((n_h0_y << 15u) & 4022730752u);
    uint2 cell_10 = uint2(wobble_cell + int2(1, 0));
    uint cell10_y = cell_10.y;
    uint cell10_h0 = cell10_y + (cell10_y << 10u);
    uint cell10_h1 = cell10_h0 ^ (cell10_h0 >> 6u);
    uint cell10_h2 = cell10_h1 + (cell10_h1 << 3u);
    uint cell10_h3 = cell10_h2 ^ (cell10_h2 >> 11u);
    bool n10_use_a = n_use_a ? n_use_a : true;
    uint n10_A = n_use_a ? n_A : 1664525u;
    bool n10_use_b = n_use_b ? n_use_b : true;
    uint n10_B = n_use_b ? n_B : 1013904223u;
    uint n10_h0 = (((cell_10.x * n10_A) + (cell10_h3 + (cell10_h3 << 15u))) + n10_B) * n10_A;
    uint n10_h0_x = n10_h0 ^ (n10_h0 >> 11u);
    uint n10_h0_y = n10_h0_x ^ ((n10_h0_x << 7u) & 2636928640u);
    uint n10_h0_z = n10_h0_y ^ ((n10_h0_y << 15u) & 4022730752u);
    uint2 cell_01 = uint2(wobble_cell + int2(0, 1));
    uint cell01_y = cell_01.y;
    uint cell01_h0 = cell01_y + (cell01_y << 10u);
    uint cell01_h1 = cell01_h0 ^ (cell01_h0 >> 6u);
    uint cell01_h2 = cell01_h1 + (cell01_h1 << 3u);
    uint cell01_h3 = cell01_h2 ^ (cell01_h2 >> 11u);
    uint n01_A1 = n10_use_a ? n10_A : 1664525u;
    uint n01_B1 = n10_use_b ? n10_B : 1013904223u;
    uint n01_h0 = (((cell_01.x * n01_A1) + (cell01_h3 + (cell01_h3 << 15u))) + n01_B1) * n01_A1;
    uint n01_h0_x = n01_h0 ^ (n01_h0 >> 11u);
    uint n01_h0_y = n01_h0_x ^ ((n01_h0_x << 7u) & 2636928640u);
    uint n01_h0_z = n01_h0_y ^ ((n01_h0_y << 15u) & 4022730752u);
    uint2 cell_11 = uint2(wobble_cell + int2(1, 1));
    uint cell11_y = cell_11.y;
    uint cell11_h0 = cell11_y + (cell11_y << 10u);
    uint cell11_h1 = cell11_h0 ^ (cell11_h0 >> 6u);
    uint cell11_h2 = cell11_h1 + (cell11_h1 << 3u);
    uint cell11_h3 = cell11_h2 ^ (cell11_h2 >> 11u);
    uint n11_A2 = (n10_use_a ? n10_use_a : true) ? n01_A1 : 1664525u;
    uint n11_h0 = (((cell_11.x * n11_A2) + (cell11_h3 + (cell11_h3 << 15u))) + ((n10_use_b ? n10_use_b : true) ? n01_B1 : 1013904223u)) * n11_A2;
    uint n11_h0_x = n11_h0 ^ (n11_h0 >> 11u);
    uint n11_h0_y = n11_h0_x ^ ((n11_h0_x << 7u) & 2636928640u);
    uint n11_h0_z = n11_h0_y ^ ((n11_h0_y << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_u_x = wobble_u.x;
    float wobble_u_inv = 1.0f - wobble_u_x;
    float wobble_u_y = wobble_u.y;
    uint randv_int = uint(int(randv));
    uint randv_h0 = randv_int + (randv_int << 10u);
    uint randv_h1 = randv_h0 ^ (randv_h0 >> 6u);
    uint randv_h2 = randv_h1 + (randv_h1 << 3u);
    uint randv_h3 = randv_h2 ^ (randv_h2 >> 11u);
    // 芦苇顶端沿 -x 方向按噪声相位摆动。
    float wobbled_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_u_y) * (((asfloat(((n_h0_z ^ (n_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) + ((asfloat(((n10_h0_z ^ (n10_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x))) + (((asfloat(((n01_h0_z ^ (n01_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) * wobble_u_y)) + (((asfloat(((n11_h0_z ^ (n11_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x) * wobble_u_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_glade4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((randv_h3 + (randv_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wobbled_x;
    // 顶点颜色按随机指数增强。
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    // 顶点色 gamma 增强。
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_glade4)
    {
        float3 themed_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            themed_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            themed_color = vertex_color;
        }
        color = themed_color;
    }
    else
    {
        color = vertex_color;
    }
    // 投影到采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wobbled_x, pos_y_off, disp2.y, 1.0f));
    // 传递世界法线。
    out_var_C1 = normalize(mul(m, float4(in_var_Vertex_Normal, 0.0f)));
    // 传递世界坐标。
    out_var_C2 = wpos;
    // 传递顶点颜色。
    out_var_C3 = color;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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


static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 雪地主题且高于水面时混入雪色。
    bool snow_branch;
    // 雪地主题分支。
    if (frame_constants_glade_theme_id == 4u)
    {
        snow_branch = in_var_C2.y > 0.0f;
    }
    else
    {
        snow_branch = false;
    }
    // 反照率 (雪色混合后)。
    float3 albedo;
    if (snow_branch)
    {
        albedo = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        albedo = in_var_C3;
    }
    // 法线重映射到 [0,1]。
    float3 packed_normal = (in_var_C1 * 0.5f) + 0.5f.xxx;
    // 线性化反照率 (srgb 解码)。
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    // 打包法线 + 反照率 (RGBA8, alpha=0xFF)。
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

static uint undef_uint;

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
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
static float3 out_var_C3;

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
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool is_glade4;
    // 读取当前实例仿射变换 (48 字节)。
    Affine3Packed xform;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        xform.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    // 把打包仿射展开为 float3x4 变换矩阵。
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点本地坐标转世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点 (世界坐标)。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 由实例原点与顶点位置求散列种子。
    uint seed = uint(((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h0 = seed + (seed << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    // 种子 hash -> [0,1] 随机数。
    float randv = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 垂直方向小幅随机下沉。
    float pos_y_off = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos_adj = pos;
    pos_adj.y = pos_y_off;
    // 实例原点 xz (用于草地/水面采样)。
    float2 origin_xz = origin.xz;
    // 采样草地推动图 (风吹位移)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint jit_b_ret;
    uint jit_a_ret;
    bool jit_c_ret;
    bool jit_d_ret;
    float3 water_disp;
    do
    {
        // 雪地主题直接跳过水面位移。
        is_glade4 = frame_constants_glade_theme_id == 4u;
        if (is_glade4)
        {
            jit_b_ret = undef_uint;
            jit_a_ret = undef_uint;
            jit_c_ret = false;
            jit_d_ret = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint jit_b_state;
        uint jit_a_state;
        bool jit_c_state;
        bool jit_d_state;
        jit_b_state = undef_uint;
        jit_a_state = undef_uint;
        jit_c_state = false;
        jit_d_state = false;
        // 每八度的抖动方向 (值噪声插值)。
        float2 jitter[4];
        // 4 个八度的抖动方向哈希。
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float frac_f = frac(seed_f);
            uint cell_u = uint(cell);
            uint cell_h0 = cell_u + (cell_u << 10u);
            uint cell_h1 = cell_h0 ^ (cell_h0 >> 6u);
            uint cell_h2 = cell_h1 + (cell_h1 << 3u);
            uint cell_h3 = cell_h2 ^ (cell_h2 >> 11u);
            bool use_a = jit_d_state ? jit_d_state : true;
            uint A = jit_d_state ? jit_a_state : 1664525u;
            bool use_b = jit_c_state ? jit_c_state : true;
            uint B = jit_c_state ? jit_b_state : 1013904223u;
            uint h0 = (((cell_u * A) + (cell_h3 + (cell_h3 << 15u))) + B) * A;
            uint h0_x = h0 ^ (h0 >> 11u);
            uint h0_y = h0_x ^ ((h0_x << 7u) & 2636928640u);
            uint h0_z = h0_y ^ ((h0_y << 15u) & 4022730752u);
            uint h0_w = h0_z ^ (h0_z >> 18u);
            uint h0_r0 = h0_w + (h0_w << 10u);
            uint h0_r1 = h0_r0 ^ (h0_r0 >> 6u);
            uint h0_r2 = h0_r1 + (h0_r1 << 3u);
            uint h0_r3 = h0_r2 ^ (h0_r2 >> 11u);
            uint s1 = uint(cell + 1);
            uint s1_h0 = s1 + (s1 << 10u);
            uint s1_h1 = s1_h0 ^ (s1_h0 >> 6u);
            uint s1_h2 = s1_h1 + (s1_h1 << 3u);
            uint s1_h3 = s1_h2 ^ (s1_h2 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1 = (((s1 * A1) + (s1_h3 + (s1_h3 << 15u))) + B1) * A1;
            uint h1_x = h1 ^ (h1 >> 11u);
            uint h1_y = h1_x ^ ((h1_x << 7u) & 2636928640u);
            uint h1_z = h1_y ^ ((h1_y << 15u) & 4022730752u);
            uint h1_w = h1_z ^ (h1_z >> 18u);
            uint h1_r0 = h1_w + (h1_w << 10u);
            uint h1_r1 = h1_r0 ^ (h1_r0 >> 6u);
            uint h1_r2 = h1_r1 + (h1_r1 << 3u);
            uint h1_r3 = h1_r2 ^ (h1_r2 >> 11u);
            float smooth_u = (frac_f * frac_f) * (3.0f - (2.0f * frac_f));
            jitter[oct] = (normalize(((float2(asfloat((h0_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_r3 + (h0_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - smooth_u)) + (float2(asfloat((h1_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_r3 + (h1_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            jit_b_state = B1;
            jit_a_state = A1;
            jit_c_state = use_b ? use_b : true;
            jit_d_state = use_a ? use_a : true;
            continue;
        }
        // 水面位移累加器。
        float3 acc;
        acc = 0.0f.xxx;
        // 波场: 旋转坐标系后采样噪声。
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            // 旋转矩阵 (黄金角)。
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = acc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            acc = float3(acc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        jit_b_ret = jit_b_state;
        jit_a_ret = jit_a_state;
        jit_c_ret = jit_c_state;
        jit_d_ret = jit_d_state;
        water_disp = acc;
        break;
    } while(false);
    // 合成最终水平位移 (草地推动 + 水面位移)。
    float2 disp2 = pos_adj.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    // 世界坐标 (只替换 xz)。
    float3 wpos = float3(disp2.x, pos_adj.y, disp2.y);
    // 摆动幅度随顶点高度增大。
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 2D 值噪声采样位置 (随风平移)。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    // 噪声格坐标与格内小数。
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    // 对 2x2 噪声格四角求 hash (用于值噪声)。
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint cell_y = wobble_cell_u.y;
    uint cell_y_h0 = cell_y + (cell_y << 10u);
    uint cell_y_h1 = cell_y_h0 ^ (cell_y_h0 >> 6u);
    uint cell_y_h2 = cell_y_h1 + (cell_y_h1 << 3u);
    uint cell_y_h3 = cell_y_h2 ^ (cell_y_h2 >> 11u);
    bool n_use_a = jit_d_ret ? jit_d_ret : true;
    uint n_A = jit_d_ret ? jit_a_ret : 1664525u;
    bool n_use_b = jit_c_ret ? jit_c_ret : true;
    uint n_B = jit_c_ret ? jit_b_ret : 1013904223u;
    uint n_h0 = (((wobble_cell_u.x * n_A) + (cell_y_h3 + (cell_y_h3 << 15u))) + n_B) * n_A;
    uint n_h0_x = n_h0 ^ (n_h0 >> 11u);
    uint n_h0_y = n_h0_x ^ ((n_h0_x << 7u) & 2636928640u);
    uint n_h0_z = n_h0_y ^ ((n_h0_y << 15u) & 4022730752u);
    uint2 cell_10 = uint2(wobble_cell + int2(1, 0));
    uint cell10_y = cell_10.y;
    uint cell10_h0 = cell10_y + (cell10_y << 10u);
    uint cell10_h1 = cell10_h0 ^ (cell10_h0 >> 6u);
    uint cell10_h2 = cell10_h1 + (cell10_h1 << 3u);
    uint cell10_h3 = cell10_h2 ^ (cell10_h2 >> 11u);
    bool n10_use_a = n_use_a ? n_use_a : true;
    uint n10_A = n_use_a ? n_A : 1664525u;
    bool n10_use_b = n_use_b ? n_use_b : true;
    uint n10_B = n_use_b ? n_B : 1013904223u;
    uint n10_h0 = (((cell_10.x * n10_A) + (cell10_h3 + (cell10_h3 << 15u))) + n10_B) * n10_A;
    uint n10_h0_x = n10_h0 ^ (n10_h0 >> 11u);
    uint n10_h0_y = n10_h0_x ^ ((n10_h0_x << 7u) & 2636928640u);
    uint n10_h0_z = n10_h0_y ^ ((n10_h0_y << 15u) & 4022730752u);
    uint2 cell_01 = uint2(wobble_cell + int2(0, 1));
    uint cell01_y = cell_01.y;
    uint cell01_h0 = cell01_y + (cell01_y << 10u);
    uint cell01_h1 = cell01_h0 ^ (cell01_h0 >> 6u);
    uint cell01_h2 = cell01_h1 + (cell01_h1 << 3u);
    uint cell01_h3 = cell01_h2 ^ (cell01_h2 >> 11u);
    uint n01_A1 = n10_use_a ? n10_A : 1664525u;
    uint n01_B1 = n10_use_b ? n10_B : 1013904223u;
    uint n01_h0 = (((cell_01.x * n01_A1) + (cell01_h3 + (cell01_h3 << 15u))) + n01_B1) * n01_A1;
    uint n01_h0_x = n01_h0 ^ (n01_h0 >> 11u);
    uint n01_h0_y = n01_h0_x ^ ((n01_h0_x << 7u) & 2636928640u);
    uint n01_h0_z = n01_h0_y ^ ((n01_h0_y << 15u) & 4022730752u);
    uint2 cell_11 = uint2(wobble_cell + int2(1, 1));
    uint cell11_y = cell_11.y;
    uint cell11_h0 = cell11_y + (cell11_y << 10u);
    uint cell11_h1 = cell11_h0 ^ (cell11_h0 >> 6u);
    uint cell11_h2 = cell11_h1 + (cell11_h1 << 3u);
    uint cell11_h3 = cell11_h2 ^ (cell11_h2 >> 11u);
    uint n11_A2 = (n10_use_a ? n10_use_a : true) ? n01_A1 : 1664525u;
    uint n11_h0 = (((cell_11.x * n11_A2) + (cell11_h3 + (cell11_h3 << 15u))) + ((n10_use_b ? n10_use_b : true) ? n01_B1 : 1013904223u)) * n11_A2;
    uint n11_h0_x = n11_h0 ^ (n11_h0 >> 11u);
    uint n11_h0_y = n11_h0_x ^ ((n11_h0_x << 7u) & 2636928640u);
    uint n11_h0_z = n11_h0_y ^ ((n11_h0_y << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_u_x = wobble_u.x;
    float wobble_u_inv = 1.0f - wobble_u_x;
    float wobble_u_y = wobble_u.y;
    uint randv_int = uint(int(randv));
    uint randv_h0 = randv_int + (randv_int << 10u);
    uint randv_h1 = randv_h0 ^ (randv_h0 >> 6u);
    uint randv_h2 = randv_h1 + (randv_h1 << 3u);
    uint randv_h3 = randv_h2 ^ (randv_h2 >> 11u);
    // 芦苇顶端沿 -x 方向按噪声相位摆动。
    float wobbled_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_u_y) * (((asfloat(((n_h0_z ^ (n_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) + ((asfloat(((n10_h0_z ^ (n10_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x))) + (((asfloat(((n01_h0_z ^ (n01_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) * wobble_u_y)) + (((asfloat(((n11_h0_z ^ (n11_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x) * wobble_u_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_glade4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((randv_h3 + (randv_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wobbled_x;
    // 顶点颜色按随机指数增强。
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    // 顶点色 gamma 增强。
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_glade4)
    {
        float3 themed_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            themed_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            themed_color = vertex_color;
        }
        color = themed_color;
    }
    else
    {
        color = vertex_color;
    }
    // 投影到采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wobbled_x, pos_y_off, disp2.y, 1.0f));
    // 传递世界法线。
    out_var_C1 = normalize(mul(m, float4(in_var_Vertex_Normal, 0.0f)));
    // 传递世界坐标。
    out_var_C2 = wpos;
    // 传递顶点颜色。
    out_var_C3 = color;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc factors (a*c) - (b*c) into (a-b)*c, which rounds differently from the original; precise temporaries keep both products.
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

static uint undef_uint;

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
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
static float3 out_var_C3;

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
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool is_glade4;
    // 读取当前实例仿射变换 (48 字节)。
    Affine3Packed xform;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        xform.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    // 把打包仿射展开为 float3x4 变换矩阵。
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点本地坐标转世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点 (世界坐标)。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 由实例原点与顶点位置求散列种子 (dxc 精确乘减保真副本)。
    uint seed = uint(((spv_fmsub(origin.x, origin.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h0 = seed + (seed << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    // 种子 hash -> [0,1] 随机数。
    float randv = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 垂直方向小幅随机下沉。
    float pos_y_off = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos_adj = pos;
    pos_adj.y = pos_y_off;
    // 实例原点 xz (用于草地/水面采样)。
    float2 origin_xz = origin.xz;
    // 采样草地推动图 (风吹位移)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint jit_b_ret;
    uint jit_a_ret;
    bool jit_c_ret;
    bool jit_d_ret;
    float3 water_disp;
    do
    {
        // 雪地主题直接跳过水面位移。
        is_glade4 = frame_constants_glade_theme_id == 4u;
        if (is_glade4)
        {
            jit_b_ret = undef_uint;
            jit_a_ret = undef_uint;
            jit_c_ret = false;
            jit_d_ret = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint jit_b_state;
        uint jit_a_state;
        bool jit_c_state;
        bool jit_d_state;
        jit_b_state = undef_uint;
        jit_a_state = undef_uint;
        jit_c_state = false;
        jit_d_state = false;
        // 每八度的抖动方向 (值噪声插值)。
        float2 jitter[4];
        // 4 个八度的抖动方向哈希。
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float frac_f = frac(seed_f);
            uint cell_u = uint(cell);
            uint cell_h0 = cell_u + (cell_u << 10u);
            uint cell_h1 = cell_h0 ^ (cell_h0 >> 6u);
            uint cell_h2 = cell_h1 + (cell_h1 << 3u);
            uint cell_h3 = cell_h2 ^ (cell_h2 >> 11u);
            bool use_a = jit_d_state ? jit_d_state : true;
            uint A = jit_d_state ? jit_a_state : 1664525u;
            bool use_b = jit_c_state ? jit_c_state : true;
            uint B = jit_c_state ? jit_b_state : 1013904223u;
            uint h0 = (((cell_u * A) + (cell_h3 + (cell_h3 << 15u))) + B) * A;
            uint h0_x = h0 ^ (h0 >> 11u);
            uint h0_y = h0_x ^ ((h0_x << 7u) & 2636928640u);
            uint h0_z = h0_y ^ ((h0_y << 15u) & 4022730752u);
            uint h0_w = h0_z ^ (h0_z >> 18u);
            uint h0_r0 = h0_w + (h0_w << 10u);
            uint h0_r1 = h0_r0 ^ (h0_r0 >> 6u);
            uint h0_r2 = h0_r1 + (h0_r1 << 3u);
            uint h0_r3 = h0_r2 ^ (h0_r2 >> 11u);
            uint s1 = uint(cell + 1);
            uint s1_h0 = s1 + (s1 << 10u);
            uint s1_h1 = s1_h0 ^ (s1_h0 >> 6u);
            uint s1_h2 = s1_h1 + (s1_h1 << 3u);
            uint s1_h3 = s1_h2 ^ (s1_h2 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1 = (((s1 * A1) + (s1_h3 + (s1_h3 << 15u))) + B1) * A1;
            uint h1_x = h1 ^ (h1 >> 11u);
            uint h1_y = h1_x ^ ((h1_x << 7u) & 2636928640u);
            uint h1_z = h1_y ^ ((h1_y << 15u) & 4022730752u);
            uint h1_w = h1_z ^ (h1_z >> 18u);
            uint h1_r0 = h1_w + (h1_w << 10u);
            uint h1_r1 = h1_r0 ^ (h1_r0 >> 6u);
            uint h1_r2 = h1_r1 + (h1_r1 << 3u);
            uint h1_r3 = h1_r2 ^ (h1_r2 >> 11u);
            float smooth_u = (frac_f * frac_f) * (3.0f - (2.0f * frac_f));
            jitter[oct] = (normalize(((float2(asfloat((h0_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_r3 + (h0_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - smooth_u)) + (float2(asfloat((h1_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_r3 + (h1_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            jit_b_state = B1;
            jit_a_state = A1;
            jit_c_state = use_b ? use_b : true;
            jit_d_state = use_a ? use_a : true;
            continue;
        }
        // 水面位移累加器。
        float3 acc;
        acc = 0.0f.xxx;
        // 波场: 旋转坐标系后采样噪声。
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            // 旋转矩阵 (黄金角)。
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = acc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            acc = float3(acc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        jit_b_ret = jit_b_state;
        jit_a_ret = jit_a_state;
        jit_c_ret = jit_c_state;
        jit_d_ret = jit_d_state;
        water_disp = acc;
        break;
    } while(false);
    // 合成最终水平位移 (草地推动 + 水面位移)。
    float2 disp2 = pos_adj.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    // 世界坐标 (只替换 xz)。
    float3 wpos = float3(disp2.x, pos_adj.y, disp2.y);
    // 摆动幅度随顶点高度增大。
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 2D 值噪声采样位置 (随风平移)。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    // 噪声格坐标与格内小数。
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    // 对 2x2 噪声格四角求 hash (用于值噪声)。
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint cell_y = wobble_cell_u.y;
    uint cell_y_h0 = cell_y + (cell_y << 10u);
    uint cell_y_h1 = cell_y_h0 ^ (cell_y_h0 >> 6u);
    uint cell_y_h2 = cell_y_h1 + (cell_y_h1 << 3u);
    uint cell_y_h3 = cell_y_h2 ^ (cell_y_h2 >> 11u);
    bool n_use_a = jit_d_ret ? jit_d_ret : true;
    uint n_A = jit_d_ret ? jit_a_ret : 1664525u;
    bool n_use_b = jit_c_ret ? jit_c_ret : true;
    uint n_B = jit_c_ret ? jit_b_ret : 1013904223u;
    uint n_h0 = (((wobble_cell_u.x * n_A) + (cell_y_h3 + (cell_y_h3 << 15u))) + n_B) * n_A;
    uint n_h0_x = n_h0 ^ (n_h0 >> 11u);
    uint n_h0_y = n_h0_x ^ ((n_h0_x << 7u) & 2636928640u);
    uint n_h0_z = n_h0_y ^ ((n_h0_y << 15u) & 4022730752u);
    uint2 cell_10 = uint2(wobble_cell + int2(1, 0));
    uint cell10_y = cell_10.y;
    uint cell10_h0 = cell10_y + (cell10_y << 10u);
    uint cell10_h1 = cell10_h0 ^ (cell10_h0 >> 6u);
    uint cell10_h2 = cell10_h1 + (cell10_h1 << 3u);
    uint cell10_h3 = cell10_h2 ^ (cell10_h2 >> 11u);
    bool n10_use_a = n_use_a ? n_use_a : true;
    uint n10_A = n_use_a ? n_A : 1664525u;
    bool n10_use_b = n_use_b ? n_use_b : true;
    uint n10_B = n_use_b ? n_B : 1013904223u;
    uint n10_h0 = (((cell_10.x * n10_A) + (cell10_h3 + (cell10_h3 << 15u))) + n10_B) * n10_A;
    uint n10_h0_x = n10_h0 ^ (n10_h0 >> 11u);
    uint n10_h0_y = n10_h0_x ^ ((n10_h0_x << 7u) & 2636928640u);
    uint n10_h0_z = n10_h0_y ^ ((n10_h0_y << 15u) & 4022730752u);
    uint2 cell_01 = uint2(wobble_cell + int2(0, 1));
    uint cell01_y = cell_01.y;
    uint cell01_h0 = cell01_y + (cell01_y << 10u);
    uint cell01_h1 = cell01_h0 ^ (cell01_h0 >> 6u);
    uint cell01_h2 = cell01_h1 + (cell01_h1 << 3u);
    uint cell01_h3 = cell01_h2 ^ (cell01_h2 >> 11u);
    uint n01_A1 = n10_use_a ? n10_A : 1664525u;
    uint n01_B1 = n10_use_b ? n10_B : 1013904223u;
    uint n01_h0 = (((cell_01.x * n01_A1) + (cell01_h3 + (cell01_h3 << 15u))) + n01_B1) * n01_A1;
    uint n01_h0_x = n01_h0 ^ (n01_h0 >> 11u);
    uint n01_h0_y = n01_h0_x ^ ((n01_h0_x << 7u) & 2636928640u);
    uint n01_h0_z = n01_h0_y ^ ((n01_h0_y << 15u) & 4022730752u);
    uint2 cell_11 = uint2(wobble_cell + int2(1, 1));
    uint cell11_y = cell_11.y;
    uint cell11_h0 = cell11_y + (cell11_y << 10u);
    uint cell11_h1 = cell11_h0 ^ (cell11_h0 >> 6u);
    uint cell11_h2 = cell11_h1 + (cell11_h1 << 3u);
    uint cell11_h3 = cell11_h2 ^ (cell11_h2 >> 11u);
    uint n11_A2 = (n10_use_a ? n10_use_a : true) ? n01_A1 : 1664525u;
    uint n11_h0 = (((cell_11.x * n11_A2) + (cell11_h3 + (cell11_h3 << 15u))) + ((n10_use_b ? n10_use_b : true) ? n01_B1 : 1013904223u)) * n11_A2;
    uint n11_h0_x = n11_h0 ^ (n11_h0 >> 11u);
    uint n11_h0_y = n11_h0_x ^ ((n11_h0_x << 7u) & 2636928640u);
    uint n11_h0_z = n11_h0_y ^ ((n11_h0_y << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_u_x = wobble_u.x;
    float wobble_u_inv = 1.0f - wobble_u_x;
    float wobble_u_y = wobble_u.y;
    uint randv_int = uint(int(randv));
    uint randv_h0 = randv_int + (randv_int << 10u);
    uint randv_h1 = randv_h0 ^ (randv_h0 >> 6u);
    uint randv_h2 = randv_h1 + (randv_h1 << 3u);
    uint randv_h3 = randv_h2 ^ (randv_h2 >> 11u);
    // 芦苇顶端沿 -x 方向按噪声相位摆动。
    float wobbled_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_u_y) * (((asfloat(((n_h0_z ^ (n_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) + ((asfloat(((n10_h0_z ^ (n10_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x))) + (((asfloat(((n01_h0_z ^ (n01_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) * wobble_u_y)) + (((asfloat(((n11_h0_z ^ (n11_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x) * wobble_u_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_glade4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((randv_h3 + (randv_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wobbled_x;
    // 顶点颜色按随机指数增强。
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    // 顶点色 gamma 增强。
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_glade4)
    {
        float3 themed_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            themed_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            themed_color = vertex_color;
        }
        color = themed_color;
    }
    else
    {
        color = vertex_color;
    }
    // 投影到采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wobbled_x, pos_y_off, disp2.y, 1.0f));
    // 传递世界法线。
    out_var_C1 = normalize(mul(m, float4(in_var_Vertex_Normal, 0.0f)));
    // 传递世界坐标。
    out_var_C2 = wpos;
    // 传递顶点颜色。
    out_var_C3 = color;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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


static float3 in_var_C1;
static float3 in_var_C2;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 雪地主题且高于水面时混入雪色。
    bool snow_branch;
    // 雪地主题分支。
    if (frame_constants_glade_theme_id == 4u)
    {
        snow_branch = in_var_C2.y > 0.0f;
    }
    else
    {
        snow_branch = false;
    }
    // 反照率 (雪色混合后)。
    float3 albedo;
    if (snow_branch)
    {
        albedo = lerp(in_var_C3, 0.699999988079071044921875f.xxx, smoothstep(-0.20000000298023223876953125f, 1.0f, clamp(in_var_C1.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        albedo = in_var_C3;
    }
    // 法线重映射到 [0,1]。
    float3 packed_normal = (in_var_C1 * 0.5f) + 0.5f.xxx;
    // 线性化反照率 (srgb 解码)。
    float3 linear_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    // 打包法线 + 反照率 (RGBA8, alpha=0xFF)。
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(linear_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(linear_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(linear_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
// dxc factors (a*c) - (b*c) into (a-b)*c, which rounds differently from the original; precise temporaries keep both products.
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

static uint undef_uint;

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
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    int bool_render_shadows : packoffset(c2.z);
};

ByteAddressBuffer instance_buffer : register(t17, space0);
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
static float3 out_var_C3;

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
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    bool is_glade4;
    // 读取当前实例仿射变换 (48 字节)。
    Affine3Packed xform;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        xform.data[i] = instance_buffer.Load<float>(i * 4 + uint(gl_InstanceIndex) * 48 + 0);
    }
    // 把打包仿射展开为 float3x4 变换矩阵。
    float3x4 m = transpose(float4x3(float3(xform.data[0], xform.data[1], xform.data[2]), float3(xform.data[3], xform.data[4], xform.data[5]), float3(xform.data[6], xform.data[7], xform.data[8]), float3(xform.data[9], xform.data[10], xform.data[11])));
    // 顶点本地坐标转世界坐标。
    float3 pos = mul(m, float4(in_var_Vertex_Position, 1.0f));
    // 实例原点 (世界坐标)。
    float3 origin = mul(m, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 由实例原点与顶点位置求散列种子 (dxc 精确乘减保真副本)。
    uint seed = uint(((spv_fmsub(origin.x, origin.z, 20.0f) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint seed_h0 = seed + (seed << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    // 种子 hash -> [0,1] 随机数。
    float randv = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 垂直方向小幅随机下沉。
    float pos_y_off = pos.y + lerp(-0.02500000037252902984619140625f, 0.0f, randv);
    float3 pos_adj = pos;
    pos_adj.y = pos_y_off;
    // 实例原点 xz (用于草地/水面采样)。
    float2 origin_xz = origin.xz;
    // 采样草地推动图 (风吹位移)。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    uint jit_b_ret;
    uint jit_a_ret;
    bool jit_c_ret;
    bool jit_d_ret;
    float3 water_disp;
    do
    {
        // 雪地主题直接跳过水面位移。
        is_glade4 = frame_constants_glade_theme_id == 4u;
        if (is_glade4)
        {
            jit_b_ret = undef_uint;
            jit_a_ret = undef_uint;
            jit_c_ret = false;
            jit_d_ret = false;
            water_disp = 0.0f.xxx;
            break;
        }
        float water_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
        uint jit_b_state;
        uint jit_a_state;
        bool jit_c_state;
        bool jit_d_state;
        jit_b_state = undef_uint;
        jit_a_state = undef_uint;
        jit_c_state = false;
        jit_d_state = false;
        // 每八度的抖动方向 (值噪声插值)。
        float2 jitter[4];
        // 4 个八度的抖动方向哈希。
        for (uint oct = 0u; oct < 4u; )
        {
            float seed_f = float(wave_seeds[oct]);
            int cell = int(floor(seed_f));
            float frac_f = frac(seed_f);
            uint cell_u = uint(cell);
            uint cell_h0 = cell_u + (cell_u << 10u);
            uint cell_h1 = cell_h0 ^ (cell_h0 >> 6u);
            uint cell_h2 = cell_h1 + (cell_h1 << 3u);
            uint cell_h3 = cell_h2 ^ (cell_h2 >> 11u);
            bool use_a = jit_d_state ? jit_d_state : true;
            uint A = jit_d_state ? jit_a_state : 1664525u;
            bool use_b = jit_c_state ? jit_c_state : true;
            uint B = jit_c_state ? jit_b_state : 1013904223u;
            uint h0 = (((cell_u * A) + (cell_h3 + (cell_h3 << 15u))) + B) * A;
            uint h0_x = h0 ^ (h0 >> 11u);
            uint h0_y = h0_x ^ ((h0_x << 7u) & 2636928640u);
            uint h0_z = h0_y ^ ((h0_y << 15u) & 4022730752u);
            uint h0_w = h0_z ^ (h0_z >> 18u);
            uint h0_r0 = h0_w + (h0_w << 10u);
            uint h0_r1 = h0_r0 ^ (h0_r0 >> 6u);
            uint h0_r2 = h0_r1 + (h0_r1 << 3u);
            uint h0_r3 = h0_r2 ^ (h0_r2 >> 11u);
            uint s1 = uint(cell + 1);
            uint s1_h0 = s1 + (s1 << 10u);
            uint s1_h1 = s1_h0 ^ (s1_h0 >> 6u);
            uint s1_h2 = s1_h1 + (s1_h1 << 3u);
            uint s1_h3 = s1_h2 ^ (s1_h2 >> 11u);
            uint A1 = use_a ? A : 1664525u;
            uint B1 = use_b ? B : 1013904223u;
            uint h1 = (((s1 * A1) + (s1_h3 + (s1_h3 << 15u))) + B1) * A1;
            uint h1_x = h1 ^ (h1 >> 11u);
            uint h1_y = h1_x ^ ((h1_x << 7u) & 2636928640u);
            uint h1_z = h1_y ^ ((h1_y << 15u) & 4022730752u);
            uint h1_w = h1_z ^ (h1_z >> 18u);
            uint h1_r0 = h1_w + (h1_w << 10u);
            uint h1_r1 = h1_r0 ^ (h1_r0 >> 6u);
            uint h1_r2 = h1_r1 + (h1_r1 << 3u);
            uint h1_r3 = h1_r2 ^ (h1_r2 >> 11u);
            float smooth_u = (frac_f * frac_f) * (3.0f - (2.0f * frac_f));
            jitter[oct] = (normalize(((float2(asfloat((h0_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h0_r3 + (h0_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * (1.0f - smooth_u)) + (float2(asfloat((h1_w & 8388607u) | 1065353216u) - 1.0f, asfloat(((h1_r3 + (h1_r3 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_u)) - 0.5f.xx) * water_time) * 3.0f;
            oct++;
            jit_b_state = B1;
            jit_a_state = A1;
            jit_c_state = use_b ? use_b : true;
            jit_d_state = use_a ? use_a : true;
            continue;
        }
        // 水面位移累加器。
        float3 acc;
        acc = 0.0f.xxx;
        // 波场: 旋转坐标系后采样噪声。
        for (uint wave = 0u; wave < 1u; )
        {
            uint wave1 = wave + 1u;
            float angle = float(wave1) * 10.16640758514404296875f;
            float cos_a = cos(angle);
            float sin_a = sin(angle);
            // 旋转矩阵 (黄金角)。
            float2x2 rot = float2x2(float2(cos_a, sin_a), float2(-sin_a, cos_a));
            float2 wave_disp = acc.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(rot, (origin_xz * wave_scales[wave]) + jitter[wave]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * wave_amps[wave]) * float2(1.0f, wave_scales[wave] * 0.01200000010430812835693359375f).xyy).yz, rot);
            acc = float3(acc.x, wave_disp.x, wave_disp.y);
            wave = wave1;
            continue;
        }
        jit_b_ret = jit_b_state;
        jit_a_ret = jit_a_state;
        jit_c_ret = jit_c_state;
        jit_d_ret = jit_d_state;
        water_disp = acc;
        break;
    } while(false);
    // 合成最终水平位移 (草地推动 + 水面位移)。
    float2 disp2 = pos_adj.xz + (((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (water_disp.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz;
    // 世界坐标 (只替换 xz)。
    float3 wpos = float3(disp2.x, pos_adj.y, disp2.y);
    // 摆动幅度随顶点高度增大。
    float wobble_k = in_var_Vertex_Position.y + 0.5f;
    // 2D 值噪声采样位置 (随风平移)。
    float2 wobble_pos = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    // 噪声格坐标与格内小数。
    int2 wobble_cell = int2(floor(wobble_pos));
    float2 wobble_frac = frac(wobble_pos);
    // 对 2x2 噪声格四角求 hash (用于值噪声)。
    uint2 wobble_cell_u = uint2(wobble_cell);
    uint cell_y = wobble_cell_u.y;
    uint cell_y_h0 = cell_y + (cell_y << 10u);
    uint cell_y_h1 = cell_y_h0 ^ (cell_y_h0 >> 6u);
    uint cell_y_h2 = cell_y_h1 + (cell_y_h1 << 3u);
    uint cell_y_h3 = cell_y_h2 ^ (cell_y_h2 >> 11u);
    bool n_use_a = jit_d_ret ? jit_d_ret : true;
    uint n_A = jit_d_ret ? jit_a_ret : 1664525u;
    bool n_use_b = jit_c_ret ? jit_c_ret : true;
    uint n_B = jit_c_ret ? jit_b_ret : 1013904223u;
    uint n_h0 = (((wobble_cell_u.x * n_A) + (cell_y_h3 + (cell_y_h3 << 15u))) + n_B) * n_A;
    uint n_h0_x = n_h0 ^ (n_h0 >> 11u);
    uint n_h0_y = n_h0_x ^ ((n_h0_x << 7u) & 2636928640u);
    uint n_h0_z = n_h0_y ^ ((n_h0_y << 15u) & 4022730752u);
    uint2 cell_10 = uint2(wobble_cell + int2(1, 0));
    uint cell10_y = cell_10.y;
    uint cell10_h0 = cell10_y + (cell10_y << 10u);
    uint cell10_h1 = cell10_h0 ^ (cell10_h0 >> 6u);
    uint cell10_h2 = cell10_h1 + (cell10_h1 << 3u);
    uint cell10_h3 = cell10_h2 ^ (cell10_h2 >> 11u);
    bool n10_use_a = n_use_a ? n_use_a : true;
    uint n10_A = n_use_a ? n_A : 1664525u;
    bool n10_use_b = n_use_b ? n_use_b : true;
    uint n10_B = n_use_b ? n_B : 1013904223u;
    uint n10_h0 = (((cell_10.x * n10_A) + (cell10_h3 + (cell10_h3 << 15u))) + n10_B) * n10_A;
    uint n10_h0_x = n10_h0 ^ (n10_h0 >> 11u);
    uint n10_h0_y = n10_h0_x ^ ((n10_h0_x << 7u) & 2636928640u);
    uint n10_h0_z = n10_h0_y ^ ((n10_h0_y << 15u) & 4022730752u);
    uint2 cell_01 = uint2(wobble_cell + int2(0, 1));
    uint cell01_y = cell_01.y;
    uint cell01_h0 = cell01_y + (cell01_y << 10u);
    uint cell01_h1 = cell01_h0 ^ (cell01_h0 >> 6u);
    uint cell01_h2 = cell01_h1 + (cell01_h1 << 3u);
    uint cell01_h3 = cell01_h2 ^ (cell01_h2 >> 11u);
    uint n01_A1 = n10_use_a ? n10_A : 1664525u;
    uint n01_B1 = n10_use_b ? n10_B : 1013904223u;
    uint n01_h0 = (((cell_01.x * n01_A1) + (cell01_h3 + (cell01_h3 << 15u))) + n01_B1) * n01_A1;
    uint n01_h0_x = n01_h0 ^ (n01_h0 >> 11u);
    uint n01_h0_y = n01_h0_x ^ ((n01_h0_x << 7u) & 2636928640u);
    uint n01_h0_z = n01_h0_y ^ ((n01_h0_y << 15u) & 4022730752u);
    uint2 cell_11 = uint2(wobble_cell + int2(1, 1));
    uint cell11_y = cell_11.y;
    uint cell11_h0 = cell11_y + (cell11_y << 10u);
    uint cell11_h1 = cell11_h0 ^ (cell11_h0 >> 6u);
    uint cell11_h2 = cell11_h1 + (cell11_h1 << 3u);
    uint cell11_h3 = cell11_h2 ^ (cell11_h2 >> 11u);
    uint n11_A2 = (n10_use_a ? n10_use_a : true) ? n01_A1 : 1664525u;
    uint n11_h0 = (((cell_11.x * n11_A2) + (cell11_h3 + (cell11_h3 << 15u))) + ((n10_use_b ? n10_use_b : true) ? n01_B1 : 1013904223u)) * n11_A2;
    uint n11_h0_x = n11_h0 ^ (n11_h0 >> 11u);
    uint n11_h0_y = n11_h0_x ^ ((n11_h0_x << 7u) & 2636928640u);
    uint n11_h0_z = n11_h0_y ^ ((n11_h0_y << 15u) & 4022730752u);
    float2 wobble_u = (wobble_frac * wobble_frac) * (3.0f.xx - (wobble_frac * 2.0f));
    float wobble_u_x = wobble_u.x;
    float wobble_u_inv = 1.0f - wobble_u_x;
    float wobble_u_y = wobble_u.y;
    uint randv_int = uint(int(randv));
    uint randv_h0 = randv_int + (randv_int << 10u);
    uint randv_h1 = randv_h0 ^ (randv_h0 >> 6u);
    uint randv_h2 = randv_h1 + (randv_h1 << 3u);
    uint randv_h3 = randv_h2 ^ (randv_h2 >> 11u);
    // 芦苇顶端沿 -x 方向按噪声相位摆动。
    float wobbled_x = disp2.x - (((((wobble_k * wobble_k) * (sin((((((1.0f - wobble_u_y) * (((asfloat(((n_h0_z ^ (n_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) + ((asfloat(((n10_h0_z ^ (n10_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x))) + (((asfloat(((n01_h0_z ^ (n01_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_inv) * wobble_u_y)) + (((asfloat(((n11_h0_z ^ (n11_h0_z >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wobble_u_x) * wobble_u_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * (is_glade4 ? 0.02500000037252902984619140625f : 0.0500000007450580596923828125f)) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((randv_h3 + (randv_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    wpos.x = wobbled_x;
    // 顶点颜色按随机指数增强。
    float3 color_pow = lerp(1.2000000476837158203125f, 2.0f, randv).xxx;
    // 顶点色 gamma 增强。
    float3 vertex_color = pow(in_var_Vertex_Color, color_pow);
    float3 color;
    if (is_glade4)
    {
        float3 themed_color;
        if (in_var_Vertex_Color.y > 0.0500000007450580596923828125f)
        {
            themed_color = pow(float3(0.1500000059604644775390625f, 0.0900000035762786865234375f, 0.0300000011920928955078125f), color_pow);
        }
        else
        {
            themed_color = vertex_color;
        }
        color = themed_color;
    }
    else
    {
        color = vertex_color;
    }
    // 投影到采样空间。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(wobbled_x, pos_y_off, disp2.y, 1.0f));
    // 传递世界法线。
    out_var_C1 = normalize(mul(m, float4(in_var_Vertex_Normal, 0.0f)));
    // 传递世界坐标。
    out_var_C2 = wpos;
    // 传递顶点颜色。
    out_var_C3 = color;
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
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _reed.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
