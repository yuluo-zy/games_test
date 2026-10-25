// ============================================================
// grass_lod1.raster.hlsl
// 功能:草叶 LOD1 渲染(远处草叶的简化三角片)。vs_main 用 SV_VertexID 生成
//       3 顶点叶片三角(叶片索引从缓冲区末尾倒序取:(grass_buffer_size-1) - vertex/3),
//       执行与 grass.raster 相同的地形法线/风力/弯曲机制;ps_main 与 grass.raster 像素
//       着色一致,只是丢弃判断的 lerp 使用常量 -0.8(无 C8 插值)。
// 入口:vs_main(SV_VertexID)、ps_main(SV_Target0 打包法线/反照率)。
// 变体:本文件 include grass.raster.hlsl 并定义 GRASS_LOD1,复用其声明与辅助函数并排除
//       LOD0 入口;自身提供 TGV_b903e43ffb3da915 / TGV_f0adff76827e73ff 两个变体的提升段
//       (shader bytecode 转译,见下方 #if defined(TGV_...) 段)。
// 绑定:t17 grass_blade_buffer_data、t15 terrain_heightmap、s9 sampler_default、
//       b13 $Globals(current_view_index/time/terrain_heightmap_dims/visual_terrain_heightmap_dims/
//       grass_buffer_size)、b0 space1 frame_constants。
// ============================================================
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// 说明:以下手写段为语义化版本(非 TGV 变体时编译),与下方提升段逻辑一一对应;
// 本文件 include grass.raster.hlsl 并定义 GRASS_LOD1,复用其声明/辅助函数。

#include "inc/common.hlsl"
#define GRASS_LOD1
#include "grass.raster.hlsl"

struct LOD1_VS_OUT
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C4 : TEXCOORD4;
    float3 C5 : TEXCOORD5;
    float3 C6 : TEXCOORD6;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
    float4 position : SV_Position;
};

// 顶点着色器:按 SV_VertexID 生成 LOD1 叶片三角片(3 顶点,st 决定三角形状)。
LOD1_VS_OUT vs_main(uint vertex_id : SV_VertexID)
{
    LOD1_VS_OUT o;

    float2 st;
    uint tri = vertex_id % 3u;
    if (tri == 0u)
    {
        st = float2(-1.0f, 1.1f);
    }
    else if (tri == 1u)
    {
        st = float2(0.0f, 1.0f);
    }
    else
    {
        st = float2(1.0f, 1.0f);
    }
    float3 blade_geom = float3(st.x * 0.135f, st.y, 0.0f);

    uint blade_index = (grass_buffer_size - 1u) - (vertex_id / 3u);
    GrassBladeDataPacked blade = grass_blade_buffer_data[blade_index];
    float3 blade_pos = blade.position;
    float3 albedo = blade_color_rgb(blade);
    float3 albedo2 = albedo * albedo;
    float2 widths = grass_blade_widths(blade);
    float width_bottom = widths.x;

    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * (1.0f / 130.0f)) + 0.5f, int2(0, 0));
    float2 grad = ((float2((gathered.z + gathered.y) - (gathered.w + gathered.x),
                           (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f)
        / (130.0f / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.2f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.8f) * 0.36f, bend_amt, 1.0f) * 2.5f;

    float dist_scale = max(30.0f / (((-1.0f) / mul(view_constants[0].world_to_view, float4(blade_pos, 1.0f)).z)
        * ((0.5f * view_constants[0].view_to_clip[1].y) * float(clamp(inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale.y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale.y * dist_scale;

    float3 axis = normalize(float3((float((blade.packed0 >> 24u) & 255u) * (2.0f / 255.0f)) - 1.0f, 0.0f, grass_blade_bend_xz(blade)));
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                         float4(0.0f, 1.0f, 0.0f, 0.0f),
                                         float4(axis, 0.0f),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float h_clamped = clamp(st.y, 0.0f, 1.0f);
    float curve = 3.0f * ((0.03f * widths.y) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    blade_geom.z = curve_factor;

    float3 blade_geom2;
    if (glade_theme_id == 4u)
    {
        blade_geom2 = blade_geom;
        blade_geom2.x = st.x * 0.0675f;
    }
    else
    {
        blade_geom2 = blade_geom;
    }

    // 绕叶片轴按打包弯曲角旋转(Rodrigues 旋转,绕 X 轴)。
    float bend_angle = float((blade.packed0 >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = 1.0f;
    float axis_y = 0.0f;
    float axis_z = 0.0f;
    float t_oc_x = oc * axis_x;
    float t_oc_xy = t_oc_x * axis_y;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f),
                                        float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f),
                                        float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 1e-5f * wind_scale, (curve * ((h_clamped + 0.0001f) * (h_clamped + 0.0001f))) - curve_factor)));
    blade_normal.x += 0.3f * st.x;

    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4(blade_scale.x * (float((blade.packed0 >> 8u) & 255u) * (1.0f / 255.0f)) * dist_scale, 0.0f, 0.0f, 0.0f),
                                                          float4(0.0f, wind_scale, 0.0f, 0.0f),
                                                          float4(0.0f, 0.0f, blade_scale.z, 0.0f),
                                                          float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom2, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);

    // Wind value-noise (inline blend grouping).
    float2 wind_uv = (ground_pos.xz * 0.1f) + (float2(time, 0.0f) * 1.2f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f - (2.0f * wind_frac));
    float wind_fx = wind_smooth.x;
    float wind_fy = wind_smooth.y;
    float wind_n00 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 0))));
    float wind_n10 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 0))));
    float wind_n01 = incb_perlin_hash01(hash(uint2(wind_cell + int2(0, 1))));
    float wind_n11 = incb_perlin_hash01(hash(uint2(wind_cell + int2(1, 1))));
    float wind_noise = ((1.0f - wind_fy) * ((wind_n00 * (1.0f - wind_fx)) + (wind_n10 * wind_fx)))
        + ((wind_n01 * (1.0f - wind_fx)) * wind_fy)
        + ((wind_n11 * wind_fx) * wind_fy);
    float wind = (smoothstep(-0.2f, 0.5f, bend_amt) * (st.y * st.y))
        * (sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f * grass_blade_bend_xz(blade);
    rotated_xyz.y -= wind * 0.7f;
    world_offset.x -= wind * 0.25f;

    o.position = mul(view_constants[current_view_index].world_to_sample, world_offset);
    o.C0 = lerp(albedo2 * 0.8f, albedo2, (st.y * st.y).xxx);
    o.C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    o.C3 = world_offset.xyz;
    o.C4 = ground_pos;
    o.C5 = normalize(rotated_xyz);
    o.C6 = axis;
    o.C7 = st;
    o.C8 = 1.0f;
    o.C9 = float3(st * float2(st.y, 1.0f), st.y);
    return o;
}

struct LOD1_PS_IN
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C5 : TEXCOORD5;
    float2 C7 : TEXCOORD7;
    float3 C9 : TEXCOORD9;
};

struct LOD1_PS_OUT
{
    uint2 target : SV_Target0;
};

// 像素着色器:丢弃背面 + 打包法线/反照率输出。
LOD1_PS_OUT ps_main(LOD1_PS_IN i)
{
    LOD1_PS_OUT o;
    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(i.C9.x / i.C9.z) < lerp(-0.8f, 1.0f, i.C7.y))
    {
        discard;
    }
    float3 view_normal;
    if (dot(normalize((cam_pos.xyz / cam_pos.w) - i.C3), i.C5) <= 0.0f)
    {
        view_normal = -i.C5;
    }
    else
    {
        view_normal = i.C5;
    }
    float3 n = normalize(view_normal);
    float3 albedo = (i.C0 * 0.55f) * lerp(0.7f, 1.1f, i.C7.y);
    float3 albedo_out;
    if (glade_theme_id == 4u)
    {
        albedo_out = lerp(0.9f.xxx, albedo * 0.5f, pow(i.C7.y, 0.5f).xxx);
    }
    else
    {
        albedo_out = albedo;
    }
    float3 octa = n / ((abs(n.x) + abs(n.y)) + abs(n.z));
    if (octa.z < 0.0f)
    {
        float2 xy = (1.0f - abs(octa.yx)) * ((step(0.0f, octa.xy) * 2.0f) - 1.0f);
        octa = float3(xy.x, xy.y, octa.z);
    }
    float2 octa_uv = (octa.xy * 0.5f) + 0.5f;
    float3 packed_n = (normalize(i.C2) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo_out, 0.0f, 1.0f));
    o.target = uint2(pack_normal8(packed_n) + (uint(clamp(octa_uv.x, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u),
                     pack_normal8(albedo_sqrt) + (uint(clamp(octa_uv.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u));
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    // 叶片三角:顶点 id 决定 st(0/1/2)。
    uint tri = uint(gl_VertexIndex) % 3u;
    float2 st;
    if (tri == 0u)
    {
        st = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 tri_eq1 = (tri == 1u).xx;
        st = float2(tri_eq1.x ? 0.0f.xx.x : 1.0f.xx.x, tri_eq1.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 blade_geom = float3(st.x * 0.13500000536441802978515625f, st.y, 0.0f);
    // 叶片索引从缓冲区末尾倒序取。
    uint blade_index = (grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 blade_pos = grass_blade_buffer_data.Load<float3>(blade_index * 32 + 0);
    float3 albedo = float3(float(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 albedo2 = albedo * albedo;
    float2 widths = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 20) & 65535u);
    float width_bottom = widths.x;
    // 地形高度 Gather:计算叶片底部地表法线梯度。
    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float g_z = gathered.z;
    float g_y = gathered.y;
    float g_w = gathered.w;
    float g_x = gathered.x;
    float2 grad = ((float2((g_z + g_y) - (g_w + g_x), (g_x + g_y) - (g_w + g_z)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.20000000298023223876953125f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.800000011920928955078125f) * 0.3599999845027923583984375f, bend_amt, 1.0f) * 2.5f;
    // 距离缩放:远距离时保证叶片可见尺寸。
    float dist_scale = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(blade_pos, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float blade_scale_y = blade_scale.y;
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale_y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale_y * dist_scale;
    // 叶片弯曲轴:由打包字节构造。
    float3 axis = float3((float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 24) >> 16u) & 65535u).x);
    float3 axis_n = normalize(axis);
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis_n, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(axis_n, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 相机位置:用于叶片弯曲方向。
    float4 cam_pos = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 叶片曲线:随高度与视线夹角增强。
    float h_clamped = clamp(st.y, 0.0f, 1.0f);
    float curve = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w.xxx) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    float h_clamped_eps = h_clamped + 0.00010001659393310546875f;
    blade_geom.z = curve_factor;
    float3 blade_geom2;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 blade_geom2_theme = blade_geom;
        blade_geom2_theme.x = st.x * 0.067500002682209014892578125f;
        blade_geom2 = blade_geom2_theme;
    }
    else
    {
        blade_geom2 = blade_geom;
    }
    // 绕轴扭转角:构造 Rodrigues 旋转矩阵。
    float bend_angle = float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 twist_axis = normalize(float3(1.0f, 0.0f, 0.0f));
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = twist_axis.x;
    float t_oc_x = oc * axis_x;
    float axis_y = twist_axis.y;
    float t_oc_xy = t_oc_x * axis_y;
    float axis_z = twist_axis.z;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f), float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f), float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 叶片法线:由弯曲切线叉积得到。
    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * wind_scale, (curve * (h_clamped_eps * h_clamped_eps)) - curve_factor)));
    blade_normal.x = blade_normal.x + (0.300000011920928955078125f * st.x);
    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    // 世界偏移:叶片几何经弯曲矩阵变换 + 平移到地面位置。
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4((blade_scale.x * (float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * dist_scale, 0.0f, 0.0f, 0.0f), float4(0.0f, wind_scale, 0.0f, 0.0f), float4(0.0f, 0.0f, blade_scale.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom2, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);
    // 风场:值噪声(内联 hash)风力扰动。
    float2 wind_uv = (ground_pos.xz * 0.100000001490116119384765625f) + (float2(time, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    uint2 cell00 = uint2(wind_cell);
    uint c00_y = cell00.y;
    uint h00_a = c00_y + (c00_y << 10u);
    uint h00_b = h00_a ^ (h00_a >> 6u);
    uint h00_c = h00_b + (h00_b << 3u);
    uint h00_d = h00_c ^ (h00_c >> 11u);
    uint h00_e = (((cell00.x * 1664525u) + (h00_d + (h00_d << 15u))) + 1013904223u) * 1664525u;
    uint h00_f = h00_e ^ (h00_e >> 11u);
    uint h00_g = h00_f ^ ((h00_f << 7u) & 2636928640u);
    uint hash00 = h00_g ^ ((h00_g << 15u) & 4022730752u);
    uint2 cell10 = uint2(wind_cell + int2(1, 0));
    uint c10_y = cell10.y;
    uint h10_a = c10_y + (c10_y << 10u);
    uint h10_b = h10_a ^ (h10_a >> 6u);
    uint h10_c = h10_b + (h10_b << 3u);
    uint h10_d = h10_c ^ (h10_c >> 11u);
    uint h10_e = (((cell10.x * 1664525u) + (h10_d + (h10_d << 15u))) + 1013904223u) * 1664525u;
    uint h10_f = h10_e ^ (h10_e >> 11u);
    uint h10_g = h10_f ^ ((h10_f << 7u) & 2636928640u);
    uint hash10 = h10_g ^ ((h10_g << 15u) & 4022730752u);
    uint2 cell01 = uint2(wind_cell + int2(0, 1));
    uint c01_y = cell01.y;
    uint h01_a = c01_y + (c01_y << 10u);
    uint h01_b = h01_a ^ (h01_a >> 6u);
    uint h01_c = h01_b + (h01_b << 3u);
    uint h01_d = h01_c ^ (h01_c >> 11u);
    uint h01_e = (((cell01.x * 1664525u) + (h01_d + (h01_d << 15u))) + 1013904223u) * 1664525u;
    uint h01_f = h01_e ^ (h01_e >> 11u);
    uint h01_g = h01_f ^ ((h01_f << 7u) & 2636928640u);
    uint hash01 = h01_g ^ ((h01_g << 15u) & 4022730752u);
    uint2 cell11 = uint2(wind_cell + int2(1, 1));
    uint c11_y = cell11.y;
    uint h11_a = c11_y + (c11_y << 10u);
    uint h11_b = h11_a ^ (h11_a >> 6u);
    uint h11_c = h11_b + (h11_b << 3u);
    uint h11_d = h11_c ^ (h11_c >> 11u);
    uint h11_e = (((cell11.x * 1664525u) + (h11_d + (h11_d << 15u))) + 1013904223u) * 1664525u;
    uint h11_f = h11_e ^ (h11_e >> 11u);
    uint h11_g = h11_f ^ ((h11_f << 7u) & 2636928640u);
    uint hash11 = h11_g ^ ((h11_g << 15u) & 4022730752u);
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float wind_fx = wind_smooth.x;
    float wind_inv_fx = 1.0f - wind_fx;
    float wind_fy = wind_smooth.y;
    // 风力合成:高度平滑 + 正弦扰动 + 弯曲系数。
    float st_y_sq = st.y * st.y;
    float wind = (((smoothstep(-0.20000000298023223876953125f, 0.5f, bend_amt) * st_y_sq) * (sin((((((1.0f - wind_fy) * (((asfloat(((hash00 ^ (hash00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_inv_fx) + ((asfloat(((hash10 ^ (hash10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_fx))) + (((asfloat(((hash01 ^ (hash01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_inv_fx) * wind_fy)) + (((asfloat(((hash11 ^ (hash11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_fx) * wind_fy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 24) & 65535u).x;
    rotated_xyz.y = rotated.y - (wind * 0.699999988079071044921875f);
    world_offset.x = world_offset.x - (wind * 0.25f);
    // 输出:顶点位置 + 反照率/法线/几何插值。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, world_offset);
    out_var_C0 = lerp(albedo2 * 0.800000011920928955078125f, albedo2, st_y_sq.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    out_var_C3 = world_offset.xyz;
    out_var_C4 = ground_pos;
    out_var_C5 = normalize(rotated_xyz);
    out_var_C6 = axis;
    out_var_C7 = st;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(st * float2(st.y, 1.0f), st.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
// 提升段:TGV_b903e43ffb3da915 变体,像素着色器(行为等价,语义重命名)。
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
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};


static float3 in_var_C0;
static float3 in_var_C2;
static float3 in_var_C3;
static float3 in_var_C5;
static float2 in_var_C7;
static float3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    // 相机位置:用于背面判定。
    float4 cam_pos = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
    float3 view_normal;
    if (dot(normalize((cam_pos.xyz / cam_pos.w.xxx) - in_var_C3), in_var_C5) <= 0.0f)
    {
        view_normal = -in_var_C5;
    }
    else
    {
        view_normal = in_var_C5;
    }
    float3 n = normalize(view_normal);
    // 反照率:按叶片高度 lerp。
    float3 albedo = (in_var_C0 * 0.550000011920928955078125f) * lerp(0.699999988079071044921875f, 1.10000002384185791015625f, in_var_C7.y);
    float3 albedo_out;
    if (frame_constants_glade_theme_id == 4u)
    {
        albedo_out = lerp(0.89999997615814208984375f.xxx, albedo * 0.5f, pow(in_var_C7.y, 0.5f).xxx);
    }
    else
    {
        albedo_out = albedo;
    }
    // Octahedral 法线编码。
    float3 octa = n / ((abs(n.x) + abs(n.y)) + abs(n.z)).xxx;
    float3 octa_folded;
    if (octa.z < 0.0f)
    {
        float2 xy = (1.0f.xx - abs(octa.yx)) * ((step(0.0f.xx, octa.xy) * 2.0f) - 1.0f.xx);
        octa_folded = float3(xy.x, xy.y, octa.z);
    }
    else
    {
        octa_folded = octa;
    }
    float2 octa_uv = (octa_folded.xy * 0.5f) + 0.5f.xx;
    float3 packed_n = (normalize(in_var_C2) * 0.5f) + 0.5f.xxx;
    // 打包输出:法线与反照率(开方)各 8 位。
    float3 albedo_sqrt = sqrt(clamp(albedo_out, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(packed_n.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_n.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_n.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(octa_uv.x, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), ((uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(octa_uv.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u));
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
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

struct GrassBladeDataPacked
{
    float3 position;
    uint color;
    uint packed0;
    uint packed1;
    uint packed2;
    uint pad;
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
    float time : packoffset(c0.y);
    uint2 terrain_heightmap_dims : packoffset(c0.z);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 out_var_C0;
static float3 out_var_C2;
static float3 out_var_C3;
static float3 out_var_C4;
static float3 out_var_C5;
static float3 out_var_C6;
static float2 out_var_C7;
static float out_var_C8;
static float3 out_var_C9;

struct SPIRV_Cross_Input
{
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float3 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float3 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float3 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] float2 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 out_var_C9 : TEXCOORD9;
    float4 gl_Position : SV_Position;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void vert_main()
{
    // 叶片三角:顶点 id 决定 st(0/1/2)。
    uint tri = uint(gl_VertexIndex) % 3u;
    float2 st;
    if (tri == 0u)
    {
        st = float2(-1.0f, 1.10000002384185791015625f);
    }
    else
    {
        bool2 tri_eq1 = (tri == 1u).xx;
        st = float2(tri_eq1.x ? 0.0f.xx.x : 1.0f.xx.x, tri_eq1.y ? 0.0f.xx.y : 1.0f.xx.y);
    }
    float3 blade_geom = float3(st.x * 0.13500000536441802978515625f, st.y, 0.0f);
    // 叶片索引从缓冲区末尾倒序取。
    uint blade_index = (grass_buffer_size - 1u) - (uint(gl_VertexIndex) / 3u);
    float3 blade_pos = grass_blade_buffer_data.Load<float3>(blade_index * 32 + 0);
    float3 albedo = float3(float(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 albedo2 = albedo * albedo;
    float2 widths = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 20) & 65535u);
    float width_bottom = widths.x;
    // 地形高度 Gather:计算叶片底部地表法线梯度。
    float4 gathered = terrain_heightmap.GatherRed(sampler_default, (blade_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float g_z = gathered.z;
    float g_y = gathered.y;
    float g_w = gathered.w;
    float g_x = gathered.x;
    float2 grad = ((float2((g_z + g_y) - (g_w + g_x), (g_x + g_y) - (g_w + g_z)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(terrain_heightmap_dims));
    float bend_amt = width_bottom * 0.20000000298023223876953125f;
    float3 blade_scale = float3(min(1.0f, width_bottom * 0.800000011920928955078125f) * 0.3599999845027923583984375f, bend_amt, 1.0f) * 2.5f;
    // 距离缩放:远距离时保证叶片可见尺寸。
    float dist_scale = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(blade_pos, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float blade_scale_y = blade_scale.y;
    float bend_offset = blade_pos.y - ((dist_scale - 1.0f) * blade_scale_y);
    float3 ground_pos = blade_pos;
    ground_pos.y = bend_offset;
    float wind_scale = blade_scale_y * dist_scale;
    // 叶片弯曲轴:由打包字节构造。
    float3 axis = float3((float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 24) >> 16u) & 65535u).x);
    float3 axis_n = normalize(axis);
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis_n, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(axis_n, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 相机位置:用于叶片弯曲方向。
    float4 cam_pos = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    // 叶片曲线:随高度与视线夹角增强。
    float h_clamped = clamp(st.y, 0.0f, 1.0f);
    float curve = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w.xxx) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    float h_clamped_eps = h_clamped + 0.00010001659393310546875f;
    blade_geom.z = curve_factor;
    float3 blade_geom2;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 blade_geom2_theme = blade_geom;
        blade_geom2_theme.x = st.x * 0.067500002682209014892578125f;
        blade_geom2 = blade_geom2_theme;
    }
    else
    {
        blade_geom2 = blade_geom;
    }
    // 绕轴扭转角:构造 Rodrigues 旋转矩阵。
    float bend_angle = float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 twist_axis = normalize(float3(1.0f, 0.0f, 0.0f));
    float sina = sin(bend_angle);
    float cosa = cos(bend_angle);
    float oc = 1.0f - cosa;
    float axis_x = twist_axis.x;
    float t_oc_x = oc * axis_x;
    float axis_y = twist_axis.y;
    float t_oc_xy = t_oc_x * axis_y;
    float axis_z = twist_axis.z;
    float t_z_sin = axis_z * sina;
    float t_oc_z = oc * axis_z;
    float t_oc_zx = t_oc_z * axis_x;
    float t_y_sin = axis_y * sina;
    float t_oc_y = oc * axis_y;
    float t_oc_yz = t_oc_y * axis_z;
    float t_x_sin = axis_x * sina;
    float4x4 twist = transpose(float4x4(float4((t_oc_x * axis_x) + cosa, t_oc_xy - t_z_sin, t_oc_zx + t_y_sin, 0.0f), float4(t_oc_xy + t_z_sin, (t_oc_y * axis_y) + cosa, t_oc_yz - t_x_sin, 0.0f), float4(t_oc_zx - t_y_sin, t_oc_yz + t_x_sin, (t_oc_z * axis_z) + cosa, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    // 叶片法线:由弯曲切线叉积得到。
    float3 blade_normal = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * wind_scale, (curve * (h_clamped_eps * h_clamped_eps)) - curve_factor)));
    blade_normal.x = blade_normal.x + (0.300000011920928955078125f * st.x);
    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    // 世界偏移:叶片几何经弯曲矩阵变换 + 平移到地面位置。
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4((blade_scale.x * (float((grass_blade_buffer_data.Load<uint>(blade_index * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * dist_scale, 0.0f, 0.0f, 0.0f), float4(0.0f, wind_scale, 0.0f, 0.0f), float4(0.0f, 0.0f, blade_scale.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom2, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);
    // 风场:值噪声(内联 hash)风力扰动。
    float2 wind_uv = (ground_pos.xz * 0.100000001490116119384765625f) + (float2(time, 0.0f) * 1.2000000476837158203125f);
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    uint2 cell00 = uint2(wind_cell);
    uint c00_y = cell00.y;
    uint h00_a = c00_y + (c00_y << 10u);
    uint h00_b = h00_a ^ (h00_a >> 6u);
    uint h00_c = h00_b + (h00_b << 3u);
    uint h00_d = h00_c ^ (h00_c >> 11u);
    uint h00_e = (((cell00.x * 1664525u) + (h00_d + (h00_d << 15u))) + 1013904223u) * 1664525u;
    uint h00_f = h00_e ^ (h00_e >> 11u);
    uint h00_g = h00_f ^ ((h00_f << 7u) & 2636928640u);
    uint hash00 = h00_g ^ ((h00_g << 15u) & 4022730752u);
    uint2 cell10 = uint2(wind_cell + int2(1, 0));
    uint c10_y = cell10.y;
    uint h10_a = c10_y + (c10_y << 10u);
    uint h10_b = h10_a ^ (h10_a >> 6u);
    uint h10_c = h10_b + (h10_b << 3u);
    uint h10_d = h10_c ^ (h10_c >> 11u);
    uint h10_e = (((cell10.x * 1664525u) + (h10_d + (h10_d << 15u))) + 1013904223u) * 1664525u;
    uint h10_f = h10_e ^ (h10_e >> 11u);
    uint h10_g = h10_f ^ ((h10_f << 7u) & 2636928640u);
    uint hash10 = h10_g ^ ((h10_g << 15u) & 4022730752u);
    uint2 cell01 = uint2(wind_cell + int2(0, 1));
    uint c01_y = cell01.y;
    uint h01_a = c01_y + (c01_y << 10u);
    uint h01_b = h01_a ^ (h01_a >> 6u);
    uint h01_c = h01_b + (h01_b << 3u);
    uint h01_d = h01_c ^ (h01_c >> 11u);
    uint h01_e = (((cell01.x * 1664525u) + (h01_d + (h01_d << 15u))) + 1013904223u) * 1664525u;
    uint h01_f = h01_e ^ (h01_e >> 11u);
    uint h01_g = h01_f ^ ((h01_f << 7u) & 2636928640u);
    uint hash01 = h01_g ^ ((h01_g << 15u) & 4022730752u);
    uint2 cell11 = uint2(wind_cell + int2(1, 1));
    uint c11_y = cell11.y;
    uint h11_a = c11_y + (c11_y << 10u);
    uint h11_b = h11_a ^ (h11_a >> 6u);
    uint h11_c = h11_b + (h11_b << 3u);
    uint h11_d = h11_c ^ (h11_c >> 11u);
    uint h11_e = (((cell11.x * 1664525u) + (h11_d + (h11_d << 15u))) + 1013904223u) * 1664525u;
    uint h11_f = h11_e ^ (h11_e >> 11u);
    uint h11_g = h11_f ^ ((h11_f << 7u) & 2636928640u);
    uint hash11 = h11_g ^ ((h11_g << 15u) & 4022730752u);
    float2 wind_smooth = (wind_frac * wind_frac) * (3.0f.xx - (wind_frac * 2.0f));
    float wind_fx = wind_smooth.x;
    float wind_inv_fx = 1.0f - wind_fx;
    float wind_fy = wind_smooth.y;
    // 风力合成:高度平滑 + 正弦扰动 + 弯曲系数。
    float st_y_sq = st.y * st.y;
    float wind = (((smoothstep(-0.20000000298023223876953125f, 0.5f, bend_amt) * st_y_sq) * (sin((((((1.0f - wind_fy) * (((asfloat(((hash00 ^ (hash00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_inv_fx) + ((asfloat(((hash10 ^ (hash10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_fx))) + (((asfloat(((hash01 ^ (hash01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_inv_fx) * wind_fy)) + (((asfloat(((hash11 ^ (hash11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * wind_fx) * wind_fy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(blade_index * 32 + 24) & 65535u).x;
    rotated_xyz.y = rotated.y - (wind * 0.699999988079071044921875f);
    world_offset.x = world_offset.x - (wind * 0.25f);
    // 输出:顶点位置 + 反照率/法线/几何插值。
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, world_offset);
    out_var_C0 = lerp(albedo2 * 0.800000011920928955078125f, albedo2, st_y_sq.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    out_var_C3 = world_offset.xyz;
    out_var_C4 = ground_pos;
    out_var_C5 = normalize(rotated_xyz);
    out_var_C6 = axis;
    out_var_C7 = st;
    out_var_C8 = 1.0f;
    out_var_C9 = float3(st * float2(st.y, 1.0f), st.y);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
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
// 提升段:TGV_f0adff76827e73ff 变体,像素着色器(仅丢弃判定)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float2 in_var_C7;
static float3 in_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

void frag_main()
{
    if (abs(in_var_C9.x / in_var_C9.z) < lerp(-0.800000011920928955078125f, 1.0f, in_var_C7.y))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C7 = stage_input.in_var_C7;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
