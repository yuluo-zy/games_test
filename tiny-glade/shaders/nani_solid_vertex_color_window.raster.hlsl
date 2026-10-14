// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// 【文件用途】Nani 窗户实体光栅化。VS 用 3D 值噪声格点(按世界坐标 0.4 缩放,
//   8 角点三线性哈希)给立面顶点加 0.06 幅度抖动,转发半透明标志 + 窗色 id 到 PS;
//   PS 做半透明阴影 dither 剔除,按 color_id 哈希抖动采样窗色板(window_colors),
//   color_id==0 压暗 0.7,打包 RGB8 反照率 + RGB8 法线 到 uint2 SV_Target0。
// 【入口】vs_main/ps_main | 绑定:window_colors t14、instance_indirection_buffer t15、
//   nani_instance_data t16、sampler_llr、$Globals b13、frame_constants b0/space1。
// 【变体】b903+pixel 用提起的完整 ps_main;f0ad+pixel 仅 dither 剔除。
// 重写说明:手写重建段补充中文注释;提起段(TGV_*)逐字保留,仅加块级中文注释。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_solid_vertex_color_window.raster.hlsl | B nani raster | bins: _nani_solid_vertex_color_window.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani window solid: VS jitters the facade vertices with a 3D value-noise
// lattice (per-window hash) and forwards the semitransparent flag + window
// color id; PS dithers the semitransparent holes and samples the window color
// palette.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
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
#include "inc/wall_distortion.hlsl"
#include "inc/window_color_customization.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

int bool_render_shadows;

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

// 3D integer hash (z then y then x avalanche with LCG constants), producing a
// uniform float2 in [-1,1] per lattice corner.
// 3D 整数哈希(LCG 常数,z-y-x 雪崩),对格点角点产生均匀随机
uint window_hash(uint3 p)
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
    return x;
}

// 由哈希产生两个独立 [-1,1] 随机数(位取 mantissa 重解释为 float)
float2 window_corner_rand(uint3 cell)
{
    uint h = window_hash(cell);
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
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    float4 position : SV_Position;
};

// VS:经 indirection 取出实例变换;对世界坐标做 3D 值噪声窗口抖动,再变换到采样空间
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

// 3D 值噪声窗口抖动:8 角点三线性哈希格点
    // 3D value-noise window jitter (8-corner trilinear hash lattice).
    float3 scaled = world * 0.4f;
    int3 cell = int3(floor(scaled));
    float3 fr = frac(scaled);
    float3 f = fr * fr * (3.0f.xxx - (fr * 2.0f));
    float2 n = float2(0.0f, 0.0f);
    n += window_corner_rand(uint3(cell)) * (1.0f - f.x) * (1.0f - f.y) * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(1, 0, 0))) * f.x * (1.0f - f.y) * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(0, 1, 0))) * (1.0f - f.x) * f.y * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(1, 1, 0))) * f.x * f.y * (1.0f - f.z);
    n += window_corner_rand(uint3(cell + int3(0, 0, 1))) * (1.0f - f.x) * (1.0f - f.y) * f.z;
    n += window_corner_rand(uint3(cell + int3(1, 0, 1))) * f.x * (1.0f - f.y) * f.z;
    n += window_corner_rand(uint3(cell + int3(0, 1, 1))) * (1.0f - f.x) * f.y * f.z;
    n += window_corner_rand(uint3(cell + int3(1, 1, 1))) * f.x * f.y * f.z;
    float2 offset = clamp((n - 0.5f.xx) * 2.5f, -0.5f.xx, 0.5f.xx);
// 抖动幅度:随机偏移 (offset.x, 0, offset.y) * 0.06 叠加到世界位置(仅水平面)
    float3 displaced = world + (float3(offset.x, 0.0f, offset.y) * 0.06f);

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(displaced, 1.0f));
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = displaced;
    o.C3 = vin.Vertex_UV;
    o.C4 = nani_instance_data[idx].is_semitransparent;
    o.C5 = nani_instance_data[idx].color_id;
    return o;
}

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float2 C3 : TEXCOORD3;
    nointerpolation uint C4 : TEXCOORD4;
    nointerpolation uint C5 : TEXCOORD5;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// PS:半透明 dither 剔除 + 窗色板采样 + G-Buffer 打包
PsOut ps_main(PsIn pin)
{
    PsOut o;
// 半透明实例在渲染阴影时做 2x2 棋盘 dither(与 vertex_color 版相同)
    bool dither = (pin.C4 & 1u) != 0u && bool_render_shadows != 0;
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

// 以 color_id 为种子做 LCG 雪崩哈希,得到窗色采样的随机抖动
    uint seed = pin.C5;
    uint h = seed + (seed << 10u);
    h = h ^ (h >> 6u);
    h = h + (h << 3u);
    h = h ^ (h >> 11u);
    float rnd = asfloat(((h + (h << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    int color_id = int(pin.C5);
// 采样窗色板:uv 缩放 0.5 + 随机抖动,层 = color_id
    float3 window_color = window_colors.Sample(sampler_llr, float3((pin.C3 * 0.5f) + rnd.xx, float(color_id))).xyz;
// color_id==0 的窗户压暗 0.7
    if (color_id == 0)
    {
        window_color = window_color * 0.7f;
    }

    float3 packed_normal = (normalize(pin.C1) * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(window_color, 0.0f.xxx, 1.0f.xxx));
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_solid_vertex_color_window.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【PS 提起段(b903)】行为精确转译,名字是 SPIR-V id。与手写 ps_main 等价:
//   dither 剔除(v121) -> 由 color_id 哈希出随机抖动(v144) -> 采样窗口色板
//   (v161) -> color_id==0 压暗 0.7(v167) -> 法线/反照率打包(v170/v192 -> v212)。
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

static const uint2 v104[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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

SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> window_colors : register(t14, space0);

static float4 gl_FragCoord;
static float3 in_var_C1;
static float2 in_var_C3;
static uint in_var_C4;
static uint in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] nointerpolation uint in_var_C5 : TEXCOORD5;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    bool v121;
    if ((in_var_C4 & 1u) != 0u)
    {
        v121 = bool_render_shadows != 0;
    }
    else
    {
        v121 = false;
    }
    if (v121)
    {
        uint2 v126 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint v129 = frame_constants_frame_index & 3u;
        if (all(bool2(v126.x != v104[v129].x, v126.y != v104[v129].y)))
        {
            discard;
        }
    }
    uint v138 = in_var_C5 + (in_var_C5 << 10u);
    uint v140 = v138 ^ (v138 >> 6u);
    uint v142 = v140 + (v140 << 3u);
    uint v144 = v142 ^ (v142 >> 11u);
    int v153 = int(in_var_C5);
    float4 v161 = window_colors.Sample(sampler_llr, float3((in_var_C3 * 0.5f) + (asfloat(((v144 + (v144 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xx, float(v153)));
    float3 v162 = v161.xyz;
    float3 v167;
    if (v153 == 0)
    {
        v167 = v162 * 0.699999988079071044921875f;
    }
    else
    {
        v167 = v162;
    }
    float3 v170 = (normalize(in_var_C1) * 0.5f) + 0.5f.xxx;
    float3 v192 = sqrt(clamp(v167, 0.0f.xxx, 1.0f.xxx));
    uint v211 = (uint((clamp(v192.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v192.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v192.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 v212 = uint2(((uint((clamp(v170.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v170.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v170.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, v211);
    v212.y = v211 | 117440512u;
    out_var_SV_TARGET0 = v212;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C1 = stage_input.in_var_C1;
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

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_solid_vertex_color_window.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【PS 提起段(f0ad)】仅做 dither 剔除判定(v86),不写 GBuffer 输出。
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

static const uint2 v75[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

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
static uint in_var_C4;

struct SPIRV_Cross_Input
{
    [[vk::location(4)]] nointerpolation uint in_var_C4 : TEXCOORD4;
    float4 gl_FragCoord : SV_Position;
};

void frag_main()
{
    bool v86;
    if ((in_var_C4 & 1u) != 0u)
    {
        v86 = bool_render_shadows != 0;
    }
    else
    {
        v86 = false;
    }
    if (v86)
    {
        uint2 v91 = uint2(gl_FragCoord.xy) & uint2(1u, 1u);
        uint v94 = frame_constants_frame_index & 3u;
        if (all(bool2(v91.x != v75[v94].x, v91.y != v75[v94].y)))
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C4 = stage_input.in_var_C4;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
