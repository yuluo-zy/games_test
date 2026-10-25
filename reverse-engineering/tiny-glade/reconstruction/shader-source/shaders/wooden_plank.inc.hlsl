// ==RECONSTRUCTED== shaders/wooden_plank.inc.hlsl | F-V | embedded in nani_cull_and_bucket_wooden_plank.cs (2) + nani_wooden_plank.raster (2)
// Wooden-plank shared record type. Member layout/names recovered verbatim from the consumer
// bins' OpMemberName tables:
//   WoodenPlank: transform (Affine3Packed, 12 floats), gate_seed (uint), piece_seed (uint),
//     wall_id (uint), flags (uint)  (64-byte packed layout).
// Affine3Packed is defined in inc/utils.hlsl. Function bodies are Unknown in this pass
// (physical-addressing consumers; nani family refines).

#ifndef TINYGLADE_WOODEN_PLANK_INC_HLSL
#define TINYGLADE_WOODEN_PLANK_INC_HLSL

#include "inc/utils.hlsl"

// Per-plank instance record (planks hang from a gate/arch in the wooden wall).
struct WoodenPlank
{
    Affine3Packed transform;
    uint gate_seed;
    uint piece_seed;
    uint wall_id;
    uint flags;
};

#endif // TINYGLADE_WOODEN_PLANK_INC_HLSL
