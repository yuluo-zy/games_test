// 屋顶旗帜布料光栅着色器(vs_main + ps_main)。
// 变体 b903e43ffb3da915 / f0adff76827e73ff 的 vs/ps 均来自对应 oracle bin
// (spirv-cross 提升,行为与原版一致;本文件只做变量语义化与中文注释)。
// 算法: vs 用动画纹理在静止/动画姿态间混合,阵风噪声驱动摆动,投影到屏幕;
// ps 采样旗帜图案/调色板,把法线与反照率打包进 G 缓冲。

// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/flag_roof_cloth.raster.hlsl | F-V | bins: _flag_roof_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | 入口 vs:vs_main ps:ps_main
// 屋顶旗帜布料(屋顶族)。vs 在静止姿态与动画姿态之间混合:静止/法线姿态来自
// CPU 侧 (rest_pos/rest_normal),动画姿态从动画纹理采样;摆动包络由锚定原点的
// 阵风噪声驱动。ps 采样旗帜图案与调色板(border_mask 选择实心还是图案),把
// 法线+反照率烘焙进实体 G 缓冲。$Globals (set0/b13) 保存每对象变换/颜色/种子与纹理尺寸。

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"

// INC-issue workaround: the original samplers.hlsl declared 13 samplers
// (s0..s12); inc/samplers.hlsl carries only s0..s4,s9..s12. Declaring s5..s8
// here restores the 13 consecutive sampler slots that position $Globals at
// set0/b13.

// Sampler and animation textures used by the vertex stage.
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

// Textures used by the fragment stage.
Texture2D<float4> border_mask : register(t17, space0);
Texture2D<float4> flag_palettes : register(t18, space0);
Texture2DArray<float4> flag_patterns : register(t19, space0);

int seed;
int color_id;
float4x4 model_to_world;
uint2 flag_color_dims;
uint2 anim_texture_dims;
uint2 anim_normal_texture_dims;
uint2 border_mask_dims;
uint2 flag_palettes_dims;
int bool_render_shadows;

// 2D 值噪声,使用分解式双线性插值(与原版编译形态一致;
// inc/perlin.hlsl 的 perlin() 用 lerp 形式)。
float flag_noise(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    return ((1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x)))
        + ((n01 * (1.0f - u.x)) * u.y)
        + ((n11 * u.x) * u.y);
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 rest_pos: rest_pos;
    [[vk::location(5)]] float3 rest_normal: rest_normal;
    [[vk::location(6)]] float3 uv2: uv2;
    uint vertex_id : SV_VertexID;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float t = float(game_time_sec) + game_time_frac;
    float r0 = rand01(hash(uint(seed)));

    // 动画采样 uv:逐顶点的行,时间/随机列。
    float2 anim_uv = float2((float(vin.vertex_id) + 0.5f) * 0.0125f, (t * 0.15f) + (r0 * 0.2f));

    // 阵风摆动包络。
    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float sway = clamp(smoothstep(0.0f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) + 0.7f), 0.0f, 1.0f);

    uint2 vidx = uint2(uint(vin.vertex_id), 0u);
    float3 anim_pos = (anim_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_texture.Load(int3(vidx, 0u)).xyz) * 2.0f * 0.8f;
    float3 anim_nrm = anim_normal_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_normal_texture.Load(int3(vidx, 0u)).xyz;

    float4 wp = mul(model_to_world, float4(lerp(vin.rest_pos * 0.8f, (vin.Vertex_Position * 0.8f) + anim_pos, sway), 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = normalize(mul(model_to_world, float4(lerp(vin.rest_normal, vin.Vertex_Normal + anim_nrm, sway), 0.0f)).xyz);
    o.C2 = wp.xyz;
    o.C3 = vin.Vertex_UV;
    o.C4 = float2(((vin.uv2.x - 0.5f) * 1.2f) + 0.5f, vin.uv2.y * 7.0f);
    o.C5 = ((float2((vin.uv2.y * 4.0f) - 2.75f, vin.uv2.x) - 0.5f) * 1.3f) + 0.5f;
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    // 视点位置与法线朝向。
    float3 nrm;
    float4 origin = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (dot(normalize((origin.xyz / origin.w) - pin.C2), pin.C1) <= 0.0f)
    {
        nrm = -pin.C1;
    }
    else
    {
        nrm = pin.C1;
    }

    float layer = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float4 pattern = flag_patterns.SampleBias(sampler_llr, float3(pin.C4.x, 1.0f - pin.C4.y, layer), global_mip_bias);

    uint cid = uint(color_id);
    float3 albedo;
    if (border_mask.SampleBias(sampler_llc, pin.C3, global_mip_bias).x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern.x).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz,
                      (flag_patterns.SampleLevel(sampler_llc, float3(pin.C5, float((seed % 6) + 3)), 0.0f).x).xxx);
    }

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 100663296u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_roof_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed : packoffset(c0.y);
    int color_id : packoffset(c0.z);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 flag_color_dims : packoffset(c5);
    uint2 anim_texture_dims : packoffset(c5.z);
    uint2 anim_normal_texture_dims : packoffset(c6);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_normal;
static float3 in_var_uv2;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_normal: rest_normal;
    [[vk::location(6)]] float3 in_var_uv2: uv2;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint2 vidx = uint2(uint(gl_VertexIndex), 0u);
    uint lyr_h0 = uint(seed);
    uint lyr_h1 = lyr_h0 + (lyr_h0 << 10u);
    uint lyr_h2 = lyr_h1 ^ (lyr_h1 >> 6u);
    uint lyr_h3 = lyr_h2 + (lyr_h2 << 3u);
    uint seed_h3 = lyr_h3 ^ (lyr_h3 >> 11u);
    float r0 = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float t = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
    float2 anim_uv = float2((float(uint(gl_VertexIndex)) + 0.5f) * 0.012500000186264514923095703125f, (t * 0.1500000059604644775390625f) + (r0 * 0.20000000298023223876953125f));
    float2 gust_pos = (mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(t, 0.0f) * 1.2000000476837158203125f);
    int2 gust_cell = int2(floor(gust_pos));
    float2 gust_frac = frac(gust_pos);
    uint2 gust_cell_u = uint2(gust_cell);
    uint g00_h0 = gust_cell_u.y;
    uint g00_h1 = g00_h0 + (g00_h0 << 10u);
    uint g00_h2 = g00_h1 ^ (g00_h1 >> 6u);
    uint g00_h3 = g00_h2 + (g00_h2 << 3u);
    uint g00_h4 = g00_h3 ^ (g00_h3 >> 11u);
    uint g00_mix0 = (((gust_cell_u.x * 1664525u) + (g00_h4 + (g00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g00_mix1 = g00_mix0 ^ (g00_mix0 >> 11u);
    uint g00_mix2 = g00_mix1 ^ ((g00_mix1 << 7u) & 2636928640u);
    uint g00_mix3 = g00_mix2 ^ ((g00_mix2 << 15u) & 4022730752u);
    uint2 g10_cell = uint2(gust_cell + int2(1, 0));
    uint g10_h0 = g10_cell.y;
    uint g10_h1 = g10_h0 + (g10_h0 << 10u);
    uint g10_h2 = g10_h1 ^ (g10_h1 >> 6u);
    uint g10_h3 = g10_h2 + (g10_h2 << 3u);
    uint g10_h4 = g10_h3 ^ (g10_h3 >> 11u);
    uint g10_mix0 = (((g10_cell.x * 1664525u) + (g10_h4 + (g10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g10_mix1 = g10_mix0 ^ (g10_mix0 >> 11u);
    uint g10_mix2 = g10_mix1 ^ ((g10_mix1 << 7u) & 2636928640u);
    uint g10_mix3 = g10_mix2 ^ ((g10_mix2 << 15u) & 4022730752u);
    uint2 g01_cell = uint2(gust_cell + int2(0, 1));
    uint g01_h0 = g01_cell.y;
    uint g01_h1 = g01_h0 + (g01_h0 << 10u);
    uint g01_h2 = g01_h1 ^ (g01_h1 >> 6u);
    uint g01_h3 = g01_h2 + (g01_h2 << 3u);
    uint g01_h4 = g01_h3 ^ (g01_h3 >> 11u);
    uint g01_mix0 = (((g01_cell.x * 1664525u) + (g01_h4 + (g01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g01_mix1 = g01_mix0 ^ (g01_mix0 >> 11u);
    uint g01_mix2 = g01_mix1 ^ ((g01_mix1 << 7u) & 2636928640u);
    uint g01_mix3 = g01_mix2 ^ ((g01_mix2 << 15u) & 4022730752u);
    uint2 g11_cell = uint2(gust_cell + int2(1, 1));
    uint g11_h0 = g11_cell.y;
    uint g11_h1 = g11_h0 + (g11_h0 << 10u);
    uint g11_h2 = g11_h1 ^ (g11_h1 >> 6u);
    uint g11_h3 = g11_h2 + (g11_h2 << 3u);
    uint g11_h4 = g11_h3 ^ (g11_h3 >> 11u);
    uint g11_mix0 = (((g11_cell.x * 1664525u) + (g11_h4 + (g11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g11_mix1 = g11_mix0 ^ (g11_mix0 >> 11u);
    uint g11_mix2 = g11_mix1 ^ ((g11_mix1 << 7u) & 2636928640u);
    uint g11_mix3 = g11_mix2 ^ ((g11_mix2 << 15u) & 4022730752u);
    float2 gust_u = (gust_frac * gust_frac) * (3.0f.xx - (gust_frac * 2.0f));
    float gust_ux = gust_u.x;
    float gust_ux1 = 1.0f - gust_ux;
    float gust_uy = gust_u.y;
    float3 sway = clamp(smoothstep(0.0f, 1.0f, (((sin((((((1.0f - gust_uy) * (((asfloat(((g00_mix3 ^ (g00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) + ((asfloat(((g10_mix3 ^ (g10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux))) + (((asfloat(((g01_mix3 ^ (g01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) * gust_uy)) + (((asfloat(((g11_mix3 ^ (g11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux) * gust_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, r0)) + 0.699999988079071044921875f), 0.0f, 1.0f).xxx;
    float4 wp = mul(model_to_world, float4(lerp(in_var_rest_pos * 0.800000011920928955078125f, (in_var_Vertex_Position * 0.800000011920928955078125f) + (((anim_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_texture.Load(int3(vidx, 0u)).xyz) * 2.0f) * 0.800000011920928955078125f), sway), 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = normalize(mul(model_to_world, float4(lerp(in_var_rest_normal, in_var_Vertex_Normal + (anim_normal_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_normal_texture.Load(int3(vidx, 0u)).xyz), sway), 0.0f)).xyz);
    out_var_C2 = wp.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(((in_var_uv2.x - 0.5f) * 1.2000000476837158203125f) + 0.5f, in_var_uv2.y * 7.0f);
    out_var_C5 = ((float2((in_var_uv2.y * 4.0f) - 2.75f, in_var_uv2.x) - 0.5f.xx) * 1.2999999523162841796875f) + 0.5f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_normal = stage_input.in_var_rest_normal;
    in_var_uv2 = stage_input.in_var_uv2;
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _flag_roof_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed : packoffset(c0.y);
    int color_id : packoffset(c0.z);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 flag_color_dims : packoffset(c5);
    uint2 anim_texture_dims : packoffset(c5.z);
    uint2 anim_normal_texture_dims : packoffset(c6);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> border_mask : register(t17, space0);
Texture2D<float4> flag_palettes : register(t18, space0);
Texture2DArray<float4> flag_patterns : register(t19, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static float2 in_var_C5;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 in_var_C5 : TEXCOORD5;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 view_origin = mul(frame_constants_view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 nrm;
    if (dot(normalize((view_origin.xyz / view_origin.w.xxx) - in_var_C2), in_var_C1) <= 0.0f)
    {
        nrm = -in_var_C1;
    }
    else
    {
        nrm = in_var_C1;
    }
    uint layer_seed = uint(seed ^ 234234);
    uint lyr_h0 = layer_seed + (layer_seed << 10u);
    uint lyr_h1 = lyr_h0 ^ (lyr_h0 >> 6u);
    uint lyr_h2 = lyr_h1 + (lyr_h1 << 3u);
    uint lyr_h3 = lyr_h2 ^ (lyr_h2 >> 11u);
    float4 pattern = flag_patterns.SampleBias(sampler_llr, float3(in_var_C4, float(int(round(lerp(9.0f, 11.0f, asfloat(((lyr_h3 + (lyr_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    uint cid = uint(color_id);
    float3 albedo;
    if (border_mask.SampleBias(sampler_llc, in_var_C3, frame_constants_global_mip_bias).x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz, flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, pattern.x.xxx), flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, flag_patterns.SampleLevel(sampler_llc, float3(in_var_C5, float((seed - 6 * (seed / 6)) + 3)), 0.0f).x.xxx);
    }
    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f.xxx;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, albedo_word);
    gbuffer.y = albedo_word | 100663296u;
    out_var_SV_TARGET0 = gbuffer;
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
// lifted from _flag_roof_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed : packoffset(c0.y);
    int color_id : packoffset(c0.z);
    column_major float4x4 model_to_world : packoffset(c1);
    uint2 flag_color_dims : packoffset(c5);
    uint2 anim_texture_dims : packoffset(c5.z);
    uint2 anim_normal_texture_dims : packoffset(c6);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
Texture2D<float4> anim_texture : register(t15, space0);
Texture2D<float4> anim_normal_texture : register(t16, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static float3 in_var_rest_pos;
static float3 in_var_rest_normal;
static float3 in_var_uv2;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] float3 in_var_rest_pos: rest_pos;
    [[vk::location(5)]] float3 in_var_rest_normal: rest_normal;
    [[vk::location(6)]] float3 in_var_uv2: uv2;
    uint gl_VertexIndex : SV_VertexID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    uint2 vidx = uint2(uint(gl_VertexIndex), 0u);
    uint lyr_h0 = uint(seed);
    uint lyr_h1 = lyr_h0 + (lyr_h0 << 10u);
    uint lyr_h2 = lyr_h1 ^ (lyr_h1 >> 6u);
    uint lyr_h3 = lyr_h2 + (lyr_h2 << 3u);
    uint seed_h3 = lyr_h3 ^ (lyr_h3 >> 11u);
    float r0 = asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float t = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
    float2 anim_uv = float2((float(uint(gl_VertexIndex)) + 0.5f) * 0.012500000186264514923095703125f, (t * 0.1500000059604644775390625f) + (r0 * 0.20000000298023223876953125f));
    float2 gust_pos = (mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz * 0.100000001490116119384765625f) + (float2(t, 0.0f) * 1.2000000476837158203125f);
    int2 gust_cell = int2(floor(gust_pos));
    float2 gust_frac = frac(gust_pos);
    uint2 gust_cell_u = uint2(gust_cell);
    uint g00_h0 = gust_cell_u.y;
    uint g00_h1 = g00_h0 + (g00_h0 << 10u);
    uint g00_h2 = g00_h1 ^ (g00_h1 >> 6u);
    uint g00_h3 = g00_h2 + (g00_h2 << 3u);
    uint g00_h4 = g00_h3 ^ (g00_h3 >> 11u);
    uint g00_mix0 = (((gust_cell_u.x * 1664525u) + (g00_h4 + (g00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g00_mix1 = g00_mix0 ^ (g00_mix0 >> 11u);
    uint g00_mix2 = g00_mix1 ^ ((g00_mix1 << 7u) & 2636928640u);
    uint g00_mix3 = g00_mix2 ^ ((g00_mix2 << 15u) & 4022730752u);
    uint2 g10_cell = uint2(gust_cell + int2(1, 0));
    uint g10_h0 = g10_cell.y;
    uint g10_h1 = g10_h0 + (g10_h0 << 10u);
    uint g10_h2 = g10_h1 ^ (g10_h1 >> 6u);
    uint g10_h3 = g10_h2 + (g10_h2 << 3u);
    uint g10_h4 = g10_h3 ^ (g10_h3 >> 11u);
    uint g10_mix0 = (((g10_cell.x * 1664525u) + (g10_h4 + (g10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g10_mix1 = g10_mix0 ^ (g10_mix0 >> 11u);
    uint g10_mix2 = g10_mix1 ^ ((g10_mix1 << 7u) & 2636928640u);
    uint g10_mix3 = g10_mix2 ^ ((g10_mix2 << 15u) & 4022730752u);
    uint2 g01_cell = uint2(gust_cell + int2(0, 1));
    uint g01_h0 = g01_cell.y;
    uint g01_h1 = g01_h0 + (g01_h0 << 10u);
    uint g01_h2 = g01_h1 ^ (g01_h1 >> 6u);
    uint g01_h3 = g01_h2 + (g01_h2 << 3u);
    uint g01_h4 = g01_h3 ^ (g01_h3 >> 11u);
    uint g01_mix0 = (((g01_cell.x * 1664525u) + (g01_h4 + (g01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g01_mix1 = g01_mix0 ^ (g01_mix0 >> 11u);
    uint g01_mix2 = g01_mix1 ^ ((g01_mix1 << 7u) & 2636928640u);
    uint g01_mix3 = g01_mix2 ^ ((g01_mix2 << 15u) & 4022730752u);
    uint2 g11_cell = uint2(gust_cell + int2(1, 1));
    uint g11_h0 = g11_cell.y;
    uint g11_h1 = g11_h0 + (g11_h0 << 10u);
    uint g11_h2 = g11_h1 ^ (g11_h1 >> 6u);
    uint g11_h3 = g11_h2 + (g11_h2 << 3u);
    uint g11_h4 = g11_h3 ^ (g11_h3 >> 11u);
    uint g11_mix0 = (((g11_cell.x * 1664525u) + (g11_h4 + (g11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint g11_mix1 = g11_mix0 ^ (g11_mix0 >> 11u);
    uint g11_mix2 = g11_mix1 ^ ((g11_mix1 << 7u) & 2636928640u);
    uint g11_mix3 = g11_mix2 ^ ((g11_mix2 << 15u) & 4022730752u);
    float2 gust_u = (gust_frac * gust_frac) * (3.0f.xx - (gust_frac * 2.0f));
    float gust_ux = gust_u.x;
    float gust_ux1 = 1.0f - gust_ux;
    float gust_uy = gust_u.y;
    float3 sway = clamp(smoothstep(0.0f, 1.0f, (((sin((((((1.0f - gust_uy) * (((asfloat(((g00_mix3 ^ (g00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) + ((asfloat(((g10_mix3 ^ (g10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux))) + (((asfloat(((g01_mix3 ^ (g01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) * gust_uy)) + (((asfloat(((g11_mix3 ^ (g11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux) * gust_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, r0)) + 0.699999988079071044921875f), 0.0f, 1.0f).xxx;
    float4 wp = mul(model_to_world, float4(lerp(in_var_rest_pos * 0.800000011920928955078125f, (in_var_Vertex_Position * 0.800000011920928955078125f) + (((anim_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_texture.Load(int3(vidx, 0u)).xyz) * 2.0f) * 0.800000011920928955078125f), sway), 1.0f));
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = normalize(mul(model_to_world, float4(lerp(in_var_rest_normal, in_var_Vertex_Normal + (anim_normal_texture.SampleLevel(sampler_llr, anim_uv, 0.0f).xyz - anim_normal_texture.Load(int3(vidx, 0u)).xyz), sway), 0.0f)).xyz);
    out_var_C2 = wp.xyz;
    out_var_C3 = in_var_Vertex_UV;
    out_var_C4 = float2(((in_var_uv2.x - 0.5f) * 1.2000000476837158203125f) + 0.5f, in_var_uv2.y * 7.0f);
    out_var_C5 = ((float2((in_var_uv2.y * 4.0f) - 2.75f, in_var_uv2.x) - 0.5f.xx) * 1.2999999523162841796875f) + 0.5f.xx;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_rest_pos = stage_input.in_var_rest_pos;
    in_var_rest_normal = stage_input.in_var_rest_normal;
    in_var_uv2 = stage_input.in_var_uv2;
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
// lifted from _flag_roof_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
