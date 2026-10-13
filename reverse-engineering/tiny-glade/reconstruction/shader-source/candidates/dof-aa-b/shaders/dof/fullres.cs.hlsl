// ==SKELETON==
// SOURCE PATH : shaders/dof/fullres.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (2 bins):
//   _dof_fullres.cs.hlsl_cs_main_{6f1dd5dd8a66e104,72a2a95d73e37edc}.bin
//     pass/stage   : _dof_fullres (dof) / cs
//     entry        : cs:cs_main
//     variant      : 6f1dd5dd8a66e104 / 72a2a95d73e37edc
//     index_pass   : W51 (Inference)
//
// ==RECONSTRUCTED== shaders/dof/fullres.cs.hlsl
// Evidence: _dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin (SPIR-V, offset 23).
//   bindings: sampler_nnc s2, sampler_llc s3, $Globals b13 {bokeh_anisotropy,
//   blue_noise_texture_dims, color_tex_dims, coc_tiles_dilated_tex_dims,
//   depth_tex_dims, coc_tex_dims, img_output_dims, halfres_tex_dims},
//   blue_noise_texture t14, color_tex t15, coc_tiles_dilated_tex t16, coc_tex t18,
//   img_output u19, halfres_tex t20, spatial_resolve_offsets b21 (int4[512]),
//   tile_offsets_buffer t22, tile_coords_buffer t23, frame_constants set1/b0.
//   Full-res bokeh: half-res spatial resolve (upsample_halfres.inc) + the
//   dithered 2-ring x 17-tap CoC gather (multires.inc / shared with halfres),
//   summed and alpha-normalized into img_output.  Status: O (structure from
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
#include "tiles.inc.hlsl"
#include "upsample_halfres.inc.hlsl"
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
    uint2 halfres_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t14, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<float4> img_output : register(u19, space0);
Texture2D<float4> halfres_tex : register(t20, space0);
cbuffer spatial_resolve_offsets : register(b21, space0)
{
    int4 spatial_resolve_offsets_array[512];
};
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t22, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t23, space0);

groupshared uint dof_ring_samples[64][9];

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_WorkGroupID : SV_GroupID,
             uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    uint2 pixel = (gl_GlobalInvocationID.xy & uint2(7u, 7u)) + (uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[0] + gl_WorkGroupID.x]);
    float2 pixel_uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);

    // Half-res spatial resolve (8-tap upsample of the half-res bokeh).
    uint2 in_tile = pixel & uint2(1u, 1u);
    uint quadrant = in_tile.x + (in_tile.y * 2u);
    float4 upsample = upsample_halfres(halfres_tex, spatial_resolve_offsets_array, pixel, quadrant);

    float coc_limit = min(float(color_tex_dims.y) * 0.0416666679084300994873046875f, coc_tiles_dilated_tex.SampleLevel(sampler_nnc, pixel_uv, 0.0f).x);

    float4 dither = ((blue_noise_texture.Load(int3(((pixel + uint2(13u, 13u))
        + uint2(int2(int(frame_index * 59u), int(frame_index * 37u)))) & uint2(255u, 255u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;

    uint2 hash_pos = uint2(7u, 7u) + pixel;
    float sub = (frac(52.98291778564453125f * frac((0.067110560834407806396484375f * float(hash_pos.x))
        + (0.005837149918079376220703125f * float(hash_pos.y)))) - 0.5f) * 0.5f;
    float sub2 = 0.5f + sub;
    float dither_y = dither.y;
    float coc_step = coc_limit * 0.125f;
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
    uint ring_bits[8];
    ring_bits[0] = 0u; ring_bits[1] = 0u; ring_bits[2] = 0u; ring_bits[3] = 0u;
    ring_bits[4] = 0u; ring_bits[5] = 0u; ring_bits[6] = 0u; ring_bits[7] = 0u;
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

        float coc0 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        float m = abs(coc0);
        float w = lerp(weight_accum, max(weight_accum, m), lerp_gain);
        float coc1 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc1)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][0] = dof_pack_half2(float2(coc0, 0.0f)) | (dof_pack_half2(float2(coc1, 0.0f)) << 16u);

        float coc2 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc2)), lerp_gain);
        float coc3 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc3)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][1] = dof_pack_half2(float2(coc2, 0.0f)) | (dof_pack_half2(float2(coc3, 0.0f)) << 16u);

        float coc4 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc4)), lerp_gain);
        float coc5 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc5)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][2] = dof_pack_half2(float2(coc4, 0.0f)) | (dof_pack_half2(float2(coc5, 0.0f)) << 16u);

        float coc6 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc6)), lerp_gain);
        float coc7 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((-1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc7)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][3] = dof_pack_half2(float2(coc6, 0.0f)) | (dof_pack_half2(float2(coc7, 0.0f)) << 16u);

        float coc8 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (dither_y * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc8)), lerp_gain);
        float coc9 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((1.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc9)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][4] = dof_pack_half2(float2(coc8, 0.0f)) | (dof_pack_half2(float2(coc9, 0.0f)) << 16u);

        float coc10 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((2.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc10)), lerp_gain);
        float coc11 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((3.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc11)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][5] = dof_pack_half2(float2(coc10, 0.0f)) | (dof_pack_half2(float2(coc11, 0.0f)) << 16u);

        float coc12 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((4.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc12)), lerp_gain);
        float coc13 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((5.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc13)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][6] = dof_pack_half2(float2(coc12, 0.0f)) | (dof_pack_half2(float2(coc13, 0.0f)) << 16u);

        float coc14 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((6.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc14)), lerp_gain);
        float coc15 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((7.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        w = lerp(w, max(w, abs(coc15)), lerp_gain);
        dof_ring_samples[gl_LocalInvocationIndex][7] = dof_pack_half2(float2(coc14, 0.0f)) | (dof_pack_half2(float2(coc15, 0.0f)) << 16u);

        float coc16 = clamp(coc_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * ((8.0f + dither_y) * coc_step)), 0.0f).x, neg_coc, coc_limit);
        weight_accum = lerp(w, max(w, abs(coc16)), lerp_gain);
        coc_min = min(min(min(min(min(min(min(min(min(coc_min, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        coc_max = max(max(max(max(max(max(max(max(max(coc_max, abs(coc0)), abs(coc2)), abs(coc4)), abs(coc6)), abs(coc8)), abs(coc10)), abs(coc12)), abs(coc14)), abs(coc16));
        dof_ring_samples[gl_LocalInvocationIndex][8] = dof_pack_half2(float2(coc16, 0.0f));

        for (uint i = 0u; i < 8u; ++i)
        {
            ring_bits[i] = 0u;
        }
        for (uint tap = 0u; tap < 17u; ++tap)
        {
            uint par = tap & 1u;
            uint half = tap >> 1u;
            float2 slot_coc = float2(dof_unpack_half2(dof_ring_samples[gl_LocalInvocationIndex][half] & 65535u).x,
                                     dof_unpack_half2((dof_ring_samples[gl_LocalInvocationIndex][half] >> 16u) & 65535u).x);
            float tap_coc = (float(int(tap - 8u)) + dither_y) * coc_step;
            for (int slot = 0; slot < 8; ++slot)
            {
                if ((tap_coc / (((float(slot) + tap_sub) * 0.25f) - 1.0f)) >= slot_coc[par])
                {
                    ring_bits[slot] |= (1u << (tap & 31u));
                }
            }
        }

        float4 slot_color = 0.0f.xxxx;
        float slot_weight = 0.0f;
        float slot_depth = 0.0f;
        for (int slot = 0; slot < 8; slot_weight = depth_weight, slot_depth = slot_depth, slot_color = slot_color, ++slot)
        {
            float slot_frac = ((float(slot) + tap_sub) * 0.25f) - 1.0f;
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
                uint next_bit = min(16u, (high_bit + 1u));
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
            float tap_coc = (float(int(tap_idx - 8u)) + dither_y) * coc_step;
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
                float abs_resolved = abs(coc_at_resolved);
                if (abs_resolved < 8.0f)
                {
                    continue;
                }
                ring_color = slot_color;
                ring_weight = slot_span * smoothstep(8.0f, 11.0f, abs_resolved);
            }
            else
            {
                float abs_hi = abs(hi_coc);
                if (abs_hi < 8.0f)
                {
                    continue;
                }
                float w2 = slot_span * smoothstep(8.0f, 11.0f, abs_hi);
                float depth_gate = smoothstep(0.0f, 30.0f, hi_coc) * smoothstep(0.2f * coc_limit, 0.0f, lo_coc);
                ring_color = slot_color + (w2 * depth_gate);
                ring_weight = w2 * (1.0f - depth_gate);
            }
            depth_weight = ring_color.w;
            slot_color = slot_color + (color_tex.SampleLevel(sampler_nnc, pixel_uv + (step_px * (close_to_edge ? resolved : lo_edge)), min(occlusion, occ_wide)) * ring_weight);
        }
        ring_color_accum = ring_color_accum + slot_color;
    }

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

    // Combine the half-res upsample with the full-res gather and normalize.
    float4 combined = upsample + (final_color / depth_weight);
    img_output[pixel] = float4((combined / max(1e-10f, combined.w)).xyz, 1.0f);
}
