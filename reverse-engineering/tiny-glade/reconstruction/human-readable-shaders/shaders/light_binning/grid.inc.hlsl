// ==RECONSTRUCTED== shaders/light_binning/grid.inc.hlsl
// 中文说明:
//   灯光装箱网格布局/常量(来自 _light_binning_bin_lights_to_cells 与
//   _deferred_*_light_sun_sky 的 spirv-cross)。证据:
//   - 灯笼影响球与 32x16x32 单元格 AABB 相交判定;
//   - 单元格 -> 线性下标 32*(32*y+z)+x(与 lighting_indirect.get_light_grid_index 一致);
//   - 世界缩放 0.246153846.../0.123076923...(单元格尺寸 4.0625/8.125/4.0625),
//     偏移 (16,2,16);
//   - 间接网格字的低 20 位 = 首灯下标,高 12 位 = 灯数量。
//   仅作 include 使用(从不作为主文件编译),不能 #include 任何东西(原闭包
//   无出边 include)。

#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL

// 灯光装箱网格分辨率:约 130x32x130 世界区域内 32x16x32 个单元格。
static const float3 GRID_WORLD_SCALE = float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f);
static const float3 GRID_WORLD_OFFSET = float3(16.0f, 2.0f, 16.0f);
static const uint3 GRID_DIMS = uint3(32u, 16u, 32u);
static const float3 GRID_CELL_SIZE = float3(4.0625f, 8.125f, 4.0625f);

// 世界位置 -> 网格单元格(向下取整并夹紧),与延迟 pass 的查找一致。
uint3 grid_world_pos_to_cell(float3 world_pos)
{
    return uint3(clamp(floor(world_pos * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                       float3(0.0f, 0.0f, 0.0f), float3(GRID_DIMS) - 1.0f));
}

// 单元格的线性下标(与 bin_lights 的 _alias_indirection_grid 寻址及
// lighting_indirect.get_light_grid_index 一致)。
uint grid_cell_index(uint3 cell)
{
    return 32u * (32u * cell.y + cell.z) + cell.x;
}

// 单元格的世界空间 AABB 边界。
float3 grid_cell_min_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET) * GRID_CELL_SIZE;
}

float3 grid_cell_max_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET + float3(1.0f, 1.0f, 1.0f)) * GRID_CELL_SIZE;
}

// 点到单元格 AABB 的(带符号)平方距离。返回值为正表示半径平方 `r2` 的
// 影响球与该单元格相交。
float grid_point_to_cell_dist2(float3 p, float r2, uint3 cell)
{
    float3 mn = grid_cell_min_world(cell);
    float3 mx = grid_cell_max_world(cell);
    float d2 = r2;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        if (p[i] < mn[i]) {
            float e = p[i] - mn[i];
            d2 -= e * e;
        } else if (p[i] > mx[i]) {
            float e = p[i] - mx[i];
            d2 -= e * e;
        }
    }
    return d2;
}

#endif // TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
