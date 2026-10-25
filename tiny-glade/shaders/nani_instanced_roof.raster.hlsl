// PATCHED: modules lsl_b903e43ffb3da915.bin:ps_main, lsl_b903e43ffb3da915.bin:vs_main, lsl_f0adff76827e73ff.bin:ps_main, lsl_f0adff76827e73ff.bin:vs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_b903e43ffb3da915) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_VERTEX)) || (defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/nani_instanced_roof.raster.hlsl | B nani raster | bins: _nani_instanced_roof.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// Nani 实例化屋顶:VS 从 instanced_data 做逐瓦片变换(尺寸/旋转四元数/位移);
//   PS 用苔藓、破损与法线贴图给屋顶瓦调色板着色并打包 gbuffer。

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
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
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

static uint mask_uint;
static float3 mask_vec;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 moss_texture_dims : packoffset(c0.y);
    uint2 roof_tile_normal_dims : packoffset(c1);
    uint2 roof_tile_damage_dims : packoffset(c1.z);
    uint highlighted_wall_id : packoffset(c2);
    uint highlight_mode : packoffset(c2.y);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
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
    float2 quat_p0 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x & 65535u);
    float quat_x = quat_p0.x;
    float2 quat_p1 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x >> 16u) & 65535u);
    float quat_y = quat_p1.x;
    float2 quat_p2 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y & 65535u);
    float quat_z = quat_p2.x;
    float2 quat_p3 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y >> 16u) & 65535u);
    float quat_w = quat_p3.x;
    float q_xx = quat_x + quat_x;
    float q_xy = quat_y + quat_y;
    float q_xz = quat_z + quat_z;
    float q_xw = quat_x * q_xx;
    float q_yx = quat_x * q_xy;
    float q_yz = quat_x * q_xz;
    float q_yw = quat_y * q_xy;
    float q_zw = quat_y * q_xz;
    float q_wx = quat_z * q_xz;
    float q_wy = quat_w * q_xx;
    float q_wz = quat_w * q_xy;
    float q_ww = quat_w * q_xz;
    float3x4 xform3x4 = transpose(float4x3(float3(1.0f - (q_yw + q_wx), q_yx + q_ww, q_yz - q_wz), float3(q_yx - q_ww, 1.0f - (q_xw + q_wx), q_zw + q_wy), float3(q_yz + q_wz, q_zw - q_wy, 1.0f - (q_xw + q_yw)), instanced_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0)));
    float2 scale_p = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y >> 16u) & 65535u);
    float scale2 = scale_p.x;
    float4 v295 = float4(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y & 65535u).x, scale2);
    float2 grow_p = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y >> 16u) & 65535u);
    float grow_f = grow_p.x;
    bool is_flagged = instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) != 0;
    float3 shingle_size = (float3(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y & 65535u).x) * smoothstep(grow_f * 0.800000011920928955078125f, grow_f, pow(instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0.800000011920928955078125f))) * float3(0.505970001220703125f, 0.5f, 0.5f);
    float size_x = shingle_size.x * 0.949999988079071044921875f;
    float3 size3 = shingle_size;
    size3.x = size_x;
    float3 size_scaled = size3 * float3(3.58920383453369140625f, 3.70370388031005859375f, 3.70370388031005859375f);
    float size_min = max(min(max(size_x, max(shingle_size.y, shingle_size.z)), min(size_scaled.x, min(size_scaled.y, size_scaled.z))), 9.9999997473787516355514526367188e-06f);
    float3 scaled_vertex = in_var_Vertex_Position * float3(1.976401805877685546875f, 2.0f, 2.0f);
    float3 vertex_scaled = scaled_vertex * size_min;
    bool3 flip_dir = bool3(vertex_scaled.x >= 0.0f.xxx.x, vertex_scaled.y >= 0.0f.xxx.y, vertex_scaled.z >= 0.0f.xxx.z);
    float2 rot_p = spvUnpackHalf2x16((instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) >> 16u) & 65535u);
    float rot_angle = rot_p.x;
    float3 axis = normalize(float3(1.0f, 0.0f, 0.0f));
    float sin_a = sin(rot_angle);
    float cos_a = cos(rot_angle);
    float one_minus_cos = 1.0f - cos_a;
    float axis_x = axis.x;
    float oc_ax = one_minus_cos * axis_x;
    float axis_y = axis.y;
    float oc_axy = oc_ax * axis_y;
    float axis_z = axis.z;
    float az_sin = axis_z * sin_a;
    float oc_az = one_minus_cos * axis_z;
    float oc_azx = oc_az * axis_x;
    float ay_sin = axis_y * sin_a;
    float oc_ay = one_minus_cos * axis_y;
    float oc_ayz = oc_ay * axis_z;
    float ax_sin = axis_x * sin_a;
    float3 rotated_vertex = mul(transpose(float3x3(float3((oc_ax * axis_x) + cos_a, oc_axy - az_sin, oc_azx + ay_sin), float3(oc_axy + az_sin, (oc_ay * axis_y) + cos_a, oc_ayz - ax_sin), float3(oc_azx - ay_sin, oc_ayz + ax_sin, (oc_az * axis_z) + cos_a))), vertex_scaled - ((size_min.xxx - size3) * float3(int3(flip_dir.x ? int3(1, 1, 1).x : int3(-1, -1, -1).x, flip_dir.y ? int3(1, 1, 1).y : int3(-1, -1, -1).y, flip_dir.z ? int3(1, 1, 1).z : int3(-1, -1, -1).z))));
    float3 displaced;
    if (is_flagged)
    {
        uint seed_damage = (uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u) + instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 40);
        uint sd10 = seed_damage + (seed_damage << 10u);
        uint sd6 = sd10 ^ (sd10 >> 6u);
        uint sd3 = sd6 + (sd6 << 3u);
        uint sd11 = sd3 ^ (sd3 >> 11u);
        float damage_amount = ((asfloat(((sd11 + (sd11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 0.0199999995529651641845703125f : 0.0599999986588954925537109375f;
        float3 disp_cur;
        if (in_var_Vertex_Position.y > 0.100000001490116119384765625f)
        {
            float3 disp_sel;
            if (instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) != 0)
            {
                float3 disp_low = rotated_vertex;
                disp_low.z = rotated_vertex.z - damage_amount;
                disp_sel = disp_low;
            }
            else
            {
                float3 disp_high = rotated_vertex;
                disp_high.z = rotated_vertex.z + damage_amount;
                disp_sel = disp_high;
            }
            disp_cur = disp_sel;
        }
        else
        {
            disp_cur = rotated_vertex;
        }
        displaced = disp_cur;
    }
    else
    {
        displaced = rotated_vertex;
    }
    uint mask_a = is_flagged ? 1065353216u : mask_uint;
    uint mask_b = is_flagged ? 8388607u : mask_uint;
    bool mask_flag = is_flagged ? true : false;
    float3 world_pos = mul(xform3x4, float4(displaced.xy, displaced.z + ((1.0f - smoothstep(0.0f, 0.800000011920928955078125f, mul(xform3x4, float4(displaced, 1.0f)).y)) * 0.115000002086162567138671875f), 1.0f));
    float world_y = world_pos.y;
    float roof_y = world_y + instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12);
    float3 raised_pos = world_pos;
    raised_pos.y = roof_y;
    float3 disp_base;
    uint mask_c;
    uint mask_d;
    bool flag_c;
    if (is_flagged)
    {
        float2 ncell_pos = raised_pos.xz * 0.5f;
        int2 ncell = int2(floor(ncell_pos));
        float2 nfrac = frac(ncell_pos);
        uint2 ncell_u = uint2(ncell);
        uint ny = ncell_u.y;
        uint ny10 = ny + (ny << 10u);
        uint ny6 = ny10 ^ (ny10 >> 6u);
        uint ny3 = ny6 + (ny6 << 3u);
        uint ny11 = ny3 ^ (ny3 >> 11u);
        uint nmix = (((ncell_u.x * 1664525u) + (ny11 + (ny11 << 15u))) + 1013904223u) * 1664525u;
        uint nx11 = nmix ^ (nmix >> 11u);
        uint nx7 = nx11 ^ ((nx11 << 7u) & 2636928640u);
        uint nx15 = nx7 ^ ((nx7 << 15u) & 4022730752u);
        bool sel1 = mask_flag ? mask_flag : true;
        uint mul1 = mask_flag ? mask_b : 8388607u;
        uint add1 = mask_flag ? mask_a : 1065353216u;
        uint2 ncell10 = uint2(ncell + int2(1, 0));
        uint ny2 = ncell10.y;
        uint ny102 = ny2 + (ny2 << 10u);
        uint ny62 = ny102 ^ (ny102 >> 6u);
        uint ny32 = ny62 + (ny62 << 3u);
        uint ny112 = ny32 ^ (ny32 >> 11u);
        uint nmix2 = (((ncell10.x * 1664525u) + (ny112 + (ny112 << 15u))) + 1013904223u) * 1664525u;
        uint nx112 = nmix2 ^ (nmix2 >> 11u);
        uint nx72 = nx112 ^ ((nx112 << 7u) & 2636928640u);
        uint nx152 = nx72 ^ ((nx72 << 15u) & 4022730752u);
        bool sel2 = sel1 ? sel1 : true;
        uint mul2 = sel1 ? mul1 : 8388607u;
        uint add2 = sel1 ? add1 : 1065353216u;
        uint2 ncell01 = uint2(ncell + int2(0, 1));
        uint ny3b = ncell01.y;
        uint ny103 = ny3b + (ny3b << 10u);
        uint ny63 = ny103 ^ (ny103 >> 6u);
        uint ny33 = ny63 + (ny63 << 3u);
        uint ny113 = ny33 ^ (ny33 >> 11u);
        uint nmix3 = (((ncell01.x * 1664525u) + (ny113 + (ny113 << 15u))) + 1013904223u) * 1664525u;
        uint nx113 = nmix3 ^ (nmix3 >> 11u);
        uint nx73 = nx113 ^ ((nx113 << 7u) & 2636928640u);
        uint nx153 = nx73 ^ ((nx73 << 15u) & 4022730752u);
        bool sel3 = sel2 ? sel2 : true;
        uint mul3 = sel2 ? mul2 : 8388607u;
        uint add3 = sel2 ? add2 : 1065353216u;
        uint2 ncell11 = uint2(ncell + int2(1, 1));
        uint ny4 = ncell11.y;
        uint ny104 = ny4 + (ny4 << 10u);
        uint ny64 = ny104 ^ (ny104 >> 6u);
        uint v562 = ny64 + (ny64 << 3u);
        uint v564 = v562 ^ (v562 >> 11u);
        uint v570 = (((ncell11.x * 1664525u) + (v564 + (v564 << 15u))) + 1013904223u) * 1664525u;
        uint v572 = v570 ^ (v570 >> 11u);
        uint v575 = v572 ^ ((v572 << 7u) & 2636928640u);
        uint v578 = v575 ^ ((v575 << 15u) & 4022730752u);
        uint v582 = sel3 ? mul3 : 8388607u;
        uint v583 = sel3 ? add3 : 1065353216u;
        float2 v591 = (nfrac * nfrac) * (3.0f.xx - (nfrac * 2.0f));
        float v592 = v591.x;
        float v593 = 1.0f - v592;
        float v595 = v591.y;
        float3 v609 = raised_pos;
        v609.y = roof_y + ((((((1.0f - v595) * (((asfloat(((nx15 ^ (nx15 >> 18u)) & mul1) | add1) - 1.0f) * v593) + ((asfloat(((nx152 ^ (nx152 >> 18u)) & mul2) | add2) - 1.0f) * v592))) + (((asfloat(((nx153 ^ (nx153 >> 18u)) & mul3) | add3) - 1.0f) * v593) * v595)) + (((asfloat(((v578 ^ (v578 >> 18u)) & v582) | v583) - 1.0f) * v592) * v595)) * 0.300000011920928955078125f) * smoothstep(0.0f, 1.0f, world_y));
        disp_base = v609;
        mask_c = v583;
        mask_d = v582;
        flag_c = sel3 ? sel3 : true;
    }
    else
    {
        disp_base = raised_pos;
        mask_c = mask_a;
        mask_d = mask_b;
        flag_c = mask_flag;
    }
    float3 v616 = disp_base * 0.4000000059604644775390625f;
    int3 ncell3 = int3(floor(v616));
    float3 nfrac3 = frac(v616);
    uint3 v620 = uint3(ncell3);
    uint v623 = v620.z;
    uint v625 = v623 + (v623 << 10u);
    uint v627 = v625 ^ (v625 >> 6u);
    uint v629 = v627 + (v627 << 3u);
    uint v631 = v629 ^ (v629 >> 11u);
    bool v634 = mask_flag ? mask_flag : true;
    uint v635 = mask_flag ? (is_flagged ? 1664525u : mask_uint) : 1664525u;
    uint v636 = mask_flag ? (is_flagged ? 1013904223u : mask_uint) : 1013904223u;
    uint v640 = (((v620.y * v635) + (v631 + (v631 << 15u))) + v636) * v635;
    uint v642 = v640 ^ (v640 >> 11u);
    uint v645 = v642 ^ ((v642 << 7u) & 2636928640u);
    uint v648 = v645 ^ ((v645 << 15u) & 4022730752u);
    bool v651 = v634 ? v634 : true;
    uint v652 = v634 ? v635 : 1664525u;
    uint v653 = v634 ? v636 : 1013904223u;
    uint v657 = (((v620.x * v652) + (v648 ^ (v648 >> 18u))) + v653) * v652;
    uint v659 = v657 ^ (v657 >> 11u);
    uint v662 = v659 ^ ((v659 << 7u) & 2636928640u);
    uint v665 = v662 ^ ((v662 << 15u) & 4022730752u);
    uint v667 = v665 ^ (v665 >> 18u);
    uint v669 = v667 + (v667 << 10u);
    uint v671 = v669 ^ (v669 >> 6u);
    uint v673 = v671 + (v671 << 3u);
    uint v675 = v673 ^ (v673 >> 11u);
    bool v678 = flag_c ? flag_c : true;
    uint v679 = flag_c ? mask_d : 8388607u;
    uint v680 = flag_c ? mask_c : 1065353216u;
    bool v685 = v678 ? v678 : true;
    uint v686 = v678 ? v679 : 8388607u;
    uint v687 = v678 ? v680 : 1065353216u;
    uint3 h3_100_cell = uint3(ncell3 + int3(1, 0, 0));
    uint h3_100_s0 = h3_100_cell.z;
    uint h3_100_s1 = h3_100_s0 + (h3_100_s0 << 10u);
    uint h3_100_s2 = h3_100_s1 ^ (h3_100_s1 >> 6u);
    uint h3_100_s3 = h3_100_s2 + (h3_100_s2 << 3u);
    uint h3_100_s4 = h3_100_s3 ^ (h3_100_s3 >> 11u);
    bool h3_100_s5 = v651 ? v651 : true;
    uint h3_100_s6 = v651 ? v652 : 1664525u;
    uint h3_100_s7 = v651 ? v653 : 1013904223u;
    uint h3_100_s8 = (((h3_100_cell.y * h3_100_s6) + (h3_100_s4 + (h3_100_s4 << 15u))) + h3_100_s7) * h3_100_s6;
    uint h3_100_s9 = h3_100_s8 ^ (h3_100_s8 >> 11u);
    uint h3_100_s10 = h3_100_s9 ^ ((h3_100_s9 << 7u) & 2636928640u);
    uint h3_100_s11 = h3_100_s10 ^ ((h3_100_s10 << 15u) & 4022730752u);
    bool h3_100_s12 = h3_100_s5 ? h3_100_s5 : true;
    uint h3_100_s13 = h3_100_s5 ? h3_100_s6 : 1664525u;
    uint h3_100_s14 = h3_100_s5 ? h3_100_s7 : 1013904223u;
    uint h3_100_s15 = (((h3_100_cell.x * h3_100_s13) + (h3_100_s11 ^ (h3_100_s11 >> 18u))) + h3_100_s14) * h3_100_s13;
    uint h3_100_s16 = h3_100_s15 ^ (h3_100_s15 >> 11u);
    uint h3_100_s17 = h3_100_s16 ^ ((h3_100_s16 << 7u) & 2636928640u);
    uint h3_100_s18 = h3_100_s17 ^ ((h3_100_s17 << 15u) & 4022730752u);
    uint h3_100_s19 = h3_100_s18 ^ (h3_100_s18 >> 18u);
    uint h3_100_s20 = h3_100_s19 + (h3_100_s19 << 10u);
    uint h3_100_s21 = h3_100_s20 ^ (h3_100_s20 >> 6u);
    uint h3_100_s22 = h3_100_s21 + (h3_100_s21 << 3u);
    uint h3_100_s23 = h3_100_s22 ^ (h3_100_s22 >> 11u);
    bool h3_100_s24 = v685 ? v685 : true;
    uint h3_100_s25 = v685 ? v686 : 8388607u;
    uint h3_100_s26 = v685 ? v687 : 1065353216u;
    bool h3_100_s27 = h3_100_s24 ? h3_100_s24 : true;
    uint h3_100_s28 = h3_100_s24 ? h3_100_s25 : 8388607u;
    uint h3_100_s29 = h3_100_s24 ? h3_100_s26 : 1065353216u;
    uint3 h3_010_cell = uint3(ncell3 + int3(0, 1, 0));
    uint h3_010_s0 = h3_010_cell.z;
    uint h3_010_s1 = h3_010_s0 + (h3_010_s0 << 10u);
    uint h3_010_s2 = h3_010_s1 ^ (h3_010_s1 >> 6u);
    uint h3_010_s3 = h3_010_s2 + (h3_010_s2 << 3u);
    uint h3_010_s4 = h3_010_s3 ^ (h3_010_s3 >> 11u);
    bool h3_010_s5 = h3_100_s12 ? h3_100_s12 : true;
    uint h3_010_s6 = h3_100_s12 ? h3_100_s13 : 1664525u;
    uint h3_010_s7 = h3_100_s12 ? h3_100_s14 : 1013904223u;
    uint h3_010_s8 = (((h3_010_cell.y * h3_010_s6) + (h3_010_s4 + (h3_010_s4 << 15u))) + h3_010_s7) * h3_010_s6;
    uint h3_010_s9 = h3_010_s8 ^ (h3_010_s8 >> 11u);
    uint h3_010_s10 = h3_010_s9 ^ ((h3_010_s9 << 7u) & 2636928640u);
    uint h3_010_s11 = h3_010_s10 ^ ((h3_010_s10 << 15u) & 4022730752u);
    bool h3_010_s12 = h3_010_s5 ? h3_010_s5 : true;
    uint h3_010_s13 = h3_010_s5 ? h3_010_s6 : 1664525u;
    uint h3_010_s14 = h3_010_s5 ? h3_010_s7 : 1013904223u;
    uint h3_010_s15 = (((h3_010_cell.x * h3_010_s13) + (h3_010_s11 ^ (h3_010_s11 >> 18u))) + h3_010_s14) * h3_010_s13;
    uint h3_010_s16 = h3_010_s15 ^ (h3_010_s15 >> 11u);
    uint h3_010_s17 = h3_010_s16 ^ ((h3_010_s16 << 7u) & 2636928640u);
    uint h3_010_s18 = h3_010_s17 ^ ((h3_010_s17 << 15u) & 4022730752u);
    uint h3_010_s19 = h3_010_s18 ^ (h3_010_s18 >> 18u);
    uint h3_010_s20 = h3_010_s19 + (h3_010_s19 << 10u);
    uint h3_010_s21 = h3_010_s20 ^ (h3_010_s20 >> 6u);
    uint h3_010_s22 = h3_010_s21 + (h3_010_s21 << 3u);
    uint h3_010_s23 = h3_010_s22 ^ (h3_010_s22 >> 11u);
    bool h3_010_s24 = h3_100_s27 ? h3_100_s27 : true;
    uint h3_010_s25 = h3_100_s27 ? h3_100_s28 : 8388607u;
    uint h3_010_s26 = h3_100_s27 ? h3_100_s29 : 1065353216u;
    bool h3_010_s27 = h3_010_s24 ? h3_010_s24 : true;
    uint h3_010_s28 = h3_010_s24 ? h3_010_s25 : 8388607u;
    uint h3_010_s29 = h3_010_s24 ? h3_010_s26 : 1065353216u;
    uint3 h3_110_cell = uint3(ncell3 + int3(1, 1, 0));
    uint h3_110_s0 = h3_110_cell.z;
    uint h3_110_s1 = h3_110_s0 + (h3_110_s0 << 10u);
    uint h3_110_s2 = h3_110_s1 ^ (h3_110_s1 >> 6u);
    uint h3_110_s3 = h3_110_s2 + (h3_110_s2 << 3u);
    uint h3_110_s4 = h3_110_s3 ^ (h3_110_s3 >> 11u);
    bool h3_110_s5 = h3_010_s12 ? h3_010_s12 : true;
    uint h3_110_s6 = h3_010_s12 ? h3_010_s13 : 1664525u;
    uint h3_110_s7 = h3_010_s12 ? h3_010_s14 : 1013904223u;
    uint h3_110_s8 = (((h3_110_cell.y * h3_110_s6) + (h3_110_s4 + (h3_110_s4 << 15u))) + h3_110_s7) * h3_110_s6;
    uint h3_110_s9 = h3_110_s8 ^ (h3_110_s8 >> 11u);
    uint h3_110_s10 = h3_110_s9 ^ ((h3_110_s9 << 7u) & 2636928640u);
    uint h3_110_s11 = h3_110_s10 ^ ((h3_110_s10 << 15u) & 4022730752u);
    bool h3_110_s12 = h3_110_s5 ? h3_110_s5 : true;
    uint h3_110_s13 = h3_110_s5 ? h3_110_s6 : 1664525u;
    uint h3_110_s14 = h3_110_s5 ? h3_110_s7 : 1013904223u;
    uint h3_110_s15 = (((h3_110_cell.x * h3_110_s13) + (h3_110_s11 ^ (h3_110_s11 >> 18u))) + h3_110_s14) * h3_110_s13;
    uint h3_110_s16 = h3_110_s15 ^ (h3_110_s15 >> 11u);
    uint h3_110_s17 = h3_110_s16 ^ ((h3_110_s16 << 7u) & 2636928640u);
    uint h3_110_s18 = h3_110_s17 ^ ((h3_110_s17 << 15u) & 4022730752u);
    uint h3_110_s19 = h3_110_s18 ^ (h3_110_s18 >> 18u);
    uint h3_110_s20 = h3_110_s19 + (h3_110_s19 << 10u);
    uint h3_110_s21 = h3_110_s20 ^ (h3_110_s20 >> 6u);
    uint h3_110_s22 = h3_110_s21 + (h3_110_s21 << 3u);
    uint h3_110_s23 = h3_110_s22 ^ (h3_110_s22 >> 11u);
    bool h3_110_s24 = h3_010_s27 ? h3_010_s27 : true;
    uint h3_110_s25 = h3_010_s27 ? h3_010_s28 : 8388607u;
    uint h3_110_s26 = h3_010_s27 ? h3_010_s29 : 1065353216u;
    bool h3_110_s27 = h3_110_s24 ? h3_110_s24 : true;
    uint h3_110_s28 = h3_110_s24 ? h3_110_s25 : 8388607u;
    uint h3_110_s29 = h3_110_s24 ? h3_110_s26 : 1065353216u;
    uint3 h3_001_cell = uint3(ncell3 + int3(0, 0, 1));
    uint h3_001_s0 = h3_001_cell.z;
    uint h3_001_s1 = h3_001_s0 + (h3_001_s0 << 10u);
    uint h3_001_s2 = h3_001_s1 ^ (h3_001_s1 >> 6u);
    uint h3_001_s3 = h3_001_s2 + (h3_001_s2 << 3u);
    uint h3_001_s4 = h3_001_s3 ^ (h3_001_s3 >> 11u);
    bool h3_001_s5 = h3_110_s12 ? h3_110_s12 : true;
    uint h3_001_s6 = h3_110_s12 ? h3_110_s13 : 1664525u;
    uint h3_001_s7 = h3_110_s12 ? h3_110_s14 : 1013904223u;
    uint h3_001_s8 = (((h3_001_cell.y * h3_001_s6) + (h3_001_s4 + (h3_001_s4 << 15u))) + h3_001_s7) * h3_001_s6;
    uint h3_001_s9 = h3_001_s8 ^ (h3_001_s8 >> 11u);
    uint h3_001_s10 = h3_001_s9 ^ ((h3_001_s9 << 7u) & 2636928640u);
    uint h3_001_s11 = h3_001_s10 ^ ((h3_001_s10 << 15u) & 4022730752u);
    bool h3_001_s12 = h3_001_s5 ? h3_001_s5 : true;
    uint h3_001_s13 = h3_001_s5 ? h3_001_s6 : 1664525u;
    uint h3_001_s14 = h3_001_s5 ? h3_001_s7 : 1013904223u;
    uint h3_001_s15 = (((h3_001_cell.x * h3_001_s13) + (h3_001_s11 ^ (h3_001_s11 >> 18u))) + h3_001_s14) * h3_001_s13;
    uint h3_001_s16 = h3_001_s15 ^ (h3_001_s15 >> 11u);
    uint h3_001_s17 = h3_001_s16 ^ ((h3_001_s16 << 7u) & 2636928640u);
    uint h3_001_s18 = h3_001_s17 ^ ((h3_001_s17 << 15u) & 4022730752u);
    uint h3_001_s19 = h3_001_s18 ^ (h3_001_s18 >> 18u);
    uint h3_001_s20 = h3_001_s19 + (h3_001_s19 << 10u);
    uint h3_001_s21 = h3_001_s20 ^ (h3_001_s20 >> 6u);
    uint h3_001_s22 = h3_001_s21 + (h3_001_s21 << 3u);
    uint h3_001_s23 = h3_001_s22 ^ (h3_001_s22 >> 11u);
    bool h3_001_s24 = h3_110_s27 ? h3_110_s27 : true;
    uint h3_001_s25 = h3_110_s27 ? h3_110_s28 : 8388607u;
    uint h3_001_s26 = h3_110_s27 ? h3_110_s29 : 1065353216u;
    bool h3_001_s27 = h3_001_s24 ? h3_001_s24 : true;
    uint h3_001_s28 = h3_001_s24 ? h3_001_s25 : 8388607u;
    uint h3_001_s29 = h3_001_s24 ? h3_001_s26 : 1065353216u;
    uint3 h3_101_cell = uint3(ncell3 + int3(1, 0, 1));
    uint h3_101_s0 = h3_101_cell.z;
    uint h3_101_s1 = h3_101_s0 + (h3_101_s0 << 10u);
    uint h3_101_s2 = h3_101_s1 ^ (h3_101_s1 >> 6u);
    uint h3_101_s3 = h3_101_s2 + (h3_101_s2 << 3u);
    uint h3_101_s4 = h3_101_s3 ^ (h3_101_s3 >> 11u);
    bool h3_101_s5 = h3_001_s12 ? h3_001_s12 : true;
    uint h3_101_s6 = h3_001_s12 ? h3_001_s13 : 1664525u;
    uint h3_101_s7 = h3_001_s12 ? h3_001_s14 : 1013904223u;
    uint h3_101_s8 = (((h3_101_cell.y * h3_101_s6) + (h3_101_s4 + (h3_101_s4 << 15u))) + h3_101_s7) * h3_101_s6;
    uint h3_101_s9 = h3_101_s8 ^ (h3_101_s8 >> 11u);
    uint h3_101_s10 = h3_101_s9 ^ ((h3_101_s9 << 7u) & 2636928640u);
    uint h3_101_s11 = h3_101_s10 ^ ((h3_101_s10 << 15u) & 4022730752u);
    bool h3_101_s12 = h3_101_s5 ? h3_101_s5 : true;
    uint h3_101_s13 = h3_101_s5 ? h3_101_s6 : 1664525u;
    uint h3_101_s14 = h3_101_s5 ? h3_101_s7 : 1013904223u;
    uint h3_101_s15 = (((h3_101_cell.x * h3_101_s13) + (h3_101_s11 ^ (h3_101_s11 >> 18u))) + h3_101_s14) * h3_101_s13;
    uint h3_101_s16 = h3_101_s15 ^ (h3_101_s15 >> 11u);
    uint h3_101_s17 = h3_101_s16 ^ ((h3_101_s16 << 7u) & 2636928640u);
    uint h3_101_s18 = h3_101_s17 ^ ((h3_101_s17 << 15u) & 4022730752u);
    uint h3_101_s19 = h3_101_s18 ^ (h3_101_s18 >> 18u);
    uint h3_101_s20 = h3_101_s19 + (h3_101_s19 << 10u);
    uint h3_101_s21 = h3_101_s20 ^ (h3_101_s20 >> 6u);
    uint h3_101_s22 = h3_101_s21 + (h3_101_s21 << 3u);
    uint h3_101_s23 = h3_101_s22 ^ (h3_101_s22 >> 11u);
    bool h3_101_s24 = h3_001_s27 ? h3_001_s27 : true;
    uint h3_101_s25 = h3_001_s27 ? h3_001_s28 : 8388607u;
    uint h3_101_s26 = h3_001_s27 ? h3_001_s29 : 1065353216u;
    bool h3_101_s27 = h3_101_s24 ? h3_101_s24 : true;
    uint h3_101_s28 = h3_101_s24 ? h3_101_s25 : 8388607u;
    uint h3_101_s29 = h3_101_s24 ? h3_101_s26 : 1065353216u;
    uint3 h3_011_cell = uint3(ncell3 + int3(0, 1, 1));
    uint h3_011_s0 = h3_011_cell.z;
    uint h3_011_s1 = h3_011_s0 + (h3_011_s0 << 10u);
    uint h3_011_s2 = h3_011_s1 ^ (h3_011_s1 >> 6u);
    uint h3_011_s3 = h3_011_s2 + (h3_011_s2 << 3u);
    uint h3_011_s4 = h3_011_s3 ^ (h3_011_s3 >> 11u);
    bool h3_011_s5 = h3_101_s12 ? h3_101_s12 : true;
    uint h3_011_s6 = h3_101_s12 ? h3_101_s13 : 1664525u;
    uint h3_011_s7 = h3_101_s12 ? h3_101_s14 : 1013904223u;
    uint h3_011_s8 = (((h3_011_cell.y * h3_011_s6) + (h3_011_s4 + (h3_011_s4 << 15u))) + h3_011_s7) * h3_011_s6;
    uint h3_011_s9 = h3_011_s8 ^ (h3_011_s8 >> 11u);
    uint h3_011_s10 = h3_011_s9 ^ ((h3_011_s9 << 7u) & 2636928640u);
    uint h3_011_s11 = h3_011_s10 ^ ((h3_011_s10 << 15u) & 4022730752u);
    bool h3_011_s12 = h3_011_s5 ? h3_011_s5 : true;
    uint h3_011_s13 = h3_011_s5 ? h3_011_s6 : 1664525u;
    uint h3_011_s14 = h3_011_s5 ? h3_011_s7 : 1013904223u;
    uint h3_011_s15 = (((h3_011_cell.x * h3_011_s13) + (h3_011_s11 ^ (h3_011_s11 >> 18u))) + h3_011_s14) * h3_011_s13;
    uint h3_011_s16 = h3_011_s15 ^ (h3_011_s15 >> 11u);
    uint h3_011_s17 = h3_011_s16 ^ ((h3_011_s16 << 7u) & 2636928640u);
    uint h3_011_s18 = h3_011_s17 ^ ((h3_011_s17 << 15u) & 4022730752u);
    uint h3_011_s19 = h3_011_s18 ^ (h3_011_s18 >> 18u);
    uint h3_011_s20 = h3_011_s19 + (h3_011_s19 << 10u);
    uint h3_011_s21 = h3_011_s20 ^ (h3_011_s20 >> 6u);
    uint h3_011_s22 = h3_011_s21 + (h3_011_s21 << 3u);
    uint h3_011_s23 = h3_011_s22 ^ (h3_011_s22 >> 11u);
    bool h3_011_s24 = h3_101_s27 ? h3_101_s27 : true;
    uint h3_011_s25 = h3_101_s27 ? h3_101_s28 : 8388607u;
    uint h3_011_s26 = h3_101_s27 ? h3_101_s29 : 1065353216u;
    bool h3_011_s27 = h3_011_s24 ? h3_011_s24 : true;
    uint h3_011_s28 = h3_011_s24 ? h3_011_s25 : 8388607u;
    uint h3_011_s29 = h3_011_s24 ? h3_011_s26 : 1065353216u;
    uint3 h3_111_cell = uint3(ncell3 + int3(1, 1, 1));
    uint h3_111_s0 = h3_111_cell.z;
    uint h3_111_s1 = h3_111_s0 + (h3_111_s0 << 10u);
    uint h3_111_s2 = h3_111_s1 ^ (h3_111_s1 >> 6u);
    uint h3_111_s3 = h3_111_s2 + (h3_111_s2 << 3u);
    uint h3_111_s4 = h3_111_s3 ^ (h3_111_s3 >> 11u);
    bool h3_111_s5 = h3_011_s12 ? h3_011_s12 : true;
    uint h3_111_s6 = h3_011_s12 ? h3_011_s13 : 1664525u;
    uint h3_111_s7 = h3_011_s12 ? h3_011_s14 : 1013904223u;
    uint h3_111_s8 = (((h3_111_cell.y * h3_111_s6) + (h3_111_s4 + (h3_111_s4 << 15u))) + h3_111_s7) * h3_111_s6;
    uint h3_111_s9 = h3_111_s8 ^ (h3_111_s8 >> 11u);
    uint h3_111_s10 = h3_111_s9 ^ ((h3_111_s9 << 7u) & 2636928640u);
    uint h3_111_s11 = h3_111_s10 ^ ((h3_111_s10 << 15u) & 4022730752u);
    bool h3_111_s12 = h3_111_s5 ? h3_111_s5 : true;
    uint h3_111_s13 = h3_111_s5 ? h3_111_s6 : 1664525u;
    uint h3_111_s14 = h3_111_s5 ? h3_111_s7 : 1013904223u;
    uint h3_111_s15 = (((h3_111_cell.x * h3_111_s13) + (h3_111_s11 ^ (h3_111_s11 >> 18u))) + h3_111_s14) * h3_111_s13;
    uint h3_111_s16 = h3_111_s15 ^ (h3_111_s15 >> 11u);
    uint h3_111_s17 = h3_111_s16 ^ ((h3_111_s16 << 7u) & 2636928640u);
    uint h3_111_s18 = h3_111_s17 ^ ((h3_111_s17 << 15u) & 4022730752u);
    uint h3_111_s19 = h3_111_s18 ^ (h3_111_s18 >> 18u);
    uint h3_111_s20 = h3_111_s19 + (h3_111_s19 << 10u);
    uint h3_111_s21 = h3_111_s20 ^ (h3_111_s20 >> 6u);
    uint h3_111_s22 = h3_111_s21 + (h3_111_s21 << 3u);
    uint h3_111_s23 = h3_111_s22 ^ (h3_111_s22 >> 11u);
    bool h3_111_s24 = h3_011_s27 ? h3_011_s27 : true;
    uint h3_111_s25 = h3_011_s27 ? h3_011_s28 : 8388607u;
    uint h3_111_s26 = h3_011_s27 ? h3_011_s29 : 1065353216u;
    bool h3_111_s27 = h3_111_s24 ? h3_111_s24 : true;
    uint h3_111_s28 = h3_111_s24 ? h3_111_s25 : 8388607u;
    uint h3_111_s29 = h3_111_s24 ? h3_111_s26 : 1065353216u;
    float3 smooth3 = (nfrac3 * nfrac3) * (3.0f.xxx - (nfrac3 * 2.0f));
    float sx = smooth3.x;
    float sx_inv = 1.0f - sx;
    float sy = smooth3.y;
    float sy_inv = 1.0f - sy;
    float sz = smooth3.z;
    float sz_inv = 1.0f - sz;
    float2 noise_xy = (((((float2(asfloat((v667 & v679) | v680) - 1.0f, asfloat(((v675 + (v675 << 15u)) & v686) | v687) - 1.0f) * sx_inv) * sy_inv) * sz_inv) + (((float2(asfloat((h3_100_s19 & h3_100_s25) | h3_100_s26) - 1.0f, asfloat(((h3_100_s23 + (h3_100_s23 << 15u)) & h3_100_s28) | h3_100_s29) - 1.0f) * sx) * sy_inv) * sz_inv)) + (((float2(asfloat((h3_010_s19 & h3_010_s25) | h3_010_s26) - 1.0f, asfloat(((h3_010_s23 + (h3_010_s23 << 15u)) & h3_010_s28) | h3_010_s29) - 1.0f) * sx_inv) * sy) * sz_inv)) + (((float2(asfloat((h3_110_s19 & h3_110_s25) | h3_110_s26) - 1.0f, asfloat(((h3_110_s23 + (h3_110_s23 << 15u)) & h3_110_s28) | h3_110_s29) - 1.0f) * sx) * sy) * sz_inv);
    float2 noise_xy2 = (((noise_xy + (((float2(asfloat((h3_001_s19 & h3_001_s25) | h3_001_s26) - 1.0f, asfloat(((h3_001_s23 + (h3_001_s23 << 15u)) & h3_001_s28) | h3_001_s29) - 1.0f) * sx_inv) * sy_inv) * sz)) + (((float2(asfloat((h3_101_s19 & h3_101_s25) | h3_101_s26) - 1.0f, asfloat(((h3_101_s23 + (h3_101_s23 << 15u)) & h3_101_s28) | h3_101_s29) - 1.0f) * sx) * sy_inv) * sz)) + (((float2(asfloat((h3_011_s19 & h3_011_s25) | h3_011_s26) - 1.0f, asfloat(((h3_011_s23 + (h3_011_s23 << 15u)) & h3_011_s28) | h3_011_s29) - 1.0f) * sx_inv) * sy) * sz)) + (((float2(asfloat((h3_111_s19 & h3_111_s25) | h3_111_s26) - 1.0f, asfloat(((h3_111_s23 + (h3_111_s23 << 15u)) & h3_111_s28) | h3_111_s29) - 1.0f) * sx) * sy) * sz);
    float2 jitter = clamp((noise_xy2 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced2 = disp_base + (float3(jitter.x, 0.0f, jitter.y) * 0.0599999986588954925537109375f);
    float2 displaced_xz = displaced2.xz;
    uint h3_111_s30 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52) ^ 787234);
    uint h3_111_s31 = h3_111_s30 + (h3_111_s30 << 10u);
    uint h3_111_s32 = h3_111_s31 ^ (h3_111_s31 >> 6u);
    uint h3_111_s33 = h3_111_s32 + (h3_111_s32 << 3u);
    uint h3_111_s34 = h3_111_s33 ^ (h3_111_s33 >> 11u);
    bool h3_111_s35 = h3_111_s27 ? h3_111_s27 : true;
    uint h3_111_s36 = h3_111_s27 ? h3_111_s28 : 8388607u;
    uint h3_111_s37 = h3_111_s27 ? h3_111_s29 : 1065353216u;
    float2 uv_pos = displaced_xz + (asfloat(((h3_111_s34 + (h3_111_s34 << 15u)) & h3_111_s36) | h3_111_s37) - 1.0f).xx;
    int2 uv_cell = int2(floor(uv_pos));
    float2 uv_frac = frac(uv_pos);
    uint2 uv_cell_u = uint2(uv_cell);
    uint h3_111_s38 = uv_cell_u.y;
    uint h3_111_s39 = h3_111_s38 + (h3_111_s38 << 10u);
    uint h3_111_s40 = h3_111_s39 ^ (h3_111_s39 >> 6u);
    uint h3_111_s41 = h3_111_s40 + (h3_111_s40 << 3u);
    uint h3_111_s42 = h3_111_s41 ^ (h3_111_s41 >> 11u);
    bool h3_111_s43 = h3_111_s12 ? h3_111_s12 : true;
    uint h3_111_s44 = h3_111_s12 ? h3_111_s13 : 1664525u;
    uint h3_111_s45 = h3_111_s12 ? h3_111_s14 : 1013904223u;
    uint h3_111_s46 = (((uv_cell_u.x * h3_111_s44) + (h3_111_s42 + (h3_111_s42 << 15u))) + h3_111_s45) * h3_111_s44;
    uint h3_111_s47 = h3_111_s46 ^ (h3_111_s46 >> 11u);
    uint h3_111_s48 = h3_111_s47 ^ ((h3_111_s47 << 7u) & 2636928640u);
    uint h3_111_s49 = h3_111_s48 ^ ((h3_111_s48 << 15u) & 4022730752u);
    bool h3_111_s50 = h3_111_s35 ? h3_111_s35 : true;
    uint h3_111_s51 = h3_111_s35 ? h3_111_s36 : 8388607u;
    uint h3_111_s52 = h3_111_s35 ? h3_111_s37 : 1065353216u;
    uint2 h2_10_cell = uint2(uv_cell + int2(1, 0));
    uint h3_111_s53 = h2_10_cell.y;
    uint h3_111_s54 = h3_111_s53 + (h3_111_s53 << 10u);
    uint h3_111_s55 = h3_111_s54 ^ (h3_111_s54 >> 6u);
    uint h3_111_s56 = h3_111_s55 + (h3_111_s55 << 3u);
    uint h3_111_s57 = h3_111_s56 ^ (h3_111_s56 >> 11u);
    bool h3_111_s58 = h3_111_s43 ? h3_111_s43 : true;
    uint h3_111_s59 = h3_111_s43 ? h3_111_s44 : 1664525u;
    uint h3_111_s60 = h3_111_s43 ? h3_111_s45 : 1013904223u;
    uint h3_111_s61 = (((h2_10_cell.x * h3_111_s59) + (h3_111_s57 + (h3_111_s57 << 15u))) + h3_111_s60) * h3_111_s59;
    uint h3_111_s62 = h3_111_s61 ^ (h3_111_s61 >> 11u);
    uint h3_111_s63 = h3_111_s62 ^ ((h3_111_s62 << 7u) & 2636928640u);
    uint h3_111_s64 = h3_111_s63 ^ ((h3_111_s63 << 15u) & 4022730752u);
    bool h3_111_s65 = h3_111_s50 ? h3_111_s50 : true;
    uint h3_111_s66 = h3_111_s50 ? h3_111_s51 : 8388607u;
    uint h3_111_s67 = h3_111_s50 ? h3_111_s52 : 1065353216u;
    uint2 h2_01_cell = uint2(uv_cell + int2(0, 1));
    uint h3_111_s68 = h2_01_cell.y;
    uint h3_111_s69 = h3_111_s68 + (h3_111_s68 << 10u);
    uint h3_111_s70 = h3_111_s69 ^ (h3_111_s69 >> 6u);
    uint h3_111_s71 = h3_111_s70 + (h3_111_s70 << 3u);
    uint h3_111_s72 = h3_111_s71 ^ (h3_111_s71 >> 11u);
    bool h3_111_s73 = h3_111_s58 ? h3_111_s58 : true;
    uint h3_111_s74 = h3_111_s58 ? h3_111_s59 : 1664525u;
    uint h3_111_s75 = h3_111_s58 ? h3_111_s60 : 1013904223u;
    uint h3_111_s76 = (((h2_01_cell.x * h3_111_s74) + (h3_111_s72 + (h3_111_s72 << 15u))) + h3_111_s75) * h3_111_s74;
    uint h3_111_s77 = h3_111_s76 ^ (h3_111_s76 >> 11u);
    uint h3_111_s78 = h3_111_s77 ^ ((h3_111_s77 << 7u) & 2636928640u);
    uint h3_111_s79 = h3_111_s78 ^ ((h3_111_s78 << 15u) & 4022730752u);
    bool h3_111_s80 = h3_111_s65 ? h3_111_s65 : true;
    uint h3_111_s81 = h3_111_s65 ? h3_111_s66 : 8388607u;
    uint h3_111_s82 = h3_111_s65 ? h3_111_s67 : 1065353216u;
    uint2 h2_11_cell = uint2(uv_cell + int2(1, 1));
    uint h3_111_s83 = h2_11_cell.y;
    uint h3_111_s84 = h3_111_s83 + (h3_111_s83 << 10u);
    uint h3_111_s85 = h3_111_s84 ^ (h3_111_s84 >> 6u);
    uint h3_111_s86 = h3_111_s85 + (h3_111_s85 << 3u);
    uint h3_111_s87 = h3_111_s86 ^ (h3_111_s86 >> 11u);
    uint h3_111_s88 = h3_111_s73 ? h3_111_s74 : 1664525u;
    uint h3_111_s89 = (((h2_11_cell.x * h3_111_s88) + (h3_111_s87 + (h3_111_s87 << 15u))) + (h3_111_s73 ? h3_111_s75 : 1013904223u)) * h3_111_s88;
    uint h3_111_s90 = h3_111_s89 ^ (h3_111_s89 >> 11u);
    uint h3_111_s91 = h3_111_s90 ^ ((h3_111_s90 << 7u) & 2636928640u);
    uint h3_111_s92 = h3_111_s91 ^ ((h3_111_s91 << 15u) & 4022730752u);
    bool h3_111_s93 = h3_111_s80 ? h3_111_s80 : true;
    uint h3_111_s94 = h3_111_s80 ? h3_111_s81 : 8388607u;
    uint h3_111_s95 = h3_111_s80 ? h3_111_s82 : 1065353216u;
    float2 v1440 = (uv_frac * uv_frac) * (3.0f.xx - (uv_frac * 2.0f));
    float v1441 = v1440.x;
    float v1442 = 1.0f - v1441;
    float v1444 = v1440.y;
    float4x4 v1460 = float4x4(frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 624), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 640), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 656), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 672), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 628), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 644), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 660), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 676), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 632), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 648), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 664), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 680), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 636), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 652), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 668), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 684));
    float2 v1471 = ((displaced_xz * 0.2860000133514404296875f) * 0.5f) + ((displaced2.yy * 0.2860000133514404296875f) * 0.5f);
    float2 v1475 = floor(v1471 + dot(v1471, 0.3660254180431365966796875f.xx).xx);
    float2 v1479 = (v1471 - v1475) + dot(v1475, 0.211324870586395263671875f.xx).xx;
    float v1480 = v1479.x;
    float v1481 = v1479.y;
    bool2 v1483 = (v1480 > v1481).xx;
    float2 v1484 = float2(v1483.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, v1483.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 v1486 = v1479.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 v1488 = v1486.xy - v1484;
    float4 v1489 = float4(v1488.x, v1488.y, v1486.z, v1486.w);
    float2 v1490 = fmod(v1475, 289.0f.xx);
    float3 v1495 = v1490.y.xxx + float3(0.0f, v1484.y, 1.0f);
    float3 v1505 = (fmod(((v1495 * 34.0f) + 1.0f.xxx) * v1495, 289.0f.xxx) + v1490.x.xxx) + float3(0.0f, v1484.x, 1.0f);
    float2 v1511 = v1488.xy;
    float2 v1513 = v1486.zw;
    float3 v1517 = max(0.5f.xxx - float3(dot(v1479, v1479), dot(v1511, v1511), dot(v1513, v1513)), 0.0f.xxx);
    float3 v1518 = v1517 * v1517;
    float3 v1522 = frac(fmod(((v1505 * 34.0f) + 1.0f.xxx) * v1505, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 v1523 = v1522 - 1.0f.xxx;
    float3 v1525 = abs(v1523) - 0.5f.xxx;
    float3 v1528 = v1523 - floor(v1522 + (-0.5f).xxx);
    float3 v1540;
    v1540.x = (v1528.x * v1480) + (v1525.x * v1481);
    float2 v1547 = (v1528.yz * v1489.xz) + (v1525.yz * v1489.yw);
    v295.w = scale2 + dot(world_pos - displaced2, v295.xyz);
    uint h3_111_s96 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u;
    uint h3_111_s97 = h3_111_s96 + (h3_111_s96 << 10u);
    uint h3_111_s98 = h3_111_s97 ^ (h3_111_s97 >> 6u);
    uint h3_111_s99 = h3_111_s98 + (h3_111_s98 << 3u);
    uint h3_111_s100 = h3_111_s99 ^ (h3_111_s99 >> 11u);
    bool h3_111_s101 = h3_111_s93 ? h3_111_s93 : true;
    uint h3_111_s102 = h3_111_s93 ? h3_111_s94 : 8388607u;
    uint h3_111_s103 = h3_111_s93 ? h3_111_s95 : 1065353216u;
    uint h3_111_s104 = h3_111_s96 + 34568u;
    uint h3_111_s105 = h3_111_s104 + (h3_111_s104 << 10u);
    uint h3_111_s106 = h3_111_s105 ^ (h3_111_s105 >> 6u);
    uint h3_111_s107 = h3_111_s106 + (h3_111_s106 << 3u);
    uint h3_111_s108 = h3_111_s107 ^ (h3_111_s107 >> 11u);
    float2 v1602 = float2(round((asfloat(((h3_111_s100 + (h3_111_s100 << 15u)) & h3_111_s102) | h3_111_s103) - 1.0f) * 5.0f) * 0.20000000298023223876953125f, round((asfloat(((h3_111_s108 + (h3_111_s108 << 15u)) & (h3_111_s101 ? h3_111_s102 : 8388607u)) | (h3_111_s101 ? h3_111_s103 : 1065353216u)) - 1.0f) * 5.0f) * 0.20000000298023223876953125f);
    float v1616;
    if (!is_flagged)
    {
        v1616 = clamp(world_y * 0.0500000007450580596923828125f, 0.0f, 1.0f) * 0.699999988079071044921875f;
    }
    else
    {
        v1616 = grow_f;
    }
    gl_Position = mul(v1460, float4(displaced2, 1.0f));
    out_var_C2 = displaced2;
    out_var_C3 = pow(frac((((1.0f - v1444) * (((asfloat(((h3_111_s49 ^ (h3_111_s49 >> 18u)) & h3_111_s51) | h3_111_s52) - 1.0f) * v1442) + ((asfloat(((h3_111_s64 ^ (h3_111_s64 >> 18u)) & h3_111_s66) | h3_111_s67) - 1.0f) * v1441))) + (((asfloat(((h3_111_s79 ^ (h3_111_s79 >> 18u)) & h3_111_s81) | h3_111_s82) - 1.0f) * v1442) * v1444)) + (((asfloat(((h3_111_s92 ^ (h3_111_s92 >> 18u)) & h3_111_s94) | h3_111_s95) - 1.0f) * v1441) * v1444)), 1.5f);
    out_var_C4 = dot((v1518 * v1518) * (1.792842864990234375f.xxx - (((v1528 * v1528) + (v1525 * v1525)) * 0.8537347316741943359375f)), float3(v1540.x, v1547.x, v1547.y)) * 32.5f;
    out_var_C5 = v1616;
    out_var_C6 = (((in_var_Vertex_Position.xy * float2(0.85000002384185791015625f, 0.920000016689300537109375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + v1602;
    out_var_C7 = v295;
    out_var_C8 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C9 = scaled_vertex.z;
    out_var_C10 = (((in_var_Vertex_Position.yz * float2(0.800000011920928955078125f, 0.89999997615814208984375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + v1602;
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

static bool hole_flag;
static uint hole_flag_b;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 moss_texture_dims : packoffset(c0.y);
    uint2 roof_tile_normal_dims : packoffset(c1);
    uint2 roof_tile_damage_dims : packoffset(c1.z);
    uint highlighted_wall_id : packoffset(c2);
    uint highlight_mode : packoffset(c2.y);
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
    bool is_highlighted;
    if (1u == highlight_mode)
    {
        is_highlighted = instanced_data.Load<uint>(in_var_C8 * 64 + 40) == highlighted_wall_id;
    }
    else
    {
        is_highlighted = false;
    }
    bool is_highlighted_b;
    if (2u == highlight_mode)
    {
        is_highlighted_b = instanced_data.Load<uint>(in_var_C8 * 64 + 40) == highlighted_wall_id;
    }
    else
    {
        is_highlighted_b = false;
    }
    float3 ddx_v = ddx(in_var_C2);
    float3 ddy_v = ddy(in_var_C2);
    float3 geom_normal = normalize(cross(ddx_v, -ddy_v));
    if (dot(float4(in_var_C2, 1.0f), in_var_C7) < 0.0f)
    {
        discard;
    }
    if (instanced_data.Load<int>(in_var_C8 * 64 + 60) != 0)
    {
        int wall_id = int(instanced_data.Load<uint>(in_var_C8 * 64 + 40));
        bool any_hole;
        do
        {
            float2 cell_uv = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
            int cell_idx = int(floor(cell_uv.x)) + (15 * int(floor(cell_uv.y)));
            uint hole_count = min(128u, dormer_hole_count.Load<uint>(uint(cell_idx) * 4 + 0));
            bool is_hole;
            bool is_hole_b;
            uint hole_i = 0u;
            for (;;)
            {
                if (hole_i < hole_count)
                {
                    uint hole_idx = uint(int(uint(cell_idx * 50) + hole_i));
                    if (wall_id == dormer_holes.Load<int>(hole_idx * 48 + 12))
                    {
                        float3 hole_axis = float3(dormer_holes.Load<float>(hole_idx * 48 + 32), 0.0f, dormer_holes.Load<float>(hole_idx * 48 + 36));
                        float3x3 basis = transpose(float3x3(cross(hole_axis, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), hole_axis));
                        float3 bx = mul(basis, float3(1.0f, 0.0f, 0.0f));
                        float3 by = mul(basis, float3(0.0f, 1.0f, 0.0f));
                        float3 bz = mul(basis, float3(0.0f, 0.0f, 1.0f));
                        float3 cross_b = cross(bx, by);
                        float3 inv_det = (1.0f / dot(bz, cross_b)).xxx;
                        float3 local_pos = mul(in_var_C2 - dormer_holes.Load<float3>(hole_idx * 48 + 0), float3x3(cross(by, bz) * inv_det, cross(bz, bx) * inv_det, cross_b * inv_det)) / dormer_holes.Load<float3>(hole_idx * 48 + 16);
                        float local_y = local_pos.y;
                        float hole_curve = clamp((local_y * dormer_holes.Load<float>(hole_idx * 48 + 40)) + dormer_holes.Load<float>(hole_idx * 48 + 44), 0.0f, 1.0f);
                        bool in_hole;
                        if (abs(local_pos.x) < lerp(0.5f, 0.0f, (hole_curve * hole_curve) * hole_curve))
                        {
                            bool local_y_ok;
                            if (local_y < ((dormer_holes.Load<float>(hole_idx * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                            {
                                local_y_ok = local_y > (-0.5f);
                            }
                            else
                            {
                                local_y_ok = false;
                            }
                            in_hole = local_y_ok;
                        }
                        else
                        {
                            in_hole = false;
                        }
                        bool in_hole_x;
                        if (in_hole)
                        {
                            in_hole_x = abs(local_pos.z) < 0.5f;
                        }
                        else
                        {
                            in_hole_x = false;
                        }
                        if (in_hole_x)
                        {
                            is_hole = true;
                            is_hole_b = true;
                            break;
                        }
                    }
                    hole_i++;
                    continue;
                }
                else
                {
                    is_hole = hole_flag;
                    is_hole_b = false;
                    break;
                }
            }
            if (is_hole_b)
            {
                any_hole = is_hole;
                break;
            }
            any_hole = false;
            break;
        } while(false);
        if (any_hole)
        {
            discard;
        }
    }
    bool is_special = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 10u;
    float4 roof_sample = roof_colors.Sample(sampler_llr, float3(in_var_C2.xz * 0.20000000298023223876953125f, float(is_special ? 7 : 1)));
    float3 roof_color = (roof_sample.xyz * 0.60000002384185791015625f) * 1.5f;
    bool not_plain = !(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 0u);
    bool is_color8;
    if (not_plain)
    {
        is_color8 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 8u;
    }
    else
    {
        is_color8 = true;
    }
    bool is_color7;
    if (!is_color8)
    {
        is_color7 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 7u;
    }
    else
    {
        is_color7 = true;
    }
    bool is_color6;
    if (!is_color7)
    {
        is_color6 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 6u;
    }
    else
    {
        is_color6 = true;
    }
    float3 bright_color;
    if (is_color6)
    {
        bright_color = roof_color * 1.5f;
    }
    else
    {
        float3 dark_color;
        if (roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 1u)
        {
            dark_color = roof_color * 0.3499999940395355224609375f;
        }
        else
        {
            dark_color = roof_color;
        }
        bright_color = dark_color;
    }
    bool is_color3;
    if (!(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 2u))
    {
        is_color3 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 3u;
    }
    else
    {
        is_color3 = true;
    }
    bool is_color4;
    if (!is_color3)
    {
        is_color4 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 4u;
    }
    else
    {
        is_color4 = true;
    }
    bool is_color6b;
    if (!is_color4)
    {
        is_color6b = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 6u;
    }
    else
    {
        is_color6b = true;
    }
    bool is_color_b;
    if (!is_color6b)
    {
        is_color_b = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 9u;
    }
    else
    {
        is_color_b = true;
    }
    float v424;
    if ((!is_color_b) ? is_special : true)
    {
        v424 = 0.5f;
    }
    else
    {
        bool v407;
        if (not_plain)
        {
            v407 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 5u;
        }
        else
        {
            v407 = true;
        }
        bool v412;
        if (!v407)
        {
            v412 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 7u;
        }
        else
        {
            v412 = true;
        }
        bool v417;
        if (!v412)
        {
            v417 = roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 8u;
        }
        else
        {
            v417 = true;
        }
        float v423;
        if (v417)
        {
            v423 = 1.0f;
        }
        else
        {
            v423 = (roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0) == 1u) ? 2.0f : 0.75f;
        }
        v424 = v423;
    }
    bool v429;
    if (in_var_C9 > (-0.949999988079071044921875f))
    {
        v429 = in_var_C9 < 0.89999997615814208984375f;
    }
    else
    {
        v429 = false;
    }
    bool2 v430 = v429.xx;
    float4 v436 = roof_tile_damage.SampleBias(sampler_llr, float2(v430.x ? in_var_C10.x : in_var_C6.x, v430.y ? in_var_C10.y : in_var_C6.y), frame_constants_storage_buffer.Load<float>(208));
    float v438 = 1.0f - v436.x;
    float v439 = v438 * in_var_C3;
    float3 v444 = in_var_C2 * 0.100000001490116119384765625f;
    bool v445 = v439 > 0.00999999977648258209228515625f;
    float3 v466;
    if (v445)
    {
        uint v449 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 2834);
        uint v451 = v449 + (v449 << 10u);
        uint v453 = v451 ^ (v451 >> 6u);
        uint v455 = v453 + (v453 << 3u);
        uint v457 = v455 ^ (v455 >> 11u);
        v466 = v444 + (asfloat(((v457 + (v457 << 15u)) & 8388607u) | 1065353216u) - 1.0f).xxx;
    }
    else
    {
        v466 = v444;
    }
    bool v469 = v445 ? true : false;
    uint v471 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 34534);
    uint v473 = v471 + (v471 << 10u);
    uint v475 = v473 ^ (v473 >> 6u);
    uint v477 = v475 + (v475 << 3u);
    uint v479 = v477 ^ (v477 >> 11u);
    bool v482 = v469 ? v469 : true;
    uint v483 = v469 ? (v445 ? 8388607u : hole_flag_b) : 8388607u;
    uint v484 = v469 ? (v445 ? 1065353216u : hole_flag_b) : 1065353216u;
    float3 v513;
    if ((asfloat(((v479 + (v479 << 15u)) & v483) | v484) - 1.0f) < 0.5f)
    {
        uint v493 = uint(instanced_data.Load<int>(in_var_C8 * 64 + 52) ^ 234);
        uint v495 = v493 + (v493 << 10u);
        uint v497 = v495 ^ (v495 >> 6u);
        uint v499 = v497 + (v497 << 3u);
        uint v501 = v499 ^ (v499 >> 11u);
        v513 = v466 + ((asfloat(((v501 + (v501 << 15u)) & (v482 ? v483 : 8388607u)) | (v482 ? v484 : 1065353216u)) - 1.0f) * 0.4000000059604644775390625f).xxx;
    }
    else
    {
        v513 = v466;
    }
    float v514 = float(roof_color_ids.Load<uint>(instanced_data.Load<uint>(in_var_C8 * 64 + 40) * 4 + 0));
    float4 v519 = roof_colors.Sample(sampler_llr, float3(v513.zy, v514));
    float3 v520 = v519.xyz;
    float4 v524 = roof_colors.Sample(sampler_llr, float3(v513.xy, v514));
    float3 v525 = v524.xyz;
    float3 v527 = pow(abs(geom_normal), 1.0f.xxx);
    float v532 = dot(v520, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + v527.x;
    float v533 = dot(v525, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + v527.z;
    float v535 = max(v532, v533) - 0.20000000298023223876953125f;
    float v537 = max(v532 - v535, 0.0f);
    float v539 = max(v533 - v535, 0.0f);
    float3 v546 = (((v520 * v537) + (v525 * v539)) / (v537 + v539).xxx) * 0.60000002384185791015625f;
    float3 v556 = lerp(lerp(v546 * 0.89999997615814208984375f, v546 * 1.2999999523162841796875f, clamp(1.0f - (in_var_C3 * 3.0f), 0.0f, 1.0f).xxx), bright_color, clamp((clamp(v439 * 5.0f, 0.0f, 1.0f) * v424) + in_var_C4, 0.0f, 1.0f).xxx);
    float3 v559 = lerp(v556, v556 * 0.89999997615814208984375f, v439.xxx);
    float3 v564;
    if (is_highlighted)
    {
        v564 = lerp(v559, float3(0.300000011920928955078125f, 0.0900000035762786865234375f, 0.0f), 0.5f.xxx);
    }
    else
    {
        v564 = v559;
    }
    bool v567 = int(is_highlighted_b) == 1;
    float3 v571;
    if (v567)
    {
        v571 = lerp(v564, 0.5f.xxx, 0.100000001490116119384765625f.xxx);
    }
    else
    {
        v571 = v564;
    }
    float3 v611;
    if (in_var_C9 > 0.999000012874603271484375f)
    {
        float2 v579 = ddx(in_var_C6);
        float2 v581 = -ddy(in_var_C6);
        float3 v582 = cross(geom_normal, -ddy(in_var_C2));
        float3 v583 = cross(ddx(in_var_C2), geom_normal);
        float v584 = v579.x;
        float v585 = v581.y;
        float v587 = v579.y;
        float v588 = v581.x;
        float v591 = 1.0f / ((v584 * v585) - (v587 * v588));
        v611 = normalize(mul(transpose(float3x3(((v582 * v584) + (v583 * v588)) * v591, ((v582 * v587) + (v583 * v585)) * v591, geom_normal)), (roof_tile_normal.SampleBias(sampler_llr, in_var_C6, frame_constants_storage_buffer.Load<float>(208)).xyz * 2.0f) - 1.0f.xxx));
    }
    else
    {
        v611 = geom_normal;
    }
    float3 v625;
    if (frame_constants_storage_buffer.Load<uint>(140) == 4u)
    {
        v625 = lerp(v571, 0.699999988079071044921875f.xxx, ((smoothstep(0.100000001490116119384765625f, 1.0f, in_var_C5) * 0.5f) + ((v438 * 0.5f) * smoothstep(0.0f, 0.5f, in_var_C5))).xxx);
    }
    else
    {
        v625 = v571;
    }
    float3 v627 = (v611 * 0.5f) + 0.5f.xxx;
    float3 v654 = sqrt(clamp(v625, 0.0f.xxx, 1.0f.xxx));
    uint v673 = (uint((clamp(v654.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v654.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v654.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 v674 = uint2(((uint((clamp(v627.x, 0.0f, 1.0f) * 255.0f) + 0.5f) + (uint((clamp(v627.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u)) + (uint((clamp(v627.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + (uint((clamp(lerp(0.73500001430511474609375f, 1.0f, v439), 0.0f, 1.0f) * 255.0f) + 0.5f) << 24u), v673);
    v674.y = v673 | (((((7u | (uint(v567 ? true : (is_highlighted ? true : false)) << 3u)) | 0u) | 0u) | 0u) << 24u);
    out_var_SV_TARGET0 = v674;
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

static uint mask_uint;
static float3 mask_vec;

ByteAddressBuffer frame_constants_storage_buffer : register(t1, space1);
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index : packoffset(c0);
    uint2 moss_texture_dims : packoffset(c0.y);
    uint2 roof_tile_normal_dims : packoffset(c1);
    uint2 roof_tile_damage_dims : packoffset(c1.z);
    uint highlighted_wall_id : packoffset(c2);
    uint highlight_mode : packoffset(c2.y);
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
    [[vk::location(0)]] float3 in_var_Vertex_Position: Vertex_Position;
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
    float2 quat_p0 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x & 65535u);
    float quat_x = quat_p0.x;
    float2 quat_p1 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).x >> 16u) & 65535u);
    float quat_y = quat_p1.x;
    float2 quat_p2 = spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y & 65535u);
    float quat_z = quat_p2.x;
    float2 quat_p3 = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 32).y >> 16u) & 65535u);
    float quat_w = quat_p3.x;
    float q_xx = quat_x + quat_x;
    float q_xy = quat_y + quat_y;
    float q_xz = quat_z + quat_z;
    float q_xw = quat_x * q_xx;
    float q_yx = quat_x * q_xy;
    float q_yz = quat_x * q_xz;
    float q_yw = quat_y * q_xy;
    float q_zw = quat_y * q_xz;
    float q_wx = quat_z * q_xz;
    float q_wy = quat_w * q_xx;
    float q_wz = quat_w * q_xy;
    float q_ww = quat_w * q_xz;
    float3x4 xform3x4 = transpose(float4x3(float3(1.0f - (q_yw + q_wx), q_yx + q_ww, q_yz - q_wz), float3(q_yx - q_ww, 1.0f - (q_xw + q_wx), q_zw + q_wy), float3(q_yz + q_wz, q_zw - q_wy, 1.0f - (q_xw + q_yw)), instanced_data.Load<float3>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 0)));
    float2 scale_p = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y >> 16u) & 65535u);
    float scale2 = scale_p.x;
    float4 v295 = float4(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 16).y & 65535u).x, scale2);
    float2 grow_p = spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y >> 16u) & 65535u);
    float grow_f = grow_p.x;
    bool is_flagged = instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 60) != 0;
    float3 shingle_size = (float3(spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x & 65535u).x, spvUnpackHalf2x16((instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).x >> 16u) & 65535u).x, spvUnpackHalf2x16(instanced_data.Load<uint2>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 24).y & 65535u).x) * smoothstep(grow_f * 0.800000011920928955078125f, grow_f, pow(instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 48), 0.800000011920928955078125f))) * float3(0.505970001220703125f, 0.5f, 0.5f);
    float size_x = shingle_size.x * 0.949999988079071044921875f;
    float3 size3 = shingle_size;
    size3.x = size_x;
    float3 size_scaled = size3 * float3(3.58920383453369140625f, 3.70370388031005859375f, 3.70370388031005859375f);
    float size_min = max(min(max(size_x, max(shingle_size.y, shingle_size.z)), min(size_scaled.x, min(size_scaled.y, size_scaled.z))), 9.9999997473787516355514526367188e-06f);
    float3 scaled_vertex = in_var_Vertex_Position * float3(1.976401805877685546875f, 2.0f, 2.0f);
    float3 vertex_scaled = scaled_vertex * size_min;
    bool3 flip_dir = bool3(vertex_scaled.x >= 0.0f.xxx.x, vertex_scaled.y >= 0.0f.xxx.y, vertex_scaled.z >= 0.0f.xxx.z);
    float2 rot_p = spvUnpackHalf2x16((instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 44) >> 16u) & 65535u);
    float rot_angle = rot_p.x;
    float3 axis = normalize(float3(1.0f, 0.0f, 0.0f));
    float sin_a = sin(rot_angle);
    float cos_a = cos(rot_angle);
    float one_minus_cos = 1.0f - cos_a;
    float axis_x = axis.x;
    float oc_ax = one_minus_cos * axis_x;
    float axis_y = axis.y;
    float oc_axy = oc_ax * axis_y;
    float axis_z = axis.z;
    float az_sin = axis_z * sin_a;
    float oc_az = one_minus_cos * axis_z;
    float oc_azx = oc_az * axis_x;
    float ay_sin = axis_y * sin_a;
    float oc_ay = one_minus_cos * axis_y;
    float oc_ayz = oc_ay * axis_z;
    float ax_sin = axis_x * sin_a;
    float3 rotated_vertex = mul(transpose(float3x3(float3((oc_ax * axis_x) + cos_a, oc_axy - az_sin, oc_azx + ay_sin), float3(oc_axy + az_sin, (oc_ay * axis_y) + cos_a, oc_ayz - ax_sin), float3(oc_azx - ay_sin, oc_ayz + ax_sin, (oc_az * axis_z) + cos_a))), vertex_scaled - ((size_min.xxx - size3) * float3(int3(flip_dir.x ? int3(1, 1, 1).x : int3(-1, -1, -1).x, flip_dir.y ? int3(1, 1, 1).y : int3(-1, -1, -1).y, flip_dir.z ? int3(1, 1, 1).z : int3(-1, -1, -1).z))));
    float3 displaced;
    if (is_flagged)
    {
        uint seed_damage = (uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u) + instanced_data.Load<uint>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 40);
        uint sd10 = seed_damage + (seed_damage << 10u);
        uint sd6 = sd10 ^ (sd10 >> 6u);
        uint sd3 = sd6 + (sd6 << 3u);
        uint sd11 = sd3 ^ (sd3 >> 11u);
        float damage_amount = ((asfloat(((sd11 + (sd11 << 15u)) & 8388607u) | 1065353216u) - 1.0f) > 0.89999997615814208984375f) ? 0.0199999995529651641845703125f : 0.0599999986588954925537109375f;
        float3 disp_cur;
        if (in_var_Vertex_Position.y > 0.100000001490116119384765625f)
        {
            float3 disp_sel;
            if (instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 56) != 0)
            {
                float3 disp_low = rotated_vertex;
                disp_low.z = rotated_vertex.z - damage_amount;
                disp_sel = disp_low;
            }
            else
            {
                float3 disp_high = rotated_vertex;
                disp_high.z = rotated_vertex.z + damage_amount;
                disp_sel = disp_high;
            }
            disp_cur = disp_sel;
        }
        else
        {
            disp_cur = rotated_vertex;
        }
        displaced = disp_cur;
    }
    else
    {
        displaced = rotated_vertex;
    }
    uint mask_a = is_flagged ? 1065353216u : mask_uint;
    uint mask_b = is_flagged ? 8388607u : mask_uint;
    bool mask_flag = is_flagged ? true : false;
    float3 world_pos = mul(xform3x4, float4(displaced.xy, displaced.z + ((1.0f - smoothstep(0.0f, 0.800000011920928955078125f, mul(xform3x4, float4(displaced, 1.0f)).y)) * 0.115000002086162567138671875f), 1.0f));
    float world_y = world_pos.y;
    float roof_y = world_y + instanced_data.Load<float>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 12);
    float3 raised_pos = world_pos;
    raised_pos.y = roof_y;
    float3 disp_base;
    uint mask_c;
    uint mask_d;
    bool flag_c;
    if (is_flagged)
    {
        float2 ncell_pos = raised_pos.xz * 0.5f;
        int2 ncell = int2(floor(ncell_pos));
        float2 nfrac = frac(ncell_pos);
        uint2 ncell_u = uint2(ncell);
        uint ny = ncell_u.y;
        uint ny10 = ny + (ny << 10u);
        uint ny6 = ny10 ^ (ny10 >> 6u);
        uint ny3 = ny6 + (ny6 << 3u);
        uint ny11 = ny3 ^ (ny3 >> 11u);
        uint nmix = (((ncell_u.x * 1664525u) + (ny11 + (ny11 << 15u))) + 1013904223u) * 1664525u;
        uint nx11 = nmix ^ (nmix >> 11u);
        uint nx7 = nx11 ^ ((nx11 << 7u) & 2636928640u);
        uint nx15 = nx7 ^ ((nx7 << 15u) & 4022730752u);
        bool sel1 = mask_flag ? mask_flag : true;
        uint mul1 = mask_flag ? mask_b : 8388607u;
        uint add1 = mask_flag ? mask_a : 1065353216u;
        uint2 ncell10 = uint2(ncell + int2(1, 0));
        uint ny2 = ncell10.y;
        uint ny102 = ny2 + (ny2 << 10u);
        uint ny62 = ny102 ^ (ny102 >> 6u);
        uint ny32 = ny62 + (ny62 << 3u);
        uint ny112 = ny32 ^ (ny32 >> 11u);
        uint nmix2 = (((ncell10.x * 1664525u) + (ny112 + (ny112 << 15u))) + 1013904223u) * 1664525u;
        uint nx112 = nmix2 ^ (nmix2 >> 11u);
        uint nx72 = nx112 ^ ((nx112 << 7u) & 2636928640u);
        uint nx152 = nx72 ^ ((nx72 << 15u) & 4022730752u);
        bool sel2 = sel1 ? sel1 : true;
        uint mul2 = sel1 ? mul1 : 8388607u;
        uint add2 = sel1 ? add1 : 1065353216u;
        uint2 ncell01 = uint2(ncell + int2(0, 1));
        uint ny3b = ncell01.y;
        uint ny103 = ny3b + (ny3b << 10u);
        uint ny63 = ny103 ^ (ny103 >> 6u);
        uint ny33 = ny63 + (ny63 << 3u);
        uint ny113 = ny33 ^ (ny33 >> 11u);
        uint nmix3 = (((ncell01.x * 1664525u) + (ny113 + (ny113 << 15u))) + 1013904223u) * 1664525u;
        uint nx113 = nmix3 ^ (nmix3 >> 11u);
        uint nx73 = nx113 ^ ((nx113 << 7u) & 2636928640u);
        uint nx153 = nx73 ^ ((nx73 << 15u) & 4022730752u);
        bool sel3 = sel2 ? sel2 : true;
        uint mul3 = sel2 ? mul2 : 8388607u;
        uint add3 = sel2 ? add2 : 1065353216u;
        uint2 ncell11 = uint2(ncell + int2(1, 1));
        uint ny4 = ncell11.y;
        uint ny104 = ny4 + (ny4 << 10u);
        uint ny64 = ny104 ^ (ny104 >> 6u);
        uint v562 = ny64 + (ny64 << 3u);
        uint v564 = v562 ^ (v562 >> 11u);
        uint v570 = (((ncell11.x * 1664525u) + (v564 + (v564 << 15u))) + 1013904223u) * 1664525u;
        uint v572 = v570 ^ (v570 >> 11u);
        uint v575 = v572 ^ ((v572 << 7u) & 2636928640u);
        uint v578 = v575 ^ ((v575 << 15u) & 4022730752u);
        uint v582 = sel3 ? mul3 : 8388607u;
        uint v583 = sel3 ? add3 : 1065353216u;
        float2 v591 = (nfrac * nfrac) * (3.0f.xx - (nfrac * 2.0f));
        float v592 = v591.x;
        float v593 = 1.0f - v592;
        float v595 = v591.y;
        float3 v609 = raised_pos;
        v609.y = roof_y + ((((((1.0f - v595) * (((asfloat(((nx15 ^ (nx15 >> 18u)) & mul1) | add1) - 1.0f) * v593) + ((asfloat(((nx152 ^ (nx152 >> 18u)) & mul2) | add2) - 1.0f) * v592))) + (((asfloat(((nx153 ^ (nx153 >> 18u)) & mul3) | add3) - 1.0f) * v593) * v595)) + (((asfloat(((v578 ^ (v578 >> 18u)) & v582) | v583) - 1.0f) * v592) * v595)) * 0.300000011920928955078125f) * smoothstep(0.0f, 1.0f, world_y));
        disp_base = v609;
        mask_c = v583;
        mask_d = v582;
        flag_c = sel3 ? sel3 : true;
    }
    else
    {
        disp_base = raised_pos;
        mask_c = mask_a;
        mask_d = mask_b;
        flag_c = mask_flag;
    }
    float3 v616 = disp_base * 0.4000000059604644775390625f;
    int3 ncell3 = int3(floor(v616));
    float3 nfrac3 = frac(v616);
    uint3 v620 = uint3(ncell3);
    uint v623 = v620.z;
    uint v625 = v623 + (v623 << 10u);
    uint v627 = v625 ^ (v625 >> 6u);
    uint v629 = v627 + (v627 << 3u);
    uint v631 = v629 ^ (v629 >> 11u);
    bool v634 = mask_flag ? mask_flag : true;
    uint v635 = mask_flag ? (is_flagged ? 1664525u : mask_uint) : 1664525u;
    uint v636 = mask_flag ? (is_flagged ? 1013904223u : mask_uint) : 1013904223u;
    uint v640 = (((v620.y * v635) + (v631 + (v631 << 15u))) + v636) * v635;
    uint v642 = v640 ^ (v640 >> 11u);
    uint v645 = v642 ^ ((v642 << 7u) & 2636928640u);
    uint v648 = v645 ^ ((v645 << 15u) & 4022730752u);
    bool v651 = v634 ? v634 : true;
    uint v652 = v634 ? v635 : 1664525u;
    uint v653 = v634 ? v636 : 1013904223u;
    uint v657 = (((v620.x * v652) + (v648 ^ (v648 >> 18u))) + v653) * v652;
    uint v659 = v657 ^ (v657 >> 11u);
    uint v662 = v659 ^ ((v659 << 7u) & 2636928640u);
    uint v665 = v662 ^ ((v662 << 15u) & 4022730752u);
    uint v667 = v665 ^ (v665 >> 18u);
    uint v669 = v667 + (v667 << 10u);
    uint v671 = v669 ^ (v669 >> 6u);
    uint v673 = v671 + (v671 << 3u);
    uint v675 = v673 ^ (v673 >> 11u);
    bool v678 = flag_c ? flag_c : true;
    uint v679 = flag_c ? mask_d : 8388607u;
    uint v680 = flag_c ? mask_c : 1065353216u;
    bool v685 = v678 ? v678 : true;
    uint v686 = v678 ? v679 : 8388607u;
    uint v687 = v678 ? v680 : 1065353216u;
    uint3 h3_100_cell = uint3(ncell3 + int3(1, 0, 0));
    uint h3_100_s0 = h3_100_cell.z;
    uint h3_100_s1 = h3_100_s0 + (h3_100_s0 << 10u);
    uint h3_100_s2 = h3_100_s1 ^ (h3_100_s1 >> 6u);
    uint h3_100_s3 = h3_100_s2 + (h3_100_s2 << 3u);
    uint h3_100_s4 = h3_100_s3 ^ (h3_100_s3 >> 11u);
    bool h3_100_s5 = v651 ? v651 : true;
    uint h3_100_s6 = v651 ? v652 : 1664525u;
    uint h3_100_s7 = v651 ? v653 : 1013904223u;
    uint h3_100_s8 = (((h3_100_cell.y * h3_100_s6) + (h3_100_s4 + (h3_100_s4 << 15u))) + h3_100_s7) * h3_100_s6;
    uint h3_100_s9 = h3_100_s8 ^ (h3_100_s8 >> 11u);
    uint h3_100_s10 = h3_100_s9 ^ ((h3_100_s9 << 7u) & 2636928640u);
    uint h3_100_s11 = h3_100_s10 ^ ((h3_100_s10 << 15u) & 4022730752u);
    bool h3_100_s12 = h3_100_s5 ? h3_100_s5 : true;
    uint h3_100_s13 = h3_100_s5 ? h3_100_s6 : 1664525u;
    uint h3_100_s14 = h3_100_s5 ? h3_100_s7 : 1013904223u;
    uint h3_100_s15 = (((h3_100_cell.x * h3_100_s13) + (h3_100_s11 ^ (h3_100_s11 >> 18u))) + h3_100_s14) * h3_100_s13;
    uint h3_100_s16 = h3_100_s15 ^ (h3_100_s15 >> 11u);
    uint h3_100_s17 = h3_100_s16 ^ ((h3_100_s16 << 7u) & 2636928640u);
    uint h3_100_s18 = h3_100_s17 ^ ((h3_100_s17 << 15u) & 4022730752u);
    uint h3_100_s19 = h3_100_s18 ^ (h3_100_s18 >> 18u);
    uint h3_100_s20 = h3_100_s19 + (h3_100_s19 << 10u);
    uint h3_100_s21 = h3_100_s20 ^ (h3_100_s20 >> 6u);
    uint h3_100_s22 = h3_100_s21 + (h3_100_s21 << 3u);
    uint h3_100_s23 = h3_100_s22 ^ (h3_100_s22 >> 11u);
    bool h3_100_s24 = v685 ? v685 : true;
    uint h3_100_s25 = v685 ? v686 : 8388607u;
    uint h3_100_s26 = v685 ? v687 : 1065353216u;
    bool h3_100_s27 = h3_100_s24 ? h3_100_s24 : true;
    uint h3_100_s28 = h3_100_s24 ? h3_100_s25 : 8388607u;
    uint h3_100_s29 = h3_100_s24 ? h3_100_s26 : 1065353216u;
    uint3 h3_010_cell = uint3(ncell3 + int3(0, 1, 0));
    uint h3_010_s0 = h3_010_cell.z;
    uint h3_010_s1 = h3_010_s0 + (h3_010_s0 << 10u);
    uint h3_010_s2 = h3_010_s1 ^ (h3_010_s1 >> 6u);
    uint h3_010_s3 = h3_010_s2 + (h3_010_s2 << 3u);
    uint h3_010_s4 = h3_010_s3 ^ (h3_010_s3 >> 11u);
    bool h3_010_s5 = h3_100_s12 ? h3_100_s12 : true;
    uint h3_010_s6 = h3_100_s12 ? h3_100_s13 : 1664525u;
    uint h3_010_s7 = h3_100_s12 ? h3_100_s14 : 1013904223u;
    uint h3_010_s8 = (((h3_010_cell.y * h3_010_s6) + (h3_010_s4 + (h3_010_s4 << 15u))) + h3_010_s7) * h3_010_s6;
    uint h3_010_s9 = h3_010_s8 ^ (h3_010_s8 >> 11u);
    uint h3_010_s10 = h3_010_s9 ^ ((h3_010_s9 << 7u) & 2636928640u);
    uint h3_010_s11 = h3_010_s10 ^ ((h3_010_s10 << 15u) & 4022730752u);
    bool h3_010_s12 = h3_010_s5 ? h3_010_s5 : true;
    uint h3_010_s13 = h3_010_s5 ? h3_010_s6 : 1664525u;
    uint h3_010_s14 = h3_010_s5 ? h3_010_s7 : 1013904223u;
    uint h3_010_s15 = (((h3_010_cell.x * h3_010_s13) + (h3_010_s11 ^ (h3_010_s11 >> 18u))) + h3_010_s14) * h3_010_s13;
    uint h3_010_s16 = h3_010_s15 ^ (h3_010_s15 >> 11u);
    uint h3_010_s17 = h3_010_s16 ^ ((h3_010_s16 << 7u) & 2636928640u);
    uint h3_010_s18 = h3_010_s17 ^ ((h3_010_s17 << 15u) & 4022730752u);
    uint h3_010_s19 = h3_010_s18 ^ (h3_010_s18 >> 18u);
    uint h3_010_s20 = h3_010_s19 + (h3_010_s19 << 10u);
    uint h3_010_s21 = h3_010_s20 ^ (h3_010_s20 >> 6u);
    uint h3_010_s22 = h3_010_s21 + (h3_010_s21 << 3u);
    uint h3_010_s23 = h3_010_s22 ^ (h3_010_s22 >> 11u);
    bool h3_010_s24 = h3_100_s27 ? h3_100_s27 : true;
    uint h3_010_s25 = h3_100_s27 ? h3_100_s28 : 8388607u;
    uint h3_010_s26 = h3_100_s27 ? h3_100_s29 : 1065353216u;
    bool h3_010_s27 = h3_010_s24 ? h3_010_s24 : true;
    uint h3_010_s28 = h3_010_s24 ? h3_010_s25 : 8388607u;
    uint h3_010_s29 = h3_010_s24 ? h3_010_s26 : 1065353216u;
    uint3 h3_110_cell = uint3(ncell3 + int3(1, 1, 0));
    uint h3_110_s0 = h3_110_cell.z;
    uint h3_110_s1 = h3_110_s0 + (h3_110_s0 << 10u);
    uint h3_110_s2 = h3_110_s1 ^ (h3_110_s1 >> 6u);
    uint h3_110_s3 = h3_110_s2 + (h3_110_s2 << 3u);
    uint h3_110_s4 = h3_110_s3 ^ (h3_110_s3 >> 11u);
    bool h3_110_s5 = h3_010_s12 ? h3_010_s12 : true;
    uint h3_110_s6 = h3_010_s12 ? h3_010_s13 : 1664525u;
    uint h3_110_s7 = h3_010_s12 ? h3_010_s14 : 1013904223u;
    uint h3_110_s8 = (((h3_110_cell.y * h3_110_s6) + (h3_110_s4 + (h3_110_s4 << 15u))) + h3_110_s7) * h3_110_s6;
    uint h3_110_s9 = h3_110_s8 ^ (h3_110_s8 >> 11u);
    uint h3_110_s10 = h3_110_s9 ^ ((h3_110_s9 << 7u) & 2636928640u);
    uint h3_110_s11 = h3_110_s10 ^ ((h3_110_s10 << 15u) & 4022730752u);
    bool h3_110_s12 = h3_110_s5 ? h3_110_s5 : true;
    uint h3_110_s13 = h3_110_s5 ? h3_110_s6 : 1664525u;
    uint h3_110_s14 = h3_110_s5 ? h3_110_s7 : 1013904223u;
    uint h3_110_s15 = (((h3_110_cell.x * h3_110_s13) + (h3_110_s11 ^ (h3_110_s11 >> 18u))) + h3_110_s14) * h3_110_s13;
    uint h3_110_s16 = h3_110_s15 ^ (h3_110_s15 >> 11u);
    uint h3_110_s17 = h3_110_s16 ^ ((h3_110_s16 << 7u) & 2636928640u);
    uint h3_110_s18 = h3_110_s17 ^ ((h3_110_s17 << 15u) & 4022730752u);
    uint h3_110_s19 = h3_110_s18 ^ (h3_110_s18 >> 18u);
    uint h3_110_s20 = h3_110_s19 + (h3_110_s19 << 10u);
    uint h3_110_s21 = h3_110_s20 ^ (h3_110_s20 >> 6u);
    uint h3_110_s22 = h3_110_s21 + (h3_110_s21 << 3u);
    uint h3_110_s23 = h3_110_s22 ^ (h3_110_s22 >> 11u);
    bool h3_110_s24 = h3_010_s27 ? h3_010_s27 : true;
    uint h3_110_s25 = h3_010_s27 ? h3_010_s28 : 8388607u;
    uint h3_110_s26 = h3_010_s27 ? h3_010_s29 : 1065353216u;
    bool h3_110_s27 = h3_110_s24 ? h3_110_s24 : true;
    uint h3_110_s28 = h3_110_s24 ? h3_110_s25 : 8388607u;
    uint h3_110_s29 = h3_110_s24 ? h3_110_s26 : 1065353216u;
    uint3 h3_001_cell = uint3(ncell3 + int3(0, 0, 1));
    uint h3_001_s0 = h3_001_cell.z;
    uint h3_001_s1 = h3_001_s0 + (h3_001_s0 << 10u);
    uint h3_001_s2 = h3_001_s1 ^ (h3_001_s1 >> 6u);
    uint h3_001_s3 = h3_001_s2 + (h3_001_s2 << 3u);
    uint h3_001_s4 = h3_001_s3 ^ (h3_001_s3 >> 11u);
    bool h3_001_s5 = h3_110_s12 ? h3_110_s12 : true;
    uint h3_001_s6 = h3_110_s12 ? h3_110_s13 : 1664525u;
    uint h3_001_s7 = h3_110_s12 ? h3_110_s14 : 1013904223u;
    uint h3_001_s8 = (((h3_001_cell.y * h3_001_s6) + (h3_001_s4 + (h3_001_s4 << 15u))) + h3_001_s7) * h3_001_s6;
    uint h3_001_s9 = h3_001_s8 ^ (h3_001_s8 >> 11u);
    uint h3_001_s10 = h3_001_s9 ^ ((h3_001_s9 << 7u) & 2636928640u);
    uint h3_001_s11 = h3_001_s10 ^ ((h3_001_s10 << 15u) & 4022730752u);
    bool h3_001_s12 = h3_001_s5 ? h3_001_s5 : true;
    uint h3_001_s13 = h3_001_s5 ? h3_001_s6 : 1664525u;
    uint h3_001_s14 = h3_001_s5 ? h3_001_s7 : 1013904223u;
    uint h3_001_s15 = (((h3_001_cell.x * h3_001_s13) + (h3_001_s11 ^ (h3_001_s11 >> 18u))) + h3_001_s14) * h3_001_s13;
    uint h3_001_s16 = h3_001_s15 ^ (h3_001_s15 >> 11u);
    uint h3_001_s17 = h3_001_s16 ^ ((h3_001_s16 << 7u) & 2636928640u);
    uint h3_001_s18 = h3_001_s17 ^ ((h3_001_s17 << 15u) & 4022730752u);
    uint h3_001_s19 = h3_001_s18 ^ (h3_001_s18 >> 18u);
    uint h3_001_s20 = h3_001_s19 + (h3_001_s19 << 10u);
    uint h3_001_s21 = h3_001_s20 ^ (h3_001_s20 >> 6u);
    uint h3_001_s22 = h3_001_s21 + (h3_001_s21 << 3u);
    uint h3_001_s23 = h3_001_s22 ^ (h3_001_s22 >> 11u);
    bool h3_001_s24 = h3_110_s27 ? h3_110_s27 : true;
    uint h3_001_s25 = h3_110_s27 ? h3_110_s28 : 8388607u;
    uint h3_001_s26 = h3_110_s27 ? h3_110_s29 : 1065353216u;
    bool h3_001_s27 = h3_001_s24 ? h3_001_s24 : true;
    uint h3_001_s28 = h3_001_s24 ? h3_001_s25 : 8388607u;
    uint h3_001_s29 = h3_001_s24 ? h3_001_s26 : 1065353216u;
    uint3 h3_101_cell = uint3(ncell3 + int3(1, 0, 1));
    uint h3_101_s0 = h3_101_cell.z;
    uint h3_101_s1 = h3_101_s0 + (h3_101_s0 << 10u);
    uint h3_101_s2 = h3_101_s1 ^ (h3_101_s1 >> 6u);
    uint h3_101_s3 = h3_101_s2 + (h3_101_s2 << 3u);
    uint h3_101_s4 = h3_101_s3 ^ (h3_101_s3 >> 11u);
    bool h3_101_s5 = h3_001_s12 ? h3_001_s12 : true;
    uint h3_101_s6 = h3_001_s12 ? h3_001_s13 : 1664525u;
    uint h3_101_s7 = h3_001_s12 ? h3_001_s14 : 1013904223u;
    uint h3_101_s8 = (((h3_101_cell.y * h3_101_s6) + (h3_101_s4 + (h3_101_s4 << 15u))) + h3_101_s7) * h3_101_s6;
    uint h3_101_s9 = h3_101_s8 ^ (h3_101_s8 >> 11u);
    uint h3_101_s10 = h3_101_s9 ^ ((h3_101_s9 << 7u) & 2636928640u);
    uint h3_101_s11 = h3_101_s10 ^ ((h3_101_s10 << 15u) & 4022730752u);
    bool h3_101_s12 = h3_101_s5 ? h3_101_s5 : true;
    uint h3_101_s13 = h3_101_s5 ? h3_101_s6 : 1664525u;
    uint h3_101_s14 = h3_101_s5 ? h3_101_s7 : 1013904223u;
    uint h3_101_s15 = (((h3_101_cell.x * h3_101_s13) + (h3_101_s11 ^ (h3_101_s11 >> 18u))) + h3_101_s14) * h3_101_s13;
    uint h3_101_s16 = h3_101_s15 ^ (h3_101_s15 >> 11u);
    uint h3_101_s17 = h3_101_s16 ^ ((h3_101_s16 << 7u) & 2636928640u);
    uint h3_101_s18 = h3_101_s17 ^ ((h3_101_s17 << 15u) & 4022730752u);
    uint h3_101_s19 = h3_101_s18 ^ (h3_101_s18 >> 18u);
    uint h3_101_s20 = h3_101_s19 + (h3_101_s19 << 10u);
    uint h3_101_s21 = h3_101_s20 ^ (h3_101_s20 >> 6u);
    uint h3_101_s22 = h3_101_s21 + (h3_101_s21 << 3u);
    uint h3_101_s23 = h3_101_s22 ^ (h3_101_s22 >> 11u);
    bool h3_101_s24 = h3_001_s27 ? h3_001_s27 : true;
    uint h3_101_s25 = h3_001_s27 ? h3_001_s28 : 8388607u;
    uint h3_101_s26 = h3_001_s27 ? h3_001_s29 : 1065353216u;
    bool h3_101_s27 = h3_101_s24 ? h3_101_s24 : true;
    uint h3_101_s28 = h3_101_s24 ? h3_101_s25 : 8388607u;
    uint h3_101_s29 = h3_101_s24 ? h3_101_s26 : 1065353216u;
    uint3 h3_011_cell = uint3(ncell3 + int3(0, 1, 1));
    uint h3_011_s0 = h3_011_cell.z;
    uint h3_011_s1 = h3_011_s0 + (h3_011_s0 << 10u);
    uint h3_011_s2 = h3_011_s1 ^ (h3_011_s1 >> 6u);
    uint h3_011_s3 = h3_011_s2 + (h3_011_s2 << 3u);
    uint h3_011_s4 = h3_011_s3 ^ (h3_011_s3 >> 11u);
    bool h3_011_s5 = h3_101_s12 ? h3_101_s12 : true;
    uint h3_011_s6 = h3_101_s12 ? h3_101_s13 : 1664525u;
    uint h3_011_s7 = h3_101_s12 ? h3_101_s14 : 1013904223u;
    uint h3_011_s8 = (((h3_011_cell.y * h3_011_s6) + (h3_011_s4 + (h3_011_s4 << 15u))) + h3_011_s7) * h3_011_s6;
    uint h3_011_s9 = h3_011_s8 ^ (h3_011_s8 >> 11u);
    uint h3_011_s10 = h3_011_s9 ^ ((h3_011_s9 << 7u) & 2636928640u);
    uint h3_011_s11 = h3_011_s10 ^ ((h3_011_s10 << 15u) & 4022730752u);
    bool h3_011_s12 = h3_011_s5 ? h3_011_s5 : true;
    uint h3_011_s13 = h3_011_s5 ? h3_011_s6 : 1664525u;
    uint h3_011_s14 = h3_011_s5 ? h3_011_s7 : 1013904223u;
    uint h3_011_s15 = (((h3_011_cell.x * h3_011_s13) + (h3_011_s11 ^ (h3_011_s11 >> 18u))) + h3_011_s14) * h3_011_s13;
    uint h3_011_s16 = h3_011_s15 ^ (h3_011_s15 >> 11u);
    uint h3_011_s17 = h3_011_s16 ^ ((h3_011_s16 << 7u) & 2636928640u);
    uint h3_011_s18 = h3_011_s17 ^ ((h3_011_s17 << 15u) & 4022730752u);
    uint h3_011_s19 = h3_011_s18 ^ (h3_011_s18 >> 18u);
    uint h3_011_s20 = h3_011_s19 + (h3_011_s19 << 10u);
    uint h3_011_s21 = h3_011_s20 ^ (h3_011_s20 >> 6u);
    uint h3_011_s22 = h3_011_s21 + (h3_011_s21 << 3u);
    uint h3_011_s23 = h3_011_s22 ^ (h3_011_s22 >> 11u);
    bool h3_011_s24 = h3_101_s27 ? h3_101_s27 : true;
    uint h3_011_s25 = h3_101_s27 ? h3_101_s28 : 8388607u;
    uint h3_011_s26 = h3_101_s27 ? h3_101_s29 : 1065353216u;
    bool h3_011_s27 = h3_011_s24 ? h3_011_s24 : true;
    uint h3_011_s28 = h3_011_s24 ? h3_011_s25 : 8388607u;
    uint h3_011_s29 = h3_011_s24 ? h3_011_s26 : 1065353216u;
    uint3 h3_111_cell = uint3(ncell3 + int3(1, 1, 1));
    uint h3_111_s0 = h3_111_cell.z;
    uint h3_111_s1 = h3_111_s0 + (h3_111_s0 << 10u);
    uint h3_111_s2 = h3_111_s1 ^ (h3_111_s1 >> 6u);
    uint h3_111_s3 = h3_111_s2 + (h3_111_s2 << 3u);
    uint h3_111_s4 = h3_111_s3 ^ (h3_111_s3 >> 11u);
    bool h3_111_s5 = h3_011_s12 ? h3_011_s12 : true;
    uint h3_111_s6 = h3_011_s12 ? h3_011_s13 : 1664525u;
    uint h3_111_s7 = h3_011_s12 ? h3_011_s14 : 1013904223u;
    uint h3_111_s8 = (((h3_111_cell.y * h3_111_s6) + (h3_111_s4 + (h3_111_s4 << 15u))) + h3_111_s7) * h3_111_s6;
    uint h3_111_s9 = h3_111_s8 ^ (h3_111_s8 >> 11u);
    uint h3_111_s10 = h3_111_s9 ^ ((h3_111_s9 << 7u) & 2636928640u);
    uint h3_111_s11 = h3_111_s10 ^ ((h3_111_s10 << 15u) & 4022730752u);
    bool h3_111_s12 = h3_111_s5 ? h3_111_s5 : true;
    uint h3_111_s13 = h3_111_s5 ? h3_111_s6 : 1664525u;
    uint h3_111_s14 = h3_111_s5 ? h3_111_s7 : 1013904223u;
    uint h3_111_s15 = (((h3_111_cell.x * h3_111_s13) + (h3_111_s11 ^ (h3_111_s11 >> 18u))) + h3_111_s14) * h3_111_s13;
    uint h3_111_s16 = h3_111_s15 ^ (h3_111_s15 >> 11u);
    uint h3_111_s17 = h3_111_s16 ^ ((h3_111_s16 << 7u) & 2636928640u);
    uint h3_111_s18 = h3_111_s17 ^ ((h3_111_s17 << 15u) & 4022730752u);
    uint h3_111_s19 = h3_111_s18 ^ (h3_111_s18 >> 18u);
    uint h3_111_s20 = h3_111_s19 + (h3_111_s19 << 10u);
    uint h3_111_s21 = h3_111_s20 ^ (h3_111_s20 >> 6u);
    uint h3_111_s22 = h3_111_s21 + (h3_111_s21 << 3u);
    uint h3_111_s23 = h3_111_s22 ^ (h3_111_s22 >> 11u);
    bool h3_111_s24 = h3_011_s27 ? h3_011_s27 : true;
    uint h3_111_s25 = h3_011_s27 ? h3_011_s28 : 8388607u;
    uint h3_111_s26 = h3_011_s27 ? h3_011_s29 : 1065353216u;
    bool h3_111_s27 = h3_111_s24 ? h3_111_s24 : true;
    uint h3_111_s28 = h3_111_s24 ? h3_111_s25 : 8388607u;
    uint h3_111_s29 = h3_111_s24 ? h3_111_s26 : 1065353216u;
    float3 smooth3 = (nfrac3 * nfrac3) * (3.0f.xxx - (nfrac3 * 2.0f));
    float sx = smooth3.x;
    float sx_inv = 1.0f - sx;
    float sy = smooth3.y;
    float sy_inv = 1.0f - sy;
    float sz = smooth3.z;
    float sz_inv = 1.0f - sz;
    float2 noise_xy = (((((float2(asfloat((v667 & v679) | v680) - 1.0f, asfloat(((v675 + (v675 << 15u)) & v686) | v687) - 1.0f) * sx_inv) * sy_inv) * sz_inv) + (((float2(asfloat((h3_100_s19 & h3_100_s25) | h3_100_s26) - 1.0f, asfloat(((h3_100_s23 + (h3_100_s23 << 15u)) & h3_100_s28) | h3_100_s29) - 1.0f) * sx) * sy_inv) * sz_inv)) + (((float2(asfloat((h3_010_s19 & h3_010_s25) | h3_010_s26) - 1.0f, asfloat(((h3_010_s23 + (h3_010_s23 << 15u)) & h3_010_s28) | h3_010_s29) - 1.0f) * sx_inv) * sy) * sz_inv)) + (((float2(asfloat((h3_110_s19 & h3_110_s25) | h3_110_s26) - 1.0f, asfloat(((h3_110_s23 + (h3_110_s23 << 15u)) & h3_110_s28) | h3_110_s29) - 1.0f) * sx) * sy) * sz_inv);
    float2 noise_xy2 = (((noise_xy + (((float2(asfloat((h3_001_s19 & h3_001_s25) | h3_001_s26) - 1.0f, asfloat(((h3_001_s23 + (h3_001_s23 << 15u)) & h3_001_s28) | h3_001_s29) - 1.0f) * sx_inv) * sy_inv) * sz)) + (((float2(asfloat((h3_101_s19 & h3_101_s25) | h3_101_s26) - 1.0f, asfloat(((h3_101_s23 + (h3_101_s23 << 15u)) & h3_101_s28) | h3_101_s29) - 1.0f) * sx) * sy_inv) * sz)) + (((float2(asfloat((h3_011_s19 & h3_011_s25) | h3_011_s26) - 1.0f, asfloat(((h3_011_s23 + (h3_011_s23 << 15u)) & h3_011_s28) | h3_011_s29) - 1.0f) * sx_inv) * sy) * sz)) + (((float2(asfloat((h3_111_s19 & h3_111_s25) | h3_111_s26) - 1.0f, asfloat(((h3_111_s23 + (h3_111_s23 << 15u)) & h3_111_s28) | h3_111_s29) - 1.0f) * sx) * sy) * sz);
    float2 jitter = clamp((noise_xy2 - 0.5f.xx) * 2.5f, (-0.5f).xx, 0.5f.xx);
    float3 displaced2 = disp_base + (float3(jitter.x, 0.0f, jitter.y) * 0.0599999986588954925537109375f);
    float2 displaced_xz = displaced2.xz;
    uint h3_111_s30 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52) ^ 787234);
    uint h3_111_s31 = h3_111_s30 + (h3_111_s30 << 10u);
    uint h3_111_s32 = h3_111_s31 ^ (h3_111_s31 >> 6u);
    uint h3_111_s33 = h3_111_s32 + (h3_111_s32 << 3u);
    uint h3_111_s34 = h3_111_s33 ^ (h3_111_s33 >> 11u);
    bool h3_111_s35 = h3_111_s27 ? h3_111_s27 : true;
    uint h3_111_s36 = h3_111_s27 ? h3_111_s28 : 8388607u;
    uint h3_111_s37 = h3_111_s27 ? h3_111_s29 : 1065353216u;
    float2 uv_pos = displaced_xz + (asfloat(((h3_111_s34 + (h3_111_s34 << 15u)) & h3_111_s36) | h3_111_s37) - 1.0f).xx;
    int2 uv_cell = int2(floor(uv_pos));
    float2 uv_frac = frac(uv_pos);
    uint2 uv_cell_u = uint2(uv_cell);
    uint h3_111_s38 = uv_cell_u.y;
    uint h3_111_s39 = h3_111_s38 + (h3_111_s38 << 10u);
    uint h3_111_s40 = h3_111_s39 ^ (h3_111_s39 >> 6u);
    uint h3_111_s41 = h3_111_s40 + (h3_111_s40 << 3u);
    uint h3_111_s42 = h3_111_s41 ^ (h3_111_s41 >> 11u);
    bool h3_111_s43 = h3_111_s12 ? h3_111_s12 : true;
    uint h3_111_s44 = h3_111_s12 ? h3_111_s13 : 1664525u;
    uint h3_111_s45 = h3_111_s12 ? h3_111_s14 : 1013904223u;
    uint h3_111_s46 = (((uv_cell_u.x * h3_111_s44) + (h3_111_s42 + (h3_111_s42 << 15u))) + h3_111_s45) * h3_111_s44;
    uint h3_111_s47 = h3_111_s46 ^ (h3_111_s46 >> 11u);
    uint h3_111_s48 = h3_111_s47 ^ ((h3_111_s47 << 7u) & 2636928640u);
    uint h3_111_s49 = h3_111_s48 ^ ((h3_111_s48 << 15u) & 4022730752u);
    bool h3_111_s50 = h3_111_s35 ? h3_111_s35 : true;
    uint h3_111_s51 = h3_111_s35 ? h3_111_s36 : 8388607u;
    uint h3_111_s52 = h3_111_s35 ? h3_111_s37 : 1065353216u;
    uint2 h2_10_cell = uint2(uv_cell + int2(1, 0));
    uint h3_111_s53 = h2_10_cell.y;
    uint h3_111_s54 = h3_111_s53 + (h3_111_s53 << 10u);
    uint h3_111_s55 = h3_111_s54 ^ (h3_111_s54 >> 6u);
    uint h3_111_s56 = h3_111_s55 + (h3_111_s55 << 3u);
    uint h3_111_s57 = h3_111_s56 ^ (h3_111_s56 >> 11u);
    bool h3_111_s58 = h3_111_s43 ? h3_111_s43 : true;
    uint h3_111_s59 = h3_111_s43 ? h3_111_s44 : 1664525u;
    uint h3_111_s60 = h3_111_s43 ? h3_111_s45 : 1013904223u;
    uint h3_111_s61 = (((h2_10_cell.x * h3_111_s59) + (h3_111_s57 + (h3_111_s57 << 15u))) + h3_111_s60) * h3_111_s59;
    uint h3_111_s62 = h3_111_s61 ^ (h3_111_s61 >> 11u);
    uint h3_111_s63 = h3_111_s62 ^ ((h3_111_s62 << 7u) & 2636928640u);
    uint h3_111_s64 = h3_111_s63 ^ ((h3_111_s63 << 15u) & 4022730752u);
    bool h3_111_s65 = h3_111_s50 ? h3_111_s50 : true;
    uint h3_111_s66 = h3_111_s50 ? h3_111_s51 : 8388607u;
    uint h3_111_s67 = h3_111_s50 ? h3_111_s52 : 1065353216u;
    uint2 h2_01_cell = uint2(uv_cell + int2(0, 1));
    uint h3_111_s68 = h2_01_cell.y;
    uint h3_111_s69 = h3_111_s68 + (h3_111_s68 << 10u);
    uint h3_111_s70 = h3_111_s69 ^ (h3_111_s69 >> 6u);
    uint h3_111_s71 = h3_111_s70 + (h3_111_s70 << 3u);
    uint h3_111_s72 = h3_111_s71 ^ (h3_111_s71 >> 11u);
    bool h3_111_s73 = h3_111_s58 ? h3_111_s58 : true;
    uint h3_111_s74 = h3_111_s58 ? h3_111_s59 : 1664525u;
    uint h3_111_s75 = h3_111_s58 ? h3_111_s60 : 1013904223u;
    uint h3_111_s76 = (((h2_01_cell.x * h3_111_s74) + (h3_111_s72 + (h3_111_s72 << 15u))) + h3_111_s75) * h3_111_s74;
    uint h3_111_s77 = h3_111_s76 ^ (h3_111_s76 >> 11u);
    uint h3_111_s78 = h3_111_s77 ^ ((h3_111_s77 << 7u) & 2636928640u);
    uint h3_111_s79 = h3_111_s78 ^ ((h3_111_s78 << 15u) & 4022730752u);
    bool h3_111_s80 = h3_111_s65 ? h3_111_s65 : true;
    uint h3_111_s81 = h3_111_s65 ? h3_111_s66 : 8388607u;
    uint h3_111_s82 = h3_111_s65 ? h3_111_s67 : 1065353216u;
    uint2 h2_11_cell = uint2(uv_cell + int2(1, 1));
    uint h3_111_s83 = h2_11_cell.y;
    uint h3_111_s84 = h3_111_s83 + (h3_111_s83 << 10u);
    uint h3_111_s85 = h3_111_s84 ^ (h3_111_s84 >> 6u);
    uint h3_111_s86 = h3_111_s85 + (h3_111_s85 << 3u);
    uint h3_111_s87 = h3_111_s86 ^ (h3_111_s86 >> 11u);
    uint h3_111_s88 = h3_111_s73 ? h3_111_s74 : 1664525u;
    uint h3_111_s89 = (((h2_11_cell.x * h3_111_s88) + (h3_111_s87 + (h3_111_s87 << 15u))) + (h3_111_s73 ? h3_111_s75 : 1013904223u)) * h3_111_s88;
    uint h3_111_s90 = h3_111_s89 ^ (h3_111_s89 >> 11u);
    uint h3_111_s91 = h3_111_s90 ^ ((h3_111_s90 << 7u) & 2636928640u);
    uint h3_111_s92 = h3_111_s91 ^ ((h3_111_s91 << 15u) & 4022730752u);
    bool h3_111_s93 = h3_111_s80 ? h3_111_s80 : true;
    uint h3_111_s94 = h3_111_s80 ? h3_111_s81 : 8388607u;
    uint h3_111_s95 = h3_111_s80 ? h3_111_s82 : 1065353216u;
    float2 v1440 = (uv_frac * uv_frac) * (3.0f.xx - (uv_frac * 2.0f));
    float v1441 = v1440.x;
    float v1442 = 1.0f - v1441;
    float v1444 = v1440.y;
    float4x4 v1460 = float4x4(frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 624), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 640), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 656), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 672), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 628), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 644), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 660), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 676), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 632), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 648), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 664), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 680), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 636), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 652), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 668), frame_constants_storage_buffer.Load<float>(current_view_index * 800 + 684));
    float2 v1471 = ((displaced_xz * 0.2860000133514404296875f) * 0.5f) + ((displaced2.yy * 0.2860000133514404296875f) * 0.5f);
    float2 v1475 = floor(v1471 + dot(v1471, 0.3660254180431365966796875f.xx).xx);
    float2 v1479 = (v1471 - v1475) + dot(v1475, 0.211324870586395263671875f.xx).xx;
    float v1480 = v1479.x;
    float v1481 = v1479.y;
    bool2 v1483 = (v1480 > v1481).xx;
    float2 v1484 = float2(v1483.x ? float2(1.0f, 0.0f).x : float2(0.0f, 1.0f).x, v1483.y ? float2(1.0f, 0.0f).y : float2(0.0f, 1.0f).y);
    float4 v1486 = v1479.xyxy + float4(0.211324870586395263671875f, 0.211324870586395263671875f, -0.57735025882720947265625f, -0.57735025882720947265625f);
    float2 v1488 = v1486.xy - v1484;
    float4 v1489 = float4(v1488.x, v1488.y, v1486.z, v1486.w);
    float2 v1490 = fmod(v1475, 289.0f.xx);
    float3 v1495 = v1490.y.xxx + float3(0.0f, v1484.y, 1.0f);
    float3 v1505 = (fmod(((v1495 * 34.0f) + 1.0f.xxx) * v1495, 289.0f.xxx) + v1490.x.xxx) + float3(0.0f, v1484.x, 1.0f);
    float2 v1511 = v1488.xy;
    float2 v1513 = v1486.zw;
    float3 v1517 = max(0.5f.xxx - float3(dot(v1479, v1479), dot(v1511, v1511), dot(v1513, v1513)), 0.0f.xxx);
    float3 v1518 = v1517 * v1517;
    float3 v1522 = frac(fmod(((v1505 * 34.0f) + 1.0f.xxx) * v1505, 289.0f.xxx) * 0.024390242993831634521484375f.xxx) * 2.0f;
    float3 v1523 = v1522 - 1.0f.xxx;
    float3 v1525 = abs(v1523) - 0.5f.xxx;
    float3 v1528 = v1523 - floor(v1522 + (-0.5f).xxx);
    float3 v1540;
    v1540.x = (v1528.x * v1480) + (v1525.x * v1481);
    float2 v1547 = (v1528.yz * v1489.xz) + (v1525.yz * v1489.yw);
    v295.w = scale2 + dot(world_pos - displaced2, v295.xyz);
    uint h3_111_s96 = uint(instanced_data.Load<int>(instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0) * 64 + 52)) * 1664525u;
    uint h3_111_s97 = h3_111_s96 + (h3_111_s96 << 10u);
    uint h3_111_s98 = h3_111_s97 ^ (h3_111_s97 >> 6u);
    uint h3_111_s99 = h3_111_s98 + (h3_111_s98 << 3u);
    uint h3_111_s100 = h3_111_s99 ^ (h3_111_s99 >> 11u);
    bool h3_111_s101 = h3_111_s93 ? h3_111_s93 : true;
    uint h3_111_s102 = h3_111_s93 ? h3_111_s94 : 8388607u;
    uint h3_111_s103 = h3_111_s93 ? h3_111_s95 : 1065353216u;
    uint h3_111_s104 = h3_111_s96 + 34568u;
    uint h3_111_s105 = h3_111_s104 + (h3_111_s104 << 10u);
    uint h3_111_s106 = h3_111_s105 ^ (h3_111_s105 >> 6u);
    uint h3_111_s107 = h3_111_s106 + (h3_111_s106 << 3u);
    uint h3_111_s108 = h3_111_s107 ^ (h3_111_s107 >> 11u);
    float2 v1602 = float2(round((asfloat(((h3_111_s100 + (h3_111_s100 << 15u)) & h3_111_s102) | h3_111_s103) - 1.0f) * 5.0f) * 0.20000000298023223876953125f, round((asfloat(((h3_111_s108 + (h3_111_s108 << 15u)) & (h3_111_s101 ? h3_111_s102 : 8388607u)) | (h3_111_s101 ? h3_111_s103 : 1065353216u)) - 1.0f) * 5.0f) * 0.20000000298023223876953125f);
    float v1616;
    if (!is_flagged)
    {
        v1616 = clamp(world_y * 0.0500000007450580596923828125f, 0.0f, 1.0f) * 0.699999988079071044921875f;
    }
    else
    {
        v1616 = grow_f;
    }
    gl_Position = mul(v1460, float4(displaced2, 1.0f));
    out_var_C2 = displaced2;
    out_var_C3 = pow(frac((((1.0f - v1444) * (((asfloat(((h3_111_s49 ^ (h3_111_s49 >> 18u)) & h3_111_s51) | h3_111_s52) - 1.0f) * v1442) + ((asfloat(((h3_111_s64 ^ (h3_111_s64 >> 18u)) & h3_111_s66) | h3_111_s67) - 1.0f) * v1441))) + (((asfloat(((h3_111_s79 ^ (h3_111_s79 >> 18u)) & h3_111_s81) | h3_111_s82) - 1.0f) * v1442) * v1444)) + (((asfloat(((h3_111_s92 ^ (h3_111_s92 >> 18u)) & h3_111_s94) | h3_111_s95) - 1.0f) * v1441) * v1444)), 1.5f);
    out_var_C4 = dot((v1518 * v1518) * (1.792842864990234375f.xxx - (((v1528 * v1528) + (v1525 * v1525)) * 0.8537347316741943359375f)), float3(v1540.x, v1547.x, v1547.y)) * 32.5f;
    out_var_C5 = v1616;
    out_var_C6 = (((in_var_Vertex_Position.xy * float2(0.85000002384185791015625f, 0.920000016689300537109375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + v1602;
    out_var_C7 = v295;
    out_var_C8 = instance_indirection_buffer.Load<uint>(uint(gl_InstanceIndex) * 4 + 0);
    out_var_C9 = scaled_vertex.z;
    out_var_C10 = (((in_var_Vertex_Position.yz * float2(0.800000011920928955078125f, 0.89999997615814208984375f)) + 0.5f.xx) * 0.20000000298023223876953125f) + v1602;
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

static bool v95;

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
        int wall_id = int(instanced_data.Load<uint>(in_var_C8 * 64 + 40));
        bool any_hole;
        do
        {
            float2 cell_uv = (in_var_C2.xz + 65.0f.xx) * 0.1153846085071563720703125f.xx;
            int cell_idx = int(floor(cell_uv.x)) + (15 * int(floor(cell_uv.y)));
            uint hole_count = min(128u, dormer_hole_count.Load<uint>(uint(cell_idx) * 4 + 0));
            bool is_hole;
            bool is_hole_b;
            uint hole_i = 0u;
            for (;;)
            {
                if (hole_i < hole_count)
                {
                    uint hole_idx = uint(int(uint(cell_idx * 50) + hole_i));
                    if (wall_id == dormer_holes.Load<int>(hole_idx * 48 + 12))
                    {
                        float3 hole_axis = float3(dormer_holes.Load<float>(hole_idx * 48 + 32), 0.0f, dormer_holes.Load<float>(hole_idx * 48 + 36));
                        float3x3 basis = transpose(float3x3(cross(hole_axis, float3(0.0f, 1.0f, 0.0f)), float3(0.0f, 1.0f, 0.0f), hole_axis));
                        float3 bx = mul(basis, float3(1.0f, 0.0f, 0.0f));
                        float3 by = mul(basis, float3(0.0f, 1.0f, 0.0f));
                        float3 bz = mul(basis, float3(0.0f, 0.0f, 1.0f));
                        float3 cross_b = cross(bx, by);
                        float3 inv_det = (1.0f / dot(bz, cross_b)).xxx;
                        float3 local_pos = mul(in_var_C2 - dormer_holes.Load<float3>(hole_idx * 48 + 0), float3x3(cross(by, bz) * inv_det, cross(bz, bx) * inv_det, cross_b * inv_det)) / dormer_holes.Load<float3>(hole_idx * 48 + 16);
                        float local_y = local_pos.y;
                        float hole_curve = clamp((local_y * dormer_holes.Load<float>(hole_idx * 48 + 40)) + dormer_holes.Load<float>(hole_idx * 48 + 44), 0.0f, 1.0f);
                        bool in_hole;
                        if (abs(local_pos.x) < lerp(0.5f, 0.0f, (hole_curve * hole_curve) * hole_curve))
                        {
                            bool local_y_ok;
                            if (local_y < ((dormer_holes.Load<float>(hole_idx * 48 + 40) == 1.0f) ? 0.5f : 0.699999988079071044921875f))
                            {
                                local_y_ok = local_y > (-0.5f);
                            }
                            else
                            {
                                local_y_ok = false;
                            }
                            in_hole = local_y_ok;
                        }
                        else
                        {
                            in_hole = false;
                        }
                        bool in_hole_x;
                        if (in_hole)
                        {
                            in_hole_x = abs(local_pos.z) < 0.5f;
                        }
                        else
                        {
                            in_hole_x = false;
                        }
                        if (in_hole_x)
                        {
                            is_hole = true;
                            is_hole_b = true;
                            break;
                        }
                    }
                    hole_i++;
                    continue;
                }
                else
                {
                    is_hole = v95;
                    is_hole_b = false;
                    break;
                }
            }
            if (is_hole_b)
            {
                any_hole = is_hole;
                break;
            }
            any_hole = false;
            break;
        } while(false);
        if (any_hole)
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
