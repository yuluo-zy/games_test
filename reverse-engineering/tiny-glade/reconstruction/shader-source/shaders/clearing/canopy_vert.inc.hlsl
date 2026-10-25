// ==RECONSTRUCTED== shaders/clearing/canopy_vert.inc.hlsl
// Evidence: embedded in the tree_canopy bins. Shared canopy vertex helpers used by
// tree_canopy.raster.hlsl (growth-scaled canopy placement and the canopy sway that
// combines the placement bend_vector with a time-driven bend). The placement record /
// transform live in clearing/inc/instanced_placement.hlsl and canopy_common.inc.hlsl.
// Status: O (placement/growth from use sites); I (helper names — inlined).

#ifndef TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL
#define TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL

// Canopy vertex position: local canopy vertex offset by the growth scale and the
// wind sway vector (placement bend_vector * sway_amount).
float3 canopy_vertex(float3 local_pos, float growth, float2 bend, float sway)
{
    return local_pos * growth + float3(bend.x * sway, 0.0f, bend.y * sway);
}

#endif // TINYGLADE_CLEARING_CANOPY_VERT_INC_HLSL
