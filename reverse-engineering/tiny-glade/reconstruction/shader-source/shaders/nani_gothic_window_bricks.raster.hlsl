// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_gothic_window_bricks.raster.hlsl | B nani raster | bins: _nani_gothic_window_bricks.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani gothic window brick wall: brick material + window frame colors, snow
// tint and semitransparent dither; packs the gbuffer.

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
#include "wall/inc/brick_frag.hlsl"
#include "wall/inc/brick_instance_flags.hlsl"
#include "wall/inc/brick_vs_out.hlsl"
#include "wall/inc/highlight_mode.hlsl"
#include "inc/hash.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/perlin.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_distortion.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/wall_color_customization.hlsl"
#include "inc/roof_colors_include.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"

int bool_render_shadows;

struct InstanceData
{
    Affine3Packed xform;
    uint wallspace_x_range_packed;
    uint seed;
    uint wall_id_is_wooden_packed;
    float mesh_size_x;
};

ByteAddressBuffer wall_color_ids : register(t18);
StructuredBuffer<uint> instance_indirection_buffer : register(t14);
StructuredBuffer<InstanceData> nani_instance_data : register(t15);
ByteAddressBuffer halftimber_color_ids : register(t19);
Texture2DArray<float4> halftimber_colors : register(t20);
Texture2DArray<float4> brick_colors : register(t22);
Texture2DArray<float4> roof_colors : register(t24);

uint hash3_rand(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 8388607u) | 1065353216u) - 1.0f;
}

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD2;
    float2 Vertex_UV : TEXCOORD3;
    uint instance_id : SV_InstanceID;
};

struct VsOut
{
    nointerpolation uint C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C4 : TEXCOORD4;
    float C7 : TEXCOORD7;
    float2 C10 : TEXCOORD10;
    nointerpolation int C11 : TEXCOORD11;
    nointerpolation float2 C12 : TEXCOORD12;
    nointerpolation int C13 : TEXCOORD13;
    nointerpolation uint C14 : TEXCOORD14;
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
    ViewConstants vc = view_constants[current_view_index];
    o.position = mul(vc.world_to_sample, float4(world, 1.0f));
    o.C0 = idx;
    o.C1 = mul(xform, float4(vin.Vertex_Normal, 0.0f));
    o.C2 = world;
    o.C4 = mul(xform, float4(vin.Vertex_Position, 1.0f)).xyz;
    o.C7 = world.y;
    o.C10 = vin.Vertex_UV;
    uint wall_seed = hash(nani_instance_data[idx].seed ^ 934565u);
    o.C11 = int(wall_color_ids.Load<uint>(wall_seed * 4 + 0));
    o.C12 = float2(float(nani_instance_data[idx].wallspace_x_range_packed), nani_instance_data[idx].mesh_size_x);
    o.C13 = int(nani_instance_data[idx].seed);
    o.C14 = idx;
    return o;
}

struct PsIn
{
    nointerpolation uint C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C4 : TEXCOORD4;
    float C7 : TEXCOORD7;
    float2 C10 : TEXCOORD10;
    nointerpolation int C11 : TEXCOORD11;
    nointerpolation float2 C12 : TEXCOORD12;
    nointerpolation int C13 : TEXCOORD13;
    nointerpolation uint C14 : TEXCOORD14;
    float4 position : SV_Position;
};

struct PsOut
{
    uint2 gbuffer : SV_Target0;
};

PsOut ps_main(PsIn pin)
{
    PsOut o;
    bool is_wooden = (pin.C13 & 8) != 0;
    bool has_roof = (pin.C13 & 2) != 0;
    bool semitransparent = (pin.C13 & 512) != 0;
    bool dither = semitransparent && bool_render_shadows != 0;
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

    uint color_id = uint(pin.C11);
    float3 albedo;
    if (has_roof)
    {
        albedo = roof_colors.Sample(sampler_llr, float3((pin.C0.xx * 0.2f) + (pin.C2.xz * 0.2f), float(color_id))).xyz * 0.7f;
    }
    else
    {
        float2 brick_uv = pin.C10;
        uint timber_color_id = uint(halftimber_color_ids.Load<uint>(uint(pin.C11) * 4 + 0));
        float3 timber = halftimber_colors.SampleBias(sampler_llr, float3(brick_uv, float(timber_color_id)), global_mip_bias).xyz;
        bool brick_like = color_id == 0u || color_id == 4u || color_id == 10u || color_id == 12u;
        albedo = brick_like
            ? brick_colors.Sample(sampler_llr, float3(brick_uv + 0.85f.xx, 0.0f)).xyz
            : timber;
    }

    float3 normal = normalize(pin.C1);
    if (glade_theme_id == 4u)
    {
        albedo = lerp(albedo, 0.7f.xxx, (smoothstep(0.0f, 0.2f, clamp(normal.y, 0.0f, 1.0f)) * smoothstep(-0.1f, 0.0f, pin.C2.y)).xxx);
    }
    float instance_seed = float(nani_instance_data[pin.C0].seed);
    float seed_jitter = hash3_rand(hash(uint(instance_seed)));
    albedo *= (0.9f + (seed_jitter * 0.2f)).xxx;
    albedo = lerp(albedo, color_library.wall_1.xyz, 0.02f.xxx);

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
// lifted from _nani_gothic_window_bricks.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed;
    int _wall_id;
    uint wallspace_x_range_packed;
    uint _is_wooden;
    float3 bone_0_pos;
    uint bone_0_normal_packed;
    float3 bone_1_pos;
    uint bone_1_normal_packed;
    float3 bone_2_pos;
    uint bone_2_normal_packed;
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
    uint _Globals_highlight_mode : packoffset(c1.z);
    int _Globals_highlight_bricks_to_delete : packoffset(c1.w);
    float3 _Globals_brush_position : packoffset(c2);
    uint _Globals_bool_render_shadows : packoffset(c2.w);
    float _Globals_eraser_mult : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer wall_color_ids : register(t18, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_bevel;
static int in_var_brick_id;
static float in_var_bbx_x;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_Vertex_UV : TEXCOORD3;
    [[vk::location(4)]] int in_var_is_bevel : TEXCOORD4;
    [[vk::location(5)]] int in_var_brick_id : TEXCOORD5;
    [[vk::location(6)]] float in_var_bbx_x : TEXCOORD6;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
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
    InstanceData _174;
    [unroll]
    for (int _2ident = 0; _2ident < 12; _2ident++)
    {
        _174.xform.data[_2ident] = nani_instance_data.Load<float>(_2ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    _174.seed = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    _174._wall_id = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 52);
    _174.wallspace_x_range_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 56);
    _174._is_wooden = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 60);
    _174.bone_0_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64);
    _174.bone_0_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76);
    _174.bone_1_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80);
    _174.bone_1_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92);
    _174.bone_2_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96);
    _174.bone_2_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108);
    float2 _214 = float2(spvUnpackHalf2x16(_174.bone_1_normal_packed & 65535u).x, spvUnpackHalf2x16((_174.bone_1_normal_packed >> 16u) & 65535u).x);
    float3 _271;
    float3 _272;
    if (in_var_bbx_x < 0.5f)
    {
        float _250 = in_var_bbx_x * 2.0f;
        float _259 = clamp(_250, 0.0f, 1.0f);
        float3 _263 = float3(lerp(_174.bone_0_pos.x, _174.bone_1_pos.x, _250), in_var_Vertex_Position.y + lerp(_174.bone_0_pos.y, _174.bone_1_pos.y, _259), lerp(_174.bone_0_pos.z, _174.bone_1_pos.z, _250));
        float2 _269 = _263.xz + (normalize(lerp(float2(spvUnpackHalf2x16(_174.bone_0_normal_packed & 65535u).x, spvUnpackHalf2x16((_174.bone_0_normal_packed >> 16u) & 65535u).x), _214, _259.xx)) * in_var_Vertex_Position.z);
        _271 = float3(_269.x, _263.y, _269.y);
        _272 = _263;
    }
    else
    {
        float _229 = (in_var_bbx_x - 0.5f) * 2.0f;
        float _238 = clamp(_229, 0.0f, 1.0f);
        float3 _242 = float3(lerp(_174.bone_1_pos.x, _174.bone_2_pos.x, _229), in_var_Vertex_Position.y + lerp(_174.bone_1_pos.y, _174.bone_2_pos.y, _238), lerp(_174.bone_1_pos.z, _174.bone_2_pos.z, _229));
        float2 _248 = _242.xz + (normalize(lerp(_214, float2(spvUnpackHalf2x16(_174.bone_2_normal_packed & 65535u).x, spvUnpackHalf2x16((_174.bone_2_normal_packed >> 16u) & 65535u).x), _238.xx)) * in_var_Vertex_Position.z);
        _271 = float3(_248.x, _242.y, _248.y);
        _272 = _242;
    }
    float3 _273 = _271 * 0.4000000059604644775390625f;
    int3 _275 = int3(floor(_273));
    float3 _276 = frac(_273);
    uint3 _277 = uint3(_275);
    uint _280 = _277.z;
    uint _282 = _280 + (_280 << 10u);
    uint _284 = _282 ^ (_282 >> 6u);
    uint _286 = _284 + (_284 << 3u);
    uint _288 = _286 ^ (_286 >> 11u);
    uint _294 = (((_277.y * 1664525u) + (_288 + (_288 << 15u))) + 1013904223u) * 1664525u;
    uint _296 = _294 ^ (_294 >> 11u);
    uint _299 = _296 ^ ((_296 << 7u) & 2636928640u);
    uint _302 = _299 ^ ((_299 << 15u) & 4022730752u);
    uint _308 = (((_277.x * 1664525u) + (_302 ^ (_302 >> 18u))) + 1013904223u) * 1664525u;
    uint _310 = _308 ^ (_308 >> 11u);
    uint _313 = _310 ^ ((_310 << 7u) & 2636928640u);
    uint _316 = _313 ^ ((_313 << 15u) & 4022730752u);
    uint _318 = _316 ^ (_316 >> 18u);
    uint _320 = _318 + (_318 << 10u);
    uint _322 = _320 ^ (_320 >> 6u);
    uint _324 = _322 + (_322 << 3u);
    uint _326 = _324 ^ (_324 >> 11u);
    uint3 _339 = uint3(_275 + int3(1, 0, 0));
    uint _342 = _339.z;
    uint _344 = _342 + (_342 << 10u);
    uint _346 = _344 ^ (_344 >> 6u);
    uint _348 = _346 + (_346 << 3u);
    uint _350 = _348 ^ (_348 >> 11u);
    uint _356 = (((_339.y * 1664525u) + (_350 + (_350 << 15u))) + 1013904223u) * 1664525u;
    uint _358 = _356 ^ (_356 >> 11u);
    uint _361 = _358 ^ ((_358 << 7u) & 2636928640u);
    uint _364 = _361 ^ ((_361 << 15u) & 4022730752u);
    uint _370 = (((_339.x * 1664525u) + (_364 ^ (_364 >> 18u))) + 1013904223u) * 1664525u;
    uint _372 = _370 ^ (_370 >> 11u);
    uint _375 = _372 ^ ((_372 << 7u) & 2636928640u);
    uint _378 = _375 ^ ((_375 << 15u) & 4022730752u);
    uint _380 = _378 ^ (_378 >> 18u);
    uint _382 = _380 + (_380 << 10u);
    uint _384 = _382 ^ (_382 >> 6u);
    uint _386 = _384 + (_384 << 3u);
    uint _388 = _386 ^ (_386 >> 11u);
    uint3 _401 = uint3(_275 + int3(0, 1, 0));
    uint _404 = _401.z;
    uint _406 = _404 + (_404 << 10u);
    uint _408 = _406 ^ (_406 >> 6u);
    uint _410 = _408 + (_408 << 3u);
    uint _412 = _410 ^ (_410 >> 11u);
    uint _418 = (((_401.y * 1664525u) + (_412 + (_412 << 15u))) + 1013904223u) * 1664525u;
    uint _420 = _418 ^ (_418 >> 11u);
    uint _423 = _420 ^ ((_420 << 7u) & 2636928640u);
    uint _426 = _423 ^ ((_423 << 15u) & 4022730752u);
    uint _432 = (((_401.x * 1664525u) + (_426 ^ (_426 >> 18u))) + 1013904223u) * 1664525u;
    uint _434 = _432 ^ (_432 >> 11u);
    uint _437 = _434 ^ ((_434 << 7u) & 2636928640u);
    uint _440 = _437 ^ ((_437 << 15u) & 4022730752u);
    uint _442 = _440 ^ (_440 >> 18u);
    uint _444 = _442 + (_442 << 10u);
    uint _446 = _444 ^ (_444 >> 6u);
    uint _448 = _446 + (_446 << 3u);
    uint _450 = _448 ^ (_448 >> 11u);
    uint3 _463 = uint3(_275 + int3(1, 1, 0));
    uint _466 = _463.z;
    uint _468 = _466 + (_466 << 10u);
    uint _470 = _468 ^ (_468 >> 6u);
    uint _472 = _470 + (_470 << 3u);
    uint _474 = _472 ^ (_472 >> 11u);
    uint _480 = (((_463.y * 1664525u) + (_474 + (_474 << 15u))) + 1013904223u) * 1664525u;
    uint _482 = _480 ^ (_480 >> 11u);
    uint _485 = _482 ^ ((_482 << 7u) & 2636928640u);
    uint _488 = _485 ^ ((_485 << 15u) & 4022730752u);
    uint _494 = (((_463.x * 1664525u) + (_488 ^ (_488 >> 18u))) + 1013904223u) * 1664525u;
    uint _496 = _494 ^ (_494 >> 11u);
    uint _499 = _496 ^ ((_496 << 7u) & 2636928640u);
    uint _502 = _499 ^ ((_499 << 15u) & 4022730752u);
    uint _504 = _502 ^ (_502 >> 18u);
    uint _506 = _504 + (_504 << 10u);
    uint _508 = _506 ^ (_506 >> 6u);
    uint _510 = _508 + (_508 << 3u);
    uint _512 = _510 ^ (_510 >> 11u);
    uint3 _525 = uint3(_275 + int3(0, 0, 1));
    uint _528 = _525.z;
    uint _530 = _528 + (_528 << 10u);
    uint _532 = _530 ^ (_530 >> 6u);
    uint _534 = _532 + (_532 << 3u);
    uint _536 = _534 ^ (_534 >> 11u);
    uint _542 = (((_525.y * 1664525u) + (_536 + (_536 << 15u))) + 1013904223u) * 1664525u;
    uint _544 = _542 ^ (_542 >> 11u);
    uint _547 = _544 ^ ((_544 << 7u) & 2636928640u);
    uint _550 = _547 ^ ((_547 << 15u) & 4022730752u);
    uint _556 = (((_525.x * 1664525u) + (_550 ^ (_550 >> 18u))) + 1013904223u) * 1664525u;
    uint _558 = _556 ^ (_556 >> 11u);
    uint _561 = _558 ^ ((_558 << 7u) & 2636928640u);
    uint _564 = _561 ^ ((_561 << 15u) & 4022730752u);
    uint _566 = _564 ^ (_564 >> 18u);
    uint _568 = _566 + (_566 << 10u);
    uint _570 = _568 ^ (_568 >> 6u);
    uint _572 = _570 + (_570 << 3u);
    uint _574 = _572 ^ (_572 >> 11u);
    uint3 _587 = uint3(_275 + int3(1, 0, 1));
    uint _590 = _587.z;
    uint _592 = _590 + (_590 << 10u);
    uint _594 = _592 ^ (_592 >> 6u);
    uint _596 = _594 + (_594 << 3u);
    uint _598 = _596 ^ (_596 >> 11u);
    uint _604 = (((_587.y * 1664525u) + (_598 + (_598 << 15u))) + 1013904223u) * 1664525u;
    uint _606 = _604 ^ (_604 >> 11u);
    uint _609 = _606 ^ ((_606 << 7u) & 2636928640u);
    uint _612 = _609 ^ ((_609 << 15u) & 4022730752u);
    uint _618 = (((_587.x * 1664525u) + (_612 ^ (_612 >> 18u))) + 1013904223u) * 1664525u;
    uint _620 = _618 ^ (_618 >> 11u);
    uint _623 = _620 ^ ((_620 << 7u) & 2636928640u);
    uint _626 = _623 ^ ((_623 << 15u) & 4022730752u);
    uint _628 = _626 ^ (_626 >> 18u);
    uint _630 = _628 + (_628 << 10u);
    uint _632 = _630 ^ (_630 >> 6u);
    uint _634 = _632 + (_632 << 3u);
    uint _636 = _634 ^ (_634 >> 11u);
    uint3 _649 = uint3(_275 + int3(0, 1, 1));
    uint _652 = _649.z;
    uint _654 = _652 + (_652 << 10u);
    uint _656 = _654 ^ (_654 >> 6u);
    uint _658 = _656 + (_656 << 3u);
    uint _660 = _658 ^ (_658 >> 11u);
    uint _666 = (((_649.y * 1664525u) + (_660 + (_660 << 15u))) + 1013904223u) * 1664525u;
    uint _668 = _666 ^ (_666 >> 11u);
    uint _671 = _668 ^ ((_668 << 7u) & 2636928640u);
    uint _674 = _671 ^ ((_671 << 15u) & 4022730752u);
    uint _680 = (((_649.x * 1664525u) + (_674 ^ (_674 >> 18u))) + 1013904223u) * 1664525u;
    uint _682 = _680 ^ (_680 >> 11u);
    uint _685 = _682 ^ ((_682 << 7u) & 2636928640u);
    uint _688 = _685 ^ ((_685 << 15u) & 4022730752u);
    uint _690 = _688 ^ (_688 >> 18u);
    uint _692 = _690 + (_690 << 10u);
    uint _694 = _692 ^ (_692 >> 6u);
    uint _696 = _694 + (_694 << 3u);
    uint _698 = _696 ^ (_696 >> 11u);
    uint3 _711 = uint3(_275 + int3(1, 1, 1));
    uint _714 = _711.z;
    uint _716 = _714 + (_714 << 10u);
    uint _718 = _716 ^ (_716 >> 6u);
    uint _720 = _718 + (_718 << 3u);
    uint _722 = _720 ^ (_720 >> 11u);
    uint _728 = (((_711.y * 1664525u) + (_722 + (_722 << 15u))) + 1013904223u) * 1664525u;
    uint _730 = _728 ^ (_728 >> 11u);
    uint _733 = _730 ^ ((_730 << 7u) & 2636928640u);
    uint _736 = _733 ^ ((_733 << 15u) & 4022730752u);
    uint _742 = (((_711.x * 1664525u) + (_736 ^ (_736 >> 18u))) + 1013904223u) * 1664525u;
    uint _744 = _742 ^ (_742 >> 11u);
    uint _747 = _744 ^ ((_744 << 7u) & 2636928640u);
    uint _750 = _747 ^ ((_747 << 15u) & 4022730752u);
    uint _752 = _750 ^ (_750 >> 18u);
    uint _754 = _752 + (_752 << 10u);
    uint _756 = _754 ^ (_754 >> 6u);
    uint _758 = _756 + (_756 << 3u);
    uint _760 = _758 ^ (_758 >> 11u);
    float3 _775 = (_276 * _276) * (3.0f.xxx - (_276 * 2.0f));
    float _776 = _775.x;
    float _777 = 1.0f - _776;
    float _779 = _775.y;
    float _780 = 1.0f - _779;
    float _782 = _775.z;
    float _783 = 1.0f - _782;
    float2 _800 = ((((((float2(asfloat((_318 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_326 + (_326 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _777) * _780) * _783) + (((float2(asfloat((_380 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_388 + (_388 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _776) * _780) * _783)) + (((float2(asfloat((_442 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_450 + (_450 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _777) * _779) * _783)) + (((float2(asfloat((_504 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_512 + (_512 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _776) * _779) * _783)) + (((float2(asfloat((_566 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_574 + (_574 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _777) * _780) * _782);
    float2 _815 = clamp(((((_800 + (((float2(asfloat((_628 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_636 + (_636 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _776) * _780) * _782)) + (((float2(asfloat((_690 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_698 + (_698 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _777) * _779) * _782)) + (((float2(asfloat((_752 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_760 + (_760 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _776) * _779) * _782)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _820 = _271 + (float3(_815.x, 0.0f, _815.y) * 0.0599999986588954925537109375f);
    int _822 = (in_var_is_bevel != 0) ? 768 : 512;
    bool _823 = _174._is_wooden != 0u;
    int _827;
    if (_823)
    {
        _827 = _822 | 1024;
    }
    else
    {
        _827 = _822;
    }
    uint _838 = uint(_174.seed + in_var_brick_id);
    uint _860 = uint(_174._wall_id);
    uint _864 = _860 & 65535u;
    uint _867 = _864 ^ 7993234u;
    uint _869 = _867 + (_867 << 10u);
    uint _871 = _869 ^ (_869 >> 6u);
    uint _873 = _871 + (_871 << 3u);
    uint _875 = _873 ^ (_873 >> 11u);
    uint _882 = _864 ^ 3450234u;
    uint _884 = _882 + (_882 << 10u);
    uint _886 = _884 ^ (_884 >> 6u);
    uint _888 = _886 + (_886 << 3u);
    uint _890 = _888 ^ (_888 >> 11u);
    float2 _904 = (((float2(lerp(spvUnpackHalf2x16(_174.wallspace_x_range_packed & 65535u).x, spvUnpackHalf2x16((_174.wallspace_x_range_packed >> 16u) & 65535u).x, in_var_bbx_x), _820.y) * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(_864) * 0.20000000298023223876953125f).xx) + float2(asfloat(((_875 + (_875 << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((_890 + (_890 << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint _905 = _838 ^ 292478890u;
    uint _907 = _905 + (_905 << 10u);
    uint _909 = _907 ^ (_907 >> 6u);
    uint _911 = _909 + (_909 << 3u);
    uint _913 = _911 ^ (_911 >> 11u);
    float _919 = asfloat(((_913 + (_913 << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 _928;
    if (_919 > 0.89999997615814208984375f)
    {
        _928 = _904 * 3.0f;
    }
    else
    {
        _928 = _904 + (0.61000001430511474609375f * _919).xx;
    }
    float2 _934;
    if (int((_827 & 512) != 0) == 1)
    {
        _934 = _928 * 2.2000000476837158203125f;
    }
    else
    {
        _934 = _928;
    }
    bool2 _935 = _823.xx;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_820, 1.0f));
    out_var_C0 = _838;
    out_var_C1 = mul(transpose(float4x3(float3(_174.xform.data[0], _174.xform.data[1], _174.xform.data[2]), float3(_174.xform.data[3], _174.xform.data[4], _174.xform.data[5]), float3(_174.xform.data[6], _174.xform.data[7], _174.xform.data[8]), float3(_174.xform.data[9], _174.xform.data[10], _174.xform.data[11]))), float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _820;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(_935.x ? in_var_Vertex_UV.x : _934.x, _935.y ? in_var_Vertex_UV.y : _934.y);
    out_var_C11 = int(wall_color_ids.Load<uint>(_860 * 4 + 0));
    out_var_C12 = _272.xz;
    out_var_C13 = _827;
    out_var_C14 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_bevel = stage_input.in_var_is_bevel;
    in_var_brick_id = stage_input.in_var_brick_id;
    in_var_bbx_x = stage_input.in_var_bbx_x;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_gothic_window_bricks.raster.hlsl_b903e43ffb3da915.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed;
    int _wall_id;
    uint wallspace_x_range_packed;
    uint _is_wooden;
    float3 bone_0_pos;
    uint bone_0_normal_packed;
    float3 bone_1_pos;
    uint bone_1_normal_packed;
    float3 bone_2_pos;
    uint bone_2_normal_packed;
};

static uint _197;
static float3 _209;

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
    uint _Globals_highlight_mode : packoffset(c1.z);
    int _Globals_highlight_bricks_to_delete : packoffset(c1.w);
    float3 _Globals_brush_position : packoffset(c2);
    uint _Globals_bool_render_shadows : packoffset(c2.w);
    float _Globals_eraser_mult : packoffset(c3);
};

ByteAddressBuffer nani_instance_data : register(t15, space0);
ByteAddressBuffer halftimber_color_ids : register(t19, space0);
cbuffer color_library : register(b21, space0)
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

SamplerState sampler_llr : register(s1, space0);
Texture2DArray<float4> halftimber_colors : register(t20, space0);
Texture2DArray<float4> brick_colors : register(t22, space0);
Texture2DArray<float4> roof_colors : register(t24, space0);

static float4 gl_FragCoord;
static float gl_FragDepth;
static uint in_var_C0;
static float3 in_var_C1;
static float3 in_var_C2;
static float in_var_C7;
static float2 in_var_C10;
static int in_var_C11;
static float2 in_var_C12;
static int in_var_C13;
static uint in_var_C14;
static uint2 out_var_SV_TARGET0;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] nointerpolation uint in_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 in_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 in_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int in_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 in_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint in_var_C14 : TEXCOORD14;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    uint2 out_var_SV_TARGET0 : SV_Target0;
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _227 = (in_var_C13 & 512) != 0;
    bool _229 = (in_var_C13 & 8) != 0;
    bool _233 = (in_var_C13 & 16) != 0;
    bool _237 = (in_var_C13 & 2048) != 0;
    bool _239 = (in_var_C13 & 256) != 0;
    bool _244;
    if (!_229)
    {
        _244 = in_var_C7 > (-2.5f);
    }
    else
    {
        _244 = false;
    }
    bool _249;
    if (_244)
    {
        _249 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _249 = false;
    }
    if (_249)
    {
        discard;
    }
    else
    {
        bool _257;
        if (_229)
        {
            _257 = in_var_C2.y > in_var_C7;
        }
        else
        {
            _257 = false;
        }
        if (_257)
        {
            discard;
        }
    }
    uint _263 = uint(in_var_C11);
    float3 _480;
    if ((in_var_C13 & 2) != 0)
    {
        _480 = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(_263))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 _275;
        if (_239)
        {
            _275 = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            _275 = in_var_C10;
        }
        bool _281;
        if (!(_263 == 1u))
        {
            _281 = _263 == 0u;
        }
        else
        {
            _281 = true;
        }
        float3 _288;
        if (_281)
        {
            _288 = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            _288 = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        int _289 = int(_263);
        bool _295;
        if (!(_289 == 0))
        {
            _295 = _289 == 4;
        }
        else
        {
            _295 = true;
        }
        bool _300;
        if (!_295)
        {
            _300 = _289 == 10;
        }
        else
        {
            _300 = true;
        }
        bool _305;
        if (!_300)
        {
            _305 = _289 == 12;
        }
        else
        {
            _305 = true;
        }
        float3 _396;
        if (_305)
        {
            _396 = brick_colors.Sample(sampler_llr, float3(_275 + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool _314;
            if (!(_289 == 1))
            {
                _314 = _289 == 5;
            }
            else
            {
                _314 = true;
            }
            float3 _386;
            if (_314)
            {
                _386 = pow(brick_colors.Sample(sampler_llr, float3(_275 + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool _323;
                if (!(_289 == 2))
                {
                    _323 = _289 == 7;
                }
                else
                {
                    _323 = true;
                }
                float3 _375;
                if (_323)
                {
                    _375 = brick_colors.Sample(sampler_llr, float3(_275, 2.0f)).xyz;
                }
                else
                {
                    bool _332;
                    if (!(_289 == 3))
                    {
                        _332 = _289 == 6;
                    }
                    else
                    {
                        _332 = true;
                    }
                    bool _337;
                    if (!_332)
                    {
                        _337 = _289 == 8;
                    }
                    else
                    {
                        _337 = true;
                    }
                    bool _342;
                    if (!_337)
                    {
                        _342 = _289 == 9;
                    }
                    else
                    {
                        _342 = true;
                    }
                    float3 _366;
                    if (_342)
                    {
                        _366 = brick_colors.Sample(sampler_llr, float3(_275, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 _357;
                        if (_289 == 11)
                        {
                            _357 = brick_colors.Sample(sampler_llr, float3(_275, 4.0f)).xyz;
                        }
                        else
                        {
                            _357 = 0.0f.xxx;
                        }
                        _366 = _357;
                    }
                    _375 = _366;
                }
                _386 = _375;
            }
            _396 = _386;
        }
        bool _398 = !(_263 == 7u);
        bool _402;
        if (_398)
        {
            _402 = _263 == 2u;
        }
        else
        {
            _402 = true;
        }
        bool _407;
        if (!_402)
        {
            _407 = _263 == 11u;
        }
        else
        {
            _407 = true;
        }
        bool3 _408 = _407.xxx;
        float3 _431;
        if (_227)
        {
            uint _412 = in_var_C0 ^ 8902390u;
            uint _414 = _412 + (_412 << 10u);
            uint _416 = _414 ^ (_414 >> 6u);
            uint _418 = _416 + (_416 << 3u);
            uint _420 = _418 ^ (_418 >> 11u);
            _431 = lerp(_396, float3(_408.x ? _396.x : _288.x, _408.y ? _396.y : _288.y, _408.z ? _396.z : _288.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((_420 + (_420 << 15u)) & 8388607u) | 1065353216u) - 1.0f)).xxx);
        }
        else
        {
            _431 = _396;
        }
        bool _434 = _227 ? true : false;
        float3 _463;
        if (_239)
        {
            bool _440;
            if (_398)
            {
                _440 = _263 == 2u;
            }
            else
            {
                _440 = true;
            }
            uint _442 = in_var_C0 ^ 239085u;
            uint _444 = _442 + (_442 << 10u);
            uint _446 = _444 ^ (_444 >> 6u);
            uint _448 = _446 + (_446 << 3u);
            uint _450 = _448 ^ (_448 >> 11u);
            _463 = lerp(_431, color_library_wall_1.xyz, lerp(0.0f, _440 ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((_450 + (_450 << 15u)) & (_434 ? (_227 ? 8388607u : _197) : 8388607u)) | (_434 ? (_227 ? 1065353216u : _197) : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            _463 = _431;
        }
        _480 = _463;
    }
    float3 _481 = normalize(in_var_C1);
    bool _520;
    float3 _521;
    if (!((in_var_C13 & 64) != 0))
    {
        bool _498;
        float3 _499;
        if (_Globals_highlight_mode == 1u)
        {
            _498 = true;
            _499 = lerp(_480, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool _491 = _Globals_highlight_mode == 2u;
            float3 _495;
            if (_491)
            {
                _495 = lerp(_480, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                _495 = _480;
            }
            _498 = _491 ? true : false;
            _499 = _495;
        }
        bool _518;
        float3 _519;
        if (_Globals_highlight_bricks_to_delete == 1)
        {
            bool _512 = distance(_Globals_brush_position.xz, in_var_C12) < (1.88900005817413330078125f * _Globals_eraser_mult);
            float3 _516;
            if (_512)
            {
                _516 = lerp(_480, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                _516 = _499;
            }
            _518 = _512 ? true : _498;
            _519 = _516;
        }
        else
        {
            _518 = _498;
            _519 = _499;
        }
        _520 = _518;
        _521 = _519;
    }
    else
    {
        _520 = false;
        _521 = _480;
    }
    float3 _525;
    if (_237)
    {
        _525 = lerp(_480, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        _525 = _521;
    }
    float3 _547;
    if (int((in_var_C13 & 1024) != 0) == 1)
    {
        _547 = halftimber_colors.Sample(sampler_llr, float3(in_var_C10 * 1.0f, float(int(halftimber_color_ids.Load<uint>(uint(nani_instance_data.Load<int>(in_var_C14 * 112 + 52)) * 4 + 0))))).xyz;
    }
    else
    {
        _547 = _525;
    }
    float3 _648;
    if (frame_constants_glade_theme_id == 4u)
    {
        float2 _558 = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 _562 = floor(_558 + dot(_558, 0.3660254180431365966796875f.xx).xx);
        float2 _566 = (_558 - _562) + dot(_562, 0.211324870586395263671875f.xx).xx;
        float _567 = _566.x;
        float _568 = _566.y;
        bool2 _570 = (_567 > _568).xx;
        float2 _571 = float2(_570.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, _570.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 _573 = _566.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 _575 = _573.xy - _571;
        float4 _576 = float4(_575.x, _575.y, _573.z, _573.w);
        float2 _577 = fmod(_562, 289.0f.xx);
        float3 _582 = _577.y.xxx + float3(0.0f, _571.y, 1.0f);
        float3 _592 = (fmod(((_582 * 34.0f) + 1.0f.xxx) * _582, 289.0f.xxx) + _577.x.xxx) + float3(0.0f, _571.x, 1.0f);
        float2 _598 = _575.xy;
        float2 _600 = _573.zw;
        float3 _604 = max(0.5f.xxx - float3(dot(_566, _566), dot(_598, _598), dot(_600, _600)), 0.0f.xxx);
        float3 _605 = _604 * _604;
        float3 _609 = frac(fmod(((_592 * 34.0f) + 1.0f.xxx) * _592, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 _610 = _609 - 1.0f.xxx;
        float3 _612 = abs(_610) - 0.5f.xxx;
        float3 _615 = _610 - floor(_609 + (-0.5f).xxx);
        float3 _627;
        _627.x = (_615.x * _567) + (_612.x * _568);
        float2 _634 = (_615.yz * _576.xz) + (_612.yz * _576.yw);
        _648 = lerp(_547, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(_481.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((_605 * _605) * (1.792842864990234375f.xxx - (((_615 * _615) + (_612 * _612)) * 0.8537347316741943359375f)), float3(_627.x, _634.x, _634.y))) + 0.75f, 0.0f, 1.0f), _233 ? 0.0f : 0.4000000059604644775390625f, _233 ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        _648 = _547;
    }
    float3 _650 = (_481 * 0.5f) + 0.5f.xxx;
    float3 _672 = sqrt(clamp(_648, 0.0f.xxx, 1.0f.xxx));
    uint _691 = (uint((clamp(_672.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_672.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_672.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 _692 = uint2(((uint((clamp(_650.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(_650.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(_650.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, _691);
    _692.y = _691 | (((((7u | (uint(_237 ? true : _520) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = _692;
    gl_FragDepth = gl_FragCoord.z;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C0 = stage_input.in_var_C0;
    in_var_C1 = stage_input.in_var_C1;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C10 = stage_input.in_var_C10;
    in_var_C11 = stage_input.in_var_C11;
    in_var_C12 = stage_input.in_var_C12;
    in_var_C13 = stage_input.in_var_C13;
    in_var_C14 = stage_input.in_var_C14;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    stage_output.out_var_SV_TARGET0 = out_var_SV_TARGET0;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)
// lifted from _nani_gothic_window_bricks.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
    int seed;
    int _wall_id;
    uint wallspace_x_range_packed;
    uint _is_wooden;
    float3 bone_0_pos;
    uint bone_0_normal_packed;
    float3 bone_1_pos;
    uint bone_1_normal_packed;
    float3 bone_2_pos;
    uint bone_2_normal_packed;
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
    uint _Globals_highlight_mode : packoffset(c1.z);
    int _Globals_highlight_bricks_to_delete : packoffset(c1.w);
    float3 _Globals_brush_position : packoffset(c2);
    uint _Globals_bool_render_shadows : packoffset(c2.w);
    float _Globals_eraser_mult : packoffset(c3);
};

ByteAddressBuffer instance_indirection_buffer : register(t14, space0);
ByteAddressBuffer nani_instance_data : register(t15, space0);

static float4 gl_Position;
static int gl_InstanceIndex;
static float3 in_var_Vertex_Position;
static float3 in_var_Vertex_Normal;
static float2 in_var_Vertex_UV;
static int in_var_is_bevel;
static int in_var_brick_id;
static float in_var_bbx_x;
static uint out_var_C0;
static float3 out_var_C1;
static float3 out_var_C2;
static float3 out_var_C5;
static float out_var_C7;
static float2 out_var_C10;
static int out_var_C11;
static float2 out_var_C12;
static int out_var_C13;
static uint out_var_C14;

struct SPIRV_Cross_Input
{
    [[vk::location(0)]] float3 in_var_Vertex_Position : TEXCOORD0;
    [[vk::location(2)]] float3 in_var_Vertex_Normal : TEXCOORD2;
    [[vk::location(3)]] float2 in_var_Vertex_UV : TEXCOORD3;
    [[vk::location(4)]] int in_var_is_bevel : TEXCOORD4;
    [[vk::location(5)]] int in_var_brick_id : TEXCOORD5;
    [[vk::location(6)]] float in_var_bbx_x : TEXCOORD6;
    uint gl_InstanceIndex : SV_InstanceID;
};

struct SPIRV_Cross_Output
{
    [[vk::location(0)]] nointerpolation uint out_var_C0 : TEXCOORD0;
    [[vk::location(1)]] float3 out_var_C1 : TEXCOORD1;
    [[vk::location(2)]] float3 out_var_C2 : TEXCOORD2;
    [[vk::location(5)]] nointerpolation float3 out_var_C5 : TEXCOORD5;
    [[vk::location(7)]] float out_var_C7 : TEXCOORD7;
    [[vk::location(10)]] float2 out_var_C10 : TEXCOORD10;
    [[vk::location(11)]] nointerpolation int out_var_C11 : TEXCOORD11;
    [[vk::location(12)]] nointerpolation float2 out_var_C12 : TEXCOORD12;
    [[vk::location(13)]] nointerpolation int out_var_C13 : TEXCOORD13;
    [[vk::location(14)]] nointerpolation uint out_var_C14 : TEXCOORD14;
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
    Affine3Packed _172;
    [unroll]
    for (int _1ident = 0; _1ident < 12; _1ident++)
    {
        _172.data[_1ident] = nani_instance_data.Load<float>(_1ident * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    float2 _217 = float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92) >> 16u) & 65535u).x);
    float3 _274;
    float3 _275;
    if (in_var_bbx_x < 0.5f)
    {
        float _253 = in_var_bbx_x * 2.0f;
        float _262 = clamp(_253, 0.0f, 1.0f);
        float3 _266 = float3(lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).x, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).x, _253), in_var_Vertex_Position.y + lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).y, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).y, _262), lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).z, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).z, _253));
        float2 _272 = _266.xz + (normalize(lerp(float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76) >> 16u) & 65535u).x), _217, _262.xx)) * in_var_Vertex_Position.z);
        _274 = float3(_272.x, _266.y, _272.y);
        _275 = _266;
    }
    else
    {
        float _232 = (in_var_bbx_x - 0.5f) * 2.0f;
        float _241 = clamp(_232, 0.0f, 1.0f);
        float3 _245 = float3(lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).x, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).x, _232), in_var_Vertex_Position.y + lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).y, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).y, _241), lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).z, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).z, _232));
        float2 _251 = _245.xz + (normalize(lerp(_217, float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108) >> 16u) & 65535u).x), _241.xx)) * in_var_Vertex_Position.z);
        _274 = float3(_251.x, _245.y, _251.y);
        _275 = _245;
    }
    float3 _276 = _274 * 0.4000000059604644775390625f;
    int3 _278 = int3(floor(_276));
    float3 _279 = frac(_276);
    uint3 _280 = uint3(_278);
    uint _283 = _280.z;
    uint _285 = _283 + (_283 << 10u);
    uint _287 = _285 ^ (_285 >> 6u);
    uint _289 = _287 + (_287 << 3u);
    uint _291 = _289 ^ (_289 >> 11u);
    uint _297 = (((_280.y * 1664525u) + (_291 + (_291 << 15u))) + 1013904223u) * 1664525u;
    uint _299 = _297 ^ (_297 >> 11u);
    uint _302 = _299 ^ ((_299 << 7u) & 2636928640u);
    uint _305 = _302 ^ ((_302 << 15u) & 4022730752u);
    uint _311 = (((_280.x * 1664525u) + (_305 ^ (_305 >> 18u))) + 1013904223u) * 1664525u;
    uint _313 = _311 ^ (_311 >> 11u);
    uint _316 = _313 ^ ((_313 << 7u) & 2636928640u);
    uint _319 = _316 ^ ((_316 << 15u) & 4022730752u);
    uint _321 = _319 ^ (_319 >> 18u);
    uint _323 = _321 + (_321 << 10u);
    uint _325 = _323 ^ (_323 >> 6u);
    uint _327 = _325 + (_325 << 3u);
    uint _329 = _327 ^ (_327 >> 11u);
    uint3 _342 = uint3(_278 + int3(1, 0, 0));
    uint _345 = _342.z;
    uint _347 = _345 + (_345 << 10u);
    uint _349 = _347 ^ (_347 >> 6u);
    uint _351 = _349 + (_349 << 3u);
    uint _353 = _351 ^ (_351 >> 11u);
    uint _359 = (((_342.y * 1664525u) + (_353 + (_353 << 15u))) + 1013904223u) * 1664525u;
    uint _361 = _359 ^ (_359 >> 11u);
    uint _364 = _361 ^ ((_361 << 7u) & 2636928640u);
    uint _367 = _364 ^ ((_364 << 15u) & 4022730752u);
    uint _373 = (((_342.x * 1664525u) + (_367 ^ (_367 >> 18u))) + 1013904223u) * 1664525u;
    uint _375 = _373 ^ (_373 >> 11u);
    uint _378 = _375 ^ ((_375 << 7u) & 2636928640u);
    uint _381 = _378 ^ ((_378 << 15u) & 4022730752u);
    uint _383 = _381 ^ (_381 >> 18u);
    uint _385 = _383 + (_383 << 10u);
    uint _387 = _385 ^ (_385 >> 6u);
    uint _389 = _387 + (_387 << 3u);
    uint _391 = _389 ^ (_389 >> 11u);
    uint3 _404 = uint3(_278 + int3(0, 1, 0));
    uint _407 = _404.z;
    uint _409 = _407 + (_407 << 10u);
    uint _411 = _409 ^ (_409 >> 6u);
    uint _413 = _411 + (_411 << 3u);
    uint _415 = _413 ^ (_413 >> 11u);
    uint _421 = (((_404.y * 1664525u) + (_415 + (_415 << 15u))) + 1013904223u) * 1664525u;
    uint _423 = _421 ^ (_421 >> 11u);
    uint _426 = _423 ^ ((_423 << 7u) & 2636928640u);
    uint _429 = _426 ^ ((_426 << 15u) & 4022730752u);
    uint _435 = (((_404.x * 1664525u) + (_429 ^ (_429 >> 18u))) + 1013904223u) * 1664525u;
    uint _437 = _435 ^ (_435 >> 11u);
    uint _440 = _437 ^ ((_437 << 7u) & 2636928640u);
    uint _443 = _440 ^ ((_440 << 15u) & 4022730752u);
    uint _445 = _443 ^ (_443 >> 18u);
    uint _447 = _445 + (_445 << 10u);
    uint _449 = _447 ^ (_447 >> 6u);
    uint _451 = _449 + (_449 << 3u);
    uint _453 = _451 ^ (_451 >> 11u);
    uint3 _466 = uint3(_278 + int3(1, 1, 0));
    uint _469 = _466.z;
    uint _471 = _469 + (_469 << 10u);
    uint _473 = _471 ^ (_471 >> 6u);
    uint _475 = _473 + (_473 << 3u);
    uint _477 = _475 ^ (_475 >> 11u);
    uint _483 = (((_466.y * 1664525u) + (_477 + (_477 << 15u))) + 1013904223u) * 1664525u;
    uint _485 = _483 ^ (_483 >> 11u);
    uint _488 = _485 ^ ((_485 << 7u) & 2636928640u);
    uint _491 = _488 ^ ((_488 << 15u) & 4022730752u);
    uint _497 = (((_466.x * 1664525u) + (_491 ^ (_491 >> 18u))) + 1013904223u) * 1664525u;
    uint _499 = _497 ^ (_497 >> 11u);
    uint _502 = _499 ^ ((_499 << 7u) & 2636928640u);
    uint _505 = _502 ^ ((_502 << 15u) & 4022730752u);
    uint _507 = _505 ^ (_505 >> 18u);
    uint _509 = _507 + (_507 << 10u);
    uint _511 = _509 ^ (_509 >> 6u);
    uint _513 = _511 + (_511 << 3u);
    uint _515 = _513 ^ (_513 >> 11u);
    uint3 _528 = uint3(_278 + int3(0, 0, 1));
    uint _531 = _528.z;
    uint _533 = _531 + (_531 << 10u);
    uint _535 = _533 ^ (_533 >> 6u);
    uint _537 = _535 + (_535 << 3u);
    uint _539 = _537 ^ (_537 >> 11u);
    uint _545 = (((_528.y * 1664525u) + (_539 + (_539 << 15u))) + 1013904223u) * 1664525u;
    uint _547 = _545 ^ (_545 >> 11u);
    uint _550 = _547 ^ ((_547 << 7u) & 2636928640u);
    uint _553 = _550 ^ ((_550 << 15u) & 4022730752u);
    uint _559 = (((_528.x * 1664525u) + (_553 ^ (_553 >> 18u))) + 1013904223u) * 1664525u;
    uint _561 = _559 ^ (_559 >> 11u);
    uint _564 = _561 ^ ((_561 << 7u) & 2636928640u);
    uint _567 = _564 ^ ((_564 << 15u) & 4022730752u);
    uint _569 = _567 ^ (_567 >> 18u);
    uint _571 = _569 + (_569 << 10u);
    uint _573 = _571 ^ (_571 >> 6u);
    uint _575 = _573 + (_573 << 3u);
    uint _577 = _575 ^ (_575 >> 11u);
    uint3 _590 = uint3(_278 + int3(1, 0, 1));
    uint _593 = _590.z;
    uint _595 = _593 + (_593 << 10u);
    uint _597 = _595 ^ (_595 >> 6u);
    uint _599 = _597 + (_597 << 3u);
    uint _601 = _599 ^ (_599 >> 11u);
    uint _607 = (((_590.y * 1664525u) + (_601 + (_601 << 15u))) + 1013904223u) * 1664525u;
    uint _609 = _607 ^ (_607 >> 11u);
    uint _612 = _609 ^ ((_609 << 7u) & 2636928640u);
    uint _615 = _612 ^ ((_612 << 15u) & 4022730752u);
    uint _621 = (((_590.x * 1664525u) + (_615 ^ (_615 >> 18u))) + 1013904223u) * 1664525u;
    uint _623 = _621 ^ (_621 >> 11u);
    uint _626 = _623 ^ ((_623 << 7u) & 2636928640u);
    uint _629 = _626 ^ ((_626 << 15u) & 4022730752u);
    uint _631 = _629 ^ (_629 >> 18u);
    uint _633 = _631 + (_631 << 10u);
    uint _635 = _633 ^ (_633 >> 6u);
    uint _637 = _635 + (_635 << 3u);
    uint _639 = _637 ^ (_637 >> 11u);
    uint3 _652 = uint3(_278 + int3(0, 1, 1));
    uint _655 = _652.z;
    uint _657 = _655 + (_655 << 10u);
    uint _659 = _657 ^ (_657 >> 6u);
    uint _661 = _659 + (_659 << 3u);
    uint _663 = _661 ^ (_661 >> 11u);
    uint _669 = (((_652.y * 1664525u) + (_663 + (_663 << 15u))) + 1013904223u) * 1664525u;
    uint _671 = _669 ^ (_669 >> 11u);
    uint _674 = _671 ^ ((_671 << 7u) & 2636928640u);
    uint _677 = _674 ^ ((_674 << 15u) & 4022730752u);
    uint _683 = (((_652.x * 1664525u) + (_677 ^ (_677 >> 18u))) + 1013904223u) * 1664525u;
    uint _685 = _683 ^ (_683 >> 11u);
    uint _688 = _685 ^ ((_685 << 7u) & 2636928640u);
    uint _691 = _688 ^ ((_688 << 15u) & 4022730752u);
    uint _693 = _691 ^ (_691 >> 18u);
    uint _695 = _693 + (_693 << 10u);
    uint _697 = _695 ^ (_695 >> 6u);
    uint _699 = _697 + (_697 << 3u);
    uint _701 = _699 ^ (_699 >> 11u);
    uint3 _714 = uint3(_278 + int3(1, 1, 1));
    uint _717 = _714.z;
    uint _719 = _717 + (_717 << 10u);
    uint _721 = _719 ^ (_719 >> 6u);
    uint _723 = _721 + (_721 << 3u);
    uint _725 = _723 ^ (_723 >> 11u);
    uint _731 = (((_714.y * 1664525u) + (_725 + (_725 << 15u))) + 1013904223u) * 1664525u;
    uint _733 = _731 ^ (_731 >> 11u);
    uint _736 = _733 ^ ((_733 << 7u) & 2636928640u);
    uint _739 = _736 ^ ((_736 << 15u) & 4022730752u);
    uint _745 = (((_714.x * 1664525u) + (_739 ^ (_739 >> 18u))) + 1013904223u) * 1664525u;
    uint _747 = _745 ^ (_745 >> 11u);
    uint _750 = _747 ^ ((_747 << 7u) & 2636928640u);
    uint _753 = _750 ^ ((_750 << 15u) & 4022730752u);
    uint _755 = _753 ^ (_753 >> 18u);
    uint _757 = _755 + (_755 << 10u);
    uint _759 = _757 ^ (_757 >> 6u);
    uint _761 = _759 + (_759 << 3u);
    uint _763 = _761 ^ (_761 >> 11u);
    float3 _778 = (_279 * _279) * (3.0f.xxx - (_279 * 2.0f));
    float _779 = _778.x;
    float _780 = 1.0f - _779;
    float _782 = _778.y;
    float _783 = 1.0f - _782;
    float _785 = _778.z;
    float _786 = 1.0f - _785;
    float2 _803 = ((((((float2(asfloat((_321 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_329 + (_329 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _780) * _783) * _786) + (((float2(asfloat((_383 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_391 + (_391 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _779) * _783) * _786)) + (((float2(asfloat((_445 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_453 + (_453 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _780) * _782) * _786)) + (((float2(asfloat((_507 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_515 + (_515 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _779) * _782) * _786)) + (((float2(asfloat((_569 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_577 + (_577 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _780) * _783) * _785);
    float2 _818 = clamp(((((_803 + (((float2(asfloat((_631 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_639 + (_639 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _779) * _783) * _785)) + (((float2(asfloat((_693 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_701 + (_701 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _780) * _782) * _785)) + (((float2(asfloat((_755 & 8388607u) | 1065353216u) - 1.0f, asfloat(((_763 + (_763 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * _779) * _782) * _785)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 _823 = _274 + (float3(_818.x, 0.0f, _818.y) * 0.0599999986588954925537109375f);
    int _825 = (in_var_is_bevel != 0) ? 768 : 512;
    bool _826 = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 60) != 0u;
    int _830;
    if (_826)
    {
        _830 = _825 | 1024;
    }
    else
    {
        _830 = _825;
    }
    bool2 _854 = _826.xx;
    gl_Position = mul(frame_constants_view_constants[_Globals_current_view_index].world_to_sample, float4(_823, 1.0f));
    out_var_C0 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48) + in_var_brick_id);
    out_var_C1 = mul(transpose(float4x3(float3(_172.data[0], _172.data[1], _172.data[2]), float3(_172.data[3], _172.data[4], _172.data[5]), float3(_172.data[6], _172.data[7], _172.data[8]), float3(_172.data[9], _172.data[10], _172.data[11]))), float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = _823;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(_854.x ? in_var_Vertex_UV.x : 0.0f.xx.x, _854.y ? in_var_Vertex_UV.y : 0.0f.xx.y);
    out_var_C11 = 0;
    out_var_C12 = _275.xz;
    out_var_C13 = _830;
    out_var_C14 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
}

SPIRV_Cross_Output vs_main(SPIRV_Cross_Input stage_input)
{
    gl_InstanceIndex = int(stage_input.gl_InstanceIndex);
    in_var_Vertex_Position = stage_input.in_var_Vertex_Position;
    in_var_Vertex_Normal = stage_input.in_var_Vertex_Normal;
    in_var_Vertex_UV = stage_input.in_var_Vertex_UV;
    in_var_is_bevel = stage_input.in_var_is_bevel;
    in_var_brick_id = stage_input.in_var_brick_id;
    in_var_bbx_x = stage_input.in_var_bbx_x;
    vert_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_Position = gl_Position;
    stage_output.out_var_C0 = out_var_C0;
    stage_output.out_var_C1 = out_var_C1;
    stage_output.out_var_C2 = out_var_C2;
    stage_output.out_var_C5 = out_var_C5;
    stage_output.out_var_C7 = out_var_C7;
    stage_output.out_var_C10 = out_var_C10;
    stage_output.out_var_C11 = out_var_C11;
    stage_output.out_var_C12 = out_var_C12;
    stage_output.out_var_C13 = out_var_C13;
    stage_output.out_var_C14 = out_var_C14;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _nani_gothic_window_bricks.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
static float4 gl_FragCoord;
static float gl_FragDepth;
static float3 in_var_C2;
static float in_var_C7;
static int in_var_C13;

struct SPIRV_Cross_Input
{
    [[vk::location(2)]] float3 in_var_C2 : TEXCOORD2;
    [[vk::location(7)]] float in_var_C7 : TEXCOORD7;
    [[vk::location(13)]] nointerpolation int in_var_C13 : TEXCOORD13;
    float4 gl_FragCoord : SV_Position;
};

struct SPIRV_Cross_Output
{
    float gl_FragDepth : SV_DepthLessEqual;
};

void frag_main()
{
    bool _70 = (in_var_C13 & 8) != 0;
    bool _75;
    if (!_70)
    {
        _75 = in_var_C7 > (-2.5f);
    }
    else
    {
        _75 = false;
    }
    bool _80;
    if (_75)
    {
        _80 = in_var_C2.y < in_var_C7;
    }
    else
    {
        _80 = false;
    }
    if (_80)
    {
        discard;
    }
    else
    {
        bool _88;
        if (_70)
        {
            _88 = in_var_C2.y > in_var_C7;
        }
        else
        {
            _88 = false;
        }
        if (_88)
        {
            discard;
        }
    }
    gl_FragDepth = gl_FragCoord.z;
}

SPIRV_Cross_Output ps_main(SPIRV_Cross_Input stage_input)
{
    gl_FragCoord = stage_input.gl_FragCoord;
    gl_FragCoord.w = 1.0 / gl_FragCoord.w;
    in_var_C2 = stage_input.in_var_C2;
    in_var_C7 = stage_input.in_var_C7;
    in_var_C13 = stage_input.in_var_C13;
    frag_main();
    SPIRV_Cross_Output stage_output;
    stage_output.gl_FragDepth = gl_FragDepth;
    return stage_output;
}
#pragma dxc diagnostic pop
#endif
