// ==RECONSTRUCTED== shaders/smoke.inc.hlsl | F-V | embedded in nani_cull_and_bucket_smoke.cs (2) + nani_smoke.raster (2)
// Smoke-instance shared record type. Member layout/names recovered verbatim from the consumer
// bins' OpMemberName tables:
//   SmokeInstanceData: xform (Affine3Packed, 12 floats), smoke_type (uint), seed (uint),
//     thickness (float), pad0 (uint)  (64-byte packed layout).
// Affine3Packed is defined in inc/utils.hlsl (12-float column-major 3x4 transform).
// Function bodies are Unknown in this pass (physical-addressing consumers; nani family refines).

#ifndef TINYGLADE_SMOKE_INC_HLSL
#define TINYGLADE_SMOKE_INC_HLSL

#include "inc/utils.hlsl"

// Per-smoke-column instance record.
struct SmokeInstanceData
{
    Affine3Packed xform;
    uint smoke_type;
    uint seed;
    float thickness;
    uint pad0;
};

#endif // TINYGLADE_SMOKE_INC_HLSL
