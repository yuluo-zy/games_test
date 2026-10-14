// ==RECONSTRUCTED== shaders/light_binning/cell_prefix_merge.cs.hlsl
// 中文说明:
//   灯光装箱:把每 1024 字块的块总和(cell_prefix_sum 写入 subtotals_buf)加到
//   块内每个元素:inout[i] += subtotals[i / 1024]。证据来自
//   _light_binning_cell_prefix_merge.cs.hlsl_cs_main_0.bin(LocalSize 64)。
//   绑定:__alias_inout_buf u13(RWStructuredBuffer<uint>)、subtotals_buf t14
//   (StructuredBuffer<uint>);caps 仅 {Shader}。

#include "inc/common.hlsl"
#include "grid.inc.hlsl"
#include "inc/consts.hlsl"

RWStructuredBuffer<uint> __alias_inout_buf : register(u13, space0);
StructuredBuffer<uint> subtotals_buf : register(t14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    // 取出本线程所属 1024 字块的块前缀和,叠加回块内元素。
    uint block_total = subtotals_buf[gl_GlobalInvocationID.x / 1024u];
    __alias_inout_buf[gl_GlobalInvocationID.x] += block_total;
}
