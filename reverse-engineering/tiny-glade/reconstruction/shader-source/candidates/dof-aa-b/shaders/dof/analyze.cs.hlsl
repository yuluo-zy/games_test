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
//     index_pass   : W32 (Inference)
//     names        : $Globals, coc_tiles_tex, coc_tiles_dilated_tex, DofTileOffsets, tile_offsets_buffer, __alias_dispatch_args_buffer, __alias_tile_coords_buffer, cs_main
//
// ==RECONSTRUCTED== shaders/dof/analyze.cs.hlsl
// Evidence: _dof_analyze.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23, bound 444).
//   bindings: $Globals b13 {coc_tiles_tex_dims, coc_tiles_dilated_tex_dims},
//   coc_tiles_tex t14, coc_tiles_dilated_tex u15, tile_offsets_buffer t16
//   (StructuredBuffer<DofTileOffsets>), __alias_dispatch_args_buffer u17
//   (RWStructuredBuffer<uint4>), __alias_tile_coords_buffer u18
//   (RWStructuredBuffer<uint2>); caps Shader + RuntimeDescriptorArray +
//   StorageImageExtendedFormats + GroupNonUniform{Quad,Arithmetic,Ballot};
//   LocalSize [8,8,1], entry cs_main.
//   Tile analysis: Morton-ordered tile ids, CoC dilation (max abs CoC / min CoC),
//   per-quad classification into 7 tile classes gated on quad/wave agreement,
//   wave-aggregated category counts atomically appended to dispatch args, and
//   per-category tile-coord lists.  Status: O (structure from disassembly;
//   names from debug_names).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/hash.hlsl"
#include "inc/morton.hlsl"
#include "inc/coc.hlsl"
#include "tiles.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 coc_tiles_tex_dims;
    uint2 coc_tiles_dilated_tex_dims;
};

Texture2D<float4> coc_tiles_tex : register(t14, space0);
RWTexture2D<half> coc_tiles_dilated_tex : register(u15, space0);
StructuredBuffer<DofTileOffsets> tile_offsets_buffer : register(t16, space0);
RWStructuredBuffer<uint4> __alias_dispatch_args_buffer : register(u17, space0);
RWStructuredBuffer<uint2> __alias_tile_coords_buffer : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID,
             uint gl_LocalInvocationIndex : SV_GroupIndex)
{
    // Interleave (tile.x, tile.y) into a single Morton id (Z-order dispatch).
    uint x_part = gl_LocalInvocationIndex & 1431655765u;
    x_part = (x_part ^ (x_part >> 1u)) & 858993459u;
    x_part = (x_part ^ (x_part >> 2u)) & 252645135u;
    x_part = (x_part ^ (x_part >> 4u)) & 16711935u;
    uint y_part = (gl_LocalInvocationIndex >> 1u) & 1431655765u;
    y_part = (y_part ^ (y_part >> 1u)) & 858993459u;
    y_part = (y_part ^ (y_part >> 2u)) & 252645135u;
    y_part = (y_part ^ (y_part >> 4u)) & 16711935u;
    uint2 tile_pos = (gl_GlobalInvocationID.xy & uint2(0xFFFFFFF8u, 0xFFFFFFF8u))
        + uint2((x_part ^ (x_part >> 8u)) & 65535u, (y_part ^ (y_part >> 8u)) & 65535u);

    bool in_bounds = all(tile_pos < coc_tiles_tex_dims);

    // Dilation radius (a fraction of the tile-grid height).
    int radius = int(uint(min(16.0f, ceil(float(coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624359130859375f))));
    float max_abs_coc = 1e10f;
    float min_coc = 0.0f;
    for (int dy = -radius; dy <= radius; ++dy)
    {
        for (int dx = -radius; dx <= radius; ++dx)
        {
            uint2 sample_pos = tile_pos + uint2(dx, dy);
            if ((all(sample_pos < coc_tiles_tex_dims) && ((dx * dx) + (dy * dy)) < (radius * radius)))
            {
                float4 sample = coc_tiles_tex.Load(int3(sample_pos, 0u));
                float coc = sample.y;
                if ((coc + 8.0f) > (sqrt(float((dx * dx) + (dy * dy))) * 8.0f))
                {
                    max_abs_coc = max(max_abs_coc, coc);
                    min_coc = min(min_coc, sample.x);
                }
            }
        }
    }
    coc_tiles_dilated_tex[tile_pos] = min_coc;

    uint quad_all_small = uint(min_coc <= 7.0f);
    quad_all_small &= QuadReadAcrossX(quad_all_small);
    quad_all_small &= QuadReadAcrossY(quad_all_small);
    uint quad_all_big = uint(max_abs_coc > 11.0f);
    quad_all_big &= QuadReadAcrossX(quad_all_big);
    quad_all_big &= QuadReadAcrossY(quad_all_big);

    uint tile_class_count[7];
    tile_class_count[0] = 0u;
    tile_class_count[1] = 0u;
    tile_class_count[2] = 0u;
    tile_class_count[3] = 0u;
    tile_class_count[4] = 0u;
    tile_class_count[5] = 0u;
    tile_class_count[6] = 0u;

    if (in_bounds)
    {
        if (quad_all_small != 0u)
        {
            uint2 mod2 = tile_pos & uint2(1u, 1u);
            if (all(mod2 == uint2(0u, 0u)))
            {
                tile_class_count[6]++;
            }
            tile_class_count[(min_coc <= 5.0f) ? 2u : 1u]++;
        }
        else
        {
            uint2 mod2 = tile_pos & uint2(1u, 1u);
            if (all(mod2 == uint2(0u, 0u)))
            {
                if (quad_all_big != 0u)
                {
                    tile_class_count[5]++;
                    tile_class_count[3] += 4u;
                }
                else
                {
                    tile_class_count[4]++;
                    tile_class_count[0] += 4u;
                }
            }
        }
    }

    uint class_offsets[7];
    uint total0 = WaveActiveSum(tile_class_count[0]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[0].x, total0, prev);
        class_offsets[0] = tile_offsets_buffer[0].offsets[0] + prev;
    }
    uint total1 = WaveActiveSum(tile_class_count[1]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[1].x, total1, prev);
        class_offsets[1] = tile_offsets_buffer[0].offsets[1] + prev;
    }
    uint total2 = WaveActiveSum(tile_class_count[2]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[2].x, total2, prev);
        class_offsets[2] = tile_offsets_buffer[0].offsets[2] + prev;
    }
    uint total3 = WaveActiveSum(tile_class_count[3]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[3].x, total3, prev);
        class_offsets[3] = tile_offsets_buffer[0].offsets[3] + prev;
    }
    uint total4 = WaveActiveSum(tile_class_count[4]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[4].x, total4, prev);
        class_offsets[4] = tile_offsets_buffer[0].offsets[4] + prev;
    }
    uint total5 = WaveActiveSum(tile_class_count[5]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[5].x, total5, prev);
        class_offsets[5] = tile_offsets_buffer[0].offsets[5] + prev;
    }
    uint total6 = WaveActiveSum(tile_class_count[6]);
    if (WaveIsFirstLane())
    {
        uint prev;
        InterlockedAdd(__alias_dispatch_args_buffer[6].x, total6, prev);
        class_offsets[6] = tile_offsets_buffer[0].offsets[6] + prev;
    }

    class_offsets[0] = WaveReadLaneFirst(class_offsets[0]);
    class_offsets[1] = WaveReadLaneFirst(class_offsets[1]);
    class_offsets[2] = WaveReadLaneFirst(class_offsets[2]);
    class_offsets[3] = WaveReadLaneFirst(class_offsets[3]);
    class_offsets[4] = WaveReadLaneFirst(class_offsets[4]);
    class_offsets[5] = WaveReadLaneFirst(class_offsets[5]);
    class_offsets[6] = WaveReadLaneFirst(class_offsets[6]);
    class_offsets[0] += WavePrefixSum(tile_class_count[0]);
    class_offsets[1] += WavePrefixSum(tile_class_count[1]);
    class_offsets[2] += WavePrefixSum(tile_class_count[2]);
    class_offsets[3] += WavePrefixSum(tile_class_count[3]);
    class_offsets[4] += WavePrefixSum(tile_class_count[4]);
    class_offsets[5] += WavePrefixSum(tile_class_count[5]);
    class_offsets[6] += WavePrefixSum(tile_class_count[6]);

    if (in_bounds)
    {
        if (quad_all_small != 0u)
        {
            uint2 mod2 = tile_pos & uint2(1u, 1u);
            if (all(mod2 == uint2(0u, 0u)))
            {
                __alias_tile_coords_buffer[class_offsets[6]] = tile_pos / uint2(2u, 2u);
            }
            __alias_tile_coords_buffer[class_offsets[(min_coc <= 5.0f) ? 2u : 1u]] = tile_pos;
        }
        else
        {
            uint2 mod2 = tile_pos & uint2(1u, 1u);
            if (all(mod2 == uint2(0u, 0u)))
            {
                if (quad_all_big != 0u)
                {
                    __alias_tile_coords_buffer[class_offsets[5]] = tile_pos / uint2(2u, 2u);
                    __alias_tile_coords_buffer[class_offsets[3]] = tile_pos;
                    __alias_tile_coords_buffer[class_offsets[3] + 1u] = tile_pos + uint2(1u, 0u);
                    __alias_tile_coords_buffer[class_offsets[3] + 2u] = tile_pos + uint2(0u, 1u);
                    __alias_tile_coords_buffer[class_offsets[3] + 3u] = tile_pos + uint2(1u, 1u);
                }
                else
                {
                    __alias_tile_coords_buffer[class_offsets[4]] = tile_pos / uint2(2u, 2u);
                    __alias_tile_coords_buffer[class_offsets[0]] = tile_pos;
                    __alias_tile_coords_buffer[class_offsets[0] + 1u] = tile_pos + uint2(1u, 0u);
                    __alias_tile_coords_buffer[class_offsets[0] + 2u] = tile_pos + uint2(0u, 1u);
                    __alias_tile_coords_buffer[class_offsets[0] + 3u] = tile_pos + uint2(1u, 1u);
                }
            }
        }
    }
}
