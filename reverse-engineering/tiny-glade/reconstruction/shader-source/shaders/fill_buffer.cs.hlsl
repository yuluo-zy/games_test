// ==RECONSTRUCTED== shaders/fill_buffer.cs.hlsl | F-D
// Evidence: _fill_buffer.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: push_constants (PushConstant storage, cbuffer, members value+range),
//   __alias_output u13/space0 (StorageBuffer uint[]), LocalSize [64,1,1], entry cs_main.
//   spirv-cross: if (gid.x < range) _alias_output.Store<uint>(gid.x*4, value).
// Status: O (storage class, member names `value`/`range` verbatim from OpMemberName).

#include "inc/common.hlsl"

struct PushConstants
{
    uint value;
    uint range;
};

[[vk::push_constant]]
ConstantBuffer<PushConstants> push_constants;

RWStructuredBuffer<uint> __alias_output : register(u13, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    if (dispatchThreadID.x < push_constants.range)
    {
        __alias_output[dispatchThreadID.x] = push_constants.value;
    }
}
