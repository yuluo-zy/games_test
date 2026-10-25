// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_roof_floor.raster.hlsl | B nani raster | bins: _nani_roof_floor.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani roof/floor: per-instance roof-tile transform; PS selects a wooden or
// stone floor material by instance type, applies height-parallax + normal map,
// cuts dormer holes via a spatial hash grid and packs the gbuffer.
// 【文件用途】Nani 屋顶/地板光栅化。VS 按实例把每个瓦片 quad 变换到世界空间,
//   以实例朝向翻转法线并用 512 格网把瓦片 y 固定到 1/512 高度台阶。PS 按
//   material_id 选石材或木地板材质(高度视差 + 法线贴图 + 种子/粗糙度变化),
//   用空间哈希格网裁掉天窗(dormer)洞,叠加灰泥/砖色,雪主题/高亮混合,打包
//   (法线, 反照率) uint2 + 深度写。
// 【入口】vs_main/ps_main | 绑定:t14 instance_indirection_buffer、t15 dirthpath_heightmap、
//   t16 nani_instance_data(64B)、t18 dormer_holes、t19 dormer_hole_count、t20 halftimber_color_ids、
//   t21 halftimber_colors、t22..t33 木地板(1..3 组 normal/height/seed/roughness)、t34 wall_color_ids、
//   t35 brick_colors、t37..t48 石地板(1..3 组)、sampler_llr、$Globals b13、frame_constants b0/space1。
// 【变体】b903+f0ad 的四个 (variant,stage) 组合均用提起段;手写段仅作参考基线。
// 重写说明:手写重建段补充中文注释;提起段(TGV_*)逐字保留,仅加块级中文注释。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "roof_material_id.inc.hlsl"
#include "roof/stone_floor.inc.hlsl"
#include "roof/wooden_floor.inc.hlsl"
#include "roof/dormer_hole.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/color_library.hlsl"
#include "inc/wall_color_customization.hlsl"
#include "inc/highlight_color.hlsl"
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

uint highlighted_wall_id;
uint highlight_mode;

struct InstanceData
{
    Affine3Packed xform;
    uint material_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
Texture2D<float4> dirthpath_heightmap : register(t15);
StructuredBuffer<InstanceData> nani_instance_data : register(t16);
ByteAddressBuffer dormer_holes : register(t18);
ByteAddressBuffer dormer_hole_count : register(t19);
ByteAddressBuffer halftimber_color_ids : register(t20);
Texture2DArray<float4> halftimber_colors : register(t21);
Texture2D<float4> wooden_floor_normal : register(t22);
Texture2D<float4> wooden_floor_height : register(t23);
Texture2D<float4> wooden_floor_seed : register(t24);
Texture2D<float4> wooden_floor_roughness : register(t25);
Texture2D<float4> wooden_floor_normal_2 : register(t26);
Texture2D<float4> wooden_floor_height_2 : register(t27);
Texture2D<float4> wooden_floor_seed_2 : register(t28);
Texture2D<float4> wooden_floor_roughness_2 : register(t29);
Texture2D<float4> wooden_floor_normal_3 : register(t30);
Texture2D<float4> wooden_floor_height_3 : register(t31);
Texture2D<float4> wooden_floor_seed_3 : register(t32);
Texture2D<float4> wooden_floor_roughness_3 : register(t33);
ByteAddressBuffer wall_color_ids : register(t34);
Texture2DArray<float4> brick_colors : register(t35);
Texture2D<float4> stone_floor_normal : register(t37);
Texture2D<float4> stone_floor_seed : register(t38);
Texture2D<float4> stone_floor_height : register(t39);
Texture2D<float4> stone_floor_roughness : register(t40);
Texture2D<float4> stone_floor_seed_2 : register(t41);
Texture2D<float4> stone_floor_normal_2 : register(t42);
Texture2D<float4> stone_floor_height_2 : register(t43);
Texture2D<float4> stone_floor_roughness_2 : register(t44);
Texture2D<float4> stone_floor_seed_3 : register(t45);
Texture2D<float4> stone_floor_normal_3 : register(t46);
Texture2D<float4> stone_floor_height_3 : register(t47);
Texture2D<float4> stone_floor_roughness_3 : register(t48);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float4 position : SV_Position;
};

// VS:经 indirection 取实例 3x4 仿射变换,把瓦片顶点转到世界并投影;向上轴与实例朝向
VsOut vs_main(VsIn vin)
{
    VsOut o;
// 读取 3x4 仿射变换(平移/旋转/缩放打包在 Affine3Packed)
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
// 输出:世界空间法线(C1)、世界位置(C2)、实例索引(C4)
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// PS:按材质选木/石地板 -> 高度视差 + 法线 -> 天窗洞剔除 -> 灰泥/砖/雪/高亮 -> 打包
PsOut ps_main(PsIn pin)
{
    PsOut o;
    InstanceData inst = nani_instance_data[pin.C4];
// 地板 uv = 世界 xz * 0.2,法线取 C1
    float2 floor_uv = pin.C2.xz * 0.2f;
    float3 normal = normalize(pin.C1);

    float3 albedo;
    float3 normal_map;
    float seed_val;
// material_id <3 为石材,否则为木地板;variant 选 3 组贴图之一
    uint variant = inst.material_id & 3u;
    if (inst.material_id < 3u)
    {
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (variant == 0u)
        {
            height_lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (variant == 1u)
        {
            height_lod = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = stone_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = stone_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = stone_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.8f.xxx, 0.9f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    else
    {
        uint wv = variant;
        float height_lod;
        float rough;
        float3 nm;
        float sd;
        if (wv == 0u)
        {
            height_lod = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else if (wv == 1u)
        {
            height_lod = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_2.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_2.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        else
        {
            height_lod = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, floor_uv) + global_mip_bias;
            rough = wooden_floor_roughness_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
            nm = wooden_floor_normal_3.SampleBias(sampler_llr, floor_uv, height_lod).xyz * 2.0f - 1.0f.xxx;
            sd = wooden_floor_seed_3.SampleBias(sampler_llr, floor_uv, height_lod).x;
        }
        albedo = lerp(0.6f.xxx, 0.8f.xxx, rough);
        normal_map = nm;
        seed_val = sd;
        albedo *= 0.9f + seed_val * 0.2f;
    }
    normal = normalize(normal + (normal_map * 0.4f));
    uint wall_id = nani_instance_data[pin.C4].material_id & 0xFFFFu;
    int timber_color_id = int(halftimber_color_ids.Load<uint>(wall_id * 4 + 0));
    float3 timber = halftimber_colors.SampleBias(sampler_llr, float3(floor_uv, float(timber_color_id)), global_mip_bias).xyz;
    albedo = lerp(albedo, timber, 0.3f);
    float3 brick = brick_colors.SampleBias(sampler_llr, float3(floor_uv, float(wall_color_ids.Load<uint>(wall_id * 4 + 0))), global_mip_bias).xyz;
    albedo = lerp(albedo, brick, 0.1f);
// 空间哈希格网:8x8 格索引天窗列表,命中同 wall_id 的洞则 discard
    uint dormer_cell = uint(int(floor(floor_uv.x * 8.0f)) + 15 * int(floor(floor_uv.y * 8.0f)));
    uint dormer_count = min(128u, dormer_hole_count.Load<uint>(dormer_cell * 4 + 0));
    for (uint i = 0u; i < dormer_count; i++)
    {
        uint hole_idx = uint(int(uint(dormer_cell * 50) + i));
        if (dormer_holes.Load<uint>(hole_idx * 48 + 12) == wall_id)
        {
            discard;
        }
    }
// 雪主题:按路径高度图把地板向白淡化
    if (glade_theme_id == 4u)
    {
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (path * smoothstep(-0.1f, 0.0f, pin.C2.y) * clamp(normal.y, 0.0f, 1.0f)).xxx);
    }
// 高亮模式:目标墙的屋顶/地板染橙红
    if (highlight_mode == 1u && wall_id == highlighted_wall_id)
    {
        albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
    }
// 打包:法线 RGB8 + 0xCC 标志;反照率 sqrt 反 gamma + 0x07000000 标志
    float3 packed_normal = (normal * 0.5f) + 0.5f.xxx;
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
// lifted from _nani_roof_floor.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS:瓦片变换 + 512 格网台阶;
//   PS:木/石材质(高度视差/法线)+ 天窗剔除 + G-Buffer 打包。
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

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
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
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static uint out_var_C6;
static uint out_var_C7;
static uint out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

// VS 主函数:按实例朝向翻转法线,顶点 xz 乘缩放后平移到世界,y 固定 1/512 台阶
void vert_main()
{
    float3 v141 = float3(nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16), 0.0f).xzy;
    float3 v143 = float3(-nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y, 0.0f, nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x);
    float4 v148 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    bool v157 = (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) != 0u) ? true : ((nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0) - (v148.xyz / v148.w.xxx)).y > 0.0f);
    float3 v161;
    if (v157)
    {
        v161 = v141 * (-1.0f);
    }
    else
    {
        v161 = v141;
    }
    bool3 v162 = v157.xxx;
    float3 v163 = float3(v162.x ? float3(-0.0f, -1.0f, -0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v162.y ? float3(-0.0f, -1.0f, -0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v162.z ? float3(-0.0f, -1.0f, -0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
    float2 v165 = in_var_Vertex_Position.xz * nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24);
    float3 v168 = mul(v165, float2x3(v161, v143)) + nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    float v176 = 0.001953125f * (floor(v168.y * 512.0f) + frac(float(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12)) * 1.61803400516510009765625f));
    float3 v177 = v168;
    v177.y = v176;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v168.x, v176, v168.z, 1.0f));
    out_var_C1 = v163;
    out_var_C2 = v177;
    out_var_C3 = v165;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
    out_var_C7 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C8 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56);
    out_var_C9 = transpose(float3x3(v161, v143, v163));
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
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
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_roof_floor.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS:瓦片变换 + 512 格网台阶;
//   PS:木/石材质(高度视差/法线)+ 天窗剔除 + G-Buffer 打包。
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

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

struct DormerHole
{
    float3 position;
    int wall_id;
    float3 size;
    int pad0;
    float2 normal;
    float k;
    float a;
};

static bool v225;

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
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer dormer_holes : register(t18, space0);
ByteAddressBuffer dormer_hole_count : register(t19, space0);
ByteAddressBuffer halftimber_color_ids : register(t20, space0);
ByteAddressBuffer wall_color_ids : register(t34, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> dirthpath_heightmap : register(t15, space0);
Texture2DArray<float4> halftimber_colors : register(t21, space0);
Texture2D<float4> wooden_floor_normal : register(t22, space0);
Texture2D<float4> wooden_floor_height : register(t23, space0);
Texture2D<float4> wooden_floor_seed : register(t24, space0);
Texture2D<float4> wooden_floor_roughness : register(t25, space0);
Texture2D<float4> wooden_floor_normal_2 : register(t26, space0);
Texture2D<float4> wooden_floor_height_2 : register(t27, space0);
Texture2D<float4> wooden_floor_seed_2 : register(t28, space0);
Texture2D<float4> wooden_floor_roughness_2 : register(t29, space0);
Texture2D<float4> wooden_floor_normal_3 : register(t30, space0);
Texture2D<float4> wooden_floor_height_3 : register(t31, space0);
Texture2D<float4> wooden_floor_seed_3 : register(t32, space0);
Texture2D<float4> wooden_floor_roughness_3 : register(t33, space0);
Texture2DArray<float4> brick_colors : register(t35, space0);
Texture2D<float4> stone_floor_normal : register(t37, space0);
Texture2D<float4> stone_floor_seed : register(t38, space0);
Texture2D<float4> stone_floor_height : register(t39, space0);
Texture2D<float4> stone_floor_roughness : register(t40, space0);
Texture2D<float4> stone_floor_seed_2 : register(t41, space0);
Texture2D<float4> stone_floor_normal_2 : register(t42, space0);
Texture2D<float4> stone_floor_height_2 : register(t43, space0);
Texture2D<float4> stone_floor_roughness_2 : register(t44, space0);
Texture2D<float4> stone_floor_seed_3 : register(t45, space0);
Texture2D<float4> stone_floor_normal_3 : register(t46, space0);
Texture2D<float4> stone_floor_height_3 : register(t47, space0);
Texture2D<float4> stone_floor_roughness_3 : register(t48, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C6;
static uint in_var_C7;
static uint in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

// PS 主函数:高亮判定 -> 天窗剔除 -> 木/石材质采样/视差 -> 主题/高亮混合 -> 打包
void frag_main()
{
// 高亮判定:mode1 高亮目标墙(material_id 低 16 位=wall_id),mode2 变灰
    bool v266;
    if (1u == highlight_mode)
    {
        v266 = nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) == highlighted_wall_id;
    }
    else
    {
        v266 = false;
    }
    bool v273;
    if (2u == highlight_mode)
    {
        v273 = nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) == highlighted_wall_id;
    }
    else
    {
        v273 = false;
    }
// 天窗剔除:8x8 格网空间哈希,命中同 wall_id 的 dormer_holes 即 discard
    float2 v277;
    int v274 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    bool v372;
    do
    {
        v277 = in_var_C2.xz;
        float2 v279 = (v277 + 65.0f.xx) * 0.1153846085071563720703125f.xx;
        int v287 = int(floor(v279.x)) + (15 * int(floor(v279.y)));
        uint v291 = min(128u, dormer_hole_count.Load<uint>(uint(v287) * 4 + 0));
        bool v369;
        bool v370;
        uint v293 = 0u;
        for (;;)
        {
            if (v293 < v291)
            {
                uint v303 = uint(int(uint(v287 * 50) + v293));
                if (v274 == dormer_holes.Load<int>(v303 * 48 + 12))
                {
                    float3 v319 = float3(dormer_holes.Load<float>(v303 * 48 + 32), 0.0f, dormer_holes.Load<float>(v303 * 48 + 36));
                    float3x3 v323 = transpose(float3x3(cross(v319, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), v319));
                    float3 v324 = mul(v323, float3(1.0f, 0.0f, 0.0f));
                    float3 v325 = mul(v323, float3(0.0f, 1.0f, 0.0f));
                    float3 v326 = mul(v323, float3(0.0f, 0.0f, 1.0f));
                    float3 v329 = cross(v324, v325);
                    float3 v332 = (1.0f / dot(v326, v329)).xxx;
                    float3 v340 = mul(in_var_C2 - dormer_holes.Load<float3>(v303 * 48 + 0), float3x3(cross(v325, v326) * v332, cross(v326, v324) * v332, v329 * v332)) / dormer_holes.Load<float3>(v303 * 48 + 16);
                    float v341 = v340.y;
                    float v344 = clamp((v341 * dormer_holes.Load<float>(v303 * 48 + 40)) + dormer_holes.Load<float>(v303 * 48 + 44), 0.0f, 1.0f);
                    bool v360;
                    if (abs(v340.x) < lerp(0.5f, 0.0f, (v344 * v344) * v344))
                    {
                        bool v359;
                        if (v341 < ((dormer_holes.Load<float>(v303 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                        {
                            v359 = v341 > (-0.5f);
                        }
                        else
                        {
                            v359 = false;
                        }
                        v360 = v359;
                    }
                    else
                    {
                        v360 = false;
                    }
                    bool v366;
                    if (v360)
                    {
                        v366 = abs(v340.z) < 0.5f;
                    }
                    else
                    {
                        v366 = false;
                    }
                    if (v366)
                    {
                        v369 = true;
                        v370 = true;
                        break;
                    }
                }
                v293++;
                continue;
            }
            else
            {
                v369 = v225;
                v370 = false;
                break;
            }
        }
        if (v370)
        {
            v372 = v369;
            break;
        }
        v372 = false;
        break;
    } while(false);
    if (v372)
    {
        discard;
    }
    int v376 = int(in_var_C6);
    bool v382;
    if (!(v376 == 0))
    {
        v382 = v376 == 2;
    }
    else
    {
        v382 = true;
    }
// 材质输出累积:反照率/粗糙度/深度/法线;按 C6 材质类型分流木/石
    float3 v2781;
    float v2782;
    float v2783;
    float v2784;
    float3 v2785;
    if (v382)
    {
        bool v1521;
        if (!(v376 == 2))
        {
            v1521 = v376 == 3;
        }
        else
        {
            v1521 = true;
        }
        int v1525;
        if (v1521)
        {
            v1525 = int(in_var_C7);
        }
        else
        {
            v1525 = int(wall_color_ids.Load<uint>(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) * 4 + 0));
        }
        float2 v1526 = in_var_C3 * 0.20000000298023223876953125f;
        float v2771;
        float v2772;
        float v2773;
        float3 v2774;
        float3 v2775;
        float v2776;
        if (in_var_C8 == 0u)
        {
            float3 v2449;
            uint2 v2365 = uint2(gl_FragCoord.yx);
            float v2372 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
            float v2382 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v2365.x) + v2372)) + (0.005837149918079376220703125f * (float(v2365.y) + v2372))));
            float _2385_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, v1526);
            float2 v2385 = _2385_tmp.xx;
            float v2389 = v2385.x + frame_constants_global_mip_bias;
            float3 v2390 = ddx(in_var_C2);
            float3 v2391 = ddy(in_var_C2);
            float2 v2393 = ddx(v1526);
            float2 v2394 = ddy(v1526);
            float2 v2395 = -v2394;
            float3 v2396 = cross(in_var_C1, -v2391);
            float3 v2397 = cross(v2390, in_var_C1);
            float v2398 = v2393.x;
            float v2399 = v2395.y;
            float v2401 = v2393.y;
            float v2402 = v2395.x;
            float v2405 = 1.0f / ((v2398 * v2399) - (v2401 * v2402));
            float3x3 v2415 = transpose(float3x3(((v2396 * v2398) + (v2397 * v2402)) * v2405, ((v2396 * v2401) + (v2397 * v2399)) * v2405, in_var_C1));
            float4 v2420 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
            float3 v2426 = mul(v2415, float3(1.0f, 0.0f, 0.0f));
            float3 v2427 = mul(v2415, float3(0.0f, 1.0f, 0.0f));
            float3 v2428 = mul(v2415, float3(0.0f, 0.0f, 1.0f));
            float3 v2431 = cross(v2426, v2427);
            float3 v2434 = (1.0f / dot(v2428, v2431)).xxx;
            float3x3 v2438 = float3x3(cross(v2427, v2428) * v2434, cross(v2428, v2426) * v2434, v2431 * v2434);
            float3 v2439 = mul(v2438, in_var_C2 - (v2420.xyz / v2420.w.xxx));
            float3 v2445 = float3(v1526, 0.0f);
            float v2558;
            do
            {
                v2449 = (v2445 + (v2439 * ((-0.07500000298023223876953125f) / v2439.z))) - v2445;
                float v2451 = 1.0f / max(1.0f, 2.0f);
                float v2454 = lerp(0.0f, 1.0f, pow(v2382 * v2451, 2.0f));
                float3 v2456 = v2445 + (v2449 * v2454);
                float v2464 = v2456.z - ((stone_floor_height.SampleLevel(sampler_llr, v2456.xy, v2389).x - 1.0f) * 0.07500000298023223876953125f);
                bool v2466 = (v2464 < 0.0f) ? true : false;
                float v2503;
                float v2504;
                float v2505;
                bool v2506;
                float v2507;
                if (v2466)
                {
                    v2503 = v2464;
                    v2504 = 0.0f;
                    v2505 = v2454;
                    v2506 = v2466;
                    v2507 = 0.0f;
                }
                else
                {
                    float v2474;
                    float v2478;
                    v2474 = v2464;
                    v2478 = v2454;
                    float v2475;
                    bool v2477;
                    float v2479;
                    float v2500;
                    float v2501;
                    bool v2502;
                    uint v2471 = 1u;
                    bool v2476 = v2466;
                    for (;;)
                    {
                        if (v2471 < 3u)
                        {
                            v2479 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v2471) + v2382) * v2451, 2.0f)));
                            float3 v2489 = v2445 + (v2449 * v2479);
                            v2475 = v2489.z - ((stone_floor_height.SampleLevel(sampler_llr, v2489.xy, v2389).x - 1.0f) * 0.07500000298023223876953125f);
                            v2477 = (v2475 < 0.0f) ? true : false;
                            if (v2477)
                            {
                                v2500 = v2475;
                                v2501 = v2479;
                                v2502 = v2477;
                                break;
                            }
                            v2471++;
                            v2474 = v2475;
                            v2476 = v2477;
                            v2478 = v2479;
                            continue;
                        }
                        else
                        {
                            v2500 = 0.0f;
                            v2501 = 1.0f;
                            v2502 = v2476;
                            break;
                        }
                    }
                    v2503 = v2500;
                    v2504 = v2474;
                    v2505 = v2501;
                    v2506 = v2502;
                    v2507 = v2478;
                }
                if (v2506)
                {
                    float v2512;
                    float v2515;
                    float v2517;
                    float v2519;
                    v2512 = v2507;
                    v2515 = v2503;
                    v2517 = v2504;
                    v2519 = v2505;
                    for (uint v2521 = 0u; v2521 < 4u; )
                    {
                        float v2525 = v2512 + v2519;
                        float v2526 = v2525 * 0.5f;
                        float3 v2528 = v2445 + (v2449 * v2526);
                        float v2536 = v2528.z - ((stone_floor_height.SampleLevel(sampler_llr, v2528.xy, v2389).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v2538 = (v2536 < 0.0f) ? true : false;
                        v2512 = v2538 ? v2512 : v2526;
                        v2515 = v2538 ? v2536 : v2515;
                        v2517 = v2538 ? v2517 : v2536;
                        v2519 = v2538 ? v2526 : v2519;
                        v2521++;
                        continue;
                    }
                    float v2542 = lerp(v2512, v2519, v2517 / (v2517 + (-v2515)));
                    float3 v2544 = v2445 + (v2449 * v2542);
                    v2558 = ((abs(v2544.z - ((stone_floor_height.SampleLevel(sampler_llr, v2544.xy, v2389).x - 1.0f) * 0.07500000298023223876953125f)) < (v2517 * 0.89999997615814208984375f)) ? true : false) ? v2542 : v2519;
                    break;
                }
                else
                {
                    v2558 = v2507;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            float3 v2559 = v2449 * v2558;
            float2 v2561 = v1526 + v2559.xy;
            float4 v2573 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v2415, v2559), 1.0f)));
            float3 v2582 = (stone_floor_normal.SampleLevel(sampler_llr, v2561, v2389).xyz * 2.0f) - 1.0f.xxx;
            v2582.z = v2582.z * 0.13333332538604736328125f;
            float4 v2590 = stone_floor_seed.SampleBias(sampler_llr, v2561, frame_constants_global_mip_bias);
            float v2591 = v2590.z;
            float v2592 = v2590.x;
            int v2601 = int(trunc((v2592 * 32.0f) - 0.5f) + trunc((v2590.y * 32.0f) - 0.5f));
            uint v2603 = uint(v2601 ^ 7993234);
            uint v2605 = v2603 + (v2603 << 10u);
            uint v2607 = v2605 ^ (v2605 >> 6u);
            uint v2609 = v2607 + (v2607 << 3u);
            uint v2611 = v2609 ^ (v2609 >> 11u);
            uint v2620 = uint(v2601 ^ 9730541);
            uint v2622 = v2620 + (v2620 << 10u);
            uint v2624 = v2622 ^ (v2622 >> 6u);
            uint v2626 = v2624 + (v2624 << 3u);
            uint v2628 = v2626 ^ (v2626 >> 11u);
            float v2634 = asfloat(((v2628 + (v2628 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
            float4 v2643 = stone_floor_height.SampleBias(sampler_llr, v2561, frame_constants_global_mip_bias);
            float4 v2646 = stone_floor_roughness.SampleBias(sampler_llr, v2561, frame_constants_global_mip_bias);
            float v2647 = v2646.x;
            float2 v2650 = (((v277 * 0.100000001490116119384765625f) + float2((asfloat(((v2611 + (v2611 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, v2634 * 0.300000011920928955078125f)) + (0.5f * v2591).xx) + (v2647 * 0.100000001490116119384765625f).xx;
            bool v2656;
            if (!(v1525 == 0))
            {
                v2656 = v1525 == 4;
            }
            else
            {
                v2656 = true;
            }
            bool v2661;
            if (!v2656)
            {
                v2661 = v1525 == 10;
            }
            else
            {
                v2661 = true;
            }
            bool v2666;
            if (!v2661)
            {
                v2666 = v1525 == 12;
            }
            else
            {
                v2666 = true;
            }
            float3 v2758;
            if (v2666)
            {
                v2758 = brick_colors.Sample(sampler_llr, float3(v2650, 0.0f)).xyz * 0.89999997615814208984375f;
            }
            else
            {
                bool v2675;
                if (!(v1525 == 1))
                {
                    v2675 = v1525 == 5;
                }
                else
                {
                    v2675 = true;
                }
                float3 v2749;
                if (v2675)
                {
                    v2749 = brick_colors.Sample(sampler_llr, float3(v2650, 1.0f)).xyz;
                }
                else
                {
                    bool v2684;
                    if (!(v1525 == 2))
                    {
                        v2684 = v1525 == 7;
                    }
                    else
                    {
                        v2684 = true;
                    }
                    float3 v2741;
                    if (v2684)
                    {
                        v2741 = (brick_colors.Sample(sampler_llr, float3(v2650, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, v2591);
                    }
                    else
                    {
                        bool v2693;
                        if (!(v1525 == 3))
                        {
                            v2693 = v1525 == 6;
                        }
                        else
                        {
                            v2693 = true;
                        }
                        bool v2698;
                        if (!v2693)
                        {
                            v2698 = v1525 == 8;
                        }
                        else
                        {
                            v2698 = true;
                        }
                        bool v2703;
                        if (!v2698)
                        {
                            v2703 = v1525 == 9;
                        }
                        else
                        {
                            v2703 = true;
                        }
                        float3 v2730;
                        if (v2703)
                        {
                            v2730 = (brick_colors.Sample(sampler_llr, float3(v2650, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, v2591);
                        }
                        else
                        {
                            float3 v2719;
                            if (v1525 == 11)
                            {
                                v2719 = brick_colors.Sample(sampler_llr, float3(v2650, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, v2591);
                            }
                            else
                            {
                                v2719 = 0.0f.xxx;
                            }
                            v2730 = v2719;
                        }
                        v2741 = v2730;
                    }
                    v2749 = v2741;
                }
                v2758 = v2749;
            }
            v2771 = smoothstep(0.20000000298023223876953125f, 0.75f, 1.0f - v2643.x);
            v2772 = (v2573.xyz / v2573.w.xxx).z;
            v2773 = lerp(0.75f, 1.0f, v2647) * lerp(0.800000011920928955078125f, 1.0f, v2634);
            v2774 = normalize(mul(v2582, v2438));
            v2775 = (v2758 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height.SampleBias(sampler_llr, v2561, frame_constants_global_mip_bias).x);
            v2776 = lerp(0.0f, 0.96875f, v2592);
        }
        else
        {
            float v2354;
            float v2355;
            float v2356;
            float3 v2357;
            float3 v2358;
            float v2359;
            if (in_var_C8 == 1u)
            {
                float3 v2032;
                uint2 v1948 = uint2(gl_FragCoord.yx);
                float v1955 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v1965 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v1948.x) + v1955)) + (0.005837149918079376220703125f * (float(v1948.y) + v1955))));
                float _1968_tmp = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, v1526);
                float2 v1968 = _1968_tmp.xx;
                float v1972 = v1968.x + frame_constants_global_mip_bias;
                float3 v1973 = ddx(in_var_C2);
                float3 v1974 = ddy(in_var_C2);
                float2 v1976 = ddx(v1526);
                float2 v1977 = ddy(v1526);
                float2 v1978 = -v1977;
                float3 v1979 = cross(in_var_C1, -v1974);
                float3 v1980 = cross(v1973, in_var_C1);
                float v1981 = v1976.x;
                float v1982 = v1978.y;
                float v1984 = v1976.y;
                float v1985 = v1978.x;
                float v1988 = 1.0f / ((v1981 * v1982) - (v1984 * v1985));
                float3x3 v1998 = transpose(float3x3(((v1979 * v1981) + (v1980 * v1985)) * v1988, ((v1979 * v1984) + (v1980 * v1982)) * v1988, in_var_C1));
                float4 v2003 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v2009 = mul(v1998, float3(1.0f, 0.0f, 0.0f));
                float3 v2010 = mul(v1998, float3(0.0f, 1.0f, 0.0f));
                float3 v2011 = mul(v1998, float3(0.0f, 0.0f, 1.0f));
                float3 v2014 = cross(v2009, v2010);
                float3 v2017 = (1.0f / dot(v2011, v2014)).xxx;
                float3x3 v2021 = float3x3(cross(v2010, v2011) * v2017, cross(v2011, v2009) * v2017, v2014 * v2017);
                float3 v2022 = mul(v2021, in_var_C2 - (v2003.xyz / v2003.w.xxx));
                float3 v2028 = float3(v1526, 0.0f);
                float v2141;
                do
                {
                    v2032 = (v2028 + (v2022 * ((-0.0500000007450580596923828125f) / v2022.z))) - v2028;
                    float v2034 = 1.0f / max(1.0f, 2.0f);
                    float v2037 = lerp(0.0f, 1.0f, pow(v1965 * v2034, 2.0f));
                    float3 v2039 = v2028 + (v2032 * v2037);
                    float v2047 = v2039.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v2039.xy, v1972).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool v2049 = (v2047 < 0.0f) ? true : false;
                    float v2086;
                    float v2087;
                    float v2088;
                    bool v2089;
                    float v2090;
                    if (v2049)
                    {
                        v2086 = v2047;
                        v2087 = 0.0f;
                        v2088 = v2037;
                        v2089 = v2049;
                        v2090 = 0.0f;
                    }
                    else
                    {
                        float v2057;
                        float v2061;
                        v2057 = v2047;
                        v2061 = v2037;
                        float v2058;
                        bool v2060;
                        float v2062;
                        float v2083;
                        float v2084;
                        bool v2085;
                        uint v2054 = 1u;
                        bool v2059 = v2049;
                        for (;;)
                        {
                            if (v2054 < 3u)
                            {
                                v2062 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v2054) + v1965) * v2034, 2.0f)));
                                float3 v2072 = v2028 + (v2032 * v2062);
                                v2058 = v2072.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v2072.xy, v1972).x - 1.0f) * 0.0500000007450580596923828125f);
                                v2060 = (v2058 < 0.0f) ? true : false;
                                if (v2060)
                                {
                                    v2083 = v2058;
                                    v2084 = v2062;
                                    v2085 = v2060;
                                    break;
                                }
                                v2054++;
                                v2057 = v2058;
                                v2059 = v2060;
                                v2061 = v2062;
                                continue;
                            }
                            else
                            {
                                v2083 = 0.0f;
                                v2084 = 1.0f;
                                v2085 = v2059;
                                break;
                            }
                        }
                        v2086 = v2083;
                        v2087 = v2057;
                        v2088 = v2084;
                        v2089 = v2085;
                        v2090 = v2061;
                    }
                    if (v2089)
                    {
                        float v2095;
                        float v2098;
                        float v2100;
                        float v2102;
                        v2095 = v2090;
                        v2098 = v2086;
                        v2100 = v2087;
                        v2102 = v2088;
                        for (uint v2104 = 0u; v2104 < 4u; )
                        {
                            float v2108 = v2095 + v2102;
                            float v2109 = v2108 * 0.5f;
                            float3 v2111 = v2028 + (v2032 * v2109);
                            float v2119 = v2111.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v2111.xy, v1972).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool v2121 = (v2119 < 0.0f) ? true : false;
                            v2095 = v2121 ? v2095 : v2109;
                            v2098 = v2121 ? v2119 : v2098;
                            v2100 = v2121 ? v2100 : v2119;
                            v2102 = v2121 ? v2109 : v2102;
                            v2104++;
                            continue;
                        }
                        float v2125 = lerp(v2095, v2102, v2100 / (v2100 + (-v2098)));
                        float3 v2127 = v2028 + (v2032 * v2125);
                        v2141 = ((abs(v2127.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v2127.xy, v1972).x - 1.0f) * 0.0500000007450580596923828125f)) < (v2100 * 0.89999997615814208984375f)) ? true : false) ? v2125 : v2102;
                        break;
                    }
                    else
                    {
                        v2141 = v2090;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                float3 v2142 = v2032 * v2141;
                float2 v2144 = v1526 + v2142.xy;
                float4 v2156 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v1998, v2142), 1.0f)));
                float3 v2165 = (stone_floor_normal_2.SampleLevel(sampler_llr, v2144, v1972).xyz * 2.0f) - 1.0f.xxx;
                v2165.z = v2165.z * 0.199999988079071044921875f;
                float4 v2173 = stone_floor_seed_2.SampleBias(sampler_llr, v2144, frame_constants_global_mip_bias);
                float v2174 = v2173.z;
                float v2175 = v2173.x;
                int v2184 = int(trunc((v2175 * 32.0f) - 0.5f) + trunc((v2173.y * 32.0f) - 0.5f));
                uint v2186 = uint(v2184 ^ 7993234);
                uint v2188 = v2186 + (v2186 << 10u);
                uint v2190 = v2188 ^ (v2188 >> 6u);
                uint v2192 = v2190 + (v2190 << 3u);
                uint v2194 = v2192 ^ (v2192 >> 11u);
                uint v2203 = uint(v2184 ^ 9730541);
                uint v2205 = v2203 + (v2203 << 10u);
                uint v2207 = v2205 ^ (v2205 >> 6u);
                uint v2209 = v2207 + (v2207 << 3u);
                uint v2211 = v2209 ^ (v2209 >> 11u);
                float v2217 = asfloat(((v2211 + (v2211 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
                float4 v2226 = stone_floor_height_2.SampleBias(sampler_llr, v2144, frame_constants_global_mip_bias);
                float4 v2229 = stone_floor_roughness_2.SampleBias(sampler_llr, v2144, frame_constants_global_mip_bias);
                float v2230 = v2229.x;
                float2 v2233 = (((v277 * 0.100000001490116119384765625f) + float2((asfloat(((v2194 + (v2194 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, v2217 * 0.300000011920928955078125f)) + (0.5f * v2174).xx) + (v2230 * 0.100000001490116119384765625f).xx;
                bool v2239;
                if (!(v1525 == 0))
                {
                    v2239 = v1525 == 4;
                }
                else
                {
                    v2239 = true;
                }
                bool v2244;
                if (!v2239)
                {
                    v2244 = v1525 == 10;
                }
                else
                {
                    v2244 = true;
                }
                bool v2249;
                if (!v2244)
                {
                    v2249 = v1525 == 12;
                }
                else
                {
                    v2249 = true;
                }
                float3 v2341;
                if (v2249)
                {
                    v2341 = brick_colors.Sample(sampler_llr, float3(v2233, 0.0f)).xyz * 0.89999997615814208984375f;
                }
                else
                {
                    bool v2258;
                    if (!(v1525 == 1))
                    {
                        v2258 = v1525 == 5;
                    }
                    else
                    {
                        v2258 = true;
                    }
                    float3 v2332;
                    if (v2258)
                    {
                        v2332 = brick_colors.Sample(sampler_llr, float3(v2233, 1.0f)).xyz;
                    }
                    else
                    {
                        bool v2267;
                        if (!(v1525 == 2))
                        {
                            v2267 = v1525 == 7;
                        }
                        else
                        {
                            v2267 = true;
                        }
                        float3 v2324;
                        if (v2267)
                        {
                            v2324 = (brick_colors.Sample(sampler_llr, float3(v2233, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, v2174);
                        }
                        else
                        {
                            bool v2276;
                            if (!(v1525 == 3))
                            {
                                v2276 = v1525 == 6;
                            }
                            else
                            {
                                v2276 = true;
                            }
                            bool v2281;
                            if (!v2276)
                            {
                                v2281 = v1525 == 8;
                            }
                            else
                            {
                                v2281 = true;
                            }
                            bool v2286;
                            if (!v2281)
                            {
                                v2286 = v1525 == 9;
                            }
                            else
                            {
                                v2286 = true;
                            }
                            float3 v2313;
                            if (v2286)
                            {
                                v2313 = (brick_colors.Sample(sampler_llr, float3(v2233, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, v2174);
                            }
                            else
                            {
                                float3 v2302;
                                if (v1525 == 11)
                                {
                                    v2302 = brick_colors.Sample(sampler_llr, float3(v2233, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, v2174);
                                }
                                else
                                {
                                    v2302 = 0.0f.xxx;
                                }
                                v2313 = v2302;
                            }
                            v2324 = v2313;
                        }
                        v2332 = v2324;
                    }
                    v2341 = v2332;
                }
                v2354 = smoothstep(0.300000011920928955078125f, 0.5f, 1.0f - v2226.x);
                v2355 = (v2156.xyz / v2156.w.xxx).z;
                v2356 = lerp(0.75f, 1.0f, v2230) * lerp(0.800000011920928955078125f, 1.0f, v2217);
                v2357 = normalize(mul(v2165, v2021));
                v2358 = (v2341 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_2.SampleBias(sampler_llr, v2144, frame_constants_global_mip_bias).x);
                v2359 = lerp(0.0f, 0.96875f, v2175);
            }
            else
            {
                float3 v1624;
                uint2 v1540 = uint2(gl_FragCoord.yx);
                float v1547 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v1557 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v1540.x) + v1547)) + (0.005837149918079376220703125f * (float(v1540.y) + v1547))));
                float _1560_tmp = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, v1526);
                float2 v1560 = _1560_tmp.xx;
                float v1564 = v1560.x + frame_constants_global_mip_bias;
                float3 v1565 = ddx(in_var_C2);
                float3 v1566 = ddy(in_var_C2);
                float2 v1568 = ddx(v1526);
                float2 v1569 = ddy(v1526);
                float2 v1570 = -v1569;
                float3 v1571 = cross(in_var_C1, -v1566);
                float3 v1572 = cross(v1565, in_var_C1);
                float v1573 = v1568.x;
                float v1574 = v1570.y;
                float v1576 = v1568.y;
                float v1577 = v1570.x;
                float v1580 = 1.0f / ((v1573 * v1574) - (v1576 * v1577));
                float3x3 v1590 = transpose(float3x3(((v1571 * v1573) + (v1572 * v1577)) * v1580, ((v1571 * v1576) + (v1572 * v1574)) * v1580, in_var_C1));
                float4 v1595 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v1601 = mul(v1590, float3(1.0f, 0.0f, 0.0f));
                float3 v1602 = mul(v1590, float3(0.0f, 1.0f, 0.0f));
                float3 v1603 = mul(v1590, float3(0.0f, 0.0f, 1.0f));
                float3 v1606 = cross(v1601, v1602);
                float3 v1609 = (1.0f / dot(v1603, v1606)).xxx;
                float3x3 v1613 = float3x3(cross(v1602, v1603) * v1609, cross(v1603, v1601) * v1609, v1606 * v1609);
                float3 v1614 = mul(v1613, in_var_C2 - (v1595.xyz / v1595.w.xxx));
                float3 v1620 = float3(v1526, 0.0f);
                float v1733;
                do
                {
                    v1624 = (v1620 + (v1614 * ((-0.0500000007450580596923828125f) / v1614.z))) - v1620;
                    float v1626 = 1.0f / max(1.0f, 2.0f);
                    float v1629 = lerp(0.0f, 1.0f, pow(v1557 * v1626, 2.0f));
                    float3 v1631 = v1620 + (v1624 * v1629);
                    float v1639 = v1631.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1631.xy, v1564).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool v1641 = (v1639 < 0.0f) ? true : false;
                    float v1678;
                    float v1679;
                    float v1680;
                    bool v1681;
                    float v1682;
                    if (v1641)
                    {
                        v1678 = v1639;
                        v1679 = 0.0f;
                        v1680 = v1629;
                        v1681 = v1641;
                        v1682 = 0.0f;
                    }
                    else
                    {
                        float v1649;
                        float v1653;
                        v1649 = v1639;
                        v1653 = v1629;
                        float v1650;
                        bool v1652;
                        float v1654;
                        float v1675;
                        float v1676;
                        bool v1677;
                        uint v1646 = 1u;
                        bool v1651 = v1641;
                        for (;;)
                        {
                            if (v1646 < 3u)
                            {
                                v1654 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v1646) + v1557) * v1626, 2.0f)));
                                float3 v1664 = v1620 + (v1624 * v1654);
                                v1650 = v1664.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1664.xy, v1564).x - 1.0f) * 0.0500000007450580596923828125f);
                                v1652 = (v1650 < 0.0f) ? true : false;
                                if (v1652)
                                {
                                    v1675 = v1650;
                                    v1676 = v1654;
                                    v1677 = v1652;
                                    break;
                                }
                                v1646++;
                                v1649 = v1650;
                                v1651 = v1652;
                                v1653 = v1654;
                                continue;
                            }
                            else
                            {
                                v1675 = 0.0f;
                                v1676 = 1.0f;
                                v1677 = v1651;
                                break;
                            }
                        }
                        v1678 = v1675;
                        v1679 = v1649;
                        v1680 = v1676;
                        v1681 = v1677;
                        v1682 = v1653;
                    }
                    if (v1681)
                    {
                        float v1687;
                        float v1690;
                        float v1692;
                        float v1694;
                        v1687 = v1682;
                        v1690 = v1678;
                        v1692 = v1679;
                        v1694 = v1680;
                        for (uint v1696 = 0u; v1696 < 4u; )
                        {
                            float v1700 = v1687 + v1694;
                            float v1701 = v1700 * 0.5f;
                            float3 v1703 = v1620 + (v1624 * v1701);
                            float v1711 = v1703.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1703.xy, v1564).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool v1713 = (v1711 < 0.0f) ? true : false;
                            v1687 = v1713 ? v1687 : v1701;
                            v1690 = v1713 ? v1711 : v1690;
                            v1692 = v1713 ? v1692 : v1711;
                            v1694 = v1713 ? v1701 : v1694;
                            v1696++;
                            continue;
                        }
                        float v1717 = lerp(v1687, v1694, v1692 / (v1692 + (-v1690)));
                        float3 v1719 = v1620 + (v1624 * v1717);
                        v1733 = ((abs(v1719.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1719.xy, v1564).x - 1.0f) * 0.0500000007450580596923828125f)) < (v1692 * 0.89999997615814208984375f)) ? true : false) ? v1717 : v1694;
                        break;
                    }
                    else
                    {
                        v1733 = v1682;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                float3 v1734 = v1624 * v1733;
                float2 v1736 = v1526 + v1734.xy;
                float4 v1748 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v1590, v1734), 1.0f)));
                float3 v1757 = (stone_floor_normal_3.SampleLevel(sampler_llr, v1736, v1564).xyz * 2.0f) - 1.0f.xxx;
                v1757.z = v1757.z * 0.199999988079071044921875f;
                float4 v1765 = stone_floor_seed_3.SampleBias(sampler_llr, v1736, frame_constants_global_mip_bias);
                float v1766 = v1765.z;
                float v1767 = v1765.x;
                int v1776 = int(trunc((v1767 * 32.0f) - 0.5f) + trunc((v1765.y * 32.0f) - 0.5f));
                uint v1778 = uint(v1776 ^ 7993234);
                uint v1780 = v1778 + (v1778 << 10u);
                uint v1782 = v1780 ^ (v1780 >> 6u);
                uint v1784 = v1782 + (v1782 << 3u);
                uint v1786 = v1784 ^ (v1784 >> 11u);
                uint v1795 = uint(v1776 ^ 9730541);
                uint v1797 = v1795 + (v1795 << 10u);
                uint v1799 = v1797 ^ (v1797 >> 6u);
                uint v1801 = v1799 + (v1799 << 3u);
                uint v1803 = v1801 ^ (v1801 >> 11u);
                float v1809 = asfloat(((v1803 + (v1803 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
                float2 v1816 = ((v277 * 0.100000001490116119384765625f) + float2((asfloat(((v1786 + (v1786 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.300000011920928955078125f, v1809 * 0.300000011920928955078125f)) + (0.5f * v1766).xx;
                float4 v1818 = stone_floor_height_3.SampleBias(sampler_llr, v1736, frame_constants_global_mip_bias);
                float4 v1821 = stone_floor_roughness_3.SampleBias(sampler_llr, v1736, frame_constants_global_mip_bias);
                bool v1828;
                if (!(v1525 == 0))
                {
                    v1828 = v1525 == 4;
                }
                else
                {
                    v1828 = true;
                }
                bool v1833;
                if (!v1828)
                {
                    v1833 = v1525 == 10;
                }
                else
                {
                    v1833 = true;
                }
                bool v1838;
                if (!v1833)
                {
                    v1838 = v1525 == 12;
                }
                else
                {
                    v1838 = true;
                }
                float3 v1930;
                if (v1838)
                {
                    v1930 = brick_colors.Sample(sampler_llr, float3(v1816, 0.0f)).xyz * 0.89999997615814208984375f;
                }
                else
                {
                    bool v1847;
                    if (!(v1525 == 1))
                    {
                        v1847 = v1525 == 5;
                    }
                    else
                    {
                        v1847 = true;
                    }
                    float3 v1921;
                    if (v1847)
                    {
                        v1921 = brick_colors.Sample(sampler_llr, float3(v1816, 1.0f)).xyz;
                    }
                    else
                    {
                        bool v1856;
                        if (!(v1525 == 2))
                        {
                            v1856 = v1525 == 7;
                        }
                        else
                        {
                            v1856 = true;
                        }
                        float3 v1913;
                        if (v1856)
                        {
                            v1913 = (brick_colors.Sample(sampler_llr, float3(v1816, 2.0f)).xyz * 1.2999999523162841796875f) * lerp(1.0f, 1.2999999523162841796875f, v1766);
                        }
                        else
                        {
                            bool v1865;
                            if (!(v1525 == 3))
                            {
                                v1865 = v1525 == 6;
                            }
                            else
                            {
                                v1865 = true;
                            }
                            bool v1870;
                            if (!v1865)
                            {
                                v1870 = v1525 == 8;
                            }
                            else
                            {
                                v1870 = true;
                            }
                            bool v1875;
                            if (!v1870)
                            {
                                v1875 = v1525 == 9;
                            }
                            else
                            {
                                v1875 = true;
                            }
                            float3 v1902;
                            if (v1875)
                            {
                                v1902 = (brick_colors.Sample(sampler_llr, float3(v1816, 3.0f)).xyz * 0.89999997615814208984375f) * lerp(1.0f, 1.10000002384185791015625f, v1766);
                            }
                            else
                            {
                                float3 v1891;
                                if (v1525 == 11)
                                {
                                    v1891 = brick_colors.Sample(sampler_llr, float3(v1816, 4.0f)).xyz * lerp(1.0f, 1.39999997615814208984375f, v1766);
                                }
                                else
                                {
                                    v1891 = 0.0f.xxx;
                                }
                                v1902 = v1891;
                            }
                            v1913 = v1902;
                        }
                        v1921 = v1913;
                    }
                    v1930 = v1921;
                }
                v2354 = smoothstep(0.300000011920928955078125f, 1.0f, 1.0f - v1818.x);
                v2355 = (v1748.xyz / v1748.w.xxx).z;
                v2356 = lerp(0.75f, 1.0f, v1821.x) * lerp(0.800000011920928955078125f, 1.0f, v1809);
                v2357 = normalize(mul(v1757, v1613));
                v2358 = (v1930 * 0.800000011920928955078125f) * lerp(0.699999988079071044921875f, 1.0f, stone_floor_height_3.SampleBias(sampler_llr, v1736, frame_constants_global_mip_bias).x);
                v2359 = lerp(0.0f, 0.96875f, v1767);
            }
            v2771 = v2354;
            v2772 = v2355;
            v2773 = v2356;
            v2774 = v2357;
            v2775 = v2358;
            v2776 = v2359;
        }
        if ((v2776 * v2776) >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
        {
            discard;
        }
        v2781 = v2774;
        v2782 = v2773;
        v2783 = v2772;
        v2784 = v2771;
        v2785 = v2775;
    }
    else
    {
        bool v391;
        if (!(v376 == 1))
        {
            v391 = v376 == 3;
        }
        else
        {
            v391 = true;
        }
        float3 v1508;
        float v1509;
        float v1510;
        float v1511;
        float3 v1512;
        if (v391)
        {
            bool v402;
            if (!(v376 == 2))
            {
                v402 = v376 == 3;
            }
            else
            {
                v402 = true;
            }
            int v406;
            if (v402)
            {
                v406 = int(in_var_C7);
            }
            else
            {
                v406 = int(halftimber_color_ids.Load<uint>(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12) * 4 + 0));
            }
            float2 v407 = in_var_C3 * 0.20000000298023223876953125f;
            float v1499;
            float v1500;
            float v1501;
            float3 v1502;
            float3 v1503;
            float v1504;
            if (in_var_C8 == 0u)
            {
                float3 v1229;
                uint2 v1145 = uint2(gl_FragCoord.yx);
                float v1152 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v1162 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v1145.x) + v1152)) + (0.005837149918079376220703125f * (float(v1145.y) + v1152))));
                float _1165_tmp = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, v407);
                float2 v1165 = _1165_tmp.xx;
                float v1169 = v1165.x + frame_constants_global_mip_bias;
                float3 v1170 = ddx(in_var_C2);
                float3 v1171 = ddy(in_var_C2);
                float2 v1173 = ddx(v407);
                float2 v1174 = ddy(v407);
                float2 v1175 = -v1174;
                float3 v1176 = cross(in_var_C1, -v1171);
                float3 v1177 = cross(v1170, in_var_C1);
                float v1178 = v1173.x;
                float v1179 = v1175.y;
                float v1181 = v1173.y;
                float v1182 = v1175.x;
                float v1185 = 1.0f / ((v1178 * v1179) - (v1181 * v1182));
                float3x3 v1195 = transpose(float3x3(((v1176 * v1178) + (v1177 * v1182)) * v1185, ((v1176 * v1181) + (v1177 * v1179)) * v1185, in_var_C1));
                float4 v1200 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v1206 = mul(v1195, float3(1.0f, 0.0f, 0.0f));
                float3 v1207 = mul(v1195, float3(0.0f, 1.0f, 0.0f));
                float3 v1208 = mul(v1195, float3(0.0f, 0.0f, 1.0f));
                float3 v1211 = cross(v1206, v1207);
                float3 v1214 = (1.0f / dot(v1208, v1211)).xxx;
                float3x3 v1218 = float3x3(cross(v1207, v1208) * v1214, cross(v1208, v1206) * v1214, v1211 * v1214);
                float3 v1219 = mul(v1218, in_var_C2 - (v1200.xyz / v1200.w.xxx));
                float3 v1225 = float3(v407, 0.0f);
                float v1338;
                do
                {
                    v1229 = (v1225 + (v1219 * ((-0.07500000298023223876953125f) / v1219.z))) - v1225;
                    float v1231 = 1.0f / max(1.0f, 2.0f);
                    float v1234 = lerp(0.0f, 1.0f, pow(v1162 * v1231, 2.0f));
                    float3 v1236 = v1225 + (v1229 * v1234);
                    float v1244 = v1236.z - ((wooden_floor_height.SampleLevel(sampler_llr, v1236.xy, v1169).x - 1.0f) * 0.07500000298023223876953125f);
                    bool v1246 = (v1244 < 0.0f) ? true : false;
                    float v1283;
                    float v1284;
                    float v1285;
                    bool v1286;
                    float v1287;
                    if (v1246)
                    {
                        v1283 = v1244;
                        v1284 = 0.0f;
                        v1285 = v1234;
                        v1286 = v1246;
                        v1287 = 0.0f;
                    }
                    else
                    {
                        float v1254;
                        float v1258;
                        v1254 = v1244;
                        v1258 = v1234;
                        float v1255;
                        bool v1257;
                        float v1259;
                        float v1280;
                        float v1281;
                        bool v1282;
                        uint v1251 = 1u;
                        bool v1256 = v1246;
                        for (;;)
                        {
                            if (v1251 < 3u)
                            {
                                v1259 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v1251) + v1162) * v1231, 2.0f)));
                                float3 v1269 = v1225 + (v1229 * v1259);
                                v1255 = v1269.z - ((wooden_floor_height.SampleLevel(sampler_llr, v1269.xy, v1169).x - 1.0f) * 0.07500000298023223876953125f);
                                v1257 = (v1255 < 0.0f) ? true : false;
                                if (v1257)
                                {
                                    v1280 = v1255;
                                    v1281 = v1259;
                                    v1282 = v1257;
                                    break;
                                }
                                v1251++;
                                v1254 = v1255;
                                v1256 = v1257;
                                v1258 = v1259;
                                continue;
                            }
                            else
                            {
                                v1280 = 0.0f;
                                v1281 = 1.0f;
                                v1282 = v1256;
                                break;
                            }
                        }
                        v1283 = v1280;
                        v1284 = v1254;
                        v1285 = v1281;
                        v1286 = v1282;
                        v1287 = v1258;
                    }
                    if (v1286)
                    {
                        float v1292;
                        float v1295;
                        float v1297;
                        float v1299;
                        v1292 = v1287;
                        v1295 = v1283;
                        v1297 = v1284;
                        v1299 = v1285;
                        for (uint v1301 = 0u; v1301 < 4u; )
                        {
                            float v1305 = v1292 + v1299;
                            float v1306 = v1305 * 0.5f;
                            float3 v1308 = v1225 + (v1229 * v1306);
                            float v1316 = v1308.z - ((wooden_floor_height.SampleLevel(sampler_llr, v1308.xy, v1169).x - 1.0f) * 0.07500000298023223876953125f);
                            bool v1318 = (v1316 < 0.0f) ? true : false;
                            v1292 = v1318 ? v1292 : v1306;
                            v1295 = v1318 ? v1316 : v1295;
                            v1297 = v1318 ? v1297 : v1316;
                            v1299 = v1318 ? v1306 : v1299;
                            v1301++;
                            continue;
                        }
                        float v1322 = lerp(v1292, v1299, v1297 / (v1297 + (-v1295)));
                        float3 v1324 = v1225 + (v1229 * v1322);
                        v1338 = ((abs(v1324.z - ((wooden_floor_height.SampleLevel(sampler_llr, v1324.xy, v1169).x - 1.0f) * 0.07500000298023223876953125f)) < (v1297 * 0.89999997615814208984375f)) ? true : false) ? v1322 : v1299;
                        break;
                    }
                    else
                    {
                        v1338 = v1287;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                float3 v1339 = v1229 * v1338;
                float2 v1341 = v407 + v1339.xy;
                float4 v1353 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v1195, v1339), 1.0f)));
                float3 v1362 = (wooden_floor_normal.SampleLevel(sampler_llr, v1341, v1169).xyz * 2.0f) - 1.0f.xxx;
                v1362.z = v1362.z * 0.199999988079071044921875f;
                float4 v1373 = wooden_floor_seed.SampleBias(sampler_llr, v1341, frame_constants_global_mip_bias);
                float v1374 = v1373.z;
                float v1376 = v1373.x;
                int v1380 = int(trunc((v1376 * 32.0f) - 0.5f));
                float2 v1387 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(v1380))) + 0.5f.xx) + (float2(v1341.yx) * 0.87999999523162841796875f);
                float v1388 = v1373.y;
                float v1397;
                if (0.003130800090730190277099609375f >= v1388)
                {
                    v1397 = 12.9200000762939453125f * v1388;
                }
                else
                {
                    v1397 = (1.05499994754791259765625f * pow(v1388, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                }
                float v1400 = round(v1397 * 8.0f) * 0.785398185253143310546875f;
                float v1401 = v1387.x;
                float v1402 = cos(v1400);
                float v1404 = v1387.y;
                float v1405 = sin(v1400);
                float2 v1411 = float2((v1401 * v1402) - (v1404 * v1405), (v1401 * v1405) + (v1404 * v1402));
                float2 v1415;
                if (v1374 > 0.5f)
                {
                    v1415 = v1411 + 0.20000000298023223876953125f.xx;
                }
                else
                {
                    v1415 = v1411;
                }
                float4 v1422 = halftimber_colors.SampleBias(sampler_llr, float3(v1415, float(v406)), frame_constants_global_mip_bias);
                float3 v1423 = v1422.xyz;
                bool v1429;
                if (!(v406 == 2))
                {
                    v1429 = v406 == 8;
                }
                else
                {
                    v1429 = true;
                }
                bool v1434;
                if (!v1429)
                {
                    v1434 = v406 == 9;
                }
                else
                {
                    v1434 = true;
                }
                float3 v1440;
                if (v1434)
                {
                    v1440 = lerp(v1423, dot(v1423, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                }
                else
                {
                    v1440 = v1423;
                }
                float4 v1443 = halftimber_colors.SampleBias(sampler_llr, float3(v1415, 0.0f), frame_constants_global_mip_bias);
                float3 v1444 = v1443.xyz;
                float v1466;
                float3 v1467;
                if (v406 == 0)
                {
                    v1466 = v1374;
                    v1467 = v1444 * 1.2000000476837158203125f;
                }
                else
                {
                    bool v1455;
                    if (!(v406 == 9))
                    {
                        v1455 = v406 == 3;
                    }
                    else
                    {
                        v1455 = true;
                    }
                    bool v1460;
                    if (!v1455)
                    {
                        v1460 = v406 == 4;
                    }
                    else
                    {
                        v1460 = true;
                    }
                    float v1464;
                    if (v1460)
                    {
                        v1464 = v1374 * 0.5f;
                    }
                    else
                    {
                        v1464 = v1374 * 0.20000000298023223876953125f;
                    }
                    v1466 = v1464;
                    v1467 = v1444;
                }
                uint v1480 = uint(v1380 ^ 9730541);
                uint v1482 = v1480 + (v1480 << 10u);
                uint v1484 = v1482 ^ (v1482 >> 6u);
                uint v1486 = v1484 + (v1484 << 3u);
                uint v1488 = v1486 ^ (v1486 >> 11u);
                float v1496 = lerp(0.75f, 1.0f, wooden_floor_roughness.SampleBias(sampler_llr, v1341, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((v1488 + (v1488 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                v1499 = smoothstep(0.5f, 1.0f, v1496);
                v1500 = (v1353.xyz / v1353.w.xxx).z;
                v1501 = v1496;
                v1502 = normalize(mul(v1362, v1218));
                v1503 = lerp(v1440 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height.SampleBias(sampler_llr, v1341, frame_constants_global_mip_bias).x), v1467, v1466.xxx);
                v1504 = lerp(0.0f, 0.96875f, v1376);
            }
            else
            {
                float v1134;
                float v1135;
                float v1136;
                float3 v1137;
                float3 v1138;
                float v1139;
                if (in_var_C8 == 1u)
                {
                    float3 v864;
                    uint2 v780 = uint2(gl_FragCoord.yx);
                    float v787 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float v797 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v780.x) + v787)) + (0.005837149918079376220703125f * (float(v780.y) + v787))));
                    float _800_tmp = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, v407);
                    float2 v800 = _800_tmp.xx;
                    float v804 = v800.x + frame_constants_global_mip_bias;
                    float3 v805 = ddx(in_var_C2);
                    float3 v806 = ddy(in_var_C2);
                    float2 v808 = ddx(v407);
                    float2 v809 = ddy(v407);
                    float2 v810 = -v809;
                    float3 v811 = cross(in_var_C1, -v806);
                    float3 v812 = cross(v805, in_var_C1);
                    float v813 = v808.x;
                    float v814 = v810.y;
                    float v816 = v808.y;
                    float v817 = v810.x;
                    float v820 = 1.0f / ((v813 * v814) - (v816 * v817));
                    float3x3 v830 = transpose(float3x3(((v811 * v813) + (v812 * v817)) * v820, ((v811 * v816) + (v812 * v814)) * v820, in_var_C1));
                    float4 v835 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 v841 = mul(v830, float3(1.0f, 0.0f, 0.0f));
                    float3 v842 = mul(v830, float3(0.0f, 1.0f, 0.0f));
                    float3 v843 = mul(v830, float3(0.0f, 0.0f, 1.0f));
                    float3 v846 = cross(v841, v842);
                    float3 v849 = (1.0f / dot(v843, v846)).xxx;
                    float3x3 v853 = float3x3(cross(v842, v843) * v849, cross(v843, v841) * v849, v846 * v849);
                    float3 v854 = mul(v853, in_var_C2 - (v835.xyz / v835.w.xxx));
                    float3 v860 = float3(v407, 0.0f);
                    float v973;
                    do
                    {
                        v864 = (v860 + (v854 * ((-0.07500000298023223876953125f) / v854.z))) - v860;
                        float v866 = 1.0f / max(1.0f, 2.0f);
                        float v869 = lerp(0.0f, 1.0f, pow(v797 * v866, 2.0f));
                        float3 v871 = v860 + (v864 * v869);
                        float v879 = v871.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v871.xy, v804).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v881 = (v879 < 0.0f) ? true : false;
                        float v918;
                        float v919;
                        float v920;
                        bool v921;
                        float v922;
                        if (v881)
                        {
                            v918 = v879;
                            v919 = 0.0f;
                            v920 = v869;
                            v921 = v881;
                            v922 = 0.0f;
                        }
                        else
                        {
                            float v889;
                            float v893;
                            v889 = v879;
                            v893 = v869;
                            float v890;
                            bool v892;
                            float v894;
                            float v915;
                            float v916;
                            bool v917;
                            uint v886 = 1u;
                            bool v891 = v881;
                            for (;;)
                            {
                                if (v886 < 3u)
                                {
                                    v894 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v886) + v797) * v866, 2.0f)));
                                    float3 v904 = v860 + (v864 * v894);
                                    v890 = v904.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v904.xy, v804).x - 1.0f) * 0.07500000298023223876953125f);
                                    v892 = (v890 < 0.0f) ? true : false;
                                    if (v892)
                                    {
                                        v915 = v890;
                                        v916 = v894;
                                        v917 = v892;
                                        break;
                                    }
                                    v886++;
                                    v889 = v890;
                                    v891 = v892;
                                    v893 = v894;
                                    continue;
                                }
                                else
                                {
                                    v915 = 0.0f;
                                    v916 = 1.0f;
                                    v917 = v891;
                                    break;
                                }
                            }
                            v918 = v915;
                            v919 = v889;
                            v920 = v916;
                            v921 = v917;
                            v922 = v893;
                        }
                        if (v921)
                        {
                            float v927;
                            float v930;
                            float v932;
                            float v934;
                            v927 = v922;
                            v930 = v918;
                            v932 = v919;
                            v934 = v920;
                            for (uint v936 = 0u; v936 < 4u; )
                            {
                                float v940 = v927 + v934;
                                float v941 = v940 * 0.5f;
                                float3 v943 = v860 + (v864 * v941);
                                float v951 = v943.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v943.xy, v804).x - 1.0f) * 0.07500000298023223876953125f);
                                bool v953 = (v951 < 0.0f) ? true : false;
                                v927 = v953 ? v927 : v941;
                                v930 = v953 ? v951 : v930;
                                v932 = v953 ? v932 : v951;
                                v934 = v953 ? v941 : v934;
                                v936++;
                                continue;
                            }
                            float v957 = lerp(v927, v934, v932 / (v932 + (-v930)));
                            float3 v959 = v860 + (v864 * v957);
                            v973 = ((abs(v959.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v959.xy, v804).x - 1.0f) * 0.07500000298023223876953125f)) < (v932 * 0.89999997615814208984375f)) ? true : false) ? v957 : v934;
                            break;
                        }
                        else
                        {
                            v973 = v922;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    float3 v974 = v864 * v973;
                    float2 v976 = v407 + v974.xy;
                    float4 v988 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v830, v974), 1.0f)));
                    float3 v997 = (wooden_floor_normal_2.SampleLevel(sampler_llr, v976, v804).xyz * 2.0f) - 1.0f.xxx;
                    v997.z = v997.z * 0.199999988079071044921875f;
                    float4 v1008 = wooden_floor_seed_2.SampleBias(sampler_llr, v976, frame_constants_global_mip_bias);
                    float v1009 = v1008.z;
                    float v1011 = v1008.x;
                    int v1015 = int(trunc((v1011 * 32.0f) - 0.5f));
                    float2 v1022 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(v1015))) + 0.5f.xx) + (float2(v976.yx) * 0.87999999523162841796875f);
                    float v1023 = v1008.y;
                    float v1032;
                    if (0.003130800090730190277099609375f >= v1023)
                    {
                        v1032 = 12.9200000762939453125f * v1023;
                    }
                    else
                    {
                        v1032 = (1.05499994754791259765625f * pow(v1023, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                    }
                    float v1035 = round(v1032 * 8.0f) * 0.785398185253143310546875f;
                    float v1036 = v1022.x;
                    float v1037 = cos(v1035);
                    float v1039 = v1022.y;
                    float v1040 = sin(v1035);
                    float2 v1046 = float2((v1036 * v1037) - (v1039 * v1040), (v1036 * v1040) + (v1039 * v1037));
                    float2 v1050;
                    if (v1009 > 0.5f)
                    {
                        v1050 = v1046 + 0.20000000298023223876953125f.xx;
                    }
                    else
                    {
                        v1050 = v1046;
                    }
                    float4 v1057 = halftimber_colors.SampleBias(sampler_llr, float3(v1050, float(v406)), frame_constants_global_mip_bias);
                    float3 v1058 = v1057.xyz;
                    bool v1064;
                    if (!(v406 == 2))
                    {
                        v1064 = v406 == 8;
                    }
                    else
                    {
                        v1064 = true;
                    }
                    bool v1069;
                    if (!v1064)
                    {
                        v1069 = v406 == 9;
                    }
                    else
                    {
                        v1069 = true;
                    }
                    float3 v1075;
                    if (v1069)
                    {
                        v1075 = lerp(v1058, dot(v1058, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                    }
                    else
                    {
                        v1075 = v1058;
                    }
                    float4 v1078 = halftimber_colors.SampleBias(sampler_llr, float3(v1050, 0.0f), frame_constants_global_mip_bias);
                    float3 v1079 = v1078.xyz;
                    float v1101;
                    float3 v1102;
                    if (v406 == 0)
                    {
                        v1101 = v1009;
                        v1102 = v1079 * 1.2000000476837158203125f;
                    }
                    else
                    {
                        bool v1090;
                        if (!(v406 == 9))
                        {
                            v1090 = v406 == 3;
                        }
                        else
                        {
                            v1090 = true;
                        }
                        bool v1095;
                        if (!v1090)
                        {
                            v1095 = v406 == 4;
                        }
                        else
                        {
                            v1095 = true;
                        }
                        float v1099;
                        if (v1095)
                        {
                            v1099 = v1009 * 0.5f;
                        }
                        else
                        {
                            v1099 = v1009 * 0.20000000298023223876953125f;
                        }
                        v1101 = v1099;
                        v1102 = v1079;
                    }
                    uint v1115 = uint(v1015 ^ 9730541);
                    uint v1117 = v1115 + (v1115 << 10u);
                    uint v1119 = v1117 ^ (v1117 >> 6u);
                    uint v1121 = v1119 + (v1119 << 3u);
                    uint v1123 = v1121 ^ (v1121 >> 11u);
                    float v1131 = lerp(0.75f, 1.0f, wooden_floor_roughness_2.SampleBias(sampler_llr, v976, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((v1123 + (v1123 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                    v1134 = smoothstep(0.550000011920928955078125f, 1.0f, v1131);
                    v1135 = (v988.xyz / v988.w.xxx).z;
                    v1136 = v1131;
                    v1137 = normalize(mul(v997, v853));
                    v1138 = lerp(v1075 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height_2.SampleBias(sampler_llr, v976, frame_constants_global_mip_bias).x), v1102, v1101.xxx);
                    v1139 = lerp(0.0f, 0.96875f, v1011);
                }
                else
                {
                    float3 v505;
                    uint2 v421 = uint2(gl_FragCoord.yx);
                    float v428 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float v438 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v421.x) + v428)) + (0.005837149918079376220703125f * (float(v421.y) + v428))));
                    float _441_tmp = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, v407);
                    float2 v441 = _441_tmp.xx;
                    float v445 = v441.x + frame_constants_global_mip_bias;
                    float3 v446 = ddx(in_var_C2);
                    float3 v447 = ddy(in_var_C2);
                    float2 v449 = ddx(v407);
                    float2 v450 = ddy(v407);
                    float2 v451 = -v450;
                    float3 v452 = cross(in_var_C1, -v447);
                    float3 v453 = cross(v446, in_var_C1);
                    float v454 = v449.x;
                    float v455 = v451.y;
                    float v457 = v449.y;
                    float v458 = v451.x;
                    float v461 = 1.0f / ((v454 * v455) - (v457 * v458));
                    float3x3 v471 = transpose(float3x3(((v452 * v454) + (v453 * v458)) * v461, ((v452 * v457) + (v453 * v455)) * v461, in_var_C1));
                    float4 v476 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 v482 = mul(v471, float3(1.0f, 0.0f, 0.0f));
                    float3 v483 = mul(v471, float3(0.0f, 1.0f, 0.0f));
                    float3 v484 = mul(v471, float3(0.0f, 0.0f, 1.0f));
                    float3 v487 = cross(v482, v483);
                    float3 v490 = (1.0f / dot(v484, v487)).xxx;
                    float3x3 v494 = float3x3(cross(v483, v484) * v490, cross(v484, v482) * v490, v487 * v490);
                    float3 v495 = mul(v494, in_var_C2 - (v476.xyz / v476.w.xxx));
                    float3 v501 = float3(v407, 0.0f);
                    float v614;
                    do
                    {
                        v505 = (v501 + (v495 * ((-0.07500000298023223876953125f) / v495.z))) - v501;
                        float v507 = 1.0f / max(1.0f, 2.0f);
                        float v510 = lerp(0.0f, 1.0f, pow(v438 * v507, 2.0f));
                        float3 v512 = v501 + (v505 * v510);
                        float v520 = v512.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v512.xy, v445).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v522 = (v520 < 0.0f) ? true : false;
                        float v559;
                        float v560;
                        float v561;
                        bool v562;
                        float v563;
                        if (v522)
                        {
                            v559 = v520;
                            v560 = 0.0f;
                            v561 = v510;
                            v562 = v522;
                            v563 = 0.0f;
                        }
                        else
                        {
                            float v530;
                            float v534;
                            v530 = v520;
                            v534 = v510;
                            float v531;
                            bool v533;
                            float v535;
                            float v556;
                            float v557;
                            bool v558;
                            uint v527 = 1u;
                            bool v532 = v522;
                            for (;;)
                            {
                                if (v527 < 3u)
                                {
                                    v535 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v527) + v438) * v507, 2.0f)));
                                    float3 v545 = v501 + (v505 * v535);
                                    v531 = v545.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v545.xy, v445).x - 1.0f) * 0.07500000298023223876953125f);
                                    v533 = (v531 < 0.0f) ? true : false;
                                    if (v533)
                                    {
                                        v556 = v531;
                                        v557 = v535;
                                        v558 = v533;
                                        break;
                                    }
                                    v527++;
                                    v530 = v531;
                                    v532 = v533;
                                    v534 = v535;
                                    continue;
                                }
                                else
                                {
                                    v556 = 0.0f;
                                    v557 = 1.0f;
                                    v558 = v532;
                                    break;
                                }
                            }
                            v559 = v556;
                            v560 = v530;
                            v561 = v557;
                            v562 = v558;
                            v563 = v534;
                        }
                        if (v562)
                        {
                            float v568;
                            float v571;
                            float v573;
                            float v575;
                            v568 = v563;
                            v571 = v559;
                            v573 = v560;
                            v575 = v561;
                            for (uint v577 = 0u; v577 < 4u; )
                            {
                                float v581 = v568 + v575;
                                float v582 = v581 * 0.5f;
                                float3 v584 = v501 + (v505 * v582);
                                float v592 = v584.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v584.xy, v445).x - 1.0f) * 0.07500000298023223876953125f);
                                bool v594 = (v592 < 0.0f) ? true : false;
                                v568 = v594 ? v568 : v582;
                                v571 = v594 ? v592 : v571;
                                v573 = v594 ? v573 : v592;
                                v575 = v594 ? v582 : v575;
                                v577++;
                                continue;
                            }
                            float v598 = lerp(v568, v575, v573 / (v573 + (-v571)));
                            float3 v600 = v501 + (v505 * v598);
                            v614 = ((abs(v600.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v600.xy, v445).x - 1.0f) * 0.07500000298023223876953125f)) < (v573 * 0.89999997615814208984375f)) ? true : false) ? v598 : v575;
                            break;
                        }
                        else
                        {
                            v614 = v563;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    float3 v615 = v505 * v614;
                    float2 v617 = v407 + v615.xy;
                    float4 v629 = mul(frame_constants_view_constants[current_view_index].view_to_clip, mul(frame_constants_view_constants[current_view_index].world_to_view, float4(in_var_C2 + mul(v471, v615), 1.0f)));
                    float3 v638 = (wooden_floor_normal_3.SampleLevel(sampler_llr, v617, v445).xyz * 2.0f) - 1.0f.xxx;
                    v638.z = v638.z * 0.199999988079071044921875f;
                    float4 v649 = wooden_floor_seed_3.SampleBias(sampler_llr, v617, frame_constants_global_mip_bias);
                    float v650 = v649.z;
                    float v652 = v649.x;
                    int v656 = int(trunc((v652 * 32.0f) - 0.5f));
                    float2 v663 = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(uint(v656))) + 0.5f.xx) + (float2(v617.yx) * 0.87999999523162841796875f);
                    float v664 = v649.y;
                    float v673;
                    if (0.003130800090730190277099609375f >= v664)
                    {
                        v673 = 12.9200000762939453125f * v664;
                    }
                    else
                    {
                        v673 = (1.05499994754791259765625f * pow(v664, 0.4166666567325592041015625f)) - 0.054999999701976776123046875f;
                    }
                    float v676 = round(v673 * 8.0f) * 0.785398185253143310546875f;
                    float v677 = v663.x;
                    float v678 = cos(v676);
                    float v680 = v663.y;
                    float v681 = sin(v676);
                    float2 v687 = float2((v677 * v678) - (v680 * v681), (v677 * v681) + (v680 * v678));
                    float2 v691;
                    if (v650 > 0.5f)
                    {
                        v691 = v687 + 0.20000000298023223876953125f.xx;
                    }
                    else
                    {
                        v691 = v687;
                    }
                    float4 v698 = halftimber_colors.SampleBias(sampler_llr, float3(v691, float(v406)), frame_constants_global_mip_bias);
                    float3 v699 = v698.xyz;
                    bool v705;
                    if (!(v406 == 2))
                    {
                        v705 = v406 == 8;
                    }
                    else
                    {
                        v705 = true;
                    }
                    bool v710;
                    if (!v705)
                    {
                        v710 = v406 == 9;
                    }
                    else
                    {
                        v710 = true;
                    }
                    float3 v716;
                    if (v710)
                    {
                        v716 = lerp(v699, dot(v699, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)).xxx, 0.300000011920928955078125f.xxx);
                    }
                    else
                    {
                        v716 = v699;
                    }
                    float4 v719 = halftimber_colors.SampleBias(sampler_llr, float3(v691, 0.0f), frame_constants_global_mip_bias);
                    float3 v720 = v719.xyz;
                    float v742;
                    float3 v743;
                    if (v406 == 0)
                    {
                        v742 = v650;
                        v743 = v720 * 1.2000000476837158203125f;
                    }
                    else
                    {
                        bool v731;
                        if (!(v406 == 9))
                        {
                            v731 = v406 == 3;
                        }
                        else
                        {
                            v731 = true;
                        }
                        bool v736;
                        if (!v731)
                        {
                            v736 = v406 == 4;
                        }
                        else
                        {
                            v736 = true;
                        }
                        float v740;
                        if (v736)
                        {
                            v740 = v650 * 0.5f;
                        }
                        else
                        {
                            v740 = v650 * 0.20000000298023223876953125f;
                        }
                        v742 = v740;
                        v743 = v720;
                    }
                    uint v756 = uint(v656 ^ 9730541);
                    uint v758 = v756 + (v756 << 10u);
                    uint v760 = v758 ^ (v758 >> 6u);
                    uint v762 = v760 + (v760 << 3u);
                    uint v764 = v762 ^ (v762 >> 11u);
                    float v772 = lerp(0.75f, 1.0f, wooden_floor_roughness_3.SampleBias(sampler_llr, v617, frame_constants_global_mip_bias).x) * lerp(0.800000011920928955078125f, 1.0f, asfloat(((v764 + (v764 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
                    v1134 = smoothstep(0.5f, 1.0f, v772);
                    v1135 = (v629.xyz / v629.w.xxx).z;
                    v1136 = v772;
                    v1137 = normalize(mul(v638, v494));
                    v1138 = lerp(v716 * lerp(0.699999988079071044921875f, 1.0f, wooden_floor_height_3.SampleBias(sampler_llr, v617, frame_constants_global_mip_bias).x), v743, v742.xxx);
                    v1139 = lerp(0.0f, 0.96875f, v652);
                }
                v1499 = v1134;
                v1500 = v1135;
                v1501 = v1136;
                v1502 = v1137;
                v1503 = v1138;
                v1504 = v1139;
            }
            if (v1504 >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
            {
                discard;
            }
            v1508 = v1502;
            v1509 = v1501;
            v1510 = v1500;
            v1511 = v1499;
            v1512 = v1503;
        }
        else
        {
            v1508 = 0.0f.xxx;
            v1509 = 0.800000011920928955078125f;
            v1510 = gl_FragCoord.z;
            v1511 = 0.0f;
            v1512 = 0.0f.xxx;
        }
        v2781 = v1508;
        v2782 = v1509;
        v2783 = v1510;
        v2784 = v1511;
        v2785 = v1512;
    }
// 高亮/变灰混合 + 雪主题路径淡化 + 法线/反照率打包,写深度偏移
    bool3 v2786 = v266.xxx;
    float3 v2787 = float3(v2786.x ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).x : v2785.x, v2786.y ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).y : v2785.y, v2786.z ? float3(0.800000011920928955078125f, 0.519999980926513671875f, 0.4000000059604644775390625f).z : v2785.z);
    bool v2789 = int(v273) == 1;
    float3 v2793;
    if (v2789)
    {
        v2793 = lerp(v2787, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
    }
    else
    {
        v2793 = v2787;
    }
    float3 v2819;
    if (frame_constants_glade_theme_id == 4u)
    {
        float4 v2809 = dirthpath_heightmap.SampleBias(sampler_llr, (in_var_C3 * 0.0199999995529651641845703125f) + (v277 * 0.0500000007450580596923828125f), frame_constants_global_mip_bias);
        float v2810 = v2809.x;
        v2819 = lerp(v2793, 0.699999988079071044921875f.xxx, ((smoothstep(0.0f, 0.20000000298023223876953125f, v2784 * lerp(0.0f, 0.5f, v2810)) * 0.60000002384185791015625f) + (v2810 * 0.20000000298023223876953125f)).xxx);
    }
    else
    {
        v2819 = v2793;
    }
    float3 v2821 = (v2781 * 0.5f) + 0.5f.xxx;
    float3 v2848 = sqrt(clamp(v2819, 0.0f.xxx, 1.0f.xxx));
    uint v2867 = (uint((clamp(v2848.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v2848.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v2848.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 v2868 = uint2(((uint((clamp(v2821.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v2821.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v2821.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(v2782, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), v2867);
    v2868.y = v2867 | (((((7u | (uint(v266 ? true : (v2789 ? true : false)) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = v2868;
    gl_FragDepth = v2783;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_roof_floor.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 逐字节一致;
//   PS 为完整的木/石材质流程(变量命名不同),含天窗剔除与深度写。
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

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
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
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t16, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static uint out_var_C4;
static uint out_var_C5;
static uint out_var_C6;
static uint out_var_C7;
static uint out_var_C8;
static float3x3 out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] nointerpolation uint out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation uint out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3x3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

// VS 主函数:按实例朝向翻转法线,顶点 xz 乘缩放后平移到世界,y 固定 1/512 台阶
void vert_main()
{
    float3 v141 = float3(nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16), 0.0f).xzy;
    float3 v143 = float3(-nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y, 0.0f, nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x);
    float4 v148 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    bool v157 = (nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) != 0u) ? true : ((nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0) - (v148.xyz / v148.w.xxx)).y > 0.0f);
    float3 v161;
    if (v157)
    {
        v161 = v141 * (-1.0f);
    }
    else
    {
        v161 = v141;
    }
    bool3 v162 = v157.xxx;
    float3 v163 = float3(v162.x ? float3(-0.0f, -1.0f, -0.0f).x : float3(0.0f, 1.0f, 0.0f).x, v162.y ? float3(-0.0f, -1.0f, -0.0f).y : float3(0.0f, 1.0f, 0.0f).y, v162.z ? float3(-0.0f, -1.0f, -0.0f).z : float3(0.0f, 1.0f, 0.0f).z);
    float2 v165 = in_var_Vertex_Position.xz * nani_instance_data.Load<float2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24);
    float3 v168 = mul(v165, float2x3(v161, v143)) + nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0);
    float v176 = 0.001953125f * (floor(v168.y * 512.0f) + frac(float(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12)) * 1.61803400516510009765625f));
    float3 v177 = v168;
    v177.y = v176;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v168.x, v176, v168.z, 1.0f));
    out_var_C1 = v163;
    out_var_C2 = v177;
    out_var_C3 = v165;
    out_var_C4 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C5 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44);
    out_var_C6 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48);
    out_var_C7 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52);
    out_var_C8 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56);
    out_var_C9 = transpose(float3x3(v161, v143, v163));
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
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
    stage_output.out_var_C9 = out_var_C9;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_roof_floor.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。VS 与 b903 逐字节一致;
//   PS 为完整的木/石材质流程(变量命名不同),含天窗剔除与深度写。
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

struct InstanceData
{
    float3 position;
    uint wall_id;
    float2 basis0_xz;
    float2 scale;
    float roof_animation;
    uint shape;
    uint double_sided;
    uint is_roof_bottom;
    uint material_id;
    uint override_color_id;
    uint pattern_id;
    uint pad_1;
};

struct DormerHole
{
    float3 position;
    int wall_id;
    float3 size;
    int pad0;
    float2 normal;
    float k;
    float a;
};

static bool v147;

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
    uint2 dirthpath_heightmap_dims : packoffset(c0.y);
    uint highlighted_wall_id : packoffset(c0.w);
    uint highlight_mode : packoffset(c1);
    uint2 wooden_floor_normal_dims : packoffset(c1.y);
    uint2 wooden_floor_height_dims : packoffset(c2);
    uint2 wooden_floor_seed_dims : packoffset(c2.z);
    uint2 wooden_floor_roughness_dims : packoffset(c3);
    uint2 wooden_floor_normal_2_dims : packoffset(c3.z);
    uint2 wooden_floor_height_2_dims : packoffset(c4);
    uint2 wooden_floor_seed_2_dims : packoffset(c4.z);
    uint2 wooden_floor_roughness_2_dims : packoffset(c5);
    uint2 wooden_floor_normal_3_dims : packoffset(c5.z);
    uint2 wooden_floor_height_3_dims : packoffset(c6);
    uint2 wooden_floor_seed_3_dims : packoffset(c6.z);
    uint2 wooden_floor_roughness_3_dims : packoffset(c7);
    uint2 stone_floor_normal_dims : packoffset(c7.z);
    uint2 stone_floor_seed_dims : packoffset(c8);
    uint2 stone_floor_height_dims : packoffset(c8.z);
    uint2 stone_floor_roughness_dims : packoffset(c9);
    uint2 stone_floor_seed_2_dims : packoffset(c9.z);
    uint2 stone_floor_normal_2_dims : packoffset(c10);
    uint2 stone_floor_height_2_dims : packoffset(c10.z);
    uint2 stone_floor_roughness_2_dims : packoffset(c11);
    uint2 stone_floor_seed_3_dims : packoffset(c11.z);
    uint2 stone_floor_normal_3_dims : packoffset(c12);
    uint2 stone_floor_height_3_dims : packoffset(c12.z);
    uint2 stone_floor_roughness_3_dims : packoffset(c13);
};

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer dormer_holes : register(t18, space0);
ByteAddressBuffer dormer_hole_count : register(t19, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> wooden_floor_height : register(t23, space0);
Texture2D<float4> wooden_floor_seed : register(t24, space0);
Texture2D<float4> wooden_floor_height_2 : register(t27, space0);
Texture2D<float4> wooden_floor_seed_2 : register(t28, space0);
Texture2D<float4> wooden_floor_height_3 : register(t31, space0);
Texture2D<float4> wooden_floor_seed_3 : register(t32, space0);
Texture2D<float4> stone_floor_seed : register(t38, space0);
Texture2D<float4> stone_floor_height : register(t39, space0);
Texture2D<float4> stone_floor_seed_2 : register(t41, space0);
Texture2D<float4> stone_floor_height_2 : register(t43, space0);
Texture2D<float4> stone_floor_seed_3 : register(t45, space0);
Texture2D<float4> stone_floor_height_3 : register(t47, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C6;
static uint in_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(6)]] nointerpolation uint in_var_C6 : TEXCOORD6;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
    float4 gl_FragCoord : SV_Position;
};

// PS 主函数:高亮判定 -> 天窗剔除 -> 木/石材质采样/视差 -> 主题/高亮混合 -> 打包
void frag_main()
{
// f0ad PS:读取 wall_id,做同样的天窗空间哈希剔除
    int v169 = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    bool v267;
    do
    {
        float2 v174 = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
        int v182 = int(floor(v174.x)) + (15 * int(floor(v174.y)));
        uint v186 = min(128u, dormer_hole_count.Load<uint>(uint(v182) * 4 + 0));
        bool v264;
        bool v265;
        uint v188 = 0u;
        for (;;)
        {
            if (v188 < v186)
            {
                uint v198 = uint(int(uint(v182 * 50) + v188));
                if (v169 == dormer_holes.Load<int>(v198 * 48 + 12))
                {
                    float3 v214 = float3(dormer_holes.Load<float>(v198 * 48 + 32), 0.0f, dormer_holes.Load<float>(v198 * 48 + 36));
                    float3x3 v218 = transpose(float3x3(cross(v214, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), v214));
                    float3 v219 = mul(v218, float3(1.0f, 0.0f, 0.0f));
                    float3 v220 = mul(v218, float3(0.0f, 1.0f, 0.0f));
                    float3 v221 = mul(v218, float3(0.0f, 0.0f, 1.0f));
                    float3 v224 = cross(v219, v220);
                    float3 v227 = (1.0f / dot(v221, v224)).xxx;
                    float3 v235 = mul(in_var_C2 - dormer_holes.Load<float3>(v198 * 48 + 0), float3x3(cross(v220, v221) * v227, cross(v221, v219) * v227, v224 * v227)) / dormer_holes.Load<float3>(v198 * 48 + 16);
                    float v236 = v235.y;
                    float v239 = clamp((v236 * dormer_holes.Load<float>(v198 * 48 + 40)) + dormer_holes.Load<float>(v198 * 48 + 44), 0.0f, 1.0f);
                    bool v255;
                    if (abs(v235.x) < lerp(0.5f, 0.0f, (v239 * v239) * v239))
                    {
                        bool v254;
                        if (v236 < ((dormer_holes.Load<float>(v198 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                        {
                            v254 = v236 > (-0.5f);
                        }
                        else
                        {
                            v254 = false;
                        }
                        v255 = v254;
                    }
                    else
                    {
                        v255 = false;
                    }
                    bool v261;
                    if (v255)
                    {
                        v261 = abs(v235.z) < 0.5f;
                    }
                    else
                    {
                        v261 = false;
                    }
                    if (v261)
                    {
                        v264 = true;
                        v265 = true;
                        break;
                    }
                }
                v188++;
                continue;
            }
            else
            {
                v264 = v147;
                v265 = false;
                break;
            }
        }
        if (v265)
        {
            v267 = v264;
            break;
        }
        v267 = false;
        break;
    } while(false);
    if (v267)
    {
        discard;
    }
    int v270 = int(in_var_C6);
    bool v276;
    if (!(v270 == 0))
    {
        v276 = v270 == 2;
    }
    else
    {
        v276 = true;
    }
    if (v276)
    {
// 地板 uv;C8 材质位分流,ddx/ddy 构造切空间做高度视差
        float2 v914 = in_var_C3 * 0.20000000298023223876953125f;
        float v1536;
        if (in_var_C8 == 0u)
        {
            float3 v1419;
            uint2 v1335 = uint2(gl_FragCoord.yx);
            float v1342 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
            float v1352 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v1335.x) + v1342)) + (0.005837149918079376220703125f * (float(v1335.y) + v1342))));
            float _1355_tmp = stone_floor_height.CalculateLevelOfDetail(sampler_llr, v914);
            float2 v1355 = _1355_tmp.xx;
            float v1359 = v1355.x + frame_constants_global_mip_bias;
            float3 v1360 = ddx(in_var_C2);
            float3 v1361 = ddy(in_var_C2);
            float2 v1363 = ddx(v914);
            float2 v1364 = ddy(v914);
            float2 v1365 = -v1364;
            float3 v1366 = cross(in_var_C1, -v1361);
            float3 v1367 = cross(v1360, in_var_C1);
            float v1368 = v1363.x;
            float v1369 = v1365.y;
            float v1371 = v1363.y;
            float v1372 = v1365.x;
            float v1375 = 1.0f / ((v1368 * v1369) - (v1371 * v1372));
            float3x3 v1385 = transpose(float3x3(((v1366 * v1368) + (v1367 * v1372)) * v1375, ((v1366 * v1371) + (v1367 * v1369)) * v1375, in_var_C1));
            float4 v1390 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
            float3 v1396 = mul(v1385, float3(1.0f, 0.0f, 0.0f));
            float3 v1397 = mul(v1385, float3(0.0f, 1.0f, 0.0f));
            float3 v1398 = mul(v1385, float3(0.0f, 0.0f, 1.0f));
            float3 v1401 = cross(v1396, v1397);
            float3 v1404 = (1.0f / dot(v1398, v1401)).xxx;
            float3 v1409 = mul(float3x3(cross(v1397, v1398) * v1404, cross(v1398, v1396) * v1404, v1401 * v1404), in_var_C2 - (v1390.xyz / v1390.w.xxx));
// 视差映射:沿视线推进采样高度场,得到带位移的 uv
            float3 v1415 = float3(v914, 0.0f);
            float v1528;
            do
            {
                v1419 = (v1415 + (v1409 * ((-0.07500000298023223876953125f) / v1409.z))) - v1415;
                float v1421 = 1.0f / max(1.0f, 2.0f);
                float v1424 = lerp(0.0f, 1.0f, pow(v1352 * v1421, 2.0f));
                float3 v1426 = v1415 + (v1419 * v1424);
                float v1434 = v1426.z - ((stone_floor_height.SampleLevel(sampler_llr, v1426.xy, v1359).x - 1.0f) * 0.07500000298023223876953125f);
                bool v1436 = (v1434 < 0.0f) ? true : false;
                float v1473;
                float v1474;
                float v1475;
                bool v1476;
                float v1477;
                if (v1436)
                {
                    v1473 = v1434;
                    v1474 = 0.0f;
                    v1475 = v1424;
                    v1476 = v1436;
                    v1477 = 0.0f;
                }
                else
                {
                    float v1444;
                    float v1448;
                    v1444 = v1434;
                    v1448 = v1424;
                    float v1445;
                    bool v1447;
                    float v1449;
                    float v1470;
                    float v1471;
                    bool v1472;
                    uint v1441 = 1u;
                    bool v1446 = v1436;
                    for (;;)
                    {
                        if (v1441 < 3u)
                        {
                            v1449 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v1441) + v1352) * v1421, 2.0f)));
                            float3 v1459 = v1415 + (v1419 * v1449);
                            v1445 = v1459.z - ((stone_floor_height.SampleLevel(sampler_llr, v1459.xy, v1359).x - 1.0f) * 0.07500000298023223876953125f);
                            v1447 = (v1445 < 0.0f) ? true : false;
                            if (v1447)
                            {
                                v1470 = v1445;
                                v1471 = v1449;
                                v1472 = v1447;
                                break;
                            }
                            v1441++;
                            v1444 = v1445;
                            v1446 = v1447;
                            v1448 = v1449;
                            continue;
                        }
                        else
                        {
                            v1470 = 0.0f;
                            v1471 = 1.0f;
                            v1472 = v1446;
                            break;
                        }
                    }
                    v1473 = v1470;
                    v1474 = v1444;
                    v1475 = v1471;
                    v1476 = v1472;
                    v1477 = v1448;
                }
                if (v1476)
                {
                    float v1482;
                    float v1485;
                    float v1487;
                    float v1489;
                    v1482 = v1477;
                    v1485 = v1473;
                    v1487 = v1474;
                    v1489 = v1475;
                    for (uint v1491 = 0u; v1491 < 4u; )
                    {
                        float v1495 = v1482 + v1489;
                        float v1496 = v1495 * 0.5f;
                        float3 v1498 = v1415 + (v1419 * v1496);
                        float v1506 = v1498.z - ((stone_floor_height.SampleLevel(sampler_llr, v1498.xy, v1359).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v1508 = (v1506 < 0.0f) ? true : false;
                        v1482 = v1508 ? v1482 : v1496;
                        v1485 = v1508 ? v1506 : v1485;
                        v1487 = v1508 ? v1487 : v1506;
                        v1489 = v1508 ? v1496 : v1489;
                        v1491++;
                        continue;
                    }
                    float v1512 = lerp(v1482, v1489, v1487 / (v1487 + (-v1485)));
                    float3 v1514 = v1415 + (v1419 * v1512);
                    v1528 = ((abs(v1514.z - ((stone_floor_height.SampleLevel(sampler_llr, v1514.xy, v1359).x - 1.0f) * 0.07500000298023223876953125f)) < (v1487 * 0.89999997615814208984375f)) ? true : false) ? v1512 : v1489;
                    break;
                }
                else
                {
                    v1528 = v1477;
                    break;
                }
                break; // unreachable workaround
            } while(false);
            v1536 = lerp(0.0f, 0.96875f, stone_floor_seed.SampleBias(sampler_llr, v914 + (v1419 * v1528).xy, frame_constants_global_mip_bias).x);
        }
        else
        {
            float v1331;
            if (in_var_C8 == 1u)
            {
                float3 v1214;
                uint2 v1130 = uint2(gl_FragCoord.yx);
                float v1137 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v1147 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v1130.x) + v1137)) + (0.005837149918079376220703125f * (float(v1130.y) + v1137))));
                float _1150_tmp = stone_floor_height_2.CalculateLevelOfDetail(sampler_llr, v914);
                float2 v1150 = _1150_tmp.xx;
                float v1154 = v1150.x + frame_constants_global_mip_bias;
                float3 v1155 = ddx(in_var_C2);
                float3 v1156 = ddy(in_var_C2);
                float2 v1158 = ddx(v914);
                float2 v1159 = ddy(v914);
                float2 v1160 = -v1159;
                float3 v1161 = cross(in_var_C1, -v1156);
                float3 v1162 = cross(v1155, in_var_C1);
                float v1163 = v1158.x;
                float v1164 = v1160.y;
                float v1166 = v1158.y;
                float v1167 = v1160.x;
                float v1170 = 1.0f / ((v1163 * v1164) - (v1166 * v1167));
                float3x3 v1180 = transpose(float3x3(((v1161 * v1163) + (v1162 * v1167)) * v1170, ((v1161 * v1166) + (v1162 * v1164)) * v1170, in_var_C1));
                float4 v1185 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v1191 = mul(v1180, float3(1.0f, 0.0f, 0.0f));
                float3 v1192 = mul(v1180, float3(0.0f, 1.0f, 0.0f));
                float3 v1193 = mul(v1180, float3(0.0f, 0.0f, 1.0f));
                float3 v1196 = cross(v1191, v1192);
                float3 v1199 = (1.0f / dot(v1193, v1196)).xxx;
                float3 v1204 = mul(float3x3(cross(v1192, v1193) * v1199, cross(v1193, v1191) * v1199, v1196 * v1199), in_var_C2 - (v1185.xyz / v1185.w.xxx));
                float3 v1210 = float3(v914, 0.0f);
                float v1323;
                do
                {
                    v1214 = (v1210 + (v1204 * ((-0.0500000007450580596923828125f) / v1204.z))) - v1210;
                    float v1216 = 1.0f / max(1.0f, 2.0f);
                    float v1219 = lerp(0.0f, 1.0f, pow(v1147 * v1216, 2.0f));
                    float3 v1221 = v1210 + (v1214 * v1219);
                    float v1229 = v1221.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v1221.xy, v1154).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool v1231 = (v1229 < 0.0f) ? true : false;
                    float v1268;
                    float v1269;
                    float v1270;
                    bool v1271;
                    float v1272;
                    if (v1231)
                    {
                        v1268 = v1229;
                        v1269 = 0.0f;
                        v1270 = v1219;
                        v1271 = v1231;
                        v1272 = 0.0f;
                    }
                    else
                    {
                        float v1239;
                        float v1243;
                        v1239 = v1229;
                        v1243 = v1219;
                        float v1240;
                        bool v1242;
                        float v1244;
                        float v1265;
                        float v1266;
                        bool v1267;
                        uint v1236 = 1u;
                        bool v1241 = v1231;
                        for (;;)
                        {
                            if (v1236 < 3u)
                            {
                                v1244 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v1236) + v1147) * v1216, 2.0f)));
                                float3 v1254 = v1210 + (v1214 * v1244);
                                v1240 = v1254.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v1254.xy, v1154).x - 1.0f) * 0.0500000007450580596923828125f);
                                v1242 = (v1240 < 0.0f) ? true : false;
                                if (v1242)
                                {
                                    v1265 = v1240;
                                    v1266 = v1244;
                                    v1267 = v1242;
                                    break;
                                }
                                v1236++;
                                v1239 = v1240;
                                v1241 = v1242;
                                v1243 = v1244;
                                continue;
                            }
                            else
                            {
                                v1265 = 0.0f;
                                v1266 = 1.0f;
                                v1267 = v1241;
                                break;
                            }
                        }
                        v1268 = v1265;
                        v1269 = v1239;
                        v1270 = v1266;
                        v1271 = v1267;
                        v1272 = v1243;
                    }
                    if (v1271)
                    {
                        float v1277;
                        float v1280;
                        float v1282;
                        float v1284;
                        v1277 = v1272;
                        v1280 = v1268;
                        v1282 = v1269;
                        v1284 = v1270;
                        for (uint v1286 = 0u; v1286 < 4u; )
                        {
                            float v1290 = v1277 + v1284;
                            float v1291 = v1290 * 0.5f;
                            float3 v1293 = v1210 + (v1214 * v1291);
                            float v1301 = v1293.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v1293.xy, v1154).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool v1303 = (v1301 < 0.0f) ? true : false;
                            v1277 = v1303 ? v1277 : v1291;
                            v1280 = v1303 ? v1301 : v1280;
                            v1282 = v1303 ? v1282 : v1301;
                            v1284 = v1303 ? v1291 : v1284;
                            v1286++;
                            continue;
                        }
                        float v1307 = lerp(v1277, v1284, v1282 / (v1282 + (-v1280)));
                        float3 v1309 = v1210 + (v1214 * v1307);
                        v1323 = ((abs(v1309.z - ((stone_floor_height_2.SampleLevel(sampler_llr, v1309.xy, v1154).x - 1.0f) * 0.0500000007450580596923828125f)) < (v1282 * 0.89999997615814208984375f)) ? true : false) ? v1307 : v1284;
                        break;
                    }
                    else
                    {
                        v1323 = v1272;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                v1331 = lerp(0.0f, 0.96875f, stone_floor_seed_2.SampleBias(sampler_llr, v914 + (v1214 * v1323).xy, frame_constants_global_mip_bias).x);
            }
            else
            {
                float3 v1010;
                uint2 v926 = uint2(gl_FragCoord.yx);
                float v933 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v943 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v926.x) + v933)) + (0.005837149918079376220703125f * (float(v926.y) + v933))));
                float _946_tmp = stone_floor_height_3.CalculateLevelOfDetail(sampler_llr, v914);
                float2 v946 = _946_tmp.xx;
                float v950 = v946.x + frame_constants_global_mip_bias;
                float3 v951 = ddx(in_var_C2);
                float3 v952 = ddy(in_var_C2);
                float2 v954 = ddx(v914);
                float2 v955 = ddy(v914);
                float2 v956 = -v955;
                float3 v957 = cross(in_var_C1, -v952);
                float3 v958 = cross(v951, in_var_C1);
                float v959 = v954.x;
                float v960 = v956.y;
                float v962 = v954.y;
                float v963 = v956.x;
                float v966 = 1.0f / ((v959 * v960) - (v962 * v963));
                float3x3 v976 = transpose(float3x3(((v957 * v959) + (v958 * v963)) * v966, ((v957 * v962) + (v958 * v960)) * v966, in_var_C1));
                float4 v981 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v987 = mul(v976, float3(1.0f, 0.0f, 0.0f));
                float3 v988 = mul(v976, float3(0.0f, 1.0f, 0.0f));
                float3 v989 = mul(v976, float3(0.0f, 0.0f, 1.0f));
                float3 v992 = cross(v987, v988);
                float3 v995 = (1.0f / dot(v989, v992)).xxx;
                float3 v1000 = mul(float3x3(cross(v988, v989) * v995, cross(v989, v987) * v995, v992 * v995), in_var_C2 - (v981.xyz / v981.w.xxx));
                float3 v1006 = float3(v914, 0.0f);
                float v1119;
                do
                {
                    v1010 = (v1006 + (v1000 * ((-0.0500000007450580596923828125f) / v1000.z))) - v1006;
                    float v1012 = 1.0f / max(1.0f, 2.0f);
                    float v1015 = lerp(0.0f, 1.0f, pow(v943 * v1012, 2.0f));
                    float3 v1017 = v1006 + (v1010 * v1015);
                    float v1025 = v1017.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1017.xy, v950).x - 1.0f) * 0.0500000007450580596923828125f);
                    bool v1027 = (v1025 < 0.0f) ? true : false;
                    float v1064;
                    float v1065;
                    float v1066;
                    bool v1067;
                    float v1068;
                    if (v1027)
                    {
                        v1064 = v1025;
                        v1065 = 0.0f;
                        v1066 = v1015;
                        v1067 = v1027;
                        v1068 = 0.0f;
                    }
                    else
                    {
                        float v1035;
                        float v1039;
                        v1035 = v1025;
                        v1039 = v1015;
                        float v1036;
                        bool v1038;
                        float v1040;
                        float v1061;
                        float v1062;
                        bool v1063;
                        uint v1032 = 1u;
                        bool v1037 = v1027;
                        for (;;)
                        {
                            if (v1032 < 3u)
                            {
                                v1040 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v1032) + v943) * v1012, 2.0f)));
                                float3 v1050 = v1006 + (v1010 * v1040);
                                v1036 = v1050.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1050.xy, v950).x - 1.0f) * 0.0500000007450580596923828125f);
                                v1038 = (v1036 < 0.0f) ? true : false;
                                if (v1038)
                                {
                                    v1061 = v1036;
                                    v1062 = v1040;
                                    v1063 = v1038;
                                    break;
                                }
                                v1032++;
                                v1035 = v1036;
                                v1037 = v1038;
                                v1039 = v1040;
                                continue;
                            }
                            else
                            {
                                v1061 = 0.0f;
                                v1062 = 1.0f;
                                v1063 = v1037;
                                break;
                            }
                        }
                        v1064 = v1061;
                        v1065 = v1035;
                        v1066 = v1062;
                        v1067 = v1063;
                        v1068 = v1039;
                    }
                    if (v1067)
                    {
                        float v1073;
                        float v1076;
                        float v1078;
                        float v1080;
                        v1073 = v1068;
                        v1076 = v1064;
                        v1078 = v1065;
                        v1080 = v1066;
                        for (uint v1082 = 0u; v1082 < 4u; )
                        {
                            float v1086 = v1073 + v1080;
                            float v1087 = v1086 * 0.5f;
                            float3 v1089 = v1006 + (v1010 * v1087);
                            float v1097 = v1089.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1089.xy, v950).x - 1.0f) * 0.0500000007450580596923828125f);
                            bool v1099 = (v1097 < 0.0f) ? true : false;
                            v1073 = v1099 ? v1073 : v1087;
                            v1076 = v1099 ? v1097 : v1076;
                            v1078 = v1099 ? v1078 : v1097;
                            v1080 = v1099 ? v1087 : v1080;
                            v1082++;
                            continue;
                        }
                        float v1103 = lerp(v1073, v1080, v1078 / (v1078 + (-v1076)));
                        float3 v1105 = v1006 + (v1010 * v1103);
                        v1119 = ((abs(v1105.z - ((stone_floor_height_3.SampleLevel(sampler_llr, v1105.xy, v950).x - 1.0f) * 0.0500000007450580596923828125f)) < (v1078 * 0.89999997615814208984375f)) ? true : false) ? v1103 : v1080;
                        break;
                    }
                    else
                    {
                        v1119 = v1068;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                v1331 = lerp(0.0f, 0.96875f, stone_floor_seed_3.SampleBias(sampler_llr, v914 + (v1010 * v1119).xy, frame_constants_global_mip_bias).x);
            }
            v1536 = v1331;
        }
        if ((v1536 * v1536) >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
        {
            discard;
        }
    }
    else
    {
        bool v285;
        if (!(v270 == 1))
        {
            v285 = v270 == 3;
        }
        else
        {
            v285 = true;
        }
        if (v285)
        {
            float2 v288 = in_var_C3 * 0.20000000298023223876953125f;
            float v910;
            if (in_var_C8 == 0u)
            {
                float3 v793;
                uint2 v709 = uint2(gl_FragCoord.yx);
                float v716 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                float v726 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v709.x) + v716)) + (0.005837149918079376220703125f * (float(v709.y) + v716))));
                float _729_tmp = wooden_floor_height.CalculateLevelOfDetail(sampler_llr, v288);
                float2 v729 = _729_tmp.xx;
                float v733 = v729.x + frame_constants_global_mip_bias;
                float3 v734 = ddx(in_var_C2);
                float3 v735 = ddy(in_var_C2);
                float2 v737 = ddx(v288);
                float2 v738 = ddy(v288);
                float2 v739 = -v738;
                float3 v740 = cross(in_var_C1, -v735);
                float3 v741 = cross(v734, in_var_C1);
                float v742 = v737.x;
                float v743 = v739.y;
                float v745 = v737.y;
                float v746 = v739.x;
                float v749 = 1.0f / ((v742 * v743) - (v745 * v746));
                float3x3 v759 = transpose(float3x3(((v740 * v742) + (v741 * v746)) * v749, ((v740 * v745) + (v741 * v743)) * v749, in_var_C1));
                float4 v764 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                float3 v770 = mul(v759, float3(1.0f, 0.0f, 0.0f));
                float3 v771 = mul(v759, float3(0.0f, 1.0f, 0.0f));
                float3 v772 = mul(v759, float3(0.0f, 0.0f, 1.0f));
                float3 v775 = cross(v770, v771);
                float3 v778 = (1.0f / dot(v772, v775)).xxx;
                float3 v783 = mul(float3x3(cross(v771, v772) * v778, cross(v772, v770) * v778, v775 * v778), in_var_C2 - (v764.xyz / v764.w.xxx));
                float3 v789 = float3(v288, 0.0f);
                float v902;
                do
                {
                    v793 = (v789 + (v783 * ((-0.07500000298023223876953125f) / v783.z))) - v789;
                    float v795 = 1.0f / max(1.0f, 2.0f);
                    float v798 = lerp(0.0f, 1.0f, pow(v726 * v795, 2.0f));
                    float3 v800 = v789 + (v793 * v798);
                    float v808 = v800.z - ((wooden_floor_height.SampleLevel(sampler_llr, v800.xy, v733).x - 1.0f) * 0.07500000298023223876953125f);
                    bool v810 = (v808 < 0.0f) ? true : false;
                    float v847;
                    float v848;
                    float v849;
                    bool v850;
                    float v851;
                    if (v810)
                    {
                        v847 = v808;
                        v848 = 0.0f;
                        v849 = v798;
                        v850 = v810;
                        v851 = 0.0f;
                    }
                    else
                    {
                        float v818;
                        float v822;
                        v818 = v808;
                        v822 = v798;
                        float v819;
                        bool v821;
                        float v823;
                        float v844;
                        float v845;
                        bool v846;
                        uint v815 = 1u;
                        bool v820 = v810;
                        for (;;)
                        {
                            if (v815 < 3u)
                            {
                                v823 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v815) + v726) * v795, 2.0f)));
                                float3 v833 = v789 + (v793 * v823);
                                v819 = v833.z - ((wooden_floor_height.SampleLevel(sampler_llr, v833.xy, v733).x - 1.0f) * 0.07500000298023223876953125f);
                                v821 = (v819 < 0.0f) ? true : false;
                                if (v821)
                                {
                                    v844 = v819;
                                    v845 = v823;
                                    v846 = v821;
                                    break;
                                }
                                v815++;
                                v818 = v819;
                                v820 = v821;
                                v822 = v823;
                                continue;
                            }
                            else
                            {
                                v844 = 0.0f;
                                v845 = 1.0f;
                                v846 = v820;
                                break;
                            }
                        }
                        v847 = v844;
                        v848 = v818;
                        v849 = v845;
                        v850 = v846;
                        v851 = v822;
                    }
                    if (v850)
                    {
                        float v856;
                        float v859;
                        float v861;
                        float v863;
                        v856 = v851;
                        v859 = v847;
                        v861 = v848;
                        v863 = v849;
                        for (uint v865 = 0u; v865 < 4u; )
                        {
                            float v869 = v856 + v863;
                            float v870 = v869 * 0.5f;
                            float3 v872 = v789 + (v793 * v870);
                            float v880 = v872.z - ((wooden_floor_height.SampleLevel(sampler_llr, v872.xy, v733).x - 1.0f) * 0.07500000298023223876953125f);
                            bool v882 = (v880 < 0.0f) ? true : false;
                            v856 = v882 ? v856 : v870;
                            v859 = v882 ? v880 : v859;
                            v861 = v882 ? v861 : v880;
                            v863 = v882 ? v870 : v863;
                            v865++;
                            continue;
                        }
                        float v886 = lerp(v856, v863, v861 / (v861 + (-v859)));
                        float3 v888 = v789 + (v793 * v886);
                        v902 = ((abs(v888.z - ((wooden_floor_height.SampleLevel(sampler_llr, v888.xy, v733).x - 1.0f) * 0.07500000298023223876953125f)) < (v861 * 0.89999997615814208984375f)) ? true : false) ? v886 : v863;
                        break;
                    }
                    else
                    {
                        v902 = v851;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                v910 = lerp(0.0f, 0.96875f, wooden_floor_seed.SampleBias(sampler_llr, v288 + (v793 * v902).xy, frame_constants_global_mip_bias).x);
            }
            else
            {
                float v705;
                if (in_var_C8 == 1u)
                {
                    float3 v588;
                    uint2 v504 = uint2(gl_FragCoord.yx);
                    float v511 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float v521 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v504.x) + v511)) + (0.005837149918079376220703125f * (float(v504.y) + v511))));
                    float _524_tmp = wooden_floor_height_2.CalculateLevelOfDetail(sampler_llr, v288);
                    float2 v524 = _524_tmp.xx;
                    float v528 = v524.x + frame_constants_global_mip_bias;
                    float3 v529 = ddx(in_var_C2);
                    float3 v530 = ddy(in_var_C2);
                    float2 v532 = ddx(v288);
                    float2 v533 = ddy(v288);
                    float2 v534 = -v533;
                    float3 v535 = cross(in_var_C1, -v530);
                    float3 v536 = cross(v529, in_var_C1);
                    float v537 = v532.x;
                    float v538 = v534.y;
                    float v540 = v532.y;
                    float v541 = v534.x;
                    float v544 = 1.0f / ((v537 * v538) - (v540 * v541));
                    float3x3 v554 = transpose(float3x3(((v535 * v537) + (v536 * v541)) * v544, ((v535 * v540) + (v536 * v538)) * v544, in_var_C1));
                    float4 v559 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 v565 = mul(v554, float3(1.0f, 0.0f, 0.0f));
                    float3 v566 = mul(v554, float3(0.0f, 1.0f, 0.0f));
                    float3 v567 = mul(v554, float3(0.0f, 0.0f, 1.0f));
                    float3 v570 = cross(v565, v566);
                    float3 v573 = (1.0f / dot(v567, v570)).xxx;
                    float3 v578 = mul(float3x3(cross(v566, v567) * v573, cross(v567, v565) * v573, v570 * v573), in_var_C2 - (v559.xyz / v559.w.xxx));
                    float3 v584 = float3(v288, 0.0f);
                    float v697;
                    do
                    {
                        v588 = (v584 + (v578 * ((-0.07500000298023223876953125f) / v578.z))) - v584;
                        float v590 = 1.0f / max(1.0f, 2.0f);
                        float v593 = lerp(0.0f, 1.0f, pow(v521 * v590, 2.0f));
                        float3 v595 = v584 + (v588 * v593);
                        float v603 = v595.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v595.xy, v528).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v605 = (v603 < 0.0f) ? true : false;
                        float v642;
                        float v643;
                        float v644;
                        bool v645;
                        float v646;
                        if (v605)
                        {
                            v642 = v603;
                            v643 = 0.0f;
                            v644 = v593;
                            v645 = v605;
                            v646 = 0.0f;
                        }
                        else
                        {
                            float v613;
                            float v617;
                            v613 = v603;
                            v617 = v593;
                            float v614;
                            bool v616;
                            float v618;
                            float v639;
                            float v640;
                            bool v641;
                            uint v610 = 1u;
                            bool v615 = v605;
                            for (;;)
                            {
                                if (v610 < 3u)
                                {
                                    v618 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v610) + v521) * v590, 2.0f)));
                                    float3 v628 = v584 + (v588 * v618);
                                    v614 = v628.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v628.xy, v528).x - 1.0f) * 0.07500000298023223876953125f);
                                    v616 = (v614 < 0.0f) ? true : false;
                                    if (v616)
                                    {
                                        v639 = v614;
                                        v640 = v618;
                                        v641 = v616;
                                        break;
                                    }
                                    v610++;
                                    v613 = v614;
                                    v615 = v616;
                                    v617 = v618;
                                    continue;
                                }
                                else
                                {
                                    v639 = 0.0f;
                                    v640 = 1.0f;
                                    v641 = v615;
                                    break;
                                }
                            }
                            v642 = v639;
                            v643 = v613;
                            v644 = v640;
                            v645 = v641;
                            v646 = v617;
                        }
                        if (v645)
                        {
                            float v651;
                            float v654;
                            float v656;
                            float v658;
                            v651 = v646;
                            v654 = v642;
                            v656 = v643;
                            v658 = v644;
                            for (uint v660 = 0u; v660 < 4u; )
                            {
                                float v664 = v651 + v658;
                                float v665 = v664 * 0.5f;
                                float3 v667 = v584 + (v588 * v665);
                                float v675 = v667.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v667.xy, v528).x - 1.0f) * 0.07500000298023223876953125f);
                                bool v677 = (v675 < 0.0f) ? true : false;
                                v651 = v677 ? v651 : v665;
                                v654 = v677 ? v675 : v654;
                                v656 = v677 ? v656 : v675;
                                v658 = v677 ? v665 : v658;
                                v660++;
                                continue;
                            }
                            float v681 = lerp(v651, v658, v656 / (v656 + (-v654)));
                            float3 v683 = v584 + (v588 * v681);
                            v697 = ((abs(v683.z - ((wooden_floor_height_2.SampleLevel(sampler_llr, v683.xy, v528).x - 1.0f) * 0.07500000298023223876953125f)) < (v656 * 0.89999997615814208984375f)) ? true : false) ? v681 : v658;
                            break;
                        }
                        else
                        {
                            v697 = v646;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    v705 = lerp(0.0f, 0.96875f, wooden_floor_seed_2.SampleBias(sampler_llr, v288 + (v588 * v697).xy, frame_constants_global_mip_bias).x);
                }
                else
                {
                    float3 v384;
                    uint2 v300 = uint2(gl_FragCoord.yx);
                    float v307 = 5.588237762451171875f * float(frame_constants_frame_index % 64u);
                    float v317 = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(v300.x) + v307)) + (0.005837149918079376220703125f * (float(v300.y) + v307))));
                    float _320_tmp = wooden_floor_height_3.CalculateLevelOfDetail(sampler_llr, v288);
                    float2 v320 = _320_tmp.xx;
                    float v324 = v320.x + frame_constants_global_mip_bias;
                    float3 v325 = ddx(in_var_C2);
                    float3 v326 = ddy(in_var_C2);
                    float2 v328 = ddx(v288);
                    float2 v329 = ddy(v288);
                    float2 v330 = -v329;
                    float3 v331 = cross(in_var_C1, -v326);
                    float3 v332 = cross(v325, in_var_C1);
                    float v333 = v328.x;
                    float v334 = v330.y;
                    float v336 = v328.y;
                    float v337 = v330.x;
                    float v340 = 1.0f / ((v333 * v334) - (v336 * v337));
                    float3x3 v350 = transpose(float3x3(((v331 * v333) + (v332 * v337)) * v340, ((v331 * v336) + (v332 * v334)) * v340, in_var_C1));
                    float4 v355 = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
                    float3 v361 = mul(v350, float3(1.0f, 0.0f, 0.0f));
                    float3 v362 = mul(v350, float3(0.0f, 1.0f, 0.0f));
                    float3 v363 = mul(v350, float3(0.0f, 0.0f, 1.0f));
                    float3 v366 = cross(v361, v362);
                    float3 v369 = (1.0f / dot(v363, v366)).xxx;
                    float3 v374 = mul(float3x3(cross(v362, v363) * v369, cross(v363, v361) * v369, v366 * v369), in_var_C2 - (v355.xyz / v355.w.xxx));
                    float3 v380 = float3(v288, 0.0f);
                    float v493;
                    do
                    {
                        v384 = (v380 + (v374 * ((-0.07500000298023223876953125f) / v374.z))) - v380;
                        float v386 = 1.0f / max(1.0f, 2.0f);
                        float v389 = lerp(0.0f, 1.0f, pow(v317 * v386, 2.0f));
                        float3 v391 = v380 + (v384 * v389);
                        float v399 = v391.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v391.xy, v324).x - 1.0f) * 0.07500000298023223876953125f);
                        bool v401 = (v399 < 0.0f) ? true : false;
                        float v438;
                        float v439;
                        float v440;
                        bool v441;
                        float v442;
                        if (v401)
                        {
                            v438 = v399;
                            v439 = 0.0f;
                            v440 = v389;
                            v441 = v401;
                            v442 = 0.0f;
                        }
                        else
                        {
                            float v409;
                            float v413;
                            v409 = v399;
                            v413 = v389;
                            float v410;
                            bool v412;
                            float v414;
                            float v435;
                            float v436;
                            bool v437;
                            uint v406 = 1u;
                            bool v411 = v401;
                            for (;;)
                            {
                                if (v406 < 3u)
                                {
                                    v414 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(v406) + v317) * v386, 2.0f)));
                                    float3 v424 = v380 + (v384 * v414);
                                    v410 = v424.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v424.xy, v324).x - 1.0f) * 0.07500000298023223876953125f);
                                    v412 = (v410 < 0.0f) ? true : false;
                                    if (v412)
                                    {
                                        v435 = v410;
                                        v436 = v414;
                                        v437 = v412;
                                        break;
                                    }
                                    v406++;
                                    v409 = v410;
                                    v411 = v412;
                                    v413 = v414;
                                    continue;
                                }
                                else
                                {
                                    v435 = 0.0f;
                                    v436 = 1.0f;
                                    v437 = v411;
                                    break;
                                }
                            }
                            v438 = v435;
                            v439 = v409;
                            v440 = v436;
                            v441 = v437;
                            v442 = v413;
                        }
                        if (v441)
                        {
                            float v447;
                            float v450;
                            float v452;
                            float v454;
                            v447 = v442;
                            v450 = v438;
                            v452 = v439;
                            v454 = v440;
                            for (uint v456 = 0u; v456 < 4u; )
                            {
                                float v460 = v447 + v454;
                                float v461 = v460 * 0.5f;
                                float3 v463 = v380 + (v384 * v461);
                                float v471 = v463.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v463.xy, v324).x - 1.0f) * 0.07500000298023223876953125f);
                                bool v473 = (v471 < 0.0f) ? true : false;
                                v447 = v473 ? v447 : v461;
                                v450 = v473 ? v471 : v450;
                                v452 = v473 ? v452 : v471;
                                v454 = v473 ? v461 : v454;
                                v456++;
                                continue;
                            }
                            float v477 = lerp(v447, v454, v452 / (v452 + (-v450)));
                            float3 v479 = v380 + (v384 * v477);
                            v493 = ((abs(v479.z - ((wooden_floor_height_3.SampleLevel(sampler_llr, v479.xy, v324).x - 1.0f) * 0.07500000298023223876953125f)) < (v452 * 0.89999997615814208984375f)) ? true : false) ? v477 : v454;
                            break;
                        }
                        else
                        {
                            v493 = v442;
                            break;
                        }
                        break; // unreachable workaround
                    } while(false);
                    v705 = lerp(0.0f, 0.96875f, wooden_floor_seed_3.SampleBias(sampler_llr, v288 + (v384 * v493).xy, frame_constants_global_mip_bias).x);
                }
                v910 = v705;
            }
            if (v910 >= nani_instance_data.Load<float>(in_var_C4 * 64 + 32))
            {
                discard;
            }
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
