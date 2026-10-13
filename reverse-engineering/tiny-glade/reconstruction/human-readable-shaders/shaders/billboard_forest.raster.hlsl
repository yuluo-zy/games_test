// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 森林公告板渲染:vs_main 按实例号程序化生成森林树木/灌木公告板四边形
//   (黄金角朝向、按距离外扩的圆环排布、4 种云朵尺寸变体、500 之外用 NaN 剔除);
//   ps_main 用 5 点云簇采样合成公告板法线,按主题着色,打包成 uint2 目标。
// 入口: vs_main / ps_main。变体: b903e43ffb3da915 与 f0adff76827e73ff 的 vs/ps 均 lifted。
// 绑定(vs): $Globals b13 (current_view_index/framebuffer_dims/cloud1..4_dims),
//   frame_constants set1/b0。(ps): sampler_lnc s0, cloud1..4 t14..17,
//   color_library b18, frame_constants set1/b0。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/billboard_forest.raster.hlsl
// Evidence: _billboard_forest.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/framebuffer_dims/cloud1..4_dims),
//   frame_constants set1/b0. (ps): sampler_lnc s0, cloud1..4 t14..17, color_library b18,
//   frame_constants set1/b0. vs_main: procedural forest billboard quads (golden-ratio angle,
//   distance ring, 4 cloud-size variants, NaN cull past 500). ps_main: 5-tap cloud-cluster
//   normal from the per-type cloud texture, theme-colored albedo, packed uint2 target.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/simplex_noise.hlsl"

// color_library.hlsl is guard-skipped; flat-member cbuffer at set0/b18.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
cbuffer color_library : register(b18, space0)
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/sh.hlsl"
// NOTE: inc/lighting.hlsl is omitted: it references color_library.indirect_light_tint
// (nested-struct form) which conflicts with this shader's flat-member `color_library`
// cbuffer (embedded target is flat, 37 members). Its functions are unused here; the
// embedded lighting.hlsl OpString is recorded only in the original build (residual).

// $Globals members (implicit cbuffer at set0/b13).
uint2 framebuffer_dims;
uint2 cloud1_dims;
uint2 cloud2_dims;
uint2 cloud3_dims;
uint2 cloud4_dims;

Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

struct VS_OUT
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float4 position : SV_Position;
};

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    // 顶点角点(0..3 四角)与实例种子。
    float vx = float(vertex_id & 1u);
    float vy = float((vertex_id >> 1u) & 1u);
    float2 quad = (float2(vx, vy) - 0.5f) * float2(1.0f, 2.0f);
    uint seed = (instance_id * 12u) + 25u;
    float inst = float(instance_id);

    // 黄金角朝向与圆环排布(距离中心 450~550,基础高度下沉 35~45)。
    float angle = hash_angle(seed);
    float2 facing = -float2(cos(angle), sin(angle));
    float2 facing_perp = float2(facing.y, -facing.x);
    float3 dir = float3(facing.x, 0.0f, facing.y);
    float dist_f = 1.0f - (inst * 0.002f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y -= lerp(35.0f, 45.0f, dist2);

    // 实例哈希链 h1..h4:尺寸随机;类型(0..3)按实例号每 4 个一循环。
    uint h1 = hash(seed);
    uint h2 = hash(h1);
    uint h3 = hash(h2);
    uint h4 = hash(h3);
    float2 size = float2(36.0f, 18.0f) * lerp(0.9f, 1.2f, rand01(h4));
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        // 类型 1:更高更窄。
        size2 = size * 1.2f;
        size2.y *= 0.7f;
    }
    else if (type == 2)
    {
        // 类型 2:整体缩小。
        size2 = size * 0.6f;
    }
    else if (type == 3)
    {
        // 类型 3:高度略减。
        size2 = size * 1.0f;
        size2.y *= 0.9f;
    }
    else
    {
        size2 = size;
    }

    // 位置偏移:水平沿朝向垂直方向铺开,垂直按高度铺开。
    uint h5 = hash(h4);
    uint h6 = hash(h5);
    uint h7 = hash(h6);
    float3 pos = base
        + (float3(facing.y, 0.0f, -facing.x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, rand01(h6))))
        + (float3(0.0f, lerp(1.5f, 2.5f, rand01(h5)), 0.0f) * (size2.y * quad.y));

    // 随机水平翻转 UV 与朝向。
    float2 uv_base = float2(vx, 1.0f - vy);
    float2 uv_out;
    float2 facing_out;
    if (rand01(h7) > 0.5f)
    {
        uv_out = float2(1.0f - vx, uv_base.y);
        facing_out = -facing_perp;
    }
    else
    {
        uv_out = uv_base;
        facing_out = facing_perp;
    }

    // 世界坐标 -> 裁剪空间;500 之后的实例用 NaN 移出屏幕。
    float4 clip = mul(view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    if (instance_id > 500u)
    {
        clip = asfloat(0x7fbfffffu);
    }
    o.position = clip;
    o.C0 = uv_out;
    o.C1 = instance_id;
    o.C2 = pos;
    o.C3 = float3(facing_out.x, 0.0f, facing_out.y);
    o.C4 = dir;
    return o;
}

struct PS_IN
{
    float2 C0 : TEXCOORD0;
    nointerpolation uint C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    // 按实例号确定类型(0..3),决定使用哪张云朵纹理。
    int type = int(frac(float(i.C1) * 0.25f) * 4.0f);
    float s;
    float t;
    float u;
    float v;
    float w;
    if (type == 1)
    {
        // 5 点云簇采样(2x2 子图 + 中心),纹理为 cloud2。
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud2.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud2.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud2.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 2)
    {
        // 5 点采样,纹理为 cloud3。
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud3.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud3.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud3.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else if (type == 3)
    {
        // 5 点采样,纹理为 cloud4。
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud4.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud4.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud4.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else
    {
        // 默认类型:5 点采样,纹理为 cloud1。
        float sx = i.C0.x * 0.3333333432674407958984375f;
        float sy = i.C0.y * 0.3333333432674407958984375f;
        s = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud1.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud1.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud1.Sample(sampler_lnc, float2(sx, sy)).x;
    }

    // 云簇法线:5 点 alpha 的非对称多项式映射。
    float w2 = w * w;
    float v2 = v * v;
    float v3 = (v2 * v2) * v;
    float u2 = u * u;
    float t2 = t * t;
    float s2 = s * s;
    float3 n = normalize(float3(((w2 * w2) * w) - (v3 * v3), (t2 * t2) * t, 1.0f - ((s2 * s2) * s)));
    // alpha 过低丢弃。
    if (smoothstep(0.0f, 0.5f, (u2 * u2) * u) < 0.5f)
    {
        discard;
    }

    // 按主题取树冠颜色(主题 2/4 用树冠色,其余用深绿)。
    float3 albedo;
    if (glade_theme_id == 2u)
    {
        albedo = pow(canopy_color_1.xyz, 1.5f) * 0.9f;
    }
    else if (glade_theme_id == 4u)
    {
        albedo = lerp(pow(canopy_color_1.xyz, 1.5f), 1.0f, 0.5f);
    }
    else
    {
        albedo = float3(0.024f, 0.054f, 0.018f);
    }

    // 公告板世界法线编码到 [0,1];颜色按时刻天空色混合后开平方打包。
    float3 world_n = (normalize((i.C3 * n.x) + (float3(0.0f, 1.0f, 0.0f) * n.y) + (i.C4 * n.z)) * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(lerp(albedo * 0.5f, sky_low_color.xyz * (1.0f - ((smoothstep(0.483f, 0.505f, time_of_day) - smoothstep(0.84f, 0.91f, time_of_day)) * 0.75f)), 0.02f), 0.0f, 1.0f));
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_packed = (uint(clamp(world_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(world_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(world_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xFF000000u;
    // 输出 (法线, 反照率);反照率高位写材质标志 0x46000000。
    o.target = uint2(normal_packed, albedo_packed | 0x46000000u);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _billboard_forest.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// b903 变体的顶点段:程序化森林公告板(vs 语义与手写段一致,变量名已语义化)。
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
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 顶点角点 x/y 与四边形基。
    float vx = float(uint(gl_VertexIndex) & 1u);
    uint vy = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 quad = (float2(vx, float(vy)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    // 实例哈希链:hash(seed)。
    uint seed_hash_a = seed + (seed << 10u);
    uint seed_hash_b = seed_hash_a ^ (seed_hash_a >> 6u);
    uint seed_hash_c = seed_hash_b + (seed_hash_b << 3u);
    uint seed_hash_d = seed_hash_c ^ (seed_hash_c >> 11u);
    uint h1 = seed_hash_d + (seed_hash_d << 15u);
    float inst = float(uint(gl_InstanceIndex));
    uint h1_hash_a = h1 + (h1 << 10u);
    uint h1_hash_b = h1_hash_a ^ (h1_hash_a >> 6u);
    uint h1_hash_c = h1_hash_b + (h1_hash_b << 3u);
    uint h1_hash_d = h1_hash_c ^ (h1_hash_c >> 11u);
    uint h2 = h1_hash_d + (h1_hash_d << 15u);
    uint h2_hash_a = h2 + (h2 << 10u);
    uint h2_hash_b = h2_hash_a ^ (h2_hash_a >> 6u);
    uint h2_hash_c = h2_hash_b + (h2_hash_b << 3u);
    uint h2_hash_d = h2_hash_c ^ (h2_hash_c >> 11u);
    uint h3 = h2_hash_d + (h2_hash_d << 15u);
    uint h3_hash_a = h3 + (h3 << 10u);
    uint h3_hash_b = h3_hash_a ^ (h3_hash_a >> 6u);
    uint h3_hash_c = h3_hash_b + (h3_hash_b << 3u);
    uint h3_hash_d = h3_hash_c ^ (h3_hash_c >> 11u);
    uint h4 = h3_hash_d + (h3_hash_d << 15u);
    // 黄金角朝向与垂直方向。
    float angle = frac(0.89999997615814208984375f + (inst * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 facing = -float2(cos(angle), sin(angle));
    float facing_y = facing.y;
    float facing_x = facing.x;
    float neg_facing_x = -facing_x;
    float2 facing_perp = float2(facing_y, neg_facing_x);
    float3 dir = float3(facing_x, 0.0f, facing_y);
    // 距离圆环与基础高度。
    float dist_f = 1.0f - (inst * 0.00200000009499490261077880859375f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y = base.y - lerp(35.0f, 45.0f, dist2);
    // 哈希链 h5、尺寸随机与类型。
    uint h4_hash_a = h4 + (h4 << 10u);
    uint h4_hash_b = h4_hash_a ^ (h4_hash_a >> 6u);
    uint h4_hash_c = h4_hash_b + (h4_hash_b << 3u);
    uint h4_hash_d = h4_hash_c ^ (h4_hash_c >> 11u);
    uint h5 = h4_hash_d + (h4_hash_d << 15u);
    float2 size = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((h4 & 8388607u) | 1065353216u) - 1.0f);
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        // 类型 1:更高更窄。
        float2 size2_t1 = size * 1.2000000476837158203125f;
        size2_t1.y = size2_t1.y * 0.699999988079071044921875f;
        size2 = size2_t1;
    }
    else
    {
        float2 size2_t2;
        if (type == 2)
        {
            // 类型 2:整体缩小。
            size2_t2 = size * 0.60000002384185791015625f;
        }
        else
        {
            float2 size2_t3;
            if (type == 3)
            {
                // 类型 3:高度略减。
                float2 size2_t3a = size * 1.0f;
                size2_t3a.y = size2_t3a.y * 0.89999997615814208984375f;
                size2_t3 = size2_t3a;
            }
            else
            {
                size2_t3 = size;
            }
            size2_t2 = size2_t3;
        }
        size2 = size2_t2;
    }
    // 哈希链 h6、h7(h7 末步内联)。
    uint h5_hash_a = h5 + (h5 << 10u);
    uint h5_hash_b = h5_hash_a ^ (h5_hash_a >> 6u);
    uint h5_hash_c = h5_hash_b + (h5_hash_b << 3u);
    uint h5_hash_d = h5_hash_c ^ (h5_hash_c >> 11u);
    uint h6 = h5_hash_d + (h5_hash_d << 15u);
    uint h6_hash_a = h6 + (h6 << 10u);
    uint h6_hash_b = h6_hash_a ^ (h6_hash_a >> 6u);
    uint h6_hash_c = h6_hash_b + (h6_hash_b << 3u);
    uint h6_hash_d = h6_hash_c ^ (h6_hash_c >> 11u);
    uint h7_partial = h6_hash_d;
    // 公告板位置:水平沿朝向垂直方向铺开 + 垂直高度。
    float3 pos = base + ((float3(facing_y, 0.0f, neg_facing_x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, asfloat((h6 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((h5 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (size2.y * quad.y)));
    float2 uv_base = float2(vx, float(1u - vy));
    // 随机水平翻转 UV 与朝向。
    float2 facing_out;
    float2 uv_out;
    if ((asfloat(((h7_partial + (h7_partial << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 uv_flip = uv_base;
        uv_flip.x = 1.0f - vx;
        facing_out = -facing_perp;
        uv_out = uv_flip;
    }
    else
    {
        facing_out = facing_perp;
        uv_out = uv_base;
    }
    // 世界坐标 -> 裁剪空间;500 之后的实例置 NaN 移出屏幕。
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    bool4 is_far_instance = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(is_far_instance.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, is_far_instance.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, is_far_instance.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, is_far_instance.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = uv_out;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = pos;
    out_var_C3 = float3(facing_out.x, 0.0f, facing_out.y);
    out_var_C4 = dir;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _billboard_forest.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// b903 变体的片元段:5 点云簇法线 + 主题着色 + uint2 打包(与手写 ps 语义一致)。
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

cbuffer color_library : register(b18, space0)
{
    float4 color_library_temp_color_1 : packoffset(c0);
    float4 color_library_temp_color_2 : packoffset(c1);
    float4 color_library_temp_color_3 : packoffset(c2);
    float4 color_library_roof_color_1 : packoffset(c3);
    float4 color_library_roof_color_2 : packoffset(c4);
    float4 color_library_sky_low_day : packoffset(c5);
    float4 color_library_sky_high_day : packoffset(c6);
    float4 color_library_sun_tint_day : packoffset(c7);
    float4 color_library_sky_low_dusk : packoffset(c8);
    float4 color_library_sky_high_dusk : packoffset(c9);
    float4 color_library_sun_tint_dusk : packoffset(c10);
    float4 color_library_indirect_light_tint : packoffset(c11);
    float4 color_library_fence_color_1 : packoffset(c12);
    float4 color_library_fence_color_2 : packoffset(c13);
    float4 color_library_flower_color_1 : packoffset(c14);
    float4 color_library_flower_color_2 : packoffset(c15);
    float4 color_library_canopy_color_1 : packoffset(c16);
    float4 color_library_canopy_color_2 : packoffset(c17);
    float4 color_library_grass_1 : packoffset(c18);
    float4 color_library_grass_2 : packoffset(c19);
    float4 color_library_path : packoffset(c20);
    float4 color_library_sky_base : packoffset(c21);
    float4 color_library_sky_horizon : packoffset(c22);
    float4 color_library_sky_clouds_1 : packoffset(c23);
    float4 color_library_sky_clouds_2 : packoffset(c24);
    float4 color_library_terrain_1 : packoffset(c25);
    float4 color_library_terrain_2 : packoffset(c26);
    float4 color_library_terrain_flattened : packoffset(c27);
    float4 color_library_wall_1 : packoffset(c28);
    float4 color_library_wall_2 : packoffset(c29);
    float4 color_library_wall_moss : packoffset(c30);
    float4 color_library_wall_bricks : packoffset(c31);
    float4 color_library_ivy_branch : packoffset(c32);
    float4 color_library_ivy_leaf_1 : packoffset(c33);
    float4 color_library_ivy_leaf_2 : packoffset(c34);
    float4 color_library_terrain_highlight : packoffset(c35);
    float4 color_library_night_tint : packoffset(c36);
};

SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;
static float3 in_var_C3;
static float3 in_var_C4;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 in_var_C4 : TEXCOORD4;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 按实例号确定类型(0..3),决定使用哪张云朵纹理。
    int type = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float s;
    float t;
    float u;
    float v;
    float w;
    if (type == 1)
    {
        // 类型 1:cloud2,5 点云簇采样。
        float sx = in_var_C0.x * 0.3333333432674407958984375f;
        float sy = in_var_C0.y * 0.3333333432674407958984375f;
        s = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
        t = cloud2.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
        u = cloud2.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
        v = cloud2.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
        w = cloud2.Sample(sampler_lnc, float2(sx, sy)).x;
    }
    else
    {
        float s_b;
        float t_b;
        float u_b;
        float v_b;
        float w_b;
        if (type == 2)
        {
            // 类型 2:cloud3,5 点云簇采样。
            float sx = in_var_C0.x * 0.3333333432674407958984375f;
            float sy = in_var_C0.y * 0.3333333432674407958984375f;
            s_b = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
            t_b = cloud3.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
            u_b = cloud3.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
            v_b = cloud3.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
            w_b = cloud3.Sample(sampler_lnc, float2(sx, sy)).x;
        }
        else
        {
            float s_c;
            float t_c;
            float u_c;
            float v_c;
            float w_c;
            if (type == 3)
            {
                // 类型 3:cloud4,5 点云簇采样。
                float sx = in_var_C0.x * 0.3333333432674407958984375f;
                float sy = in_var_C0.y * 0.3333333432674407958984375f;
                s_c = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
                t_c = cloud4.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
                u_c = cloud4.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
                v_c = cloud4.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
                w_c = cloud4.Sample(sampler_lnc, float2(sx, sy)).x;
            }
            else
            {
                // 默认类型:cloud1,5 点云簇采样。
                float sx = in_var_C0.x * 0.3333333432674407958984375f;
                float sy = in_var_C0.y * 0.3333333432674407958984375f;
                s_c = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.666666686534881591796875f)).x;
                t_c = cloud1.Sample(sampler_lnc, float2(sx, sy + 0.3333333432674407958984375f)).x;
                u_c = cloud1.Sample(sampler_lnc, float2(sx + 0.666666686534881591796875f, sy)).x;
                v_c = cloud1.Sample(sampler_lnc, float2(sx + 0.3333333432674407958984375f, sy)).x;
                w_c = cloud1.Sample(sampler_lnc, float2(sx, sy)).x;
            }
            s_b = s_c;
            t_b = t_c;
            u_b = u_c;
            v_b = v_c;
            w_b = w_c;
        }
        s = s_b;
        t = t_b;
        u = u_b;
        v = v_b;
        w = w_b;
    }
    // 云簇法线:5 点 alpha 的非对称多项式映射。
    float w2 = w * w;
    float v2 = v * v;
    float v3 = (v2 * v2) * v;
    float u2 = u * u;
    float t2 = t * t;
    float s2 = s * s;
    float3 n = normalize(float3(((w2 * w2) * w) - (v3 * v3), (t2 * t2) * t, 1.0f - ((s2 * s2) * s)));
    // alpha 过低丢弃。
    if (smoothstep(0.0f, 0.5f, (u2 * u2) * u) < 0.5f)
    {
        discard;
    }
    // 按主题取树冠颜色。
    float3 albedo;
    if (frame_constants_glade_theme_id == 2u)
    {
        albedo = pow(color_library_canopy_color_1.xyz, 1.5f.xxx) * 0.89999997615814208984375f;
    }
    else
    {
        float3 albedo_theme4;
        if (frame_constants_glade_theme_id == 4u)
        {
            albedo_theme4 = lerp(pow(color_library_canopy_color_1.xyz, 1.5f.xxx), 1.0f.xxx, 0.5f.xxx);
        }
        else
        {
            albedo_theme4 = float3(0.0240000002086162567138671875f, 0.053999997675418853759765625f, 0.01800000108778476715087890625f);
        }
        albedo = albedo_theme4;
    }
    // 公告板世界法线编码到 [0,1]。
    float3 world_n = (normalize(((in_var_C3 * n.x) + (float3(0.0f, 1.0f, 0.0f) * n.y)) + (in_var_C4 * n.z)) * 0.5f) + 0.5f.xxx;
    // 颜色按时刻天空色混合后开平方打包。
    float3 color = sqrt(clamp(lerp(albedo * 0.5f, frame_constants_sky_low_color.xyz * (1.0f - ((smoothstep(0.4830000102519989013671875f, 0.50499999523162841796875f, frame_constants_time_of_day) - smoothstep(0.839999973773956298828125f, 0.910000026226043701171875f, frame_constants_time_of_day)) * 0.75f)), 0.0199999995529651641845703125f.xxx), 0.0f.xxx, 1.0f.xxx));
    uint albedo_packed = (uint((clamp(color.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    // 目标: x=法线打包 + 0xFF000000, y=反照率打包(后补 0x46000000 材质标志)。
    uint2 target = uint2(((uint((clamp(world_n.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(world_n.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(world_n.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, albedo_packed);
    target.y = albedo_packed | 1174405120u;
    out_var_SV_TARGET0 = target;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
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
// lifted from _billboard_forest.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// f0ad 变体的顶点段:与 b903 变体内容一致(变量名已语义化)。
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
    uint2 framebuffer_dims : packoffset(c0.y);
    uint2 cloud1_dims : packoffset(c1);
    uint2 cloud2_dims : packoffset(c1.z);
    uint2 cloud3_dims : packoffset(c2);
    uint2 cloud4_dims : packoffset(c2.z);
};


static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
static float2 out_var_C0;
static uint out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float2 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    // 顶点角点 x/y 与四边形基。
    float vx = float(uint(gl_VertexIndex) & 1u);
    uint vy = (uint(gl_VertexIndex) >> 1u) & 1u;
    float2 quad = (float2(vx, float(vy)) - 0.5f.xx) * float2(1.0f, 2.0f);
    uint seed = (uint(gl_InstanceIndex) * 12u) + 25u;
    // 实例哈希链:hash(seed)。
    uint seed_hash_a = seed + (seed << 10u);
    uint seed_hash_b = seed_hash_a ^ (seed_hash_a >> 6u);
    uint seed_hash_c = seed_hash_b + (seed_hash_b << 3u);
    uint seed_hash_d = seed_hash_c ^ (seed_hash_c >> 11u);
    uint h1 = seed_hash_d + (seed_hash_d << 15u);
    float inst = float(uint(gl_InstanceIndex));
    uint h1_hash_a = h1 + (h1 << 10u);
    uint h1_hash_b = h1_hash_a ^ (h1_hash_a >> 6u);
    uint h1_hash_c = h1_hash_b + (h1_hash_b << 3u);
    uint h1_hash_d = h1_hash_c ^ (h1_hash_c >> 11u);
    uint h2 = h1_hash_d + (h1_hash_d << 15u);
    uint h2_hash_a = h2 + (h2 << 10u);
    uint h2_hash_b = h2_hash_a ^ (h2_hash_a >> 6u);
    uint h2_hash_c = h2_hash_b + (h2_hash_b << 3u);
    uint h2_hash_d = h2_hash_c ^ (h2_hash_c >> 11u);
    uint h3 = h2_hash_d + (h2_hash_d << 15u);
    uint h3_hash_a = h3 + (h3 << 10u);
    uint h3_hash_b = h3_hash_a ^ (h3_hash_a >> 6u);
    uint h3_hash_c = h3_hash_b + (h3_hash_b << 3u);
    uint h3_hash_d = h3_hash_c ^ (h3_hash_c >> 11u);
    uint h4 = h3_hash_d + (h3_hash_d << 15u);
    // 黄金角朝向与垂直方向。
    float angle = frac(0.89999997615814208984375f + (inst * 1.61803400516510009765625f)) * 6.283185482025146484375f;
    float2 facing = -float2(cos(angle), sin(angle));
    float facing_y = facing.y;
    float facing_x = facing.x;
    float neg_facing_x = -facing_x;
    float2 facing_perp = float2(facing_y, neg_facing_x);
    float3 dir = float3(facing_x, 0.0f, facing_y);
    // 距离圆环与基础高度。
    float dist_f = 1.0f - (inst * 0.00200000009499490261077880859375f);
    float dist2 = dist_f * dist_f;
    float3 base = (-dir) * lerp(450.0f, 550.0f, dist2);
    base.y = base.y - lerp(35.0f, 45.0f, dist2);
    // 哈希链 h5、尺寸随机与类型。
    uint h4_hash_a = h4 + (h4 << 10u);
    uint h4_hash_b = h4_hash_a ^ (h4_hash_a >> 6u);
    uint h4_hash_c = h4_hash_b + (h4_hash_b << 3u);
    uint h4_hash_d = h4_hash_c ^ (h4_hash_c >> 11u);
    uint h5 = h4_hash_d + (h4_hash_d << 15u);
    float2 size = float2(36.0f, 18.0f) * lerp(0.89999997615814208984375f, 1.2000000476837158203125f, asfloat((h4 & 8388607u) | 1065353216u) - 1.0f);
    int type = int(frac(inst * 0.25f) * 4.0f);
    float2 size2;
    if (type == 1)
    {
        // 类型 1:更高更窄。
        float2 size2_t1 = size * 1.2000000476837158203125f;
        size2_t1.y = size2_t1.y * 0.699999988079071044921875f;
        size2 = size2_t1;
    }
    else
    {
        float2 size2_t2;
        if (type == 2)
        {
            // 类型 2:整体缩小。
            size2_t2 = size * 0.60000002384185791015625f;
        }
        else
        {
            float2 size2_t3;
            if (type == 3)
            {
                // 类型 3:高度略减。
                float2 size2_t3a = size * 1.0f;
                size2_t3a.y = size2_t3a.y * 0.89999997615814208984375f;
                size2_t3 = size2_t3a;
            }
            else
            {
                size2_t3 = size;
            }
            size2_t2 = size2_t3;
        }
        size2 = size2_t2;
    }
    // 哈希链 h6、h7(h7 末步内联)。
    uint h5_hash_a = h5 + (h5 << 10u);
    uint h5_hash_b = h5_hash_a ^ (h5_hash_a >> 6u);
    uint h5_hash_c = h5_hash_b + (h5_hash_b << 3u);
    uint h5_hash_d = h5_hash_c ^ (h5_hash_c >> 11u);
    uint h6 = h5_hash_d + (h5_hash_d << 15u);
    uint h6_hash_a = h6 + (h6 << 10u);
    uint h6_hash_b = h6_hash_a ^ (h6_hash_a >> 6u);
    uint h6_hash_c = h6_hash_b + (h6_hash_b << 3u);
    uint h6_hash_d = h6_hash_c ^ (h6_hash_c >> 11u);
    uint h7_partial = h6_hash_d;
    // 公告板位置:水平沿朝向垂直方向铺开 + 垂直高度。
    float3 pos = base + ((float3(facing_y, 0.0f, neg_facing_x) * ((size2.x * quad.x) * lerp(1.5f, 2.0f, asfloat((h6 & 8388607u) | 1065353216u) - 1.0f))) + (float3(0.0f, lerp(1.5f, 2.5f, asfloat((h5 & 8388607u) | 1065353216u) - 1.0f), 0.0f) * (size2.y * quad.y)));
    float2 uv_base = float2(vx, float(1u - vy));
    // 随机水平翻转 UV 与朝向。
    float2 facing_out;
    float2 uv_out;
    if ((asfloat(((h7_partial + (h7_partial << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.5f)
    {
        float2 uv_flip = uv_base;
        uv_flip.x = 1.0f - vx;
        facing_out = -facing_perp;
        uv_out = uv_flip;
    }
    else
    {
        facing_out = facing_perp;
        uv_out = uv_base;
    }
    // 世界坐标 -> 裁剪空间;500 之后的实例置 NaN 移出屏幕。
    float4 clip = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(pos, 1.0f));
    bool4 is_far_instance = (uint(gl_InstanceIndex) > 500u).xxxx;
    gl_Position = float4(is_far_instance.x ? asfloat(0x7fbfffffu /* nan */).xxxx.x : clip.x, is_far_instance.y ? asfloat(0x7fbfffffu /* nan */).xxxx.y : clip.y, is_far_instance.z ? asfloat(0x7fbfffffu /* nan */).xxxx.z : clip.z, is_far_instance.w ? asfloat(0x7fbfffffu /* nan */).xxxx.w : clip.w);
    out_var_C0 = uv_out;
    out_var_C1 = uint(gl_InstanceIndex);
    out_var_C2 = pos;
    out_var_C3 = float3(facing_out.x, 0.0f, facing_out.y);
    out_var_C4 = dir;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _billboard_forest.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// f0ad 变体的片元段:目标 bin 更精简,只采样单点 alpha 并做低 alpha 丢弃。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
SamplerState sampler_lnc : register(s0, space0);
Texture2D<float4> cloud1 : register(t14, space0);
Texture2D<float4> cloud2 : register(t15, space0);
Texture2D<float4> cloud3 : register(t16, space0);
Texture2D<float4> cloud4 : register(t17, space0);

static float2 in_var_C0;
static uint in_var_C1;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float2 in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] nointerpolation uint in_var_C1 : TEXCOORD1;
};

void frag_main()
{
    // 按实例号确定类型,采样对应云朵纹理的 alpha(图集右上子图)。
    int type = int(frac(float(in_var_C1) * 0.25f) * 4.0f);
    float alpha;
    if (type == 1)
    {
        // 类型 1:cloud2。
        alpha = cloud2.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
    }
    else
    {
        float alpha_t2;
        if (type == 2)
        {
            // 类型 2:cloud3。
            alpha_t2 = cloud3.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
        }
        else
        {
            float alpha_t3;
            if (type == 3)
            {
                // 类型 3:cloud4。
                alpha_t3 = cloud4.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            else
            {
                // 默认类型:cloud1。
                alpha_t3 = cloud1.Sample(sampler_lnc, float2((in_var_C0.x * 0.3333333432674407958984375f) + 0.666666686534881591796875f, in_var_C0.y * 0.3333333432674407958984375f)).x;
            }
            alpha_t2 = alpha_t3;
        }
        alpha = alpha_t2;
    }
    // alpha 过低丢弃。
    float alpha_sq = alpha * alpha;
    if (smoothstep(0.0f, 0.5f, (alpha_sq * alpha_sq) * alpha) < 0.5f)
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
