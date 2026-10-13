// ==SKELETON==
// SOURCE PATH : shaders/dof/analyze.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _dof_analyze.cs.hlsl_cs_main_0.bin
//     file_sha256  : 75510235b057be3be42c635028c53f0affc008dfe68c1482d69fc740e5495b6b
//     pass/stage   : _dof_analyze (dof) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [8, 8, 1]
//     names        : $Globals, coc_tiles_tex, coc_tiles_dilated_tex, DofTileOffsets, tile_offsets_buffer, __alias_dispatch_args_buffer, __alias_tile_coords_buffer, cs_main
// ==RECONSTRUCTED== shaders/dof/analyze.cs.hlsl | dof-aa-a | status: O (structure from SPIR-V; morton_decode from inc/morton.hlsl verified against inlined use)
//   Dilates the per-tile min/max CoC over a circular ring, classifies 8x8 tile quads into
//   near/far/both buckets, and appends tile coords + dispatch counts with wave-atomic prefix sums.

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/hash.hlsl"
#include "inc/morton.hlsl"
#include "inc/coc.hlsl"
#include "dof/tiles.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 coc_tiles_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
};

StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t16, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u17, space0);
RWStructuredBuffer<uint2> __alias_tile_coords_buffer : register(u18, space0);
Texture2D<float4> coc_tiles_tex : register(t14, space0);
RWTexture2D<half> coc_tiles_dilated_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_id : SV_DispatchThreadID, uint local_index : SV_GroupIndex)
{
    uint2 tile = (dispatch_id.xy & uint2(0xFFFFFFF8u, 0xFFFFFFF8u)) + morton_decode(local_index);
    bool in_bounds = all(tile < coc_tiles_tex_dims);

    int radius = int(min(16.0f, ceil(float(coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624f)));
    float min_coc = 1.0e10f;
    float max_coc = 0.0f;
    for (int dy = -radius; dy <= radius; ++dy)
    {
        for (int dx = -radius; dx <= radius; ++dx)
        {
            int dist_sq = (dy * dy) + (dx * dx);
            uint2 sample_tile = tile + uint2(dx, dy);
            if (in_bounds ? (dist_sq < (radius * radius)) : false)
            {
                float4 c = coc_tiles_tex.Load(int3(sample_tile, 0));
                float coc = c.y;
                if ((coc + 8.0f) > (sqrt(float(dist_sq)) * 8.0f))
                {
                    max_coc = max(max_coc, coc);
                    min_coc = min(min_coc, c.x);
                }
            }
        }
    }
    coc_tiles_dilated_tex[tile] = max_coc;

    uint near_quad = uint(max_coc <= 7.0f);
    near_quad = QuadReadAcrossX(near_quad) & near_quad;
    near_quad = QuadReadAcrossY(near_quad) & near_quad;
    uint far_quad = uint(min_coc > 11.0f);
    far_quad = QuadReadAcrossX(far_quad) & far_quad;
    far_quad = QuadReadAcrossY(far_quad) & far_quad;

    uint counts[7];
    [unroll]
    for (uint b = 0u; b < 7u; ++b)
    {
        counts[b] = 0u;
    }

    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                counts[6]++;
            }
            counts[(max_coc <= 5.0f) ? 2u : 1u]++;
        }
        else
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                if (far_quad != 0u)
                {
                    counts[5]++;
                    counts[3] += 4u;
                }
                else
                {
                    counts[4]++;
                    counts[0] += 4u;
                }
            }
        }
    }

    uint sums[7];
    [unroll]
    for (uint g = 0u; g < 7u; ++g)
    {
        uint wave_count = WaveActiveSum(counts[g]);
        if (WaveIsFirstLane())
        {
            uint prev;
            InterlockedAdd(__alias_dispatch_args_buffer[g].x, wave_count, prev);
            sums[g] = tile_offsets_buffer[0].offsets[g] + prev;
        }
        sums[g] = WaveReadLaneFirst(sums[g]);
        sums[g] += WavePrefixSum(counts[g]);
    }

    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                __alias_tile_coords_buffer[sums[6]] = tile / uint2(2u, 2u);
            }
            __alias_tile_coords_buffer[sums[(max_coc <= 5.0f) ? 2u : 1u]] = tile;
        }
        else
        {
            if (all((tile & uint2(1u, 1u)) == uint2(0u, 0u)))
            {
                if (far_quad != 0u)
                {
                    __alias_tile_coords_buffer[sums[5]] = tile / uint2(2u, 2u);
                    __alias_tile_coords_buffer[sums[3]] = tile;
                    __alias_tile_coords_buffer[sums[3] + 1u] = tile + uint2(1u, 0u);
                    __alias_tile_coords_buffer[sums[3] + 2u] = tile + uint2(0u, 1u);
                    __alias_tile_coords_buffer[sums[3] + 3u] = tile + uint2(1u, 1u);
                }
                else
                {
                    __alias_tile_coords_buffer[sums[4]] = tile / uint2(2u, 2u);
                    __alias_tile_coords_buffer[sums[0]] = tile;
                    __alias_tile_coords_buffer[sums[0] + 1u] = tile + uint2(1u, 0u);
                    __alias_tile_coords_buffer[sums[0] + 2u] = tile + uint2(0u, 1u);
                    __alias_tile_coords_buffer[sums[0] + 3u] = tile + uint2(1u, 1u);
                }
            }
        }
    }
}
