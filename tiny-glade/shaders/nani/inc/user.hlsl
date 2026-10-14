// ============================================================
// nani/inc/user.hlsl
// 功能:nani 系列"每用户"着色/实例共享 include。
//       内嵌在 52 个 nani 光栅/剔除 bin 中,汇总 hsv、楼梯地板、
//       砖墙 vs 输出等依赖。
// 说明:自身无独立 bin,body 阶段被内联,消费方各有差异。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/inc/user.hlsl | evidence: include-graph edge -> hsv.hlsl +
// stairs_floor.inc + wall/inc/brick_vs_out.hlsl; embedded in 52 nani raster/cull bins; role is
// the shared nani per-user shading/instance include | status: O (closure); I (body — inlined,
// no standalone bin; consumer-specific)

#ifndef TINYGLADE_NANI_INC_USER_HLSL
#define TINYGLADE_NANI_INC_USER_HLSL

// HSV 颜色转换、楼梯地板与砖墙 vs 输出的共享定义。
#include "inc/hsv.hlsl"
#include "stairs_floor.inc.hlsl"
#include "wall/inc/brick_vs_out.hlsl"

#endif // TINYGLADE_NANI_INC_USER_HLSL
