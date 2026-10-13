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
