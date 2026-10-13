// ==RECONSTRUCTED== shaders/light_binning/grid.inc.hlsl (candidate gi-deferred-b)
// Evidence: embedded as include in 13 bins (deferred_*_light_sun_sky x8,
// light_binning_bin_lights_to_cells x2, cell_prefix_merge/sum/subtotal);
// no OpName survives (fully inlined/folded). Grid layout inferred from the
// cell-mapping literals used by bin_lights_to_cells and the deferred PS
// (world scale 16/65 x 8/65 x 16/65 with +16,+2,+16 origin over 32x16x32
// cells, 64 lights per cell, packed cell word = count<<20 | first_index).
// Names use the BIN_LIGHT_GRID_ prefix to avoid clashing with the identically
// named constants lighting_indirect.hlsl declares for the deferred pass.
// Status: I (names/values Inference — constants fold to the same literals).

#ifndef TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
#define TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL

// Light-grid constants shared by the light-binning and deferred passes. The
// grid covers a ~130x32x130 world region as 32x16x32 cells (cell world size
// 65/16 x 65/8 x 65/16). A packed cell word stores the light count in bits
// 20..31 and the first grid-light-index entry in bits 0..19.
static const uint3 BIN_LIGHT_GRID_DIMS = uint3(32u, 16u, 32u);
static const uint BIN_LIGHT_GRID_MAX_LIGHTS = 64u;
static const float3 BIN_LIGHT_GRID_WORLD_SCALE = float3(0.24615384638309478759765625, 0.123076923191547393798828125, 0.24615384638309478759765625);
static const float3 BIN_LIGHT_GRID_OFFSET = float3(16.0, 2.0, 16.0);
static const float3 BIN_LIGHT_GRID_CELL_SIZE = float3(4.0625, 8.125, 4.0625);

#endif // TINYGLADE_LIGHT_BINNING_GRID_INC_HLSL
