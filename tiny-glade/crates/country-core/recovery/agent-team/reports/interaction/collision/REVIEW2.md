# REVIEW2 — systems/collision(R1 终验 → 核心查询族)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md 二.R1 节 + 三.E7 节;默认 + `--features native-physics` 均 exit 0
(独立复验)。

## 核对

### ① SubworldColliderHandle / RaycastMeshData / RaycastWorld — PASS
- `SubworldColliderHandle::new`(0x1408cc620):{index u32@0, gen u32@4, slot u8@8},空判别
  0x11(Default slot=0x11)✓。
- `RaycastMeshData::new_convex`(0x1408cec10):parry `SharedShape::convex_hull`,Some 写 tag=4、
  Arc<SharedShape>(0xd0 B 载荷)、hash@+0x70 ✓。
- `RaycastWorld`:17 槽 ColliderSet/RigidBodySet/QueryPipeline/dirty_tiles(native-physics)✓;
  `raycast`(0x1408cc870)mask 遍历 + `cast_ray_and_get_normal` 最近命中 ✓(结构)。
- `legacy_raycast`(0x1408cc680)返回 None(mask=0 遗留语义)、`mesh_intersect_collider_diff_
  publisher`(0x1408cc670)返回 0(真实 tabloid Publisher 字段@+0x3cb0 接线**待续**,诚实)✓。

### ② Raycasting 三件 — **FIX(小,restorer 自述超实现)**
- `raycast`(0x1408cc2e0):native-physics 转发 ✓;非 native-physics 壳返回 None ✓。
- `spherecast`(0x1408cc390):桩返回 None,**PounceLight rapier fork 球体 API 未定待取证**
  (诚实)✓。
- **`intersects_any`(0x1408cc320):RESTORE R1 列在落地清单,但 world.rs 无此函数**
  (grep 空)。属自述超实现——需补实现(转发 raycast 命中)或移入待续登记。FIX 项。

### ③ 非 native-physics 默认壳 — PASS
`#[cfg(not(feature="native-physics"))]` 下 raycast 返回 None,convex 字段 cfg 门控——查询
不落地,类型壳确认 ✓。

### ④ E7 ABI 对齐登记 — PASS(诚实)
原 EditTerrainStrokeCmd=0x28B{falloff,height,position,id} id@+0x1c、Adjustment 紧凑打包
{disc u32,[value 仅非 Keep]}、falloff.value 0.002 灵敏度、+0x20 u64 事件 id——**登记待做,
明确"涉及共享 crate 改动,需先核现有消费者(edit_terrain_stroke)再改"**,未低预算仓促;
position y/z 语义名/+0x20 与 EventInstance id/SetTerrainStrokePtsCmd 布局 U ✓。

### ⑤ 待续登记 — PASS
dirty 瓦片生命周期(flush/publish/QueryPipeline::update)、collider upsert/remove 泛型族、
parry 网格构造、intersect_meshes(21)/visualize_colliders(6)——全登记 ✓。

## 结论与 DISPATCH 建议

- R1 核心查询族(handle/convex/world/raycast)实现与机器码对得上;唯一 FIX = `intersects_any`
  自述超实现(RESTORE 需补实现或移入待续)。
- 建议:R1 → **fix_pending**(补 intersects_any 或改 RESTORE 登记);修后复验。E7 ABI 对齐
  维持登记待做(共享 crate 消费者核完再改)。默认 + native-physics 双构建 0 error 已验。
