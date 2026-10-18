# REBUILD-COLLISION-TYPES — country-core collision 变换/rt 类型族收口

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
依据: reports/impl-unimplemented/REBUILD-UNIMPLEMENTED.md 批 1（collision monolith 33 登记）
证据: planning/impl-collision-{deep,remainder}/results/b{1..8}*.result.json（2026-10-17 fresh）+
      本轮 ghidra shared 0x140c91b30/60/80、0x1408d0490/0x1408d7ed0/0x1408e8d70 fresh
      （2026-10-17 12:57）+ `--write` worker `impl-collision-types` 定点签名修复
      （0x1408d0490，save 证据见 evidence/toolchain 会话落盘）。
编译: 两特征（default + native-physics）`cargo check -p country-core --lib` 0 error
      （并行 agent WIP 曾间歇阻塞 decorator_edit.rs，非本模块）；本批 16 单测 PASS。

## 批 1 — 缺类型落地

| 类型 | 落点 | 布局锚定证据 | verdict |
|---|---|---|---|
| `IsoTransform` | utils crate `src/iso_transform.rs` | `from_translation_rotation`（0x140c91b60）+ `nested_in`（0x140c91b80）：平移 @+0x00、旋转 @+0x10 | 落地（+0x20..+0x2f 预留区与 +0x10 复刻语义登记） |
| `OffsetColliderTransform` | world.rs | `into_transformed_shape` impl$42（0x1408d98c0）/impl$45（0x140b26d20）store 序列：平移@+0x00、旋转@+0x0c、shape Arc@+0x20 | 落地（repr(Rust) 对齐与机器偏移不同，登记） |
| `RaycastMeshKey` | world.rs | `slotmap::basic::SlotMap<RaycastMeshKey, RaycastMesh>`（new_key_type!） | 落地 |
| `MeshInstanceSources` | world.rs | `on_level_reload`（0x1408d8710）：@+0x3dc8、0xe0 B 默认副本、`Vec<Slot>` @+0x3ea8/+0x3eb0 | 落地（精确字段偏移登记） |
| `Polyline`/`PolylinePoint`/`VertexId`/`Segment` | intersect_meshes.rs | `merge_segments_into_polylines`（0x1408f2850）：0x28 B/条、PolylinePoint 0x18 B、HashMap<IVec3,VertexId> | 落地（VecDeque 环缓冲字节布局登记） |
| `RtMeshHandle`/`RtMesh`/`DirtyCow`/`RtWorld`/`GpuRtTriangle`/`RtBvhBuilderGlue(R)` | resources/rt.rs（新建） | `update_rt_world`/`upload_rt_world`/`RtWorld::get_mut`（0x1415626f0 默认写序列）/`is_idle`（0x1408bc530） | 落地（RtMesh 字段语义未锚定，登记） |
| `MeshIntersectionGraph` 装配 | collision.rs（既有） | `find_mesh_mesh_intersections` 的 add_node/add_edge 面 | 保持（图类型已有；find_mesh 装配登记） |

## 批 2 — 函数实现/判等

| VA | 函数 | verdict | 实现/分歧 |
|---|---|---|---|
| 0x1408d8710 | `on_level_reload` | 结构判等 | 全量 `RaycastWorld::new()` 重建（发布器/槽/MeshInstanceSources 复位）；+0x10/+0x1c 计数器写登记 |
| 0x1408d0390/0x140b26a80/0x140b26bd0 | `to_offset_collider_builder` impl$23/21/22 | 结构判等 | `ColliderBuilder::new(shape).position(iso).sensor(true)`（Arc 增引用计数面等价） |
| 0x1408d98c0/0x14155d340/0x1408d9960/0x140b26d20 | `into_transformed_shape` impl$42/41/43/45 | 结构判等 | `into_transformed_shape(handle)` / `rectangle_to_transformed` / `into_transformed_shape_iso`（Rectangle3d 取向 from_rotation_arc 登记） |
| 0x1408d9990 | `as_collider_shape` impl$47 | 判等 | Arc 克隆访问器 |
| 0x14129b780/0x14129bad0/0x140254890/0x140254aa0 | `iter_shape_overlaps` ×4 | 结构判等 | `iter_shape_overlaps`/`_handle`/`_iso`：mask 门控 + `intersections_with_shape`（rapier 0.22 回调 `FnMut(ColliderHandle)`，source 由槽号映射） |
| 0x142612d10 | `shape_overlaps_with<Rectangle3d>` | 结构判等 | 矩形→cuboid + `intersection_with_shape`（from_rotation_arc 取向登记） |
| 0x14129be00 | `decorator_overlaps_with_others` | 结构判等 | 装饰形状扫掠 + 注册表 kind 门控（来源槽映射、DecoratorRank 校验登记） |
| 0x141326370 | `visualize_colliders` | 结构判等 | `for_each_collider` 全槽 + 注册表 (index,gen) 校验 + kind×DebugGui+0x104 过滤（kind 过滤 11 分支逐条对应）→ `visualize_shape` |
| 0x141326930 | `visualize_shape` | 结构判等 | compound/trimesh/convex_polyhedron/cuboid/ball 分派；三角线框 `[a,b,c,a]` 4 点闭环；`DebugLine`/`DebugSphere` 请求清单（`show` ECS 胶水登记 host） |
| 0x1408f2850 | `merge_segments_into_polylines` | 结构判等 | HashMap<IVec3,VertexId> 端点查表 + Polyline（VecDeque）初建/追加/拼接/闭环 + 中点平均；方向整理与 VecDeque 环缓冲字节布局登记 |
| 0x1408bc530 | `RtBvhBuilderGlueResource::is_idle` | 判等 | RwLock 读 + poison 位 + pending==0 |
| 0x1408bc460/510/520 | `impl$1/2/5::from` | 判等 | Vec3→u32 颜色（sqrt*255+0.5 打包）/ {1,值} / 空 |

**实现：12 函数**（to_offset 3、into_transformed 3 入口、overlaps 6 入口、on_level_reload、
visualize 2、merge_segments 1、rt 纯逻辑 3 —— 按入口计共 12）。

## 批 3 — 登记（未实现/未完全判等）

| VA | 函数 | 登记原因 |
|---|---|---|
| 0x140a97b70 | `find_mesh_mesh_intersections` | 13359 B monolith：GraphMap 装配 + `intersect_newly_added_colliders` + `remove_invalid_polylines` + `BirdNestCandidateLocation`/`SparseInstanceBuffer<SparkleSsbo>`/`take<ColliderDiff>` 跨域依赖（render/bird-nest），登记 |
| 0x1408d0490 | `update_rapier_colliders` | **decompile-hardpoint**：`--write` worker `impl-collision-types` 已修 `void()` → `void update_rapier_colliders(longlong *this)` 并 save（save 证据）；fresh 仍 `Recoverable Error: Unable to find unique hash for varnode`（AI_REVERSE_TOOLS.md §7 已登记同类）。callee 面 41 个已落盘（construct_arch_geo/get_collision_geo/new_indexed/upsert_collider/dispose_collider_mesh_instance/floor_mesh 等）——逐分支实现待 hardpoint 攻克 |
| 0x1408bc650/0x1408bcff0 | `update_rt_world`/`upload_rt_world` | 需 `rhapsody GraphicsDevice::update_buffer_bytes` + `ssbo_library::SsboId` FrozenMap（country-core 现 `TextureDevice` 抽象未接，host 面），登记 |
| 0x140b0da50/0x140b24d50 | `build_gpu_bvh`/`bvh_builder_worker` | 需 `obvhs` crate（workspace 未引入）+ rhapsody 设备；`build_cwbvh_from_tris` 依赖未接，登记 |
| 0x1415626f0/0x141562aa0/0x1400cce50 | `RtWorld::get_mut`×2/`get` | DirtyCow COW 语义 + HashMap 插删已结构了解，未落地（RtMesh 字段语义未锚定），登记 |

## 编译状态

- `cargo check -p country-core --lib`（default）：0 error。
- `cargo check -p country-core --lib --features native-physics`：0 error。
- 并行 agent WIP 间歇在 `resources/history/decorator_edit.rs`（非本模块）报 2 E，随后自愈；本批文件无错误。
- 单测：collision（13）+ rt（3）= 16 PASS（`--features native-physics`）。

## 后续批次建议

1. `update_rapier_colliders`：继续按 half_timbering 攻克路径（更完整 typed prototype / 反编译器选项）在
   `impl-collision-types` worker 定点；callee 面已备，攻克后逐分支重建 collider 生命周期。
2. `find_mesh_mesh_intersections`：需 render/bird-nest 域类型（BirdNestCandidateLocation、SparkleSsbo、
   ColliderDiff 载荷）到位后实现。
3. `RtWorld::get_mut` 族 + `update_rt_world`/`upload_rt_world`：RtMesh 字段语义锚定（default 写序列已备）+
   rhapsody 设备接线。
4. `resources/rt` 的 `bvh`/`bvh_builder`：`obvhs` 依赖入 workspace 后实现。
