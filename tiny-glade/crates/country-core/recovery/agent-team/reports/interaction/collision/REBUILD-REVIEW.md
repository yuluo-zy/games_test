# REVIEW — systems/collision 真实逻辑重建(rebuild-collision 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-collision REBUILD.md(遵循 planning/2026-10-16-logic-verify-protocol);
默认 + `--features native-physics` 双构建 exit 0(独立复验)。

## 逐项复验

### ① 分类(10 全真业务,0 宏胶水)— PASS
RaycastWorld::new(布局构造器)/mesh_intersect_collider_diff_publisher(访问器 `lea rax,
[rcx+0x3cb0]; ret`)/SubworldIndex::from_usize_unchecked(`mov rax,rcx; ret` 无检查包装)/
Raycasting::{raycast,intersects_any,spherecast}/RaycastWorld::{raycast,legacy_raycast}/
RaycastMeshData::new_convex/collision.rs impl$0::build——均按业务重建;Default for
RaycastWorld/RaycastMeshData 保持 glue(委托 new)。**无宏胶水误标** ✓。

### ② raycast 跨槽最后一次命中覆盖 + u32 mask + RaycastWorldResult — PASS
- `Rapier::raycast`(0x1408cdc60):17 槽 **u32** mask 遍历、每槽 cast_ray_and_get_normal
  (solid=1),**最后一次命中覆盖,无 toi 比较**(fresh 证实)✓。
- `RaycastWorldResult` {handle[3]@+0x00, position@+0x18, normal@+0x24, toi@+0x30,
  discriminant@+0x34(0=命中/2=未中), extra@+0x38} + miss() ✓。
- `RaycastWorld::raycast`(0x1408cc870)转发 Rapier::raycast(mask u32)✓;legacy_raycast
  (0x1408cc680)未命中把 2 写 +0x30(toi 槽,legacy 特例)✓。

### ③ Raycasting 转发/短路/构造/布局/build — PASS
- `Raycasting::raycast`(0x1408cc2e0)转发 `raycast_w_terrain`(0x1408cd430),mask&0x20 →
  terrain.intersect_ray 地形 0x20 分支 ✓。
- `intersects_any`(0x1408cc320)地形短路 + 17 槽 Qbvh `traverse_depth_first_node` +
  RayIntersectionsVisitor,命中即 true ✓。
- `new_convex`(0x1408cec10):`calculate_hash(&points)`→hash、`try_convex_hull`+
  `ConvexPolyhedron::from_convex_mesh`→`Box<dyn Shape>`,入参 `&[glam::UVec3]`,None 写判别
  0 ✓。
- `RaycastWorld`:`[RapierSubWorld; 17]` 数组 + `publisher: Publisher<ColliderDiff>`(0x3cb0)
  ✓;精确字节偏移为近似(rapier fork 类型尺寸未逐字段锚定,记分歧说明,诚实)。
- `CollisionPlugin::build`(0x140b685e0):insert_resource(RaycastWorld::new() +
  MeshIntersectionGraph::default());MeshIntersectionArtifacts 载荷空(待取证)✓;Startup/
  Update/OnExit 注册依赖初始化系统,登记待续(诚实)。

### ④ roof_shape.rs 跨模块最小改动 — PASS(语义不变)
两消费点(`roof_uv_and_point_at_ws_pos`/`roof_uv_at_ws_pos`)`world.raycast(ws_pos,
Vec3::NEG_Y, u32::MAX)` + `hit.is_miss()` + `hit.toi`;mask u64::MAX→u32::MAX、?/解构→
is_miss() 判别,向下射线取 toi 业务语义不变 ✓。

### ⑤ 待续登记诚实 — PASS
rapier 三角网格 barycentric 法线插值(0x1408ce4a0 非三角路径已实现,三角路径待续)、
+0x3d80 Qbvh、ColliderDiff 载荷、Startup/Update/OnExit 初始化系统注册——全登记,无冒充。

## 结论与 DISPATCH 建议

- collision 真实逻辑重建复验通过:**需重做项(9+1)全部闭合**,双构建 0 error。
- 建议:collision 需重做项 → 闭合;barycentric/+0x3d80 Qbvh/ColliderDiff 载荷/初始化系统
  注册归后续卡。cross-module roof_shape 机械适配语义不变确认。
