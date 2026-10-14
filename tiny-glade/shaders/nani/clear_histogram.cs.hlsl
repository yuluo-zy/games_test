// ==RECONSTRUCTED== shaders/nani/clear_histogram.cs.hlsl | evidence: raw disasm of
// _nani_clear_histogram.cs.hlsl_cs_main_0.bin (clear_histogram family): body zeroes one
// histogram element per thread; local_size 64; binding u13 RWStructuredBuffer<uint>
// (type.RWStructuredBuffer.uint) | status: O (body, binding, entry, localsize)
//
// nani 直方图清零:每个线程把直方图的一个元素写为 0。

#line 1 "shaders/nani/clear_histogram.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"

RWStructuredBuffer<uint> __alias_histogram : register(u13, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    __alias_histogram[thread_id.x] = 0u;
}
