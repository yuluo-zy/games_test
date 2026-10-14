// ==RECONSTRUCTED== shaders/nani/clear_subtotals.cs.hlsl | evidence: raw disasm of
// _nani_clear_subtotals.cs.hlsl_cs_main_0.bin: RWStructuredBuffer<uint> __alias_subtotals u13,
// push_constants.item_count, zero subtotals[x] for x<item_count | status: O
//
// nani 子合计清零:把 [0, item_count) 范围内的子合计元素写为 0。

#line 1 "shaders/nani/clear_subtotals.cs.hlsl"

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "draw_lists.inc.hlsl"
#include "inc/pack_unpack.hlsl"

RWStructuredBuffer<uint> __alias_subtotals : register(u13, space0);

struct PushConstants
{
    uint item_count;
};

[[vk::push_constant]]
PushConstants push_constants;

[numthreads(64, 1, 1)]
void cs_main(uint3 thread_id : SV_DispatchThreadID)
{
    if (thread_id.x < push_constants.item_count)
    {
        __alias_subtotals[thread_id.x] = 0u;
    }
}
