// ======================================================================
// inc/lighting_indirect.hlsl —— 延迟间接光栅格查找公共头文件
// ======================================================================
// 用途:被 _deferred_*_light_sun_sky 用于查找世界位置附近的灯光(灯笼)栅格。
// 结构:32x16x32 栅格覆盖 ~130x32x130 世界区域;light_indirection_grid 存每个
//       栅格单元的 [起始索引, 灯光数](低 20 位索引,高 12 位数量)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/lighting_indirect.hlsl | evidence: raw-per-bin cbuffer{spatial_resolve_offsets_array[512] int4}; spirv-cross (_deferred_*_light_sun_sky) light grid lookup: cell = clamp(floor(world_pos*float3(0.246..,0.123..,0.246..)+float3(16,2,16)),0,(31,15,31)), index=32*(32*cell.y+cell.z)+cell.x, light_indirection_grid.Load<uint>(idx*4): low20 = first grid_light_indices entry, >>20 = light count, grid_light_indices.Load<uint>(n*4)=lantern index | status: O (grid layout/cbuffer); O/I (helper names Inference)
// FIX (gi-deferred merge 2026-10-13, review inc-fix): the `#include "lanterns.hlsl"` edge is
// spurious — the original closure embeds lighting_indirect.hlsl WITHOUT lanterns.hlsl in the
// gi trace/update bins (both embed lighting_indirect, neither embeds lanterns). Deferred
// closures get lanterns from deferred/light_sun_sky.inc.hlsl directly. Edge removed.

#ifndef TINYGLADE_INC_LIGHTING_INDIRECT_HLSL
#define TINYGLADE_INC_LIGHTING_INDIRECT_HLSL

// 空间解析邻域偏移(环 0..15 加一个外环),用于延迟间接光收集。
// 消费者在各自的 set0 槽位绑定。
cbuffer spatial_resolve_offsets : register(b24, space0) {
    int4 spatial_resolve_offsets_array[512];
};

// 灯光栅格分辨率:32x16x32 个单元,覆盖 ~130x32x130 世界区域。
static const float3 LIGHT_GRID_WORLD_SCALE = float3(0.24615384638309478759765625, 0.123076923191547393798828125, 0.24615384638309478759765625);
static const float3 LIGHT_GRID_OFFSET = float3(16.0, 2.0, 16.0);
static const uint3 LIGHT_GRID_DIMS = uint3(32, 16, 32);

// 世界位置 -> 灯光栅格单元(截断)。
uint3 get_light_grid_cell(float3 world_pos)
{
    uint3 cell = uint3(clamp(floor(world_pos * LIGHT_GRID_WORLD_SCALE + LIGHT_GRID_OFFSET),
                             float3(0.0, 0.0, 0.0), float3(LIGHT_GRID_DIMS - 1u)));
    return cell;
}

// 单元 -> 线性栅格索引。
uint get_light_grid_index(uint3 cell)
{
    return (LIGHT_GRID_DIMS.x * ((LIGHT_GRID_DIMS.y * cell.y) + cell.z)) + cell.x;
}

// 读取一个单元的灯光 [起始, 数量) 范围:低 20 位 = grid_light_indices 的首索引,
// 第 20 位以上 = 数量。
uint2 get_light_cell_range(ByteAddressBuffer light_indirection_grid, uint3 cell)
{
    uint packed = light_indirection_grid.Load<uint>(get_light_grid_index(cell) * 4u + 0u);
    return uint2(packed & 1048575u, min(64u, packed >> 20u));
}

// 从栅格索引缓冲解析出灯笼索引。
uint get_light_grid_lantern_index(ByteAddressBuffer grid_light_indices, uint index)
{
    return grid_light_indices.Load<uint>(index * 4u + 0u);
}

#endif // TINYGLADE_INC_LIGHTING_INDIRECT_HLSL
