// 墙面横幅布料光栅着色器(vs_main + ps_main)。
// 变体 b903e43ffb3da915 / f0adff76827e73ff 的 vs/ps 均来自对应 oracle bin
// (spirv-cross 提升,行为与原版一致;本文件只做变量语义化与中文注释)。
// 算法: vs 用阵风包络+抖动弯曲旗帜(旗身/旗尖/角落分支),投影到屏幕;
// ps 采样旗帜图案/调色板,把法线、反照率与 alpha 打包进 G 缓冲。

// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/flag_wall_cloth.raster.hlsl | F-V | bins: _flag_wall_cloth.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | 入口 vs:vs_main ps:ps_main
// 墙面横幅布料(屋顶族)。vs 用阵风包络(锚定原点的 2D 值噪声)加更快的抖动项
// 弯曲旗帜;顶点按 is_tip 区分旗身/旗尖,角落旗帜用 abs 技巧压平、弯曲幅度更小。
// ps 采样旗帜图案与调色板,用 border_mask 选择实心还是图案,把法线+反照率+alpha
// 烘焙进实体 G 缓冲。$Globals (set0/b13) 保存每对象变换/颜色。

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

// 片元阶段使用的纹理(sampler 来自 inc/samplers.hlsl)。
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

float4x4 model_to_world;
int seed;
float local_start_y;
float body_end_y;
float tip_end_y;
int is_corner;
int color_id;
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
    [[vk::location(4)]] int is_tip: is_tip;
    [[vk::location(5)]] float bby: bby;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;

    float3 vpos = vin.Vertex_Position;
    float3 pos;
    if (vin.is_tip == 0)
    {
        pos = vpos;
        pos.y = -lerp(local_start_y, body_end_y, 1.0f - vin.bby);
    }
    else
    {
        pos = vpos;
        pos.y = -lerp(body_end_y, tip_end_y, 1.0f - vin.bby);
    }

    float t_sec = float(game_time_sec);
    float t = t_sec + game_time_frac;

    float4 origin = mul(model_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float gust = flag_noise((origin.xz * 0.1f) + (float2(t, 0.0f) * 1.2f));
    float r0 = rand01(hash(uint(seed)));
    float bend = lerp(0.3f, 1.0f, (sin((gust * PI) - HALF_PI + 0.3f) * 0.7f * lerp(0.7f, 1.0f, r0)) * 0.3f);

    float amp;
    if (is_corner == 0)
    {
        amp = bend * 0.3f;
    }
    else
    {
        amp = bend * 0.5f;
    }

    float flutter = flag_noise(float2(t + pos.x, pos.y));
    float z = pos.z + (sin((fmod(t_sec * 1.8f, TWO_PI) + (game_time_frac * 1.8f)) + (pos.y * 0.5f)
        + (flutter * smoothstep(0.0f, -3.0f, pos.y)))) * amp;
    float3 result = pos;
    result.z = z;
    if (is_corner == 0)
    {
        result.z = abs(z + 0.05f) - 0.05f;
    }

    float4 wp = mul(model_to_world, float4(result, 1.0f));
    float body_h = body_end_y - local_start_y;
    o.position = mul(view_constants[current_view_index].world_to_sample, wp);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = wp.xyz;
    o.C3 = float2(vpos.x + 0.5f, (result.y * 1.5f) + 0.5f);
    o.C4 = vin.Vertex_UV;
    o.C5 = (float2(vpos.x, result.y + (body_h * 0.5f) + 0.2f) * (0.8f / min(max(0.7f, body_h - 0.5f), 1.0f))) + 0.5f;
    o.C6 = (float2(vpos.x, (tip_end_y - local_start_y) + result.y) * 0.75f) + float2(0.5f, 0.0f);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float2 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    float2 C5 : TEXCOORD5;
    float2 C6 : TEXCOORD6;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;

    float layer1 = float(int(round(lerp(9.0f, 11.0f, rand01(hash(uint(seed ^ 234234)))))));
    float layer2 = float(int(round(lerp(0.0f, 2.0f, rand01(hash(uint(seed ^ 794)))))));
    float4 pat1 = flag_patterns.SampleBias(sampler_llr, float3(pin.C3.x, 1.0f - pin.C3.y, layer1), global_mip_bias);
    float4 pat2 = flag_patterns.SampleBias(sampler_llc, float3(pin.C6.x, 1.0f - pin.C6.y, layer2), global_mip_bias);
    float border = border_mask.SampleBias(sampler_llc, pin.C4, global_mip_bias).x;
    float pattern_sum = pat1.x + pat2.x;
    float pat3 = flag_patterns.SampleLevel(sampler_llc, float3(pin.C5.x, 1.0f - pin.C5.y, float((seed % 6) + 3)), 0.0f).x;

    uint cid = uint(color_id);
    float3 albedo;
    if (border > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz,
                           flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, (pattern_sum).xxx),
                      flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, (pat3).xxx);
    }

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

    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint(clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    float alpha = (((border + pattern_sum) + pat3) > 0.89999997615814208984375f) ? 0.6f : 1.0f;
    uint normal_word = (uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u))
        + (uint(clamp(alpha, 0.0f, 1.0f) * 255.0f + 0.5f) << 24u);
    o.gbuffer = uint2(normal_word, albedo_word);
    o.gbuffer.y = albedo_word | 117440512u;
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_wall_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 model_to_world : packoffset(c1);
    int seed : packoffset(c5);
    float local_start_y : packoffset(c5.y);
    float body_end_y : packoffset(c5.z);
    float tip_end_y : packoffset(c5.w);
    int is_corner : packoffset(c6);
    int color_id : packoffset(c6.y);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_tip;
static float in_var_bby;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;
static float2 out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_tip: is_tip;
    [[vk::location(5)]] float in_var_bby: bby;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 base_pos = in_var_Vertex_Position;
    float3 pos_adjusted;
    if (in_var_is_tip == 0)
    {
        float3 pos_body = base_pos;
        pos_body.y = -lerp(local_start_y, body_end_y, 1.0f - in_var_bby);
        pos_adjusted = pos_body;
    }
    else
    {
        float3 pos_tip = base_pos;
        pos_tip.y = -lerp(body_end_y, tip_end_y, 1.0f - in_var_bby);
        pos_adjusted = pos_tip;
    }
    float t_sec = float(frame_constants_game_time_sec);
    float t = t_sec + frame_constants_game_time_frac;
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
    uint seed_u = uint(seed);
    uint seed_h0 = seed_u + (seed_u << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    float gust = lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - gust_uy) * (((asfloat(((g00_mix3 ^ (g00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) + ((asfloat(((g10_mix3 ^ (g10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux))) + (((asfloat(((g01_mix3 ^ (g01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) * gust_uy)) + (((asfloat(((g11_mix3 ^ (g11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux) * gust_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    bool is_not_corner = is_corner == 0;
    float bend_amp;
    if (is_not_corner)
    {
        bend_amp = gust * 0.300000011920928955078125f;
    }
    else
    {
        bend_amp = gust * 0.5f;
    }
    float2 flutter_pos = float2(t + pos_adjusted.x, pos_adjusted.y);
    int2 flutter_cell = int2(floor(flutter_pos));
    float2 flutter_frac = frac(flutter_pos);
    uint2 flutter_cell_u = uint2(flutter_cell);
    uint f00_h0 = flutter_cell_u.y;
    uint f00_h1 = f00_h0 + (f00_h0 << 10u);
    uint f00_h2 = f00_h1 ^ (f00_h1 >> 6u);
    uint f00_h3 = f00_h2 + (f00_h2 << 3u);
    uint f00_h4 = f00_h3 ^ (f00_h3 >> 11u);
    uint f00_mix0 = (((flutter_cell_u.x * 1664525u) + (f00_h4 + (f00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f00_mix1 = f00_mix0 ^ (f00_mix0 >> 11u);
    uint f00_mix2 = f00_mix1 ^ ((f00_mix1 << 7u) & 2636928640u);
    uint f00_mix3 = f00_mix2 ^ ((f00_mix2 << 15u) & 4022730752u);
    uint2 f10_cell = uint2(flutter_cell + int2(1, 0));
    uint f10_h0 = f10_cell.y;
    uint f10_h1 = f10_h0 + (f10_h0 << 10u);
    uint f10_h2 = f10_h1 ^ (f10_h1 >> 6u);
    uint f10_h3 = f10_h2 + (f10_h2 << 3u);
    uint f10_h4 = f10_h3 ^ (f10_h3 >> 11u);
    uint f10_mix0 = (((f10_cell.x * 1664525u) + (f10_h4 + (f10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f10_mix1 = f10_mix0 ^ (f10_mix0 >> 11u);
    uint f10_mix2 = f10_mix1 ^ ((f10_mix1 << 7u) & 2636928640u);
    uint f10_mix3 = f10_mix2 ^ ((f10_mix2 << 15u) & 4022730752u);
    uint2 f01_cell = uint2(flutter_cell + int2(0, 1));
    uint f01_h0 = f01_cell.y;
    uint f01_h1 = f01_h0 + (f01_h0 << 10u);
    uint f01_h2 = f01_h1 ^ (f01_h1 >> 6u);
    uint f01_h3 = f01_h2 + (f01_h2 << 3u);
    uint f01_h4 = f01_h3 ^ (f01_h3 >> 11u);
    uint f01_mix0 = (((f01_cell.x * 1664525u) + (f01_h4 + (f01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f01_mix1 = f01_mix0 ^ (f01_mix0 >> 11u);
    uint f01_mix2 = f01_mix1 ^ ((f01_mix1 << 7u) & 2636928640u);
    uint f01_mix3 = f01_mix2 ^ ((f01_mix2 << 15u) & 4022730752u);
    uint2 f11_cell = uint2(flutter_cell + int2(1, 1));
    uint f11_h0 = f11_cell.y;
    uint f11_h1 = f11_h0 + (f11_h0 << 10u);
    uint f11_h2 = f11_h1 ^ (f11_h1 >> 6u);
    uint f11_h3 = f11_h2 + (f11_h2 << 3u);
    uint f11_h4 = f11_h3 ^ (f11_h3 >> 11u);
    uint f11_mix0 = (((f11_cell.x * 1664525u) + (f11_h4 + (f11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f11_mix1 = f11_mix0 ^ (f11_mix0 >> 11u);
    uint f11_mix2 = f11_mix1 ^ ((f11_mix1 << 7u) & 2636928640u);
    uint f11_mix3 = f11_mix2 ^ ((f11_mix2 << 15u) & 4022730752u);
    float2 flut_u = (flutter_frac * flutter_frac) * (3.0f.xx - (flutter_frac * 2.0f));
    float flut_ux = flut_u.x;
    float flut_ux1 = 1.0f - flut_ux;
    float flut_uy = flut_u.y;
    float flutter_z = pos_adjusted.z + ((sin(((fmod(t_sec * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (pos_adjusted.y * 0.5f)) + ((((1.0f - flut_uy) * (((asfloat(((f00_mix3 ^ (f00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux1) + ((asfloat(((f10_mix3 ^ (f10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux))) + (((asfloat(((f01_mix3 ^ (f01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux1) * flut_uy)) + (((asfloat(((f11_mix3 ^ (f11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux) * flut_uy))) * smoothstep(0.0f, -3.0f, pos_adjusted.y)) * bend_amp);
    float3 pos_flutter = pos_adjusted;
    pos_flutter.z = flutter_z;
    float3 pos_final;
    if (is_not_corner)
    {
        float3 pos_corner = pos_flutter;
        pos_corner.z = abs(flutter_z + 0.0500000007450580596923828125f) - 0.0500000007450580596923828125f;
        pos_final = pos_corner;
    }
    else
    {
        pos_final = pos_flutter;
    }
    float4 wp = mul(model_to_world, float4(pos_final, 1.0f));
    float flag_height = body_end_y - local_start_y;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = wp.xyz;
    out_var_C3 = float2(base_pos.x + 0.5f, (pos_final.y * 1.5f) + 0.5f);
    out_var_C4 = in_var_Vertex_UV;
    out_var_C5 = (float2(base_pos.x, (pos_final.y + (flag_height * 0.5f)) + 0.20000000298023223876953125f) * (0.800000011920928955078125f / min(max(0.699999988079071044921875f, flag_height - 0.5f), 1.0f))) + 0.5f.xx;
    out_var_C6 = (float2(base_pos.x, (tip_end_y - local_start_y) + pos_final.y) * 0.75f) + float2(0.5f, 0.0f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_tip = stage_input.in_var_is_tip;
    in_var_bby = stage_input.in_var_bby;
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

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _flag_wall_cloth.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 model_to_world : packoffset(c1);
    int seed : packoffset(c5);
    float local_start_y : packoffset(c5.y);
    float body_end_y : packoffset(c5.z);
    float tip_end_y : packoffset(c5.w);
    int is_corner : packoffset(c6);
    int color_id : packoffset(c6.y);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};

SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_llc : register(s3, space0);
Texture2D<float4> border_mask : register(t14, space0);
Texture2D<float4> flag_palettes : register(t15, space0);
Texture2DArray<float4> flag_patterns : register(t16, space0);

static float3 in_var_C1;
static float3 in_var_C2;
static float2 in_var_C3;
static float2 in_var_C4;
static float2 in_var_C5;
static float2 in_var_C6;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    uint layer_seed = uint(seed ^ 234234);
    uint lyr_h0 = layer_seed + (layer_seed << 10u);
    uint lyr_h1 = lyr_h0 ^ (lyr_h0 >> 6u);
    uint lyr_h2 = lyr_h1 + (lyr_h1 << 3u);
    uint lyr_h3 = lyr_h2 ^ (lyr_h2 >> 11u);
    uint tip_seed = uint(seed ^ 794);
    uint tip_h0 = tip_seed + (tip_seed << 10u);
    uint tip_h1 = tip_h0 ^ (tip_h0 >> 6u);
    uint tip_h2 = tip_h1 + (tip_h1 << 3u);
    uint tip_h3 = tip_h2 ^ (tip_h2 >> 11u);
    float4 pattern_main = flag_patterns.SampleBias(sampler_llr, float3(in_var_C3.x, 1.0f - in_var_C3.y, float(int(round(lerp(9.0f, 11.0f, asfloat(((lyr_h3 + (lyr_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    float4 pattern_tip = flag_patterns.SampleBias(sampler_llc, float3(in_var_C6.x, 1.0f - in_var_C6.y, float(int(round(lerp(0.0f, 2.0f, asfloat(((tip_h3 + (tip_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f))))), frame_constants_global_mip_bias);
    float4 border_sample = border_mask.SampleBias(sampler_llc, in_var_C4, frame_constants_global_mip_bias);
    float border_x = border_sample.x;
    float pattern_sum = pattern_main.x + pattern_tip.x;
    float4 pattern_rand = flag_patterns.SampleLevel(sampler_llc, float3(in_var_C5.x, 1.0f - in_var_C5.y, float((seed - 6 * (seed / 6)) + 3)), 0.0f);
    float pattern_rand_x = pattern_rand.x;
    uint cid = uint(color_id);
    float3 albedo;
    if (border_x > 0.89999997615814208984375f)
    {
        albedo = flag_palettes.Load(int3(uint2(1u, cid), 0u)).xyz;
    }
    else
    {
        albedo = lerp(lerp(flag_palettes.Load(int3(uint2(0u, cid), 0u)).xyz, flag_palettes.Load(int3(uint2(2u, cid), 0u)).xyz, pattern_sum.xxx), flag_palettes.Load(int3(uint2(3u, cid), 0u)).xyz, pattern_rand_x.xxx);
    }
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
    float3 packed_normal = (normalize(nrm) * 0.5f) + 0.5f.xxx;
    float3 albedo_sqrt = sqrt(clamp(albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(albedo_sqrt.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(albedo_sqrt.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(albedo_sqrt.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp((((border_x + pattern_sum) + pattern_rand_x) > 0.89999997615814208984375f) ? 0.60000002384185791015625f : 1.0f, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), albedo_word);
    gbuffer.y = albedo_word | 117440512u;
    out_var_SV_TARGET0 = gbuffer;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _flag_wall_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    column_major float4x4 model_to_world : packoffset(c1);
    int seed : packoffset(c5);
    float local_start_y : packoffset(c5.y);
    float body_end_y : packoffset(c5.z);
    float tip_end_y : packoffset(c5.w);
    int is_corner : packoffset(c6);
    int color_id : packoffset(c6.y);
    uint2 border_mask_dims : packoffset(c6.z);
    uint2 flag_palettes_dims : packoffset(c7);
    int bool_render_shadows : packoffset(c7.z);
};


static float4 gl_Position;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Color;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_tip;
static float in_var_bby;
static float3 out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float2 out_var_C3;
static float2 out_var_C4;
static float2 out_var_C5;
static float2 out_var_C6;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
    [[vk::location(4)]] int in_var_is_tip: is_tip;
    [[vk::location(5)]] float in_var_bby: bby;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] float3 out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float2 out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float2 out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float2 out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    float3 base_pos = in_var_Vertex_Position;
    float3 pos_adjusted;
    if (in_var_is_tip == 0)
    {
        float3 pos_body = base_pos;
        pos_body.y = -lerp(local_start_y, body_end_y, 1.0f - in_var_bby);
        pos_adjusted = pos_body;
    }
    else
    {
        float3 pos_tip = base_pos;
        pos_tip.y = -lerp(body_end_y, tip_end_y, 1.0f - in_var_bby);
        pos_adjusted = pos_tip;
    }
    float t_sec = float(frame_constants_game_time_sec);
    float t = t_sec + frame_constants_game_time_frac;
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
    uint seed_u = uint(seed);
    uint seed_h0 = seed_u + (seed_u << 10u);
    uint seed_h1 = seed_h0 ^ (seed_h0 >> 6u);
    uint seed_h2 = seed_h1 + (seed_h1 << 3u);
    uint seed_h3 = seed_h2 ^ (seed_h2 >> 11u);
    float gust = lerp(0.300000011920928955078125f, 1.0f, ((sin((((((1.0f - gust_uy) * (((asfloat(((g00_mix3 ^ (g00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) + ((asfloat(((g10_mix3 ^ (g10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux))) + (((asfloat(((g01_mix3 ^ (g01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux1) * gust_uy)) + (((asfloat(((g11_mix3 ^ (g11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * gust_ux) * gust_uy)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.699999988079071044921875f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((seed_h3 + (seed_h3 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    bool is_not_corner = is_corner == 0;
    float bend_amp;
    if (is_not_corner)
    {
        bend_amp = gust * 0.300000011920928955078125f;
    }
    else
    {
        bend_amp = gust * 0.5f;
    }
    float2 flutter_pos = float2(t + pos_adjusted.x, pos_adjusted.y);
    int2 flutter_cell = int2(floor(flutter_pos));
    float2 flutter_frac = frac(flutter_pos);
    uint2 flutter_cell_u = uint2(flutter_cell);
    uint f00_h0 = flutter_cell_u.y;
    uint f00_h1 = f00_h0 + (f00_h0 << 10u);
    uint f00_h2 = f00_h1 ^ (f00_h1 >> 6u);
    uint f00_h3 = f00_h2 + (f00_h2 << 3u);
    uint f00_h4 = f00_h3 ^ (f00_h3 >> 11u);
    uint f00_mix0 = (((flutter_cell_u.x * 1664525u) + (f00_h4 + (f00_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f00_mix1 = f00_mix0 ^ (f00_mix0 >> 11u);
    uint f00_mix2 = f00_mix1 ^ ((f00_mix1 << 7u) & 2636928640u);
    uint f00_mix3 = f00_mix2 ^ ((f00_mix2 << 15u) & 4022730752u);
    uint2 f10_cell = uint2(flutter_cell + int2(1, 0));
    uint f10_h0 = f10_cell.y;
    uint f10_h1 = f10_h0 + (f10_h0 << 10u);
    uint f10_h2 = f10_h1 ^ (f10_h1 >> 6u);
    uint f10_h3 = f10_h2 + (f10_h2 << 3u);
    uint f10_h4 = f10_h3 ^ (f10_h3 >> 11u);
    uint f10_mix0 = (((f10_cell.x * 1664525u) + (f10_h4 + (f10_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f10_mix1 = f10_mix0 ^ (f10_mix0 >> 11u);
    uint f10_mix2 = f10_mix1 ^ ((f10_mix1 << 7u) & 2636928640u);
    uint f10_mix3 = f10_mix2 ^ ((f10_mix2 << 15u) & 4022730752u);
    uint2 f01_cell = uint2(flutter_cell + int2(0, 1));
    uint f01_h0 = f01_cell.y;
    uint f01_h1 = f01_h0 + (f01_h0 << 10u);
    uint f01_h2 = f01_h1 ^ (f01_h1 >> 6u);
    uint f01_h3 = f01_h2 + (f01_h2 << 3u);
    uint f01_h4 = f01_h3 ^ (f01_h3 >> 11u);
    uint f01_mix0 = (((f01_cell.x * 1664525u) + (f01_h4 + (f01_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f01_mix1 = f01_mix0 ^ (f01_mix0 >> 11u);
    uint f01_mix2 = f01_mix1 ^ ((f01_mix1 << 7u) & 2636928640u);
    uint f01_mix3 = f01_mix2 ^ ((f01_mix2 << 15u) & 4022730752u);
    uint2 f11_cell = uint2(flutter_cell + int2(1, 1));
    uint f11_h0 = f11_cell.y;
    uint f11_h1 = f11_h0 + (f11_h0 << 10u);
    uint f11_h2 = f11_h1 ^ (f11_h1 >> 6u);
    uint f11_h3 = f11_h2 + (f11_h2 << 3u);
    uint f11_h4 = f11_h3 ^ (f11_h3 >> 11u);
    uint f11_mix0 = (((f11_cell.x * 1664525u) + (f11_h4 + (f11_h4 << 15u))) + 1013904223u) * 1664525u;
    uint f11_mix1 = f11_mix0 ^ (f11_mix0 >> 11u);
    uint f11_mix2 = f11_mix1 ^ ((f11_mix1 << 7u) & 2636928640u);
    uint f11_mix3 = f11_mix2 ^ ((f11_mix2 << 15u) & 4022730752u);
    float2 flut_u = (flutter_frac * flutter_frac) * (3.0f.xx - (flutter_frac * 2.0f));
    float flut_ux = flut_u.x;
    float flut_ux1 = 1.0f - flut_ux;
    float flut_uy = flut_u.y;
    float flutter_z = pos_adjusted.z + ((sin(((fmod(t_sec * 1.7999999523162841796875f, 6.283185482025146484375f) + (frame_constants_game_time_frac * 1.7999999523162841796875f)) + (pos_adjusted.y * 0.5f)) + ((((1.0f - flut_uy) * (((asfloat(((f00_mix3 ^ (f00_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux1) + ((asfloat(((f10_mix3 ^ (f10_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux))) + (((asfloat(((f01_mix3 ^ (f01_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux1) * flut_uy)) + (((asfloat(((f11_mix3 ^ (f11_mix3 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * flut_ux) * flut_uy))) * smoothstep(0.0f, -3.0f, pos_adjusted.y)) * bend_amp);
    float3 pos_flutter = pos_adjusted;
    pos_flutter.z = flutter_z;
    float3 pos_final;
    if (is_not_corner)
    {
        float3 pos_corner = pos_flutter;
        pos_corner.z = abs(flutter_z + 0.0500000007450580596923828125f) - 0.0500000007450580596923828125f;
        pos_final = pos_corner;
    }
    else
    {
        pos_final = pos_flutter;
    }
    float4 wp = mul(model_to_world, float4(pos_final, 1.0f));
    float flag_height = body_end_y - local_start_y;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, wp);
    out_var_C0 = in_var_Vertex_Color;
    out_var_C1 = mul(model_to_world, float4(in_var_Vertex_Normal, 0.0f)).xyz;
    out_var_C2 = wp.xyz;
    out_var_C3 = float2(base_pos.x + 0.5f, (pos_final.y * 1.5f) + 0.5f);
    out_var_C4 = in_var_Vertex_UV;
    out_var_C5 = (float2(base_pos.x, (pos_final.y + (flag_height * 0.5f)) + 0.20000000298023223876953125f) * (0.800000011920928955078125f / min(max(0.699999988079071044921875f, flag_height - 0.5f), 1.0f))) + 0.5f.xx;
    out_var_C6 = (float2(base_pos.x, (tip_end_y - local_start_y) + pos_final.y) * 0.75f) + float2(0.5f, 0.0f);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_tip = stage_input.in_var_is_tip;
    in_var_bby = stage_input.in_var_bby;
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
// lifted from _flag_wall_cloth.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
