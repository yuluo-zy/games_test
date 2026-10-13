// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_merge.cs.hlsl (candidate gi-deferred-b)
// Evidence: _light_binning_cell_prefix_merge.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: __alias_inout_buf u13 (RWStructuredBuffer<uint>), subtotals_buf t14
//   (StructuredBuffer<uint>), LocalSize [64,1,1], entry cs_main.
//   disassembly: per thread i, adds the exclusive block prefix
//   subtotals_buf[i/1024] into __alias_inout_buf[i] (1024-element blocks,
//   matching cell_prefix_sum). Third stage of the light-grid prefix chain.
// Status: O (structure from disassembly).

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "grid.inc.hlsl"

RWStructuredBuffer<uint> __alias_inout_buf : register(u13, space0);
StructuredBuffer<uint> subtotals_buf : register(t14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    __alias_inout_buf[globalThreadID.x] = __alias_inout_buf[globalThreadID.x] + subtotals_buf[globalThreadID.x / 1024u];
}
