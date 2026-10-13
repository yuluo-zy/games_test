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

static uint _149;

static const float _150[4] = { 2.0f, 4.0f, 8.0f, 12.0f };
static const float _151[4] = { 2.0f, 2.0f, 0.5f, 0.20000000298023223876953125f };
static const uint _152[4] = { 17u, 18u, 20u, 21u };

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
    uint2 _Globals_water_noise_texture_dims : packoffset(c0);
    uint2 _Globals_grass_object_push_dims : packoffset(c0.z);
    int _Globals_particle_emitter_count : packoffset(c1);
    int _Globals_total_particle_count : packoffset(c1.y);
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
        if (gl_GlobalInvocationID.x >= uint(_Globals_total_particle_count))
        {
            break;
        }
        LeafParticle _185;
        _185.position = __alias_leaf_particle_buffer.Load<float3>(gl_GlobalInvocationID.x * 32 + 0);
        _185.rotation = __alias_leaf_particle_buffer.Load<float>(gl_GlobalInvocationID.x * 32 + 12);
        _185.scale = __alias_leaf_particle_buffer.Load<float>(gl_GlobalInvocationID.x * 32 + 16);
        _185.up_axis = __alias_leaf_particle_buffer.Load<float3>(gl_GlobalInvocationID.x * 32 + 20);
        float3 _186 = _185.position;
        if (_Globals_particle_emitter_count == 0)
        {
            LeafParticle _195 = { 0.0f.xxx, _185.rotation, 0.0f, _185.up_axis };
            __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 0, _195.position);
            __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 12, _195.rotation);
            __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 16, _195.scale);
            __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 20, _195.up_axis);
            break;
        }
        float3 _1295;
        float _1296;
        float _1297;
        float3 _1298;
        if (_186.y > 0.0500000007450580596923828125f)
        {
            float _781 = float(frame_constants_frame_index);
            float3 _793 = normalize(_186);
            float3 _801 = (_793 * 0.300000011920928955078125f) + (((float3(1.0f, 0.0f, 0.0f) * (((sin((_781 * 0.0588235296308994293212890625f) + _186.x) * sin((_781 * 0.02564102597534656524658203125f) + _186.z)) + 1.0f) * 2.0f)) * 0.4000000059604644775390625f) * ((_793.x + 1.0f) * 0.5f));
            float2 _808 = ((_186.xy + _186.zy) + float(gl_GlobalInvocationID.x).xx) * 0.20000000298023223876953125f;
            float2 _809 = _808 + 234.0f.xx;
            int2 _811 = int2(floor(_809));
            float2 _812 = frac(_809);
            uint2 _813 = uint2(_811);
            uint _815 = _813.y;
            uint _817 = _815 + (_815 << 10u);
            uint _819 = _817 ^ (_817 >> 6u);
            uint _821 = _819 + (_819 << 3u);
            uint _823 = _821 ^ (_821 >> 11u);
            uint _829 = (((_813.x * 1664525u) + (_823 + (_823 << 15u))) + 1013904223u) * 1664525u;
            uint _831 = _829 ^ (_829 >> 11u);
            uint _834 = _831 ^ ((_831 << 7u) & 2636928640u);
            uint _837 = _834 ^ ((_834 << 15u) & 4022730752u);
            uint2 _845 = uint2(_811 + int2(1, 0));
            uint _847 = _845.y;
            uint _849 = _847 + (_847 << 10u);
            uint _851 = _849 ^ (_849 >> 6u);
            uint _853 = _851 + (_851 << 3u);
            uint _855 = _853 ^ (_853 >> 11u);
            uint _861 = (((_845.x * 1664525u) + (_855 + (_855 << 15u))) + 1013904223u) * 1664525u;
            uint _863 = _861 ^ (_861 >> 11u);
            uint _866 = _863 ^ ((_863 << 7u) & 2636928640u);
            uint _869 = _866 ^ ((_866 << 15u) & 4022730752u);
            uint2 _877 = uint2(_811 + int2(0, 1));
            uint _879 = _877.y;
            uint _881 = _879 + (_879 << 10u);
            uint _883 = _881 ^ (_881 >> 6u);
            uint _885 = _883 + (_883 << 3u);
            uint _887 = _885 ^ (_885 >> 11u);
            uint _893 = (((_877.x * 1664525u) + (_887 + (_887 << 15u))) + 1013904223u) * 1664525u;
            uint _895 = _893 ^ (_893 >> 11u);
            uint _898 = _895 ^ ((_895 << 7u) & 2636928640u);
            uint _901 = _898 ^ ((_898 << 15u) & 4022730752u);
            uint2 _909 = uint2(_811 + int2(1, 1));
            uint _911 = _909.y;
            uint _913 = _911 + (_911 << 10u);
            uint _915 = _913 ^ (_913 >> 6u);
            uint _917 = _915 + (_915 << 3u);
            uint _919 = _917 ^ (_917 >> 11u);
            uint _925 = (((_909.x * 1664525u) + (_919 + (_919 << 15u))) + 1013904223u) * 1664525u;
            uint _927 = _925 ^ (_925 >> 11u);
            uint _930 = _927 ^ ((_927 << 7u) & 2636928640u);
            uint _933 = _930 ^ ((_930 << 15u) & 4022730752u);
            float2 _943 = (_812 * _812) * (3.0f.xx - (_812 * 2.0f));
            float _944 = _943.x;
            float _945 = 1.0f - _944;
            float _947 = _943.y;
            float2 _961 = _808 + 452.0f.xx;
            int2 _963 = int2(floor(_961));
            float2 _964 = frac(_961);
            uint2 _965 = uint2(_963);
            uint _967 = _965.y;
            uint _969 = _967 + (_967 << 10u);
            uint _971 = _969 ^ (_969 >> 6u);
            uint _973 = _971 + (_971 << 3u);
            uint _975 = _973 ^ (_973 >> 11u);
            uint _981 = (((_965.x * 1664525u) + (_975 + (_975 << 15u))) + 1013904223u) * 1664525u;
            uint _983 = _981 ^ (_981 >> 11u);
            uint _986 = _983 ^ ((_983 << 7u) & 2636928640u);
            uint _989 = _986 ^ ((_986 << 15u) & 4022730752u);
            uint2 _997 = uint2(_963 + int2(1, 0));
            uint _999 = _997.y;
            uint _1001 = _999 + (_999 << 10u);
            uint _1003 = _1001 ^ (_1001 >> 6u);
            uint _1005 = _1003 + (_1003 << 3u);
            uint _1007 = _1005 ^ (_1005 >> 11u);
            uint _1013 = (((_997.x * 1664525u) + (_1007 + (_1007 << 15u))) + 1013904223u) * 1664525u;
            uint _1015 = _1013 ^ (_1013 >> 11u);
            uint _1018 = _1015 ^ ((_1015 << 7u) & 2636928640u);
            uint _1021 = _1018 ^ ((_1018 << 15u) & 4022730752u);
            uint2 _1029 = uint2(_963 + int2(0, 1));
            uint _1031 = _1029.y;
            uint _1033 = _1031 + (_1031 << 10u);
            uint _1035 = _1033 ^ (_1033 >> 6u);
            uint _1037 = _1035 + (_1035 << 3u);
            uint _1039 = _1037 ^ (_1037 >> 11u);
            uint _1045 = (((_1029.x * 1664525u) + (_1039 + (_1039 << 15u))) + 1013904223u) * 1664525u;
            uint _1047 = _1045 ^ (_1045 >> 11u);
            uint _1050 = _1047 ^ ((_1047 << 7u) & 2636928640u);
            uint _1053 = _1050 ^ ((_1050 << 15u) & 4022730752u);
            uint2 _1061 = uint2(_963 + int2(1, 1));
            uint _1063 = _1061.y;
            uint _1065 = _1063 + (_1063 << 10u);
            uint _1067 = _1065 ^ (_1065 >> 6u);
            uint _1069 = _1067 + (_1067 << 3u);
            uint _1071 = _1069 ^ (_1069 >> 11u);
            uint _1077 = (((_1061.x * 1664525u) + (_1071 + (_1071 << 15u))) + 1013904223u) * 1664525u;
            uint _1079 = _1077 ^ (_1077 >> 11u);
            uint _1082 = _1079 ^ ((_1079 << 7u) & 2636928640u);
            uint _1085 = _1082 ^ ((_1082 << 15u) & 4022730752u);
            float2 _1095 = (_964 * _964) * (3.0f.xx - (_964 * 2.0f));
            float _1096 = _1095.x;
            float _1097 = 1.0f - _1096;
            float _1099 = _1095.y;
            float2 _1113 = _808 + 99452.0f.xx;
            int2 _1115 = int2(floor(_1113));
            float2 _1116 = frac(_1113);
            uint2 _1117 = uint2(_1115);
            uint _1119 = _1117.y;
            uint _1121 = _1119 + (_1119 << 10u);
            uint _1123 = _1121 ^ (_1121 >> 6u);
            uint _1125 = _1123 + (_1123 << 3u);
            uint _1127 = _1125 ^ (_1125 >> 11u);
            uint _1133 = (((_1117.x * 1664525u) + (_1127 + (_1127 << 15u))) + 1013904223u) * 1664525u;
            uint _1135 = _1133 ^ (_1133 >> 11u);
            uint _1138 = _1135 ^ ((_1135 << 7u) & 2636928640u);
            uint _1141 = _1138 ^ ((_1138 << 15u) & 4022730752u);
            uint2 _1149 = uint2(_1115 + int2(1, 0));
            uint _1151 = _1149.y;
            uint _1153 = _1151 + (_1151 << 10u);
            uint _1155 = _1153 ^ (_1153 >> 6u);
            uint _1157 = _1155 + (_1155 << 3u);
            uint _1159 = _1157 ^ (_1157 >> 11u);
            uint _1165 = (((_1149.x * 1664525u) + (_1159 + (_1159 << 15u))) + 1013904223u) * 1664525u;
            uint _1167 = _1165 ^ (_1165 >> 11u);
            uint _1170 = _1167 ^ ((_1167 << 7u) & 2636928640u);
            uint _1173 = _1170 ^ ((_1170 << 15u) & 4022730752u);
            uint2 _1181 = uint2(_1115 + int2(0, 1));
            uint _1183 = _1181.y;
            uint _1185 = _1183 + (_1183 << 10u);
            uint _1187 = _1185 ^ (_1185 >> 6u);
            uint _1189 = _1187 + (_1187 << 3u);
            uint _1191 = _1189 ^ (_1189 >> 11u);
            uint _1197 = (((_1181.x * 1664525u) + (_1191 + (_1191 << 15u))) + 1013904223u) * 1664525u;
            uint _1199 = _1197 ^ (_1197 >> 11u);
            uint _1202 = _1199 ^ ((_1199 << 7u) & 2636928640u);
            uint _1205 = _1202 ^ ((_1202 << 15u) & 4022730752u);
            uint2 _1213 = uint2(_1115 + int2(1, 1));
            uint _1215 = _1213.y;
            uint _1217 = _1215 + (_1215 << 10u);
            uint _1219 = _1217 ^ (_1217 >> 6u);
            uint _1221 = _1219 + (_1219 << 3u);
            uint _1223 = _1221 ^ (_1221 >> 11u);
            uint _1229 = (((_1213.x * 1664525u) + (_1223 + (_1223 << 15u))) + 1013904223u) * 1664525u;
            uint _1231 = _1229 ^ (_1229 >> 11u);
            uint _1234 = _1231 ^ ((_1231 << 7u) & 2636928640u);
            uint _1237 = _1234 ^ ((_1234 << 15u) & 4022730752u);
            float2 _1247 = (_1116 * _1116) * (3.0f.xx - (_1116 * 2.0f));
            float _1248 = _1247.x;
            float _1249 = 1.0f - _1248;
            float _1251 = _1247.y;
            float _1261 = (((1.0f - _1251) * (((asfloat(((_1141 ^ (_1141 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1249) + ((asfloat(((_1173 ^ (_1173 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1248))) + (((asfloat(((_1205 ^ (_1205 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1249) * _1251)) + (((asfloat(((_1237 ^ (_1237 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1248) * _1251);
            float3 _1266 = -float3(_801.x + lerp(-0.5f, 0.5f, (((1.0f - _947) * (((asfloat(((_837 ^ (_837 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _945) + ((asfloat(((_869 ^ (_869 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _944))) + (((asfloat(((_901 ^ (_901 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _945) * _947)) + (((asfloat(((_933 ^ (_933 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _944) * _947)), lerp(0.4000000059604644775390625f, 3.0f, _1261 * _1261) * 0.699999988079071044921875f, _801.z + lerp(-0.5f, 0.5f, (((1.0f - _1099) * (((asfloat(((_989 ^ (_989 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1097) + ((asfloat(((_1021 ^ (_1021 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1096))) + (((asfloat(((_1053 ^ (_1053 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1097) * _1099)) + (((asfloat(((_1085 ^ (_1085 >> 18u)) & 8388607u) | 1065353216u) - 1.0f) * _1096) * _1099)));
            float3 _1270 = (_1266 * frame_constants_delta_time_seconds) * 1.5f;
            float3 _1271 = _186 + _1270;
            uint _1272 = gl_GlobalInvocationID.x ^ 345346u;
            uint _1274 = _1272 + (_1272 << 10u);
            uint _1276 = _1274 ^ (_1274 >> 6u);
            uint _1278 = _1276 + (_1276 << 3u);
            uint _1280 = _1278 ^ (_1278 >> 11u);
            _1271.y = max(_1271.y, 0.0500000007450580596923828125f);
            _1295 = _1271;
            _1296 = _185.rotation + ((length(_1270) * 3.0f) * lerp(0.5f, 1.0f, asfloat(((_1280 + (_1280 << 15u)) & 8388607u) | 1065353216u) - 1.0f));
            _1297 = _185.scale;
            _1298 = _185.up_axis;
        }
        else
        {
            float _204 = _186.y - (frame_constants_delta_time_seconds * 0.012500000186264514923095703125f);
            float3 _205 = _186;
            _205.y = _204;
            float4 _214 = grass_object_push.SampleLevel(sampler_default, (_205.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
            uint _426;
            uint _427;
            bool _428;
            bool _429;
            uint _430;
            uint _431;
            bool _432;
            bool _433;
            float3 _434;
            do
            {
                if (frame_constants_glade_theme_id == 4u)
                {
                    _426 = _149;
                    _427 = _149;
                    _428 = false;
                    _429 = false;
                    _430 = _149;
                    _431 = _149;
                    _432 = false;
                    _433 = false;
                    _434 = 0.0f.xxx;
                    break;
                }
                float _236 = float(frame_constants_game_time_sec) + frame_constants_game_time_frac;
                uint _241;
                uint _243;
                bool _245;
                bool _247;
                uint _249;
                uint _251;
                bool _253;
                bool _255;
                _241 = _149;
                _243 = _149;
                _245 = false;
                _247 = false;
                _249 = _149;
                _251 = _149;
                _253 = false;
                _255 = false;
                float2 _173[4];
                for (uint _238 = 0u; _238 < 4u; )
                {
                    float _261 = float(_152[_238]);
                    int _263 = int(floor(_261));
                    float _264 = frac(_261);
                    uint _265 = uint(_263);
                    uint _267 = _265 + (_265 << 10u);
                    uint _269 = _267 ^ (_267 >> 6u);
                    uint _271 = _269 + (_269 << 3u);
                    uint _273 = _271 ^ (_271 >> 11u);
                    bool _276 = _255 ? _255 : true;
                    uint _277 = _255 ? _251 : 1664525u;
                    bool _278 = _253 ? _253 : true;
                    uint _279 = _253 ? _249 : 1013904223u;
                    uint _283 = (((_265 * _277) + (_273 + (_273 << 15u))) + _279) * _277;
                    uint _285 = _283 ^ (_283 >> 11u);
                    uint _288 = _285 ^ ((_285 << 7u) & 2636928640u);
                    uint _291 = _288 ^ ((_288 << 15u) & 4022730752u);
                    uint _293 = _291 ^ (_291 >> 18u);
                    uint _295 = _293 + (_293 << 10u);
                    uint _297 = _295 ^ (_295 >> 6u);
                    uint _299 = _297 + (_297 << 3u);
                    uint _301 = _299 ^ (_299 >> 11u);
                    bool _304 = _247 ? _247 : true;
                    uint _305 = _247 ? _243 : 8388607u;
                    bool _306 = _245 ? _245 : true;
                    uint _307 = _245 ? _241 : 1065353216u;
                    bool _312 = _304 ? _304 : true;
                    uint _313 = _304 ? _305 : 8388607u;
                    bool _314 = _306 ? _306 : true;
                    uint _315 = _306 ? _307 : 1065353216u;
                    uint _322 = uint(_263 + 1);
                    uint _324 = _322 + (_322 << 10u);
                    uint _326 = _324 ^ (_324 >> 6u);
                    uint _328 = _326 + (_326 << 3u);
                    uint _330 = _328 ^ (_328 >> 11u);
                    uint _252 = _276 ? _277 : 1664525u;
                    uint _250 = _278 ? _279 : 1013904223u;
                    uint _336 = (((_322 * _252) + (_330 + (_330 << 15u))) + _250) * _252;
                    uint _338 = _336 ^ (_336 >> 11u);
                    uint _341 = _338 ^ ((_338 << 7u) & 2636928640u);
                    uint _344 = _341 ^ ((_341 << 15u) & 4022730752u);
                    uint _346 = _344 ^ (_344 >> 18u);
                    uint _348 = _346 + (_346 << 10u);
                    uint _350 = _348 ^ (_348 >> 6u);
                    uint _352 = _350 + (_350 << 3u);
                    uint _354 = _352 ^ (_352 >> 11u);
                    bool _357 = _312 ? _312 : true;
                    uint _358 = _312 ? _313 : 8388607u;
                    bool _359 = _314 ? _314 : true;
                    uint _360 = _314 ? _315 : 1065353216u;
                    uint _244 = _357 ? _358 : 8388607u;
                    uint _242 = _359 ? _360 : 1065353216u;
                    float _373 = (_264 * _264) * (3.0f - (2.0f * _264));
                    _173[_238] = (normalize(((float2(asfloat((_293 & _305) | _307) - 1.0f, asfloat(((_301 + (_301 << 15u)) & _313) | _315) - 1.0f) * (1.0f - _373)) + (float2(asfloat((_346 & _358) | _360) - 1.0f, asfloat(((_354 + (_354 << 15u)) & _244) | _242) - 1.0f) * _373)) - 0.5f.xx) * _236) * 3.0f;
                    _238++;
                    _241 = _242;
                    _243 = _244;
                    _245 = _359 ? _359 : true;
                    _247 = _357 ? _357 : true;
                    _249 = _250;
                    _251 = _252;
                    _253 = _278 ? _278 : true;
                    _255 = _276 ? _276 : true;
                    continue;
                }
                float3 _384;
                _384 = 0.0f.xxx;
                for (uint _387 = 0u; _387 < 1u; )
                {
                    uint _388 = _387 + 1u;
                    float _392 = float(_388) * 10.16640758514404296875f;
                    float _393 = cos(_392);
                    float _394 = sin(_392);
                    float2x2 _398 = float2x2(float2(_393, _394), float2(-_394, _393));
                    float2 _425 = _384.yz + mul(((((water_noise_texture.SampleLevel(sampler_llr, mul(_398, (_205.xz * _150[_387]) + _173[_387]) * 0.01200000010430812835693359375f, 0.0f).xyz * float3(1.0f, 2.0f, 2.0f)) + float3(-0.5f, -1.0f, -1.0f)) * _151[_387]) * float2(1.0f, _150[_387] * 0.01200000010430812835693359375f).xyy).yz, _398);
                    _384 = float3(_384.x, _425.x, _425.y);
                    _387 = _388;
                    continue;
                }
                _426 = _249;
                _427 = _251;
                _428 = _253;
                _429 = _255;
                _430 = _241;
                _431 = _243;
                _432 = _245;
                _433 = _247;
                _434 = _384;
                break;
            } while(false);
            float2 _440 = _205.xz + ((((float3(_214.x, 0.0f, _214.y) * ((clamp(_214.z, 0.0f, 1.0f) > 0.0f) ? 1.0f : 0.0f)) * 0.3499999940395355224609375f) + (_434.yxz * float3(2.0f, 0.00999999977648258209228515625f, 2.0f))).xz * 0.100000001490116119384765625f);
            float3 _441 = float3(_440.x, _205.y, _440.y);
            float2 _452 = (_440.xy * 0.100000001490116119384765625f) + (float2(float(frame_constants_game_time_sec) + frame_constants_game_time_frac, 0.0f) * 1.2000000476837158203125f);
            int2 _454 = int2(floor(_452));
            float2 _455 = frac(_452);
            uint2 _456 = uint2(_454);
            uint _458 = _456.y;
            uint _460 = _458 + (_458 << 10u);
            uint _462 = _460 ^ (_460 >> 6u);
            uint _464 = _462 + (_462 << 3u);
            uint _466 = _464 ^ (_464 >> 11u);
            bool _469 = _429 ? _429 : true;
            uint _470 = _429 ? _427 : 1664525u;
            bool _471 = _428 ? _428 : true;
            uint _472 = _428 ? _426 : 1013904223u;
            uint _476 = (((_456.x * _470) + (_466 + (_466 << 15u))) + _472) * _470;
            uint _478 = _476 ^ (_476 >> 11u);
            uint _481 = _478 ^ ((_478 << 7u) & 2636928640u);
            uint _484 = _481 ^ ((_481 << 15u) & 4022730752u);
            bool _487 = _433 ? _433 : true;
            uint _488 = _433 ? _431 : 8388607u;
            bool _489 = _432 ? _432 : true;
            uint _490 = _432 ? _430 : 1065353216u;
            uint2 _496 = uint2(_454 + int2(1, 0));
            uint _498 = _496.y;
            uint _500 = _498 + (_498 << 10u);
            uint _502 = _500 ^ (_500 >> 6u);
            uint _504 = _502 + (_502 << 3u);
            uint _506 = _504 ^ (_504 >> 11u);
            bool _509 = _469 ? _469 : true;
            uint _510 = _469 ? _470 : 1664525u;
            bool _511 = _471 ? _471 : true;
            uint _512 = _471 ? _472 : 1013904223u;
            uint _516 = (((_496.x * _510) + (_506 + (_506 << 15u))) + _512) * _510;
            uint _518 = _516 ^ (_516 >> 11u);
            uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
            uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
            bool _527 = _487 ? _487 : true;
            uint _528 = _487 ? _488 : 8388607u;
            bool _529 = _489 ? _489 : true;
            uint _530 = _489 ? _490 : 1065353216u;
            uint2 _536 = uint2(_454 + int2(0, 1));
            uint _538 = _536.y;
            uint _540 = _538 + (_538 << 10u);
            uint _542 = _540 ^ (_540 >> 6u);
            uint _544 = _542 + (_542 << 3u);
            uint _546 = _544 ^ (_544 >> 11u);
            uint _550 = _509 ? _510 : 1664525u;
            uint _552 = _511 ? _512 : 1013904223u;
            uint _556 = (((_536.x * _550) + (_546 + (_546 << 15u))) + _552) * _550;
            uint _558 = _556 ^ (_556 >> 11u);
            uint _561 = _558 ^ ((_558 << 7u) & 2636928640u);
            uint _564 = _561 ^ ((_561 << 15u) & 4022730752u);
            bool _567 = _527 ? _527 : true;
            uint _568 = _527 ? _528 : 8388607u;
            bool _569 = _529 ? _529 : true;
            uint _570 = _529 ? _530 : 1065353216u;
            uint2 _576 = uint2(_454 + int2(1, 1));
            uint _578 = _576.y;
            uint _580 = _578 + (_578 << 10u);
            uint _582 = _580 ^ (_580 >> 6u);
            uint _584 = _582 + (_582 << 3u);
            uint _586 = _584 ^ (_584 >> 11u);
            uint _589 = (_509 ? _509 : true) ? _550 : 1664525u;
            uint _594 = (((_576.x * _589) + (_586 + (_586 << 15u))) + ((_511 ? _511 : true) ? _552 : 1013904223u)) * _589;
            uint _596 = _594 ^ (_594 >> 11u);
            uint _599 = _596 ^ ((_596 << 7u) & 2636928640u);
            uint _602 = _599 ^ ((_599 << 15u) & 4022730752u);
            bool _605 = _567 ? _567 : true;
            uint _606 = _567 ? _568 : 8388607u;
            bool _607 = _569 ? _569 : true;
            uint _608 = _569 ? _570 : 1065353216u;
            float2 _616 = (_455 * _455) * (3.0f.xx - (_455 * 2.0f));
            float _617 = _616.x;
            float _618 = 1.0f - _617;
            float _620 = _616.y;
            bool _635 = _605 ? _605 : true;
            bool _637 = _607 ? _607 : true;
            uint _638 = _607 ? _608 : 1065353216u;
            _441.x = _440.x - (((sin((((((1.0f - _620) * (((asfloat(((_484 ^ (_484 >> 18u)) & _488) | _490) - 1.0f) * _618) + ((asfloat(((_524 ^ (_524 >> 18u)) & _528) | _530) - 1.0f) * _617))) + (((asfloat(((_564 ^ (_564 >> 18u)) & _568) | _570) - 1.0f) * _618) * _620)) + (((asfloat(((_602 ^ (_602 >> 18u)) & _606) | _608) - 1.0f) * _617) * _620)) * 3.1400001049041748046875f) - 1.57000005245208740234375f) + 0.300000011920928955078125f) * 0.0056000002659857273101806640625f) * lerp(0.699999988079071044921875f, 1.0f, asfloat(0u | _638) - 1.0f));
            float3 _776;
            float _777;
            float3 _778;
            if (_204 < (-0.02999999932944774627685546875f))
            {
                int _651 = int(gl_GlobalInvocationID.x);
                uint _663 = uint(_651 ^ 10945);
                uint _665 = _663 + (_663 << 10u);
                uint _667 = _665 ^ (_665 >> 6u);
                uint _669 = _667 + (_667 << 3u);
                uint _671 = _669 ^ (_669 >> 11u);
                bool _674 = _635 ? _635 : true;
                uint _675 = _635 ? (_605 ? _606 : 8388607u) : 8388607u;
                bool _676 = _637 ? _637 : true;
                uint _677 = _637 ? _638 : 1065353216u;
                uint _684 = uint(_651 ^ 49234);
                uint _686 = _684 + (_684 << 10u);
                uint _688 = _686 ^ (_686 >> 6u);
                uint _690 = _688 + (_688 << 3u);
                uint _692 = _690 ^ (_690 >> 11u);
                bool _695 = _674 ? _674 : true;
                uint _696 = _674 ? _675 : 8388607u;
                bool _697 = _676 ? _676 : true;
                uint _698 = _676 ? _677 : 1065353216u;
                uint _705 = uint(_651 ^ 23458);
                uint _707 = _705 + (_705 << 10u);
                uint _709 = _707 ^ (_707 >> 6u);
                uint _711 = _709 + (_709 << 3u);
                uint _713 = _711 ^ (_711 >> 11u);
                bool _716 = _695 ? _695 : true;
                uint _717 = _695 ? _696 : 8388607u;
                bool _718 = _697 ? _697 : true;
                uint _719 = _697 ? _698 : 1065353216u;
                uint _728 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
                uint _730 = _728 ^ (_728 >> 6u);
                uint _732 = _730 + (_730 << 3u);
                uint _734 = _732 ^ (_732 >> 11u);
                uint _736 = _734 + (_734 << 15u);
                bool _737 = _716 ? _716 : true;
                uint _738 = _716 ? _717 : 8388607u;
                bool _739 = _718 ? _718 : true;
                uint _740 = _718 ? _719 : 1065353216u;
                uint _748 = _737 ? _738 : 8388607u;
                uint _750 = _739 ? _740 : 1065353216u;
                uint _756 = gl_GlobalInvocationID.x ^ 2345346u;
                uint _758 = _756 + (_756 << 10u);
                uint _760 = _758 ^ (_758 >> 6u);
                uint _762 = _760 + (_760 << 3u);
                uint _764 = _762 ^ (_762 >> 11u);
                _776 = __alias_particle_emitter_buffer.Load<float3>(((uint(_651) ^ (345345u * (frame_constants_frame_index + 1u))) % uint(_Globals_particle_emitter_count)) * 16 + 0) + float3(lerp(-0.5f, 0.5f, asfloat(((_671 + (_671 << 15u)) & _675) | _677) - 1.0f), lerp(-0.5f, 0.5f, asfloat(((_692 + (_692 << 15u)) & _696) | _698) - 1.0f), lerp(-0.5f, 0.5f, asfloat(((_713 + (_713 << 15u)) & _717) | _719) - 1.0f));
                _777 = lerp(0.60000002384185791015625f, 1.0f, asfloat((_736 & _738) | _740) - 1.0f) * 0.800000011920928955078125f;
                _778 = normalize(float3(lerp(-1.0f, 1.0f, asfloat((_736 & _748) | _750) - 1.0f), 2.0f, lerp(-1.0f, 1.0f, asfloat(((_764 + (_764 << 15u)) & ((_737 ? _737 : true) ? _748 : 8388607u)) | ((_739 ? _739 : true) ? _750 : 1065353216u)) - 1.0f)));
            }
            else
            {
                _776 = _441;
                _777 = _185.scale - (frame_constants_delta_time_seconds * 0.0500000007450580596923828125f);
                _778 = float3(0.0f, 1.0f, 0.0f);
            }
            _1295 = _776;
            _1296 = _185.rotation;
            _1297 = _777;
            _1298 = _778;
        }
        LeafParticle _1299 = { _1295, _1296, _1297, _1298 };
        __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 0, _1299.position);
        __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 12, _1299.rotation);
        __alias_leaf_particle_buffer.Store<float>(gl_GlobalInvocationID.x * 32 + 16, _1299.scale);
        __alias_leaf_particle_buffer.Store<float3>(gl_GlobalInvocationID.x * 32 + 20, _1299.up_axis);
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
