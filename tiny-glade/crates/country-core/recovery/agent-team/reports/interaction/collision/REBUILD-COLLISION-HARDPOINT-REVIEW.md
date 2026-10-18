# REVIEW — collision 剩余收口(REBUILD-COLLISION-HARDPOINT 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-collision-hardpoint REBUILD-COLLISION-HARDPOINT.md;默认 + native-physics 双构建
exit 0(独立复验)、rt 8 + collision 13 单测 PASS。

## 逐项复验

### ① rt 设备面类型修正 — PASS(fresh 锚定,旧登记纠正)
- `RtMesh`:0x50B repr(C),fresh 默认写序列 `{+0x00 scale=1.0, +0x04 0, +0x08 u64 0,
  +0x10 f64 1.0, +0x18 0, +0x20 0, +0x28 (1.0,1.0), +0x30 0, +0x38 0, +0x40 0, +0x48
  kind=4}`;旧 `{+0x08=(1.0,0.0), +0x38=4}` 与机器不符**已修**;scale/kind 语义锚定,
  其余 _unk 登记 ✓。
- `RtMeshHandle`:**16B {tag u64@+0, payload u64@+8}**(hash_one 键 2×u64 比较,
  impl$2::from={1,值});旧 1B 单元枚举错**已修** ✓。
- `DirtyCow`:**{inner Arc<T>@+0, dirty@+8}**(get_mut 写 +8 脏字节、读 +0 Arc);旧
  {dirty@+0, inner@+8} 顺序反**已修** ✓。
- SSBO 三缓冲名(upload_rt_world disasm LEA 0x142a9dc30/0x142a9cf75/0x142a9e726 +
  read_memory):`RtTrianglesSsbo`(0x2c)/`RtTriangleMetaSsbo`(0x2f)/`RtBvhSsbo`(0x26),
  元素 0x18/4/0x50 **锚定落地** ✓。
- `RtWorld::get_mut/get`(HashMap entry + or_insert_with + DirtyCow COW,哈希表内部登记)✓。

### ② update_rapier_colliders 行为骨架 — PASS(诚实登记,不伪造)
- hardpoint **未收敛**:ReVa --write 签名修复 `void(longlong*)`+save(h1 success),fresh 仍
  "Unable to find unique hash for varnode",High P-code 不可得(h2)——如实登记。
- 分段 disasm 4630 指令 → **13 段 call 序列**(puffin scope + 5 组件族 collider 生命周期 →
  upsert/dispose),骨架落盘 update_rapier_colliders-skeleton.md;41 callee 面备,
  后续 typed prototype 逐段判等 ✓。

### ③ find_mesh_mesh_intersections — PASS(域内构建块就绪,跨域登记)
- fresh 7783B 全量:域内 mesh 相交部分(图剪枝 take<ColliderDiff>→GraphMap::remove_node、
  逐节点 AABB ColliderSet::get_mut_internal+compute_aabb+SimdAabb::splat、候选槽 BVH
  traverse_depth_first_node+intersect_newly_added_colliders closure)——**构建块均已实现**
  (should_intersect_collider_meshes/intersect_mesh_mesh/merge_segments_into_polylines,
  REBUILD-DEEP 批 E)✓。
- 跨域消费登记:BirdNestCandidateLocation 类型未恢复、SparseInstanceBuffer<BrickTransformSsbo>
  折线铺砖(中点步进 0.5×100 取整)、SparseInstanceBufferAllocation<SparkleSsbo> drop、
  ColliderDiff 载荷未恢复→图剪枝入口待类型——全登记,不伪造 ✓。

## 结论与 DISPATCH 建议

- collision 剩余收口复验通过:**rt 设备面类型修正(3)+ RtWorld 实现 + find_mesh 构建块 + 
  update_rapier_colliders 行为骨架**,双构建 0 error(独立复验)。
- 建议:collision 剩余 → 闭合(实现层面 + 诚实登记);update_rapier_colliders 待 typed
  prototype 判等、find_mesh 待跨域类型(BirdNest/BrickTransformSsbo/ColliderDiff)、
  rt 设备面(update/upload_rt_world host 接线 + obvhs 入 workspace)由后续卡推进。
