// ==RECONSTRUCTED== shaders/inc/debug_utils.hlsl | evidence: OpLine attribution shows only ~12 ops total across all bins (one consumer: _nani_generate_dispatch_args); included by 402 bins via samplers.hlsl -> debug_utils.hlsl; include-graph edge debug_utils.hlsl -> tracked_buffer.hlsl | status: I/U (names/bodies Inference — the file is almost always empty at codegen; helpers below are the minimal debug-write surface)
//
// Debug visualization helpers. Nearly always no-op'd at codegen (only one
// consumer produces code), but kept as a small utility surface.

#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#include "tracked_buffer.hlsl"

// Encode a scalar in [0,1] as a debug heat-map color (blue -> red).
float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif // TINYGLADE_INC_DEBUG_UTILS_HLSL
