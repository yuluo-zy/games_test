// ==SKELETON==
// SOURCE PATH : shaders/dof/fused.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_fused.cs.hlsl_cs_main_0.bin
//     file_sha256  : 43c0079571555f0a8a3615335b2ffb0667b23a54527c46e5fcb1583e49922ebd
//     pass/stage   : _dof_fused (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     index_pass   : W51 (Inference)
//
// ==RECONSTRUCTED== shaders/dof/fused.cs.hlsl
// Evidence: _dof_fused.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 867).
//   bindings: sampler_nnc s2, sampler_llc s3, $Globals b13 {bokeh_anisotropy,
//   blue_noise_texture_dims, color_tex_dims, depth_tex_dims, coc_tex_dims,
//   img_output_dims}, blue_noise_texture t14, color_tex t15, coc_tex t17,
//   img_output u18 (R11g11b10f), frame_constants set1/b0.
//   Fused single-pass bokeh (no tile dispatch): dithered 2-ring x 21-tap CoC
//   gather (coc_step = dims.y/240), 6-slot classification, a per-slot occlusion
//   resolve with a runtime debug_tweakers.z anisotropy, a 6-sample background
//   scatter, and an alpha-normalized RGB write.  Status: O (structure from
//   disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/hash.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/morton.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/color/srgb.hlsl"
#include "multires.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tex : register(t17, space0);
[[vk::image_format("r11g11b10f")]]
RWTexture2D<float3> img_output : register(u18, space0);

groupshared uint dof_ring_samples[64][11];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    float2 pixel = float2(gl_GlobalInvocationID.xy) + 0.5f;
    float2 color_dims = float2(color_tex_dims);
    float2 pixel_uv = pixel / color_dims;
    float dims_y = float(color_tex_dims.y);
    float coc_limit = dims_y * 0.0416666679084300994873046875f;
    uint2 pixel_uint = uint2(pixel);

    float4 dither = ((blue_noise_texture.Load(int3(((pixel_uint + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
    float dither_x = dither.x;
    float dither_y = dither.y;
    float coc_step = dims_y * 0.0041666668839752674102783203125f;
    float dither_z = dither.z;
    bool use_low = dither_z < 0.5f;
    float tap_sub = use_low ? (1.0f - dither_z) : dither_z;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[6];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u;
    ring_bits[3] = 0u; ring_bits[4] = 0u; ring_bits[5] = 0u;

    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + dither_x) * 1.57079637050628662109375f;
        float4 ctv = mul(view_constants[0].clip_to_view, float4((pixel_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
        float2 view_dir = ctv.xy / ctv.z;
        view_dir.y = view_dir.y * (-1.0f);
        float2 norm_dir = normalize(view_dir);
        float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
        float aniso = lerp(0.7f, 1.0f, exp(((-debug_tweakers.z) * bokeh_anisotropy) * dot(view_dir, view_dir)));
        float2 bokeh_dir = mul(transpose(rot), mul(rot, float2(cos(angle), sin(angle))) * float2(aniso, 1.0f / aniso)) * tap_sign;
        float2 step_px = float2(pixel_uint);
        float neg_coc = dims_y * (-0.0416666679084300994873046875f);
        float lerp_gain = dither_x * 0.2f;

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-10.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-9.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-8.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-7.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-6.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-5.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-4.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-3.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-2.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (((-1.0f) + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * (dither_y * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((1.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((2.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((3.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((4.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((5.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((6.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc16)), lerp_gain);
        float coc17 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((7.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc17)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][8] = dof_pack_half2(float2(coc16, 0.0f)) | (dof_pack_half2(float2(coc17, 0.0f)) << 16u);

        float coc18 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((8.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc18)), lerp_gain);
        float coc19 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((9.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc19)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][9] = dof_pack_half2(float2(coc18, 0.0f)) | (dof_pack_half2(float2(coc19, 0.0f)) << 16u);

        float coc20 = clamp(coc_tex.SampleLevel(sampler_nnc, (step_px + (bokeh_dir * ((10.0f + dither_y) * coc_step))) / color_dims, 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc20)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][10] = dof_pack_half2(float2(coc20, 0.0f));

        for (uint i = 0u; i < 6u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 21u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 10u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 6; ++slot)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.3333333432674407958984375f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float4 slot_accum = 0.0f.xxxx;
        for (int slot = 0; slot < 6; slot_weight = depth_weight, slot_accum = slot_color, ++slot)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.3333333432674407958984375f) - 1.0f;
            if (ring_bits[slot] == 0u)
            {
                continue;
            }
            bool positive = slot_frac >= 0.0f;
            float lo_coc, hi_coc;
            uint tap_idx;
            if (positive)
            {
                uint low_bit = firstbitlow(ring_bits[slot]);
                uint prev_bit = uint(max(0, (int(low_bit) - 1)));
                uint low_half = low_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][low_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][low_half] >> 16u) & 65535u).x);
                uint prev_half = prev_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][prev_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][prev_half] >> 16u) & 65535u).x);
                lo_coc = a[low_bit & 1u];
                hi_coc = b[prev_bit & 1u];
                tap_idx = low_bit;
            }
            else
            {
                uint high_bit = firstbithigh(ring_bits[slot]);
                uint next_bit = min(20u, (high_bit + 1u));
                uint high_half = high_bit >> 1u;
                float2 a = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][high_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][high_half] >> 16u) & 65535u).x);
                uint next_half = next_bit >> 1u;
                float2 b = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][next_half] & 65535u).x,
                                  dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][next_half] >> 16u) & 65535u).x);
                lo_coc = a[high_bit & 1u];
                hi_coc = b[next_bit & 1u];
                tap_idx = high_bit;
            }
            float tap_coc = (float(int(tap_idx - 10u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs(slot_frac) * float(ring_bits[slot] != 0u);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.Load(int3(uint2(int2(step_px + (bokeh_dir * resolved))), 0u)).x, neg_coc, coc_limit);
            float edge_eps = hi_edge * 0.1f;
            bool close_to_edge;
            if (((resolved / slot_frac) - coc_at_resolved) < edge_eps)
            {
                close_to_edge = (coc_at_resolved - lo_coc) < edge_eps;
            }
            else
            {
                close_to_edge = false;
            }
            float ring_weight;
            if (close_to_edge)
            {
                ring_weight = slot_span;
            }
            else
            {
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(dims_y * 0.008333333767950534820556640625f, 0.0f, lo_coc);
                slot_color += (slot_span * depth_gate);
                ring_weight = slot_span * (1.0f - depth_gate);
            }
            slot_color += (color_tex.SampleLevel(sampler_llc,
                (floor(step_px + (bokeh_dir * (close_to_edge ? resolved : lo_edge))) + 0.5f) / color_dims,
                log2(max(0.0f, (abs(lo_coc) - hi_edge) * 0.25f))) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    // Background scatter (6 samples) when the foreground contribution is low.
    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = dims_y * (-0.0416666679084300994873046875f);
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 6u; ++s)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            uint2 bg_px = uint2(int2(float2(pixel_uint) + ((float2(cos(phi) * r, sin(phi) * r) * 1.0f) * weight_accum)));
            float bg_coc = coc_tex.Load(int3(bg_px, 0u)).x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum += color_tex.Load(int3(bg_px, 0u));
                max_bg_coc = max(max_bg_coc, bg_coc);
            }
        }
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w)) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    img_output[gl_GlobalInvocationID.xy] = max(0.0f.xxx, (final_color.xyz / max(1e-5f, final_color.w)).xyz);
}
