// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_merge.cs.hlsl | status: O (structure from disassembly)
// MERGE (gi-deferred review 2026-10-13): tie winner; adopted candidate A (structurally
// identical to B, histdiff 30, hard gate PASS).
// Evidence: _light_binning_cell_prefix_merge.cs.hlsl_cs_main_0.bin (LocalSize 64, entry cs_main).
//   bindings: __alias_inout_buf u13 (RWStructuredBuffer<uint>), subtotals_buf t14
//   (StructuredBuffer<uint>); caps {Shader} only.
//   Adds each 1024-word block's total (written by cell_prefix_sum into subtotals_buf)
//   to every element of the block: inout[i] += subtotals[i / 1024]. Includes:
//   common, grid.inc, consts.

#include "inc/common.hlsl"
#include "grid.inc.hlsl"
#include "inc/consts.hlsl"

RWStructuredBuffer<uint> __alias_inout_buf : register(u13, space0);
StructuredBuffer<uint> subtotals_buf : register(t14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint block_total = subtotals_buf[gl_GlobalInvocationID.x / 1024u];
    __alias_inout_buf[gl_GlobalInvocationID.x] += block_total;
}
