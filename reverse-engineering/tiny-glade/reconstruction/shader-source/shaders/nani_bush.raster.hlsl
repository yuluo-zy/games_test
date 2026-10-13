// PATCHED: modules lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_bush.raster.hlsl | B nani raster | bins: _nani_bush.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani bush: VS bends the bush with a 2D wind lattice + grass_object_push and
// forwards the part id + vertex height; PS alpha-discards via bush_alpha,
// culls the billboard edge, shades with bush_color and packs the gbuffer.

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
    float2 Vertex_UV : TEXCOORD1;
    float3 Vertex_Normal : TEXCOORD2;
    int bush_part_id : TEXCOORD3;
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
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = nani_instance_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float2 origin_xz = mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;

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
    world.x -= (vertex_h * vertex_h) * (sin((wind_noise * 3.14f) - 1.57f + 0.3f) * 0.0735f);

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
    if (bush_alpha.SampleBias(sampler_lnc, pin.C3, global_mip_bias).w < 0.5f)
    {
        discard;
    }
    float4 cam_vw = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam_vw.xyz / cam_vw.w.xxx;
    if (abs(dot(normalize(cam_pos - pin.C2), pin.C1)) > 0.5f)
    {
        discard;
    }
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_bush_alpha_dims : packoffset(c0.y);
    uint2 _Globals_bush_color_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_grass_object_push_dims : packoffset(c2.z);
    int _Globals_bool_render_shadows : packoffset(c3);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float2 in_var_Vertex_UV : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] int in_var_bush_part_id : TEXCOORD3;
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
    float2 _141 = in_var_Vertex_UV;
    Affine3Packed _148;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _148.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _167 = transpose(float4x3(float3(_148.data[0], _148.data[1], _148.data[2]), float3(_148.data[3], _148.data[4], _148.data[5]), float3(_148.data[6], _148.data[7], _148.data[8]), float3(_148.data[9], _148.data[10], _148.data[11])));
    float3 _172 = mul(_167, float4(in_var_Vertex_Position, 1.0f));
    _141.y = 1.0f - _141.y;
    float2 _177 = mul(_167, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    float _178 = in_var_Vertex_Position.y * 0.5f;
    float2 _188 = (_177 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _190 = int2(floor(_188));
    float2 _191 = frac(_188);
    uint2 _192 = uint2(_190);
    uint _194 = _192.y;
    uint _196 = _194 + (_194 << 10u);
    uint _198 = _196 ^ (_196 >> 6u);
    uint _200 = _198 + (_198 << 3u);
    uint _202 = _200 ^ (_200 >> 11u);
    uint _208 = (((_192.x * 1664525u) + (_202 + (_202 << 15u))) + 1013904223u) * 1664525u;
    uint _210 = _208 ^ (_208 >> 11u);
    uint _213 = _210 ^ ((_210 << 7u) & 2636928640u);
    uint _216 = _213 ^ ((_213 << 15u) & 4022730752u);
    uint2 _224 = uint2(_190 + int2(1, 0));
    uint _226 = _224.y;
    uint _228 = _226 + (_226 << 10u);
    uint _230 = _228 ^ (_228 >> 6u);
    uint _232 = _230 + (_230 << 3u);
    uint _234 = _232 ^ (_232 >> 11u);
    uint _240 = (((_224.x * 1664525u) + (_234 + (_234 << 15u))) + 1013904223u) * 1664525u;
    uint _242 = _240 ^ (_240 >> 11u);
    uint _245 = _242 ^ ((_242 << 7u) & 2636928640u);
    uint _248 = _245 ^ ((_245 << 15u) & 4022730752u);
    uint2 _256 = uint2(_190 + int2(0, 1));
    uint _258 = _256.y;
    uint _260 = _258 + (_258 << 10u);
    uint _262 = _260 ^ (_260 >> 6u);
    uint _264 = _262 + (_262 << 3u);
    uint _266 = _264 ^ (_264 >> 11u);
    uint _272 = (((_256.x * 1664525u) + (_266 + (_266 << 15u))) + 1013904223u) * 1664525u;
    uint _274 = _272 ^ (_272 >> 11u);
    uint _277 = _274 ^ ((_274 << 7u) & 2636928640u);
    uint _280 = _277 ^ ((_277 << 15u) & 4022730752u);
    uint2 _288 = uint2(_190 + int2(1, 1));
    uint _290 = _288.y;
    uint _292 = _290 + (_290 << 10u);
    uint _294 = _292 ^ (_292 >> 6u);
    uint _296 = _294 + (_294 << 3u);
    uint _298 = _296 ^ (_296 >> 11u);
    uint _304 = (((_288.x * 1664525u) + (_298 + (_298 << 15u))) + 1013904223u) * 1664525u;
    uint _306 = _304 ^ (_304 >> 11u);
    uint _309 = _306 ^ ((_306 << 7u) & 2636928640u);
    uint _312 = _309 ^ ((_309 << 15u) & 4022730752u);
    float2 _322 = (_191 * _191) * (3.0f.xx - (_191 * 2.0f));
    float _323 = _322.x;
    float _324 = 1.0f - _323;
    float _326 = _322.y;
    _172.x = _172.x - (((_178 * _178) * (sin((((((1.0f - _326) * (((asfloat(((_216 ^ (_216 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _324) + ((asfloat(((_248 ^ (_248 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _323))) + (((asfloat(((_280 ^ (_280 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _324) * _326)) + (((asfloat(((_312 ^ (_312 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _323) * _326)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _352 = grass_object_push.SampleLevel(sampler_default, (_177 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _354 = clamp(_352.z, 0.0f, 1.0f);
    float3 _365;
    if (_354 > 0.0f)
    {
        float2 _363 = _172.xz + ((_352.xy * _354) * smoothstep(0.0f, 1.0f, _178));
        _365 = float3(_363.x, _172.y, _363.y);
    }
    else
    {
        _365 = _172;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_365, 1.0f));
    out_var_C0 = 0.5f.xxx;
    out_var_C1 = mul(_167, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _365;
    out_var_C3 = _141;
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_bush_alpha_dims : packoffset(c0.y);
    uint2 _Globals_bush_color_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_grass_object_push_dims : packoffset(c2.z);
    int _Globals_bool_render_shadows : packoffset(c3);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(1)]] float2 in_var_Vertex_UV : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] int in_var_bush_part_id : TEXCOORD3;
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
    float2 _141 = in_var_Vertex_UV;
    Affine3Packed _148;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _148.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 48 + 0);
    }
    float3x4 _167 = transpose(float4x3(float3(_148.data[0], _148.data[1], _148.data[2]), float3(_148.data[3], _148.data[4], _148.data[5]), float3(_148.data[6], _148.data[7], _148.data[8]), float3(_148.data[9], _148.data[10], _148.data[11])));
    float3 _172 = mul(_167, float4(in_var_Vertex_Position, 1.0f));
    _141.y = 1.0f - _141.y;
    float2 _177 = mul(_167, float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
    float _178 = in_var_Vertex_Position.y * 0.5f;
    float2 _188 = (_177 * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
    int2 _190 = int2(floor(_188));
    float2 _191 = frac(_188);
    uint2 _192 = uint2(_190);
    uint _194 = _192.y;
    uint _196 = _194 + (_194 << 10u);
    uint _198 = _196 ^ (_196 >> 6u);
    uint _200 = _198 + (_198 << 3u);
    uint _202 = _200 ^ (_200 >> 11u);
    uint _208 = (((_192.x * 1664525u) + (_202 + (_202 << 15u))) + 1013904223u) * 1664525u;
    uint _210 = _208 ^ (_208 >> 11u);
    uint _213 = _210 ^ ((_210 << 7u) & 2636928640u);
    uint _216 = _213 ^ ((_213 << 15u) & 4022730752u);
    uint2 _224 = uint2(_190 + int2(1, 0));
    uint _226 = _224.y;
    uint _228 = _226 + (_226 << 10u);
    uint _230 = _228 ^ (_228 >> 6u);
    uint _232 = _230 + (_230 << 3u);
    uint _234 = _232 ^ (_232 >> 11u);
    uint _240 = (((_224.x * 1664525u) + (_234 + (_234 << 15u))) + 1013904223u) * 1664525u;
    uint _242 = _240 ^ (_240 >> 11u);
    uint _245 = _242 ^ ((_242 << 7u) & 2636928640u);
    uint _248 = _245 ^ ((_245 << 15u) & 4022730752u);
    uint2 _256 = uint2(_190 + int2(0, 1));
    uint _258 = _256.y;
    uint _260 = _258 + (_258 << 10u);
    uint _262 = _260 ^ (_260 >> 6u);
    uint _264 = _262 + (_262 << 3u);
    uint _266 = _264 ^ (_264 >> 11u);
    uint _272 = (((_256.x * 1664525u) + (_266 + (_266 << 15u))) + 1013904223u) * 1664525u;
    uint _274 = _272 ^ (_272 >> 11u);
    uint _277 = _274 ^ ((_274 << 7u) & 2636928640u);
    uint _280 = _277 ^ ((_277 << 15u) & 4022730752u);
    uint2 _288 = uint2(_190 + int2(1, 1));
    uint _290 = _288.y;
    uint _292 = _290 + (_290 << 10u);
    uint _294 = _292 ^ (_292 >> 6u);
    uint _296 = _294 + (_294 << 3u);
    uint _298 = _296 ^ (_296 >> 11u);
    uint _304 = (((_288.x * 1664525u) + (_298 + (_298 << 15u))) + 1013904223u) * 1664525u;
    uint _306 = _304 ^ (_304 >> 11u);
    uint _309 = _306 ^ ((_306 << 7u) & 2636928640u);
    uint _312 = _309 ^ ((_309 << 15u) & 4022730752u);
    float2 _322 = (_191 * _191) * (3.0f.xx - (_191 * 2.0f));
    float _323 = _322.x;
    float _324 = 1.0f - _323;
    float _326 = _322.y;
    _172.x = _172.x - (((_178 * _178) * (sin((((((1.0f - _326) * (((asfloat(((_216 ^ (_216 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _324) + ((asfloat(((_248 ^ (_248 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _323))) + (((asfloat(((_280 ^ (_280 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _324) * _326)) + (((asfloat(((_312 ^ (_312 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _323) * _326)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.073499999940395355224609375f);
    float4 _352 = grass_object_push.SampleLevel(sampler_default, (_177 * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float _354 = clamp(_352.z, 0.0f, 1.0f);
    float3 _365;
    if (_354 > 0.0f)
    {
        float2 _363 = _172.xz + ((_352.xy * _354) * smoothstep(0.0f, 1.0f, _178));
        _365 = float3(_363.x, _172.y, _363.y);
    }
    else
    {
        _365 = _172;
    }
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_365, 1.0f));
    out_var_C0 = 0.5f.xxx;
    out_var_C1 = mul(_167, float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _365;
    out_var_C3 = _141;
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
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_bush_alpha_dims : packoffset(c0.y);
    uint2 _Globals_bush_color_dims : packoffset(c1);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c1.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c2);
    uint2 _Globals_grass_object_push_dims : packoffset(c2.z);
    int _Globals_bool_render_shadows : packoffset(c3);
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
    if (bush_alpha.SampleBias(sampler_lnc, in_var_C3, frame_constants_global_mip_bias).w < 0.5f)
    {
        discard;
    }
    float4 _95 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(dot(normalize((_95.xyz / _95.w.xxx) - in_var_C2), in_var_C1)) > 0.5f)
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
