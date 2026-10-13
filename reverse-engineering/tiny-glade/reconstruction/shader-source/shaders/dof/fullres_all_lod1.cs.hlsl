// ==RECONSTRUCTED== shaders/dof/fullres_all_lod1.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (format-correct RWTexture2D<half4>, marginal histogram edge). blue_noise_texture declared locally (main inc/blue_noise.hlsl no longer declares the global). 6f1d noise scheme only (72a2 residual).
//   Full-res bokeh pass at LOD1 scale: 5-tap ring, 2 bins, no half-res upsample
//   blend (tile list offsets[2]).

#include "inc/common.hlsl"
#include "dof/multires.inc.hlsl"
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
#include "dof/tiles.inc.hlsl"

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
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t20, space0);
StructuredBuffer<uint2> tile_coords_buffer : register(t21, space0);
Texture2D<float4> color_tex : register(t15, space0);
Texture2D<float4> coc_tiles_dilated_tex : register(t16, space0);
Texture2D<float4> coc_tex : register(t18, space0);
RWTexture2D<half4> img_output : register(u19, space0);

groupshared uint dof_ring_samples[64][3];

uint PackRingPair(float2 v)
{
    return f32tof16(v.x) | (f32tof16(v.y) << 16u);
}

float2 UnpackRingPair(uint v)
{
    return float2(f16tof32(v & 0xFFFFu), f16tof32(v >> 16u));
}

uint2 TileOrigin(uint3 group_id, uint3 dispatch_id, uint offset_index)
{
    return (dispatch_id.xy & uint2(7u, 7u))
        + uint2(8u, 8u) * tile_coords_buffer[tile_offsets_buffer[0].offsets[offset_index] + group_id.x];
}

float2 BokehDir(float2 dir, float2 uv)
{
    if (bokeh_anisotropy > 0.0f)
    {
        float4 v = mul(view_constants[0].clip_to_view, float4((uv - 0.5f) * float2(2.0f, -2.0f), 0.0f, 1.0f));
        float2 p = v.xy / v.z;
        p.y = -p.y;
        float2 n = normalize(p);
        float2x2 rot = float2x2(n, float2(-n.y, n.x));
        float a = lerp(0.699999988079071f, 1.0f, exp((-4.0f * bokeh_anisotropy) * dot(p, p)));
        return mul(transpose(rot), mul(rot, dir) * float2(a, 1.0f / a));
    }
    return dir;
}

void SampleRing(uint thread, float2 uv, float2 dir_px, float ring_offset, float coc_step, float coc_limit,
                float lerp_factor, inout float coc_acc, inout float min_abs_coc, inout float max_abs_coc)
{
    float pending = 0.0f;
    [unroll]
    for (uint k = 0u; k < 5u; ++k)
    {
        float coc = clamp(coc_tex.SampleLevel(sampler_nnc,
            uv + dir_px * ((float(int(k - 2u)) + ring_offset) * coc_step), 0.0f).x, -coc_limit, coc_limit);
        float coc_abs = abs(coc);
        coc_acc = lerp(coc_acc, max(coc_acc, coc_abs), lerp_factor);
        min_abs_coc = min(min_abs_coc, coc_abs);
        max_abs_coc = max(max_abs_coc, coc_abs);
        if ((k & 1u) == 0u)
        {
            pending = coc;
        }
        else
        {
            dof_ring_samples[thread][k >> 1u] =
                PackRingPair(float2(pending, 0.0f)) | (PackRingPair(float2(coc, 0.0f)) << 16u);
        }
    }
    dof_ring_samples[thread][2u] = PackRingPair(float2(pending, 0.0f));
}

[numthreads(8, 8, 1)]
void cs_main(uint3 group_id : SV_GroupID, uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint2 pixel = TileOrigin(group_id, dispatch_id, 2u);
    float2 uv = (float2(int2(pixel)) + 0.5f) / float2(img_output_dims);
    float coc_limit = min(float(color_tex_dims.y) * 0.041666667908430099f,
        coc_tiles_dilated_tex.SampleLevel(sampler_nnc, uv, 0.0f).x);
    float4 noise = ((blue_noise_texture.Load(int3((pixel + uint2(13u, 13u)
        + uint2(int(frame_index * 59u), int(frame_index * 37u))) & uint2(255u, 255u), 0)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
    uint2 jitter_coord = uint2(7u, 7u) + pixel;
    float ring_jitter = (frac(52.982917785644531f * frac((0.067110560834407806f * float(jitter_coord.x))
        + (0.005837149918079376f * float(jitter_coord.y)))) - 0.5f) * 0.5f;
    float ring_phase = 0.5f + ring_jitter;
    float ring_offset = noise.y;
    float coc_step = coc_limit * 0.5f;
    float spoke_seed = 5.588237762451172f * float(frame_index % 64u);
    float spoke_rot = frac(52.982917785644531f * frac((0.067110560834407806f * (float(pixel.x) + spoke_seed))
        + (0.005837149918079376f * (float(pixel.y) + spoke_seed))));
    bool flip = ring_phase < 0.5f;
    float mask_phase = flip ? (0.5f - ring_jitter) : ring_phase;
    float spoke_sign = flip ? -1.0f : 1.0f;

    float4 color_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
    float coc_acc = 0.0f;
    float coc_weight = 0.0f;
    float weight_acc = 0.0f;
    float min_abs_coc = 1.0e10f;
    float max_abs_coc = -1.0e10f;

    [loop]
    for (uint spoke = 0u; spoke < 2u; ++spoke)
    {
        float angle = (float(spoke) + spoke_rot) * 1.5707963705062866f;
        float2 dir = BokehDir(float2(cos(angle), sin(angle)), uv);
        float2 dir_px = dir * (spoke_sign / float2(color_tex_dims));
        SampleRing(local_index, uv, dir_px, ring_offset, coc_step, coc_limit, noise.x * 0.2f,
            coc_acc, min_abs_coc, max_abs_coc);

        uint mask[2];
        [unroll]
        for (uint mi = 0u; mi < 2u; ++mi)
        {
            mask[mi] = 0u;
        }
        [loop]
        for (uint j = 0u; j < 5u; ++j)
        {
            uint lane_bit = j & 1u;
            uint pair = j >> 1u;
            float2 pair_coc = UnpackRingPair(dof_ring_samples[local_index][pair]);
            float ring_dist = (float(int(j - 2u)) + ring_offset) * coc_step;
            [unroll]
            for (uint b = 0u; b < 2u; ++b)
            {
                if ((ring_dist / ((float(b) + mask_phase) - 1.0f)) >= pair_coc[lane_bit])
                {
                    mask[b] |= (1u << (j & 31u));
                }
            }
        }

        float4 spoke_color = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint bin = 0u; bin < 2u; ++bin)
        {
            float mask_dist = (float(bin) + mask_phase) - 1.0f;
            float abs_dist = abs(mask_dist);
            coc_weight += abs_dist;
            if (mask[bin] == 0u)
            {
                continue;
            }
            bool positive = mask_dist >= 0.0f;
            float coc_a;
            float coc_b;
            uint crossing;
            if (positive)
            {
                crossing = firstbitlow(mask[bin]);
                uint prev = uint(max(0, (int(crossing) - 1)));
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][prev >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[prev & 1u];
            }
            else
            {
                crossing = firstbithigh(mask[bin]);
                uint next = min(4u, crossing + 1u);
                float2 c1 = UnpackRingPair(dof_ring_samples[local_index][crossing >> 1u]);
                float2 c2 = UnpackRingPair(dof_ring_samples[local_index][next >> 1u]);
                coc_a = c1[crossing & 1u];
                coc_b = c2[next & 1u];
            }
            float ring_dist = (float(int(crossing - 2u)) + ring_offset) * coc_step;
            float dist_ratio = ring_dist / mask_dist;
            float step = coc_step * (positive ? 1.0f : -1.0f);
            float candidate = ring_dist - step;
            float cross_weight = coc_b - (dist_ratio - (step / mask_dist));
            float sample_weight = abs_dist * float(mask[bin] != 0u);
            float coc_pos = lerp(candidate, ring_dist,
                clamp(cross_weight / (cross_weight - (coc_a - dist_ratio)), 0.0f, 1.0f));
            float coc_check = clamp(coc_tex.SampleLevel(sampler_nnc, uv + (dir_px * coc_pos), 0.0f).x,
                -coc_limit, coc_limit);
            float mip = log2(max(0.0f, abs(coc_a) * 0.125f)) * smoothstep(4.0f, 0.0f, max_abs_coc - min_abs_coc);
            float mip_x = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 1u);
            mip = min(mip, mip_x);
            float mip_y = WaveReadLaneAt(mip, WaveGetLaneIndex() ^ 8u);
            if (coc_a > coc_limit)
            {
                continue;
            }
            float epsilon = cross_weight * 0.1f;
            bool near = false;
            if (((coc_pos / mask_dist) - coc_check) < epsilon)
            {
                near = (coc_check - coc_a) < epsilon;
            }
            float add_weight;
            float add_color_weight;
            // No 8/11 near-branch guard for lod1 (target).
            if (near)
            {
                add_weight = weight_acc;
                add_color_weight = sample_weight;
            }
            else
            {
                float s = smoothstep(0.0f, 30.0f, coc_b) * smoothstep(0.2f * coc_limit, 0.0f, coc_a);
                add_weight = weight_acc + (sample_weight * s);
                add_color_weight = sample_weight * (1.0f - s);
            }
            weight_acc = add_weight;
            spoke_color += color_tex.SampleLevel(sampler_nnc,
                uv + (dir_px * (near ? coc_pos : candidate)), min(mip, mip_y)) * add_color_weight;
        }
        color_acc += spoke_color;
    }

    float scatter = weight_acc * smoothstep(0.01f * color_acc.w,
        max(9.9999997473787516e-06f, 0.2f * color_acc.w), weight_acc);
    if (scatter > 0.0f)
    {
        float min_coc = -coc_limit;
        float4 disk_acc = float4(0.0f, 0.0f, 0.0f, 0.0f);
        [loop]
        for (uint d = 0u; d < 16u; ++d)
        {
            float2 disc = frac(frac((float2(0.7548776865005493f, 0.5698403120040894f) * float(d)) + 0.5f) + noise.xy);
            float r = sqrt(disc.x);
            float theta = disc.y * 6.2831854820251465f;
            float2 sample_uv = uv + (((float2(cos(theta) * r, sin(theta) * r) * 1.0f) / float2(color_tex_dims)) * coc_acc);
            float4 c = coc_tex.SampleLevel(sampler_nnc, sample_uv, 0.0f);
            if (c.x > (min_coc - 10.0f))
            {
                disk_acc += color_tex.SampleLevel(sampler_llc, sample_uv, 0.0f);
                min_coc = max(min_coc, c.x);
            }
        }
        color_acc += (disk_acc / max(1.0f, disk_acc.w)) * scatter;
    }

    img_output[pixel] = max(float4(0.0f, 0.0f, 0.0f, 0.0f), color_acc / coc_weight);
}
