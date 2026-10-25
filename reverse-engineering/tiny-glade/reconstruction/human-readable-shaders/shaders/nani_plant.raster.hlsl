// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// 【文件用途】Nani 植物光栅化。VS 用 grass_object_push 掩码 + 2D 风值噪声格点
//   弯曲植被,并按实例做颜色变化;PS 打包 (法线, 反照率),反照率高字节带固体标志。
// 【入口】vs_main/ps_main | 绑定:instance_indirection_buffer t14、
//   nani_instance_data t15、grass_object_push t18、sampler_default s9、
//   terrain_heightmap t63(仅占位,dxc DCE)、$Globals b13、frame_constants b0/space1。
// 【变体】b903+vertex、b903+pixel、f0ad+vertex 用提起段;f0ad+pixel 用上方手写 PS。
// 重写说明:手写重建段补充中文注释;提起段(TGV_*)逐字保留,仅加块级中文注释。
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)))
// ==RECONSTRUCTED== shaders/nani_plant.raster.hlsl | B nani raster | bins: _nani_plant.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani plant: VS bends the foliage with grass_object_push masks + a 2D wind
// value-noise lattice and per-instance color variation; PS packs (normal,
// albedo) with a solid flag top byte.

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
#include "inc/perlin.hlsl"
#include "inc/wind.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/material/plant_material.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
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

uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint2 grass_object_push_dims;
int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
Texture2D<float4> grass_object_push : register(t18);

// 2D LCG 雪崩整数哈希(与 perlin 常数一致)
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

// 哈希位重解释为 [-1,1] 随机数(取 mantissa + 1.0 减 1)
float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 Vertex_Color: Vertex_Color;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

// VS:按实例变换顶点 -> grass_object_push 位移 -> 2D 风值噪声摆动 -> 采样空间投影
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
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));

// 以实例原点 + 顶点位置混出该实例的稳定随机种子
    uint origin_hash = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    origin_hash = hash(origin_hash);

// 采样草对象推送掩码(世界坐标 /130 + 0.5):xy=位移方向,z=是否在作用域内
    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float3 wind_pos = world;
// 在掩码作用域内按高度(vertex.y^2,0.1..1 lerp 后开方)*0.25 位移 xz
    if (clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 displaced_xz = world.xz + (grass_push.xy * (pow(clamp(lerp(0.1f, 1.0f, vin.Vertex_Position.y * vin.Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        wind_pos = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

// 风摆动:2D 值噪声(4 角点 LCG 哈希,平滑插值)随游戏时间平移
    float vertex_h = vin.Vertex_Position.y + 0.5f;
    float time = game_time_sec + game_time_frac;
    float2 wind_uv = (origin.xz * 0.1f) + float2(time, 0.0f) * 1.2f;
    int2 wind_cell = int2(floor(wind_uv));
    float2 wind_frac = frac(wind_uv);
    float2 wind_smooth = wind_frac * wind_frac * (3.0f.xx - (wind_frac * 2.0f));
    float n00 = hash2_rand(hash2(uint2(wind_cell))) * (1.0f - wind_smooth.x) * (1.0f - wind_smooth.y);
    float n10 = hash2_rand(hash2(uint2(wind_cell + int2(1, 0)))) * wind_smooth.x * (1.0f - wind_smooth.y);
    float n01 = hash2_rand(hash2(uint2(wind_cell + int2(0, 1)))) * (1.0f - wind_smooth.x) * wind_smooth.y;
    float n11 = hash2_rand(hash2(uint2(wind_cell + int2(1, 1)))) * wind_smooth.x * wind_smooth.y;
// 4 角点双线性插值累加得到 wind_noise
    float wind_noise = (n00 + n10) + (n01 + n11);
    float instance_rand = hash2_rand(origin_hash);
// sway = 顶点高平方 * sin(wind_noise*π-π/2+0.3) * 0.035 * 实例随机;从 x 减去
    float sway = (vertex_h * vertex_h) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.035f) * lerp(0.7f, 1.0f, instance_rand);
    wind_pos.x = wind_pos.x - sway;

    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(wind_pos, 1.0f));
    o.C1 = normalize(mul(xform, float4(vin.Vertex_Normal, 0.0f)));
    o.C2 = wind_pos;
    o.C3 = pow(vin.Vertex_Color, lerp(1.0f, 2.0f, hash2_rand(origin_hash)).xxx);
    return o;
}

struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C3 : TEXCOORD3;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

// PS:法线/反照率 sqrt 反 gamma 打包到 uint2 SV_Target0(纯 G-Buffer,无光照)
PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 packed_normal = (pin.C1 * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(pin.C3, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = uint(clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    uint normal_word = uint(clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f + 0.5f)
                     + (uint(clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
                     + (uint(clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)
                     + 4278190080u;
    o.gbuffer = uint2(normal_word, albedo_word);
    return o;
}

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_plant.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价
//   (48 字节步长读实例变换);PS 为法线/反照率打包(与手写 ps_main 等价)。
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
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 grass_object_push_dims : packoffset(c1.z);
    int bool_render_shadows : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    InstanceData v141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        v141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 v161 = transpose(float4x3(float3(v141.xform.data[0], v141.xform.data[1], v141.xform.data[2]), float3(v141.xform.data[3], v141.xform.data[4], v141.xform.data[5]), float3(v141.xform.data[6], v141.xform.data[7], v141.xform.data[8]), float3(v141.xform.data[9], v141.xform.data[10], v141.xform.data[11])));
    float3 v166 = mul(v161, float4(in_var_Vertex_Position, 1.0f));
    float3 v167 = mul(v161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint v176 = uint(((((v167.x * 20.0f) - (v167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint v178 = v176 + (v176 << 10u);
    uint v180 = v178 ^ (v178 >> 6u);
    uint v182 = v180 + (v180 << 3u);
    uint v184 = v182 ^ (v182 >> 11u);
    float2 v193 = v167.xz;
    float4 v197 = grass_object_push.SampleLevel(sampler_default, (v193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 v213;
    if (clamp(v197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 v211 = v166.xz + (v197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        v213 = float3(v211.x, v166.y, v211.y);
    }
    else
    {
        v213 = v166;
    }
    float v214 = in_var_Vertex_Position.y + 0.5f;
    float2 v225 = (v193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 v227 = int2(floor(v225));
    float2 v228 = frac(v225);
    uint2 v229 = uint2(v227);
    uint v231 = v229.y;
    uint v233 = v231 + (v231 << 10u);
    uint v235 = v233 ^ (v233 >> 6u);
    uint v237 = v235 + (v235 << 3u);
    uint v239 = v237 ^ (v237 >> 11u);
    uint v245 = (((v229.x * 1664525u) + (v239 + (v239 << 15u))) + 1013904223u) * 1664525u;
    uint v247 = v245 ^ (v245 >> 11u);
    uint v250 = v247 ^ ((v247 << 7u) & 2636928640u);
    uint v253 = v250 ^ ((v250 << 15u) & 4022730752u);
    uint2 v261 = uint2(v227 + int2(1, 0));
    uint v263 = v261.y;
    uint v265 = v263 + (v263 << 10u);
    uint v267 = v265 ^ (v265 >> 6u);
    uint v269 = v267 + (v267 << 3u);
    uint v271 = v269 ^ (v269 >> 11u);
    uint v277 = (((v261.x * 1664525u) + (v271 + (v271 << 15u))) + 1013904223u) * 1664525u;
    uint v279 = v277 ^ (v277 >> 11u);
    uint v282 = v279 ^ ((v279 << 7u) & 2636928640u);
    uint v285 = v282 ^ ((v282 << 15u) & 4022730752u);
    uint2 v293 = uint2(v227 + int2(0, 1));
    uint v295 = v293.y;
    uint v297 = v295 + (v295 << 10u);
    uint v299 = v297 ^ (v297 >> 6u);
    uint v301 = v299 + (v299 << 3u);
    uint v303 = v301 ^ (v301 >> 11u);
    uint v309 = (((v293.x * 1664525u) + (v303 + (v303 << 15u))) + 1013904223u) * 1664525u;
    uint v311 = v309 ^ (v309 >> 11u);
    uint v314 = v311 ^ ((v311 << 7u) & 2636928640u);
    uint v317 = v314 ^ ((v314 << 15u) & 4022730752u);
    uint2 v325 = uint2(v227 + int2(1, 1));
    uint v327 = v325.y;
    uint v329 = v327 + (v327 << 10u);
    uint v331 = v329 ^ (v329 >> 6u);
    uint v333 = v331 + (v331 << 3u);
    uint v335 = v333 ^ (v333 >> 11u);
    uint v341 = (((v325.x * 1664525u) + (v335 + (v335 << 15u))) + 1013904223u) * 1664525u;
    uint v343 = v341 ^ (v341 >> 11u);
    uint v346 = v343 ^ ((v343 << 7u) & 2636928640u);
    uint v349 = v346 ^ ((v346 << 15u) & 4022730752u);
    float2 v359 = (v228 * v228) * (3.0f.xx - (v228 * 2.0f));
    float v360 = v359.x;
    float v361 = 1.0f - v360;
    float v363 = v359.y;
    uint v380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint v382 = v380 + (v380 << 10u);
    uint v384 = v382 ^ (v382 >> 6u);
    uint v386 = v384 + (v384 << 3u);
    uint v388 = v386 ^ (v386 >> 11u);
// 内联后的风摆表达式:与手写 sway 一致 —— 4 角点 2D 值噪声插值出 wind_noise,
// 乘顶点高平方与实例随机,得到 x 方向的摆动位移
    float v399 = v213.x - ((((v214 * v214) * (sin((((((1.0f - v363) * (((asfloat(((v253 ^ (v253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v361) + ((asfloat(((v285 ^ (v285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v360))) + (((asfloat(((v317 ^ (v317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v361) * v363)) + (((asfloat(((v349 ^ (v349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v360) * v363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v388 + (v388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 v400 = v213;
    v400.x = v399;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v399, v213.yz, 1.0f));
    out_var_C1 = normalize(mul(v161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = v400;
// 顶点色按实例随机次幂(1..2)增强,做每实例颜色变化
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((v184 + (v184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_plant.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【b903 提起段】行为精确转译,名字是 SPIR-V id。VS 与手写 vs_main 等价
//   (48 字节步长读实例变换);PS 为法线/反照率打包(与手写 ps_main 等价)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float3 in_var_C1;
static float3 in_var_C3;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float3 v58 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 v80 = sqrt(clamp(in_var_C3, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(v58.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v58.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v58.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(v80.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v80.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v80.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C1 = stage_input.in_var_C1;
    in_var_C3 = stage_input.in_var_C3;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_plant.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 【f0ad 提起段】行为精确转译,名字是 SPIR-V id。与 b903 的 VS 逐字节一致
//   (仅提起 VS;PS 走上方手写段)。
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
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint2 grass_object_push_dims : packoffset(c1.z);
    int bool_render_shadows : packoffset(c2);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> grass_object_push : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float3 in_var_Vertex_Color;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C3;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(2)]] float3 in_var_Vertex_Color: Vertex_Color;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 out_var_C3 : TEXCOORD3;
    float4 gl_Position : SV_Position;
};

void vert_main()
{
    InstanceData v141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        v141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 v161 = transpose(float4x3(float3(v141.xform.data[0], v141.xform.data[1], v141.xform.data[2]), float3(v141.xform.data[3], v141.xform.data[4], v141.xform.data[5]), float3(v141.xform.data[6], v141.xform.data[7], v141.xform.data[8]), float3(v141.xform.data[9], v141.xform.data[10], v141.xform.data[11])));
    float3 v166 = mul(v161, float4(in_var_Vertex_Position, 1.0f));
    float3 v167 = mul(v161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint v176 = uint(((((v167.x * 20.0f) - (v167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint v178 = v176 + (v176 << 10u);
    uint v180 = v178 ^ (v178 >> 6u);
    uint v182 = v180 + (v180 << 3u);
    uint v184 = v182 ^ (v182 >> 11u);
    float2 v193 = v167.xz;
    float4 v197 = grass_object_push.SampleLevel(sampler_default, (v193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 v213;
    if (clamp(v197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 v211 = v166.xz + (v197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        v213 = float3(v211.x, v166.y, v211.y);
    }
    else
    {
        v213 = v166;
    }
    float v214 = in_var_Vertex_Position.y + 0.5f;
    float2 v225 = (v193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 v227 = int2(floor(v225));
    float2 v228 = frac(v225);
    uint2 v229 = uint2(v227);
    uint v231 = v229.y;
    uint v233 = v231 + (v231 << 10u);
    uint v235 = v233 ^ (v233 >> 6u);
    uint v237 = v235 + (v235 << 3u);
    uint v239 = v237 ^ (v237 >> 11u);
    uint v245 = (((v229.x * 1664525u) + (v239 + (v239 << 15u))) + 1013904223u) * 1664525u;
    uint v247 = v245 ^ (v245 >> 11u);
    uint v250 = v247 ^ ((v247 << 7u) & 2636928640u);
    uint v253 = v250 ^ ((v250 << 15u) & 4022730752u);
    uint2 v261 = uint2(v227 + int2(1, 0));
    uint v263 = v261.y;
    uint v265 = v263 + (v263 << 10u);
    uint v267 = v265 ^ (v265 >> 6u);
    uint v269 = v267 + (v267 << 3u);
    uint v271 = v269 ^ (v269 >> 11u);
    uint v277 = (((v261.x * 1664525u) + (v271 + (v271 << 15u))) + 1013904223u) * 1664525u;
    uint v279 = v277 ^ (v277 >> 11u);
    uint v282 = v279 ^ ((v279 << 7u) & 2636928640u);
    uint v285 = v282 ^ ((v282 << 15u) & 4022730752u);
    uint2 v293 = uint2(v227 + int2(0, 1));
    uint v295 = v293.y;
    uint v297 = v295 + (v295 << 10u);
    uint v299 = v297 ^ (v297 >> 6u);
    uint v301 = v299 + (v299 << 3u);
    uint v303 = v301 ^ (v301 >> 11u);
    uint v309 = (((v293.x * 1664525u) + (v303 + (v303 << 15u))) + 1013904223u) * 1664525u;
    uint v311 = v309 ^ (v309 >> 11u);
    uint v314 = v311 ^ ((v311 << 7u) & 2636928640u);
    uint v317 = v314 ^ ((v314 << 15u) & 4022730752u);
    uint2 v325 = uint2(v227 + int2(1, 1));
    uint v327 = v325.y;
    uint v329 = v327 + (v327 << 10u);
    uint v331 = v329 ^ (v329 >> 6u);
    uint v333 = v331 + (v331 << 3u);
    uint v335 = v333 ^ (v333 >> 11u);
    uint v341 = (((v325.x * 1664525u) + (v335 + (v335 << 15u))) + 1013904223u) * 1664525u;
    uint v343 = v341 ^ (v341 >> 11u);
    uint v346 = v343 ^ ((v343 << 7u) & 2636928640u);
    uint v349 = v346 ^ ((v346 << 15u) & 4022730752u);
    float2 v359 = (v228 * v228) * (3.0f.xx - (v228 * 2.0f));
    float v360 = v359.x;
    float v361 = 1.0f - v360;
    float v363 = v359.y;
    uint v380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint v382 = v380 + (v380 << 10u);
    uint v384 = v382 ^ (v382 >> 6u);
    uint v386 = v384 + (v384 << 3u);
    uint v388 = v386 ^ (v386 >> 11u);
// 内联后的风摆表达式:与手写 sway 一致 —— 4 角点 2D 值噪声插值出 wind_noise,
// 乘顶点高平方与实例随机,得到 x 方向的摆动位移
    float v399 = v213.x - ((((v214 * v214) * (sin((((((1.0f - v363) * (((asfloat(((v253 ^ (v253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v361) + ((asfloat(((v285 ^ (v285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v360))) + (((asfloat(((v317 ^ (v317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v361) * v363)) + (((asfloat(((v349 ^ (v349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * v360) * v363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((v388 + (v388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 v400 = v213;
    v400.x = v399;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(v399, v213.yz, 1.0f));
    out_var_C1 = normalize(mul(v161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = v400;
// 顶点色按实例随机次幂(1..2)增强,做每实例颜色变化
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((v184 + (v184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_Color = stage_input.in_var_Vertex_Color;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
