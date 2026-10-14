// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_balcony_door.raster.hlsl | B nani raster | bins: _nani_balcony_door.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// Nani 阳台门着色器。
//   顶点阶段 vs_main:
//     - 经实例间接缓冲读 packed 仿射变换(64 字节实例);
//     - 门面用 3D 值噪声晶格(8 角点三线性哈希)抖动平面;
//     - 输出顶点色、法线、位移后坐标、UV、is_metal_part、
//       color_id(偏移 52)与 is_semitransparent(偏移 48)。
//   像素阶段 ps_main:
//     - 半透明门做棋盘抖动剔除(每帧轮换亚像素);
//     - 木门采样 window_colors 调色板着色,color_id==0 时调暗 0.7;
//       金属部分直接用顶点色;
//     - 打包 G-Buffer(法线 RGB8 + 0xCC000000,反照率 RGB8 | 0x07000000)。
//   绑定: window_colors(t14)、instance_indirection_buffer(t15)、
//         nani_instance_data(t16)、$Globals(b13)。
//   变体: TGV_b903e43ffb3da915(顶点 + 像素用手写段)、
//         TGV_f0adff76827e73ff(顶点 + 像素只做抖动剔除)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/color_library.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/window_color_customization.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
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

int bool_render_shadows;

// 阳台门实例数据(packed 仿射 + 半透明标志 + 颜色 id + 填充)。
struct InstanceData
{
    Affine3Packed xform;
    uint is_semitransparent;
    uint color_id;
    uint pad0;
    uint pad1;
};

Texture2DArray<float4> window_colors : register(t14);
StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);

// 3D 哈希:混合 p.z/p.y 与 p.x 两个独立混洗通道。
uint hash3(uint3 p)
{
    p.y *= 1664525u;
    uint yz = p.z;
    yz += (yz << 10u);
    yz ^= (yz >> 6u);
    yz += (yz << 3u);
    yz ^= (yz >> 11u);
    p.y += yz + (yz << 15u);
    p.y += 1013904223u;
    p.y *= 1664525u;
    uint y = p.y ^ (p.y >> 11u);
    y = y ^ ((y << 7u) & 2636928640u);
    y = y ^ ((y << 15u) & 4022730752u);
    p.x *= 1664525u;
    p.x += y ^ (y >> 18u);
    p.x += 1013904223u;
    p.x *= 1664525u;
    uint x = p.x ^ (p.x >> 11u);
    x = x ^ ((x << 7u) & 2636928640u);
    x = x ^ ((x << 15u) & 4022730752u);
    x = x ^ (x >> 18u);
    x += (x << 10u);
    x ^= (x >> 6u);
    x += (x << 3u);
    x ^= (x >> 11u);
    return x;
}

// 单个晶格角点的两个随机分量:r1 直接取哈希位,r2 再走一轮哈希。
float2 corner_rand(uint3 cell)
{
    uint h = hash3(cell);
    float r1 = asfloat((h & 8388607u) | 1065353216u) - 1.0f;
    h = h + (h << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float r2 = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    return float2(r1, r2);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int is_metal_part: is_metal_part;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));

    // 门面 3D 值噪声抖动(8 角点三线性哈希晶格)。
    float3 scaled = world * 0.4f;
    int3 cell = int3(floor(scaled));
    float3 fr = frac(scaled);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float2 n = float2(0.0f, 0.0f);
    n += corner_rand(uint3(cell)) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 0, 0))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 1, 0))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(1, 1, 0))) * f.x * f.y * (1.0f - f.z);
    n += corner_rand(uint3(cell + int3(0, 0, 1))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(1, 0, 1))) * f.x * (1.0f - f.y) * f.z;
    n += corner_rand(uint3(cell + int3(0, 1, 1))) * (1.0f - f.x) * f.y * f.z;
    n += corner_rand(uint3(cell + int3(1, 1, 1))) * f.x * f.y * f.z;
    float2 offset = clamp((n - 0.5f.xx) * 2.5f, -0.5f.xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.06f);

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = vin.is_metal_part;
    o.C5 = int(nani_instance_data[idx].color_id);
    o.C6 = nani_instance_data[idx].is_semitransparent;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    nointerpolation int C5 : TEXCOORD5;
    nointerpolation uint C6 : TEXCOORD6;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 半透明门在阴影渲染时做棋盘抖动剔除(按帧索引轮换亚像素)。
    bool dither = pin.C6 != 0u && bool_render_shadows != 0;
    if (dither)
    {
        static const uint2 hi_px_subpixels[4] =
        {
            uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u)
        };
        uint2 sub = hi_px_subpixels[frame_index & 3u];
        uint2 cell = uint2(pin.position.xy) & uint2(1u, 1u);
        if (all(cell != sub))
        {
            discard;
        }
    }

    // 木门部分:采样 window_colors 调色板,按 color_id 选色;id==0 调暗 0.7。
    float3 albedo;
    if (pin.C4 != 1)
    {
        uint seed = uint(pin.C5);
        seed = hash(seed);
        float rnd = asfloat((seed & 8388607u) | 1065353216u) - 1.0f;
        float3 door_color = window_colors.Sample(sampler_llr, float3((pin.C3 + rnd.xx) + (pin.C2.xz * 0.1f), float(pin.C5))).xyz;
        if (pin.C5 == 0)
        {
            door_color = door_color * 0.7f;
        }
        albedo = pin.C5 == 0 ? lerp(door_color, pin.C0, 0.3f.xxx) : door_color;
    }
    else
    {
        // 金属部分直接用顶点色。
        albedo = pin.C0;
    }

    // 打包 G-Buffer:法线压到 [0,1],反照率 sqrt 到 sRGB 后 8bit 量化,带材质标志。
    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3422552064u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif


#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_balcony_door.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
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
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    // 门面 3D 值噪声抖动:世界坐标缩放 0.4 后做三线性哈希晶格。
    float3 scaled = world * 0.4000000059604644775390625f;
    int3 cell_int = int3(floor(scaled));
    float3 frac_part = frac(scaled);
    // 8 个角点逐一的 hash3 + 二次哈希,得到 (r1, r2) 两个随机分量。
    // 角点晶格 cnr_000 的 hash3 与角点随机向量。
    uint3 cnr_000_cell = uint3(cell_int);
    uint cnr_000_z = cnr_000_cell.z;
    uint cnr_000_z1 = cnr_000_z + (cnr_000_z << 10u);
    uint cnr_000_z2 = cnr_000_z1 ^ (cnr_000_z1 >> 6u);
    uint cnr_000_z3 = cnr_000_z2 + (cnr_000_z2 << 3u);
    uint cnr_000_z4 = cnr_000_z3 ^ (cnr_000_z3 >> 11u);
    uint cnr_000_y = (((cnr_000_cell.y * 1664525u) + (cnr_000_z4 + (cnr_000_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_000_y1 = cnr_000_y ^ (cnr_000_y >> 11u);
    uint cnr_000_y2 = cnr_000_y1 ^ ((cnr_000_y1 << 7u) & 2636928640u);
    uint cnr_000_y3 = cnr_000_y2 ^ ((cnr_000_y2 << 15u) & 4022730752u);
    uint cnr_000_x = (((cnr_000_cell.x * 1664525u) + (cnr_000_y3 ^ (cnr_000_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_000_x1 = cnr_000_x ^ (cnr_000_x >> 11u);
    uint cnr_000_x2 = cnr_000_x1 ^ ((cnr_000_x1 << 7u) & 2636928640u);
    uint cnr_000_x3 = cnr_000_x2 ^ ((cnr_000_x2 << 15u) & 4022730752u);
    uint cnr_000_rs = cnr_000_x3 ^ (cnr_000_x3 >> 18u);
    uint cnr_000_r1 = cnr_000_rs + (cnr_000_rs << 10u);
    uint cnr_000_r2 = cnr_000_r1 ^ (cnr_000_r1 >> 6u);
    uint cnr_000_r3 = cnr_000_r2 + (cnr_000_r2 << 3u);
    uint cnr_000_r4 = cnr_000_r3 ^ (cnr_000_r3 >> 11u);
    // 角点晶格 cnr_100 的 hash3 与角点随机向量。
    uint3 cnr_100_cell = uint3(cell_int + int3(1, 0, 0));
    uint cnr_100_z = cnr_100_cell.z;
    uint cnr_100_z1 = cnr_100_z + (cnr_100_z << 10u);
    uint cnr_100_z2 = cnr_100_z1 ^ (cnr_100_z1 >> 6u);
    uint cnr_100_z3 = cnr_100_z2 + (cnr_100_z2 << 3u);
    uint cnr_100_z4 = cnr_100_z3 ^ (cnr_100_z3 >> 11u);
    uint cnr_100_y = (((cnr_100_cell.y * 1664525u) + (cnr_100_z4 + (cnr_100_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_100_y1 = cnr_100_y ^ (cnr_100_y >> 11u);
    uint cnr_100_y2 = cnr_100_y1 ^ ((cnr_100_y1 << 7u) & 2636928640u);
    uint cnr_100_y3 = cnr_100_y2 ^ ((cnr_100_y2 << 15u) & 4022730752u);
    uint cnr_100_x = (((cnr_100_cell.x * 1664525u) + (cnr_100_y3 ^ (cnr_100_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_100_x1 = cnr_100_x ^ (cnr_100_x >> 11u);
    uint cnr_100_x2 = cnr_100_x1 ^ ((cnr_100_x1 << 7u) & 2636928640u);
    uint cnr_100_x3 = cnr_100_x2 ^ ((cnr_100_x2 << 15u) & 4022730752u);
    uint cnr_100_rs = cnr_100_x3 ^ (cnr_100_x3 >> 18u);
    uint cnr_100_r1 = cnr_100_rs + (cnr_100_rs << 10u);
    uint cnr_100_r2 = cnr_100_r1 ^ (cnr_100_r1 >> 6u);
    uint cnr_100_r3 = cnr_100_r2 + (cnr_100_r2 << 3u);
    uint cnr_100_r4 = cnr_100_r3 ^ (cnr_100_r3 >> 11u);
    // 角点晶格 cnr_010 的 hash3 与角点随机向量。
    uint3 cnr_010_cell = uint3(cell_int + int3(0, 1, 0));
    uint cnr_010_z = cnr_010_cell.z;
    uint cnr_010_z1 = cnr_010_z + (cnr_010_z << 10u);
    uint cnr_010_z2 = cnr_010_z1 ^ (cnr_010_z1 >> 6u);
    uint cnr_010_z3 = cnr_010_z2 + (cnr_010_z2 << 3u);
    uint cnr_010_z4 = cnr_010_z3 ^ (cnr_010_z3 >> 11u);
    uint cnr_010_y = (((cnr_010_cell.y * 1664525u) + (cnr_010_z4 + (cnr_010_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_010_y1 = cnr_010_y ^ (cnr_010_y >> 11u);
    uint cnr_010_y2 = cnr_010_y1 ^ ((cnr_010_y1 << 7u) & 2636928640u);
    uint cnr_010_y3 = cnr_010_y2 ^ ((cnr_010_y2 << 15u) & 4022730752u);
    uint cnr_010_x = (((cnr_010_cell.x * 1664525u) + (cnr_010_y3 ^ (cnr_010_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_010_x1 = cnr_010_x ^ (cnr_010_x >> 11u);
    uint cnr_010_x2 = cnr_010_x1 ^ ((cnr_010_x1 << 7u) & 2636928640u);
    uint cnr_010_x3 = cnr_010_x2 ^ ((cnr_010_x2 << 15u) & 4022730752u);
    uint cnr_010_rs = cnr_010_x3 ^ (cnr_010_x3 >> 18u);
    uint cnr_010_r1 = cnr_010_rs + (cnr_010_rs << 10u);
    uint cnr_010_r2 = cnr_010_r1 ^ (cnr_010_r1 >> 6u);
    uint cnr_010_r3 = cnr_010_r2 + (cnr_010_r2 << 3u);
    uint cnr_010_r4 = cnr_010_r3 ^ (cnr_010_r3 >> 11u);
    // 角点晶格 cnr_110 的 hash3 与角点随机向量。
    uint3 cnr_110_cell = uint3(cell_int + int3(1, 1, 0));
    uint cnr_110_z = cnr_110_cell.z;
    uint cnr_110_z1 = cnr_110_z + (cnr_110_z << 10u);
    uint cnr_110_z2 = cnr_110_z1 ^ (cnr_110_z1 >> 6u);
    uint cnr_110_z3 = cnr_110_z2 + (cnr_110_z2 << 3u);
    uint cnr_110_z4 = cnr_110_z3 ^ (cnr_110_z3 >> 11u);
    uint cnr_110_y = (((cnr_110_cell.y * 1664525u) + (cnr_110_z4 + (cnr_110_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_110_y1 = cnr_110_y ^ (cnr_110_y >> 11u);
    uint cnr_110_y2 = cnr_110_y1 ^ ((cnr_110_y1 << 7u) & 2636928640u);
    uint cnr_110_y3 = cnr_110_y2 ^ ((cnr_110_y2 << 15u) & 4022730752u);
    uint cnr_110_x = (((cnr_110_cell.x * 1664525u) + (cnr_110_y3 ^ (cnr_110_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_110_x1 = cnr_110_x ^ (cnr_110_x >> 11u);
    uint cnr_110_x2 = cnr_110_x1 ^ ((cnr_110_x1 << 7u) & 2636928640u);
    uint cnr_110_x3 = cnr_110_x2 ^ ((cnr_110_x2 << 15u) & 4022730752u);
    uint cnr_110_rs = cnr_110_x3 ^ (cnr_110_x3 >> 18u);
    uint cnr_110_r1 = cnr_110_rs + (cnr_110_rs << 10u);
    uint cnr_110_r2 = cnr_110_r1 ^ (cnr_110_r1 >> 6u);
    uint cnr_110_r3 = cnr_110_r2 + (cnr_110_r2 << 3u);
    uint cnr_110_r4 = cnr_110_r3 ^ (cnr_110_r3 >> 11u);
    // 角点晶格 cnr_001 的 hash3 与角点随机向量。
    uint3 cnr_001_cell = uint3(cell_int + int3(0, 0, 1));
    uint cnr_001_z = cnr_001_cell.z;
    uint cnr_001_z1 = cnr_001_z + (cnr_001_z << 10u);
    uint cnr_001_z2 = cnr_001_z1 ^ (cnr_001_z1 >> 6u);
    uint cnr_001_z3 = cnr_001_z2 + (cnr_001_z2 << 3u);
    uint cnr_001_z4 = cnr_001_z3 ^ (cnr_001_z3 >> 11u);
    uint cnr_001_y = (((cnr_001_cell.y * 1664525u) + (cnr_001_z4 + (cnr_001_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_001_y1 = cnr_001_y ^ (cnr_001_y >> 11u);
    uint cnr_001_y2 = cnr_001_y1 ^ ((cnr_001_y1 << 7u) & 2636928640u);
    uint cnr_001_y3 = cnr_001_y2 ^ ((cnr_001_y2 << 15u) & 4022730752u);
    uint cnr_001_x = (((cnr_001_cell.x * 1664525u) + (cnr_001_y3 ^ (cnr_001_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_001_x1 = cnr_001_x ^ (cnr_001_x >> 11u);
    uint cnr_001_x2 = cnr_001_x1 ^ ((cnr_001_x1 << 7u) & 2636928640u);
    uint cnr_001_x3 = cnr_001_x2 ^ ((cnr_001_x2 << 15u) & 4022730752u);
    uint cnr_001_rs = cnr_001_x3 ^ (cnr_001_x3 >> 18u);
    uint cnr_001_r1 = cnr_001_rs + (cnr_001_rs << 10u);
    uint cnr_001_r2 = cnr_001_r1 ^ (cnr_001_r1 >> 6u);
    uint cnr_001_r3 = cnr_001_r2 + (cnr_001_r2 << 3u);
    uint cnr_001_r4 = cnr_001_r3 ^ (cnr_001_r3 >> 11u);
    // 角点晶格 cnr_101 的 hash3 与角点随机向量。
    uint3 cnr_101_cell = uint3(cell_int + int3(1, 0, 1));
    uint cnr_101_z = cnr_101_cell.z;
    uint cnr_101_z1 = cnr_101_z + (cnr_101_z << 10u);
    uint cnr_101_z2 = cnr_101_z1 ^ (cnr_101_z1 >> 6u);
    uint cnr_101_z3 = cnr_101_z2 + (cnr_101_z2 << 3u);
    uint cnr_101_z4 = cnr_101_z3 ^ (cnr_101_z3 >> 11u);
    uint cnr_101_y = (((cnr_101_cell.y * 1664525u) + (cnr_101_z4 + (cnr_101_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_101_y1 = cnr_101_y ^ (cnr_101_y >> 11u);
    uint cnr_101_y2 = cnr_101_y1 ^ ((cnr_101_y1 << 7u) & 2636928640u);
    uint cnr_101_y3 = cnr_101_y2 ^ ((cnr_101_y2 << 15u) & 4022730752u);
    uint cnr_101_x = (((cnr_101_cell.x * 1664525u) + (cnr_101_y3 ^ (cnr_101_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_101_x1 = cnr_101_x ^ (cnr_101_x >> 11u);
    uint cnr_101_x2 = cnr_101_x1 ^ ((cnr_101_x1 << 7u) & 2636928640u);
    uint cnr_101_x3 = cnr_101_x2 ^ ((cnr_101_x2 << 15u) & 4022730752u);
    uint cnr_101_rs = cnr_101_x3 ^ (cnr_101_x3 >> 18u);
    uint cnr_101_r1 = cnr_101_rs + (cnr_101_rs << 10u);
    uint cnr_101_r2 = cnr_101_r1 ^ (cnr_101_r1 >> 6u);
    uint cnr_101_r3 = cnr_101_r2 + (cnr_101_r2 << 3u);
    uint cnr_101_r4 = cnr_101_r3 ^ (cnr_101_r3 >> 11u);
    // 角点晶格 cnr_011 的 hash3 与角点随机向量。
    uint3 cnr_011_cell = uint3(cell_int + int3(0, 1, 1));
    uint cnr_011_z = cnr_011_cell.z;
    uint cnr_011_z1 = cnr_011_z + (cnr_011_z << 10u);
    uint cnr_011_z2 = cnr_011_z1 ^ (cnr_011_z1 >> 6u);
    uint cnr_011_z3 = cnr_011_z2 + (cnr_011_z2 << 3u);
    uint cnr_011_z4 = cnr_011_z3 ^ (cnr_011_z3 >> 11u);
    uint cnr_011_y = (((cnr_011_cell.y * 1664525u) + (cnr_011_z4 + (cnr_011_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_011_y1 = cnr_011_y ^ (cnr_011_y >> 11u);
    uint cnr_011_y2 = cnr_011_y1 ^ ((cnr_011_y1 << 7u) & 2636928640u);
    uint cnr_011_y3 = cnr_011_y2 ^ ((cnr_011_y2 << 15u) & 4022730752u);
    uint cnr_011_x = (((cnr_011_cell.x * 1664525u) + (cnr_011_y3 ^ (cnr_011_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_011_x1 = cnr_011_x ^ (cnr_011_x >> 11u);
    uint cnr_011_x2 = cnr_011_x1 ^ ((cnr_011_x1 << 7u) & 2636928640u);
    uint cnr_011_x3 = cnr_011_x2 ^ ((cnr_011_x2 << 15u) & 4022730752u);
    uint cnr_011_rs = cnr_011_x3 ^ (cnr_011_x3 >> 18u);
    uint cnr_011_r1 = cnr_011_rs + (cnr_011_rs << 10u);
    uint cnr_011_r2 = cnr_011_r1 ^ (cnr_011_r1 >> 6u);
    uint cnr_011_r3 = cnr_011_r2 + (cnr_011_r2 << 3u);
    uint cnr_011_r4 = cnr_011_r3 ^ (cnr_011_r3 >> 11u);
    // 角点晶格 cnr_111 的 hash3 与角点随机向量。
    uint3 cnr_111_cell = uint3(cell_int + int3(1, 1, 1));
    uint cnr_111_z = cnr_111_cell.z;
    uint cnr_111_z1 = cnr_111_z + (cnr_111_z << 10u);
    uint cnr_111_z2 = cnr_111_z1 ^ (cnr_111_z1 >> 6u);
    uint cnr_111_z3 = cnr_111_z2 + (cnr_111_z2 << 3u);
    uint cnr_111_z4 = cnr_111_z3 ^ (cnr_111_z3 >> 11u);
    uint cnr_111_y = (((cnr_111_cell.y * 1664525u) + (cnr_111_z4 + (cnr_111_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_111_y1 = cnr_111_y ^ (cnr_111_y >> 11u);
    uint cnr_111_y2 = cnr_111_y1 ^ ((cnr_111_y1 << 7u) & 2636928640u);
    uint cnr_111_y3 = cnr_111_y2 ^ ((cnr_111_y2 << 15u) & 4022730752u);
    uint cnr_111_x = (((cnr_111_cell.x * 1664525u) + (cnr_111_y3 ^ (cnr_111_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_111_x1 = cnr_111_x ^ (cnr_111_x >> 11u);
    uint cnr_111_x2 = cnr_111_x1 ^ ((cnr_111_x1 << 7u) & 2636928640u);
    uint cnr_111_x3 = cnr_111_x2 ^ ((cnr_111_x2 << 15u) & 4022730752u);
    uint cnr_111_rs = cnr_111_x3 ^ (cnr_111_x3 >> 18u);
    uint cnr_111_r1 = cnr_111_rs + (cnr_111_rs << 10u);
    uint cnr_111_r2 = cnr_111_r1 ^ (cnr_111_r1 >> 6u);
    uint cnr_111_r3 = cnr_111_r2 + (cnr_111_r2 << 3u);
    uint cnr_111_r4 = cnr_111_r3 ^ (cnr_111_r3 >> 11u);    float3 smooth = (frac_part * frac_part) * (3.0f.xxx - (frac_part * 2.0f));
    float fx = smooth.x;
    float fx_inv = 1.0f - fx;
    float fy = smooth.y;
    float fy_inv = 1.0f - fy;
    float fz = smooth.z;
    float fz_inv = 1.0f - fz;
    // 三线性插值 8 个角点随机向量 → 抖动偏移,clamp 到 ±0.5。
    float2 n = ((((((float2(asfloat((cnr_000_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_000_r4 + (cnr_000_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy_inv) * fz_inv) + (((float2(asfloat((cnr_100_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_100_r4 + (cnr_100_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy_inv) * fz_inv)) + (((float2(asfloat((cnr_010_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_010_r4 + (cnr_010_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy) * fz_inv)) + (((float2(asfloat((cnr_110_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_110_r4 + (cnr_110_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy) * fz_inv)) + (((float2(asfloat((cnr_001_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_001_r4 + (cnr_001_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy_inv) * fz);
    float2 offset = clamp(((((n + (((float2(asfloat((cnr_101_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_101_r4 + (cnr_101_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy_inv) * fz)) + (((float2(asfloat((cnr_011_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_011_r4 + (cnr_011_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy) * fz)) + (((float2(asfloat((cnr_111_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_111_r4 + (cnr_111_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy) * fz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_balcony_door.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint is_semitransparent;
    int color_id;
    uint pad0;
    uint pad1;
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
    int bool_render_shadows : packoffset(c0.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_metal_part;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static int out_var_C5;
static uint out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_metal_part: is_metal_part;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation int out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
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
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    // 门面 3D 值噪声抖动:世界坐标缩放 0.4 后做三线性哈希晶格。
    float3 scaled = world * 0.4000000059604644775390625f;
    int3 cell_int = int3(floor(scaled));
    float3 frac_part = frac(scaled);
    // 8 个角点逐一的 hash3 + 二次哈希,得到 (r1, r2) 两个随机分量。
    // 角点晶格 cnr_000 的 hash3 与角点随机向量。
    uint3 cnr_000_cell = uint3(cell_int);
    uint cnr_000_z = cnr_000_cell.z;
    uint cnr_000_z1 = cnr_000_z + (cnr_000_z << 10u);
    uint cnr_000_z2 = cnr_000_z1 ^ (cnr_000_z1 >> 6u);
    uint cnr_000_z3 = cnr_000_z2 + (cnr_000_z2 << 3u);
    uint cnr_000_z4 = cnr_000_z3 ^ (cnr_000_z3 >> 11u);
    uint cnr_000_y = (((cnr_000_cell.y * 1664525u) + (cnr_000_z4 + (cnr_000_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_000_y1 = cnr_000_y ^ (cnr_000_y >> 11u);
    uint cnr_000_y2 = cnr_000_y1 ^ ((cnr_000_y1 << 7u) & 2636928640u);
    uint cnr_000_y3 = cnr_000_y2 ^ ((cnr_000_y2 << 15u) & 4022730752u);
    uint cnr_000_x = (((cnr_000_cell.x * 1664525u) + (cnr_000_y3 ^ (cnr_000_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_000_x1 = cnr_000_x ^ (cnr_000_x >> 11u);
    uint cnr_000_x2 = cnr_000_x1 ^ ((cnr_000_x1 << 7u) & 2636928640u);
    uint cnr_000_x3 = cnr_000_x2 ^ ((cnr_000_x2 << 15u) & 4022730752u);
    uint cnr_000_rs = cnr_000_x3 ^ (cnr_000_x3 >> 18u);
    uint cnr_000_r1 = cnr_000_rs + (cnr_000_rs << 10u);
    uint cnr_000_r2 = cnr_000_r1 ^ (cnr_000_r1 >> 6u);
    uint cnr_000_r3 = cnr_000_r2 + (cnr_000_r2 << 3u);
    uint cnr_000_r4 = cnr_000_r3 ^ (cnr_000_r3 >> 11u);
    // 角点晶格 cnr_100 的 hash3 与角点随机向量。
    uint3 cnr_100_cell = uint3(cell_int + int3(1, 0, 0));
    uint cnr_100_z = cnr_100_cell.z;
    uint cnr_100_z1 = cnr_100_z + (cnr_100_z << 10u);
    uint cnr_100_z2 = cnr_100_z1 ^ (cnr_100_z1 >> 6u);
    uint cnr_100_z3 = cnr_100_z2 + (cnr_100_z2 << 3u);
    uint cnr_100_z4 = cnr_100_z3 ^ (cnr_100_z3 >> 11u);
    uint cnr_100_y = (((cnr_100_cell.y * 1664525u) + (cnr_100_z4 + (cnr_100_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_100_y1 = cnr_100_y ^ (cnr_100_y >> 11u);
    uint cnr_100_y2 = cnr_100_y1 ^ ((cnr_100_y1 << 7u) & 2636928640u);
    uint cnr_100_y3 = cnr_100_y2 ^ ((cnr_100_y2 << 15u) & 4022730752u);
    uint cnr_100_x = (((cnr_100_cell.x * 1664525u) + (cnr_100_y3 ^ (cnr_100_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_100_x1 = cnr_100_x ^ (cnr_100_x >> 11u);
    uint cnr_100_x2 = cnr_100_x1 ^ ((cnr_100_x1 << 7u) & 2636928640u);
    uint cnr_100_x3 = cnr_100_x2 ^ ((cnr_100_x2 << 15u) & 4022730752u);
    uint cnr_100_rs = cnr_100_x3 ^ (cnr_100_x3 >> 18u);
    uint cnr_100_r1 = cnr_100_rs + (cnr_100_rs << 10u);
    uint cnr_100_r2 = cnr_100_r1 ^ (cnr_100_r1 >> 6u);
    uint cnr_100_r3 = cnr_100_r2 + (cnr_100_r2 << 3u);
    uint cnr_100_r4 = cnr_100_r3 ^ (cnr_100_r3 >> 11u);
    // 角点晶格 cnr_010 的 hash3 与角点随机向量。
    uint3 cnr_010_cell = uint3(cell_int + int3(0, 1, 0));
    uint cnr_010_z = cnr_010_cell.z;
    uint cnr_010_z1 = cnr_010_z + (cnr_010_z << 10u);
    uint cnr_010_z2 = cnr_010_z1 ^ (cnr_010_z1 >> 6u);
    uint cnr_010_z3 = cnr_010_z2 + (cnr_010_z2 << 3u);
    uint cnr_010_z4 = cnr_010_z3 ^ (cnr_010_z3 >> 11u);
    uint cnr_010_y = (((cnr_010_cell.y * 1664525u) + (cnr_010_z4 + (cnr_010_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_010_y1 = cnr_010_y ^ (cnr_010_y >> 11u);
    uint cnr_010_y2 = cnr_010_y1 ^ ((cnr_010_y1 << 7u) & 2636928640u);
    uint cnr_010_y3 = cnr_010_y2 ^ ((cnr_010_y2 << 15u) & 4022730752u);
    uint cnr_010_x = (((cnr_010_cell.x * 1664525u) + (cnr_010_y3 ^ (cnr_010_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_010_x1 = cnr_010_x ^ (cnr_010_x >> 11u);
    uint cnr_010_x2 = cnr_010_x1 ^ ((cnr_010_x1 << 7u) & 2636928640u);
    uint cnr_010_x3 = cnr_010_x2 ^ ((cnr_010_x2 << 15u) & 4022730752u);
    uint cnr_010_rs = cnr_010_x3 ^ (cnr_010_x3 >> 18u);
    uint cnr_010_r1 = cnr_010_rs + (cnr_010_rs << 10u);
    uint cnr_010_r2 = cnr_010_r1 ^ (cnr_010_r1 >> 6u);
    uint cnr_010_r3 = cnr_010_r2 + (cnr_010_r2 << 3u);
    uint cnr_010_r4 = cnr_010_r3 ^ (cnr_010_r3 >> 11u);
    // 角点晶格 cnr_110 的 hash3 与角点随机向量。
    uint3 cnr_110_cell = uint3(cell_int + int3(1, 1, 0));
    uint cnr_110_z = cnr_110_cell.z;
    uint cnr_110_z1 = cnr_110_z + (cnr_110_z << 10u);
    uint cnr_110_z2 = cnr_110_z1 ^ (cnr_110_z1 >> 6u);
    uint cnr_110_z3 = cnr_110_z2 + (cnr_110_z2 << 3u);
    uint cnr_110_z4 = cnr_110_z3 ^ (cnr_110_z3 >> 11u);
    uint cnr_110_y = (((cnr_110_cell.y * 1664525u) + (cnr_110_z4 + (cnr_110_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_110_y1 = cnr_110_y ^ (cnr_110_y >> 11u);
    uint cnr_110_y2 = cnr_110_y1 ^ ((cnr_110_y1 << 7u) & 2636928640u);
    uint cnr_110_y3 = cnr_110_y2 ^ ((cnr_110_y2 << 15u) & 4022730752u);
    uint cnr_110_x = (((cnr_110_cell.x * 1664525u) + (cnr_110_y3 ^ (cnr_110_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_110_x1 = cnr_110_x ^ (cnr_110_x >> 11u);
    uint cnr_110_x2 = cnr_110_x1 ^ ((cnr_110_x1 << 7u) & 2636928640u);
    uint cnr_110_x3 = cnr_110_x2 ^ ((cnr_110_x2 << 15u) & 4022730752u);
    uint cnr_110_rs = cnr_110_x3 ^ (cnr_110_x3 >> 18u);
    uint cnr_110_r1 = cnr_110_rs + (cnr_110_rs << 10u);
    uint cnr_110_r2 = cnr_110_r1 ^ (cnr_110_r1 >> 6u);
    uint cnr_110_r3 = cnr_110_r2 + (cnr_110_r2 << 3u);
    uint cnr_110_r4 = cnr_110_r3 ^ (cnr_110_r3 >> 11u);
    // 角点晶格 cnr_001 的 hash3 与角点随机向量。
    uint3 cnr_001_cell = uint3(cell_int + int3(0, 0, 1));
    uint cnr_001_z = cnr_001_cell.z;
    uint cnr_001_z1 = cnr_001_z + (cnr_001_z << 10u);
    uint cnr_001_z2 = cnr_001_z1 ^ (cnr_001_z1 >> 6u);
    uint cnr_001_z3 = cnr_001_z2 + (cnr_001_z2 << 3u);
    uint cnr_001_z4 = cnr_001_z3 ^ (cnr_001_z3 >> 11u);
    uint cnr_001_y = (((cnr_001_cell.y * 1664525u) + (cnr_001_z4 + (cnr_001_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_001_y1 = cnr_001_y ^ (cnr_001_y >> 11u);
    uint cnr_001_y2 = cnr_001_y1 ^ ((cnr_001_y1 << 7u) & 2636928640u);
    uint cnr_001_y3 = cnr_001_y2 ^ ((cnr_001_y2 << 15u) & 4022730752u);
    uint cnr_001_x = (((cnr_001_cell.x * 1664525u) + (cnr_001_y3 ^ (cnr_001_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_001_x1 = cnr_001_x ^ (cnr_001_x >> 11u);
    uint cnr_001_x2 = cnr_001_x1 ^ ((cnr_001_x1 << 7u) & 2636928640u);
    uint cnr_001_x3 = cnr_001_x2 ^ ((cnr_001_x2 << 15u) & 4022730752u);
    uint cnr_001_rs = cnr_001_x3 ^ (cnr_001_x3 >> 18u);
    uint cnr_001_r1 = cnr_001_rs + (cnr_001_rs << 10u);
    uint cnr_001_r2 = cnr_001_r1 ^ (cnr_001_r1 >> 6u);
    uint cnr_001_r3 = cnr_001_r2 + (cnr_001_r2 << 3u);
    uint cnr_001_r4 = cnr_001_r3 ^ (cnr_001_r3 >> 11u);
    // 角点晶格 cnr_101 的 hash3 与角点随机向量。
    uint3 cnr_101_cell = uint3(cell_int + int3(1, 0, 1));
    uint cnr_101_z = cnr_101_cell.z;
    uint cnr_101_z1 = cnr_101_z + (cnr_101_z << 10u);
    uint cnr_101_z2 = cnr_101_z1 ^ (cnr_101_z1 >> 6u);
    uint cnr_101_z3 = cnr_101_z2 + (cnr_101_z2 << 3u);
    uint cnr_101_z4 = cnr_101_z3 ^ (cnr_101_z3 >> 11u);
    uint cnr_101_y = (((cnr_101_cell.y * 1664525u) + (cnr_101_z4 + (cnr_101_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_101_y1 = cnr_101_y ^ (cnr_101_y >> 11u);
    uint cnr_101_y2 = cnr_101_y1 ^ ((cnr_101_y1 << 7u) & 2636928640u);
    uint cnr_101_y3 = cnr_101_y2 ^ ((cnr_101_y2 << 15u) & 4022730752u);
    uint cnr_101_x = (((cnr_101_cell.x * 1664525u) + (cnr_101_y3 ^ (cnr_101_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_101_x1 = cnr_101_x ^ (cnr_101_x >> 11u);
    uint cnr_101_x2 = cnr_101_x1 ^ ((cnr_101_x1 << 7u) & 2636928640u);
    uint cnr_101_x3 = cnr_101_x2 ^ ((cnr_101_x2 << 15u) & 4022730752u);
    uint cnr_101_rs = cnr_101_x3 ^ (cnr_101_x3 >> 18u);
    uint cnr_101_r1 = cnr_101_rs + (cnr_101_rs << 10u);
    uint cnr_101_r2 = cnr_101_r1 ^ (cnr_101_r1 >> 6u);
    uint cnr_101_r3 = cnr_101_r2 + (cnr_101_r2 << 3u);
    uint cnr_101_r4 = cnr_101_r3 ^ (cnr_101_r3 >> 11u);
    // 角点晶格 cnr_011 的 hash3 与角点随机向量。
    uint3 cnr_011_cell = uint3(cell_int + int3(0, 1, 1));
    uint cnr_011_z = cnr_011_cell.z;
    uint cnr_011_z1 = cnr_011_z + (cnr_011_z << 10u);
    uint cnr_011_z2 = cnr_011_z1 ^ (cnr_011_z1 >> 6u);
    uint cnr_011_z3 = cnr_011_z2 + (cnr_011_z2 << 3u);
    uint cnr_011_z4 = cnr_011_z3 ^ (cnr_011_z3 >> 11u);
    uint cnr_011_y = (((cnr_011_cell.y * 1664525u) + (cnr_011_z4 + (cnr_011_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_011_y1 = cnr_011_y ^ (cnr_011_y >> 11u);
    uint cnr_011_y2 = cnr_011_y1 ^ ((cnr_011_y1 << 7u) & 2636928640u);
    uint cnr_011_y3 = cnr_011_y2 ^ ((cnr_011_y2 << 15u) & 4022730752u);
    uint cnr_011_x = (((cnr_011_cell.x * 1664525u) + (cnr_011_y3 ^ (cnr_011_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_011_x1 = cnr_011_x ^ (cnr_011_x >> 11u);
    uint cnr_011_x2 = cnr_011_x1 ^ ((cnr_011_x1 << 7u) & 2636928640u);
    uint cnr_011_x3 = cnr_011_x2 ^ ((cnr_011_x2 << 15u) & 4022730752u);
    uint cnr_011_rs = cnr_011_x3 ^ (cnr_011_x3 >> 18u);
    uint cnr_011_r1 = cnr_011_rs + (cnr_011_rs << 10u);
    uint cnr_011_r2 = cnr_011_r1 ^ (cnr_011_r1 >> 6u);
    uint cnr_011_r3 = cnr_011_r2 + (cnr_011_r2 << 3u);
    uint cnr_011_r4 = cnr_011_r3 ^ (cnr_011_r3 >> 11u);
    // 角点晶格 cnr_111 的 hash3 与角点随机向量。
    uint3 cnr_111_cell = uint3(cell_int + int3(1, 1, 1));
    uint cnr_111_z = cnr_111_cell.z;
    uint cnr_111_z1 = cnr_111_z + (cnr_111_z << 10u);
    uint cnr_111_z2 = cnr_111_z1 ^ (cnr_111_z1 >> 6u);
    uint cnr_111_z3 = cnr_111_z2 + (cnr_111_z2 << 3u);
    uint cnr_111_z4 = cnr_111_z3 ^ (cnr_111_z3 >> 11u);
    uint cnr_111_y = (((cnr_111_cell.y * 1664525u) + (cnr_111_z4 + (cnr_111_z4 << 15u))) + 1013904223u) * 1664525u;
    uint cnr_111_y1 = cnr_111_y ^ (cnr_111_y >> 11u);
    uint cnr_111_y2 = cnr_111_y1 ^ ((cnr_111_y1 << 7u) & 2636928640u);
    uint cnr_111_y3 = cnr_111_y2 ^ ((cnr_111_y2 << 15u) & 4022730752u);
    uint cnr_111_x = (((cnr_111_cell.x * 1664525u) + (cnr_111_y3 ^ (cnr_111_y3 >> 18u))) + 1013904223u) * 1664525u;
    uint cnr_111_x1 = cnr_111_x ^ (cnr_111_x >> 11u);
    uint cnr_111_x2 = cnr_111_x1 ^ ((cnr_111_x1 << 7u) & 2636928640u);
    uint cnr_111_x3 = cnr_111_x2 ^ ((cnr_111_x2 << 15u) & 4022730752u);
    uint cnr_111_rs = cnr_111_x3 ^ (cnr_111_x3 >> 18u);
    uint cnr_111_r1 = cnr_111_rs + (cnr_111_rs << 10u);
    uint cnr_111_r2 = cnr_111_r1 ^ (cnr_111_r1 >> 6u);
    uint cnr_111_r3 = cnr_111_r2 + (cnr_111_r2 << 3u);
    uint cnr_111_r4 = cnr_111_r3 ^ (cnr_111_r3 >> 11u);    float3 smooth = (frac_part * frac_part) * (3.0f.xxx - (frac_part * 2.0f));
    float fx = smooth.x;
    float fx_inv = 1.0f - fx;
    float fy = smooth.y;
    float fy_inv = 1.0f - fy;
    float fz = smooth.z;
    float fz_inv = 1.0f - fz;
    // 三线性插值 8 个角点随机向量 → 抖动偏移,clamp 到 ±0.5。
    float2 n = ((((((float2(asfloat((cnr_000_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_000_r4 + (cnr_000_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy_inv) * fz_inv) + (((float2(asfloat((cnr_100_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_100_r4 + (cnr_100_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy_inv) * fz_inv)) + (((float2(asfloat((cnr_010_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_010_r4 + (cnr_010_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy) * fz_inv)) + (((float2(asfloat((cnr_110_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_110_r4 + (cnr_110_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy) * fz_inv)) + (((float2(asfloat((cnr_001_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_001_r4 + (cnr_001_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy_inv) * fz);
    float2 offset = clamp(((((n + (((float2(asfloat((cnr_101_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_101_r4 + (cnr_101_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy_inv) * fz)) + (((float2(asfloat((cnr_011_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_011_r4 + (cnr_011_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx_inv) * fy) * fz)) + (((float2(asfloat((cnr_111_rs & 8388607u) | 1065353216u) - 1.0f, asfloat(((cnr_111_r4 + (cnr_111_r4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * fx) * fy) * fz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.0599999986588954925537109375f);
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = in_var_is_metal_part;
    out_var_C5 = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_metal_part = stage_input.in_var_is_metal_part;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_balcony_door.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int bool_render_shadows : packoffset(c0.y);
};


static float4 gl_FragCoord;
static uint in_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    // 半透明门在阴影渲染时做棋盘抖动剔除。
    bool dither;
    if (in_var_C6 != 0u)
    {
        dither = bool_render_shadows != 0;
    }
    else
    {
        dither = false;
    }
    if (dither)
    {
        uint2 cell = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint sub_idx = frame_constants_frame_index & 3u;
        if (all(bool2(cell.x != hi_px_subpixels[sub_idx].x, cell.y != hi_px_subpixels[sub_idx].y)))
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

