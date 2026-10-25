// ==SKELETON==
// SOURCE PATH : shaders/dof/fullres_all_lod1.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (2 bins):
//   _dof_fullres_all_lod1.cs.hlsl_cs_main_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin
//     pass/stage   : _dof_fullres_all_lod1 (dof) / cs
//     entry        : cs:cs_main
//     variant      : 6f1dd5dd8a66e104 / 72a2a95d73e37edc
//     index_pass   : W51 (Inference)
//
// ==RECONSTRUCTED== shaders/dof/fullres_all_lod1.cs.hlsl
// Evidence: _dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (SPIR-V).
//   bindings identical to halfres; tile offsets[2], coc_step * 0.5, the coarsest
//   LOD1 variant: a 2-ring x 5-tap CoC gather packed into a 3-slot groupshared
//   ring table, 2-slot classification ((slot + tap_sub) - 1), and an
//   alpha-normalized RGB write (alpha 1).  Status: O.

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
#include "tiles.inc.hlsl"
#include "multires.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float bokeh_anisotropy;
    uint2 blue_noise_texture_dims;
    uint2 color_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
    uint2 depth_tex_dims;
    uint2 coc_tex_dims;
    uint2 img_output_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);

groupshared uint dof_ring_samples[64][3];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[2] + gl_WorkGroupID.x]);
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);

    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.5f;
    float frame_rand = 5.588237762451171875f * float(frame_index % 64u);
    float tap_seed = frac(52.98291778564453125f * frac((0.067110560834407806396484375f * (float(pixel.x) + frame_rand))
        + (0.005837149918079376220703125f * (float(pixel.y) + frame_rand))));
    bool use_low = sub2 < 0.5f;
    float tap_sub = use_low ? (0.5f - sub) : sub2;
    float tap_sign = use_low ? (-1.0f) : 1.0f;

    float4 color_accum = 0.0f.xxxx;
    float weight_accum = 0.0f;
    float depth_weight = 0.0f;
    float4 ring_color_accum = 0.0f.xxxx;
    uint ring_bits[2];
    ring_bits[0] = 0u; ring_bits[1] = 0u;
    float coc_min = 1e10f;
    float coc_max = -1e10f;

    for (uint ring = 0u; ring < 2u; ++ring)
    {
        float angle = (float(ring) + tap_seed) * 1.57079637050628662109375f;
        float2 dir = float2(cos(angle), sin(angle));
        float2 bokeh_dir;
        if (bokeh_anisotropy > 0.0f)
        {
            float4 ctv = mul(view_constants[0].clip_to_view, float4((pixel_uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
            float2 view_dir = ctv.xy / ctv.z;
            view_dir.y = view_dir.y * (-1.0f);
            float2 norm_dir = normalize(view_dir);
            float2x2 rot = float2x2(norm_dir, float2(-norm_dir.y, norm_dir.x));
            float aniso = lerp(0.7f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(view_dir, view_dir)));
            bokeh_dir = mul(transpose(rot), mul(rot, dir) * float2(aniso, 1.0f / aniso));
        }
        else
        {
            bokeh_dir = dir;
        }
        float2 step_px = bokeh_dir * (tap_sign / float2(color_tex_dims));
        float neg_coc = -coc_limit;
        float lerp_gain = dither.x * 0.2f;

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc4)), lerp_gain);
        coc_min = min(min(min(min(min(coc_min, abs(coc0)), abs(coc1)), abs(coc2)), abs(coc3)), abs(coc4));
        coc_max = max(max(max(max(max(coc_max, abs(coc0)), abs(coc1)), abs(coc2)), abs(coc3)), abs(coc4));
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f));

        for (uint i = 0u; i < 2u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 5u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 2u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 2; ++slot)
            {
                if ((tap_coc / ((float(slot) + tap_sub) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float slot_depth = 0.0f;
        for (int slot = 0; slot < 2; slot_weight = depth_weight, slot_depth = slot_depth, slot_color = slot_color, ++slot)
        {
            float slot_frac = (float(slot) + tap_sub) - 1.0f;
            float abs_frac = abs(slot_frac);
            depth_weight = slot_depth + abs_frac;
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
                uint next_bit = min(4u, (high_bit + 1u));
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
            float tap_coc = (float(int(tap_idx - 2u)) + dither_y) * coc_step;
            float coc_at_slot = tap_coc / slot_frac;
            float delta = coc_step * (positive ? 1.0f : (-1.0f));
            float lo_edge = tap_coc - delta;
            float hi_edge = hi_coc - (coc_at_slot - (delta / slot_frac));
            float slot_span = abs_frac * float(ring_bits[slot] != 0u);
            float resolved = lerp(lo_edge, tap_coc, clamp(hi_edge / (hi_edge + (-(lo_coc - coc_at_slot))), 0.0f, 1.0f));
            float coc_at_resolved = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * resolved), 0.0f).x, neg_coc, coc_limit);
            float occlusion = log2(max(0.0f, abs(lo_coc) * 0.125f)) * smoothstep(4.0f, 0.0f, coc_max - coc_min);
            float occ_lane = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 1u);
            occlusion = min(occlusion, occ_lane);
            float occ_wide = WaveReadLaneAt(occlusion, WaveGetLaneIndex() ^ 8u);
            if (lo_coc > coc_limit)
            {
                continue;
            }
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
            float4 ring_color;
            float ring_weight;
            if (close_to_edge)
            {
                ring_color = slot_color;
                ring_weight = slot_span;
            }
            else
            {
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                ring_color = slot_color + (slot_span * depth_gate);
                ring_weight = slot_span * (1.0f - depth_gate);
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

    // Background scatter (16 samples) when the foreground contribution is low.
    float scatter = depth_weight * smoothstep(0.01f * ring_color_accum.w, max(1e-5f, 0.2f * ring_color_accum.w), depth_weight);
    float4 final_color;
    if (scatter > 0.0f)
    {
        float neg_coc = -coc_limit;
        float4 bg_accum = 0.0f.xxxx;
        float max_bg_coc = neg_coc;
        for (uint s = 0u; s < 16u; ++s)
        {
            float2 rnd = frac(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(s)) + 0.5f) + dither.xy);
            float r = sqrt(rnd.x);
            float phi = rnd.y * 6.283185482025146484375f;
            float2 bg_uv = pixel_uv + (((float2(cos(phi) * r, sin(phi) * r) * 1.0f) / float2(color_tex_dims)) * weight_accum);
            float bg_coc = coc_tex.SampleLevel(sampler_nnc, bg_uv, 0.0f).x;
            if (bg_coc > (max_bg_coc - 10.0f))
            {
                bg_accum += color_tex.SampleLevel(sampler_llc, bg_uv, 0.0f);
                max_bg_coc = max(max_bg_coc, bg_coc);
            }
        }
        final_color = ring_color_accum + ((bg_accum / max(1.0f, bg_accum.w)) * scatter);
    }
    else
    {
        final_color = ring_color_accum;
    }
    float4 result = final_color / depth_weight;
    img_output[pixel] = float4((result / max(1e-10f, result.w)).xyz, 1.0f);
}
