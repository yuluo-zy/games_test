// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/grass.raster.hlsl
// Evidence: _grass.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin (SPIR-V, offsets 23/34106).
//   bindings: sampler_default s9, terrain_heightmap t15, grass_blade_buffer_data t17
//   (StructuredBuffer<GrassBladeDataPacked>), $Globals b13 (current_view_index/time/
//   terrain_heightmap_dims/visual_terrain_heightmap_dims/grass_buffer_size), frame_constants set1/b0.
//   vs_main (SV_VertexID/SV_InstanceID) builds a 6-vertex blade quad (bend/width/skew),
//   terrain-normal GatherRed, wind value-noise, rotation around the blade axis, and outputs
//   C0 albedo2, C2..C9 blade params (locations 0,2..9); ps_main packs the blade normal + albedo
//   + height/parallax into a uint2 target.
//   Status: O (structure verbatim). grass_lod1.raster.hlsl includes this file with the
//   GRASS_LOD1 guard so only shared helpers are imported there.

#ifndef TINYGLADE_GRASS_RASTER_HLSL
#define TINYGLADE_GRASS_RASTER_HLSL

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/color_library.hlsl"
#include "inc/perlin.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/grass_blade_data.hlsl"
#include "inc/atomics.hlsl"
#include "inc/lod.hlsl"

Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/material/grass_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

// $Globals members (implicit cbuffer at set0/b13): current_view_index comes from
// inc/current_view.hlsl; the remaining members are file-scope globals.
float time;
uint2 terrain_heightmap_dims;
uint2 visual_terrain_heightmap_dims;
uint grass_buffer_size;

StructuredBuffer<GrassBladeDataPacked> grass_blade_buffer_data : register(t17, space0);

// Decode the rgb8 color field into 0..1 rgb (matches grass_blade_color).
float3 blade_color_rgb(GrassBladeDataPacked b)
{
    return float3(float(b.color & 255u) * (1.0f / 255.0f),
                  float((b.color >> 8u) & 255u) * (1.0f / 255.0f),
                  float((b.color >> 16u) & 255u) * (1.0f / 255.0f));
}

// Pack a [0,1] normal rgb into one uint channel of the target (uint2 target).
uint pack_normal8(float3 n)
{
    return (uint(clamp(n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
}

#if !defined(GRASS_LOD1)

struct VS_OUT
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

VS_OUT vs_main(uint vertex_id : SV_VertexID, uint instance_id : SV_InstanceID)
{
    GrassBladeDataPacked blade = grass_blade_buffer_data[instance_id];
    float3 blade_pos = blade.position;
    float3 albedo = blade_color_rgb(blade);
    float3 albedo2 = albedo * albedo;
    float blend = float(blade.packed0 & 255u) * (1.0f / 255.0f);
    float2 widths = grass_blade_widths(blade);
    float width_bottom = widths.x;

    float v = (float(vertex_id & 1u) * 2.0f) - 1.0f;
    float t = clamp(float(vertex_id & 0xFFFFFFFEu) / (exp2((1.0f - blend) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float skew = lerp(clamp(2.5f * (1.0f - (t * t)), 0.0f, 1.0f) * 0.04f, t * 0.135f, blend);
    float blade_h = t * lerp(1.0f, 1.1f, blend);
    float2 st = float2(v, blade_h);
    float3 s = float3(v * skew, blade_h, 0.0f);

    // Terrain normal from a central-difference GatherRed.
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

    // Blade local axis from the packed direction byte + xz bend.
    float3 axis = normalize(float3((float((blade.packed0 >> 24u) & 255u) * (2.0f / 255.0f)) - 1.0f, 0.0f, grass_blade_bend_xz(blade)));
    float4x4 basis = transpose(float4x4(float4(normalize(cross(axis, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                         float4(0.0f, 1.0f, 0.0f, 0.0f),
                                         float4(axis, 0.0f),
                                         float4(0.0f, 0.0f, 0.0f, 1.0f)));

    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float h_clamped = clamp(blade_h, 0.0f, 1.0f);
    float curve = 3.0f * ((0.03f * widths.y) * lerp(1.0f, 2.0f, abs(normalize((cam_pos.xyz / cam_pos.w) - ground_pos).y)));
    float curve_factor = curve * (h_clamped * h_clamped);
    s.z = curve_factor;

    float3 blade_geom;
    if (glade_theme_id == 4u)
    {
        blade_geom = s;
        blade_geom.x = v * skew * 0.5f;
    }
    else
    {
        blade_geom = s;
    }

    // Twist around the blade axis by the packed bend angle (Rodrigues around X).
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
    blade_normal.x += 0.3f * v;

    float4 rotated = mul(basis, mul(twist, float4(normalize(blade_normal), 0.0f)));
    float3 rotated_xyz = rotated.xyz;
    float4 world_offset = mul(basis, mul(twist, mul(float4x4(float4(blade_scale.x * (float((blade.packed0 >> 8u) & 255u) * (1.0f / 255.0f)) * dist_scale, 0.0f, 0.0f, 0.0f),
                                                          float4(0.0f, wind_scale, 0.0f, 0.0f),
                                                          float4(0.0f, 0.0f, blade_scale.z, 0.0f),
                                                          float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(blade_geom, 1.0f)))) + float4(blade_pos.x, bend_offset, blade_pos.z, 0.0f);

    // Wind value-noise (inline blend grouping, matching the original opcode mix).
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
    float wind = (smoothstep(-0.2f, 0.5f, bend_amt) * (blade_h * blade_h))
        * (sin((wind_noise * 3.14f) - 1.57f + 0.3f)) * 0.7f * grass_blade_bend_xz(blade);
    rotated_xyz.y -= wind * 0.7f;
    world_offset.x -= wind * 0.25f;

    VS_OUT o;
    o.position = mul(view_constants[current_view_index].world_to_sample, world_offset);
    o.C0 = lerp(albedo2 * 0.8f, albedo2, (blade_h * blade_h).xxx);
    o.C2 = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    o.C3 = world_offset.xyz;
    o.C4 = ground_pos;
    o.C5 = normalize(rotated_xyz);
    o.C6 = axis;
    o.C7 = st;
    o.C8 = blend;
    o.C9 = float3(st * float2(skew, 1.0f), skew);
    return o;
}

struct PS_IN
{
    float3 C0 : TEXCOORD0;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float3 C5 : TEXCOORD5;
    float2 C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float3 C9 : TEXCOORD9;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    float4 cam_pos = mul(view_constants[current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(i.C9.x / i.C9.z) < lerp((-0.8f) / i.C8, 1.0f, i.C7.y))
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

#endif // !GRASS_LOD1

#endif // TINYGLADE_GRASS_RASTER_HLSL

#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _grass.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint _Globals_current_view_index : packoffset(c0);
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
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
    uint gl_InstanceIndex : SV_InstanceID;
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
    float3 _168 = grass_blade_buffer_data.Load<float3>(uint(gl_InstanceIndex) * 32 + 0);
    float3 _182 = float3(float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _183 = _182 * _182;
    float _188 = float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) & 255u) * 0.0039215688593685626983642578125f;
    float2 _204 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) & 65535u);
    float _205 = _204.x;
    float _226 = (float(uint(gl_VertexIndex) & 1u) * 2.0f) - 1.0f;
    float _230 = clamp(float(uint(gl_VertexIndex) & 4294967294u) / (exp2((1.0f - _188) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float _237 = lerp(clamp(2.5f * (1.0f - (_230 * _230)), 0.0f, 1.0f) * 0.039999999105930328369140625f, _230 * 0.13500000536441802978515625f, _188);
    float _239 = _230 * lerp(1.0f, 1.10000002384185791015625f, _188);
    float2 _240 = float2(_226, _239);
    float _241 = _226 * _237;
    float3 _242 = float3(_241, _239, 0.0f);
    float4 _249 = terrain_heightmap.GatherRed(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _254 = _249.z;
    float _255 = _249.y;
    float _257 = _249.w;
    float _258 = _249.x;
    float2 _267 = ((float2((_254 + _255) - (_257 + _258), (_258 + _255) - (_257 + _254)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _278 = _205 * 0.20000000298023223876953125f;
    float3 _280 = float3(min(1.0f, _205 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _278, 1.0f) * 2.5f;
    float _303 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_168, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _306 = _280.y;
    float _308 = _168.y - ((_303 - 1.0f) * _306);
    float3 _309 = _168;
    _309.y = _308;
    float _310 = _306 * _303;
    float3 _316 = float3((float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) >> 16u) & 65535u).x);
    float3 _317 = normalize(_316);
    float4x4 _329 = transpose(float4x4(float4(normalize(cross(_317, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_317, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _335 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _346 = clamp(_239, 0.0f, 1.0f);
    float _347 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_335.xyz / _335.w.xxx) - _309).y)));
    float _349 = _347 * (_346 * _346);
    float _350 = _346 + 0.00010001659393310546875f;
    _242.z = _349;
    float3 _362;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _361 = _242;
        _361.x = _241 * 0.5f;
        _362 = _361;
    }
    else
    {
        _362 = _242;
    }
    float _363 = float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _364 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _365 = sin(_363);
    float _366 = cos(_363);
    float _367 = 1.0f - _366;
    float _368 = _364.x;
    float _369 = _367 * _368;
    float _372 = _364.y;
    float _373 = _369 * _372;
    float _374 = _364.z;
    float _375 = _374 * _365;
    float _377 = _367 * _374;
    float _378 = _377 * _368;
    float _379 = _372 * _365;
    float _382 = _367 * _372;
    float _385 = _382 * _374;
    float _386 = _368 * _365;
    float4x4 _396 = transpose(float4x4(float4((_369 * _368) + _366, _373 - _375, _378 + _379, 0.0f), float4(_373 + _375, (_382 * _372) + _366, _385 - _386, 0.0f), float4(_378 - _379, _385 + _386, (_377 * _374) + _366, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _400 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _310, (_347 * (_350 * _350)) - _349)));
    _400.x = _400.x + (0.300000011920928955078125f * _226);
    float4 _411 = mul(_329, mul(_396, float4(normalize(_400), 0.0f)));
    float3 _412 = _411.xyz;
    float4 _421 = mul(_329, mul(_396, mul(float4x4(float4((_280.x * (float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _303, 0.0f, 0.0f, 0.0f), float4(0.0f, _310, 0.0f, 0.0f), float4(0.0f, 0.0f, _280.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_362, 1.0f)))) + float4(_168.x, _308, _168.z, 0.0f);
    float2 _428 = (_309.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _430 = int2(floor(_428));
    float2 _431 = frac(_428);
    uint2 _432 = uint2(_430);
    uint _434 = _432.y;
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint _448 = (((_432.x * 1664525u) + (_442 + (_442 << 15u))) + 1013904223u) * 1664525u;
    uint _450 = _448 ^ (_448 >> 11u);
    uint _453 = _450 ^ ((_450 << 7u) & 2636928640u);
    uint _456 = _453 ^ ((_453 << 15u) & 4022730752u);
    uint2 _464 = uint2(_430 + int2(1, 0));
    uint _466 = _464.y;
    uint _468 = _466 + (_466 << 10u);
    uint _470 = _468 ^ (_468 >> 6u);
    uint _472 = _470 + (_470 << 3u);
    uint _474 = _472 ^ (_472 >> 11u);
    uint _480 = (((_464.x * 1664525u) + (_474 + (_474 << 15u))) + 1013904223u) * 1664525u;
    uint _482 = _480 ^ (_480 >> 11u);
    uint _485 = _482 ^ ((_482 << 7u) & 2636928640u);
    uint _488 = _485 ^ ((_485 << 15u) & 4022730752u);
    uint2 _496 = uint2(_430 + int2(0, 1));
    uint _498 = _496.y;
    uint _500 = _498 + (_498 << 10u);
    uint _502 = _500 ^ (_500 >> 6u);
    uint _504 = _502 + (_502 << 3u);
    uint _506 = _504 ^ (_504 >> 11u);
    uint _512 = (((_496.x * 1664525u) + (_506 + (_506 << 15u))) + 1013904223u) * 1664525u;
    uint _514 = _512 ^ (_512 >> 11u);
    uint _517 = _514 ^ ((_514 << 7u) & 2636928640u);
    uint _520 = _517 ^ ((_517 << 15u) & 4022730752u);
    uint2 _528 = uint2(_430 + int2(1, 1));
    uint _530 = _528.y;
    uint _532 = _530 + (_530 << 10u);
    uint _534 = _532 ^ (_532 >> 6u);
    uint _536 = _534 + (_534 << 3u);
    uint _538 = _536 ^ (_536 >> 11u);
    uint _544 = (((_528.x * 1664525u) + (_538 + (_538 << 15u))) + 1013904223u) * 1664525u;
    uint _546 = _544 ^ (_544 >> 11u);
    uint _549 = _546 ^ ((_546 << 7u) & 2636928640u);
    uint _552 = _549 ^ ((_549 << 15u) & 4022730752u);
    float2 _562 = (_431 * _431) * (3.0f.xx - (_431 * 2.0f));
    float _563 = _562.x;
    float _564 = 1.0f - _563;
    float _566 = _562.y;
    float _578 = _239 * _239;
    float _586 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _278) * _578) * (sin((((((1.0f - _566) * (((asfloat(((_456 ^ (_456 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) + ((asfloat(((_488 ^ (_488 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563))) + (((asfloat(((_520 ^ (_520 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) * _566)) + (((asfloat(((_552 ^ (_552 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563) * _566)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) & 65535u).x;
    _412.y = _411.y - (_586 * 0.699999988079071044921875f);
    _421.x = _421.x - (_586 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _421);
    out_var_C0 = lerp(_183 * 0.800000011920928955078125f, _183, _578.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _267.y, 1.0f)), normalize(float3(1.0f, _267.x, 0.0f)));
    out_var_C3 = _421.xyz;
    out_var_C4 = _309;
    out_var_C5 = normalize(_412);
    out_var_C6 = _316;
    out_var_C7 = _240;
    out_var_C8 = _188;
    out_var_C9 = float3(_240 * float2(_237, 1.0f), _237);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
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
// lifted from _grass.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};


static float3 in_var_C0;
static float3 in_var_C2;
static float3 in_var_C3;
static float3 in_var_C5;
static float2 in_var_C7;
static float in_var_C8;
static float3 in_var_C9;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_C0 : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(3)]] float3 in_var_C3 : TEXCOORD3;
    [[vk::location(5)]] float3 in_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
};

void frag_main()
{
    float4 _107 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    if (abs(in_var_C9.x / in_var_C9.z) < lerp((-0.800000011920928955078125f) / in_var_C8, 1.0f, in_var_C7.y))
    {
        discard;
    }
    float3 _129;
    if (dot(normalize((_107.xyz / _107.w.xxx) - in_var_C3), in_var_C5) <= 0.0f)
    {
        _129 = -in_var_C5;
    }
    else
    {
        _129 = in_var_C5;
    }
    float3 _130 = normalize(_129);
    float3 _133 = (in_var_C0 * 0.550000011920928955078125f) * lerp(0.699999988079071044921875f, 1.10000002384185791015625f, in_var_C7.y);
    float3 _143;
    if (frame_constants_glade_theme_id == 4u)
    {
        _143 = lerp(0.89999997615814208984375f.xxx, _133 * 0.5f, pow(in_var_C7.y, 0.5f).xxx);
    }
    else
    {
        _143 = _133;
    }
    float3 _153 = _130 / ((abs(_130.x) + abs(_130.y)) + abs(_130.z)).xxx;
    float3 _167;
    if (_153.z < 0.0f)
    {
        float2 _165 = (1.0f.xx - abs(_153.yx)) * ((step(0.0f.xx, _153.xy) * 2.0f) - 1.0f.xx);
        _167 = float3(_165.x, _165.y, _153.z);
    }
    else
    {
        _167 = _153;
    }
    float2 _170 = (_167.xy * 0.5f) + 0.5f.xx;
    float3 _172 = (normalize(in_var_C2) * 0.5f) + 0.5f.xxx;
    float3 _200 = sqrt(clamp(_143, 0.0f.xxx, 1.0f.xxx));
    out_var_SV_TARGET0 = uint2(((uint((clamp(_172.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_172.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_172.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_170.x, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), ((uint((clamp(_200.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_200.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_200.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(_170.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u));
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C0 = stage_input.in_var_C0;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C3 = stage_input.in_var_C3;
    in_var_C5 = stage_input.in_var_C5;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _grass.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    uint _Globals_current_view_index : packoffset(c0);
    float _Globals_time : packoffset(c0.y);
    uint2 _Globals_terrain_heightmap_dims : packoffset(c0.z);
    uint2 _Globals_visual_terrain_heightmap_dims : packoffset(c1);
    uint _Globals_grass_buffer_size : packoffset(c1.z);
};

ByteAddressBuffer grass_blade_buffer_data : register(t17, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> terrain_heightmap : register(t15, space0);

static float4 gl_Position;
static int gl_VertexIndex;
static int gl_InstanceIndex;
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
    uint gl_InstanceIndex : SV_InstanceID;
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
    float3 _168 = grass_blade_buffer_data.Load<float3>(uint(gl_InstanceIndex) * 32 + 0);
    float3 _182 = float3(float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 8u) & 255u) * 0.0039215688593685626983642578125f, float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 12) >> 16u) & 255u) * 0.0039215688593685626983642578125f);
    float3 _183 = _182 * _182;
    float _188 = float(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) & 255u) * 0.0039215688593685626983642578125f;
    float2 _204 = spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) & 65535u);
    float _205 = _204.x;
    float _226 = (float(uint(gl_VertexIndex) & 1u) * 2.0f) - 1.0f;
    float _230 = clamp(float(uint(gl_VertexIndex) & 4294967294u) / (exp2((1.0f - _188) * 2.3219280242919921875f) * 2.0f), 0.0f, 1.0f);
    float _237 = lerp(clamp(2.5f * (1.0f - (_230 * _230)), 0.0f, 1.0f) * 0.039999999105930328369140625f, _230 * 0.13500000536441802978515625f, _188);
    float _239 = _230 * lerp(1.0f, 1.10000002384185791015625f, _188);
    float2 _240 = float2(_226, _239);
    float _241 = _226 * _237;
    float3 _242 = float3(_241, _239, 0.0f);
    float4 _249 = terrain_heightmap.GatherRed(sampler_default, (_168.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, int2(0, 0));
    float _254 = _249.z;
    float _255 = _249.y;
    float _257 = _249.w;
    float _258 = _249.x;
    float2 _267 = ((float2((_254 + _255) - (_257 + _258), (_258 + _255) - (_257 + _254)) * 0.5f.xx) * 26.5f) / (130.0f.xx / float2(_Globals_terrain_heightmap_dims));
    float _278 = _205 * 0.20000000298023223876953125f;
    float3 _280 = float3(min(1.0f, _205 * 0.800000011920928955078125f) * 0.3599999845027923583984375f, _278, 1.0f) * 2.5f;
    float _303 = max(30.0f / (((-1.0f) / mul(frame_constants_view_constants[0u].world_to_view, float4(_168, 1.0f)).z) * ((0.5f * frame_constants_view_constants[0u].view_to_clip[1].y) * float(clamp(frame_constants_inner_rendering_resolution.y, 1080u, 2160u)))), 1.0f);
    float _306 = _280.y;
    float _308 = _168.y - ((_303 - 1.0f) * _306);
    float3 _309 = _168;
    _309.y = _308;
    float _310 = _306 * _303;
    float3 _316 = float3((float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 24u) & 255u) * 0.007843137718737125396728515625f) - 1.0f, 0.0f, spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) >> 16u) & 65535u).x);
    float3 _317 = normalize(_316);
    float4x4 _329 = transpose(float4x4(float4(normalize(cross(_317, float3(0.0f, 1.0f, 0.0f))), 0.0f), float4(0.0f, 1.0f, 0.0f, 0.0f), float4(_317, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4 _335 = mul(frame_constants_view_constants[_Globals_current_view_index].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float _346 = clamp(_239, 0.0f, 1.0f);
    float _347 = 3.0f * ((0.02999999932944774627685546875f * spvUnpackHalf2x16((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 20) >> 16u) & 65535u).x) * lerp(1.0f, 2.0f, abs(normalize((_335.xyz / _335.w.xxx) - _309).y)));
    float _349 = _347 * (_346 * _346);
    float _350 = _346 + 0.00010001659393310546875f;
    _242.z = _349;
    float3 _362;
    if (frame_constants_glade_theme_id == 4u)
    {
        float3 _361 = _242;
        _361.x = _241 * 0.5f;
        _362 = _361;
    }
    else
    {
        _362 = _242;
    }
    float _363 = float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 16u) & 255u) * (-0.0061568631790578365325927734375f);
    float3 _364 = normalize(float3(1.0f, 0.0f, 0.0f));
    float _365 = sin(_363);
    float _366 = cos(_363);
    float _367 = 1.0f - _366;
    float _368 = _364.x;
    float _369 = _367 * _368;
    float _372 = _364.y;
    float _373 = _369 * _372;
    float _374 = _364.z;
    float _375 = _374 * _365;
    float _377 = _367 * _374;
    float _378 = _377 * _368;
    float _379 = _372 * _365;
    float _382 = _367 * _372;
    float _385 = _382 * _374;
    float _386 = _368 * _365;
    float4x4 _396 = transpose(float4x4(float4((_369 * _368) + _366, _373 - _375, _378 + _379, 0.0f), float4(_373 + _375, (_382 * _372) + _366, _385 - _386, 0.0f), float4(_378 - _379, _385 + _386, (_377 * _374) + _366, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float3 _400 = normalize(cross(float3(1.0f, 0.0f, 0.0f), float3(0.0f, 9.9999997473787516355514526367188e-05f * _310, (_347 * (_350 * _350)) - _349)));
    _400.x = _400.x + (0.300000011920928955078125f * _226);
    float4 _411 = mul(_329, mul(_396, float4(normalize(_400), 0.0f)));
    float3 _412 = _411.xyz;
    float4 _421 = mul(_329, mul(_396, mul(float4x4(float4((_280.x * (float((grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 16) >> 8u) & 255u) * 0.0039215688593685626983642578125f)) * _303, 0.0f, 0.0f, 0.0f), float4(0.0f, _310, 0.0f, 0.0f), float4(0.0f, 0.0f, _280.z, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(_362, 1.0f)))) + float4(_168.x, _308, _168.z, 0.0f);
    float2 _428 = (_309.xz * 0.100000001490116119384765625f) + (float2(_Globals_time, 0.0f) * 1.2000000476837158203125f);
    int2 _430 = int2(floor(_428));
    float2 _431 = frac(_428);
    uint2 _432 = uint2(_430);
    uint _434 = _432.y;
    uint _436 = _434 + (_434 << 10u);
    uint _438 = _436 ^ (_436 >> 6u);
    uint _440 = _438 + (_438 << 3u);
    uint _442 = _440 ^ (_440 >> 11u);
    uint _448 = (((_432.x * 1664525u) + (_442 + (_442 << 15u))) + 1013904223u) * 1664525u;
    uint _450 = _448 ^ (_448 >> 11u);
    uint _453 = _450 ^ ((_450 << 7u) & 2636928640u);
    uint _456 = _453 ^ ((_453 << 15u) & 4022730752u);
    uint2 _464 = uint2(_430 + int2(1, 0));
    uint _466 = _464.y;
    uint _468 = _466 + (_466 << 10u);
    uint _470 = _468 ^ (_468 >> 6u);
    uint _472 = _470 + (_470 << 3u);
    uint _474 = _472 ^ (_472 >> 11u);
    uint _480 = (((_464.x * 1664525u) + (_474 + (_474 << 15u))) + 1013904223u) * 1664525u;
    uint _482 = _480 ^ (_480 >> 11u);
    uint _485 = _482 ^ ((_482 << 7u) & 2636928640u);
    uint _488 = _485 ^ ((_485 << 15u) & 4022730752u);
    uint2 _496 = uint2(_430 + int2(0, 1));
    uint _498 = _496.y;
    uint _500 = _498 + (_498 << 10u);
    uint _502 = _500 ^ (_500 >> 6u);
    uint _504 = _502 + (_502 << 3u);
    uint _506 = _504 ^ (_504 >> 11u);
    uint _512 = (((_496.x * 1664525u) + (_506 + (_506 << 15u))) + 1013904223u) * 1664525u;
    uint _514 = _512 ^ (_512 >> 11u);
    uint _517 = _514 ^ ((_514 << 7u) & 2636928640u);
    uint _520 = _517 ^ ((_517 << 15u) & 4022730752u);
    uint2 _528 = uint2(_430 + int2(1, 1));
    uint _530 = _528.y;
    uint _532 = _530 + (_530 << 10u);
    uint _534 = _532 ^ (_532 >> 6u);
    uint _536 = _534 + (_534 << 3u);
    uint _538 = _536 ^ (_536 >> 11u);
    uint _544 = (((_528.x * 1664525u) + (_538 + (_538 << 15u))) + 1013904223u) * 1664525u;
    uint _546 = _544 ^ (_544 >> 11u);
    uint _549 = _546 ^ ((_546 << 7u) & 2636928640u);
    uint _552 = _549 ^ ((_549 << 15u) & 4022730752u);
    float2 _562 = (_431 * _431) * (3.0f.xx - (_431 * 2.0f));
    float _563 = _562.x;
    float _564 = 1.0f - _563;
    float _566 = _562.y;
    float _578 = _239 * _239;
    float _586 = (((smoothstep(-0.20000000298023223876953125f, 0.5f, _278) * _578) * (sin((((((1.0f - _566) * (((asfloat(((_456 ^ (_456 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) + ((asfloat(((_488 ^ (_488 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563))) + (((asfloat(((_520 ^ (_520 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _564) * _566)) + (((asfloat(((_552 ^ (_552 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _563) * _566)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f)) * 0.699999988079071044921875f) * spvUnpackHalf2x16(grass_blade_buffer_data.Load<uint>(uint(gl_InstanceIndex) * 32 + 24) & 65535u).x;
    _412.y = _411.y - (_586 * 0.699999988079071044921875f);
    _421.x = _421.x - (_586 * 0.25f);
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, _421);
    out_var_C0 = lerp(_183 * 0.800000011920928955078125f, _183, _578.xxx);
    out_var_C2 = cross(normalize(float3(0.0f, _267.y, 1.0f)), normalize(float3(1.0f, _267.x, 0.0f)));
    out_var_C3 = _421.xyz;
    out_var_C4 = _309;
    out_var_C5 = normalize(_412);
    out_var_C6 = _316;
    out_var_C7 = _240;
    out_var_C8 = _188;
    out_var_C9 = float3(_240 * float2(_237, 1.0f), _237);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_VertexIndex = int(stage_input.gl_VertexIndex);
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
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
// lifted from _grass.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float2 in_var_C7;
static float in_var_C8;
static float3 in_var_C9;

struct SPIRV_Cross_Input
{
    [[vk::location(7)]] float2 in_var_C7 : TEXCOORD7;
    [[vk::location(8)]] float in_var_C8 : TEXCOORD8;
    [[vk::location(9)]] float3 in_var_C9 : TEXCOORD9;
};

void frag_main()
{
    if (abs(in_var_C9.x / in_var_C9.z) < lerp((-0.800000011920928955078125f) / in_var_C8, 1.0f, in_var_C7.y))
    {
        discard;
    }
}

void ps_main(SPIRV_Cross_Input stage_input)
{
    in_var_C7 = stage_input.in_var_C7;
    in_var_C8 = stage_input.in_var_C8;
    in_var_C9 = stage_input.in_var_C9;
    frag_main();
}
#pragma dxc diagnostic pop
#endif
