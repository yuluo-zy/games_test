#pragma once
// ======================================================================
// inc/scatter.hlsl —— 网格单元散布点辅助
// ======================================================================
// 来源:_path_pebbles.cs.hlsl_cs_main_0 内联的网格单元散布
//       (scatter_grid_dim,cell = index mod grid^2,
//        (float2(col,row)/grid - 0.5) * 104 + jitter*6.3)与
//       _generate_grass.cs.hlsl_cs_main_0 内联的 3x3 最近点搜索
//       (单元 2.5,seed = cx + cy*577,黄金比例抖动)。
// 说明:scatter_grid_dim 是消费者着色器声明的 $Globals cbuffer 成员,
//       scatter.hlsl 以全局引用它(不在此声明)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/scatter.hlsl | evidence: _path_pebbles.cs.hlsl_cs_main_0 inlined grid-cell scatter (scatter_grid_dim, cell = index mod grid^2, (float2(col,row)/grid - 0.5) * 104 + jitter*6.3) and _generate_grass.cs.hlsl_cs_main_0 inlined 3x3 nearest-point search (cell size 2.5, seed = cx + cy*577, golden-ratio jitter); raw-per-bin.jsonl debug_names "scatter_grid_dim" | status: U
//
// Observation: scatter_grid_dim is a $Globals cbuffer member declared by the consuming shaders;
// scatter.hlsl references it as a global (not declared here).  Two primitives are evidenced:
//   * scatter_point(index): grid-cell position + hash jitter  (_path_pebbles, world scale 104)
//   * a golden-ratio jittered point within a 2.5-unit cell   (_generate_grass nearest-point search)
// The full generate_grass algorithm (nearest-neighbor spacing + direction slerp) is only partially
// recovered; this file's exact function set/signatures are Unknown.  hash(uint)/rand01(uint) come
// from hash.hlsl (INC-A, reconstructed); include order puts hash.hlsl before scatter.hlsl.

// 网格单元位置 + 哈希抖动(_path_pebbles,世界尺度 104)。
float2 scatter_point(uint index)
{
    int grid = scatter_grid_dim;
    int area = grid * grid;
    uint cell = index % uint(area);
    float g = float(grid);
    float2 grid_pos = (float2(float(cell % uint(grid)), float(cell / uint(grid))) / g - 0.5f) * 104.0f;
    float2 jitter = float2(rand01(hash(index)),
                           rand01(hash(index ^ 188053769u))) * 6.3f;
    return grid_pos + jitter;
}

// 2.5 单位散布单元内的点,用黄金比例向量抖动(用于 _generate_grass 最近点
// 搜索;单元 seed = cx + cy*577)。
float2 scatter_cell_point(int2 cell, int seed)
{
    float2 p = float2(cell) * 2.5f;
    p += (frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f)
        * float(uint(seed) & 65535u)) + 0.5f) - 0.5f) * 2.5f;
    return p;
}
