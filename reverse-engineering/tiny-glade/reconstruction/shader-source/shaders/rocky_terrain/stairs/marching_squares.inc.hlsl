// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/marching_squares.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   the 4-bit corner-state mask (bit i set when corner i is at/above the target iso-height,
//   encoded as 8>>i, so mask 1 == corner 3, mask 8 == corner 0) selects the cell edges crossed by
//   the iso-line. Single-corner masks (1..4) and their complements (14..11) use the two edges
//   adjacent to the odd corner; two-adjacent-corner masks (3,6 and complements 12,9) use the two
//   edges between the high and low corners; the saddle masks (5,10) place a step on each of the two
//   disjoint segments. Edge pairing extracted verbatim from the inlined switch/if-else ladder of the
//   bin (each case = one or two place_stairs calls).
// status: O (case table verbatim from inlined dispatch); I (function name — inlined)

#pragma once
#ifndef TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL
#define TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL

#include "place_stairs.inc.hlsl"

// Dispatch the marching-squares case for a contouring-grid cell's corner-state
// mask: pick the crossing edges for this mask and place a stair step on each
// iso-line segment at height `h`.
void place_stair_case(uint mask, float2 corners[4], float h)
{
    switch (mask)
    {
    case 1:  case 14:
        place_stairs(corners[0], corners[3], corners[3], corners[2], h);
        break;
    case 2:  case 13:
        place_stairs(corners[1], corners[2], corners[3], corners[2], h);
        break;
    case 3:  case 12:
        place_stairs(corners[0], corners[3], corners[1], corners[2], h);
        break;
    case 4:  case 11:
        place_stairs(corners[0], corners[1], corners[1], corners[2], h);
        break;
    case 5:
        place_stairs(corners[0], corners[1], corners[0], corners[3], h);
        place_stairs(corners[1], corners[2], corners[3], corners[2], h);
        break;
    case 6:  case 9:
        place_stairs(corners[0], corners[1], corners[3], corners[2], h);
        break;
    case 7:  case 8:
        place_stairs(corners[0], corners[1], corners[0], corners[3], h);
        break;
    case 10:
        place_stairs(corners[0], corners[1], corners[1], corners[2], h);
        place_stairs(corners[0], corners[3], corners[3], corners[2], h);
        break;
    }
}

#endif // TINYGLADE_STAIRS_MARCHING_SQUARES_INC_HLSL
