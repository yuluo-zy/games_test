// ==RECONSTRUCTED== shaders/inc/lighting_indirect.hlsl | evidence: raw-per-bin cbuffer{spatial_resolve_offsets_array[512] int4}; spirv-cross (_deferred_*_light_sun_sky) light grid lookup: cell = clamp(floor(world_pos*float3(0.246..,0.123..,0.246..)+float3(16,2,16)),0,(31,15,31)), index=32*(32*cell.y+cell.z)+cell.x, light_indirection_grid.Load<uint>(idx*4): low20 = first grid_light_indices entry, >>20 = light count, grid_light_indices.Load<uint>(n*4)=lantern index | status: O (grid layout/cbuffer); O/I (helper names Inference)
// FIX (gi-deferred merge 2026-10-13, review inc-fix): the `#include "lanterns.hlsl"` edge is
// spurious — the original closure embeds lighting_indirect.hlsl WITHOUT lanterns.hlsl in the
// gi trace/update bins (both embed lighting_indirect, neither embeds lanterns). Deferred
// closures get lanterns from deferred/light_sun_sky.inc.hlsl directly. Edge removed.

#ifndef TINYGLADE_INC_LIGHTING_INDIRECT_HLSL
#define TINYGLADE_INC_LIGHTING_INDIRECT_HLSL

// Spatial-resolve neighborhood offsets (ring 0..15 plus an outer ring) used by
// the deferred indirect-light gather. Consumer binds at its own set0 slot.
cbuffer spatial_resolve_offsets : register(b24, space0) {
    int4 spatial_resolve_offsets_array[512];
};

// Light grid resolution: 32x16x32 cells over a ~130x32x130 world region.
static const float3 LIGHT_GRID_WORLD_SCALE = float3(0.24615384638309478759765625, 0.123076923191547393798828125, 0.24615384638309478759765625);
static const float3 LIGHT_GRID_OFFSET = float3(16.0, 2.0, 16.0);
static const uint3 LIGHT_GRID_DIMS = uint3(32, 16, 32);

// World position -> light grid cell (clamped).
uint3 get_light_grid_cell(float3 world_pos)
{
    uint3 cell = uint3(clamp(floor(world_pos * LIGHT_GRID_WORLD_SCALE + LIGHT_GRID_OFFSET),
                             float3(0.0, 0.0, 0.0), float3(LIGHT_GRID_DIMS - 1u)));
    return cell;
}

// Linear grid index for a cell.
uint get_light_grid_index(uint3 cell)
{
    return (LIGHT_GRID_DIMS.x * ((LIGHT_GRID_DIMS.y * cell.y) + cell.z)) + cell.x;
}

// Read the [start, count) range of lantern indices for a cell: low 20 bits =
// first index into grid_light_indices, bits 20+ = count.
uint2 get_light_cell_range(ByteAddressBuffer light_indirection_grid, uint3 cell)
{
    uint packed = light_indirection_grid.Load<uint>(get_light_grid_index(cell) * 4u + 0u);
    return uint2(packed & 1048575u, min(64u, packed >> 20u));
}

// Resolve a lantern index from the grid index buffer.
uint get_light_grid_lantern_index(ByteAddressBuffer grid_light_indices, uint index)
{
    return grid_light_indices.Load<uint>(index * 4u + 0u);
}

#endif // TINYGLADE_INC_LIGHTING_INDIRECT_HLSL
