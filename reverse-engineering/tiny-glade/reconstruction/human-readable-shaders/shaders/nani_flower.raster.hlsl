// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_flower.raster.hlsl | B nani raster | bins: _nani_flower.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// 花(nani flower)着色器。
//   顶点阶段 vs_main:
//     - 用原点坐标哈希选花色:glade 主题 4(秋)用 autumn_color 调淡,
//       主题 1(花)用 flowery_color,主题 3 的 flower_type 0 用特殊紫蓝色,
//       否则用顶点色;按主题选明暗色;
//     - 2D 风噪声晶格(LCG 哈希 + 双线性插值)按 wind_bend 弯曲花茎;
//     - 采样 grass_object_push 推开被草地物体挤压的部分;
//     - 明暗色按 color_var 指数加深(pow)后输出。
//   像素阶段 ps_main:
//     - 法线归一化;glade 主题 4 时朝上表面混入浅色;
//     - 打包 G-Buffer(法线 RGB8 + 0x66000000,反照率 RGB8 | 0x33000000)。
//   绑定: instance_indirection_buffer(t14)、nani_instance_data(t15)、
//         grass_object_push(t16)、$Globals(b13)。
//   变体: TGV_b903e43ffb3da915(顶点,像素用手写段)、
//         TGV_f0adff76827e73ff(顶点,像素为空占位)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "garden/defs.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

uint2 grass_object_push_dims;
int bool_render_shadows;

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<FlowerXform> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t16);

uint hash2(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    p.x = (p.x * 1664525u) + (y + (y << 15u)) + 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    return x;
}

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float wind_bend: wind_bend;
    [[vk::location(4)]] float3 autumn_color: autumn_color;
    [[vk::location(5)]] float3 flowery_color: flowery_color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 经实例间接缓冲取实例数据,组装仿射变换。
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].transform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));

    // 用原点坐标(xz 合成)派生颜色随机数。
    uint origin_hash = uint((origin.x * 20.0f) - (origin.z * 20.0f) + 2000.0f);
    origin_hash = hash(origin_hash);
    float color_rand = hash2_rand(origin_hash);

    // 主题分支:4=秋(autumn)、1=花(flowery)、3 且 flower_type 0 用紫蓝,否则顶点色。
    bool theme_a = glade_theme_id == 4u;
    bool theme_b = glade_theme_id == 1u;
    float3 base_color;
    float color_var;
    if (theme_a)
    {
        base_color = lerp(vin.autumn_color, 0.7f.xxx, 0.5f.xxx);
        color_var = color_rand * 0.5f;
    }
    else if (theme_b)
    {
        base_color = vin.flowery_color;
        color_var = color_rand * 0.5f;
    }
    else if (glade_theme_id == 3u && nani_instance_data[idx].flower_type == 0u)
    {
        base_color = vin.Vertex_Color.z > 0.2f
            ? float3(0.21f, 0.21f, 0.35f)
            : vin.Vertex_Color;
        color_var = color_rand;
    }
    else
    {
        base_color = vin.Vertex_Color;
        color_var = color_rand;
    }

    // 明暗色:实例有自定义 color(x>-1)时用主题对应的暗色调,否则用 base_color。
    float3 shade_color;
    if (nani_instance_data[idx].color.x > -1.0f)
    {
        shade_color = theme_a
            ? float3(0.207063f, 0.164317f, 0.291863f)
            : (theme_b
                ? float3(0.310594f, 0.246475f, 0.437794f)
                : (glade_theme_id == 3u
                    ? float3(0.310594f, 0.246475f, 0.437794f)
                    : float3(0.126791f, 0.305342f, 0.495884f)));
    }
    else
    {
        shade_color = base_color;
    }

    // 2D 风噪声晶格:原点 xz 平移 0.1 倍 + 时间平移,按 wind_bend 弯曲花茎。
    float2 wind_uv = (origin.xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    world.x -= (vin.wind_bend * vin.wind_bend) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.0735f);

    // 草地物体挤压:采样 push 贴图,按 wind_bend 平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * vin.wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = pow(shade_color, lerp(1.0f, 2.5f, color_var).xxx);
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 法线归一化;主题 4 时朝上表面混入浅色。
    float3 normal = normalize(pin.C1);
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(pin.C0, 0.7f.xxx, smoothstep(0.5f, 1.0f, clamp(normal.y, 0.0f, 1.0f) * 0.8f));
    }
    else
    {
        albedo = pin.C0;
    }
    // 打包 G-Buffer:法线压到 [0,1],反照率 sqrt 到 sRGB 后 8bit 量化,带材质标志。
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 855638016u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 1711276032u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_flower.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint2 grass_object_push_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    [[vk::location(4)]] float3 in_var_autumn_color: autumn_color;
    [[vk::location(5)]] float3 in_var_flowery_color: flowery_color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 经实例间接缓冲读出 12 个 float 的 packed 仿射变换(每实例 64 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    // 原点坐标合成颜色种子并做 LCG 哈希 → [0,1) 颜色随机数。
    uint origin_hash_seed = uint(((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f);
    uint oh1 = origin_hash_seed + (origin_hash_seed << 10u);
    uint oh2 = oh1 ^ (oh1 >> 6u);
    uint oh3 = oh2 + (oh2 << 3u);
    uint oh4 = oh3 ^ (oh3 >> 11u);
    float color_rand = asfloat(((oh4 + (oh4 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 主题判定:非主题 2,且 (主题 2 或 主题 4) 走"秋色"分支。
    bool not_theme2 = !(frame_constants_glade_theme_id == 2u);
    bool is_theme2_or_4;
    if (not_theme2)
    {
        is_theme2_or_4 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        is_theme2_or_4 = true;
    }
    // 花色与颜色变化量。
    float3 base_color;
    float color_var;
    if (is_theme2_or_4)
    {
        // 主题 4:秋色混入 0.7;主题 2:直接用秋色。
        float3 autumn_base;
        if (frame_constants_glade_theme_id == 4u)
        {
            autumn_base = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            autumn_base = in_var_autumn_color;
        }
        base_color = autumn_base;
        color_var = color_rand * 0.5f;
    }
    else
    {
        float3 flowery_or_vertex_color;
        float color_var_alt;
        if (frame_constants_glade_theme_id == 1u)
        {
            flowery_or_vertex_color = in_var_flowery_color;
            color_var_alt = color_rand * 0.5f;
        }
        else
        {
            // 主题 3 且 flower_type==0:高蓝通道顶点色用紫蓝,否则顶点色。
            bool is_theme3_flowertype0;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                is_theme3_flowertype0 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                is_theme3_flowertype0 = false;
            }
            float3 theme3_color;
            if (is_theme3_flowertype0)
            {
                bool3 is_color_z_high = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                theme3_color = float3(is_color_z_high.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, is_color_z_high.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, is_color_z_high.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                theme3_color = in_var_Vertex_Color;
            }
            flowery_or_vertex_color = theme3_color;
            color_var_alt = color_rand;
        }
        base_color = flowery_or_vertex_color;
        color_var = color_var_alt;
    }
    // 明暗色:实例有自定义 color 时按主题选固定暗色调,否则用 base_color。
    float3 shade_color;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool is_theme2_or_4_shade;
        if (not_theme2)
        {
            is_theme2_or_4_shade = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            is_theme2_or_4_shade = true;
        }
        float3 theme_a_shade;
        if (is_theme2_or_4_shade)
        {
            theme_a_shade = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 theme_b_or_3_shade;
            if (frame_constants_glade_theme_id == 1u)
            {
                theme_b_or_3_shade = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 is_theme3 = (frame_constants_glade_theme_id == 3u).xxx;
                theme_b_or_3_shade = float3(is_theme3.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, is_theme3.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, is_theme3.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            theme_a_shade = theme_b_or_3_shade;
        }
        shade_color = theme_a_shade;
    }
    else
    {
        shade_color = base_color;
    }
    // 风噪声晶格:原点 xz 平移 0.1 倍 + 时间平移。
    float2 origin_xz = origin.xz;
    float2 wind_uv = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    // 角点 (0,0) 的 LCG 哈希。
    uint2 wind_cell00_u = uint2(wind_cell);
    uint cell00_y = wind_cell00_u.y;
    uint h00_a = cell00_y + (cell00_y << 10u);
    uint h00_b = h00_a ^ (h00_a >> 6u);
    uint h00_c = h00_b + (h00_b << 3u);
    uint h00_d = h00_c ^ (h00_c >> 11u);
    uint h00_e = (((wind_cell00_u.x * 1664525u) + (h00_d + (h00_d << 15u))) + 1013904223u) * 1664525u;
    uint h00_f = h00_e ^ (h00_e >> 11u);
    uint h00_g = h00_f ^ ((h00_f << 7u) & 2636928640u);
    uint h00 = h00_g ^ ((h00_g << 15u) & 4022730752u);
    // 角点 (1,0) 的 LCG 哈希。
    uint2 wind_cell10_u = uint2(wind_cell + int2(1, 0));
    uint cell10_y = wind_cell10_u.y;
    uint h10_a = cell10_y + (cell10_y << 10u);
    uint h10_b = h10_a ^ (h10_a >> 6u);
    uint h10_c = h10_b + (h10_b << 3u);
    uint h10_d = h10_c ^ (h10_c >> 11u);
    uint h10_e = (((wind_cell10_u.x * 1664525u) + (h10_d + (h10_d << 15u))) + 1013904223u) * 1664525u;
    uint h10_f = h10_e ^ (h10_e >> 11u);
    uint h10_g = h10_f ^ ((h10_f << 7u) & 2636928640u);
    uint h10 = h10_g ^ ((h10_g << 15u) & 4022730752u);
    // 角点 (0,1) 的 LCG 哈希。
    uint2 wind_cell01_u = uint2(wind_cell + int2(0, 1));
    uint cell01_y = wind_cell01_u.y;
    uint h01_a = cell01_y + (cell01_y << 10u);
    uint h01_b = h01_a ^ (h01_a >> 6u);
    uint h01_c = h01_b + (h01_b << 3u);
    uint h01_d = h01_c ^ (h01_c >> 11u);
    uint h01_e = (((wind_cell01_u.x * 1664525u) + (h01_d + (h01_d << 15u))) + 1013904223u) * 1664525u;
    uint h01_f = h01_e ^ (h01_e >> 11u);
    uint h01_g = h01_f ^ ((h01_f << 7u) & 2636928640u);
    uint h01 = h01_g ^ ((h01_g << 15u) & 4022730752u);
    // 角点 (1,1) 的 LCG 哈希。
    uint2 wind_cell11_u = uint2(wind_cell + int2(1, 1));
    uint cell11_y = wind_cell11_u.y;
    uint h11_a = cell11_y + (cell11_y << 10u);
    uint h11_b = h11_a ^ (h11_a >> 6u);
    uint h11_c = h11_b + (h11_b << 3u);
    uint h11_d = h11_c ^ (h11_c >> 11u);
    uint h11_e = (((wind_cell11_u.x * 1664525u) + (h11_d + (h11_d << 15u))) + 1013904223u) * 1664525u;
    uint h11_f = h11_e ^ (h11_e >> 11u);
    uint h11_g = h11_f ^ ((h11_f << 7u) & 2636928640u);
    uint h11 = h11_g ^ ((h11_g << 15u) & 4022730752u);
    // 双线性插值平滑因子与四角噪声 → 花茎弯曲正弦偏移。
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float smooth_x = wind_smooth.x;
    float smooth_x_inv = 1.0f - smooth_x;
    float smooth_y = wind_smooth.y;
    world.x = world.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - smooth_y) * (((asfloat(((h00 ^ (h00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) + ((asfloat(((h10 ^ (h10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x))) + (((asfloat(((h01 ^ (h01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) * smooth_y)) + (((asfloat(((h11 ^ (h11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    // 草地物体挤压:采样 push 贴图,按 wind_bend 平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * in_var_wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }
    else
    {
        displaced = world;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = pow(shade_color, lerp(1.0f, 2.5f, color_var).xxx);
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FlowerXform
{
    Affine3Packed transform;
    float3 color;
    uint flower_type;
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
    uint2 grass_object_push_dims : packoffset(c0.y);
    int bool_render_shadows : packoffset(c0.w);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float in_var_wind_bend;
static float3 in_var_autumn_color;
static float3 in_var_flowery_color;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float in_var_wind_bend: wind_bend;
    [[vk::location(4)]] float3 in_var_autumn_color: autumn_color;
    [[vk::location(5)]] float3 in_var_flowery_color: flowery_color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 经实例间接缓冲读出 12 个 float 的 packed 仿射变换(每实例 64 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    }
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    // 原点坐标合成颜色种子并做 LCG 哈希 → [0,1) 颜色随机数。
    uint origin_hash_seed = uint(((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f);
    uint oh1 = origin_hash_seed + (origin_hash_seed << 10u);
    uint oh2 = oh1 ^ (oh1 >> 6u);
    uint oh3 = oh2 + (oh2 << 3u);
    uint oh4 = oh3 ^ (oh3 >> 11u);
    float color_rand = asfloat(((oh4 + (oh4 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    // 主题判定:非主题 2,且 (主题 2 或 主题 4) 走"秋色"分支。
    bool not_theme2 = !(frame_constants_glade_theme_id == 2u);
    bool is_theme2_or_4;
    if (not_theme2)
    {
        is_theme2_or_4 = frame_constants_glade_theme_id == 4u;
    }
    else
    {
        is_theme2_or_4 = true;
    }
    // 花色与颜色变化量。
    float3 base_color;
    float color_var;
    if (is_theme2_or_4)
    {
        // 主题 4:秋色混入 0.7;主题 2:直接用秋色。
        float3 autumn_base;
        if (frame_constants_glade_theme_id == 4u)
        {
            autumn_base = lerp(in_var_autumn_color, 0.699999988079071044921875f.xxx, 0.5f.xxx);
        }
        else
        {
            autumn_base = in_var_autumn_color;
        }
        base_color = autumn_base;
        color_var = color_rand * 0.5f;
    }
    else
    {
        float3 flowery_or_vertex_color;
        float color_var_alt;
        if (frame_constants_glade_theme_id == 1u)
        {
            flowery_or_vertex_color = in_var_flowery_color;
            color_var_alt = color_rand * 0.5f;
        }
        else
        {
            // 主题 3 且 flower_type==0:高蓝通道顶点色用紫蓝,否则顶点色。
            bool is_theme3_flowertype0;
            if (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) == 0u)
            {
                is_theme3_flowertype0 = frame_constants_glade_theme_id == 3u;
            }
            else
            {
                is_theme3_flowertype0 = false;
            }
            float3 theme3_color;
            if (is_theme3_flowertype0)
            {
                bool3 is_color_z_high = (in_var_Vertex_Color.z > 0.20000000298023223876953125f).xxx;
                theme3_color = float3(is_color_z_high.x ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).x : in_var_Vertex_Color.x, is_color_z_high.y ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).y : in_var_Vertex_Color.y, is_color_z_high.z ? float3(0.2100000083446502685546875f, 0.2100000083446502685546875f, 0.3499999940395355224609375f).z : in_var_Vertex_Color.z);
            }
            else
            {
                theme3_color = in_var_Vertex_Color;
            }
            flowery_or_vertex_color = theme3_color;
            color_var_alt = color_rand;
        }
        base_color = flowery_or_vertex_color;
        color_var = color_var_alt;
    }
    // 明暗色:实例有自定义 color 时按主题选固定暗色调,否则用 base_color。
    float3 shade_color;
    if (nani_instance_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48) > (-1.0f))
    {
        bool is_theme2_or_4_shade;
        if (not_theme2)
        {
            is_theme2_or_4_shade = frame_constants_glade_theme_id == 4u;
        }
        else
        {
            is_theme2_or_4_shade = true;
        }
        float3 theme_a_shade;
        if (is_theme2_or_4_shade)
        {
            theme_a_shade = float3(0.207062780857086181640625f, 0.16431677341461181640625f, 0.291862964630126953125f);
        }
        else
        {
            float3 theme_b_or_3_shade;
            if (frame_constants_glade_theme_id == 1u)
            {
                theme_b_or_3_shade = float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f);
            }
            else
            {
                bool3 is_theme3 = (frame_constants_glade_theme_id == 3u).xxx;
                theme_b_or_3_shade = float3(is_theme3.x ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).x : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).x, is_theme3.y ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).y : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).y, is_theme3.z ? float3(0.3105941712856292724609375f, 0.246475160121917724609375f, 0.4377944469451904296875f).z : float3(0.12679146230220794677734375f, 0.30534231662750244140625f, 0.4958840310573577880859375f).z);
            }
            theme_a_shade = theme_b_or_3_shade;
        }
        shade_color = theme_a_shade;
    }
    else
    {
        shade_color = base_color;
    }
    // 风噪声晶格:原点 xz 平移 0.1 倍 + 时间平移。
    float2 origin_xz = origin.xz;
    float2 wind_uv = (origin_xz * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    // 角点 (0,0) 的 LCG 哈希。
    uint2 wind_cell00_u = uint2(wind_cell);
    uint cell00_y = wind_cell00_u.y;
    uint h00_a = cell00_y + (cell00_y << 10u);
    uint h00_b = h00_a ^ (h00_a >> 6u);
    uint h00_c = h00_b + (h00_b << 3u);
    uint h00_d = h00_c ^ (h00_c >> 11u);
    uint h00_e = (((wind_cell00_u.x * 1664525u) + (h00_d + (h00_d << 15u))) + 1013904223u) * 1664525u;
    uint h00_f = h00_e ^ (h00_e >> 11u);
    uint h00_g = h00_f ^ ((h00_f << 7u) & 2636928640u);
    uint h00 = h00_g ^ ((h00_g << 15u) & 4022730752u);
    // 角点 (1,0) 的 LCG 哈希。
    uint2 wind_cell10_u = uint2(wind_cell + int2(1, 0));
    uint cell10_y = wind_cell10_u.y;
    uint h10_a = cell10_y + (cell10_y << 10u);
    uint h10_b = h10_a ^ (h10_a >> 6u);
    uint h10_c = h10_b + (h10_b << 3u);
    uint h10_d = h10_c ^ (h10_c >> 11u);
    uint h10_e = (((wind_cell10_u.x * 1664525u) + (h10_d + (h10_d << 15u))) + 1013904223u) * 1664525u;
    uint h10_f = h10_e ^ (h10_e >> 11u);
    uint h10_g = h10_f ^ ((h10_f << 7u) & 2636928640u);
    uint h10 = h10_g ^ ((h10_g << 15u) & 4022730752u);
    // 角点 (0,1) 的 LCG 哈希。
    uint2 wind_cell01_u = uint2(wind_cell + int2(0, 1));
    uint cell01_y = wind_cell01_u.y;
    uint h01_a = cell01_y + (cell01_y << 10u);
    uint h01_b = h01_a ^ (h01_a >> 6u);
    uint h01_c = h01_b + (h01_b << 3u);
    uint h01_d = h01_c ^ (h01_c >> 11u);
    uint h01_e = (((wind_cell01_u.x * 1664525u) + (h01_d + (h01_d << 15u))) + 1013904223u) * 1664525u;
    uint h01_f = h01_e ^ (h01_e >> 11u);
    uint h01_g = h01_f ^ ((h01_f << 7u) & 2636928640u);
    uint h01 = h01_g ^ ((h01_g << 15u) & 4022730752u);
    // 角点 (1,1) 的 LCG 哈希。
    uint2 wind_cell11_u = uint2(wind_cell + int2(1, 1));
    uint cell11_y = wind_cell11_u.y;
    uint h11_a = cell11_y + (cell11_y << 10u);
    uint h11_b = h11_a ^ (h11_a >> 6u);
    uint h11_c = h11_b + (h11_b << 3u);
    uint h11_d = h11_c ^ (h11_c >> 11u);
    uint h11_e = (((wind_cell11_u.x * 1664525u) + (h11_d + (h11_d << 15u))) + 1013904223u) * 1664525u;
    uint h11_f = h11_e ^ (h11_e >> 11u);
    uint h11_g = h11_f ^ ((h11_f << 7u) & 2636928640u);
    uint h11 = h11_g ^ ((h11_g << 15u) & 4022730752u);
    // 双线性插值平滑因子与四角噪声 → 花茎弯曲正弦偏移。
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float smooth_x = wind_smooth.x;
    float smooth_x_inv = 1.0f - smooth_x;
    float smooth_y = wind_smooth.y;
    world.x = world.x - (((in_var_wind_bend * in_var_wind_bend) * (sin((((((1.0f - smooth_y) * (((asfloat(((h00 ^ (h00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) + ((asfloat(((h10 ^ (h10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x))) + (((asfloat(((h01 ^ (h01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) * smooth_y)) + (((asfloat(((h11 ^ (h11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    // 草地物体挤压:采样 push 贴图,按 wind_bend 平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * in_var_wind_bend);
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }
    else
    {
        displaced = world;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = pow(shade_color, lerp(1.0f, 2.5f, color_var).xxx);
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_wind_bend = stage_input.in_var_wind_bend;
    in_var_autumn_color = stage_input.in_var_autumn_color;
    in_var_flowery_color = stage_input.in_var_flowery_color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_flower.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
