#pragma once
// ======================================================================
// inc/perlin.hlsl —— 2D 值噪声("perlin")
// ======================================================================
// 来源:_leaf_particles/_paths_path_mask_visual/_water_plane/_rocky_terrain_displace
//       内联的 2D 值噪声(floor/frac/hash2D/smoothstep 双线性混合,
//       晶格值 = asfloat((h ^ (h>>18)) & 0x7FFFFF | 0x3F800000) - 1);
//       include 顺序表明 perlin.hlsl include hash.hlsl;133 个消费者 bin。
// 注意:这里的"perlin"是 2D *值* 噪声(哈希晶格而非梯度),用 smoothstep 曲线
//       f*f*(3-2f) 插值,输出 [0,1)。2D 哈希(1664525/1013904223 LCG 混合)
//       定义在本文件,不在 INC-A hash.hlsl。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/perlin.hlsl | evidence: _leaf_particles/_paths_path_mask_visual/_water_plane/_rocky_terrain_displace inlined 2D value-noise (floor/frac/hash2D/smoothstep bilinear blend, lattice value = asfloat((h ^ (h>>18)) & 0x7FFFFF | 0x3F800000) - 1); include order shows perlin.hlsl includes hash.hlsl (water_plane: perlin(10), hash(11)); 133 consumer bins | status: I
// MERGE (water-ice review inc-fix #4): perlin() rewritten to the explicit-arithmetic
// bilinear blend (multiply-add of the four lattice values, not lerp()) — the lerp body
// emits extra OpExtInst in dxc SPIR-V, while the water/ice family consumers
// (water_plane foam, water_snow_plane, water_x_paths, waterfall) embed the explicit
// asfloat(hash & 0x7FFFFF | 0x3F800000) - 1 bilinear/trilinear form.
//
// Observation: this "perlin" is 2D *value* noise (hash lattice, not gradient), interpolated with
// the smoothstep curve f*f*(3-2f), returning [0,1).  The 2D hash (LCG mix with 1664525/1013904223)
// is defined here (not in INC-A hash.hlsl).  Function name not embedded -> "perlin" Inference (alt: perlin_noise).

#include "hash.hlsl"

// 2D 整数哈希(1664525/1013904223 LCG 混合 + 最终雪崩式扩散),已在
// _leaf_particles/_paths_path_mask_visual/_water_plane/_rocky_terrain_displace
// 的 perlin 晶格与直接消费者抖动中内联观测到。不在 INC-A hash.hlsl 中
// (那里只有 hash(uint)、rand01 等),所以定义于此;若 INC-A 日后补充 hash(uint2)
// 需注意重名协调。
uint hash(uint2 p)
{
    uint y = p.y;
    y += (y << 10u);
    y ^= (y >> 6u);
    y += (y << 3u);
    y ^= (y >> 11u);
    uint h = ((p.x * 1664525u) + (y + (y << 15u))) + 1013904223u;
    h *= 1664525u;
    h ^= (h >> 11u);
    h ^= (h << 7u) & 2636928640u;
    h ^= (h << 15u) & 4022730752u;
    return h;
}

// 把整数哈希映射到 [0,1):取低 23 位尾数构造 1.0~2.0 再减 1。
float incb_perlin_hash01(uint h)
{
    return asfloat(((h ^ (h >> 18u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
}

// 2D 值噪声,输出 [0,1)。
float perlin(float2 p)
{
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);

    // 四个晶格角的值。
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));

    // 显式双线性混合(家族消费者嵌入的正是这种形式,不能用 lerp)。
    return (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
         + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
}
