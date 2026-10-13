// ==RECONSTRUCTED== shaders/light_binning/grid.inc.hlsl | status: O (grid layout/constants from spirv-cross of _light_binning_bin_lights_to_cells + _deferred_*_light_sun_sky)
// MERGE (gi-deferred review 2026-10-13): content adopted from candidate A (cell-index +
// min/max-world + point-to-cell helpers match the bin; B was constants-only). Residual
// flagged for follow-up: the bin clamps cell_min to (30,15,30) and cell_max to (33,16,33),
// while A clamps to (31,15,31)/(32,16,32) — both candidates differ from the bin.
// Evidence: lantern influence sphere vs 32x16x32 cell AABB overlap; cell -> linear
// index 32*(32*y+z)+x (matches lighting_indirect.get_light_grid_index); world scale
// 0.246153846.../0.123076923... (cell size 4.0625/8.125/4.0625) with offset (16,2,16);
// low20 bits of indirection-grid word = first light index, high bits = light count.
// Include-only file (never compiled as main); must not #include anything (original
// closure has no outgoing include edges).

#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL

// Light binning grid resolution: 32x16x32 cells over a ~130x32x130 world region.
static const float3 GRID_WORLD_SCALE = float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f);
static const float3 GRID_WORLD_OFFSET = float3(16.0f, 2.0f, 16.0f);
static const uint3 GRID_DIMS = uint3(32u, 16u, 32u);
static const float3 GRID_CELL_SIZE = float3(4.0625f, 8.125f, 4.0625f);

// World position -> grid cell (floor, clamped), matching the deferred pass lookup.
uint3 grid_world_pos_to_cell(float3 world_pos)
{
    return uint3(clamp(floor(world_pos * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                       float3(0.0f, 0.0f, 0.0f), float3(GRID_DIMS) - 1.0f));
}

// Linear index of a cell (matches _alias_indirection_grid addressing in bin_lights
// and lighting_indirect.get_light_grid_index).
uint grid_cell_index(uint3 cell)
{
    return 32u * (32u * cell.y + cell.z) + cell.x;
}

// Axis-aligned cell bounds in world space.
float3 grid_cell_min_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET) * GRID_CELL_SIZE;
}

float3 grid_cell_max_world(uint3 cell)
{
    return (float3(cell) - GRID_WORLD_OFFSET + float3(1.0f, 1.0f, 1.0f)) * GRID_CELL_SIZE;
}

// Squared distance from a point to the cell AABB. Positive means the point's
// influence sphere of radius squared `r2` overlaps the cell.
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
