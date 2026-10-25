// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_bush.raster.hlsl | B nani raster | bins: _nani_bush.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// 矮灌木(nani bush)着色器。
//   顶点阶段 vs_main:
//     - 经实例变换得到世界位置,并用 2D 风噪声晶格(LCG 哈希 + 双线性插值)
//       按顶点高度弯曲灌木(world.x 偏移);
//     - 采样 grass_object_push 高度贴图,把被草地物体挤压的顶点向外推开;
//     - 输出世界位置、法线、UV、灌木部位 id 与顶点高度。
//   像素阶段 ps_main:
//     - 用 bush_alpha 做 alpha 丢弃;
//     - 视线与法线夹角过大(广告牌侧面)时丢弃;
//     - 采样 bush_color 并按顶点高度增亮,glade 主题 4 时把朝上的面淡化;
//     - 打包 G-Buffer(法线 RGB8 + 0x66000000,反照率 RGB8 | 0xB3000000)。
//   绑定: instance_indirection_buffer(t14)、bush_alpha(t15)、bush_color(t16)、
//         grass_object_push(t19)、nani_instance_data(t20)、$Globals(b13)。
//   变体: TGV_b903e43ffb3da915(顶点,像素用手写段)、
//         TGV_f0adff76827e73ff(顶点+像素,像素为精简转译段)。
// ============================================================

// Registerless terrain heightmap global for the inc/terrain_heightmap.inc.hlsl
// terrain_height() helper (INC fix #3); unused here, dxc DCEs it.
Texture2D terrain_heightmap : register(t63);

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
#include "inc/terrain_heightmap.inc.hlsl"
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

uint2 bush_alpha_dims;
uint2 bush_color_dims;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 grass_object_push_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
Texture2D<float4> bush_alpha : register(t15);
Texture2D<float4> bush_color : register(t16);
Texture2D<float4> grass_object_push : register(t19);
StructuredBuffer<InstanceData> nani_instance_data : register(t20);

// 2D 晶格哈希(角点两坐标合成一个 uint)。
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

// 哈希高 23 位映射到 [0,1) 随机数。
float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] int bush_part_id: bush_part_id;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 经实例间接缓冲取实例数据,组装仿射变换。
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;

    // 2D 风噪声晶格:原点 xz 平移 0.1 倍 + 时间平移,按顶点高度加权弯曲。
    float vertex_h = vin.Vertex_Position.y * 0.5f;
    float2 wind_uv = (origin_xz * 0.1f) + float2(game_time_sec + game_time_frac, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
    float wind_noise = (n00 + n10) + (n01 + n11);
    // 弯曲量:顶点越高位移越大,正弦驱动往复摆动。
    world.x -= (vertex_h * vertex_h) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.0735f);

    // 草地物体挤压:采样 push 贴图,按顶点高度平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced = world;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * smoothstep(0.0f, 1.0f, vertex_h));
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = 0.5f.xxx;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = vin.bush_part_id;
    o.C5 = vin.Vertex_Position.y;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation int C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // alpha 测试:透明处丢弃。
    if (bush_alpha.SampleBias(sampler_lnc, pin.C3, global_mip_bias).w < 0.5f)
    {
        discard;
    }
    // 视线与法线夹角过大(看到的是广告牌侧面)时丢弃。
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    if (abs(dot(normalize(cam_pos - pin.C2), pin.C1)) > 0.5f)
    {
        discard;
    }
    // 采样灌木颜色,顶部越高越亮;glade 主题 4 时朝上表面混入浅色。
    float3 normal = normalize(pin.C1);
    float3 bush_color_sampled = bush_color.SampleBias(sampler_llr, (pin.C2.xz + (pin.C3 * 0.2f)) * 0.1f, 0.0f).xyz
        * lerp(0.8f, 1.0f, smoothstep(0.2f, 2.0f, pin.C5));
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(bush_color_sampled, 0.56f.xxx, smoothstep(0.2f, 1.0f, clamp(normal.y, 0.0f, 1.0f)) * 0.4f);
    }
    else
    {
        albedo = bush_color_sampled;
    }
    // 打包 G-Buffer:法线压到 [0,1],反照率 sqrt 到 sRGB 后 8bit 量化,带材质标志。
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 3003121664u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 1711276032u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_bush.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct BushXform
{
    Affine3Packed transform;
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
    uint2 bush_alpha_dims : packoffset(c0.y);
    uint2 bush_color_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 grass_object_push_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t20, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float3 in_var_Vertex_Normal;
static int in_var_bush_part_id;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] int in_var_bush_part_id: bush_part_id;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // UV 输入(转译段随后翻转 y)。
    float2 uv = in_var_Vertex_UV;
    // 经实例间接缓冲读出 12 个 float 的 packed 仿射变换(每实例 48 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    // 组装仿射变换,得到世界位置与原点 xz。
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    uv.y = 1.0f - uv.y;
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    // 顶点高度(用于弯曲权重)。
    float vertex_h = in_var_Vertex_Position.y * 0.5f;
    // 风噪声晶格坐标与四角哈希。
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
    // 双线性插值平滑因子与四角噪声 → 弯曲正弦偏移。
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float smooth_x = wind_smooth.x;
    float smooth_x_inv = 1.0f - smooth_x;
    float smooth_y = wind_smooth.y;
    world.x = world.x - (((vertex_h * vertex_h) * (sin((((((1.0f - smooth_y) * (((asfloat(((h00 ^ (h00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) + ((asfloat(((h10 ^ (h10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x))) + (((asfloat(((h01 ^ (h01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) * smooth_y)) + (((asfloat(((h11 ^ (h11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    // 草地物体挤压:采样 push 贴图并平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * smoothstep(0.0f, 1.0f, vertex_h));
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }
    else
    {
        displaced = world;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = 0.5f.xxx;
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C3 = uv;
    out_var_C4 = in_var_bush_part_id;
    out_var_C5 = in_var_Vertex_Position.y;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_bush_part_id = stage_input.in_var_bush_part_id;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_bush.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct BushXform
{
    Affine3Packed transform;
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
    uint2 bush_alpha_dims : packoffset(c0.y);
    uint2 bush_color_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 grass_object_push_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t20, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t19, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float2 in_var_Vertex_UV;
static float3 in_var_Vertex_Normal;
static int in_var_bush_part_id;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static int out_var_C4;
static float out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] int in_var_bush_part_id: bush_part_id;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation int out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // UV 输入(转译段随后翻转 y)。
    float2 uv = in_var_Vertex_UV;
    // 经实例间接缓冲读出 12 个 float 的 packed 仿射变换(每实例 48 字节)。
    Affine3Packed packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        packed.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    // 组装仿射变换,得到世界位置与原点 xz。
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]), float3(packed.data[3], packed.data[4], packed.data[5]), float3(packed.data[6], packed.data[7], packed.data[8]), float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(in_var_Vertex_Position, 1.0f));
    uv.y = 1.0f - uv.y;
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    // 顶点高度(用于弯曲权重)。
    float vertex_h = in_var_Vertex_Position.y * 0.5f;
    // 风噪声晶格坐标与四角哈希。
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
    // 双线性插值平滑因子与四角噪声 → 弯曲正弦偏移。
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float smooth_x = wind_smooth.x;
    float smooth_x_inv = 1.0f - smooth_x;
    float smooth_y = wind_smooth.y;
    world.x = world.x - (((vertex_h * vertex_h) * (sin((((((1.0f - smooth_y) * (((asfloat(((h00 ^ (h00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) + ((asfloat(((h10 ^ (h10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x))) + (((asfloat(((h01 ^ (h01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x_inv) * smooth_y)) + (((asfloat(((h11 ^ (h11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * smooth_x) * smooth_y)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    // 草地物体挤压:采样 push 贴图并平滑推开。
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin_xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float push_amount = clamp(grass_push.z, 0.0f, 1.0f);
    float3 displaced;
    if (push_amount > 0.0f)
    {
        float2 displaced_xz = world.xz + ((grass_push.xy * push_amount) * smoothstep(0.0f, 1.0f, vertex_h));
        displaced = float3(displaced_xz.x, world.y, displaced_xz.y);
    }
    else
    {
        displaced = world;
    }
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = 0.5f.xxx;
    out_var_C1 = mul(xform, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C3 = uv;
    out_var_C4 = in_var_bush_part_id;
    out_var_C5 = in_var_Vertex_Position.y;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_bush_part_id = stage_input.in_var_bush_part_id;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
// lifted from _nani_bush.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint2 bush_alpha_dims : packoffset(c0.y);
    uint2 bush_color_dims : packoffset(c1);
    uint2 terrain_heightmap_dims : packoffset(c1.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c2);
    uint2 grass_object_push_dims : packoffset(c2.z);
    int bool_render_shadows : packoffset(c3);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> bush_alpha : register(t15, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
};

void frag_main()
{
    // alpha 测试:透明处丢弃。
    if (bush_alpha.SampleBias(sampler_lnc, in_var_C3, frame_constants_global_mip_bias).w < 0.5f)
    {
        discard;
    }
    // 视线与法线夹角过大(广告牌侧面)时丢弃。
    float4 cam_vw = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(dot(normalize((cam_vw.xyz / cam_vw.w.xxx) - in_var_C2), in_var_C1)) > 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
