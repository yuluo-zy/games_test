// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_curved_halftimber.raster.hlsl | B nani raster | bins: _nani_curved_halftimber.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// ============================================================
// Nani 弯曲半木结构着色器。
//   顶点阶段 vs_main:
//     - 按实例控制点生成弯曲/直梁梁体(每实例 64 字节:
//       position/color_id/radius0/radius1/angle0/angle1/is_straight/
//       is_xz/plank_depth/caps/mesh_atlas_part_idx);
//     - 顶点号 %7 取 7 个轮廓槽(side_dirs / edge_offsets / curve_points),
//       按 is_xz 决定是否换轴;直梁走直线插值,曲梁走 cos/sin 圆弧;
//     - 输出法线、世界坐标、caps 标志、z、UV、atlas 片元与段数。
//   像素阶段 ps_main:
//     - 端盖面用偏导构造 TBN 做法线扰动;木面按 C8 高度在端部淡出;
//     - 采样 halftimber_colors 配色、wood_roughness 粗糙度,主题 4 混入
//       泥土小径;打包 G-Buffer(法线 RGB8 + 粗糙度<<24,反照率 RGB8 | 0x07000000)。
//   绑定: instance_indirection_buffer(t14)、nani_instance_data(t15)、
//         wood_normal(t16)、wood_roughness(t17)、wood_edge_damage(t18)、
//         halftimber_colors(t20)、dirthpath_heightmap(t21)、$Globals(b13)。
//   变体: TGV_b903e43ffb3da915(顶点+像素)、TGV_f0adff76827e73ff(顶点,像素为空占位)。
// ============================================================

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "buildings/curved_halftimber.inc.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
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

uint2 wood_normal_dims;
uint2 wood_roughness_dims;
uint2 wood_edge_damage_dims;
uint2 dirthpath_heightmap_dims;

// 弯曲半木实例数据(64 字节)。
struct InstanceData
{
    Affine3Packed xform;
    uint flags;
    float beam_count;
    uint curve_type;
    uint color_id;
    uint wall_id;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> wood_normal : register(t16);
Texture2D<float4> wood_roughness : register(t17);
Texture2D<float4> wood_edge_damage : register(t18);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2D<float4> dirthpath_heightmap : register(t21);

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    uint vertex_id : SV_VertexID;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
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
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C1 = mul(xform, float4(0.0f, 1.0f, 0.0f, 0.0f));
    o.C2 = world;
    o.C4 = idx;
    o.C5 = vin.Vertex_Position.z;
    o.C6 = 1.0f;
    o.C7 = vin.Vertex_Position.xy;
    o.C8 = vin.Vertex_Position.x;
    o.C9 = 0u;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    nointerpolation uint C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    float C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    nointerpolation uint C9 : TEXCOORD9;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    // 木纹 UV:横向 0.4、纵向 0.06。
    float2 uv = pin.C7 * float2(0.4f, 0.06f);
    float3 normal = pin.C1;
    float2 beam_uv = uv;
    float edge = 0.0f;
    // 端盖面(C5<=0.01):用屏幕偏导重建法线,并采样边缘破损。
    if (pin.C5 <= 0.01f)
    {
        normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
        beam_uv = float2(uv.y, uv.x * 2.0f) + 0.87f.xx;
        edge = 1.0f - wood_edge_damage.SampleBias(sampler_llr, pin.C7 * float2(1.0f, 0.05f), global_mip_bias).x;
    }
    int color_id = int(nani_instance_data[pin.C4].color_id);
    float2 sample_uv = beam_uv + (edge * 0.3f);
    float3 albedo = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv.x, sample_uv.y, float(color_id)), global_mip_bias).xyz;
    float2 rough_uv = sample_uv * float2(0.5f, (pin.C8 < 0.0f) ? 3.0f : 2.0f);
    float roughness = lerp(0.7f, 0.9f, 1.0f - wood_roughness.SampleBias(sampler_llr, rough_uv, global_mip_bias).x);
    if (pin.C5 <= 0.01f)
    {
        albedo = lerp(albedo, 0.75f.xxx, (pin.C8 < 0.0f ? 0.2f : 0.0f).xxx);
    }
    float3 normal_map = wood_normal.SampleBias(sampler_llr, sample_uv, global_mip_bias).xyz * 2.0f - 1.0f.xxx;
    float3 shaded_normal = normalize(normal + (normal_map * 0.4f));
    if (glade_theme_id == 4u)
    {
        // 主题 4:沿泥土小径把木面压暗混浅。
        float path = dirthpath_heightmap.SampleBias(sampler_llr, pin.C2.xz * 0.05f, global_mip_bias).x;
        albedo = lerp(albedo, 0.7f.xxx, (((smoothstep(0.0f, 0.2f, smoothstep(0.5f, 1.0f, roughness) * lerp(0.0f, 0.5f, path)) * 0.6f) + (path * 0.2f)) * clamp(shaded_normal.y, 0.0f, 1.0f)).xxx);
    }
    // 打包 G-Buffer:法线压到 [0,1] 并带粗糙度高位,反照率 sqrt 到 sRGB 后 8bit 量化。
    float3 packed_normal = (normalize(shaded_normal) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u);
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + (uint(clamp(roughness, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = o.gbuffer.y | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// 提取自 _nani_curved_halftimber.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 edge_offsets[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float curve_points[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 side_dirs[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

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
    uint2 wood_normal_dims : packoffset(c0.y);
    uint2 wood_roughness_dims : packoffset(c1);
    uint2 wood_edge_damage_dims : packoffset(c1.z);
    uint2 dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint idx;
    float4 clip_pos;
    float3 out_normal;
    float3 out_world;
    float out_caps;
    float out_z;
    float2 out_uv;
    float out_atlas;
    uint out_count;
    do
    {
        idx = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int segments = int(1u << (uint(nani_instance_data.Load<int>(idx * 64 + 48)) & 31u));
        uint v = uint(gl_VertexIndex) % 7u;
        float u = curve_points[v] / curve_points[5];
        bool is_xz_plane = nani_instance_data.Load<int>(idx * 64 + 36) == 0;
        float2 n_side;
        float2 e_side;
        if (is_xz_plane)
        {
            n_side = float2(side_dirs[v].yx);
            e_side = float2(edge_offsets[v].yx);
        }
        else
        {
            n_side = side_dirs[v];
            e_side = edge_offsets[v];
        }
        float inv_segments = 1.0f / float(segments);
        if (nani_instance_data.Load<int>(idx * 64 + 32) > 0)
        {
            float3 dir = float3(nani_instance_data.Load<float>(idx * 64 + 16), 0.0f, nani_instance_data.Load<float>(idx * 64 + 20));
            float end_angle;
            if (nani_instance_data.Load<int>(idx * 64 + 32) == 2)
            {
                float end_angle_2;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    end_angle_2 = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                }
                else
                {
                    end_angle_2 = nani_instance_data.Load<float>(idx * 64 + 28);
                }
                end_angle = end_angle_2;
            }
            else
            {
                float end_angle_3;
                if (nani_instance_data.Load<int>(idx * 64 + 32) == 3)
                {
                    float end_angle_3a;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        end_angle_3a = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                    }
                    else
                    {
                        end_angle_3a = nani_instance_data.Load<float>(idx * 64 + 28);
                    }
                    end_angle_3 = end_angle_3a;
                }
                else
                {
                    float end_angle_4;
                    if (nani_instance_data.Load<int>(idx * 64 + 32) == 4)
                    {
                        end_angle_4 = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                    }
                    else
                    {
                        end_angle_4 = nani_instance_data.Load<float>(idx * 64 + 28);
                    }
                    end_angle_3 = end_angle_4;
                }
                end_angle = end_angle_3;
            }
            float3 straight_pos = (nani_instance_data.Load<float3>(idx * 64 + 0) + ((dir * lerp(-0.5f, 0.5f, e_side.x)) * nani_instance_data.Load<float>(idx * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(idx * 64 + 20), 0.0f, nani_instance_data.Load<float>(idx * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * end_angle);
            uint seed_h1 = idx + (idx << 10u);
            uint seed_h2 = seed_h1 ^ (seed_h1 >> 6u);
            uint seed_h3 = seed_h2 + (seed_h2 << 3u);
            uint seed_h4 = seed_h3 ^ (seed_h3 >> 11u);
            float straight_y = (straight_pos.y + (e_side.y * nani_instance_data.Load<float>(idx * 64 + 40))) + ((asfloat(((seed_h4 + (seed_h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 straight_out = straight_pos;
            straight_out.y = straight_y;
            clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(straight_pos.x, straight_y, straight_pos.z, 1.0f));
            out_normal = (dir * n_side.x) + float3(0.0f, n_side.y, 0.0f);
            out_world = straight_out;
            out_caps = 0.0f;
            out_z = in_var_Vertex_Position.z;
            out_uv = float2(u, in_var_Vertex_Position.z * end_angle);
            out_atlas = -1.0f;
            out_count = uint(segments);
            break;
        }
        else
        {
            float caps_flag;
            float angle;
            if (is_xz_plane)
            {
                caps_flag = 0.0f;
                angle = lerp(nani_instance_data.Load<float>(idx * 64 + 24), nani_instance_data.Load<float>(idx * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float caps_flag_mid;
                float angle_mid;
                if (nani_instance_data.Load<int>(idx * 64 + 44) == 1)
                {
                    float caps_flag_c1;
                    float angle_c1;
                    if (in_var_Vertex_Position.z >= inv_segments)
                    {
                        caps_flag_c1 = 0.0f;
                        angle_c1 = (in_var_Vertex_Position.z - inv_segments) / (1.0f - inv_segments);
                    }
                    else
                    {
                        bool z_lt_inv = in_var_Vertex_Position.z < inv_segments;
                        caps_flag_c1 = float(z_lt_inv);
                        angle_c1 = z_lt_inv ? 0.0f : in_var_Vertex_Position.z;
                    }
                    caps_flag_mid = caps_flag_c1;
                    angle_mid = angle_c1;
                }
                else
                {
                    float caps_flag_c2;
                    float angle_c2;
                    if (nani_instance_data.Load<int>(idx * 64 + 44) == 2)
                    {
                        float end_inv = 1.0f - inv_segments;
                        float caps_flag_c2a;
                        float angle_c2a;
                        if (in_var_Vertex_Position.z <= end_inv)
                        {
                            caps_flag_c2a = 0.0f;
                            angle_c2a = in_var_Vertex_Position.z / end_inv;
                        }
                        else
                        {
                            bool z_gt_end = in_var_Vertex_Position.z > end_inv;
                            caps_flag_c2a = float(z_gt_end);
                            angle_c2a = z_gt_end ? 1.0f : in_var_Vertex_Position.z;
                        }
                        caps_flag_c2 = caps_flag_c2a;
                        angle_c2 = angle_c2a;
                    }
                    else
                    {
                        float caps_flag_c3;
                        float angle_c3;
                        if (nani_instance_data.Load<int>(idx * 64 + 44) == 3)
                        {
                            bool z_mid;
                            if (in_var_Vertex_Position.z >= inv_segments)
                            {
                                z_mid = in_var_Vertex_Position.z <= (1.0f - inv_segments);
                            }
                            else
                            {
                                z_mid = false;
                            }
                            float angle_c3mid;
                            if (z_mid)
                            {
                                angle_c3mid = (in_var_Vertex_Position.z - inv_segments) / ((1.0f - inv_segments) - inv_segments);
                            }
                            else
                            {
                                angle_c3mid = (in_var_Vertex_Position.z < inv_segments) ? 0.0f : 1.0f;
                            }
                            caps_flag_c3 = z_mid ? 0.0f : 1.0f;
                            angle_c3 = angle_c3mid;
                        }
                        else
                        {
                            caps_flag_c3 = 0.0f;
                            angle_c3 = in_var_Vertex_Position.z;
                        }
                        caps_flag_c2 = caps_flag_c3;
                        angle_c2 = angle_c3;
                    }
                    caps_flag_mid = caps_flag_c2;
                    angle_mid = angle_c2;
                }
                caps_flag = caps_flag_mid;
                angle = lerp(nani_instance_data.Load<float>(idx * 64 + 24), nani_instance_data.Load<float>(idx * 64 + 28), angle_mid);
            }
            float cos_a = cos(angle);
            float sin_a = -sin(angle);
            float3 arc_pos = (float3(cos_a, 0.0f, sin_a) * lerp(nani_instance_data.Load<float>(idx * 64 + 16), nani_instance_data.Load<float>(idx * 64 + 20), e_side.x)) + nani_instance_data.Load<float3>(idx * 64 + 0);
            float3 arc_out;
            if (nani_instance_data.Load<int>(idx * 64 + 44) == 0)
            {
                float3 arc_out_c0 = arc_pos;
                arc_out_c0.y = arc_pos.y + (e_side.y * nani_instance_data.Load<float>(idx * 64 + 40));
                arc_out = arc_out_c0;
            }
            else
            {
                float cap_sel;
                if (nani_instance_data.Load<int>(idx * 64 + 44) == 1)
                {
                    cap_sel = ((in_var_Vertex_Position.z >= inv_segments) ? 1.0f : 0.0f);
                }
                else
                {
                    float cap_sel_c2;
                    if (nani_instance_data.Load<int>(idx * 64 + 44) == 2)
                    {
                        cap_sel_c2 = ((in_var_Vertex_Position.z <= (1.0f - inv_segments)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool z_mid2;
                        if (in_var_Vertex_Position.z >= inv_segments)
                        {
                            z_mid2 = in_var_Vertex_Position.z <= (1.0f - inv_segments);
                        }
                        else
                        {
                            z_mid2 = false;
                        }
                        cap_sel_c2 = float(z_mid2);
                    }
                    cap_sel = cap_sel_c2;
                }
                float3 arc_out_c = arc_pos;
                arc_out_c.y = arc_pos.y + ((cap_sel * e_side.y) * nani_instance_data.Load<float>(idx * 64 + 40));
                arc_out = arc_out_c;
            }
            float quantized_y = 0.001953125f * (floor(arc_out.y * 512.0f) + frac(float(idx) * 1.61803400516510009765625f));
            float3 arc_out_q = arc_out;
            arc_out_q.y = quantized_y;
            clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(arc_out.x, quantized_y, arc_out.z, 1.0f));
            out_normal = float3(cos_a, 1.0f, sin_a) * float3(n_side.xyx);
            out_world = arc_out_q;
            out_caps = (uint(gl_VertexIndex) == 6u) ? 1.0f : caps_flag;
            out_z = in_var_Vertex_Position.z;
            out_uv = float2(u, (angle * nani_instance_data.Load<float>(idx * 64 + 16)) * 3.1400001049041748046875f);
            out_atlas = angle * 0.159235656261444091796875f;
            out_count = uint(segments);
            break;
        }
        break; // unreachable workaround
    } while(false);
    gl_Position = clip_pos;
    out_var_C1 = out_normal;
    out_var_C2 = out_world;
    out_var_C4 = idx;
    out_var_C5 = out_caps;
    out_var_C6 = out_z;
    out_var_C7 = out_uv;
    out_var_C8 = out_atlas;
    out_var_C9 = out_count;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
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
// 提取自 _nani_curved_halftimber.raster.hlsl_b903e43ffb3da915.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
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

ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> wood_normal : register(t16, space0);
Texture2D<float4> wood_roughness : register(t17, space0);
Texture2D<float4> wood_edge_damage : register(t18, space0);
Texture2DArray<float4> halftimber_colors : register(t20, space0);
Texture2D<float4> dirthpath_heightmap : register(t21, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static uint in_var_C4;
static float in_var_C5;
static float2 in_var_C7;
static float in_var_C8;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float2 uv = in_var_C7 * float2(0.4000000059604644775390625f, 0.0599999986588954925537109375f);
    float3 normal;
    float2 beam_uv;
    float edge;
    if (in_var_C5 > 0.00999999977648258209228515625f)
    {
        normal = normalize(cross(ddx(in_var_C2), -ddy(in_var_C2)));
        beam_uv = float2(uv.y, uv.x * 2.0f) + 0.87000000476837158203125f.xx;
        edge = 0.0f;
    }
    else
    {
        normal = in_var_C1;
        beam_uv = uv;
        edge = 1.0f - wood_edge_damage.SampleBias(sampler_llr, in_var_C7 * float2(1.0f, 0.0500000007450580596923828125f), frame_constants_global_mip_bias).x;
    }
    int color_id = int(nani_instance_data.Load<uint>(in_var_C4 * 64 + 12));
    float2 sample_uv = beam_uv + (edge.xx * 0.300000011920928955078125f);
    float color_id_f = float(color_id);
    float sample_uv_x = sample_uv.x;
    float4 halftimber_samp = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv_x, sample_uv.y, color_id_f), frame_constants_global_mip_bias);
    float3 albedo = halftimber_samp.xyz;
    float2 rough_uv = sample_uv * float2(0.5f, (in_var_C8 < 0.0f) ? 3.0f : 2.0f);
    float4 rough_samp = wood_roughness.SampleBias(sampler_llr, rough_uv, frame_constants_global_mip_bias);
    float roughness = lerp(0.699999988079071044921875f, 0.89999997615814208984375f, 1.0f - rough_samp.x);
    float3 shaded_normal;
    if (in_var_C5 <= 0.00999999977648258209228515625f)
    {
        float2 d_uv_x = ddx(rough_uv);
        float2 d_uv_y = -ddy(rough_uv);
        float3 cross_n_dy = cross(in_var_C1, -ddy(in_var_C2));
        float3 cross_dx_n = cross(ddx(in_var_C2), in_var_C1);
        float dxdx = d_uv_x.x;
        float dydy = d_uv_y.y;
        float dxdy = d_uv_x.y;
        float dydx = d_uv_y.x;
        float det_inv = 1.0f / ((dxdx * dydy) - (dxdy * dydx));
        float3 normal_map = (wood_normal.SampleBias(sampler_llr, rough_uv, frame_constants_global_mip_bias).xyz * 2.0f) - 1.0f.xxx;
        normal_map.z = normal_map.z * 18.1818180084228515625f;
        shaded_normal = normalize(mul(transpose(float3x3(((cross_n_dy * dxdx) + (cross_dx_n * dydx)) * det_inv, ((cross_n_dy * dxdy) + (cross_dx_n * dydy)) * det_inv, in_var_C1)), normal_map));
    }
    else
    {
        shaded_normal = normal;
    }
    float3 final_normal;
    float final_roughness;
    float3 final_albedo;
    if (in_var_C8 > 0.0f)
    {
        float3 lerp_normal;
        float lerp_roughness;
        float3 lerp_albedo;
        if (in_var_C8 > 0.89999997615814208984375f)
        {
            float fade = smoothstep(0.89999997615814208984375f, 1.0f, in_var_C8);
            float3 fade3 = fade.xxx;
            lerp_normal = lerp(shaded_normal, in_var_C1, fade3);
            lerp_roughness = lerp(roughness, 0.89999997615814208984375f, fade);
            lerp_albedo = lerp(albedo, halftimber_colors.SampleBias(sampler_llr, float3(beam_uv.x, -fade, color_id_f), frame_constants_global_mip_bias).xyz, fade3);
        }
        else
        {
            float3 lerp_normal_lo;
            float lerp_roughness_lo;
            if (in_var_C8 < 0.100000001490116119384765625f)
            {
                float lo_fade = (0.100000001490116119384765625f - in_var_C8) * 10.0f;
                lerp_normal_lo = lerp(shaded_normal, in_var_C1, lo_fade.xxx);
                lerp_roughness_lo = lerp(roughness, 0.89999997615814208984375f, lo_fade);
            }
            else
            {
                lerp_normal_lo = shaded_normal;
                lerp_roughness_lo = roughness;
            }
            lerp_normal = lerp_normal_lo;
            lerp_roughness = lerp_roughness_lo;
            lerp_albedo = albedo;
        }
        final_normal = lerp_normal;
        final_roughness = lerp_roughness;
        final_albedo = lerp_albedo;
    }
    else
    {
        final_normal = shaded_normal;
        final_roughness = roughness;
        final_albedo = albedo;
    }
    float4 samp_extra = halftimber_colors.SampleBias(sampler_llr, float3(sample_uv_x, sample_uv.y, 0.0f), frame_constants_global_mip_bias);
    float3 wood_color = lerp(final_albedo, samp_extra.xyz * ((color_id == 0) ? 2.0f : 1.0f), (edge * 0.4000000059604644775390625f).xxx);
    float3 theme_albedo;
    if (frame_constants_glade_theme_id == 4u)
    {
        float4 path_samp = dirthpath_heightmap.SampleBias(sampler_llr, in_var_C2.xz * 0.0500000007450580596923828125f, frame_constants_global_mip_bias);
        float path = path_samp.x;
        theme_albedo = lerp(wood_color, 0.699999988079071044921875f.xxx, (((smoothstep(0.0f, 0.20000000298023223876953125f, smoothstep(0.5f, 1.0f, final_roughness) * lerp(0.0f, 0.5f, path)) * 0.60000002384185791015625f) + (path * 0.20000000298023223876953125f)) * clamp(final_normal.y, 0.0f, 1.0f)).xxx);
    }
    else
    {
        theme_albedo = wood_color;
    }
    float3 packed_normal = (final_normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(theme_albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(final_roughness, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), albedo_word);
    gbuffer.y = albedo_word | 117440512u;
    out_var_SV_TARGET0 = gbuffer;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
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
// 提取自 _nani_curved_halftimber.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
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

struct CurvedHalftimberInstanceData
{
    float3 position;
    uint color_id;
    float radius0;
    float radius1;
    float angle0;
    float angle1;
    int is_straight;
    int is_xz;
    float plank_depth;
    int caps;
    int mesh_atlas_part_idx;
    int pad0;
    int pad1;
    int pad2;
};

static const float2 edge_offsets[7] = { 0.0f.xx, float2(0.0f, 0.89999997615814208984375f), float2(0.100000001490116119384765625f, 1.0f), float2(0.89999997615814208984375f, 1.0f), float2(1.0f, 0.89999997615814208984375f), float2(1.0f, 0.0f), 0.0f.xx };
static const float curve_points[7] = { 0.0f, 0.89999997615814208984375f, 1.10000002384185791015625f, 1.89999997615814208984375f, 2.099999904632568359375f, 3.0f, 4.0f };
static const float2 side_dirs[7] = { float2(-1.0f, 0.0f), float2(-1.0f, 0.0f), float2(0.0f, 1.0f), float2(0.0f, 1.0f), float2(1.0f, 0.0f), float2(1.0f, 0.0f), float2(0.0f, -1.0f) };

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
    uint2 wood_normal_dims : packoffset(c0.y);
    uint2 wood_roughness_dims : packoffset(c1);
    uint2 wood_edge_damage_dims : packoffset(c1.z);
    uint2 dirthpath_heightmap_dims : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C1;
static float3 out_var_C2;
static uint out_var_C4;
static float out_var_C5;
static float out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static uint out_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(4)]] nointerpolation uint out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] nointerpolation uint out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint idx;
    float4 clip_pos;
    float3 out_normal;
    float3 out_world;
    float out_caps;
    float out_z;
    float2 out_uv;
    float out_atlas;
    uint out_count;
    do
    {
        idx = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
        int segments = int(1u << (uint(nani_instance_data.Load<int>(idx * 64 + 48)) & 31u));
        uint v = uint(gl_VertexIndex) % 7u;
        float u = curve_points[v] / curve_points[5];
        bool is_xz_plane = nani_instance_data.Load<int>(idx * 64 + 36) == 0;
        float2 n_side;
        float2 e_side;
        if (is_xz_plane)
        {
            n_side = float2(side_dirs[v].yx);
            e_side = float2(edge_offsets[v].yx);
        }
        else
        {
            n_side = side_dirs[v];
            e_side = edge_offsets[v];
        }
        float inv_segments = 1.0f / float(segments);
        if (nani_instance_data.Load<int>(idx * 64 + 32) > 0)
        {
            float3 dir = float3(nani_instance_data.Load<float>(idx * 64 + 16), 0.0f, nani_instance_data.Load<float>(idx * 64 + 20));
            float end_angle;
            if (nani_instance_data.Load<int>(idx * 64 + 32) == 2)
            {
                float end_angle_2;
                if (in_var_Vertex_Position.z < 0.5f)
                {
                    end_angle_2 = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                }
                else
                {
                    end_angle_2 = nani_instance_data.Load<float>(idx * 64 + 28);
                }
                end_angle = end_angle_2;
            }
            else
            {
                float end_angle_3;
                if (nani_instance_data.Load<int>(idx * 64 + 32) == 3)
                {
                    float end_angle_3a;
                    if (in_var_Vertex_Position.z > 0.5f)
                    {
                        end_angle_3a = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                    }
                    else
                    {
                        end_angle_3a = nani_instance_data.Load<float>(idx * 64 + 28);
                    }
                    end_angle_3 = end_angle_3a;
                }
                else
                {
                    float end_angle_4;
                    if (nani_instance_data.Load<int>(idx * 64 + 32) == 4)
                    {
                        end_angle_4 = nani_instance_data.Load<float>(idx * 64 + 28) + (nani_instance_data.Load<float>(idx * 64 + 24) * lerp(-1.0f, 1.0f, e_side.x));
                    }
                    else
                    {
                        end_angle_4 = nani_instance_data.Load<float>(idx * 64 + 28);
                    }
                    end_angle_3 = end_angle_4;
                }
                end_angle = end_angle_3;
            }
            float3 straight_pos = (nani_instance_data.Load<float3>(idx * 64 + 0) + ((dir * lerp(-0.5f, 0.5f, e_side.x)) * nani_instance_data.Load<float>(idx * 64 + 24))) + (((-float3(-nani_instance_data.Load<float>(idx * 64 + 20), 0.0f, nani_instance_data.Load<float>(idx * 64 + 16))) * lerp(-0.5f, 0.5f, in_var_Vertex_Position.z)) * end_angle);
            uint seed_h1 = idx + (idx << 10u);
            uint seed_h2 = seed_h1 ^ (seed_h1 >> 6u);
            uint seed_h3 = seed_h2 + (seed_h2 << 3u);
            uint seed_h4 = seed_h3 ^ (seed_h3 >> 11u);
            float straight_y = (straight_pos.y + (e_side.y * nani_instance_data.Load<float>(idx * 64 + 40))) + ((asfloat(((seed_h4 + (seed_h4 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * 0.00999999977648258209228515625f);
            float3 straight_out = straight_pos;
            straight_out.y = straight_y;
            clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(straight_pos.x, straight_y, straight_pos.z, 1.0f));
            out_normal = (dir * n_side.x) + float3(0.0f, n_side.y, 0.0f);
            out_world = straight_out;
            out_caps = 0.0f;
            out_z = in_var_Vertex_Position.z;
            out_uv = float2(u, in_var_Vertex_Position.z * end_angle);
            out_atlas = -1.0f;
            out_count = uint(segments);
            break;
        }
        else
        {
            float caps_flag;
            float angle;
            if (is_xz_plane)
            {
                caps_flag = 0.0f;
                angle = lerp(nani_instance_data.Load<float>(idx * 64 + 24), nani_instance_data.Load<float>(idx * 64 + 28), 1.0f - in_var_Vertex_Position.z);
            }
            else
            {
                float caps_flag_mid;
                float angle_mid;
                if (nani_instance_data.Load<int>(idx * 64 + 44) == 1)
                {
                    float caps_flag_c1;
                    float angle_c1;
                    if (in_var_Vertex_Position.z >= inv_segments)
                    {
                        caps_flag_c1 = 0.0f;
                        angle_c1 = (in_var_Vertex_Position.z - inv_segments) / (1.0f - inv_segments);
                    }
                    else
                    {
                        bool z_lt_inv = in_var_Vertex_Position.z < inv_segments;
                        caps_flag_c1 = float(z_lt_inv);
                        angle_c1 = z_lt_inv ? 0.0f : in_var_Vertex_Position.z;
                    }
                    caps_flag_mid = caps_flag_c1;
                    angle_mid = angle_c1;
                }
                else
                {
                    float caps_flag_c2;
                    float angle_c2;
                    if (nani_instance_data.Load<int>(idx * 64 + 44) == 2)
                    {
                        float end_inv = 1.0f - inv_segments;
                        float caps_flag_c2a;
                        float angle_c2a;
                        if (in_var_Vertex_Position.z <= end_inv)
                        {
                            caps_flag_c2a = 0.0f;
                            angle_c2a = in_var_Vertex_Position.z / end_inv;
                        }
                        else
                        {
                            bool z_gt_end = in_var_Vertex_Position.z > end_inv;
                            caps_flag_c2a = float(z_gt_end);
                            angle_c2a = z_gt_end ? 1.0f : in_var_Vertex_Position.z;
                        }
                        caps_flag_c2 = caps_flag_c2a;
                        angle_c2 = angle_c2a;
                    }
                    else
                    {
                        float caps_flag_c3;
                        float angle_c3;
                        if (nani_instance_data.Load<int>(idx * 64 + 44) == 3)
                        {
                            bool z_mid;
                            if (in_var_Vertex_Position.z >= inv_segments)
                            {
                                z_mid = in_var_Vertex_Position.z <= (1.0f - inv_segments);
                            }
                            else
                            {
                                z_mid = false;
                            }
                            float angle_c3mid;
                            if (z_mid)
                            {
                                angle_c3mid = (in_var_Vertex_Position.z - inv_segments) / ((1.0f - inv_segments) - inv_segments);
                            }
                            else
                            {
                                angle_c3mid = (in_var_Vertex_Position.z < inv_segments) ? 0.0f : 1.0f;
                            }
                            caps_flag_c3 = z_mid ? 0.0f : 1.0f;
                            angle_c3 = angle_c3mid;
                        }
                        else
                        {
                            caps_flag_c3 = 0.0f;
                            angle_c3 = in_var_Vertex_Position.z;
                        }
                        caps_flag_c2 = caps_flag_c3;
                        angle_c2 = angle_c3;
                    }
                    caps_flag_mid = caps_flag_c2;
                    angle_mid = angle_c2;
                }
                caps_flag = caps_flag_mid;
                angle = lerp(nani_instance_data.Load<float>(idx * 64 + 24), nani_instance_data.Load<float>(idx * 64 + 28), angle_mid);
            }
            float cos_a = cos(angle);
            float sin_a = -sin(angle);
            float3 arc_pos = (float3(cos_a, 0.0f, sin_a) * lerp(nani_instance_data.Load<float>(idx * 64 + 16), nani_instance_data.Load<float>(idx * 64 + 20), e_side.x)) + nani_instance_data.Load<float3>(idx * 64 + 0);
            float3 arc_out;
            if (nani_instance_data.Load<int>(idx * 64 + 44) == 0)
            {
                float3 arc_out_c0 = arc_pos;
                arc_out_c0.y = arc_pos.y + (e_side.y * nani_instance_data.Load<float>(idx * 64 + 40));
                arc_out = arc_out_c0;
            }
            else
            {
                float cap_sel;
                if (nani_instance_data.Load<int>(idx * 64 + 44) == 1)
                {
                    cap_sel = ((in_var_Vertex_Position.z >= inv_segments) ? 1.0f : 0.0f);
                }
                else
                {
                    float cap_sel_c2;
                    if (nani_instance_data.Load<int>(idx * 64 + 44) == 2)
                    {
                        cap_sel_c2 = ((in_var_Vertex_Position.z <= (1.0f - inv_segments)) ? 1.0f : 0.0f);
                    }
                    else
                    {
                        bool z_mid2;
                        if (in_var_Vertex_Position.z >= inv_segments)
                        {
                            z_mid2 = in_var_Vertex_Position.z <= (1.0f - inv_segments);
                        }
                        else
                        {
                            z_mid2 = false;
                        }
                        cap_sel_c2 = float(z_mid2);
                    }
                    cap_sel = cap_sel_c2;
                }
                float3 arc_out_c = arc_pos;
                arc_out_c.y = arc_pos.y + ((cap_sel * e_side.y) * nani_instance_data.Load<float>(idx * 64 + 40));
                arc_out = arc_out_c;
            }
            float quantized_y = 0.001953125f * (floor(arc_out.y * 512.0f) + frac(float(idx) * 1.61803400516510009765625f));
            float3 arc_out_q = arc_out;
            arc_out_q.y = quantized_y;
            clip_pos = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(arc_out.x, quantized_y, arc_out.z, 1.0f));
            out_normal = float3(cos_a, 1.0f, sin_a) * float3(n_side.xyx);
            out_world = arc_out_q;
            out_caps = (uint(gl_VertexIndex) == 6u) ? 1.0f : caps_flag;
            out_z = in_var_Vertex_Position.z;
            out_uv = float2(u, (angle * nani_instance_data.Load<float>(idx * 64 + 16)) * 3.1400001049041748046875f);
            out_atlas = angle * 0.159235656261444091796875f;
            out_count = uint(segments);
            break;
        }
        break; // unreachable workaround
    } while(false);
    gl_Position = clip_pos;
    out_var_C1 = out_normal;
    out_var_C2 = out_world;
    out_var_C4 = idx;
    out_var_C5 = out_caps;
    out_var_C6 = out_z;
    out_var_C7 = out_uv;
    out_var_C8 = out_atlas;
    out_var_C9 = out_count;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
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
// 提取自 _nani_curved_halftimber.raster.hlsl_f0adff76827e73ff.bin(spirv-cross;行为精确,名称即 SPIR-V 编号)
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
