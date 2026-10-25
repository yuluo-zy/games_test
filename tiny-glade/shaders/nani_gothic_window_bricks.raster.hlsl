// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_gothic_window_bricks.raster.hlsl | B nani raster | bins: _nani_gothic_window_bricks.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani 哥特窗砖墙:砖块材质 + 窗框配色、积雪着色与半透明抖动;打包 gbuffer。

// Registerless terrain heightmap global for the inc/terrain_heightmap.inc.hlsl
// terrain_height() helper (INC fix #3); unused here, dxc DCEs it.
Texture2D terrain_heightmap : register(t63);

// 原 color 变体的 color_library 为 set0 binding21，不能与 VS 的 t14 混用。
#define TINYGLADE_COLOR_LIBRARY_REGISTER b21
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
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 Vertex_UV: Vertex_UV;
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
    uint current_view_index : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
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
    InstanceData instance;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        instance.xform.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    instance.seed = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48);
    instance._wall_id = nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 52);
    instance.wallspace_x_range_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 56);
    instance._is_wooden = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 60);
    instance.bone_0_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64);
    instance.bone_0_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76);
    instance.bone_1_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80);
    instance.bone_1_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92);
    instance.bone_2_pos = nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96);
    instance.bone_2_normal_packed = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108);
    float2 bone1_normal = float2(spvUnpackHalf2x16(instance.bone_1_normal_packed & 65535u).x, spvUnpackHalf2x16((instance.bone_1_normal_packed >> 16u) & 65535u).x);
    float3 bent_pos;
    float3 bone_pos;
    if (in_var_bbx_x < 0.5f)
    {
        float bbx_x2 = in_var_bbx_x * 2.0f;
        float bbx_x2c = clamp(bbx_x2, 0.0f, 1.0f);
        float3 bone_pos01 = float3(lerp(instance.bone_0_pos.x, instance.bone_1_pos.x, bbx_x2), in_var_Vertex_Position.y + lerp(instance.bone_0_pos.y, instance.bone_1_pos.y, bbx_x2c), lerp(instance.bone_0_pos.z, instance.bone_1_pos.z, bbx_x2));
        float2 bone_pos01_xz = bone_pos01.xz + (normalize(lerp(float2(spvUnpackHalf2x16(instance.bone_0_normal_packed & 65535u).x, spvUnpackHalf2x16((instance.bone_0_normal_packed >> 16u) & 65535u).x), bone1_normal, bbx_x2c.xx)) * in_var_Vertex_Position.z);
        bent_pos = float3(bone_pos01_xz.x, bone_pos01.y, bone_pos01_xz.y);
        bone_pos = bone_pos01;
    }
    else
    {
        float bbx_x2b = (in_var_bbx_x - 0.5f) * 2.0f;
        float bbx_x2bc = clamp(bbx_x2b, 0.0f, 1.0f);
        float3 bone_pos12 = float3(lerp(instance.bone_1_pos.x, instance.bone_2_pos.x, bbx_x2b), in_var_Vertex_Position.y + lerp(instance.bone_1_pos.y, instance.bone_2_pos.y, bbx_x2bc), lerp(instance.bone_1_pos.z, instance.bone_2_pos.z, bbx_x2b));
        float2 bone_pos12_xz = bone_pos12.xz + (normalize(lerp(bone1_normal, float2(spvUnpackHalf2x16(instance.bone_2_normal_packed & 65535u).x, spvUnpackHalf2x16((instance.bone_2_normal_packed >> 16u) & 65535u).x), bbx_x2bc.xx)) * in_var_Vertex_Position.z);
        bent_pos = float3(bone_pos12_xz.x, bone_pos12.y, bone_pos12_xz.y);
        bone_pos = bone_pos12;
    }
    float3 scaled_pos = bent_pos * 0.4000000059604644775390625f;
    int3 lattice_cell = int3(floor(scaled_pos));
    float3 lattice_frac = frac(scaled_pos);
    uint3 c000_cell = uint3(lattice_cell);
    uint c000_z = c000_cell.z;
    uint c000_z10 = c000_z + (c000_z << 10u);
    uint c000_z6 = c000_z10 ^ (c000_z10 >> 6u);
    uint c000_z3 = c000_z6 + (c000_z6 << 3u);
    uint c000_z11 = c000_z3 ^ (c000_z3 >> 11u);
    uint c000_mix = (((c000_cell.y * 1664525u) + (c000_z11 + (c000_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c000_x11 = c000_mix ^ (c000_mix >> 11u);
    uint c000_x7 = c000_x11 ^ ((c000_x11 << 7u) & 2636928640u);
    uint c000_x15 = c000_x7 ^ ((c000_x7 << 15u) & 4022730752u);
    uint c000_mix2 = (((c000_cell.x * 1664525u) + (c000_x15 ^ (c000_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c000_x112 = c000_mix2 ^ (c000_mix2 >> 11u);
    uint c000_x72 = c000_x112 ^ ((c000_x112 << 7u) & 2636928640u);
    uint c000_x152 = c000_x72 ^ ((c000_x72 << 15u) & 4022730752u);
    uint c000_rand = c000_x152 ^ (c000_x152 >> 18u);
    uint c000_h10 = c000_rand + (c000_rand << 10u);
    uint c000_h6 = c000_h10 ^ (c000_h10 >> 6u);
    uint c000_h3 = c000_h6 + (c000_h6 << 3u);
    uint c000_h11 = c000_h3 ^ (c000_h3 >> 11u);
    uint3 c100_cell = uint3(lattice_cell + int3(1, 0, 0));
    uint c100_z = c100_cell.z;
    uint c100_z10 = c100_z + (c100_z << 10u);
    uint c100_z6 = c100_z10 ^ (c100_z10 >> 6u);
    uint c100_z3 = c100_z6 + (c100_z6 << 3u);
    uint c100_z11 = c100_z3 ^ (c100_z3 >> 11u);
    uint c100_mix = (((c100_cell.y * 1664525u) + (c100_z11 + (c100_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c100_x11 = c100_mix ^ (c100_mix >> 11u);
    uint c100_x7 = c100_x11 ^ ((c100_x11 << 7u) & 2636928640u);
    uint c100_x15 = c100_x7 ^ ((c100_x7 << 15u) & 4022730752u);
    uint c100_mix2 = (((c100_cell.x * 1664525u) + (c100_x15 ^ (c100_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c100_x112 = c100_mix2 ^ (c100_mix2 >> 11u);
    uint c100_x72 = c100_x112 ^ ((c100_x112 << 7u) & 2636928640u);
    uint c100_x152 = c100_x72 ^ ((c100_x72 << 15u) & 4022730752u);
    uint c100_rand = c100_x152 ^ (c100_x152 >> 18u);
    uint c100_h10 = c100_rand + (c100_rand << 10u);
    uint c100_h6 = c100_h10 ^ (c100_h10 >> 6u);
    uint c100_h3 = c100_h6 + (c100_h6 << 3u);
    uint c100_h11 = c100_h3 ^ (c100_h3 >> 11u);
    uint3 c010_cell = uint3(lattice_cell + int3(0, 1, 0));
    uint c010_z = c010_cell.z;
    uint c010_z10 = c010_z + (c010_z << 10u);
    uint c010_z6 = c010_z10 ^ (c010_z10 >> 6u);
    uint c010_z3 = c010_z6 + (c010_z6 << 3u);
    uint c010_z11 = c010_z3 ^ (c010_z3 >> 11u);
    uint c010_mix = (((c010_cell.y * 1664525u) + (c010_z11 + (c010_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c010_x11 = c010_mix ^ (c010_mix >> 11u);
    uint c010_x7 = c010_x11 ^ ((c010_x11 << 7u) & 2636928640u);
    uint c010_x15 = c010_x7 ^ ((c010_x7 << 15u) & 4022730752u);
    uint c010_mix2 = (((c010_cell.x * 1664525u) + (c010_x15 ^ (c010_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c010_x112 = c010_mix2 ^ (c010_mix2 >> 11u);
    uint c010_x72 = c010_x112 ^ ((c010_x112 << 7u) & 2636928640u);
    uint c010_x152 = c010_x72 ^ ((c010_x72 << 15u) & 4022730752u);
    uint c010_rand = c010_x152 ^ (c010_x152 >> 18u);
    uint c010_h10 = c010_rand + (c010_rand << 10u);
    uint c010_h6 = c010_h10 ^ (c010_h10 >> 6u);
    uint c010_h3 = c010_h6 + (c010_h6 << 3u);
    uint c010_h11 = c010_h3 ^ (c010_h3 >> 11u);
    uint3 c110_cell = uint3(lattice_cell + int3(1, 1, 0));
    uint c110_z = c110_cell.z;
    uint c110_z10 = c110_z + (c110_z << 10u);
    uint c110_z6 = c110_z10 ^ (c110_z10 >> 6u);
    uint c110_z3 = c110_z6 + (c110_z6 << 3u);
    uint c110_z11 = c110_z3 ^ (c110_z3 >> 11u);
    uint c110_mix = (((c110_cell.y * 1664525u) + (c110_z11 + (c110_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c110_x11 = c110_mix ^ (c110_mix >> 11u);
    uint c110_x7 = c110_x11 ^ ((c110_x11 << 7u) & 2636928640u);
    uint c110_x15 = c110_x7 ^ ((c110_x7 << 15u) & 4022730752u);
    uint c110_mix2 = (((c110_cell.x * 1664525u) + (c110_x15 ^ (c110_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c110_x112 = c110_mix2 ^ (c110_mix2 >> 11u);
    uint c110_x72 = c110_x112 ^ ((c110_x112 << 7u) & 2636928640u);
    uint c110_x152 = c110_x72 ^ ((c110_x72 << 15u) & 4022730752u);
    uint c110_rand = c110_x152 ^ (c110_x152 >> 18u);
    uint c110_h10 = c110_rand + (c110_rand << 10u);
    uint c110_h6 = c110_h10 ^ (c110_h10 >> 6u);
    uint c110_h3 = c110_h6 + (c110_h6 << 3u);
    uint c110_h11 = c110_h3 ^ (c110_h3 >> 11u);
    uint3 c001_cell = uint3(lattice_cell + int3(0, 0, 1));
    uint c001_z = c001_cell.z;
    uint c001_z10 = c001_z + (c001_z << 10u);
    uint c001_z6 = c001_z10 ^ (c001_z10 >> 6u);
    uint c001_z3 = c001_z6 + (c001_z6 << 3u);
    uint c001_z11 = c001_z3 ^ (c001_z3 >> 11u);
    uint c001_mix = (((c001_cell.y * 1664525u) + (c001_z11 + (c001_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c001_x11 = c001_mix ^ (c001_mix >> 11u);
    uint c001_x7 = c001_x11 ^ ((c001_x11 << 7u) & 2636928640u);
    uint c001_x15 = c001_x7 ^ ((c001_x7 << 15u) & 4022730752u);
    uint c001_mix2 = (((c001_cell.x * 1664525u) + (c001_x15 ^ (c001_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c001_x112 = c001_mix2 ^ (c001_mix2 >> 11u);
    uint c001_x72 = c001_x112 ^ ((c001_x112 << 7u) & 2636928640u);
    uint c001_x152 = c001_x72 ^ ((c001_x72 << 15u) & 4022730752u);
    uint c001_rand = c001_x152 ^ (c001_x152 >> 18u);
    uint c001_h10 = c001_rand + (c001_rand << 10u);
    uint c001_h6 = c001_h10 ^ (c001_h10 >> 6u);
    uint c001_h3 = c001_h6 + (c001_h6 << 3u);
    uint c001_h11 = c001_h3 ^ (c001_h3 >> 11u);
    uint3 c101_cell = uint3(lattice_cell + int3(1, 0, 1));
    uint c101_z = c101_cell.z;
    uint c101_z10 = c101_z + (c101_z << 10u);
    uint c101_z6 = c101_z10 ^ (c101_z10 >> 6u);
    uint c101_z3 = c101_z6 + (c101_z6 << 3u);
    uint c101_z11 = c101_z3 ^ (c101_z3 >> 11u);
    uint c101_mix = (((c101_cell.y * 1664525u) + (c101_z11 + (c101_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c101_x11 = c101_mix ^ (c101_mix >> 11u);
    uint c101_x7 = c101_x11 ^ ((c101_x11 << 7u) & 2636928640u);
    uint c101_x15 = c101_x7 ^ ((c101_x7 << 15u) & 4022730752u);
    uint c101_mix2 = (((c101_cell.x * 1664525u) + (c101_x15 ^ (c101_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c101_x112 = c101_mix2 ^ (c101_mix2 >> 11u);
    uint c101_x72 = c101_x112 ^ ((c101_x112 << 7u) & 2636928640u);
    uint c101_x152 = c101_x72 ^ ((c101_x72 << 15u) & 4022730752u);
    uint c101_rand = c101_x152 ^ (c101_x152 >> 18u);
    uint c101_h10 = c101_rand + (c101_rand << 10u);
    uint c101_h6 = c101_h10 ^ (c101_h10 >> 6u);
    uint c101_h3 = c101_h6 + (c101_h6 << 3u);
    uint c101_h11 = c101_h3 ^ (c101_h3 >> 11u);
    uint3 c011_cell = uint3(lattice_cell + int3(0, 1, 1));
    uint c011_z = c011_cell.z;
    uint c011_z10 = c011_z + (c011_z << 10u);
    uint c011_z6 = c011_z10 ^ (c011_z10 >> 6u);
    uint c011_z3 = c011_z6 + (c011_z6 << 3u);
    uint c011_z11 = c011_z3 ^ (c011_z3 >> 11u);
    uint c011_mix = (((c011_cell.y * 1664525u) + (c011_z11 + (c011_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c011_x11 = c011_mix ^ (c011_mix >> 11u);
    uint c011_x7 = c011_x11 ^ ((c011_x11 << 7u) & 2636928640u);
    uint c011_x15 = c011_x7 ^ ((c011_x7 << 15u) & 4022730752u);
    uint c011_mix2 = (((c011_cell.x * 1664525u) + (c011_x15 ^ (c011_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c011_x112 = c011_mix2 ^ (c011_mix2 >> 11u);
    uint c011_x72 = c011_x112 ^ ((c011_x112 << 7u) & 2636928640u);
    uint c011_x152 = c011_x72 ^ ((c011_x72 << 15u) & 4022730752u);
    uint c011_rand = c011_x152 ^ (c011_x152 >> 18u);
    uint c011_h10 = c011_rand + (c011_rand << 10u);
    uint c011_h6 = c011_h10 ^ (c011_h10 >> 6u);
    uint c011_h3 = c011_h6 + (c011_h6 << 3u);
    uint c011_h11 = c011_h3 ^ (c011_h3 >> 11u);
    uint3 c111_cell = uint3(lattice_cell + int3(1, 1, 1));
    uint c111_z = c111_cell.z;
    uint c111_z10 = c111_z + (c111_z << 10u);
    uint c111_z6 = c111_z10 ^ (c111_z10 >> 6u);
    uint c111_z3 = c111_z6 + (c111_z6 << 3u);
    uint c111_z11 = c111_z3 ^ (c111_z3 >> 11u);
    uint c111_mix = (((c111_cell.y * 1664525u) + (c111_z11 + (c111_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c111_x11 = c111_mix ^ (c111_mix >> 11u);
    uint c111_x7 = c111_x11 ^ ((c111_x11 << 7u) & 2636928640u);
    uint c111_x15 = c111_x7 ^ ((c111_x7 << 15u) & 4022730752u);
    uint c111_mix2 = (((c111_cell.x * 1664525u) + (c111_x15 ^ (c111_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c111_x112 = c111_mix2 ^ (c111_mix2 >> 11u);
    uint c111_x72 = c111_x112 ^ ((c111_x112 << 7u) & 2636928640u);
    uint c111_x152 = c111_x72 ^ ((c111_x72 << 15u) & 4022730752u);
    uint c111_rand = c111_x152 ^ (c111_x152 >> 18u);
    uint c111_h10 = c111_rand + (c111_rand << 10u);
    uint c111_h6 = c111_h10 ^ (c111_h10 >> 6u);
    uint c111_h3 = c111_h6 + (c111_h6 << 3u);
    uint c111_h11 = c111_h3 ^ (c111_h3 >> 11u);
    float3 smooth = (lattice_frac * lattice_frac) * (3.0f.xxx - (lattice_frac * 2.0f));
    float sx = smooth.x;
    float sx_inv = 1.0f - sx;
    float sy = smooth.y;
    float sy_inv = 1.0f - sy;
    float sz = smooth.z;
    float sz_inv = 1.0f - sz;
    float2 noise_xy = ((((((float2(asfloat((c000_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c000_h11 + (c000_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy_inv) * sz_inv) + (((float2(asfloat((c100_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c100_h11 + (c100_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy_inv) * sz_inv)) + (((float2(asfloat((c010_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c010_h11 + (c010_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy) * sz_inv)) + (((float2(asfloat((c110_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c110_h11 + (c110_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy) * sz_inv)) + (((float2(asfloat((c001_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c001_h11 + (c001_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy_inv) * sz);
    float2 jitter = clamp(((((noise_xy + (((float2(asfloat((c101_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c101_h11 + (c101_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy_inv) * sz)) + (((float2(asfloat((c011_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c011_h11 + (c011_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy) * sz)) + (((float2(asfloat((c111_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c111_h11 + (c111_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy) * sz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced = bent_pos + (float3(jitter.x, 0.0f, jitter.y) * 0.0599999986588954925537109375f);
    int flags = (in_var_is_bevel != 0) ? 768 : 512;
    bool is_wooden = instance._is_wooden != 0u;
    int flags_out;
    if (is_wooden)
    {
        flags_out = flags | 1024;
    }
    else
    {
        flags_out = flags;
    }
    uint seed_combined = uint(instance.seed + in_var_brick_id);
    uint wall_id_u = uint(instance._wall_id);
    uint wall_id = wall_id_u & 65535u;
    uint ws_h10 = wall_id ^ 7993234u;
    uint ws_h6 = ws_h10 + (ws_h10 << 10u);
    uint ws_h3 = ws_h6 ^ (ws_h6 >> 6u);
    uint ws_h11 = ws_h3 + (ws_h3 << 3u);
    uint ws_rand = ws_h11 ^ (ws_h11 >> 11u);
    uint wc_h10 = wall_id ^ 3450234u;
    uint wc_h6 = wc_h10 + (wc_h10 << 10u);
    uint wc_h3 = wc_h6 ^ (wc_h6 >> 6u);
    uint wc_h11 = wc_h3 + (wc_h3 << 3u);
    uint wc_rand = wc_h11 ^ (wc_h11 >> 11u);
    float2 wallspace = (((float2(lerp(spvUnpackHalf2x16(instance.wallspace_x_range_packed & 65535u).x, spvUnpackHalf2x16((instance.wallspace_x_range_packed >> 16u) & 65535u).x, in_var_bbx_x), displaced.y) * 0.100000001490116119384765625f) * 0.699999988079071044921875f) + (float(wall_id) * 0.20000000298023223876953125f).xx) + float2(asfloat(((ws_rand + (ws_rand << 15u)) & 8388607u) | 1065353216u) - 1.0f, asfloat(((wc_rand + (wc_rand << 15u)) & 8388607u) | 1065353216u) - 1.0f);
    uint wj_h10 = seed_combined ^ 292478890u;
    uint wj_h6 = wj_h10 + (wj_h10 << 10u);
    uint wj_h3 = wj_h6 ^ (wj_h6 >> 6u);
    uint wj_h11 = wj_h3 + (wj_h3 << 3u);
    uint wj_rand = wj_h11 ^ (wj_h11 >> 11u);
    float rand_wall = asfloat(((wj_rand + (wj_rand << 15u)) & 8388607u) | 1065353216u) - 1.0f;
    float2 brick_uv;
    if (rand_wall > 0.89999997615814208984375f)
    {
        brick_uv = wallspace * 3.0f;
    }
    else
    {
        brick_uv = wallspace + (0.61000001430511474609375f * rand_wall).xx;
    }
    float2 brick_uv_scaled;
    if (int((flags_out & 512) != 0) == 1)
    {
        brick_uv_scaled = brick_uv * 2.2000000476837158203125f;
    }
    else
    {
        brick_uv_scaled = brick_uv;
    }
    bool2 is_wooden2 = is_wooden.xx;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = seed_combined;
    out_var_C1 = mul(transpose(float4x3(float3(instance.xform.data[0], instance.xform.data[1], instance.xform.data[2]), float3(instance.xform.data[3], instance.xform.data[4], instance.xform.data[5]), float3(instance.xform.data[6], instance.xform.data[7], instance.xform.data[8]), float3(instance.xform.data[9], instance.xform.data[10], instance.xform.data[11]))), float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(is_wooden2.x ? in_var_Vertex_UV.x : brick_uv_scaled.x, is_wooden2.y ? in_var_Vertex_UV.y : brick_uv_scaled.y);
    out_var_C11 = int(wall_color_ids.Load<uint>(wall_id_u * 4 + 0));
    out_var_C12 = bone_pos.xz;
    out_var_C13 = flags_out;
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

static uint mask_uint;
static float3 mask_vec;

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
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
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
    bool is_bevel = (in_var_C13 & 512) != 0;
    bool is_edge = (in_var_C13 & 8) != 0;
    bool is_rough = (in_var_C13 & 16) != 0;
    bool is_highlight = (in_var_C13 & 2048) != 0;
    bool is_variant = (in_var_C13 & 256) != 0;
    bool below_terrain;
    if (!is_edge)
    {
        below_terrain = in_var_C7 > (-2.5f);
    }
    else
    {
        below_terrain = false;
    }
    bool is_solid;
    if (below_terrain)
    {
        is_solid = in_var_C2.y < in_var_C7;
    }
    else
    {
        is_solid = false;
    }
    if (is_solid)
    {
        discard;
    }
    else
    {
        bool above_terrain;
        if (is_edge)
        {
            above_terrain = in_var_C2.y > in_var_C7;
        }
        else
        {
            above_terrain = false;
        }
        if (above_terrain)
        {
            discard;
        }
    }
    uint wall_id = uint(in_var_C11);
    float3 base_color;
    if ((in_var_C13 & 2) != 0)
    {
        base_color = roof_colors.Sample(sampler_llr, float3((float2(in_var_C0.xx) * 0.20000000298023223876953125f) + (in_var_C2.xz * 0.20000000298023223876953125f), float(wall_id))).xyz * 0.699999988079071044921875f;
    }
    else
    {
        float2 brick_uv;
        if (is_variant)
        {
            brick_uv = in_var_C10 + 0.20999999344348907470703125f.xx;
        }
        else
        {
            brick_uv = in_var_C10;
        }
        bool is_wall01;
        if (!(wall_id == 1u))
        {
            is_wall01 = wall_id == 0u;
        }
        else
        {
            is_wall01 = true;
        }
        float3 wall_color;
        if (is_wall01)
        {
            wall_color = pow(color_library_wall_bricks.xyz, 1.2999999523162841796875f.xxx);
        }
        else
        {
            wall_color = pow(color_library_wall_moss.xyz, 1.0f.xxx) * 0.800000011920928955078125f;
        }
        int wall_id_i = int(wall_id);
        bool is_wall4;
        if (!(wall_id_i == 0))
        {
            is_wall4 = wall_id_i == 4;
        }
        else
        {
            is_wall4 = true;
        }
        bool is_wall10;
        if (!is_wall4)
        {
            is_wall10 = wall_id_i == 10;
        }
        else
        {
            is_wall10 = true;
        }
        bool is_wall12;
        if (!is_wall10)
        {
            is_wall12 = wall_id_i == 12;
        }
        else
        {
            is_wall12 = true;
        }
        float3 brick_color;
        if (is_wall12)
        {
            brick_color = brick_colors.Sample(sampler_llr, float3(brick_uv + 0.85000002384185791015625f.xx, 0.0f)).xyz;
        }
        else
        {
            bool is_brick15;
            if (!(wall_id_i == 1))
            {
                is_brick15 = wall_id_i == 5;
            }
            else
            {
                is_brick15 = true;
            }
            float3 brick_color_pow;
            if (is_brick15)
            {
                brick_color_pow = pow(brick_colors.Sample(sampler_llr, float3(brick_uv + 0.449999988079071044921875f.xx, 1.0f)).xyz, 1.10000002384185791015625f.xxx);
            }
            else
            {
                bool is_brick27;
                if (!(wall_id_i == 2))
                {
                    is_brick27 = wall_id_i == 7;
                }
                else
                {
                    is_brick27 = true;
                }
                float3 brick_color2;
                if (is_brick27)
                {
                    brick_color2 = brick_colors.Sample(sampler_llr, float3(brick_uv, 2.0f)).xyz;
                }
                else
                {
                    bool is_brick36;
                    if (!(wall_id_i == 3))
                    {
                        is_brick36 = wall_id_i == 6;
                    }
                    else
                    {
                        is_brick36 = true;
                    }
                    bool is_brick8;
                    if (!is_brick36)
                    {
                        is_brick8 = wall_id_i == 8;
                    }
                    else
                    {
                        is_brick8 = true;
                    }
                    bool is_brick9;
                    if (!is_brick8)
                    {
                        is_brick9 = wall_id_i == 9;
                    }
                    else
                    {
                        is_brick9 = true;
                    }
                    float3 brick_color3;
                    if (is_brick9)
                    {
                        brick_color3 = brick_colors.Sample(sampler_llr, float3(brick_uv, 3.0f)).xyz;
                    }
                    else
                    {
                        float3 brick_color4;
                        if (wall_id_i == 11)
                        {
                            brick_color4 = brick_colors.Sample(sampler_llr, float3(brick_uv, 4.0f)).xyz;
                        }
                        else
                        {
                            brick_color4 = 0.0f.xxx;
                        }
                        brick_color3 = brick_color4;
                    }
                    brick_color2 = brick_color3;
                }
                brick_color_pow = brick_color2;
            }
            brick_color = brick_color_pow;
        }
        bool not_wall7 = !(wall_id == 7u);
        bool is_wall2;
        if (not_wall7)
        {
            is_wall2 = wall_id == 2u;
        }
        else
        {
            is_wall2 = true;
        }
        bool is_wall11;
        if (!is_wall2)
        {
            is_wall11 = wall_id == 11u;
        }
        else
        {
            is_wall11 = true;
        }
        bool3 wall11_vec = is_wall11.xxx;
        float3 mortar_color;
        if (is_bevel)
        {
            uint seed_a = in_var_C0 ^ 8902390u;
            uint seed_a10 = seed_a + (seed_a << 10u);
            uint seed_a6 = seed_a10 ^ (seed_a10 >> 6u);
            uint seed_a3 = seed_a6 + (seed_a6 << 3u);
            uint seed_a_rand = seed_a3 ^ (seed_a3 >> 11u);
            mortar_color = lerp(brick_color, float3(wall11_vec.x ? brick_color.x : wall_color.x, wall11_vec.y ? brick_color.y : wall_color.y, wall11_vec.z ? brick_color.z : wall_color.z), lerp(0.0f, 0.5f, smoothstep(0.20000000298023223876953125f, 0.800000011920928955078125f, asfloat(((seed_a_rand + (seed_a_rand << 15u)) & 8388607u) | 1065353216u) - 1.0f)).xxx);
        }
        else
        {
            mortar_color = brick_color;
        }
        bool blend_flag = is_bevel ? true : false;
        float3 wood_blend;
        if (is_variant)
        {
            bool is_wall2b;
            if (not_wall7)
            {
                is_wall2b = wall_id == 2u;
            }
            else
            {
                is_wall2b = true;
            }
            uint seed_b = in_var_C0 ^ 239085u;
            uint seed_b10 = seed_b + (seed_b << 10u);
            uint seed_b6 = seed_b10 ^ (seed_b10 >> 6u);
            uint seed_b3 = seed_b6 + (seed_b6 << 3u);
            uint seed_b_rand = seed_b3 ^ (seed_b3 >> 11u);
            wood_blend = lerp(mortar_color, color_library_wall_1.xyz, lerp(0.0f, is_wall2b ? 0.20000000298023223876953125f : 0.4000000059604644775390625f, smoothstep(0.0f, 0.800000011920928955078125f, asfloat(((seed_b_rand + (seed_b_rand << 15u)) & (blend_flag ? (is_bevel ? 8388607u : mask_uint) : 8388607u)) | (blend_flag ? (is_bevel ? 1065353216u : mask_uint) : 1065353216u)) - 1.0f)).xxx);
        }
        else
        {
            wood_blend = mortar_color;
        }
        base_color = wood_blend;
    }
    float3 world_normal = normalize(in_var_C1);
    bool is_eraser;
    float3 eraser_color;
    if (!((in_var_C13 & 64) != 0))
    {
        bool is_painted;
        float3 painted_color;
        if (highlight_mode == 1u)
        {
            is_painted = true;
            painted_color = lerp(base_color, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
        }
        else
        {
            bool is_painted_b = highlight_mode == 2u;
            float3 painted_color_b;
            if (is_painted_b)
            {
                painted_color_b = lerp(base_color, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
            }
            else
            {
                painted_color_b = base_color;
            }
            is_painted = is_painted_b ? true : false;
            painted_color = painted_color_b;
        }
        bool is_brush;
        float3 brush_color_out;
        if (highlight_bricks_to_delete == 1)
        {
            bool near_brush = distance(brush_position.xz, in_var_C12) < (1.88900005817413330078125f * eraser_mult);
            float3 brush_color;
            if (near_brush)
            {
                brush_color = lerp(base_color, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.800000011920928955078125f.xxx);
            }
            else
            {
                brush_color = painted_color;
            }
            is_brush = near_brush ? true : is_painted;
            brush_color_out = brush_color;
        }
        else
        {
            is_brush = is_painted;
            brush_color_out = painted_color;
        }
        is_eraser = is_brush;
        eraser_color = brush_color_out;
    }
    else
    {
        is_eraser = false;
        eraser_color = base_color;
    }
    float3 highlight_color;
    if (is_highlight)
    {
        highlight_color = lerp(base_color, float3(0.699999988079071044921875f, 0.2100000083446502685546875f, 0.0f), 0.5f.xxx);
    }
    else
    {
        highlight_color = eraser_color;
    }
    float3 halftimber_color;
    if (int((in_var_C13 & 1024) != 0) == 1)
    {
        halftimber_color = halftimber_colors.Sample(sampler_llr, float3(in_var_C10 * 1.0f, float(int(halftimber_color_ids.Load<uint>(uint(nani_instance_data.Load<int>(in_var_C14 * 112 + 52)) * 4 + 0))))).xyz;
    }
    else
    {
        halftimber_color = highlight_color;
    }
    float3 final_albedo;
    if (frame_constants_glade_theme_id == 4u)
    {
        float2 simplex_pos = in_var_C2.xz * 0.4000000059604644775390625f;
        float2 simplex_cell = floor(simplex_pos + dot(simplex_pos, 0.3660254180431365966796875f.xx).xx);
        float2 simplex_frac = (simplex_pos - simplex_cell) + dot(simplex_cell, 0.211324870586395263671875f.xx).xx;
        float frac_x = simplex_frac.x;
        float frac_y = simplex_frac.y;
        bool2 ge = (frac_x > frac_y).xx;
        float2 i1 = float2(ge.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, ge.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
        float4 x1x2 = simplex_frac.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
        float2 x1 = x1x2.xy - i1;
        float4 x1x2b = float4(x1.x, x1.y, x1x2.z, x1x2.w);
        float2 cell_mod = fmod(simplex_cell, 289.0f.xx);
        float3 p0 = cell_mod.y.xxx + float3(0.0f, i1.y, 1.0f);
        float3 p1 = (fmod(((p0 * 34.0f) + 1.0f.xxx) * p0, 289.0f.xxx) + cell_mod.x.xxx) + float3(0.0f, i1.x, 1.0f);
        float2 x1b = x1.xy;
        float2 x2 = x1x2.zw;
        float3 m = max(0.5f.xxx - float3(dot(simplex_frac, simplex_frac), dot(x1b, x1b), dot(x2, x2)), 0.0f.xxx);
        float3 m2 = m * m;
        float3 n2 = frac(fmod(((p1 * 34.0f) + 1.0f.xxx) * p1, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
        float3 n = n2 - 1.0f.xxx;
        float3 s = abs(n) - 0.5f.xxx;
        float3 t = n - floor(n2 + (-0.5f).xxx);
        float3 grad_x;
        grad_x.x = (t.x * frac_x) + (s.x * frac_y);
        float2 gradyz = (t.yz * x1x2b.xz) + (s.yz * x1x2b.yw);
        final_albedo = lerp(halftimber_color, 0.699999988079071044921875f.xxx, (smoothstep(0.0f, 1.0f, smoothstep(-0.100000001490116119384765625f, 0.0f, in_var_C2.y) * clamp(world_normal.y, 0.0f, 1.0f)) * clamp(clamp((130.0f * dot((m2 * m2) * (1.792842864990234375f.xxx - (((t * t) + (s * s)) * 0.8537347316741943359375f)), float3(grad_x.x, gradyz.x, gradyz.y))) + 0.75f, 0.0f, 1.0f), is_rough ? 0.0f : 0.4000000059604644775390625f, is_rough ? 0.100000001490116119384765625f : 1.0f)).xxx);
    }
    else
    {
        final_albedo = halftimber_color;
    }
    float3 packed_normal = (world_normal * 0.5f) + 0.5f.xxx;
    float3 srgb_albedo = sqrt(clamp(final_albedo, 0.0f.xxx, 1.0f.xxx));
    uint albedo_word = (uint((clamp(srgb_albedo.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(srgb_albedo.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(srgb_albedo.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 gbuffer = uint2(((uint((clamp(packed_normal.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(packed_normal.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(packed_normal.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u, albedo_word);
    gbuffer.y = albedo_word | (((((7u | (uint(is_highlight ? true : is_eraser) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = gbuffer;
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
    uint current_view_index : packoffset(c0);
    uint2 terrain_heightmap_dims : packoffset(c0.y);
    uint2 visual_terrain_heightmap_dims : packoffset(c1);
    uint highlight_mode : packoffset(c1.z);
    int highlight_bricks_to_delete : packoffset(c1.w);
    float3 brush_position : packoffset(c2);
    uint bool_render_shadows : packoffset(c2.w);
    float eraser_mult : packoffset(c3);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
    [[vk::location(2)]] float3 in_var_Vertex_Normal: Vertex_Normal;
    [[vk::location(3)]] float2 in_var_Vertex_UV: Vertex_UV;
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
    Affine3Packed xform_packed;
    [unroll]
    for (int i = 0; i < 12; i++)
    {
        xform_packed.data[i] = nani_instance_data.Load<float>(i * 4 + instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 0);
    }
    float2 bone1_normal = float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 92) >> 16u) & 65535u).x);
    float3 bent_pos;
    float3 bone_pos;
    if (in_var_bbx_x < 0.5f)
    {
        float bbx_x2 = in_var_bbx_x * 2.0f;
        float bbx_x2c = clamp(bbx_x2, 0.0f, 1.0f);
        float3 bone_pos01 = float3(lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).x, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).x, bbx_x2), in_var_Vertex_Position.y + lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).y, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).y, bbx_x2c), lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 64).z, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).z, bbx_x2));
        float2 bone_pos01_xz = bone_pos01.xz + (normalize(lerp(float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 76) >> 16u) & 65535u).x), bone1_normal, bbx_x2c.xx)) * in_var_Vertex_Position.z);
        bent_pos = float3(bone_pos01_xz.x, bone_pos01.y, bone_pos01_xz.y);
        bone_pos = bone_pos01;
    }
    else
    {
        float bbx_x2b = (in_var_bbx_x - 0.5f) * 2.0f;
        float bbx_x2bc = clamp(bbx_x2b, 0.0f, 1.0f);
        float3 bone_pos12 = float3(lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).x, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).x, bbx_x2b), in_var_Vertex_Position.y + lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).y, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).y, bbx_x2bc), lerp(nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 80).z, nani_instance_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 96).z, bbx_x2b));
        float2 bone_pos12_xz = bone_pos12.xz + (normalize(lerp(bone1_normal, float2(spvUnpackHalf2x16(nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108) & 65535u).x, spvUnpackHalf2x16((nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 108) >> 16u) & 65535u).x), bbx_x2bc.xx)) * in_var_Vertex_Position.z);
        bent_pos = float3(bone_pos12_xz.x, bone_pos12.y, bone_pos12_xz.y);
        bone_pos = bone_pos12;
    }
    float3 scaled_pos = bent_pos * 0.4000000059604644775390625f;
    int3 lattice_cell = int3(floor(scaled_pos));
    float3 lattice_frac = frac(scaled_pos);
    uint3 c000_cell = uint3(lattice_cell);
    uint c000_z = c000_cell.z;
    uint c000_z10 = c000_z + (c000_z << 10u);
    uint c000_z6 = c000_z10 ^ (c000_z10 >> 6u);
    uint c000_z3 = c000_z6 + (c000_z6 << 3u);
    uint c000_z11 = c000_z3 ^ (c000_z3 >> 11u);
    uint c000_mix = (((c000_cell.y * 1664525u) + (c000_z11 + (c000_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c000_x11 = c000_mix ^ (c000_mix >> 11u);
    uint c000_x7 = c000_x11 ^ ((c000_x11 << 7u) & 2636928640u);
    uint c000_x15 = c000_x7 ^ ((c000_x7 << 15u) & 4022730752u);
    uint c000_mix2 = (((c000_cell.x * 1664525u) + (c000_x15 ^ (c000_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c000_x112 = c000_mix2 ^ (c000_mix2 >> 11u);
    uint c000_x72 = c000_x112 ^ ((c000_x112 << 7u) & 2636928640u);
    uint c000_x152 = c000_x72 ^ ((c000_x72 << 15u) & 4022730752u);
    uint c000_rand = c000_x152 ^ (c000_x152 >> 18u);
    uint c000_h10 = c000_rand + (c000_rand << 10u);
    uint c000_h6 = c000_h10 ^ (c000_h10 >> 6u);
    uint c000_h3 = c000_h6 + (c000_h6 << 3u);
    uint c000_h11 = c000_h3 ^ (c000_h3 >> 11u);
    uint3 c100_cell = uint3(lattice_cell + int3(1, 0, 0));
    uint c100_z = c100_cell.z;
    uint c100_z10 = c100_z + (c100_z << 10u);
    uint c100_z6 = c100_z10 ^ (c100_z10 >> 6u);
    uint c100_z3 = c100_z6 + (c100_z6 << 3u);
    uint c100_z11 = c100_z3 ^ (c100_z3 >> 11u);
    uint c100_mix = (((c100_cell.y * 1664525u) + (c100_z11 + (c100_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c100_x11 = c100_mix ^ (c100_mix >> 11u);
    uint c100_x7 = c100_x11 ^ ((c100_x11 << 7u) & 2636928640u);
    uint c100_x15 = c100_x7 ^ ((c100_x7 << 15u) & 4022730752u);
    uint c100_mix2 = (((c100_cell.x * 1664525u) + (c100_x15 ^ (c100_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c100_x112 = c100_mix2 ^ (c100_mix2 >> 11u);
    uint c100_x72 = c100_x112 ^ ((c100_x112 << 7u) & 2636928640u);
    uint c100_x152 = c100_x72 ^ ((c100_x72 << 15u) & 4022730752u);
    uint c100_rand = c100_x152 ^ (c100_x152 >> 18u);
    uint c100_h10 = c100_rand + (c100_rand << 10u);
    uint c100_h6 = c100_h10 ^ (c100_h10 >> 6u);
    uint c100_h3 = c100_h6 + (c100_h6 << 3u);
    uint c100_h11 = c100_h3 ^ (c100_h3 >> 11u);
    uint3 c010_cell = uint3(lattice_cell + int3(0, 1, 0));
    uint c010_z = c010_cell.z;
    uint c010_z10 = c010_z + (c010_z << 10u);
    uint c010_z6 = c010_z10 ^ (c010_z10 >> 6u);
    uint c010_z3 = c010_z6 + (c010_z6 << 3u);
    uint c010_z11 = c010_z3 ^ (c010_z3 >> 11u);
    uint c010_mix = (((c010_cell.y * 1664525u) + (c010_z11 + (c010_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c010_x11 = c010_mix ^ (c010_mix >> 11u);
    uint c010_x7 = c010_x11 ^ ((c010_x11 << 7u) & 2636928640u);
    uint c010_x15 = c010_x7 ^ ((c010_x7 << 15u) & 4022730752u);
    uint c010_mix2 = (((c010_cell.x * 1664525u) + (c010_x15 ^ (c010_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c010_x112 = c010_mix2 ^ (c010_mix2 >> 11u);
    uint c010_x72 = c010_x112 ^ ((c010_x112 << 7u) & 2636928640u);
    uint c010_x152 = c010_x72 ^ ((c010_x72 << 15u) & 4022730752u);
    uint c010_rand = c010_x152 ^ (c010_x152 >> 18u);
    uint c010_h10 = c010_rand + (c010_rand << 10u);
    uint c010_h6 = c010_h10 ^ (c010_h10 >> 6u);
    uint c010_h3 = c010_h6 + (c010_h6 << 3u);
    uint c010_h11 = c010_h3 ^ (c010_h3 >> 11u);
    uint3 c110_cell = uint3(lattice_cell + int3(1, 1, 0));
    uint c110_z = c110_cell.z;
    uint c110_z10 = c110_z + (c110_z << 10u);
    uint c110_z6 = c110_z10 ^ (c110_z10 >> 6u);
    uint c110_z3 = c110_z6 + (c110_z6 << 3u);
    uint c110_z11 = c110_z3 ^ (c110_z3 >> 11u);
    uint c110_mix = (((c110_cell.y * 1664525u) + (c110_z11 + (c110_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c110_x11 = c110_mix ^ (c110_mix >> 11u);
    uint c110_x7 = c110_x11 ^ ((c110_x11 << 7u) & 2636928640u);
    uint c110_x15 = c110_x7 ^ ((c110_x7 << 15u) & 4022730752u);
    uint c110_mix2 = (((c110_cell.x * 1664525u) + (c110_x15 ^ (c110_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c110_x112 = c110_mix2 ^ (c110_mix2 >> 11u);
    uint c110_x72 = c110_x112 ^ ((c110_x112 << 7u) & 2636928640u);
    uint c110_x152 = c110_x72 ^ ((c110_x72 << 15u) & 4022730752u);
    uint c110_rand = c110_x152 ^ (c110_x152 >> 18u);
    uint c110_h10 = c110_rand + (c110_rand << 10u);
    uint c110_h6 = c110_h10 ^ (c110_h10 >> 6u);
    uint c110_h3 = c110_h6 + (c110_h6 << 3u);
    uint c110_h11 = c110_h3 ^ (c110_h3 >> 11u);
    uint3 c001_cell = uint3(lattice_cell + int3(0, 0, 1));
    uint c001_z = c001_cell.z;
    uint c001_z10 = c001_z + (c001_z << 10u);
    uint c001_z6 = c001_z10 ^ (c001_z10 >> 6u);
    uint c001_z3 = c001_z6 + (c001_z6 << 3u);
    uint c001_z11 = c001_z3 ^ (c001_z3 >> 11u);
    uint c001_mix = (((c001_cell.y * 1664525u) + (c001_z11 + (c001_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c001_x11 = c001_mix ^ (c001_mix >> 11u);
    uint c001_x7 = c001_x11 ^ ((c001_x11 << 7u) & 2636928640u);
    uint c001_x15 = c001_x7 ^ ((c001_x7 << 15u) & 4022730752u);
    uint c001_mix2 = (((c001_cell.x * 1664525u) + (c001_x15 ^ (c001_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c001_x112 = c001_mix2 ^ (c001_mix2 >> 11u);
    uint c001_x72 = c001_x112 ^ ((c001_x112 << 7u) & 2636928640u);
    uint c001_x152 = c001_x72 ^ ((c001_x72 << 15u) & 4022730752u);
    uint c001_rand = c001_x152 ^ (c001_x152 >> 18u);
    uint c001_h10 = c001_rand + (c001_rand << 10u);
    uint c001_h6 = c001_h10 ^ (c001_h10 >> 6u);
    uint c001_h3 = c001_h6 + (c001_h6 << 3u);
    uint c001_h11 = c001_h3 ^ (c001_h3 >> 11u);
    uint3 c101_cell = uint3(lattice_cell + int3(1, 0, 1));
    uint c101_z = c101_cell.z;
    uint c101_z10 = c101_z + (c101_z << 10u);
    uint c101_z6 = c101_z10 ^ (c101_z10 >> 6u);
    uint c101_z3 = c101_z6 + (c101_z6 << 3u);
    uint c101_z11 = c101_z3 ^ (c101_z3 >> 11u);
    uint c101_mix = (((c101_cell.y * 1664525u) + (c101_z11 + (c101_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c101_x11 = c101_mix ^ (c101_mix >> 11u);
    uint c101_x7 = c101_x11 ^ ((c101_x11 << 7u) & 2636928640u);
    uint c101_x15 = c101_x7 ^ ((c101_x7 << 15u) & 4022730752u);
    uint c101_mix2 = (((c101_cell.x * 1664525u) + (c101_x15 ^ (c101_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c101_x112 = c101_mix2 ^ (c101_mix2 >> 11u);
    uint c101_x72 = c101_x112 ^ ((c101_x112 << 7u) & 2636928640u);
    uint c101_x152 = c101_x72 ^ ((c101_x72 << 15u) & 4022730752u);
    uint c101_rand = c101_x152 ^ (c101_x152 >> 18u);
    uint c101_h10 = c101_rand + (c101_rand << 10u);
    uint c101_h6 = c101_h10 ^ (c101_h10 >> 6u);
    uint c101_h3 = c101_h6 + (c101_h6 << 3u);
    uint c101_h11 = c101_h3 ^ (c101_h3 >> 11u);
    uint3 c011_cell = uint3(lattice_cell + int3(0, 1, 1));
    uint c011_z = c011_cell.z;
    uint c011_z10 = c011_z + (c011_z << 10u);
    uint c011_z6 = c011_z10 ^ (c011_z10 >> 6u);
    uint c011_z3 = c011_z6 + (c011_z6 << 3u);
    uint c011_z11 = c011_z3 ^ (c011_z3 >> 11u);
    uint c011_mix = (((c011_cell.y * 1664525u) + (c011_z11 + (c011_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c011_x11 = c011_mix ^ (c011_mix >> 11u);
    uint c011_x7 = c011_x11 ^ ((c011_x11 << 7u) & 2636928640u);
    uint c011_x15 = c011_x7 ^ ((c011_x7 << 15u) & 4022730752u);
    uint c011_mix2 = (((c011_cell.x * 1664525u) + (c011_x15 ^ (c011_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c011_x112 = c011_mix2 ^ (c011_mix2 >> 11u);
    uint c011_x72 = c011_x112 ^ ((c011_x112 << 7u) & 2636928640u);
    uint c011_x152 = c011_x72 ^ ((c011_x72 << 15u) & 4022730752u);
    uint c011_rand = c011_x152 ^ (c011_x152 >> 18u);
    uint c011_h10 = c011_rand + (c011_rand << 10u);
    uint c011_h6 = c011_h10 ^ (c011_h10 >> 6u);
    uint c011_h3 = c011_h6 + (c011_h6 << 3u);
    uint c011_h11 = c011_h3 ^ (c011_h3 >> 11u);
    uint3 c111_cell = uint3(lattice_cell + int3(1, 1, 1));
    uint c111_z = c111_cell.z;
    uint c111_z10 = c111_z + (c111_z << 10u);
    uint c111_z6 = c111_z10 ^ (c111_z10 >> 6u);
    uint c111_z3 = c111_z6 + (c111_z6 << 3u);
    uint c111_z11 = c111_z3 ^ (c111_z3 >> 11u);
    uint c111_mix = (((c111_cell.y * 1664525u) + (c111_z11 + (c111_z11 << 15u))) + 1013904223u) * 1664525u;
    uint c111_x11 = c111_mix ^ (c111_mix >> 11u);
    uint c111_x7 = c111_x11 ^ ((c111_x11 << 7u) & 2636928640u);
    uint c111_x15 = c111_x7 ^ ((c111_x7 << 15u) & 4022730752u);
    uint c111_mix2 = (((c111_cell.x * 1664525u) + (c111_x15 ^ (c111_x15 >> 18u))) + 1013904223u) * 1664525u;
    uint c111_x112 = c111_mix2 ^ (c111_mix2 >> 11u);
    uint c111_x72 = c111_x112 ^ ((c111_x112 << 7u) & 2636928640u);
    uint c111_x152 = c111_x72 ^ ((c111_x72 << 15u) & 4022730752u);
    uint c111_rand = c111_x152 ^ (c111_x152 >> 18u);
    uint c111_h10 = c111_rand + (c111_rand << 10u);
    uint c111_h6 = c111_h10 ^ (c111_h10 >> 6u);
    uint c111_h3 = c111_h6 + (c111_h6 << 3u);
    uint c111_h11 = c111_h3 ^ (c111_h3 >> 11u);
    float3 smooth = (lattice_frac * lattice_frac) * (3.0f.xxx - (lattice_frac * 2.0f));
    float sx = smooth.x;
    float sx_inv = 1.0f - sx;
    float sy = smooth.y;
    float sy_inv = 1.0f - sy;
    float sz = smooth.z;
    float sz_inv = 1.0f - sz;
    float2 noise_xy = ((((((float2(asfloat((c000_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c000_h11 + (c000_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy_inv) * sz_inv) + (((float2(asfloat((c100_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c100_h11 + (c100_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy_inv) * sz_inv)) + (((float2(asfloat((c010_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c010_h11 + (c010_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy) * sz_inv)) + (((float2(asfloat((c110_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c110_h11 + (c110_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy) * sz_inv)) + (((float2(asfloat((c001_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c001_h11 + (c001_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy_inv) * sz);
    float2 jitter = clamp(((((noise_xy + (((float2(asfloat((c101_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c101_h11 + (c101_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy_inv) * sz)) + (((float2(asfloat((c011_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c011_h11 + (c011_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx_inv) * sy) * sz)) + (((float2(asfloat((c111_rand & 8388607u) | 1065353216u) - 1.0f, asfloat(((c111_h11 + (c111_h11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) * sx) * sy) * sz)) - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced = bent_pos + (float3(jitter.x, 0.0f, jitter.y) * 0.0599999986588954925537109375f);
    int flags = (in_var_is_bevel != 0) ? 768 : 512;
    bool is_wooden = nani_instance_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 60) != 0u;
    int flags_out;
    if (is_wooden)
    {
        flags_out = flags | 1024;
    }
    else
    {
        flags_out = flags;
    }
    bool2 is_wooden2 = is_wooden.xx;
    gl_Position = mul(frame_constants_view_constants[current_view_index].world_to_sample, float4(displaced, 1.0f));
    out_var_C0 = uint(nani_instance_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 112 + 48) + in_var_brick_id);
    out_var_C1 = mul(transpose(float4x3(float3(xform_packed.data[0], xform_packed.data[1], xform_packed.data[2]), float3(xform_packed.data[3], xform_packed.data[4], xform_packed.data[5]), float3(xform_packed.data[6], xform_packed.data[7], xform_packed.data[8]), float3(xform_packed.data[9], xform_packed.data[10], xform_packed.data[11]))), float4(in_var_Vertex_Normal, 0.0f));
    out_var_C2 = displaced;
    out_var_C5 = 0.0f.xxx;
    out_var_C7 = 0.0f;
    out_var_C10 = float2(is_wooden2.x ? in_var_Vertex_UV.x : 0.0f.xx.x, is_wooden2.y ? in_var_Vertex_UV.y : 0.0f.xx.y);
    out_var_C11 = 0;
    out_var_C12 = bone_pos.xz;
    out_var_C13 = flags_out;
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
    bool is_edge = (in_var_C13 & 8) != 0;
    bool below_terrain;
    if (!is_edge)
    {
        below_terrain = in_var_C7 > (-2.5f);
    }
    else
    {
        below_terrain = false;
    }
    bool is_solid;
    if (below_terrain)
    {
        is_solid = in_var_C2.y < in_var_C7;
    }
    else
    {
        is_solid = false;
    }
    if (is_solid)
    {
        discard;
    }
    else
    {
        bool above_terrain;
        if (is_edge)
        {
            above_terrain = in_var_C2.y > in_var_C7;
        }
        else
        {
            above_terrain = false;
        }
        if (above_terrain)
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
