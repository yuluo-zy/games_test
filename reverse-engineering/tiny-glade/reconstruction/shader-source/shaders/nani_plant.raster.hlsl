// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
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

float hash2_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float3 Vertex_Color : TEXCOORD2;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
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
    float3 origin = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f));

    uint origin_hash = uint((((origin.x * 20.0f) - (origin.z * 20.0f)) + 2000.0f) + vin.Vertex_Position.x + vin.Vertex_Position.z);
    origin_hash = hash(origin_hash);

    float4 grass_push = grass_object_push.SampleLevel(sampler_default, (origin.xz * 0.0076923077f) + 0.5f.xx, 0.0f);
    float3 wind_pos = world;
    if (clamp(grass_push.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 displaced_xz = world.xz + (grass_push.xy * (pow(clamp(lerp(0.1f, 1.0f, vin.Vertex_Position.y * vin.Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        wind_pos = float3(displaced_xz.x, world.y, displaced_xz.y);
    }

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
    float wind_noise = (n00 + n10) + (n01 + n11);
    float instance_rand = hash2_rand(origin_hash);
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    int _Globals_bool_render_shadows : packoffset(c2);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Normal : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Color : TEXCOORD2;
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
    InstanceData _141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _161 = transpose(float4x3(float3(_141.xform.data[0], _141.xform.data[1], _141.xform.data[2]), float3(_141.xform.data[3], _141.xform.data[4], _141.xform.data[5]), float3(_141.xform.data[6], _141.xform.data[7], _141.xform.data[8]), float3(_141.xform.data[9], _141.xform.data[10], _141.xform.data[11])));
    float3 _166 = mul(_161, float4(in_var_Vertex_Position, 1.0f));
    float3 _167 = mul(_161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _176 = uint(((((_167.x * 20.0f) - (_167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    float2 _193 = _167.xz;
    float4 _197 = grass_object_push.SampleLevel(sampler_default, (_193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _213;
    if (clamp(_197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 _211 = _166.xz + (_197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        _213 = float3(_211.x, _166.y, _211.y);
    }
    else
    {
        _213 = _166;
    }
    float _214 = in_var_Vertex_Position.y + 0.5f;
    float2 _225 = (_193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _227 = int2(floor(_225));
    float2 _228 = frac(_225);
    uint2 _229 = uint2(_227);
    uint _231 = _229.y;
    uint _233 = _231 + (_231 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint _245 = (((_229.x * 1664525u) + (_239 + (_239 << 15u))) + 1013904223u) * 1664525u;
    uint _247 = _245 ^ (_245 >> 11u);
    uint _250 = _247 ^ ((_247 << 7u) & 2636928640u);
    uint _253 = _250 ^ ((_250 << 15u) & 4022730752u);
    uint2 _261 = uint2(_227 + int2(1, 0));
    uint _263 = _261.y;
    uint _265 = _263 + (_263 << 10u);
    uint _267 = _265 ^ (_265 >> 6u);
    uint _269 = _267 + (_267 << 3u);
    uint _271 = _269 ^ (_269 >> 11u);
    uint _277 = (((_261.x * 1664525u) + (_271 + (_271 << 15u))) + 1013904223u) * 1664525u;
    uint _279 = _277 ^ (_277 >> 11u);
    uint _282 = _279 ^ ((_279 << 7u) & 2636928640u);
    uint _285 = _282 ^ ((_282 << 15u) & 4022730752u);
    uint2 _293 = uint2(_227 + int2(0, 1));
    uint _295 = _293.y;
    uint _297 = _295 + (_295 << 10u);
    uint _299 = _297 ^ (_297 >> 6u);
    uint _301 = _299 + (_299 << 3u);
    uint _303 = _301 ^ (_301 >> 11u);
    uint _309 = (((_293.x * 1664525u) + (_303 + (_303 << 15u))) + 1013904223u) * 1664525u;
    uint _311 = _309 ^ (_309 >> 11u);
    uint _314 = _311 ^ ((_311 << 7u) & 2636928640u);
    uint _317 = _314 ^ ((_314 << 15u) & 4022730752u);
    uint2 _325 = uint2(_227 + int2(1, 1));
    uint _327 = _325.y;
    uint _329 = _327 + (_327 << 10u);
    uint _331 = _329 ^ (_329 >> 6u);
    uint _333 = _331 + (_331 << 3u);
    uint _335 = _333 ^ (_333 >> 11u);
    uint _341 = (((_325.x * 1664525u) + (_335 + (_335 << 15u))) + 1013904223u) * 1664525u;
    uint _343 = _341 ^ (_341 >> 11u);
    uint _346 = _343 ^ ((_343 << 7u) & 2636928640u);
    uint _349 = _346 ^ ((_346 << 15u) & 4022730752u);
    float2 _359 = (_228 * _228) * (3.0f.xx - (_228 * 2.0f));
    float _360 = _359.x;
    float _361 = 1.0f - _360;
    float _363 = _359.y;
    uint _380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint _382 = _380 + (_380 << 10u);
    uint _384 = _382 ^ (_382 >> 6u);
    uint _386 = _384 + (_384 << 3u);
    uint _388 = _386 ^ (_386 >> 11u);
    float _399 = _213.x - ((((_214 * _214) * (sin((((((1.0f - _363) * (((asfloat(((_253 ^ (_253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) + ((asfloat(((_285 ^ (_285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360))) + (((asfloat(((_317 ^ (_317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) * _363)) + (((asfloat(((_349 ^ (_349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360) * _363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_388 + (_388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 _400 = _213;
    _400.x = _399;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_399, _213.yz, 1.0f));
    out_var_C1 = normalize(mul(_161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _400;
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((_184 + (_184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
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
    float3 _58 = (in_var_C1 * 0.5f) + 0.5f.xxx;
    float3 _80 = sqrt(clamp(in_var_C3, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_58.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_58.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_58.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u, ((uint((clamp(_80.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_80.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_80.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 4278190080u);
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.y);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint2 _Globals_grass_object_push_dims : packoffset(c1.z);
    int _Globals_bool_render_shadows : packoffset(c2);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_Vertex_Normal : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Color : TEXCOORD2;
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
    InstanceData _141;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _141.xform.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _161 = transpose(float4x3(float3(_141.xform.data[0], _141.xform.data[1], _141.xform.data[2]), float3(_141.xform.data[3], _141.xform.data[4], _141.xform.data[5]), float3(_141.xform.data[6], _141.xform.data[7], _141.xform.data[8]), float3(_141.xform.data[9], _141.xform.data[10], _141.xform.data[11])));
    float3 _166 = mul(_161, float4(in_var_Vertex_Position, 1.0f));
    float3 _167 = mul(_161, float4(0.0f, 0.0f, 0.0f, 1.0f));
    uint _176 = uint(((((_167.x * 20.0f) - (_167.z * 20.0f)) + 2000.0f) + in_var_Vertex_Position.x) + in_var_Vertex_Position.z);
    uint _178 = _176 + (_176 << 10u);
    uint _180 = _178 ^ (_178 >> 6u);
    uint _182 = _180 + (_180 << 3u);
    uint _184 = _182 ^ (_182 >> 11u);
    float2 _193 = _167.xz;
    float4 _197 = grass_object_push.SampleLevel(sampler_default, (_193 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float3 _213;
    if (clamp(_197.z, 0.0f, 1.0f) > 0.0f)
    {
        float2 _211 = _166.xz + (_197.xy * (pow(clamp(lerp(0.100000001490116119384765625f, 1.0f, in_var_Vertex_Position.y * in_var_Vertex_Position.y), 0.0f, 1.0f), 0.5f) * 0.25f));
        _213 = float3(_211.x, _166.y, _211.y);
    }
    else
    {
        _213 = _166;
    }
    float _214 = in_var_Vertex_Position.y + 0.5f;
    float2 _225 = (_193 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _227 = int2(floor(_225));
    float2 _228 = frac(_225);
    uint2 _229 = uint2(_227);
    uint _231 = _229.y;
    uint _233 = _231 + (_231 << 10u);
    uint _235 = _233 ^ (_233 >> 6u);
    uint _237 = _235 + (_235 << 3u);
    uint _239 = _237 ^ (_237 >> 11u);
    uint _245 = (((_229.x * 1664525u) + (_239 + (_239 << 15u))) + 1013904223u) * 1664525u;
    uint _247 = _245 ^ (_245 >> 11u);
    uint _250 = _247 ^ ((_247 << 7u) & 2636928640u);
    uint _253 = _250 ^ ((_250 << 15u) & 4022730752u);
    uint2 _261 = uint2(_227 + int2(1, 0));
    uint _263 = _261.y;
    uint _265 = _263 + (_263 << 10u);
    uint _267 = _265 ^ (_265 >> 6u);
    uint _269 = _267 + (_267 << 3u);
    uint _271 = _269 ^ (_269 >> 11u);
    uint _277 = (((_261.x * 1664525u) + (_271 + (_271 << 15u))) + 1013904223u) * 1664525u;
    uint _279 = _277 ^ (_277 >> 11u);
    uint _282 = _279 ^ ((_279 << 7u) & 2636928640u);
    uint _285 = _282 ^ ((_282 << 15u) & 4022730752u);
    uint2 _293 = uint2(_227 + int2(0, 1));
    uint _295 = _293.y;
    uint _297 = _295 + (_295 << 10u);
    uint _299 = _297 ^ (_297 >> 6u);
    uint _301 = _299 + (_299 << 3u);
    uint _303 = _301 ^ (_301 >> 11u);
    uint _309 = (((_293.x * 1664525u) + (_303 + (_303 << 15u))) + 1013904223u) * 1664525u;
    uint _311 = _309 ^ (_309 >> 11u);
    uint _314 = _311 ^ ((_311 << 7u) & 2636928640u);
    uint _317 = _314 ^ ((_314 << 15u) & 4022730752u);
    uint2 _325 = uint2(_227 + int2(1, 1));
    uint _327 = _325.y;
    uint _329 = _327 + (_327 << 10u);
    uint _331 = _329 ^ (_329 >> 6u);
    uint _333 = _331 + (_331 << 3u);
    uint _335 = _333 ^ (_333 >> 11u);
    uint _341 = (((_325.x * 1664525u) + (_335 + (_335 << 15u))) + 1013904223u) * 1664525u;
    uint _343 = _341 ^ (_341 >> 11u);
    uint _346 = _343 ^ ((_343 << 7u) & 2636928640u);
    uint _349 = _346 ^ ((_346 << 15u) & 4022730752u);
    float2 _359 = (_228 * _228) * (3.0f.xx - (_228 * 2.0f));
    float _360 = _359.x;
    float _361 = 1.0f - _360;
    float _363 = _359.y;
    uint _380 = uint(int(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0)));
    uint _382 = _380 + (_380 << 10u);
    uint _384 = _382 ^ (_382 >> 6u);
    uint _386 = _384 + (_384 << 3u);
    uint _388 = _386 ^ (_386 >> 11u);
    float _399 = _213.x - ((((_214 * _214) * (sin((((((1.0f - _363) * (((asfloat(((_253 ^ (_253 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) + ((asfloat(((_285 ^ (_285 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360))) + (((asfloat(((_317 ^ (_317 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _361) * _363)) + (((asfloat(((_349 ^ (_349 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _360) * _363)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.0350000001490116119384765625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(((_388 + (_388 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
    float3 _400 = _213;
    _400.x = _399;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_399, _213.yz, 1.0f));
    out_var_C1 = normalize(mul(_161, float4(in_var_Vertex_Normal, 0.0f)));
    out_var_C2 = _400;
    out_var_C3 = pow(in_var_Vertex_Color, lerp(1.0f, 2.0f, asfloat(((_184 + (_184 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx);
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
