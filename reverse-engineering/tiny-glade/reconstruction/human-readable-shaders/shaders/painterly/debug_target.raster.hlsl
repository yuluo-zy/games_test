// ==RECONSTRUCTED== shaders/painterly/debug_target.raster.hlsl | evidence: _painterly_debug_target.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin — vs_main = fullscreen triangle (shared fullscreen_triangle_vert.inc.hlsl); ps_main = pure discard (OpKill, no binds, no $Globals); both variants identical | status: O (vs triangle + ps discard from inlined SPIR-V)
//
// 功能:画家风调试目标通道。顶点阶段是标准全屏三角形(vs_main,共享
// fullscreen_triangle_vert.inc.hlsl);像素阶段直接 discard,不输出任何颜色。
// 两个变体(b903e43ffb3da915 / f0adff76827e73ff)行为完全一致。

#include "inc/common.hlsl"            // 采样器(sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/uv.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

float4 ps_main() : SV_Target0
{
    // 调试目标:丢弃所有片段,不产生任何像素输出。
    discard;
    return float4(0.0f, 0.0f, 0.0f, 1.0f);
}
