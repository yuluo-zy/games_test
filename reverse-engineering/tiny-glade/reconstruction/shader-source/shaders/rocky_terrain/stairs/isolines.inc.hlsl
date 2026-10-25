// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/isolines.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   drives the iso-height contour extraction of a contouring-grid cell: the stair iso-level window
//   is level_min = min(ceil((minCornerHeight-0.9)*1.84782612323760986328125),16) to
//   level_max = max(floor((maxCornerHeight-0.7)*1.84782612323760986328125),0), and each level's
//   target height is lerp(0.8,10,level/16). For every level the 4-bit corner-state mask
//   (8>>corner, set when cornerHeight >= h) selects the marching-squares case; masks 0/15 (cell
//   fully below/above the level) are skipped. The name reflects that each level traces the terrain
//   contour ("isolines") through the cell.
// status: O (level window + mask loop verbatim from inlined main); I (function name — inlined)

#pragma once
#ifndef TINYGLADE_STAIRS_ISOLINES_INC_HLSL
#define TINYGLADE_STAIRS_ISOLINES_INC_HLSL

#include "marching_squares.inc.hlsl"

// Place a stair step on every terrain iso-line crossing this contouring-grid
// cell, one per stair iso-level in the cell's height window.
void place_stair_isolines(float2 corners[4], float4 corner_heights)
{
    float min_h = min(min(corner_heights.x, corner_heights.y), min(corner_heights.z, corner_heights.w));
    float max_h = max(max(corner_heights.x, corner_heights.y), max(corner_heights.z, corner_heights.w));

    int level_min = min(int(ceil((min_h - 0.9f) * 1.84782612323760986328125f)), 16);
    int level_max = max(int(floor((max_h - 0.7f) * 1.84782612323760986328125f)), 0);

    for (int level = level_min; level <= level_max; level++)
    {
        float h = lerp(0.8f, 10.0f, float(level) * 0.0625f);

        uint mask = 0u;
        for (int i = 0; i < 4; i++)
        {
            if (corner_heights[i] >= h)
                mask |= (8u >> uint(i));
        }

        if (mask == 0u || mask == 15u)
            continue;

        place_stair_case(mask, corners, h);
    }
}

#endif // TINYGLADE_STAIRS_ISOLINES_INC_HLSL
