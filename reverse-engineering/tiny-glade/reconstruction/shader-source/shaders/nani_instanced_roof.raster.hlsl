// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_instanced_roof.raster.hlsl | B nani raster | bins: _nani_instanced_roof.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani instanced roof: per-tile transform from instanced_data; PS shades the
// roof tile palette with moss, damage and normal map and packs the gbuffer.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/current_view.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/perlin.hlsl"
#include "inc/parallax_mapping.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/texture_blend.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/roof_colors_include.hlsl"
#include "roof/roof_tile.inc.hlsl"
#include "roof/dormer_hole.inc.hlsl"
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

uint2 moss_texture_dims;
uint2 roof_tile_normal_dims;
uint2 roof_tile_damage_dims;
uint highlighted_wall_id;
uint highlight_mode;

struct RoofTileData
{
    Affine3Packed xform;
    uint flags;
    uint wall_id;
    float3 apex;
    uint roof_type;
};

StructuredBuffer<uint> instance_indirection_buffer : register(t15);
StructuredBuffer<RoofTileData> instanced_data : register(t21);
ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
Texture2DArray<float4> roof_colors : register(t16);
ByteAddressBuffer roof_color_ids : register(t17);
Texture2D<float4> roof_tile_normal : register(t19);
Texture2D<float4> roof_tile_damage : register(t20);
StructuredBuffer<uint> dormer_holes : register(t22);
StructuredBuffer<uint> dormer_hole_count : register(t23);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    float3 C2 : TEXCOORD2;
    float4 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    nointerpolation uint C8 : TEXCOORD8;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    uint idx = instance_indirection_buffer[vin.instance_id];
    Affine3Packed packed = instanced_data[idx].xform;
    float3x4 xform = transpose(float4x3(float3(packed.data[0], packed.data[1], packed.data[2]),
                                        float3(packed.data[3], packed.data[4], packed.data[5]),
                                        float3(packed.data[6], packed.data[7], packed.data[8]),
                                        float3(packed.data[9], packed.data[10], packed.data[11])));
    float3 world = mul(xform, float4(vin.Vertex_Position, 1.0f));
    float4x4 view_matrix = float4x4(
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 624),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 640),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 656),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 672),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 628),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 644),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 660),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 676),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 632),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 648),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 664),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 680),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 636),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 652),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 668),
        frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 684));
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(view_matrix, float4(world, 1.0f));
    o.C2 = world;
    o.C3 = float4(mul(xform, float4(0.0f, 0.0f, 0.0f, 1.0f)), 1.0f);
    o.C4 = vin.Vertex_Position.xz;
    o.C8 = idx;
    return o;
}

struct PsIn
{
    float3 C2 : TEXCOORD2;
    float4 C3 : TEXCOORD3;
    float2 C4 : TEXCOORD4;
    nointerpolation uint C8 : TEXCOORD8;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    float3 normal = normalize(cross(ddx(pin.C2), -ddy(pin.C2)));
    if (dot(float4(pin.C2, 1.0f), float4(0.0f, 1.0f, 0.0f, 0.0f)) < 0.0f)
    {
        discard;
    }
    uint dormer_count = dormer_hole_count[0];
    float4 dormer_check = frame_constants_storage_buffer.Load<float4>(current_view_index * 800 + 784);
    if (dormer_count > 0u)
    {
        for (uint i = 0u; i < min(dormer_count, 64u); i++)
        {
            if (distance(dormer_holes[i].xxx + dormer_check.xyz, pin.C2) < 1.0f)
            {
                discard;
            }
        }
    }
    RoofTileData inst = instanced_data[pin.C8];
    uint color_id = roof_color_ids.Load<uint>(inst.wall_id * 4 + 0);
    float3 albedo = roof_colors.SampleBias(sampler_llr, float3(pin.C4, float(color_id)), 0.0f).xyz;
    float tile_damage = roof_tile_damage.SampleBias(sampler_llr, pin.C4 * 0.5f, 0.0f).x;
    albedo *= lerp(0.7f, 1.0f, tile_damage);
    float3 tile_normal_map = roof_tile_normal.SampleBias(sampler_llr, pin.C4 * 0.5f, 0.0f).xyz * 2.0f - 1.0f.xxx;
    normal = normalize(normal + (tile_normal_map * 0.3f));
    if (highlight_mode == 1u && inst.wall_id == highlighted_wall_id)
    {
        albedo = lerp(albedo, float3(0.7f, 0.21f, 0.0f), 0.8f.xxx);
    }
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
// lifted from _nani_instanced_roof.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};

static uint _180;
static float3 _207;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_moss_texture_dims : packoffset(c0.y);
    uint2 _Globals_roof_tile_normal_dims : packoffset(c1);
    uint2 _Globals_roof_tile_damage_dims : packoffset(c1.z);
    uint _Globals_highlighted_wall_id : packoffset(c2);
    uint _Globals_highlight_mode : packoffset(c2.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer instanced_data : register(t21, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C2;
static float out_var_C3;
static float out_var_C4;
static float out_var_C5;
static float2 out_var_C6;
static float4 out_var_C7;
static uint out_var_C8;
static float out_var_C9;
static float2 out_var_C10;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation float4 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float out_var_C9 : TEXCOORD9;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
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
    float2 _218 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x & 65535u);
    float _219 = _218.x;
    float2 _222 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x >> 16u) & 65535u);
    float _223 = _222.x;
    float2 _226 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y & 65535u);
    float _227 = _226.x;
    float2 _230 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y >> 16u) & 65535u);
    float _231 = _230.x;
    float _234 = _219 + _219;
    float _235 = _223 + _223;
    float _236 = _227 + _227;
    float _237 = _219 * _234;
    float _238 = _219 * _235;
    float _239 = _219 * _236;
    float _240 = _223 * _235;
    float _241 = _223 * _236;
    float _242 = _227 * _236;
    float _243 = _231 * _234;
    float _244 = _231 * _235;
    float _245 = _231 * _236;
    float3x4 _262 = transpose(float4x3(float3(1.0f - (_240 + _242), _238 + _245, _239 - _244), float3(_238 - _245, 1.0f - (_237 + _242), _241 + _243), float3(_239 + _244, _241 - _243, 1.0f - (_237 + _240)), instanced_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0)));
    float2 _293 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y >> 16u) & 65535u);
    float _294 = _293.x;
    float4 _295 = float4(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y & 65535u).x, _294);
    float2 _315 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y >> 16u) & 65535u);
    float _316 = _315.x;
    bool _317 = instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) != 0;
    float3 _322 = (float3(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y & 65535u).x) * smoothstep(_316 * 0.800000011920928955078125f, _316, pow(instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0.800000011920928955078125f))) * float3(0.505970001220703125f, 0.5f, 0.5f);
    float _324 = _322.x * 0.949999988079071044921875f;
    float3 _325 = _322;
    _325.x = _324;
    float3 _326 = _325 * float3(3.58920383453369140625f, 3.70370388031005859375f, 3.70370388031005859375f);
    float _337 = max(min(max(_324, max(_322.y, _322.z)), min(_326.x, min(_326.y, _326.z))), 9.9999997473787516355514526367188e-06f);
    float3 _338 = in_var_Vertex_Position * float3(1.976401805877685546875f, 2.0f, 2.0f);
    float3 _339 = _338 * _337;
    bool3 _342 = bool3(_339.x >= 0.0f.xxx.x, _339.y >= 0.0f.xxx.y, _339.z >= 0.0f.xxx.z);
    float2 _349 = spvUnpackHalf2x16((instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) >> 16u) & 65535u);
    float _350 = _349.x;
    float3 _351 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _352 = sin(_350);
    float _353 = cos(_350);
    float _354 = 1.0f - _353;
    float _355 = _351.x;
    float _356 = _354 * _355;
    float _359 = _351.y;
    float _360 = _356 * _359;
    float _361 = _351.z;
    float _362 = _361 * _352;
    float _364 = _354 * _361;
    float _365 = _364 * _355;
    float _366 = _359 * _352;
    float _369 = _354 * _359;
    float _372 = _369 * _361;
    float _373 = _355 * _352;
    float3 _384 = mul(transpose(float3x3(float3((_356 * _355) + _353, _360 - _362, _365 + _366), float3(_360 + _362, (_369 * _359) + _353, _372 - _373), float3(_365 - _366, _372 + _373, (_364 * _361) + _353))), _339 - ((_337.xxx - _325) * float3(int3(_342.x ? int3(1, 1, 1).x : int3(-1, -1, -1).x, _342.y ? int3(1, 1, 1).y : int3(-1, -1, -1).y, _342.z ? int3(1, 1, 1).z : int3(-1, -1, -1).z))));
    float3 _422;
    if (_317)
    {
        uint _389 = (uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u) + instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 40);
        uint _391 = _389 + (_389 << 10u);
        uint _393 = _391 ^ (_391 >> 6u);
        uint _395 = _393 + (_393 << 3u);
        uint _397 = _395 ^ (_395 >> 11u);
        float _405 = ((asfloat(((_397 + (_397 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 0.0199999995529651641845703125f : 0.0599999986588954925537109375f;
        float3 _421;
        if (in_var_Vertex_Position.y > 0.100000001490116119384765625f)
        {
            float3 _420;
            if (instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) != 0)
            {
                float3 _419 = _384;
                _419.z = _384.z - _405;
                _420 = _419;
            }
            else
            {
                float3 _416 = _384;
                _416.z = _384.z + _405;
                _420 = _416;
            }
            _421 = _420;
        }
        else
        {
            _421 = _384;
        }
        _422 = _421;
    }
    else
    {
        _422 = _384;
    }
    uint _423 = _317 ? 1065353216u : _180;
    uint _424 = _317 ? 8388607u : _180;
    bool _425 = _317 ? true : false;
    float3 _437 = mul(_262, float4(_422.xy, _422.z + ((1.0f - smoothstep(0.0f, 0.800000011920928955078125f, mul(_262, float4(_422, 1.0f)).y)) * 0.115000002086162567138671875f), 1.0f));
    float _438 = _437.y;
    float _439 = _438 + instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12);
    float3 _440 = _437;
    _440.y = _439;
    float3 _610;
    uint _611;
    uint _612;
    bool _613;
    if (_317)
    {
        float2 _445 = _440.xz * 0.5f;
        int2 _447 = int2(floor(_445));
        float2 _448 = frac(_445);
        uint2 _449 = uint2(_447);
        uint _451 = _449.y;
        uint _453 = _451 + (_451 << 10u);
        uint _455 = _453 ^ (_453 >> 6u);
        uint _457 = _455 + (_455 << 3u);
        uint _459 = _457 ^ (_457 >> 11u);
        uint _465 = (((_449.x * 1664525u) + (_459 + (_459 << 15u))) + 1013904223u) * 1664525u;
        uint _467 = _465 ^ (_465 >> 11u);
        uint _470 = _467 ^ ((_467 << 7u) & 2636928640u);
        uint _473 = _470 ^ ((_470 << 15u) & 4022730752u);
        bool _476 = _425 ? _425 : true;
        uint _477 = _425 ? _424 : 8388607u;
        uint _478 = _425 ? _423 : 1065353216u;
        uint2 _484 = uint2(_447 + int2(1, 0));
        uint _486 = _484.y;
        uint _488 = _486 + (_486 << 10u);
        uint _490 = _488 ^ (_488 >> 6u);
        uint _492 = _490 + (_490 << 3u);
        uint _494 = _492 ^ (_492 >> 11u);
        uint _500 = (((_484.x * 1664525u) + (_494 + (_494 << 15u))) + 1013904223u) * 1664525u;
        uint _502 = _500 ^ (_500 >> 11u);
        uint _505 = _502 ^ ((_502 << 7u) & 2636928640u);
        uint _508 = _505 ^ ((_505 << 15u) & 4022730752u);
        bool _511 = _476 ? _476 : true;
        uint _512 = _476 ? _477 : 8388607u;
        uint _513 = _476 ? _478 : 1065353216u;
        uint2 _519 = uint2(_447 + int2(0, 1));
        uint _521 = _519.y;
        uint _523 = _521 + (_521 << 10u);
        uint _525 = _523 ^ (_523 >> 6u);
        uint _527 = _525 + (_525 << 3u);
        uint _529 = _527 ^ (_527 >> 11u);
        uint _535 = (((_519.x * 1664525u) + (_529 + (_529 << 15u))) + 1013904223u) * 1664525u;
        uint _537 = _535 ^ (_535 >> 11u);
        uint _540 = _537 ^ ((_537 << 7u) & 2636928640u);
        uint _543 = _540 ^ ((_540 << 15u) & 4022730752u);
        bool _546 = _511 ? _511 : true;
        uint _547 = _511 ? _512 : 8388607u;
        uint _548 = _511 ? _513 : 1065353216u;
        uint2 _554 = uint2(_447 + int2(1, 1));
        uint _556 = _554.y;
        uint _558 = _556 + (_556 << 10u);
        uint _560 = _558 ^ (_558 >> 6u);
        uint _562 = _560 + (_560 << 3u);
        uint _564 = _562 ^ (_562 >> 11u);
        uint _570 = (((_554.x * 1664525u) + (_564 + (_564 << 15u))) + 1013904223u) * 1664525u;
        uint _572 = _570 ^ (_570 >> 11u);
        uint _575 = _572 ^ ((_572 << 7u) & 2636928640u);
        uint _578 = _575 ^ ((_575 << 15u) & 4022730752u);
        uint _582 = _546 ? _547 : 8388607u;
        uint _583 = _546 ? _548 : 1065353216u;
        float2 _591 = (_448 * _448) * (3.0f.xx - (_448 * 2.0f));
        float _592 = _591.x;
        float _593 = 1.0f - _592;
        float _595 = _591.y;
        float3 _609 = _440;
        _609.y = _439 + ((((((1.0f - _595) * (((asfloat(((_473 ^ (_473 >> 18u)) & _477) | _478) - 1.0f) * _593) + ((asfloat(((_508 ^ (_508 >> 18u)) & _512) | _513) - 1.0f) * _592))) + (((asfloat(((_543 ^ (_543 >> 18u)) & _547) | _548) - 1.0f) * _593) * _595)) + (((asfloat(((_578 ^ (_578 >> 18u)) & _582) | _583) - 1.0f) * _592) * _595)) * 0.300000011920928955078125f) * smoothstep(0.0f, 1.0f, _438));
        _610 = _609;
        _611 = _583;
        _612 = _582;
        _613 = _546 ? _546 : true;
    }
    else
    {
        _610 = _440;
        _611 = _423;
        _612 = _424;
        _613 = _425;
    }
    float3 _616 = _610 * 0.4000000059604644775390625f;
    int3 _618 = int3(floor(_616));
    float3 _619 = frac(_616);
    uint3 _620 = uint3(_618);
    uint _623 = _620.z;
    uint _625 = _623 + (_623 << 10u);
    uint _627 = _625 ^ (_625 >> 6u);
    uint _629 = _627 + (_627 << 3u);
    uint _631 = _629 ^ (_629 >> 11u);
    bool _634 = _425 ? _425 : true;
    uint _635 = _425 ? (_317 ? 1664525u : _180) : 1664525u;
    uint _636 = _425 ? (_317 ? 1013904223u : _180) : 1013904223u;
    uint _640 = (((_620.y * _635) + (_631 + (_631 << 15u))) + _636) * _635;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    bool _651 = _634 ? _634 : true;
    uint _652 = _634 ? _635 : 1664525u;
    uint _653 = _634 ? _636 : 1013904223u;
    uint _657 = (((_620.x * _652) + (_648 ^ (_648 >> 18u))) + _653) * _652;
    uint _659 = _657 ^ (_657 >> 11u);
    uint _662 = _659 ^ ((_659 << 7u) & 2636928640u);
    uint _665 = _662 ^ ((_662 << 15u) & 4022730752u);
    uint _667 = _665 ^ (_665 >> 18u);
    uint _669 = _667 + (_667 << 10u);
    uint _671 = _669 ^ (_669 >> 6u);
    uint _673 = _671 + (_671 << 3u);
    uint _675 = _673 ^ (_673 >> 11u);
    bool _678 = _613 ? _613 : true;
    uint _679 = _613 ? _612 : 8388607u;
    uint _680 = _613 ? _611 : 1065353216u;
    bool _685 = _678 ? _678 : true;
    uint _686 = _678 ? _679 : 8388607u;
    uint _687 = _678 ? _680 : 1065353216u;
    uint3 _694 = uint3(_618 + int3(1, 0, 0));
    uint _697 = _694.z;
    uint _699 = _697 + (_697 << 10u);
    uint _701 = _699 ^ (_699 >> 6u);
    uint _703 = _701 + (_701 << 3u);
    uint _705 = _703 ^ (_703 >> 11u);
    bool _708 = _651 ? _651 : true;
    uint _709 = _651 ? _652 : 1664525u;
    uint _710 = _651 ? _653 : 1013904223u;
    uint _714 = (((_694.y * _709) + (_705 + (_705 << 15u))) + _710) * _709;
    uint _716 = _714 ^ (_714 >> 11u);
    uint _719 = _716 ^ ((_716 << 7u) & 2636928640u);
    uint _722 = _719 ^ ((_719 << 15u) & 4022730752u);
    bool _725 = _708 ? _708 : true;
    uint _726 = _708 ? _709 : 1664525u;
    uint _727 = _708 ? _710 : 1013904223u;
    uint _731 = (((_694.x * _726) + (_722 ^ (_722 >> 18u))) + _727) * _726;
    uint _733 = _731 ^ (_731 >> 11u);
    uint _736 = _733 ^ ((_733 << 7u) & 2636928640u);
    uint _739 = _736 ^ ((_736 << 15u) & 4022730752u);
    uint _741 = _739 ^ (_739 >> 18u);
    uint _743 = _741 + (_741 << 10u);
    uint _745 = _743 ^ (_743 >> 6u);
    uint _747 = _745 + (_745 << 3u);
    uint _749 = _747 ^ (_747 >> 11u);
    bool _752 = _685 ? _685 : true;
    uint _753 = _685 ? _686 : 8388607u;
    uint _754 = _685 ? _687 : 1065353216u;
    bool _759 = _752 ? _752 : true;
    uint _760 = _752 ? _753 : 8388607u;
    uint _761 = _752 ? _754 : 1065353216u;
    uint3 _768 = uint3(_618 + int3(0, 1, 0));
    uint _771 = _768.z;
    uint _773 = _771 + (_771 << 10u);
    uint _775 = _773 ^ (_773 >> 6u);
    uint _777 = _775 + (_775 << 3u);
    uint _779 = _777 ^ (_777 >> 11u);
    bool _782 = _725 ? _725 : true;
    uint _783 = _725 ? _726 : 1664525u;
    uint _784 = _725 ? _727 : 1013904223u;
    uint _788 = (((_768.y * _783) + (_779 + (_779 << 15u))) + _784) * _783;
    uint _790 = _788 ^ (_788 >> 11u);
    uint _793 = _790 ^ ((_790 << 7u) & 2636928640u);
    uint _796 = _793 ^ ((_793 << 15u) & 4022730752u);
    bool _799 = _782 ? _782 : true;
    uint _800 = _782 ? _783 : 1664525u;
    uint _801 = _782 ? _784 : 1013904223u;
    uint _805 = (((_768.x * _800) + (_796 ^ (_796 >> 18u))) + _801) * _800;
    uint _807 = _805 ^ (_805 >> 11u);
    uint _810 = _807 ^ ((_807 << 7u) & 2636928640u);
    uint _813 = _810 ^ ((_810 << 15u) & 4022730752u);
    uint _815 = _813 ^ (_813 >> 18u);
    uint _817 = _815 + (_815 << 10u);
    uint _819 = _817 ^ (_817 >> 6u);
    uint _821 = _819 + (_819 << 3u);
    uint _823 = _821 ^ (_821 >> 11u);
    bool _826 = _759 ? _759 : true;
    uint _827 = _759 ? _760 : 8388607u;
    uint _828 = _759 ? _761 : 1065353216u;
    bool _833 = _826 ? _826 : true;
    uint _834 = _826 ? _827 : 8388607u;
    uint _835 = _826 ? _828 : 1065353216u;
    uint3 _842 = uint3(_618 + int3(1, 1, 0));
    uint _845 = _842.z;
    uint _847 = _845 + (_845 << 10u);
    uint _849 = _847 ^ (_847 >> 6u);
    uint _851 = _849 + (_849 << 3u);
    uint _853 = _851 ^ (_851 >> 11u);
    bool _856 = _799 ? _799 : true;
    uint _857 = _799 ? _800 : 1664525u;
    uint _858 = _799 ? _801 : 1013904223u;
    uint _862 = (((_842.y * _857) + (_853 + (_853 << 15u))) + _858) * _857;
    uint _864 = _862 ^ (_862 >> 11u);
    uint _867 = _864 ^ ((_864 << 7u) & 2636928640u);
    uint _870 = _867 ^ ((_867 << 15u) & 4022730752u);
    bool _873 = _856 ? _856 : true;
    uint _874 = _856 ? _857 : 1664525u;
    uint _875 = _856 ? _858 : 1013904223u;
    uint _879 = (((_842.x * _874) + (_870 ^ (_870 >> 18u))) + _875) * _874;
    uint _881 = _879 ^ (_879 >> 11u);
    uint _884 = _881 ^ ((_881 << 7u) & 2636928640u);
    uint _887 = _884 ^ ((_884 << 15u) & 4022730752u);
    uint _889 = _887 ^ (_887 >> 18u);
    uint _891 = _889 + (_889 << 10u);
    uint _893 = _891 ^ (_891 >> 6u);
    uint _895 = _893 + (_893 << 3u);
    uint _897 = _895 ^ (_895 >> 11u);
    bool _900 = _833 ? _833 : true;
    uint _901 = _833 ? _834 : 8388607u;
    uint _902 = _833 ? _835 : 1065353216u;
    bool _907 = _900 ? _900 : true;
    uint _908 = _900 ? _901 : 8388607u;
    uint _909 = _900 ? _902 : 1065353216u;
    uint3 _916 = uint3(_618 + int3(0, 0, 1));
    uint _919 = _916.z;
    uint _921 = _919 + (_919 << 10u);
    uint _923 = _921 ^ (_921 >> 6u);
    uint _925 = _923 + (_923 << 3u);
    uint _927 = _925 ^ (_925 >> 11u);
    bool _930 = _873 ? _873 : true;
    uint _931 = _873 ? _874 : 1664525u;
    uint _932 = _873 ? _875 : 1013904223u;
    uint _936 = (((_916.y * _931) + (_927 + (_927 << 15u))) + _932) * _931;
    uint _938 = _936 ^ (_936 >> 11u);
    uint _941 = _938 ^ ((_938 << 7u) & 2636928640u);
    uint _944 = _941 ^ ((_941 << 15u) & 4022730752u);
    bool _947 = _930 ? _930 : true;
    uint _948 = _930 ? _931 : 1664525u;
    uint _949 = _930 ? _932 : 1013904223u;
    uint _953 = (((_916.x * _948) + (_944 ^ (_944 >> 18u))) + _949) * _948;
    uint _955 = _953 ^ (_953 >> 11u);
    uint _958 = _955 ^ ((_955 << 7u) & 2636928640u);
    uint _961 = _958 ^ ((_958 << 15u) & 4022730752u);
    uint _963 = _961 ^ (_961 >> 18u);
    uint _965 = _963 + (_963 << 10u);
    uint _967 = _965 ^ (_965 >> 6u);
    uint _969 = _967 + (_967 << 3u);
    uint _971 = _969 ^ (_969 >> 11u);
    bool _974 = _907 ? _907 : true;
    uint _975 = _907 ? _908 : 8388607u;
    uint _976 = _907 ? _909 : 1065353216u;
    bool _981 = _974 ? _974 : true;
    uint _982 = _974 ? _975 : 8388607u;
    uint _983 = _974 ? _976 : 1065353216u;
    uint3 _990 = uint3(_618 + int3(1, 0, 1));
    uint _993 = _990.z;
    uint _995 = _993 + (_993 << 10u);
    uint _997 = _995 ^ (_995 >> 6u);
    uint _999 = _997 + (_997 << 3u);
    uint _1001 = _999 ^ (_999 >> 11u);
    bool _1004 = _947 ? _947 : true;
    uint _1005 = _947 ? _948 : 1664525u;
    uint _1006 = _947 ? _949 : 1013904223u;
    uint _1010 = (((_990.y * _1005) + (_1001 + (_1001 << 15u))) + _1006) * _1005;
    uint _1012 = _1010 ^ (_1010 >> 11u);
    uint _1015 = _1012 ^ ((_1012 << 7u) & 2636928640u);
    uint _1018 = _1015 ^ ((_1015 << 15u) & 4022730752u);
    bool _1021 = _1004 ? _1004 : true;
    uint _1022 = _1004 ? _1005 : 1664525u;
    uint _1023 = _1004 ? _1006 : 1013904223u;
    uint _1027 = (((_990.x * _1022) + (_1018 ^ (_1018 >> 18u))) + _1023) * _1022;
    uint _1029 = _1027 ^ (_1027 >> 11u);
    uint _1032 = _1029 ^ ((_1029 << 7u) & 2636928640u);
    uint _1035 = _1032 ^ ((_1032 << 15u) & 4022730752u);
    uint _1037 = _1035 ^ (_1035 >> 18u);
    uint _1039 = _1037 + (_1037 << 10u);
    uint _1041 = _1039 ^ (_1039 >> 6u);
    uint _1043 = _1041 + (_1041 << 3u);
    uint _1045 = _1043 ^ (_1043 >> 11u);
    bool _1048 = _981 ? _981 : true;
    uint _1049 = _981 ? _982 : 8388607u;
    uint _1050 = _981 ? _983 : 1065353216u;
    bool _1055 = _1048 ? _1048 : true;
    uint _1056 = _1048 ? _1049 : 8388607u;
    uint _1057 = _1048 ? _1050 : 1065353216u;
    uint3 _1064 = uint3(_618 + int3(0, 1, 1));
    uint _1067 = _1064.z;
    uint _1069 = _1067 + (_1067 << 10u);
    uint _1071 = _1069 ^ (_1069 >> 6u);
    uint _1073 = _1071 + (_1071 << 3u);
    uint _1075 = _1073 ^ (_1073 >> 11u);
    bool _1078 = _1021 ? _1021 : true;
    uint _1079 = _1021 ? _1022 : 1664525u;
    uint _1080 = _1021 ? _1023 : 1013904223u;
    uint _1084 = (((_1064.y * _1079) + (_1075 + (_1075 << 15u))) + _1080) * _1079;
    uint _1086 = _1084 ^ (_1084 >> 11u);
    uint _1089 = _1086 ^ ((_1086 << 7u) & 2636928640u);
    uint _1092 = _1089 ^ ((_1089 << 15u) & 4022730752u);
    bool _1095 = _1078 ? _1078 : true;
    uint _1096 = _1078 ? _1079 : 1664525u;
    uint _1097 = _1078 ? _1080 : 1013904223u;
    uint _1101 = (((_1064.x * _1096) + (_1092 ^ (_1092 >> 18u))) + _1097) * _1096;
    uint _1103 = _1101 ^ (_1101 >> 11u);
    uint _1106 = _1103 ^ ((_1103 << 7u) & 2636928640u);
    uint _1109 = _1106 ^ ((_1106 << 15u) & 4022730752u);
    uint _1111 = _1109 ^ (_1109 >> 18u);
    uint _1113 = _1111 + (_1111 << 10u);
    uint _1115 = _1113 ^ (_1113 >> 6u);
    uint _1117 = _1115 + (_1115 << 3u);
    uint _1119 = _1117 ^ (_1117 >> 11u);
    bool _1122 = _1055 ? _1055 : true;
    uint _1123 = _1055 ? _1056 : 8388607u;
    uint _1124 = _1055 ? _1057 : 1065353216u;
    bool _1129 = _1122 ? _1122 : true;
    uint _1130 = _1122 ? _1123 : 8388607u;
    uint _1131 = _1122 ? _1124 : 1065353216u;
    uint3 _1138 = uint3(_618 + int3(1, 1, 1));
    uint _1141 = _1138.z;
    uint _1143 = _1141 + (_1141 << 10u);
    uint _1145 = _1143 ^ (_1143 >> 6u);
    uint _1147 = _1145 + (_1145 << 3u);
    uint _1149 = _1147 ^ (_1147 >> 11u);
    bool _1152 = _1095 ? _1095 : true;
    uint _1153 = _1095 ? _1096 : 1664525u;
    uint _1154 = _1095 ? _1097 : 1013904223u;
    uint _1158 = (((_1138.y * _1153) + (_1149 + (_1149 << 15u))) + _1154) * _1153;
    uint _1160 = _1158 ^ (_1158 >> 11u);
    uint _1163 = _1160 ^ ((_1160 << 7u) & 2636928640u);
    uint _1166 = _1163 ^ ((_1163 << 15u) & 4022730752u);
    bool _1169 = _1152 ? _1152 : true;
    uint _1170 = _1152 ? _1153 : 1664525u;
    uint _1171 = _1152 ? _1154 : 1013904223u;
    uint _1175 = (((_1138.x * _1170) + (_1166 ^ (_1166 >> 18u))) + _1171) * _1170;
    uint _1177 = _1175 ^ (_1175 >> 11u);
    uint _1180 = _1177 ^ ((_1177 << 7u) & 2636928640u);
    uint _1183 = _1180 ^ ((_1180 << 15u) & 4022730752u);
    uint _1185 = _1183 ^ (_1183 >> 18u);
    uint _1187 = _1185 + (_1185 << 10u);
    uint _1189 = _1187 ^ (_1187 >> 6u);
    uint _1191 = _1189 + (_1189 << 3u);
    uint _1193 = _1191 ^ (_1191 >> 11u);
    bool _1196 = _1129 ? _1129 : true;
    uint _1197 = _1129 ? _1130 : 8388607u;
    uint _1198 = _1129 ? _1131 : 1065353216u;
    bool _1203 = _1196 ? _1196 : true;
    uint _1204 = _1196 ? _1197 : 8388607u;
    uint _1205 = _1196 ? _1198 : 1065353216u;
    float3 _1214 = (_619 * _619) * (3.0f.xxx - (_619 * 2.0f));
    float _1215 = _1214.x;
    float _1216 = 1.0f - _1215;
    float _1218 = _1214.y;
    float _1219 = 1.0f - _1218;
    float _1221 = _1214.z;
    float _1222 = 1.0f - _1221;
    float2 _1235 = (((((float2(asfloat((_667 & _679) | _680) - 1.0f, asfloat(((_675 + (_675 << 15u)) & _686) | _687) - 1.0f) * _1216) * _1219) * _1222) + (((float2(asfloat((_741 & _753) | _754) - 1.0f, asfloat(((_749 + (_749 << 15u)) & _760) | _761) - 1.0f) * _1215) * _1219) * _1222)) + (((float2(asfloat((_815 & _827) | _828) - 1.0f, asfloat(((_823 + (_823 << 15u)) & _834) | _835) - 1.0f) * _1216) * _1218) * _1222)) + (((float2(asfloat((_889 & _901) | _902) - 1.0f, asfloat(((_897 + (_897 << 15u)) & _908) | _909) - 1.0f) * _1215) * _1218) * _1222);
    float2 _1251 = (((_1235 + (((float2(asfloat((_963 & _975) | _976) - 1.0f, asfloat(((_971 + (_971 << 15u)) & _982) | _983) - 1.0f) * _1216) * _1219) * _1221)) + (((float2(asfloat((_1037 & _1049) | _1050) - 1.0f, asfloat(((_1045 + (_1045 << 15u)) & _1056) | _1057) - 1.0f) * _1215) * _1219) * _1221)) + (((float2(asfloat((_1111 & _1123) | _1124) - 1.0f, asfloat(((_1119 + (_1119 << 15u)) & _1130) | _1131) - 1.0f) * _1216) * _1218) * _1221)) + (((float2(asfloat((_1185 & _1197) | _1198) - 1.0f, asfloat(((_1193 + (_1193 << 15u)) & _1204) | _1205) - 1.0f) * _1215) * _1218) * _1221);
    float2 _1254 = clamp((_1251 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _1259 = _610 + (float3(_1254.x, 0.0f, _1254.y) * 0.0599999986588954925537109375f);
    float2 _1260 = _1259.xz;
    uint _1264 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52) ^ 787234);
    uint _1266 = _1264 + (_1264 << 10u);
    uint _1268 = _1266 ^ (_1266 >> 6u);
    uint _1270 = _1268 + (_1268 << 3u);
    uint _1272 = _1270 ^ (_1270 >> 11u);
    bool _1275 = _1203 ? _1203 : true;
    uint _1276 = _1203 ? _1204 : 8388607u;
    uint _1277 = _1203 ? _1205 : 1065353216u;
    float2 _1283 = _1260 + (asfloat(((_1272 + (_1272 << 15u)) & _1276) | _1277) - 1.0f).xx;
    int2 _1285 = int2(floor(_1283));
    float2 _1286 = frac(_1283);
    uint2 _1287 = uint2(_1285);
    uint _1289 = _1287.y;
    uint _1291 = _1289 + (_1289 << 10u);
    uint _1293 = _1291 ^ (_1291 >> 6u);
    uint _1295 = _1293 + (_1293 << 3u);
    uint _1297 = _1295 ^ (_1295 >> 11u);
    bool _1300 = _1169 ? _1169 : true;
    uint _1301 = _1169 ? _1170 : 1664525u;
    uint _1302 = _1169 ? _1171 : 1013904223u;
    uint _1306 = (((_1287.x * _1301) + (_1297 + (_1297 << 15u))) + _1302) * _1301;
    uint _1308 = _1306 ^ (_1306 >> 11u);
    uint _1311 = _1308 ^ ((_1308 << 7u) & 2636928640u);
    uint _1314 = _1311 ^ ((_1311 << 15u) & 4022730752u);
    bool _1317 = _1275 ? _1275 : true;
    uint _1318 = _1275 ? _1276 : 8388607u;
    uint _1319 = _1275 ? _1277 : 1065353216u;
    uint2 _1325 = uint2(_1285 + int2(1, 0));
    uint _1327 = _1325.y;
    uint _1329 = _1327 + (_1327 << 10u);
    uint _1331 = _1329 ^ (_1329 >> 6u);
    uint _1333 = _1331 + (_1331 << 3u);
    uint _1335 = _1333 ^ (_1333 >> 11u);
    bool _1338 = _1300 ? _1300 : true;
    uint _1339 = _1300 ? _1301 : 1664525u;
    uint _1340 = _1300 ? _1302 : 1013904223u;
    uint _1344 = (((_1325.x * _1339) + (_1335 + (_1335 << 15u))) + _1340) * _1339;
    uint _1346 = _1344 ^ (_1344 >> 11u);
    uint _1349 = _1346 ^ ((_1346 << 7u) & 2636928640u);
    uint _1352 = _1349 ^ ((_1349 << 15u) & 4022730752u);
    bool _1355 = _1317 ? _1317 : true;
    uint _1356 = _1317 ? _1318 : 8388607u;
    uint _1357 = _1317 ? _1319 : 1065353216u;
    uint2 _1363 = uint2(_1285 + int2(0, 1));
    uint _1365 = _1363.y;
    uint _1367 = _1365 + (_1365 << 10u);
    uint _1369 = _1367 ^ (_1367 >> 6u);
    uint _1371 = _1369 + (_1369 << 3u);
    uint _1373 = _1371 ^ (_1371 >> 11u);
    bool _1376 = _1338 ? _1338 : true;
    uint _1377 = _1338 ? _1339 : 1664525u;
    uint _1378 = _1338 ? _1340 : 1013904223u;
    uint _1382 = (((_1363.x * _1377) + (_1373 + (_1373 << 15u))) + _1378) * _1377;
    uint _1384 = _1382 ^ (_1382 >> 11u);
    uint _1387 = _1384 ^ ((_1384 << 7u) & 2636928640u);
    uint _1390 = _1387 ^ ((_1387 << 15u) & 4022730752u);
    bool _1393 = _1355 ? _1355 : true;
    uint _1394 = _1355 ? _1356 : 8388607u;
    uint _1395 = _1355 ? _1357 : 1065353216u;
    uint2 _1401 = uint2(_1285 + int2(1, 1));
    uint _1403 = _1401.y;
    uint _1405 = _1403 + (_1403 << 10u);
    uint _1407 = _1405 ^ (_1405 >> 6u);
    uint _1409 = _1407 + (_1407 << 3u);
    uint _1411 = _1409 ^ (_1409 >> 11u);
    uint _1414 = _1376 ? _1377 : 1664525u;
    uint _1419 = (((_1401.x * _1414) + (_1411 + (_1411 << 15u))) + (_1376 ? _1378 : 1013904223u)) * _1414;
    uint _1421 = _1419 ^ (_1419 >> 11u);
    uint _1424 = _1421 ^ ((_1421 << 7u) & 2636928640u);
    uint _1427 = _1424 ^ ((_1424 << 15u) & 4022730752u);
    bool _1430 = _1393 ? _1393 : true;
    uint _1431 = _1393 ? _1394 : 8388607u;
    uint _1432 = _1393 ? _1395 : 1065353216u;
    float2 _1440 = (_1286 * _1286) * (3.0f.xx - (_1286 * 2.0f));
    float _1441 = _1440.x;
    float _1442 = 1.0f - _1441;
    float _1444 = _1440.y;
    float4x4 _1460 = float4x4(frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 624), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 640), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 656), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 672), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 628), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 644), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 660), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 676), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 632), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 648), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 664), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 680), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 636), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 652), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 668), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 684));
    float2 _1471 = ((_1260 * 0.2860000133514404296875f) * 0.5f) + ((_1259.yy * 0.2860000133514404296875f) * 0.5f);
    float2 _1475 = floor(_1471 + dot(_1471, 0.3660254180431365966796875f.xx).xx);
    float2 _1479 = (_1471 - _1475) + dot(_1475, 0.211324870586395263671875f.xx).xx;
    float _1480 = _1479.x;
    float _1481 = _1479.y;
    bool2 _1483 = (_1480 > _1481).xx;
    float2 _1484 = float2(_1483.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _1483.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 _1486 = _1479.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 _1488 = _1486.xy - _1484;
    float4 _1489 = float4(_1488.x, _1488.y, _1486.z, _1486.w);
    float2 _1490 = fmod(_1475, 289.0f.xx);
    float3 _1495 = _1490.y.xxx + float3(0.0f, _1484.y, 1.0f);
    float3 _1505 = (fmod(((_1495 * 34.0f) + 1.0f.xxx) * _1495, 289.0f.xxx) + _1490.x.xxx) + float3(0.0f, _1484.x, 1.0f);
    float2 _1511 = _1488.xy;
    float2 _1513 = _1486.zw;
    float3 _1517 = max(0.5f.xxx - float3(dot(_1479, _1479), dot(_1511, _1511), dot(_1513, _1513)), 0.0f.xxx);
    float3 _1518 = _1517 * _1517;
    float3 _1522 = frac(fmod(((_1505 * 34.0f) + 1.0f.xxx) * _1505, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 _1523 = _1522 - 1.0f.xxx;
    float3 _1525 = abs(_1523) - 0.5f.xxx;
    float3 _1528 = _1523 - floor(_1522 + (-0.5f).xxx);
    float3 _1540;
    _1540.x = (_1528.x * _1480) + (_1525.x * _1481);
    float2 _1547 = (_1528.yz * _1489.xz) + (_1525.yz * _1489.yw);
    _295.w = _294 + dot(_437 - _1259, _295.xyz);
    uint _1557 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u;
    uint _1559 = _1557 + (_1557 << 10u);
    uint _1561 = _1559 ^ (_1559 >> 6u);
    uint _1563 = _1561 + (_1561 << 3u);
    uint _1565 = _1563 ^ (_1563 >> 11u);
    bool _1568 = _1430 ? _1430 : true;
    uint _1569 = _1430 ? _1431 : 8388607u;
    uint _1570 = _1430 ? _1432 : 1065353216u;
    uint _1578 = _1557 + 34568u;
    uint _1580 = _1578 + (_1578 << 10u);
    uint _1582 = _1580 ^ (_1580 >> 6u);
    uint _1584 = _1582 + (_1582 << 3u);
    uint _1586 = _1584 ^ (_1584 >> 11u);
    float2 _1602 = float2(round((asfloat(((_1565 + (_1565 << 15u)) & _1569) | _1570) - 1.0f) * 5.0f) * 0.20000000298023223876953125f, round((asfloat(((_1586 + (_1586 << 15u)) & (_1568 ? _1569 : 8388607u)) | (_1568 ? _1570 : 1065353216u)) - 1.0f) * 5.0f) * 0.20000000298023223876953125f);
    float _1616;
    if (!_317)
    {
        _1616 = clamp(_438 * 0.0500000007450580596923828125f, 0.0f, 1.0f) * 0.699999988079071044921875f;
    }
    else
    {
        _1616 = _316;
    }
    gl_Position = mul(_1460, float4(_1259, 1.0f));
    out_var_C2 = _1259;
    out_var_C3 = pow(frac((((1.0f - _1444) * (((asfloat(((_1314 ^ (_1314 >> 18u)) & _1318) | _1319) - 1.0f) * _1442) + ((asfloat(((_1352 ^ (_1352 >> 18u)) & _1356) | _1357) - 1.0f) * _1441))) + (((asfloat(((_1390 ^ (_1390 >> 18u)) & _1394) | _1395) - 1.0f) * _1442) * _1444)) + (((asfloat(((_1427 ^ (_1427 >> 18u)) & _1431) | _1432) - 1.0f) * _1441) * _1444)), 1.5f);
    out_var_C4 = dot((_1518 * _1518) * (1.792842864990234375f.xxx - (((_1528 * _1528) + (_1525 * _1525)) * 0.8537347316741943359375f)), float3(_1540.x, _1547.x, _1547.y)) * 32.5f;
    out_var_C5 = _1616;
    out_var_C6 = (((in_var_Vertex_Position.xy * float2(0.85000002384185791015625f, 0.920000016689300537109375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + _1602;
    out_var_C7 = _295;
    out_var_C8 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C9 = _338.z;
    out_var_C10 = (((in_var_Vertex_Position.yz * float2(0.800000011920928955078125f, 0.89999997615814208984375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + _1602;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    stage_output.out_var_C10 = out_var_C10;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_instanced_roof.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
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

static bool _176;
static uint _177;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_moss_texture_dims : packoffset(c0.y);
    uint2 _Globals_roof_tile_normal_dims : packoffset(c1);
    uint2 _Globals_roof_tile_damage_dims : packoffset(c1.z);
    uint _Globals_highlighted_wall_id : packoffset(c2);
    uint _Globals_highlight_mode : packoffset(c2.y);
};

ByteAddressBuffer roof_color_ids : register(t17, space0);
ByteAddressBuffer instanced_data : register(t21, space0);
ByteAddressBuffer dormer_holes : register(t22, space0);
ByteAddressBuffer dormer_hole_count : register(t23, space0);
SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> roof_colors : register(t16, space0);
Texture2D<float4> roof_tile_normal : register(t19, space0);
Texture2D<float4> roof_tile_damage : register(t20, space0);

static float3 in_var_C2;
static float in_var_C3;
static float in_var_C4;
static float in_var_C5;
static float2 in_var_C6;
static float4 in_var_C7;
static uint in_var_C8;
static float in_var_C9;
static float2 in_var_C10;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float in_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float in_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float in_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 in_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation float4 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float in_var_C9 : TEXCOORD9;
    [[vk::location(10)]] float2 in_var_C10 : TEXCOORD10;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    bool _210;
    if (1u == _Globals_highlight_mode)
    {
        _210 = instanced_data.Load<uint>(in_var_C8 * 64 + 40) == _Globals_highlighted_wall_id;
    }
    else
    {
        _210 = false;
    }
    bool _217;
    if (2u == _Globals_highlight_mode)
    {
        _217 = instanced_data.Load<uint>(in_var_C8 * 64 + 40) == _Globals_highlighted_wall_id;
    }
    else
    {
        _217 = false;
    }
    float3 _220 = ddx(in_var_C2);
    float3 _221 = ddy(in_var_C2);
    float3 _224 = normalize(cross(_220, -_221));
    if (dot(float4(in_var_C2, 1.0f), in_var_C7) < 0.0f)
    {
        discard;
    }
    if (instanced_data.Load<int>(in_var_C8 * 64 + 60) != 0)
    {
        int _236 = int(instanced_data.Load<uint>(in_var_C8 * 64 + 40));
        bool _334;
        do
        {
            float2 _241 = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
            int _249 = int(floor(_241.x)) + (15 * int(floor(_241.y)));
            uint _253 = min(128u, dormer_hole_count.Load<uint>(uint(_249) * 4 + 0));
            bool _331;
            bool _332;
            uint _255 = 0u;
            for (;;)
            {
                if (_255 < _253)
                {
                    uint _265 = uint(int(uint(_249 * 50) + _255));
                    if (_236 == dormer_holes.Load<int>(_265 * 48 + 12))
                    {
                        float3 _281 = float3(dormer_holes.Load<float>(_265 * 48 + 32), 0.0f, dormer_holes.Load<float>(_265 * 48 + 36));
                        float3x3 _285 = transpose(float3x3(cross(_281, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), _281));
                        float3 _286 = mul(_285, float3(1.0f, 0.0f, 0.0f));
                        float3 _287 = mul(_285, float3(0.0f, 1.0f, 0.0f));
                        float3 _288 = mul(_285, float3(0.0f, 0.0f, 1.0f));
                        float3 _291 = cross(_286, _287);
                        float3 _294 = (1.0f / dot(_288, _291)).xxx;
                        float3 _302 = mul(in_var_C2 - dormer_holes.Load<float3>(_265 * 48 + 0), float3x3(cross(_287, _288) * _294, cross(_288, _286) * _294, _291 * _294)) / dormer_holes.Load<float3>(_265 * 48 + 16);
                        float _303 = _302.y;
                        float _306 = clamp((_303 * dormer_holes.Load<float>(_265 * 48 + 40)) + dormer_holes.Load<float>(_265 * 48 + 44), 0.0f, 1.0f);
                        bool _322;
                        if (abs(_302.x) < lerp(0.5f, 0.0f, (_306 * _306) * _306))
                        {
                            bool _321;
                            if (_303 < ((dormer_holes.Load<float>(_265 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                            {
                                _321 = _303 > (-0.5f);
                            }
                            else
                            {
                                _321 = false;
                            }
                            _322 = _321;
                        }
                        else
                        {
                            _322 = false;
                        }
                        bool _328;
                        if (_322)
                        {
                            _328 = abs(_302.z) < 0.5f;
                        }
                        else
                        {
                            _328 = false;
                        }
                        if (_328)
                        {
                            _331 = true;
                            _332 = true;
                            break;
                        }
                    }
                    _255++;
                    continue;
                }
                else
                {
                    _331 = _176;
                    _332 = false;
                    break;
                }
            }
            if (_332)
            {
                _334 = _331;
                break;
            }
            _334 = false;
            break;
        } while(false);
        if (_334)
        {
            discard;
        }
    }
    bool _337 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 10u;
    float4 _348 = roof_colors.Sample(sampler_llr, float3(in_var_C2.xz * 0.20000000298023223876953125f, float(_337 ? 7 : 1)));
    float3 _351 = (_348.xyz * 0.60000002384185791015625f) * 1.5f;
    bool _353 = !(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 0u);
    bool _357;
    if (_353)
    {
        _357 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 8u;
    }
    else
    {
        _357 = true;
    }
    bool _362;
    if (!_357)
    {
        _362 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 7u;
    }
    else
    {
        _362 = true;
    }
    bool _367;
    if (!_362)
    {
        _367 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 6u;
    }
    else
    {
        _367 = true;
    }
    float3 _377;
    if (_367)
    {
        _377 = _351 * 1.5f;
    }
    else
    {
        float3 _375;
        if (roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 1u)
        {
            _375 = _351 * 0.3499999940395355224609375f;
        }
        else
        {
            _375 = _351;
        }
        _377 = _375;
    }
    bool _383;
    if (!(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 2u))
    {
        _383 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 3u;
    }
    else
    {
        _383 = true;
    }
    bool _388;
    if (!_383)
    {
        _388 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 4u;
    }
    else
    {
        _388 = true;
    }
    bool _393;
    if (!_388)
    {
        _393 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 6u;
    }
    else
    {
        _393 = true;
    }
    bool _398;
    if (!_393)
    {
        _398 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 9u;
    }
    else
    {
        _398 = true;
    }
    float _424;
    if ((!_398) ? _337 : true)
    {
        _424 = 0.5f;
    }
    else
    {
        bool _407;
        if (_353)
        {
            _407 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 5u;
        }
        else
        {
            _407 = true;
        }
        bool _412;
        if (!_407)
        {
            _412 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 7u;
        }
        else
        {
            _412 = true;
        }
        bool _417;
        if (!_412)
        {
            _417 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 8u;
        }
        else
        {
            _417 = true;
        }
        float _423;
        if (_417)
        {
            _423 = 1.0f;
        }
        else
        {
            _423 = (roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 1u) ? 2.0f : 0.75f;
        }
        _424 = _423;
    }
    bool _429;
    if (in_var_C9 > (-0.949999988079071044921875f))
    {
        _429 = in_var_C9 < 0.89999997615814208984375f;
    }
    else
    {
        _429 = false;
    }
    bool2 _430 = _429.xx;
    float4 _436 = roof_tile_damage.SampleBias(sampler_llr, float2(_430.x ? in_var_C10.x : in_var_C6.x, _430.y ? in_var_C10.y : in_var_C6.y), frame_constants_storage_buffer.Load<float>(208));
    float _438 = 1.0f - _436.x;
    float _439 = _438 * in_var_C3;
    float3 _444 = in_var_C2 * 0.100000001490116119384765625f;
    bool _445 = _439 > 0.00999999977648258209228515625f;
    float3 _466;
    if (_445)
    {
        uint _449 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 2834);
        uint _451 = _449 + (_449 << 10u);
        uint _453 = _451 ^ (_451 >> 6u);
        uint _455 = _453 + (_453 << 3u);
        uint _457 = _455 ^ (_455 >> 11u);
        _466 = _444 + (asfloat(((_457 + (_457 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx;
    }
    else
    {
        _466 = _444;
    }
    bool _469 = _445 ? true : false;
    uint _471 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 34534);
    uint _473 = _471 + (_471 << 10u);
    uint _475 = _473 ^ (_473 >> 6u);
    uint _477 = _475 + (_475 << 3u);
    uint _479 = _477 ^ (_477 >> 11u);
    bool _482 = _469 ? _469 : true;
    uint _483 = _469 ? (_445 ? 8388607u : _177) : 8388607u;
    uint _484 = _469 ? (_445 ? 1065353216u : _177) : 1065353216u;
    float3 _513;
    if ((asfloat(((_479 + (_479 << 15u)) & _483) | _484) - 1.0f) < 0.5f)
    {
        uint _493 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 234);
        uint _495 = _493 + (_493 << 10u);
        uint _497 = _495 ^ (_495 >> 6u);
        uint _499 = _497 + (_497 << 3u);
        uint _501 = _499 ^ (_499 >> 11u);
        _513 = _466 + ((asfloat(((_501 + (_501 << 15u)) & (_482 ? _483 : 8388607u)) | (_482 ? _484 : 1065353216u)) - 1.0f) * 0.4000000059604644775390625f).xxx;
    }
    else
    {
        _513 = _466;
    }
    float _514 = float(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0));
    float4 _519 = roof_colors.Sample(sampler_llr, float3(_513.zy, _514));
    float3 _520 = _519.xyz;
    float4 _524 = roof_colors.Sample(sampler_llr, float3(_513.xy, _514));
    float3 _525 = _524.xyz;
    float3 _527 = pow(abs(_224), 1.0f.xxx);
    float _532 = dot(_520, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + _527.x;
    float _533 = dot(_525, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + _527.z;
    float _535 = max(_532, _533) - 0.20000000298023223876953125f;
    float _537 = max(_532 - _535, 0.0f);
    float _539 = max(_533 - _535, 0.0f);
    float3 _546 = (((_520 * _537) + (_525 * _539)) / (_537 + _539).xxx) * 0.60000002384185791015625f;
    float3 _556 = lerp(lerp(_546 * 0.89999997615814208984375f, _546 * 1.2999999523162841796875f, clamp(1.0f - (in_var_C3 * 3.0f), 0.0f, 1.0f).xxx), _377, clamp((clamp(_439 * 5.0f, 0.0f, 1.0f) * _424) + in_var_C4, 0.0f, 1.0f).xxx);
    float3 _559 = lerp(_556, _556 * 0.89999997615814208984375f, _439.xxx);
    float3 _564;
    if (_210)
    {
        _564 = lerp(_559, float3(0.300000011920928955078125f, 0.0900000035762786865234375f, 0.0f), 0.5f.xxx);
    }
    else
    {
        _564 = _559;
    }
    bool _567 = int(_217) == 1;
    float3 _571;
    if (_567)
    {
        _571 = lerp(_564, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
    }
    else
    {
        _571 = _564;
    }
    float3 _611;
    if (in_var_C9 > 0.999000012874603271484375f)
    {
        float2 _579 = ddx(in_var_C6);
        float2 _581 = -ddy(in_var_C6);
        float3 _582 = cross(_224, -ddy(in_var_C2));
        float3 _583 = cross(ddx(in_var_C2), _224);
        float _584 = _579.x;
        float _585 = _581.y;
        float _587 = _579.y;
        float _588 = _581.x;
        float _591 = 1.0f / ((_584 * _585) - (_587 * _588));
        _611 = normalize(mul(transpose(float3x3(((_582 * _584) + (_583 * _588)) * _591, ((_582 * _587) + (_583 * _585)) * _591, _224)), (roof_tile_normal.SampleBias(sampler_llr, in_var_C6, frame_constants_storage_buffer.Load<float>(208)).xyz * 2.0f) - 1.0f.xxx));
    }
    else
    {
        _611 = _224;
    }
    float3 _625;
    if (frame_constants_storage_buffer.Load<uint>(140) == 4u)
    {
        _625 = lerp(_571, 0.699999988079071044921875f.xxx, ((smoothstep(0.100000001490116119384765625f, 1.0f, in_var_C5) * 0.5f) + ((_438 * 0.5f) * smoothstep(0.0f, 0.5f, in_var_C5))).xxx);
    }
    else
    {
        _625 = _571;
    }
    float3 _627 = (_611 * 0.5f) + 0.5f.xxx;
    float3 _654 = sqrt(clamp(_625, 0.0f.xxx, 1.0f.xxx));
    uint _673 = (uint((clamp(_654.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_654.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_654.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _674 = uint2(((uint((clamp(_627.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_627.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_627.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(lerp(0.73500001430511474609375f, 1.0f, _439), 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), _673);
    _674.y = _673 | (((((7u | (uint(_567 ? true : (_210 ? true : false)) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _674;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C4 = stage_input.in_var_C4;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C6 = stage_input.in_var_C6;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    in_var_C10 = stage_input.in_var_C10;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_instanced_roof.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct FrameConstants
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
};

static uint _180;
static float3 _207;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint _Globals_current_view_index : packoffset(c0);
    uint2 _Globals_moss_texture_dims : packoffset(c0.y);
    uint2 _Globals_roof_tile_normal_dims : packoffset(c1);
    uint2 _Globals_roof_tile_damage_dims : packoffset(c1.z);
    uint _Globals_highlighted_wall_id : packoffset(c2);
    uint _Globals_highlight_mode : packoffset(c2.y);
};

ByteAddressBuffer instance_indirection_buffer : register(t15, space0);
ByteAddressBuffer instanced_data : register(t21, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 out_var_C2;
static float out_var_C3;
static float out_var_C4;
static float out_var_C5;
static float2 out_var_C6;
static float4 out_var_C7;
static uint out_var_C8;
static float out_var_C9;
static float2 out_var_C10;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float out_var_C3 : TEXCOORD3;
    [[vk::location(4)]] float out_var_C4 : TEXCOORD4;
    [[vk::location(5)]] float out_var_C5 : TEXCOORD5;
    [[vk::location(6)]] float2 out_var_C6 : TEXCOORD6;
    [[vk::location(7)]] nointerpolation float4 out_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint out_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float out_var_C9 : TEXCOORD9;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
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
    float2 _218 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x & 65535u);
    float _219 = _218.x;
    float2 _222 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x >> 16u) & 65535u);
    float _223 = _222.x;
    float2 _226 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y & 65535u);
    float _227 = _226.x;
    float2 _230 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y >> 16u) & 65535u);
    float _231 = _230.x;
    float _234 = _219 + _219;
    float _235 = _223 + _223;
    float _236 = _227 + _227;
    float _237 = _219 * _234;
    float _238 = _219 * _235;
    float _239 = _219 * _236;
    float _240 = _223 * _235;
    float _241 = _223 * _236;
    float _242 = _227 * _236;
    float _243 = _231 * _234;
    float _244 = _231 * _235;
    float _245 = _231 * _236;
    float3x4 _262 = transpose(float4x3(float3(1.0f - (_240 + _242), _238 + _245, _239 - _244), float3(_238 - _245, 1.0f - (_237 + _242), _241 + _243), float3(_239 + _244, _241 - _243, 1.0f - (_237 + _240)), instanced_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0)));
    float2 _293 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y >> 16u) & 65535u);
    float _294 = _293.x;
    float4 _295 = float4(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y & 65535u).x, _294);
    float2 _315 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y >> 16u) & 65535u);
    float _316 = _315.x;
    bool _317 = instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) != 0;
    float3 _322 = (float3(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y & 65535u).x) * smoothstep(_316 * 0.800000011920928955078125f, _316, pow(instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0.800000011920928955078125f))) * float3(0.505970001220703125f, 0.5f, 0.5f);
    float _324 = _322.x * 0.949999988079071044921875f;
    float3 _325 = _322;
    _325.x = _324;
    float3 _326 = _325 * float3(3.58920383453369140625f, 3.70370388031005859375f, 3.70370388031005859375f);
    float _337 = max(min(max(_324, max(_322.y, _322.z)), min(_326.x, min(_326.y, _326.z))), 9.9999997473787516355514526367188e-06f);
    float3 _338 = in_var_Vertex_Position * float3(1.976401805877685546875f, 2.0f, 2.0f);
    float3 _339 = _338 * _337;
    bool3 _342 = bool3(_339.x >= 0.0f.xxx.x, _339.y >= 0.0f.xxx.y, _339.z >= 0.0f.xxx.z);
    float2 _349 = spvUnpackHalf2x16((instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) >> 16u) & 65535u);
    float _350 = _349.x;
    float3 _351 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _352 = sin(_350);
    float _353 = cos(_350);
    float _354 = 1.0f - _353;
    float _355 = _351.x;
    float _356 = _354 * _355;
    float _359 = _351.y;
    float _360 = _356 * _359;
    float _361 = _351.z;
    float _362 = _361 * _352;
    float _364 = _354 * _361;
    float _365 = _364 * _355;
    float _366 = _359 * _352;
    float _369 = _354 * _359;
    float _372 = _369 * _361;
    float _373 = _355 * _352;
    float3 _384 = mul(transpose(float3x3(float3((_356 * _355) + _353, _360 - _362, _365 + _366), float3(_360 + _362, (_369 * _359) + _353, _372 - _373), float3(_365 - _366, _372 + _373, (_364 * _361) + _353))), _339 - ((_337.xxx - _325) * float3(int3(_342.x ? int3(1, 1, 1).x : int3(-1, -1, -1).x, _342.y ? int3(1, 1, 1).y : int3(-1, -1, -1).y, _342.z ? int3(1, 1, 1).z : int3(-1, -1, -1).z))));
    float3 _422;
    if (_317)
    {
        uint _389 = (uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u) + instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 40);
        uint _391 = _389 + (_389 << 10u);
        uint _393 = _391 ^ (_391 >> 6u);
        uint _395 = _393 + (_393 << 3u);
        uint _397 = _395 ^ (_395 >> 11u);
        float _405 = ((asfloat(((_397 + (_397 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 0.0199999995529651641845703125f : 0.0599999986588954925537109375f;
        float3 _421;
        if (in_var_Vertex_Position.y > 0.100000001490116119384765625f)
        {
            float3 _420;
            if (instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) != 0)
            {
                float3 _419 = _384;
                _419.z = _384.z - _405;
                _420 = _419;
            }
            else
            {
                float3 _416 = _384;
                _416.z = _384.z + _405;
                _420 = _416;
            }
            _421 = _420;
        }
        else
        {
            _421 = _384;
        }
        _422 = _421;
    }
    else
    {
        _422 = _384;
    }
    uint _423 = _317 ? 1065353216u : _180;
    uint _424 = _317 ? 8388607u : _180;
    bool _425 = _317 ? true : false;
    float3 _437 = mul(_262, float4(_422.xy, _422.z + ((1.0f - smoothstep(0.0f, 0.800000011920928955078125f, mul(_262, float4(_422, 1.0f)).y)) * 0.115000002086162567138671875f), 1.0f));
    float _438 = _437.y;
    float _439 = _438 + instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12);
    float3 _440 = _437;
    _440.y = _439;
    float3 _610;
    uint _611;
    uint _612;
    bool _613;
    if (_317)
    {
        float2 _445 = _440.xz * 0.5f;
        int2 _447 = int2(floor(_445));
        float2 _448 = frac(_445);
        uint2 _449 = uint2(_447);
        uint _451 = _449.y;
        uint _453 = _451 + (_451 << 10u);
        uint _455 = _453 ^ (_453 >> 6u);
        uint _457 = _455 + (_455 << 3u);
        uint _459 = _457 ^ (_457 >> 11u);
        uint _465 = (((_449.x * 1664525u) + (_459 + (_459 << 15u))) + 1013904223u) * 1664525u;
        uint _467 = _465 ^ (_465 >> 11u);
        uint _470 = _467 ^ ((_467 << 7u) & 2636928640u);
        uint _473 = _470 ^ ((_470 << 15u) & 4022730752u);
        bool _476 = _425 ? _425 : true;
        uint _477 = _425 ? _424 : 8388607u;
        uint _478 = _425 ? _423 : 1065353216u;
        uint2 _484 = uint2(_447 + int2(1, 0));
        uint _486 = _484.y;
        uint _488 = _486 + (_486 << 10u);
        uint _490 = _488 ^ (_488 >> 6u);
        uint _492 = _490 + (_490 << 3u);
        uint _494 = _492 ^ (_492 >> 11u);
        uint _500 = (((_484.x * 1664525u) + (_494 + (_494 << 15u))) + 1013904223u) * 1664525u;
        uint _502 = _500 ^ (_500 >> 11u);
        uint _505 = _502 ^ ((_502 << 7u) & 2636928640u);
        uint _508 = _505 ^ ((_505 << 15u) & 4022730752u);
        bool _511 = _476 ? _476 : true;
        uint _512 = _476 ? _477 : 8388607u;
        uint _513 = _476 ? _478 : 1065353216u;
        uint2 _519 = uint2(_447 + int2(0, 1));
        uint _521 = _519.y;
        uint _523 = _521 + (_521 << 10u);
        uint _525 = _523 ^ (_523 >> 6u);
        uint _527 = _525 + (_525 << 3u);
        uint _529 = _527 ^ (_527 >> 11u);
        uint _535 = (((_519.x * 1664525u) + (_529 + (_529 << 15u))) + 1013904223u) * 1664525u;
        uint _537 = _535 ^ (_535 >> 11u);
        uint _540 = _537 ^ ((_537 << 7u) & 2636928640u);
        uint _543 = _540 ^ ((_540 << 15u) & 4022730752u);
        bool _546 = _511 ? _511 : true;
        uint _547 = _511 ? _512 : 8388607u;
        uint _548 = _511 ? _513 : 1065353216u;
        uint2 _554 = uint2(_447 + int2(1, 1));
        uint _556 = _554.y;
        uint _558 = _556 + (_556 << 10u);
        uint _560 = _558 ^ (_558 >> 6u);
        uint _562 = _560 + (_560 << 3u);
        uint _564 = _562 ^ (_562 >> 11u);
        uint _570 = (((_554.x * 1664525u) + (_564 + (_564 << 15u))) + 1013904223u) * 1664525u;
        uint _572 = _570 ^ (_570 >> 11u);
        uint _575 = _572 ^ ((_572 << 7u) & 2636928640u);
        uint _578 = _575 ^ ((_575 << 15u) & 4022730752u);
        uint _582 = _546 ? _547 : 8388607u;
        uint _583 = _546 ? _548 : 1065353216u;
        float2 _591 = (_448 * _448) * (3.0f.xx - (_448 * 2.0f));
        float _592 = _591.x;
        float _593 = 1.0f - _592;
        float _595 = _591.y;
        float3 _609 = _440;
        _609.y = _439 + ((((((1.0f - _595) * (((asfloat(((_473 ^ (_473 >> 18u)) & _477) | _478) - 1.0f) * _593) + ((asfloat(((_508 ^ (_508 >> 18u)) & _512) | _513) - 1.0f) * _592))) + (((asfloat(((_543 ^ (_543 >> 18u)) & _547) | _548) - 1.0f) * _593) * _595)) + (((asfloat(((_578 ^ (_578 >> 18u)) & _582) | _583) - 1.0f) * _592) * _595)) * 0.300000011920928955078125f) * smoothstep(0.0f, 1.0f, _438));
        _610 = _609;
        _611 = _583;
        _612 = _582;
        _613 = _546 ? _546 : true;
    }
    else
    {
        _610 = _440;
        _611 = _423;
        _612 = _424;
        _613 = _425;
    }
    float3 _616 = _610 * 0.4000000059604644775390625f;
    int3 _618 = int3(floor(_616));
    float3 _619 = frac(_616);
    uint3 _620 = uint3(_618);
    uint _623 = _620.z;
    uint _625 = _623 + (_623 << 10u);
    uint _627 = _625 ^ (_625 >> 6u);
    uint _629 = _627 + (_627 << 3u);
    uint _631 = _629 ^ (_629 >> 11u);
    bool _634 = _425 ? _425 : true;
    uint _635 = _425 ? (_317 ? 1664525u : _180) : 1664525u;
    uint _636 = _425 ? (_317 ? 1013904223u : _180) : 1013904223u;
    uint _640 = (((_620.y * _635) + (_631 + (_631 << 15u))) + _636) * _635;
    uint _642 = _640 ^ (_640 >> 11u);
    uint _645 = _642 ^ ((_642 << 7u) & 2636928640u);
    uint _648 = _645 ^ ((_645 << 15u) & 4022730752u);
    bool _651 = _634 ? _634 : true;
    uint _652 = _634 ? _635 : 1664525u;
    uint _653 = _634 ? _636 : 1013904223u;
    uint _657 = (((_620.x * _652) + (_648 ^ (_648 >> 18u))) + _653) * _652;
    uint _659 = _657 ^ (_657 >> 11u);
    uint _662 = _659 ^ ((_659 << 7u) & 2636928640u);
    uint _665 = _662 ^ ((_662 << 15u) & 4022730752u);
    uint _667 = _665 ^ (_665 >> 18u);
    uint _669 = _667 + (_667 << 10u);
    uint _671 = _669 ^ (_669 >> 6u);
    uint _673 = _671 + (_671 << 3u);
    uint _675 = _673 ^ (_673 >> 11u);
    bool _678 = _613 ? _613 : true;
    uint _679 = _613 ? _612 : 8388607u;
    uint _680 = _613 ? _611 : 1065353216u;
    bool _685 = _678 ? _678 : true;
    uint _686 = _678 ? _679 : 8388607u;
    uint _687 = _678 ? _680 : 1065353216u;
    uint3 _694 = uint3(_618 + int3(1, 0, 0));
    uint _697 = _694.z;
    uint _699 = _697 + (_697 << 10u);
    uint _701 = _699 ^ (_699 >> 6u);
    uint _703 = _701 + (_701 << 3u);
    uint _705 = _703 ^ (_703 >> 11u);
    bool _708 = _651 ? _651 : true;
    uint _709 = _651 ? _652 : 1664525u;
    uint _710 = _651 ? _653 : 1013904223u;
    uint _714 = (((_694.y * _709) + (_705 + (_705 << 15u))) + _710) * _709;
    uint _716 = _714 ^ (_714 >> 11u);
    uint _719 = _716 ^ ((_716 << 7u) & 2636928640u);
    uint _722 = _719 ^ ((_719 << 15u) & 4022730752u);
    bool _725 = _708 ? _708 : true;
    uint _726 = _708 ? _709 : 1664525u;
    uint _727 = _708 ? _710 : 1013904223u;
    uint _731 = (((_694.x * _726) + (_722 ^ (_722 >> 18u))) + _727) * _726;
    uint _733 = _731 ^ (_731 >> 11u);
    uint _736 = _733 ^ ((_733 << 7u) & 2636928640u);
    uint _739 = _736 ^ ((_736 << 15u) & 4022730752u);
    uint _741 = _739 ^ (_739 >> 18u);
    uint _743 = _741 + (_741 << 10u);
    uint _745 = _743 ^ (_743 >> 6u);
    uint _747 = _745 + (_745 << 3u);
    uint _749 = _747 ^ (_747 >> 11u);
    bool _752 = _685 ? _685 : true;
    uint _753 = _685 ? _686 : 8388607u;
    uint _754 = _685 ? _687 : 1065353216u;
    bool _759 = _752 ? _752 : true;
    uint _760 = _752 ? _753 : 8388607u;
    uint _761 = _752 ? _754 : 1065353216u;
    uint3 _768 = uint3(_618 + int3(0, 1, 0));
    uint _771 = _768.z;
    uint _773 = _771 + (_771 << 10u);
    uint _775 = _773 ^ (_773 >> 6u);
    uint _777 = _775 + (_775 << 3u);
    uint _779 = _777 ^ (_777 >> 11u);
    bool _782 = _725 ? _725 : true;
    uint _783 = _725 ? _726 : 1664525u;
    uint _784 = _725 ? _727 : 1013904223u;
    uint _788 = (((_768.y * _783) + (_779 + (_779 << 15u))) + _784) * _783;
    uint _790 = _788 ^ (_788 >> 11u);
    uint _793 = _790 ^ ((_790 << 7u) & 2636928640u);
    uint _796 = _793 ^ ((_793 << 15u) & 4022730752u);
    bool _799 = _782 ? _782 : true;
    uint _800 = _782 ? _783 : 1664525u;
    uint _801 = _782 ? _784 : 1013904223u;
    uint _805 = (((_768.x * _800) + (_796 ^ (_796 >> 18u))) + _801) * _800;
    uint _807 = _805 ^ (_805 >> 11u);
    uint _810 = _807 ^ ((_807 << 7u) & 2636928640u);
    uint _813 = _810 ^ ((_810 << 15u) & 4022730752u);
    uint _815 = _813 ^ (_813 >> 18u);
    uint _817 = _815 + (_815 << 10u);
    uint _819 = _817 ^ (_817 >> 6u);
    uint _821 = _819 + (_819 << 3u);
    uint _823 = _821 ^ (_821 >> 11u);
    bool _826 = _759 ? _759 : true;
    uint _827 = _759 ? _760 : 8388607u;
    uint _828 = _759 ? _761 : 1065353216u;
    bool _833 = _826 ? _826 : true;
    uint _834 = _826 ? _827 : 8388607u;
    uint _835 = _826 ? _828 : 1065353216u;
    uint3 _842 = uint3(_618 + int3(1, 1, 0));
    uint _845 = _842.z;
    uint _847 = _845 + (_845 << 10u);
    uint _849 = _847 ^ (_847 >> 6u);
    uint _851 = _849 + (_849 << 3u);
    uint _853 = _851 ^ (_851 >> 11u);
    bool _856 = _799 ? _799 : true;
    uint _857 = _799 ? _800 : 1664525u;
    uint _858 = _799 ? _801 : 1013904223u;
    uint _862 = (((_842.y * _857) + (_853 + (_853 << 15u))) + _858) * _857;
    uint _864 = _862 ^ (_862 >> 11u);
    uint _867 = _864 ^ ((_864 << 7u) & 2636928640u);
    uint _870 = _867 ^ ((_867 << 15u) & 4022730752u);
    bool _873 = _856 ? _856 : true;
    uint _874 = _856 ? _857 : 1664525u;
    uint _875 = _856 ? _858 : 1013904223u;
    uint _879 = (((_842.x * _874) + (_870 ^ (_870 >> 18u))) + _875) * _874;
    uint _881 = _879 ^ (_879 >> 11u);
    uint _884 = _881 ^ ((_881 << 7u) & 2636928640u);
    uint _887 = _884 ^ ((_884 << 15u) & 4022730752u);
    uint _889 = _887 ^ (_887 >> 18u);
    uint _891 = _889 + (_889 << 10u);
    uint _893 = _891 ^ (_891 >> 6u);
    uint _895 = _893 + (_893 << 3u);
    uint _897 = _895 ^ (_895 >> 11u);
    bool _900 = _833 ? _833 : true;
    uint _901 = _833 ? _834 : 8388607u;
    uint _902 = _833 ? _835 : 1065353216u;
    bool _907 = _900 ? _900 : true;
    uint _908 = _900 ? _901 : 8388607u;
    uint _909 = _900 ? _902 : 1065353216u;
    uint3 _916 = uint3(_618 + int3(0, 0, 1));
    uint _919 = _916.z;
    uint _921 = _919 + (_919 << 10u);
    uint _923 = _921 ^ (_921 >> 6u);
    uint _925 = _923 + (_923 << 3u);
    uint _927 = _925 ^ (_925 >> 11u);
    bool _930 = _873 ? _873 : true;
    uint _931 = _873 ? _874 : 1664525u;
    uint _932 = _873 ? _875 : 1013904223u;
    uint _936 = (((_916.y * _931) + (_927 + (_927 << 15u))) + _932) * _931;
    uint _938 = _936 ^ (_936 >> 11u);
    uint _941 = _938 ^ ((_938 << 7u) & 2636928640u);
    uint _944 = _941 ^ ((_941 << 15u) & 4022730752u);
    bool _947 = _930 ? _930 : true;
    uint _948 = _930 ? _931 : 1664525u;
    uint _949 = _930 ? _932 : 1013904223u;
    uint _953 = (((_916.x * _948) + (_944 ^ (_944 >> 18u))) + _949) * _948;
    uint _955 = _953 ^ (_953 >> 11u);
    uint _958 = _955 ^ ((_955 << 7u) & 2636928640u);
    uint _961 = _958 ^ ((_958 << 15u) & 4022730752u);
    uint _963 = _961 ^ (_961 >> 18u);
    uint _965 = _963 + (_963 << 10u);
    uint _967 = _965 ^ (_965 >> 6u);
    uint _969 = _967 + (_967 << 3u);
    uint _971 = _969 ^ (_969 >> 11u);
    bool _974 = _907 ? _907 : true;
    uint _975 = _907 ? _908 : 8388607u;
    uint _976 = _907 ? _909 : 1065353216u;
    bool _981 = _974 ? _974 : true;
    uint _982 = _974 ? _975 : 8388607u;
    uint _983 = _974 ? _976 : 1065353216u;
    uint3 _990 = uint3(_618 + int3(1, 0, 1));
    uint _993 = _990.z;
    uint _995 = _993 + (_993 << 10u);
    uint _997 = _995 ^ (_995 >> 6u);
    uint _999 = _997 + (_997 << 3u);
    uint _1001 = _999 ^ (_999 >> 11u);
    bool _1004 = _947 ? _947 : true;
    uint _1005 = _947 ? _948 : 1664525u;
    uint _1006 = _947 ? _949 : 1013904223u;
    uint _1010 = (((_990.y * _1005) + (_1001 + (_1001 << 15u))) + _1006) * _1005;
    uint _1012 = _1010 ^ (_1010 >> 11u);
    uint _1015 = _1012 ^ ((_1012 << 7u) & 2636928640u);
    uint _1018 = _1015 ^ ((_1015 << 15u) & 4022730752u);
    bool _1021 = _1004 ? _1004 : true;
    uint _1022 = _1004 ? _1005 : 1664525u;
    uint _1023 = _1004 ? _1006 : 1013904223u;
    uint _1027 = (((_990.x * _1022) + (_1018 ^ (_1018 >> 18u))) + _1023) * _1022;
    uint _1029 = _1027 ^ (_1027 >> 11u);
    uint _1032 = _1029 ^ ((_1029 << 7u) & 2636928640u);
    uint _1035 = _1032 ^ ((_1032 << 15u) & 4022730752u);
    uint _1037 = _1035 ^ (_1035 >> 18u);
    uint _1039 = _1037 + (_1037 << 10u);
    uint _1041 = _1039 ^ (_1039 >> 6u);
    uint _1043 = _1041 + (_1041 << 3u);
    uint _1045 = _1043 ^ (_1043 >> 11u);
    bool _1048 = _981 ? _981 : true;
    uint _1049 = _981 ? _982 : 8388607u;
    uint _1050 = _981 ? _983 : 1065353216u;
    bool _1055 = _1048 ? _1048 : true;
    uint _1056 = _1048 ? _1049 : 8388607u;
    uint _1057 = _1048 ? _1050 : 1065353216u;
    uint3 _1064 = uint3(_618 + int3(0, 1, 1));
    uint _1067 = _1064.z;
    uint _1069 = _1067 + (_1067 << 10u);
    uint _1071 = _1069 ^ (_1069 >> 6u);
    uint _1073 = _1071 + (_1071 << 3u);
    uint _1075 = _1073 ^ (_1073 >> 11u);
    bool _1078 = _1021 ? _1021 : true;
    uint _1079 = _1021 ? _1022 : 1664525u;
    uint _1080 = _1021 ? _1023 : 1013904223u;
    uint _1084 = (((_1064.y * _1079) + (_1075 + (_1075 << 15u))) + _1080) * _1079;
    uint _1086 = _1084 ^ (_1084 >> 11u);
    uint _1089 = _1086 ^ ((_1086 << 7u) & 2636928640u);
    uint _1092 = _1089 ^ ((_1089 << 15u) & 4022730752u);
    bool _1095 = _1078 ? _1078 : true;
    uint _1096 = _1078 ? _1079 : 1664525u;
    uint _1097 = _1078 ? _1080 : 1013904223u;
    uint _1101 = (((_1064.x * _1096) + (_1092 ^ (_1092 >> 18u))) + _1097) * _1096;
    uint _1103 = _1101 ^ (_1101 >> 11u);
    uint _1106 = _1103 ^ ((_1103 << 7u) & 2636928640u);
    uint _1109 = _1106 ^ ((_1106 << 15u) & 4022730752u);
    uint _1111 = _1109 ^ (_1109 >> 18u);
    uint _1113 = _1111 + (_1111 << 10u);
    uint _1115 = _1113 ^ (_1113 >> 6u);
    uint _1117 = _1115 + (_1115 << 3u);
    uint _1119 = _1117 ^ (_1117 >> 11u);
    bool _1122 = _1055 ? _1055 : true;
    uint _1123 = _1055 ? _1056 : 8388607u;
    uint _1124 = _1055 ? _1057 : 1065353216u;
    bool _1129 = _1122 ? _1122 : true;
    uint _1130 = _1122 ? _1123 : 8388607u;
    uint _1131 = _1122 ? _1124 : 1065353216u;
    uint3 _1138 = uint3(_618 + int3(1, 1, 1));
    uint _1141 = _1138.z;
    uint _1143 = _1141 + (_1141 << 10u);
    uint _1145 = _1143 ^ (_1143 >> 6u);
    uint _1147 = _1145 + (_1145 << 3u);
    uint _1149 = _1147 ^ (_1147 >> 11u);
    bool _1152 = _1095 ? _1095 : true;
    uint _1153 = _1095 ? _1096 : 1664525u;
    uint _1154 = _1095 ? _1097 : 1013904223u;
    uint _1158 = (((_1138.y * _1153) + (_1149 + (_1149 << 15u))) + _1154) * _1153;
    uint _1160 = _1158 ^ (_1158 >> 11u);
    uint _1163 = _1160 ^ ((_1160 << 7u) & 2636928640u);
    uint _1166 = _1163 ^ ((_1163 << 15u) & 4022730752u);
    bool _1169 = _1152 ? _1152 : true;
    uint _1170 = _1152 ? _1153 : 1664525u;
    uint _1171 = _1152 ? _1154 : 1013904223u;
    uint _1175 = (((_1138.x * _1170) + (_1166 ^ (_1166 >> 18u))) + _1171) * _1170;
    uint _1177 = _1175 ^ (_1175 >> 11u);
    uint _1180 = _1177 ^ ((_1177 << 7u) & 2636928640u);
    uint _1183 = _1180 ^ ((_1180 << 15u) & 4022730752u);
    uint _1185 = _1183 ^ (_1183 >> 18u);
    uint _1187 = _1185 + (_1185 << 10u);
    uint _1189 = _1187 ^ (_1187 >> 6u);
    uint _1191 = _1189 + (_1189 << 3u);
    uint _1193 = _1191 ^ (_1191 >> 11u);
    bool _1196 = _1129 ? _1129 : true;
    uint _1197 = _1129 ? _1130 : 8388607u;
    uint _1198 = _1129 ? _1131 : 1065353216u;
    bool _1203 = _1196 ? _1196 : true;
    uint _1204 = _1196 ? _1197 : 8388607u;
    uint _1205 = _1196 ? _1198 : 1065353216u;
    float3 _1214 = (_619 * _619) * (3.0f.xxx - (_619 * 2.0f));
    float _1215 = _1214.x;
    float _1216 = 1.0f - _1215;
    float _1218 = _1214.y;
    float _1219 = 1.0f - _1218;
    float _1221 = _1214.z;
    float _1222 = 1.0f - _1221;
    float2 _1235 = (((((float2(asfloat((_667 & _679) | _680) - 1.0f, asfloat(((_675 + (_675 << 15u)) & _686) | _687) - 1.0f) * _1216) * _1219) * _1222) + (((float2(asfloat((_741 & _753) | _754) - 1.0f, asfloat(((_749 + (_749 << 15u)) & _760) | _761) - 1.0f) * _1215) * _1219) * _1222)) + (((float2(asfloat((_815 & _827) | _828) - 1.0f, asfloat(((_823 + (_823 << 15u)) & _834) | _835) - 1.0f) * _1216) * _1218) * _1222)) + (((float2(asfloat((_889 & _901) | _902) - 1.0f, asfloat(((_897 + (_897 << 15u)) & _908) | _909) - 1.0f) * _1215) * _1218) * _1222);
    float2 _1251 = (((_1235 + (((float2(asfloat((_963 & _975) | _976) - 1.0f, asfloat(((_971 + (_971 << 15u)) & _982) | _983) - 1.0f) * _1216) * _1219) * _1221)) + (((float2(asfloat((_1037 & _1049) | _1050) - 1.0f, asfloat(((_1045 + (_1045 << 15u)) & _1056) | _1057) - 1.0f) * _1215) * _1219) * _1221)) + (((float2(asfloat((_1111 & _1123) | _1124) - 1.0f, asfloat(((_1119 + (_1119 << 15u)) & _1130) | _1131) - 1.0f) * _1216) * _1218) * _1221)) + (((float2(asfloat((_1185 & _1197) | _1198) - 1.0f, asfloat(((_1193 + (_1193 << 15u)) & _1204) | _1205) - 1.0f) * _1215) * _1218) * _1221);
    float2 _1254 = clamp((_1251 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _1259 = _610 + (float3(_1254.x, 0.0f, _1254.y) * 0.0599999986588954925537109375f);
    float2 _1260 = _1259.xz;
    uint _1264 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52) ^ 787234);
    uint _1266 = _1264 + (_1264 << 10u);
    uint _1268 = _1266 ^ (_1266 >> 6u);
    uint _1270 = _1268 + (_1268 << 3u);
    uint _1272 = _1270 ^ (_1270 >> 11u);
    bool _1275 = _1203 ? _1203 : true;
    uint _1276 = _1203 ? _1204 : 8388607u;
    uint _1277 = _1203 ? _1205 : 1065353216u;
    float2 _1283 = _1260 + (asfloat(((_1272 + (_1272 << 15u)) & _1276) | _1277) - 1.0f).xx;
    int2 _1285 = int2(floor(_1283));
    float2 _1286 = frac(_1283);
    uint2 _1287 = uint2(_1285);
    uint _1289 = _1287.y;
    uint _1291 = _1289 + (_1289 << 10u);
    uint _1293 = _1291 ^ (_1291 >> 6u);
    uint _1295 = _1293 + (_1293 << 3u);
    uint _1297 = _1295 ^ (_1295 >> 11u);
    bool _1300 = _1169 ? _1169 : true;
    uint _1301 = _1169 ? _1170 : 1664525u;
    uint _1302 = _1169 ? _1171 : 1013904223u;
    uint _1306 = (((_1287.x * _1301) + (_1297 + (_1297 << 15u))) + _1302) * _1301;
    uint _1308 = _1306 ^ (_1306 >> 11u);
    uint _1311 = _1308 ^ ((_1308 << 7u) & 2636928640u);
    uint _1314 = _1311 ^ ((_1311 << 15u) & 4022730752u);
    bool _1317 = _1275 ? _1275 : true;
    uint _1318 = _1275 ? _1276 : 8388607u;
    uint _1319 = _1275 ? _1277 : 1065353216u;
    uint2 _1325 = uint2(_1285 + int2(1, 0));
    uint _1327 = _1325.y;
    uint _1329 = _1327 + (_1327 << 10u);
    uint _1331 = _1329 ^ (_1329 >> 6u);
    uint _1333 = _1331 + (_1331 << 3u);
    uint _1335 = _1333 ^ (_1333 >> 11u);
    bool _1338 = _1300 ? _1300 : true;
    uint _1339 = _1300 ? _1301 : 1664525u;
    uint _1340 = _1300 ? _1302 : 1013904223u;
    uint _1344 = (((_1325.x * _1339) + (_1335 + (_1335 << 15u))) + _1340) * _1339;
    uint _1346 = _1344 ^ (_1344 >> 11u);
    uint _1349 = _1346 ^ ((_1346 << 7u) & 2636928640u);
    uint _1352 = _1349 ^ ((_1349 << 15u) & 4022730752u);
    bool _1355 = _1317 ? _1317 : true;
    uint _1356 = _1317 ? _1318 : 8388607u;
    uint _1357 = _1317 ? _1319 : 1065353216u;
    uint2 _1363 = uint2(_1285 + int2(0, 1));
    uint _1365 = _1363.y;
    uint _1367 = _1365 + (_1365 << 10u);
    uint _1369 = _1367 ^ (_1367 >> 6u);
    uint _1371 = _1369 + (_1369 << 3u);
    uint _1373 = _1371 ^ (_1371 >> 11u);
    bool _1376 = _1338 ? _1338 : true;
    uint _1377 = _1338 ? _1339 : 1664525u;
    uint _1378 = _1338 ? _1340 : 1013904223u;
    uint _1382 = (((_1363.x * _1377) + (_1373 + (_1373 << 15u))) + _1378) * _1377;
    uint _1384 = _1382 ^ (_1382 >> 11u);
    uint _1387 = _1384 ^ ((_1384 << 7u) & 2636928640u);
    uint _1390 = _1387 ^ ((_1387 << 15u) & 4022730752u);
    bool _1393 = _1355 ? _1355 : true;
    uint _1394 = _1355 ? _1356 : 8388607u;
    uint _1395 = _1355 ? _1357 : 1065353216u;
    uint2 _1401 = uint2(_1285 + int2(1, 1));
    uint _1403 = _1401.y;
    uint _1405 = _1403 + (_1403 << 10u);
    uint _1407 = _1405 ^ (_1405 >> 6u);
    uint _1409 = _1407 + (_1407 << 3u);
    uint _1411 = _1409 ^ (_1409 >> 11u);
    uint _1414 = _1376 ? _1377 : 1664525u;
    uint _1419 = (((_1401.x * _1414) + (_1411 + (_1411 << 15u))) + (_1376 ? _1378 : 1013904223u)) * _1414;
    uint _1421 = _1419 ^ (_1419 >> 11u);
    uint _1424 = _1421 ^ ((_1421 << 7u) & 2636928640u);
    uint _1427 = _1424 ^ ((_1424 << 15u) & 4022730752u);
    bool _1430 = _1393 ? _1393 : true;
    uint _1431 = _1393 ? _1394 : 8388607u;
    uint _1432 = _1393 ? _1395 : 1065353216u;
    float2 _1440 = (_1286 * _1286) * (3.0f.xx - (_1286 * 2.0f));
    float _1441 = _1440.x;
    float _1442 = 1.0f - _1441;
    float _1444 = _1440.y;
    float4x4 _1460 = float4x4(frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 624), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 640), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 656), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 672), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 628), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 644), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 660), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 676), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 632), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 648), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 664), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 680), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 636), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 652), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 668), frame_constants_storage_buffer.Load<float>(_Globals_current_view_index * 800 + 684));
    float2 _1471 = ((_1260 * 0.2860000133514404296875f) * 0.5f) + ((_1259.yy * 0.2860000133514404296875f) * 0.5f);
    float2 _1475 = floor(_1471 + dot(_1471, 0.3660254180431365966796875f.xx).xx);
    float2 _1479 = (_1471 - _1475) + dot(_1475, 0.211324870586395263671875f.xx).xx;
    float _1480 = _1479.x;
    float _1481 = _1479.y;
    bool2 _1483 = (_1480 > _1481).xx;
    float2 _1484 = float2(_1483.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _1483.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 _1486 = _1479.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 _1488 = _1486.xy - _1484;
    float4 _1489 = float4(_1488.x, _1488.y, _1486.z, _1486.w);
    float2 _1490 = fmod(_1475, 289.0f.xx);
    float3 _1495 = _1490.y.xxx + float3(0.0f, _1484.y, 1.0f);
    float3 _1505 = (fmod(((_1495 * 34.0f) + 1.0f.xxx) * _1495, 289.0f.xxx) + _1490.x.xxx) + float3(0.0f, _1484.x, 1.0f);
    float2 _1511 = _1488.xy;
    float2 _1513 = _1486.zw;
    float3 _1517 = max(0.5f.xxx - float3(dot(_1479, _1479), dot(_1511, _1511), dot(_1513, _1513)), 0.0f.xxx);
    float3 _1518 = _1517 * _1517;
    float3 _1522 = frac(fmod(((_1505 * 34.0f) + 1.0f.xxx) * _1505, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 _1523 = _1522 - 1.0f.xxx;
    float3 _1525 = abs(_1523) - 0.5f.xxx;
    float3 _1528 = _1523 - floor(_1522 + (-0.5f).xxx);
    float3 _1540;
    _1540.x = (_1528.x * _1480) + (_1525.x * _1481);
    float2 _1547 = (_1528.yz * _1489.xz) + (_1525.yz * _1489.yw);
    _295.w = _294 + dot(_437 - _1259, _295.xyz);
    uint _1557 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u;
    uint _1559 = _1557 + (_1557 << 10u);
    uint _1561 = _1559 ^ (_1559 >> 6u);
    uint _1563 = _1561 + (_1561 << 3u);
    uint _1565 = _1563 ^ (_1563 >> 11u);
    bool _1568 = _1430 ? _1430 : true;
    uint _1569 = _1430 ? _1431 : 8388607u;
    uint _1570 = _1430 ? _1432 : 1065353216u;
    uint _1578 = _1557 + 34568u;
    uint _1580 = _1578 + (_1578 << 10u);
    uint _1582 = _1580 ^ (_1580 >> 6u);
    uint _1584 = _1582 + (_1582 << 3u);
    uint _1586 = _1584 ^ (_1584 >> 11u);
    float2 _1602 = float2(round((asfloat(((_1565 + (_1565 << 15u)) & _1569) | _1570) - 1.0f) * 5.0f) * 0.20000000298023223876953125f, round((asfloat(((_1586 + (_1586 << 15u)) & (_1568 ? _1569 : 8388607u)) | (_1568 ? _1570 : 1065353216u)) - 1.0f) * 5.0f) * 0.20000000298023223876953125f);
    float _1616;
    if (!_317)
    {
        _1616 = clamp(_438 * 0.0500000007450580596923828125f, 0.0f, 1.0f) * 0.699999988079071044921875f;
    }
    else
    {
        _1616 = _316;
    }
    gl_Position = mul(_1460, float4(_1259, 1.0f));
    out_var_C2 = _1259;
    out_var_C3 = pow(frac((((1.0f - _1444) * (((asfloat(((_1314 ^ (_1314 >> 18u)) & _1318) | _1319) - 1.0f) * _1442) + ((asfloat(((_1352 ^ (_1352 >> 18u)) & _1356) | _1357) - 1.0f) * _1441))) + (((asfloat(((_1390 ^ (_1390 >> 18u)) & _1394) | _1395) - 1.0f) * _1442) * _1444)) + (((asfloat(((_1427 ^ (_1427 >> 18u)) & _1431) | _1432) - 1.0f) * _1441) * _1444)), 1.5f);
    out_var_C4 = dot((_1518 * _1518) * (1.792842864990234375f.xxx - (((_1528 * _1528) + (_1525 * _1525)) * 0.8537347316741943359375f)), float3(_1540.x, _1547.x, _1547.y)) * 32.5f;
    out_var_C5 = _1616;
    out_var_C6 = (((in_var_Vertex_Position.xy * float2(0.85000002384185791015625f, 0.920000016689300537109375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + _1602;
    out_var_C7 = _295;
    out_var_C8 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C9 = _338.z;
    out_var_C10 = (((in_var_Vertex_Position.yz * float2(0.800000011920928955078125f, 0.89999997615814208984375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + _1602;
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C3 = out_var_C3;
    stage_output.out_var_C4 = out_var_C4;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C6 = out_var_C6;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C8 = out_var_C8;
    stage_output.out_var_C9 = out_var_C9;
    stage_output.out_var_C10 = out_var_C10;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_instanced_roof.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct RoofShingle
{
    float3 base_position;
    float current_height;
    uint2 clipping_plane_;
    uint2 scale_t;
    uint2 rotation_quat_;
    uint roof_id;
    uint velocity_rotation;
    float animation_t;
    int seed;
    int is_circle;
    int is_main_roof_body;
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

static bool _95;

ByteAddressBuffer instanced_data : register(t21, space0);
ByteAddressBuffer dormer_holes : register(t22, space0);
ByteAddressBuffer dormer_hole_count : register(t23, space0);

static float3 in_var_C2;
static float4 in_var_C7;
static uint in_var_C8;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] nointerpolation float4 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] nointerpolation uint in_var_C8 : TEXCOORD8;
};

void frag_main()
{
    if (dot(float4(in_var_C2, 1.0f), in_var_C7) < 0.0f)
    {
        discard;
    }
    if (instanced_data.Load<int>(in_var_C8 * 64 + 60) != 0)
    {
        int _121 = int(instanced_data.Load<uint>(in_var_C8 * 64 + 40));
        bool _219;
        do
        {
            float2 _126 = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
            int _134 = int(floor(_126.x)) + (15 * int(floor(_126.y)));
            uint _138 = min(128u, dormer_hole_count.Load<uint>(uint(_134) * 4 + 0));
            bool _216;
            bool _217;
            uint _140 = 0u;
            for (;;)
            {
                if (_140 < _138)
                {
                    uint _150 = uint(int(uint(_134 * 50) + _140));
                    if (_121 == dormer_holes.Load<int>(_150 * 48 + 12))
                    {
                        float3 _166 = float3(dormer_holes.Load<float>(_150 * 48 + 32), 0.0f, dormer_holes.Load<float>(_150 * 48 + 36));
                        float3x3 _170 = transpose(float3x3(cross(_166, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), _166));
                        float3 _171 = mul(_170, float3(1.0f, 0.0f, 0.0f));
                        float3 _172 = mul(_170, float3(0.0f, 1.0f, 0.0f));
                        float3 _173 = mul(_170, float3(0.0f, 0.0f, 1.0f));
                        float3 _176 = cross(_171, _172);
                        float3 _179 = (1.0f / dot(_173, _176)).xxx;
                        float3 _187 = mul(in_var_C2 - dormer_holes.Load<float3>(_150 * 48 + 0), float3x3(cross(_172, _173) * _179, cross(_173, _171) * _179, _176 * _179)) / dormer_holes.Load<float3>(_150 * 48 + 16);
                        float _188 = _187.y;
                        float _191 = clamp((_188 * dormer_holes.Load<float>(_150 * 48 + 40)) + dormer_holes.Load<float>(_150 * 48 + 44), 0.0f, 1.0f);
                        bool _207;
                        if (abs(_187.x) < lerp(0.5f, 0.0f, (_191 * _191) * _191))
                        {
                            bool _206;
                            if (_188 < ((dormer_holes.Load<float>(_150 * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                            {
                                _206 = _188 > (-0.5f);
                            }
                            else
                            {
                                _206 = false;
                            }
                            _207 = _206;
                        }
                        else
                        {
                            _207 = false;
                        }
                        bool _213;
                        if (_207)
                        {
                            _213 = abs(_187.z) < 0.5f;
                        }
                        else
                        {
                            _213 = false;
                        }
                        if (_213)
                        {
                            _216 = true;
                            _217 = true;
                            break;
                        }
                    }
                    _140++;
                    continue;
                }
                else
                {
                    _216 = _95;
                    _217 = false;
                    break;
                }
            }
            if (_217)
            {
                _219 = _216;
                break;
            }
            _219 = false;
            break;
        } while(false);
        if (_219)
        {
            discard;
        }
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
