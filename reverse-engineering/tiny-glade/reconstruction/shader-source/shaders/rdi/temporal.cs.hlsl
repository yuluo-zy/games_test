// ==RECONSTRUCTED== shaders/rdi/temporal.cs.hlsl | F-D
// Evidence: _rdi_temporal.cs.hlsl_cs_main_0.bin (SPIR-V, module 0).
//   bindings: sampler_nnc s2, sampler_llc s3, lantern_buf t13 (StructuredBuffer
//   LanternPacked), $Globals b14, blue_noise_texture t15, reprojection_tex t16,
//   half_depth_tex t17, half_view_normal_tex t18, rdi_temporal_history_tex t19,
//   rdi_temporal_out_tex u20, light_indirection_grid t21, grid_light_indices t22,
//   frame_constants set1/b0. LocalSize [8,8,1], entry cs_main.
//   spirv-cross: RDI (ReSTIR direct illumination) temporal merge. Two fresh
//   lantern samples are generated from the light grid, occlusion-tested against
//   the half-res depth, then up to four history reservoirs are reprojected and
//   merged with the candidate reservoir (weighted-reservoir stream state). The
//   surviving payload / M / W is packed into the rdi_temporal reservoir texel.
// Status: O (structure from disassembly); I (helper names inlined).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/reservoir.hlsl"
// history_valid.hlsl: the inc's helper is is_history_valid (adjudicated
// inc-fix #16), so no name clash with this consumer's $Globals member.
#include "inc/history_valid.hlsl"
// blue_noise.hlsl is NOT included: its blue_noise_texture register does not
// match this consumer (binds t15) and its blue_noise() seed pattern does not
// match this consumer.
#include "inc/hash.hlsl"
#include "inc/perlin.hlsl"
#include "inc/halfres.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/morton.hlsl"
#include "inc/math.hlsl"
#include "inc/consts.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/ray_cone.hlsl"
#include "light_binning/grid.inc.hlsl"

// Bindless descriptor array declared only to reproduce the target module's
// RuntimeDescriptorArray capability + SPV_EXT_descriptor_indexing extension
// (the original declaration is unused here and was dropped by spirv-cross).
Texture2D<float4> rdi_temporal_unused_tex_array[];

cbuffer $Globals : register(b14, space0)
{
    uint2 blue_noise_texture_dims;
    uint history_valid;
    uint2 reprojection_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 rdi_temporal_history_tex_dims;
    uint2 rdi_temporal_out_tex_dims;
    uint lantern_count;
};

StructuredBuffer<LanternPacked> lantern_buf : register(t13, space0);
StructuredBuffer<uint> light_indirection_grid : register(t21, space0);
StructuredBuffer<uint> grid_light_indices : register(t22, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> reprojection_tex : register(t16, space0);
Texture2D<float4> half_depth_tex : register(t17, space0);
Texture2D<float4> half_view_normal_tex : register(t18, space0);
Texture2D<uint4> rdi_temporal_history_tex : register(t19, space0);
RWTexture2D<uint2> rdi_temporal_out_tex : register(u20, space0);

// Module-scope statics corresponding to the target's OpUndef values: the initial
// reservoir stream-state random bits and the initial depth-test values.
static uint undefined_uint;
static float undefined_float;

// Unpack / pack half16 pairs (lantern intensity, reservoir weights).
float2 unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

uint pack_half2x16(float2 value)
{
    uint2 packed = f32tof16(value);
    return packed.x | (packed.y << 16);
}

// Luminance weights.
static const float3 LUMA = float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    do
    {
        float depth = half_depth_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).x;
        float2 half_dims = float2(half_depth_tex_dims);
        float2 uv = (float2(int2(gl_GlobalInvocationID.xy)) + 0.5f.xx) / half_dims;

        // 2D integer hash seeded by the frame index.
        uint seed = frame_index + 192192u;
        uint h1 = seed;
        h1 += (h1 << 10u);
        h1 ^= (h1 >> 6u);
        h1 += (h1 << 3u);
        h1 ^= (h1 >> 11u);
        uint a1 = (((gl_GlobalInvocationID.y * 1664525u) + (h1 + (h1 << 15u))) + 1013904223u) * 1664525u;
        a1 ^= (a1 >> 11u);
        a1 ^= ((a1 << 7u) & 2636928640u);
        a1 ^= ((a1 << 15u) & 4022730752u);
        uint b1 = (((gl_GlobalInvocationID.x * 1664525u) + (a1 ^ (a1 >> 18u))) + 1013904223u) * 1664525u;
        b1 ^= (b1 >> 11u);
        b1 ^= ((b1 << 7u) & 2636928640u);
        b1 ^= ((b1 << 15u) & 4022730752u);
        uint rng0 = b1 ^ (b1 >> 18u);

        float2 sample_pos = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
        float sample_x = sample_pos.x;
        float4 view_pos_h = float4(sample_x, sample_pos.y, depth, 1.0f);
        float4 world_pos_h = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, view_pos_h));

        if (((-1.0f) / (depth * (-view_constants[0].clip_to_view[3].z))) > 2000.0f)
        {
            rdi_temporal_out_tex[gl_GlobalInvocationID.xy] = uint2(0u, 0u);
            break;
        }

        float3 world_normal = mul(view_constants[0].view_to_world, float4(half_view_normal_tex.Load(int3(gl_GlobalInvocationID.xy, 0u)).xyz, 0.0f)).xyz;
        float3 world_pos = world_pos_h.xyz / world_pos_h.w.xxx;

        float4 rnd = ((blue_noise_texture.Load(int3((gl_GlobalInvocationID.xy + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;

        uint3 cell = uint3(clamp(floor((world_pos * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint cell_index = (32u * ((32u * cell.y) + cell.z)) + cell.x;
        uint grid_entry = light_indirection_grid[cell_index];
        uint grid_start = grid_entry & 1048575u;
        uint grid_count = min(64u, grid_entry >> 20u);

        uint rng_state = rng0;
        rng_state += (rng_state << 10u);
        rng_state ^= (rng_state >> 6u);
        rng_state += (rng_state << 3u);
        rng_state ^= (rng_state >> 11u);
        rng_state += (rng_state << 15u);

        uint rand_exp = undefined_uint;
        uint rand_mant = undefined_uint;
        bool exp_seen = false;
        bool mant_seen = false;
        float acc_w = 0.0f;
        uint sel_payload = 0u;
        float m_sum = 0.0f;
        float w_sum = 0.0f;

        for (uint candidate = 0u; candidate < min(2u, grid_count); candidate++)
        {
            uint entry_index = grid_start + ((rng0 + candidate) % grid_count);
            uint lantern_index = grid_light_indices[entry_index];
            float lantern_intensity = unpack_half2x16(lantern_buf[lantern_index].intensity__color_id & 65535u).x;
            uint lantern_color_id = lantern_buf[lantern_index].intensity__color_id >> 16u;
            float3 to_light = lantern_buf[lantern_index].position - world_pos;
            float dist2 = dot(to_light, to_light);
            float3 up_vec = to_light + float3(0.0f, 0.1500000059604644775390625f, 0.0f);

            float3 lantern_color;
            if (lantern_color_id == 0u)
            {
                lantern_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
            }
            else if (lantern_color_id == 1u)
            {
                lantern_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
            }
            else if (lantern_color_id == 2u)
            {
                lantern_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
            }
            else if (lantern_color_id == 3u)
            {
                lantern_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
            }
            else
            {
                lantern_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
            }

            float w = dot(((lantern_color * 4.0f) * lantern_intensity) * ((max(0.0f, dot(world_normal, to_light / length(to_light).xxx)) * ((1.0f / (0.3359999954700469970703125f + dist2)) * smoothstep(36.0f * lantern_intensity, 16.0f * lantern_intensity, dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (up_vec / length(up_vec).xxx).y)), LUMA);

            uint next_rng = rng_state;
            next_rng += (next_rng << 10u);
            next_rng ^= (next_rng >> 6u);
            next_rng += (next_rng << 3u);
            next_rng ^= (next_rng >> 11u);
            next_rng += (next_rng << 15u);

            bool mant_seen_new = mant_seen ? mant_seen : true;
            uint mant_new = mant_seen ? rand_mant : 8388607u;
            bool exp_seen_new = exp_seen ? exp_seen : true;
            uint exp_new = exp_seen ? rand_exp : 1065353216u;

            float next_m_sum = m_sum + 1.0f;
            float target_w = w * (float(grid_count) / float(lantern_count));
            float next_w_sum = w_sum + target_w;
            uint next_payload;
            float next_acc_w;
            if ((target_w / next_w_sum) >= (asfloat((rng_state & mant_new) | exp_new) - 1.0f))
            {
                next_payload = grid_light_indices[entry_index];
                next_acc_w = w;
            }
            else
            {
                next_payload = sel_payload;
                next_acc_w = acc_w;
            }

            m_sum = next_m_sum;
            rand_mant = mant_new;
            rand_exp = exp_new;
            mant_seen = mant_seen_new;
            exp_seen = exp_seen_new;
            rng_state = next_rng;
            sel_payload = next_payload;
            acc_w = next_acc_w;
            w_sum = next_w_sum;
        }

        bool occluded;
        if (w_sum > 0.0f)
        {
            bool valid;
            do
            {
                float3 to_sel = lantern_buf[sel_payload].position - world_pos;
                float sel_dist = length(to_sel);
                if (sel_dist > 0.5f)
                {
                    float3 view_hit = view_pos_h.xyz;
                    float4 reproj = mul(view_constants[0].view_to_sample, mul(view_constants[0].world_to_view, float4((world_pos + ((to_sel * (sel_dist - 0.5f)) / sel_dist.xxx)) + (world_normal * (0.0199999995529651641845703125f * rnd.y)), 1.0f)));
                    float3 reproj_ndc = reproj.xyz / reproj.w.xxx;
                    float4 seg = view_pos_h + (float4(reproj_ndc - view_hit, 0.0f) * float(int(sign(reproj_ndc.z))));
                    float seg_w = seg.w;
                    float3 seg_ndc = (seg / (float((seg_w >= 0.0f) ? 1 : (-1)) * max(1.0000000133514319600180897396058e-10f, abs(seg_w))).xxxx).xyz;
                    float3 seg_dir = seg_ndc - view_hit;
                    bool3 neg = bool3(seg_dir.x < 0.0f.xxx.x, seg_dir.y < 0.0f.xxx.y, seg_dir.z < 0.0f.xxx.z);
                    float3 t_enter = (float3(neg.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - view_hit) / seg_dir;
                    float3 a = view_hit + (seg_dir * max(0.0f, max(t_enter.x, t_enter.y)));
                    float3 seg_dir2 = seg_ndc - a;
                    bool3 neg2 = bool3(seg_dir2.x >= 0.0f.xxx.x, seg_dir2.y >= 0.0f.xxx.y, seg_dir2.z >= 0.0f.xxx.z);
                    float3 t_exit = (float3(neg2.x ? 1.0f.xxx.x : float3(-1.0f, -1.0f, 0.0f).x, neg2.y ? 1.0f.xxx.y : float3(-1.0f, -1.0f, 0.0f).y, neg2.z ? 1.0f.xxx.z : float3(-1.0f, -1.0f, 0.0f).z) - a) / seg_dir2;
                    float3 b = a + (seg_dir2 * min(1.0f, min(min(t_exit.x, t_exit.y), t_exit.z)));
                    float rnd_x = rnd.x;

                    bool hit;
                    float hit_t;
                    float hit_scale;
                    do
                    {
                        float eps = 0.800000011920928955078125f * view_constants[0].clip_to_view[3].z;
                        uint step_count = uint(int(max(2u, min(4u, uint(int(floor(length((((b.xy * float2(0.5f, -0.5f)) + 0.5f.xx) - ((a.xy * float2(0.5f, -0.5f)) + 0.5f.xx)) * half_dims))))))));

                        float d_hi;
                        float d_lo;
                        float t_eps;
                        float t_first;
                        float t_prev;
                        bool crossed;
                        do
                        {
                            float3 seg3 = b - a;
                            float step_scale;
                            if (b.z > 9.5367431640625e-07f)
                            {
                                step_scale = float(step_count);
                            }
                            else
                            {
                                step_scale = max(1.0f, float(step_count) - 1.0f);
                            }
                            float step_inv = 1.0f / step_scale;

                            if (step_count > 0u)
                            {
                                float t0 = lerp(0.0f, 1.0f, pow(rnd_x * step_inv, 1.0f));
                                float3 p0 = a + (seg3 * t0);
                                float2 uv0 = (p0.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                float inv_z0 = 1.0f / p0.z;
                                float inv_d_hi = 1.0f / half_depth_tex.SampleLevel(sampler_llc, uv0, 0.0f).x;
                                float inv_d_lo = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, uv0, 0.0f).x;
                                float err_pos = (max(inv_d_hi, inv_d_lo) * 1.00000202655792236328125f) - inv_z0;
                                float err_neg = inv_z0 - min(inv_d_hi, inv_d_lo);
                                bool crossed0 = (err_pos < 0.0f) ? (err_neg < eps) : false;

                                if (crossed0)
                                {
                                    d_hi = err_pos;
                                    d_lo = err_neg;
                                    t_eps = eps;
                                    t_first = t0;
                                    t_prev = 0.0f;
                                    crossed = crossed0;
                                }
                                else
                                {
                                    float prev_t = t0;
                                    bool prev_crossed = crossed0;
                                    uint step_i = 1u;
                                    bool found_step;
                                    float f_d_hi;
                                    float f_d_lo;
                                    float f_t_eps;
                                    float f_t_first;
                                    for (;;)
                                    {
                                        if (step_i < step_count)
                                        {
                                            float t1 = lerp(0.0f, 1.0f, min(0.9999847412109375f, pow((float(step_i) + rnd_x) * step_inv, 1.0f)));
                                            float3 p1 = a + (seg3 * t1);
                                            float2 uv1 = (p1.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
                                            float inv_z1 = 1.0f / p1.z;
                                            float inv_d_hi1 = 1.0f / half_depth_tex.SampleLevel(sampler_llc, uv1, 0.0f).x;
                                            float inv_d_lo1 = 1.0f / half_depth_tex.SampleLevel(sampler_nnc, uv1, 0.0f).x;
                                            float err_pos1 = (max(inv_d_hi1, inv_d_lo1) * 1.00000202655792236328125f) - inv_z1;
                                            float err_neg1 = inv_z1 - min(inv_d_hi1, inv_d_lo1);
                                            bool crossed1 = (err_pos1 < 0.0f) ? (err_neg1 < eps) : false;
                                            if (crossed1)
                                            {
                                                f_d_hi = err_pos1;
                                                f_d_lo = err_neg1;
                                                f_t_eps = eps;
                                                f_t_first = t1;
                                                found_step = true;
                                                break;
                                            }
                                            step_i++;
                                            prev_crossed = crossed1;
                                            prev_t = t1;
                                            continue;
                                        }
                                        else
                                        {
                                            f_d_hi = 0.0f;
                                            f_d_lo = 0.0f;
                                            f_t_eps = 0.0f;
                                            f_t_first = 1.0f;
                                            found_step = prev_crossed;
                                            break;
                                        }
                                    }
                                    d_hi = f_d_hi;
                                    d_lo = f_d_lo;
                                    t_eps = f_t_eps;
                                    t_first = f_t_first;
                                    t_prev = prev_t;
                                    crossed = found_step;
                                }
                            }
                            else
                            {
                                d_hi = undefined_float;
                                d_lo = undefined_float;
                                t_eps = undefined_float;
                                t_first = 1.0f;
                                t_prev = 0.0f;
                                crossed = false;
                            }
                        } while (false);

                        bool occluded_here;
                        if (crossed)
                        {
                            bool cond;
                            if (d_lo < t_eps)
                            {
                                cond = d_hi < t_eps;
                            }
                            else
                            {
                                cond = false;
                            }
                            occluded_here = cond;
                        }
                        else
                        {
                            occluded_here = false;
                        }
                        if (occluded_here)
                        {
                            hit = true;
                            hit_t = t_first;
                            hit_scale = d_lo / view_constants[0].clip_to_view[3].z;
                            break;
                        }
                        hit = false;
                        hit_t = t_prev;
                        hit_scale = 0.0f;
                        break;
                    } while (false);

                    if (hit)
                    {
                        float4 hit_world = mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(lerp(a, b, hit_t.xxx), 1.0f)));
                        float3 to_light2 = ((hit_world.xyz / hit_world.w.xxx) + ((-normalize(mul(view_constants[0].view_to_world, mul(view_constants[0].sample_to_view, float4(sample_x, sample_pos.y, 0.0f, 1.0f))).xyz)) * hit_scale)) - lantern_buf[sel_payload].position;
                        bool occluded2;
                        if (abs(to_light2.y) > 0.5f)
                        {
                            occluded2 = true;
                        }
                        else
                        {
                            float2 xz = to_light2.xz;
                            occluded2 = dot(xz, xz) > 0.136900007724761962890625f;
                        }
                        if (occluded2)
                        {
                            valid = false;
                            break;
                        }
                    }
                }
                valid = true;
                break;
            } while (false);
            occluded = !valid;
        }
        else
        {
            occluded = false;
        }

        uint payload_out = occluded ? 0u : sel_payload;
        float w_sum_out = occluded ? 0.0f : w_sum;

        bool have_history;
        if (history_valid != 0u)
        {
            have_history = frame_index != 0u;
        }
        else
        {
            have_history = false;
        }

        uint payload_final;
        float w_sum_final;
        float acc_w_final;
        float m_sum_final;
        if (have_history)
        {
            uint payload_h = payload_out;
            float w_sum_h = w_sum_out;
            float acc_w_h = acc_w;
            float m_sum_h = m_sum;
            uint rand_exp_h = rand_exp;
            uint rand_mant_h = rand_mant;
            bool exp_seen_h = exp_seen;
            bool mant_seen_h = mant_seen;
            uint rng_h = rng_state;

            for (uint hist_i = 0u; hist_i < 4u; hist_i++)
            {
                uint rng_1 = rng_h;
                rng_1 += (rng_1 << 10u);
                rng_1 ^= (rng_1 >> 6u);
                rng_1 += (rng_1 << 3u);
                rng_1 ^= (rng_1 >> 11u);
                rng_1 += (rng_1 << 15u);

                bool mant_seen_1 = mant_seen_h ? mant_seen_h : true;
                uint mant_1 = mant_seen_h ? rand_mant_h : 8388607u;
                bool exp_seen_1 = exp_seen_h ? exp_seen_h : true;
                uint exp_1 = exp_seen_h ? rand_exp_h : 1065353216u;

                uint rng_2 = rng_1;
                rng_2 += (rng_2 << 10u);
                rng_2 ^= (rng_2 >> 6u);
                rng_2 += (rng_2 << 3u);
                rng_2 ^= (rng_2 >> 11u);
                rng_2 += (rng_2 << 15u);

                bool mant_seen_2 = mant_seen_1 ? mant_seen_1 : true;
                uint mant_2 = mant_seen_1 ? mant_1 : 8388607u;
                bool exp_seen_2 = exp_seen_1 ? exp_seen_1 : true;
                uint exp_2 = exp_seen_1 ? exp_1 : 1065353216u;

                bool first = 0u == hist_i;
                int2 jitter;
                if (first)
                {
                    jitter = int2(0, 0);
                }
                else
                {
                    jitter = int2(((float2(asfloat((rng_h & mant_1) | exp_1) - 1.0f, asfloat((rng_1 & mant_2) | exp_2) - 1.0f) * 2.0f) - 1.0f.xx) * 12.0f);
                }

                uint4 hist_texel = rdi_temporal_history_tex.Load(int3(uint2(jitter) + uint2((uv + reprojection_tex.Load(int3(gl_GlobalInvocationID.xy * uint2(2u, 2u), 0u)).xy) * float2(rdi_temporal_history_tex_dims)), 0u));
                uint hist_payload = hist_texel.x;
                uint hist_packed = hist_texel.y;
                float2 hist_weights = max(0.0f.xx, float2(unpack_half2x16(hist_packed & 65535u).x, unpack_half2x16((hist_packed >> 16u) & 65535u).x));
                float hist_w_sum = min(hist_weights.x, float(first ? 100 : 5));

                float hist_intensity = unpack_half2x16(lantern_buf[hist_payload].intensity__color_id & 65535u).x;
                uint hist_color_id = lantern_buf[hist_payload].intensity__color_id >> 16u;
                float3 to_hist = lantern_buf[hist_payload].position - world_pos;
                float hist_dist2 = dot(to_hist, to_hist);
                float3 up_hist = to_hist + float3(0.0f, 0.1500000059604644775390625f, 0.0f);

                float3 hist_color;
                if (hist_color_id == 0u)
                {
                    hist_color = float3(0.7919800281524658203125f, 0.63358402252197265625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.7919800281524658203125f).xxx;
                }
                else if (hist_color_id == 1u)
                {
                    hist_color = float3(0.0f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.215240001678466796875f).xxx;
                }
                else if (hist_color_id == 2u)
                {
                    hist_color = float3(0.1583960056304931640625f, 0.7919800281524658203125f, 0.1583960056304931640625f) / max(1.0000000133514319600180897396058e-10f, 0.772159993648529052734375f).xxx;
                }
                else if (hist_color_id == 3u)
                {
                    hist_color = float3(0.23759402334690093994140625f, 0.1583960056304931640625f, 0.7919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.279020011425018310546875f).xxx;
                }
                else
                {
                    hist_color = float3(0.7919800281524658203125f, 0.23759402334690093994140625f, 0.07919800281524658203125f) / max(1.0000000133514319600180897396058e-10f, 0.4343800246715545654296875f).xxx;
                }

                float hist_w = dot(((hist_color * 4.0f) * hist_intensity) * ((max(0.0f, dot(world_normal, to_hist / length(to_hist).xxx)) * ((1.0f / (0.3359999954700469970703125f + hist_dist2)) * smoothstep(36.0f * hist_intensity, 16.0f * hist_intensity, hist_dist2))) * smoothstep(-0.5f, 0.20000000298023223876953125f, (up_hist / length(up_hist).xxx).y)), LUMA);

                uint rng_3 = rng_2;
                rng_3 += (rng_3 << 10u);
                rng_3 ^= (rng_3 >> 6u);
                rng_3 += (rng_3 << 3u);
                rng_3 ^= (rng_3 >> 11u);
                rng_3 += (rng_3 << 15u);

                bool mant_seen_3 = mant_seen_2 ? mant_seen_2 : true;
                uint mant_3 = mant_seen_2 ? mant_2 : 8388607u;
                bool exp_seen_3 = exp_seen_2 ? exp_seen_2 : true;
                uint exp_3 = exp_seen_2 ? exp_2 : 1065353216u;

                float next_m_sum_h = m_sum_h + hist_w_sum;
                float w_h = (hist_w * hist_weights.y) * hist_w_sum;
                float next_w_sum_h = w_sum_h + w_h;
                uint next_payload_h;
                float next_acc_w_h;
                if ((w_h / next_w_sum_h) >= (asfloat((rng_2 & mant_3) | exp_3) - 1.0f))
                {
                    next_payload_h = hist_payload;
                    next_acc_w_h = hist_w;
                }
                else
                {
                    next_payload_h = payload_h;
                    next_acc_w_h = acc_w_h;
                }

                m_sum_h = next_m_sum_h;
                rand_mant_h = mant_3;
                rand_exp_h = exp_3;
                mant_seen_h = mant_seen_3;
                exp_seen_h = exp_seen_3;
                rng_h = rng_3;
                payload_h = next_payload_h;
                w_sum_h = next_w_sum_h;
                acc_w_h = next_acc_w_h;
            }
            payload_final = payload_h;
            w_sum_final = w_sum_h;
            acc_w_final = acc_w_h;
            m_sum_final = m_sum_h;
        }
        else
        {
            payload_final = payload_out;
            w_sum_final = w_sum_out;
            acc_w_final = acc_w;
            m_sum_final = m_sum;
        }

        rdi_temporal_out_tex[gl_GlobalInvocationID.xy] = uint2(payload_final, pack_half2x16(float2(m_sum_final, 0.0f)) | (pack_half2x16(float2(min(w_sum_final / max(9.9999999392252902907785028219223e-09f, m_sum_final * acc_w_final), 1.0f), 0.0f)) << 16u));
        break;
    } while (false);
}
