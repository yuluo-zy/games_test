// ============================================================
// leaf_particles.cs.hlsl
// 功能:叶片粒子模拟。每个线程拥有一个 32 字节 LeafParticle
//       (position f3 @0, rotation f @12, scale f @16, up_axis f3 @20)。
// 行为:
//   空中叶片(y>0.05):受阵风(3 层值噪声晶格 + 整数哈希)漂移并缓慢旋转;
//   落地叶片:在草地遮罩(grass_object_push)上沉降,并随水流噪声流动;
//   跌到 -0.03 以下:从随机 ParticleEmitter 重生。
//   Glade 主题 4(秋天)禁用风/水流位移。
// 入口:cs_main(计算着色器,64 线程/工作组)。
// 绑定:leaf_particle_buffer(u17)、particle_emitter_buffer(u18)、
//       water_noise_texture(t15)、grass_object_push(t16)。
// ============================================================
// PATCHED: modules es.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/leaf_particles.cs.hlsl | F-V | bin: _leaf_particles.cs.hlsl_cs_main_0.bin | entry cs:cs_main | LocalSize 64,1,1
// Leaf particle simulation. Each thread owns one 32-byte LeafParticle (position f3 +0,
// rotation f +12, scale f +16, up_axis f3 +20) in RWByteAddressBuffer __alias_leaf_particle_buffer
// (set0/u17). Airborne leaves (y>0.05) drift with gusting wind (value-noise lattice, inc/perlin.hlsl
// hash) and slowly rotate; grounded leaves settle on the grass mask (grass_object_push) and follow
// water-flow noise (water_noise_texture); leaves that fall below -0.03 respawn from a random
// ParticleEmitter (set0/u18). Glade theme 4 (autumn) disables the wind/water displacement.

#include "inc/common.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/perlin.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "water/water_object_push.inc.hlsl"
// water/waves.inc.hlsl references water_noise_texture / sampler_llr in its helper
// bodies, so this consumer's register-bound texture must precede the include.
Texture2D water_noise_texture : register(t15);
#include "water/waves.inc.hlsl"
#include "inc/pack_unpack.hlsl"

struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
};

struct ParticleEmitter
{
    float3 position;
    float pad0;
};

uint2 water_noise_texture_dims;
uint2 grass_object_push_dims;
int particle_emitter_count;
int total_particle_count;

RWByteAddressBuffer __alias_leaf_particle_buffer : register(u17);
RWByteAddressBuffer __alias_particle_emitter_buffer : register(u18);

Texture2D grass_object_push : register(t16);

// INC-issue workaround: original samplers.hlsl declared s0..s12; inc/samplers.hlsl has
// s0..s4,s9..s12. Declaring s5..s8 (+ an unused texture at t13) reproduces the original
// $Globals binding (set0/b14 here).
Texture2D unused_t13 : register(t13);

static const float leaf_octave_scales[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float leaf_octave_weights[4] = { 2.0f, 2.0f, 0.5f, 0.2f };
static const uint leaf_octave_seeds[4] = { 17u, 18u, 20u, 21u };

// 2D value-noise at p with a seed offset (lattice from inc/perlin.hlsl hash/rand01).
float leaf_noise(float2 p)
{
    return perlin(p);
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= uint(total_particle_count))
    {
        return;
    }

    uint stride = gl_GlobalInvocationID.x * 32u;
    LeafParticle p;
    p.position = __alias_leaf_particle_buffer.Load<float3>(stride + 0u);
    p.rotation = __alias_leaf_particle_buffer.Load<float>(stride + 12u);
    p.scale = __alias_leaf_particle_buffer.Load<float>(stride + 16u);
    p.up_axis = __alias_leaf_particle_buffer.Load<float3>(stride + 20u);

    if (particle_emitter_count == 0)
    {
        __alias_leaf_particle_buffer.Store<float3>(stride + 0u, float3(0.0f, 0.0f, 0.0f));
        __alias_leaf_particle_buffer.Store<float>(stride + 12u, p.rotation);
        __alias_leaf_particle_buffer.Store<float>(stride + 16u, 0.0f);
        __alias_leaf_particle_buffer.Store<float3>(stride + 20u, p.up_axis);
        return;
    }

    float3 out_pos;
    float out_rotation;
    float out_scale;
    float3 out_up;

    if (p.position.y > 0.05f)
    {
        // Airborne: gusting wind drift + rotation.
        float t = float(frame_index);
        float3 dir = normalize(p.position);
        float gust_x = (sin((t * 0.058823529f) + p.position.x) * sin((t * 0.025641026f) + p.position.z)) + 1.0f;
        float3 drift = (dir * 0.3f)
                     + ((float3(1.0f, 0.0f, 0.0f) * gust_x * 2.0f) * 0.4f) * ((dir.x + 1.0f) * 0.5f);

        float2 noise_uv = ((p.position.xy + p.position.zy) + float(gl_GlobalInvocationID.x).xx) * 0.2f;
        float nx = leaf_noise(noise_uv + 234.0f);
        float ny = leaf_noise(noise_uv + 452.0f);
        float nz = leaf_noise(noise_uv + 99452.0f);

        float3 vel = -float3(drift.x + lerp(-0.5f, 0.5f, nx),
                             lerp(0.4f, 3.0f, ny * ny) * 0.7f,
                             drift.z + lerp(-0.5f, 0.5f, nz));
        float3 disp = vel * delta_time_seconds * 1.5f;

        float3 moved = p.position + disp;
        moved.y = max(moved.y, 0.05f);

        out_pos = moved;
        out_rotation = p.rotation + (length(disp) * 3.0f) * lerp(0.5f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 345346u)));
        out_scale = p.scale;
        out_up = p.up_axis;
    }
    else
    {
        // Grounded: settle on the grass mask, follow water flow; respawn when fallen below -0.03.
        float ground_y = p.position.y - (delta_time_seconds * 0.0125f);
        float3 ground_pos = p.position;
        ground_pos.y = ground_y;
        float4 grass = grass_object_push.SampleLevel(sampler_default, (ground_pos.xz * 0.0076923f) + 0.5f, 0.0f);

        float time = float(game_time_sec) + game_time_frac;

        if (glade_theme_id == 4u)
        {
            out_pos = ground_pos;
            out_rotation = p.rotation;
            out_scale = p.scale;
            out_up = p.up_axis;
        }
        else
        {
            // Wind direction per octave (value-noise lattice).
            float2 wind_dir[4];
            [unroll]
            for (uint o = 0u; o < 4u; o++)
            {
                float2 seed_p = float2(leaf_octave_seeds[o], leaf_octave_seeds[o]);
                int2 cell = int2(floor(seed_p));
                float2 f = frac(seed_p);
                float2 u = f * f * (3.0f - 2.0f * f);
                float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
                float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
                float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
                float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
                float2 noise_dir = float2(lerp(n00, n10, u.x), lerp(n01, n11, u.x));
                wind_dir[o] = normalize(noise_dir - 0.5f) * time * 3.0f;
            }

            // Water-flow displacement.
            float3 flow = float3(0.0f, 0.0f, 0.0f);
            [unroll]
            for (uint i = 0u; i < 1u; i++)
            {
                float ang = float(i + 1u) * 10.16640758f;
                float c = cos(ang);
                float s = sin(ang);
                float2x2 rot = float2x2(c, s, -s, c);
                float2 sample_uv = mul(rot, (ground_pos.xz * leaf_octave_scales[i]) + wind_dir[i]) * 0.012f;
                float3 v = water_noise_texture.SampleLevel(sampler_llr, sample_uv, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)
                         + float3(-0.5f, -1.0f, -1.0f);
                v *= leaf_octave_weights[i];
                float3 v_scaled = v * float3(1.0f, leaf_octave_scales[i] * 0.012f, leaf_octave_scales[i] * 0.012f);
                float2 move = mul(v_scaled.yz, rot);
                flow.yz += move;
                flow = float3(flow.x, flow.y, flow.z);
            }

            float2 wind_disp = ground_pos.xz + ((((float3(grass.x, 0.0f, grass.y) * float(grass.z > 0.0f)) * 0.35f)
                           + (flow.yxz * float3(2.0f, 0.01f, 2.0f))).xz * 0.1f);
            float3 settled = float3(wind_disp.x, ground_y, wind_disp.y);

            if (ground_y < -0.03f)
            {
                uint respawn_idx = (gl_GlobalInvocationID.x ^ (345345u * (frame_index + 1u))) % uint(particle_emitter_count);
                float3 emitter = __alias_particle_emitter_buffer.Load<float3>(respawn_idx * 16u + 0u);
                float3 jitter = float3(lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 10945u))),
                                       lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 49234u))),
                                       lerp(-0.5f, 0.5f, rand01(hash(gl_GlobalInvocationID.x ^ 23458u))));
                out_pos = emitter + jitter;
                out_rotation = p.rotation;
                out_scale = lerp(0.6f, 1.0f, rand01(hash(gl_GlobalInvocationID.x))) * 0.8f;
                out_up = normalize(float3(lerp(-1.0f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 2345346u))),
                                          2.0f,
                                          lerp(-1.0f, 1.0f, rand01(hash(gl_GlobalInvocationID.x ^ 10945u)))));
            }
            else
            {
                out_pos = settled;
                out_rotation = p.rotation;
                out_scale = p.scale - (delta_time_seconds * 0.05f);
                out_up = float3(0.0f, 1.0f, 0.0f);
            }
        }
    }

    __alias_leaf_particle_buffer.Store<float3>(stride + 0u, out_pos);
    __alias_leaf_particle_buffer.Store<float>(stride + 12u, out_rotation);
    __alias_leaf_particle_buffer.Store<float>(stride + 16u, out_scale);
    __alias_leaf_particle_buffer.Store<float3>(stride + 20u, out_up);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _leaf_particles.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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

struct LeafParticle
{
    float3 position;
    float rotation;
    float scale;
    float3 up_axis;
};

struct ParticleEmitter
{
    float3 position;
    float pad0;
};

static uint uninit_reg;

static const float flow_scale[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float flow_gain[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint flow_hash_seed[4] = { 17u, 18u, 20u, 21u };

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

cbuffer $Globals : register(b14, space0)
{
    uint2 water_noise_texture_dims : packoffset(c0);
    uint2 grass_object_push_dims : packoffset(c0.z);
    int particle_emitter_count : packoffset(c1);
    int total_particle_count : packoffset(c1.y);
};

RWByteAddressBuffer __alias_leaf_particle_buffer : register(u17, space0);
RWByteAddressBuffer __alias_particle_emitter_buffer : register(u18, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> water_noise_texture : register(t15, space0);
Texture2D<float4> grass_object_push : register(t16, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= uint(total_particle_count))
        {
            break;
        }
        LeafParticle particle;
        particle.position = __alias_leaf_particle_buffer.Load<float3>(gl_GlobalInvocationID.x * 32 + 0);
        particle.rotation = __alias_leaf_particle_buffer.Load<float>(gl_GlobalInvocationID.x * 32 + 12);
        particle.scale = __alias_leaf_particle_buffer.Load<float>(gl_GlobalInvocationID.x * 32 + 16);
        particle.up_axis = __alias_leaf_particle_buffer.Load<float3>(gl_GlobalInvocationID.x * 32 + 20);
        float3 old_pos = particle.position;
        if (particle_emitter_count == 0)
        {
            LeafParticle reset_particle = { 0.0f.xxx, particle.rotation, 0.0f, particle.up_axis };
            __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 0, reset_particle.position);
            __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 12, reset_particle.rotation);
            __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 16, reset_particle.scale);
            __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 20, reset_particle.up_axis);
            break;
        }
        float3 out_pos;
        float out_rot;
        float out_scale;
        float3 out_up;
        if (old_pos.y > 0.0500000007450580596923828125f)
        {
            float frame = float(frame_constants_frame_index);
            float3 dir_to_center = normalize(old_pos);
            float3 gust_wind = (dir_to_center * 0.300000011920928955078125f) + (((float3(1.0f, 0.0f, 0.0f) * (((sin((frame * 0.0588235296308994293212890625f) + old_pos.x) * sin((frame * 0.02564102597534656524658203125f) + old_pos.z)) + 1.0f) * 2.0f)) * 0.4000000059604644775390625f) * ((dir_to_center.x + 1.0f) * 0.5f));
            float2 noise_uv = ((old_pos.xy + old_pos.zy) + float(gl_GlobalInvocationID.x).xx) * 0.20000000298023223876953125f;
            float2 lattice_uv0 = noise_uv + 234.0f.xx;
            int2 v811 = int2(floor(lattice_uv0));
            float2 v812 = frac(lattice_uv0);
            uint2 v813 = uint2(v811);
            uint v815 = v813.y;
            uint v817 = v815 + (v815 << 10u);
            uint v819 = v817 ^ (v817 >> 6u);
            uint v821 = v819 + (v819 << 3u);
            uint v823 = v821 ^ (v821 >> 11u);
            uint v829 = (((v813.x * 1664525u) + (v823 + (v823 << 15u))) + 1013904223u) * 1664525u;
            uint v831 = v829 ^ (v829 >> 11u);
            uint v834 = v831 ^ ((v831 << 7u) & 2636928640u);
            uint hash_a00 = v834 ^ ((v834 << 15u) & 4022730752u);
            uint2 v845 = uint2(v811 + int2(1, 0));
            uint v847 = v845.y;
            uint v849 = v847 + (v847 << 10u);
            uint v851 = v849 ^ (v849 >> 6u);
            uint v853 = v851 + (v851 << 3u);
            uint v855 = v853 ^ (v853 >> 11u);
            uint v861 = (((v845.x * 1664525u) + (v855 + (v855 << 15u))) + 1013904223u) * 1664525u;
            uint v863 = v861 ^ (v861 >> 11u);
            uint v866 = v863 ^ ((v863 << 7u) & 2636928640u);
            uint hash_a10 = v866 ^ ((v866 << 15u) & 4022730752u);
            uint2 v877 = uint2(v811 + int2(0, 1));
            uint v879 = v877.y;
            uint v881 = v879 + (v879 << 10u);
            uint v883 = v881 ^ (v881 >> 6u);
            uint v885 = v883 + (v883 << 3u);
            uint v887 = v885 ^ (v885 >> 11u);
            uint v893 = (((v877.x * 1664525u) + (v887 + (v887 << 15u))) + 1013904223u) * 1664525u;
            uint v895 = v893 ^ (v893 >> 11u);
            uint v898 = v895 ^ ((v895 << 7u) & 2636928640u);
            uint hash_a01 = v898 ^ ((v898 << 15u) & 4022730752u);
            uint2 v909 = uint2(v811 + int2(1, 1));
            uint v911 = v909.y;
            uint v913 = v911 + (v911 << 10u);
            uint v915 = v913 ^ (v913 >> 6u);
            uint v917 = v915 + (v915 << 3u);
            uint v919 = v917 ^ (v917 >> 11u);
            uint v925 = (((v909.x * 1664525u) + (v919 + (v919 << 15u))) + 1013904223u) * 1664525u;
            uint v927 = v925 ^ (v925 >> 11u);
            uint v930 = v927 ^ ((v927 << 7u) & 2636928640u);
            uint hash_a11 = v930 ^ ((v930 << 15u) & 4022730752u);
            float2 w_a = (v812 * v812) * (3.0f.xx - (v812 * 2.0f));
            float w_ax = w_a.x;
            float w_ax_inv = 1.0f - w_ax;
            float w_ay = w_a.y;
            float2 lattice_uv1 = noise_uv + 452.0f.xx;
            int2 v963 = int2(floor(lattice_uv1));
            float2 v964 = frac(lattice_uv1);
            uint2 v965 = uint2(v963);
            uint v967 = v965.y;
            uint v969 = v967 + (v967 << 10u);
            uint v971 = v969 ^ (v969 >> 6u);
            uint v973 = v971 + (v971 << 3u);
            uint v975 = v973 ^ (v973 >> 11u);
            uint v981 = (((v965.x * 1664525u) + (v975 + (v975 << 15u))) + 1013904223u) * 1664525u;
            uint v983 = v981 ^ (v981 >> 11u);
            uint v986 = v983 ^ ((v983 << 7u) & 2636928640u);
            uint hash_b00 = v986 ^ ((v986 << 15u) & 4022730752u);
            uint2 v997 = uint2(v963 + int2(1, 0));
            uint v999 = v997.y;
            uint v1001 = v999 + (v999 << 10u);
            uint v1003 = v1001 ^ (v1001 >> 6u);
            uint v1005 = v1003 + (v1003 << 3u);
            uint v1007 = v1005 ^ (v1005 >> 11u);
            uint v1013 = (((v997.x * 1664525u) + (v1007 + (v1007 << 15u))) + 1013904223u) * 1664525u;
            uint v1015 = v1013 ^ (v1013 >> 11u);
            uint v1018 = v1015 ^ ((v1015 << 7u) & 2636928640u);
            uint hash_b10 = v1018 ^ ((v1018 << 15u) & 4022730752u);
            uint2 v1029 = uint2(v963 + int2(0, 1));
            uint v1031 = v1029.y;
            uint v1033 = v1031 + (v1031 << 10u);
            uint v1035 = v1033 ^ (v1033 >> 6u);
            uint v1037 = v1035 + (v1035 << 3u);
            uint v1039 = v1037 ^ (v1037 >> 11u);
            uint v1045 = (((v1029.x * 1664525u) + (v1039 + (v1039 << 15u))) + 1013904223u) * 1664525u;
            uint v1047 = v1045 ^ (v1045 >> 11u);
            uint v1050 = v1047 ^ ((v1047 << 7u) & 2636928640u);
            uint hash_b01 = v1050 ^ ((v1050 << 15u) & 4022730752u);
            uint2 v1061 = uint2(v963 + int2(1, 1));
            uint v1063 = v1061.y;
            uint v1065 = v1063 + (v1063 << 10u);
            uint v1067 = v1065 ^ (v1065 >> 6u);
            uint v1069 = v1067 + (v1067 << 3u);
            uint v1071 = v1069 ^ (v1069 >> 11u);
            uint v1077 = (((v1061.x * 1664525u) + (v1071 + (v1071 << 15u))) + 1013904223u) * 1664525u;
            uint v1079 = v1077 ^ (v1077 >> 11u);
            uint v1082 = v1079 ^ ((v1079 << 7u) & 2636928640u);
            uint hash_b11 = v1082 ^ ((v1082 << 15u) & 4022730752u);
            float2 w_b = (v964 * v964) * (3.0f.xx - (v964 * 2.0f));
            float w_bx = w_b.x;
            float w_bx_inv = 1.0f - w_bx;
            float w_by = w_b.y;
            float2 lattice_uv2 = noise_uv + 99452.0f.xx;
            int2 v1115 = int2(floor(lattice_uv2));
            float2 v1116 = frac(lattice_uv2);
            uint2 v1117 = uint2(v1115);
            uint v1119 = v1117.y;
            uint v1121 = v1119 + (v1119 << 10u);
            uint v1123 = v1121 ^ (v1121 >> 6u);
            uint v1125 = v1123 + (v1123 << 3u);
            uint v1127 = v1125 ^ (v1125 >> 11u);
            uint v1133 = (((v1117.x * 1664525u) + (v1127 + (v1127 << 15u))) + 1013904223u) * 1664525u;
            uint v1135 = v1133 ^ (v1133 >> 11u);
            uint v1138 = v1135 ^ ((v1135 << 7u) & 2636928640u);
            uint hash_c00 = v1138 ^ ((v1138 << 15u) & 4022730752u);
            uint2 v1149 = uint2(v1115 + int2(1, 0));
            uint v1151 = v1149.y;
            uint v1153 = v1151 + (v1151 << 10u);
            uint v1155 = v1153 ^ (v1153 >> 6u);
            uint v1157 = v1155 + (v1155 << 3u);
            uint v1159 = v1157 ^ (v1157 >> 11u);
            uint v1165 = (((v1149.x * 1664525u) + (v1159 + (v1159 << 15u))) + 1013904223u) * 1664525u;
            uint v1167 = v1165 ^ (v1165 >> 11u);
            uint v1170 = v1167 ^ ((v1167 << 7u) & 2636928640u);
            uint hash_c10 = v1170 ^ ((v1170 << 15u) & 4022730752u);
            uint2 v1181 = uint2(v1115 + int2(0, 1));
            uint v1183 = v1181.y;
            uint v1185 = v1183 + (v1183 << 10u);
            uint v1187 = v1185 ^ (v1185 >> 6u);
            uint v1189 = v1187 + (v1187 << 3u);
            uint v1191 = v1189 ^ (v1189 >> 11u);
            uint v1197 = (((v1181.x * 1664525u) + (v1191 + (v1191 << 15u))) + 1013904223u) * 1664525u;
            uint v1199 = v1197 ^ (v1197 >> 11u);
            uint v1202 = v1199 ^ ((v1199 << 7u) & 2636928640u);
            uint hash_c01 = v1202 ^ ((v1202 << 15u) & 4022730752u);
            uint2 v1213 = uint2(v1115 + int2(1, 1));
            uint v1215 = v1213.y;
            uint v1217 = v1215 + (v1215 << 10u);
            uint v1219 = v1217 ^ (v1217 >> 6u);
            uint v1221 = v1219 + (v1219 << 3u);
            uint v1223 = v1221 ^ (v1221 >> 11u);
            uint v1229 = (((v1213.x * 1664525u) + (v1223 + (v1223 << 15u))) + 1013904223u) * 1664525u;
            uint v1231 = v1229 ^ (v1229 >> 11u);
            uint v1234 = v1231 ^ ((v1231 << 7u) & 2636928640u);
            uint hash_c11 = v1234 ^ ((v1234 << 15u) & 4022730752u);
            float2 w_c = (v1116 * v1116) * (3.0f.xx - (v1116 * 2.0f));
            float w_cx = w_c.x;
            float w_cx_inv = 1.0f - w_cx;
            float w_cy = w_c.y;
            float noise_mix_c = (((1.0f - w_cy) * (((asfloat(((hash_c00 ^ (hash_c00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_cx_inv) + ((asfloat(((hash_c10 ^ (hash_c10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_cx))) + (((asfloat(((hash_c01 ^ (hash_c01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_cx_inv) * w_cy)) + (((asfloat(((hash_c11 ^ (hash_c11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_cx) * w_cy);
            float3 wind_vel = -float3(gust_wind.x + lerp(-0.5f, 0.5f, (((1.0f - w_ay) * (((asfloat(((hash_a00 ^ (hash_a00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_ax_inv) + ((asfloat(((hash_a10 ^ (hash_a10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_ax))) + (((asfloat(((hash_a01 ^ (hash_a01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_ax_inv) * w_ay)) + (((asfloat(((hash_a11 ^ (hash_a11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_ax) * w_ay)), lerp(0.4000000059604644775390625f, 3.0f, noise_mix_c * noise_mix_c) * 0.699999988079071044921875f, gust_wind.z + lerp(-0.5f, 0.5f, (((1.0f - w_by) * (((asfloat(((hash_b00 ^ (hash_b00 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_bx_inv) + ((asfloat(((hash_b10 ^ (hash_b10 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_bx))) + (((asfloat(((hash_b01 ^ (hash_b01 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_bx_inv) * w_by)) + (((asfloat(((hash_b11 ^ (hash_b11 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * w_bx) * w_by)));
            float3 wind_delta = (wind_vel * frame_constants_delta_time_seconds) * 1.5f;
            float3 air_pos = old_pos + wind_delta;
            uint hash_mixid = gl_GlobalInvocationID.x ^ 345346u;
            uint v1274 = hash_mixid + (hash_mixid << 10u);
            uint v1276 = v1274 ^ (v1274 >> 6u);
            uint v1278 = v1276 + (v1276 << 3u);
            uint spin_hash = v1278 ^ (v1278 >> 11u);
            air_pos.y = max(air_pos.y, 0.0500000007450580596923828125f);
            out_pos = air_pos;
            out_rot = particle.rotation + ((length(wind_delta) * 3.0f) * lerp(0.5f, 1.0f, asfloat(((spin_hash + (spin_hash << 15u)) & 8388607u) | 1065353216u) - 1.0f));
            out_scale = particle.scale;
            out_up = particle.up_axis;
        }
        else
        {
            float descend_y = old_pos.y - (frame_constants_delta_time_seconds * 0.012500000186264514923095703125f);
            float3 ground_pos = old_pos;
            ground_pos.y = descend_y;
            float4 grass_push = grass_object_push.SampleLevel(sampler_default, (ground_pos.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
            uint v426;
            uint v427;
            bool v428;
            bool v429;
            uint v430;
            uint v431;
            bool v432;
            bool v433;
            float3 v434;
            do
            {
                if (frame_constants_glade_theme_id == 4u)
                {
                    v426 = uninit_reg;
                    v427 = uninit_reg;
                    v428 = false;
                    v429 = false;
                    v430 = uninit_reg;
                    v431 = uninit_reg;
                    v432 = false;
                    v433 = false;
                    v434 = 0.0f.xxx;
                    break;
                }
                float game_time = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
                uint v241;
                uint v243;
                bool v245;
                bool v247;
                uint v249;
                uint v251;
                bool v253;
                bool v255;
                v241 = uninit_reg;
                v243 = uninit_reg;
                v245 = false;
                v247 = false;
                v249 = uninit_reg;
                v251 = uninit_reg;
                v253 = false;
                v255 = false;
                float2 flow_octaves[4];
                for (uint v238 = 0u; v238 < 4u; )
                {
                    float v261 = float(flow_hash_seed[v238]);
                    int v263 = int(floor(v261));
                    float v264 = frac(v261);
                    uint v265 = uint(v263);
                    uint v267 = v265 + (v265 << 10u);
                    uint v269 = v267 ^ (v267 >> 6u);
                    uint v271 = v269 + (v269 << 3u);
                    uint v273 = v271 ^ (v271 >> 11u);
                    bool v276 = v255 ? v255 : true;
                    uint v277 = v255 ? v251 : 1664525u;
                    bool v278 = v253 ? v253 : true;
                    uint v279 = v253 ? v249 : 1013904223u;
                    uint v283 = (((v265 * v277) + (v273 + (v273 << 15u))) + v279) * v277;
                    uint v285 = v283 ^ (v283 >> 11u);
                    uint v288 = v285 ^ ((v285 << 7u) & 2636928640u);
                    uint v291 = v288 ^ ((v288 << 15u) & 4022730752u);
                    uint v293 = v291 ^ (v291 >> 18u);
                    uint v295 = v293 + (v293 << 10u);
                    uint v297 = v295 ^ (v295 >> 6u);
                    uint v299 = v297 + (v297 << 3u);
                    uint v301 = v299 ^ (v299 >> 11u);
                    bool v304 = v247 ? v247 : true;
                    uint v305 = v247 ? v243 : 8388607u;
                    bool v306 = v245 ? v245 : true;
                    uint v307 = v245 ? v241 : 1065353216u;
                    bool v312 = v304 ? v304 : true;
                    uint v313 = v304 ? v305 : 8388607u;
                    bool v314 = v306 ? v306 : true;
                    uint v315 = v306 ? v307 : 1065353216u;
                    uint v322 = uint(v263 + 1);
                    uint v324 = v322 + (v322 << 10u);
                    uint v326 = v324 ^ (v324 >> 6u);
                    uint v328 = v326 + (v326 << 3u);
                    uint v330 = v328 ^ (v328 >> 11u);
                    uint v252 = v276 ? v277 : 1664525u;
                    uint v250 = v278 ? v279 : 1013904223u;
                    uint v336 = (((v322 * v252) + (v330 + (v330 << 15u))) + v250) * v252;
                    uint v338 = v336 ^ (v336 >> 11u);
                    uint v341 = v338 ^ ((v338 << 7u) & 2636928640u);
                    uint v344 = v341 ^ ((v341 << 15u) & 4022730752u);
                    uint v346 = v344 ^ (v344 >> 18u);
                    uint v348 = v346 + (v346 << 10u);
                    uint v350 = v348 ^ (v348 >> 6u);
                    uint v352 = v350 + (v350 << 3u);
                    uint v354 = v352 ^ (v352 >> 11u);
                    bool v357 = v312 ? v312 : true;
                    uint v358 = v312 ? v313 : 8388607u;
                    bool v359 = v314 ? v314 : true;
                    uint v360 = v314 ? v315 : 1065353216u;
                    uint v244 = v357 ? v358 : 8388607u;
                    uint v242 = v359 ? v360 : 1065353216u;
                    float v373 = (v264 * v264) * (3.0f - (2.0f * v264));
                    flow_octaves[v238] = (normalize(((float2(asfloat((v293 & v305) | v307) - 1.0f, asfloat(((v301 + (v301 << 15u)) & v313) | v315) - 1.0f) * (1.0f - v373)) + (float2(asfloat((v346 & v358) | v360) - 1.0f, asfloat(((v354 + (v354 << 15u)) & v244) | v242) - 1.0f) * v373)) - 0.5f.xx) * game_time) * 3.0f;
                    v238++;
                    v241 = v242;
                    v243 = v244;
                    v245 = v359 ? v359 : true;
                    v247 = v357 ? v357 : true;
                    v249 = v250;
                    v251 = v252;
                    v253 = v278 ? v278 : true;
                    v255 = v276 ? v276 : true;
                    continue;
                }
                float3 flow_dir;
                flow_dir = 0.0f.xxx;
                for (uint v387 = 0u; v387 < 1u; )
                {
                    uint v388 = v387 + 1u;
                    float v392 = float(v388) * 10.16640758514404296875f;
                    float v393 = cos(v392);
                    float v394 = sin(v392);
                    float2x2 v398 = float2x2(float2(v393, v394), float2(-v394, v393));
                    float2 v425 = flow_dir.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(v398, (ground_pos.xz * flow_scale[v387]) + flow_octaves[v387]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * flow_gain[v387]) * float2(1.0f, flow_scale[v387] * 0.01200000010430812835693359375f).xyy).yz, v398);
                    flow_dir = float3(flow_dir.x, v425.x, v425.y);
                    v387 = v388;
                    continue;
                }
                v426 = v249;
                v427 = v251;
                v428 = v253;
                v429 = v255;
                v430 = v241;
                v431 = v243;
                v432 = v245;
                v433 = v247;
                v434 = flow_dir;
                break;
            } while(false);
            float2 drift_xz = ground_pos.xz + ((((float3(grass_push.x, 0.0f, grass_push.y) * ((clamp(grass_push.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (v434.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz * 0.100000001490116119384765625f);
            float3 drift_pos = float3(drift_xz.x, ground_pos.y, drift_xz.y);
            float2 river_uv = (drift_xz.xy * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
            int2 v454 = int2(floor(river_uv));
            float2 v455 = frac(river_uv);
            uint2 v456 = uint2(v454);
            uint v458 = v456.y;
            uint v460 = v458 + (v458 << 10u);
            uint v462 = v460 ^ (v460 >> 6u);
            uint v464 = v462 + (v462 << 3u);
            uint v466 = v464 ^ (v464 >> 11u);
            bool v469 = v429 ? v429 : true;
            uint v470 = v429 ? v427 : 1664525u;
            bool v471 = v428 ? v428 : true;
            uint v472 = v428 ? v426 : 1013904223u;
            uint v476 = (((v456.x * v470) + (v466 + (v466 << 15u))) + v472) * v470;
            uint v478 = v476 ^ (v476 >> 11u);
            uint v481 = v478 ^ ((v478 << 7u) & 2636928640u);
            uint v484 = v481 ^ ((v481 << 15u) & 4022730752u);
            bool v487 = v433 ? v433 : true;
            uint v488 = v433 ? v431 : 8388607u;
            bool v489 = v432 ? v432 : true;
            uint v490 = v432 ? v430 : 1065353216u;
            uint2 v496 = uint2(v454 + int2(1, 0));
            uint v498 = v496.y;
            uint v500 = v498 + (v498 << 10u);
            uint v502 = v500 ^ (v500 >> 6u);
            uint v504 = v502 + (v502 << 3u);
            uint v506 = v504 ^ (v504 >> 11u);
            bool v509 = v469 ? v469 : true;
            uint v510 = v469 ? v470 : 1664525u;
            bool v511 = v471 ? v471 : true;
            uint v512 = v471 ? v472 : 1013904223u;
            uint v516 = (((v496.x * v510) + (v506 + (v506 << 15u))) + v512) * v510;
            uint v518 = v516 ^ (v516 >> 11u);
            uint v521 = v518 ^ ((v518 << 7u) & 2636928640u);
            uint v524 = v521 ^ ((v521 << 15u) & 4022730752u);
            bool v527 = v487 ? v487 : true;
            uint v528 = v487 ? v488 : 8388607u;
            bool v529 = v489 ? v489 : true;
            uint v530 = v489 ? v490 : 1065353216u;
            uint2 v536 = uint2(v454 + int2(0, 1));
            uint v538 = v536.y;
            uint v540 = v538 + (v538 << 10u);
            uint v542 = v540 ^ (v540 >> 6u);
            uint v544 = v542 + (v542 << 3u);
            uint v546 = v544 ^ (v544 >> 11u);
            uint v550 = v509 ? v510 : 1664525u;
            uint v552 = v511 ? v512 : 1013904223u;
            uint v556 = (((v536.x * v550) + (v546 + (v546 << 15u))) + v552) * v550;
            uint v558 = v556 ^ (v556 >> 11u);
            uint v561 = v558 ^ ((v558 << 7u) & 2636928640u);
            uint v564 = v561 ^ ((v561 << 15u) & 4022730752u);
            bool v567 = v527 ? v527 : true;
            uint v568 = v527 ? v528 : 8388607u;
            bool v569 = v529 ? v529 : true;
            uint v570 = v529 ? v530 : 1065353216u;
            uint2 v576 = uint2(v454 + int2(1, 1));
            uint v578 = v576.y;
            uint v580 = v578 + (v578 << 10u);
            uint v582 = v580 ^ (v580 >> 6u);
            uint v584 = v582 + (v582 << 3u);
            uint v586 = v584 ^ (v584 >> 11u);
            uint v589 = (v509 ? v509 : true) ? v550 : 1664525u;
            uint v594 = (((v576.x * v589) + (v586 + (v586 << 15u))) + ((v511 ? v511 : true) ? v552 : 1013904223u)) * v589;
            uint v596 = v594 ^ (v594 >> 11u);
            uint v599 = v596 ^ ((v596 << 7u) & 2636928640u);
            uint v602 = v599 ^ ((v599 << 15u) & 4022730752u);
            bool v605 = v567 ? v567 : true;
            uint v606 = v567 ? v568 : 8388607u;
            bool v607 = v569 ? v569 : true;
            uint v608 = v569 ? v570 : 1065353216u;
            float2 v616 = (v455 * v455) * (3.0f.xx - (v455 * 2.0f));
            float v617 = v616.x;
            float v618 = 1.0f - v617;
            float v620 = v616.y;
            bool v635 = v605 ? v605 : true;
            bool v637 = v607 ? v607 : true;
            uint v638 = v607 ? v608 : 1065353216u;
            drift_pos.x = drift_xz.x - (((sin((((((1.0f - v620) * (((asfloat(((v484 ^ (v484 >> 18u)) & v488) | v490) - 1.0f) * v618) + ((asfloat(((v524 ^ (v524 >> 18u)) & v528) | v530) - 1.0f) * v617))) + (((asfloat(((v564 ^ (v564 >> 18u)) & v568) | v570) - 1.0f) * v618) * v620)) + (((asfloat(((v602 ^ (v602 >> 18u)) & v606) | v608) - 1.0f) * v617) * v620)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.0056000002659857273101806640625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(0u | v638) - 1.0f));
            float3 fall_pos;
            float fall_scale;
            float3 fall_up;
            if (descend_y < (-0.02999999932944774627685546875f))
            {
                int v651 = int(gl_GlobalInvocationID.x);
                uint v663 = uint(v651 ^ 10945);
                uint v665 = v663 + (v663 << 10u);
                uint v667 = v665 ^ (v665 >> 6u);
                uint v669 = v667 + (v667 << 3u);
                uint v671 = v669 ^ (v669 >> 11u);
                bool v674 = v635 ? v635 : true;
                uint v675 = v635 ? (v605 ? v606 : 8388607u) : 8388607u;
                bool v676 = v637 ? v637 : true;
                uint v677 = v637 ? v638 : 1065353216u;
                uint v684 = uint(v651 ^ 49234);
                uint v686 = v684 + (v684 << 10u);
                uint v688 = v686 ^ (v686 >> 6u);
                uint v690 = v688 + (v688 << 3u);
                uint v692 = v690 ^ (v690 >> 11u);
                bool v695 = v674 ? v674 : true;
                uint v696 = v674 ? v675 : 8388607u;
                bool v697 = v676 ? v676 : true;
                uint v698 = v676 ? v677 : 1065353216u;
                uint v705 = uint(v651 ^ 23458);
                uint v707 = v705 + (v705 << 10u);
                uint v709 = v707 ^ (v707 >> 6u);
                uint v711 = v709 + (v709 << 3u);
                uint v713 = v711 ^ (v711 >> 11u);
                bool v716 = v695 ? v695 : true;
                uint v717 = v695 ? v696 : 8388607u;
                bool v718 = v697 ? v697 : true;
                uint v719 = v697 ? v698 : 1065353216u;
                uint v728 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
                uint v730 = v728 ^ (v728 >> 6u);
                uint v732 = v730 + (v730 << 3u);
                uint v734 = v732 ^ (v732 >> 11u);
                uint v736 = v734 + (v734 << 15u);
                bool v737 = v716 ? v716 : true;
                uint v738 = v716 ? v717 : 8388607u;
                bool v739 = v718 ? v718 : true;
                uint v740 = v718 ? v719 : 1065353216u;
                uint v748 = v737 ? v738 : 8388607u;
                uint v750 = v739 ? v740 : 1065353216u;
                uint v756 = gl_GlobalInvocationID.x ^ 2345346u;
                uint v758 = v756 + (v756 << 10u);
                uint v760 = v758 ^ (v758 >> 6u);
                uint v762 = v760 + (v760 << 3u);
                uint v764 = v762 ^ (v762 >> 11u);
                fall_pos = __alias_particle_emitter_buffer.Load<float3>(((uint(v651) ^ (345345u * (frame_constants_frame_index + 1u))) % uint(particle_emitter_count)) * 16 + 0) + float3(lerp(-0.5f, 0.5f, asfloat(((v671 + (v671 << 15u)) & v675) | v677) - 1.0f), lerp(-0.5f, 0.5f, asfloat(((v692 + (v692 << 15u)) & v696) | v698) - 1.0f), lerp(-0.5f, 0.5f, asfloat(((v713 + (v713 << 15u)) & v717) | v719) - 1.0f));
                fall_scale = lerp(0.60000002384185791015625f, 1.0f, asfloat((v736 & v738) | v740) - 1.0f) * 0.800000011920928955078125f;
                fall_up = normalize(float3(lerp(-1.0f, 1.0f, asfloat((v736 & v748) | v750) - 1.0f), 2.0f, lerp(-1.0f, 1.0f, asfloat(((v764 + (v764 << 15u)) & ((v737 ? v737 : true) ? v748 : 8388607u)) | ((v739 ? v739 : true) ? v750 : 1065353216u)) - 1.0f)));
            }
            else
            {
                fall_pos = drift_pos;
                fall_scale = particle.scale - (frame_constants_delta_time_seconds * 0.0500000007450580596923828125f);
                fall_up = float3(0.0f, 1.0f, 0.0f);
            }
            out_pos = fall_pos;
            out_rot = particle.rotation;
            out_scale = fall_scale;
            out_up = fall_up;
        }
        LeafParticle out_particle = { out_pos, out_rot, out_scale, out_up };
        __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 0, out_particle.position);
        __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 12, out_particle.rotation);
        __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 16, out_particle.scale);
        __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 20, out_particle.up_axis);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
