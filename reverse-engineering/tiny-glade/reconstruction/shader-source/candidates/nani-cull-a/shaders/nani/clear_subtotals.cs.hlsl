// ==RECONSTRUCTED== shaders/nani/clear_subtotals.cs.hlsl | nani-cull-a
// Observation: _nani_clear_subtotals.cs.hlsl_cs_main_0.bin (LocalSize 64,1,1,
// u13 RWStructuredBuffer<uint> __alias_subtotals, push_constants {item_count}).
// Body: zero each subtotal slot while gid.x < item_count (64 threads cover the
// 64 subtotal slots of the scan window).
#include "inc/common.hlsl"

struct PushConstants
{
    uint item_count;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

RWStructuredBuffer<uint> __alias_subtotals : register(u13, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x < push_constants.item_count)
    {
        __alias_subtotals[dispatchThreadID.x] = 0;
    }
}
