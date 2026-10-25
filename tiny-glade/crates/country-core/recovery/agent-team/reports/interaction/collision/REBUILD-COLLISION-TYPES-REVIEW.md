# REVIEW — collision 类型族收口(REBUILD-COLLISION-TYPES 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-collision-types REBUILD-COLLISION-TYPES.md(类型落地 16 + 函数实现 12/16 入口);
默认 + `--features native-physics` 双构建 exit 0(独立复验,13/9 警告均既有)、单测 16 PASS
+ utils 89 PASS。

## 逐项复验

### ① 类型落地 16 — PASS
- `IsoTransform`(utils crate,{translation Vec3@+0x00, rotation Quat@+0x10},nested_in 239B +
  from_translation_rotation 31B;+0x20..+0x2f 预留区/嵌套乘积语义登记)✓。
- `OffsetColliderTransform`(平移@+0x00/旋转@+0x0c/shape Arc@+0x20;repr(Rust) 对齐与机器偏移
  不同,登记)✓。
- `RaycastMeshKey`(slotmap SlotMap<RaycastMeshKey, RaycastMesh>)/`MeshInstanceSources`(@+0x3dc8、
  0xe0B 默认副本、Vec<Slot>@+0x3ea8/+0x3eb0,字段偏移登记)✓。
- `Polyline`/`PolylinePoint`/`VertexId`/`Segment`(intersect_meshes.rs,0x28B/条 + VecDeque 环缓冲
  布局登记)✓。
- `RtMeshHandle`/`RtMesh`/`DirtyCow`/`RtWorld`/`GpuRtTriangle`/`RtBvhBuilderGlue`(新 resources/
  rt.rs;RtMesh 字段语义未锚定,登记)✓。

### ② 函数实现 12/16 入口 — PASS(结构判等)
- `on_level_reload`(0x1408d8710):全量 RaycastWorld::new() 重建(+0x10/+0x1c 计数器写登记)+
  测试 ✓。
- `to_offset_collider_builder`×3:ColliderBuilder::new(shape).position(iso).sensor(true) ✓。
- `into_transformed_shape`×4 + `as_collider_shape`:Arc 克隆/Rectangle3d 取向
  from_rotation_arc 登记 ✓。
- `iter_shape_overlaps`×4 + `shape_overlaps_with` + `decorator_overlaps_with_others`:mask 门控 +
  intersections_with_shape(rapier 0.22 FnMut(ColliderHandle),source 由槽号映射)+
  DecoratorRank 校验登记 ✓。
- `visualize_colliders` + `visualize_shape`:for_each_collider 全槽 + (index,gen) 校验 +
  **kind×DebugGui+0x104 过滤(11 分支逐条)**;compound/trimesh/convex/cuboid/ball 分派 + 三角
  线框 [a,b,c,a] 4 点闭环 + DebugLine/DebugSphere 请求清单(show ECS 胶水登记 host)✓。
- `merge_segments_into_polylines`:HashMap<IVec3,VertexId> 端点查表 + Polyline(VecDeque)
  初建/追加/拼接/闭环 + 中点平均,方向整理与环缓冲布局登记 ✓。
- rt 纯逻辑:is_idle(RwLock+poison+pending==0)/from×3(Vec3→u32 颜色 sqrt*255+0.5 打包)✓。

### ③ 剩余登记 — PASS(诚实)
- `update_rapier_colliders`:decompile-hardpoint(--write worker 修 void()→longlong*+save,fresh
  仍 varnode U,AI_REVERSE_TOOLS §7 同类),callee 面 41 个已落盘——登记待 hardpoint 攻克 ✓。
- `find_mesh_mesh_intersections`(13359B):GraphMap 装配 + BirdNestCandidateLocation/
  SparseInstanceBuffer<SparkleSsbo>/ColliderDiff 跨域(render/bird-nest),登记 ✓。
- rt 设备面:update_rt_world/upload_rt_world(需 rhapsody GraphicsDevice + SsboId FrozenMap,
  host 面)、build_gpu_bvh/bvh_builder_worker(需 obvhs crate,workspace 未引入)、RtWorld::
  get_mut/get(RtMesh 字段语义未锚定)——全登记,不伪造 ✓。

### ④ utils 共享基础改动范围 — PASS
IsoTransform 为 utils crate 新增模块(嵌套平移组合,字段语义登记),改动范围限于 utils +
collision/rt 消费,未涉他模块 ✓。

## 结论与 DISPATCH 建议

- collision 类型族收口复验通过:**16 类型落地 + 12 函数实现(16 入口中重叠计入)闭合**,双构建
  0 error(独立复验)。
- 建议:collision 类型族 → 闭合(实现层面);剩余(update_rapier_colliders hardpoint 攻克、
  find_mesh_mesh_intersections 跨域类型、rt 设备面 obvhs/rhapsody 接线、RtMesh 字段语义)由
  后续卡推进。
