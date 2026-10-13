// ==RECONSTRUCTED== shaders/nani/clear_histogram.cs.hlsl | nani-cull-a
// Observation: _nani_clear_histogram.cs.hlsl_cs_main_0.bin (LocalSize 64,1,1,
// binding u13 RWStructuredBuffer<uint> __alias_histogram). Body is a plain
// per-thread zero-store, no bounds guard (threads 64 == histogram width here).
#include "inc/common.hlsl"

RWStructuredBuffer<uint> __alias_histogram : register(u13, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    __alias_histogram[dispatchThreadID.x] = 0;
}
