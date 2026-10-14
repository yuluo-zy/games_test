// PATCHED: modules ze.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
// === 中文说明 ===
// 散焦分析 (dof/analyze):把每个 tile 的最小/最大 CoC 沿圆形半径膨胀后写出到
//   coc_tiles_dilated_tex,再把 8x8 tile 的 2x2 quad 分类为近/远/两者桶,
//   用 wave 原子前缀和把 tile 坐标与调度计数追加进 tile_coords / dispatch_args。
// 入口: cs_main (compute, numthreads 8x8x1)。变体: TGV_0 的 cs_main 被 lifted。
// 绑定: coc_tiles_tex t14, coc_tiles_dilated_tex u15, tile_offsets_buffer t16,
//   dispatch_args_buffer u17, tile_coords_buffer u18, $Globals b13。
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/dof/analyze.cs.hlsl | A (dof-aa review, 2026-10-13): A winner (accumulator inits max=0/min=1e10, max-over-y written to dilated tex, near<=7 / far>11; B's inits inverted = behaviorally broken).
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
    // 用 morton 解码把组内线性索引映射回 8x8 的 tile 坐标。
    uint2 tile = (dispatch_id.xy & uint2(0xFFFFFFF8u, 0xFFFFFFF8u)) + morton_decode(local_index);
    bool in_bounds = all(tile < coc_tiles_tex_dims);

    // 膨胀半径:按 tile 数自适应(上限 16)。
    int radius = int(min(16.0f, ceil(float(coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624f)));
    float min_coc = 1.0e10f;
    float max_coc = 0.0f;
    // 圆形邻域内聚合最大/最小 CoC。
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
                // 仅统计落在当前 tile 圆内的 CoC(带误差带)。
                if ((coc + 8.0f) > (sqrt(float(dist_sq)) * 8.0f))
                {
                    max_coc = max(max_coc, coc);
                    min_coc = min(min_coc, c.x);
                }
            }
        }
    }
    // 写出膨胀后的最大 CoC。
    coc_tiles_dilated_tex[tile] = max_coc;

    // 2x2 quad 内做 AND 约简:判定整块是"近"还是"远"。
    uint near_quad = uint(max_coc <= 7.0f);
    near_quad = QuadReadAcrossX(near_quad) & near_quad;
    near_quad = QuadReadAcrossY(near_quad) & near_quad;
    uint far_quad = uint(min_coc > 11.0f);
    far_quad = QuadReadAcrossX(far_quad) & far_quad;
    far_quad = QuadReadAcrossY(far_quad) & far_quad;

    // 7 个桶的计数。
    uint counts[7];
    [unroll]
    for (uint b = 0u; b < 7u; ++b)
    {
        counts[b] = 0u;
    }

    // 分类:近桶(细/粗近景),或远桶(四邻写满),或中景桶。
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

    // wave 级原子前缀和:每桶拿到全局偏移,再展开到各 lane 的唯一槽位。
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

    // 按桶写 tile 坐标(远桶展开成 2x2 四个子 tile)。
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

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _dof_analyze.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// === 中文说明 ===
// TGV_0 变体的 cs 段:与手写 cs 语义一致(膨胀 + quad 分类 + wave 原子前缀和),
//   资源用 ByteAddressBuffer/Store 形式(变量名已语义化)。
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct DofTileOffsets
{
    uint offsets[8];
};

cbuffer $Globals : register(b13, space0)
{
    uint2 coc_tiles_tex_dims : packoffset(c0);
    uint2 coc_tiles_dilated_tex_dims : packoffset(c0.z);
};

ByteAddressBuffer tile_offsets_buffer : register(t16, space0);
RWByteAddressBuffer __alias_dispatch_args_buffer : register(u17, space0);
RWByteAddressBuffer __alias_tile_coords_buffer : register(u18, space0);
Texture2D<float4> coc_tiles_tex : register(t14, space0);
RWTexture2D<float> coc_tiles_dilated_tex : register(u15, space0);

static uint3 gl_GlobalInvocationID;
static uint gl_LocalInvocationIndex;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
    uint gl_LocalInvocationIndex : SV_GroupIndex;
};

void comp_main()
{
    // morton 解码:线性 lane 索引 -> 8x8 tile 内坐标。
    uint morton_x_a = gl_LocalInvocationIndex & 1431655765u;
    uint morton_x_b = (morton_x_a ^ (morton_x_a >> 1u)) & 858993459u;
    uint morton_x_c = (morton_x_b ^ (morton_x_b >> 2u)) & 252645135u;
    uint morton_x_d = (morton_x_c ^ (morton_x_c >> 4u)) & 16711935u;
    uint morton_y_a = (gl_LocalInvocationIndex >> 1u) & 1431655765u;
    uint morton_y_b = (morton_y_a ^ (morton_y_a >> 1u)) & 858993459u;
    uint morton_y_c = (morton_y_b ^ (morton_y_b >> 2u)) & 252645135u;
    uint morton_y_d = (morton_y_c ^ (morton_y_c >> 4u)) & 16711935u;
    uint2 tile = (gl_GlobalInvocationID.xy & uint2(4294967288u, 4294967288u)) + uint2((morton_x_d ^ (morton_x_d >> 8u)) & 65535u, (morton_y_d ^ (morton_y_d >> 8u)) & 65535u);
    bool in_bounds = all(bool2(tile.x < coc_tiles_tex_dims.x, tile.y < coc_tiles_tex_dims.y));
    // 膨胀半径。
    int radius = int(uint(min(16.0f, ceil(float(coc_tiles_tex_dims.y * 8u) * 0.0052083334885537624359130859375f))));
    int neg_radius = -radius;
    float min_coc;
    float max_coc;
    min_coc = 10000000000.0f;
    max_coc = 0.0f;
    float next_min_coc;
    float next_max_coc;
    // 圆形邻域聚合最小/最大 CoC(按行推进)。
    for (int dy = neg_radius; dy <= radius; min_coc = next_min_coc, max_coc = next_max_coc, dy++)
    {
        next_max_coc = max_coc;
        next_min_coc = min_coc;
        float next_max_dx;
        float next_min_dx;
        for (int dx = neg_radius; dx <= radius; next_max_coc = next_max_dx, next_min_coc = next_min_dx, dx++)
        {
            uint2 sample_tile = tile + uint2(int2(dy, dx));
            int dist_sq = (dy * dy) + (dx * dx);
            if (all(bool2(sample_tile.x < coc_tiles_tex_dims.x, sample_tile.y < coc_tiles_tex_dims.y)) ? (dist_sq < (radius * radius)) : false)
            {
                float4 c = coc_tiles_tex.Load(int3(sample_tile, 0u));
                float coc = c.y;
                float new_max_coc;
                float new_min_coc;
                if ((coc + 8.0f) > (sqrt(float(dist_sq)) * 8.0f))
                {
                    new_max_coc = max(next_max_coc, coc);
                    new_min_coc = min(next_min_coc, c.x);
                }
                else
                {
                    new_max_coc = next_max_coc;
                    new_min_coc = next_min_coc;
                }
                next_max_dx = new_max_coc;
                next_min_dx = new_min_coc;
            }
            else
            {
                next_max_dx = next_max_coc;
                next_min_dx = next_min_coc;
            }
        }
    }
    // 写出膨胀后的最大 CoC。
    coc_tiles_dilated_tex[tile] = max_coc;
    // 2x2 quad AND 约简:近/远判定。
    uint near = uint(max_coc <= 7.0f);
    uint near_across_x = QuadReadAcrossX(near);
    uint near_and_x = near & near_across_x;
    uint near_across_y = QuadReadAcrossY(near_and_x);
    uint near_quad = near_and_x & near_across_y;
    uint far = uint(min_coc > 11.0f);
    uint far_across_x = QuadReadAcrossX(far);
    uint far_and_x = far & far_across_x;
    uint far_across_y = QuadReadAcrossY(far_and_x);
    uint far_quad = far_and_x & far_across_y;
    // 7 个桶计数清零。
    uint counts[7];
    counts[0u] = 0u;
    counts[1u] = 0u;
    counts[2u] = 0u;
    counts[3u] = 0u;
    counts[4u] = 0u;
    counts[5u] = 0u;
    counts[6u] = 0u;
    // 分类:近桶 / 远桶(展开成 4 子 tile) / 中景桶。
    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            uint2 tile_parity = tile & uint2(1u, 1u);
            if (all(bool2(tile_parity.x == uint2(0u, 0u).x, tile_parity.y == uint2(0u, 0u).y)))
            {
                counts[6]++;
            }
            int bucket_index = (max_coc <= 5.0f) ? 2 : 1;
            counts[bucket_index]++;
        }
        else
        {
            uint2 tile_parity = tile & uint2(1u, 1u);
            if (all(bool2(tile_parity.x == uint2(0u, 0u).x, tile_parity.y == uint2(0u, 0u).y)))
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
    // 每个桶:wave 原子累加拿到全局起始,再读 tile 偏移。
    uint sum_count0 = WaveActiveSum(counts[0u]);
    uint sums[7];
    if (WaveIsFirstLane())
    {
        uint prev0;
        __alias_dispatch_args_buffer.InterlockedAdd(0, sum_count0, prev0);
        sums[0u] = tile_offsets_buffer.Load<uint>(0) + prev0;
    }
    uint sum_count1 = WaveActiveSum(counts[1u]);
    if (WaveIsFirstLane())
    {
        uint prev1;
        __alias_dispatch_args_buffer.InterlockedAdd(16, sum_count1, prev1);
        sums[1u] = tile_offsets_buffer.Load<uint>(4) + prev1;
    }
    uint sum_count2 = WaveActiveSum(counts[2u]);
    if (WaveIsFirstLane())
    {
        uint prev2;
        __alias_dispatch_args_buffer.InterlockedAdd(32, sum_count2, prev2);
        sums[2u] = tile_offsets_buffer.Load<uint>(8) + prev2;
    }
    uint sum_count3 = WaveActiveSum(counts[3u]);
    if (WaveIsFirstLane())
    {
        uint prev3;
        __alias_dispatch_args_buffer.InterlockedAdd(48, sum_count3, prev3);
        sums[3u] = tile_offsets_buffer.Load<uint>(12) + prev3;
    }
    uint sum_count4 = WaveActiveSum(counts[4u]);
    if (WaveIsFirstLane())
    {
        uint prev4;
        __alias_dispatch_args_buffer.InterlockedAdd(64, sum_count4, prev4);
        sums[4u] = tile_offsets_buffer.Load<uint>(16) + prev4;
    }
    uint sum_count5 = WaveActiveSum(counts[5u]);
    if (WaveIsFirstLane())
    {
        uint prev5;
        __alias_dispatch_args_buffer.InterlockedAdd(80, sum_count5, prev5);
        sums[5u] = tile_offsets_buffer.Load<uint>(20) + prev5;
    }
    uint sum_count6 = WaveActiveSum(counts[6u]);
    if (WaveIsFirstLane())
    {
        uint prev6;
        __alias_dispatch_args_buffer.InterlockedAdd(96, sum_count6, prev6);
        sums[6u] = tile_offsets_buffer.Load<uint>(24) + prev6;
    }
    // 广播首 lane 起始 + 组内前缀和 = 每个 lane 的唯一槽位。
    sums[0u] = WaveReadLaneFirst(sums[0u]);
    sums[1u] = WaveReadLaneFirst(sums[1u]);
    sums[2u] = WaveReadLaneFirst(sums[2u]);
    sums[3u] = WaveReadLaneFirst(sums[3u]);
    sums[4u] = WaveReadLaneFirst(sums[4u]);
    sums[5u] = WaveReadLaneFirst(sums[5u]);
    sums[6u] = WaveReadLaneFirst(sums[6u]);
    sums[0u] += WavePrefixSum(counts[0u]);
    sums[1u] += WavePrefixSum(counts[1u]);
    sums[2u] += WavePrefixSum(counts[2u]);
    sums[3u] += WavePrefixSum(counts[3u]);
    sums[4u] += WavePrefixSum(counts[4u]);
    sums[5u] += WavePrefixSum(counts[5u]);
    sums[6u] += WavePrefixSum(counts[6u]);
    // 按桶写 tile 坐标(远桶展开成 2x2 四个子 tile)。
    if (in_bounds)
    {
        if (near_quad != 0u)
        {
            uint2 tile_parity = tile & uint2(1u, 1u);
            if (all(bool2(tile_parity.x == uint2(0u, 0u).x, tile_parity.y == uint2(0u, 0u).y)))
            {
                __alias_tile_coords_buffer.Store<uint2>(sums[6] * 8 + 0, tile / uint2(2u, 2u));
            }
            __alias_tile_coords_buffer.Store<uint2>(sums[(max_coc <= 5.0f) ? 2 : 1] * 8 + 0, tile);
        }
        else
        {
            uint2 tile_parity = tile & uint2(1u, 1u);
            if (all(bool2(tile_parity.x == uint2(0u, 0u).x, tile_parity.y == uint2(0u, 0u).y)))
            {
                if (far_quad != 0u)
                {
                    __alias_tile_coords_buffer.Store<uint2>(sums[5] * 8 + 0, tile / uint2(2u, 2u));
                    __alias_tile_coords_buffer.Store<uint2>(sums[3] * 8 + 0, tile);
                    __alias_tile_coords_buffer.Store<uint2>((sums[3] + 1u) * 8 + 0, tile + uint2(1u, 0u));
                    __alias_tile_coords_buffer.Store<uint2>((sums[3] + 2u) * 8 + 0, tile + uint2(0u, 1u));
                    __alias_tile_coords_buffer.Store<uint2>((sums[3] + 3u) * 8 + 0, tile + uint2(1u, 1u));
                }
                else
                {
                    __alias_tile_coords_buffer.Store<uint2>(sums[4] * 8 + 0, tile / uint2(2u, 2u));
                    __alias_tile_coords_buffer.Store<uint2>(sums[0] * 8 + 0, tile);
                    __alias_tile_coords_buffer.Store<uint2>((sums[0] + 1u) * 8 + 0, tile + uint2(1u, 0u));
                    __alias_tile_coords_buffer.Store<uint2>((sums[0] + 2u) * 8 + 0, tile + uint2(0u, 1u));
                    __alias_tile_coords_buffer.Store<uint2>((sums[0] + 3u) * 8 + 0, tile + uint2(1u, 1u));
                }
            }
        }
    }
}

[numthreads(8, 8, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    gl_LocalInvocationIndex = stage_input.gl_LocalInvocationIndex;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
