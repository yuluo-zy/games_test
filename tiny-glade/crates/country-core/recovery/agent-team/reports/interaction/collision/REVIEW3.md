# REVIEW3 — systems/collision(R1 FIX 复验 → 闭合)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md 四.终验 FIX 节;默认 + `--features native-physics` 均 exit 0
(独立复验)。

## FIX 复验

### intersects_any(0x1408cc320)— **PASS(已补)**
- native-physics:`self.raycast(origin, dir, mask).is_some()` ✓(0x1408cc320 语义 =
  raycast 命中即真)。
- 非 native-physics 壳:返回 false ✓(与 raycast 壳一致,查询不落地)。
- 双构建编译 exit 0 ✓。

### 其余维持
SubworldColliderHandle::new / RaycastMeshData::new_convex / RaycastWorld 17 槽 / raycast
最近命中 / legacy_raycast 返回 None / mesh_intersect Publisher 返回 0(接线待续) / 非 native
类型壳——全部维持前轮结论,无回归。

## 结论与 DISPATCH 建议

- R1 FIX **复验通过**;**R1 → 闭合(核心查询族 + intersects_any)**。
- E7 ABI 对齐维持登记待做(共享 crate 消费者核完再改)。
- 双构建 0 error 已验(独立复验)。
