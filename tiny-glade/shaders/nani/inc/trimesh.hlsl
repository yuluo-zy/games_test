// ============================================================
// nani/inc/trimesh.hlsl
// 功能:楼梯-地板(stairs-floor)实例的三角网格块(trimesh chunk)解码头。
//       被楼梯地板剔除与 nani_stairs_floor 光栅化包含。
// 说明:仅提供 include 汇总(roof_material_id.inc.hlsl);实际的网格物理
//       读取(需要 Int64)在 body 阶段近似处理。
// ============================================================

// ==RECONSTRUCTED== shaders/nani/inc/trimesh.hlsl | evidence: embedded in stairs-floor cull +
// nani_stairs_floor raster; provides the trimesh chunk decode for stairs-floor instances
// (NaniTrimeshChunk from stairs_floor.inc: gpu_address / vertex_index_count /
// vertex_byte_offset / bounding_sphere) | status: O (closure); I (helper body approximated;
// physical mesh read not expressible without Int64)

#ifndef TINYGLADE_NANI_INC_TRIMESH_HLSL
#define TINYGLADE_NANI_INC_TRIMESH_HLSL

// 三角网格块解码所需屋顶材质 id 的 include 依赖。
#include "roof_material_id.inc.hlsl"

#endif // TINYGLADE_NANI_INC_TRIMESH_HLSL
